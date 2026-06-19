use std::collections::BTreeMap;
use std::path::{Component, Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::error::{DxError, DxResult};

use super::graph::{SourceBuildManifest, hash_bytes};

pub(super) const CONTENT_DOCS_HASH_ALGORITHM: &str = "blake3-16";
pub(super) const CONTENT_DOCS_SAFE_PATH_ROOTS: &[&str] = &["docs", "content"];
pub(super) const CONTENT_DOCS_UNSAFE_PATH_POLICY: &str =
    "reject absolute, parent, and non-docs/content receipt paths before hashing source bytes";

/// Result of comparing a content-docs receipt hash manifest with current source bytes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentDocsFreshnessStatus {
    /// Stable schema name for source-owned content freshness consumers.
    pub schema: String,
    /// Hash algorithm used by the content-docs receipt hash manifest.
    pub hash_algorithm: String,
    /// Number of receipt document entries evaluated, including rejected unsafe paths.
    pub checked_document_count: usize,
    /// Safe receipt paths whose current source hash differs from the receipt hash.
    pub stale_paths: Vec<String>,
    /// Safe receipt paths that no longer exist on disk.
    pub missing_paths: Vec<String>,
    /// Receipt paths rejected before touching the filesystem.
    #[serde(default)]
    pub unsafe_paths: Vec<String>,
    /// True only when there are no stale, missing, or unsafe receipt paths.
    pub current: bool,
    /// Whether this status proves a runtime MDX compile/evaluate pipeline.
    pub runtime_proof: bool,
}

/// Compare expected receipt hashes with already computed current source hashes.
pub fn evaluate_content_docs_hashes(
    expected_hashes: &BTreeMap<String, String>,
    current_hashes: &BTreeMap<String, String>,
) -> ContentDocsFreshnessStatus {
    let mut stale_paths = Vec::new();
    let mut missing_paths = Vec::new();

    for (path, expected_hash) in expected_hashes {
        match current_hashes.get(path) {
            Some(current_hash) if current_hash != expected_hash => stale_paths.push(path.clone()),
            None => missing_paths.push(path.clone()),
            _ => {}
        }
    }

    ContentDocsFreshnessStatus {
        schema: "dx.www.contentDocsFreshnessStatus".to_string(),
        hash_algorithm: CONTENT_DOCS_HASH_ALGORITHM.to_string(),
        checked_document_count: expected_hashes.len(),
        current: stale_paths.is_empty() && missing_paths.is_empty(),
        stale_paths,
        missing_paths,
        unsafe_paths: Vec::new(),
        runtime_proof: false,
    }
}

/// Read a content-docs receipt from disk and compare it with current safe source bytes.
pub fn evaluate_content_docs_receipt_freshness(
    project_root: &Path,
    receipt_path: &Path,
) -> DxResult<ContentDocsFreshnessStatus> {
    let expected_hashes = read_content_docs_hash_manifest(receipt_path)?;
    let unsafe_paths = unsafe_content_doc_hash_paths(&expected_hashes);
    let safe_expected_hashes = safe_content_doc_hashes(&expected_hashes);
    let current_hashes = current_content_doc_hashes(project_root, &safe_expected_hashes)?;
    let mut status = evaluate_content_docs_hashes(&safe_expected_hashes, &current_hashes);
    status.checked_document_count = expected_hashes.len();
    status.current = status.current && unsafe_paths.is_empty();
    status.unsafe_paths = unsafe_paths;
    Ok(status)
}

/// Read `hash_manifest.document_hashes` from a content-docs receipt.
pub fn read_content_docs_hash_manifest(receipt_path: &Path) -> DxResult<BTreeMap<String, String>> {
    let source = std::fs::read_to_string(receipt_path).map_err(|error| DxError::IoError {
        path: Some(receipt_path.to_path_buf()),
        message: error.to_string(),
    })?;
    let value: Value = serde_json::from_str(&source)?;
    let hashes = value
        .get("hash_manifest")
        .and_then(|manifest| manifest.get("document_hashes"))
        .and_then(Value::as_object)
        .ok_or_else(|| DxError::ParseError {
            message: "content-docs receipt is missing hash_manifest.document_hashes".to_string(),
            file: receipt_path.to_path_buf(),
            line: None,
            column: None,
            src: Some(source.clone()),
            span: None,
        })?;

    let mut document_hashes = BTreeMap::new();
    for (path, hash) in hashes {
        let Some(hash) = hash.as_str() else {
            return Err(DxError::ParseError {
                message: format!("content-docs hash for `{path}` must be a string"),
                file: receipt_path.to_path_buf(),
                line: None,
                column: None,
                src: Some(source),
                span: None,
            });
        };
        document_hashes.insert(path.clone(), hash.to_string());
    }

    Ok(document_hashes)
}

/// Compute current hashes for safe content document paths that still exist.
pub fn current_content_doc_hashes(
    project_root: &Path,
    expected_hashes: &BTreeMap<String, String>,
) -> DxResult<BTreeMap<String, String>> {
    let mut current_hashes = BTreeMap::new();
    for path in expected_hashes.keys() {
        let Some(relative_path) = safe_project_relative_content_path(path) else {
            continue;
        };
        let source_path = project_root.join(relative_path);
        if !source_path.is_file() {
            continue;
        }
        let bytes = std::fs::read(&source_path).map_err(|error| DxError::IoError {
            path: Some(source_path.clone()),
            message: error.to_string(),
        })?;
        current_hashes.insert(path.clone(), hash_bytes(&bytes));
    }

    Ok(current_hashes)
}

/// Build the compact freshness section embedded in readiness and editor handoff receipts.
pub fn content_docs_freshness_summary(manifest: &SourceBuildManifest) -> Value {
    let document_hashes = content_document_hashes(manifest);
    let status = evaluate_content_docs_hashes(&document_hashes, &document_hashes);

    json!({
        "schema": status.schema,
        "hash_algorithm": status.hash_algorithm,
        "status": "current-at-build",
        "comparison": "manifest-self-check",
        "checked_document_count": status.checked_document_count,
        "current": status.current,
        "stale_paths": status.stale_paths,
        "missing_paths": status.missing_paths,
        "unsafe_paths": status.unsafe_paths,
        "safe_path_roots": CONTENT_DOCS_SAFE_PATH_ROOTS,
        "unsafe_path_policy": CONTENT_DOCS_UNSAFE_PATH_POLICY,
        "runtime_proof": status.runtime_proof,
        "receipt_section": "content-docs.json#freshness_contract",
        "consumer_action": "compare content-docs hash_manifest.document_hashes against current source bytes and reject unsafe receipt paths before trusting older receipts"
    })
}

pub(super) fn content_document_hashes(manifest: &SourceBuildManifest) -> BTreeMap<String, String> {
    manifest
        .content_documents
        .iter()
        .map(|document| (document.path.clone(), document.hash.clone()))
        .collect()
}

fn safe_content_doc_hashes(expected_hashes: &BTreeMap<String, String>) -> BTreeMap<String, String> {
    expected_hashes
        .iter()
        .filter(|(path, _hash)| safe_project_relative_content_path(path).is_some())
        .map(|(path, hash)| (path.clone(), hash.clone()))
        .collect()
}

fn unsafe_content_doc_hash_paths(expected_hashes: &BTreeMap<String, String>) -> Vec<String> {
    expected_hashes
        .keys()
        .filter(|path| safe_project_relative_content_path(path).is_none())
        .cloned()
        .collect()
}

fn safe_project_relative_content_path(path: &str) -> Option<PathBuf> {
    let candidate = Path::new(path);
    if candidate.is_absolute() {
        return None;
    }

    let mut normalized = PathBuf::new();
    for component in candidate.components() {
        match component {
            Component::Normal(value) => normalized.push(value),
            Component::CurDir => {}
            _ => return None,
        }
    }

    let first = normalized.components().next()?.as_os_str().to_str()?;
    if !CONTENT_DOCS_SAFE_PATH_ROOTS.contains(&first) {
        return None;
    }

    Some(normalized)
}
