use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use log::info;

pub const MAX_JOBS: usize = 16;
pub const STOP_ALL_MENU_ID: &str = "stop_all_running";
const STOP_PREFIX: &str = "stop_running_";

static NEXT_RUN_ID: AtomicU64 = AtomicU64::new(1);
static REGISTRY: Lazy<Arc<Mutex<ProcessRegistry>>> =
    Lazy::new(|| Arc::new(Mutex::new(ProcessRegistry::new())));
static EXIT_WATCHER_STARTED: AtomicBool = AtomicBool::new(false);

fn ensure_exit_watcher() {
    if EXIT_WATCHER_STARTED.swap(true, Ordering::SeqCst) {
        return;
    }
    thread::spawn(|| loop {
        thread::sleep(Duration::from_secs(1));
        let removed = with_registry(|reg| reg.prune());
        if !removed.is_empty() {
            crate::menu::refresh_tray_after_running_change();
        }
    });
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JobInfo {
    pub run_id: u64,
    pub command_id: String,
    pub display_name: String,
}

struct Job {
    run_id: u64,
    command_id: String,
    display_name: String,
    child: Child,
    #[allow(dead_code)]
    started_at: Instant,
}

pub struct ProcessRegistry {
    jobs: HashMap<u64, Job>,
}

impl ProcessRegistry {
    pub fn new() -> Self {
        Self {
            jobs: HashMap::new(),
        }
    }

    pub fn start(
        &mut self,
        command_id: &str,
        display_name: &str,
        shell_lines: &[String],
    ) -> Result<u64, String> {
        self.prune();
        if self.jobs.len() >= MAX_JOBS {
            return Err(format!(
                "Too many running commands (max {})",
                MAX_JOBS
            ));
        }

        let script = join_shell_lines(shell_lines)?;
        let child = spawn_job_process(&script)?;
        let run_id = NEXT_RUN_ID.fetch_add(1, Ordering::SeqCst);

        // Detach wait so natural exit can be observed via prune/try_wait
        let pid = child.id();
        info!(
            "[Running] started run_id={} command_id={} pid={} name={}",
            run_id, command_id, pid, display_name
        );

        self.jobs.insert(
            run_id,
            Job {
                run_id,
                command_id: command_id.to_string(),
                display_name: display_name.to_string(),
                child,
                started_at: Instant::now(),
            },
        );

        ensure_exit_watcher();
        Ok(run_id)
    }

    pub fn stop(&mut self, run_id: u64) -> Result<(), String> {
        let Some(mut job) = self.jobs.remove(&run_id) else {
            return Ok(());
        };
        kill_process_tree(&mut job.child);
        let _ = job.child.wait();
        info!("[Running] stopped run_id={}", run_id);
        Ok(())
    }

    pub fn stop_all(&mut self) -> Result<(), String> {
        let ids: Vec<u64> = self.jobs.keys().copied().collect();
        for id in ids {
            let _ = self.stop(id);
        }
        Ok(())
    }

    pub fn prune(&mut self) -> Vec<u64> {
        let mut removed = Vec::new();
        let mut finished = Vec::new();
        for (id, job) in self.jobs.iter_mut() {
            match job.child.try_wait() {
                Ok(Some(_)) => finished.push(*id),
                Ok(None) => {}
                Err(_) => finished.push(*id),
            }
        }
        for id in finished {
            self.jobs.remove(&id);
            removed.push(id);
        }
        removed
    }

    pub fn list(&mut self) -> Vec<JobInfo> {
        self.prune();
        let mut items: Vec<JobInfo> = self
            .jobs
            .values()
            .map(|job| JobInfo {
                run_id: job.run_id,
                command_id: job.command_id.clone(),
                display_name: job.display_name.clone(),
            })
            .collect();
        items.sort_by_key(|j| j.run_id);
        items
    }

    pub fn has_alive_for_command(&mut self, command_id: &str) -> bool {
        self.prune();
        self.jobs.values().any(|j| j.command_id == command_id)
    }

    pub fn job_labels(list: &[JobInfo]) -> Vec<(u64, String)> {
        list.iter()
            .map(|job| {
                let dup = list
                    .iter()
                    .filter(|other| other.display_name == job.display_name)
                    .count()
                    > 1;
                let label = if dup {
                    format!("{} · #{}", job.display_name, job.run_id)
                } else {
                    job.display_name.clone()
                };
                (job.run_id, label)
            })
            .collect()
    }
}

pub fn with_registry<F, R>(f: F) -> R
where
    F: FnOnce(&mut ProcessRegistry) -> R,
{
    let mut guard = REGISTRY.lock().unwrap();
    f(&mut guard)
}

pub fn prepare_quit() {
    let _ = with_registry(|reg| reg.stop_all());
}

pub fn stop_menu_id(run_id: u64) -> String {
    format!("{}{}", STOP_PREFIX, run_id)
}

pub fn parse_stop_menu_id(id: &str) -> Option<u64> {
    id.strip_prefix(STOP_PREFIX)?.parse().ok()
}

pub fn join_shell_lines(lines: &[String]) -> Result<String, String> {
    let parts: Vec<&str> = lines
        .iter()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
    if parts.is_empty() {
        return Err("No commands to run".into());
    }
    Ok(parts.join(" && "))
}

fn spawn_job_process(script: &str) -> Result<Child, String> {
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NEW_PROCESS_GROUP: u32 = 0x00000200;
        Command::new("cmd.exe")
            .args(["/C", script])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .creation_flags(CREATE_NEW_PROCESS_GROUP)
            .spawn()
            .map_err(|e| format!("Failed to spawn background process: {}", e))
    }

    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        unsafe {
            Command::new("/bin/sh")
                .args(["-c", script])
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .pre_exec(|| {
                    if libc::setsid() == -1 {
                        return Err(std::io::Error::last_os_error());
                    }
                    Ok(())
                })
                .spawn()
                .map_err(|e| format!("Failed to spawn background process: {}", e))
        }
    }
}

fn kill_process_tree(child: &mut Child) {
    let pid = child.id();

    #[cfg(unix)]
    {
        unsafe {
            // Negative pid = process group (after setsid, pgid == pid)
            let _ = libc::kill(-(pid as i32), libc::SIGTERM);
        }
        thread::sleep(Duration::from_millis(200));
        match child.try_wait() {
            Ok(None) => {
                unsafe {
                    let _ = libc::kill(-(pid as i32), libc::SIGKILL);
                }
                let _ = child.kill();
            }
            _ => {}
        }
    }

    #[cfg(windows)]
    {
        let _ = Command::new("taskkill")
            .args(["/F", "/T", "/PID", &pid.to_string()])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        let _ = child.kill();
    }
}

/// Pure helper for Soft tray refresh scheduler reconcile.
#[derive(Debug, PartialEq, Eq)]
pub struct SchedulerReconcile {
    pub keep: Vec<String>,
    pub start: Vec<String>,
    pub stop: Vec<String>,
}

pub fn reconcile_scheduler_ids(old_ids: &[String], new_ids: &[String]) -> SchedulerReconcile {
    let mut keep = Vec::new();
    let mut start = Vec::new();
    let mut stop = Vec::new();

    for id in new_ids {
        if old_ids.iter().any(|o| o == id) {
            keep.push(id.clone());
        } else {
            start.push(id.clone());
        }
    }
    for id in old_ids {
        if !new_ids.iter().any(|n| n == id) {
            stop.push(id.clone());
        }
    }

    SchedulerReconcile { keep, start, stop }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command as StdCommand;

    #[test]
    fn join_shell_lines_fail_fast() {
        let lines = vec!["echo a".into(), "echo b".into()];
        assert_eq!(join_shell_lines(&lines).unwrap(), "echo a && echo b");
    }

    #[test]
    fn join_shell_lines_rejects_empty() {
        assert!(join_shell_lines(&[]).is_err());
        assert!(join_shell_lines(&["".into(), "  ".into()]).is_err());
    }

    #[test]
    fn stop_menu_id_roundtrip() {
        let id = stop_menu_id(42);
        assert_eq!(parse_stop_menu_id(&id), Some(42));
        assert_eq!(parse_stop_menu_id("cmd_1"), None);
        assert_eq!(parse_stop_menu_id(STOP_ALL_MENU_ID), None);
    }

    #[test]
    fn reconcile_keeps_overlap() {
        let old = vec!["a".into(), "b".into()];
        let new = vec!["b".into(), "c".into()];
        let r = reconcile_scheduler_ids(&old, &new);
        assert_eq!(r.keep, vec!["b".to_string()]);
        assert_eq!(r.start, vec!["c".to_string()]);
        assert_eq!(r.stop, vec!["a".to_string()]);
    }

    #[test]
    fn start_list_stop_lifecycle() {
        let mut reg = ProcessRegistry::new();
        let sleep_cmd = if cfg!(windows) {
            "timeout /T 30 /NOBREAK >NUL".to_string()
        } else {
            "sleep 30".to_string()
        };
        let run_id = reg
            .start("cmd_1", "Sleep", &[sleep_cmd])
            .expect("start");
        let list = reg.list();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].run_id, run_id);
        reg.stop(run_id).unwrap();
        assert!(reg.list().is_empty());
    }

    #[test]
    fn stop_is_idempotent() {
        let mut reg = ProcessRegistry::new();
        reg.stop(999).unwrap();
    }

    #[test]
    fn stop_kills_process_group_child() {
        if cfg!(windows) {
            return;
        }
        let mut reg = ProcessRegistry::new();
        // Outer sh is setsid leader; inner sleep is in same group
        let run_id = reg
            .start("cmd_pg", "Nested", &["sleep 60".into()])
            .unwrap();
        let pid = {
            let job = reg.jobs.get(&run_id).unwrap();
            job.child.id()
        };
        reg.stop(run_id).unwrap();
        thread::sleep(Duration::from_millis(300));
        // kill -0 should fail for dead process
        let status = StdCommand::new("kill")
            .args(["-0", &pid.to_string()])
            .status()
            .unwrap();
        assert!(!status.success(), "parent pid should be dead");
    }

    #[test]
    fn prune_removes_exited() {
        let mut reg = ProcessRegistry::new();
        let cmd = if cfg!(windows) {
            "echo ok".to_string()
        } else {
            "true".to_string()
        };
        let _ = reg.start("cmd_x", "Quick", &[cmd]).unwrap();
        thread::sleep(Duration::from_millis(400));
        assert!(reg.list().is_empty());
    }

    #[test]
    fn allows_duplicate_command_ids() {
        let mut reg = ProcessRegistry::new();
        let sleep_cmd = if cfg!(windows) {
            "timeout /T 30 /NOBREAK >NUL".to_string()
        } else {
            "sleep 30".to_string()
        };
        let a = reg.start("cmd_1", "Dup", &[sleep_cmd.clone()]).unwrap();
        let b = reg.start("cmd_1", "Dup", &[sleep_cmd]).unwrap();
        assert_ne!(a, b);
        assert_eq!(reg.list().len(), 2);
        let labels = ProcessRegistry::job_labels(&reg.list());
        assert!(labels.iter().all(|(_, l)| l.contains('#')));
        reg.stop_all().unwrap();
    }

    #[test]
    fn enforces_max_jobs() {
        let mut reg = ProcessRegistry::new();
        let sleep_cmd = if cfg!(windows) {
            "timeout /T 60 /NOBREAK >NUL".to_string()
        } else {
            "sleep 60".to_string()
        };
        for i in 0..MAX_JOBS {
            reg.start(&format!("c{}", i), "J", &[sleep_cmd.clone()])
                .unwrap();
        }
        let err = reg.start("overflow", "J", &[sleep_cmd]).unwrap_err();
        assert!(err.contains("Too many"));
        reg.stop_all().unwrap();
    }

    #[test]
    fn prepare_quit_clears_global_registry() {
        with_registry(|reg| {
            let sleep_cmd = if cfg!(windows) {
                "timeout /T 30 /NOBREAK >NUL".to_string()
            } else {
                "sleep 30".to_string()
            };
            reg.start("q", "QuitTest", &[sleep_cmd]).unwrap();
        });
        prepare_quit();
        with_registry(|reg| assert!(reg.list().is_empty()));
    }

    #[test]
    fn has_alive_for_command() {
        let mut reg = ProcessRegistry::new();
        let sleep_cmd = if cfg!(windows) {
            "timeout /T 30 /NOBREAK >NUL".to_string()
        } else {
            "sleep 30".to_string()
        };
        reg.start("cmd_a", "A", &[sleep_cmd]).unwrap();
        assert!(reg.has_alive_for_command("cmd_a"));
        assert!(!reg.has_alive_for_command("cmd_b"));
        reg.stop_all().unwrap();
    }
}
