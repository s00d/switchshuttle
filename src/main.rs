// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod about;
mod config;
mod helpers;

use std::fs;
use std::path::PathBuf;
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, SystemTraySubmenu};
use crate::about::{check_for_updates, show_about_dialog};
use auto_launch::*;
use tauri::utils::platform::current_exe;
use crate::config::{CommandConfig, Config};
use crate::helpers::{execute_command, get_config_path, open_folder_in_default_explorer, open_in_default_editor};

fn load_config() -> Config {
    let config_path = get_config_path();
    let config = Config::load(&config_path).unwrap_or_else(|err| {
        println!("Failed to load config: {}. Using default config.", err);
        Config::default_config()
    });
    config.validate()
}

fn load_all_configs(config_dir: &PathBuf) -> Vec<Config> {
    let mut configs = Vec::new();
    if let Ok(entries) = fs::read_dir(config_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        if let Ok(config) = Config::load(&entry.path()) {
                            configs.push(config.validate());
                        }
                    }
                }
            }
        }
    }
    configs
}

fn create_command_menu(command_config: &CommandConfig) -> SystemTrayMenu {
    let mut menu = SystemTrayMenu::new();
    if let Some(submenu) = &command_config.submenu {
        for subcommand in submenu {
            if let Some(_subsubmenu) = &subcommand.submenu {
                let submenu_item = create_command_menu(subcommand);
                menu = menu.add_submenu(SystemTraySubmenu::new(subcommand.name.clone(), submenu_item));
            } else {
                let mut item = CustomMenuItem::new(subcommand.name.clone(), subcommand.name.clone());
                if let Some(hotkey) = &subcommand.hotkey {
                    item = item.accelerator(hotkey);
                }
                menu = menu.add_item(item);
            }
        }
    }
    menu
}

fn create_system_tray_menu(autostart: bool) -> SystemTrayMenu {
    let config_path = get_config_path();
    let config_dir = config_path.parent().unwrap().to_path_buf();
    let configs = load_all_configs(&config_dir);

    let mut tray_menu = SystemTrayMenu::new();
    for config in &configs {
        let mut command_menu = SystemTrayMenu::new();

        for command in &config.commands {
            if let Some(_submenu) = &command.submenu {
                let submenu_item = create_command_menu(command);
                command_menu = command_menu.add_submenu(SystemTraySubmenu::new(command.name.clone(), submenu_item));
            } else {
                let mut item = CustomMenuItem::new(command.name.clone(), command.name.clone());
                if let Some(hotkey) = &command.hotkey {
                    item = item.accelerator(hotkey);
                }
                command_menu = command_menu.add_item(item);
            }
        }

        tray_menu = tray_menu.add_submenu(SystemTraySubmenu::new(&config.menu_title, command_menu));
    }

    tray_menu = tray_menu.add_native_item(SystemTrayMenuItem::Separator);
    tray_menu = tray_menu.add_item(CustomMenuItem::new("edit_config".to_string(), "Edit Config"));
    tray_menu = tray_menu.add_item(CustomMenuItem::new("open_config_folder".to_string(), "Open Config Folder"));

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
    let config_path = get_config_path();
    Config::ensure_default(&config_path).expect("Failed to ensure default config");

    let system_tray_menu = create_system_tray_menu(false);

    let app = tauri::Builder::default()
        .setup(|app| {
            #[cfg(target_os = "macos")]
            {
                app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            }

            Ok(())
        })
        .system_tray(SystemTray::new().with_menu(system_tray_menu))
        .on_system_tray_event(move |app, event| {
            let config_path = get_config_path();
            let config_dir = config_path.parent().unwrap().to_path_buf();
            let configs = load_all_configs(&config_dir);

            let app_name = &app.package_info().name;
            let current_exe = current_exe().unwrap();

            let auto_start = AutoLaunchBuilder::new()
                .set_app_name(&app_name)
                .set_app_path(&current_exe.to_str().unwrap())
                .set_use_launch_agent(true)
                .build()
                .unwrap();

            if let SystemTrayEvent::MenuItemClick { id, .. } = event {
                match id.as_str() {
                    "quit" => std::process::exit(0),
                    "edit_config" => open_in_default_editor(&config_path),
                    "open_config_folder" => open_folder_in_default_explorer(&config_path.parent().unwrap().to_path_buf()),
                    "toggle_launch_at_login" => {
                        let enabled = auto_start.is_enabled().unwrap();
                        if enabled {
                            auto_start.disable().unwrap();
                        } else {
                            auto_start.enable().unwrap();
                        }
                        let new_system_tray_menu = create_system_tray_menu(enabled);
                        app.tray_handle().set_menu(new_system_tray_menu).unwrap();
                    },
                    "about" => {
                        let tauri_version = app.package_info().version.to_string();
                        let about_text = format!("SwitchShuttle v{} \n\n by s00d.", tauri_version);
                        show_about_dialog(app, about_text);
                    },
                    "homepage" => {
                        let homepage_url = "https://github.com/s00d/SwitchShuttle";
                        tauri::api::shell::open(&app.shell_scope(), homepage_url, None).unwrap();
                    },
                    "check_updates" => {
                        let tauri_version = app.package_info().version.to_string();
                        check_for_updates(app, tauri_version.to_string());
                    },
                    _ => {
                        for config in &configs {
                            if let Some(command_config) = config.commands.iter().find(|c| c.name == id.as_str()) {
                                execute_command(command_config, &config.terminal, &config.launch_in, &config.theme, &config.title);
                                break;
                            }
                        }
                    }
                }

            }

            let new_system_tray_menu = create_system_tray_menu(auto_start.is_enabled().unwrap());
            app.tray_handle().set_menu(new_system_tray_menu).unwrap();
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|_app_handle, _event| {});
}



