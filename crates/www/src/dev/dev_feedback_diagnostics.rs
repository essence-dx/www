//! Diagnostics artifact freshness helpers for the DX dev feedback surface.

use std::path::{Component, Path};
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::{Value, json};
use walkdir::WalkDir;

pub(super) const DX_DEV_FEEDBACK_DIAGNOSTICS_LATEST_PATH: &str = ".dx/diagnostics/latest.json";
pub(super) const DX_DEV_FEEDBACK_CHECK_LATEST_PATH: &str = ".dx/receipts/check/check-latest.json";

#[derive(Debug, Clone, PartialEq, Eq)]
struct DxDevFeedbackSourceArtifact {
    path: String,
    modified: SystemTime,
    modified_unix_ms: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxDevFeedbackDiagnosticsArtifact {
    pub(super) path: &'static str,
    pub(super) status: &'static str,
    pub(super) present: bool,
    pub(super) modified_unix_ms: Option<u64>,
    pub(super) newest_source_path: Option<String>,
    pub(super) newest_source_modified_unix_ms: Option<u64>,
}

impl DxDevFeedbackDiagnosticsArtifact {
    pub(super) fn to_json(&self) -> Value {
        json!({
            "path": self.path,
            "status": self.status,
            "present": self.present,
            "modified_unix_ms": self.modified_unix_ms,
            "newest_source_path": self.newest_source_path.as_deref(),
            "newest_source_modified_unix_ms": self.newest_source_modified_unix_ms,
            "source_owned_contract": true,
            "node_modules_required": false,
            "node_modules_scanned": false,
        })
    }
}

pub(super) fn diagnostics_artifact_status(
    project_root: &Path,
    diagnostics_path: &Path,
) -> DxDevFeedbackDiagnosticsArtifact {
    let diagnostics_metadata = std::fs::metadata(diagnostics_path).ok();
    let present = diagnostics_metadata
        .as_ref()
        .is_some_and(|metadata| metadata.is_file());
    let modified = diagnostics_metadata
        .as_ref()
        .and_then(|metadata| metadata.modified().ok());
    let modified_unix_ms = modified
        .as_ref()
        .and_then(|modified| system_time_unix_ms(*modified));
    let newest_source = newest_source_artifact(project_root);
    let is_stale = match (modified.as_ref(), newest_source.as_ref()) {
        (Some(diagnostics_modified), Some(source)) => source.modified > *diagnostics_modified,
        _ => false,
    };
    let status = if !present {
        "missing"
    } else if is_stale {
        "stale"
    } else {
        "current"
    };

    DxDevFeedbackDiagnosticsArtifact {
        path: DX_DEV_FEEDBACK_DIAGNOSTICS_LATEST_PATH,
        status,
        present,
        modified_unix_ms,
        newest_source_path: newest_source.as_ref().map(|source| source.path.clone()),
        newest_source_modified_unix_ms: newest_source
            .as_ref()
            .and_then(|source| source.modified_unix_ms),
    }
}

pub(super) fn diagnostic_artifact_issue(
    artifact: &DxDevFeedbackDiagnosticsArtifact,
) -> Option<Value> {
    match artifact.status {
        "missing" => Some(json!({
            "severity": "info",
            "diagnostic_code": "dx.dev_feedback.diagnostics_missing",
            "title": "DX diagnostics artifact missing",
            "message": "DX has not written .dx/diagnostics/latest.json yet, so the overlay cannot prove the latest source state.",
            "next_action": "Run a DX check/build command or save a source file so the Rust dev server can publish fresh diagnostics.",
            "code_frame": Value::Null,
            "code_frame_adapter_boundary": "diagnostics-artifact-missing",
            "source_owned_contract": true,
            "node_modules_required": false,
        })),
        "stale" => Some(json!({
            "severity": "warning",
            "diagnostic_code": "dx.dev_feedback.diagnostics_stale",
            "title": "DX diagnostics artifact is stale",
            "message": "DX diagnostics are older than a source file, so the overlay may be hiding a newer route, build, or style failure.",
            "next_action": "Refresh diagnostics with the DX Rust pipeline before trusting this overlay state.",
            "file": artifact.newest_source_path.as_deref(),
            "line": 1,
            "column": 1,
            "code_frame_adapter_boundary": "diagnostics-artifact-stale",
            "source_owned_contract": true,
            "node_modules_required": false,
        })),
        _ => None,
    }
}

fn newest_source_artifact(project_root: &Path) -> Option<DxDevFeedbackSourceArtifact> {
    let mut newest: Option<DxDevFeedbackSourceArtifact> = None;
    let mut visited = 0usize;

    for root in ["app", "src/app", "pages", "src/pages", "styles", "public"] {
        let absolute_root = project_root.join(root);
        if !absolute_root.exists() {
            continue;
        }

        for entry in WalkDir::new(&absolute_root)
            .into_iter()
            .filter_entry(|entry| !is_ignored_diagnostics_artifact_path(entry.path()))
            .filter_map(Result::ok)
        {
            if visited >= 4096 {
                break;
            }
            let path = entry.path();
            if !path.is_file() || !is_diagnostics_source_artifact(path) {
                continue;
            }
            visited += 1;

            let Ok(metadata) = entry.metadata() else {
                continue;
            };
            if metadata.len() > 512 * 1024 {
                continue;
            }
            let Ok(modified) = metadata.modified() else {
                continue;
            };
            let Some(relative_path) = diagnostics_relative_source_path(project_root, path) else {
                continue;
            };
            let modified_unix_ms = system_time_unix_ms(modified);
            let candidate = DxDevFeedbackSourceArtifact {
                path: relative_path,
                modified,
                modified_unix_ms,
            };
            if newest
                .as_ref()
                .is_none_or(|current| candidate.modified > current.modified)
            {
                newest = Some(candidate);
            }
        }
    }

    newest
}

fn is_diagnostics_source_artifact(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .map(str::to_ascii_lowercase)
        .is_some_and(|extension| {
            matches!(
                extension.as_str(),
                "ts" | "tsx" | "js" | "jsx" | "css" | "md" | "mdx" | "json" | "html" | "svg"
            )
        })
}

fn is_ignored_diagnostics_artifact_path(path: &Path) -> bool {
    path.components().any(|component| {
        let Component::Normal(part) = component else {
            return false;
        };
        part.to_str().is_some_and(|part| {
            matches!(
                part,
                ".git" | ".dx" | "node_modules" | "target" | "dist" | "build"
            )
        })
    })
}

fn diagnostics_relative_source_path(project_root: &Path, path: &Path) -> Option<String> {
    let relative = path.strip_prefix(project_root).ok()?;
    let mut parts = Vec::new();
    for component in relative.components() {
        let Component::Normal(part) = component else {
            return None;
        };
        parts.push(part.to_str()?);
    }
    Some(parts.join("/"))
}

fn system_time_unix_ms(time: SystemTime) -> Option<u64> {
    time.duration_since(UNIX_EPOCH)
        .ok()
        .and_then(|duration| u64::try_from(duration.as_millis()).ok())
}
