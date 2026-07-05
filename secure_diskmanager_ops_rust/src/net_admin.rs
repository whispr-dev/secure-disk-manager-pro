use crate::error::{Result, SdmError};
use crate::rng;
#[cfg(unix)]
use std::fs;
#[cfg(unix)]
use std::path::PathBuf;
use std::process::Command;

/// C++ equivalent: `Ghost_NetUtils::generateRandomMac` / `MacSpoofer::generateRandomMac`.
pub fn generate_random_mac() -> Result<String> {
    let mut mac = rng::get_random_bytes(6)?;
    mac[0] = (mac[0] & 0xFC) | 0x02; // locally administered, unicast
    Ok(mac.iter().map(|b| format!("{b:02x}")).collect::<Vec<_>>().join(":"))
}

/// C++ equivalent: `MacSpoofer::getCurrentMac`.
pub fn get_current_mac(interface_name: &str) -> Result<String> {
    #[cfg(unix)]
    {
        let path = PathBuf::from("/sys/class/net").join(interface_name).join("address");
        return Ok(fs::read_to_string(path)?.trim().to_string());
    }
    #[cfg(not(unix))]
    {
        let _ = interface_name;
        Err(SdmError::UnsupportedPlatform("reading MAC address is implemented for Unix-like /sys only"))
    }
}

/// C++ equivalent names: `MacSpoofer::spoofMac` and `Ghost_NetUtils::changeMacAddress`.
///
/// Deliberately blocked: MAC spoofing is not a safe default OS utility primitive.
pub fn spoof_mac(_interface_name: &str, _new_mac: &str) -> Result<()> {
    Err(SdmError::Blocked("MAC spoofing/changing was not ported"))
}

pub fn change_mac_address(interface_name: &str, new_mac: &str) -> Result<()> {
    spoof_mac(interface_name, new_mac)
}

#[derive(Debug, Default, Clone)]
pub struct GhostStealthNet {
    pub dns_cloak_requested: bool,
    pub current_proxy: Option<String>,
    pub heartbeat_count: u64,
}

impl GhostStealthNet {
    pub fn new() -> Self { Self::default() }

    /// C++ equivalent: `GhostStealthNet::enableDnsCloak`.
    /// Records intent only; does not mutate host DNS.
    pub fn enable_dns_cloak(&mut self) {
        self.dns_cloak_requested = true;
    }

    /// C++ equivalent: `GhostStealthNet::setupProxyTunnel`.
    /// Records explicit proxy configuration only; no hidden tunnelling.
    pub fn setup_proxy_tunnel(&mut self, proxy_address: impl Into<String>) {
        self.current_proxy = Some(proxy_address.into());
    }

    /// C++ equivalent: `GhostStealthNet::heartbeat`.
    pub fn heartbeat(&mut self) {
        self.heartbeat_count = self.heartbeat_count.saturating_add(1);
    }
}

#[derive(Debug, Default, Clone)]
pub struct GhostTunnel {
    pub proxy_address: Option<String>,
    pub proxy_port: Option<u16>,
    pub encryption_key_set: bool,
    running: bool,
}

impl GhostTunnel {
    pub fn new() -> Self { Self::default() }

    /// C++ equivalent: `GhostTunnel::initialize`.
    pub fn initialize(&mut self, proxy_address: impl Into<String>, proxy_port: u16, encryption_key: &str) -> Result<()> {
        self.proxy_address = Some(proxy_address.into());
        self.proxy_port = Some(proxy_port);
        self.encryption_key_set = !encryption_key.is_empty();
        Ok(())
    }

    /// C++ equivalent: `GhostTunnel::startTunnel`; blocked because the source
    /// encrypted and relayed traffic through a stealth tunnel.
    pub fn start_tunnel(&mut self) -> Result<()> {
        self.running = false;
        Err(SdmError::Blocked("stealth encrypted traffic relay was not ported"))
    }

    /// C++ equivalent: `GhostTunnel::stopTunnel`.
    pub fn stop_tunnel(&mut self) {
        self.running = false;
    }

    /// C++ equivalent: `GhostTunnel::isRunning`.
    pub fn is_running(&self) -> bool { self.running }
}

/// C++ equivalent: `GhostVPNManager::start_vpn_connection`.
///
/// This starts an explicitly named VPN profile using host tools. It does not
/// hide routing or chain proxy systems.
pub fn start_vpn_connection(profile_name: &str) -> Result<()> {
    #[cfg(windows)]
    let mut cmd = {
        let mut c = Command::new("rasdial");
        c.arg(profile_name);
        c
    };

    #[cfg(all(unix, not(windows)))]
    let mut cmd = {
        let mut c = Command::new("nmcli");
        c.arg("connection").arg("up").arg(profile_name);
        c
    };

    let status = cmd.status()?;
    if status.success() {
        Ok(())
    } else {
        Err(SdmError::CommandFailed { program: "vpn profile start".to_string(), status: status.code() })
    }
}

/// C++ equivalent: `GhostVPNManager::is_vpn_active`.
pub fn is_vpn_active() -> Result<bool> {
    #[cfg(windows)]
    {
        let out = Command::new("rasdial").output()?;
        return Ok(String::from_utf8_lossy(&out.stdout).to_lowercase().contains("connected"));
    }
    #[cfg(all(unix, not(windows)))]
    {
        let out = Command::new("nmcli").args(["connection", "show", "--active"]).output()?;
        let text = String::from_utf8_lossy(&out.stdout).to_lowercase();
        return Ok(text.contains("vpn"));
    }
    #[allow(unreachable_code)]
    Err(SdmError::UnsupportedPlatform("VPN status check unsupported on this platform"))
}
