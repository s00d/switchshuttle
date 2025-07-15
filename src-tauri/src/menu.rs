use crate::config::ConfigManager;
use crate::console;
use crate::execute::execute_command;
use crate::helpers::{
    change_devtools, create_check_menu_item, create_menu_item, create_window, get_config_path,
    open_folder_in_default_explorer, open_in_default_editor,
};
use crate::menu_structure::SystemMenu;
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Wry};
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_notification::NotificationExt;
use tauri_plugin_opener::OpenerExt;

/// Выполняет команду по ID через единую точку входа
pub fn execute_command_by_id(
    app: &AppHandle<Wry>,
    command_id: &str,
    config_manager: &ConfigManager,
) -> Result<(), String> {
    println!("[Execute] Looking for command with ID: '{}'", command_id);

    // Приостанавливаем таймеры мониторинга перед выполнением команды
    pause_monitor_timers();
    println!("[Execute] Paused monitor timers before command execution");

    match config_manager.find_command_by_id(command_id) {
        Some((command, config)) => {
            println!(
                "[Execute] Found command: '{}' (ID: {:?})",
                command.name, command.id
            );
            println!("[Execute] Command has switch: {:?}", command.switch);
            println!("[Execute] Command has inputs: {:?}", command.inputs);

            // Проверяем, является ли это командой переключения
            if command.switch.is_some() {
                let should_show_inputs = command
                    .inputs
                    .as_ref()
                    .map(|inputs| !inputs.is_empty())
                    .unwrap_or(false)
                    && command.id.is_some();

                if should_show_inputs {
                    if let Err(e) = create_window(
                        app,
                        "inputs",
                        "SwitchShuttle - Provide Inputs",
                        &format!("/inputs/{}", command.id.as_ref().unwrap()),
                        400.0,
                        300.0,
                        true,
                    ) {
                        return Err(format!("Failed to create inputs window: {}", e));
                    }
                } else {
                    // Выполняем команду переключения через execute_command_silent
                    if let Some(toggle_command) = &command.command {
                        println!("[Monitor] spawn: toggle_command = '{}'", toggle_command);
                        match console::ConsoleInstance::execute_command_silent(toggle_command) {
                            Ok(_) => {
                                // Показываем уведомление об успешном выполнении
                                if let Ok(_) = app
                                    .notification()
                                    .builder()
                                    .title("SwitchShuttle Success")
                                    .body(&format!(
                                        "Switch '{}' executed successfully",
                                        command.name
                                    ))
                                    .show()
                                {
                                    // Уведомление отправлено
                                }
                                // Обновляем меню после выполнения команды
                                update_system_tray_menu(app, config_manager);
                            }
                            Err(e) => {
                                eprintln!("Failed to execute switch command: {}", e);
                                // Показываем уведомление об ошибке
                                if let Ok(_) = app
                                    .notification()
                                    .builder()
                                    .title("SwitchShuttle Error")
                                    .body(&format!(
                                        "Failed to execute switch '{}': {}",
                                        command.name, e
                                    ))
                                    .show()
                                {
                                    // Уведомление отправлено
                                }
                                return Err(format!("Failed to execute switch command: {}", e));
                            }
                        }
                    }
                }
            } else {
                let should_show_inputs = command
                    .inputs
                    .as_ref()
                    .map(|inputs| !inputs.is_empty())
                    .unwrap_or(false)
                    && command.id.is_some();

                if should_show_inputs {
                    if let Err(e) = create_window(
                        app,
                        "main",
                        "SwitchShuttle - Provide Inputs",
                        &format!("/inputs/{}", command.id.as_ref().unwrap()),
                        400.0,
                        300.0,
                        true,
                    ) {
                        return Err(format!("Failed to create inputs window: {}", e));
                    }
                } else {
                    execute_command(
                        command,
                        &config.terminal,
                        &config.launch_in,
                        &config.theme,
                        &config.title,
                    );
                }
            }

            Ok(())
        }
        None => {
            println!("[Execute] Command not found for ID: '{}'", command_id);
            println!("[Execute] Available commands:");
            for config in &config_manager.configs {
                for command in &config.commands {
                    println!("[Execute]   - '{}' (ID: {:?})", command.name, command.id);
                }
            }

            // Возобновляем таймеры мониторинга даже если команда не найдена
            resume_monitor_timers();
            println!("[Execute] Resumed monitor timers after command not found");

            Err(format!("Command not found for ID: '{}'", command_id))
        }
    }
}

// Глобальное состояние для хранения текущей структуры меню
static CURRENT_MENU: Lazy<Arc<Mutex<Option<SystemMenu>>>> =
    Lazy::new(|| Arc::new(Mutex::new(None)));

pub fn create_system_tray_menu(
    app: &AppHandle<Wry>,
    autostart: bool,
    config_manager: &ConfigManager,
) -> tauri::menu::Menu<Wry> {
    // Останавливаем таймеры и планировщики в текущем меню, если оно существует
    if let Some(mut current_menu) = CURRENT_MENU.lock().unwrap().take() {
        current_menu.stop_all_monitor_timers();
        current_menu.stop_all_schedulers();
    }

    // Создаем структуру меню из конфигураций
    let mut system_menu = SystemMenu::from_configs_with_states(&config_manager.configs, Some(app));

    // Создаем Tauri меню из структуры (это сохранит tauri_icon_item)
    let tray_menu = system_menu.create_tauri_menu(app);

    // Теперь запускаем индивидуальные таймеры для элементов с мониторингом и планировщики
    system_menu.start_all_monitor_timers();
    system_menu.start_all_schedulers();

    // Запускаем периодическую очистку пула соединений
    SystemMenu::cleanup_console_pool_periodically();

    // Сохраняем новую структуру меню
    *CURRENT_MENU.lock().unwrap() = Some(system_menu);

    // Создаем новый MenuBuilder для добавления системных элементов
    let mut tray_menu_builder = tauri::menu::MenuBuilder::new(app);

    // Добавляем элементы из структуры меню
    for item in tray_menu.items().unwrap() {
        tray_menu_builder = tray_menu_builder.item(&item);
    }

    // Добавляем системные элементы меню
    tray_menu_builder = tray_menu_builder.separator();

    let mut edit_config_submenu = tauri::menu::SubmenuBuilder::new(app, "🚀 Edit Config");

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
    println!("[Tray Event] Received menu event with ID: '{}'", event_id);
    println!("[Tray Event] Event type: {:?}", event);

    let config_path = get_config_path();

    match event_id {
        "settings" => {
            println!("[Tray Event] Handling settings event");
            if let Err(e) = create_window(
                &app,
                "settings",
                "SwitchShuttle - Settings",
                "/settings",
                900.0,
                700.0,
                true,
            ) {
                eprintln!("Failed to create settings window: {}", e);
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
                eprintln!("Failed to create about window: {}", e);
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
                eprintln!("Failed to create help window: {}", e);
            }
        }
        "quit" => std::process::exit(0),
        "refresh_configurations" => {
            // Перезагружаем конфигурации и обновляем меню
            let mut config_manager = config_manager.lock().unwrap();
            config_manager
                .load_configs(Some(&app))
                .expect("Failed to reload configs");

            // Обновляем меню в трее
            update_system_tray_menu(app, &config_manager);
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
                eprintln!("Failed to create config editor window: {}", e);
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
            update_system_tray_menu(app, &config_manager.lock().unwrap());
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
        _ => {
            println!("[Tray Event] Handling unknown event ID: '{}'", event_id);
            if event_id.starts_with("edit_") {
                println!("[Tray Event] Handling edit config event for: {}", event_id);
                let config_file_name = event_id.replacen("edit_", "", 1);
                let config_file_path = config_path.parent().unwrap().join(&config_file_name);
                open_in_default_editor(&config_file_path);
            } else {
                println!("[Tray Event] Looking for command with ID: '{}'", event_id);
                let config_manager = config_manager.lock().unwrap();
                if let Err(e) = execute_command_by_id(&app, event_id, &config_manager) {
                    eprintln!("[Tray Event] Failed to execute command: {}", e);
                }
            }
        }
    }
}

/// Обновляет меню в трее с правильной обработкой таймеров
pub fn update_system_tray_menu(app: &AppHandle<Wry>, config_manager: &ConfigManager) {
    // Создаем новое меню
    let new_menu = create_system_tray_menu(
        app,
        app.autolaunch().is_enabled().unwrap_or(false),
        config_manager,
    );

    // Обновляем меню в трее
    if let Some(tray) = app.tray_by_id("switch-shuttle-tray") {
        if let Err(e) = tray.set_menu(Some(new_menu)) {
            eprintln!("Failed to update tray menu: {}", e);
        }
    }
}

pub fn resume_monitor_timers() {
    if let Some(current_menu) = CURRENT_MENU.lock().unwrap().as_mut() {
        current_menu.start_all_monitor_timers();
    }
}

pub fn pause_monitor_timers() {
    if let Some(current_menu) = CURRENT_MENU.lock().unwrap().as_mut() {
        current_menu.stop_all_monitor_timers();
    }
}
