#![allow(clippy::too_many_arguments)]
use std::path::Path;

use super::super::forge_security::{DxSourceManifest, DxSupplyChainSeverity};
use super::file_hashes::{count_sha256_file_hash_mismatches, count_sha256_path_hash_mismatches};
use super::{
    DxCheckFinding, DxCheckMetric, check_finding, check_metric, json_array_entries, json_text,
    read_optional_forge_json, resolve_dx_check_relative_path,
};

const DOCUMENTATION_SYSTEM_PACKAGE_ID: &str = "content/fumadocs-next";
const DOCUMENTATION_SYSTEM_OFFICIAL_NAME: &str = "Documentation System";
const DOCUMENTATION_SYSTEM_PACKAGE_STATUS: &str = ".dx/forge/package-status.json";
const DOCUMENTATION_SYSTEM_DASHBOARD_RECEIPT: &str =
    "examples/template/.dx/forge/receipts/2026-05-22-content-fumadocs-dashboard-workflow.json";

pub(super) fn forge_documentation_system_package_metrics(
    root: &Path,
    manifest: &DxSourceManifest,
) -> (Vec<DxCheckMetric>, Vec<DxCheckFinding>) {
    let mut findings = Vec::new();
    let manifest_package_present = manifest
        .packages
        .iter()
        .any(|package| package.package_id == DOCUMENTATION_SYSTEM_PACKAGE_ID);
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

    let Some(package_status) = read_optional_forge_json(
        root,
        DOCUMENTATION_SYSTEM_PACKAGE_STATUS,
        "documentation-system-package-status-invalid",
        &mut findings,
    ) else {
        if manifest_package_present
            || package_receipt_exists(root, DOCUMENTATION_SYSTEM_DASHBOARD_RECEIPT)
        {
            package_present = 1;
            missing_receipt = 1;
            receipt_hash_refresh_missing = 1;
            findings.push(missing_package_status_finding());
        }
        return (
            documentation_system_metrics(
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
            ),
            findings,
        );
    };

    let Some(visibility) = json_array_entries(&package_status, &["package_lane_visibility"])
        .into_iter()
        .find(|entry| json_text(entry, &["package_id"]) == Some(DOCUMENTATION_SYSTEM_PACKAGE_ID))
    else {
        if manifest_package_present
            || package_receipt_exists(root, DOCUMENTATION_SYSTEM_DASHBOARD_RECEIPT)
        {
            package_present = 1;
            missing_receipt = 1;
            receipt_hash_refresh_missing = 1;
            findings.push(missing_package_status_finding());
        }
        return (
            documentation_system_metrics(
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
            ),
            findings,
        );
    };

    package_present = 1;
    (
        receipt_hash_refresh_current,
        receipt_hash_refresh_stale,
        receipt_hash_refresh_missing,
    ) = receipt_hash_refresh_counts(visibility);

    let package_receipt_path = json_text(visibility, &["package_receipt_path"])
        .unwrap_or(DOCUMENTATION_SYSTEM_DASHBOARD_RECEIPT);
    if package_receipt_exists(root, package_receipt_path) {
        receipt_present = 1;
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

        hash_mismatches += count_sha256_file_hash_mismatches(root, surface);

        match json_text(surface, &["status"]) {
            status if status_is_stale(status) => stale_receipt = 1,
            status if status_is_missing_receipt(status) => missing_receipt = 1,
            status if status_is_blocked(status) => blocked_surfaces += 1,
            status if status_is_unsupported_surface(status) => unsupported_surfaces += 1,
            _ => {}
        }
    }

    if has_sha256_source_hashes(visibility) {
        hash_manifest_present = 1;
    }
    hash_mismatches += count_source_hash_mismatches(root, visibility);

    if dx_style_compatibility_is_present(visibility) {
        dx_style_compatibility_present = 1;
    } else {
        dx_style_compatibility_missing = 1;
    }

    if hash_mismatches > 0 || receipt_hash_refresh_stale > 0 || receipt_hash_refresh_missing > 0 {
        stale_receipt = 1;
    }

    if stale_receipt > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "documentation-system-stale-receipt",
            format!("{DOCUMENTATION_SYSTEM_OFFICIAL_NAME} package-status visibility is stale"),
            Some(DOCUMENTATION_SYSTEM_PACKAGE_STATUS.to_string()),
            "Regenerate the Documentation System package-status row from the dashboard workflow receipt before claiming source freshness.",
        ));
    }
    if blocked_surfaces > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "documentation-system-blocked-surface",
            format!(
                "{DOCUMENTATION_SYSTEM_OFFICIAL_NAME} has {blocked_surfaces} app-owned runtime boundary surface(s)"
            ),
            Some(DOCUMENTATION_SYSTEM_PACKAGE_STATUS.to_string()),
            "Resolve the app-owned Documentation System dependency installation, docs runtime, OpenAPI proxy, search hosting, content policy, and governed browser QA boundaries before claiming live Fumadocs runtime proof.",
        ));
    }
    if unsupported_surfaces > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "documentation-system-unsupported-surface",
            format!(
                "{DOCUMENTATION_SYSTEM_OFFICIAL_NAME} has {unsupported_surfaces} unsupported requested surface(s)"
            ),
            Some(DOCUMENTATION_SYSTEM_PACKAGE_STATUS.to_string()),
            "Request only supported Documentation System docs route, dashboard workflow, LLM export, OpenAPI, or search surfaces, or add a real upstream-backed Forge surface first.",
        ));
    }
    if hash_mismatches > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "documentation-system-hash-mismatch",
            format!(
                "{DOCUMENTATION_SYSTEM_OFFICIAL_NAME} has {hash_mismatches} missing or stale hash-backed source file(s)"
            ),
            Some(DOCUMENTATION_SYSTEM_PACKAGE_STATUS.to_string()),
            "Regenerate the Documentation System dashboard workflow receipt after reviewing the changed docs, launch, dashboard, or Forge package files.",
        ));
    }
    if dx_style_compatibility_missing > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "documentation-system-missing-dx-style-compatibility",
            format!(
                "{DOCUMENTATION_SYSTEM_OFFICIAL_NAME} is missing dx-style compatibility evidence"
            ),
            Some(DOCUMENTATION_SYSTEM_PACKAGE_STATUS.to_string()),
            "Regenerate the Documentation System dx-style compatibility row from the receipt and verify token-surface markers without claiming live Fumadocs renderer runtime proof.",
        ));
    }

    (
        documentation_system_metrics(
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
        ),
        findings,
    )
}

fn documentation_system_metrics(
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
) -> Vec<DxCheckMetric> {
    vec![
        check_metric("documentation_system_package_present", package_present),
        check_metric("documentation_system_receipt_present", receipt_present),
        check_metric("documentation_system_receipt_stale", stale_receipt),
        check_metric("documentation_system_missing_receipt", missing_receipt),
        check_metric("documentation_system_blocked_surface", blocked_surfaces),
        check_metric(
            "documentation_system_unsupported_surface",
            unsupported_surfaces,
        ),
        check_metric(
            "documentation_system_hash_manifest_present",
            hash_manifest_present,
        ),
        check_metric("documentation_system_hash_mismatch", hash_mismatches),
        check_metric(
            "documentation_system_receipt_hash_refresh_current",
            receipt_hash_refresh_current,
        ),
        check_metric(
            "documentation_system_receipt_hash_refresh_stale",
            receipt_hash_refresh_stale,
        ),
        check_metric(
            "documentation_system_receipt_hash_refresh_missing",
            receipt_hash_refresh_missing,
        ),
        check_metric(
            "documentation_system_dx_style_compatibility_present",
            dx_style_compatibility_present,
        ),
        check_metric(
            "documentation_system_dx_style_compatibility_missing",
            dx_style_compatibility_missing,
        ),
    ]
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

fn has_sha256_file_hashes(surface: &serde_json::Value) -> bool {
    json_text(surface, &["hash_algorithm"]) == Some("sha256")
        && surface
            .get("file_hashes")
            .and_then(serde_json::Value::as_object)
            .is_some_and(|hashes| !hashes.is_empty())
}

fn has_sha256_source_hashes(visibility: &serde_json::Value) -> bool {
    source_hash_algorithm_is_sha256(visibility) && source_hash_entries(visibility).next().is_some()
}

fn count_source_hash_mismatches(root: &Path, visibility: &serde_json::Value) -> u64 {
    if !source_hash_algorithm_is_sha256(visibility) {
        return 0;
    }

    count_sha256_path_hash_mismatches(root, source_hash_entries(visibility))
}

fn source_hash_algorithm_is_sha256(visibility: &serde_json::Value) -> bool {
    visibility
        .get("source_hashes")
        .and_then(|value| json_text(value, &["algorithm"]))
        == Some("sha256")
}

fn source_hash_entries<'a>(
    visibility: &'a serde_json::Value,
) -> Box<dyn Iterator<Item = (&'a str, &'a str)> + 'a> {
    let Some(files) = visibility
        .get("source_hashes")
        .and_then(|value| value.get("files"))
    else {
        return Box::new(std::iter::empty());
    };

    if let Some(entries) = files.as_object() {
        return Box::new(
            entries
                .iter()
                .filter_map(|(path, hash)| hash.as_str().map(|hash| (path.as_str(), hash))),
        );
    }

    Box::new(files.as_array().into_iter().flatten().filter_map(|entry| {
        let path = json_text(entry, &["path"])?;
        let hash = json_text(entry, &["sha256", "hash"])?;
        Some((path, hash))
    }))
}

fn dx_style_compatibility_is_present(visibility: &serde_json::Value) -> bool {
    let Some(dx_style) = visibility.get("dx_style_compatibility") else {
        return false;
    };

    json_text(dx_style, &["schema"]) == Some("dx.forge.package.dx_style_compatibility")
        && json_text(dx_style, &["status"]).unwrap_or("present") == "present"
}

fn receipt_hash_refresh_counts(visibility: &serde_json::Value) -> (u64, u64, u64) {
    let Some(refresh) = visibility.get("receipt_hash_refresh") else {
        return (0, 0, 1);
    };
    if json_text(refresh, &["schema"]) != Some("dx.forge.package.receipt_hash_refresh") {
        return (0, 0, 1);
    }

    let stale_files = json_u64(refresh, "stale_file_count");
    let missing_files = json_u64(refresh, "missing_file_count");
    let status = json_text(refresh, &["status"]).unwrap_or("missing");
    let stale = u64::from(status == "stale" || stale_files > 0);
    let missing = u64::from(status == "missing" || missing_files > 0);
    let current = u64::from(status == "current" && stale == 0 && missing == 0);

    (current, stale, missing)
}

fn json_u64(value: &serde_json::Value, key: &str) -> u64 {
    value
        .get(key)
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0)
}

fn missing_package_status_finding() -> DxCheckFinding {
    check_finding(
        DxSupplyChainSeverity::Medium,
        "documentation-system-missing-package-status",
        format!("{DOCUMENTATION_SYSTEM_OFFICIAL_NAME} is missing from package-status visibility"),
        Some(DOCUMENTATION_SYSTEM_PACKAGE_STATUS.to_string()),
        "Regenerate .dx/forge/package-status.json so dx-check can report Documentation System visibility states.",
    )
}

fn missing_receipt_finding(receipt_path: &str) -> DxCheckFinding {
    check_finding(
        DxSupplyChainSeverity::Medium,
        "documentation-system-missing-receipt",
        format!("{DOCUMENTATION_SYSTEM_OFFICIAL_NAME} is missing its package-lane receipt"),
        Some(receipt_path.to_string()),
        "Regenerate or restore the Documentation System dashboard workflow receipt so dx-check can report source-owned package visibility without claiming live Fumadocs renderer runtime proof.",
    )
}

fn package_receipt_exists(root: &Path, receipt_path: &str) -> bool {
    if resolve_dx_check_relative_path(root, receipt_path).is_some_and(|path| path.is_file()) {
        return true;
    }

    receipt_path
        .strip_prefix("examples/template/")
        .and_then(|template_relative| resolve_dx_check_relative_path(root, template_relative))
        .is_some_and(|path| path.is_file())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use super::*;

    const DOCS_INDEX_PATH: &str = "examples/template/content/docs/index.mdx";
    const FRESH_DOCS_INDEX: &str = "# Launch Docs\n\nFresh documentation content.\n";
    const FRESH_DOCS_INDEX_HASH: &str =
        "bc33e446260709892216ac39230c75feaecb72931268a58558fc39190ec62dda";

    #[test]
    fn documentation_system_hash_mismatch_metric_and_finding_are_byte_derived() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_documentation_system_receipt(dir.path());

        let docs_index = dir.path().join(DOCS_INDEX_PATH);
        fs::create_dir_all(docs_index.parent().expect("docs index parent"))
            .expect("docs index directory");
        fs::write(&docs_index, FRESH_DOCS_INDEX).expect("docs index file");
        write_documentation_system_package_status(dir.path(), FRESH_DOCS_INDEX_HASH);

        let manifest = DxSourceManifest::default();
        let (metrics, findings) = forge_documentation_system_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(&metrics, "documentation_system_hash_manifest_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(&metrics, "documentation_system_receipt_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(&metrics, "documentation_system_receipt_stale"),
            Some(0)
        );
        assert_eq!(
            metric_value(&metrics, "documentation_system_hash_mismatch"),
            Some(0)
        );
        assert_eq!(
            metric_value(
                &metrics,
                "documentation_system_receipt_hash_refresh_current"
            ),
            Some(1)
        );
        assert_eq!(
            metric_value(&metrics, "documentation_system_receipt_hash_refresh_stale"),
            Some(0)
        );
        assert_eq!(
            metric_value(
                &metrics,
                "documentation_system_receipt_hash_refresh_missing"
            ),
            Some(0)
        );
        assert!(
            !findings
                .iter()
                .any(|finding| finding.code == "documentation-system-hash-mismatch")
        );

        let package_status_path = dir.path().join(DOCUMENTATION_SYSTEM_PACKAGE_STATUS);
        let mut package_status: serde_json::Value =
            serde_json::from_slice(&fs::read(&package_status_path).expect("package status bytes"))
                .expect("package status json");
        package_status["package_lane_visibility"][0]["receipt_hash_refresh"]["status"] =
            serde_json::json!("stale");
        package_status["package_lane_visibility"][0]["receipt_hash_refresh"]["stale_file_count"] =
            serde_json::json!(1);
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&package_status).expect("stale helper package status json"),
        )
        .expect("write stale helper package status");

        let (helper_stale_metrics, helper_stale_findings) =
            forge_documentation_system_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(
                &helper_stale_metrics,
                "documentation_system_receipt_hash_refresh_current"
            ),
            Some(0)
        );
        assert_eq!(
            metric_value(
                &helper_stale_metrics,
                "documentation_system_receipt_hash_refresh_stale"
            ),
            Some(1)
        );
        assert_eq!(
            metric_value(
                &helper_stale_metrics,
                "documentation_system_receipt_hash_refresh_missing"
            ),
            Some(0)
        );
        assert_eq!(
            metric_value(&helper_stale_metrics, "documentation_system_receipt_stale"),
            Some(1)
        );
        assert_eq!(
            metric_value(&helper_stale_metrics, "documentation_system_hash_mismatch"),
            Some(0)
        );
        assert!(
            helper_stale_findings
                .iter()
                .any(|finding| finding.code == "documentation-system-stale-receipt")
        );

        write_documentation_system_package_status(dir.path(), FRESH_DOCS_INDEX_HASH);
        fs::write(
            &docs_index,
            "# Launch Docs\n\nStale documentation content.\n",
        )
        .expect("mutate docs index");
        let (stale_metrics, stale_findings) =
            forge_documentation_system_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(&stale_metrics, "documentation_system_receipt_stale"),
            Some(1)
        );
        assert_eq!(
            metric_value(&stale_metrics, "documentation_system_hash_mismatch"),
            Some(1)
        );
        assert!(
            stale_findings
                .iter()
                .any(|finding| finding.code == "documentation-system-hash-mismatch")
        );
    }

    fn write_documentation_system_receipt(root: &Path) {
        let receipt_path = root.join(DOCUMENTATION_SYSTEM_DASHBOARD_RECEIPT);
        fs::create_dir_all(receipt_path.parent().expect("receipt parent"))
            .expect("receipt directory");
        fs::write(
            receipt_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.forge.package_dashboard_workflow_receipt",
                "official_dx_package_name": DOCUMENTATION_SYSTEM_OFFICIAL_NAME,
                "package_id": DOCUMENTATION_SYSTEM_PACKAGE_ID
            }))
            .expect("receipt json"),
        )
        .expect("write receipt");
    }

    fn write_documentation_system_package_status(root: &Path, expected_hash: &str) {
        let package_status_path = root.join(DOCUMENTATION_SYSTEM_PACKAGE_STATUS);
        fs::create_dir_all(package_status_path.parent().expect("package status parent"))
            .expect("package status directory");
        fs::write(
            package_status_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.www.template.forge_package_status",
                "package_lane_visibility": [
                    {
                        "official_package_name": DOCUMENTATION_SYSTEM_OFFICIAL_NAME,
                        "package_id": DOCUMENTATION_SYSTEM_PACKAGE_ID,
                        "upstream_package": "fumadocs",
                        "upstream_version": "16.8.12",
                        "source_mirror": "G:/WWW/inspirations/fumadocs",
                        "status": "present",
                        "receipt_status": "present",
                        "package_receipt_path": DOCUMENTATION_SYSTEM_DASHBOARD_RECEIPT,
                        "selected_surfaces": [
                            {
                                "surface_id": "docs-app-router",
                                "status": "present",
                                "hash_algorithm": "sha256",
                                "file_hashes": {
                                    DOCS_INDEX_PATH: expected_hash
                                }
                            }
                        ],
                        "blocked_surfaces": [],
                        "unsupported_surfaces": [],
                        "receipt_hash_refresh": {
                            "schema": "dx.forge.package.receipt_hash_refresh",
                            "status": "current",
                            "tracked_file_count": 1,
                            "stale_file_count": 0,
                            "missing_file_count": 0,
                            "zed_visibility": "documentation-system:receipt-hash-refresh"
                        },
                        "dx_style_compatibility": {
                            "schema": "dx.forge.package.dx_style_compatibility",
                            "status": "present"
                        }
                    }
                ]
            }))
            .expect("package status json"),
        )
        .expect("write package status");
    }

    fn metric_value(metrics: &[DxCheckMetric], name: &str) -> Option<u64> {
        metrics
            .iter()
            .find(|metric| metric.name == name)
            .map(|metric| metric.value)
    }
}
