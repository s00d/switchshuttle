use std::{fs, io};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub terminal: String,
    pub launch_in: String,
    pub theme: String,
    pub title: String,
    pub commands: Vec<CommandConfig>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CommandConfig {
    pub name: String,
    pub command: Option<String>,
    pub submenu: Option<Vec<CommandConfig>>,
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

    pub(crate) fn load(path: &PathBuf) -> io::Result<Self> {
        let config_data = fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&config_data)?;
        Ok(config)
    }

    fn save(&self, path: &PathBuf) -> io::Result<()> {
        let config_data = serde_json::to_string_pretty(&self)?;
        fs::write(path, config_data)?;
        Ok(())
    }

    pub(crate) fn default_config() -> Self {
        Config::new(
            "iterm",
            "current",
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

    pub(crate) fn ensure_default(path: &PathBuf) -> io::Result<()> {
        if !path.exists() {
            let config = Config::default_config();
            config.save(path)?;
        }
        Ok(())
    }

    pub(crate) fn validate(&self) -> Config {
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
