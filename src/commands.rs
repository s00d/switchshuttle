use std::collections::HashMap;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde_json::json;
use tauri::Window;
use crate::config::{CommandConfig, Config, ConfigManager};
use crate::{GitHubRelease};
use crate::helpers::{execute_command, get_config_path, open_in_default_editor};

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
    new_config.save(&new_config_path).map_err(|e| e.to_string())?;
    open_in_default_editor(&new_config_path);

    Ok(())
}

#[tauri::command]
pub fn get_about_message(app: tauri::AppHandle) -> String {
    let tauri_version = app.package_info().version.to_string();
    format!("SwitchShuttle v{} \n\n by s00d.", tauri_version)
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

    let response = client.get(latest_release_url).send().map_err(|e| e.to_string())?;

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
pub fn execute_command_with_inputs(inputs: HashMap<String, String>, command: String) -> Result<String, String> {
    println!("execute_command_with_inputs {:?} {:?}", inputs, command);

    let mut config_manager = ConfigManager::new();
    config_manager.load_configs().map_err(|e| e.to_string())?;

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

    println!("execute_command_with_inputs {:?}", updated_command);

    execute_command(command, &config.terminal, &config.launch_in, &config.theme, &config.title);
    Ok("Ok".to_string())
}

#[tauri::command]
pub fn get_menu_data() -> Result<String, String> {
    let mut config_manager = ConfigManager::new();
    config_manager.load_configs().map_err(|e| e.to_string())?;

    fn build_menu_items(commands: &Vec<CommandConfig>) -> Vec<serde_json::Value> {
        let mut items = Vec::new();

        for command in commands {
            let event_name = command.name
                .replace(" ", "_")
                .to_lowercase()
                .chars()
                .filter(|c| c.is_alphanumeric() || *c == '_')
                .collect::<String>();

            let mut item = json!({
                "label": command.name,
                "disabled": false,
                "event": format!("command_{}", event_name),
                "payload": command.id.clone(),
                "shortcut": command.hotkey.clone().unwrap_or_default()
            });

            if let Some(submenu) = &command.submenu {
                item["subitems"] = json!(build_menu_items(submenu));
            }

            items.push(item);
        }

        items
    }

    let mut all_items = Vec::new();
    for config in &config_manager.configs {
        all_items.extend(build_menu_items(&config.commands));
    }

    Ok(json!({ "items": all_items }).to_string())
}

#[tauri::command]
pub fn execute(command: String) -> Result<String, String> {
    println!("Executing command: {}", command);

    let mut config_manager = ConfigManager::new();
    config_manager.load_configs().map_err(|e| e.to_string())?;

    match config_manager.find_command_by_id(&command) {
        Some((command, config)) => {
            execute_command(command, &config.terminal, &config.launch_in, &config.theme, &config.title);
            Ok("Ok".to_string())
        }
        None => Err(format!("Command '{}' not found", command)),
    }
}

#[tauri::command]
pub fn show_context_menu(window: Window, x: i32, y: i32) {
    window.set_position(tauri::Position::Physical(tauri::PhysicalPosition::new(0, 0))).unwrap();
    // window.show().unwrap();
    // window.set_focus().unwrap();
    window.emit("show_context_menu", {}).unwrap();
    window.emit("menu-did-open", json!({ "x": x, "y": y })).unwrap();
}