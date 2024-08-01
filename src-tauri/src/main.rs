// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod helpers;
mod commands;

use std::{collections::HashMap, sync::Mutex, thread};
use std::str::FromStr;
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, SystemTraySubmenu};
use auto_launch::*;
use tauri::utils::platform::current_exe;
use global_hotkey::{GlobalHotKeyManager, GlobalHotKeyEvent, hotkey::{HotKey}, HotKeyState};
use mouse_position::mouse_position::Mouse;
use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::config::{CommandConfig, ConfigManager};
use crate::helpers::{create_window, execute_command, get_config_path, open_folder_in_default_explorer, open_in_default_editor};
use crate::commands::{check_for_updates, create_new_config, execute, execute_command_with_inputs, about_message, get_menu_data, get_version, show_context_menu, fetch_input_data};


#[derive(Deserialize)]
struct GitHubRelease {
    tag_name: String,
    html_url: String,
}

static HOTKEY_COMMAND_MAP: Lazy<Mutex<HashMap<u32, (CommandConfig, String, String, String, String)>>> = Lazy::new(|| Mutex::new(HashMap::new()));

fn create_command_menu(command_config: &CommandConfig) -> SystemTrayMenu {
    let mut menu = SystemTrayMenu::new();
    if let Some(submenu) = &command_config.submenu {
        for subcommand in submenu {
            if let Some(_subsubmenu) = &subcommand.submenu {
                let submenu_item = create_command_menu(subcommand);
                menu = menu.add_submenu(SystemTraySubmenu::new(subcommand.name.clone(), submenu_item));
            } else {
                let id = subcommand.id.clone().unwrap_or(subcommand.name.clone());
                let mut item = CustomMenuItem::new(id, subcommand.name.clone());
                if let Some(hotkey) = &subcommand.hotkey {
                    item = item.accelerator(hotkey);
                }
                menu = menu.add_item(item);
            }
        }
    }
    menu
}

fn create_system_tray_menu(autostart: bool, config_manager: &ConfigManager) -> SystemTrayMenu {
    let mut tray_menu = SystemTrayMenu::new();
    for config in &config_manager.configs {
        for command in &config.commands {
            if let Some(_submenu) = &command.submenu {
                let submenu_item = create_command_menu(command);
                tray_menu = tray_menu.add_submenu(SystemTraySubmenu::new(command.name.clone(), submenu_item));
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

    tray_menu = tray_menu.add_native_item(SystemTrayMenuItem::Separator);

    let mut edit_config_submenu = SystemTrayMenu::new();
    edit_config_submenu = edit_config_submenu.add_item(CustomMenuItem::new("add_new_config".to_string(), "Create New Config"));
    edit_config_submenu = edit_config_submenu.add_native_item(SystemTrayMenuItem::Separator);
    for path in &config_manager.config_paths {
        let file_name = path.file_name().unwrap().to_string_lossy().to_string();
        edit_config_submenu = edit_config_submenu.add_item(CustomMenuItem::new(format!("edit_{}", file_name), file_name.clone()));
    }

    edit_config_submenu = edit_config_submenu.add_native_item(SystemTrayMenuItem::Separator);
    edit_config_submenu = edit_config_submenu.add_item(CustomMenuItem::new("open_config_folder".to_string(), "Show Config Folder"));
    edit_config_submenu = edit_config_submenu.add_item(CustomMenuItem::new("open_config_editor".to_string(), "Open Visual Editor"));

    tray_menu = tray_menu.add_submenu(SystemTraySubmenu::new("Edit Config", edit_config_submenu));

    tray_menu = tray_menu.add_native_item(SystemTrayMenuItem::Separator);

    let launch_at_login_text = if autostart {
        "✔ Launch at Login"
    } else {
        "✖ Launch at Login"
    };
    tray_menu = tray_menu.add_item(CustomMenuItem::new("toggle_launch_at_login".to_string(), launch_at_login_text));

    tray_menu = tray_menu.add_native_item(SystemTrayMenuItem::Separator);
    tray_menu = tray_menu.add_item(CustomMenuItem::new("about".to_string(), "About"));
    tray_menu = tray_menu.add_item(CustomMenuItem::new("homepage".to_string(), "Homepage"));
    tray_menu = tray_menu.add_item(CustomMenuItem::new("check_updates".to_string(), "Check for Updates"));

    tray_menu = tray_menu.add_native_item(SystemTrayMenuItem::Separator);
    tray_menu = tray_menu.add_item(CustomMenuItem::new("quit".to_string(), "Quit"));

    tray_menu
}

fn main() {
    let mut config_manager = ConfigManager::new();
    config_manager.load_configs(None).expect("Failed to load configs");

    let system_tray_menu = create_system_tray_menu(false, &config_manager);

    let app = tauri::Builder::default()
        .setup(move |app| {
            #[cfg(target_os = "macos")]
            {
                app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            }


            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create_new_config,
            about_message,
            check_for_updates,
            get_version,
            execute_command_with_inputs,
            get_menu_data,
            execute,
            fetch_input_data
        ])
        .system_tray(SystemTray::new().with_menu(system_tray_menu))
        .plugin(tauri_plugin_context_menu::init())
        .on_system_tray_event(move |app, event| {
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
            config_manager.load_configs(Some(&app.get_window("main").unwrap())).expect("Failed to reload configs");
            let new_system_tray_menu = create_system_tray_menu(auto_start.is_enabled().unwrap(), &config_manager);
            app.tray_handle().set_menu(new_system_tray_menu).unwrap();

            if let SystemTrayEvent::MenuItemClick { id, .. } = event {
                match id.as_str() {
                    "quit" => std::process::exit(0),
                    "edit_config" => open_in_default_editor(&config_path),
                    "open_config_folder" => open_folder_in_default_explorer(&config_path.parent().unwrap().to_path_buf()),
                    "open_config_editor" => {
                        create_window(&app, "config_editor", "Config Editor", "editor", 800.0, 600.0, true);
                    },
                    "toggle_launch_at_login" => {
                        let enabled = auto_start.is_enabled().unwrap();
                        if enabled {
                            auto_start.disable().unwrap();
                        } else {
                            auto_start.enable().unwrap();
                        }
                        let new_system_tray_menu = create_system_tray_menu(enabled, &config_manager);
                        app.tray_handle().set_menu(new_system_tray_menu).unwrap();
                    },
                    "about" => {
                        create_window(&app, "main", "About", "about", 400.0, 180.0, true);
                    },
                    "homepage" => {
                        let homepage_url = "https://github.com/s00d/SwitchShuttle";
                        tauri::api::shell::open(&app.shell_scope(), homepage_url, None).unwrap();
                    },
                    "check_updates" => {
                        create_window(&app, "update", "Update Available", "update", 400.0, 300.0, true);
                    },
                    "add_new_config" => {
                        create_window(&app, "new_config_window", "Create New Config", "create", 400.0, 300.0, true);
                    },
                    _ => {
                        if id.starts_with("edit_") {
                            println!("ID starts with 'edit_': {}", id);

                            let config_file_name = id.replacen("edit_", "", 1); // Remove "edit_" prefix
                            println!("Config file name after removing 'edit_' prefix: {}", config_file_name);

                            let config_file_path = config_path.parent().unwrap().join(&config_file_name);
                            println!("Full config file path: {:?}", config_file_path);

                            open_in_default_editor(&config_file_path);
                        } else {
                            println!("Processing configs for ID: {}", id);

                            match config_manager.find_command_by_id(id.as_str()) {
                                Some((command, config)) => {
                                    if let Some(inputs) = &command.inputs {
                                        if let Some(id) = &command.id {
                                            let window = create_window(&app, "input_window", "Provide Inputs", &format!("inputs/{}", id), 400.0, 300.0, true);
                                            window.emit("input_data", (id.clone(), inputs.clone())).unwrap();
                                        }

                                    } else {
                                        execute_command(command, &config.terminal, &config.launch_in, &config.theme, &config.title);
                                    }

                                },
                                None => eprintln!("Command '{}' not found", id),
                            }
                        }
                    }
                }

            }
        })
        .on_window_event(|event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
                event.window().hide().unwrap();
                api.prevent_close();
            }
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application");


    let app_handle = app.app_handle();

    thread::spawn(move || {
        let app_handle = app_handle.clone();
        // Регистрация глобальных горячих клавиш в основном потоке
        let manager = GlobalHotKeyManager::new().unwrap();

        // Регистрация глобальных горячих клавиш
        let mut config_manager = ConfigManager::new();
        config_manager.load_configs(Some(&app_handle.get_window("main").unwrap())).expect("Failed to load configs");

        let mut hotkey = false;

        for config in &config_manager.configs {
            for command in &config.commands {
                if let Some(hotkey) = &command.hotkey {

                    let hotkey = match HotKey::from_str(hotkey) {
                        Ok(hk) => hk,
                        Err(err) => {
                            eprintln!("Failed to parse hotkey {}: {}", hotkey, err);
                            continue;
                        }
                    };

                    manager.register(hotkey).unwrap();

                    // Сохранение команды и конфигурации для вызова по горячей клавише
                    HOTKEY_COMMAND_MAP.lock().unwrap().insert(hotkey.id(), (
                        command.clone(),
                        config.terminal.clone(),
                        config.launch_in.clone(),
                        config.theme.clone(),
                        config.title.clone()
                    ));
                }
            }

            if let Some(menu_hotkey) = &config.menu_hotkey {
                if hotkey {
                    continue;
                }
                hotkey = true;
                let hotkey = match HotKey::from_str(&menu_hotkey) {
                    Ok(hk) => hk,
                    Err(err) => {
                        eprintln!("Failed to parse hotkey {}: {}", menu_hotkey, err);
                        continue;
                    }
                };
                manager.register(hotkey).unwrap();

                HOTKEY_COMMAND_MAP.lock().unwrap().insert(hotkey.id(), (
                    CommandConfig {
                        id: None,
                        name: "show_context_menu".to_string(),
                        command: None,
                        submenu: None,
                        hotkey: Some(menu_hotkey.clone()),
                        commands: None,
                        inputs: None,
                    },
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                ));
            }
        }
        loop {
            match GlobalHotKeyEvent::receiver().try_recv() {
                Ok(event) => {
                    println!("{:?}", event);
                    if event.state == HotKeyState::Released {
                        if let Some((command, terminal, launch_in, theme, title)) = HOTKEY_COMMAND_MAP.lock().unwrap().get(&event.id) {
                            if command.name == "show_context_menu" {
                                // Получение позиции курсора

                                let position = Mouse::get_mouse_position();
                                match position {
                                    Mouse::Position { x, y } => {
                                        let app_handle = app_handle.clone();
                                        let _ = show_context_menu(app_handle.get_window("main").unwrap(), x, y);
                                    },
                                    Mouse::Error => println!("Error getting mouse position"),
                                }
                            } else {
                                execute_command(command, terminal, launch_in, theme, title);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    });

    app.run(|_app_handle, _event| {});
}
