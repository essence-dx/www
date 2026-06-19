#![allow(clippy::too_many_arguments)]
use std::path::Path;

use super::super::forge_security::{DxSourceManifest, DxSupplyChainSeverity};
use super::file_hashes::count_sha256_file_hash_mismatches;
use super::{
    DxCheckFinding, DxCheckMetric, check_finding, check_metric, json_array_entries, json_text,
    read_optional_forge_json, resolve_dx_check_relative_path,
};

const TYPE_SAFE_API_PACKAGE_ID: &str = "api/trpc";
const TYPE_SAFE_API_OFFICIAL_NAME: &str = "Type-Safe API";
const TYPE_SAFE_API_PACKAGE_STATUS: &str = ".dx/forge/package-status.json";
const TYPE_SAFE_API_DASHBOARD_RECEIPT: &str =
    ".dx/forge/receipts/2026-05-22-api-trpc-dashboard-workflow.json";

pub(super) fn forge_type_safe_api_package_metrics(
    root: &Path,
    manifest: &DxSourceManifest,
) -> (Vec<DxCheckMetric>, Vec<DxCheckFinding>) {
    let mut findings = Vec::new();
    let manifest_package_present = manifest
        .packages
        .iter()
        .any(|package| package.package_id == TYPE_SAFE_API_PACKAGE_ID);
    let mut package_present = u64::from(manifest_package_present);
    let mut receipt_present = 0u64;
    let mut stale_receipt = 0u64;
    let mut missing_receipt = 0u64;
    let mut missing_receipt_finding_emitted = false;
    let mut blocked_surfaces = 0u64;
    let mut unsupported_surfaces = 0u64;
    let mut hash_manifest_present = 0u64;
    let mut hash_mismatches = 0u64;

    let Some(package_status) = read_optional_forge_json(
        root,
        TYPE_SAFE_API_PACKAGE_STATUS,
        "type-safe-api-package-status-invalid",
        &mut findings,
    ) else {
        if manifest_package_present || package_receipt_exists(root, TYPE_SAFE_API_DASHBOARD_RECEIPT)
        {
            package_present = 1;
            missing_receipt = 1;
            findings.push(missing_package_status_finding());
        }
        return (
            type_safe_api_metrics(
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
        .find(|entry| json_text(entry, &["package_id"]) == Some(TYPE_SAFE_API_PACKAGE_ID))
    else {
        if manifest_package_present || package_receipt_exists(root, TYPE_SAFE_API_DASHBOARD_RECEIPT)
        {
            package_present = 1;
            missing_receipt = 1;
            findings.push(missing_package_status_finding());
        }
        return (
            type_safe_api_metrics(
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

    let package_receipt_path =
        json_text(visibility, &["package_receipt_path"]).unwrap_or(TYPE_SAFE_API_DASHBOARD_RECEIPT);
    if package_receipt_exists(root, package_receipt_path) {
        receipt_present = 1;
    } else {
        missing_receipt = 1;
        findings.push(missing_receipt_finding(package_receipt_path));
        missing_receipt_finding_emitted = true;
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

    if hash_mismatches > 0 {
        stale_receipt = 1;
    }

    if missing_receipt > 0 && !missing_receipt_finding_emitted {
        findings.push(missing_receipt_finding(package_receipt_path));
    }
    if stale_receipt > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "type-safe-api-stale-receipt",
            format!("{TYPE_SAFE_API_OFFICIAL_NAME} package-status visibility is stale"),
            Some(TYPE_SAFE_API_PACKAGE_STATUS.to_string()),
            "Regenerate the Type-Safe API package-status row from the dashboard workflow receipt before claiming source freshness.",
        ));
    }
    if blocked_surfaces > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "type-safe-api-blocked-surface",
            format!(
                "{TYPE_SAFE_API_OFFICIAL_NAME} has {blocked_surfaces} blocked selected surface(s)"
            ),
            Some(TYPE_SAFE_API_PACKAGE_STATUS.to_string()),
            "Resolve the app-owned Type-Safe API runtime boundary before claiming the selected surface is release-ready.",
        ));
    }
    if unsupported_surfaces > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "type-safe-api-unsupported-surface",
            format!(
                "{TYPE_SAFE_API_OFFICIAL_NAME} has {unsupported_surfaces} unsupported requested surface(s)"
            ),
            Some(TYPE_SAFE_API_PACKAGE_STATUS.to_string()),
            "Request only supported Type-Safe API surfaces or add a real upstream-backed Forge surface first.",
        ));
    }
    if hash_mismatches > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "type-safe-api-hash-mismatch",
            format!(
                "{TYPE_SAFE_API_OFFICIAL_NAME} has {hash_mismatches} missing or stale hash-backed source file(s)"
            ),
            Some(TYPE_SAFE_API_PACKAGE_STATUS.to_string()),
            "Regenerate the Type-Safe API dashboard workflow receipt after reviewing the changed front-facing typed API files.",
        ));
    }

    (
        type_safe_api_metrics(
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

fn type_safe_api_metrics(
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
        check_metric("type_safe_api_package_present", package_present),
        check_metric("type_safe_api_receipt_present", receipt_present),
        check_metric("type_safe_api_receipt_stale", stale_receipt),
        check_metric("type_safe_api_missing_receipt", missing_receipt),
        check_metric("type_safe_api_blocked_surface", blocked_surfaces),
        check_metric("type_safe_api_unsupported_surface", unsupported_surfaces),
        check_metric("type_safe_api_hash_manifest_present", hash_manifest_present),
        check_metric("type_safe_api_hash_mismatch", hash_mismatches),
    ]
}

fn missing_package_status_finding() -> DxCheckFinding {
    check_finding(
        DxSupplyChainSeverity::Medium,
        "type-safe-api-missing-package-status",
        format!("{TYPE_SAFE_API_OFFICIAL_NAME} is missing from package-status visibility"),
        Some(TYPE_SAFE_API_PACKAGE_STATUS.to_string()),
        "Regenerate .dx/forge/package-status.json so dx-check can report Type-Safe API visibility states.",
    )
}

fn missing_receipt_finding(receipt_path: &str) -> DxCheckFinding {
    check_finding(
        DxSupplyChainSeverity::Medium,
        "type-safe-api-missing-receipt",
        format!("{TYPE_SAFE_API_OFFICIAL_NAME} is missing its package-lane receipt"),
        Some(receipt_path.to_string()),
        "Regenerate or restore the Type-Safe API dashboard workflow receipt so dx-check can report source-owned package visibility.",
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
    use std::{fs, path::Path};

    use sha2::{Digest, Sha256};

    use super::*;

    #[test]
    fn type_safe_api_hash_mismatch_flips_when_selected_file_changes() {
        let dir = tempfile::tempdir().expect("tempdir");
        let launch_dir = dir.path().join("examples/template");
        fs::create_dir_all(&launch_dir).expect("launch template dir");
        let health_surface = launch_dir.join("trpc-launch-health.tsx");
        fs::write(
            &health_surface,
            "export function TrpcLaunchHealth() { return null; }\n",
        )
        .expect("type-safe api launch health surface");

        let expected_hash =
            sha256_project_file(dir.path(), "examples/template/trpc-launch-health.tsx")
                .expect("type-safe api surface hash");
        write_type_safe_api_receipt(dir.path());
        write_type_safe_api_package_status(dir.path(), &expected_hash);

        let manifest = DxSourceManifest::default();
        let (metrics, findings) = forge_type_safe_api_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(&metrics, "type_safe_api_hash_manifest_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(&metrics, "type_safe_api_receipt_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(&metrics, "type_safe_api_receipt_stale"),
            Some(0)
        );
        assert_eq!(
            metric_value(&metrics, "type_safe_api_hash_mismatch"),
            Some(0)
        );
        assert!(
            !findings
                .iter()
                .any(|finding| finding.code == "type-safe-api-hash-mismatch")
        );

        fs::write(
            &health_surface,
            "export function TrpcLaunchHealth() { return <section />; }\n",
        )
        .expect("mutate type-safe api launch health surface");

        let (stale_metrics, stale_findings) =
            forge_type_safe_api_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(&stale_metrics, "type_safe_api_hash_mismatch"),
            Some(1)
        );
        assert_eq!(
            metric_value(&stale_metrics, "type_safe_api_receipt_stale"),
            Some(1)
        );
        assert!(
            stale_findings
                .iter()
                .any(|finding| finding.code == "type-safe-api-hash-mismatch")
        );
    }

    fn write_type_safe_api_receipt(root: &Path) {
        let receipt_path = root.join(TYPE_SAFE_API_DASHBOARD_RECEIPT);
        fs::create_dir_all(receipt_path.parent().expect("receipt parent"))
            .expect("receipt directory");
        fs::write(
            receipt_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.forge.type_safe_api_receipt",
                "official_package_name": TYPE_SAFE_API_OFFICIAL_NAME,
                "package_id": TYPE_SAFE_API_PACKAGE_ID
            }))
            .expect("receipt json"),
        )
        .expect("write type-safe api receipt");
    }

    fn write_type_safe_api_package_status(root: &Path, expected_hash: &str) {
        let package_status_path = root.join(TYPE_SAFE_API_PACKAGE_STATUS);
        fs::create_dir_all(package_status_path.parent().expect("package status parent"))
            .expect("package status directory");
        fs::write(
            package_status_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.www.template.forge_package_status",
                "package_lane_visibility": [
                    {
                        "official_package_name": TYPE_SAFE_API_OFFICIAL_NAME,
                        "package_id": TYPE_SAFE_API_PACKAGE_ID,
                        "upstream_package": "@trpc/server",
                        "upstream_version": "11.17.0",
                        "source_mirror": "G:/WWW/inspirations/trpc",
                        "status": "present",
                        "receipt_status": "present",
                        "package_receipt_path": TYPE_SAFE_API_DASHBOARD_RECEIPT,
                        "selected_surfaces": [
                            {
                                "surface_id": "trpc-launch-dashboard-workflow",
                                "status": "present",
                                "hash_algorithm": "sha256",
                                "files": [
                                    "components/launch/trpc-launch-health.tsx"
                                ],
                                "file_hashes": {
                                    "examples/template/trpc-launch-health.tsx": format!("sha256:{expected_hash}")
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
        .expect("write type-safe api package status");
    }

    fn sha256_project_file(root: &Path, relative_path: &str) -> Option<String> {
        let path = resolve_dx_check_relative_path(root, relative_path)?;
        let bytes = fs::read(path).ok()?;
        Some(format!("{:x}", Sha256::digest(&bytes)))
    }

    fn metric_value(metrics: &[DxCheckMetric], name: &str) -> Option<u64> {
        metrics
            .iter()
            .find(|metric| metric.name == name)
            .map(|metric| metric.value)
    }
}
