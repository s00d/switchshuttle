#[cfg_attr(mobile, tauri::mobile_entry_point)]

mod commands;
mod config;
mod helpers;
mod menu;

use std::str::FromStr;
use config::ConfigManager;
use std::sync::{Arc, Mutex};
use tauri::{Manager};
use tauri_plugin_autostart::MacosLauncher;

use crate::commands::{
    about_message, check_for_updates, create_new_config, execute, execute_command_with_inputs,
    fetch_input_data, get_menu_data, get_version,
};
use crate::helpers::execute_command;
use crate::menu::{create_system_tray_menu, handle_system_tray_event};


pub fn run() {
    let config_manager = Arc::new(Mutex::new(ConfigManager::new()));
    {
        let mut config_manager = config_manager.lock().unwrap();
        config_manager
            .load_configs(None)
            .expect("Failed to load configs");
    }

    let app = tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        // .plugin(tauri_plugin_cli::init())
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, None))
        .manage(config_manager.clone())
        .setup(move |app| {
            #[cfg(target_os = "macos")]
            {
                app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            }

            let system_tray_menu = create_system_tray_menu(app.handle(), false, &config_manager.lock().unwrap());


            let tray = tauri::tray::TrayIconBuilder::with_id("switch-shuttle-tray")
                .menu(&system_tray_menu)
                .on_menu_event(move |app, event| handle_system_tray_event(app, event, config_manager.clone()))
                .on_tray_icon_event(|tray, event| {
                    if let tauri::tray::TrayIconEvent::Click {
                        button: tauri::tray::MouseButton::Left,
                        button_state: tauri::tray::MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let window = tray.app_handle().get_window("main").unwrap();
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                })
                .build(app)?;


            #[cfg(desktop)]
            {
                use tauri_plugin_global_shortcut::{ShortcutState};
                use global_hotkey::hotkey::HotKey;

                let app_handle = app.handle();
                app_handle.plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_handler(move |app, shortcut, event| {
                            if event.state == ShortcutState::Pressed {
                                let mut config_manager = ConfigManager::new();
                                config_manager
                                    .load_configs(Some(app))
                                    .expect("Failed to load configs");

                                let mut unique_hotkeys = std::collections::HashMap::new();
                                for config in &config_manager.configs {
                                    for command in &config.commands {
                                        if let Some(hotkey_str) = &command.hotkey {
                                            if let Ok(hotkey) = HotKey::from_str(hotkey_str) {
                                                if !unique_hotkeys.contains_key(&hotkey.id()) {
                                                    unique_hotkeys.insert(hotkey.id(), true);
                                                    if shortcut.matches(hotkey.mods, hotkey.key) {
                                                        // Execute command
                                                        execute_command(
                                                            &command,
                                                            &config.terminal,
                                                            &config.launch_in,
                                                            &config.theme,
                                                            &config.title,
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        })
                        .build(),
                )?;
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
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    });
}
