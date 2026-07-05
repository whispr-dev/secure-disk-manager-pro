use crate::error::Result;
use std::fs;
use std::path::Path;

/// C++ equivalent: `DiskManagement::getDiskUsage`.
///
/// The source code only summed immediate non-directory files in `path`; it did
/// not recurse. This intentionally preserves that behavior.
pub fn get_disk_usage(path: impl AsRef<Path>) -> Result<u64> {
    let mut total = 0_u64;
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let meta = entry.metadata()?;
        if meta.is_file() {
            total = total.saturating_add(meta.len());
        }
    }
    Ok(total)
}
