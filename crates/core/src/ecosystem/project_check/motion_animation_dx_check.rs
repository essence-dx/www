#![allow(clippy::too_many_arguments)]
use std::path::{Path, PathBuf};

use super::super::forge_security::{DxSourceManifest, DxSupplyChainSeverity};
use super::file_hashes::{count_sha256_file_hash_mismatches, has_sha256_file_hashes};
use super::{
    DxCheckFinding, DxCheckMetric, check_finding, check_metric, json_array_entries, json_text,
    read_optional_forge_json, resolve_dx_check_relative_path,
};

const MOTION_ANIMATION_PACKAGE_ID: &str = "animation/motion";
const MOTION_ANIMATION_OFFICIAL_NAME: &str = "Motion & Animation";
const MOTION_ANIMATION_PACKAGE_STATUS: &str = ".dx/forge/package-status.json";
const MOTION_ANIMATION_DASHBOARD_RECEIPT: &str =
    "examples/template/.dx/forge/receipts/2026-05-22-animation-motion-dashboard-workflow.json";

pub(super) fn forge_motion_animation_package_metrics(
    root: &Path,
    manifest: &DxSourceManifest,
) -> (Vec<DxCheckMetric>, Vec<DxCheckFinding>) {
    let mut findings = Vec::new();
    let manifest_package_present = manifest
        .packages
        .iter()
        .any(|package| package.package_id == MOTION_ANIMATION_PACKAGE_ID);
    let mut package_present = u64::from(manifest_package_present);
    let mut receipt_present = 0u64;
    let mut stale_receipt = 0u64;
    let mut missing_receipt = 0u64;
    let mut blocked_surfaces = 0u64;
    let mut unsupported_surfaces = 0u64;
    let mut hash_manifest_present = 0u64;
    let mut hash_mismatches = 0u64;

    let Some(package_status) = read_optional_forge_json(
        root,
        MOTION_ANIMATION_PACKAGE_STATUS,
        "motion-animation-package-status-invalid",
        &mut findings,
    ) else {
        if manifest_package_present
            || package_receipt_exists(root, MOTION_ANIMATION_DASHBOARD_RECEIPT)
        {
            package_present = 1;
            missing_receipt = 1;
            findings.push(missing_package_status_finding());
        }
        return (
            motion_animation_metrics(
                package_present,
                receipt_present,
                stale_receipt,
                missing_receipt,
                blocked_surfaces,
                unsupported_surfaces,
                hash_manifest_present,
                hash_mismatches,
            ),
            findings,
        );
    };

    let Some(visibility) = json_array_entries(&package_status, &["package_lane_visibility"])
        .into_iter()
        .find(|entry| json_text(entry, &["package_id"]) == Some(MOTION_ANIMATION_PACKAGE_ID))
    else {
        if manifest_package_present
            || package_receipt_exists(root, MOTION_ANIMATION_DASHBOARD_RECEIPT)
        {
            package_present = 1;
            missing_receipt = 1;
            findings.push(missing_package_status_finding());
        }
        return (
            motion_animation_metrics(
                package_present,
                receipt_present,
                stale_receipt,
                missing_receipt,
                blocked_surfaces,
                unsupported_surfaces,
                hash_manifest_present,
                hash_mismatches,
            ),
            findings,
        );
    };

    package_present = 1;
    let package_receipt_path = json_text(visibility, &["package_receipt_path"])
        .unwrap_or(MOTION_ANIMATION_DASHBOARD_RECEIPT);
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
        if receipt_present > 0 {
            findings.push(missing_receipt_finding(package_receipt_path));
        }
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
            Some("missing-receipt") => missing_receipt = 1,
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
            "motion-animation-stale-receipt",
            format!("{MOTION_ANIMATION_OFFICIAL_NAME} package-status visibility is stale"),
            Some(MOTION_ANIMATION_PACKAGE_STATUS.to_string()),
            "Regenerate the Motion & Animation package-status row from the dashboard workflow receipt before claiming source freshness.",
        ));
    }
    if blocked_surfaces > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "motion-animation-blocked-surface",
            format!(
                "{MOTION_ANIMATION_OFFICIAL_NAME} has {blocked_surfaces} blocked selected surface(s)"
            ),
            Some(MOTION_ANIMATION_PACKAGE_STATUS.to_string()),
            "Resolve the SOURCE-ONLY / ADAPTER-BOUNDARY Motion & Animation runtime proof boundary before claiming governed browser animation readiness.",
        ));
    }
    if unsupported_surfaces > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "motion-animation-unsupported-surface",
            format!(
                "{MOTION_ANIMATION_OFFICIAL_NAME} has {unsupported_surfaces} unsupported requested surface(s)"
            ),
            Some(MOTION_ANIMATION_PACKAGE_STATUS.to_string()),
            "Request only supported Motion & Animation provider, layout, reorder, motion-value, scroll-progress, or dashboard-workflow surfaces, or add a real upstream-backed Forge surface first.",
        ));
    }
    if hash_mismatches > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "motion-animation-hash-mismatch",
            format!(
                "{MOTION_ANIMATION_OFFICIAL_NAME} has {hash_mismatches} missing or stale hash-backed source file(s)"
            ),
            Some(MOTION_ANIMATION_PACKAGE_STATUS.to_string()),
            "Regenerate the Motion & Animation dashboard workflow receipt after reviewing the changed front-facing animation files.",
        ));
    }

    (
        motion_animation_metrics(
            package_present,
            receipt_present,
            stale_receipt,
            missing_receipt,
            blocked_surfaces,
            unsupported_surfaces,
            hash_manifest_present,
            hash_mismatches,
        ),
        findings,
    )
}

fn motion_animation_metrics(
    package_present: u64,
    receipt_present: u64,
    stale_receipt: u64,
    missing_receipt: u64,
    blocked_surfaces: u64,
    unsupported_surfaces: u64,
    hash_manifest_present: u64,
    hash_mismatches: u64,
) -> Vec<DxCheckMetric> {
    vec![
        check_metric("motion_animation_package_present", package_present),
        check_metric("motion_animation_receipt_present", receipt_present),
        check_metric("motion_animation_receipt_stale", stale_receipt),
        check_metric("motion_animation_missing_receipt", missing_receipt),
        check_metric("motion_animation_blocked_surface", blocked_surfaces),
        check_metric("motion_animation_unsupported_surface", unsupported_surfaces),
        check_metric(
            "motion_animation_hash_manifest_present",
            hash_manifest_present,
        ),
        check_metric("motion_animation_hash_mismatch", hash_mismatches),
    ]
}

fn missing_package_status_finding() -> DxCheckFinding {
    check_finding(
        DxSupplyChainSeverity::Medium,
        "motion-animation-missing-package-status",
        format!("{MOTION_ANIMATION_OFFICIAL_NAME} is missing from package-status visibility"),
        Some(MOTION_ANIMATION_PACKAGE_STATUS.to_string()),
        "Regenerate .dx/forge/package-status.json so dx-check can report Motion & Animation visibility states.",
    )
}

fn missing_receipt_finding(receipt_path: &str) -> DxCheckFinding {
    check_finding(
        DxSupplyChainSeverity::Medium,
        "motion-animation-missing-receipt",
        format!("{MOTION_ANIMATION_OFFICIAL_NAME} is missing its package-lane receipt"),
        Some(receipt_path.to_string()),
        "Regenerate or restore the Motion & Animation dashboard workflow receipt so dx-check can report source-owned package visibility without claiming live browser animation runtime proof.",
    )
}

fn package_receipt_exists(root: &Path, receipt_path: &str) -> bool {
    project_file_exists(root, receipt_path)
}

fn project_file_exists(root: &Path, relative_path: &str) -> bool {
    resolve_project_file(root, relative_path).is_some()
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

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    const STABLE_MOTION_HASH: &str =
        "5cc23b310188f47095878df95ece01d19adaff7ae9a86e6b56f174b9b36c7959";

    #[test]
    fn motion_animation_shared_hash_helper_matches_known_digest() {
        let dir = tempfile::tempdir().expect("tempdir");
        let file_dir = dir.path().join("examples/template/motion");
        fs::create_dir_all(&file_dir).expect("motion dir");
        fs::write(file_dir.join("hash-probe.ts"), b"dx-motion\n").expect("probe");

        let fresh_surface = serde_json::json!({
            "status": "present",
            "hash_algorithm": "sha256",
            "file_hashes": {
                "examples/template/motion/hash-probe.ts": "a1fc4a64132b3e8ca81cfe1e70bce062f173628d25a5a6d943c2b0b9723a3047"
            }
        });
        assert_eq!(
            count_sha256_file_hash_mismatches(dir.path(), &fresh_surface),
            0
        );
    }

    #[test]
    fn motion_animation_shared_hash_helper_detects_stale_bytes() {
        let dir = tempfile::tempdir().expect("tempdir");
        let motion_dir = dir.path().join("components/launch");
        fs::create_dir_all(&motion_dir).expect("motion dir");
        fs::write(
            motion_dir.join("motion-interaction-proof.tsx"),
            b"stable-motion",
        )
        .expect("motion file");

        let fresh_surface = serde_json::json!({
            "status": "present",
            "hash_algorithm": "sha256",
            "file_hashes": {
                "components/launch/motion-interaction-proof.tsx": format!("sha256:{STABLE_MOTION_HASH}")
            }
        });
        assert_eq!(
            count_sha256_file_hash_mismatches(dir.path(), &fresh_surface),
            0
        );

        fs::write(
            motion_dir.join("motion-interaction-proof.tsx"),
            b"changed-motion",
        )
        .expect("changed motion file");
        assert_eq!(
            count_sha256_file_hash_mismatches(dir.path(), &fresh_surface),
            1
        );
    }

    #[test]
    fn motion_animation_package_metrics_report_byte_derived_hash_mismatch() {
        let dir = tempfile::tempdir().expect("tempdir");
        let motion_dir = dir.path().join("components/launch");
        fs::create_dir_all(&motion_dir).expect("motion dir");
        fs::write(
            motion_dir.join("motion-interaction-proof.tsx"),
            b"stable-motion",
        )
        .expect("motion file");
        let receipt_dir = dir.path().join("examples/template/.dx/forge/receipts");
        fs::create_dir_all(&receipt_dir).expect("receipt dir");
        fs::write(
            receipt_dir.join("2026-05-22-animation-motion-dashboard-workflow.json"),
            b"{\"package_id\":\"animation/motion\"}",
        )
        .expect("receipt");
        write_motion_package_status(dir.path(), STABLE_MOTION_HASH, "present", "present");

        let (metrics, findings) =
            forge_motion_animation_package_metrics(dir.path(), &DxSourceManifest::default());

        assert_eq!(
            metric_value(&metrics, "motion_animation_package_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(&metrics, "motion_animation_receipt_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(&metrics, "motion_animation_receipt_stale"),
            Some(0)
        );
        assert_eq!(
            metric_value(&metrics, "motion_animation_hash_manifest_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(&metrics, "motion_animation_hash_mismatch"),
            Some(0)
        );
        assert!(
            !findings
                .iter()
                .any(|finding| finding.code == "motion-animation-hash-mismatch")
        );

        fs::write(
            motion_dir.join("motion-interaction-proof.tsx"),
            b"changed-motion",
        )
        .expect("mutated motion file");

        let (stale_metrics, stale_findings) =
            forge_motion_animation_package_metrics(dir.path(), &DxSourceManifest::default());

        assert_eq!(
            metric_value(&stale_metrics, "motion_animation_receipt_stale"),
            Some(1)
        );
        assert_eq!(
            metric_value(&stale_metrics, "motion_animation_hash_mismatch"),
            Some(1)
        );
        assert!(
            stale_findings
                .iter()
                .any(|finding| finding.code == "motion-animation-hash-mismatch")
        );
    }

    #[test]
    fn motion_animation_package_metrics_honor_missing_receipt_status() {
        let dir = tempfile::tempdir().expect("tempdir");
        let motion_dir = dir.path().join("components/launch");
        fs::create_dir_all(&motion_dir).expect("motion dir");
        fs::write(
            motion_dir.join("motion-interaction-proof.tsx"),
            b"stable-motion",
        )
        .expect("motion file");
        let receipt_dir = dir.path().join("examples/template/.dx/forge/receipts");
        fs::create_dir_all(&receipt_dir).expect("receipt dir");
        fs::write(
            receipt_dir.join("2026-05-22-animation-motion-dashboard-workflow.json"),
            b"{\"package_id\":\"animation/motion\"}",
        )
        .expect("receipt");
        write_motion_package_status(dir.path(), STABLE_MOTION_HASH, "present", "missing-receipt");

        let (metrics, findings) =
            forge_motion_animation_package_metrics(dir.path(), &DxSourceManifest::default());

        assert_eq!(
            metric_value(&metrics, "motion_animation_receipt_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(&metrics, "motion_animation_missing_receipt"),
            Some(1)
        );
        assert!(
            findings
                .iter()
                .any(|finding| finding.code == "motion-animation-missing-receipt")
        );
    }

    fn metric_value(metrics: &[DxCheckMetric], name: &str) -> Option<u64> {
        metrics
            .iter()
            .find(|metric| metric.name == name)
            .map(|metric| metric.value)
    }

    fn write_motion_package_status(
        root: &Path,
        expected_hash: &str,
        visibility_status: &str,
        receipt_status: &str,
    ) {
        let status_dir = root.join(".dx/forge");
        fs::create_dir_all(&status_dir).expect("status dir");
        fs::write(
            status_dir.join("package-status.json"),
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.www.template.forge_package_status",
                "package_lane_visibility": [
                    {
                        "official_package_name": "Motion & Animation",
                        "package_id": "animation/motion",
                        "upstream_package": "motion",
                        "upstream_version": "12.38.0",
                        "source_mirror": "G:/WWW/inspirations/motion",
                        "status": visibility_status,
                        "receipt_status": receipt_status,
                        "package_receipt_path": "examples/template/.dx/forge/receipts/2026-05-22-animation-motion-dashboard-workflow.json",
                        "selected_surfaces": [
                            {
                                "surface_id": "motion-interaction-proof",
                                "status": "present",
                                "receipt_path": "examples/template/.dx/forge/receipts/2026-05-22-animation-motion-dashboard-workflow.json",
                                "files": ["components/launch/motion-interaction-proof.tsx"],
                                "hash_algorithm": "sha256",
                                "file_hashes": {
                                    "components/launch/motion-interaction-proof.tsx": format!("sha256:{expected_hash}")
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
        .expect("write package status");
    }
}
