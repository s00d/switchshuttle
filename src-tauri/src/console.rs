use lazy_static::lazy_static;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use tauri::{AppHandle, Wry};
use tauri_plugin_notification::NotificationExt;

// Глобальный инстанс консоли
lazy_static! {
    static ref CONSOLE_INSTANCE: Arc<Mutex<Option<ConsoleInstance>>> = Arc::new(Mutex::new(None));
}

pub struct ConsoleInstance {
    process: Option<std::process::Child>,
    stdin_sender: Option<std::sync::mpsc::Sender<String>>, // Канал для отправки команд
    response: Arc<Mutex<Option<String>>>,                  // Переменная для хранения ответа
    stdout_thread: Option<JoinHandle<()>>,
    stderr_thread: Option<JoinHandle<()>>, // Отдельный поток для stderr
    stdin_thread: Option<JoinHandle<()>>,
}

impl ConsoleInstance {
    pub fn new() -> Result<Self, String> {
        println!("[Console] Creating new console instance...");

        let (stdin_tx, stdin_rx) = std::sync::mpsc::channel();
        let response = Arc::new(Mutex::new(None));

        let (stdout_thread, stderr_thread, stdin_thread, process) =
            Self::spawn_process(stdin_rx, response.clone())?;

        Ok(ConsoleInstance {
            process,
            stdin_sender: Some(stdin_tx),
            response,
            stdout_thread,
            stderr_thread,
            stdin_thread,
        })
    }

    fn spawn_process(
        stdin_rx: std::sync::mpsc::Receiver<String>,
        response: Arc<Mutex<Option<String>>>,
    ) -> Result<
        (
            Option<JoinHandle<()>>,
            Option<JoinHandle<()>>,
            Option<JoinHandle<()>>,
            Option<std::process::Child>,
        ),
        String,
    > {
        let (command, args) = if cfg!(target_os = "macos") {
            ("/bin/bash".to_string(), vec![]) // Убираем -i для неинтерактивного режима
        } else if cfg!(target_os = "windows") {
            ("cmd".to_string(), vec!["/K".to_string()])
        } else if cfg!(target_os = "linux") {
            ("/bin/bash".to_string(), vec![]) // Убираем -i для неинтерактивного режима
        } else {
            return Err("Unsupported operating system".to_string());
        };

        let mut child = Command::new(&command)
            .args(&args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to spawn {}: {}", command, e))?;

        let stdin = child.stdin.take();
        let stdout = child.stdout.take();
        let stderr = child.stderr.take();

        if let (Some(stdin), Some(stdout), Some(stderr)) = (stdin, stdout, stderr) {
            // Поток для записи команд в stdin
            let stdin_thread = thread::spawn(move || {
                let mut stdin_writer = stdin;
                println!("[Console] Stdin thread started");

                while let Ok(command) = stdin_rx.recv() {
                    println!("[Console] Stdin thread received command: {:?}", command);
                    if command == "exit" {
                        println!("[Console] Stdin thread received exit command");
                        break;
                    }
                    if let Err(e) = writeln!(stdin_writer, "{}", command) {
                        println!("[Console] Error writing to stdin: {}", e);
                        break;
                    }
                    println!("[Console] Stdin thread wrote command to process");
                }
                println!("[Console] Stdin thread finished");
            });

            // Клонируем response для потоков
            let response_stdout = response.clone();
            let response_stderr = response.clone();

            // Поток для чтения stdout
            let stdout_thread = thread::spawn(move || {
                let mut stdout_reader = BufReader::new(stdout);
                println!("[Console] Stdout thread started");
                loop {
                    // Читаем из stdout
                    let mut line = String::new();
                    match stdout_reader.read_line(&mut line) {
                        Ok(0) => {
                            // EOF - поток закрыт
                            println!("[Console] Stdout EOF reached");
                            break;
                        }
                        Ok(_) => {
                            if !line.trim().is_empty() {
                                println!("[Console] Got stdout: {:?}", line.trim());
                                *response_stdout.lock().unwrap() = Some(line.trim().to_string());
                            }
                        }
                        Err(e) => {
                            println!("[Console] Error reading stdout: {}", e);
                            break;
                        }
                    }

                    // Небольшая пауза
                    std::thread::sleep(Duration::from_millis(10));
                }
                println!("[Console] Stdout thread finished");
            });

            // Поток для чтения stderr
            let stderr_thread = thread::spawn(move || {
                let mut stderr_reader = BufReader::new(stderr);
                println!("[Console] Stderr thread started");

                loop {
                    // Читаем из stderr
                    let mut err_line = String::new();
                    match stderr_reader.read_line(&mut err_line) {
                        Ok(0) => {
                            // EOF - поток закрыт
                            println!("[Console] Stderr EOF reached");
                            break;
                        }
                        Ok(_) => {
                            if !err_line.trim().is_empty() {
                                // Фильтруем эхо команды и промпт bash
                                let trimmed = err_line.trim();
                                if !trimmed.contains("bash-")
                                    && !trimmed.contains("$")
                                    && !trimmed.contains("PS1")
                                {
                                    println!("[Console] Got stderr: {:?}", trimmed);
                                    *response_stderr.lock().unwrap() =
                                        Some(format!("stderr: {}", trimmed));
                                } else {
                                    println!(
                                        "[Console] Filtered stderr (echo/prompt): {:?}",
                                        trimmed
                                    );
                                }
                            }
                        }
                        Err(e) => {
                            println!("[Console] Error reading stderr: {}", e);
                            break;
                        }
                    }

                    // Небольшая пауза
                    std::thread::sleep(Duration::from_millis(10));
                }
                println!("[Console] Stderr thread finished");
            });

            Ok((
                Some(stdout_thread),
                Some(stderr_thread),
                Some(stdin_thread),
                Some(child),
            ))
        } else {
            Err("Failed to get stdin/stdout/stderr".to_string())
        }
    }

    pub fn execute_command(&mut self, command: &str) -> Result<String, String> {
        println!("[Console] Executing command: {}", command);
        if command.trim().is_empty() {
            return Ok("".to_string());
        }

        // Проверяем состояние процесса
        if let Some(ref mut process) = self.process {
            match process.try_wait() {
                Ok(Some(exit_status)) => {
                    println!(
                        "[Console] ERROR: Process has exited with status: {:?}",
                        exit_status
                    );
                    return Err("Process has exited".to_string());
                }
                Ok(None) => {
                    println!("[Console] Process is still running");
                }
                Err(e) => {
                    println!("[Console] ERROR: Failed to check process status: {}", e);
                    return Err("Failed to check process status".to_string());
                }
            }
        } else {
            println!("[Console] ERROR: No process available");
            return Err("No process available".to_string());
        }

        // Проверяем состояние потоков
        if let Some(ref _stdin_sender) = self.stdin_sender {
            println!("[Console] Stdin sender is available");
        } else {
            println!("[Console] ERROR: No stdin sender available");
            return Err("No stdin sender available".to_string());
        }

        // Очищаем старый ответ
        *self.response.lock().unwrap() = None;
        println!("[Console] Cleared old response");

        // Отправляем команду в stdin поток
        if let Some(ref stdin_sender) = self.stdin_sender {
            match stdin_sender.send(command.to_string()) {
                Ok(_) => println!("[Console] Command sent to stdin thread successfully"),
                Err(e) => {
                    println!("[Console] ERROR: Failed to send command: {}", e);
                    return Err(format!("Failed to send command: {}", e));
                }
            }
        } else {
            return Err("No stdin sender available".to_string());
        }

        // Ждем ответ с таймаутом 3 секунды
        let timeout = Duration::from_secs(3);
        let start = std::time::Instant::now();
        println!("[Console] Starting to wait for response...");

        while start.elapsed() < timeout {
            let response_guard = self.response.lock().unwrap();
            if let Some(response) = response_guard.as_ref() {
                println!("[Console] Got response: {:?}", response);
                return Ok(response.clone());
            }
            drop(response_guard); // Освобождаем блокировку

            std::thread::sleep(Duration::from_millis(100));
        }

        println!("[Console] ERROR: Timeout - no response received in 3 seconds");
        Err("Timeout: no response received in 3 seconds".to_string())
    }

    pub fn close(&mut self) {
        if let Some(ref stdin_sender) = self.stdin_sender {
            let _ = stdin_sender.send("exit".to_string());
        }
        if let Some(ref mut process) = self.process {
            let _ = process.kill();
        }
        if let Some(thread) = self.stdout_thread.take() {
            let _ = thread.join();
        }
        if let Some(thread) = self.stderr_thread.take() {
            let _ = thread.join();
        }
        if let Some(thread) = self.stdin_thread.take() {
            let _ = thread.join();
        }
    }
}

impl Drop for ConsoleInstance {
    fn drop(&mut self) {
        self.close();
    }
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

/// Выполняет команду в постоянном инстансе консоли
pub fn execute_command_silent(command: &str) -> Result<String, String> {
    println!(
        "[Console] execute_command_silent called with: '{}'",
        command
    );

    if command.trim().is_empty() {
        return Ok("".to_string());
    }

    if command.starts_with("cmd_") {
        println!("[Console] WARNING: Command looks like an ID: '{}'", command);
        return Ok("".to_string());
    }

    let mut console_guard = CONSOLE_INSTANCE.lock().unwrap();

    if console_guard.is_none() {
        println!("[Console] No console instance found, creating new one...");
        *console_guard = Some(ConsoleInstance::new()?);
        println!("[Console] New console instance created");
    } else {
        println!("[Console] Using existing console instance");
    }

    if let Some(ref mut console) = *console_guard {
        println!("[Console] Executing command in console instance");
        let result = console.execute_command(command);
        println!("[Console] Command execution completed");
        result
    } else {
        println!("[Console] Failed to create console instance");
        Err("Failed to create console instance".to_string())
    }
}

/// Проверяет состояние переключателя, выполняя команду
pub fn is_switch_enabled(switch_command: &str, app: Option<&AppHandle<Wry>>) -> bool {
    println!(
        "[Helpers] is_switch_enabled called with: '{}'",
        switch_command
    );
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
                if let Ok(_) = app_handle
                    .notification()
                    .builder()
                    .title("SwitchShuttle Error")
                    .body(&format!("Failed to check switch status: {}", e))
                    .show()
                {
                    // Уведомление отправлено
                }
            }
            false
        }
    }
}
