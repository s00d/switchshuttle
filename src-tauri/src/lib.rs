#[cfg_attr(mobile, tauri::mobile_entry_point)]
mod commands;
mod config;
mod helpers;
mod menu;

use crate::commands::{
    about_message, check_for_updates, create_new_config, cursor_pos, execute,
    execute_command_with_inputs, fetch_input_data, get_menu_data, get_version,
};
use crate::menu::{create_system_tray_menu, handle_system_tray_event};
use config::ConfigManager;
use std::sync::{Arc, Mutex};
use tauri_plugin_autostart::MacosLauncher;

pub fn run() {
    let config_manager = Arc::new(Mutex::new(ConfigManager::new()));
    {
        let mut config_manager = config_manager.lock().unwrap();
        config_manager
            .load_configs(None)
            .expect("Failed to load configs");
    }

    let app = tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        // .plugin(tauri_plugin_cli::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .manage(config_manager.clone())
        .setup(move |app| {
            #[cfg(target_os = "macos")]
            {
                app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            }

            let system_tray_menu =
                create_system_tray_menu(app.handle(), false, &config_manager.lock().unwrap());

            let tray = app.tray_by_id("switch-shuttle-tray").unwrap();

            tray.on_menu_event(move |app, event| {
                handle_system_tray_event(app, event, config_manager.clone())
            });
            tray.set_menu(Some(system_tray_menu)).unwrap();

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
            fetch_input_data,
            cursor_pos
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
