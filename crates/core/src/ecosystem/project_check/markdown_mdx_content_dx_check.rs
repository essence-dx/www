#![allow(clippy::too_many_arguments)]
use std::{
    fs,
    path::{Path, PathBuf},
};

use sha2::{Digest, Sha256};

use super::super::forge_security::{DxSourceManifest, DxSupplyChainSeverity};
use super::{
    DxCheckFinding, DxCheckMetric, check_finding, check_metric, json_array_entries, json_text,
    read_optional_forge_json, resolve_dx_check_relative_path,
};

const MARKDOWN_MDX_CONTENT_PACKAGE_ID: &str = "content/react-markdown";
const MARKDOWN_MDX_CONTENT_OFFICIAL_NAME: &str = "Markdown & MDX Content";
const MARKDOWN_MDX_CONTENT_PACKAGE_STATUS: &str = ".dx/forge/package-status.json";
const MARKDOWN_MDX_CONTENT_PACKAGE_RECEIPT: &str =
    ".dx/forge/receipts/packages/content-react-markdown.json";

pub(super) fn forge_markdown_mdx_content_package_metrics(
    root: &Path,
    manifest: &DxSourceManifest,
) -> (Vec<DxCheckMetric>, Vec<DxCheckFinding>) {
    let mut findings = Vec::new();
    let manifest_package_present = manifest
        .packages
        .iter()
        .any(|package| package.package_id == MARKDOWN_MDX_CONTENT_PACKAGE_ID);
    let mut package_present = u64::from(manifest_package_present);
    let mut receipt_present = 0u64;
    let mut stale_receipt = 0u64;
    let mut missing_receipt = 0u64;
    let mut blocked_surfaces = 0u64;
    let mut unsupported_surfaces = 0u64;
    let mut hash_manifest_present = 0u64;
    let mut hash_mismatches = 0u64;
    let mut receipt_hash_refresh_current = 0u64;
    let mut receipt_hash_refresh_stale = 0u64;
    let mut receipt_hash_refresh_missing = 0u64;
    let mut dx_style_compatibility_present = 0u64;
    let mut dx_style_compatibility_missing = 0u64;
    let mut materialized_source_present = 0u64;
    let mut materialized_source_missing = 0u64;

    let Some(package_status) = read_optional_forge_json(
        root,
        MARKDOWN_MDX_CONTENT_PACKAGE_STATUS,
        "markdown-mdx-content-package-status-invalid",
        &mut findings,
    ) else {
        if manifest_package_present
            || package_receipt_exists(root, MARKDOWN_MDX_CONTENT_PACKAGE_RECEIPT)
        {
            package_present = 1;
            missing_receipt = 1;
            receipt_hash_refresh_missing = 1;
            findings.push(missing_package_status_finding());
        }
        return (
            markdown_mdx_content_metrics(
                package_present,
                receipt_present,
                stale_receipt,
                missing_receipt,
                blocked_surfaces,
                unsupported_surfaces,
                hash_manifest_present,
                hash_mismatches,
                receipt_hash_refresh_current,
                receipt_hash_refresh_stale,
                receipt_hash_refresh_missing,
                dx_style_compatibility_present,
                dx_style_compatibility_missing,
                materialized_source_present,
                materialized_source_missing,
            ),
            findings,
        );
    };

    let Some(visibility) = json_array_entries(&package_status, &["package_lane_visibility"])
        .into_iter()
        .find(|entry| json_text(entry, &["package_id"]) == Some(MARKDOWN_MDX_CONTENT_PACKAGE_ID))
    else {
        if manifest_package_present
            || package_receipt_exists(root, MARKDOWN_MDX_CONTENT_PACKAGE_RECEIPT)
        {
            package_present = 1;
            missing_receipt = 1;
            receipt_hash_refresh_missing = 1;
            findings.push(missing_package_status_finding());
        }
        return (
            markdown_mdx_content_metrics(
                package_present,
                receipt_present,
                stale_receipt,
                missing_receipt,
                blocked_surfaces,
                unsupported_surfaces,
                hash_manifest_present,
                hash_mismatches,
                receipt_hash_refresh_current,
                receipt_hash_refresh_stale,
                receipt_hash_refresh_missing,
                dx_style_compatibility_present,
                dx_style_compatibility_missing,
                materialized_source_present,
                materialized_source_missing,
            ),
            findings,
        );
    };

    package_present = 1;
    let (refresh_current, refresh_stale, refresh_missing) = receipt_hash_refresh_counts(visibility);
    receipt_hash_refresh_current = refresh_current;
    receipt_hash_refresh_stale = refresh_stale;
    receipt_hash_refresh_missing = refresh_missing;

    let package_receipt_path = json_text(visibility, &["package_receipt_path"])
        .unwrap_or(MARKDOWN_MDX_CONTENT_PACKAGE_RECEIPT);
    if package_receipt_exists(root, package_receipt_path) {
        receipt_present = 1;
        if let Some(receipt) = read_package_receipt(root, package_receipt_path) {
            if receipt_has_sha256_file_hashes(&receipt) {
                hash_manifest_present = 1;
            }
            hash_mismatches += count_receipt_hash_mismatches(root, &receipt);
        }
    } else {
        missing_receipt = 1;
        findings.push(missing_receipt_finding(package_receipt_path));
    }

    let visibility_status = json_text(visibility, &["status"]);
    let receipt_status = json_text(visibility, &["receipt_status"]);
    if status_is_stale(visibility_status) || status_is_stale(receipt_status) {
        stale_receipt = 1;
    }
    if status_is_missing_receipt(visibility_status) || status_is_missing_receipt(receipt_status) {
        missing_receipt = 1;
    }
    if status_is_blocked(visibility_status) || status_is_blocked(receipt_status) {
        blocked_surfaces += 1;
    }
    if status_is_unsupported_surface(visibility_status)
        || status_is_unsupported_surface(receipt_status)
    {
        unsupported_surfaces += 1;
    }

    blocked_surfaces += json_array_entries(visibility, &["blocked_surfaces"]).len() as u64;
    unsupported_surfaces += json_array_entries(visibility, &["unsupported_surfaces"]).len() as u64;

    for surface in json_array_entries(visibility, &["selected_surfaces"]) {
        if has_sha256_file_hashes(surface) {
            hash_manifest_present = 1;
        }

        hash_mismatches += count_surface_hash_mismatches(root, surface);

        match json_text(surface, &["status"]) {
            status if status_is_stale(status) => stale_receipt = 1,
            status if status_is_missing_receipt(status) => missing_receipt = 1,
            status if status_is_blocked(status) => blocked_surfaces += 1,
            status if status_is_unsupported_surface(status) => unsupported_surfaces += 1,
            _ => {}
        }
    }

    if dx_style_compatibility_is_present(visibility) {
        dx_style_compatibility_present = 1;
    } else {
        dx_style_compatibility_missing = 1;
    }
    if materialized_source_is_present(visibility) {
        materialized_source_present = 1;
    } else {
        materialized_source_missing = 1;
    }

    if hash_mismatches > 0 || receipt_hash_refresh_stale > 0 || receipt_hash_refresh_missing > 0 {
        stale_receipt = 1;
    }

    if stale_receipt > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "markdown-mdx-content-stale-receipt",
            format!("{MARKDOWN_MDX_CONTENT_OFFICIAL_NAME} package-status visibility is stale"),
            Some(MARKDOWN_MDX_CONTENT_PACKAGE_STATUS.to_string()),
            "Regenerate the Markdown & MDX Content package-status row from the package receipt before claiming source freshness.",
        ));
    }
    if blocked_surfaces > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "markdown-mdx-content-blocked-surface",
            format!(
                "{MARKDOWN_MDX_CONTENT_OFFICIAL_NAME} has {blocked_surfaces} blocked selected surface(s)"
            ),
            Some(MARKDOWN_MDX_CONTENT_PACKAGE_STATUS.to_string()),
            "Resolve the app-owned Markdown/MDX runtime, dependency, plugin, or content-trust boundary before claiming the selected surface is release-ready.",
        ));
    }
    if unsupported_surfaces > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "markdown-mdx-content-unsupported-surface",
            format!(
                "{MARKDOWN_MDX_CONTENT_OFFICIAL_NAME} has {unsupported_surfaces} unsupported requested surface(s)"
            ),
            Some(MARKDOWN_MDX_CONTENT_PACKAGE_STATUS.to_string()),
            "Request only supported Markdown & MDX Content surfaces or add a real upstream-backed Forge surface first.",
        ));
    }
    if hash_mismatches > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "markdown-mdx-content-hash-mismatch",
            format!(
                "{MARKDOWN_MDX_CONTENT_OFFICIAL_NAME} has {hash_mismatches} missing or stale hash-backed selected file(s)"
            ),
            Some(MARKDOWN_MDX_CONTENT_PACKAGE_STATUS.to_string()),
            "Regenerate the Markdown & MDX Content package receipt after reviewing the changed renderer, MDX provider, server compile, or receipt-helper files.",
        ));
    }
    if dx_style_compatibility_missing > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "markdown-mdx-content-missing-dx-style-compatibility",
            format!(
                "{MARKDOWN_MDX_CONTENT_OFFICIAL_NAME} is missing dx-style compatibility evidence"
            ),
            Some(MARKDOWN_MDX_CONTENT_PACKAGE_STATUS.to_string()),
            "Regenerate the Markdown & MDX Content dx-style compatibility row from the package-status read model and verify source-owned style markers without claiming live Markdown/MDX renderer proof.",
        ));
    }
    if materialized_source_missing > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "markdown-mdx-content-missing-materialized-source",
            format!(
                "{MARKDOWN_MDX_CONTENT_OFFICIAL_NAME} is missing materialized receipt-helper source evidence"
            ),
            Some(MARKDOWN_MDX_CONTENT_PACKAGE_STATUS.to_string()),
            "Regenerate the Markdown & MDX Content materializedSource row from the package-status read model and verify the receipt helper source without claiming live Markdown/MDX renderer proof.",
        ));
    }

    (
        markdown_mdx_content_metrics(
            package_present,
            receipt_present,
            stale_receipt,
            missing_receipt,
            blocked_surfaces,
            unsupported_surfaces,
            hash_manifest_present,
            hash_mismatches,
            receipt_hash_refresh_current,
            receipt_hash_refresh_stale,
            receipt_hash_refresh_missing,
            dx_style_compatibility_present,
            dx_style_compatibility_missing,
            materialized_source_present,
            materialized_source_missing,
        ),
        findings,
    )
}

fn markdown_mdx_content_metrics(
    package_present: u64,
    receipt_present: u64,
    stale_receipt: u64,
    missing_receipt: u64,
    blocked_surfaces: u64,
    unsupported_surfaces: u64,
    hash_manifest_present: u64,
    hash_mismatches: u64,
    receipt_hash_refresh_current: u64,
    receipt_hash_refresh_stale: u64,
    receipt_hash_refresh_missing: u64,
    dx_style_compatibility_present: u64,
    dx_style_compatibility_missing: u64,
    materialized_source_present: u64,
    materialized_source_missing: u64,
) -> Vec<DxCheckMetric> {
    vec![
        check_metric("markdown_mdx_content_package_present", package_present),
        check_metric("markdown_mdx_content_receipt_present", receipt_present),
        check_metric("markdown_mdx_content_receipt_stale", stale_receipt),
        check_metric("markdown_mdx_content_missing_receipt", missing_receipt),
        check_metric("markdown_mdx_content_blocked_surface", blocked_surfaces),
        check_metric(
            "markdown_mdx_content_unsupported_surface",
            unsupported_surfaces,
        ),
        check_metric(
            "markdown_mdx_content_hash_manifest_present",
            hash_manifest_present,
        ),
        check_metric("markdown_mdx_content_hash_mismatch", hash_mismatches),
        check_metric(
            "markdown_mdx_content_receipt_hash_refresh_current",
            receipt_hash_refresh_current,
        ),
        check_metric(
            "markdown_mdx_content_receipt_hash_refresh_stale",
            receipt_hash_refresh_stale,
        ),
        check_metric(
            "markdown_mdx_content_receipt_hash_refresh_missing",
            receipt_hash_refresh_missing,
        ),
        check_metric(
            "markdown_mdx_content_dx_style_compatibility_present",
            dx_style_compatibility_present,
        ),
        check_metric(
            "markdown_mdx_content_dx_style_compatibility_missing",
            dx_style_compatibility_missing,
        ),
        check_metric(
            "markdown_mdx_content_materialized_source_present",
            materialized_source_present,
        ),
        check_metric(
            "markdown_mdx_content_materialized_source_missing",
            materialized_source_missing,
        ),
    ]
}

fn has_sha256_file_hashes(surface: &serde_json::Value) -> bool {
    json_text(surface, &["hash_algorithm"]) == Some("sha256")
        && surface
            .get("file_hashes")
            .and_then(serde_json::Value::as_object)
            .is_some_and(|hashes| !hashes.is_empty())
}

fn count_surface_hash_mismatches(root: &Path, surface: &serde_json::Value) -> u64 {
    let mut mismatches = u64::from(status_is_stale(json_text(surface, &["status"])));
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
        let Some(expected_hash) = expected_hash.as_str() else {
            mismatches += 1;
            continue;
        };
        match sha256_project_file(root, relative_path) {
            Some(actual_hash) if normalize_sha256_hash(expected_hash) == actual_hash => {}
            _ => mismatches += 1,
        }
    }

    mismatches
}

fn read_package_receipt(root: &Path, receipt_path: &str) -> Option<serde_json::Value> {
    let path = project_file_path(root, receipt_path)?;
    let bytes = fs::read(path).ok()?;
    serde_json::from_slice(&bytes).ok()
}

fn receipt_has_sha256_file_hashes(receipt: &serde_json::Value) -> bool {
    json_array_entries(receipt, &["files"])
        .into_iter()
        .any(|file| receipt_file_sha256(file).is_some())
}

fn count_receipt_hash_mismatches(root: &Path, receipt: &serde_json::Value) -> u64 {
    let mut mismatches = 0u64;

    for file in json_array_entries(receipt, &["files"]) {
        let Some(expected_hash) = receipt_file_sha256(file) else {
            continue;
        };
        let Some(relative_path) = json_text(file, &["path"]) else {
            mismatches += 1;
            continue;
        };

        match sha256_project_file(root, relative_path) {
            Some(actual_hash) if normalize_sha256_hash(expected_hash) == actual_hash => {}
            _ => mismatches += 1,
        }
    }

    mismatches
}

fn receipt_file_sha256(file: &serde_json::Value) -> Option<&str> {
    file.get("hashes")
        .and_then(|hashes| hashes.get("sha256"))
        .and_then(serde_json::Value::as_str)
        .or_else(|| file.get("sha256").and_then(serde_json::Value::as_str))
}

fn status_is_stale(status: Option<&str>) -> bool {
    matches!(status, Some("stale"))
}

fn status_is_missing_receipt(status: Option<&str>) -> bool {
    matches!(status, Some("missing-receipt" | "missing receipt"))
}

fn status_is_blocked(status: Option<&str>) -> bool {
    matches!(status, Some("blocked"))
}

fn status_is_unsupported_surface(status: Option<&str>) -> bool {
    matches!(status, Some("unsupported-surface" | "unsupported surface"))
}

fn receipt_hash_refresh_counts(visibility: &serde_json::Value) -> (u64, u64, u64) {
    let Some(refresh) = visibility.get("receipt_hash_refresh") else {
        return (0, 0, 1);
    };
    if json_text(refresh, &["schema"]) != Some("dx.forge.package.receipt_hash_refresh") {
        return (0, 0, 1);
    }

    let stale_files = json_u64(refresh, &["stale_file_count"]).unwrap_or(0);
    let missing_files = json_u64(refresh, &["missing_file_count"]).unwrap_or(0);
    let status = json_text(refresh, &["status"]).unwrap_or("missing");
    let stale = u64::from(status == "stale" || stale_files > 0);
    let missing = u64::from(status == "missing" || missing_files > 0);
    let current = u64::from(status == "current" && stale == 0 && missing == 0);

    (current, stale, missing)
}

fn json_u64(value: &serde_json::Value, path: &[&str]) -> Option<u64> {
    let mut current = value;
    for key in path {
        current = current.get(*key)?;
    }
    current.as_u64()
}

fn dx_style_compatibility_is_present(visibility: &serde_json::Value) -> bool {
    let Some(dx_style) = visibility.get("dx_style_compatibility") else {
        return false;
    };

    json_text(dx_style, &["schema"]) == Some("dx.forge.package.dx_style_compatibility")
        && json_text(dx_style, &["status"]).unwrap_or("present") == "present"
}

fn materialized_source_is_present(visibility: &serde_json::Value) -> bool {
    let Some(materialized_source) = visibility.get("materialized_source") else {
        return false;
    };

    let execution_guard = json_text(materialized_source, &["execution_guard"]).unwrap_or("");

    json_text(materialized_source, &["schema"]) == Some("dx.forge.package.materialized_source")
        && json_text(materialized_source, &["source_file"])
            == Some("lib/markdown-mdx-content/receipt.ts")
        && json_text(materialized_source, &["materialized_file"])
            == Some("lib/markdown-mdx-content/receipt.ts")
        && json_text(materialized_source, &["surface"]) == Some("forge-receipt-helper")
        && execution_guard.contains("markdown-mdx-content-slice.test.ts")
        && materialized_source
            .get("runtime_proof")
            .and_then(serde_json::Value::as_bool)
            == Some(false)
}

fn missing_package_status_finding() -> DxCheckFinding {
    check_finding(
        DxSupplyChainSeverity::Medium,
        "markdown-mdx-content-missing-package-status",
        format!("{MARKDOWN_MDX_CONTENT_OFFICIAL_NAME} is missing from package-status visibility"),
        Some(MARKDOWN_MDX_CONTENT_PACKAGE_STATUS.to_string()),
        "Regenerate .dx/forge/package-status.json so dx-check can report Markdown & MDX Content visibility states.",
    )
}

fn missing_receipt_finding(receipt_path: &str) -> DxCheckFinding {
    check_finding(
        DxSupplyChainSeverity::Medium,
        "markdown-mdx-content-missing-receipt",
        format!("{MARKDOWN_MDX_CONTENT_OFFICIAL_NAME} is missing its package-lane receipt"),
        Some(receipt_path.to_string()),
        "Regenerate or restore the Markdown & MDX Content package receipt so dx-check can report source-owned package visibility.",
    )
}

fn package_receipt_exists(root: &Path, receipt_path: &str) -> bool {
    project_file_path(root, receipt_path).is_some()
}

fn sha256_project_file(root: &Path, relative_path: &str) -> Option<String> {
    let path = project_file_path(root, relative_path)?;
    let bytes = fs::read(path).ok()?;
    let digest = Sha256::digest(&bytes);
    Some(format!("{digest:x}"))
}

fn project_file_path(root: &Path, relative_path: &str) -> Option<PathBuf> {
    let normalized = normalize_path(relative_path);
    if let Some(path) =
        resolve_dx_check_relative_path(root, &normalized).filter(|path| path.is_file())
    {
        return Some(path);
    }

    normalized
        .strip_prefix("examples/template/")
        .and_then(|template_relative| resolve_dx_check_relative_path(root, template_relative))
        .filter(|path| path.is_file())
}

fn normalize_path(path: &str) -> String {
    path.replace('\\', "/")
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
    use tempfile::tempdir;

    fn metric_value(metrics: &[DxCheckMetric], name: &str) -> Option<u64> {
        metrics
            .iter()
            .find(|metric| metric.name == name)
            .map(|metric| metric.value)
    }

    #[test]
    fn markdown_mdx_content_hash_mismatches_are_byte_derived() {
        let dir = tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join("components/content")).expect("content dir");
        fs::write(
            dir.path().join("components/content/markdown.tsx"),
            "export function DxMarkdown() { return null; }\n",
        )
        .expect("markdown source");
        let expected_hash = sha256_project_file(dir.path(), "components/content/markdown.tsx")
            .expect("expected hash");

        let receipt = serde_json::json!({
            "files": [
                {
                    "path": "components/content/markdown.tsx",
                    "surface": "safe-markdown-renderer",
                    "hashes": {
                        "sha256": format!("sha256:{expected_hash}")
                    }
                }
            ]
        });
        assert!(receipt_has_sha256_file_hashes(&receipt));
        assert_eq!(count_receipt_hash_mismatches(dir.path(), &receipt), 0);

        fs::write(
            dir.path().join("components/content/markdown.tsx"),
            "export function DxMarkdown() { return <article />; }\n",
        )
        .expect("mutate markdown source");
        assert_eq!(count_receipt_hash_mismatches(dir.path(), &receipt), 1);

        let surface = serde_json::json!({
            "surface_id": "safe-markdown-renderer",
            "status": "present",
            "hash_algorithm": "sha256",
            "file_hashes": {
                "components/content/markdown.tsx": expected_hash
            }
        });
        assert!(has_sha256_file_hashes(&surface));
        assert_eq!(count_surface_hash_mismatches(dir.path(), &surface), 1);
    }

    #[test]
    fn markdown_mdx_content_package_metrics_reports_hash_mismatch_metric_and_finding() {
        let dir = tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join("components/content")).expect("content dir");
        fs::create_dir_all(dir.path().join(".dx/forge/receipts/packages")).expect("receipt dir");
        fs::write(
            dir.path().join("components/content/markdown.tsx"),
            "export function DxMarkdown() { return null; }\n",
        )
        .expect("markdown source");
        let expected_hash = sha256_project_file(dir.path(), "components/content/markdown.tsx")
            .expect("expected hash");
        let expected_sha256 = format!("sha256:{expected_hash}");

        let receipt = serde_json::json!({
            "schema": "dx.forge.markdown_mdx_content_receipt",
            "officialDxPackageName": MARKDOWN_MDX_CONTENT_OFFICIAL_NAME,
            "packageId": MARKDOWN_MDX_CONTENT_PACKAGE_ID,
            "files": [
                {
                    "path": "components/content/markdown.tsx",
                    "surface": "safe-markdown-renderer",
                    "hashes": {
                        "sha256": expected_sha256
                    },
                    "provenance": {
                        "upstream_package": "react-markdown",
                        "source_mirror": "G:/WWW/inspirations/react-markdown"
                    }
                }
            ]
        });
        fs::write(
            dir.path().join(MARKDOWN_MDX_CONTENT_PACKAGE_RECEIPT),
            serde_json::to_vec_pretty(&receipt).expect("receipt json"),
        )
        .expect("write receipt");

        let package_status = serde_json::json!({
            "package_lane_visibility": [
                {
                    "official_package_name": MARKDOWN_MDX_CONTENT_OFFICIAL_NAME,
                    "package_id": MARKDOWN_MDX_CONTENT_PACKAGE_ID,
                    "status": "present",
                    "receipt_status": "present",
                    "package_receipt_path": MARKDOWN_MDX_CONTENT_PACKAGE_RECEIPT,
                    "selected_surfaces": [
                        {
                            "surface_id": "safe-markdown-renderer",
                            "status": "present",
                            "files": ["components/content/markdown.tsx"],
                            "hash_algorithm": "sha256",
                            "file_hashes": {
                                "components/content/markdown.tsx": expected_hash
                            }
                        }
                    ]
                }
            ]
        });
        fs::write(
            dir.path().join(MARKDOWN_MDX_CONTENT_PACKAGE_STATUS),
            serde_json::to_vec_pretty(&package_status).expect("package status json"),
        )
        .expect("write package status");

        fs::write(
            dir.path().join("components/content/markdown.tsx"),
            "export function DxMarkdown() { return <article />; }\n",
        )
        .expect("mutate markdown source");

        let (metrics, findings) =
            forge_markdown_mdx_content_package_metrics(dir.path(), &DxSourceManifest::default());

        assert_eq!(
            metric_value(&metrics, "markdown_mdx_content_package_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(&metrics, "markdown_mdx_content_receipt_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(&metrics, "markdown_mdx_content_receipt_stale"),
            Some(1)
        );
        assert_eq!(
            metric_value(&metrics, "markdown_mdx_content_hash_manifest_present"),
            Some(1)
        );
        assert!(metric_value(&metrics, "markdown_mdx_content_hash_mismatch").unwrap_or(0) >= 1);
        assert!(
            findings
                .iter()
                .any(|finding| finding.code == "markdown-mdx-content-stale-receipt")
        );
        assert!(
            findings
                .iter()
                .any(|finding| finding.code == "markdown-mdx-content-hash-mismatch")
        );
    }

    #[test]
    fn markdown_mdx_content_package_metrics_reports_dx_style_compatibility() {
        let dir = tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join(".dx/forge/receipts/packages")).expect("receipt dir");
        fs::write(
            dir.path().join(MARKDOWN_MDX_CONTENT_PACKAGE_RECEIPT),
            br#"{"schema":"dx.forge.markdown_mdx_content_receipt","files":[]}"#,
        )
        .expect("write receipt");

        let package_status_without_dx_style = serde_json::json!({
            "package_lane_visibility": [
                {
                    "official_package_name": MARKDOWN_MDX_CONTENT_OFFICIAL_NAME,
                    "package_id": MARKDOWN_MDX_CONTENT_PACKAGE_ID,
                    "status": "present",
                    "receipt_status": "present",
                    "package_receipt_path": MARKDOWN_MDX_CONTENT_PACKAGE_RECEIPT,
                    "selected_surfaces": []
                }
            ]
        });
        fs::write(
            dir.path().join(MARKDOWN_MDX_CONTENT_PACKAGE_STATUS),
            serde_json::to_vec_pretty(&package_status_without_dx_style)
                .expect("package status json"),
        )
        .expect("write package status");

        let (missing_metrics, missing_findings) =
            forge_markdown_mdx_content_package_metrics(dir.path(), &DxSourceManifest::default());
        assert_eq!(
            metric_value(
                &missing_metrics,
                "markdown_mdx_content_dx_style_compatibility_present"
            ),
            Some(0)
        );
        assert_eq!(
            metric_value(
                &missing_metrics,
                "markdown_mdx_content_dx_style_compatibility_missing"
            ),
            Some(1)
        );
        assert!(missing_findings.iter().any(|finding| {
            finding.code == "markdown-mdx-content-missing-dx-style-compatibility"
        }));

        let package_status_with_dx_style = serde_json::json!({
            "package_lane_visibility": [
                {
                    "official_package_name": MARKDOWN_MDX_CONTENT_OFFICIAL_NAME,
                    "package_id": MARKDOWN_MDX_CONTENT_PACKAGE_ID,
                    "status": "present",
                    "receipt_status": "present",
                    "package_receipt_path": MARKDOWN_MDX_CONTENT_PACKAGE_RECEIPT,
                    "selected_surfaces": [],
                    "dx_style_compatibility": {
                        "schema": "dx.forge.package.dx_style_compatibility",
                        "status": "present",
                        "token_source": "styles/theme.css",
                        "generated_css": "styles/generated.css",
                        "visible_surfaces": ["mdx-provider"],
                        "source_files": ["components/content/mdx-provider.tsx"],
                        "runtime_proof": false
                    }
                }
            ]
        });
        fs::write(
            dir.path().join(MARKDOWN_MDX_CONTENT_PACKAGE_STATUS),
            serde_json::to_vec_pretty(&package_status_with_dx_style).expect("package status json"),
        )
        .expect("write package status");

        let (present_metrics, present_findings) =
            forge_markdown_mdx_content_package_metrics(dir.path(), &DxSourceManifest::default());
        assert_eq!(
            metric_value(
                &present_metrics,
                "markdown_mdx_content_dx_style_compatibility_present"
            ),
            Some(1)
        );
        assert_eq!(
            metric_value(
                &present_metrics,
                "markdown_mdx_content_dx_style_compatibility_missing"
            ),
            Some(0)
        );
        assert!(!present_findings.iter().any(|finding| {
            finding.code == "markdown-mdx-content-missing-dx-style-compatibility"
        }));
    }

    #[test]
    fn markdown_mdx_content_package_metrics_reports_materialized_source_visibility() {
        let dir = tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join(".dx/forge/receipts/packages")).expect("receipt dir");
        fs::write(
            dir.path().join(MARKDOWN_MDX_CONTENT_PACKAGE_RECEIPT),
            br#"{"schema":"dx.forge.markdown_mdx_content_receipt","files":[]}"#,
        )
        .expect("write receipt");

        let package_status_without_materialized_source = serde_json::json!({
            "package_lane_visibility": [
                {
                    "official_package_name": MARKDOWN_MDX_CONTENT_OFFICIAL_NAME,
                    "package_id": MARKDOWN_MDX_CONTENT_PACKAGE_ID,
                    "status": "present",
                    "receipt_status": "present",
                    "package_receipt_path": MARKDOWN_MDX_CONTENT_PACKAGE_RECEIPT,
                    "selected_surfaces": [],
                    "dx_style_compatibility": {
                        "schema": "dx.forge.package.dx_style_compatibility",
                        "status": "present",
                        "token_source": "styles/theme.css",
                        "generated_css": "styles/generated.css",
                        "visible_surfaces": ["mdx-provider"],
                        "source_files": ["components/content/mdx-provider.tsx"],
                        "runtime_proof": false
                    }
                }
            ]
        });
        fs::write(
            dir.path().join(MARKDOWN_MDX_CONTENT_PACKAGE_STATUS),
            serde_json::to_vec_pretty(&package_status_without_materialized_source)
                .expect("package status json"),
        )
        .expect("write package status");

        let (missing_metrics, missing_findings) =
            forge_markdown_mdx_content_package_metrics(dir.path(), &DxSourceManifest::default());
        assert_eq!(
            metric_value(
                &missing_metrics,
                "markdown_mdx_content_materialized_source_present"
            ),
            Some(0)
        );
        assert_eq!(
            metric_value(
                &missing_metrics,
                "markdown_mdx_content_materialized_source_missing"
            ),
            Some(1)
        );
        assert!(
            missing_findings.iter().any(|finding| {
                finding.code == "markdown-mdx-content-missing-materialized-source"
            })
        );

        let package_status_with_materialized_source = serde_json::json!({
            "package_lane_visibility": [
                {
                    "official_package_name": MARKDOWN_MDX_CONTENT_OFFICIAL_NAME,
                    "package_id": MARKDOWN_MDX_CONTENT_PACKAGE_ID,
                    "status": "present",
                    "receipt_status": "present",
                    "package_receipt_path": MARKDOWN_MDX_CONTENT_PACKAGE_RECEIPT,
                    "selected_surfaces": [],
                    "dx_style_compatibility": {
                        "schema": "dx.forge.package.dx_style_compatibility",
                        "status": "present",
                        "token_source": "styles/theme.css",
                        "generated_css": "styles/generated.css",
                        "visible_surfaces": ["mdx-provider"],
                        "source_files": ["components/content/mdx-provider.tsx"],
                        "runtime_proof": false
                    },
                    "materialized_source": {
                        "schema": "dx.forge.package.materialized_source",
                        "source_file": "lib/markdown-mdx-content/receipt.ts",
                        "materialized_file": "lib/markdown-mdx-content/receipt.ts",
                        "surface": "forge-receipt-helper",
                        "execution_guard": "dx run --test .\\benchmarks\\markdown-mdx-content-slice.test.ts",
                        "runtime_proof": false
                    }
                }
            ]
        });
        fs::write(
            dir.path().join(MARKDOWN_MDX_CONTENT_PACKAGE_STATUS),
            serde_json::to_vec_pretty(&package_status_with_materialized_source)
                .expect("package status json"),
        )
        .expect("write package status");

        let (present_metrics, present_findings) =
            forge_markdown_mdx_content_package_metrics(dir.path(), &DxSourceManifest::default());
        assert_eq!(
            metric_value(
                &present_metrics,
                "markdown_mdx_content_materialized_source_present"
            ),
            Some(1)
        );
        assert_eq!(
            metric_value(
                &present_metrics,
                "markdown_mdx_content_materialized_source_missing"
            ),
            Some(0)
        );
        assert!(
            !present_findings.iter().any(|finding| {
                finding.code == "markdown-mdx-content-missing-materialized-source"
            })
        );
    }

    #[test]
    fn markdown_mdx_content_package_metrics_reports_helper_freshness() {
        let dir = tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join(".dx/forge/receipts/packages")).expect("receipt dir");
        fs::write(
            dir.path().join(MARKDOWN_MDX_CONTENT_PACKAGE_RECEIPT),
            br#"{"schema":"dx.forge.markdown_mdx_content_receipt","files":[]}"#,
        )
        .expect("write receipt");

        let package_status = serde_json::json!({
            "package_lane_visibility": [
                {
                    "official_package_name": MARKDOWN_MDX_CONTENT_OFFICIAL_NAME,
                    "package_id": MARKDOWN_MDX_CONTENT_PACKAGE_ID,
                    "status": "present",
                    "receipt_status": "present",
                    "package_receipt_path": MARKDOWN_MDX_CONTENT_PACKAGE_RECEIPT,
                    "selected_surfaces": [],
                    "receipt_hash_refresh": {
                        "schema": "dx.forge.package.receipt_hash_refresh",
                        "status": "current",
                        "stale_file_count": 0,
                        "missing_file_count": 0
                    },
                    "dx_style_compatibility": {
                        "schema": "dx.forge.package.dx_style_compatibility",
                        "status": "present",
                        "token_source": "styles/theme.css",
                        "generated_css": "styles/generated.css",
                        "visible_surfaces": ["mdx-provider"],
                        "source_files": ["components/content/mdx-provider.tsx"],
                        "runtime_proof": false
                    },
                    "materialized_source": {
                        "schema": "dx.forge.package.materialized_source",
                        "source_file": "lib/markdown-mdx-content/receipt.ts",
                        "materialized_file": "lib/markdown-mdx-content/receipt.ts",
                        "surface": "forge-receipt-helper",
                        "execution_guard": "dx run --test .\\benchmarks\\markdown-mdx-content-slice.test.ts",
                        "runtime_proof": false
                    }
                }
            ]
        });
        fs::write(
            dir.path().join(MARKDOWN_MDX_CONTENT_PACKAGE_STATUS),
            serde_json::to_vec_pretty(&package_status).expect("package status json"),
        )
        .expect("write package status");

        let (current_metrics, _) =
            forge_markdown_mdx_content_package_metrics(dir.path(), &DxSourceManifest::default());
        assert_eq!(
            metric_value(
                &current_metrics,
                "markdown_mdx_content_receipt_hash_refresh_current"
            ),
            Some(1)
        );
        assert_eq!(
            metric_value(
                &current_metrics,
                "markdown_mdx_content_receipt_hash_refresh_stale"
            ),
            Some(0)
        );
        assert_eq!(
            metric_value(
                &current_metrics,
                "markdown_mdx_content_receipt_hash_refresh_missing"
            ),
            Some(0)
        );
        assert_eq!(
            metric_value(&current_metrics, "markdown_mdx_content_hash_mismatch"),
            Some(0)
        );

        let mut stale_package_status = package_status;
        stale_package_status["package_lane_visibility"][0]["receipt_hash_refresh"]["status"] =
            serde_json::json!("stale");
        stale_package_status["package_lane_visibility"][0]["receipt_hash_refresh"]["stale_file_count"] =
            serde_json::json!(1);
        fs::write(
            dir.path().join(MARKDOWN_MDX_CONTENT_PACKAGE_STATUS),
            serde_json::to_vec_pretty(&stale_package_status).expect("package status json"),
        )
        .expect("write stale package status");

        let (stale_metrics, stale_findings) =
            forge_markdown_mdx_content_package_metrics(dir.path(), &DxSourceManifest::default());
        assert_eq!(
            metric_value(
                &stale_metrics,
                "markdown_mdx_content_receipt_hash_refresh_current"
            ),
            Some(0)
        );
        assert_eq!(
            metric_value(
                &stale_metrics,
                "markdown_mdx_content_receipt_hash_refresh_stale"
            ),
            Some(1)
        );
        assert_eq!(
            metric_value(
                &stale_metrics,
                "markdown_mdx_content_receipt_hash_refresh_missing"
            ),
            Some(0)
        );
        assert_eq!(
            metric_value(&stale_metrics, "markdown_mdx_content_hash_mismatch"),
            Some(0)
        );
        assert_eq!(
            metric_value(&stale_metrics, "markdown_mdx_content_receipt_stale"),
            Some(1)
        );
        assert!(
            stale_findings
                .iter()
                .any(|finding| { finding.code == "markdown-mdx-content-stale-receipt" })
        );
    }
}
