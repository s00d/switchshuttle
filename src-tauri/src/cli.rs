use crate::config::ConfigManager;
use crate::helpers;
use std::sync::{Arc, Mutex};
use tauri_plugin_cli::Matches;

pub fn handle_cli_commands(matches: &Matches, config_manager: &Arc<Mutex<ConfigManager>>) -> bool {
    // Handle command execution
    if let Some(command) = matches.args.get("command") {
        if let Some(command_value) = command.value.as_str() {
            // Execute command by ID or name
            let config_manager = config_manager.lock().unwrap();
            if let Some((command_config, config)) = config_manager.find_command_by_id(command_value) {
                // Command found by ID
                helpers::execute_command(
                    command_config,
                    &config.terminal,
                    &config.launch_in,
                    &config.theme,
                    &config.title,
                );
            } else {
                // Try to find by name
                let mut found = false;
                for config in &config_manager.configs {
                    // Пропускаем отключенные конфигурации
                    if let Some(enabled) = config.enabled {
                        if !enabled {
                            continue;
                        }
                    }
                    
                    for cmd in &config.commands {
                        if cmd.name.to_lowercase() == command_value.to_lowercase() {
                            helpers::execute_command(
                                cmd,
                                &config.terminal,
                                &config.launch_in,
                                &config.theme,
                                &config.title,
                            );
                            found = true;
                            break;
                        }
                    }
                    if found {
                        break;
                    }
                }
                if !found {
                    eprintln!("Command '{}' not found", command_value);
                    std::process::exit(1);
                }
            }
            std::process::exit(0);
        }
    }
    
    // Handle list command
    if let Some(list) = matches.args.get("list") {
        if list.value == true {
            // List all commands
            let config_manager = config_manager.lock().unwrap();
            println!("Available commands:");
            for config in &config_manager.configs {
                // Пропускаем отключенные конфигурации
                if let Some(enabled) = config.enabled {
                    if !enabled {
                        continue;
                    }
                }
                
                for cmd in &config.commands {
                    if let Some(id) = &cmd.id {
                        println!("  {} (ID: {})", cmd.name, id);
                    } else {
                        println!("  {}", cmd.name);
                    }
                }
            }
            std::process::exit(0);
        }
    }
    
    // Handle search command
    if let Some(search) = matches.args.get("search") {
        if let Some(search_value) = search.value.as_str() {
            // Search commands by name
            let config_manager = config_manager.lock().unwrap();
            println!("Searching for commands containing '{}':", search_value);
            let mut found = false;
            for config in &config_manager.configs {
                // Пропускаем отключенные конфигурации
                if let Some(enabled) = config.enabled {
                    if !enabled {
                        continue;
                    }
                }
                
                for cmd in &config.commands {
                    if cmd.name.to_lowercase().contains(&search_value.to_lowercase()) {
                        if let Some(id) = &cmd.id {
                            println!("  {} (ID: {})", cmd.name, id);
                        } else {
                            println!("  {}", cmd.name);
                        }
                        found = true;
                    }
                }
            }
            if !found {
                println!("No commands found matching '{}'", search_value);
            }
            std::process::exit(0);
        }
    }
    
    // No CLI commands to handle
    false
} 