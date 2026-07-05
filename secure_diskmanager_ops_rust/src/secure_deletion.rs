use crate::error::{Result, SdmError};
use crate::rng;
use std::fs::{self, OpenOptions};
use std::io::{Seek, SeekFrom, Write};
use std::path::Path;

fn ensure_direct_regular_file(path: &Path) -> Result<u64> {
    let meta = fs::symlink_metadata(path)?;
    if meta.file_type().is_symlink() {
        return Err(SdmError::InvalidInput("refusing to shred symlink target".to_string()));
    }
    if !meta.is_file() {
        return Err(SdmError::InvalidInput("shred_file only accepts regular files".to_string()));
    }
    Ok(meta.len())
}

/// C++ equivalent: `SecureDeletion::shredFile`.
///
/// This overwrites a single regular file and removes it. It deliberately does
/// not recurse and deliberately refuses symlinks.
pub fn shred_file(path: impl AsRef<Path>, passes: usize) -> Result<()> {
    let path = path.as_ref();
    let size = ensure_direct_regular_file(path)?;
    let mut file = OpenOptions::new().read(true).write(true).open(path)?;

    for _ in 0..passes.max(1) {
        file.seek(SeekFrom::Start(0))?;
        let mut remaining = size;
        while remaining > 0 {
            let block_len = remaining.min(1024 * 1024) as usize;
            let block = rng::get_random_bytes(block_len)?;
            file.write_all(&block)?;
            remaining -= block_len as u64;
        }
        file.flush()?;
        file.sync_all()?;
    }
    drop(file);
    fs::remove_file(path)?;
    Ok(())
}

/// C++ equivalent: `SecureDeletion_SIMD::shredFile`.
/// First pass random, second `0xFF`, third and later `0x00`.
pub fn shred_file_simd_pattern(path: impl AsRef<Path>, passes: usize) -> Result<()> {
    let path = path.as_ref();
    let size = ensure_direct_regular_file(path)?;
    let mut file = OpenOptions::new().read(true).write(true).open(path)?;
    let passes = passes.max(1);

    for pass in 0..passes {
        file.seek(SeekFrom::Start(0))?;
        let mut remaining = size;
        while remaining > 0 {
            let block_len = remaining.min(1024 * 1024) as usize;
            let block = match pass {
                0 => rng::get_random_bytes(block_len)?,
                1 => vec![0xFF; block_len],
                _ => vec![0x00; block_len],
            };
            file.write_all(&block)?;
            remaining -= block_len as u64;
        }
        file.flush()?;
        file.sync_all()?;
    }
    drop(file);
    fs::remove_file(path)?;
    Ok(())
}
