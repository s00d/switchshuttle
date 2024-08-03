use global_hotkey::{hotkey::HotKey, GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};
use mouse_position::mouse_position::Mouse;
use once_cell::sync::Lazy;
use std::str::FromStr;
use std::{collections::HashMap, sync::Mutex, thread};
use tauri::{AppHandle, Manager};

use crate::config::{CommandConfig, ConfigManager};
use crate::helpers::{execute_command, show_context_menu};

static HOTKEY_COMMAND_MAP: Lazy<
    Mutex<HashMap<u32, (CommandConfig, String, String, String, String)>>,
> = Lazy::new(|| Mutex::new(HashMap::new()));

pub fn register_global_hotkeys(app_handle: AppHandle) {
    thread::spawn(move || {
        let manager = GlobalHotKeyManager::new().unwrap();
        register_hotkeys(&app_handle, &manager);
        handle_hotkey_events(app_handle);
    });
}

fn register_hotkeys(app_handle: &AppHandle, manager: &GlobalHotKeyManager) {
    let mut config_manager = ConfigManager::new();
    config_manager
        .load_configs(Some(&app_handle.get_window("main").unwrap()))
        .expect("Failed to load configs");

    let mut unique_hotkeys = HashMap::new();

    for config in &config_manager.configs {
        if let Some(menu_hotkey) = &config.menu_hotkey {
            if let Ok(hk) = HotKey::from_str(&menu_hotkey) {
                if !unique_hotkeys.contains_key(&hk) {
                    manager.register(hk).unwrap();
                    HOTKEY_COMMAND_MAP.lock().unwrap().insert(
                        hk.id(),
                        (
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
                        ),
                    );
                    unique_hotkeys.insert(hk, true);
                }
            }
        }

        for command in &config.commands {
            if let Some(hotkey) = &command.hotkey {
                if let Ok(hk) = HotKey::from_str(hotkey) {
                    if !unique_hotkeys.contains_key(&hk) {
                        manager.register(hk).unwrap();
                        HOTKEY_COMMAND_MAP.lock().unwrap().insert(
                            hk.id(),
                            (
                                command.clone(),
                                config.terminal.clone(),
                                config.launch_in.clone(),
                                config.theme.clone(),
                                config.title.clone(),
                            ),
                        );
                        unique_hotkeys.insert(hk, true);
                    }
                }
            }
        }
    }
}

fn handle_hotkey_events(app_handle: AppHandle) {
    loop {
        if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
            if event.state == HotKeyState::Released {
                if let Some((command, terminal, launch_in, theme, title)) =
                    HOTKEY_COMMAND_MAP.lock().unwrap().get(&event.id)
                {
                    if command.name == "show_context_menu" {
                        let position = Mouse::get_mouse_position();
                        if let Mouse::Position { x, y } = position {
                            let _ = show_context_menu(app_handle.get_window("main").unwrap(), x, y);
                        }
                    } else {
                        execute_command(command, terminal, launch_in, theme, title);
                    }
                }
            }
        }
    }
}
