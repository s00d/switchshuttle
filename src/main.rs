// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs, io};
use std::path::PathBuf;
use std::process::Command;
use include_dir::{Dir, include_dir};
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, SystemTraySubmenu};
use serde::{Deserialize, Serialize};

static SCRIPTS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/scripts");

#[derive(Serialize, Deserialize)]
struct Config {
    terminal: String,
    launch_in: String,
    theme: String,
    title: String,
    commands: Vec<CommandConfig>,
}

#[derive(Serialize, Deserialize, Clone)]
struct CommandConfig {
    name: String,
    command: Option<String>,
    submenu: Option<Vec<CommandConfig>>,
}

impl Config {
    fn new(terminal: &str, launch_in: &str, theme: &str, title: &str, commands: Vec<CommandConfig>) -> Self {
        Config {
            terminal: terminal.to_string(),
            launch_in: launch_in.to_string(),
            theme: theme.to_string(),
            title: title.to_string(),
            commands,
        }
    }

    fn load(path: &PathBuf) -> io::Result<Self> {
        let config_data = fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&config_data)?;
        Ok(config)
    }

    fn save(&self, path: &PathBuf) -> io::Result<()> {
        let config_data = serde_json::to_string_pretty(&self)?;
        fs::write(path, config_data)?;
        Ok(())
    }

    fn default_config() -> Self {
        Config::new(
            "iterm",
            "new_tab",
            "Homebrew",
            "New tab",
            vec![
                CommandConfig {
                    name: "Example Command".to_string(),
                    command: Some("echo Hello, world!".to_string()),
                    submenu: None,
                },
                CommandConfig {
                    name: "Example Submenu".to_string(),
                    command: None,
                    submenu: Some(vec![
                        CommandConfig {
                            name: "Subcommand 1".to_string(),
                            command: Some("echo Subcommand 1".to_string()),
                            submenu: None,
                        },
                        CommandConfig {
                            name: "Subcommand 2".to_string(),
                            command: Some("echo Subcommand 2".to_string()),
                            submenu: None,
                        },
                    ]),
                },
            ],
        )
    }

    fn ensure_default(path: &PathBuf) -> io::Result<()> {
        if !path.exists() {
            let config = Config::default_config();
            config.save(path)?;
        }
        Ok(())
    }

    fn validate(&self) -> Config {
        let valid_terminals = vec!["iterm", "terminal", "warp", "starship"];
        let valid_launch_in = vec!["current", "new_tab", "new_window"];

        let terminal = if valid_terminals.contains(&self.terminal.as_str()) {
            self.terminal.clone()
        } else {
            println!("Invalid terminal: {}. Using default 'iterm'.", self.terminal);
            "iterm".to_string()
        };

        let launch_in = if valid_launch_in.contains(&self.launch_in.as_str()) {
            self.launch_in.clone()
        } else {
            println!("Invalid launch_in: {}. Using default 'new_tab'.", self.launch_in);
            "new_tab".to_string()
        };

        let commands: Vec<CommandConfig> = self.commands.iter().map(|command| {
            if command.name.is_empty() || (command.command.is_none() && command.submenu.is_none()) {
                CommandConfig {
                    name: "Example Command".to_string(),
                    command: Some("echo Hello, world!".to_string()),
                    submenu: None,
                }
            } else {
                CommandConfig {
                    name: command.name.clone(),
                    command: command.command.clone(),
                    submenu: command.submenu.clone(),
                }
            }
        }).collect();

        Config::new(&terminal, &launch_in, &self.theme, &self.title, commands)
    }
}

fn is_launch_at_login_enabled() -> bool {
    #[cfg(target_os = "windows")]
    {
        use winreg::enums::*;
        use winreg::RegKey;

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let path = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
        let key = hkcu.open_subkey(path).expect("Unable to open registry key");
        key.get_value::<String, _>("SwitchShuttle").is_ok()
    }

    #[cfg(target_os = "macos")]
    {
        let label = "com.SwitchShuttle.app";
        let plist_path = format!("{}/Library/LaunchAgents/{}.plist", dirs::home_dir().unwrap().display(), label);
        std::path::Path::new(&plist_path).exists()
    }

    #[cfg(target_os = "linux")]
    {
        let autostart_path = format!("{}/.config/autostart/SwitchShuttle.desktop", dirs::home_dir().unwrap().display());
        std::path::Path::new(&autostart_path).exists()
    }
}

#[cfg(target_os = "windows")]
fn set_launch_at_login(enable: bool) {
    use winreg::enums::*;
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
    let (key, _) = hkcu.create_subkey(path).expect("Unable to open registry key");

    if enable {
        key.set_value("SwitchShuttle", &std::env::current_exe().unwrap().to_str().unwrap())
            .expect("Unable to set registry value");
    } else {
        key.delete_value("SwitchShuttle").expect("Unable to delete registry value");
    }
}

#[cfg(target_os = "macos")]
fn set_launch_at_login(enable: bool) {
    let label = "com.SwitchShuttle.app";
    let plist_path = format!("{}/Library/LaunchAgents/{}.plist", dirs::home_dir().unwrap().display(), label);
    let plist_content = format!(r#"
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>{}</string>
    <key>ProgramArguments</key>
    <array>
        <string>{}</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
</dict>
</plist>
"#, label, std::env::current_exe().unwrap().display());

    if enable {
        fs::write(&plist_path, plist_content).expect("Unable to write plist file");
        Command::new("launchctl").arg("load").arg(&plist_path).output().expect("Failed to load plist");
    } else {
        Command::new("launchctl").arg("unload").arg(&plist_path).output().expect("Failed to unload plist");
        fs::remove_file(plist_path).expect("Unable to remove plist file");
    }
}

#[cfg(target_os = "linux")]
fn set_launch_at_login(enable: bool) {
    let autostart_path = format!("{}/.config/autostart/SwitchShuttle.desktop", dirs::home_dir().unwrap().display());
    let autostart_content = format!(r#"
[Desktop Entry]
Type=Application
Exec={}
Hidden=false
NoDisplay=false
X-GNOME-Autostart-enabled=true
Name[en_US]=SwitchShuttle
Name=SwitchShuttle
Comment[en_US]=Start SwitchShuttle at login
Comment=Start SwitchShuttle at login
"#, std::env::current_exe().unwrap().display());

    if enable {
        std::fs::create_dir_all(dirs::home_dir().unwrap().join(".config/autostart")).expect("Unable to create autostart directory");
        std::fs::write(&autostart_path, autostart_content).expect("Unable to write autostart file");
    } else {
        std::fs::remove_file(autostart_path).expect("Unable to remove autostart file");
    }
}

fn read_script(script_path: &str) -> Option<String> {
    SCRIPTS_DIR.get_file(script_path).map(|file| file.contents_utf8().unwrap().to_string())
}

fn execute_command(command_config: &CommandConfig, terminal: &str, launch_in: &str, theme: &String, title: &String) {
    if cfg!(target_os = "macos") {
        println!("{}, {}", terminal, launch_in);
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

        println!("{}", script);

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

fn open_in_default_editor(path: &PathBuf) {
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

fn get_config_path() -> PathBuf {
    let mut config_path = dirs::config_dir().unwrap();
    config_path.push("switch-shuttle");
    fs::create_dir_all(&config_path).unwrap();
    config_path.push("config.json");
    config_path
}

fn open_folder_in_default_explorer(path: &PathBuf) {
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

fn load_config() -> Config {
    let config_path = get_config_path();
    let config = Config::load(&config_path).unwrap_or_else(|err| {
        println!("Failed to load config: {}. Using default config.", err);
        Config::default_config()
    });
    config.validate()
}

fn create_command_menu(command_config: &CommandConfig) -> SystemTrayMenu {
    let mut menu = SystemTrayMenu::new();
    if let Some(submenu) = &command_config.submenu {
        for subcommand in submenu {
            if let Some(_subsubmenu) = &subcommand.submenu {
                let submenu_item = create_command_menu(subcommand);
                menu = menu.add_submenu(SystemTraySubmenu::new(subcommand.name.clone(), submenu_item));
            } else {
                let item = CustomMenuItem::new(subcommand.name.clone(), subcommand.name.clone());
                menu = menu.add_item(item);
            }
        }
    }
    menu
}

fn create_system_tray_menu() -> SystemTrayMenu {
    let config = load_config();

    let mut command_menu = SystemTrayMenu::new();

    for command in &config.commands {
        if let Some(_submenu) = &command.submenu {
            let submenu_item = create_command_menu(command);
            command_menu = command_menu.add_submenu(SystemTraySubmenu::new(command.name.clone(), submenu_item));
        } else {
            let item = CustomMenuItem::new(command.name.clone(), command.name.clone());
            command_menu = command_menu.add_item(item);
        }
    }

    let mut tray_menu = SystemTrayMenu::new();
    tray_menu = tray_menu.add_submenu(SystemTraySubmenu::new("Commands", command_menu));
    tray_menu = tray_menu.add_native_item(SystemTrayMenuItem::Separator);
    tray_menu = tray_menu.add_item(CustomMenuItem::new("edit_config".to_string(), "Edit Config"));
    tray_menu = tray_menu.add_item(CustomMenuItem::new("open_config_folder".to_string(), "Open Config Folder"));

    let launch_at_login_text = if is_launch_at_login_enabled() {
        "✔ Launch at Login"
    } else {
        "✖ Launch at Login"
    };
    tray_menu = tray_menu.add_item(CustomMenuItem::new("toggle_launch_at_login".to_string(), launch_at_login_text));

    tray_menu = tray_menu.add_native_item(SystemTrayMenuItem::Separator);
    tray_menu = tray_menu.add_item(CustomMenuItem::new("quit".to_string(), "Quit"));

    tray_menu
}

fn main() {
    let config_path = get_config_path();
    Config::ensure_default(&config_path).expect("Failed to ensure default config");

    let system_tray_menu = create_system_tray_menu();

    let app = tauri::Builder::default()
        .system_tray(SystemTray::new().with_menu(system_tray_menu))
        .on_system_tray_event(move |app, event| {
            if let SystemTrayEvent::MenuItemClick { id, .. } = event {
                if id.as_str() == "quit" {
                    std::process::exit(0);
                }
                if id.as_str() == "edit_config" {
                    open_in_default_editor(&config_path);
                } else if id.as_str() == "open_config_folder" {
                    open_folder_in_default_explorer(&config_path.parent().unwrap().to_path_buf());
                } else if id.as_str() == "toggle_launch_at_login" {
                    let enabled = is_launch_at_login_enabled();
                    set_launch_at_login(!enabled);

                    // Recreate the tray menu
                    let new_system_tray_menu = create_system_tray_menu();
                    app.tray_handle().set_menu(new_system_tray_menu).unwrap();
                } else {
                    let config = load_config();
                    if let Some(command_config) = config.commands.iter().find(|c| c.name == id.as_str()) {
                        execute_command(command_config, &config.terminal, &config.launch_in, &config.theme, &config.title);
                    } else {
                        println!("Command not found: {}", id);
                    }
                }
            }

            // Recreate the tray menu for all menu item clicks
            let new_system_tray_menu = create_system_tray_menu();
            app.tray_handle().set_menu(new_system_tray_menu).unwrap();
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|_app_handle, _event| {});
}





