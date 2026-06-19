//! # File Watcher
//!
//! Watches source files for changes and triggers rebuilds.

use std::path::{Path, PathBuf};
use std::sync::mpsc::{Receiver, channel};
use std::time::Duration;

use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};

use crate::error::{DxError, DxResult};

// =============================================================================
// File Watcher
// =============================================================================

const DX_DEV_WATCH_DIRS: [&str; 16] = [
    "app",
    "src/app",
    "pages",
    "src/pages",
    "components",
    "server",
    "api",
    "styles",
    "public",
    "forge",
    ".dx/forge",
    ".dx/diagnostics",
    ".dx/style",
    ".dx/icons",
    ".dx/imports",
    ".dx/check",
];

/// Watches files for changes.
pub struct FileWatcher {
    /// Project root used to discover source directories created after startup
    project_root: PathBuf,
    /// The underlying notify watcher
    watcher: RecommendedWatcher,
    /// Channel receiver for events
    rx: Receiver<notify::Result<Event>>,
    /// Watched paths
    watched_paths: Vec<PathBuf>,
}

impl FileWatcher {
    /// Create a new file watcher.
    pub fn new(root: &Path) -> DxResult<Self> {
        let (tx, rx) = channel();

        let watcher = RecommendedWatcher::new(
            move |res| {
                let _ = tx.send(res);
            },
            Config::default().with_poll_interval(Duration::from_millis(100)),
        )
        .map_err(|e| DxError::IoError {
            path: Some(root.to_path_buf()),
            message: format!("Failed to create watcher: {}", e),
        })?;

        let mut fw = Self {
            project_root: root.to_path_buf(),
            watcher,
            rx,
            watched_paths: Vec::new(),
        };

        fw.watch_project_root()?;
        fw.refresh_watch_directories()?;

        Ok(fw)
    }

    fn watch_project_root(&mut self) -> DxResult<()> {
        if self.project_root.exists() {
            self.watcher
                .watch(&self.project_root, RecursiveMode::NonRecursive)
                .map_err(|e| DxError::IoError {
                    path: Some(self.project_root.clone()),
                    message: format!("Failed to watch project root: {}", e),
                })?;
        }
        Ok(())
    }

    fn refresh_watch_directories(&mut self) -> DxResult<()> {
        for watch_dir in DX_DEV_WATCH_DIRS {
            let path = self.project_root.join(watch_dir);
            self.watch_directory(&path)?;
        }
        Ok(())
    }

    /// Watch a directory recursively.
    pub fn watch_directory(&mut self, path: &Path) -> DxResult<()> {
        if path.exists() && !self.watched_paths.iter().any(|watched| watched == path) {
            self.watcher
                .watch(path, RecursiveMode::Recursive)
                .map_err(|e| DxError::IoError {
                    path: Some(path.to_path_buf()),
                    message: format!("Failed to watch: {}", e),
                })?;
            self.watched_paths.push(path.to_path_buf());
        }
        Ok(())
    }

    /// Unwatch a directory.
    pub fn unwatch_directory(&mut self, path: &Path) -> DxResult<()> {
        self.watcher.unwatch(path).map_err(|e| DxError::IoError {
            path: Some(path.to_path_buf()),
            message: format!("Failed to unwatch: {}", e),
        })?;
        self.watched_paths.retain(|p| p != path);
        Ok(())
    }

    /// Poll for file change events.
    pub fn poll(&mut self) -> Option<FileChangeEvent> {
        match self.rx.try_recv() {
            Ok(Ok(event)) => {
                if let Err(error) = self.refresh_watch_directories() {
                    eprintln!("DX-WWW hot reload watcher could not adopt a source root: {error}");
                }

                let paths: Vec<PathBuf> = event.paths;
                if paths.is_empty() {
                    return None;
                }

                let kind = match event.kind {
                    notify::EventKind::Create(_) => ChangeKind::Created,
                    notify::EventKind::Modify(_) => ChangeKind::Modified,
                    notify::EventKind::Remove(_) => ChangeKind::Deleted,
                    _ => return None,
                };

                Some(FileChangeEvent { paths, kind })
            }
            _ => None,
        }
    }

    /// Get the list of watched paths.
    pub fn watched_paths(&self) -> &[PathBuf] {
        &self.watched_paths
    }

    /// Stop the watcher.
    pub fn stop(self) -> DxResult<()> {
        // Watcher is dropped automatically
        Ok(())
    }
}

/// Returns true when a watched path should invalidate DX-WWW dev output.
pub fn is_meaningful_dev_change(root: &Path, path: &Path) -> bool {
    let relative = path.strip_prefix(root).unwrap_or(path);
    let parts = relative
        .components()
        .filter_map(|component| component.as_os_str().to_str())
        .map(|part| part.replace('\\', "/"))
        .collect::<Vec<_>>();

    if parts.is_empty() || parts.iter().any(|part| part == "node_modules") {
        return false;
    }

    if path_file_name_is(relative, "dx") {
        return true;
    }

    if let Some(first) = parts.first() {
        if matches!(
            first.as_str(),
            "app" | "pages" | "components" | "server" | "api" | "styles" | "public" | "forge"
        ) {
            return true;
        }
    }

    if parts.len() >= 2 && parts[0] == "src" && parts[1] == "app" {
        return true;
    }

    if parts.len() >= 2 && parts[0] == "src" && parts[1] == "pages" {
        return true;
    }

    if parts.len() >= 2 && parts[0] == ".dx" {
        return !matches!(
            parts[1].as_str(),
            "serializer" | "run" | "receipts" | "www" | "build"
        ) && (parts[1] == "forge"
            || parts[1] == "diagnostics"
            || serializer_source_change(relative));
    }

    false
}

fn path_file_name_is(path: &Path, expected: &str) -> bool {
    path.file_name().and_then(|name| name.to_str()) == Some(expected)
}

fn serializer_source_change(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("sr"))
}

// =============================================================================
// File Change Event
// =============================================================================

/// A file change event.
#[derive(Debug, Clone)]
pub struct FileChangeEvent {
    /// Paths that changed
    pub paths: Vec<PathBuf>,
    /// Kind of change
    pub kind: ChangeKind,
}

/// Kind of file change.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeKind {
    /// File was created
    Created,
    /// File was modified
    Modified,
    /// File was deleted
    Deleted,
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_kind_eq() {
        assert_eq!(ChangeKind::Created, ChangeKind::Created);
        assert_ne!(ChangeKind::Created, ChangeKind::Modified);
    }

    #[test]
    fn meaningful_dev_change_matches_source_and_forge_paths() {
        let root = Path::new("G:/Dx/www-app");

        assert!(is_meaningful_dev_change(
            root,
            &root.join("pages/index.html")
        ));
        assert!(is_meaningful_dev_change(
            root,
            &root.join("components/Hero.tsx")
        ));
        assert!(is_meaningful_dev_change(
            root,
            &root.join("src/app/page.tsx")
        ));
        assert!(is_meaningful_dev_change(
            root,
            &root.join("src/pages/docs/[slug].tsx")
        ));
        assert!(is_meaningful_dev_change(
            root,
            &root.join("styles/dx-landing.css")
        ));
        assert!(is_meaningful_dev_change(
            root,
            &root.join("public/favicon.svg")
        ));
        assert!(is_meaningful_dev_change(
            root,
            &root.join(".dx/forge/route-discovery/routes.json")
        ));
        assert!(is_meaningful_dev_change(
            root,
            &root.join(".dx/diagnostics/latest.json")
        ));
        assert!(is_meaningful_dev_change(
            root,
            &root.join("forge/source-surfaces/dx-landing.json")
        ));

        assert!(!is_meaningful_dev_change(
            root,
            &root.join(".dx/serializer/dx.machine")
        ));
        assert!(!is_meaningful_dev_change(
            root,
            &root.join("node_modules/react/index.js")
        ));
    }
}
