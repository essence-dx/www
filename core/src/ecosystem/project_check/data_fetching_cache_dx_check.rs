#![allow(clippy::too_many_arguments)]
use std::path::Path;

use super::super::forge_security::{DxSourceManifest, DxSupplyChainSeverity};
use super::file_hashes::count_sha256_file_hash_mismatches;
use super::{
    DxCheckFinding, DxCheckMetric, check_finding, check_metric, json_array_entries, json_text,
    read_optional_forge_json, resolve_dx_check_relative_path,
};

const DATA_FETCHING_CACHE_PACKAGE_ID: &str = "tanstack/query";
const DATA_FETCHING_CACHE_OFFICIAL_NAME: &str = "Data Fetching & Cache";
const DATA_FETCHING_CACHE_PACKAGE_STATUS: &str = ".dx/forge/package-status.json";
const DATA_FETCHING_CACHE_DASHBOARD_RECEIPT: &str =
    ".dx/forge/receipts/2026-05-22-tanstack-query-dashboard-data.json";

pub(super) fn forge_data_fetching_cache_package_metrics(
    root: &Path,
    manifest: &DxSourceManifest,
) -> (Vec<DxCheckMetric>, Vec<DxCheckFinding>) {
    let mut findings = Vec::new();
    let manifest_package_present = manifest
        .packages
        .iter()
        .any(|package| package.package_id == DATA_FETCHING_CACHE_PACKAGE_ID);
    let mut package_present = u64::from(manifest_package_present);
    let mut receipt_present = 0u64;
    let mut stale_receipt = 0u64;
    let mut missing_receipt = 0u64;
    let mut blocked_surfaces = 0u64;
    let mut unsupported_surfaces = 0u64;
    let mut hash_manifest_present = 0u64;
    let mut hash_mismatches = 0u64;
    let mut dx_style_compatibility_present = 0u64;
    let mut dx_style_compatibility_missing = 0u64;

    let Some(package_status) = read_optional_forge_json(
        root,
        DATA_FETCHING_CACHE_PACKAGE_STATUS,
        "data-fetching-cache-package-status-invalid",
        &mut findings,
    ) else {
        if manifest_package_present
            || package_receipt_exists(root, DATA_FETCHING_CACHE_DASHBOARD_RECEIPT)
        {
            package_present = 1;
            missing_receipt = 1;
            findings.push(missing_package_status_finding());
        }
        return (
            data_fetching_cache_metrics(
                package_present,
                receipt_present,
                stale_receipt,
                missing_receipt,
                blocked_surfaces,
                unsupported_surfaces,
                hash_manifest_present,
                hash_mismatches,
                dx_style_compatibility_present,
                dx_style_compatibility_missing,
            ),
            findings,
        );
    };

    let Some(visibility) = json_array_entries(&package_status, &["package_lane_visibility"])
        .into_iter()
        .find(|entry| json_text(entry, &["package_id"]) == Some(DATA_FETCHING_CACHE_PACKAGE_ID))
    else {
        if manifest_package_present
            || package_receipt_exists(root, DATA_FETCHING_CACHE_DASHBOARD_RECEIPT)
        {
            package_present = 1;
            missing_receipt = 1;
            findings.push(missing_package_status_finding());
        }
        return (
            data_fetching_cache_metrics(
                package_present,
                receipt_present,
                stale_receipt,
                missing_receipt,
                blocked_surfaces,
                unsupported_surfaces,
                hash_manifest_present,
                hash_mismatches,
                dx_style_compatibility_present,
                dx_style_compatibility_missing,
            ),
            findings,
        );
    };

    package_present = 1;

    if dx_style_compatibility_is_present(visibility) {
        dx_style_compatibility_present = 1;
    } else {
        dx_style_compatibility_missing = 1;
        findings.push(missing_dx_style_compatibility_finding());
    }

    let package_receipt_path = json_text(visibility, &["package_receipt_path"])
        .unwrap_or(DATA_FETCHING_CACHE_DASHBOARD_RECEIPT);
    if package_receipt_exists(root, package_receipt_path) {
        receipt_present = 1;
    } else {
        missing_receipt = 1;
        findings.push(missing_receipt_finding(package_receipt_path));
    }

    let visibility_status = json_text(visibility, &["status"]);
    let receipt_status = json_text(visibility, &["receipt_status"]);
    if matches!(visibility_status, Some("stale")) || matches!(receipt_status, Some("stale")) {
        stale_receipt = 1;
    }
    if matches!(visibility_status, Some("blocked")) || matches!(receipt_status, Some("blocked")) {
        blocked_surfaces += 1;
    }
    if matches!(visibility_status, Some("unsupported-surface"))
        || matches!(receipt_status, Some("unsupported-surface"))
    {
        unsupported_surfaces += 1;
    }

    blocked_surfaces += json_array_entries(visibility, &["blocked_surfaces"]).len() as u64;
    unsupported_surfaces += json_array_entries(visibility, &["unsupported_surfaces"]).len() as u64;

    for surface in json_array_entries(visibility, &["selected_surfaces"]) {
        if json_text(surface, &["hash_algorithm"]) == Some("sha256")
            && surface
                .get("file_hashes")
                .and_then(serde_json::Value::as_object)
                .is_some_and(|hashes| !hashes.is_empty())
        {
            hash_manifest_present = 1;
        }

        hash_mismatches += count_sha256_file_hash_mismatches(root, surface);

        match json_text(surface, &["status"]) {
            Some("stale") => stale_receipt = 1,
            Some("blocked") => blocked_surfaces += 1,
            Some("unsupported-surface") => unsupported_surfaces += 1,
            _ => {}
        }
    }

    if hash_mismatches > 0 {
        stale_receipt = 1;
    }

    if stale_receipt > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "data-fetching-cache-stale-receipt",
            format!("{DATA_FETCHING_CACHE_OFFICIAL_NAME} package-status visibility is stale"),
            Some(DATA_FETCHING_CACHE_PACKAGE_STATUS.to_string()),
            "Regenerate the Data Fetching & Cache package-status row from the dashboard workflow receipt before claiming source freshness.",
        ));
    }
    if blocked_surfaces > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "data-fetching-cache-blocked-surface",
            format!(
                "{DATA_FETCHING_CACHE_OFFICIAL_NAME} has {blocked_surfaces} blocked selected surface(s)"
            ),
            Some(DATA_FETCHING_CACHE_PACKAGE_STATUS.to_string()),
            "Resolve the ADAPTER-BOUNDARY Data Fetching & Cache runtime ownership before claiming live QueryClient runtime proof.",
        ));
    }
    if unsupported_surfaces > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "data-fetching-cache-unsupported-surface",
            format!(
                "{DATA_FETCHING_CACHE_OFFICIAL_NAME} has {unsupported_surfaces} unsupported requested surface(s)"
            ),
            Some(DATA_FETCHING_CACHE_PACKAGE_STATUS.to_string()),
            "Request only supported Data Fetching & Cache dashboard, provider, prefetch, hydration, persistence, or cache-control surfaces, or add a real upstream-backed Forge surface first.",
        ));
    }
    if hash_mismatches > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "data-fetching-cache-hash-mismatch",
            format!(
                "{DATA_FETCHING_CACHE_OFFICIAL_NAME} has {hash_mismatches} missing or stale hash-backed selected file(s)"
            ),
            Some(DATA_FETCHING_CACHE_PACKAGE_STATUS.to_string()),
            "Regenerate the Data Fetching & Cache dashboard workflow receipt after reviewing the changed front-facing query workflow files.",
        ));
    }

    (
        data_fetching_cache_metrics(
            package_present,
            receipt_present,
            stale_receipt,
            missing_receipt,
            blocked_surfaces,
            unsupported_surfaces,
            hash_manifest_present,
            hash_mismatches,
            dx_style_compatibility_present,
            dx_style_compatibility_missing,
        ),
        findings,
    )
}

fn data_fetching_cache_metrics(
    package_present: u64,
    receipt_present: u64,
    stale_receipt: u64,
    missing_receipt: u64,
    blocked_surfaces: u64,
    unsupported_surfaces: u64,
    hash_manifest_present: u64,
    hash_mismatches: u64,
    dx_style_compatibility_present: u64,
    dx_style_compatibility_missing: u64,
) -> Vec<DxCheckMetric> {
    vec![
        check_metric("data_fetching_cache_package_present", package_present),
        check_metric("data_fetching_cache_receipt_present", receipt_present),
        check_metric("data_fetching_cache_receipt_stale", stale_receipt),
        check_metric("data_fetching_cache_missing_receipt", missing_receipt),
        check_metric("data_fetching_cache_blocked_surface", blocked_surfaces),
        check_metric(
            "data_fetching_cache_unsupported_surface",
            unsupported_surfaces,
        ),
        check_metric(
            "data_fetching_cache_hash_manifest_present",
            hash_manifest_present,
        ),
        check_metric("data_fetching_cache_hash_mismatch", hash_mismatches),
        check_metric(
            "data_fetching_cache_dx_style_compatibility_present",
            dx_style_compatibility_present,
        ),
        check_metric(
            "data_fetching_cache_dx_style_compatibility_missing",
            dx_style_compatibility_missing,
        ),
    ]
}

fn missing_package_status_finding() -> DxCheckFinding {
    check_finding(
        DxSupplyChainSeverity::Medium,
        "data-fetching-cache-missing-package-status",
        format!("{DATA_FETCHING_CACHE_OFFICIAL_NAME} is missing from package-status visibility"),
        Some(DATA_FETCHING_CACHE_PACKAGE_STATUS.to_string()),
        "Regenerate .dx/forge/package-status.json so dx-check can report Data Fetching & Cache visibility states.",
    )
}

fn missing_receipt_finding(receipt_path: &str) -> DxCheckFinding {
    check_finding(
        DxSupplyChainSeverity::Medium,
        "data-fetching-cache-missing-receipt",
        format!("{DATA_FETCHING_CACHE_OFFICIAL_NAME} is missing its package-lane receipt"),
        Some(receipt_path.to_string()),
        "Regenerate or restore the Data Fetching & Cache dashboard workflow receipt so dx-check can report source-owned package visibility without claiming live QueryClient runtime proof.",
    )
}

fn missing_dx_style_compatibility_finding() -> DxCheckFinding {
    check_finding(
        DxSupplyChainSeverity::Medium,
        "data-fetching-cache-missing-dx-style-compatibility",
        format!("{DATA_FETCHING_CACHE_OFFICIAL_NAME} is missing dx-style compatibility evidence"),
        Some(DATA_FETCHING_CACHE_PACKAGE_STATUS.to_string()),
        "Regenerate the Data Fetching & Cache package-status row from the dashboard workflow receipt and verify token-surface markers without claiming live QueryClient/browser proof.",
    )
}

fn dx_style_compatibility_is_present(visibility: &serde_json::Value) -> bool {
    let Some(dx_style_compatibility) = visibility.get("dx_style_compatibility") else {
        return false;
    };

    json_text(dx_style_compatibility, &["schema"])
        == Some("dx.forge.package.dx_style_compatibility")
        && json_text(dx_style_compatibility, &["status"]) == Some("present")
}

fn package_receipt_exists(root: &Path, receipt_path: &str) -> bool {
    project_file_path(root, receipt_path).is_some()
}

fn project_file_path(root: &Path, relative_path: &str) -> Option<std::path::PathBuf> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Digest, Sha256};
    use std::{fs, path::Path};

    #[test]
    fn data_fetching_cache_hash_mismatch_flips_when_selected_file_changes() {
        let dir = tempfile::tempdir().expect("tempdir");
        let query_status_dir = dir.path().join("examples/template");
        fs::create_dir_all(&query_status_dir).expect("query status dir");
        let query_status_file = query_status_dir.join("query-cache-status.tsx");
        fs::write(
            &query_status_file,
            b"export const stableQueryCacheStatus = true;\n",
        )
        .expect("query cache status source");

        let expected_hash = sha256_file(&query_status_file);
        write_data_fetching_cache_receipt(dir.path());
        write_data_fetching_cache_package_status(dir.path(), &expected_hash, true);

        let manifest = DxSourceManifest::default();
        let (metrics, findings) = forge_data_fetching_cache_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(&metrics, "data_fetching_cache_hash_manifest_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(&metrics, "data_fetching_cache_hash_mismatch"),
            Some(0)
        );
        assert_eq!(
            metric_value(&metrics, "data_fetching_cache_receipt_stale"),
            Some(0)
        );
        assert!(
            !findings
                .iter()
                .any(|finding| finding.code == "data-fetching-cache-hash-mismatch")
        );

        fs::write(
            &query_status_file,
            b"export const staleQueryCacheStatus = true;\n",
        )
        .expect("stale query cache status source");
        let (stale_metrics, stale_findings) =
            forge_data_fetching_cache_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(&stale_metrics, "data_fetching_cache_hash_mismatch"),
            Some(1)
        );
        assert_eq!(
            metric_value(&stale_metrics, "data_fetching_cache_receipt_stale"),
            Some(1)
        );
        assert!(
            stale_findings
                .iter()
                .any(|finding| finding.code == "data-fetching-cache-hash-mismatch")
        );
    }

    #[test]
    fn data_fetching_cache_dx_style_compatibility_metric_tracks_package_status_row() {
        let dir = tempfile::tempdir().expect("tempdir");
        let query_status_dir = dir.path().join("examples/template");
        fs::create_dir_all(&query_status_dir).expect("query status dir");
        let query_status_file = query_status_dir.join("query-cache-status.tsx");
        fs::write(
            &query_status_file,
            b"export const styledQueryCacheStatus = true;\n",
        )
        .expect("query cache status source");

        let expected_hash = sha256_file(&query_status_file);
        write_data_fetching_cache_receipt(dir.path());
        write_data_fetching_cache_package_status(dir.path(), &expected_hash, true);

        let manifest = DxSourceManifest::default();
        let (metrics, findings) = forge_data_fetching_cache_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(
                &metrics,
                "data_fetching_cache_dx_style_compatibility_present"
            ),
            Some(1)
        );
        assert_eq!(
            metric_value(
                &metrics,
                "data_fetching_cache_dx_style_compatibility_missing"
            ),
            Some(0)
        );
        assert!(
            !findings
                .iter()
                .any(|finding| finding.code
                    == "data-fetching-cache-missing-dx-style-compatibility")
        );

        write_data_fetching_cache_package_status(dir.path(), &expected_hash, false);
        let (missing_metrics, missing_findings) =
            forge_data_fetching_cache_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(
                &missing_metrics,
                "data_fetching_cache_dx_style_compatibility_present"
            ),
            Some(0)
        );
        assert_eq!(
            metric_value(
                &missing_metrics,
                "data_fetching_cache_dx_style_compatibility_missing"
            ),
            Some(1)
        );
        assert!(
            missing_findings
                .iter()
                .any(|finding| finding.code
                    == "data-fetching-cache-missing-dx-style-compatibility")
        );
    }

    fn write_data_fetching_cache_receipt(root: &Path) {
        let receipt_path = root.join(DATA_FETCHING_CACHE_DASHBOARD_RECEIPT);
        fs::create_dir_all(receipt_path.parent().expect("receipt parent"))
            .expect("receipt directory");
        fs::write(
            receipt_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.forge.data_fetching_cache_receipt",
                "official_package_name": DATA_FETCHING_CACHE_OFFICIAL_NAME,
                "package_id": DATA_FETCHING_CACHE_PACKAGE_ID
            }))
            .expect("receipt json"),
        )
        .expect("write receipt");
    }

    fn write_data_fetching_cache_package_status(
        root: &Path,
        expected_hash: &str,
        include_dx_style_compatibility: bool,
    ) {
        let package_status_path = root.join(DATA_FETCHING_CACHE_PACKAGE_STATUS);
        fs::create_dir_all(package_status_path.parent().expect("package status parent"))
            .expect("package status directory");
        let mut package_lane = serde_json::json!({
            "official_package_name": DATA_FETCHING_CACHE_OFFICIAL_NAME,
            "package_id": DATA_FETCHING_CACHE_PACKAGE_ID,
            "upstream_package": "@tanstack/react-query",
            "upstream_version": "5.100.10",
            "source_mirror": "G:/WWW/inspirations/tanstack-query",
            "status": "present",
            "receipt_status": "present",
            "package_receipt_path": DATA_FETCHING_CACHE_DASHBOARD_RECEIPT,
            "selected_surfaces": [
                {
                    "surface_id": "data-fetching-cache-query-dashboard-workflow",
                    "status": "present",
                    "hash_algorithm": "sha256",
                    "file_hashes": {
                        "examples/template/query-cache-status.tsx": expected_hash
                    }
                }
            ],
            "blocked_surfaces": [],
            "unsupported_surfaces": []
        });

        if include_dx_style_compatibility {
            package_lane["dx_style_compatibility"] = serde_json::json!({
                "schema": "dx.forge.package.dx_style_compatibility",
                "status": "present",
                "token_source": "styles/theme.css",
                "generated_css": "styles/generated.css",
                "visible_surfaces": ["data-fetching-cache-query-dashboard-workflow"],
                "source_files": ["examples/template/query-cache-status.tsx"],
                "runtime_proof": false
            });
        }

        fs::write(
            package_status_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.www.template.forge_package_status",
                "package_lane_visibility": [package_lane]
            }))
            .expect("package status json"),
        )
        .expect("write package status");
    }

    fn sha256_file(path: &Path) -> String {
        let bytes = fs::read(path).expect("hash source file");
        let digest = Sha256::digest(bytes);
        format!("{digest:x}")
    }

    fn metric_value(metrics: &[DxCheckMetric], name: &str) -> Option<u64> {
        metrics
            .iter()
            .find(|metric| metric.name == name)
            .map(|metric| metric.value)
    }
}
