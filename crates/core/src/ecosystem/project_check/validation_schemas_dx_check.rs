#![allow(clippy::too_many_arguments)]
use std::path::Path;

use super::super::forge_security::{DxSourceManifest, DxSupplyChainSeverity};
use super::file_hashes::count_sha256_file_hash_mismatches;
use super::{
    DxCheckFinding, DxCheckMetric, check_finding, check_metric, json_array_entries, json_text,
    read_optional_forge_json, resolve_dx_check_relative_path,
};

const VALIDATION_SCHEMAS_PACKAGE_ID: &str = "validation/zod";
const VALIDATION_SCHEMAS_OFFICIAL_NAME: &str = "Validation & Schemas";
const VALIDATION_SCHEMAS_PACKAGE_STATUS: &str = ".dx/forge/package-status.json";
const VALIDATION_SCHEMAS_DASHBOARD_RECEIPT: &str =
    ".dx/forge/receipts/2026-05-22-validation-zod-dashboard-settings.json";

pub(super) fn forge_validation_schemas_package_metrics(
    root: &Path,
    manifest: &DxSourceManifest,
) -> (Vec<DxCheckMetric>, Vec<DxCheckFinding>) {
    let mut findings = Vec::new();
    let manifest_package_present = manifest
        .packages
        .iter()
        .any(|package| package.package_id == VALIDATION_SCHEMAS_PACKAGE_ID);
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
        VALIDATION_SCHEMAS_PACKAGE_STATUS,
        "validation-schemas-package-status-invalid",
        &mut findings,
    ) else {
        if manifest_package_present
            || package_receipt_exists(root, VALIDATION_SCHEMAS_DASHBOARD_RECEIPT)
        {
            package_present = 1;
            missing_receipt = 1;
            dx_style_compatibility_missing = 1;
            findings.push(missing_package_status_finding());
        }
        return (
            validation_schemas_metrics(
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
        .find(|entry| json_text(entry, &["package_id"]) == Some(VALIDATION_SCHEMAS_PACKAGE_ID))
    else {
        if manifest_package_present
            || package_receipt_exists(root, VALIDATION_SCHEMAS_DASHBOARD_RECEIPT)
        {
            package_present = 1;
            missing_receipt = 1;
            dx_style_compatibility_missing = 1;
            findings.push(missing_package_status_finding());
        }
        return (
            validation_schemas_metrics(
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
    dx_style_compatibility_present = u64::from(dx_style_compatibility_is_present(visibility));
    dx_style_compatibility_missing = u64::from(dx_style_compatibility_present == 0);

    let package_receipt_path = json_text(visibility, &["package_receipt_path"])
        .unwrap_or(VALIDATION_SCHEMAS_DASHBOARD_RECEIPT);
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
            status if status_is_stale(status) => stale_receipt = 1,
            status if status_is_missing_receipt(status) => missing_receipt = 1,
            status if status_is_blocked(status) => blocked_surfaces += 1,
            status if status_is_unsupported_surface(status) => unsupported_surfaces += 1,
            _ => {}
        }
    }

    if hash_mismatches > 0 {
        stale_receipt = 1;
    }

    if stale_receipt > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "validation-schemas-stale-receipt",
            format!("{VALIDATION_SCHEMAS_OFFICIAL_NAME} package-status visibility is stale"),
            Some(VALIDATION_SCHEMAS_PACKAGE_STATUS.to_string()),
            "Regenerate the Validation & Schemas package-status row from the dashboard settings receipt before claiming source freshness.",
        ));
    }
    if blocked_surfaces > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "validation-schemas-blocked-surface",
            format!(
                "{VALIDATION_SCHEMAS_OFFICIAL_NAME} has {blocked_surfaces} blocked selected surface(s)"
            ),
            Some(VALIDATION_SCHEMAS_PACKAGE_STATUS.to_string()),
            "Resolve the app-owned Validation & Schemas runtime boundary before claiming the selected surface is release-ready.",
        ));
    }
    if unsupported_surfaces > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "validation-schemas-unsupported-surface",
            format!(
                "{VALIDATION_SCHEMAS_OFFICIAL_NAME} has {unsupported_surfaces} unsupported requested surface(s)"
            ),
            Some(VALIDATION_SCHEMAS_PACKAGE_STATUS.to_string()),
            "Request only supported Validation & Schemas surfaces or add a real upstream-backed Forge surface first.",
        ));
    }
    if hash_mismatches > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "validation-schemas-hash-mismatch",
            format!(
                "{VALIDATION_SCHEMAS_OFFICIAL_NAME} has {hash_mismatches} missing or stale hash-backed source file(s)"
            ),
            Some(VALIDATION_SCHEMAS_PACKAGE_STATUS.to_string()),
            "Regenerate the Validation & Schemas dashboard settings receipt after reviewing the changed schema and dashboard files.",
        ));
    }
    if dx_style_compatibility_missing > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "validation-schemas-missing-dx-style-compatibility",
            format!(
                "{VALIDATION_SCHEMAS_OFFICIAL_NAME} is missing dx-style compatibility metadata"
            ),
            Some(VALIDATION_SCHEMAS_PACKAGE_STATUS.to_string()),
            "Restore the Validation & Schemas dx_style_compatibility block and data-dx-style-surface markers before claiming visible UI style compatibility.",
        ));
    }

    (
        validation_schemas_metrics(
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

fn validation_schemas_metrics(
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
        check_metric("validation_schemas_package_present", package_present),
        check_metric("validation_schemas_receipt_present", receipt_present),
        check_metric("validation_schemas_receipt_stale", stale_receipt),
        check_metric("validation_schemas_missing_receipt", missing_receipt),
        check_metric("validation_schemas_blocked_surface", blocked_surfaces),
        check_metric(
            "validation_schemas_unsupported_surface",
            unsupported_surfaces,
        ),
        check_metric(
            "validation_schemas_hash_manifest_present",
            hash_manifest_present,
        ),
        check_metric("validation_schemas_hash_mismatch", hash_mismatches),
        check_metric(
            "validation_schemas_dx_style_compatibility_present",
            dx_style_compatibility_present,
        ),
        check_metric(
            "validation_schemas_dx_style_compatibility_missing",
            dx_style_compatibility_missing,
        ),
    ]
}

fn dx_style_compatibility_is_present(visibility: &serde_json::Value) -> bool {
    json_text(visibility, &["dx_style_compatibility", "schema"])
        == Some("dx.forge.package.dx_style_compatibility")
        && json_text(visibility, &["dx_style_compatibility", "status"]) == Some("present")
        && json_text(visibility, &["dx_style_compatibility", "token_source"])
            == Some("styles/theme.css")
        && json_text(visibility, &["dx_style_compatibility", "generated_css"])
            == Some("styles/generated.css")
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

fn missing_package_status_finding() -> DxCheckFinding {
    check_finding(
        DxSupplyChainSeverity::Medium,
        "validation-schemas-missing-package-status",
        format!("{VALIDATION_SCHEMAS_OFFICIAL_NAME} is missing from package-status visibility"),
        Some(VALIDATION_SCHEMAS_PACKAGE_STATUS.to_string()),
        "Regenerate .dx/forge/package-status.json so dx-check can report Validation & Schemas visibility states.",
    )
}

fn missing_receipt_finding(receipt_path: &str) -> DxCheckFinding {
    check_finding(
        DxSupplyChainSeverity::Medium,
        "validation-schemas-missing-receipt",
        format!("{VALIDATION_SCHEMAS_OFFICIAL_NAME} is missing its package-lane receipt"),
        Some(receipt_path.to_string()),
        "Regenerate or restore the Validation & Schemas dashboard settings receipt so dx-check can report source-owned package visibility.",
    )
}

fn package_receipt_exists(root: &Path, receipt_path: &str) -> bool {
    project_file_path(root, receipt_path).is_some()
}

fn project_file_path(root: &Path, relative_path: &str) -> Option<std::path::PathBuf> {
    if let Some(path) = resolve_dx_check_relative_path(root, relative_path)
        && path.is_file()
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
    use sha2::{Digest, Sha256};
    use std::{fs, path::Path};

    #[test]
    fn validation_schemas_hash_mismatch_flips_when_selected_file_changes() {
        let dir = tempfile::tempdir().expect("tempdir");
        let validation_dir = dir.path().join("lib/validation/zod");
        fs::create_dir_all(&validation_dir).expect("validation dir");
        let settings_file = validation_dir.join("dashboard-settings.ts");
        fs::write(
            &settings_file,
            b"export const dashboardSettingsSchemaVersion = 'fresh';\n",
        )
        .expect("settings source");

        let expected_hash = sha256_file(&settings_file);
        write_validation_schemas_receipt(dir.path());
        write_validation_schemas_package_status(dir.path(), &expected_hash);

        let manifest = DxSourceManifest::default();
        let (metrics, findings) = forge_validation_schemas_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(&metrics, "validation_schemas_hash_manifest_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(&metrics, "validation_schemas_hash_mismatch"),
            Some(0)
        );
        assert_eq!(
            metric_value(&metrics, "validation_schemas_receipt_stale"),
            Some(0)
        );
        assert_eq!(
            metric_value(
                &metrics,
                "validation_schemas_dx_style_compatibility_present"
            ),
            Some(1)
        );
        assert_eq!(
            metric_value(
                &metrics,
                "validation_schemas_dx_style_compatibility_missing"
            ),
            Some(0)
        );
        assert!(
            !findings
                .iter()
                .any(|finding| finding.code == "validation-schemas-hash-mismatch")
        );

        fs::write(
            &settings_file,
            b"export const dashboardSettingsSchemaVersion = 'stale';\n",
        )
        .expect("stale settings source");
        let (stale_metrics, stale_findings) =
            forge_validation_schemas_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(&stale_metrics, "validation_schemas_hash_mismatch"),
            Some(1)
        );
        assert_eq!(
            metric_value(&stale_metrics, "validation_schemas_receipt_stale"),
            Some(1)
        );
        assert!(
            stale_findings
                .iter()
                .any(|finding| finding.code == "validation-schemas-hash-mismatch")
        );
    }

    #[test]
    fn validation_schemas_dx_style_missing_metric_and_finding_flip() {
        let dir = tempfile::tempdir().expect("tempdir");
        let validation_dir = dir.path().join("lib/validation/zod");
        fs::create_dir_all(&validation_dir).expect("validation dir");
        let settings_file = validation_dir.join("dashboard-settings.ts");
        fs::write(
            &settings_file,
            b"export const dashboardSettingsSchemaVersion = 'fresh';\n",
        )
        .expect("settings source");

        let expected_hash = sha256_file(&settings_file);
        write_validation_schemas_receipt(dir.path());
        write_validation_schemas_package_status(dir.path(), &expected_hash);

        let manifest = DxSourceManifest::default();
        let (metrics, findings) = forge_validation_schemas_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(
                &metrics,
                "validation_schemas_dx_style_compatibility_present"
            ),
            Some(1)
        );
        assert_eq!(
            metric_value(
                &metrics,
                "validation_schemas_dx_style_compatibility_missing"
            ),
            Some(0)
        );
        assert!(
            !findings
                .iter()
                .any(|finding| finding.code == "validation-schemas-missing-dx-style-compatibility")
        );

        write_validation_schemas_package_status_without_dx_style(dir.path(), &expected_hash);
        let (missing_metrics, missing_findings) =
            forge_validation_schemas_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(
                &missing_metrics,
                "validation_schemas_dx_style_compatibility_present"
            ),
            Some(0)
        );
        assert_eq!(
            metric_value(
                &missing_metrics,
                "validation_schemas_dx_style_compatibility_missing"
            ),
            Some(1)
        );
        assert!(
            missing_findings
                .iter()
                .any(|finding| finding.code == "validation-schemas-missing-dx-style-compatibility")
        );
    }

    fn write_validation_schemas_receipt(root: &Path) {
        let receipt_path = root.join(VALIDATION_SCHEMAS_DASHBOARD_RECEIPT);
        fs::create_dir_all(receipt_path.parent().expect("receipt parent"))
            .expect("receipt directory");
        fs::write(
            receipt_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.forge.validation_schemas_receipt",
                "official_package_name": VALIDATION_SCHEMAS_OFFICIAL_NAME,
                "package_id": VALIDATION_SCHEMAS_PACKAGE_ID
            }))
            .expect("receipt json"),
        )
        .expect("write receipt");
    }

    fn write_validation_schemas_package_status(root: &Path, expected_hash: &str) {
        write_validation_schemas_package_status_with_dx_style(root, expected_hash, true);
    }

    fn write_validation_schemas_package_status_without_dx_style(root: &Path, expected_hash: &str) {
        write_validation_schemas_package_status_with_dx_style(root, expected_hash, false);
    }

    fn write_validation_schemas_package_status_with_dx_style(
        root: &Path,
        expected_hash: &str,
        include_dx_style: bool,
    ) {
        let package_status_path = root.join(VALIDATION_SCHEMAS_PACKAGE_STATUS);
        fs::create_dir_all(package_status_path.parent().expect("package status parent"))
            .expect("package status directory");
        let mut visibility = serde_json::json!({
            "official_package_name": VALIDATION_SCHEMAS_OFFICIAL_NAME,
            "package_id": VALIDATION_SCHEMAS_PACKAGE_ID,
            "upstream_package": "zod",
            "source_mirror": "G:/WWW/inspirations/zod",
            "status": "present",
            "receipt_status": "present",
            "package_receipt_path": VALIDATION_SCHEMAS_DASHBOARD_RECEIPT,
            "selected_surfaces": [
                {
                    "surface_id": "dashboard-settings-validation",
                    "status": "present",
                    "hash_algorithm": "sha256",
                    "file_hashes": {
                        "lib/validation/zod/dashboard-settings.ts": expected_hash
                    }
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
                "generated_css": "styles/generated.css"
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
