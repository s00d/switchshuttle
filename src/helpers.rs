use std::fs;
use std::path::PathBuf;
use std::process::Command;
use include_dir::{Dir, include_dir};
use crate::config::CommandConfig;

static SCRIPTS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/scripts");

fn read_script(script_path: &str) -> Option<String> {
    SCRIPTS_DIR.get_file(script_path).map(|file| file.contents_utf8().unwrap().to_string())
}

pub fn execute_command(command_config: &CommandConfig, terminal: &str, launch_in: &str, theme: &String, title: &String) {
    if cfg!(target_os = "macos") {
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
            ("starship", "current") => "Starship-Current.scpt",
            ("starship", "new_tab") => "Starship-Tab.scpt",
            ("starship", "new_window") => "Starship-Window.scpt",
            _ => "",
        };

        if script_path.is_empty() {
            println!("Unsupported terminal or launch_in option");
            return;
        }

        let script_content = match read_script(script_path) {
            Some(content) => content,
            None => {
                println!("Failed to read script: {}", script_path);
                return;
            }
        };

        let command = command_config.command.as_deref().unwrap_or("");

        let script = script_content.replace("{command}", command)
            .replace("{theme}", theme)
            .replace("{title}", title);

        Command::new("osascript")
            .arg("-e")
            .arg(script)
            .spawn()
            .expect("Failed to execute command");
    } else if cfg!(target_os = "windows") {
        if let Some(command) = &command_config.command {
            Command::new("cmd")
                .args(&["/C", command])
                .spawn()
                .expect("Failed to execute command");
        }
    } else if cfg!(target_os = "linux") {
        if let Some(command) = &command_config.command {
            Command::new("sh")
                .arg("-c")
                .arg(command)
                .spawn()
                .expect("Failed to execute command");
        }
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