use crate::error::{Result, SdmError};
#[cfg(target_os = "linux")]
use std::fs;
#[cfg(target_os = "linux")]
use std::sync::Mutex;

#[cfg(target_os = "linux")]
static LAST_CPU: Mutex<Option<(u64, u64)>> = Mutex::new(None);

/// C++ equivalent: `SystemMonitor::getCpuUsage`.
///
/// Unix implementation reads `/proc/stat`. First call returns `0.0` because a
/// delta sample is required. Other platforms return `UnsupportedPlatform`.
pub fn get_cpu_usage() -> Result<f64> {
    #[cfg(target_os = "linux")]
    {
        let text = fs::read_to_string("/proc/stat")?;
        let first = text.lines().next().ok_or_else(|| SdmError::InvalidFormat("empty /proc/stat".to_string()))?;
        let values: Vec<u64> = first.split_whitespace().skip(1).filter_map(|v| v.parse().ok()).collect();
        if values.len() < 4 {
            return Err(SdmError::InvalidFormat("unexpected /proc/stat cpu row".to_string()));
        }
        let idle = values.get(3).copied().unwrap_or(0) + values.get(4).copied().unwrap_or(0);
        let total: u64 = values.iter().sum();
        let mut last = LAST_CPU.lock().expect("cpu mutex poisoned");
        let usage = if let Some((last_idle, last_total)) = *last {
            let delta_idle = idle.saturating_sub(last_idle);
            let delta_total = total.saturating_sub(last_total);
            if delta_total == 0 { 0.0 } else { 100.0 * (1.0 - (delta_idle as f64 / delta_total as f64)) }
        } else {
            0.0
        };
        *last = Some((idle, total));
        return Ok(usage);
    }
    #[cfg(not(target_os = "linux"))]
    Err(SdmError::UnsupportedPlatform("CPU usage currently implemented for Linux /proc/stat"))
}

/// C++ equivalent: `SystemMonitor::getMemoryStats`.
pub fn get_memory_stats() -> Result<(u64, u64)> {
    #[cfg(target_os = "linux")]
    {
        let text = fs::read_to_string("/proc/meminfo")?;
        let mut total = 0_u64;
        let mut available = 0_u64;
        for line in text.lines() {
            if line.starts_with("MemTotal:") {
                total = line.split_whitespace().nth(1).and_then(|v| v.parse::<u64>().ok()).unwrap_or(0) * 1024;
            } else if line.starts_with("MemAvailable:") {
                available = line.split_whitespace().nth(1).and_then(|v| v.parse::<u64>().ok()).unwrap_or(0) * 1024;
            }
        }
        return Ok((total, available));
    }
    #[cfg(not(target_os = "linux"))]
    Err(SdmError::UnsupportedPlatform("memory stats currently implemented for Linux /proc/meminfo"))
}
