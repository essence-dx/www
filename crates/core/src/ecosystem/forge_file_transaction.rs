use anyhow::{Context, Result};
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Default)]
pub struct DxForgeFileTransaction {
    project_root: PathBuf,
    snapshots: Vec<DxForgeFileSnapshot>,
    committed: bool,
}

impl DxForgeFileTransaction {
    pub fn new(project_root: impl AsRef<Path>) -> Self {
        Self {
            project_root: project_root.as_ref().to_path_buf(),
            snapshots: Vec::new(),
            committed: false,
        }
    }

    pub fn snapshot_path(&mut self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref().to_path_buf();
        if self.snapshots.iter().any(|snapshot| snapshot.path == path) {
            return Ok(());
        }

        self.snapshots.push(DxForgeFileSnapshot::capture(path)?);
        Ok(())
    }

    pub fn write_bytes_atomic(&mut self, path: impl AsRef<Path>, bytes: &[u8]) -> Result<()> {
        let path = path.as_ref();
        self.snapshot_path(path)?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|| format!("create `{}`", parent.display()))?;
        }

        let temp_path = self.temp_path_for(path);
        if temp_path.exists() {
            fs::remove_file(&temp_path)
                .with_context(|| format!("remove stale temp `{}`", temp_path.display()))?;
        }

        fs::write(&temp_path, bytes)
            .with_context(|| format!("write temp `{}`", temp_path.display()))?;
        if let Err(first_error) = fs::rename(&temp_path, path) {
            if path.exists() {
                fs::remove_file(path).with_context(|| {
                    format!(
                        "replace `{}` after rename error: {first_error}",
                        path.display()
                    )
                })?;
                fs::rename(&temp_path, path).with_context(|| {
                    format!("rename `{}` to `{}`", temp_path.display(), path.display())
                })?;
            } else {
                let _ = fs::remove_file(&temp_path);
                return Err(first_error).with_context(|| {
                    format!("rename `{}` to `{}`", temp_path.display(), path.display())
                });
            }
        }
        Ok(())
    }

    pub fn write_json_pretty<T: Serialize>(
        &mut self,
        path: impl AsRef<Path>,
        value: &T,
    ) -> Result<()> {
        let json = serde_json::to_vec_pretty(value)?;
        self.write_bytes_atomic(path, &json)
    }

    pub fn commit(&mut self) {
        self.committed = true;
        self.snapshots.clear();
    }

    pub fn rollback(&mut self) -> Vec<String> {
        if self.committed {
            return Vec::new();
        }

        let mut findings = Vec::new();
        for snapshot in self.snapshots.iter().rev() {
            findings.extend(snapshot.restore(&self.project_root));
        }
        self.snapshots.clear();
        findings
    }

    fn temp_path_for(&self, path: &Path) -> PathBuf {
        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("forge-write");
        let temp_name = format!(
            ".{file_name}.dx-forge-txn-{}-{}.tmp",
            std::process::id(),
            self.snapshots.len()
        );
        path.with_file_name(temp_name)
    }
}

#[derive(Debug, Clone)]
struct DxForgeFileSnapshot {
    path: PathBuf,
    bytes: Option<Vec<u8>>,
}

impl DxForgeFileSnapshot {
    fn capture(path: PathBuf) -> Result<Self> {
        let bytes = if path.exists() {
            Some(
                fs::read(&path)
                    .with_context(|| format!("snapshot Forge file `{}`", path.display()))?,
            )
        } else {
            None
        };
        Ok(Self { path, bytes })
    }

    fn restore(&self, project_root: &Path) -> Vec<String> {
        match &self.bytes {
            Some(bytes) => self.restore_bytes(bytes),
            None => self.remove_created(project_root),
        }
    }

    fn restore_bytes(&self, bytes: &[u8]) -> Vec<String> {
        if let Some(parent) = self.path.parent() {
            if let Err(error) = fs::create_dir_all(parent) {
                return vec![format!(
                    "could not recreate `{}` during transaction rollback: {error}",
                    parent.display()
                )];
            }
        }

        fs::write(&self.path, bytes)
            .map(|()| Vec::new())
            .unwrap_or_else(|error| {
                vec![format!(
                    "could not restore `{}` during transaction rollback: {error}",
                    self.path.display()
                )]
            })
    }

    fn remove_created(&self, project_root: &Path) -> Vec<String> {
        match fs::remove_file(&self.path) {
            Ok(()) => {
                remove_empty_transaction_dirs(project_root, self.path.parent());
                Vec::new()
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Vec::new(),
            Err(error) => vec![format!(
                "could not remove `{}` during transaction rollback: {error}",
                self.path.display()
            )],
        }
    }
}

fn remove_empty_transaction_dirs(project_root: &Path, mut parent: Option<&Path>) {
    while let Some(path) = parent {
        if path == project_root {
            break;
        }
        match fs::remove_dir(path) {
            Ok(()) => parent = path.parent(),
            Err(_) => break,
        }
    }
}
