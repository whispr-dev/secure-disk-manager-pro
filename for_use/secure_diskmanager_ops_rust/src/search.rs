use crate::entropy;
use crate::error::Result;
use regex::RegexBuilder;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub path: PathBuf,
    pub entropy_score: f64,
}

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub full_path: PathBuf,
    pub size: u64,
    pub modified_time_unix: u64,
}

/// C++ equivalent: `SearchEngine::findMatchingFiles`.
pub fn find_matching_files(root_path: impl AsRef<Path>, pattern: &str, use_entropy: bool) -> Result<Vec<SearchResult>> {
    let regex = RegexBuilder::new(pattern).case_insensitive(true).build()
        .map_err(|e| crate::error::SdmError::InvalidInput(e.to_string()))?;
    let mut results = Vec::new();

    for entry in WalkDir::new(root_path).follow_links(false).into_iter().filter_map(std::result::Result::ok) {
        if !entry.file_type().is_file() {
            continue;
        }
        let path_string = entry.path().to_string_lossy();
        if regex.is_match(&path_string) {
            let entropy_score = if use_entropy {
                entropy::calculate_file_entropy(entry.path()).unwrap_or(0.0)
            } else {
                0.0
            };
            results.push(SearchResult { path: entry.path().to_path_buf(), entropy_score });
        }
    }
    Ok(results)
}

/// C++ equivalent: `SearchEngine::printResults`, but returns a string for callers.
pub fn format_search_results(results: &[SearchResult]) -> String {
    let mut out = String::new();
    for result in results {
        if result.entropy_score > 0.0 {
            out.push_str(&format!("{} | Entropy: {:.6}\n", result.path.display(), result.entropy_score));
        } else {
            out.push_str(&format!("{}\n", result.path.display()));
        }
    }
    out
}

/// C++ equivalent: `UltraFastSearch::search`.
pub fn ultra_fast_search(pattern: &str, root_path: impl AsRef<Path>) -> Result<Vec<FileEntry>> {
    let mut results = Vec::new();
    for entry in WalkDir::new(root_path).follow_links(false).into_iter().filter_map(std::result::Result::ok) {
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        if path.to_string_lossy().contains(pattern) {
            let meta = fs::metadata(path)?;
            let modified_time_unix = meta.modified()
                .ok()
                .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                .map(|d| d.as_secs())
                .unwrap_or(0);
            results.push(FileEntry {
                full_path: path.to_path_buf(),
                size: meta.len(),
                modified_time_unix,
            });
        }
    }
    Ok(results)
}

/// C++ equivalent: `UltraFastSearch::printResults`, but returns a string.
pub fn format_file_entries(results: &[FileEntry]) -> String {
    let mut out = String::new();
    for entry in results {
        out.push_str(&format!(
            "{} | Size: {} bytes | ModifiedUnix: {}\n",
            entry.full_path.display(), entry.size, entry.modified_time_unix
        ));
    }
    out
}

/// C++ equivalent: `UltraFastSearch::exportToCSV`.
pub fn export_to_csv(results: &[FileEntry], out_file: impl AsRef<Path>) -> Result<()> {
    let mut out = File::create(out_file)?;
    writeln!(out, "Path,Size,ModifiedUnix")?;
    for entry in results {
        let escaped = entry.full_path.to_string_lossy().replace('"', "\"\"");
        writeln!(out, "\"{}\",{},{}", escaped, entry.size, entry.modified_time_unix)?;
    }
    Ok(())
}

#[allow(dead_code)]
fn _system_time_to_unix(t: SystemTime) -> u64 {
    t.duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}
