use auto_launch::AutoLaunchBuilder;
use tauri::utils::platform::current_exe;
use tauri::{
    CustomMenuItem, Manager, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, SystemTraySubmenu,
};

use crate::config::{CommandConfig, ConfigManager};
use crate::helpers;
use crate::helpers::{
    create_window, get_config_path, open_folder_in_default_explorer, open_in_default_editor,
};

fn create_sub_menu(items: &[CommandConfig]) -> SystemTrayMenu {
    let mut menu = SystemTrayMenu::new();
    for item in items {
        if let Some(submenu) = &item.submenu {
            let submenu_item = create_sub_menu(submenu);
            menu = menu.add_submenu(SystemTraySubmenu::new(item.name.clone(), submenu_item));
        } else {
            let id = item.id.clone().unwrap_or(item.name.clone());
            let mut menu_item = CustomMenuItem::new(id, item.name.clone());
            if let Some(hotkey) = &item.hotkey {
                menu_item = menu_item.accelerator(hotkey);
            }
            menu = menu.add_item(menu_item);
        }
    }
    menu
}

pub fn generate_command_menus(config_manager: &ConfigManager) -> SystemTrayMenu {
    let mut tray_menu = SystemTrayMenu::new();
    for config in &config_manager.configs {
        for command in &config.commands {
            if let Some(submenu) = &command.submenu {
                let submenu_item = create_sub_menu(submenu);
                tray_menu = tray_menu
                    .add_submenu(SystemTraySubmenu::new(command.name.clone(), submenu_item));
            } else {
                let id = command.id.clone().unwrap_or(command.name.clone());
                let mut item = CustomMenuItem::new(id, command.name.clone());
                if let Some(hotkey) = &command.hotkey {
                    item = item.accelerator(hotkey);
                }
                tray_menu = tray_menu.add_item(item);
            }
        }
    }
    tray_menu
}

pub fn create_system_tray_menu(autostart: bool, config_manager: &ConfigManager) -> SystemTrayMenu {
    let mut tray_menu = generate_command_menus(config_manager);

    tray_menu = tray_menu.add_native_item(SystemTrayMenuItem::Separator);

    let mut edit_config_submenu = SystemTrayMenu::new();
    edit_config_submenu = edit_config_submenu.add_item(CustomMenuItem::new(
        "add_new_config".to_string(),
        "Create New Config",
    ));
    edit_config_submenu = edit_config_submenu.add_native_item(SystemTrayMenuItem::Separator);
    for path in &config_manager.config_paths {
        let file_name = path.file_name().unwrap().to_string_lossy().to_string();
        edit_config_submenu = edit_config_submenu.add_item(CustomMenuItem::new(
            format!("edit_{}", file_name),
            file_name.clone(),
        ));
    }

    edit_config_submenu = edit_config_submenu.add_native_item(SystemTrayMenuItem::Separator);
    edit_config_submenu = edit_config_submenu.add_item(CustomMenuItem::new(
        "open_config_folder".to_string(),
        "Show Config Folder",
    ));
    edit_config_submenu = edit_config_submenu.add_item(CustomMenuItem::new(
        "open_config_editor".to_string(),
        "Open Visual Editor",
    ));

    tray_menu = tray_menu.add_submenu(SystemTraySubmenu::new("Edit Config", edit_config_submenu));

    tray_menu = tray_menu.add_native_item(SystemTrayMenuItem::Separator);

    let launch_at_login_text = if autostart {
        "✔ Launch at Login"
    } else {
        "✖ Launch at Login"
    };
    tray_menu = tray_menu.add_item(CustomMenuItem::new(
        "toggle_launch_at_login".to_string(),
        launch_at_login_text,
    ));

    tray_menu = tray_menu.add_native_item(SystemTrayMenuItem::Separator);
    tray_menu = tray_menu.add_item(CustomMenuItem::new("about".to_string(), "About"));
    tray_menu = tray_menu.add_item(CustomMenuItem::new("homepage".to_string(), "Homepage"));
    tray_menu = tray_menu.add_item(CustomMenuItem::new(
        "check_updates".to_string(),
        "Check for Updates",
    ));

    tray_menu = tray_menu.add_native_item(SystemTrayMenuItem::Separator);
    tray_menu = tray_menu.add_item(CustomMenuItem::new("quit".to_string(), "Quit"));

    tray_menu
}

pub fn handle_system_tray_event(app: &tauri::AppHandle, event: SystemTrayEvent) {
    let config_path = get_config_path();
    let app_name = &app.package_info().name;
    let current_exe = current_exe().unwrap();

    let auto_start = AutoLaunchBuilder::new()
        .set_app_name(&app_name)
        .set_app_path(&current_exe.to_str().unwrap())
        .set_use_launch_agent(true)
        .build()
        .unwrap();

    let mut config_manager = ConfigManager::new();
    config_manager
        .load_configs(Some(&app.get_window("main").unwrap()))
        .expect("Failed to reload configs");

    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            match id.as_str() {
                "quit" => std::process::exit(0),
                "edit_config" => open_in_default_editor(&config_path),
                "open_config_folder" => {
                    open_folder_in_default_explorer(&config_path.parent().unwrap().to_path_buf())
                }
                "open_config_editor" => {
                    create_window(&app, "Config Editor", "editor", 800.0, 600.0, true);
                }
                "toggle_launch_at_login" => {
                    let enabled = auto_start.is_enabled().unwrap();
                    if enabled {
                        auto_start.disable().unwrap();
                    } else {
                        auto_start.enable().unwrap();
                    }
                    let new_system_tray_menu =
                        create_system_tray_menu(!enabled, &config_manager);
                    app.tray_handle().set_menu(new_system_tray_menu).unwrap();
                }
                "about" => {
                    create_window(&app, "About", "about", 400.0, 180.0, true);
                }
                "homepage" => {
                    let homepage_url = "https://github.com/s00d/SwitchShuttle";
                    tauri::api::shell::open(&app.shell_scope(), homepage_url, None).unwrap();
                }
                "check_updates" => {
                    create_window(&app, "Update Available", "update", 400.0, 300.0, true);
                }
                "add_new_config" => {
                    create_window(&app, "Create New Config", "create", 400.0, 300.0, true);
                }
                _ => {
                    if id.starts_with("edit_") {
                        let config_file_name = id.replacen("edit_", "", 1);
                        let config_file_path = config_path.parent().unwrap().join(&config_file_name);
                        open_in_default_editor(&config_file_path);
                    } else {
                        match config_manager.find_command_by_id(id.as_str()) {
                            Some((command, config)) => {
                                if let Some(_inputs) = &command.inputs {
                                    if let Some(id) = &command.id {
                                        create_window(
                                            &app,
                                            "Provide Inputs",
                                            &format!("inputs/{}", id),
                                            400.0,
                                            300.0,
                                            true,
                                        );
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
                            None => eprintln!("Command '{}' not found", id),
                        }
                    }
                }
            }
        }
        SystemTrayEvent::LeftClick { .. } | SystemTrayEvent::RightClick { .. } => {
            // Обновляем меню при левом клике на иконку системного трея
            let autostart = auto_start.is_enabled().unwrap();
            let new_system_tray_menu = create_system_tray_menu(autostart, &config_manager);
            app.tray_handle().set_menu(new_system_tray_menu).unwrap();
        }
        _ => {}
    }


}
