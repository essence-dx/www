#![allow(clippy::too_many_arguments)]
use std::{fs, path::Path};

use super::super::forge_registry::source_package_for_project_variant;
use super::super::forge_security::{DxSourceManifest, DxSupplyChainSeverity};
use super::{
    DxCheckFinding, DxCheckMetric, SOURCE_MANIFEST_PATH, check_finding, check_metric,
    json_array_entries, json_text, resolve_dx_check_relative_path,
};

const STATE_MANAGEMENT_PACKAGE_ID: &str = "state/zustand";
const STATE_MANAGEMENT_OFFICIAL_NAME: &str = "State Management";
const STATE_MANAGEMENT_PACKAGE_RECEIPT: &str = ".dx/forge/receipts/packages/state-zustand.json";

pub(super) fn forge_state_management_package_metrics(
    root: &Path,
    manifest: &DxSourceManifest,
) -> (Vec<DxCheckMetric>, Vec<DxCheckFinding>) {
    let mut findings = Vec::new();
    let mut package_present = 0u64;
    let mut receipt_present = 0u64;
    let mut stale_receipt = 0u64;
    let mut missing_receipt = 0u64;
    let mut blocked_surfaces = 0u64;
    let mut unsupported_surfaces = 0u64;
    let mut dx_style_compatibility_present = 0u64;
    let mut dx_style_compatibility_missing = 0u64;

    let Some(package) = manifest
        .packages
        .iter()
        .find(|package| package.package_id == STATE_MANAGEMENT_PACKAGE_ID)
    else {
        return (
            state_management_metrics(
                package_present,
                receipt_present,
                stale_receipt,
                missing_receipt,
                blocked_surfaces,
                unsupported_surfaces,
                dx_style_compatibility_present,
                dx_style_compatibility_missing,
            ),
            findings,
        );
    };

    package_present = 1;

    if source_package_for_project_variant(&package.package_id, root, &package.variant)
        .map(|latest| {
            latest.version != package.version || latest.integrity_hash != package.integrity_hash
        })
        .unwrap_or(false)
    {
        stale_receipt = 1;
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "state-management-stale-receipt",
            format!(
                "{STATE_MANAGEMENT_OFFICIAL_NAME} variant `{}` is behind the current curated source",
                package.variant
            ),
            Some(SOURCE_MANIFEST_PATH.to_string()),
            "Run dx update state/zustand, review the receipt, and keep the State Management package contract current.",
        ));
    }

    let Some(receipt_path) = resolve_dx_check_relative_path(root, STATE_MANAGEMENT_PACKAGE_RECEIPT)
    else {
        missing_receipt = 1;
        findings.push(missing_receipt_finding());
        return (
            state_management_metrics(
                package_present,
                receipt_present,
                stale_receipt,
                missing_receipt,
                blocked_surfaces,
                unsupported_surfaces,
                dx_style_compatibility_present,
                dx_style_compatibility_missing,
            ),
            findings,
        );
    };

    if !receipt_path.is_file() {
        missing_receipt = 1;
        findings.push(missing_receipt_finding());
        return (
            state_management_metrics(
                package_present,
                receipt_present,
                stale_receipt,
                missing_receipt,
                blocked_surfaces,
                unsupported_surfaces,
                dx_style_compatibility_present,
                dx_style_compatibility_missing,
            ),
            findings,
        );
    }

    let receipt = match fs::read(&receipt_path)
        .ok()
        .and_then(|bytes| serde_json::from_slice::<serde_json::Value>(&bytes).ok())
    {
        Some(receipt) => receipt,
        None => {
            missing_receipt = 1;
            findings.push(check_finding(
                DxSupplyChainSeverity::Medium,
                "state-management-receipt-invalid",
                format!("{STATE_MANAGEMENT_OFFICIAL_NAME} package receipt is not valid JSON"),
                Some(STATE_MANAGEMENT_PACKAGE_RECEIPT.to_string()),
                "Regenerate the State Management package receipt from Forge instead of editing it by hand.",
            ));
            return (
                state_management_metrics(
                    package_present,
                    receipt_present,
                    stale_receipt,
                    missing_receipt,
                    blocked_surfaces,
                    unsupported_surfaces,
                    dx_style_compatibility_present,
                    dx_style_compatibility_missing,
                ),
                findings,
            );
        }
    };

    receipt_present = 1;
    let visibility = receipt
        .get("package")
        .and_then(|package| package.get("dx_check_visibility"))
        .unwrap_or(&receipt);

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
        match json_text(surface, &["status"]) {
            Some("blocked") => blocked_surfaces += 1,
            Some("unsupported-surface") => unsupported_surfaces += 1,
            _ => {}
        }
    }

    if dx_style_compatibility_is_present(visibility) {
        dx_style_compatibility_present = 1;
    } else {
        dx_style_compatibility_missing = 1;
        findings.push(missing_dx_style_compatibility_finding());
    }

    if blocked_surfaces > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "state-management-blocked-surface",
            format!(
                "{STATE_MANAGEMENT_OFFICIAL_NAME} has {blocked_surfaces} blocked selected surface(s)"
            ),
            Some(STATE_MANAGEMENT_PACKAGE_RECEIPT.to_string()),
            "Resolve the app-owned State Management boundary before claiming the surface is release-ready.",
        ));
    }
    if unsupported_surfaces > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "state-management-unsupported-surface",
            format!(
                "{STATE_MANAGEMENT_OFFICIAL_NAME} has {unsupported_surfaces} unsupported requested surface(s)"
            ),
            Some(STATE_MANAGEMENT_PACKAGE_RECEIPT.to_string()),
            "Request only supported State Management surfaces or add a real upstream-backed Forge surface first.",
        ));
    }

    (
        state_management_metrics(
            package_present,
            receipt_present,
            stale_receipt,
            missing_receipt,
            blocked_surfaces,
            unsupported_surfaces,
            dx_style_compatibility_present,
            dx_style_compatibility_missing,
        ),
        findings,
    )
}

fn state_management_metrics(
    package_present: u64,
    receipt_present: u64,
    stale_receipt: u64,
    missing_receipt: u64,
    blocked_surfaces: u64,
    unsupported_surfaces: u64,
    dx_style_compatibility_present: u64,
    dx_style_compatibility_missing: u64,
) -> Vec<DxCheckMetric> {
    vec![
        check_metric("state_management_package_present", package_present),
        check_metric("state_management_receipt_present", receipt_present),
        check_metric("state_management_receipt_stale", stale_receipt),
        check_metric("state_management_missing_receipt", missing_receipt),
        check_metric("state_management_blocked_surface", blocked_surfaces),
        check_metric("state_management_unsupported_surface", unsupported_surfaces),
        check_metric(
            "state_management_dx_style_compatibility_present",
            dx_style_compatibility_present,
        ),
        check_metric(
            "state_management_dx_style_compatibility_missing",
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

fn missing_receipt_finding() -> DxCheckFinding {
    check_finding(
        DxSupplyChainSeverity::Medium,
        "state-management-missing-receipt",
        format!("{STATE_MANAGEMENT_OFFICIAL_NAME} is missing its package-lane receipt"),
        Some(STATE_MANAGEMENT_PACKAGE_RECEIPT.to_string()),
        "Regenerate or restore .dx/forge/receipts/packages/state-zustand.json so dx-check can report State Management visibility.",
    )
}

fn missing_dx_style_compatibility_finding() -> DxCheckFinding {
    check_finding(
        DxSupplyChainSeverity::Low,
        "state-management-missing-dx-style-compatibility",
        format!("{STATE_MANAGEMENT_OFFICIAL_NAME} is missing dx-style compatibility metadata"),
        Some(STATE_MANAGEMENT_PACKAGE_RECEIPT.to_string()),
        "Add dx.forge.package.dx_style_compatibility to the State Management package receipt before claiming style-compatible visible UI surfaces.",
    )
}

#[cfg(test)]
mod tests {
    use super::super::super::forge_security::{DxSourceFile, DxSourceKind, DxSourcePackage};
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn state_management_dx_style_missing_metric_and_finding_flip() {
        let dir = tempdir().expect("tempdir");
        let manifest = state_management_manifest();

        write_state_management_receipt(dir.path(), true);
        let (present_metrics, present_findings) =
            forge_state_management_package_metrics(dir.path(), &manifest);

        assert_eq!(
            metric_value(
                &present_metrics,
                "state_management_dx_style_compatibility_present"
            ),
            Some(1)
        );
        assert_eq!(
            metric_value(
                &present_metrics,
                "state_management_dx_style_compatibility_missing"
            ),
            Some(0)
        );
        assert!(
            !present_findings
                .iter()
                .any(|finding| finding.code == "state-management-missing-dx-style-compatibility")
        );

        write_state_management_receipt(dir.path(), false);
        let (missing_metrics, missing_findings) =
            forge_state_management_package_metrics(dir.path(), &manifest);

        assert_eq!(
            metric_value(
                &missing_metrics,
                "state_management_dx_style_compatibility_present"
            ),
            Some(0)
        );
        assert_eq!(
            metric_value(
                &missing_metrics,
                "state_management_dx_style_compatibility_missing"
            ),
            Some(1)
        );
        assert!(
            missing_findings
                .iter()
                .any(|finding| finding.code == "state-management-missing-dx-style-compatibility")
        );
    }

    fn state_management_manifest() -> DxSourceManifest {
        DxSourceManifest {
            version: 1,
            receipts: vec![STATE_MANAGEMENT_PACKAGE_RECEIPT.to_string()],
            packages: vec![DxSourcePackage {
                package_id: STATE_MANAGEMENT_PACKAGE_ID.to_string(),
                upstream_name: "zustand".to_string(),
                version: "5.0.13".to_string(),
                generator: "dx-forge/test".to_string(),
                variant: "fixture-no-latest".to_string(),
                last_accepted_update: None,
                rollback_receipt: None,
                source_kind: DxSourceKind::CuratedRegistry,
                integrity_hash: "state-management-fixture-integrity".to_string(),
                license: "MIT".to_string(),
                provenance: Default::default(),
                advisory_review: Default::default(),
                license_review: Default::default(),
                files: vec![DxSourceFile {
                    path: "lib/forge/state/zustand/index.ts".to_string(),
                    logical_path: Some("state/zustand/index.ts".to_string()),
                    hash: "state-management-fixture-file-hash".to_string(),
                    bytes: 1,
                    content: None,
                }],
            }],
        }
    }

    fn write_state_management_receipt(root: &Path, include_dx_style: bool) {
        let receipt_path = root.join(STATE_MANAGEMENT_PACKAGE_RECEIPT);
        let receipt_dir = receipt_path.parent().expect("receipt parent");
        fs::create_dir_all(receipt_dir).expect("receipt dir");

        let mut visibility = serde_json::json!({
            "status": "present",
            "receipt_status": "present",
            "selected_surfaces": [
                { "surface_id": "launch-dashboard-state-workflow", "status": "present" },
                { "surface_id": "launch-dashboard-state-shell", "status": "present" }
            ],
            "blocked_surfaces": [],
            "unsupported_surfaces": []
        });

        if include_dx_style {
            visibility
                .as_object_mut()
                .expect("visibility object")
                .insert(
                    "dx_style_compatibility".to_string(),
                    serde_json::json!({
                        "schema": "dx.forge.package.dx_style_compatibility",
                        "status": "present",
                        "token_source": "styles/theme.css",
                        "generated_css": "styles/generated.css",
                        "visible_surfaces": [
                            "launch-dashboard-state-workflow",
                            "launch-dashboard-state-shell"
                        ],
                        "runtime_proof": false
                    }),
                );
        }

        let receipt = serde_json::json!({
            "package": {
                "package_id": STATE_MANAGEMENT_PACKAGE_ID,
                "official_package_name": STATE_MANAGEMENT_OFFICIAL_NAME,
                "dx_check_visibility": visibility
            }
        });
        fs::write(
            receipt_path,
            serde_json::to_vec_pretty(&receipt).expect("state receipt json"),
        )
        .expect("write state receipt");
    }

    fn metric_value(metrics: &[DxCheckMetric], name: &str) -> Option<u64> {
        metrics
            .iter()
            .find(|metric| metric.name == name)
            .map(|metric| metric.value)
    }
}
