use crate::config::{
    switch_toggle_command, CommandConfig, Config, ConfigManager,
};
use crate::console;
use crate::execute::execute_command;
use crate::helpers::{
    change_devtools, create_check_menu_item, create_menu_item, create_window, get_config_path,
    open_folder_in_default_explorer, open_in_default_editor,
};
use crate::menu_structure::SystemMenu;
use crate::running::{
    self, parse_stop_menu_id, stop_menu_id, with_registry, ProcessRegistry, STOP_ALL_MENU_ID,
};
use crate::settings::{command_is_blocked, AppSettings};
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager, Wry, image::Image};
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_opener::OpenerExt;
use log::{error, info};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrayRefresh {
    /// Config changed — restart schedulers.
    Full,
    /// Running list changed — reconcile schedulers (keep overlapping).
    Soft,
}

/// Test seam for B3: counts resume calls after dispatch.
static RESUME_CALLS: AtomicUsize = AtomicUsize::new(0);
static PAUSE_CALLS: AtomicUsize = AtomicUsize::new(0);

pub fn with_monitors_paused<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    pause_monitor_timers();
    PAUSE_CALLS.fetch_add(1, Ordering::SeqCst);
    let result = f();
    resume_monitor_timers();
    RESUME_CALLS.fetch_add(1, Ordering::SeqCst);
    result
}

#[cfg(test)]
pub fn reset_monitor_pause_counters() {
    PAUSE_CALLS.store(0, Ordering::SeqCst);
    RESUME_CALLS.store(0, Ordering::SeqCst);
}

#[cfg(test)]
pub fn monitor_pause_resume_counts() -> (usize, usize) {
    (
        PAUSE_CALLS.load(Ordering::SeqCst),
        RESUME_CALLS.load(Ordering::SeqCst),
    )
}

fn security_blocks_lines(lines: &[String], settings: &AppSettings) -> Option<String> {
    for line in lines {
        if command_is_blocked(line, &settings.security) {
            return Some(format!("Command blocked by security settings: {}", line));
        }
    }
    None
}

fn notify_error(app: &AppHandle<Wry>, body: &str) {
    if let Some(state) = app.try_state::<Arc<Mutex<AppSettings>>>() {
        if let Ok(settings) = state.lock() {
            let _ = settings.show_error_notification(app, "SwitchShuttle Error", body);
            return;
        }
    }
}

fn notify_success(app: &AppHandle<Wry>, body: &str) {
    if let Some(state) = app.try_state::<Arc<Mutex<AppSettings>>>() {
        if let Ok(settings) = state.lock() {
            let _ = settings.show_success_notification(app, "SwitchShuttle Success", body);
        }
    }
}

/// Unified command dispatch for tray / hotkey / invoke.
pub fn dispatch_user_command(
    app: &AppHandle<Wry>,
    command: &CommandConfig,
    config: &Config,
    config_manager: &ConfigManager,
) -> Result<(), String> {
    with_monitors_paused(|| dispatch_user_command_inner(app, command, config, config_manager))
}

fn dispatch_user_command_inner(
    app: &AppHandle<Wry>,
    command: &CommandConfig,
    config: &Config,
    config_manager: &ConfigManager,
) -> Result<(), String> {
    let settings = app
        .try_state::<Arc<Mutex<AppSettings>>>()
        .and_then(|s| s.lock().ok().map(|g| g.clone()))
        .unwrap_or_else(AppSettings::default);

    match crate::config::execution_target(command) {
        crate::config::ExecutionTarget::OpenInputs => {
            let label = if command.switch.is_some() {
                "inputs"
            } else {
                "main"
            };
            create_window(
                app,
                label,
                "SwitchShuttle - Provide Inputs",
                &format!("/inputs/{}", command.id.as_ref().unwrap()),
                400.0,
                300.0,
                true,
            )
            .map_err(|e| format!("Failed to create inputs window: {}", e))?;
            Ok(())
        }
        crate::config::ExecutionTarget::SwitchSilent => {
            let Some(toggle) = switch_toggle_command(command) else {
                return Err("Switch command has no toggle command".into());
            };
            if let Some(msg) = security_blocks_lines(&[toggle.to_string()], &settings) {
                notify_error(app, &msg);
                return Err(msg);
            }
            match console::ConsoleInstance::execute_command_silent(toggle) {
                Ok(_) => {
                    notify_success(
                        app,
                        &format!("Switch '{}' executed successfully", command.name),
                    );
                    update_system_tray_menu(app, config_manager, TrayRefresh::Full);
                    Ok(())
                }
                Err(e) => {
                    let msg = format!("Failed to execute switch '{}': {}", command.name, e);
                    notify_error(app, &msg);
                    Err(msg)
                }
            }
        }
        crate::config::ExecutionTarget::BackgroundJob => {
            let lines = command
                .commands
                .clone()
                .unwrap_or_default()
                .into_iter()
                .filter(|c| !c.trim().is_empty())
                .collect::<Vec<_>>();
            if let Some(msg) = security_blocks_lines(&lines, &settings) {
                notify_error(app, &msg);
                return Err(msg);
            }
            let command_id = command
                .id
                .clone()
                .unwrap_or_else(|| command.name.clone());
            match with_registry(|reg| reg.start(&command_id, &command.name, &lines)) {
                Ok(_) => {
                    update_system_tray_menu(app, config_manager, TrayRefresh::Soft);
                    Ok(())
                }
                Err(e) => {
                    notify_error(app, &e);
                    Err(e)
                }
            }
        }
        crate::config::ExecutionTarget::Terminal => {
            let lines = command.commands.clone().unwrap_or_default();
            if let Some(msg) = security_blocks_lines(&lines, &settings) {
                notify_error(app, &msg);
                return Err(msg);
            }
            execute_command(
                command,
                &config.terminal,
                &config.launch_in,
                &config.theme,
                &config.title,
            );
            Ok(())
        }
    }
}

/// Выполняет команду по ID через единую точку входа
pub fn execute_command_by_id(
    app: &AppHandle<Wry>,
    command_id: &str,
    config_manager: &ConfigManager,
) -> Result<(), String> {
    info!("[Execute] Looking for command with ID: '{}'", command_id);

    match config_manager.find_command_by_id(command_id) {
        Some((command, config)) => {
            info!(
                "[Execute] Found command: '{}' (ID: {:?})",
                command.name, command.id
            );
            dispatch_user_command(app, command, config, config_manager)
        }
        None => {
            info!("[Execute] Command not found for ID: '{}'", command_id);
            Err(format!("Command not found for ID: '{}'", command_id))
        }
    }
}

// Глобальное состояние для хранения текущей структуры меню
static CURRENT_MENU: Lazy<Arc<Mutex<Option<SystemMenu>>>> =
    Lazy::new(|| Arc::new(Mutex::new(None)));

static APP_FOR_RUNNING: Lazy<Mutex<Option<AppHandle<Wry>>>> = Lazy::new(|| Mutex::new(None));

pub fn set_running_app_handle(app: AppHandle<Wry>) {
    *APP_FOR_RUNNING.lock().unwrap() = Some(app);
}

pub fn refresh_tray_after_running_change() {
    let app = APP_FOR_RUNNING.lock().unwrap().clone();
    let Some(app) = app else {
        return;
    };
    let Some(cm) = app.try_state::<Arc<Mutex<ConfigManager>>>() else {
        return;
    };
    let cm = cm.inner().clone();
    let app_for_update = app.clone();
    let _ = app.run_on_main_thread(move || {
        if let Ok(guard) = cm.lock() {
            update_system_tray_menu(&app_for_update, &guard, TrayRefresh::Soft);
        }
    });
}
pub fn create_system_tray_menu(
    app: &AppHandle<Wry>,
    autostart: bool,
    config_manager: &ConfigManager,
    refresh: TrayRefresh,
) -> tauri::menu::Menu<Wry> {
    let mut old_menu = CURRENT_MENU.lock().unwrap().take();

    if let Some(ref mut current_menu) = old_menu {
        current_menu.stop_all_monitor_timers();
        if refresh == TrayRefresh::Full {
            current_menu.stop_all_schedulers();
        }
    }

    let mut system_menu = SystemMenu::from_configs_with_states(&config_manager.configs, Some(app));

    let tray_menu = system_menu.create_tauri_menu(app);

    match refresh {
        TrayRefresh::Full => {
            system_menu.start_all_schedulers();
        }
        TrayRefresh::Soft => {
            if let Some(ref mut old) = old_menu {
                system_menu.reconcile_schedulers_from(old);
            } else {
                system_menu.start_all_schedulers();
            }
        }
    }

    SystemMenu::cleanup_console_pool_periodically();

    *CURRENT_MENU.lock().unwrap() = Some(system_menu);

    let mut tray_menu_builder = tauri::menu::MenuBuilder::new(app);

    for item in tray_menu.items().unwrap() {
        tray_menu_builder = tray_menu_builder.item(&item);
    }

    // Running section
    let running = with_registry(|reg| reg.list());
    if !running.is_empty() {
        tray_menu_builder = tray_menu_builder.separator();
        let labels = ProcessRegistry::job_labels(&running);
        let mut running_submenu = tauri::menu::SubmenuBuilder::new(app, "Running");
        for (run_id, label) in labels {
            running_submenu = running_submenu.item(&create_menu_item(
                app,
                &stop_menu_id(run_id),
                &format!("⏹ {}", label),
                "exit",
                None,
                None,
            ));
        }
        running_submenu = running_submenu.separator();
        running_submenu = running_submenu.item(&create_menu_item(
            app,
            STOP_ALL_MENU_ID,
            "Stop All",
            "exit",
            None,
            None,
        ));
        tray_menu_builder = tray_menu_builder.item(&running_submenu.build().unwrap());
    }

    tray_menu_builder = tray_menu_builder.separator();

    let edit_config_icon = Image::from_bytes(include_bytes!("../icons/edit.png")).unwrap();

    let mut edit_config_submenu = tauri::menu::SubmenuBuilder::new(app, "Edit Config")
        .submenu_icon(edit_config_icon);

    for path in &config_manager.config_paths {
        let file_name = path.file_name().unwrap().to_string_lossy().to_string();
        edit_config_submenu = edit_config_submenu.item(&create_menu_item(
            app,
            &format!("edit_{}", file_name),
            &file_name,
            "edit",
            None,
            None,
        ));
    }

    edit_config_submenu = edit_config_submenu.separator();
    edit_config_submenu = edit_config_submenu.item(&create_menu_item(
        app,
        "open_config_folder",
        "Show Config Folder",
        "folder",
        None,
        None,
    ));
    edit_config_submenu = edit_config_submenu.item(&create_menu_item(
        app,
        "open_config_editor",
        "Open Visual Editor",
        "visual",
        None,
        None,
    ));

    edit_config_submenu = edit_config_submenu.separator();
    edit_config_submenu = edit_config_submenu.item(&create_menu_item(
        app,
        "refresh_configurations",
        "Refresh Configurations",
        "refresh_settings",
        None,
        None,
    ));

    tray_menu_builder = tray_menu_builder.item(&edit_config_submenu.build().unwrap());

    tray_menu_builder = tray_menu_builder.separator();

    tray_menu_builder = tray_menu_builder.item(&create_check_menu_item(
        app,
        "toggle_launch_at_login",
        "Launch at Login",
        autostart,
        None,
        None,
    ));

    tray_menu_builder = tray_menu_builder.separator();

    if cfg!(debug_assertions) {
        tray_menu_builder = tray_menu_builder.item(&create_menu_item(
            app,
            "open_devtools",
            "Open DevTools",
            "devtools",
            None,
            None,
        ));

        tray_menu_builder = tray_menu_builder.separator();
    }

    tray_menu_builder = tray_menu_builder.item(&create_menu_item(
        app, "settings", "Settings", "config", None, None,
    ));
    tray_menu_builder =
        tray_menu_builder.item(&create_menu_item(app, "about", "About", "info", None, None));
    tray_menu_builder =
        tray_menu_builder.item(&create_menu_item(app, "help", "Help", "help", None, None));
    tray_menu_builder = tray_menu_builder.item(&create_menu_item(
        app, "homepage", "Homepage", "site", None, None,
    ));

    tray_menu_builder = tray_menu_builder.separator();
    tray_menu_builder = tray_menu_builder.item(&create_menu_item(
        app,
        "quit",
        "Quit SwitchShuttle",
        "exit",
        None,
        None,
    ));

    tray_menu_builder.build().unwrap()
}

pub fn handle_system_tray_event(
    app: &AppHandle<Wry>,
    event: tauri::menu::MenuEvent,
    config_manager: Arc<Mutex<ConfigManager>>,
) {
    let event_id = event.id().0.as_str();
    info!("[Tray Event] Received menu event with ID: '{}'", event_id);
    info!("[Tray Event] Event type: {:?}", event);

    let config_path = get_config_path();

    match event_id {
        "settings" => {
            info!("[Tray Event] Handling settings event");
            if let Err(e) = create_window(
                &app,
                "settings",
                "SwitchShuttle - Settings",
                "/settings",
                900.0,
                700.0,
                true,
            ) {
                error!("Failed to create settings window: {}", e);
            }
        }
        "about" => {
            if let Err(e) = create_window(
                &app,
                "about",
                "SwitchShuttle - About",
                "/about",
                800.0,
                600.0,
                true,
            ) {
                error!("Failed to create about window: {}", e);
            }
        }
        "help" => {
            if let Err(e) = create_window(
                &app,
                "help",
                "SwitchShuttle - Help",
                "/help",
                1000.0,
                800.0,
                true,
            ) {
                error!("Failed to create help window: {}", e);
            }
        }
        "quit" => {
            running::prepare_quit();
            std::process::exit(0);
        }
        "refresh_configurations" => {
            let mut config_manager = config_manager.lock().unwrap();
            config_manager
                .load_configs(Some(&app))
                .expect("Failed to reload configs");
            update_system_tray_menu(app, &config_manager, TrayRefresh::Full);
        }
        "edit_config" => open_in_default_editor(&config_path),
        "open_config_folder" => {
            open_folder_in_default_explorer(&config_path.parent().unwrap().to_path_buf())
        }
        "open_config_editor" => {
            if let Err(e) = create_window(
                &app,
                "main",
                "SwitchShuttle - Config Editor",
                "/editor",
                800.0,
                600.0,
                true,
            ) {
                error!("Failed to create config editor window: {}", e);
            }
        }
        "toggle_launch_at_login" => {
            let autostart_manager = app.autolaunch();
            let enabled = autostart_manager.is_enabled().unwrap();
            if enabled {
                autostart_manager.disable().unwrap();
            } else {
                autostart_manager.enable().unwrap();
            }
            update_system_tray_menu(app, &config_manager.lock().unwrap(), TrayRefresh::Full);
        }
        "homepage" => {
            let homepage_url = "https://github.com/s00d/SwitchShuttle";
            let opener = app.opener();
            opener.open_url(homepage_url, None::<&str>).unwrap();
        }
        "open_devtools" => {
            if cfg!(debug_assertions) {
                change_devtools(app);
            }
        }
        STOP_ALL_MENU_ID => {
            let _ = with_registry(|reg| reg.stop_all());
            update_system_tray_menu(
                app,
                &config_manager.lock().unwrap(),
                TrayRefresh::Soft,
            );
        }
        _ => {
            info!("[Tray Event] Handling unknown event ID: '{}'", event_id);
            if let Some(run_id) = parse_stop_menu_id(event_id) {
                let _ = with_registry(|reg| reg.stop(run_id));
                update_system_tray_menu(
                    app,
                    &config_manager.lock().unwrap(),
                    TrayRefresh::Soft,
                );
            } else if event_id.starts_with("edit_") {
                info!("[Tray Event] Handling edit config event for: {}", event_id);
                let config_file_name = event_id.replacen("edit_", "", 1);
                let config_file_path = config_path.parent().unwrap().join(&config_file_name);
                open_in_default_editor(&config_file_path);
            } else {
                info!("[Tray Event] Looking for command with ID: '{}'", event_id);
                let config_manager = config_manager.lock().unwrap();
                if let Err(e) = execute_command_by_id(&app, event_id, &config_manager) {
                    error!("[Tray Event] Failed to execute command: {}", e);
                }
            }
        }
    }
}

/// Обновляет меню в трее с правильной обработкой таймеров
pub fn update_system_tray_menu(
    app: &AppHandle<Wry>,
    config_manager: &ConfigManager,
    refresh: TrayRefresh,
) {
    let new_menu = create_system_tray_menu(
        app,
        app.autolaunch().is_enabled().unwrap_or(false),
        config_manager,
        refresh,
    );

    if let Some(status_item) = new_menu.get("status") {
        if let Some(menuitem) = status_item.as_menuitem() {
            if let Err(e) = menuitem.set_text("Status: Ready") {
                eprintln!("Failed to update menu text: {}", e);
            }
        }
    }

    if let Some(tray) = app.tray_by_id("switch-shuttle-tray") {
        if let Err(e) = tray.set_menu(Some(new_menu)) {
            error!("Failed to update tray menu: {}", e);
        }
    }
}

pub fn resume_monitor_timers() {
    // Устанавливаем состояние трея как активное
    if let Ok(mut tray_active) = crate::menu_structure::TRAY_ACTIVE.lock() {
        *tray_active = true;
        info!("[Monitor] Tray state set to active");
    }
    
    if let Some(current_menu) = CURRENT_MENU.lock().unwrap().as_mut() {
        current_menu.start_all_monitor_timers();
    }
}

pub fn pause_monitor_timers() {
    // Устанавливаем состояние трея как неактивное
    if let Ok(mut tray_active) = crate::menu_structure::TRAY_ACTIVE.lock() {
        *tray_active = false;
        info!("[Monitor] Tray state set to inactive");
    }
    
    if let Some(current_menu) = CURRENT_MENU.lock().unwrap().as_mut() {
        current_menu.stop_all_monitor_timers();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_monitors_paused_always_resumes_ok_and_err() {
        reset_monitor_pause_counters();
        let ok = with_monitors_paused(|| Ok::<_, String>(42));
        assert_eq!(ok.unwrap(), 42);
        let err: Result<(), String> = with_monitors_paused(|| Err("boom".into()));
        assert!(err.is_err());
        let (pauses, resumes) = monitor_pause_resume_counts();
        assert_eq!(pauses, 2);
        assert_eq!(resumes, 2);
    }
}
