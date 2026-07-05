use crate::error::{Result, SdmError};
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceInfo {
    pub raw_line: String,
}

fn command_output(program: &str, args: &[&str]) -> Result<String> {
    let out = Command::new(program).args(args).output()?;
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).into_owned())
    } else {
        Err(SdmError::CommandFailed { program: program.to_string(), status: out.status.code() })
    }
}

/// C++ equivalent: `ServiceManager::listServices`, but returns rows instead of printing.
pub fn list_services() -> Result<Vec<ServiceInfo>> {
    #[cfg(windows)]
    let text = command_output("sc", &["query", "state=", "all"])?;

    #[cfg(all(unix, not(windows)))]
    let text = command_output("systemctl", &["list-units", "--type=service", "--all", "--no-pager"])?;

    Ok(text.lines().map(|line| ServiceInfo { raw_line: line.to_string() }).collect())
}

/// C++ equivalent: `ServiceManager::startService`.
pub fn start_service(service_name: &str) -> Result<()> {
    #[cfg(windows)]
    let status = Command::new("sc").arg("start").arg(service_name).status()?;

    #[cfg(all(unix, not(windows)))]
    let status = Command::new("systemctl").arg("start").arg(service_name).status()?;

    if status.success() { Ok(()) } else { Err(SdmError::CommandFailed { program: "service start".to_string(), status: status.code() }) }
}

/// C++ equivalent: `ServiceManager::stopService`.
pub fn stop_service(service_name: &str) -> Result<()> {
    #[cfg(windows)]
    let status = Command::new("sc").arg("stop").arg(service_name).status()?;

    #[cfg(all(unix, not(windows)))]
    let status = Command::new("systemctl").arg("stop").arg(service_name).status()?;

    if status.success() { Ok(()) } else { Err(SdmError::CommandFailed { program: "service stop".to_string(), status: status.code() }) }
}

/// C++ equivalent: `SelfCleanup::run`, blocked.
pub fn self_cleanup_run() -> Result<()> {
    Err(SdmError::Blocked("self-deleting executable behavior was not ported"))
}

/// C++ equivalent: `GhostKillSwitch::*`, blocked/no-op as appropriate.
pub fn trigger_kill_switch(_full_wipe: bool) -> Result<()> {
    Err(SdmError::Blocked("kill-switch / wipe automation was not ported"))
}

pub fn wipe_identities() -> Result<()> { Err(SdmError::Blocked("identity wipe automation was not ported")) }
pub fn wipe_configs() -> Result<()> { Err(SdmError::Blocked("config wipe automation was not ported")) }
pub fn terminate_tunnels() -> Result<()> { Err(SdmError::Blocked("covert tunnel termination was not ported")) }
pub fn self_delete() -> Result<()> { Err(SdmError::Blocked("self-delete was not ported")) }

/// C++ equivalent: `GhostDaemon` static methods, represented as service-control requests.
pub fn daemon_start(service_name: &str) -> Result<()> { start_service(service_name) }
pub fn daemon_stop(service_name: &str) -> Result<()> { stop_service(service_name) }
pub fn daemon_install(_service_name: &str) -> Result<()> { Err(SdmError::Blocked("daemon install/persistence was not ported")) }
pub fn daemon_uninstall(_service_name: &str) -> Result<()> { Err(SdmError::Blocked("daemon uninstall was not ported")) }

fn process_running(name: &str) -> bool {
    #[cfg(windows)]
    {
        if let Ok(out) = Command::new("tasklist").output() {
            return String::from_utf8_lossy(&out.stdout).to_lowercase().contains(&name.to_lowercase());
        }
        false
    }
    #[cfg(all(unix, not(windows)))]
    {
        Command::new("pgrep").arg("-x").arg(name).status().map(|s| s.success()).unwrap_or(false)
    }
}

/// C++ equivalent: `GhostWatchdog`.
pub struct Watchdog {
    watched_processes: Vec<String>,
    running: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl Default for Watchdog {
    fn default() -> Self {
        Self { watched_processes: Vec::new(), running: Arc::new(AtomicBool::new(false)), thread: None }
    }
}

impl Watchdog {
    pub fn new() -> Self { Self::default() }

    /// C++ equivalent: `GhostWatchdog::addWatchedProcess`.
    pub fn add_watched_process(&mut self, process_name: impl Into<String>) {
        self.watched_processes.push(process_name.into());
    }

    /// C++ equivalent: `GhostWatchdog::start`.
    pub fn start<F>(&mut self, on_missing: F)
    where
        F: Fn(String) + Send + Sync + 'static,
    {
        if self.running.load(Ordering::SeqCst) { return; }
        self.running.store(true, Ordering::SeqCst);
        let running = Arc::clone(&self.running);
        let watched = self.watched_processes.clone();
        let on_missing = Arc::new(on_missing);
        self.thread = Some(thread::spawn(move || {
            while running.load(Ordering::SeqCst) {
                for name in &watched {
                    if !process_running(name) {
                        on_missing(name.clone());
                    }
                }
                thread::sleep(Duration::from_secs(5));
            }
        }));
    }

    /// C++ equivalent: `GhostWatchdog::stop`.
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
        if let Some(handle) = self.thread.take() {
            let _ = handle.join();
        }
    }

    /// C++ equivalent: `GhostWatchdog::isProcessRunning`.
    pub fn is_process_running(&self, name: &str) -> bool { process_running(name) }
}

impl Drop for Watchdog {
    fn drop(&mut self) { self.stop(); }
}
