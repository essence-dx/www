#![allow(clippy::too_many_arguments)]
use std::path::Path;

use super::super::forge_security::{DxSourceManifest, DxSupplyChainSeverity};
use super::file_hashes::count_sha256_file_hash_mismatches;
use super::{
    DxCheckFinding, DxCheckMetric, check_finding, check_metric, json_array_entries, json_text,
    read_optional_forge_json, resolve_dx_check_relative_path,
};

const AUTHENTICATION_PACKAGE_ID: &str = "auth/better-auth";
const AUTHENTICATION_OFFICIAL_NAME: &str = "Authentication";
const AUTHENTICATION_PACKAGE_STATUS: &str = ".dx/forge/package-status.json";
const AUTHENTICATION_PACKAGE_RECEIPT: &str = ".dx/forge/receipts/auth-better-auth.json";

pub(super) fn forge_authentication_package_metrics(
    root: &Path,
    manifest: &DxSourceManifest,
) -> (Vec<DxCheckMetric>, Vec<DxCheckFinding>) {
    let mut findings = Vec::new();
    let manifest_package_present = manifest
        .packages
        .iter()
        .any(|package| package.package_id == AUTHENTICATION_PACKAGE_ID);
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
    let mut dx_style_compatibility_present = 0u64;
    let mut dx_style_compatibility_missing = 0u64;

    let Some(package_status) = read_optional_forge_json(
        root,
        AUTHENTICATION_PACKAGE_STATUS,
        "authentication-package-status-invalid",
        &mut findings,
    ) else {
        if manifest_package_present || package_receipt_exists(root, AUTHENTICATION_PACKAGE_RECEIPT)
        {
            package_present = 1;
            missing_receipt = 1;
            receipt_hash_refresh_missing = 1;
            findings.push(missing_package_status_finding());
        }
        return (
            authentication_metrics(
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
                dx_style_compatibility_present,
                dx_style_compatibility_missing,
            ),
            findings,
        );
    };

    let Some(visibility) = json_array_entries(&package_status, &["package_lane_visibility"])
        .into_iter()
        .find(|entry| json_text(entry, &["package_id"]) == Some(AUTHENTICATION_PACKAGE_ID))
    else {
        if manifest_package_present || package_receipt_exists(root, AUTHENTICATION_PACKAGE_RECEIPT)
        {
            package_present = 1;
            missing_receipt = 1;
            receipt_hash_refresh_missing = 1;
            findings.push(missing_package_status_finding());
        }
        return (
            authentication_metrics(
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
                dx_style_compatibility_present,
                dx_style_compatibility_missing,
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
        json_text(visibility, &["package_receipt_path"]).unwrap_or(AUTHENTICATION_PACKAGE_RECEIPT);
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

        surface_hash_mismatches += count_sha256_file_hash_mismatches(root, surface);

        match json_text(surface, &["status"]) {
            Some("stale") => stale_receipt = 1,
            Some("blocked") => blocked_surfaces += 1,
            Some("unsupported-surface") => unsupported_surfaces += 1,
            _ => {}
        }
    }

    if dx_style_compatibility_is_present(visibility) {
        dx_style_compatibility_present = 1;
    } else {
        dx_style_compatibility_missing = 1;
    }

    if surface_hash_mismatches > 0 {
        stale_receipt = 1;
    }

    if stale_receipt > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "authentication-stale-receipt",
            format!("{AUTHENTICATION_OFFICIAL_NAME} package-status visibility is stale"),
            Some(AUTHENTICATION_PACKAGE_STATUS.to_string()),
            "Regenerate the Authentication package-status row from the dashboard workflow receipt before claiming source freshness.",
        ));
    }
    if blocked_surfaces > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "authentication-blocked-surface",
            format!(
                "{AUTHENTICATION_OFFICIAL_NAME} has {blocked_surfaces} blocked selected surface(s)"
            ),
            Some(AUTHENTICATION_PACKAGE_STATUS.to_string()),
            "Resolve the ADAPTER-BOUNDARY Authentication runtime ownership before claiming the selected surface is release-ready.",
        ));
    }
    if unsupported_surfaces > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "authentication-unsupported-surface",
            format!(
                "{AUTHENTICATION_OFFICIAL_NAME} has {unsupported_surfaces} unsupported requested surface(s)"
            ),
            Some(AUTHENTICATION_PACKAGE_STATUS.to_string()),
            "Request only supported Authentication surfaces or add a real upstream-backed Forge surface first.",
        ));
    }
    if surface_hash_mismatches > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "authentication-hash-mismatch",
            format!(
                "{AUTHENTICATION_OFFICIAL_NAME} has {surface_hash_mismatches} missing or stale hash-backed selected file(s)"
            ),
            Some(AUTHENTICATION_PACKAGE_STATUS.to_string()),
            "Hash the selected Authentication files again after reviewing the changed front-facing Authentication surfaces.",
        ));
    }
    if dx_style_compatibility_missing > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "authentication-missing-dx-style-compatibility",
            format!("{AUTHENTICATION_OFFICIAL_NAME} is missing dx-style compatibility evidence"),
            Some(AUTHENTICATION_PACKAGE_STATUS.to_string()),
            "Regenerate the Authentication dx-style compatibility row and verify data-dx-style-surface markers without claiming live OAuth or session runtime proof.",
        ));
    }

    (
        authentication_metrics(
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
            dx_style_compatibility_present,
            dx_style_compatibility_missing,
        ),
        findings,
    )
}

fn authentication_metrics(
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
    dx_style_compatibility_present: u64,
    dx_style_compatibility_missing: u64,
) -> Vec<DxCheckMetric> {
    vec![
        check_metric("authentication_package_present", package_present),
        check_metric("authentication_receipt_present", receipt_present),
        check_metric("authentication_receipt_stale", stale_receipt),
        check_metric("authentication_missing_receipt", missing_receipt),
        check_metric("authentication_blocked_surface", blocked_surfaces),
        check_metric("authentication_unsupported_surface", unsupported_surfaces),
        check_metric(
            "authentication_hash_manifest_present",
            hash_manifest_present,
        ),
        check_metric("authentication_hash_mismatch", surface_hash_mismatches),
        check_metric(
            "authentication_receipt_hash_refresh_current",
            receipt_hash_refresh_current,
        ),
        check_metric(
            "authentication_receipt_hash_refresh_stale",
            receipt_hash_refresh_stale,
        ),
        check_metric(
            "authentication_receipt_hash_refresh_missing",
            receipt_hash_refresh_missing,
        ),
        check_metric(
            "authentication_dx_style_compatibility_present",
            dx_style_compatibility_present,
        ),
        check_metric(
            "authentication_dx_style_compatibility_missing",
            dx_style_compatibility_missing,
        ),
    ]
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

    let stale_count = json_u64(refresh, "stale_file_count");
    let missing_count = json_u64(refresh, "missing_file_count");
    let stale_paths = json_string_array(refresh, "stale_files").len() as u64
        + json_string_array(refresh, "stale_mirror_files").len() as u64;
    let missing_paths = json_string_array(refresh, "missing_files").len() as u64
        + json_string_array(refresh, "missing_mirror_files").len() as u64;
    let status = json_text(refresh, &["status"]).unwrap_or("missing");
    let stale = u64::from(status == "stale" || stale_count > 0 || stale_paths > 0);
    let missing = u64::from(status == "missing" || missing_count > 0 || missing_paths > 0);
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
        "authentication-missing-package-status",
        format!("{AUTHENTICATION_OFFICIAL_NAME} is missing from package-status visibility"),
        Some(AUTHENTICATION_PACKAGE_STATUS.to_string()),
        "Regenerate .dx/forge/package-status.json so dx-check can report Authentication visibility states.",
    )
}

fn missing_receipt_finding(receipt_path: &str) -> DxCheckFinding {
    check_finding(
        DxSupplyChainSeverity::Medium,
        "authentication-missing-receipt",
        format!("{AUTHENTICATION_OFFICIAL_NAME} is missing its package-lane receipt"),
        Some(receipt_path.to_string()),
        "Regenerate or restore the Authentication dashboard workflow receipt so dx-check can report source-owned package visibility.",
    )
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
    use std::{fs, path::Path};

    use super::*;

    #[test]
    fn authentication_package_metrics_reports_missing_dx_style_compatibility() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_authentication_receipt(dir.path());
        write_authentication_package_status(dir.path(), false);

        let manifest = DxSourceManifest::default();
        let (missing_metrics, missing_findings) =
            forge_authentication_package_metrics(dir.path(), &manifest);

        assert_eq!(
            metric_value(&missing_metrics, "authentication_package_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(&missing_metrics, "authentication_receipt_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(&missing_metrics, "authentication_missing_receipt"),
            Some(0)
        );
        assert_eq!(
            metric_value(
                &missing_metrics,
                "authentication_dx_style_compatibility_present"
            ),
            Some(0)
        );
        assert_eq!(
            metric_value(
                &missing_metrics,
                "authentication_dx_style_compatibility_missing"
            ),
            Some(1)
        );
        assert!(
            missing_findings
                .iter()
                .any(|finding| finding.code == "authentication-missing-dx-style-compatibility")
        );

        write_authentication_package_status(dir.path(), true);

        let (present_metrics, present_findings) =
            forge_authentication_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(
                &present_metrics,
                "authentication_dx_style_compatibility_present"
            ),
            Some(1)
        );
        assert_eq!(
            metric_value(
                &present_metrics,
                "authentication_dx_style_compatibility_missing"
            ),
            Some(0)
        );
        assert!(
            !present_findings
                .iter()
                .any(|finding| finding.code == "authentication-missing-dx-style-compatibility")
        );
    }

    #[test]
    fn authentication_package_metrics_reports_helper_freshness_from_path_arrays() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_authentication_receipt(dir.path());
        write_authentication_package_status(dir.path(), true);

        let manifest = DxSourceManifest::default();
        let (current_metrics, current_findings) =
            forge_authentication_package_metrics(dir.path(), &manifest);

        assert_eq!(
            metric_value(
                &current_metrics,
                "authentication_receipt_hash_refresh_current"
            ),
            Some(1)
        );
        assert_eq!(
            metric_value(
                &current_metrics,
                "authentication_receipt_hash_refresh_stale"
            ),
            Some(0)
        );
        assert_eq!(
            metric_value(
                &current_metrics,
                "authentication_receipt_hash_refresh_missing"
            ),
            Some(0)
        );
        assert!(
            !current_findings
                .iter()
                .any(|finding| finding.code == "authentication-stale-receipt")
        );

        let package_status_path = dir.path().join(AUTHENTICATION_PACKAGE_STATUS);
        let mut package_status: serde_json::Value =
            serde_json::from_slice(&fs::read(&package_status_path).expect("package status"))
                .expect("package status json");
        package_status["package_lane_visibility"][0]["receipt_hash_refresh"]["current_files"] =
            serde_json::json!([
                "examples/template/template-shell.tsx",
                "examples/template/auth-session-status.tsx"
            ]);
        package_status["package_lane_visibility"][0]["receipt_hash_refresh"]["stale_files"] =
            serde_json::json!(["tools/launch/materialize-www-template.ts"]);
        package_status["package_lane_visibility"][0]["receipt_hash_refresh"]["stale_mirror_files"] =
            serde_json::json!(["examples/template/forge-package-status-read-model.ts"]);
        package_status["package_lane_visibility"][0]["receipt_hash_refresh"]["missing_files"] =
            serde_json::json!([]);
        package_status["package_lane_visibility"][0]["receipt_hash_refresh"]["missing_mirror_files"] =
            serde_json::json!([]);
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&package_status).expect("stale package status bytes"),
        )
        .expect("write stale package status");

        let (stale_metrics, stale_findings) =
            forge_authentication_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(
                &stale_metrics,
                "authentication_receipt_hash_refresh_current"
            ),
            Some(0)
        );
        assert_eq!(
            metric_value(&stale_metrics, "authentication_receipt_hash_refresh_stale"),
            Some(1)
        );
        assert_eq!(
            metric_value(
                &stale_metrics,
                "authentication_receipt_hash_refresh_missing"
            ),
            Some(0)
        );
        assert_eq!(
            metric_value(&stale_metrics, "authentication_hash_mismatch"),
            Some(0)
        );
        assert!(
            stale_findings
                .iter()
                .any(|finding| finding.code == "authentication-stale-receipt")
        );

        package_status["package_lane_visibility"][0]["receipt_hash_refresh"]["stale_files"] =
            serde_json::json!([]);
        package_status["package_lane_visibility"][0]["receipt_hash_refresh"]["stale_mirror_files"] =
            serde_json::json!([]);
        package_status["package_lane_visibility"][0]["receipt_hash_refresh"]["missing_files"] =
            serde_json::json!(["docs/packages/authentication.source-guard-runbook.json"]);
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&package_status).expect("missing package status bytes"),
        )
        .expect("write missing package status");

        let (missing_metrics, _) = forge_authentication_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(
                &missing_metrics,
                "authentication_receipt_hash_refresh_current"
            ),
            Some(0)
        );
        assert_eq!(
            metric_value(
                &missing_metrics,
                "authentication_receipt_hash_refresh_stale"
            ),
            Some(0)
        );
        assert_eq!(
            metric_value(
                &missing_metrics,
                "authentication_receipt_hash_refresh_missing"
            ),
            Some(1)
        );
    }

    fn write_authentication_receipt(root: &Path) {
        let receipt_path = root.join(AUTHENTICATION_PACKAGE_RECEIPT);
        fs::create_dir_all(receipt_path.parent().expect("receipt parent"))
            .expect("receipt directory");
        fs::write(
            receipt_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.forge.receipt",
                "official_package_name": AUTHENTICATION_OFFICIAL_NAME,
                "package_id": AUTHENTICATION_PACKAGE_ID,
                "surface": "authentication-session-status"
            }))
            .expect("receipt json"),
        )
        .expect("write receipt");
    }

    fn write_authentication_package_status(root: &Path, include_dx_style: bool) {
        let package_status_path = root.join(AUTHENTICATION_PACKAGE_STATUS);
        fs::create_dir_all(package_status_path.parent().expect("package status parent"))
            .expect("package status directory");

        let mut visibility = serde_json::json!({
            "official_package_name": AUTHENTICATION_OFFICIAL_NAME,
            "package_id": AUTHENTICATION_PACKAGE_ID,
            "upstream_package": "better-auth",
            "upstream_version": "1.6.11",
            "source_mirror": "G:/WWW/inspirations/better-auth",
            "status": "present",
            "receipt_status": "present",
            "package_receipt_path": AUTHENTICATION_PACKAGE_RECEIPT,
            "receipt_hash_refresh": {
                "schema": "dx.forge.package.receipt_hash_refresh",
                "status": "current",
                "helper_path": "examples/template/authentication-receipt-hashes.ts",
                "check_command": "node examples/template/authentication-receipt-hashes.ts --check",
                "write_command": "node examples/template/authentication-receipt-hashes.ts --write",
                "json_check_command": "node examples/template/authentication-receipt-hashes.ts --check --json",
                "receipt_path": AUTHENTICATION_PACKAGE_RECEIPT,
                "hash_algorithm": "sha256",
                "tracked_file_count": 6,
                "tracked_files": [
                    "examples/template/template-shell.tsx",
                    "examples/template/auth-session-status.tsx",
                    "examples/dashboard/src/lib/forge/auth/better-auth/dashboard.ts",
                    "examples/dashboard/src/components/BetterAuthAccountWorkflow.tsx",
                    "docs/packages/authentication.source-guard-runbook.json",
                    "tools/launch/materialize-www-template.ts"
                ],
                "current_files": [
                    "examples/template/template-shell.tsx",
                    "examples/template/auth-session-status.tsx",
                    "examples/dashboard/src/lib/forge/auth/better-auth/dashboard.ts",
                    "examples/dashboard/src/components/BetterAuthAccountWorkflow.tsx",
                    "docs/packages/authentication.source-guard-runbook.json",
                    "tools/launch/materialize-www-template.ts"
                ],
                "stale_files": [],
                "missing_files": [],
                "stale_mirror_files": [],
                "missing_mirror_files": [],
                "mirror_problem_count": 0,
                "stale_file_count": 0,
                "missing_file_count": 0,
                "runtime_execution": false,
                "secret_access": false,
                "zed_visibility": "authentication:receipt-hash-refresh",
                "runtime_limitations": [
                    "SOURCE-ONLY: this helper checks local Authentication receipt hash freshness only.",
                    "ADAPTER-BOUNDARY: live OAuth, cookies, credentials, and hosted session proof stay app-owned."
                ]
            },
            "selected_surfaces": [
                {
                    "surface_id": "authentication-session-status",
                    "status": "present",
                    "receipt_path": AUTHENTICATION_PACKAGE_RECEIPT,
                    "files": ["components/launch/auth-session-status.tsx"],
                    "source_markers": [
                        "data-dx-component=\"better-auth-session-status-panel\"",
                        "data-dx-style-surface=\"authentication-session-status\""
                    ]
                }
            ],
            "blocked_surfaces": [],
            "unsupported_surfaces": []
        });

        if include_dx_style {
            visibility["dx_style_compatibility"] = serde_json::json!({
                "schema": "dx.forge.package.dx_style_compatibility",
                "status": "present",
                "token_source": "styles/theme.css",
                "generated_css": "styles/generated.css",
                "visible_surfaces": ["authentication-session-status"],
                "source_files": ["components/launch/auth-session-status.tsx"],
                "data_dx_markers": [
                    "data-dx-style-surface=\"authentication-session-status\""
                ],
                "receipt_path": AUTHENTICATION_PACKAGE_RECEIPT,
                "runtime_proof": false,
                "runtime_limitations": [
                    "SOURCE-ONLY: no live OAuth/session browser proof is claimed."
                ]
            });
        }

        fs::write(
            package_status_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.www.template.forge_package_status",
                "package_lane_visibility": [visibility]
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
