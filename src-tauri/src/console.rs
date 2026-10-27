//! Тихое выполнение команд (switch, monitor, scheduler) без окна терминала.
//!
//! Каждая команда — отдельный процесс оболочки: вывод не смешивается между командами,
//! `exit`/`cd`/чтение stdin одной команды не ломают следующие.
use log::{error, info};
use std::io::Read;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(3);

/// Что делать с командой, не уложившейся в таймаут.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OnTimeout {
    /// Убить процесс вместе с дочерними (опросы состояния: monitor, switch probe).
    Kill,
    /// Оставить работать в фоне (действия: переключение switch, scheduler).
    Detach,
}

/// Действие: ждём до 3 с, дольше — команда продолжает работать в фоне.
pub fn run_silent(command: &str) -> Result<String, String> {
    run(command, DEFAULT_TIMEOUT, OnTimeout::Detach)
}

/// Опрос состояния: ждём до 3 с, дольше — процесс убивается.
pub fn query(command: &str) -> Result<String, String> {
    run(command, DEFAULT_TIMEOUT, OnTimeout::Kill)
}

/// Включён ли переключатель: вывод команды "true" / "1" / "on" / "enabled".
pub fn is_switch_enabled(switch_command: &str) -> bool {
    match query(switch_command) {
        Ok(output) => {
            let result = output.trim().to_lowercase();
            info!("[Switch] {} -> '{}'", switch_command, result);
            matches!(result.as_str(), "true" | "1" | "on" | "enabled")
        }
        Err(e) => {
            info!("[Switch] {} -> off ({})", switch_command, e);
            false
        }
    }
}

/// Выполняет команду и возвращает её stdout (без крайних пробелов).
/// Ненулевой код выхода — ошибка с текстом stderr.
pub fn run(command: &str, timeout: Duration, on_timeout: OnTimeout) -> Result<String, String> {
    if command.trim().is_empty() {
        return Ok(String::new());
    }
    crate::settings::ensure_lines_allowed(&[command.to_string()])?;

    let mut child = spawn_shell(command)?;
    let stdout = capture(child.stdout.take());
    let stderr = capture(child.stderr.take());

    let deadline = Instant::now() + timeout;
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) if Instant::now() < deadline => thread::sleep(Duration::from_millis(10)),
            Ok(None) => {
                match on_timeout {
                    OnTimeout::Kill => {
                        crate::running::kill_process_tree(&mut child);
                        let _ = child.wait();
                    }
                    OnTimeout::Detach => {
                        // Забираем код выхода позже, чтобы не оставить зомби
                        thread::spawn(move || {
                            let _ = child.wait();
                        });
                    }
                }
                return Err(format!(
                    "Timeout: no result in {} seconds",
                    timeout.as_secs_f32()
                ));
            }
            Err(e) => return Err(format!("Failed to wait for command: {}", e)),
        }
    };

    let out = collect(stdout);
    let err = collect(stderr);
    if status.success() {
        Ok(out.trim().to_string())
    } else {
        let err = err.trim();
        if err.is_empty() {
            Err(format!("Command exited with {}", status))
        } else {
            error!("[Console] {} failed: {}", command, err);
            Err(err.to_string())
        }
    }
}

fn spawn_shell(command: &str) -> Result<Child, String> {
    #[cfg(windows)]
    let mut cmd = {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        let mut cmd = Command::new("cmd");
        // raw_arg: cmd.exe разбирает строку сам, экранирование Rust ломает кавычки
        cmd.arg("/C").raw_arg(command).creation_flags(CREATE_NO_WINDOW);
        cmd
    };

    #[cfg(unix)]
    let mut cmd = {
        use std::os::unix::process::CommandExt;
        let mut cmd = Command::new("/bin/bash");
        // Своя группа процессов — по таймауту убиваем и дочерние
        cmd.arg("-c").arg(command).process_group(0);
        cmd
    };

    cmd.stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn shell: {}", e))
}

type Captured = Option<(Arc<Mutex<Vec<u8>>>, JoinHandle<()>)>;

fn capture<R: Read + Send + 'static>(pipe: Option<R>) -> Captured {
    let mut pipe = pipe?;
    let buffer = Arc::new(Mutex::new(Vec::new()));
    let sink = buffer.clone();
    let handle = thread::spawn(move || {
        let mut chunk = [0u8; 4096];
        while let Ok(n) = pipe.read(&mut chunk) {
            if n == 0 {
                break;
            }
            sink.lock().unwrap().extend_from_slice(&chunk[..n]);
        }
    });
    Some((buffer, handle))
}

/// Ждём EOF недолго: фоновый потомок (`cmd &`) может держать pipe открытым.
fn collect(captured: Captured) -> String {
    let Some((buffer, handle)) = captured else {
        return String::new();
    };
    let deadline = Instant::now() + Duration::from_millis(200);
    while !handle.is_finished() && Instant::now() < deadline {
        thread::sleep(Duration::from_millis(5));
    }
    let bytes = buffer.lock().unwrap().clone();
    String::from_utf8_lossy(&bytes).into_owned()
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;

    #[test]
    fn returns_stdout() {
        assert_eq!(query("echo 'hello world'").unwrap(), "hello world");
    }

    #[test]
    fn command_without_output_succeeds() {
        assert_eq!(run_silent("true").unwrap(), "");
    }

    #[test]
    fn nonzero_exit_is_error_with_stderr() {
        let err = query("echo boom >&2; exit 3").unwrap_err();
        assert_eq!(err, "boom");
        assert!(query("false").is_err());
    }

    #[test]
    fn exit_and_cd_do_not_leak_into_next_command() {
        let _ = query("cd /; exit 0");
        assert_eq!(query("echo still-alive").unwrap(), "still-alive");
    }

    #[test]
    fn stdin_is_closed() {
        let start = Instant::now();
        assert_eq!(query("cat").unwrap(), "");
        assert!(start.elapsed() < Duration::from_secs(1));
    }

    #[test]
    fn timeout_kill_stops_long_command() {
        let start = Instant::now();
        let err = run("sleep 5", Duration::from_millis(200), OnTimeout::Kill).unwrap_err();
        assert!(err.starts_with("Timeout"), "{}", err);
        assert!(start.elapsed() < Duration::from_secs(2));
    }

    #[test]
    fn background_child_does_not_hang() {
        let start = Instant::now();
        assert_eq!(query("sleep 5 & echo started").unwrap(), "started");
        assert!(start.elapsed() < Duration::from_secs(2));
    }

    #[test]
    fn parallel_commands_get_their_own_output() {
        let handles: Vec<_> = (0..8)
            .map(|i| thread::spawn(move || (i, query(&format!("sleep 0.0{}; echo {}", i, i)))))
            .collect();
        for handle in handles {
            let (i, result) = handle.join().unwrap();
            assert_eq!(result.unwrap(), i.to_string());
        }
    }

    #[test]
    fn switch_state_parsing() {
        assert!(is_switch_enabled("echo true"));
        assert!(is_switch_enabled("echo ON"));
        assert!(is_switch_enabled("echo 1"));
        assert!(!is_switch_enabled("echo false"));
        assert!(!is_switch_enabled("false"));
    }
}

/// Те же гарантии на настоящем cmd.exe (запускаются в CI на windows-latest).
#[cfg(all(test, windows))]
mod windows_tests {
    use super::*;

    #[test]
    fn returns_stdout() {
        assert_eq!(query("echo hello").unwrap(), "hello");
    }

    #[test]
    fn quotes_reach_cmd_unchanged() {
        // raw_arg: без него Rust превратил бы кавычки в \"
        assert_eq!(query(r#"echo "a b""#).unwrap(), r#""a b""#);
    }

    #[test]
    fn nonzero_exit_is_error() {
        assert!(query("exit /b 3").is_err());
    }

    #[test]
    fn timeout_kill_returns() {
        let start = Instant::now();
        let err = run("ping -n 6 127.0.0.1", Duration::from_millis(300), OnTimeout::Kill).unwrap_err();
        assert!(err.starts_with("Timeout"), "{}", err);
        assert!(start.elapsed() < Duration::from_secs(3));
    }

    #[test]
    fn switch_state_parsing() {
        assert!(is_switch_enabled("echo true"));
        assert!(!is_switch_enabled("echo false"));
    }
}
