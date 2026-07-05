use crate::error::{Result, SdmError};
use crate::fs_monitor::FsMonitor;
use crate::identity::{GhostIdentityManager, PersonaSwitcher};
use crate::net_admin::{GhostStealthNet, GhostTunnel};
use crate::process_services::Watchdog;
use crate::{net_admin, process_services};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::Duration;

#[derive(Debug)]
pub struct GhostAutoWipe {
    watch_path: PathBuf,
    delay: Duration,
    armed: Arc<AtomicBool>,
    worker: Option<JoinHandle<()>>,
}

impl GhostAutoWipe {
    /// C++ equivalent: `Ghost_AutoWipe::Ghost_AutoWipe`.
    pub fn new(path: impl AsRef<Path>, delay_seconds: u64) -> Self {
        Self {
            watch_path: path.as_ref().to_path_buf(),
            delay: Duration::from_secs(delay_seconds),
            armed: Arc::new(AtomicBool::new(false)),
            worker: None,
        }
    }

    /// C++ equivalent: `Ghost_AutoWipe::arm`.
    /// Deliberately blocked: timer-triggered wipe automation is not ported.
    pub fn arm(&mut self) -> Result<()> {
        let _ = (&self.watch_path, self.delay);
        self.armed.store(false, Ordering::SeqCst);
        Err(SdmError::Blocked("timer-triggered auto-wipe was not ported"))
    }

    /// C++ equivalent: `Ghost_AutoWipe::abort`.
    pub fn abort(&mut self) {
        self.armed.store(false, Ordering::SeqCst);
        if let Some(handle) = self.worker.take() {
            let _ = handle.join();
        }
    }
}

impl Drop for GhostAutoWipe {
    fn drop(&mut self) { self.abort(); }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum ControllerState {
    #[default]
    Stopped,
    Initialized,
    Running,
}

/// C++ equivalent: `GhostModeController`, with unsafe stealth/destructive
/// behaviors represented as blocked operations.
pub struct GhostModeController {
    pub state: ControllerState,
    pub identity_manager: GhostIdentityManager,
    pub persona_switcher: PersonaSwitcher,
    pub stealth_net: GhostStealthNet,
    pub tunnel: GhostTunnel,
    pub watchdog: Watchdog,
    pub fs_monitor: Option<FsMonitor>,
}

impl Default for GhostModeController {
    fn default() -> Self {
        Self {
            state: ControllerState::Stopped,
            identity_manager: GhostIdentityManager::new(),
            persona_switcher: PersonaSwitcher::new(),
            stealth_net: GhostStealthNet::new(),
            tunnel: GhostTunnel::new(),
            watchdog: Watchdog::new(),
            fs_monitor: None,
        }
    }
}

impl GhostModeController {
    pub fn new() -> Self { Self::default() }

    /// C++ equivalent: `initializeStealthOps` / `Init`.
    pub fn initialize_stealth_ops(&mut self) -> Result<()> {
        self.state = ControllerState::Initialized;
        Ok(())
    }

    pub fn init(&mut self) -> Result<()> { self.initialize_stealth_ops() }

    /// C++ equivalent: `configureWatchdog`.
    pub fn configure_watchdog(&mut self, process_names: &[String]) {
        for name in process_names {
            self.watchdog.add_watched_process(name.clone());
        }
    }

    /// C++ equivalent: `startAll` / `Run`.
    pub fn start_all(&mut self) -> Result<()> {
        self.state = ControllerState::Running;
        Ok(())
    }

    pub fn run(&mut self) -> Result<()> { self.start_all() }

    /// C++ equivalent: `stopAll`.
    pub fn stop_all(&mut self) {
        self.state = ControllerState::Stopped;
        self.tunnel.stop_tunnel();
        self.watchdog.stop();
        if let Some(mon) = &mut self.fs_monitor {
            mon.stop_watching();
        }
    }

    /// C++ equivalent: `checkStatus`.
    pub fn check_status(&self) -> ControllerState { self.state.clone() }

    /// C++ equivalent: `triggerKillSwitch`; blocked.
    pub fn trigger_kill_switch(&self) -> Result<()> { process_services::trigger_kill_switch(false) }

    pub fn install_daemon(&self, service_name: &str) -> Result<()> { process_services::daemon_install(service_name) }
    pub fn uninstall_daemon(&self, service_name: &str) -> Result<()> { process_services::daemon_uninstall(service_name) }
    pub fn start_daemon(&self, service_name: &str) -> Result<()> { process_services::daemon_start(service_name) }
    pub fn stop_daemon(&self, service_name: &str) -> Result<()> { process_services::daemon_stop(service_name) }

    /// C++ equivalent: `StartTunneling`; blocked by the tunnel module.
    pub fn start_tunneling(&mut self, endpoint: &str) -> Result<()> {
        let (host, port) = endpoint.split_once(':')
            .ok_or_else(|| SdmError::InvalidInput("endpoint must be host:port".to_string()))?;
        let port: u16 = port.parse().map_err(|_| SdmError::InvalidInput("invalid endpoint port".to_string()))?;
        self.tunnel.initialize(host, port, "")?;
        self.tunnel.start_tunnel()
    }

    pub fn stop_tunneling(&mut self) { self.tunnel.stop_tunnel(); }

    /// C++ equivalent: `RotateIdentity`, safe mode: advances active persona only.
    pub fn rotate_identity(&mut self) -> Result<()> {
        let personas = self.persona_switcher.list_personas()?;
        let next = personas.first().ok_or_else(|| SdmError::InvalidInput("no personas found".to_string()))?;
        self.persona_switcher.activate_persona(next)
    }

    pub fn generate_random_mac(&self) -> Result<String> { net_admin::generate_random_mac() }
}
