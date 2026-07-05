use crate::error::{Result, SdmError};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn xor_crypt(data: &mut [u8], key: &[u8]) {
    if key.is_empty() { return; }
    for (idx, byte) in data.iter_mut().enumerate() {
        *byte ^= key[idx % key.len()];
    }
}

fn run_command(mut cmd: Command, program: &str) -> Result<()> {
    let status = cmd.status()?;
    if status.success() {
        Ok(())
    } else {
        Err(SdmError::CommandFailed { program: program.to_string(), status: status.code() })
    }
}

fn make_xor_temp(local_path: &Path, keyfile: &Path) -> Result<PathBuf> {
    let key = fs::read(keyfile)?;
    let mut data = fs::read(local_path)?;
    xor_crypt(&mut data, &key);
    let tmp = PathBuf::from(format!("{}.sdm-upload-xor.tmp", local_path.display()));
    fs::write(&tmp, data)?;
    Ok(tmp)
}

/// C++ equivalent: `FileTransferManagerResumable::upload`.
///
/// Uses the system `curl` binary for explicit user-provided destinations only.
pub fn upload<P: AsRef<Path>, K: AsRef<Path>>(local_path: P, remote_url: &str, keyfile: Option<K>, max_retries: usize) -> Result<()> {
    let local_path = local_path.as_ref();
    if !local_path.is_file() {
        return Err(SdmError::InvalidInput(format!("not a file: {}", local_path.display())));
    }

    let mut cleanup = None;
    let upload_path: PathBuf = if let Some(k) = keyfile {
        let tmp = make_xor_temp(local_path, k.as_ref())?;
        cleanup = Some(tmp.clone());
        tmp
    } else {
        local_path.to_path_buf()
    };

    let mut cmd = Command::new("curl");
    cmd.arg("--fail")
        .arg("--retry").arg(max_retries.to_string())
        .arg("--continue-at").arg("-")
        .arg("--upload-file").arg(&upload_path)
        .arg(remote_url);

    let result = run_command(cmd, "curl");
    if let Some(tmp) = cleanup {
        let _ = fs::remove_file(tmp);
    }
    result
}

/// C++ equivalent: `FileTransferManagerResumable::download`.
pub fn download<P: AsRef<Path>, K: AsRef<Path>>(remote_url: &str, local_path: P, keyfile: Option<K>, max_retries: usize) -> Result<()> {
    let local_path = local_path.as_ref();
    let mut cmd = Command::new("curl");
    cmd.arg("--fail")
        .arg("--location")
        .arg("--retry").arg(max_retries.to_string())
        .arg("--continue-at").arg("-")
        .arg("--output").arg(local_path)
        .arg(remote_url);
    run_command(cmd, "curl")?;

    if let Some(k) = keyfile {
        let key = fs::read(k)?;
        let mut data = fs::read(local_path)?;
        xor_crypt(&mut data, &key);
        fs::write(local_path, data)?;
    }
    Ok(())
}

/// C++ source had `FileUploader::send_via_smtp_tor`; this is blocked.
pub fn send_via_smtp_tor(
    _file_path: impl AsRef<Path>,
    _smtp_server: &str,
    _smtp_user: &str,
    _smtp_password: &str,
    _to_address: &str,
) -> Result<()> {
    Err(SdmError::Blocked(
        "covert/Tor SMTP payload sending was not ported; use explicit audited mail APIs outside this crate",
    ))
}

/// C++ source had `FileUploader::send_via_ftp_tor`; this is blocked.
pub fn send_via_ftp_tor(
    _file_path: impl AsRef<Path>,
    _ftp_server: &str,
    _ftp_user: &str,
    _ftp_password: &str,
    _remote_filename: &str,
) -> Result<()> {
    Err(SdmError::Blocked(
        "covert/Tor FTP payload sending was not ported; use explicit audited transfer APIs outside this crate",
    ))
}
