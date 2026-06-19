#![allow(clippy::too_many_arguments)]
use std::path::Path;

use super::super::forge_security::{DxSourceManifest, DxSupplyChainSeverity};
use super::file_hashes::count_sha256_file_hash_mismatches;
use super::file_hashes::has_sha256_file_hashes;
use super::{
    DxCheckFinding, DxCheckMetric, check_finding, check_metric, json_array_entries, json_text,
    read_optional_forge_json, resolve_dx_check_relative_path,
};

const INTERNATIONALIZATION_PACKAGE_ID: &str = "i18n/next-intl";
const INTERNATIONALIZATION_OFFICIAL_NAME: &str = "Internationalization";
const INTERNATIONALIZATION_PACKAGE_STATUS: &str = ".dx/forge/package-status.json";
const INTERNATIONALIZATION_DASHBOARD_RECEIPT: &str =
    ".dx/forge/receipts/2026-05-22-i18n-next-intl-dashboard-locale.json";

pub(super) fn forge_internationalization_package_metrics(
    root: &Path,
    manifest: &DxSourceManifest,
) -> (Vec<DxCheckMetric>, Vec<DxCheckFinding>) {
    let mut findings = Vec::new();
    let manifest_package_present = manifest
        .packages
        .iter()
        .any(|package| package.package_id == INTERNATIONALIZATION_PACKAGE_ID);
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
        INTERNATIONALIZATION_PACKAGE_STATUS,
        "internationalization-package-status-invalid",
        &mut findings,
    ) else {
        if manifest_package_present
            || package_receipt_exists(root, INTERNATIONALIZATION_DASHBOARD_RECEIPT)
        {
            package_present = 1;
            missing_receipt = 1;
            findings.push(missing_package_status_finding());
        }
        return (
            internationalization_metrics(
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
        .find(|entry| json_text(entry, &["package_id"]) == Some(INTERNATIONALIZATION_PACKAGE_ID))
    else {
        if manifest_package_present
            || package_receipt_exists(root, INTERNATIONALIZATION_DASHBOARD_RECEIPT)
        {
            package_present = 1;
            missing_receipt = 1;
            findings.push(missing_package_status_finding());
        }
        return (
            internationalization_metrics(
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
        .unwrap_or(INTERNATIONALIZATION_DASHBOARD_RECEIPT);
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
        if has_sha256_file_hashes(surface) {
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

    if dx_style_compatibility_is_present(visibility) {
        dx_style_compatibility_present = 1;
    } else {
        dx_style_compatibility_missing = 1;
    }

    if stale_receipt > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "internationalization-stale-receipt",
            format!("{INTERNATIONALIZATION_OFFICIAL_NAME} package-status visibility is stale"),
            Some(INTERNATIONALIZATION_PACKAGE_STATUS.to_string()),
            "Regenerate the Internationalization package-status row from the dashboard locale workflow receipt before claiming source freshness.",
        ));
    }
    if blocked_surfaces > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "internationalization-blocked-surface",
            format!(
                "{INTERNATIONALIZATION_OFFICIAL_NAME} has {blocked_surfaces} blocked selected surface(s)"
            ),
            Some(INTERNATIONALIZATION_PACKAGE_STATUS.to_string()),
            "Resolve the app-owned Internationalization locale routing, translation QA, SEO, or runtime dependency boundary before claiming the selected surface is release-ready.",
        ));
    }
    if unsupported_surfaces > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "internationalization-unsupported-surface",
            format!(
                "{INTERNATIONALIZATION_OFFICIAL_NAME} has {unsupported_surfaces} unsupported requested surface(s)"
            ),
            Some(INTERNATIONALIZATION_PACKAGE_STATUS.to_string()),
            "Request only supported Internationalization dashboard locale and message-contract surfaces, or add a real upstream-backed Forge surface first.",
        ));
    }
    if hash_mismatches > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "internationalization-hash-mismatch",
            format!(
                "{INTERNATIONALIZATION_OFFICIAL_NAME} has {hash_mismatches} missing or stale hash-backed selected file(s)"
            ),
            Some(INTERNATIONALIZATION_PACKAGE_STATUS.to_string()),
            "Regenerate the Internationalization dashboard locale workflow receipt after reviewing the changed front-facing locale files.",
        ));
    }
    if dx_style_compatibility_missing > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "internationalization-missing-dx-style-compatibility",
            format!(
                "{INTERNATIONALIZATION_OFFICIAL_NAME} is missing dx-style compatibility evidence"
            ),
            Some(INTERNATIONALIZATION_PACKAGE_STATUS.to_string()),
            "Regenerate the Internationalization dx-style compatibility row and verify data-dx-style-surface markers without claiming live locale routing proof.",
        ));
    }

    (
        internationalization_metrics(
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

fn internationalization_metrics(
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
        check_metric("internationalization_package_present", package_present),
        check_metric("internationalization_receipt_present", receipt_present),
        check_metric("internationalization_receipt_stale", stale_receipt),
        check_metric("internationalization_missing_receipt", missing_receipt),
        check_metric("internationalization_blocked_surface", blocked_surfaces),
        check_metric(
            "internationalization_unsupported_surface",
            unsupported_surfaces,
        ),
        check_metric(
            "internationalization_hash_manifest_present",
            hash_manifest_present,
        ),
        check_metric("internationalization_hash_mismatch", hash_mismatches),
        check_metric(
            "internationalization_dx_style_compatibility_present",
            dx_style_compatibility_present,
        ),
        check_metric(
            "internationalization_dx_style_compatibility_missing",
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
        "internationalization-missing-package-status",
        format!("{INTERNATIONALIZATION_OFFICIAL_NAME} is missing from package-status visibility"),
        Some(INTERNATIONALIZATION_PACKAGE_STATUS.to_string()),
        "Regenerate .dx/forge/package-status.json so dx-check can report Internationalization visibility states.",
    )
}

fn missing_receipt_finding(receipt_path: &str) -> DxCheckFinding {
    check_finding(
        DxSupplyChainSeverity::Medium,
        "internationalization-missing-receipt",
        format!("{INTERNATIONALIZATION_OFFICIAL_NAME} is missing its package-lane receipt"),
        Some(receipt_path.to_string()),
        "Regenerate or restore the Internationalization dashboard locale workflow receipt so dx-check can report source-owned package visibility without claiming live locale routing proof.",
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

    use super::*;

    #[test]
    fn internationalization_hash_mismatch_metric_and_finding_are_byte_derived() {
        let dir = tempfile::tempdir().expect("tempdir");
        let generated_relative_path = "examples/template/next-intl-dashboard-locale-contract.ts";
        let file_path = dir.path().join("next-intl-dashboard-locale-contract.ts");
        fs::write(&file_path, b"LaunchDashboard").expect("write");
        fs::create_dir_all(dir.path().join(".dx/forge/receipts")).expect("receipt dir");
        fs::write(
            dir.path().join(INTERNATIONALIZATION_DASHBOARD_RECEIPT),
            "{}",
        )
        .expect("dashboard receipt");
        fs::write(
            dir.path().join(INTERNATIONALIZATION_PACKAGE_STATUS),
            serde_json::to_vec_pretty(&serde_json::json!({
                "package_lane_visibility": [
                    {
                        "official_package_name": INTERNATIONALIZATION_OFFICIAL_NAME,
                        "package_id": INTERNATIONALIZATION_PACKAGE_ID,
                        "upstream_package": "next-intl",
                        "upstream_version": "4.12.0",
                        "source_mirror": "G:/WWW/inspirations/next-intl",
                        "status": "present",
                        "receipt_status": "present",
                        "package_receipt_path": INTERNATIONALIZATION_DASHBOARD_RECEIPT,
                        "selected_surfaces": [
                            {
                                "surface_id": "next-intl-dashboard-message-contract",
                                "status": "present",
                                "hash_algorithm": "sha256",
                                "file_hashes": {
                                    generated_relative_path: "7ca079397a700aa148835d467e9583a19627c236b39fae93c9a5406ff27fe2a1"
                                }
                            }
                        ],
                        "dx_style_compatibility": {
                            "schema": "dx.forge.package.dx_style_compatibility",
                            "status": "present"
                        },
                        "blocked_surfaces": [],
                        "unsupported_surfaces": []
                    }
                ]
            }))
            .expect("package status json"),
        )
        .expect("package status");

        let manifest = DxSourceManifest::default();
        let (metrics, findings) = forge_internationalization_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(&metrics, "internationalization_hash_manifest_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(&metrics, "internationalization_hash_mismatch"),
            Some(0)
        );
        assert_eq!(
            metric_value(&metrics, "internationalization_receipt_stale"),
            Some(0)
        );
        assert!(
            !findings
                .iter()
                .any(|finding| finding.code == "internationalization-hash-mismatch")
        );

        fs::write(&file_path, b"LaunchDashboardStale").expect("mutate");
        let (stale_metrics, stale_findings) =
            forge_internationalization_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(&stale_metrics, "internationalization_hash_mismatch"),
            Some(1)
        );
        assert_eq!(
            metric_value(&stale_metrics, "internationalization_receipt_stale"),
            Some(1)
        );
        assert!(
            stale_findings
                .iter()
                .any(|finding| finding.code == "internationalization-hash-mismatch")
        );
    }

    #[test]
    fn internationalization_dx_style_compatibility_metric_tracks_package_status_row() {
        let dir = tempfile::tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join(".dx/forge/receipts")).expect("receipt dir");
        fs::write(
            dir.path().join(INTERNATIONALIZATION_DASHBOARD_RECEIPT),
            "{}",
        )
        .expect("dashboard receipt");

        write_package_status(
            dir.path(),
            Some(serde_json::json!({
                "schema": "dx.forge.package.dx_style_compatibility",
                "status": "present"
            })),
        );

        let manifest = DxSourceManifest::default();
        let (metrics, findings) = forge_internationalization_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(
                &metrics,
                "internationalization_dx_style_compatibility_present"
            ),
            Some(1)
        );
        assert_eq!(
            metric_value(
                &metrics,
                "internationalization_dx_style_compatibility_missing"
            ),
            Some(0)
        );
        assert!(
            !findings.iter().any(
                |finding| finding.code == "internationalization-missing-dx-style-compatibility"
            )
        );

        write_package_status(dir.path(), None);

        let (missing_metrics, missing_findings) =
            forge_internationalization_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(
                &missing_metrics,
                "internationalization_dx_style_compatibility_present"
            ),
            Some(0)
        );
        assert_eq!(
            metric_value(
                &missing_metrics,
                "internationalization_dx_style_compatibility_missing"
            ),
            Some(1)
        );
        assert!(
            missing_findings.iter().any(
                |finding| finding.code == "internationalization-missing-dx-style-compatibility"
            )
        );
    }

    fn metric_value(metrics: &[DxCheckMetric], name: &str) -> Option<u64> {
        metrics
            .iter()
            .find(|metric| metric.name == name)
            .map(|metric| metric.value)
    }

    fn write_package_status(root: &Path, dx_style_compatibility: Option<serde_json::Value>) {
        let mut visibility = serde_json::json!({
            "official_package_name": INTERNATIONALIZATION_OFFICIAL_NAME,
            "package_id": INTERNATIONALIZATION_PACKAGE_ID,
            "upstream_package": "next-intl",
            "upstream_version": "4.12.0",
            "source_mirror": "G:/WWW/inspirations/next-intl",
            "status": "present",
            "receipt_status": "present",
            "package_receipt_path": INTERNATIONALIZATION_DASHBOARD_RECEIPT,
            "selected_surfaces": [],
            "blocked_surfaces": [],
            "unsupported_surfaces": []
        });

        if let Some(dx_style) = dx_style_compatibility {
            visibility["dx_style_compatibility"] = dx_style;
        }

        fs::write(
            root.join(INTERNATIONALIZATION_PACKAGE_STATUS),
            serde_json::to_vec_pretty(&serde_json::json!({
                "package_lane_visibility": [visibility]
            }))
            .expect("package status json"),
        )
        .expect("package status");
    }
}
