use crate::error::{Result, SdmError};
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Equivalent to `RNG::initialize` / `RNG_SIMD::initialize`.
/// `getrandom` does not need explicit initialization, so this validates that
/// OS randomness is available.
pub fn initialize() -> Result<()> {
    let mut probe = [0_u8; 1];
    getrandom::getrandom(&mut probe).map_err(|e| SdmError::Crypto(e.to_string()))
}

pub fn get_random_bytes(size: usize) -> Result<Vec<u8>> {
    let mut data = vec![0_u8; size];
    getrandom::getrandom(&mut data).map_err(|e| SdmError::Crypto(e.to_string()))?;
    Ok(data)
}

pub fn get_random_64() -> Result<u64> {
    let bytes = get_random_bytes(8)?;
    Ok(u64::from_le_bytes(bytes.try_into().expect("fixed length")))
}

pub fn get_random_in_range(min: u64, max: u64) -> Result<u64> {
    if min >= max {
        return Ok(min);
    }
    let span = max - min + 1;
    Ok(min + (get_random_64()? % span))
}

pub fn write_keyfile(path: impl AsRef<Path>, num_bytes: usize) -> Result<()> {
    let key = get_random_bytes(num_bytes)?;
    let mut out = File::create(path)?;
    out.write_all(&key)?;
    out.sync_all()?;
    Ok(())
}
