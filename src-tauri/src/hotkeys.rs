use crate::config::{CommandConfig, Config};
use crate::settings::AppSettings;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tauri::AppHandle;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};

/// Структура для хранения информации о конфликте хоткеев
#[derive(Debug, Clone)]
pub struct HotkeyConflict {
    pub hotkey: String,
    pub commands: Vec<String>,
}

/// Проверяет конфликты горячих клавиш между конфигурациями
pub fn check_hotkey_conflicts(configs: &[Config]) -> Vec<HotkeyConflict> {
    let mut hotkey_map: HashMap<String, Vec<(String, String)>> = HashMap::new();
    let mut conflicts = Vec::new();

    // Собираем все горячие клавиши из всех конфигураций
    for config in configs {
        let config_name = &config.title;

        // Проверяем hotkey в командах
        check_commands_hotkeys(&config.commands, config_name, &mut hotkey_map);
    }

    // Находим конфликты
    for (hotkey, entries) in hotkey_map {
        if entries.len() > 1 {
            let mut commands = Vec::new();

            for (_config_name, command_name) in entries {
                commands.push(command_name);
            }

            conflicts.push(HotkeyConflict { hotkey, commands });
        }
    }

    conflicts
}

/// Рекурсивно проверяет горячие клавиши в командах
fn check_commands_hotkeys(
    commands: &[CommandConfig],
    config_name: &str,
    hotkey_map: &mut HashMap<String, Vec<(String, String)>>,
) {
    for command in commands {
        if let Some(hotkey) = &command.hotkey {
            if !hotkey.trim().is_empty() {
                hotkey_map
                    .entry(hotkey.clone())
                    .or_insert_with(Vec::new)
                    .push((config_name.to_string(), command.name.clone()));
            }
        }

        // Рекурсивно проверяем подменю
        if let Some(submenu) = &command.submenu {
            check_commands_hotkeys(submenu, config_name, hotkey_map);
        }
    }
}

/// Форматирует сообщение о конфликте для уведомления
pub fn format_conflict_message(conflicts: &[HotkeyConflict]) -> String {
    if conflicts.is_empty() {
        return "No hotkey conflicts found.".to_string();
    }

    let mut message = format!("Found {} hotkey conflict(s):\n", conflicts.len());

    for (i, conflict) in conflicts.iter().enumerate() {
        message.push_str(&format!("{}. {}: ", i + 1, conflict.hotkey));

        // Показываем только первые несколько команд
        let commands_to_show = std::cmp::min(conflict.commands.len(), 3);
        for (j, command) in conflict.commands.iter().take(commands_to_show).enumerate() {
            if j > 0 {
                message.push_str(", ");
            }
            message.push_str(command);
        }

        if conflict.commands.len() > commands_to_show {
            message.push_str(&format!(
                " (+{} more)",
                conflict.commands.len() - commands_to_show
            ));
        }

        message.push_str("\n");
    }

    message.push_str("\nPlease resolve conflicts by changing one of the conflicting hotkeys.");
    message
}

/// Находит команду по хоткею в структуре команд
pub fn find_command_by_hotkey<'a>(
    commands: &'a [CommandConfig],
    hotkey: &str,
) -> Option<&'a CommandConfig> {
    for command in commands {
        if let Some(cmd_hotkey) = &command.hotkey {
            if cmd_hotkey == hotkey {
                return Some(command);
            }
        }

        // Рекурсивно проверяем подменю
        if let Some(submenu) = &command.submenu {
            if let Some(found) = find_command_by_hotkey(submenu, hotkey) {
                return Some(found);
            }
        }
    }
    None
}

/// Структура для хранения зарегистрированных хоткеев
#[derive(Debug)]
pub struct HotkeyManager {
    pub(crate) registered_hotkeys: HashMap<String, Shortcut>,
    app_handle: Option<AppHandle>,
}

impl HotkeyManager {
    pub fn new() -> Self {
        Self {
            registered_hotkeys: HashMap::new(),
            app_handle: None,
        }
    }

    /// Устанавливает handle приложения
    pub fn set_app_handle(&mut self, app_handle: AppHandle) {
        self.app_handle = Some(app_handle);
    }

    /// Регистрирует хоткей для команды
    pub fn register_command_hotkey(
        &mut self,
        hotkey: &str,
        command_id: &str,
    ) -> Result<(), String> {
        println!("[Hotkeys] Registering command hotkey: {}", hotkey);

        // Проверяем, не зарегистрирован ли уже этот хоткей
        if self.registered_hotkeys.contains_key(hotkey) {
            return Err(format!(
                "Hotkey '{}' is already registered for another command",
                hotkey
            ));
        }

        let shortcut = self.parse_hotkey(hotkey)?;
        if let Some(app_handle) = &self.app_handle {
            app_handle
                .global_shortcut()
                .register(shortcut.clone())
                .map_err(|e| e.to_string())?;
            self.registered_hotkeys.insert(hotkey.to_string(), shortcut);
            println!(
                "[Hotkeys] Successfully registered command hotkey: {} for command: {}",
                hotkey, command_id
            );
        }
        Ok(())
    }

    /// Отменяет регистрацию всех хоткеев
    pub fn unregister_all(&mut self) {
        println!("[Hotkeys] Unregistering all hotkeys");
        if let Some(app_handle) = &self.app_handle {
            for (hotkey, shortcut) in &self.registered_hotkeys {
                if let Err(e) = app_handle.global_shortcut().unregister(shortcut.clone()) {
                    eprintln!("[Hotkeys] Failed to unregister hotkey {}: {}", hotkey, e);
                } else {
                    println!("[Hotkeys] Successfully unregistered hotkey: {}", hotkey);
                }
            }
        }
        self.registered_hotkeys.clear();
        println!("[Hotkeys] All hotkeys unregistered");
    }

    /// Регистрирует все хоткеи из конфигураций
    pub fn register_all_hotkeys(
        &mut self,
        configs: &[Config],
        app: &AppHandle,
        settings_state: &Arc<Mutex<AppSettings>>,
    ) -> Result<(), String> {
        println!("[Hotkeys] Starting registration of all hotkeys");
        // Сначала отменяем все существующие регистрации
        self.unregister_all();

        // Проверяем конфликты
        println!("[Hotkeys] Checking for hotkey conflicts");
        let conflicts = check_hotkey_conflicts(configs);
        if !conflicts.is_empty() {
            println!("[Hotkeys] Found {} conflicts", conflicts.len());
            let message = format_conflict_message(&conflicts);

            // Показываем уведомление об ошибке
            settings_state
                .lock()
                .unwrap()
                .show_error_notification(app, "Hotkey Conflict", &message)
                .ok();

            // Продолжаем регистрацию хоткеев без конфликтов
            println!("[Hotkeys] Continuing registration for non-conflicting hotkeys");
        } else {
            println!("[Hotkeys] No conflicts found");
        }

        for config in configs {
            // Пропускаем отключенные конфигурации
            if let Some(enabled) = config.enabled {
                if !enabled {
                    println!("[Hotkeys] Skipping disabled config: {}", config.title);
                    continue;
                }
            }

            // Регистрируем хоткеи для команд
            println!(
                "[Hotkeys] Registering command hotkeys for config: {}",
                config.title
            );
            self.register_commands_hotkeys(&config.commands, &config.title)?;
        }

        println!("[Hotkeys] All hotkeys registered successfully");
        Ok(())
    }

    /// Рекурсивно регистрирует хоткеи для команд
    fn register_commands_hotkeys(
        &mut self,
        commands: &[CommandConfig],
        config_title: &str,
    ) -> Result<(), String> {
        for command in commands {
            if let Some(hotkey) = &command.hotkey {
                if !hotkey.trim().is_empty() {
                    if let Some(id) = &command.id {
                        println!(
                            "[Hotkeys] Registering hotkey {} for command {} in {}",
                            hotkey, command.name, config_title
                        );
                        if let Err(e) = self.register_command_hotkey(hotkey, id) {
                            eprintln!(
                                "[Hotkeys] Failed to register hotkey {} for command {} in {}: {}",
                                hotkey, command.name, config_title, e
                            );
                        }
                    }
                }
            }

            // Рекурсивно обрабатываем подменю
            if let Some(submenu) = &command.submenu {
                self.register_commands_hotkeys(submenu, config_title)?;
            }
        }

        Ok(())
    }

    /// Находит команду по хоткею в зарегистрированных хоткеях
    pub fn find_command_by_hotkey<'a>(
        &self,
        hotkey_str: &str,
        configs: &'a [Config],
    ) -> Option<(&'a CommandConfig, &'a Config)> {
        // Проверяем, есть ли этот хоткей в зарегистрированных
        if !self.registered_hotkeys.contains_key(hotkey_str) {
            return None;
        }

        // Ищем команду с этим хоткеем
        for config in configs {
            if let Some(command) = find_command_by_hotkey(&config.commands, hotkey_str) {
                return Some((command, config));
            }
        }
        None
    }

    /// Парсит строку хоткея в Shortcut
    fn parse_hotkey(&self, hotkey: &str) -> Result<Shortcut, String> {
        let parts: Vec<&str> = hotkey.split('+').collect();
        let mut modifiers = Modifiers::empty();
        let mut key = None;

        for part in parts {
            let part = part.trim();
            match part.to_lowercase().as_str() {
                "ctrl" | "control" => modifiers |= Modifiers::CONTROL,
                "alt" => modifiers |= Modifiers::ALT,
                "shift" => modifiers |= Modifiers::SHIFT,
                "meta" | "cmd" | "command" => modifiers |= Modifiers::META,
                "super" => modifiers |= Modifiers::SUPER,
                _ => {
                    // Это должна быть клавиша
                    key = Some(self.parse_key(part)?);
                }
            }
        }

        if let Some(key) = key {
            Ok(Shortcut::new(Some(modifiers), key))
        } else {
            Err(format!("No key specified in hotkey: {}", hotkey))
        }
    }

    /// Парсит строку клавиши в Code
    fn parse_key(&self, key: &str) -> Result<Code, String> {
        match key.to_lowercase().as_str() {
            "a" => Ok(Code::KeyA),
            "b" => Ok(Code::KeyB),
            "c" => Ok(Code::KeyC),
            "d" => Ok(Code::KeyD),
            "e" => Ok(Code::KeyE),
            "f" => Ok(Code::KeyF),
            "g" => Ok(Code::KeyG),
            "h" => Ok(Code::KeyH),
            "i" => Ok(Code::KeyI),
            "j" => Ok(Code::KeyJ),
            "k" => Ok(Code::KeyK),
            "l" => Ok(Code::KeyL),
            "m" => Ok(Code::KeyM),
            "n" => Ok(Code::KeyN),
            "o" => Ok(Code::KeyO),
            "p" => Ok(Code::KeyP),
            "q" => Ok(Code::KeyQ),
            "r" => Ok(Code::KeyR),
            "s" => Ok(Code::KeyS),
            "t" => Ok(Code::KeyT),
            "u" => Ok(Code::KeyU),
            "v" => Ok(Code::KeyV),
            "w" => Ok(Code::KeyW),
            "x" => Ok(Code::KeyX),
            "y" => Ok(Code::KeyY),
            "z" => Ok(Code::KeyZ),
            "0" => Ok(Code::Digit0),
            "1" => Ok(Code::Digit1),
            "2" => Ok(Code::Digit2),
            "3" => Ok(Code::Digit3),
            "4" => Ok(Code::Digit4),
            "5" => Ok(Code::Digit5),
            "6" => Ok(Code::Digit6),
            "7" => Ok(Code::Digit7),
            "8" => Ok(Code::Digit8),
            "9" => Ok(Code::Digit9),
            "f1" => Ok(Code::F1),
            "f2" => Ok(Code::F2),
            "f3" => Ok(Code::F3),
            "f4" => Ok(Code::F4),
            "f5" => Ok(Code::F5),
            "f6" => Ok(Code::F6),
            "f7" => Ok(Code::F7),
            "f8" => Ok(Code::F8),
            "f9" => Ok(Code::F9),
            "f10" => Ok(Code::F10),
            "f11" => Ok(Code::F11),
            "f12" => Ok(Code::F12),
            "space" => Ok(Code::Space),
            "enter" => Ok(Code::Enter),
            "tab" => Ok(Code::Tab),
            "escape" | "esc" => Ok(Code::Escape),
            "backspace" => Ok(Code::Backspace),
            "delete" => Ok(Code::Delete),
            "insert" => Ok(Code::Insert),
            "home" => Ok(Code::Home),
            "end" => Ok(Code::End),
            "pageup" => Ok(Code::PageUp),
            "pagedown" => Ok(Code::PageDown),
            "up" => Ok(Code::ArrowUp),
            "down" => Ok(Code::ArrowDown),
            "left" => Ok(Code::ArrowLeft),
            "right" => Ok(Code::ArrowRight),
            _ => Err(format!("Unsupported key: {}", key)),
        }
    }
}

impl Default for HotkeyManager {
    fn default() -> Self {
        Self::new()
    }
}
