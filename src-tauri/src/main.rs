// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod config;
mod helpers;
mod hotkeys;
mod menu;

use tauri::{Manager, SystemTray};
use config::ConfigManager;

use crate::menu::{create_system_tray_menu, handle_system_tray_event};
use crate::commands::{
    about_message, check_for_updates, create_new_config, execute, execute_command_with_inputs,
    fetch_input_data, get_menu_data, get_version,
};

fn main() {
    let mut config_manager = ConfigManager::new();
    config_manager
        .load_configs(None)
        .expect("Failed to load configs");

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
        .on_system_tray_event(handle_system_tray_event)
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    hotkeys::register_global_hotkeys(app.app_handle());

    app.run(|_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    });
}
