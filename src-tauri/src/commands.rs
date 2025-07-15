use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tauri::{Manager, State};

use crate::config::{CommandConfig, Config, ConfigManager};
use crate::console;
use crate::execute::{execute_command, get_terminals, TerminalConfig};
use crate::helpers::{get_config_path, open_folder_in_default_explorer, open_in_default_editor};
use crate::hotkeys::HotkeyManager;
use crate::settings::AppSettings;

#[derive(Deserialize)]
struct GitHubRelease {
    tag_name: String,
    html_url: String,
}

#[tauri::command]
pub fn open_config_folder() -> Result<(), String> {
    let config_path = get_config_path();
    let config_dir = config_path.parent().unwrap().to_path_buf();

    open_folder_in_default_explorer(&config_dir);
    Ok(())
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
        Ok((
            "You are using the latest version.".to_string(),
            "".to_string(),
        ))
    }
}

#[tauri::command]
pub fn get_version(app: tauri::AppHandle) -> String {
    app.package_info().version.to_string()
}

#[tauri::command]
pub fn get_terminals_list() -> HashMap<String, TerminalConfig> {
    let terminals = get_terminals();

    terminals
        .into_iter()
        .map(|(key, config)| (key.to_string(), config))
        .collect()
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
pub fn execute_command_with_inputs(
    state: State<'_, Arc<Mutex<ConfigManager>>>,
    settings_state: State<'_, Arc<Mutex<AppSettings>>>,
    inputs: HashMap<String, String>,
    command: String,
    app: tauri::AppHandle,
) -> Result<String, String> {
    println!("execute_command_with_inputs {:?} {:?}", inputs, command);

    let config_manager = state.lock().unwrap();

    let (command, config) = match config_manager.find_command_by_id(&command) {
        Some((cmd, cfg)) => (cmd, cfg),
        None => return Err("Command not found".to_string()),
    };

    let mut cmd = command.command.clone();
    let mut cmds = command.commands.clone();
    let mut switch_cmd = command.switch.clone();

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

    if let Some(ref mut switch_cmd) = switch_cmd {
        for (key, value) in &inputs {
            *switch_cmd = switch_cmd.replace(&format!("[{}]", key), value);
        }
    }

    let updated_command = CommandConfig {
        id: command.id.clone(),
        name: command.name.clone(),
        inputs: command.inputs.clone(),
        command: None, // Legacy field, not used
        commands: cmds,
        submenu: command.submenu.clone(),
        hotkey: command.hotkey.clone(),
        switch: switch_cmd,
        monitor: command.monitor.clone(),
        icon: command.icon.clone(),
        scheduler: command.scheduler.clone(),
        background: command.background.clone(),
    };

    // Проверяем, является ли это switch командой
    if command.switch.is_some() {
        // Для switch команд используем execute_command_silent
        if let Some(toggle_command) = &updated_command.command {
            println!("[Monitor] switch: toggle_command = '{}'", toggle_command);
            match console::ConsoleInstance::execute_command_silent(toggle_command) {
                Ok(_) => {
                    println!("Switch command executed successfully");
                    // Показываем уведомление об успешном выполнении
                    settings_state
                        .lock()
                        .unwrap()
                        .show_success_notification(
                            &app,
                            "SwitchShuttle Success",
                            &format!("Switch command '{}' executed successfully", command.name),
                        )
                        .ok();
                }
                Err(e) => {
                    eprintln!("Failed to execute switch command: {}", e);
                    // Показываем уведомление об ошибке
                    settings_state
                        .lock()
                        .unwrap()
                        .show_error_notification(
                            &app,
                            "SwitchShuttle Error",
                            &format!("Failed to execute switch command: {}", e),
                        )
                        .ok();
                }
            }
        }
    } else {
        // Для обычных команд используем execute_command
        execute_command(
            &updated_command,
            &config.terminal,
            &config.launch_in,
            &config.theme,
            &config.title,
        );
    }
    Ok("Ok".to_string())
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
pub fn get_configurations(
    state: State<'_, Arc<Mutex<ConfigManager>>>,
) -> Result<Vec<Config>, String> {
    let config_manager = state.lock().unwrap();
    Ok(config_manager.configs.clone())
}

#[tauri::command]
pub fn open_configuration(
    id: String,
    state: State<'_, Arc<Mutex<ConfigManager>>>,
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
    app: tauri::AppHandle,
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
        config_manager
            .load_configs(None)
            .map_err(|e| format!("Failed to reload configurations: {}", e))?;

        // Обновляем меню в трее
        update_tray_menu_from_commands(&app, &config_manager)
    } else {
        Err(format!("Configuration file not found: {}", file_name))
    }
}

// Функция для поиска имени файла по title конфигурации
fn find_config_file_by_title(config_manager: &ConfigManager, title: &str) -> Option<String> {
    for (config, path) in config_manager
        .configs
        .iter()
        .zip(config_manager.config_paths.iter())
    {
        if config.title == title {
            return path
                .file_name()
                .and_then(|name| name.to_str())
                .map(|name| name.replace(".json", ""));
        }
    }
    None
}

// Универсальная функция для сохранения конфигураций
#[tauri::command]
pub fn save_or_update_configuration(
    mut config: Config,
    original_title: Option<String>, // None для новых конфигураций, Some(title) для обновления
    state: State<'_, Arc<Mutex<ConfigManager>>>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let config_path = get_config_path();
    let config_dir = config_path.parent().unwrap().to_path_buf();

    println!(
        "[Save] original_title: {:?}, config.title: {}",
        original_title, config.title
    );

    // Для обновления: проверяем что старый файл существует
    if let Some(ref original_title) = original_title {
        let original_config_file = config_dir.join(format!("{}.json", original_title));
        if !original_config_file.exists() {
            return Err(format!("Configuration file not found: {}", original_title));
        }
    }

    // Очищаем ID перед сохранением
    config.clear_ids();

    // Определяем нужно ли переименование
    let title_changed = original_title
        .as_ref()
        .map_or(false, |orig| config.title != *orig);

    println!(
        "[Save] title_changed: {}, original_title.is_some(): {}",
        title_changed,
        original_title.is_some()
    );

    if original_title.is_some() && !title_changed {
        // Заголовок не изменился при обновлении - сохраняем в тот же файл
        let original_title = original_title.unwrap(); // Safe unwrap, так как мы знаем что original_title.is_some()
        let config_file = config_dir.join(format!("{}.json", original_title));
        println!("[Save] Saving to existing file: {}", config_file.display());
        config
            .save(&config_file)
            .map_err(|e| format!("Failed to save configuration: {}", e))?;
    } else {
        // Это новая конфигурация или изменилось название - генерируем уникальное имя
        let unique_title =
            generate_unique_title(&config_dir, &config.title, original_title.as_deref());
        println!("[Save] Generated unique title: {}", unique_title);
        config.title = unique_title.clone();

        let new_config_file = config_dir.join(format!("{}.json", unique_title));
        println!("[Save] Saving to new file: {}", new_config_file.display());

        // Если новый файл уже существует, удаляем его
        if new_config_file.exists() {
            println!(
                "[Save] Removing existing file: {}",
                new_config_file.display()
            );
            std::fs::remove_file(&new_config_file)
                .map_err(|e| format!("Failed to remove existing file: {}", e))?;
        }

        // Сохраняем конфигурацию в новый файл
        config
            .save(&new_config_file)
            .map_err(|e| format!("Failed to save configuration: {}", e))?;

        // Удаляем старый файл если это обновление с измененным именем
        if let Some(original_title) = original_title {
            let original_config_file = config_dir.join(format!("{}.json", original_title));
            if original_config_file.exists() {
                println!(
                    "[Save] Removing old file: {}",
                    original_config_file.display()
                );
                std::fs::remove_file(&original_config_file)
                    .map_err(|e| format!("Failed to remove old configuration file: {}", e))?;
            }
        }
    }

    // Перезагружаем конфигурации в ConfigManager
    let mut config_manager = state.lock().unwrap();
    config_manager
        .load_configs(None)
        .map_err(|e| format!("Failed to reload configurations: {}", e))?;

    // Обновляем меню в трее
    update_tray_menu_from_commands(&app, &config_manager)?;

    // Перерегистрируем хоткеи
    let hotkey_manager = app.state::<Arc<Mutex<HotkeyManager>>>();
    let settings_state = app.state::<Arc<Mutex<AppSettings>>>();
    let mut hotkey_manager = hotkey_manager.lock().unwrap();
    hotkey_manager.set_app_handle(app.clone());
    if let Err(e) =
        hotkey_manager.register_all_hotkeys(&config_manager.configs, &app, &settings_state)
    {
        eprintln!("Failed to reregister hotkeys after save: {}", e);
    }

    Ok(())
}

fn generate_unique_title(
    config_dir: &std::path::Path,
    base_title: &str,
    original_title: Option<&str>,
) -> String {
    let mut counter = 1;
    let mut title = base_title.trim().to_string();

    println!(
        "[Generate] base_title: '{}', original_title: {:?}",
        base_title, original_title
    );

    // Если заголовок пустой, используем базовый
    if title.is_empty() {
        title = "New Configuration".to_string();
    }

    // Сохраняем исходный title для генерации вариантов
    let original_title_name = title.clone();

    // Защита от бесконечного цикла (максимум 10000 попыток)
    const MAX_ATTEMPTS: usize = 10000;
    let mut attempts = 0;

    // Проверяем, существует ли файл с таким именем
    while config_dir.join(format!("{}.json", title)).exists() {
        println!("[Generate] File exists: {}.json", title);

        // Если это тот же файл (original_title совпадает с текущим title),
        // то возвращаем текущее название без изменений
        if let Some(orig_title) = original_title {
            if title == orig_title {
                println!("[Generate] Same file, returning: {}", title);
                return title;
            }
        }

        counter += 1;
        title = format!("{} ({})", original_title_name, counter);
        println!("[Generate] Trying: {}", title);

        attempts += 1;
        if attempts >= MAX_ATTEMPTS {
            // В крайне редком случае, если не можем найти уникальное имя
            title = format!(
                "{}-{}",
                original_title_name,
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis()
            );
            break;
        }
    }

    println!("[Generate] Final title: {}", title);
    title
}

#[tauri::command]
pub fn get_unique_config_title(base_title: String) -> Result<String, String> {
    let config_path = get_config_path();
    let config_dir = config_path.parent().unwrap().to_path_buf();

    Ok(generate_unique_title(&config_dir, &base_title, None))
}

#[tauri::command]
pub fn save_configuration_by_id(
    mut config: Config,
    id: String,
    state: State<'_, Arc<Mutex<ConfigManager>>>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let config_path = get_config_path();
    let config_dir = config_path.parent().unwrap().to_path_buf();
    let config_file = config_dir.join(format!("{}.json", id));

    // Очищаем ID перед сохранением
    config.clear_ids();

    config
        .save(&config_file)
        .map_err(|e| format!("Failed to save configuration: {}", e))?;

    // Перезагружаем конфигурации в ConfigManager
    let mut config_manager = state.lock().unwrap();
    config_manager
        .load_configs(None)
        .map_err(|e| format!("Failed to reload configurations: {}", e))?;

    // Обновляем меню в трее
    update_tray_menu_from_commands(&app, &config_manager)
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

    fn check_hotkeys(
        commands: &Vec<CommandConfig>,
        hotkeys: &mut std::collections::HashSet<String>,
    ) -> Result<(), String> {
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
            let has_commands =
                command.commands.is_some() && !command.commands.as_ref().unwrap().is_empty();
            let has_submenu =
                command.submenu.is_some() && !command.submenu.as_ref().unwrap().is_empty();

            // Команда должна иметь хотя бы один из типов: command, commands или submenu
            if !has_command && !has_commands && !has_submenu {
                return Err(format!(
                    "Command '{}' must have either a single command, multiple commands, or submenu",
                    command.name
                ));
            }

            // Проверяем, что не указаны одновременно несовместимые типы
            if has_command && has_commands {
                return Err(format!(
                    "Command '{}' cannot have both 'command' and 'commands' fields",
                    command.name
                ));
            }

            if has_command && has_submenu {
                return Err(format!(
                    "Command '{}' cannot have both 'command' and 'submenu' fields",
                    command.name
                ));
            }

            if has_commands && has_submenu {
                return Err(format!(
                    "Command '{}' cannot have both 'commands' and 'submenu' fields",
                    command.name
                ));
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
        Config::load(&config_path).map_err(|e| format!("Failed to load configuration: {}", e))
    } else {
        Err(format!(
            "Configuration file not found: {}",
            config_path.display()
        ))
    }
}

// Функция для обновления меню в трее
fn update_tray_menu_from_commands(
    app: &tauri::AppHandle,
    config_manager: &ConfigManager,
) -> Result<(), String> {
    use crate::menu::create_system_tray_menu;
    use tauri_plugin_autostart::ManagerExt;

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
pub fn refresh_configurations(
    state: State<'_, Arc<Mutex<ConfigManager>>>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    // Перезагружаем конфигурации в ConfigManager
    let mut config_manager = state.lock().unwrap();
    config_manager
        .load_configs(None)
        .map_err(|e| format!("Failed to reload configurations: {}", e))?;

    // Обновляем меню в трее
    update_tray_menu_from_commands(&app, &config_manager)
}

#[tauri::command]
pub fn get_settings_schema() -> Result<serde_json::Value, String> {
    Ok(AppSettings::get_settings_schema())
}

#[tauri::command]
pub fn get_settings() -> Result<AppSettings, String> {
    AppSettings::load().map_err(|e| format!("Failed to load settings: {}", e))
}

#[tauri::command]
pub fn save_settings(
    settings: AppSettings,
    app: tauri::AppHandle,
    state: State<'_, Arc<Mutex<AppSettings>>>,
) -> Result<(), String> {
    settings
        .save()
        .map_err(|e| format!("Failed to save settings: {}", e))?;
    settings
        .apply(&app)
        .map_err(|e| format!("Failed to apply settings: {}", e))?;
    // Обновляем state
    *state.lock().unwrap() = settings;
    Ok(())
}

#[derive(Debug, Deserialize)]
pub enum NotificationType {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "warning")]
    Warning,
}

#[tauri::command]
pub fn show_notification(
    state: State<'_, Arc<Mutex<AppSettings>>>,
    app: tauri::AppHandle,
    title: String,
    body: String,
    notification_type: NotificationType,
) -> Result<(), String> {
    println!(
        "[Commands] show_notification called: title='{}', body='{}', type='{:?}'",
        title, body, notification_type
    );

    let settings = state.lock().unwrap();

    match notification_type {
        NotificationType::Default => {
            println!("[Commands] Showing default notification");
            settings
                .show_notification(&app, &title, &body)
                .map_err(|e| format!("Failed to show notification: {}", e))
        }
        NotificationType::Success => {
            println!("[Commands] Showing success notification");
            settings
                .show_success_notification(&app, &title, &body)
                .map_err(|e| format!("Failed to show success notification: {}", e))
        }
        NotificationType::Error => {
            println!("[Commands] Showing error notification");
            settings
                .show_error_notification(&app, &title, &body)
                .map_err(|e| format!("Failed to show error notification: {}", e))
        }
        NotificationType::Info => {
            println!("[Commands] Showing info notification");
            settings
                .show_info_notification(&app, &title, &body)
                .map_err(|e| format!("Failed to show info notification: {}", e))
        }
        NotificationType::Warning => {
            println!("[Commands] Showing warning notification");
            settings
                .show_warning_notification(&app, &title, &body)
                .map_err(|e| format!("Failed to show warning notification: {}", e))
        }
    }
}
