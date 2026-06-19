#![allow(clippy::too_many_arguments)]
use std::path::{Path, PathBuf};

use super::super::forge_security::{DxSourceManifest, DxSupplyChainSeverity};
use super::file_hashes::count_sha256_file_hash_mismatches;
use super::{
    DxCheckFinding, DxCheckMetric, check_finding, check_metric, json_array_entries, json_text,
    read_optional_forge_json, resolve_dx_check_relative_path,
};

const THREE_SCENE_SYSTEM_PACKAGE_ID: &str = "3d/launch-scene";
const THREE_SCENE_SYSTEM_OFFICIAL_NAME: &str = "3D Scene System";
const THREE_SCENE_SYSTEM_PACKAGE_STATUS: &str = ".dx/forge/package-status.json";
const THREE_SCENE_SYSTEM_DASHBOARD_RECEIPT: &str =
    ".dx/forge/receipts/3d-launch-scene-dashboard-workflow.json";

pub(super) fn forge_three_scene_system_package_metrics(
    root: &Path,
    manifest: &DxSourceManifest,
) -> (Vec<DxCheckMetric>, Vec<DxCheckFinding>) {
    let mut findings = Vec::new();
    let manifest_package_present = manifest
        .packages
        .iter()
        .any(|package| package.package_id == THREE_SCENE_SYSTEM_PACKAGE_ID);
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
        THREE_SCENE_SYSTEM_PACKAGE_STATUS,
        "three-scene-system-package-status-invalid",
        &mut findings,
    ) else {
        if manifest_package_present
            || package_receipt_exists(root, THREE_SCENE_SYSTEM_DASHBOARD_RECEIPT)
        {
            missing_receipt = 1;
            receipt_hash_refresh_missing = 1;
            findings.push(missing_package_status_finding());
        }
        return (
            three_scene_system_metrics(
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
        .find(|entry| json_text(entry, &["package_id"]) == Some(THREE_SCENE_SYSTEM_PACKAGE_ID))
    else {
        if manifest_package_present
            || package_receipt_exists(root, THREE_SCENE_SYSTEM_DASHBOARD_RECEIPT)
        {
            missing_receipt = 1;
            receipt_hash_refresh_missing = 1;
            findings.push(missing_package_status_finding());
        }
        return (
            three_scene_system_metrics(
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

    let (refresh_current, refresh_stale, refresh_missing) = receipt_hash_refresh_counts(visibility);
    receipt_hash_refresh_current = refresh_current;
    receipt_hash_refresh_stale = refresh_stale;
    receipt_hash_refresh_missing = refresh_missing;

    let package_receipt_path = json_text(visibility, &["package_receipt_path"])
        .unwrap_or(THREE_SCENE_SYSTEM_DASHBOARD_RECEIPT);
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

    if hash_mismatches > 0 || receipt_hash_refresh_stale > 0 || receipt_hash_refresh_missing > 0 {
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
            "three-scene-system-stale-receipt",
            format!("{THREE_SCENE_SYSTEM_OFFICIAL_NAME} package-status visibility is stale"),
            Some(THREE_SCENE_SYSTEM_PACKAGE_STATUS.to_string()),
            "Regenerate the 3D Scene System package-status row from the dashboard workflow receipt before claiming source freshness.",
        ));
    }
    if blocked_surfaces > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "three-scene-system-blocked-surface",
            format!(
                "{THREE_SCENE_SYSTEM_OFFICIAL_NAME} has {blocked_surfaces} blocked selected surface(s)"
            ),
            Some(THREE_SCENE_SYSTEM_PACKAGE_STATUS.to_string()),
            "Resolve the SOURCE-ONLY 3D Scene System runtime proof boundary before claiming the selected surface is release-ready.",
        ));
    }
    if unsupported_surfaces > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "three-scene-system-unsupported-surface",
            format!(
                "{THREE_SCENE_SYSTEM_OFFICIAL_NAME} has {unsupported_surfaces} unsupported requested surface(s)"
            ),
            Some(THREE_SCENE_SYSTEM_PACKAGE_STATUS.to_string()),
            "Request only supported 3D Scene System surfaces or add a real upstream-backed Forge surface first.",
        ));
    }
    if hash_mismatches > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "three-scene-system-hash-mismatch",
            format!(
                "{THREE_SCENE_SYSTEM_OFFICIAL_NAME} has {hash_mismatches} missing or stale hash-backed source file(s)"
            ),
            Some(THREE_SCENE_SYSTEM_PACKAGE_STATUS.to_string()),
            "Regenerate the 3D Scene System receipt after reviewing the changed front-facing scene files.",
        ));
    }
    if dx_style_compatibility_missing > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "three-scene-system-missing-dx-style-compatibility",
            format!(
                "{THREE_SCENE_SYSTEM_OFFICIAL_NAME} is missing dx-style compatibility evidence"
            ),
            Some(THREE_SCENE_SYSTEM_PACKAGE_STATUS.to_string()),
            "Regenerate the 3D Scene System dx-style compatibility row from the dashboard workflow receipt and verify source-owned style markers without claiming live browser/WebGL proof.",
        ));
    }

    (
        three_scene_system_metrics(
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

fn three_scene_system_metrics(
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
        check_metric("three_scene_system_receipt_present", receipt_present),
        check_metric("three_scene_system_receipt_stale", stale_receipt),
        check_metric("three_scene_system_missing_receipt", missing_receipt),
        check_metric("three_scene_system_blocked_surface", blocked_surfaces),
        check_metric(
            "three_scene_system_unsupported_surface",
            unsupported_surfaces,
        ),
        check_metric(
            "three_scene_system_hash_manifest_present",
            hash_manifest_present,
        ),
        check_metric("three_scene_system_hash_mismatch", hash_mismatches),
        check_metric(
            "three_scene_system_receipt_hash_refresh_current",
            receipt_hash_refresh_current,
        ),
        check_metric(
            "three_scene_system_receipt_hash_refresh_stale",
            receipt_hash_refresh_stale,
        ),
        check_metric(
            "three_scene_system_receipt_hash_refresh_missing",
            receipt_hash_refresh_missing,
        ),
        check_metric(
            "three_scene_system_dx_style_compatibility_present",
            dx_style_compatibility_present,
        ),
        check_metric(
            "three_scene_system_dx_style_compatibility_missing",
            dx_style_compatibility_missing,
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
        "three-scene-system-missing-package-status",
        format!("{THREE_SCENE_SYSTEM_OFFICIAL_NAME} is missing from package-status visibility"),
        Some(THREE_SCENE_SYSTEM_PACKAGE_STATUS.to_string()),
        "Regenerate .dx/forge/package-status.json so dx-check can report 3D Scene System visibility states.",
    )
}

fn missing_receipt_finding(receipt_path: &str) -> DxCheckFinding {
    check_finding(
        DxSupplyChainSeverity::Medium,
        "three-scene-system-missing-receipt",
        format!("{THREE_SCENE_SYSTEM_OFFICIAL_NAME} is missing its package-lane receipt"),
        Some(receipt_path.to_string()),
        "Regenerate or restore the 3D Scene System dashboard workflow receipt so dx-check can report source-owned package visibility.",
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
    use super::*;
    use std::fs;

    use sha2::{Digest, Sha256};

    #[test]
    fn three_scene_system_hash_mismatch_metric_and_finding_are_byte_derived() {
        let dir = tempfile::tempdir().expect("tempdir");
        let scene_dir = dir.path().join("scene");
        fs::create_dir_all(&scene_dir).expect("scene dir");
        let scene_source = scene_dir.join("dashboard-workflow.ts");
        fs::write(&scene_source, b"export const sceneHealth = 'fresh';\n").expect("scene source");
        let expected_hash = sha256_file(&scene_source);

        write_three_scene_system_receipt(dir.path());
        write_three_scene_system_package_status(dir.path(), &expected_hash);

        let manifest = DxSourceManifest::default();
        let (metrics, findings) = forge_three_scene_system_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(&metrics, "three_scene_system_receipt_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(&metrics, "three_scene_system_hash_manifest_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(&metrics, "three_scene_system_hash_mismatch"),
            Some(0)
        );
        assert_eq!(
            metric_value(&metrics, "three_scene_system_receipt_hash_refresh_current"),
            Some(1)
        );
        assert_eq!(
            metric_value(&metrics, "three_scene_system_receipt_hash_refresh_stale"),
            Some(0)
        );
        assert_eq!(
            metric_value(&metrics, "three_scene_system_receipt_hash_refresh_missing"),
            Some(0)
        );
        assert_eq!(
            metric_value(
                &metrics,
                "three_scene_system_dx_style_compatibility_present"
            ),
            Some(1)
        );
        assert_eq!(
            metric_value(
                &metrics,
                "three_scene_system_dx_style_compatibility_missing"
            ),
            Some(0)
        );
        assert!(
            !findings
                .iter()
                .any(|finding| finding.code == "three-scene-system-hash-mismatch")
        );

        fs::write(&scene_source, b"export const sceneHealth = 'stale';\n").expect("mutate scene");
        let (stale_metrics, stale_findings) =
            forge_three_scene_system_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(&stale_metrics, "three_scene_system_receipt_stale"),
            Some(1)
        );
        assert_eq!(
            metric_value(&stale_metrics, "three_scene_system_hash_mismatch"),
            Some(1)
        );
        assert!(
            stale_findings
                .iter()
                .any(|finding| finding.code == "three-scene-system-hash-mismatch")
        );
    }

    #[test]
    fn three_scene_system_hash_refresh_stale_helper_keeps_source_hash_clean() {
        let dir = tempfile::tempdir().expect("tempdir");
        let scene_dir = dir.path().join("scene");
        fs::create_dir_all(&scene_dir).expect("scene dir");
        let scene_source = scene_dir.join("dashboard-workflow.ts");
        fs::write(&scene_source, b"export const sceneHealth = 'fresh';\n").expect("scene source");
        let expected_hash = sha256_file(&scene_source);

        write_three_scene_system_receipt(dir.path());
        write_three_scene_system_package_status(dir.path(), &expected_hash);

        let manifest = DxSourceManifest::default();
        let (current_metrics, current_findings) =
            forge_three_scene_system_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(
                &current_metrics,
                "three_scene_system_receipt_hash_refresh_current"
            ),
            Some(1)
        );
        assert_eq!(
            metric_value(
                &current_metrics,
                "three_scene_system_receipt_hash_refresh_stale"
            ),
            Some(0)
        );
        assert_eq!(
            metric_value(&current_metrics, "three_scene_system_hash_mismatch"),
            Some(0)
        );
        assert!(
            !current_findings
                .iter()
                .any(|finding| finding.code == "three-scene-system-stale-receipt")
        );

        let package_status_path = dir.path().join(THREE_SCENE_SYSTEM_PACKAGE_STATUS);
        let mut package_status: serde_json::Value =
            serde_json::from_slice(&fs::read(&package_status_path).expect("package status bytes"))
                .expect("package status json");
        let refresh = &mut package_status["package_lane_visibility"][0]["receipt_hash_refresh"];
        refresh["status"] = serde_json::json!("stale");
        refresh["stale_file_count"] = serde_json::json!(1);
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&package_status).expect("package status json"),
        )
        .expect("write stale helper package status");

        let (stale_metrics, stale_findings) =
            forge_three_scene_system_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(
                &stale_metrics,
                "three_scene_system_receipt_hash_refresh_current"
            ),
            Some(0)
        );
        assert_eq!(
            metric_value(
                &stale_metrics,
                "three_scene_system_receipt_hash_refresh_stale"
            ),
            Some(1)
        );
        assert_eq!(
            metric_value(
                &stale_metrics,
                "three_scene_system_receipt_hash_refresh_missing"
            ),
            Some(0)
        );
        assert_eq!(
            metric_value(&stale_metrics, "three_scene_system_hash_mismatch"),
            Some(0)
        );
        assert_eq!(
            metric_value(&stale_metrics, "three_scene_system_receipt_stale"),
            Some(1)
        );
        assert!(
            stale_findings
                .iter()
                .any(|finding| finding.code == "three-scene-system-stale-receipt")
        );
    }

    #[test]
    fn three_scene_system_dx_style_compatibility_missing_is_reported() {
        let dir = tempfile::tempdir().expect("tempdir");
        let scene_dir = dir.path().join("scene");
        fs::create_dir_all(&scene_dir).expect("scene dir");
        let scene_source = scene_dir.join("dashboard-workflow.ts");
        fs::write(&scene_source, b"export const sceneHealth = 'fresh';\n").expect("scene source");
        let expected_hash = sha256_file(&scene_source);

        write_three_scene_system_receipt(dir.path());
        write_three_scene_system_package_status_without_dx_style(dir.path(), &expected_hash);

        let manifest = DxSourceManifest::default();
        let (metrics, findings) = forge_three_scene_system_package_metrics(dir.path(), &manifest);

        assert_eq!(
            metric_value(
                &metrics,
                "three_scene_system_dx_style_compatibility_present"
            ),
            Some(0)
        );
        assert_eq!(
            metric_value(
                &metrics,
                "three_scene_system_dx_style_compatibility_missing"
            ),
            Some(1)
        );
        assert!(
            findings
                .iter()
                .any(|finding| finding.code == "three-scene-system-missing-dx-style-compatibility")
        );
    }

    fn write_three_scene_system_receipt(root: &Path) {
        let receipt_path = root.join(THREE_SCENE_SYSTEM_DASHBOARD_RECEIPT);
        fs::create_dir_all(receipt_path.parent().expect("receipt parent"))
            .expect("receipt directory");
        fs::write(
            receipt_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.forge.three_scene_system_receipt",
                "official_package_name": THREE_SCENE_SYSTEM_OFFICIAL_NAME,
                "package_id": THREE_SCENE_SYSTEM_PACKAGE_ID,
                "surface": "launch-scene-dashboard-workflow"
            }))
            .expect("receipt json"),
        )
        .expect("write receipt");
    }

    fn write_three_scene_system_package_status(root: &Path, expected_hash: &str) {
        write_three_scene_system_package_status_with_dx_style(root, expected_hash, true);
    }

    fn write_three_scene_system_package_status_without_dx_style(root: &Path, expected_hash: &str) {
        write_three_scene_system_package_status_with_dx_style(root, expected_hash, false);
    }

    fn write_three_scene_system_package_status_with_dx_style(
        root: &Path,
        expected_hash: &str,
        include_dx_style_compatibility: bool,
    ) {
        let package_status_path = root.join(THREE_SCENE_SYSTEM_PACKAGE_STATUS);
        fs::create_dir_all(package_status_path.parent().expect("package status parent"))
            .expect("package status directory");
        let mut visibility = serde_json::json!({
            "official_package_name": THREE_SCENE_SYSTEM_OFFICIAL_NAME,
            "package_id": THREE_SCENE_SYSTEM_PACKAGE_ID,
            "upstream_package": "three + @react-three/fiber + @react-three/drei",
            "source_mirror": [
                "G:/WWW/inspirations/three.js",
                "G:/WWW/inspirations/react-three-fiber",
                "G:/WWW/inspirations/drei"
            ],
            "status": "present",
            "receipt_status": "present",
            "package_receipt_path": THREE_SCENE_SYSTEM_DASHBOARD_RECEIPT,
            "selected_surfaces": [
                {
                    "surface_id": "launch-scene-dashboard-workflow",
                    "status": "present",
                    "hash_algorithm": "sha256",
                    "file_hashes": {
                        "examples/template/scene/dashboard-workflow.ts": expected_hash
                    }
                }
            ],
            "blocked_surfaces": [],
            "unsupported_surfaces": [],
            "receipt_hash_refresh": {
                "schema": "dx.forge.package.receipt_hash_refresh",
                "status": "current",
                "helper_path": "examples/template/3d-scene-system-receipt-hashes.ts",
                "check_command": "node examples/template/3d-scene-system-receipt-hashes.ts --check",
                "write_command": "node examples/template/3d-scene-system-receipt-hashes.ts --write",
                "json_check_command": "node examples/template/3d-scene-system-receipt-hashes.ts --check --json",
                "receipt_path": THREE_SCENE_SYSTEM_DASHBOARD_RECEIPT,
                "hash_algorithm": "sha256",
                "tracked_file_count": 1,
                "stale_file_count": 0,
                "missing_file_count": 0,
                "runtime_execution": false,
                "secret_access": false,
                "zed_visibility": "3d-scene-system:receipt-hash-refresh"
            }
        });
        if include_dx_style_compatibility {
            visibility
                .as_object_mut()
                .expect("visibility object")
                .insert(
                    "dx_style_compatibility".to_string(),
                    serde_json::json!({
                        "schema": "dx.forge.package.dx_style_compatibility",
                        "status": "present",
                        "token_source": "examples/template/launch-scene.tsx",
                        "generated_css": "tools/launch/runtime-template/assets/launch-runtime.css",
                        "visible_surfaces": ["launch-scene-dashboard-workflow"],
                        "source_files": ["examples/template/launch-scene.tsx"],
                        "data_dx_markers": [
                            "data-dx-style-surface=\"launch-scene\"",
                            "data-dx-token-scope=\"3d/launch-scene\""
                        ],
                        "receipt_path": THREE_SCENE_SYSTEM_DASHBOARD_RECEIPT,
                        "runtime_proof": false,
                        "runtime_limitations": [
                            "SOURCE-ONLY: no live governed browser/WebGL style proof is claimed."
                        ]
                    }),
                );
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
