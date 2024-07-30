use std::{fs};
use std::path::PathBuf;
use std::process::Command;
use include_dir::{Dir, include_dir};
use tauri::{Manager, WindowBuilder, WindowUrl};
use crate::config::CommandConfig;

static SCRIPTS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/scripts");

fn read_script(script_path: &str) -> Option<String> {
    SCRIPTS_DIR.get_file(script_path).map(|file| file.contents_utf8().unwrap().to_string())
}

pub fn execute_command(command_config: &CommandConfig, terminal: &str, launch_in: &str, theme: &String, title: &String) {
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

    if cfg!(target_os = "macos") {
        println!("Detected macOS");

        let script_path = match (terminal, launch_in) {
            ("iterm", "current") => "iTerm-Current.scpt",
            ("iterm", "new_tab") => "iTerm-Tab.scpt",
            ("iterm", "new_window") => "iTerm-Window.scpt",
            ("terminal", "current") => "Terminal-Current.scpt",
            ("terminal", "new_tab") => "Terminal-Tab.scpt",
            ("terminal", "new_window") => "Terminal-Window.scpt",
            ("warp", "current") => "Warp-Current.scpt",
            ("warp", "new_tab") => "Warp-Tab.scpt",
            ("warp", "new_window") => "Warp-Window.scpt",
            _ => "",
        };

        if script_path.is_empty() {
            println!("Unsupported terminal or launch_in option");
            return;
        }

        // Дебаг: Печать пути к скрипту
        println!("Script path: {}", script_path);

        let script_content = match read_script(script_path) {
            Some(content) => content,
            None => {
                println!("Failed to read script: {}", script_path);
                return;
            }
        };

        for command in commands_to_execute {
            let script = script_content.replace("{command}", &command)
                .replace("{theme}", theme)
                .replace("{title}", title);

            // println!("Executing script: {}", script);

            let output = Command::new("osascript")
                .arg("-e")
                .arg(&script)
                .output()
                .expect("Failed to execute command");

            // Дебаг: Печать результата выполнения
            if output.status.success() {
                println!("Command succeeded: {}", command);
                println!("Standard Output: {}", String::from_utf8_lossy(&output.stdout));
            } else {
                println!("Command failed: {}", command);
                println!("Standard Output: {}", String::from_utf8_lossy(&output.stdout));
                println!("Standard Error: {}", String::from_utf8_lossy(&output.stderr));
                break;
            }
        }
    } else if cfg!(target_os = "windows") {
        println!("Detected Windows");

        for command in commands_to_execute {
            println!("Executing command: {}", command);

            let status = Command::new("cmd")
                .args(&["/C", &command])
                .status()
                .expect("Failed to execute command");

            // Дебаг: Печать результата выполнения
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

            let status = Command::new("sh")
                .arg("-c")
                .arg(&command)
                .status()
                .expect("Failed to execute command");

            // Дебаг: Печать результата выполнения
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

pub fn create_window(app: &tauri::AppHandle, label: &str, title: &str, url: &str, width: f64, height: f64) -> tauri::Window {
    match app.get_window(label) {
        Some(window) => {
            window.show().unwrap_or_else(|e| println!("Failed to show window: {:?}", e));
            window.set_focus().expect("Failed to set focus on window");
            window
        },
        None => {
            let window = WindowBuilder::new(
                app,
                label,
                WindowUrl::App(url.into())
            )
                .title(title)
                .fullscreen(false)
                .resizable(false)
                .decorations(false)
                .inner_size(width, height)
                .always_on_top(true)
                .build()
                .expect("Failed to create window");

            window.show().unwrap_or_else(|e| println!("Failed to show window: {:?}", e));
            window.set_focus().expect("Failed to set focus on window");

            window
        }
    }
}