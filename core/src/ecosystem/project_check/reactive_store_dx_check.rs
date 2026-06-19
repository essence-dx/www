#![allow(clippy::too_many_arguments)]
use std::path::Path;

use super::super::forge_security::{DxSourceManifest, DxSupplyChainSeverity};
use super::file_hashes::count_sha256_file_hash_mismatches;
use super::{
    DxCheckFinding, DxCheckMetric, check_finding, check_metric, json_array_entries, json_text,
    read_optional_forge_json, resolve_dx_check_relative_path,
};

const REACTIVE_STORE_PACKAGE_ID: &str = "reactive/store";
const REACTIVE_STORE_OFFICIAL_NAME: &str = "Reactive Store";
const REACTIVE_STORE_PACKAGE_STATUS: &str = ".dx/forge/package-status.json";
const REACTIVE_STORE_PACKAGE_RECEIPT: &str = ".dx/forge/receipts/packages/reactive-store.json";

pub(super) fn forge_reactive_store_package_metrics(
    root: &Path,
    manifest: &DxSourceManifest,
) -> (Vec<DxCheckMetric>, Vec<DxCheckFinding>) {
    let mut findings = Vec::new();
    let manifest_package_present = manifest
        .packages
        .iter()
        .any(|package| package.package_id == REACTIVE_STORE_PACKAGE_ID);
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
        REACTIVE_STORE_PACKAGE_STATUS,
        "reactive-store-package-status-invalid",
        &mut findings,
    ) else {
        if manifest_package_present || package_receipt_exists(root, REACTIVE_STORE_PACKAGE_RECEIPT)
        {
            package_present = 1;
            missing_receipt = 1;
            findings.push(missing_package_status_finding());
        }
        return (
            reactive_store_metrics(
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
        .find(|entry| json_text(entry, &["package_id"]) == Some(REACTIVE_STORE_PACKAGE_ID))
    else {
        if manifest_package_present || package_receipt_exists(root, REACTIVE_STORE_PACKAGE_RECEIPT)
        {
            package_present = 1;
            missing_receipt = 1;
            findings.push(missing_package_status_finding());
        }
        return (
            reactive_store_metrics(
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
    let (refresh_current, refresh_stale, refresh_missing) = receipt_hash_refresh_counts(visibility);
    receipt_hash_refresh_current = refresh_current;
    receipt_hash_refresh_stale = refresh_stale;
    receipt_hash_refresh_missing = refresh_missing;
    if refresh_stale > 0 {
        stale_receipt = 1;
    }

    let package_receipt_path =
        json_text(visibility, &["package_receipt_path"]).unwrap_or(REACTIVE_STORE_PACKAGE_RECEIPT);
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

    if hash_mismatches > 0 {
        stale_receipt = 1;
    }

    if stale_receipt > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "reactive-store-stale-receipt",
            format!("{REACTIVE_STORE_OFFICIAL_NAME} package-status visibility is stale"),
            Some(REACTIVE_STORE_PACKAGE_STATUS.to_string()),
            "Regenerate the Reactive Store package-status row from the package receipt before claiming source freshness.",
        ));
    }
    if blocked_surfaces > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "reactive-store-blocked-surface",
            format!(
                "{REACTIVE_STORE_OFFICIAL_NAME} has {blocked_surfaces} blocked selected surface(s)"
            ),
            Some(REACTIVE_STORE_PACKAGE_STATUS.to_string()),
            "Resolve the SOURCE-ONLY or ADAPTER-BOUNDARY Reactive Store runtime proof boundary before claiming live React readiness.",
        ));
    }
    if unsupported_surfaces > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "reactive-store-unsupported-surface",
            format!(
                "{REACTIVE_STORE_OFFICIAL_NAME} has {unsupported_surfaces} unsupported requested surface(s)"
            ),
            Some(REACTIVE_STORE_PACKAGE_STATUS.to_string()),
            "Request only supported Reactive Store core-store, atom-graph, comparison-helper, react-selector, or react-context surfaces, or add a real upstream-backed Forge surface first.",
        ));
    }
    if hash_mismatches > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "reactive-store-hash-mismatch",
            format!(
                "{REACTIVE_STORE_OFFICIAL_NAME} has {hash_mismatches} missing or stale hash-backed selected file(s)"
            ),
            Some(REACTIVE_STORE_PACKAGE_STATUS.to_string()),
            "Regenerate the Reactive Store receipt after reviewing the changed front-facing reactive store files.",
        ));
    }

    (
        reactive_store_metrics(
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

fn reactive_store_metrics(
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
        check_metric("reactive_store_package_present", package_present),
        check_metric("reactive_store_receipt_present", receipt_present),
        check_metric("reactive_store_receipt_stale", stale_receipt),
        check_metric("reactive_store_missing_receipt", missing_receipt),
        check_metric("reactive_store_blocked_surface", blocked_surfaces),
        check_metric("reactive_store_unsupported_surface", unsupported_surfaces),
        check_metric(
            "reactive_store_hash_manifest_present",
            hash_manifest_present,
        ),
        check_metric("reactive_store_hash_mismatch", hash_mismatches),
        check_metric(
            "reactive_store_receipt_hash_refresh_current",
            receipt_hash_refresh_current,
        ),
        check_metric(
            "reactive_store_receipt_hash_refresh_stale",
            receipt_hash_refresh_stale,
        ),
        check_metric(
            "reactive_store_receipt_hash_refresh_missing",
            receipt_hash_refresh_missing,
        ),
    ]
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

fn missing_package_status_finding() -> DxCheckFinding {
    check_finding(
        DxSupplyChainSeverity::Medium,
        "reactive-store-missing-package-status",
        format!("{REACTIVE_STORE_OFFICIAL_NAME} is missing from package-status visibility"),
        Some(REACTIVE_STORE_PACKAGE_STATUS.to_string()),
        "Regenerate .dx/forge/package-status.json so dx-check can report Reactive Store visibility states.",
    )
}

fn missing_receipt_finding(receipt_path: &str) -> DxCheckFinding {
    check_finding(
        DxSupplyChainSeverity::Medium,
        "reactive-store-missing-receipt",
        format!("{REACTIVE_STORE_OFFICIAL_NAME} is missing its package-lane receipt"),
        Some(receipt_path.to_string()),
        "Regenerate or restore the Reactive Store package receipt so dx-check can report source-owned package visibility without claiming live React runtime proof.",
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
    use super::*;
    use sha2::{Digest, Sha256};
    use std::{fs, path::Path};

    #[test]
    fn reactive_store_hash_mismatch_flips_when_selected_file_changes() {
        let dir = tempfile::tempdir().expect("tempdir");
        let reactive_store_dir = dir.path().join("lib/forge/state/reactive-store");
        fs::create_dir_all(&reactive_store_dir).expect("reactive store dir");
        let context_file = reactive_store_dir.join("context.tsx");
        fs::write(
            &context_file,
            b"export const stableReactiveStoreContext = true;\n",
        )
        .expect("context source");

        let expected_hash = sha256_file(&context_file);
        write_reactive_store_receipt(dir.path());
        write_reactive_store_package_status(dir.path(), &expected_hash);

        let manifest = DxSourceManifest::default();
        let (metrics, findings) = forge_reactive_store_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(&metrics, "reactive_store_hash_manifest_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(&metrics, "reactive_store_hash_mismatch"),
            Some(0)
        );
        assert_eq!(
            metric_value(&metrics, "reactive_store_receipt_stale"),
            Some(0)
        );
        assert!(
            !findings
                .iter()
                .any(|finding| finding.code == "reactive-store-hash-mismatch")
        );

        fs::write(
            &context_file,
            b"export const staleReactiveStoreContext = true;\n",
        )
        .expect("stale context source");
        let (stale_metrics, stale_findings) =
            forge_reactive_store_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(&stale_metrics, "reactive_store_hash_mismatch"),
            Some(1)
        );
        assert_eq!(
            metric_value(&stale_metrics, "reactive_store_receipt_stale"),
            Some(1)
        );
        assert!(
            stale_findings
                .iter()
                .any(|finding| finding.code == "reactive-store-hash-mismatch")
        );
    }

    #[test]
    fn reactive_store_hash_refresh_stale_helper_keeps_source_hash_clean() {
        let dir = tempfile::tempdir().expect("tempdir");
        let reactive_store_dir = dir.path().join("lib/forge/state/reactive-store");
        fs::create_dir_all(&reactive_store_dir).expect("reactive store dir");
        let context_file = reactive_store_dir.join("context.tsx");
        fs::write(
            &context_file,
            b"export const stableReactiveStoreContext = true;\n",
        )
        .expect("context source");

        let expected_hash = sha256_file(&context_file);
        write_reactive_store_receipt(dir.path());
        write_reactive_store_package_status(dir.path(), &expected_hash);

        let manifest = DxSourceManifest::default();
        let (metrics, _) = forge_reactive_store_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(&metrics, "reactive_store_receipt_hash_refresh_current"),
            Some(1)
        );
        assert_eq!(
            metric_value(&metrics, "reactive_store_receipt_hash_refresh_stale"),
            Some(0)
        );
        assert_eq!(
            metric_value(&metrics, "reactive_store_receipt_hash_refresh_missing"),
            Some(0)
        );
        assert_eq!(
            metric_value(&metrics, "reactive_store_hash_mismatch"),
            Some(0)
        );

        write_reactive_store_package_status_with_refresh(dir.path(), &expected_hash, "stale", 1, 0);
        let (helper_stale_metrics, helper_stale_findings) =
            forge_reactive_store_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(
                &helper_stale_metrics,
                "reactive_store_receipt_hash_refresh_stale"
            ),
            Some(1)
        );
        assert_eq!(
            metric_value(&helper_stale_metrics, "reactive_store_hash_mismatch"),
            Some(0)
        );
        assert_eq!(
            metric_value(&helper_stale_metrics, "reactive_store_receipt_stale"),
            Some(1)
        );
        assert!(
            helper_stale_findings
                .iter()
                .any(|finding| finding.code == "reactive-store-stale-receipt")
        );
        assert!(
            !helper_stale_findings
                .iter()
                .any(|finding| finding.code == "reactive-store-hash-mismatch")
        );
    }

    fn write_reactive_store_receipt(root: &Path) {
        let receipt_path = root.join(REACTIVE_STORE_PACKAGE_RECEIPT);
        fs::create_dir_all(receipt_path.parent().expect("receipt parent"))
            .expect("receipt directory");
        fs::write(
            receipt_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.forge.reactive_store_receipt",
                "official_package_name": REACTIVE_STORE_OFFICIAL_NAME,
                "package_id": REACTIVE_STORE_PACKAGE_ID
            }))
            .expect("receipt json"),
        )
        .expect("write receipt");
    }

    fn write_reactive_store_package_status(root: &Path, expected_hash: &str) {
        write_reactive_store_package_status_with_refresh(root, expected_hash, "current", 0, 0);
    }

    fn write_reactive_store_package_status_with_refresh(
        root: &Path,
        expected_hash: &str,
        refresh_status: &str,
        stale_file_count: u64,
        missing_file_count: u64,
    ) {
        let package_status_path = root.join(REACTIVE_STORE_PACKAGE_STATUS);
        fs::create_dir_all(package_status_path.parent().expect("package status parent"))
            .expect("package status directory");
        fs::write(
            package_status_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.www.template.forge_package_status",
                "package_lane_visibility": [
                    {
                        "official_package_name": REACTIVE_STORE_OFFICIAL_NAME,
                        "package_id": REACTIVE_STORE_PACKAGE_ID,
                        "upstream_package": "@tanstack/store",
                        "based_on": "@tanstack/react-store",
                        "source_mirror": "G:/WWW/inspirations/tanstack-store",
                        "status": "present",
                        "receipt_status": "present",
                        "package_receipt_path": REACTIVE_STORE_PACKAGE_RECEIPT,
                        "receipt_hash_refresh": {
                            "schema": "dx.forge.package.receipt_hash_refresh",
                            "helper_id": "reactive-store:receipt-hash-refresh",
                            "status": refresh_status,
                            "stale_file_count": stale_file_count,
                            "missing_file_count": missing_file_count
                        },
                        "selected_surfaces": [
                            {
                                "surface_id": "reactive-store:react-context",
                                "status": "present",
                                "hash_algorithm": "sha256",
                                "file_hashes": {
                                    "lib/forge/state/reactive-store/context.tsx": expected_hash
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
