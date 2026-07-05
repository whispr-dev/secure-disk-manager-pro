use crate::error::{Result, SdmError};
use std::fs;
use std::path::{Path, PathBuf};

fn append_suffix(path: impl AsRef<Path>, suffix: &str) -> PathBuf {
    PathBuf::from(format!("{}{}", path.as_ref().display(), suffix))
}

/// Equivalent to `FileEncryption::initializeSBox`.
pub fn initialize_sbox(key: &[u8]) -> Result<[u8; 256]> {
    if key.is_empty() {
        return Err(SdmError::Crypto("RC4 key must not be empty".to_string()));
    }

    let mut sbox = [0_u8; 256];
    for (i, item) in sbox.iter_mut().enumerate() {
        *item = i as u8;
    }

    let mut j = 0_usize;
    for i in 0..256 {
        j = (j + sbox[i] as usize + key[i % key.len()] as usize) & 0xFF;
        sbox.swap(i, j);
    }
    Ok(sbox)
}

/// Equivalent to `FileEncryption::rc4EncryptDecrypt`.
///
/// This is a legacy compatibility primitive. RC4 is not modern secure crypto.
pub fn rc4_encrypt_decrypt(data: &mut [u8], key: &[u8]) -> Result<()> {
    let mut sbox = initialize_sbox(key)?;
    let mut i = 0_usize;
    let mut j = 0_usize;

    for byte in data.iter_mut() {
        i = (i + 1) & 0xFF;
        j = (j + sbox[i] as usize) & 0xFF;
        sbox.swap(i, j);
        let k = sbox[(sbox[i] as usize + sbox[j] as usize) & 0xFF];
        *byte ^= k;
    }
    Ok(())
}

/// Equivalent to `FileEncryption::encryptFile`; writes `<file>.enc`.
pub fn encrypt_file(file_path: impl AsRef<Path>, key: &str) -> Result<PathBuf> {
    let mut data = fs::read(&file_path)?;
    rc4_encrypt_decrypt(&mut data, key.as_bytes())?;
    let out = append_suffix(file_path, ".enc");
    fs::write(&out, data)?;
    Ok(out)
}

/// Equivalent to `FileEncryption::decryptFile`; writes `<file>.dec`.
pub fn decrypt_file(file_path: impl AsRef<Path>, key: &str) -> Result<PathBuf> {
    let mut data = fs::read(&file_path)?;
    rc4_encrypt_decrypt(&mut data, key.as_bytes())?;
    let out = append_suffix(file_path, ".dec");
    fs::write(&out, data)?;
    Ok(out)
}

/// Equivalent to `FileEncryption::encryptFileWithKeyfile`; writes `<file>.enc`.
pub fn encrypt_file_with_keyfile(file_path: impl AsRef<Path>, keyfile_path: impl AsRef<Path>) -> Result<PathBuf> {
    let key = fs::read(keyfile_path)?;
    let mut data = fs::read(&file_path)?;
    rc4_encrypt_decrypt(&mut data, &key)?;
    let out = append_suffix(file_path, ".enc");
    fs::write(&out, data)?;
    Ok(out)
}

/// Equivalent to `FileEncryption::decryptFileWithKeyfile`; writes `<file>.dec`.
pub fn decrypt_file_with_keyfile(file_path: impl AsRef<Path>, keyfile_path: impl AsRef<Path>) -> Result<PathBuf> {
    let key = fs::read(keyfile_path)?;
    let mut data = fs::read(&file_path)?;
    rc4_encrypt_decrypt(&mut data, &key)?;
    let out = append_suffix(file_path, ".dec");
    fs::write(&out, data)?;
    Ok(out)
}
