use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Component, Path};

use super::types::{DxForgeImportDecision, DxForgeImportRiskFlag};

/// Path accepted for quarantine or later source materialization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeImportSafePath {
    /// Slash-normalized project-relative path.
    pub path: String,
}

impl DxForgeImportSafePath {
    /// Borrow the normalized path.
    pub fn as_str(&self) -> &str {
        &self.path
    }
}

/// Specific path problem recorded by the quarantine gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DxForgeImportPathProblem {
    /// The path is empty after trimming.
    Empty,
    /// The path contains Windows backslash separators.
    Backslash,
    /// The path contains a nul byte.
    NulByte,
    /// The path is absolute.
    Absolute,
    /// The path contains `.`.
    CurrentDir,
    /// The path contains `..`.
    ParentDir,
    /// The path has a Windows drive or prefix component.
    WindowsPrefix,
    /// The path points at a `node_modules` segment.
    NodeModulesSegment,
    /// The path contains a hidden segment other than `.well-known`.
    HiddenSegment,
    /// Filesystem symlink validation failed or requires review.
    Symlink,
    /// Resolved target can escape the intended root.
    ProjectEscape,
}

/// Full validation result for reports and receipts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeImportPathValidation {
    /// Original path string.
    pub raw: String,
    /// Slash-normalized path when validation accepts it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normalized: Option<String>,
    /// Gate decision for this path.
    pub decision: DxForgeImportDecision,
    /// Risk flags raised by this path.
    pub risk_flags: Vec<DxForgeImportRiskFlag>,
    /// Specific path problems.
    pub problems: Vec<DxForgeImportPathProblem>,
}

/// Validate a package-internal path before quarantine or materialization.
pub fn validate_import_relative_path(raw: &str) -> Result<DxForgeImportSafePath> {
    let validation = validate_import_path(raw);
    if validation.decision != DxForgeImportDecision::Accept {
        bail!("Forge import path `{}` is not safe", raw.trim());
    }

    Ok(DxForgeImportSafePath {
        path: validation
            .normalized
            .expect("accepted import path includes a normalized path"),
    })
}

/// Validate a package-internal path and keep receipt-grade problem details.
pub fn validate_import_path(raw: &str) -> DxForgeImportPathValidation {
    let trimmed = raw.trim();
    let mut problems = Vec::new();

    if trimmed.is_empty() {
        problems.push(DxForgeImportPathProblem::Empty);
    }
    if trimmed.contains('\\') {
        problems.push(DxForgeImportPathProblem::Backslash);
    }
    if trimmed.contains('\0') {
        problems.push(DxForgeImportPathProblem::NulByte);
    }

    let path = Path::new(trimmed);
    if path.is_absolute() {
        problems.push(DxForgeImportPathProblem::Absolute);
    }

    let mut parts = Vec::new();
    for component in path.components() {
        match component {
            Component::Normal(part) => {
                let part = part.to_string_lossy();
                if part == "node_modules" {
                    problems.push(DxForgeImportPathProblem::NodeModulesSegment);
                }
                if part.starts_with('.') && part != ".well-known" {
                    problems.push(DxForgeImportPathProblem::HiddenSegment);
                }
                parts.push(part.to_string());
            }
            Component::CurDir => problems.push(DxForgeImportPathProblem::CurrentDir),
            Component::ParentDir => problems.push(DxForgeImportPathProblem::ParentDir),
            Component::RootDir => problems.push(DxForgeImportPathProblem::Absolute),
            Component::Prefix(_) => problems.push(DxForgeImportPathProblem::WindowsPrefix),
        }
    }

    if parts.is_empty() {
        problems.push(DxForgeImportPathProblem::Empty);
    }

    let accepted = problems.is_empty();
    DxForgeImportPathValidation {
        raw: raw.to_string(),
        normalized: accepted.then(|| parts.join("/")),
        decision: if accepted {
            DxForgeImportDecision::Accept
        } else {
            DxForgeImportDecision::Block
        },
        risk_flags: if accepted {
            Vec::new()
        } else {
            vec![DxForgeImportRiskFlag::UnsafePath]
        },
        problems,
    }
}

/// Validate a package-internal path against a concrete quarantine or project root.
pub fn validate_import_target_path(root: &Path, raw: &str) -> DxForgeImportPathValidation {
    let mut validation = validate_import_path(raw);
    let Some(normalized) = validation.normalized.clone() else {
        return validation;
    };

    let root_canonical = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
    let target = root_canonical.join(&normalized);

    if let Ok(metadata) = fs::symlink_metadata(&target) {
        if metadata.file_type().is_symlink() {
            validation.problems.push(DxForgeImportPathProblem::Symlink);
            validation.risk_flags.push(DxForgeImportRiskFlag::Symlink);
        }
    }

    let containment_candidate = target.canonicalize().ok().or_else(|| {
        target
            .parent()
            .and_then(|parent| parent.canonicalize().ok())
    });

    if let Some(containment_candidate) = containment_candidate {
        if !containment_candidate.starts_with(&root_canonical) {
            validation
                .problems
                .push(DxForgeImportPathProblem::ProjectEscape);
            validation
                .risk_flags
                .push(DxForgeImportRiskFlag::ProjectEscape);
        }
    }

    if !validation.problems.is_empty() {
        validation.decision = DxForgeImportDecision::Block;
        validation.normalized = None;
        if validation.risk_flags.is_empty() {
            validation
                .risk_flags
                .push(DxForgeImportRiskFlag::UnsafePath);
        }
    }

    validation
}

/// Convert a path into policy findings without throwing away the caller's report.
pub fn import_path_risk_flags(raw: &str) -> Vec<DxForgeImportRiskFlag> {
    validate_import_path(raw).risk_flags
}
