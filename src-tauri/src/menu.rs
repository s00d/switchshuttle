use crate::config::{CommandConfig, ConfigManager};
use crate::helpers;
use crate::helpers::{
    create_window, get_config_path, open_folder_in_default_explorer, open_in_default_editor,
};
use std::sync::{Arc, Mutex};
use tauri::image::Image;
use tauri::menu::{
    CheckMenuItem, IconMenuItem, IconMenuItemBuilder, MenuBuilder, Submenu, SubmenuBuilder,
};
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Manager, Wry};
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_shell::ShellExt;

fn create_sub_menu(app: &AppHandle<Wry>, items: &[CommandConfig], title: &str) -> Submenu<Wry> {
    let mut submenu_builder = SubmenuBuilder::new(app, title);
    for item in items {
        if let Some(sub_items) = &item.submenu {
            let sub_submenu = create_sub_menu(app, sub_items, &item.name);
            submenu_builder = submenu_builder.item(&sub_submenu);
        } else {
            let id = item.id.clone().unwrap_or(item.name.clone());
            let icon_path = app
                .path()
                .resolve("icons/terminal.png", BaseDirectory::Resource)
                .unwrap();
            let mut menu_item = IconMenuItemBuilder::with_id(&id, &item.name)
                .icon(Image::from_path(icon_path).unwrap());
            if let Some(hotkey) = &item.hotkey {
                menu_item = menu_item.accelerator(hotkey);
            }
            submenu_builder = submenu_builder.item(&menu_item.build(app).unwrap());
        }
    }
    submenu_builder.build().unwrap()
}

enum MenuItemOrSubmenu {
    // MenuItem(MenuItem<Wry>),
    Submenu(Submenu<Wry>),
    IconItem(IconMenuItem<Wry>),
}

pub fn create_system_tray_menu(
    app: &AppHandle<Wry>,
    autostart: bool,
    config_manager: &ConfigManager,
) -> tauri::menu::Menu<Wry> {
    let mut tray_menu_builder = MenuBuilder::new(app);

    let mut menu_items = Vec::new();

    for config in &config_manager.configs {
        for command in &config.commands {
            if let Some(submenu_items) = &command.submenu {
                let submenu = create_sub_menu(app, submenu_items, &command.name);
                menu_items.push(MenuItemOrSubmenu::Submenu(submenu));
            } else {
                let id = command.id.clone().unwrap_or(command.name.clone());
                let icon_path = app
                    .path()
                    .resolve("icons/terminal.png", BaseDirectory::Resource)
                    .unwrap();
                let mut item = IconMenuItemBuilder::with_id(&id, &command.name)
                    .icon(Image::from_path(icon_path).unwrap());
                if let Some(hotkey) = &command.hotkey {
                    item = item.accelerator(hotkey);
                }
                let menu_item = item.build(app).unwrap();
                menu_items.push(MenuItemOrSubmenu::IconItem(menu_item));
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
        }
    }

    tray_menu_builder = tray_menu_builder.separator();

    let mut edit_config_submenu = SubmenuBuilder::new(app, "Edit Config");

    let icon_path = app
        .path()
        .resolve("icons/create.png", BaseDirectory::Resource)
        .unwrap();
    edit_config_submenu = edit_config_submenu.item(
        &IconMenuItemBuilder::with_id("add_new_config", "Create New Config")
            .icon(Image::from_path(icon_path).unwrap())
            .build(app)
            .unwrap(),
    );
    edit_config_submenu = edit_config_submenu.separator();
    for path in &config_manager.config_paths {
        let file_name = path.file_name().unwrap().to_string_lossy().to_string();
        let icon_path = app
            .path()
            .resolve("icons/edit.png", BaseDirectory::Resource)
            .unwrap();
        edit_config_submenu = edit_config_submenu.item(
            &IconMenuItemBuilder::with_id(&format!("edit_{}", file_name), &file_name)
                .icon(Image::from_path(icon_path).unwrap())
                .build(app)
                .unwrap(),
        );
    }

    edit_config_submenu = edit_config_submenu.separator();
    let icon_path = app
        .path()
        .resolve("icons/folder.png", BaseDirectory::Resource)
        .unwrap();
    edit_config_submenu = edit_config_submenu.item(
        &IconMenuItemBuilder::with_id("open_config_folder", "Show Config Folder")
            .icon(Image::from_path(icon_path).unwrap())
            .build(app)
            .unwrap(),
    );
    let icon_path = app
        .path()
        .resolve("icons/visual.png", BaseDirectory::Resource)
        .unwrap();
    edit_config_submenu = edit_config_submenu.item(
        &IconMenuItemBuilder::with_id("open_config_editor", "Open Visual Editor")
            .icon(Image::from_path(icon_path).unwrap())
            .build(app)
            .unwrap(),
    );

    edit_config_submenu = edit_config_submenu.separator();

    let icon_path = app
        .path()
        .resolve("icons/refresh_settings.png", BaseDirectory::Resource)
        .unwrap();
    edit_config_submenu = edit_config_submenu.item(
        &IconMenuItemBuilder::with_id("reload", "Reload App")
            .icon(Image::from_path(icon_path).unwrap())
            .build(app)
            .unwrap(),
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
        let icon_path = app
            .path()
            .resolve("icons/devtools.png", BaseDirectory::Resource)
            .unwrap();
        tray_menu_builder = tray_menu_builder.item(
            &IconMenuItemBuilder::with_id("open_devtools", "Open DevTools")
                .icon(Image::from_path(icon_path).unwrap())
                .build(app)
                .unwrap(),
        );

        tray_menu_builder = tray_menu_builder.separator();
    }

    let icon_path = app
        .path()
        .resolve("icons/info.png", BaseDirectory::Resource)
        .unwrap();
    tray_menu_builder = tray_menu_builder.item(
        &IconMenuItemBuilder::with_id("about", "About")
            .icon(Image::from_path(icon_path).unwrap())
            .build(app)
            .unwrap(),
    );
    let icon_path = app
        .path()
        .resolve("icons/site.png", BaseDirectory::Resource)
        .unwrap();
    tray_menu_builder = tray_menu_builder.item(
        &IconMenuItemBuilder::with_id("homepage", "Homepage")
            .icon(Image::from_path(icon_path).unwrap())
            .build(app)
            .unwrap(),
    );
    let icon_path = app
        .path()
        .resolve("icons/update.png", BaseDirectory::Resource)
        .unwrap();
    tray_menu_builder = tray_menu_builder.item(
        &IconMenuItemBuilder::with_id("check_updates", "Check for Updates")
            .icon(Image::from_path(icon_path).unwrap())
            .build(app)
            .unwrap(),
    );

    tray_menu_builder = tray_menu_builder.separator();
    let icon_path = app
        .path()
        .resolve("icons/exit.png", BaseDirectory::Resource)
        .unwrap();
    tray_menu_builder = tray_menu_builder.item(
        &IconMenuItemBuilder::with_id("quit", "Quit SwitchShuttle")
            .icon(Image::from_path(icon_path).unwrap())
            .build(app)
            .unwrap(),
    );

    tray_menu_builder.build().unwrap()
}

pub fn handle_system_tray_event(
    app: &AppHandle<Wry>,
    event: tauri::menu::MenuEvent,
    config_manager: Arc<Mutex<ConfigManager>>,
) {
    let config_path = get_config_path();

    config_manager
        .lock()
        .unwrap()
        .load_configs(Some(&app))
        .expect("Failed to reload configs");

    match event.id().0.as_str() {
        "about" => {
            create_window(&app, "About", "about", 400.0, 580.0, true);
        }
        "quit" => std::process::exit(0),
        "reload" => app.restart(),
        "edit_config" => open_in_default_editor(&config_path),
        "open_config_folder" => {
            open_folder_in_default_explorer(&config_path.parent().unwrap().to_path_buf())
        }
        "open_config_editor" => {
            create_window(&app, "Config Editor", "editor", 800.0, 600.0, true);
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
            app.shell().open(homepage_url, None).unwrap();
        }
        "check_updates" => {
            create_window(&app, "Update Available", "update", 400.0, 300.0, true);
        }
        "add_new_config" => {
            create_window(&app, "Create New Config", "create", 400.0, 300.0, true);
        }
        "open_devtools" => {
            let window = app.get_webview_window("main").unwrap();
            if !window.is_devtools_open() {
                window.open_devtools();
            } else {
                window.close_devtools();
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
