use crate::config::{CommandConfig, ConfigManager};
use crate::helpers;
use crate::helpers::{
    change_devtools, create_window, get_config_path, open_folder_in_default_explorer,
    open_in_default_editor, create_menu_item, create_check_menu_item
};
use std::sync::{Arc, Mutex};
use tauri::menu::{
    CheckMenuItem, IconMenuItem, MenuBuilder, Submenu, SubmenuBuilder,
};
use tauri::{AppHandle, Wry};
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_opener::OpenerExt;
use tauri_plugin_notification::NotificationExt;

fn create_sub_menu(app: &AppHandle<Wry>, submenu_items: &Vec<CommandConfig>, name: &str, icon: Option<String>) -> Submenu<Wry> {
    let name_with_icon = if let Some(icon_symbol) = icon {
        format!("{} {}", icon_symbol, name)
    } else {
        format!("📁 {}", name)
    };
    let mut submenu_builder = SubmenuBuilder::new(app, &name_with_icon);
    for item in submenu_items {
        if let Some(sub_items) = &item.submenu {
            let sub_submenu = create_sub_menu(app, sub_items, &item.name, item.icon.clone());
            submenu_builder = submenu_builder.item(&sub_submenu);
        } else {
        let id = item.id.clone().unwrap_or(item.name.clone());

            if item.switch.is_some() {
                // Это переключатель - получаем состояние
                let is_enabled = helpers::is_switch_enabled(&id, Some(app));
                let menu_item = create_check_menu_item(
                    app,
                    &id,
                    &item.name,
                    is_enabled,
                    item.hotkey.clone(),
                    item.icon.as_deref(),
                );
                submenu_builder = submenu_builder.item(&menu_item);
            } else if item.monitor.is_some() {
                // Это команда мониторинга - получаем данные
                let mut display_name = item.name.clone();
                if let Some(monitor_command) = &item.monitor {
                    match helpers::execute_command_silent(monitor_command) {
                        Ok(output) => {
                            let result = output.trim();
                            display_name = format!("{}: {}", item.name, result);
                        }
                        Err(_) => {
                            // В случае ошибки оставляем оригинальное название
                        }
                    }
                }
                submenu_builder = submenu_builder.item(&create_menu_item(app, &id, &display_name, "terminal", item.hotkey.clone(), item.icon.as_deref()));
            } else {
                // Обычная команда
                submenu_builder = submenu_builder.item(&create_menu_item(app, &id, &item.name, "terminal", item.hotkey.clone(), item.icon.as_deref()));
            }
        }
    }
    submenu_builder.build().unwrap()
}

enum MenuItemOrSubmenu {
    // MenuItem(MenuItem<Wry>),
    Submenu(Submenu<Wry>),
    IconItem(IconMenuItem<Wry>),
    CheckItem(CheckMenuItem<Wry>),
}

pub fn create_system_tray_menu(
    app: &AppHandle<Wry>,
    autostart: bool,
    config_manager: &ConfigManager,
) -> tauri::menu::Menu<Wry> {
    let mut tray_menu_builder = MenuBuilder::new(app);

    let mut menu_items = Vec::new();

    for config in &config_manager.configs {
        // Пропускаем отключенные конфигурации
        if let Some(enabled) = config.enabled {
            if !enabled {
                continue;
            }
        }
        
        for command in &config.commands {
            if let Some(submenu_items) = &command.submenu {
                let submenu = create_sub_menu(app, &submenu_items.clone(), &command.name, command.icon.clone());
                menu_items.push(MenuItemOrSubmenu::Submenu(submenu));
            } else {
                let id = command.id.clone().unwrap_or(command.name.clone());
                
                // Проверяем, является ли это переключателем
                if command.switch.is_some() {
                    // Это переключатель - получаем состояние
                    let is_enabled = helpers::is_switch_enabled(&id, Some(app));
                    let menu_item = create_check_menu_item(
                        app,
                        &id,
                        &command.name,
                        is_enabled,
                        command.hotkey.clone(),
                        command.icon.as_deref(),
                    );
                    menu_items.push(MenuItemOrSubmenu::CheckItem(menu_item));
                } else if command.monitor.is_some() {
                    // Это команда мониторинга - получаем данные
                    let mut display_name = command.name.clone();
                    if let Some(monitor_command) = &command.monitor {
                        match helpers::execute_command_silent(monitor_command) {
                            Ok(output) => {
                                let result = output.trim();
                                display_name = format!("{}: {}", command.name, result);
                            }
                            Err(_) => {
                                // В случае ошибки оставляем оригинальное название
                            }
                        }
                    }
                    let menu_item = create_menu_item(app, &id, &display_name, "terminal", command.hotkey.clone(), command.icon.as_deref());
                    menu_items.push(MenuItemOrSubmenu::IconItem(menu_item));
                } else {
                    let menu_item = create_menu_item(app, &id, &command.name, "terminal", command.hotkey.clone(), command.icon.as_deref());
                    menu_items.push(MenuItemOrSubmenu::IconItem(menu_item));
                }
            }
        }
    }

    for item in menu_items {
        match item {
            // MenuItemOrSubmenu::MenuItem(menu_item) => {
            //     tray_menu_builder = tray_menu_builder.item(&menu_item);
            // }
            MenuItemOrSubmenu::Submenu(submenu) => {
                tray_menu_builder = tray_menu_builder.item(&submenu);
            }
            MenuItemOrSubmenu::IconItem(submenu) => {
                tray_menu_builder = tray_menu_builder.item(&submenu);
            }
            MenuItemOrSubmenu::CheckItem(check_item) => {
                tray_menu_builder = tray_menu_builder.item(&check_item);
            }
        }
    }

    tray_menu_builder = tray_menu_builder.separator();

    let mut edit_config_submenu = SubmenuBuilder::new(app, "🚀 Edit Config");

    for path in &config_manager.config_paths {
        let file_name = path.file_name().unwrap().to_string_lossy().to_string();
        edit_config_submenu = edit_config_submenu.item(
            &create_menu_item(app, &format!("edit_{}", file_name), &file_name, "edit",  None, None)
        );
    }

    edit_config_submenu = edit_config_submenu.separator();
    edit_config_submenu = edit_config_submenu.item(
        &create_menu_item(app, "open_config_folder", "Show Config Folder", "folder",  None, None)
    );
    edit_config_submenu = edit_config_submenu.item(
        &create_menu_item(app, "open_config_editor", "Open Visual Editor", "visual",  None, None)
    );

    edit_config_submenu = edit_config_submenu.separator();
    edit_config_submenu = edit_config_submenu.item(
        &create_menu_item(app, "refresh_configurations", "Refresh Configurations", "refresh_settings",  None, None)
    );

    tray_menu_builder = tray_menu_builder.item(&edit_config_submenu.build().unwrap());

    tray_menu_builder = tray_menu_builder.separator();

    tray_menu_builder = tray_menu_builder.item(
        &create_check_menu_item(
            app,
            "toggle_launch_at_login",
            "Launch at Login",
            autostart,
            None,
            None,
        ),
    );

    tray_menu_builder = tray_menu_builder.separator();

    if cfg!(debug_assertions) {
        tray_menu_builder = tray_menu_builder.item(
            &create_menu_item(app, "open_devtools", "Open DevTools", "devtools",  None, None)
        );

        tray_menu_builder = tray_menu_builder.separator();
    }

    tray_menu_builder = tray_menu_builder.item(&create_menu_item(app, "about", "About", "info",  None, None));
    tray_menu_builder = tray_menu_builder.item(&create_menu_item(app, "help", "Help", "help",  None, None));
    tray_menu_builder = tray_menu_builder.item(&create_menu_item(app, "homepage", "Homepage", "site",  None, None));
    tray_menu_builder = tray_menu_builder.item(&create_menu_item(app, "check_updates", "Check for Updates", "update",  None, None));

    tray_menu_builder = tray_menu_builder.separator();
    tray_menu_builder = tray_menu_builder.item(&create_menu_item(app, "quit", "Quit SwitchShuttle", "exit",  None, None));

    tray_menu_builder.build().unwrap()
}

pub fn handle_system_tray_event(
    app: &AppHandle<Wry>,
    event: tauri::menu::MenuEvent,
    config_manager: Arc<Mutex<ConfigManager>>,
) {
    let config_path = get_config_path();

    match event.id().0.as_str() {
        "about" => {
            if let Err(e) = create_window(&app, "about", "SwitchShuttle - About", "/about", 800.0, 600.0, true) {
                eprintln!("Failed to create about window: {}", e);
            }
        }
        "help" => {
            if let Err(e) = create_window(&app, "help", "SwitchShuttle - Help", "/help", 1000.0, 800.0, true) {
                eprintln!("Failed to create help window: {}", e);
            }
        }
        "quit" => std::process::exit(0),
        "refresh_configurations" => {
            // Перезагружаем конфигурации и обновляем меню
            let mut config_manager = config_manager.lock().unwrap();
            config_manager.load_configs(Some(&app))
                .expect("Failed to reload configs");
            
            // Обновляем меню в трее
            let new_system_tray_menu = create_system_tray_menu(
                app, 
                app.autolaunch().is_enabled().unwrap_or(false), 
                &config_manager
            );
            app.set_menu(new_system_tray_menu).unwrap();
        }
        "edit_config" => open_in_default_editor(&config_path),
        "open_config_folder" => {
            open_folder_in_default_explorer(&config_path.parent().unwrap().to_path_buf())
        }
        "open_config_editor" => {
            if let Err(e) = create_window(&app, "main", "SwitchShuttle - Config Editor", "/editor", 800.0, 600.0, true) {
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
            let new_system_tray_menu =
                create_system_tray_menu(app, !enabled, &config_manager.lock().unwrap());
            app.set_menu(new_system_tray_menu).unwrap();
        }
        "homepage" => {
            let homepage_url = "https://github.com/s00d/SwitchShuttle";
            let opener = app.opener();
            opener.open_url(homepage_url, None::<&str>).unwrap();
        }
        "check_updates" => {
            if let Err(e) = create_window(&app, "check_updates", "SwitchShuttle - Update", "/update", 800.0, 600.0, true) {
                eprintln!("Failed to create check_updates window: {}", e);
            }
        }
        "open_devtools" => {
            if cfg!(debug_assertions) {
                change_devtools(app);
            }
        }
        _ => {
            if event.id().0.starts_with("edit_") {
                let config_file_name = event.id().0.replacen("edit_", "", 1);
                let config_file_path = config_path.parent().unwrap().join(&config_file_name);
                open_in_default_editor(&config_file_path);
            } else {
                let config_manager = config_manager.lock().unwrap();
                match config_manager.find_command_by_id(event.id().0.as_str()) {
                    Some((command, config)) => {
                        // Проверяем, является ли это командой мониторинга
                        if command.switch.is_some() {
                            let should_show_inputs = command.inputs.as_ref()
                                .map(|inputs| !inputs.is_empty())
                                .unwrap_or(false) && command.id.is_some();
                            
                            if should_show_inputs {
                                if let Err(e) = create_window(
                                    &app,
                                    "inputs",
                                    "SwitchShuttle - Provide Inputs",
                                    &format!("/inputs/{}", command.id.as_ref().unwrap()),
                                    400.0,
                                    300.0,
                                    true,
                                ) {
                                    eprintln!("Failed to create inputs window: {}", e);
                                }
                            } else {
                                // Выполняем команду переключения через execute_command_silent
                                if let Some(toggle_command) = &command.command {
                                    match helpers::execute_command_silent(toggle_command) {
                                        Ok(_) => {
                                            // Показываем уведомление об успешном выполнении
                                            if let Ok(_) = app.notification().builder()
                                                .title("SwitchShuttle Success")
                                                .body(&format!("Switch '{}' executed successfully", command.name))
                                                .show() {
                                                // Уведомление отправлено
                                            }
                                            // Обновляем меню после выполнения команды
                                            let new_system_tray_menu = create_system_tray_menu(
                                                app, 
                                                app.autolaunch().is_enabled().unwrap_or(false), 
                                                &config_manager
                                            );
                                            app.set_menu(new_system_tray_menu).unwrap();
                                        }
                                        Err(e) => {
                                            eprintln!("Failed to execute switch command: {}", e);
                                            // Показываем уведомление об ошибке
                                            if let Ok(_) = app.notification().builder()
                                                .title("SwitchShuttle Error")
                                                .body(&format!("Failed to execute switch '{}': {}", command.name, e))
                                                .show() {
                                                // Уведомление отправлено
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            let should_show_inputs = command.inputs.as_ref()
                                .map(|inputs| !inputs.is_empty())
                                .unwrap_or(false) && command.id.is_some();
                            
                            if should_show_inputs {
                                if let Err(e) = create_window(
                                    &app,
                                    "main",
                                    "SwitchShuttle - Provide Inputs",
                                    &format!("/inputs/{}", command.id.as_ref().unwrap()),
                                    400.0,
                                    300.0,
                                    true,
                                ) {
                                    eprintln!("Failed to create inputs window: {}", e);
                                }
                            } else {
                                helpers::execute_command(
                                    command,
                                    &config.terminal,
                                    &config.launch_in,
                                    &config.theme,
                                    &config.title,
                                );
                            }
                        }
                    }
                    None => eprintln!("Command '{}' not found", event.id().0),
                }
            }
        }
    }
}
