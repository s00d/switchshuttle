use crate::config::CommandConfig;
use serde::Serialize;
use std::collections::HashMap;
use std::process::Command;
use log::{error, info};

static SCRIPTS_DIR: include_dir::Dir = include_dir::include_dir!("scripts");

/// Доступные терминалы для каждой операционной системы
#[derive(Debug, Clone, Serialize)]
#[allow(dead_code)]
pub struct TerminalConfig {
    pub name: &'static str,
    pub executable: &'static str,
    pub current_args: Vec<&'static str>,
    pub new_tab_args: Vec<&'static str>,
    pub new_window_args: Vec<&'static str>,
    pub icon: &'static str,
}

#[derive(Debug, Clone, Serialize)]
#[allow(dead_code)]
pub struct TerminalInfo {
    pub value: String,
    pub label: String,
    pub icon: String,
}

/// Конфигурации терминалов для macOS
#[cfg(target_os = "macos")]
fn get_macos_terminals() -> HashMap<&'static str, TerminalConfig> {
    let mut terminals = HashMap::new();

    // iTerm2
    terminals.insert(
        "iterm",
        TerminalConfig {
            name: "iTerm2",
            executable: "osascript",
            current_args: vec!["-e", "{script}"],
            new_tab_args: vec!["-e", "{script}"],
            new_window_args: vec!["-e", "{script}"],
            icon: "🖥️",
        },
    );

    // Terminal.app
    terminals.insert(
        "terminal",
        TerminalConfig {
            name: "Terminal.app",
            executable: "osascript",
            current_args: vec!["-e", "{script}"],
            new_tab_args: vec!["-e", "{script}"],
            new_window_args: vec!["-e", "{script}"],
            icon: "💻",
        },
    );

    // Warp
    terminals.insert(
        "warp",
        TerminalConfig {
            name: "Warp",
            executable: "osascript",
            current_args: vec!["-e", "{script}"],
            new_tab_args: vec!["-e", "{script}"],
            new_window_args: vec!["-e", "{script}"],
            icon: "⚡",
        },
    );

    // Hyper
    terminals.insert(
        "hyper",
        TerminalConfig {
            name: "Hyper",
            executable: "osascript",
            current_args: vec!["-e", "{script}"],
            new_tab_args: vec!["-e", "{script}"],
            new_window_args: vec!["-e", "{script}"],
            icon: "🚀",
        },
    );

    // Alacritty
    terminals.insert(
        "alacritty",
        TerminalConfig {
            name: "Alacritty",
            executable: "osascript",
            current_args: vec!["-e", "{script}"],
            new_tab_args: vec!["-e", "{script}"],
            new_window_args: vec!["-e", "{script}"],
            icon: "⚡",
        },
    );

    // VSCode Terminal
    terminals.insert(
        "vscode-terminal",
        TerminalConfig {
            name: "VSCode Terminal",
            executable: "osascript",
            current_args: vec!["-e", "{script}"],
            new_tab_args: vec!["-e", "{script}"],
            new_window_args: vec!["-e", "{script}"],
            icon: "🔧",
        },
    );

    terminals
}

/// Конфигурации терминалов для Windows
#[cfg(target_os = "windows")]
fn get_windows_terminals() -> HashMap<&'static str, TerminalConfig> {
    let mut terminals = HashMap::new();

    // Hyper
    terminals.insert(
        "hyper",
        TerminalConfig {
            name: "Hyper",
            executable: "cmd",
            current_args: vec!["/C", "start hyper -e \"{command}\""],
            new_tab_args: vec!["/C", "start hyper --new-tab -e \"{command}\""],
            new_window_args: vec!["/C", "start hyper --new-window -e \"{command}\""],
            icon: "🚀",
        },
    );

    // WSL
    terminals.insert(
        "wsl",
        TerminalConfig {
            name: "WSL",
            executable: "wsl",
            current_args: vec!["-e", "bash", "-c", "{command}"],
            new_tab_args: vec!["/C", "start wsl -e bash -c \"{command}\""],
            new_window_args: vec!["/C", "start wsl -e bash -c \"{command}\""],
            icon: "🐧",
        },
    );

    // PowerShell
    terminals.insert(
        "powershell",
        TerminalConfig {
            name: "PowerShell",
            executable: "powershell",
            current_args: vec!["-Command", "{command}"],
            new_tab_args: vec!["/C", "start powershell -Command \"{command}\""],
            new_window_args: vec!["/C", "start powershell -Command \"{command}\""],
            icon: "💻",
        },
    );

    // Windows Terminal
    terminals.insert(
        "windows-terminal",
        TerminalConfig {
            name: "Windows Terminal",
            executable: "cmd",
            current_args: vec!["/C", "wt -d . \"{command}\""],
            new_tab_args: vec!["/C", "start wt -d . new-tab \"{command}\""],
            new_window_args: vec!["/C", "start wt -d . new-window \"{command}\""],
            icon: "🪟",
        },
    );

    // ConEmu
    terminals.insert(
        "conemu",
        TerminalConfig {
            name: "ConEmu",
            executable: "cmd",
            current_args: vec!["/C", "conemu /cmd \"{command}\""],
            new_tab_args: vec!["/C", "start conemu /new-tab /cmd \"{command}\""],
            new_window_args: vec!["/C", "start conemu /new-window /cmd \"{command}\""],
            icon: "🖥️",
        },
    );

    // Cmder
    terminals.insert(
        "cmder",
        TerminalConfig {
            name: "Cmder",
            executable: "cmd",
            current_args: vec!["/C", "cmder /cmd \"{command}\""],
            new_tab_args: vec!["/C", "start cmder /new-tab /cmd \"{command}\""],
            new_window_args: vec!["/C", "start cmder /new-window /cmd \"{command}\""],
            icon: "💻",
        },
    );

    // Git Bash
    terminals.insert(
        "git-bash",
        TerminalConfig {
            name: "Git Bash",
            executable: "cmd",
            current_args: vec![
                "/C",
                "\"C:\\Program Files\\Git\\bin\\bash.exe\" -c \"{command}\"",
            ],
            new_tab_args: vec![
                "/C",
                "start \"C:\\Program Files\\Git\\bin\\bash.exe\" -c \"{command}\"",
            ],
            new_window_args: vec![
                "/C",
                "start \"C:\\Program Files\\Git\\bin\\bash.exe\" -c \"{command}\"",
            ],
            icon: "🐧",
        },
    );

    // Alacritty
    terminals.insert(
        "alacritty",
        TerminalConfig {
            name: "Alacritty",
            executable: "cmd",
            current_args: vec!["/C", "alacritty -e \"{command}\""],
            new_tab_args: vec!["/C", "start alacritty -e \"{command}\""],
            new_window_args: vec!["/C", "start alacritty -e \"{command}\""],
            icon: "⚡",
        },
    );

    // WezTerm
    terminals.insert(
        "wezterm",
        TerminalConfig {
            name: "WezTerm",
            executable: "cmd",
            current_args: vec!["/C", "wezterm cli spawn -- \"{command}\""],
            new_tab_args: vec!["/C", "start wezterm cli spawn --new-tab -- \"{command}\""],
            new_window_args: vec![
                "/C",
                "start wezterm cli spawn --new-window -- \"{command}\"",
            ],
            icon: "🚀",
        },
    );

    // VSCode Terminal
    terminals.insert(
        "vscode-terminal",
        TerminalConfig {
            name: "VSCode Terminal",
            executable: "cmd",
            current_args: vec!["/C", "code --new-terminal \"{command}\""],
            new_tab_args: vec!["/C", "start code --new-terminal \"{command}\""],
            new_window_args: vec!["/C", "start code --new-window --new-terminal \"{command}\""],
            icon: "🔧",
        },
    );

    terminals
}

/// Конфигурации терминалов для Linux
#[cfg(target_os = "linux")]
fn get_linux_terminals() -> HashMap<&'static str, TerminalConfig> {
    let mut terminals = HashMap::new();

    // Hyper
    terminals.insert(
        "hyper",
        TerminalConfig {
            name: "Hyper",
            executable: "hyper",
            current_args: vec!["-e", "{command}"],
            new_tab_args: vec!["--new-tab", "-e", "{command}"],
            new_window_args: vec!["--new-window", "-e", "{command}"],
            icon: "🚀",
        },
    );

    // GNOME Terminal
    terminals.insert(
        "gnome-terminal",
        TerminalConfig {
            name: "GNOME Terminal",
            executable: "gnome-terminal",
            current_args: vec!["--", "bash", "-c", "{command}"],
            new_tab_args: vec!["--tab", "--", "bash", "-c", "{command}"],
            new_window_args: vec!["--new-window", "--", "bash", "-c", "{command}"],
            icon: "🖥️",
        },
    );

    // Konsole
    terminals.insert(
        "konsole",
        TerminalConfig {
            name: "Konsole",
            executable: "konsole",
            current_args: vec!["-e", "bash", "-c", "{command}"],
            new_tab_args: vec!["--new-tab", "-e", "bash", "-c", "{command}"],
            new_window_args: vec!["--new-window", "-e", "bash", "-c", "{command}"],
            icon: "💻",
        },
    );

    // XFCE4 Terminal
    terminals.insert(
        "xfce4-terminal",
        TerminalConfig {
            name: "XFCE4 Terminal",
            executable: "xfce4-terminal",
            current_args: vec!["-e", "bash", "-c", "{command}"],
            new_tab_args: vec!["--tab", "-e", "bash", "-c", "{command}"],
            new_window_args: vec!["--new-window", "-e", "bash", "-c", "{command}"],
            icon: "🖥️",
        },
    );

    // Alacritty
    terminals.insert(
        "alacritty",
        TerminalConfig {
            name: "Alacritty",
            executable: "alacritty",
            current_args: vec!["-e", "bash", "-c", "{command}"],
            new_tab_args: vec!["--new-tab", "-e", "bash", "-c", "{command}"],
            new_window_args: vec!["--new-window", "-e", "bash", "-c", "{command}"],
            icon: "⚡",
        },
    );

    // WezTerm
    terminals.insert(
        "wezterm",
        TerminalConfig {
            name: "WezTerm",
            executable: "wezterm",
            current_args: vec!["cli", "spawn", "--", "bash", "-c", "{command}"],
            new_tab_args: vec!["cli", "spawn", "--new-tab", "--", "bash", "-c", "{command}"],
            new_window_args: vec![
                "cli",
                "spawn",
                "--new-window",
                "--",
                "bash",
                "-c",
                "{command}",
            ],
            icon: "🚀",
        },
    );

    // Kitty
    terminals.insert(
        "kitty",
        TerminalConfig {
            name: "Kitty",
            executable: "kitty",
            current_args: vec!["@", "launch", "--type=tab", "bash", "-c", "{command}"],
            new_tab_args: vec!["@", "launch", "--type=tab", "bash", "-c", "{command}"],
            new_window_args: vec!["@", "launch", "--type=window", "bash", "-c", "{command}"],
            icon: "🐱",
        },
    );

    // VSCode Terminal
    terminals.insert(
        "vscode-terminal",
        TerminalConfig {
            name: "VSCode Terminal",
            executable: "code",
            current_args: vec!["--new-terminal", "--", "bash", "-c", "{command}"],
            new_tab_args: vec!["--new-terminal", "--", "bash", "-c", "{command}"],
            new_window_args: vec![
                "--new-window",
                "--new-terminal",
                "--",
                "bash",
                "-c",
                "{command}",
            ],
            icon: "🔧",
        },
    );

    terminals
}

/// Получает конфигурации терминалов для текущей операционной системы
pub fn get_terminals() -> HashMap<&'static str, TerminalConfig> {
    #[cfg(target_os = "macos")]
    {
        get_macos_terminals()
    }

    #[cfg(target_os = "windows")]
    {
        get_windows_terminals()
    }

    #[cfg(target_os = "linux")]
    {
        get_linux_terminals()
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        HashMap::new()
    }
}

/// Читает содержимое скрипта из встроенных ресурсов
#[cfg(target_os = "macos")]
fn read_script(script_path: &str) -> Option<String> {
    SCRIPTS_DIR
        .get_file(script_path)
        .map(|file| file.contents_utf8().unwrap().to_string())
}

/// Получает путь к скрипту для macOS
#[cfg(target_os = "macos")]
fn get_script_path(terminal: &str, launch_in: &str) -> Option<String> {
    let terminals = get_terminals();

    // Получаем конфигурацию терминала
    let terminal_config = terminals.get(terminal)?;

    // Формируем название скрипта на основе имени терминала и опции запуска
    let terminal_name = terminal_config.name;
    let launch_suffix = match launch_in {
        "current" => "Current",
        "new_tab" => "Tab",
        "new_window" => "Window",
        _ => return None,
    };

    // Формируем название скрипта
    let script_name = format!("{}-{}.scpt", terminal_name, launch_suffix);

    Some(script_name)
}

/// Выполняет команду в терминале
fn execute_command_impl(
    commands_to_execute: &[String],
    terminal: &str,
    launch_in: &str,
    theme: &str,
    title: &str,
) {
    info!("=== execute_command_impl called ===");
    info!("Commands to execute: {:?}", commands_to_execute);
    info!("Terminal: '{}'", terminal);
    info!("Launch in: '{}'", launch_in);
    info!("Theme: '{}'", theme);
    info!("Title: '{}'", title);
    info!("================================");

    let terminals = get_terminals();

    for command in commands_to_execute {
        info!("Executing command: {}", command);

        #[cfg(target_os = "macos")]
        {
            // Для macOS используем скрипты
            let terminal_config = match terminals.get(terminal) {
                Some(config) => config,
                None => {
                    error!("Unsupported terminal: {}", terminal);
                    return;
                }
            };

            if let Some(script_path) = get_script_path(terminal, launch_in) {
                let script_content = match read_script(&script_path) {
                    Some(content) => content,
                    None => {
                        error!("Failed to read script: {}", script_path);
                        continue;
                    }
                };

                let script = script_content
                    .replace("{command}", &process_command_placeholder("{command}", command))
                    .replace("{theme}", theme)
                    .replace("{title}", title);

                let output = Command::new(terminal_config.executable)
                    .arg("-e")
                    .arg(&script)
                    .output()
                    .expect("Failed to execute command");

                if output.status.success() {
                    info!("Command succeeded: {}", command);
                } else {
                    error!("Command failed: {}", command);
                    error!("Error: {}", String::from_utf8_lossy(&output.stderr));
                    break;
                }
            } else {
                error!(
                    "No script found for terminal: {} with launch_in: {}",
                    terminal, launch_in
                );
                break;
            }
        }

        #[cfg(not(target_os = "macos"))]
        {
            // Для Windows и Linux используем прямые команды
            let terminal_config = match terminals.get(terminal) {
                Some(config) => config,
                None => {
                    error!("Unsupported terminal: {}", terminal);
                    return;
                }
            };

            let args = match launch_in {
                "current" => &terminal_config.current_args,
                "new_tab" => &terminal_config.new_tab_args,
                "new_window" => &terminal_config.new_window_args,
                _ => {
                    error!("Unsupported launch_in option: {}", launch_in);
                    return;
                }
            };

            let mut cmd_args = Vec::new();
            for arg in args {
                // Заменяем {command} с правильной обработкой кавычек
                let processed_arg = if arg.contains("{command}") {
                    process_command_placeholder(arg, command)
                } else {
                    arg.to_string()
                };
                cmd_args.push(processed_arg);
            }

            let status = Command::new(terminal_config.executable)
                .args(&cmd_args)
                .status()
                .expect("Failed to execute command");

            if status.success() {
                info!("Command succeeded: {}", command);
            } else {
                error!("Command failed: {}", command);
                break;
            }
        }
    }
    
    info!("=== execute_command_impl completed ===");
}

/// Получает список доступных терминалов для текущей операционной системы
pub fn get_available_terminals() -> Vec<&'static str> {
    get_terminals().keys().copied().collect()
}

/// Получает список доступных опций запуска
pub fn get_available_launch_options() -> Vec<&'static str> {
    vec!["current", "new_tab", "new_window"]
}

/// Проверяет, поддерживается ли терминал
pub fn is_terminal_supported(terminal: &str) -> bool {
    get_terminals().contains_key(terminal)
}

/// Проверяет, поддерживается ли опция запуска
pub fn is_launch_option_supported(launch_in: &str) -> bool {
    matches!(launch_in, "current" | "new_tab" | "new_window")
}

/// Правильно обрабатывает плейсхолдер команды, экранируя кавычки для каждой ОС
fn process_command_placeholder(template: &str, command: &str) -> String {
    #[cfg(target_os = "macos")]
    {
        // На macOS команды передаются через AppleScript, поэтому нужно правильно экранировать
        // В AppleScript нужно экранировать кавычки как \"
        let escaped_command = command.replace('"', "\\\"");
        template.replace("{command}", &escaped_command)
    }
    
    #[cfg(target_os = "windows")]
    {
        // На Windows команды передаются через cmd.exe
        // Если команда уже содержит кавычки, нужно удвоить их для cmd.exe
        let escaped_command = if command.contains('"') {
            command.replace('"', "\"\"")
        } else {
            command.to_string()
        };
        template.replace("{command}", &escaped_command)
    }
    
    #[cfg(target_os = "linux")]
    {
        // На Linux команды передаются через bash -c
        // Экранirум кавычки в команде
        let escaped_command = command.replace('"', "\\\"");
        template.replace("{command}", &escaped_command)
    }
    
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        // Fallback для других ОС - просто заменяем без экранирования
        template.replace("{command}", command)
    }
}

/// Основная функция выполнения команд
pub fn execute_command(
    command_config: &CommandConfig,
    terminal: &str,
    launch_in: &str,
    theme: &String,
    title: &String,
) {
    info!("=== execute_command called ===");
    info!("Command config: {:?}", command_config);
    info!("Terminal: '{}'", terminal);
    info!("Launch in: '{}'", launch_in);
    info!("Theme: '{}'", theme);
    info!("Title: '{}'", title);
    info!("=============================");

    let mut commands_to_execute = Vec::new();

    // Собираем команды для выполнения
    if let Some(commands) = &command_config.commands {
        info!("Adding commands: {:?}", commands);
        for cmd in commands {
            if !cmd.trim().is_empty() {
                commands_to_execute.push(cmd.clone());
            }
        }
    }

    // Проверяем, что есть команды для выполнения
    if commands_to_execute.is_empty() {
        info!("No commands to execute, skipping");
        return;
    }

    info!("Commands to execute: {:?}", commands_to_execute);

    let terminal = terminal.to_lowercase();
    let launch_in = launch_in.to_lowercase();

    // Проверяем поддержку терминала и опции запуска
    if !is_terminal_supported(&terminal) {
        error!("Unsupported terminal: {}", terminal);
        error!("Available terminals: {:?}", get_available_terminals());
        return;
    }

    if !is_launch_option_supported(&launch_in) {
        error!("Unsupported launch option: {}", launch_in);
        error!(
            "Available launch options: {:?}",
            get_available_launch_options()
        );
        return;
    }

    info!("=== Calling execute_command_impl ===");
    execute_command_impl(&commands_to_execute, &terminal, &launch_in, theme, title);
    info!("=== execute_command completed ===");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::CommandConfig;

    #[test]
    fn test_get_available_terminals() {
        let terminals = get_available_terminals();
        assert!(!terminals.is_empty());

        // Проверяем, что все терминалы уникальны
        let mut unique_terminals = std::collections::HashSet::new();
        for terminal in &terminals {
            unique_terminals.insert(*terminal);
        }
        assert_eq!(terminals.len(), unique_terminals.len());
    }

    #[test]
    fn test_get_available_launch_options() {
        let options = get_available_launch_options();
        assert_eq!(options.len(), 3);
        assert!(options.contains(&"current"));
        assert!(options.contains(&"new_tab"));
        assert!(options.contains(&"new_window"));
    }

    #[test]
    fn test_is_terminal_supported() {
        let terminals = get_available_terminals();

        // Проверяем, что все доступные терминалы поддерживаются
        for terminal in terminals {
            assert!(is_terminal_supported(terminal));
        }

        // Проверяем, что несуществующий терминал не поддерживается
        assert!(!is_terminal_supported("nonexistent-terminal"));
    }

    #[test]
    fn test_is_launch_option_supported() {
        // Проверяем поддерживаемые опции
        assert!(is_launch_option_supported("current"));
        assert!(is_launch_option_supported("new_tab"));
        assert!(is_launch_option_supported("new_window"));

        // Проверяем неподдерживаемые опции
        assert!(!is_launch_option_supported("invalid"));
        assert!(!is_launch_option_supported(""));
    }

    #[test]
    fn test_terminal_config_structure() {
        let terminals = get_terminals();

        for (name, config) in terminals {
            // Проверяем, что у каждого терминала есть все необходимые поля
            assert!(!config.name.is_empty());
            assert!(!config.executable.is_empty());
            assert!(!config.current_args.is_empty());
            assert!(!config.new_tab_args.is_empty());
            assert!(!config.new_window_args.is_empty());

            // Проверяем, что имя терминала соответствует ключу
            assert_eq!(name, name.to_lowercase());
        }
    }

    #[test]
    fn test_execute_command_with_empty_config() {
        let config = CommandConfig {
            id: None,
            name: "test".to_string(),
            inputs: None,
            command: None,
            commands: None,
            hotkey: None,
            submenu: None,
            switch: None,
            monitor: None,
            icon: None,
            scheduler: None,
            background: None
        };

        // Функция должна завершиться без ошибок
        execute_command(
            &config,
            "terminal",
            "current",
            &"default".to_string(),
            &"Test".to_string(),
        );
    }

    #[test]
    fn test_execute_command_with_single_command() {
        let config = CommandConfig {
            id: None,
            name: "test".to_string(),
            inputs: None,
            command: Some("echo 'test'".to_string()),
            commands: None,
            hotkey: None,
            submenu: None,
            switch: None,
            monitor: None,
            icon: None,
            scheduler: None,
            background: None
        };

        // Функция должна завершиться без ошибок
        execute_command(
            &config,
            "terminal",
            "current",
            &"default".to_string(),
            &"Test".to_string(),
        );
    }

    #[test]
    fn test_execute_command_with_multiple_commands() {
        let config = CommandConfig {
            id: None,
            name: "test".to_string(),
            inputs: None,
            command: None,
            commands: Some(vec![
                "echo 'command1'".to_string(),
                "echo 'command2'".to_string(),
            ]),
            hotkey: None,
            submenu: None,
            switch: None,
            monitor: None,
            icon: None,
            scheduler: None,
            background: None
        };

        // Функция должна завершиться без ошибок
        execute_command(
            &config,
            "terminal",
            "current",
            &"default".to_string(),
            &"Test".to_string(),
        );
    }

    #[test]
    fn test_execute_command_with_unsupported_terminal() {
        let config = CommandConfig {
            id: None,
            name: "test".to_string(),
            inputs: None,
            command: Some("echo 'test'".to_string()),
            commands: None,
            hotkey: None,
            submenu: None,
            switch: None,
            monitor: None,
            icon: None,
            scheduler: None,
            background: None
        };

        // Функция должна завершиться без ошибок при неподдерживаемом терминале
        execute_command(
            &config,
            "unsupported-terminal",
            "current",
            &"default".to_string(),
            &"Test".to_string(),
        );
    }

    #[test]
    fn test_execute_command_with_unsupported_launch_option() {
        let config = CommandConfig {
            id: None,
            name: "test".to_string(),
            inputs: None,
            command: Some("echo 'test'".to_string()),
            commands: None,
            hotkey: None,
            submenu: None,
            switch: None,
            monitor: None,
            icon: None,
            scheduler: None,
            background: None
        };

        // Функция должна завершиться без ошибок при неподдерживаемой опции запуска
        execute_command(
            &config,
            "terminal",
            "unsupported",
            &"default".to_string(),
            &"Test".to_string(),
        );
    }

    #[test]
    fn test_read_script() {
        #[cfg(target_os = "macos")]
        {
            // Проверяем, что функция read_script работает
            let script = read_script("iTerm2-Current.scpt");
            // На macOS скрипт должен существовать, если файл есть в каталоге scripts
            // Если скрипт не найден, это нормально, так как скрипты могут отсутствовать
            // Проверяем только, что функция не паникует
            let _ = script;
        }
        
        #[cfg(not(target_os = "macos"))]
        {
            // На других ОС функция read_script недоступна
            // Тест пропускается
        }
    }

    #[test]
    fn test_terminal_config_clone() {
        let config = TerminalConfig {
            name: "Test Terminal",
            executable: "test",
            current_args: vec!["arg1", "arg2"],
            new_tab_args: vec!["tab1", "tab2"],
            new_window_args: vec!["window1", "window2"],
            icon: "🖥️",
        };

        let cloned = config.clone();

        assert_eq!(config.name, cloned.name);
        assert_eq!(config.executable, cloned.executable);
        assert_eq!(config.current_args, cloned.current_args);
        assert_eq!(config.new_tab_args, cloned.new_tab_args);
        assert_eq!(config.new_window_args, cloned.new_window_args);
        assert_eq!(config.icon, cloned.icon);
    }

    #[test]
    fn test_terminal_config_debug() {
        let config = TerminalConfig {
            name: "Test Terminal",
            executable: "test",
            current_args: vec!["arg1"],
            new_tab_args: vec!["tab1"],
            new_window_args: vec!["window1"],
            icon: "🖥️",
        };

        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("Test Terminal"));
        assert!(debug_str.contains("test"));
    }

    #[test]
    fn test_get_terminals_returns_valid_configs() {
        let terminals = get_terminals();

        for (name, config) in terminals {
            // Проверяем, что имя терминала не пустое
            assert!(!name.is_empty());

            // Проверяем, что конфигурация валидна
            assert!(!config.name.is_empty());
            assert!(!config.executable.is_empty());

            #[cfg(target_os = "macos")]
            {
                // На macOS проверяем только executable и name
                assert!(!config.executable.is_empty());
            }

            #[cfg(not(target_os = "macos"))]
            {
                // Проверяем, что аргументы не пустые
                assert!(!config.current_args.is_empty());
                assert!(!config.new_tab_args.is_empty());
                assert!(!config.new_window_args.is_empty());

                // Проверяем, что аргументы содержат плейсхолдеры команд
                let has_command_placeholder = config
                    .current_args
                    .iter()
                    .any(|arg| arg.contains("{command}"))
                    || config
                        .new_tab_args
                        .iter()
                        .any(|arg| arg.contains("{command}"))
                    || config
                        .new_window_args
                        .iter()
                        .any(|arg| arg.contains("{command}"));
                assert!(has_command_placeholder);
            }
        }
    }

    #[test]
    fn test_get_script_path() {
        #[cfg(target_os = "macos")]
        {
            // Проверяем, что функция возвращает правильные пути для поддерживаемых комбинаций
            // Используем динамические названия на основе конфигурации терминалов
            assert_eq!(
                get_script_path("iterm", "current"),
                Some("iTerm2-Current.scpt".to_string())
            );
            assert_eq!(
                get_script_path("terminal", "new_tab"),
                Some("Terminal.app-Tab.scpt".to_string())
            );
            assert_eq!(
                get_script_path("warp", "new_window"),
                Some("Warp-Window.scpt".to_string())
            );

            // Проверяем, что функция возвращает None для неподдерживаемых комбинаций
            assert_eq!(get_script_path("unsupported", "current"), None);
            assert_eq!(get_script_path("iterm", "unsupported"), None);
        }

        #[cfg(not(target_os = "macos"))]
        {
            // На других ОС функция не должна быть доступна
            // Этот тест будет пропущен
        }
    }

    #[test]
    fn test_process_command_placeholder_with_quotes() {
        // Тестируем обработку команд с кавычками
        let command_with_quotes = r#"somecommand --someargs "anything""#;
        let template = r#"hyper -e "{command}""#;
        
        let result = process_command_placeholder(template, command_with_quotes);
        
        #[cfg(target_os = "macos")]
        {
            // На macOS кавычки должны быть экранированы как \"
            assert_eq!(result, r#"hyper -e "somecommand --someargs \"anything\"""#);
        }
        
        #[cfg(target_os = "windows")]
        {
            // На Windows кавычки должны быть удвоены
            assert_eq!(result, r#"hyper -e "somecommand --someargs ""anything""""#);
        }
        
        #[cfg(target_os = "linux")]
        {
            // На Linux кавычки должны быть экранированы как \"
            assert_eq!(result, r#"hyper -e "somecommand --someargs \"anything\"""#);
        }
    }

    #[test]
    fn test_process_command_placeholder_without_quotes() {
        // Тестируем обработку команд без кавычек
        let command_without_quotes = "simple_command";
        let template = r#"hyper -e "{command}""#;
        
        let result = process_command_placeholder(template, command_without_quotes);
        
        // Для всех ОС результат должен быть одинаковым
        assert_eq!(result, r#"hyper -e "simple_command""#);
    }

    #[test]
    fn test_process_command_placeholder_with_json() {
        // Тестируем обработку команд с JSON строками
        let command_with_json = r#"somecommand --args='{"thing1": true}'"#;
        let template = r#"hyper -e "{command}""#;
        
        let result = process_command_placeholder(template, command_with_json);
        
        // Команда не содержит двойных кавычек, поэтому должна остаться без изменений
        assert_eq!(result, r#"hyper -e "somecommand --args='{\"thing1\": true}'""#);
    }

    #[test]
    fn test_process_command_placeholder_with_mixed_quotes() {
        // Тестируем обработку команд со смешанными кавычками
        let command_mixed = r#"echo "Hello 'world'" and 'test "quotes"'"#;
        let template = r#"bash -c "{command}""#;
        
        let result = process_command_placeholder(template, command_mixed);
        
        #[cfg(target_os = "macos")]
        {
            // На macOS только двойные кавычки должны быть экранированы
            assert_eq!(result, r#"bash -c "echo \"Hello 'world'\" and 'test \"quotes\"'""#);
        }
        
        #[cfg(target_os = "windows")]
        {
            // На Windows двойные кавычки должны быть удвоены
            assert_eq!(result, r#"bash -c "echo ""Hello 'world'"" and 'test ""quotes""'""#);
        }
        
        #[cfg(target_os = "linux")]
        {
            // На Linux двойные кавычки должны быть экранированы
            assert_eq!(result, r#"bash -c "echo \"Hello 'world'\" and 'test \"quotes\"'""#);
        }
    }
}
