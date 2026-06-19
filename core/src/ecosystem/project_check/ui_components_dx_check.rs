#![allow(clippy::too_many_arguments)]
use std::path::Path;

use super::super::forge_security::{DxSourceManifest, DxSupplyChainSeverity};
use super::file_hashes::count_sha256_file_hash_mismatches;
use super::{
    DxCheckFinding, DxCheckMetric, check_finding, check_metric, json_array_entries, json_text,
    read_optional_forge_json, resolve_dx_check_relative_path,
};

const UI_COMPONENTS_PACKAGE_ID: &str = "shadcn/ui/button";
const UI_COMPONENTS_OFFICIAL_NAME: &str = "UI Components";
const UI_COMPONENTS_PACKAGE_STATUS: &str = ".dx/forge/package-status.json";
const UI_COMPONENTS_DASHBOARD_RECEIPT: &str =
    "examples/template/.dx/forge/receipts/2026-05-22-shadcn-dashboard-controls.json";

pub(super) fn forge_ui_components_package_metrics(
    root: &Path,
    manifest: &DxSourceManifest,
) -> (Vec<DxCheckMetric>, Vec<DxCheckFinding>) {
    let mut findings = Vec::new();
    let manifest_package_present = manifest
        .packages
        .iter()
        .any(|package| package.package_id == UI_COMPONENTS_PACKAGE_ID);
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

    let Some(package_status) = read_optional_forge_json(
        root,
        UI_COMPONENTS_PACKAGE_STATUS,
        "ui-components-package-status-invalid",
        &mut findings,
    ) else {
        if manifest_package_present || package_receipt_exists(root, UI_COMPONENTS_DASHBOARD_RECEIPT)
        {
            package_present = 1;
            missing_receipt = 1;
            receipt_hash_refresh_missing = 1;
            findings.push(missing_package_status_finding());
        }
        return (
            ui_components_metrics(
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
            ),
            findings,
        );
    };

    let Some(visibility) = json_array_entries(&package_status, &["package_lane_visibility"])
        .into_iter()
        .find(|entry| json_text(entry, &["package_id"]) == Some(UI_COMPONENTS_PACKAGE_ID))
    else {
        if manifest_package_present || package_receipt_exists(root, UI_COMPONENTS_DASHBOARD_RECEIPT)
        {
            package_present = 1;
            missing_receipt = 1;
            receipt_hash_refresh_missing = 1;
            findings.push(missing_package_status_finding());
        }
        return (
            ui_components_metrics(
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

    let package_receipt_path =
        json_text(visibility, &["package_receipt_path"]).unwrap_or(UI_COMPONENTS_DASHBOARD_RECEIPT);
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

    if hash_mismatches > 0 || receipt_hash_refresh_stale > 0 || receipt_hash_refresh_missing > 0 {
        stale_receipt = 1;
    }

    if stale_receipt > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "ui-components-stale-receipt",
            format!("{UI_COMPONENTS_OFFICIAL_NAME} package-status visibility is stale"),
            Some(UI_COMPONENTS_PACKAGE_STATUS.to_string()),
            "Regenerate the UI Components package-status row from the shadcn-ui and Radix dashboard controls receipt before claiming SOURCE-ONLY source freshness.",
        ));
    }
    if blocked_surfaces > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "ui-components-blocked-surface",
            format!(
                "{UI_COMPONENTS_OFFICIAL_NAME} has {blocked_surfaces} blocked selected surface(s)"
            ),
            Some(UI_COMPONENTS_PACKAGE_STATUS.to_string()),
            "Resolve the SOURCE-ONLY UI Components app-owned dashboard persistence, accessibility, and governed browser UI runtime proof boundary before claiming runtime readiness.",
        ));
    }
    if unsupported_surfaces > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "ui-components-unsupported-surface",
            format!(
                "{UI_COMPONENTS_OFFICIAL_NAME} has {unsupported_surfaces} unsupported requested surface(s)"
            ),
            Some(UI_COMPONENTS_PACKAGE_STATUS.to_string()),
            "Request only supported UI Components alert, avatar, badge, button, card, field, input, item, label, separator, skeleton, textarea, or dashboard-control surfaces, or add a real upstream-backed Forge surface first.",
        ));
    }
    if hash_mismatches > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "ui-components-hash-mismatch",
            format!(
                "{UI_COMPONENTS_OFFICIAL_NAME} has {hash_mismatches} missing or stale hash-backed source file(s)"
            ),
            Some(UI_COMPONENTS_PACKAGE_STATUS.to_string()),
            "Regenerate the UI Components dashboard controls receipt after reviewing the changed front-facing shadcn-ui and Radix-derived files.",
        ));
    }

    (
        ui_components_metrics(
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
        ),
        findings,
    )
}

fn ui_components_metrics(
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
) -> Vec<DxCheckMetric> {
    vec![
        check_metric("ui_components_package_present", package_present),
        check_metric("ui_components_receipt_present", receipt_present),
        check_metric("ui_components_receipt_stale", stale_receipt),
        check_metric("ui_components_missing_receipt", missing_receipt),
        check_metric("ui_components_blocked_surface", blocked_surfaces),
        check_metric("ui_components_unsupported_surface", unsupported_surfaces),
        check_metric("ui_components_hash_manifest_present", hash_manifest_present),
        check_metric("ui_components_hash_mismatch", hash_mismatches),
        check_metric(
            "ui_components_receipt_hash_refresh_current",
            receipt_hash_refresh_current,
        ),
        check_metric(
            "ui_components_receipt_hash_refresh_stale",
            receipt_hash_refresh_stale,
        ),
        check_metric(
            "ui_components_receipt_hash_refresh_missing",
            receipt_hash_refresh_missing,
        ),
    ]
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
        "ui-components-missing-package-status",
        format!("{UI_COMPONENTS_OFFICIAL_NAME} is missing from package-status visibility"),
        Some(UI_COMPONENTS_PACKAGE_STATUS.to_string()),
        "Regenerate .dx/forge/package-status.json so dx-check can report UI Components visibility states.",
    )
}

fn missing_receipt_finding(receipt_path: &str) -> DxCheckFinding {
    check_finding(
        DxSupplyChainSeverity::Medium,
        "ui-components-missing-receipt",
        format!("{UI_COMPONENTS_OFFICIAL_NAME} is missing its package-lane receipt"),
        Some(receipt_path.to_string()),
        "Regenerate or restore the UI Components dashboard controls receipt so dx-check can report source-owned shadcn-ui and Radix visibility without claiming browser UI runtime proof.",
    )
}

fn package_receipt_exists(root: &Path, receipt_path: &str) -> bool {
    if resolve_dx_check_relative_path(root, receipt_path).is_some_and(|path| path.is_file()) {
        return true;
    }

    if let Some(template_relative) = receipt_path.strip_prefix("examples/template/") {
        return resolve_dx_check_relative_path(root, template_relative)
            .is_some_and(|path| path.is_file());
    }

    let template_prefixed = format!("examples/template/{receipt_path}");
    resolve_dx_check_relative_path(root, &template_prefixed).is_some_and(|path| path.is_file())
}
