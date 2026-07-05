use crate::error::Result;
use crate::rng;
use crate::secure_deletion;
use std::fs;
use std::path::{Path, PathBuf};

pub fn get_key_directory() -> Result<PathBuf> {
    let dir = PathBuf::from("./keys");
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

fn key_path(name: &str) -> Result<PathBuf> {
    let mut path = get_key_directory()?;
    path.push(name);
    Ok(path)
}

/// C++ equivalent: `KeyManager::generateKeyfile`.
pub fn generate_keyfile(name: &str, size: usize) -> Result<PathBuf> {
    let path = key_path(name)?;
    rng::write_keyfile(&path, size)?;
    Ok(path)
}

/// C++ equivalent: `KeyManager::deleteKeyfile`.
pub fn delete_keyfile(name: &str) -> Result<()> {
    secure_deletion::shred_file_simd_pattern(key_path(name)?, 3)
}

/// C++ equivalent: `KeyManager::loadKeyfile`.
pub fn load_keyfile(name: &str) -> Result<Vec<u8>> {
    Ok(fs::read(key_path(name)?)?)
}

/// C++ equivalent: `KeyManager::listKeyfiles`, but returns names instead of printing.
pub fn list_keyfiles() -> Result<Vec<String>> {
    let dir = get_key_directory()?;
    let mut names = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            names.push(entry.file_name().to_string_lossy().into_owned());
        }
    }
    names.sort();
    Ok(names)
}

#[allow(dead_code)]
fn _ensure_path(_: impl AsRef<Path>) {}
