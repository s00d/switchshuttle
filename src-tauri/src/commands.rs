use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::State;

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
    let tauri_version = app.package_info().version.to_string();

    // Получение текущего года
    let start = UNIX_EPOCH;
    let now = SystemTime::now();
    let duration = now.duration_since(start).unwrap();
    let secs = duration.as_secs();
    let current_year = 1970 + secs / 31_536_000; // 31_536_000 секунд в году

    let message = format!(
        "<h2>About SwitchShuttle</h1>
        <p>Version: {}</p>
        <p>by <a href='https://github.com/s00d'>s00d</a></p>
        <p><a href='https://github.com/s00d/SwitchShuttle'>Homepage</a></p>
        <p>License: MIT</p>
        <p>Description: SwitchShuttle is a tool to manage your configurations and shortcuts efficiently.</p>
        <p>&copy; {} SwitchShuttle. All rights reserved.</p>",
        tauri_version, current_year
    );
    Ok(message)
}
