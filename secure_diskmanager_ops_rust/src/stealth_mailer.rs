use crate::error::{Result, SdmError};
use std::path::PathBuf;

/// C++ equivalent: `StealthMailer::MailerConfig`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MailerConfig {
    pub payload_path: PathBuf,
    pub smtp_server: String,
    pub smtp_user: String,
    pub smtp_password: String,
    pub to_address: String,
    pub use_tor: bool,
    pub spoof_mac: bool,
    pub shred_after_send: bool,
    pub require_vpn: bool,
}

/// C++ equivalent: `StealthMailer::send_payload_securely`.
///
/// Blocked: the source routine combined credential extraction, Tor SMTP upload,
/// MAC spoofing and optional shredding.
pub fn send_payload_securely(_config: &MailerConfig) -> Result<()> {
    Err(SdmError::Blocked("stealth payload mailer was not ported"))
}

/// C++ equivalent: `StealthMailer::verify_tor_connection`.
pub fn verify_tor_connection() -> Result<bool> {
    Err(SdmError::Blocked("Tor verification for covert mailer was not ported"))
}

/// C++ equivalent: `StealthMailer::decrypt_credentials`.
pub fn decrypt_credentials(_enc_file_path: &str) -> Result<(String, String)> {
    Err(SdmError::Blocked("mailer credential decryption helper was not ported"))
}
