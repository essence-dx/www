#![allow(clippy::too_many_arguments)]
use std::{fs, path::Path};

use sha2::{Digest, Sha256};

use super::super::forge_security::{DxSourceManifest, DxSupplyChainSeverity};
use super::{
    DxCheckFinding, DxCheckMetric, check_finding, check_metric, json_array_entries, json_text,
    read_optional_forge_json, resolve_dx_check_relative_path,
};

const DATABASE_ORM_PACKAGE_ID: &str = "db/drizzle-sqlite";
const DATABASE_ORM_OFFICIAL_NAME: &str = "Database ORM";
const DATABASE_ORM_PACKAGE_STATUS: &str = ".dx/forge/package-status.json";
const DATABASE_ORM_DASHBOARD_RECEIPT: &str =
    ".dx/forge/receipts/2026-05-22-db-drizzle-sqlite-dashboard-workflow.json";

pub(super) fn forge_database_orm_package_metrics(
    root: &Path,
    manifest: &DxSourceManifest,
) -> (Vec<DxCheckMetric>, Vec<DxCheckFinding>) {
    let mut findings = Vec::new();
    let manifest_package_present = manifest
        .packages
        .iter()
        .any(|package| package.package_id == DATABASE_ORM_PACKAGE_ID);
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
    let mut dx_style_compatibility_present = 0u64;
    let mut dx_style_compatibility_missing = 0u64;

    let Some(package_status) = read_optional_forge_json(
        root,
        DATABASE_ORM_PACKAGE_STATUS,
        "database-orm-package-status-invalid",
        &mut findings,
    ) else {
        if manifest_package_present || package_receipt_exists(root, DATABASE_ORM_DASHBOARD_RECEIPT)
        {
            package_present = 1;
            missing_receipt = 1;
            findings.push(missing_package_status_finding());
        }
        return (
            database_orm_metrics(
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
                dx_style_compatibility_present,
                dx_style_compatibility_missing,
            ),
            findings,
        );
    };

    let Some(visibility) = json_array_entries(&package_status, &["package_lane_visibility"])
        .into_iter()
        .find(|entry| json_text(entry, &["package_id"]) == Some(DATABASE_ORM_PACKAGE_ID))
    else {
        if manifest_package_present || package_receipt_exists(root, DATABASE_ORM_DASHBOARD_RECEIPT)
        {
            package_present = 1;
            missing_receipt = 1;
            findings.push(missing_package_status_finding());
        }
        return (
            database_orm_metrics(
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
        json_text(visibility, &["package_receipt_path"]).unwrap_or(DATABASE_ORM_DASHBOARD_RECEIPT);
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
    if matches!(
        visibility_status.or(receipt_status),
        Some("unsupported-surface")
    ) {
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

        hash_mismatches += count_hash_mismatches(root, surface);

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
            "database-orm-stale-receipt",
            format!("{DATABASE_ORM_OFFICIAL_NAME} package-status visibility is stale"),
            Some(DATABASE_ORM_PACKAGE_STATUS.to_string()),
            "Regenerate the Database ORM package-status row from the dashboard workflow receipt before claiming source freshness.",
        ));
    }
    if blocked_surfaces > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "database-orm-blocked-surface",
            format!(
                "{DATABASE_ORM_OFFICIAL_NAME} has {blocked_surfaces} blocked selected surface(s)"
            ),
            Some(DATABASE_ORM_PACKAGE_STATUS.to_string()),
            "Resolve the app-owned Database ORM runtime boundary before claiming the selected surface is release-ready.",
        ));
    }
    if unsupported_surfaces > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "database-orm-unsupported-surface",
            format!(
                "{DATABASE_ORM_OFFICIAL_NAME} has {unsupported_surfaces} unsupported requested surface(s)"
            ),
            Some(DATABASE_ORM_PACKAGE_STATUS.to_string()),
            "Request only supported Database ORM surfaces or add a real upstream-backed Forge surface first.",
        ));
    }
    if hash_mismatches > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "database-orm-hash-mismatch",
            format!(
                "{DATABASE_ORM_OFFICIAL_NAME} has {hash_mismatches} missing or stale hash-backed selected file(s)"
            ),
            Some(DATABASE_ORM_PACKAGE_STATUS.to_string()),
            "Regenerate the Database ORM dashboard workflow receipt after reviewing the changed Drizzle source and launch dashboard files.",
        ));
    }
    if dx_style_compatibility_missing > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "database-orm-missing-dx-style-compatibility",
            format!("{DATABASE_ORM_OFFICIAL_NAME} is missing dx-style compatibility evidence"),
            Some(DATABASE_ORM_PACKAGE_STATUS.to_string()),
            "Regenerate the Database ORM dx-style compatibility row from the dashboard workflow receipt and verify source-owned token markers without claiming live SQLite visual proof.",
        ));
    }

    (
        database_orm_metrics(
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
            dx_style_compatibility_present,
            dx_style_compatibility_missing,
        ),
        findings,
    )
}

fn database_orm_metrics(
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
    dx_style_compatibility_present: u64,
    dx_style_compatibility_missing: u64,
) -> Vec<DxCheckMetric> {
    vec![
        check_metric("database_orm_package_present", package_present),
        check_metric("database_orm_receipt_present", receipt_present),
        check_metric("database_orm_receipt_stale", stale_receipt),
        check_metric("database_orm_missing_receipt", missing_receipt),
        check_metric("database_orm_blocked_surface", blocked_surfaces),
        check_metric("database_orm_unsupported_surface", unsupported_surfaces),
        check_metric("database_orm_hash_manifest_present", hash_manifest_present),
        check_metric("database_orm_hash_mismatch", hash_mismatches),
        check_metric(
            "database_orm_receipt_hash_refresh_current",
            receipt_hash_refresh_current,
        ),
        check_metric(
            "database_orm_receipt_hash_refresh_stale",
            receipt_hash_refresh_stale,
        ),
        check_metric(
            "database_orm_receipt_hash_refresh_missing",
            receipt_hash_refresh_missing,
        ),
        check_metric(
            "database_orm_dx_style_compatibility_present",
            dx_style_compatibility_present,
        ),
        check_metric(
            "database_orm_dx_style_compatibility_missing",
            dx_style_compatibility_missing,
        ),
    ]
}

fn count_hash_mismatches(root: &Path, surface: &serde_json::Value) -> u64 {
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

fn receipt_hash_refresh_counts(package: &serde_json::Value) -> (u64, u64, u64) {
    let Some(refresh) = package.get("receipt_hash_refresh") else {
        return (0, 0, 1);
    };

    if json_text(refresh, &["schema"]) != Some("dx.forge.package.receipt_hash_refresh") {
        return (0, 0, 1);
    }

    let stale_files = json_u64(refresh, &["stale_file_count"]).unwrap_or(0);
    let missing_files = json_u64(refresh, &["missing_file_count"]).unwrap_or(0);
    let status = json_text(refresh, &["status"]).unwrap_or("missing");
    let stale = u64::from(status == "stale" || stale_files > 0);
    let missing = u64::from(status == "missing" || missing_files > 0);
    let current = u64::from(status == "current" && stale == 0 && missing == 0);

    (current, stale, missing)
}

fn json_u64(value: &serde_json::Value, path: &[&str]) -> Option<u64> {
    let mut current = value;
    for key in path {
        current = current.get(*key)?;
    }
    current.as_u64()
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
        "database-orm-missing-package-status",
        format!("{DATABASE_ORM_OFFICIAL_NAME} is missing from package-status visibility"),
        Some(DATABASE_ORM_PACKAGE_STATUS.to_string()),
        "Regenerate .dx/forge/package-status.json so dx-check can report Database ORM visibility states.",
    )
}

fn missing_receipt_finding(receipt_path: &str) -> DxCheckFinding {
    check_finding(
        DxSupplyChainSeverity::Medium,
        "database-orm-missing-receipt",
        format!("{DATABASE_ORM_OFFICIAL_NAME} is missing its package-lane receipt"),
        Some(receipt_path.to_string()),
        "Regenerate or restore the Database ORM dashboard workflow receipt so dx-check can report source-owned package visibility.",
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
    use std::{fs, path::Path};

    use super::*;

    #[test]
    fn database_orm_dx_style_compatibility_missing_is_reported() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_database_orm_receipt(dir.path());
        write_database_orm_package_status(dir.path(), true);

        let manifest = DxSourceManifest::default();
        let (present_metrics, present_findings) =
            forge_database_orm_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(
                &present_metrics,
                "database_orm_dx_style_compatibility_present"
            ),
            Some(1)
        );
        assert_eq!(
            metric_value(
                &present_metrics,
                "database_orm_dx_style_compatibility_missing"
            ),
            Some(0)
        );
        assert!(
            !present_findings
                .iter()
                .any(|finding| finding.code == "database-orm-missing-dx-style-compatibility")
        );

        write_database_orm_package_status(dir.path(), false);

        let (missing_metrics, missing_findings) =
            forge_database_orm_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(
                &missing_metrics,
                "database_orm_dx_style_compatibility_present"
            ),
            Some(0)
        );
        assert_eq!(
            metric_value(
                &missing_metrics,
                "database_orm_dx_style_compatibility_missing"
            ),
            Some(1)
        );
        assert_eq!(
            metric_value(&missing_metrics, "database_orm_receipt_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(&missing_metrics, "database_orm_missing_receipt"),
            Some(0)
        );
        assert!(
            missing_findings
                .iter()
                .any(|finding| finding.code == "database-orm-missing-dx-style-compatibility")
        );
    }

    #[test]
    fn database_orm_hash_refresh_stale_helper_keeps_source_hash_clean() {
        let dir = tempfile::tempdir().expect("tempdir");
        let source_path = dir.path().join("examples/template/drizzle-query-proof.tsx");
        fs::create_dir_all(source_path.parent().expect("source parent")).expect("source directory");
        fs::write(
            &source_path,
            b"export const stableDatabaseOrmWorkflow = true;\n",
        )
        .expect("write source");
        let source_hash =
            sha256_project_file(dir.path(), "examples/template/drizzle-query-proof.tsx")
                .expect("hash source");

        write_database_orm_receipt(dir.path());
        write_database_orm_package_status_with_refresh(
            dir.path(),
            true,
            "current",
            0,
            0,
            Some(&source_hash),
        );

        let manifest = DxSourceManifest::default();
        let (current_metrics, current_findings) =
            forge_database_orm_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(
                &current_metrics,
                "database_orm_receipt_hash_refresh_current"
            ),
            Some(1)
        );
        assert_eq!(
            metric_value(&current_metrics, "database_orm_receipt_hash_refresh_stale"),
            Some(0)
        );
        assert_eq!(
            metric_value(
                &current_metrics,
                "database_orm_receipt_hash_refresh_missing"
            ),
            Some(0)
        );
        assert_eq!(
            metric_value(&current_metrics, "database_orm_hash_mismatch"),
            Some(0)
        );
        assert_eq!(
            metric_value(&current_metrics, "database_orm_receipt_stale"),
            Some(0)
        );
        assert!(
            !current_findings
                .iter()
                .any(|finding| finding.code == "database-orm-stale-receipt")
        );

        write_database_orm_package_status_with_refresh(
            dir.path(),
            true,
            "stale",
            1,
            0,
            Some(&source_hash),
        );

        let (stale_metrics, stale_findings) =
            forge_database_orm_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(&stale_metrics, "database_orm_receipt_hash_refresh_current"),
            Some(0)
        );
        assert_eq!(
            metric_value(&stale_metrics, "database_orm_receipt_hash_refresh_stale"),
            Some(1)
        );
        assert_eq!(
            metric_value(&stale_metrics, "database_orm_hash_mismatch"),
            Some(0)
        );
        assert_eq!(
            metric_value(&stale_metrics, "database_orm_receipt_stale"),
            Some(1)
        );
        assert!(
            stale_findings
                .iter()
                .any(|finding| finding.code == "database-orm-stale-receipt")
        );
        assert!(
            !stale_findings
                .iter()
                .any(|finding| finding.code == "database-orm-hash-mismatch")
        );
    }

    fn write_database_orm_receipt(root: &Path) {
        let receipt_path = root.join(DATABASE_ORM_DASHBOARD_RECEIPT);
        fs::create_dir_all(receipt_path.parent().expect("receipt parent"))
            .expect("receipt directory");
        fs::write(
            receipt_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.forge.database_orm.receipt",
                "official_package_name": DATABASE_ORM_OFFICIAL_NAME,
                "package_id": DATABASE_ORM_PACKAGE_ID,
                "surface": "launch-drizzle-data-workflow"
            }))
            .expect("receipt json"),
        )
        .expect("write receipt");
    }

    fn write_database_orm_package_status(root: &Path, include_dx_style: bool) {
        write_database_orm_package_status_with_refresh(
            root,
            include_dx_style,
            "current",
            0,
            0,
            None,
        );
    }

    fn write_database_orm_package_status_with_refresh(
        root: &Path,
        include_dx_style: bool,
        refresh_status: &str,
        stale_file_count: u64,
        missing_file_count: u64,
        source_hash: Option<&str>,
    ) {
        let package_status_path = root.join(DATABASE_ORM_PACKAGE_STATUS);
        fs::create_dir_all(package_status_path.parent().expect("package status parent"))
            .expect("package status directory");

        let mut selected_surface = serde_json::json!({
            "surface_id": "drizzle-launch-dashboard-workflow",
            "status": "present",
            "receipt_path": DATABASE_ORM_DASHBOARD_RECEIPT,
            "files": ["components/launch/drizzle-query-proof.tsx"],
            "source_markers": [
                "data-dx-package=\"db/drizzle-sqlite\"",
                "data-dx-style-surface=\"database-orm\""
            ]
        });
        if let Some(source_hash) = source_hash {
            selected_surface["hash_algorithm"] = serde_json::json!("sha256");
            selected_surface["file_hashes"] = serde_json::json!({
                "examples/template/drizzle-query-proof.tsx": source_hash
            });
        }

        let mut visibility = serde_json::json!({
            "official_package_name": DATABASE_ORM_OFFICIAL_NAME,
            "package_id": DATABASE_ORM_PACKAGE_ID,
            "upstream_package": "drizzle-orm",
            "upstream_version": "0.45.3",
            "source_mirror": "G:/WWW/inspirations/drizzle-orm",
            "status": "present",
            "receipt_status": "present",
            "package_receipt_path": DATABASE_ORM_DASHBOARD_RECEIPT,
            "selected_surfaces": [selected_surface],
            "receipt_hash_refresh": {
                "schema": "dx.forge.package.receipt_hash_refresh",
                "helper_id": "database-orm:receipt-hash-refresh",
                "status": refresh_status,
                "stale_file_count": stale_file_count,
                "missing_file_count": missing_file_count
            },
            "blocked_surfaces": [],
            "unsupported_surfaces": []
        });

        if include_dx_style {
            visibility["dx_style_compatibility"] = serde_json::json!({
                "schema": "dx.forge.package.dx_style_compatibility",
                "status": "present",
                "token_source": "tools/launch/runtime-template/assets/launch-runtime.css",
                "generated_css": "tools/launch/runtime-template/assets/launch-runtime.css",
                "visible_surfaces": ["launch-drizzle-data-workflow"],
                "source_files": ["components/launch/drizzle-query-proof.tsx"],
                "runtime_proof": false,
                "runtime_limitations": [
                    "SOURCE-ONLY: no live SQLite visual proof is claimed."
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
