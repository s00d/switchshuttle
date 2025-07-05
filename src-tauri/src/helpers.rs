use include_dir::{include_dir, Dir};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tauri::{AppHandle, Emitter, Manager, Wry, WebviewWindowBuilder};
use tauri::menu::{IconMenuItem, IconMenuItemBuilder, CheckMenuItem};
use tauri::image::Image;
use tauri::path::BaseDirectory;
use tauri_plugin_notification::NotificationExt;
use crate::config::CommandConfig;

static SCRIPTS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/scripts");

fn read_script(script_path: &str) -> Option<String> {
    SCRIPTS_DIR
        .get_file(script_path)
        .map(|file| file.contents_utf8().unwrap().to_string())
}

pub fn execute_command(
    command_config: &CommandConfig,
    terminal: &str,
    launch_in: &str,
    theme: &String,
    title: &String,
) {
    let mut commands_to_execute = Vec::new();

    // Дебаг: Инициализация commands_to_execute
    println!("Initializing commands_to_execute...");

    if let Some(command) = &command_config.command {
        println!("Adding single command: {}", command);
        commands_to_execute.push(command.clone());
    }

    if let Some(commands) = &command_config.commands {
        println!("Adding multiple commands: {:?}", commands);
        commands_to_execute.extend(commands.clone());
    }

    // Дебаг: Печать списка команд
    println!("Commands to execute: {:?}", commands_to_execute);

    let terminal = terminal.to_lowercase();
    let launch_in = launch_in.to_lowercase();

    if cfg!(target_os = "macos") {
        println!("Detected macOS");

        let script_path = match (terminal.as_str(), launch_in.as_str()) {
            ("iterm", "current") => "iTerm-Current.scpt",
            ("iterm", "new_tab") => "iTerm-Tab.scpt",
            ("iterm", "new_window") => "iTerm-Window.scpt",
            ("terminal", "current") => "Terminal-Current.scpt",
            ("terminal", "new_tab") => "Terminal-Tab.scpt",
            ("terminal", "new_window") => "Terminal-Window.scpt",
            ("warp", "current") => "Warp-Current.scpt",
            ("warp", "new_tab") => "Warp-Tab.scpt",
            ("warp", "new_window") => "Warp-Window.scpt",
            ("hyper", "current") => "Hyper-Current.scpt",
            ("hyper", "new_tab") => "Hyper-Tab.scpt",
            ("hyper", "new_window") => "Hyper-Window.scpt",
            ("alacritty", "current") => "Alacritty-Current.scpt",
            ("alacritty", "new_tab") => "Alacritty-Tab.scpt",
            ("alacritty", "new_window") => "Alacritty-Window.scpt",
            _ => "",
        };

        if script_path.is_empty() {
            println!("Unsupported terminal or launch_in option");
            return;
        }

        println!("Script path: {}", script_path);

        let script_content = match read_script(script_path) {
            Some(content) => content,
            None => {
                println!("Failed to read script: {}", script_path);
                return;
            }
        };

        for command in commands_to_execute {
            let script = script_content
                .replace("{command}", &command)
                .replace("{theme}", theme)
                .replace("{title}", title);

            println!("Executing script: {}", script);

            let output = Command::new("osascript")
                .arg("-e")
                .arg(&script)
                .output()
                .expect("Failed to execute command");

            // Дебаг: Печать результата выполнения
            if output.status.success() {
                println!("Command succeeded: {}", command);
                println!(
                    "Standard Output: {}",
                    String::from_utf8_lossy(&output.stdout)
                );
            } else {
                println!("Command failed: {}", command);
                println!(
                    "Standard Output: {}",
                    String::from_utf8_lossy(&output.stdout)
                );
                println!(
                    "Standard Error: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
                break;
            }
        }
    } else if cfg!(target_os = "windows") {
        println!("Detected Windows");

        for command in commands_to_execute {
            println!("Executing command: {}", command);

            let status = match terminal.as_str() {
                "hyper" => Command::new("cmd")
                    .args(&["/C", &format!("start hyper -e \"{}\"", command)])
                    .status(),
                _ => Command::new("cmd").args(&["/C", &command]).status(),
            }
            .expect("Failed to execute command");

            if status.success() {
                println!("Command succeeded: {}", command);
            } else {
                println!("Command failed: {}", command);
                break;
            }
        }
    } else if cfg!(target_os = "linux") {
        println!("Detected Linux");

        for command in commands_to_execute {
            println!("Executing command: {}", command);

            let status = match terminal.as_str() {
                "hyper" => Command::new("sh")
                    .arg("-c")
                    .arg(&format!("hyper -e \"{}\"", command))
                    .status(),
                _ => Command::new("sh").arg("-c").arg(&command).status(),
            }
            .expect("Failed to execute command");

            if status.success() {
                println!("Command succeeded: {}", command);
            } else {
                println!("Command failed: {}", command);
                break;
            }
        }
    } else {
        println!("Unsupported operating system");
    }
}

pub fn execute_command_silent(
    command: &str,
) -> Result<String, String> {
    if cfg!(target_os = "macos") {
        // На macOS используем sh для выполнения команды
        let output = Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .map_err(|e| format!("Failed to execute command: {}", e))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    } else if cfg!(target_os = "windows") {
        // На Windows используем cmd для выполнения команды
        let output = Command::new("cmd")
            .args(&["/C", command])
            .output()
            .map_err(|e| format!("Failed to execute command: {}", e))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    } else if cfg!(target_os = "linux") {
        // На Linux используем sh для выполнения команды
        let output = Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .map_err(|e| format!("Failed to execute command: {}", e))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    } else {
        Err("Unsupported operating system".to_string())
    }
}

pub fn open_in_default_editor(path: &PathBuf) {
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(path)
            .spawn()
            .expect("Failed to open file in default editor");
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(&["/C", "start", path.to_str().unwrap()])
            .spawn()
            .expect("Failed to open file in default editor");
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(path)
            .spawn()
            .expect("Failed to open file in default editor");
    }
}

pub fn get_config_path() -> PathBuf {
    let mut config_path = dirs::config_dir().unwrap();
    config_path.push("switch-shuttle");
    fs::create_dir_all(&config_path).unwrap();
    config_path.push("config.json");
    config_path
}

pub fn open_folder_in_default_explorer(path: &PathBuf) {
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(path)
            .spawn()
            .expect("Failed to open folder in default explorer");
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .arg(path)
            .spawn()
            .expect("Failed to open folder in default explorer");
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(path)
            .spawn()
            .expect("Failed to open folder in default explorer");
    }
}

pub fn create_window(
    app: &AppHandle,
    window_id: &str,
    title: &str,
    route: &str,
    width: f64,
    height: f64,
    center: bool,
) -> Result<(), String> {
    // Используем отдельное окно
    if let Some(existing_window) = app.get_webview_window(window_id) {
        // Если окно уже существует, просто показываем его и устанавливаем фокус
        existing_window.show().unwrap_or_else(|e| println!("Failed to show existing window: {:?}", e));
        existing_window.set_focus().unwrap_or_else(|e| println!("Failed to focus existing window: {:?}", e));
        existing_window.emit("navigate", (route, title)).unwrap();
        return Ok(());
    }

    // Создаем новое окно
    let window = WebviewWindowBuilder::new(app, window_id, tauri::WebviewUrl::App(route.into()))
        .title(title)
        .inner_size(width, height)
        .resizable(true)
        .center()
        .build()
        .map_err(|e| format!("Failed to create window: {}", e))?;

    // Устанавливаем иконку если возможно
    if let Ok(icon_path) = app.path().resolve("icons/icon.png", BaseDirectory::Resource) {
        if let Ok(image) = Image::from_path(icon_path) {
            if let Err(e) = window.set_icon(image) {
                println!("Failed to set window icon: {:?}", e);
            }
        }
    }

    if let Err(e) = window.set_focus() {
        println!("Failed to set window focus: {:?}", e);
    }

    window.emit("navigate", (route, title)).unwrap();

    if center {
        if let Err(e) = window.center() {
            println!("Failed to center window: {:?}", e);
        }
    }

    Ok(())
}


#[cfg(debug_assertions)]
pub fn change_devtools(app: &AppHandle) {
    let window = app.get_webview_window("main").unwrap();
    if !window.is_devtools_open() {
        window.open_devtools();
    } else {
        window.close_devtools();
    }
}

#[cfg(not(debug_assertions))]
pub fn change_devtools(_app: &AppHandle) {}

/// Создает пункт меню с иконкой и опциональной горячей клавишей
pub fn create_menu_item(
    app: &AppHandle<Wry>,
    id: &str,
    text: &str,
    icon_name: &str,
    hotkey: Option<String>,
    icon: Option<&str>,
) -> IconMenuItem<Wry> {
    let display_text = if let Some(icon_symbol) = icon {
        format!("{} {}", icon_symbol, text)
    } else {
        text.to_string()
    };
    
    let mut builder = IconMenuItemBuilder::with_id(id, &display_text);
    
    // Пытаемся загрузить иконку, но не падаем если её нет
    // Добавляем иконку только если нет пользовательской иконки
    if icon.is_none() {
        if let Ok(icon_path) = app.path().resolve(&format!("icons/{}.png", icon_name), BaseDirectory::Resource) {
            if let Ok(image) = Image::from_path(icon_path) {
                builder = builder.icon(image);
            }
        }
    }
    
    // Добавляем горячую клавишу если указана
    if let Some(hotkey) = hotkey {
        builder = builder.accelerator(&hotkey);
    }
    
    builder.build(app).unwrap()
}

/// Создает чекбокс пункт меню с опциональной горячей клавишей
pub fn create_check_menu_item(
    app: &AppHandle<Wry>,
    id: &str,
    text: &str,
    checked: bool,
    hotkey: Option<String>,
    icon: Option<&str>,
) -> CheckMenuItem<Wry> {
    let display_text = if let Some(icon_symbol) = icon {
        format!("{} {}", icon_symbol, text)
    } else {
        text.to_string()
    };
    
    let hotkey_ref = hotkey.as_ref().map(|s| s.as_str());
    
    CheckMenuItem::with_id(
        app.app_handle(),
        id,
        &display_text,
        true, // enabled
        checked,
        hotkey_ref,
    )
    .unwrap()
}

/// Проверяет состояние переключателя, выполняя команду
pub fn is_switch_enabled(switch_command: &str, app: Option<&AppHandle<Wry>>) -> bool {
    // Выполняем команду переключателя в фоне и получаем результат
    match execute_command_silent(switch_command) {
        Ok(output) => {
            // Проверяем результат - если команда вернула "true" или непустую строку, считаем включенным
            let output = output.trim().to_lowercase();
            output == "true" || output == "1" || output == "enabled" || output == "on"
        }
        Err(e) => {
            eprintln!("Failed to check switch status: {}", e);
            // Показываем уведомление об ошибке, если app доступен
            if let Some(app_handle) = app {
                if let Ok(_) = app_handle.notification().builder()
                    .title("SwitchShuttle Error")
                    .body(&format!("Failed to check switch status: {}", e))
                    .show() {
                    // Уведомление отправлено
                }
            }
            false
        }
    }
}
