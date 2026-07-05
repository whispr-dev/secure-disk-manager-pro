use crate::error::{Result, SdmError};
use std::path::Path;
use std::process::Command;

fn run(program: &str, args: &[String]) -> Result<()> {
    let status = Command::new(program).args(args).status()?;
    if status.success() {
        Ok(())
    } else {
        Err(SdmError::CommandFailed { program: program.to_string(), status: status.code() })
    }
}

/// C++ equivalent: `GpgWrapper::encryptFile`.
pub fn encrypt_file(filepath: impl AsRef<Path>, recipient_key_id: &str) -> Result<()> {
    run("gpg", &[
        "--yes".to_string(),
        "--encrypt".to_string(),
        "--recipient".to_string(),
        recipient_key_id.to_string(),
        filepath.as_ref().to_string_lossy().into_owned(),
    ])
}

/// C++ equivalent: `GpgWrapper::decryptFile`.
pub fn decrypt_file(filepath: impl AsRef<Path>, output_path: impl AsRef<Path>) -> Result<()> {
    run("gpg", &[
        "--yes".to_string(),
        "--output".to_string(),
        output_path.as_ref().to_string_lossy().into_owned(),
        "--decrypt".to_string(),
        filepath.as_ref().to_string_lossy().into_owned(),
    ])
}

/// C++ equivalent: `GpgWrapper::importKey`.
pub fn import_key(keyfile: impl AsRef<Path>) -> Result<()> {
    run("gpg", &["--import".to_string(), keyfile.as_ref().to_string_lossy().into_owned()])
}

/// C++ equivalent: `GpgWrapper::keyExists`.
pub fn key_exists(key_id: &str) -> Result<bool> {
    let status = Command::new("gpg").arg("--list-keys").arg(key_id).status()?;
    Ok(status.success())
}
