use crate::config::{CommandConfig, ConfigManager};
use crate::helpers;
use crate::helpers::{
    change_devtools, create_window, get_config_path, open_folder_in_default_explorer,
    open_in_default_editor, create_menu_item, create_menu_item_with_hotkey,
};
use std::sync::{Arc, Mutex};
use tauri::menu::{
    CheckMenuItem, IconMenuItem, MenuBuilder, Submenu, SubmenuBuilder,
};
use tauri::{AppHandle, Manager, Wry};
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_opener::OpenerExt;
use tauri_plugin_notification::NotificationExt;

fn create_sub_menu(app: &AppHandle<Wry>, items: &[CommandConfig], title: &str) -> Submenu<Wry> {
    let mut submenu_builder = SubmenuBuilder::new(app, title);
    for item in items {
        if let Some(sub_items) = &item.submenu {
            let sub_submenu = create_sub_menu(app, sub_items, &item.name);
            submenu_builder = submenu_builder.item(&sub_submenu);
        } else {
            let id = item.id.clone().unwrap_or(item.name.clone());
            if let Some(hotkey) = &item.hotkey {
                submenu_builder = submenu_builder.item(&create_menu_item_with_hotkey(app, &id, &item.name, "terminal", hotkey));
            } else {
                submenu_builder = submenu_builder.item(&create_menu_item(app, &id, &item.name, "terminal"));
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
                let submenu = create_sub_menu(app, submenu_items, &command.name);
                menu_items.push(MenuItemOrSubmenu::Submenu(submenu));
            } else {
                let id = command.id.clone().unwrap_or(command.name.clone());
                
                // Проверяем, является ли это переключателем
                if command.switch.is_some() {
                    let is_enabled = config_manager.is_switch_enabled(&id, Some(app));
                    let menu_item = CheckMenuItem::with_id(
                        app.app_handle(),
                        &id,
                        &command.name,
                        true,
                        is_enabled,
                        None::<&str>,
                    )
                    .unwrap();
                    menu_items.push(MenuItemOrSubmenu::CheckItem(menu_item));
                } else {
                    if let Some(hotkey) = &command.hotkey {
                        let menu_item = create_menu_item_with_hotkey(app, &id, &command.name, "terminal", hotkey);
                        menu_items.push(MenuItemOrSubmenu::IconItem(menu_item));
                    } else {
                        let menu_item = create_menu_item(app, &id, &command.name, "terminal");
                        menu_items.push(MenuItemOrSubmenu::IconItem(menu_item));
                    }
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

    let mut edit_config_submenu = SubmenuBuilder::new(app, "Edit Config");

    for path in &config_manager.config_paths {
        let file_name = path.file_name().unwrap().to_string_lossy().to_string();
        edit_config_submenu = edit_config_submenu.item(
            &create_menu_item(app, &format!("edit_{}", file_name), &file_name, "edit")
        );
    }

    edit_config_submenu = edit_config_submenu.separator();
    edit_config_submenu = edit_config_submenu.item(
        &create_menu_item(app, "open_config_folder", "Show Config Folder", "folder")
    );
    edit_config_submenu = edit_config_submenu.item(
        &create_menu_item(app, "open_config_editor", "Open Visual Editor", "visual")
    );

    edit_config_submenu = edit_config_submenu.separator();
    edit_config_submenu = edit_config_submenu.item(
        &create_menu_item(app, "refresh_configurations", "Refresh Configurations", "refresh_settings")
    );

    tray_menu_builder = tray_menu_builder.item(&edit_config_submenu.build().unwrap());

    tray_menu_builder = tray_menu_builder.separator();

    tray_menu_builder = tray_menu_builder.item(
        &CheckMenuItem::with_id(
            app.app_handle(),
            "toggle_launch_at_login",
            "Launch at Login",
            true,
            autostart,
            None::<&str>,
        )
        .unwrap(),
    );

    tray_menu_builder = tray_menu_builder.separator();

    if cfg!(debug_assertions) {
        tray_menu_builder = tray_menu_builder.item(
            &create_menu_item(app, "open_devtools", "Open DevTools", "devtools")
        );

        tray_menu_builder = tray_menu_builder.separator();
    }

    tray_menu_builder = tray_menu_builder.item(&create_menu_item(app, "about", "About", "info"));
    tray_menu_builder = tray_menu_builder.item(&create_menu_item(app, "help", "Help", "help"));
    tray_menu_builder = tray_menu_builder.item(&create_menu_item(app, "homepage", "Homepage", "site"));
    tray_menu_builder = tray_menu_builder.item(&create_menu_item(app, "check_updates", "Check for Updates", "update"));

    tray_menu_builder = tray_menu_builder.separator();
    tray_menu_builder = tray_menu_builder.item(&create_menu_item(app, "quit", "Quit SwitchShuttle", "exit"));

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
            create_window(&app, "About", "/about", 800.0, 600.0, true);
        }
        "help" => {
            create_window(&app, "Help", "/help", 1000.0, 800.0, true);
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
            create_window(&app, "Config Editor", "/editor", 800.0, 600.0, true);
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
            create_window(&app, "Update Available", "/update", 800.0, 600.0, true);
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
                        // Проверяем, является ли это переключателем
                        if command.switch.is_some() {
                            let should_show_inputs = command.inputs.as_ref()
                                .map(|inputs| !inputs.is_empty())
                                .unwrap_or(false) && command.id.is_some();
                            
                            if should_show_inputs {
                                create_window(
                                    &app,
                                    "Provide Inputs",
                                    &format!("/inputs/{}", command.id.as_ref().unwrap()),
                                    400.0,
                                    300.0,
                                    true,
                                );
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
                                create_window(
                                    &app,
                                    "Provide Inputs",
                                    &format!("/inputs/{}", command.id.as_ref().unwrap()),
                                    400.0,
                                    300.0,
                                    true,
                                );
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
