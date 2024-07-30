use std::{fs, io};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub terminal: String,
    pub launch_in: String,
    pub theme: String,
    pub title: String,
    pub commands: Vec<CommandConfig>,
    pub menu_hotkey: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommandConfig {
    pub id: Option<String>,
    pub name: String,
    pub inputs: Option<HashMap<String, String>>,
    pub command: Option<String>,
    pub commands: Option<Vec<String>>,
    pub hotkey: Option<String>,
    pub submenu: Option<Vec<CommandConfig>>,
}

#[derive(Clone, Debug)]
pub struct ConfigManager {
    pub configs: Vec<Config>,
    pub config_paths: Vec<PathBuf>,
    counter: Arc<AtomicUsize>
}


impl ConfigManager {
    pub fn new() -> Self {
        ConfigManager {
            configs: Vec::new(),
            config_paths: Vec::new(),
            counter: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn load_configs(&mut self) -> io::Result<()> {
        self.counter = Arc::new(AtomicUsize::new(0));

        let config_path = get_config_path();
        let config_dir = config_path.parent().unwrap().to_path_buf();

        self.configs.clear();
        self.config_paths.clear();

        let mut has_configs = false;

        if let Ok(entries) = fs::read_dir(&config_dir) {
            let mut paths: Vec<_> = entries.filter_map(|entry| {
                entry.ok().and_then(|e| {
                    let path = e.path();
                    if path.is_file() {
                        Some(path)
                    } else {
                        None
                    }
                })
            }).collect();

            paths.sort();

            for path in paths {
                if let Ok(mut config) = Config::load(&path) {
                    config.assign_ids(&self.counter);
                    self.config_paths.push(path);
                    self.configs.push(config);
                    has_configs = true;
                }
            }
        }

        if !has_configs {
            let mut default_config = Config::default_config();
            let default_config_path = config_dir.join("default_config.json");
            default_config.save(&default_config_path)?;
            self.config_paths.push(default_config_path);
            self.configs.push(default_config);
        }

        Ok(())
    }

    pub fn find_command_by_id(&self, id: &str) -> Option<(&CommandConfig, &Config)> {
        for config in &self.configs {
            if let Some(command) = config.find_command_by_id(id) {
                return Some((command, config));
            }
        }
        None
    }
}

impl Config {
    fn new(terminal: &str, launch_in: &str, theme: &str, title: &str, menu_hotkey: Option<String>, commands: Vec<CommandConfig>) -> Self {
        Config {
            terminal: terminal.to_string(),
            launch_in: launch_in.to_string(),
            theme: theme.to_string(),
            title: title.to_string(),
            menu_hotkey,
            commands,
        }
    }

    pub fn load(path: &PathBuf) -> io::Result<Self> {
        let config_data = fs::read_to_string(path)?;
        let mut config: Config = serde_json::from_str(&config_data)?;
        config.clear_ids();
        Ok(config)
    }

    pub fn save(&mut self, path: &PathBuf) -> io::Result<()> {
        self.clear_ids();
        let config_data = serde_json::to_string_pretty(&self)?;
        fs::write(path, config_data)?;
        Ok(())
    }

    pub fn default_config() -> Self {
        Config::new(
            "iterm",
            "current",
            "Homebrew",
            "New tab",
            Some("Ctrl+Shift+M".parse().unwrap()),
            vec![
                CommandConfig {
                    id: None,
                    name: "Command".to_string(),
                    command: None,
                    inputs: None,
                    commands: None,
                    submenu: Some(vec![
                        CommandConfig {
                            id: None,
                            name: "Example Command".to_string(),
                            command: Some("echo Hello, world!".to_string()),
                            inputs: None,
                            commands: None,
                            submenu: None,
                            hotkey: Some("Ctrl+Shift+E".to_string()),
                        },
                        CommandConfig {
                            id: None,
                            name: "Example Multi-Command with input".to_string(),
                            command: None,
                            submenu: None,
                            hotkey: Some("Ctrl+Shift+M".to_string()),
                            commands: Some(vec![
                                "export MY_VAR=$(echo 'Step 1: [key1]')".to_string(),
                                "RESULT=$(echo 'Step 2: [key2]' && echo $MY_VAR)".to_string(),
                                "echo Step 3: Finalize && echo $RESULT".to_string(),
                            ]),
                            inputs: Some(HashMap::from([
                                ("key1".to_string(), "default1".to_string()),
                                ("key2".to_string(), "default2".to_string())
                            ])),
                        },
                        CommandConfig {
                            id: None,
                            name: "Example Submenu".to_string(),
                            inputs: None,
                            command: None,
                            commands: None,
                            hotkey: None,
                            submenu: Some(vec![
                                CommandConfig {
                                    id: None,
                                    name: "Subcommand 1".to_string(),
                                    inputs: None,
                                    command: Some("echo Subcommand 1".to_string()),
                                    commands: None,
                                    submenu: None,
                                    hotkey: Some("Ctrl+Shift+S".to_string()),
                                },
                                CommandConfig {
                                    id: None,
                                    name: "Subcommand 2".to_string(),
                                    inputs: None,
                                    command: Some("echo Subcommand 2".to_string()),
                                    commands: None,
                                    submenu: None,
                                    hotkey: None,
                                },
                            ]),
                        },
                    ]),
                    hotkey: None,
                },
            ],
        )
    }

    pub fn ensure_default(path: &PathBuf) -> io::Result<()> {
        if !path.exists() {
            let mut config = Config::default_config();
            config.save(path)?;
        }
        Ok(())
    }

    pub fn assign_ids(&mut self, counter: &Arc<AtomicUsize>) {
        for command in &mut self.commands {
            command.assign_id(counter);
        }
    }

    fn find_command_by_id(&self, id: &str) -> Option<&CommandConfig> {
        for command in &self.commands {
            if let Some(found) = command.find_by_id(id) {
                return Some(found);
            }
        }
        None
    }

    fn clear_ids(&mut self) {
        for command in &mut self.commands {
            command.clear_id();
        }
    }
}

impl CommandConfig {
    fn assign_id(&mut self, counter: &Arc<AtomicUsize>) {
        self.id = Some(format!("cmd_{}", counter.fetch_add(1, Ordering::SeqCst)));

        if let Some(submenu) = &mut self.submenu {
            for command in submenu {
                command.assign_id(counter);
            }
        }
    }

    fn clear_id(&mut self) {
        self.id = None;

        if let Some(submenu) = &mut self.submenu {
            for command in submenu {
                command.clear_id();
            }
        }
    }

    fn find_by_id(&self, id: &str) -> Option<&CommandConfig> {
        if self.id.as_deref() == Some(id) {
            return Some(self);
        }

        if let Some(submenu) = &self.submenu {
            for command in submenu {
                if let Some(found) = command.find_by_id(id) {
                    return Some(found);
                }
            }
        }

        None
    }
}

pub fn get_config_path() -> PathBuf {
    let mut config_path = dirs::config_dir().unwrap();
    config_path.push("switch-shuttle");
    fs::create_dir_all(&config_path).unwrap();
    config_path.push("config.json");
    config_path
}
