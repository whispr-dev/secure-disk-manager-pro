use crate::error::Result;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::{Duration, SystemTime};
use walkdir::WalkDir;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FsAction {
    Created,
    Modified,
}

/// C++ equivalent: `Ghost_FS_Monitor`.
pub struct FsMonitor {
    watched_path: PathBuf,
    running: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl FsMonitor {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            watched_path: path.as_ref().to_path_buf(),
            running: Arc::new(AtomicBool::new(false)),
            thread: None,
        }
    }

    /// C++ equivalent: `watchDirectory`.
    pub fn watch_directory<F>(&mut self, callback: F) -> Result<()>
    where
        F: Fn(PathBuf, FsAction) + Send + Sync + 'static,
    {
        if self.running.load(Ordering::SeqCst) {
            return Ok(());
        }
        fs::metadata(&self.watched_path)?;
        self.running.store(true, Ordering::SeqCst);
        let running = Arc::clone(&self.running);
        let root = self.watched_path.clone();
        let callback = Arc::new(callback);
        self.thread = Some(thread::spawn(move || {
            let mut known: HashMap<PathBuf, SystemTime> = HashMap::new();
            while running.load(Ordering::SeqCst) {
                for entry in WalkDir::new(&root).follow_links(false).into_iter().filter_map(std::result::Result::ok) {
                    let path = entry.path().to_path_buf();
                    let modified = match entry.metadata() {
                        Ok(meta) => match meta.modified() {
                            Ok(t) => t,
                            Err(_) => continue,
                        },
                        Err(_) => continue,
                    };
                    match known.get(&path).copied() {
                        None => {
                            known.insert(path.clone(), modified);
                            callback(path, FsAction::Created);
                        }
                        Some(old) if old != modified => {
                            known.insert(path.clone(), modified);
                            callback(path, FsAction::Modified);
                        }
                        _ => {}
                    }
                }
                thread::sleep(Duration::from_secs(2));
            }
        }));
        Ok(())
    }

    /// C++ equivalent: `stopWatching`.
    pub fn stop_watching(&mut self) {
        self.running.store(false, Ordering::SeqCst);
        if let Some(handle) = self.thread.take() {
            let _ = handle.join();
        }
    }
}

impl Drop for FsMonitor {
    fn drop(&mut self) {
        self.stop_watching();
    }
}
