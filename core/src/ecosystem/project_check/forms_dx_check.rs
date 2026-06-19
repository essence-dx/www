#![allow(clippy::too_many_arguments)]
use std::{fs, path::Path};

use sha2::{Digest, Sha256};

use super::super::forge_security::{DxSourceManifest, DxSupplyChainSeverity};
use super::{
    DxCheckFinding, DxCheckMetric, check_finding, check_metric, json_array_entries, json_text,
    read_optional_forge_json, resolve_dx_check_relative_path,
};

const FORMS_PACKAGE_ID: &str = "forms/react-hook-form";
const FORMS_OFFICIAL_NAME: &str = "Forms";
const FORMS_PACKAGE_STATUS: &str = ".dx/forge/package-status.json";
const FORMS_DASHBOARD_RECEIPT: &str = ".dx/forge/receipts/2026-05-22-forms-dashboard-workflow.json";

pub(super) fn forge_forms_package_metrics(
    root: &Path,
    manifest: &DxSourceManifest,
) -> (Vec<DxCheckMetric>, Vec<DxCheckFinding>) {
    let mut findings = Vec::new();
    let manifest_package_present = manifest
        .packages
        .iter()
        .any(|package| package.package_id == FORMS_PACKAGE_ID);
    let mut package_present = u64::from(manifest_package_present);
    let mut receipt_present = 0u64;
    let mut stale_receipt = 0u64;
    let mut missing_receipt = 0u64;
    let mut blocked_surfaces = 0u64;
    let mut unsupported_surfaces = 0u64;
    let mut hash_manifest_present = 0u64;
    let mut surface_hash_mismatches = 0u64;
    let mut receipt_hash_refresh_current = 0u64;
    let mut receipt_hash_refresh_stale = 0u64;
    let mut receipt_hash_refresh_missing = 0u64;

    let Some(package_status) = read_optional_forge_json(
        root,
        FORMS_PACKAGE_STATUS,
        "forms-package-status-invalid",
        &mut findings,
    ) else {
        if manifest_package_present || package_receipt_exists(root, FORMS_DASHBOARD_RECEIPT) {
            package_present = 1;
            missing_receipt = 1;
            receipt_hash_refresh_missing = 1;
            findings.push(missing_package_status_finding());
        }
        return (
            forms_metrics(
                package_present,
                receipt_present,
                stale_receipt,
                missing_receipt,
                blocked_surfaces,
                unsupported_surfaces,
                hash_manifest_present,
                surface_hash_mismatches,
                receipt_hash_refresh_current,
                receipt_hash_refresh_stale,
                receipt_hash_refresh_missing,
            ),
            findings,
        );
    };

    let Some(visibility) = json_array_entries(&package_status, &["package_lane_visibility"])
        .into_iter()
        .find(|entry| json_text(entry, &["package_id"]) == Some(FORMS_PACKAGE_ID))
    else {
        if manifest_package_present || package_receipt_exists(root, FORMS_DASHBOARD_RECEIPT) {
            package_present = 1;
            missing_receipt = 1;
            receipt_hash_refresh_missing = 1;
            findings.push(missing_package_status_finding());
        }
        return (
            forms_metrics(
                package_present,
                receipt_present,
                stale_receipt,
                missing_receipt,
                blocked_surfaces,
                unsupported_surfaces,
                hash_manifest_present,
                surface_hash_mismatches,
                receipt_hash_refresh_current,
                receipt_hash_refresh_stale,
                receipt_hash_refresh_missing,
            ),
            findings,
        );
    };

    package_present = 1;
    let (refresh_current, refresh_stale, refresh_missing) = receipt_hash_refresh_counts(visibility);
    receipt_hash_refresh_current = refresh_current;
    receipt_hash_refresh_stale = refresh_stale;
    receipt_hash_refresh_missing = refresh_missing;
    if refresh_stale > 0 || refresh_missing > 0 {
        stale_receipt = 1;
    }

    let package_receipt_path =
        json_text(visibility, &["package_receipt_path"]).unwrap_or(FORMS_DASHBOARD_RECEIPT);
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

        surface_hash_mismatches += count_surface_hash_mismatches(root, surface);

        match json_text(surface, &["status"]) {
            Some("stale") => stale_receipt = 1,
            Some("blocked") => blocked_surfaces += 1,
            Some("unsupported-surface") => unsupported_surfaces += 1,
            _ => {}
        }
    }

    if surface_hash_mismatches > 0 {
        stale_receipt = 1;
    }

    if stale_receipt > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "forms-stale-receipt",
            format!("{FORMS_OFFICIAL_NAME} package-status visibility is stale"),
            Some(FORMS_PACKAGE_STATUS.to_string()),
            "Regenerate the Forms package-status row from the dashboard workflow receipt before claiming source freshness.",
        ));
    }
    if blocked_surfaces > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "forms-blocked-surface",
            format!("{FORMS_OFFICIAL_NAME} has {blocked_surfaces} blocked selected surface(s)"),
            Some(FORMS_PACKAGE_STATUS.to_string()),
            "Resolve the ADAPTER-BOUNDARY Forms runtime ownership before claiming browser submission proof.",
        ));
    }
    if unsupported_surfaces > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "forms-unsupported-surface",
            format!(
                "{FORMS_OFFICIAL_NAME} has {unsupported_surfaces} unsupported requested surface(s)"
            ),
            Some(FORMS_PACKAGE_STATUS.to_string()),
            "Request only supported Forms provider, field, field-array, resolver, or launch-lead surfaces, or add a real upstream-backed Forms surface first.",
        ));
    }
    if surface_hash_mismatches > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "forms-hash-mismatch",
            format!(
                "{FORMS_OFFICIAL_NAME} has {surface_hash_mismatches} missing or stale hash-backed selected file(s)"
            ),
            Some(FORMS_PACKAGE_STATUS.to_string()),
            "Regenerate the Forms dashboard workflow receipt and package-status hash manifest after reviewing the changed front-facing Forms files.",
        ));
    }

    (
        forms_metrics(
            package_present,
            receipt_present,
            stale_receipt,
            missing_receipt,
            blocked_surfaces,
            unsupported_surfaces,
            hash_manifest_present,
            surface_hash_mismatches,
            receipt_hash_refresh_current,
            receipt_hash_refresh_stale,
            receipt_hash_refresh_missing,
        ),
        findings,
    )
}

fn forms_metrics(
    package_present: u64,
    receipt_present: u64,
    stale_receipt: u64,
    missing_receipt: u64,
    blocked_surfaces: u64,
    unsupported_surfaces: u64,
    hash_manifest_present: u64,
    surface_hash_mismatches: u64,
    receipt_hash_refresh_current: u64,
    receipt_hash_refresh_stale: u64,
    receipt_hash_refresh_missing: u64,
) -> Vec<DxCheckMetric> {
    vec![
        check_metric("forms_package_present", package_present),
        check_metric("forms_receipt_present", receipt_present),
        check_metric("forms_receipt_stale", stale_receipt),
        check_metric("forms_missing_receipt", missing_receipt),
        check_metric("forms_blocked_surface", blocked_surfaces),
        check_metric("forms_unsupported_surface", unsupported_surfaces),
        check_metric("forms_hash_manifest_present", hash_manifest_present),
        check_metric("forms_hash_mismatch", surface_hash_mismatches),
        check_metric(
            "forms_receipt_hash_refresh_current",
            receipt_hash_refresh_current,
        ),
        check_metric(
            "forms_receipt_hash_refresh_stale",
            receipt_hash_refresh_stale,
        ),
        check_metric(
            "forms_receipt_hash_refresh_missing",
            receipt_hash_refresh_missing,
        ),
    ]
}

fn count_surface_hash_mismatches(root: &Path, surface: &serde_json::Value) -> u64 {
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

fn receipt_hash_refresh_counts(visibility: &serde_json::Value) -> (u64, u64, u64) {
    let Some(refresh) = visibility.get("receipt_hash_refresh") else {
        return (0, 0, 1);
    };

    if json_text(refresh, &["schema"]) != Some("dx.forge.package.receipt_hash_refresh") {
        return (0, 0, 1);
    }

    let stale_file_count = json_u64(refresh, "stale_file_count");
    let missing_file_count = json_u64(refresh, "missing_file_count");
    let stale_path_count = json_string_array(refresh, "stale_files").len() as u64
        + json_string_array(refresh, "stale_mirror_files").len() as u64;
    let missing_path_count = json_string_array(refresh, "missing_files").len() as u64
        + json_string_array(refresh, "missing_mirror_files").len() as u64;
    let status = json_text(refresh, &["status"]).unwrap_or("missing");
    let stale = u64::from(status == "stale" || stale_file_count > 0 || stale_path_count > 0);
    let missing =
        u64::from(status == "missing" || missing_file_count > 0 || missing_path_count > 0);
    let current = u64::from(status == "current" && stale == 0 && missing == 0);

    (current, stale, missing)
}

fn json_u64(value: &serde_json::Value, key: &str) -> u64 {
    value
        .get(key)
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0)
}

fn json_string_array(value: &serde_json::Value, key: &str) -> Vec<String> {
    value
        .get(key)
        .and_then(serde_json::Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(serde_json::Value::as_str)
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

fn missing_package_status_finding() -> DxCheckFinding {
    check_finding(
        DxSupplyChainSeverity::Medium,
        "forms-missing-package-status",
        format!("{FORMS_OFFICIAL_NAME} is missing from package-status visibility"),
        Some(FORMS_PACKAGE_STATUS.to_string()),
        "Regenerate .dx/forge/package-status.json so dx-check can report Forms visibility states.",
    )
}

fn missing_receipt_finding(receipt_path: &str) -> DxCheckFinding {
    check_finding(
        DxSupplyChainSeverity::Medium,
        "forms-missing-receipt",
        format!("{FORMS_OFFICIAL_NAME} is missing its package-lane receipt"),
        Some(receipt_path.to_string()),
        "Regenerate or restore the Forms dashboard workflow receipt so dx-check can report source-owned package visibility without claiming browser submission proof.",
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
    fn forms_hash_mismatch_metric_and_finding_are_byte_derived() {
        let dir = tempfile::tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join(".dx/forge/receipts")).expect("receipt dir");
        fs::write(dir.path().join(FORMS_DASHBOARD_RECEIPT), "{}").expect("forms receipt");
        fs::write(
            dir.path().join("template-lead-form.tsx"),
            "export function LaunchLeadForm() { return null; }\n",
        )
        .expect("forms surface");

        let expected_hash =
            sha256_project_file(dir.path(), "examples/template/template-lead-form.tsx")
                .expect("forms hash");
        fs::write(
            dir.path().join(FORMS_PACKAGE_STATUS),
            serde_json::to_vec_pretty(&serde_json::json!({
                "package_lane_visibility": [
                    {
                        "official_package_name": FORMS_OFFICIAL_NAME,
                        "package_id": FORMS_PACKAGE_ID,
                        "upstream_package": "react-hook-form",
                        "upstream_version": "7.75.0",
                        "source_mirror": "G:/WWW/inspirations/react-hook-form",
                        "status": "present",
                        "receipt_status": "present",
                        "package_receipt_path": FORMS_DASHBOARD_RECEIPT,
                        "selected_surfaces": [
                            {
                                "surface_id": "template-lead-form",
                                "status": "present",
                                "hash_algorithm": "sha256",
                                "file_hashes": {
                                    "examples/template/template-lead-form.tsx": format!("sha256:{expected_hash}")
                                }
                            }
                        ],
                        "blocked_surfaces": [],
                        "unsupported_surfaces": []
                    }
                ]
            }))
            .expect("package status json"),
        )
        .expect("package status");

        let manifest = DxSourceManifest::default();
        let (metrics, findings) = forge_forms_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(&metrics, "forms_hash_manifest_present"),
            Some(1)
        );
        assert_eq!(metric_value(&metrics, "forms_receipt_present"), Some(1));
        assert_eq!(metric_value(&metrics, "forms_receipt_stale"), Some(0));
        assert_eq!(metric_value(&metrics, "forms_hash_mismatch"), Some(0));
        assert!(
            !findings
                .iter()
                .any(|finding| finding.code == "forms-hash-mismatch")
        );

        fs::write(
            dir.path().join("template-lead-form.tsx"),
            "export function LaunchLeadForm() { return <form />; }\n",
        )
        .expect("mutate forms surface");
        let (metrics, findings) = forge_forms_package_metrics(dir.path(), &manifest);
        assert_eq!(metric_value(&metrics, "forms_receipt_stale"), Some(1));
        assert_eq!(metric_value(&metrics, "forms_hash_mismatch"), Some(1));
        assert!(
            findings
                .iter()
                .any(|finding| finding.code == "forms-hash-mismatch")
        );
    }

    fn metric_value(metrics: &[DxCheckMetric], name: &str) -> Option<u64> {
        metrics
            .iter()
            .find(|metric| metric.name == name)
            .map(|metric| metric.value)
    }

    #[test]
    fn forms_package_metrics_reports_helper_freshness_from_path_arrays() {
        let dir = tempfile::tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join(".dx/forge/receipts")).expect("receipt dir");
        fs::write(dir.path().join(FORMS_DASHBOARD_RECEIPT), "{}").expect("forms receipt");

        let package_status_path = dir.path().join(FORMS_PACKAGE_STATUS);
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.www.template.forge_package_status",
                "package_lane_visibility": [
                    {
                        "official_package_name": FORMS_OFFICIAL_NAME,
                        "package_id": FORMS_PACKAGE_ID,
                        "upstream_package": "react-hook-form",
                        "upstream_version": "7.75.0",
                        "source_mirror": "G:/WWW/inspirations/react-hook-form",
                        "status": "present",
                        "receipt_status": "present",
                        "package_receipt_path": FORMS_DASHBOARD_RECEIPT,
                        "receipt_hash_refresh": {
                            "schema": "dx.forge.package.receipt_hash_refresh",
                            "status": "current",
                            "helper_path": "examples/template/forms-receipt-hashes.ts",
                            "check_command": "node examples/template/forms-receipt-hashes.ts --check",
                            "write_command": "node examples/template/forms-receipt-hashes.ts --write",
                            "json_check_command": "node examples/template/forms-receipt-hashes.ts --check --json",
                            "source_guard_runbook_fixture": "docs/packages/forms.source-guard-runbook.json",
                            "receipt_path": FORMS_DASHBOARD_RECEIPT,
                            "hash_algorithm": "sha256",
                            "tracked_file_count": 6,
                            "tracked_files": [
                                "examples/template/template-lead-form.tsx",
                                "docs/packages/forms-react-hook-form.md",
                                "dx-www/src/cli/studio_manifest.rs"
                            ],
                            "current_files": [
                                "examples/template/template-lead-form.tsx",
                                "docs/packages/forms-react-hook-form.md",
                                "dx-www/src/cli/studio_manifest.rs"
                            ],
                            "stale_files": [],
                            "missing_files": [],
                            "stale_mirror_files": [],
                            "missing_mirror_files": [],
                            "stale_file_count": 0,
                            "missing_file_count": 0,
                            "runtime_execution": false,
                            "secret_access": false,
                            "zed_visibility": "forms:receipt-hash-refresh"
                        },
                        "selected_surfaces": [],
                        "blocked_surfaces": [],
                        "unsupported_surfaces": []
                    }
                ]
            }))
            .expect("package status json"),
        )
        .expect("write package status");

        let manifest = DxSourceManifest::default();
        let (current_metrics, current_findings) =
            forge_forms_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(&current_metrics, "forms_receipt_hash_refresh_current"),
            Some(1)
        );
        assert_eq!(
            metric_value(&current_metrics, "forms_receipt_hash_refresh_stale"),
            Some(0)
        );
        assert_eq!(
            metric_value(&current_metrics, "forms_receipt_hash_refresh_missing"),
            Some(0)
        );
        assert_eq!(
            metric_value(&current_metrics, "forms_receipt_stale"),
            Some(0)
        );
        assert!(
            !current_findings
                .iter()
                .any(|finding| finding.code == "forms-stale-receipt")
        );

        let mut package_status: serde_json::Value =
            serde_json::from_slice(&fs::read(&package_status_path).expect("package status bytes"))
                .expect("package status json");
        package_status["package_lane_visibility"][0]["receipt_hash_refresh"]["stale_files"] =
            serde_json::json!(["docs/packages/forms-react-hook-form.md"]);
        package_status["package_lane_visibility"][0]["receipt_hash_refresh"]["stale_mirror_files"] =
            serde_json::json!(["examples/template/forge-package-status-read-model.ts"]);
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&package_status).expect("stale package status json"),
        )
        .expect("write stale package status");

        let (stale_metrics, stale_findings) = forge_forms_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(&stale_metrics, "forms_receipt_hash_refresh_current"),
            Some(0)
        );
        assert_eq!(
            metric_value(&stale_metrics, "forms_receipt_hash_refresh_stale"),
            Some(1)
        );
        assert_eq!(
            metric_value(&stale_metrics, "forms_receipt_hash_refresh_missing"),
            Some(0)
        );
        assert_eq!(metric_value(&stale_metrics, "forms_hash_mismatch"), Some(0));
        assert_eq!(metric_value(&stale_metrics, "forms_receipt_stale"), Some(1));
        assert!(
            stale_findings
                .iter()
                .any(|finding| finding.code == "forms-stale-receipt")
        );

        package_status["package_lane_visibility"][0]["receipt_hash_refresh"]["stale_files"] =
            serde_json::json!([]);
        package_status["package_lane_visibility"][0]["receipt_hash_refresh"]["stale_mirror_files"] =
            serde_json::json!([]);
        package_status["package_lane_visibility"][0]["receipt_hash_refresh"]["missing_files"] =
            serde_json::json!(["docs/packages/forms.source-guard-runbook.json"]);
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&package_status).expect("missing package status json"),
        )
        .expect("write missing package status");

        let (missing_metrics, _) = forge_forms_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(&missing_metrics, "forms_receipt_hash_refresh_current"),
            Some(0)
        );
        assert_eq!(
            metric_value(&missing_metrics, "forms_receipt_hash_refresh_stale"),
            Some(0)
        );
        assert_eq!(
            metric_value(&missing_metrics, "forms_receipt_hash_refresh_missing"),
            Some(1)
        );
    }
}
