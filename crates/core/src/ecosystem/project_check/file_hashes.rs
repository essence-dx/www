use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use super::{json_array_entries, json_text, resolve_dx_check_relative_path};

pub(super) fn count_sha256_file_hash_mismatches(root: &Path, surface: &serde_json::Value) -> u64 {
    let mut mismatches = u64::from(json_text(surface, &["status"]) == Some("stale"));
    if json_text(surface, &["hash_algorithm"]) != Some("sha256") {
        return mismatches;
    }

    let Some(file_hashes) = surface
        .get("file_hashes")
        .and_then(serde_json::Value::as_object)
    else {
        return mismatches;
    };

    for (relative_path, expected_hash) in file_hashes {
        if is_virtual_hash_path(relative_path) {
            continue;
        }

        let Some(expected_hash) = expected_hash.as_str().map(normalize_sha256_hash) else {
            mismatches += 1;
            continue;
        };

        match sha256_project_file(root, relative_path) {
            Some(actual_hash) if expected_hash == actual_hash => {}
            _ => mismatches += 1,
        }
    }

    mismatches
}

pub(super) fn has_sha256_file_hashes(surface: &serde_json::Value) -> bool {
    json_text(surface, &["hash_algorithm"]) == Some("sha256")
        && surface
            .get("file_hashes")
            .and_then(serde_json::Value::as_object)
            .is_some_and(|hashes| !hashes.is_empty())
}

pub(super) fn count_materialized_sha256_file_hash_mismatches(
    root: &Path,
    surface: &serde_json::Value,
) -> u64 {
    if json_text(surface, &["hash_algorithm"]) != Some("sha256") {
        return 0;
    }

    let materialized_files = materialized_surface_files(surface);
    let Some(file_hashes) = surface
        .get("file_hashes")
        .and_then(serde_json::Value::as_object)
    else {
        return 0;
    };

    let mut mismatches = 0u64;
    for (relative_path, expected_hash) in file_hashes {
        if is_provenance_hash(relative_path)
            || !is_materialized_file(relative_path, &materialized_files)
        {
            continue;
        }

        let Some(expected_hash) = expected_hash.as_str().map(normalize_sha256_hash) else {
            mismatches += 1;
            continue;
        };

        match sha256_project_file(root, relative_path) {
            Some(actual_hash) if expected_hash == actual_hash => {}
            _ => mismatches += 1,
        }
    }

    mismatches
}

pub(super) fn count_sha256_path_hash_mismatches<'a, I>(root: &Path, entries: I) -> u64
where
    I: IntoIterator<Item = (&'a str, &'a str)>,
{
    let mut mismatches = 0u64;

    for (relative_path, expected_hash) in entries {
        match sha256_project_file(root, relative_path) {
            Some(actual_hash) if normalize_sha256_hash(expected_hash) == actual_hash => {}
            _ => mismatches += 1,
        }
    }

    mismatches
}

fn materialized_surface_files(surface: &serde_json::Value) -> BTreeSet<String> {
    json_array_entries(surface, &["files"])
        .into_iter()
        .filter_map(serde_json::Value::as_str)
        .flat_map(|file| {
            let normalized = normalize_path(file);
            match normalized.strip_prefix("examples/template/") {
                Some(template_relative) => {
                    vec![normalized.clone(), template_relative.to_string()]
                }
                None => vec![normalized],
            }
        })
        .collect()
}

fn is_materialized_file(relative_path: &str, materialized_files: &BTreeSet<String>) -> bool {
    let normalized = normalize_path(relative_path);
    if materialized_files.contains(&normalized) {
        return true;
    }
    normalized
        .strip_prefix("examples/template/")
        .is_some_and(|template_relative| materialized_files.contains(template_relative))
}

fn is_provenance_hash(relative_path: &str) -> bool {
    let normalized = normalize_path(relative_path);
    normalized.starts_with("upstream:")
        || normalized.starts_with("core/")
        || normalized.starts_with("docs/")
}

fn is_virtual_hash_path(relative_path: &str) -> bool {
    normalize_path(relative_path).starts_with("upstream:")
}

fn normalize_path(path: &str) -> String {
    path.replace('\\', "/")
}

fn sha256_project_file(root: &Path, relative_path: &str) -> Option<String> {
    let file_path = resolve_project_file(root, relative_path)?;
    let bytes = fs::read(file_path).ok()?;
    Some(format!("{:x}", Sha256::digest(&bytes)))
}

fn resolve_project_file(root: &Path, relative_path: &str) -> Option<PathBuf> {
    if let Some(path) =
        resolve_dx_check_relative_path(root, relative_path).filter(|path| path.is_file())
    {
        return Some(path);
    }

    relative_path
        .strip_prefix("examples/template/")
        .and_then(|template_relative| resolve_dx_check_relative_path(root, template_relative))
        .filter(|path| path.is_file())
}

fn normalize_sha256_hash(value: &str) -> String {
    value
        .trim()
        .strip_prefix("sha256:")
        .unwrap_or(value.trim())
        .to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_file_hash_mismatches_are_byte_derived() {
        let dir = tempfile::tempdir().expect("tempdir");
        let file_dir = dir.path().join("examples/template/wasm");
        fs::create_dir_all(&file_dir).expect("wasm dir");
        fs::write(file_dir.join("bridge.ts"), b"wasm bridge\n").expect("bridge file");

        let expected_hash = sha256_project_file(dir.path(), "examples/template/wasm/bridge.ts")
            .expect("expected hash");
        let fresh_surface = serde_json::json!({
            "status": "present",
            "hash_algorithm": "sha256",
            "file_hashes": {
                "examples/template/wasm/bridge.ts": expected_hash
            }
        });
        assert_eq!(
            count_sha256_file_hash_mismatches(dir.path(), &fresh_surface),
            0
        );

        let stale_surface = serde_json::json!({
            "status": "present",
            "hash_algorithm": "sha256",
            "file_hashes": {
                "examples/template/wasm/bridge.ts": "sha256:0000000000000000000000000000000000000000000000000000000000000000"
            }
        });
        assert_eq!(
            count_sha256_file_hash_mismatches(dir.path(), &stale_surface),
            1
        );
    }

    #[test]
    fn sha256_path_hash_mismatches_support_receipt_source_hash_entries() {
        let dir = tempfile::tempdir().expect("tempdir");
        let file_dir = dir.path().join("examples/template/docs");
        fs::create_dir_all(&file_dir).expect("docs dir");
        fs::write(file_dir.join("status.tsx"), b"documentation system\n").expect("docs file");

        let expected_hash = sha256_project_file(dir.path(), "examples/template/docs/status.tsx")
            .expect("expected hash");
        assert_eq!(
            count_sha256_path_hash_mismatches(
                dir.path(),
                [("examples/template/docs/status.tsx", expected_hash.as_str(),)],
            ),
            0
        );
        assert_eq!(
            count_sha256_path_hash_mismatches(
                dir.path(),
                [(
                    "examples/template/docs/status.tsx",
                    "sha256:0000000000000000000000000000000000000000000000000000000000000000",
                )],
            ),
            1
        );
    }

    #[test]
    fn materialized_sha256_file_hash_mismatches_skip_provenance_hashes() {
        let dir = tempfile::tempdir().expect("tempdir");
        fs::write(dir.path().join("ai-chat-status.tsx"), b"launch assistant\n")
            .expect("assistant file");

        let expected_hash =
            sha256_project_file(dir.path(), "ai-chat-status.tsx").expect("expected hash");
        let surface = serde_json::json!({
            "status": "present",
            "hash_algorithm": "sha256",
            "files": [
                "examples/template/ai-chat-status.tsx"
            ],
            "file_hashes": {
                "ai-chat-status.tsx": expected_hash,
                "core/src/ecosystem/forge_vercel_ai.rs": "sha256:0000000000000000000000000000000000000000000000000000000000000000",
                "docs/packages/ai-vercel-ai.md": "sha256:0000000000000000000000000000000000000000000000000000000000000000",
                "upstream:packages/ai/src/generate-text/stream-text.ts": "sha256:0000000000000000000000000000000000000000000000000000000000000000"
            }
        });

        assert!(has_sha256_file_hashes(&surface));
        assert_eq!(
            count_materialized_sha256_file_hash_mismatches(dir.path(), &surface),
            0
        );
    }
}
