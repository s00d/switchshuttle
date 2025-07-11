mod cli;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
mod commands;
mod config;
mod console;
mod execute;
mod helpers;
mod hotkeys;
mod menu;
mod menu_structure;
mod settings;

use crate::cli::handle_cli_commands;
use crate::commands::{
    check_for_updates, create_new_config, create_new_configuration,
    delete_configuration, duplicate_configuration, execute, execute_command_with_inputs,
    fetch_input_data, get_config_files, get_configurations, get_settings,
    get_settings_schema, get_terminals_list, get_unique_config_title, get_version, load_config, open_config_folder,
    open_configuration, refresh_configurations, save_configuration_by_id,
    save_or_update_configuration, save_settings, show_notification, validate_configuration,
};
use crate::console::{ConsolePool};
use crate::hotkeys::{HotkeyManager};
use crate::menu::{create_system_tray_menu, handle_system_tray_event, execute_command_by_id};
use crate::settings::AppSettings;
use config::ConfigManager;
use std::sync::{Arc, Mutex};
use tauri::{Manager};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tauri_plugin_cli::CliExt;
use tauri_plugin_global_shortcut::ShortcutState;

pub fn run() {
    let config_manager = Arc::new(Mutex::new(ConfigManager::new()));
    {
        let mut config_manager = config_manager.lock().unwrap();
        config_manager
            .load_configs(None)
            .expect("Failed to load configs");
    }

    // Загружаем настройки в состояние
    let settings = match AppSettings::load() {
        Ok(settings) => Arc::new(Mutex::new(settings)),
        Err(e) => {
            eprintln!("Failed to load settings: {}", e);
            Arc::new(Mutex::new(AppSettings::default()))
        }
    };

    // Создаем менеджер хоткеев
    let hotkey_manager = Arc::new(Mutex::new(HotkeyManager::new()));

    let app = tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_global_shortcut::Builder::new()
            .with_handler(move |app, shortcut, event| {
                println!("[Hotkeys] Global shortcut event: {:?}, state: {:?}", shortcut, event.state());
                
                if event.state() == ShortcutState::Released {
                    // Получаем зарегистрированные хоткеи
                    let hotkey_manager = app.state::<Arc<Mutex<HotkeyManager>>>();
                    let config_manager = app.state::<Arc<Mutex<ConfigManager>>>();
                    let hotkey_manager = hotkey_manager.lock().unwrap();
                    let config_manager = config_manager.lock().unwrap();
                    
                    // Ищем строку хоткея по shortcut
                    let hotkey_str = hotkey_manager.registered_hotkeys.iter()
                        .find(|(_, registered_shortcut)| *registered_shortcut == shortcut)
                        .map(|(key, _)| key.as_str());
                    
                    if let Some(hotkey_str) = hotkey_str {
                        // Ищем команду по хоткею
                        if let Some((command, _config)) = hotkey_manager.find_command_by_hotkey(hotkey_str, &config_manager.configs) {
                            if let Some(id) = &command.id {
                                println!("[Hotkeys] Executing command: {} (id: {})", command.name, id);
                                
                                // Выполняем команду напрямую через единую функцию
                                if let Err(e) = execute_command_by_id(&app, id, &config_manager) {
                                    eprintln!("[Hotkeys] Failed to execute command: {}", e);
                                } else {
                                    println!("[Hotkeys] Successfully executed command: {}", id);
                                }
                            } else {
                                eprintln!("[Hotkeys] Command has no ID: {}", command.name);
                            }
                        }
                    }
                }
            })
            .build())
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
        .manage(settings.clone())
        .manage(hotkey_manager.clone())
        .setup(move |app| {
            #[cfg(target_os = "macos")]
            {
                app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            }

            // Инициализируем постоянный инстанс консоли
            if let Err(e) = ConsolePool::init_console() {
                eprintln!("Failed to initialize console: {}", e);
            }

            // Загружаем и применяем настройки при запуске
            if let Ok(settings) = AppSettings::load() {
                if let Err(e) = settings.apply(&app.handle()) {
                    eprintln!("Failed to apply settings: {}", e);
                }
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

            // Добавляем обработчики событий мыши для управления таймерами
            tray.on_tray_icon_event(move |_tray, event| {
                use tauri::tray::{MouseButton, MouseButtonState, TrayIconEvent};
                match event {
                    TrayIconEvent::Enter { .. } => {
                        eprintln!("[Tray] Mouse entered tray icon - resuming timers");
                        crate::menu::resume_monitor_timers();
                    }
                    TrayIconEvent::Leave { .. } => {
                        eprintln!("[Tray] Mouse left tray icon - pausing timers");
                        crate::menu::pause_monitor_timers();
                    }
                    TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } => {
                        eprintln!("[Tray] Mouse click left tray icon - pausing timers");
                        // in this example, let's show and focus the main window when the tray is clicked
                        crate::menu::resume_monitor_timers();
                    }
                    _ => {
                        // Игнорируем другие события
                    }
                }
            });

            tray.set_menu(Some(system_tray_menu)).unwrap();

            // Инициализируем глобальные хоткеи
            let hotkey_manager_clone = hotkey_manager.clone();
            let config_manager_clone = config_manager.clone();
            let settings_clone = settings.clone();
            
            // ПЕРВИЧНАЯ РЕГИСТРАЦИЯ ГЛОБАЛЬНЫХ ХОТКЕЕВ ПРИ ЗАПУСКЕ
            println!("[Hotkeys] PRIMARY registration of all hotkeys at app startup");
            {
                let mut hotkey_manager = hotkey_manager_clone.lock().unwrap();
                hotkey_manager.set_app_handle(app.handle().clone());
                // Регистрируем все хоткеи из конфигураций
                let configs = config_manager_clone.lock().unwrap().configs.clone();
                if let Err(e) = hotkey_manager.register_all_hotkeys(&configs, &app.handle(), &settings_clone) {
                    eprintln!("Failed to register hotkeys: {}", e);
                }
            }

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
            get_terminals_list,
            execute_command_with_inputs,
            execute,
            fetch_input_data,
            refresh_configurations,
            open_config_folder,
            get_settings_schema,
            get_settings,
            save_settings,
            show_notification
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
