use std::process::{Command, Stdio};
use std::io::{Write, BufRead, BufReader};
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use std::println;

/// Структура для управления постоянным инстансом консоли
pub struct ConsoleInstance {
    process: Option<std::process::Child>,
    stdin: Option<std::process::ChildStdin>,
    reader: Option<BufReader<std::process::ChildStdout>>,
}

impl ConsoleInstance {
    pub fn new() -> Result<Self, String> {
        println!("[Console] Creating new console instance...");
        
        if cfg!(target_os = "macos") {
            println!("[Console] Spawning bash on macOS...");
            let mut child = Command::new("/bin/bash")
                .arg("-i")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(|e| format!("Failed to spawn bash: {}", e))?;

            let stdin = child.stdin.take();
            let stdout = child.stdout.take();
            
            if let Some(stdout) = stdout {
                let reader = BufReader::new(stdout);
                println!("[Console] Bash console created successfully");
                Ok(ConsoleInstance {
                    process: Some(child),
                    stdin,
                    reader: Some(reader),
                })
            } else {
                println!("[Console] Failed to get stdout from bash");
                Err("Failed to get stdout".to_string())
            }
        } else if cfg!(target_os = "windows") {
            println!("[Console] Spawning cmd on Windows...");
            let mut child = Command::new("cmd")
                .arg("/K")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(|e| format!("Failed to spawn cmd: {}", e))?;

            let stdin = child.stdin.take();
            let stdout = child.stdout.take();
            
            if let Some(stdout) = stdout {
                let reader = BufReader::new(stdout);
                println!("[Console] CMD console created successfully");
                Ok(ConsoleInstance {
                    process: Some(child),
                    stdin,
                    reader: Some(reader),
                })
            } else {
                println!("[Console] Failed to get stdout from cmd");
                Err("Failed to get stdout".to_string())
            }
        } else if cfg!(target_os = "linux") {
            println!("[Console] Spawning bash on Linux...");
            let mut child = Command::new("/bin/bash")
                .arg("-i")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(|e| format!("Failed to spawn bash: {}", e))?;

            let stdin = child.stdin.take();
            let stdout = child.stdout.take();
            
            if let Some(stdout) = stdout {
                let reader = BufReader::new(stdout);
                println!("[Console] Bash console created successfully");
                Ok(ConsoleInstance {
                    process: Some(child),
                    stdin,
                    reader: Some(reader),
                })
            } else {
                println!("[Console] Failed to get stdout from bash");
                Err("Failed to get stdout".to_string())
            }
        } else {
            println!("[Console] Unsupported operating system");
            Err("Unsupported operating system".to_string())
        }
    }

    pub fn execute_command(&mut self, command: &str) -> Result<String, String> {
        println!("[Console] Executing command: {}", command);
        
        // Проверяем, что команда не пустая
        if command.trim().is_empty() {
            println!("[Console] Empty command provided, returning empty string");
            return Ok("".to_string());
        }
        
        if let Some(ref mut stdin) = self.stdin {
            // Отправляем команду в консоль
            println!("[Console] Sending command to stdin...");
            writeln!(stdin, "{}", command)
                .map_err(|e| format!("Failed to write command: {}", e))?;
            
                            // Читаем результат до появления промпта
                if let Some(ref mut reader) = self.reader {
                    let mut output = String::new();
                    let mut line = String::new();
                    
                    println!("[Console] Reading output...");
                    
                    // Читаем только одну строку результата
                    line.clear();
                    match reader.read_line(&mut line) {
                        Ok(0) => {
                            println!("[Console] EOF reached");
                        }
                        Ok(_) => {
                            output.push_str(&line);
                            println!("[Console] Result: {:?}", line.trim());
                        }
                        Err(e) => {
                            println!("[Console] Error reading line: {}", e);
                        }
                    }
                
                // Очищаем остальной вывод до промпта с таймаутом
                println!("[Console] Clearing remaining output...");
                
                // Просто ждем немного и не читаем больше - результат уже получен
                std::thread::sleep(std::time::Duration::from_millis(100));
                println!("[Console] Clearing skipped - result already obtained");
                
                // Обрабатываем результат (только одна строка)
                let result = output.trim().to_string();
                println!("[Console] Command completed. Result: {:?}", result);
                Ok(result)
            } else {
                println!("[Console] No reader available");
                Ok("".to_string())
            }
        } else {
            println!("[Console] No stdin available");
            Err("No stdin available".to_string())
        }
    }

    pub fn close(&mut self) {
        if let Some(ref mut stdin) = self.stdin {
            let _ = writeln!(stdin, "exit");
        }
        if let Some(ref mut process) = self.process {
            let _ = process.kill();
        }
    }
}

impl Drop for ConsoleInstance {
    fn drop(&mut self) {
        self.close();
    }
}

// Глобальный инстанс консоли
lazy_static! {
    static ref CONSOLE_INSTANCE: Arc<Mutex<Option<ConsoleInstance>>> = Arc::new(Mutex::new(None));
}

/// Инициализирует постоянный инстанс консоли
pub fn init_console() -> Result<(), String> {
    println!("[Console] Initializing console instance...");
    let mut console_guard = CONSOLE_INSTANCE.lock().unwrap();
    if console_guard.is_none() {
        println!("[Console] Creating new console instance...");
        *console_guard = Some(ConsoleInstance::new()?);
        println!("[Console] Console instance created successfully");
    } else {
        println!("[Console] Console instance already exists");
    }
    Ok(())
}

/// Закрывает постоянный инстанс консоли
// pub fn close_console() {
//     println!("[Console] Closing console instance...");
//     let mut console_guard = CONSOLE_INSTANCE.lock().unwrap();
//     if let Some(ref mut console) = *console_guard {
//         console.close();
//         println!("[Console] Console instance closed");
//     } else {
//         println!("[Console] No console instance to close");
//     }
//     *console_guard = None;
// }

/// Выполняет команду в постоянном инстансе консоли
pub fn execute_command_silent(command: &str) -> Result<String, String> {
    println!("[Console] execute_command_silent called with: {}", command);
    
    // Проверяем, что команда не пустая
    if command.trim().is_empty() {
        println!("[Console] Empty command provided, returning empty string");
        return Ok("".to_string());
    }
    
    // Получаем или создаем инстанс консоли
    let mut console_guard = CONSOLE_INSTANCE.lock().unwrap();
    
    if console_guard.is_none() {
        println!("[Console] No console instance found, creating new one...");
        *console_guard = Some(ConsoleInstance::new()?);
        println!("[Console] New console instance created");
    } else {
        println!("[Console] Using existing console instance");
    }
    
    if let Some(ref mut console) = *console_guard {
        // Выполняем команду в постоянном инстансе консоли
        println!("[Console] Executing command in console instance");
        let result = console.execute_command(command);
        println!("[Console] Command execution completed");
        result
    } else {
        println!("[Console] Failed to create console instance");
        Err("Failed to create console instance".to_string())
    }
} 