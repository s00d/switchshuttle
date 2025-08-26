use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use tauri::{AppHandle, Wry};
use log::{error, info};

// Глобальный пул консольных соединений
static CONSOLE_POOL: Lazy<Arc<Mutex<ConsolePool>>> =
    Lazy::new(|| Arc::new(Mutex::new(ConsolePool::new())));

// Глобальный инстанс консоли (для обратной совместимости)
static CONSOLE_INSTANCE: Lazy<Arc<Mutex<Option<ConsoleInstance>>>> =
    Lazy::new(|| Arc::new(Mutex::new(None)));

/// Пул консольных соединений для параллельного выполнения команд
pub struct ConsolePool {
    connections: HashMap<String, ConsoleInstance>,
    max_connections: usize,
}

impl ConsolePool {
    pub fn new() -> Self {
        Self {
            connections: HashMap::new(),
            max_connections: 3,
        }
    }

    /// Инициализирует постоянный инстанс консоли
    pub fn init_console() -> Result<(), String> {
        info!("[Console] Initializing console instance...");
        let mut console_guard = CONSOLE_INSTANCE.lock().unwrap();
        if console_guard.is_none() {
            info!("[Console] Creating new console instance...");
            *console_guard = Some(ConsoleInstance::new()?);
            info!("[Console] Console instance created successfully");
        } else {
            info!("[Console] Console instance already exists");
        }
        Ok(())
    }

    /// Получает или создает соединение для выполнения команды
    pub fn get_connection(&mut self, command_id: &str) -> Result<&mut ConsoleInstance, String> {
        // Проверяем, есть ли уже соединение для этой команды и живо ли оно
        let should_remove = {
            if let Some(connection) = self.connections.get_mut(command_id) {
                !connection.is_alive()
            } else {
                false
            }
        };

        if should_remove {
            self.connections.remove(command_id);
        }

        // Проверяем лимит соединений
        if self.connections.len() >= self.max_connections {
            // Удаляем самое старое соединение
            if let Some(oldest_key) = self.connections.keys().next().cloned() {
                if let Some(mut old_connection) = self.connections.remove(&oldest_key) {
                    old_connection.close();
                }
            }
        }

        // Создаем новое соединение, если его нет
        if !self.connections.contains_key(command_id) {
            let new_connection = ConsoleInstance::new()?;
            self.connections
                .insert(command_id.to_string(), new_connection);
        }

        Ok(self.connections.get_mut(command_id).unwrap())
    }

    /// Выполняет команду через пул соединений
    pub fn execute_command(&mut self, command_id: &str, command: &str) -> Result<String, String> {
        let connection = self.get_connection(command_id)?;
        connection.execute_command(command)
    }

    /// Очищает неактивные соединения
    pub fn cleanup_inactive_connections(&mut self) {
        let mut to_remove = Vec::new();

        for (id, connection) in &mut self.connections {
            if !connection.is_alive() {
                to_remove.push(id.clone());
            }
        }

        for id in to_remove {
            if let Some(mut connection) = self.connections.remove(&id) {
                connection.close();
            }
        }
    }

    /// Закрывает все соединения в пуле
    pub fn close_all(&mut self) {
        for (_, mut connection) in self.connections.drain() {
            connection.close();
        }
    }
}

impl Drop for ConsolePool {
    fn drop(&mut self) {
        self.close_all();
    }
}

pub struct ConsoleInstance {
    process: Option<std::process::Child>,
    stdin_sender: Option<std::sync::mpsc::Sender<String>>, // Канал для отправки команд
    response: Arc<Mutex<Option<String>>>,                  // Переменная для хранения ответа
    stdout_thread: Option<JoinHandle<()>>,
    stderr_thread: Option<JoinHandle<()>>, // Отдельный поток для stderr
    stdin_thread: Option<JoinHandle<()>>,
    last_activity: std::time::Instant,
}

impl ConsoleInstance {
    pub fn new() -> Result<Self, String> {
        info!("[Console] Creating new console instance...");

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
            last_activity: std::time::Instant::now(),
        })
    }

    /// Проверяет, живо ли соединение
    pub fn is_alive(&mut self) -> bool {
        if let Some(ref mut process) = self.process {
            match process.try_wait() {
                Ok(Some(_)) => false, // Процесс завершился
                Ok(None) => true,     // Процесс еще работает
                Err(_) => false,      // Ошибка при проверке
            }
        } else {
            false
        }
    }

    /// Обновляет время последней активности
    pub fn update_activity(&mut self) {
        self.last_activity = std::time::Instant::now();
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
                info!("[Console] Stdin thread started");

                while let Ok(command) = stdin_rx.recv() {
                    info!("[Console] Stdin thread received command: {:?}", command);
                    if command == "exit" {
                        info!("[Console] Stdin thread received exit command");
                        break;
                    }
                    if let Err(e) = writeln!(stdin_writer, "{}", command) {
                        error!("[Console] Error writing to stdin: {}", e);
                        break;
                    }
                    info!("[Console] Stdin thread wrote command to process");
                }
                info!("[Console] Stdin thread finished");
            });

            // Клонируем response для потоков
            let response_stdout = response.clone();
            let response_stderr = response.clone();

            // Поток для чтения stdout
            let stdout_thread = thread::spawn(move || {
                let mut stdout_reader = BufReader::new(stdout);
                info!("[Console] Stdout thread started");
                loop {
                    // Читаем из stdout
                    let mut line = String::new();
                    match stdout_reader.read_line(&mut line) {
                        Ok(0) => {
                            // EOF - поток закрыт
                            info!("[Console] Stdout EOF reached");
                            break;
                        }
                        Ok(_) => {
                            if !line.trim().is_empty() {
                                info!("[Console] Got stdout: {:?}", line.trim());
                                *response_stdout.lock().unwrap() = Some(line.trim().to_string());
                            }
                        }
                        Err(e) => {
                            error!("[Console] Error reading stdout: {}", e);
                            break;
                        }
                    }

                    // Небольшая пауза
                    std::thread::sleep(Duration::from_millis(10));
                }
                info!("[Console] Stdout thread finished");
            });

            // Поток для чтения stderr
            let stderr_thread = thread::spawn(move || {
                let mut stderr_reader = BufReader::new(stderr);
                info!("[Console] Stderr thread started");

                loop {
                    // Читаем из stderr
                    let mut err_line = String::new();
                    match stderr_reader.read_line(&mut err_line) {
                        Ok(0) => {
                            // EOF - поток закрыт
                            info!("[Console] Stderr EOF reached");
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
                                    info!("[Console] Got stderr: {:?}", trimmed);
                                    *response_stderr.lock().unwrap() =
                                        Some(format!("stderr: {}", trimmed));
                                } else {
                                    info!(
                                        "[Console] Filtered stderr (echo/prompt): {:?}",
                                        trimmed
                                    );
                                }
                            }
                        }
                        Err(e) => {
                            error!("[Console] Error reading stderr: {}", e);
                            break;
                        }
                    }

                    // Небольшая пауза
                    std::thread::sleep(Duration::from_millis(10));
                }
                info!("[Console] Stderr thread finished");
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
        info!("[Console] Executing command: {}", command);
        if command.trim().is_empty() {
            return Ok("".to_string());
        }

        // Обновляем время активности
        self.update_activity();

        // Проверяем состояние процесса
        if let Some(ref mut process) = self.process {
            match process.try_wait() {
                Ok(Some(exit_status)) => {
                    error!(
                        "[Console] ERROR: Process has exited with status: {:?}",
                        exit_status
                    );
                    return Err("Process has exited".to_string());
                }
                Ok(None) => {
                    info!("[Console] Process is still running");
                }
                Err(e) => {
                    error!("[Console] ERROR: Failed to check process status: {}", e);
                    return Err("Failed to check process status".to_string());
                }
            }
        } else {
            error!("[Console] ERROR: No process available");
            return Err("No process available".to_string());
        }

        // Проверяем состояние потоков
        if let Some(ref _stdin_sender) = self.stdin_sender {
            info!("[Console] Stdin sender is available");
        } else {
            error!("[Console] ERROR: No stdin sender available");
            return Err("No stdin sender available".to_string());
        }

        // Очищаем старый ответ
        *self.response.lock().unwrap() = None;
        info!("[Console] Cleared old response");

        // Отправляем команду в stdin поток
        if let Some(ref stdin_sender) = self.stdin_sender {
            match stdin_sender.send(command.to_string()) {
                Ok(_) => info!("[Console] Command sent to stdin thread successfully"),
                Err(e) => {
                    error!("[Console] ERROR: Failed to send command: {}", e);
                    return Err(format!("Failed to send command: {}", e));
                }
            }
        } else {
            return Err("No stdin sender available".to_string());
        }

        // Ждем ответ с таймаутом 3 секунды
        let timeout = Duration::from_secs(3);
        let start = std::time::Instant::now();
        info!("[Console] Starting to wait for response...");

        while start.elapsed() < timeout {
            let response_guard = self.response.lock().unwrap();
            if let Some(response) = response_guard.as_ref() {
                info!("[Console] Got response: {:?}", response);
                return Ok(response.clone());
            }
            drop(response_guard); // Освобождаем блокировку

            std::thread::sleep(Duration::from_millis(100));
        }

        error!("[Console] ERROR: Timeout - no response received in 3 seconds");
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

impl ConsoleInstance {
    /// Выполняет команду в постоянном инстансе консоли
    pub fn execute_command_silent(command: &str) -> Result<String, String> {
        info!("[Console] execute_command_silent: {}", command);
        let mut console_guard = CONSOLE_INSTANCE.lock().unwrap();
        if let Some(ref mut console) = *console_guard {
            console.execute_command(command)
        } else {
            Err("Console instance not initialized".to_string())
        }
    }

    /// Выполняет команду через пул соединений
    pub fn execute_command_via_pool(command_id: &str, command: &str) -> Result<String, String> {
        info!(
            "[Console] execute_command_via_pool: {} -> {}",
            command_id, command
        );
        let mut pool_guard = CONSOLE_POOL.lock().unwrap();
        pool_guard.execute_command(command_id, command)
    }

    /// Очищает неактивные соединения в пуле
    pub fn cleanup_console_pool() {
        let mut pool_guard = CONSOLE_POOL.lock().unwrap();
        pool_guard.cleanup_inactive_connections();
    }

    /// Проверяет, включен ли переключатель
    pub fn is_switch_enabled(switch_command: &str, _app: Option<&AppHandle<Wry>>) -> bool {
        info!("[Switch] is_switch_enabled: {}", switch_command);
        match Self::execute_command_silent(switch_command) {
            Ok(output) => {
                let result = output.trim().to_lowercase();
                info!("[Switch] is_switch_enabled result: '{}'", result);
                result == "true" || result == "1" || result == "on" || result == "enabled"
            }
            Err(e) => {
                error!("[Switch] is_switch_enabled error: {}", e);
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_console_pool_new() {
        let pool = ConsolePool::new();
        assert_eq!(pool.connections.len(), 0);
        assert_eq!(pool.max_connections, 3);
    }

    #[test]
    fn test_console_pool_get_connection() {
        let mut pool = ConsolePool::new();

        // Тест создания нового соединения
        let result = pool.get_connection("test_command_1");
        assert!(result.is_ok());

        let connection = result.unwrap();
        assert!(connection.is_alive());

        // Тест повторного использования соединения
        let result2 = pool.get_connection("test_command_1");
        assert!(result2.is_ok());

        // Проверяем, что соединение все еще живо
        let connection2 = result2.unwrap();
        assert!(connection2.is_alive());
    }

    #[test]
    fn test_console_pool_max_connections() {
        let mut pool = ConsolePool::new();

        // Создаем максимальное количество соединений
        for i in 0..3 {
            let result = pool.get_connection(&format!("command_{}", i));
            assert!(result.is_ok());
        }

        // Проверяем, что количество соединений не превышает лимит
        assert_eq!(pool.connections.len(), 3);

        // Создаем еще одно соединение - должно удалить самое старое
        let result = pool.get_connection("command_4");
        assert!(result.is_ok());

        // Количество соединений должно остаться равным лимиту
        assert_eq!(pool.connections.len(), 3);
    }

    #[test]
    fn test_console_pool_cleanup_inactive_connections() {
        let mut pool = ConsolePool::new();

        // Создаем соединение
        let result = pool.get_connection("test_cleanup");
        assert!(result.is_ok());

        // Проверяем, что соединение активно
        assert_eq!(pool.connections.len(), 1);

        // Очищаем неактивные соединения
        pool.cleanup_inactive_connections();

        // Соединение должно остаться, так как оно активно
        assert_eq!(pool.connections.len(), 1);
    }

    #[test]
    fn test_console_instance_new() {
        let result = ConsoleInstance::new();
        assert!(result.is_ok());

        let instance = result.unwrap();
        assert!(instance.process.is_some());
        assert!(instance.stdin_sender.is_some());
        assert!(instance.stdout_thread.is_some());
        assert!(instance.stderr_thread.is_some());
        assert!(instance.stdin_thread.is_some());
    }

    #[test]
    fn test_console_instance_is_alive() {
        let mut instance = ConsoleInstance::new().unwrap();

        // Свежесозданный инстанс должен быть жив
        assert!(instance.is_alive());

        // Обновляем активность
        let old_activity = instance.last_activity;
        instance.update_activity();
        assert!(instance.last_activity > old_activity);
    }

    #[test]
    fn test_console_instance_execute_simple_command() {
        let mut instance = ConsoleInstance::new().unwrap();

        // Тестируем выполнение простой команды
        let result = instance.execute_command("echo 'test'");

        // Команда должна выполниться успешно
        assert!(result.is_ok());

        let output = result.unwrap();
        // Проверяем, что получили какой-то ответ
        assert!(!output.is_empty());
    }

    #[test]
    fn test_console_instance_execute_empty_command() {
        let mut instance = ConsoleInstance::new().unwrap();

        // Тестируем выполнение пустой команды
        let result = instance.execute_command("");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");

        let result2 = instance.execute_command("   ");
        assert!(result2.is_ok());
        assert_eq!(result2.unwrap(), "");
    }

    #[test]
    fn test_console_instance_close() {
        let mut instance = ConsoleInstance::new().unwrap();

        // Проверяем, что инстанс жив
        assert!(instance.is_alive());

        // Закрываем инстанс
        instance.close();

        // Проверяем, что инстанс больше не жив
        assert!(!instance.is_alive());
    }

    #[test]
    fn test_init_console() {
        // Очищаем глобальный инстанс перед тестом
        {
            let mut console_guard = CONSOLE_INSTANCE.lock().unwrap();
            if let Some(mut console) = console_guard.take() {
                console.close();
            }
        }

        // Инициализируем консоль через статическую функцию ConsolePool
        let result = ConsolePool::init_console();
        assert!(result.is_ok());

        // Проверяем, что инстанс создан
        {
            let console_guard = CONSOLE_INSTANCE.lock().unwrap();
            assert!(console_guard.is_some());
        }

        // Очищаем глобальный инстанс после теста
        {
            let mut console_guard = CONSOLE_INSTANCE.lock().unwrap();
            if let Some(mut console) = console_guard.take() {
                console.close();
            }
        }
    }

    #[test]
    fn test_init_console_via_pool() {
        // Очищаем глобальный инстанс перед тестом
        {
            let mut console_guard = CONSOLE_INSTANCE.lock().unwrap();
            if let Some(mut console) = console_guard.take() {
                console.close();
            }
        }

        // Инициализируем консоль через статическую функцию ConsolePool
        let result = ConsolePool::init_console();
        assert!(result.is_ok());

        // Проверяем, что инстанс создан
        {
            let console_guard = CONSOLE_INSTANCE.lock().unwrap();
            assert!(console_guard.is_some());
        }

        // Повторная инициализация должна пройти успешно (уже существует)
        let result2 = ConsolePool::init_console();
        assert!(result2.is_ok());

        // Очищаем глобальный инстанс после теста
        {
            let mut console_guard = CONSOLE_INSTANCE.lock().unwrap();
            if let Some(mut console) = console_guard.take() {
                console.close();
            }
        }
    }

    #[test]
    fn test_execute_command_silent() {
        // Очищаем глобальный инстанс перед тестом
        {
            let mut console_guard = CONSOLE_INSTANCE.lock().unwrap();
            if let Some(mut console) = console_guard.take() {
                console.close();
            }
        }

        // Инициализируем консоль
        ConsolePool::init_console().unwrap();

        // Выполняем простую команду
        let result = ConsoleInstance::execute_command_silent("echo 'silent test'");
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(!output.is_empty());

        // Очищаем глобальный инстанс после теста
        {
            let mut console_guard = CONSOLE_INSTANCE.lock().unwrap();
            if let Some(mut console) = console_guard.take() {
                console.close();
            }
        }
    }

    #[test]
    fn test_execute_command_via_pool() {
        // Выполняем команду через пул
        let result =
            ConsoleInstance::execute_command_via_pool("test_pool_command", "echo 'pool test'");
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(!output.is_empty());
    }

    #[test]
    fn test_cleanup_console_pool() {
        // Очищаем пул перед тестом
        {
            let mut pool_guard = CONSOLE_POOL.lock().unwrap();
            pool_guard.close_all();
        }

        // Создаем несколько соединений в пуле
        ConsoleInstance::execute_command_via_pool("cleanup_test_1", "echo 'test1'").unwrap();
        ConsoleInstance::execute_command_via_pool("cleanup_test_2", "echo 'test2'").unwrap();

        // Очищаем пул
        ConsoleInstance::cleanup_console_pool();

        // Проверяем, что пул все еще работает
        let result = ConsoleInstance::execute_command_via_pool("cleanup_test_3", "echo 'test3'");
        assert!(result.is_ok());

        // Очищаем пул после теста
        {
            let mut pool_guard = CONSOLE_POOL.lock().unwrap();
            pool_guard.close_all();
        }
    }

    #[test]
    fn test_is_switch_enabled() {
        // Очищаем глобальный инстанс перед тестом
        {
            let mut console_guard = CONSOLE_INSTANCE.lock().unwrap();
            if let Some(mut console) = console_guard.take() {
                console.close();
            }
        }

        // Инициализируем консоль
        ConsolePool::init_console().unwrap();

        // Тестируем с командой, которая возвращает "true"
        let result = ConsoleInstance::is_switch_enabled("echo 'true'", None);
        assert!(result);

        // Тестируем с командой, которая возвращает "false"
        let result2 = ConsoleInstance::is_switch_enabled("echo 'false'", None);
        assert!(!result2);

        // Тестируем с командой, которая возвращает "1"
        let result3 = ConsoleInstance::is_switch_enabled("echo '1'", None);
        assert!(result3);

        // Тестируем с командой, которая возвращает "on"
        let result4 = ConsoleInstance::is_switch_enabled("echo 'on'", None);
        assert!(result4);

        // Тестируем с командой, которая возвращает "enabled"
        let result5 = ConsoleInstance::is_switch_enabled("echo 'enabled'", None);
        assert!(result5);

        // Очищаем глобальный инстанс после теста
        {
            let mut console_guard = CONSOLE_INSTANCE.lock().unwrap();
            if let Some(mut console) = console_guard.take() {
                console.close();
            }
        }
    }

    #[test]
    fn test_console_pool_execute_command() {
        let mut pool = ConsolePool::new();

        // Выполняем команду через пул
        let result = pool.execute_command("test_execute", "echo 'pool execute test'");
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(!output.is_empty());
    }

    #[test]
    fn test_console_instance_timeout() {
        let mut instance = ConsoleInstance::new().unwrap();

        // Тестируем команду, которая может вызвать таймаут
        // (команда sleep с большим временем)
        let result = instance.execute_command("sleep 5");

        // Должен быть таймаут, так как команда выполняется дольше 3 секунд
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Timeout"));

        // Явно закрываем инстанс
        instance.close();
    }

    #[test]
    fn test_console_pool_drop() {
        let mut pool = ConsolePool::new();

        // Создаем несколько соединений
        pool.get_connection("drop_test_1").unwrap();
        pool.get_connection("drop_test_2").unwrap();

        // Проверяем, что соединения созданы
        assert_eq!(pool.connections.len(), 2);

        // При дропе пула все соединения должны быть закрыты
        // Это тестируется автоматически при выходе из области видимости
    }

    #[test]
    fn test_console_instance_drop() {
        let instance = ConsoleInstance::new().unwrap();

        // Проверяем, что инстанс создан
        assert!(instance.process.is_some());

        // При дропе инстанса все ресурсы должны быть освобождены
        // Это тестируется автоматически при выходе из области видимости
    }

    #[test]
    fn test_multiple_commands_same_connection() {
        let mut instance = ConsoleInstance::new().unwrap();

        // Выполняем несколько команд через одно соединение
        let result1 = instance.execute_command("echo 'command1'");
        assert!(result1.is_ok());

        let result2 = instance.execute_command("echo 'command2'");
        assert!(result2.is_ok());

        let result3 = instance.execute_command("echo 'command3'");
        assert!(result3.is_ok());

        // Все команды должны выполниться успешно
        assert!(!result1.unwrap().is_empty());
        assert!(!result2.unwrap().is_empty());
        assert!(!result3.unwrap().is_empty());

        // Явно закрываем инстанс
        instance.close();
    }

    #[test]
    fn test_console_pool_concurrent_commands() {
        let mut pool = ConsolePool::new();

        // Выполняем несколько команд с разными ID
        let results: Vec<Result<String, String>> = (0..5)
            .map(|i| {
                pool.execute_command(&format!("concurrent_{}", i), &format!("echo 'test{}'", i))
            })
            .collect();

        // Все команды должны выполниться успешно
        for result in results {
            assert!(result.is_ok());
            assert!(!result.unwrap().is_empty());
        }
    }

    #[test]
    fn test_error_handling() {
        let mut instance = ConsoleInstance::new().unwrap();

        // Тестируем команду, которая может вызвать ошибку
        let result = instance.execute_command("nonexistent_command_12345");

        // Команда должна выполниться, но вернуть сообщение об ошибке через stderr
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("command not found"));

        // Явно закрываем инстанс
        instance.close();
    }

    #[test]
    fn test_console_pool_connection_reuse() {
        let mut pool = ConsolePool::new();

        // Создаем соединение
        let connection1 = pool.get_connection("reuse_test").unwrap();
        assert!(connection1.is_alive());

        // Используем то же соединение снова
        let connection2 = pool.get_connection("reuse_test").unwrap();
        assert!(connection2.is_alive());

        // Проверяем, что это то же самое соединение
        assert_eq!(pool.connections.len(), 1);
    }

    #[test]
    fn test_console_instance_activity_tracking() {
        let mut instance = ConsoleInstance::new().unwrap();

        let initial_activity = instance.last_activity;

        // Ждем немного
        std::thread::sleep(Duration::from_millis(10));

        // Обновляем активность
        instance.update_activity();

        // Проверяем, что время активности обновилось
        assert!(instance.last_activity > initial_activity);

        // Явно закрываем инстанс
        instance.close();
    }
}
