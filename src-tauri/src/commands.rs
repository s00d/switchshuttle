use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tauri::{State};

use crate::config::{CommandConfig, Config, ConfigManager};
use crate::helpers::{execute_command, get_config_path, open_in_default_editor};

#[derive(Deserialize)]
struct GitHubRelease {
    tag_name: String,
    html_url: String,
}

#[tauri::command]
pub fn create_new_config(file_name: String) -> Result<(), String> {
    if file_name.ends_with(".json") {
        return Err("File name should not include the .json extension".into());
    }

    let config_path = get_config_path();
    let config_dir = config_path.parent().unwrap().to_path_buf();

    let new_config_path = config_dir.join(format!("{}.json", file_name));

    if new_config_path.exists() {
        return Err("File already exists".into());
    }

    let mut new_config = Config::default_config();
    if let Some(main_command) = new_config.commands.get_mut(0) {
        main_command.name = file_name.clone();
    }
    new_config
        .save(&new_config_path)
        .map_err(|e| e.to_string())?;
    open_in_default_editor(&new_config_path);

    Ok(())
}

#[tauri::command]
pub fn check_for_updates(app: tauri::AppHandle) -> Result<(String, String), String> {
    let current_version = app.package_info().version.to_string();
    let latest_release_url = "https://api.github.com/repos/s00d/switchshuttle/releases/latest";

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_str("switchshuttle").unwrap());

    let client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| e.to_string())?;

    let response = client
        .get(latest_release_url)
        .send()
        .map_err(|e| e.to_string())?;

    let latest_release: GitHubRelease = response.json().map_err(|e| e.to_string())?;

    let version = latest_release.tag_name.replace("app-v", "");
    if version != current_version {
        let update_message = format!(
            "A new version (v{}) is available! You are currently using v{}.",
            version, current_version
        );
        Ok((update_message, latest_release.html_url))
    } else {
        Err("You are using the latest version.".to_string())
    }
}

#[tauri::command]
pub fn get_version(app: tauri::AppHandle) -> String {
    app.package_info().version.to_string()
}

#[tauri::command]
pub fn execute_command_with_inputs(
    state: State<'_, Arc<Mutex<ConfigManager>>>,
    inputs: HashMap<String, String>,
    command: String,
) -> Result<String, String> {
    println!("execute_command_with_inputs {:?} {:?}", inputs, command);

    let config_manager = state.lock().unwrap();

    let (command, config) = match config_manager.find_command_by_id(&command) {
        Some((cmd, cfg)) => (cmd, cfg),
        None => return Err("Command not found".to_string()),
    };

    let mut cmd = command.command.clone();
    let mut cmds = command.commands.clone();

    if let Some(ref mut cmd) = cmd {
        for (key, value) in &inputs {
            *cmd = cmd.replace(&format!("[{}]", key), value);
        }
    }

    if let Some(ref mut cmds) = cmds {
        for cmd in cmds {
            for (key, value) in &inputs {
                *cmd = cmd.replace(&format!("[{}]", key), value);
            }
        }
    }

    let updated_command = CommandConfig {
        id: command.id.clone(),
        name: command.name.clone(),
        inputs: command.inputs.clone(),
        command: cmd,
        commands: cmds,
        submenu: command.submenu.clone(),
        hotkey: command.hotkey.clone(),
    };

    execute_command(
        &updated_command,
        &config.terminal,
        &config.launch_in,
        &config.theme,
        &config.title,
    );
    Ok("Ok".to_string())
}

#[tauri::command]
pub fn get_menu_data(state: State<'_, Arc<Mutex<ConfigManager>>>) -> Result<String, String> {
    let config_manager = state.lock().unwrap();

    fn build_menu_items(commands: &Vec<CommandConfig>) -> Vec<serde_json::Value> {
        let mut items = Vec::new();

        for command in commands {
            let event_name = command
                .name
                .replace(" ", "_")
                .to_lowercase()
                .chars()
                .filter(|c| c.is_alphanumeric() || *c == '_')
                .collect::<String>();

            let mut item = json!({
                "name": command.name,
                "disabled": false,
                "event": format!("command_{}", event_name),
                "id": command.id.clone(),
                "command": command.command.clone(),
                "hotkey": command.hotkey.clone().unwrap_or_default()
            });

            if let Some(submenu) = &command.submenu {
                item["submenu"] = json!(build_menu_items(submenu));
            }

            items.push(item);
        }

        items
    }

    let mut grouped_items: HashMap<String, Vec<serde_json::Value>> = HashMap::new();
    for config in &config_manager.configs {
        let items = build_menu_items(&config.commands);
        if let Some(hotkey) = &config.menu_hotkey {
            grouped_items
                .entry(hotkey.clone())
                .or_insert_with(Vec::new)
                .extend(items);
        }
    }

    Ok(serde_json::to_string(&grouped_items).unwrap())
}

#[tauri::command]
pub fn execute(
    state: State<'_, Arc<Mutex<ConfigManager>>>,
    command: String,
) -> Result<String, String> {
    println!("Executing command: {}", command);

    let config_manager = state.lock().unwrap();

    match config_manager.find_command_by_id(&command) {
        Some((command, config)) => {
            execute_command(
                command,
                &config.terminal,
                &config.launch_in,
                &config.theme,
                &config.title,
            );
            Ok("Ok".to_string())
        }
        None => Err(format!("Command '{}' not found", command)),
    }
}

#[tauri::command]
pub fn fetch_input_data(
    state: State<'_, Arc<Mutex<ConfigManager>>>,
    command: String,
) -> Result<String, String> {
    println!("get_inputs_data {:?}", command);

    let config_manager = state.lock().unwrap();

    let (command, _config) = match config_manager.find_command_by_id(&command) {
        Some((cmd, cfg)) => (cmd, cfg),
        None => return Err("Command not found".to_string()),
    };

    match &command.inputs {
        Some(inputs) => Ok(json!(inputs).to_string()),
        None => return Err("Inputs not found".to_string()),
    }
}

#[tauri::command]
pub fn about_message(app: tauri::AppHandle) -> Result<String, String> {
    let version = app.package_info().version.to_string();

    let message = format!(
        r#"
        <div style="text-align: center; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;">
            <h2 style="color: #1f2937; margin-bottom: 1rem;">SwitchShuttle</h2>
            <p style="color: #6b7280; margin-bottom: 0.5rem;">Version {}</p>
            <p style="color: #6b7280; margin-bottom: 1rem;">A powerful terminal configuration manager</p>
            <div style="margin-top: 1rem;">
                <a href="https://github.com/s00d/switchshuttle" style="color: #3b82f6; text-decoration: none; margin: 0 0.5rem;">GitHub</a>
                <span style="color: #d1d5db;">|</span>
                <a href="https://switchshuttle.com" style="color: #3b82f6; text-decoration: none; margin: 0 0.5rem;">Website</a>
            </div>
            <p style="color: #9ca3af; font-size: 0.875rem; margin-top: 1rem;">Built with Tauri and Vue.js</p>
        </div>
        "#,
        version
    );

    Ok(message)
}

#[tauri::command]
pub fn get_configurations(state: State<'_, Arc<Mutex<ConfigManager>>>) -> Result<Vec<Config>, String> {
    let config_manager = state.lock().unwrap();
    Ok(config_manager.configs.clone())
}

#[tauri::command]
pub fn open_configuration(
    id: String,
    state: State<'_, Arc<Mutex<ConfigManager>>>
) -> Result<(), String> {
    let config_path = get_config_path();
    let config_dir = config_path.parent().unwrap().to_path_buf();
    
    // Находим правильное имя файла по title конфигурации
    let config_manager = state.lock().unwrap();
    let file_name = find_config_file_by_title(&config_manager, &id)
        .ok_or_else(|| format!("Configuration with title '{}' not found", id))?;
    
    let config_file = config_dir.join(format!("{}.json", file_name));
    
    if config_file.exists() {
        open_in_default_editor(&config_file);
        Ok(())
    } else {
        Err(format!("Configuration file not found: {}", file_name))
    }
}

#[tauri::command]
pub fn delete_configuration(
    id: String,
    state: State<'_, Arc<Mutex<ConfigManager>>>,
    app: tauri::AppHandle
) -> Result<(), String> {
    let config_path = get_config_path();
    let config_dir = config_path.parent().unwrap().to_path_buf();
    
    // Находим правильное имя файла по title конфигурации
    let config_manager = state.lock().unwrap();
    let file_name = find_config_file_by_title(&config_manager, &id)
        .ok_or_else(|| format!("Configuration with title '{}' not found", id))?;
    
    let config_file = config_dir.join(format!("{}.json", file_name));
    
    if config_file.exists() {
        std::fs::remove_file(&config_file)
            .map_err(|e| format!("Failed to delete configuration: {}", e))?;
        
        // Перезагружаем конфигурации в ConfigManager после удаления
        drop(config_manager); // Освобождаем блокировку перед повторным получением
        let mut config_manager = state.lock().unwrap();
        config_manager.load_configs(None)
            .map_err(|e| format!("Failed to reload configurations: {}", e))?;
        
        // Обновляем меню в трее
        update_system_tray_menu(&app, &config_manager)
    } else {
        Err(format!("Configuration file not found: {}", file_name))
    }
}

// Функция для поиска имени файла по title конфигурации
fn find_config_file_by_title(config_manager: &ConfigManager, title: &str) -> Option<String> {
    for (config, path) in config_manager.configs.iter().zip(config_manager.config_paths.iter()) {
        if config.title == title {
            return path.file_name()
                .and_then(|name| name.to_str())
                .map(|name| name.replace(".json", ""));
        }
    }
    None
}

#[tauri::command]
pub fn save_configuration(
    mut config: Config,
    state: State<'_, Arc<Mutex<ConfigManager>>>,
    app: tauri::AppHandle
) -> Result<(), String> {
    let config_path = get_config_path();
    let config_dir = config_path.parent().unwrap().to_path_buf();
    
    // Генерируем уникальное имя файла
    let unique_title = generate_unique_title(&config_dir, &config.title);
    let config_file = config_dir.join(format!("{}.json", unique_title));
    
    // Обновляем заголовок конфигурации если он изменился
    if unique_title != config.title {
        config.title = unique_title.clone();
    }
    
    // Очищаем ID перед сохранением
    config.clear_ids();
    
    config.save(&config_file)
        .map_err(|e| format!("Failed to save configuration: {}", e))?;
    
    // Перезагружаем конфигурации в ConfigManager
    let mut config_manager = state.lock().unwrap();
    config_manager.load_configs(None)
        .map_err(|e| format!("Failed to reload configurations: {}", e))?;
    
    // Обновляем меню в трее
    update_system_tray_menu(&app, &config_manager)
}

fn generate_unique_title(config_dir: &std::path::Path, base_title: &str) -> String {
    let mut counter = 1;
    let mut title = base_title.to_string();
    
    // Если заголовок пустой, используем базовый
    if title.trim().is_empty() {
        title = "New Configuration".to_string();
    }
    
    // Проверяем, существует ли файл с таким именем
    while config_dir.join(format!("{}.json", title)).exists() {
        title = format!("{} ({})", base_title, counter);
        counter += 1;
    }
    
    title
}

#[tauri::command]
pub fn get_unique_config_title(base_title: String) -> Result<String, String> {
    let config_path = get_config_path();
    let config_dir = config_path.parent().unwrap().to_path_buf();
    
    Ok(generate_unique_title(&config_dir, &base_title))
}

#[tauri::command]
pub fn save_configuration_by_id(
    mut config: Config, 
    id: String,
    state: State<'_, Arc<Mutex<ConfigManager>>>,
    app: tauri::AppHandle
) -> Result<(), String> {
    let config_path = get_config_path();
    let config_dir = config_path.parent().unwrap().to_path_buf();
    let config_file = config_dir.join(format!("{}.json", id));
    
    // Очищаем ID перед сохранением
    config.clear_ids();
    
    config.save(&config_file)
        .map_err(|e| format!("Failed to save configuration: {}", e))?;
    
    // Перезагружаем конфигурации в ConfigManager
    let mut config_manager = state.lock().unwrap();
    config_manager.load_configs(None)
        .map_err(|e| format!("Failed to reload configurations: {}", e))?;
    
    // Обновляем меню в трее
    update_system_tray_menu(&app, &config_manager)
}

#[tauri::command]
pub fn create_new_configuration() -> Result<Config, String> {
    Ok(Config::default_config())
}

#[tauri::command]
pub fn duplicate_configuration(config: Config) -> Result<Config, String> {
    let mut new_config = config.clone();
    new_config.title = format!("{} (Copy)", new_config.title);
    new_config.clear_ids();
    Ok(new_config)
}

#[tauri::command]
pub fn validate_configuration(config: Config) -> Result<(), String> {
    // Проверяем уникальность hotkeys
    let mut hotkeys = std::collections::HashSet::new();
    
    fn check_hotkeys(commands: &Vec<CommandConfig>, hotkeys: &mut std::collections::HashSet<String>) -> Result<(), String> {
        for command in commands {
            if let Some(hotkey) = &command.hotkey {
                if !hotkey.trim().is_empty() && !hotkeys.insert(hotkey.clone()) {
                    return Err(format!("Duplicate hotkey found: {}", hotkey));
                }
            }
            
            if let Some(submenu) = &command.submenu {
                check_hotkeys(submenu, hotkeys)?;
            }
        }
        Ok(())
    }
    
    fn validate_command_structure(commands: &Vec<CommandConfig>) -> Result<(), String> {
        for command in commands {
            // Проверяем, что у команды есть название
            if command.name.trim().is_empty() {
                return Err("Command name cannot be empty".to_string());
            }
            
            // Проверяем структуру команды
            let has_command = command.command.is_some();
            let has_commands = command.commands.is_some() && !command.commands.as_ref().unwrap().is_empty();
            let has_submenu = command.submenu.is_some() && !command.submenu.as_ref().unwrap().is_empty();
            
            // Команда должна иметь хотя бы один из типов: command, commands или submenu
            if !has_command && !has_commands && !has_submenu {
                return Err(format!("Command '{}' must have either a single command, multiple commands, or submenu", command.name));
            }
            
            // Проверяем, что не указаны одновременно несовместимые типы
            if has_command && has_commands {
                return Err(format!("Command '{}' cannot have both 'command' and 'commands' fields", command.name));
            }
            
            if has_command && has_submenu {
                return Err(format!("Command '{}' cannot have both 'command' and 'submenu' fields", command.name));
            }
            
            if has_commands && has_submenu {
                return Err(format!("Command '{}' cannot have both 'commands' and 'submenu' fields", command.name));
            }
            
            // Проверяем подменю рекурсивно
            if let Some(submenu) = &command.submenu {
                validate_command_structure(submenu)?;
            }
        }
        Ok(())
    }
    
    // Проверяем структуру команд
    validate_command_structure(&config.commands)?;
    
    // Проверяем уникальность горячих клавиш
    check_hotkeys(&config.commands, &mut hotkeys)?;
    
    Ok(())
}

#[tauri::command]
pub fn get_config_files() -> Result<Vec<serde_json::Value>, String> {
    let config_path = get_config_path();
    let config_dir = config_path.parent().unwrap().to_path_buf();
    
    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;
    }
    
    let mut config_files = Vec::new();
    
    if let Ok(entries) = std::fs::read_dir(&config_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.ends_with(".json") {
                        let name = file_name.replace(".json", "");
                        let path = entry.path().to_str().unwrap().to_string();
                        
                        config_files.push(serde_json::json!({
                            "name": name,
                            "path": path
                        }));
                    }
                }
            }
        }
    }
    
    Ok(config_files)
}

#[tauri::command]
pub fn load_config(path: String) -> Result<Config, String> {
    let config_path = std::path::PathBuf::from(path);
    
    if config_path.exists() {
        Config::load(&config_path)
            .map_err(|e| format!("Failed to load configuration: {}", e))
    } else {
        Err(format!("Configuration file not found: {}", config_path.display()))
    }
}

// Функция для обновления меню в трее
fn update_system_tray_menu(app: &tauri::AppHandle, config_manager: &ConfigManager) -> Result<(), String> {
    use crate::menu::create_system_tray_menu;
    use tauri_plugin_autostart::{ManagerExt};
    
    // Получаем состояние автозапуска
    let autostart_manager = app.autolaunch();
    let autostart_enabled = autostart_manager.is_enabled().unwrap_or(false);
    
    // Создаем новое меню
    let new_menu = create_system_tray_menu(app, autostart_enabled, config_manager);
    
    // Обновляем меню в трее
    if let Some(tray) = app.tray_by_id("switch-shuttle-tray") {
        tray.set_menu(Some(new_menu))
            .map_err(|e| format!("Failed to update tray menu: {}", e))?;
    }
    
    Ok(())
}

#[tauri::command]
pub fn update_configuration(
    mut config: Config,
    original_title: String,
    state: State<'_, Arc<Mutex<ConfigManager>>>,
    app: tauri::AppHandle
) -> Result<(), String> {
    let config_path = get_config_path();
    let config_dir = config_path.parent().unwrap().to_path_buf();
    
    // Используем оригинальный заголовок для поиска файла
    let original_config_file = config_dir.join(format!("{}.json", original_title));
    
    if !original_config_file.exists() {
        return Err(format!("Configuration file not found: {}", original_title));
    }
    
    // Генерируем уникальное имя файла для нового заголовка
    let unique_title = generate_unique_title(&config_dir, &config.title);
    let new_config_file = config_dir.join(format!("{}.json", unique_title));
    
    // Очищаем ID перед сохранением
    config.clear_ids();
    
    // Если заголовок изменился, переименовываем файл
    if unique_title != original_title {
        // Удаляем старый файл если новый файл уже существует
        if new_config_file.exists() {
            std::fs::remove_file(&new_config_file)
                .map_err(|e| format!("Failed to remove existing file: {}", e))?;
        }
        
        // Переименовываем файл
        std::fs::rename(&original_config_file, &new_config_file)
            .map_err(|e| format!("Failed to rename configuration file: {}", e))?;
        
        // Обновляем заголовок конфигурации если он изменился
        if unique_title != config.title {
            config.title = unique_title.clone();
        }
    }
    
    // Сохраняем конфигурацию в файл (используем правильный путь)
    let save_path = if unique_title != original_title {
        &new_config_file
    } else {
        &original_config_file
    };
    
    config.save(save_path)
        .map_err(|e| format!("Failed to save configuration: {}", e))?;
    
    // Перезагружаем конфигурации в ConfigManager
    let mut config_manager = state.lock().unwrap();
    config_manager.load_configs(None)
        .map_err(|e| format!("Failed to reload configurations: {}", e))?;
    
    // Обновляем меню в трее
    update_system_tray_menu(&app, &config_manager)
}

#[tauri::command]
pub fn refresh_configurations(
    state: State<'_, Arc<Mutex<ConfigManager>>>,
    app: tauri::AppHandle
) -> Result<(), String> {
    // Перезагружаем конфигурации в ConfigManager
    let mut config_manager = state.lock().unwrap();
    config_manager.load_configs(None)
        .map_err(|e| format!("Failed to reload configurations: {}", e))?;
    
    // Обновляем меню в трее
    update_system_tray_menu(&app, &config_manager)
}
