use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::{fs, io};
use tauri::{AppHandle, Wry};
use tauri_plugin_dialog::DialogExt;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub terminal: String,
    pub launch_in: String,
    pub theme: String,
    pub title: String,
    pub commands: Vec<CommandConfig>,
    pub menu_hotkey: Option<String>,
    pub enabled: Option<bool>,
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
    pub switch: Option<String>,
    pub monitor: Option<String>,
    pub icon: Option<String>,
    pub scheduler: Option<String>, // cron expression
    pub background: Option<bool>, // true = ConsolePool, false = execute, None = auto
}

#[derive(Clone, Debug)]
pub struct ConfigManager {
    pub configs: Vec<Config>,
    pub config_paths: Vec<PathBuf>,
    counter: Arc<AtomicUsize>,
}

impl ConfigManager {
    pub fn new() -> Self {
        ConfigManager {
            configs: Vec::new(),
            config_paths: Vec::new(),
            counter: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn load_configs(&mut self, app: Option<&AppHandle<Wry>>) -> io::Result<()> {
        self.counter = Arc::new(AtomicUsize::new(0));

        let config_path = get_config_path();
        let config_dir = config_path.parent().unwrap().to_path_buf();

        self.configs.clear();
        self.config_paths.clear();

        let mut has_configs = false;

        if let Ok(entries) = fs::read_dir(&config_dir) {
            let mut paths: Vec<_> = entries
                .filter_map(|entry| {
                    entry.ok().and_then(|e| {
                        let path = e.path();
                        if path.is_file()
                            && path.extension().and_then(|ext| ext.to_str()) == Some("json")
                        {
                            Some(path)
                        } else {
                            None
                        }
                    })
                })
                .collect();

            paths.sort();

            for path in paths {
                match Config::load(&path) {
                    Ok(mut config) => {
                        // Устанавливаем title равным имени файла (без расширения)
                        if let Some(file_name) = path.file_stem() {
                            if let Some(name) = file_name.to_str() {
                                config.title = name.to_string();
                            }
                        }

                        // Устанавливаем значение по умолчанию для enabled, если оно не задано
                        if config.enabled.is_none() {
                            config.enabled = Some(true);
                        }

                        config.assign_ids(&self.counter);
                        self.config_paths.push(path);
                        self.configs.push(config);
                        has_configs = true;
                    }
                    Err(err) => {
                        eprintln!("Failed to load config from {}: {}", path.display(), err);
                        if let Some(w) = app {
                            w.dialog()
                                .message(&format!(
                                    "Failed to parse config from {}: {}",
                                    path.display(),
                                    err
                                ))
                                .title("Error")
                                .show(|result| match result {
                                    true => {}
                                    false => {}
                                });
                        }
                    }
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
            // Пропускаем отключенные конфигурации
            if let Some(enabled) = config.enabled {
                if !enabled {
                    continue;
                }
            }

            if let Some(command) = config.find_command_by_id(id) {
                return Some((command, config));
            }
        }
        None
    }
}

impl Config {
    fn new(
        terminal: &str,
        launch_in: &str,
        theme: &str,
        title: &str,
        menu_hotkey: Option<String>,
        commands: Vec<CommandConfig>,
    ) -> Self {
        Config {
            terminal: terminal.to_string(),
            launch_in: launch_in.to_string(),
            theme: theme.to_string(),
            title: title.to_string(),
            commands,
            menu_hotkey,
            enabled: None,
        }
    }

    /// Migrates legacy command field to commands array
    fn migrate_command_to_commands(&mut self) {
        for command in &mut self.commands {
            command.migrate_command_to_commands();
        }
    }

    pub fn load(path: &PathBuf) -> io::Result<Self> {
        match fs::read_to_string(path) {
            Ok(content) => {
                let mut config: Config = serde_json::from_str(&content)?;
                config.migrate_command_to_commands();
                config.clear_ids();
                Ok(config)
            }
            Err(err) => {
                eprintln!("Failed to parse config from {}: {}", path.display(), err);
                Err(io::Error::new(io::ErrorKind::InvalidData, err))
            }
        }
    }

    pub fn save(&mut self, path: &PathBuf) -> io::Result<()> {
        self.clear_ids();
        let config_data = serde_json::to_string_pretty(&self)?;
        fs::write(path, config_data)?;
        Ok(())
    }

    pub fn default_config() -> Self {
        let mut config = Config::new(
            "iterm",
            "current",
            "Homebrew",
            "New tab",
            Some("Ctrl+Shift+M".to_string()),
                            vec![CommandConfig {
                    id: None,
                    name: "Command".to_string(),
                    command: None,
                    inputs: None,
                    commands: None,
                    switch: None,
                    monitor: None,
                    icon: None,
                    scheduler: None,
                    background: None,
                submenu: Some(vec![
                    CommandConfig {
                        id: None,
                        name: "Example Command".to_string(),
                        command: Some("echo Hello, world!".to_string()),
                        inputs: None,
                        commands: None,
                        submenu: None,
                        hotkey: Some("Ctrl+Shift+E".to_string()),
                        switch: None,
                        monitor: None,
                        icon: None,
                        scheduler: None,
                        background: None,
                    },
                    CommandConfig {
                        id: None,
                        name: "Example Multi-Command with input".to_string(),
                        command: None,
                        submenu: None,
                        hotkey: None,
                        commands: Some(vec![
                            "export MY_VAR=$(echo 'Step 1: [key1]')".to_string(),
                            "RESULT=$(echo 'Step 2: [key2]' && echo $MY_VAR)".to_string(),
                            "echo Step 3: Finalize && echo $RESULT".to_string(),
                        ]),
                        inputs: Some(HashMap::from([
                            ("key1".to_string(), "default1".to_string()),
                            ("key2".to_string(), "default2".to_string()),
                        ])),
                        switch: None,
                        monitor: None,
                        icon: None,
                        scheduler: None,
                        background: None,
                    },
                    CommandConfig {
                        id: None,
                        name: "Example Submenu".to_string(),
                        inputs: None,
                        command: None,
                        commands: None,
                        hotkey: None,
                        switch: None,
                        monitor: None,
                        icon: None,
                        scheduler: None,
                        background: None,
                        submenu: Some(vec![
                            CommandConfig {
                                id: None,
                                name: "Subcommand 1".to_string(),
                                inputs: None,
                                command: Some("echo Subcommand 1".to_string()),
                                commands: None,
                                submenu: None,
                                hotkey: Some("Ctrl+Shift+S".to_string()),
                                switch: None,
                                monitor: None,
                                icon: None,
                                scheduler: None,
                                background: None,
                            },
                            CommandConfig {
                                id: None,
                                name: "Subcommand 2".to_string(),
                                inputs: None,
                                command: Some("echo Subcommand 2".to_string()),
                                commands: None,
                                submenu: None,
                                hotkey: None,
                                switch: None,
                                monitor: None,
                                icon: None,
                                scheduler: None,
                                background: None,
                            },
                            CommandConfig {
                                id: None,
                                name: "Toggle Notifications".to_string(),
                                inputs: None,
                                command: Some("echo 'Notifications toggled'".to_string()),
                                commands: None,
                                submenu: None,
                                hotkey: None,
                                                                                        switch: Some("echo 'true'".to_string()),
                                monitor: None,
                                icon: None,
                                scheduler: None,
                                background: None,
                            },
                        ]),
                    },
                ]),
                hotkey: None,
            }],
        );
        config.enabled = Some(true);
        config
    }

    #[allow(dead_code)]
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

    pub fn clear_ids(&mut self) {
        for command in &mut self.commands {
            command.clear_id();
        }
    }
}

impl CommandConfig {
    /// Migrates legacy command field to commands array
    fn migrate_command_to_commands(&mut self) {
        // Migrate self
        if let Some(command) = self.command.take() {
            if !command.trim().is_empty() {
                println!("Warning: 'command' field is deprecated. Use 'commands' array instead.");
                if self.commands.is_none() {
                    self.commands = Some(vec![command]);
                } else if let Some(ref mut commands) = self.commands {
                    commands.insert(0, command);
                }
            }
        }

        // Migrate submenu items
        if let Some(ref mut submenu) = self.submenu {
            for item in submenu {
                item.migrate_command_to_commands();
            }
        }
    }

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
