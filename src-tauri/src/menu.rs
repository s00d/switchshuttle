use auto_launch::AutoLaunchBuilder;
use std::sync::{Arc, Mutex};
use tauri::utils::platform::current_exe;
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder, PredefinedMenuItem},
    AppHandle, Wry,
};
use tauri::menu::Submenu;
use tauri_plugin_shell::ShellExt;
use crate::config::{CommandConfig, ConfigManager};
use crate::helpers;
use crate::helpers::{
    create_window, get_config_path, open_folder_in_default_explorer, open_in_default_editor,
};

fn create_sub_menu(app: &AppHandle<Wry>, items: &[CommandConfig], title: &str) -> Submenu<Wry> {
    let mut submenu_builder = SubmenuBuilder::new(app, title);
    for item in items {
        if let Some(sub_items) = &item.submenu {
            let sub_submenu = create_sub_menu(app, sub_items, &item.name);
            submenu_builder = submenu_builder.item(&sub_submenu);
        } else {
            let id = item.id.clone().unwrap_or(item.name.clone());
            let mut menu_item = MenuItemBuilder::with_id(&id, &item.name);
            if let Some(hotkey) = &item.hotkey {
                menu_item = menu_item.accelerator(hotkey);
            }
            submenu_builder = submenu_builder.item(&menu_item.build(app).unwrap());
        }
    }
    submenu_builder.build().unwrap()
}

pub fn generate_command_menus(app: &AppHandle<Wry>, config_manager: &ConfigManager) -> Vec<Submenu<Wry>> {
    let mut submenus = Vec::new();
    for config in &config_manager.configs {
        for command in &config.commands {
            if let Some(submenu_items) = &command.submenu {
                let submenu = create_sub_menu(app, submenu_items, &command.name);
                submenus.push(submenu);
            } else {
                let id = command.id.clone().unwrap_or(command.name.clone());
                let mut item = MenuItemBuilder::with_id(&id, &command.name);
                if let Some(hotkey) = &command.hotkey {
                    item = item.accelerator(hotkey);
                }
                let menu_item = item.build(app).unwrap();
                let submenu = SubmenuBuilder::new(app, &command.name)
                    .item(&menu_item)
                    .build()
                    .unwrap();
                submenus.push(submenu);
            }
        }
    }
    submenus
}

pub fn create_system_tray_menu(app: &AppHandle<Wry>, autostart: bool, config_manager: &ConfigManager) -> tauri::menu::Menu<Wry> {
    let mut tray_menu_builder = MenuBuilder::new(app);

    let command_submenus = generate_command_menus(app, config_manager);
    for submenu in command_submenus {
        tray_menu_builder = tray_menu_builder.item(&submenu);
    }

    let mut edit_config_submenu = SubmenuBuilder::new(app, "Edit Config");
    edit_config_submenu = edit_config_submenu.item(
        &MenuItemBuilder::with_id("add_new_config", "Create New Config").build(app).unwrap()
    );
    edit_config_submenu = edit_config_submenu.item(&PredefinedMenuItem::separator(app).unwrap());
    for path in &config_manager.config_paths {
        let file_name = path.file_name().unwrap().to_string_lossy().to_string();
        edit_config_submenu = edit_config_submenu.item(
            &MenuItemBuilder::with_id(&format!("edit_{}", file_name), &file_name).build(app).unwrap()
        );
    }

    edit_config_submenu = edit_config_submenu.item(&PredefinedMenuItem::separator(app).unwrap());
    edit_config_submenu = edit_config_submenu.item(
        &MenuItemBuilder::with_id("open_config_folder", "Show Config Folder").build(app).unwrap()
    );
    edit_config_submenu = edit_config_submenu.item(
        &MenuItemBuilder::with_id("open_config_editor", "Open Visual Editor").build(app).unwrap()
    );

    tray_menu_builder = tray_menu_builder.item(&edit_config_submenu.build().unwrap());

    tray_menu_builder = tray_menu_builder.item(&PredefinedMenuItem::separator(app).unwrap());

    let launch_at_login_text = if autostart {
        "✔ Launch at Login"
    } else {
        "✖ Launch at Login"
    };
    tray_menu_builder = tray_menu_builder.item(
        &MenuItemBuilder::with_id("toggle_launch_at_login", launch_at_login_text).build(app).unwrap()
    );

    tray_menu_builder = tray_menu_builder.item(&PredefinedMenuItem::separator(app).unwrap());
    tray_menu_builder = tray_menu_builder.item(
        &MenuItemBuilder::with_id("about", "About").build(app).unwrap()
    );
    tray_menu_builder = tray_menu_builder.item(
        &MenuItemBuilder::with_id("homepage", "Homepage").build(app).unwrap()
    );
    tray_menu_builder = tray_menu_builder.item(
        &MenuItemBuilder::with_id("check_updates", "Check for Updates").build(app).unwrap()
    );

    tray_menu_builder = tray_menu_builder.item(&PredefinedMenuItem::separator(app).unwrap());
    tray_menu_builder = tray_menu_builder.item(
        &MenuItemBuilder::with_id("quit", "Quit").build(app).unwrap()
    );

    tray_menu_builder.build().unwrap()
}

pub fn handle_system_tray_event(app: &AppHandle<Wry>, event: tauri::menu::MenuEvent, config_manager: Arc<Mutex<ConfigManager>>) {
    let config_path = get_config_path();
    let app_name = &app.package_info().name;
    let current_exe = current_exe().unwrap();

    let auto_start = AutoLaunchBuilder::new()
        .set_app_name(&app_name)
        .set_app_path(&current_exe.to_str().unwrap())
        .set_use_launch_agent(true)
        .build()
        .unwrap();

    config_manager
        .lock()
        .unwrap()
        .load_configs(Some(&app))
        .expect("Failed to reload configs");

    match event.id().0.as_str() {
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
            let new_system_tray_menu = create_system_tray_menu(app, !enabled, &config_manager.lock().unwrap());
            app.set_menu(new_system_tray_menu).unwrap();
        }
        "about" => {
            create_window(&app, "About", "about", 400.0, 180.0, true);
        }
        "homepage" => {
            let homepage_url = "https://github.com/s00d/SwitchShuttle";
            app.shell().open(homepage_url, None).unwrap();
        }
        "check_updates" => {
            create_window(&app, "Update Available", "update", 400.0, 300.0, true);
        }
        "add_new_config" => {
            create_window(&app, "Create New Config", "create", 400.0, 300.0, true);
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
                    None => eprintln!("Command '{}' not found", event.id().0),
                }
            }
        }
    }
}
