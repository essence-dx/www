#![allow(clippy::too_many_arguments)]
use std::path::Path;

use super::super::forge_security::{DxSourceManifest, DxSupplyChainSeverity};
use super::file_hashes::count_sha256_file_hash_mismatches;
use super::{
    DxCheckFinding, DxCheckMetric, check_finding, check_metric, json_array_entries, json_text,
    read_optional_forge_json, resolve_dx_check_relative_path,
};

const REALTIME_APP_DATABASE_PACKAGE_ID: &str = "instantdb/react";
const REALTIME_APP_DATABASE_OFFICIAL_NAME: &str = "Realtime App Database";
const REALTIME_APP_DATABASE_PACKAGE_STATUS: &str = ".dx/forge/package-status.json";
const REALTIME_APP_DATABASE_DASHBOARD_RECEIPT: &str =
    ".dx/forge/receipts/2026-05-22-instantdb-realtime-dashboard.json";

pub(super) fn forge_realtime_app_database_package_metrics(
    root: &Path,
    manifest: &DxSourceManifest,
) -> (Vec<DxCheckMetric>, Vec<DxCheckFinding>) {
    let mut findings = Vec::new();
    let manifest_package_present = manifest
        .packages
        .iter()
        .any(|package| package.package_id == REALTIME_APP_DATABASE_PACKAGE_ID);
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
        REALTIME_APP_DATABASE_PACKAGE_STATUS,
        "realtime-app-database-package-status-invalid",
        &mut findings,
    ) else {
        if manifest_package_present
            || package_receipt_exists(root, REALTIME_APP_DATABASE_DASHBOARD_RECEIPT)
        {
            package_present = 1;
            missing_receipt = 1;
            findings.push(missing_package_status_finding());
        }
        return (
            realtime_app_database_metrics(
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
        .find(|entry| json_text(entry, &["package_id"]) == Some(REALTIME_APP_DATABASE_PACKAGE_ID))
    else {
        if manifest_package_present
            || package_receipt_exists(root, REALTIME_APP_DATABASE_DASHBOARD_RECEIPT)
        {
            package_present = 1;
            missing_receipt = 1;
            findings.push(missing_package_status_finding());
        }
        return (
            realtime_app_database_metrics(
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

    let package_receipt_path = json_text(visibility, &["package_receipt_path"])
        .unwrap_or(REALTIME_APP_DATABASE_DASHBOARD_RECEIPT);
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
    if matches!(visibility_status, Some("missing-receipt"))
        || matches!(receipt_status, Some("missing-receipt"))
    {
        missing_receipt = 1;
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
            Some("missing-receipt") => missing_receipt = 1,
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

    if hash_mismatches > 0 {
        stale_receipt = 1;
    }

    if stale_receipt > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "realtime-app-database-stale-receipt",
            format!("{REALTIME_APP_DATABASE_OFFICIAL_NAME} package-status visibility is stale"),
            Some(REALTIME_APP_DATABASE_PACKAGE_STATUS.to_string()),
            "Regenerate the Realtime App Database package-status row from the dashboard workflow receipt before claiming source freshness.",
        ));
    }
    if blocked_surfaces > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "realtime-app-database-blocked-surface",
            format!(
                "{REALTIME_APP_DATABASE_OFFICIAL_NAME} has {blocked_surfaces} blocked selected surface(s)"
            ),
            Some(REALTIME_APP_DATABASE_PACKAGE_STATUS.to_string()),
            "Resolve the app-owned Realtime App Database runtime boundary before claiming the selected surface is release-ready.",
        ));
    }
    if unsupported_surfaces > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "realtime-app-database-unsupported-surface",
            format!(
                "{REALTIME_APP_DATABASE_OFFICIAL_NAME} has {unsupported_surfaces} unsupported requested surface(s)"
            ),
            Some(REALTIME_APP_DATABASE_PACKAGE_STATUS.to_string()),
            "Request only supported Realtime App Database surfaces or add a real upstream-backed Forge surface first.",
        ));
    }
    if hash_mismatches > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "realtime-app-database-hash-mismatch",
            format!(
                "{REALTIME_APP_DATABASE_OFFICIAL_NAME} has {hash_mismatches} missing or stale hash-backed source file(s)"
            ),
            Some(REALTIME_APP_DATABASE_PACKAGE_STATUS.to_string()),
            "Regenerate the Realtime App Database receipt after reviewing changed front-facing realtime files.",
        ));
    }
    if dx_style_compatibility_missing > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "realtime-app-database-missing-dx-style-compatibility",
            format!(
                "{REALTIME_APP_DATABASE_OFFICIAL_NAME} is missing dx-style compatibility evidence"
            ),
            Some(REALTIME_APP_DATABASE_PACKAGE_STATUS.to_string()),
            "Regenerate the Realtime App Database dx-style compatibility row and verify data-dx-style-surface markers without claiming hosted Instant runtime proof.",
        ));
    }

    (
        realtime_app_database_metrics(
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

fn realtime_app_database_metrics(
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
        check_metric("realtime_app_database_package_present", package_present),
        check_metric("realtime_app_database_receipt_present", receipt_present),
        check_metric("realtime_app_database_receipt_stale", stale_receipt),
        check_metric("realtime_app_database_missing_receipt", missing_receipt),
        check_metric("realtime_app_database_blocked_surface", blocked_surfaces),
        check_metric(
            "realtime_app_database_unsupported_surface",
            unsupported_surfaces,
        ),
        check_metric(
            "realtime_app_database_hash_manifest_present",
            hash_manifest_present,
        ),
        check_metric("realtime_app_database_hash_mismatch", hash_mismatches),
        check_metric(
            "realtime_app_database_dx_style_compatibility_present",
            dx_style_compatibility_present,
        ),
        check_metric(
            "realtime_app_database_dx_style_compatibility_missing",
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

fn missing_package_status_finding() -> DxCheckFinding {
    check_finding(
        DxSupplyChainSeverity::Medium,
        "realtime-app-database-missing-package-status",
        format!("{REALTIME_APP_DATABASE_OFFICIAL_NAME} is missing from package-status visibility"),
        Some(REALTIME_APP_DATABASE_PACKAGE_STATUS.to_string()),
        "Regenerate .dx/forge/package-status.json so dx-check can report Realtime App Database visibility states.",
    )
}

fn missing_receipt_finding(receipt_path: &str) -> DxCheckFinding {
    check_finding(
        DxSupplyChainSeverity::Medium,
        "realtime-app-database-missing-receipt",
        format!("{REALTIME_APP_DATABASE_OFFICIAL_NAME} is missing its package-lane receipt"),
        Some(receipt_path.to_string()),
        "Regenerate or restore the Realtime App Database dashboard workflow receipt so dx-check can report source-owned package visibility.",
    )
}

fn package_receipt_exists(root: &Path, receipt_path: &str) -> bool {
    project_file_exists(root, receipt_path)
}

fn project_file_exists(root: &Path, relative_path: &str) -> bool {
    if resolve_dx_check_relative_path(root, relative_path).is_some_and(|path| path.is_file()) {
        return true;
    }

    relative_path
        .strip_prefix("examples/template/")
        .and_then(|template_relative| resolve_dx_check_relative_path(root, template_relative))
        .is_some_and(|path| path.is_file())
}
