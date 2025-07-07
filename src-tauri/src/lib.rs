#[cfg_attr(mobile, tauri::mobile_entry_point)]
mod commands;
mod cli;
mod config;
mod console;
mod helpers;
mod menu;

use crate::commands::{
    about_message, check_for_updates, create_new_config, execute, execute_command_with_inputs,
    fetch_input_data, get_menu_data, get_version, get_configurations, delete_configuration,
    save_or_update_configuration, get_config_files, load_config, save_configuration_by_id,
    create_new_configuration, duplicate_configuration, validate_configuration, get_unique_config_title,
    open_configuration, refresh_configurations, open_config_folder
};
use crate::menu::{create_system_tray_menu, handle_system_tray_event};
use crate::cli::handle_cli_commands;
use crate::console::{init_console};
use config::ConfigManager;
use std::sync::{Arc, Mutex};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tauri_plugin_cli::CliExt;

pub fn run() {
    let config_manager = Arc::new(Mutex::new(ConfigManager::new()));
    {
        let mut config_manager = config_manager.lock().unwrap();
        config_manager
            .load_configs(None)
            .expect("Failed to load configs");
    }

    let app = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_cli::init())
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

            // Инициализируем постоянный инстанс консоли
            if let Err(e) = init_console() {
                eprintln!("Failed to initialize console: {}", e);
            }

            // Handle CLI commands
            let config_manager_clone = config_manager.clone();
            
            match app.cli().matches() {
                Ok(matches) => {
                    // Handle CLI commands and exit if any were processed
                    if handle_cli_commands(&matches, &config_manager_clone) {
                        // CLI command was handled, app will exit
                        return Ok(());
                    }
                }
                Err(_) => {
                    // No CLI arguments or error, continue with normal app startup
                }
            }

            let autostart_manager = app.autolaunch();
            let enabled = autostart_manager.is_enabled().unwrap();
            let system_tray_menu =
                create_system_tray_menu(app.handle(), enabled, &config_manager.lock().unwrap());

            let tray = app.tray_by_id("switch-shuttle-tray").unwrap();
            let config_manager_clone = config_manager.clone();

            tray.on_menu_event(move |app, event| {
                handle_system_tray_event(app, event, config_manager_clone.clone())
            });

            let config_manager_for_event = config_manager.clone();
            let app_handle = app.handle().clone();
            tray.on_tray_icon_event(move |tray, event| {
                use tauri::tray::{ TrayIconEvent};

                match event {
                    TrayIconEvent::Enter {
                        // button: MouseButton::Left,
                        // button_state: MouseButtonState::Down,
                        ..
                    } => {
                        // Обновляем меню при наведении мыши
                        let mut config_manager = config_manager_for_event.lock().unwrap();
                        let app = tray.app_handle();

                        // Перезагружаем конфигурации
                        if let Err(e) = config_manager.load_configs(Some(&app)) {
                            eprintln!("Failed to reload configs: {}", e);
                        }

                        let new_menu = create_system_tray_menu(
                            &app_handle,
                            app_handle.autolaunch().is_enabled().unwrap_or(false),
                            &config_manager
                        );

                        // Устанавливаем новое меню
                        if let Err(e) = tray.set_menu(Some(new_menu)) {
                            eprintln!("Failed to update tray menu: {}", e);
                        }
                    }
                    _ => {}
                }
            });

            tray.set_menu(Some(system_tray_menu)).unwrap();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_configurations,
            delete_configuration,
            save_or_update_configuration,
            save_configuration_by_id,
            open_configuration,
            create_new_configuration,
            duplicate_configuration,
            validate_configuration,
            get_unique_config_title,
            get_config_files,
            load_config,
            create_new_config,
            check_for_updates,
            get_version,
            execute_command_with_inputs,
            get_menu_data,
            execute,
            about_message,
            fetch_input_data,
            refresh_configurations,
            open_config_folder
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
