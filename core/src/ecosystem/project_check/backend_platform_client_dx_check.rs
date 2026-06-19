#![allow(clippy::too_many_arguments)]
use std::path::Path;

use super::super::forge_security::{DxSourceManifest, DxSupplyChainSeverity};
use super::file_hashes::count_sha256_file_hash_mismatches;
use super::{
    DxCheckFinding, DxCheckMetric, check_finding, check_metric, json_array_entries, json_text,
    read_optional_forge_json, resolve_dx_check_relative_path,
};

const BACKEND_PLATFORM_CLIENT_PACKAGE_ID: &str = "supabase/client";
const BACKEND_PLATFORM_CLIENT_OFFICIAL_NAME: &str = "Backend Platform Client";
const BACKEND_PLATFORM_CLIENT_PACKAGE_STATUS: &str = ".dx/forge/package-status.json";
const BACKEND_PLATFORM_CLIENT_DASHBOARD_RECEIPT: &str =
    "examples/template/.dx/forge/receipts/2026-05-22-supabase-client-dashboard-workflow.json";

pub(super) fn forge_backend_platform_client_package_metrics(
    root: &Path,
    manifest: &DxSourceManifest,
) -> (Vec<DxCheckMetric>, Vec<DxCheckFinding>) {
    let mut findings = Vec::new();
    let manifest_package_present = manifest
        .packages
        .iter()
        .any(|package| package.package_id == BACKEND_PLATFORM_CLIENT_PACKAGE_ID);
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
        BACKEND_PLATFORM_CLIENT_PACKAGE_STATUS,
        "backend-platform-client-package-status-invalid",
        &mut findings,
    ) else {
        if manifest_package_present
            || package_receipt_exists(root, BACKEND_PLATFORM_CLIENT_DASHBOARD_RECEIPT)
        {
            package_present = 1;
            missing_receipt = 1;
            receipt_hash_refresh_missing = 1;
            dx_style_compatibility_missing = 1;
            findings.push(missing_package_status_finding());
        }
        return (
            backend_platform_client_metrics(
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
        .find(|entry| {
            json_text(entry, &["package_id"]) == Some(BACKEND_PLATFORM_CLIENT_PACKAGE_ID)
        })
    else {
        if manifest_package_present
            || package_receipt_exists(root, BACKEND_PLATFORM_CLIENT_DASHBOARD_RECEIPT)
        {
            package_present = 1;
            missing_receipt = 1;
            receipt_hash_refresh_missing = 1;
            dx_style_compatibility_missing = 1;
            findings.push(missing_package_status_finding());
        }
        return (
            backend_platform_client_metrics(
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
    dx_style_compatibility_present = u64::from(dx_style_compatibility_is_present(visibility));
    dx_style_compatibility_missing = u64::from(dx_style_compatibility_present == 0);

    let package_receipt_path = json_text(visibility, &["package_receipt_path"])
        .unwrap_or(BACKEND_PLATFORM_CLIENT_DASHBOARD_RECEIPT);
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
            "backend-platform-client-stale-receipt",
            format!("{BACKEND_PLATFORM_CLIENT_OFFICIAL_NAME} package-status visibility is stale"),
            Some(BACKEND_PLATFORM_CLIENT_PACKAGE_STATUS.to_string()),
            "Regenerate the Backend Platform Client package-status row from the dashboard workflow receipt before claiming source freshness.",
        ));
    }
    if blocked_surfaces > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "backend-platform-client-blocked-surface",
            format!(
                "{BACKEND_PLATFORM_CLIENT_OFFICIAL_NAME} has {blocked_surfaces} app-owned runtime boundary surface(s)"
            ),
            Some(BACKEND_PLATFORM_CLIENT_PACKAGE_STATUS.to_string()),
            "Resolve the ADAPTER-BOUNDARY hosted Supabase credentials, reads, writes, realtime authorization, provider setup, and RLS rollout before claiming hosted Supabase runtime proof.",
        ));
    }
    if unsupported_surfaces > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "backend-platform-client-unsupported-surface",
            format!(
                "{BACKEND_PLATFORM_CLIENT_OFFICIAL_NAME} has {unsupported_surfaces} unsupported requested surface(s)"
            ),
            Some(BACKEND_PLATFORM_CLIENT_PACKAGE_STATUS.to_string()),
            "Request only supported Backend Platform Client profile, schema-query, Auth, Storage, Realtime, RPC, or Edge Function surfaces, or add a real upstream-backed Forge surface first.",
        ));
    }
    if hash_mismatches > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "backend-platform-client-hash-mismatch",
            format!(
                "{BACKEND_PLATFORM_CLIENT_OFFICIAL_NAME} has {hash_mismatches} missing or stale hash-backed source file(s)"
            ),
            Some(BACKEND_PLATFORM_CLIENT_PACKAGE_STATUS.to_string()),
            "Regenerate the Backend Platform Client dashboard workflow receipt after reviewing the changed front-facing Supabase files.",
        ));
    }
    if dx_style_compatibility_missing > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "backend-platform-client-missing-dx-style-compatibility",
            format!(
                "{BACKEND_PLATFORM_CLIENT_OFFICIAL_NAME} is missing dx-style compatibility metadata"
            ),
            Some(BACKEND_PLATFORM_CLIENT_PACKAGE_STATUS.to_string()),
            "Restore the Backend Platform Client dx_style_compatibility block and data-dx-style-surface markers before claiming visible UI style compatibility; hosted Supabase runtime proof remains app-owned.",
        ));
    }

    (
        backend_platform_client_metrics(
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

fn backend_platform_client_metrics(
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
        check_metric("backend_platform_client_package_present", package_present),
        check_metric("backend_platform_client_receipt_present", receipt_present),
        check_metric("backend_platform_client_receipt_stale", stale_receipt),
        check_metric("backend_platform_client_missing_receipt", missing_receipt),
        check_metric("backend_platform_client_blocked_surface", blocked_surfaces),
        check_metric(
            "backend_platform_client_unsupported_surface",
            unsupported_surfaces,
        ),
        check_metric(
            "backend_platform_client_hash_manifest_present",
            hash_manifest_present,
        ),
        check_metric("backend_platform_client_hash_mismatch", hash_mismatches),
        check_metric(
            "backend_platform_client_receipt_hash_refresh_current",
            receipt_hash_refresh_current,
        ),
        check_metric(
            "backend_platform_client_receipt_hash_refresh_stale",
            receipt_hash_refresh_stale,
        ),
        check_metric(
            "backend_platform_client_receipt_hash_refresh_missing",
            receipt_hash_refresh_missing,
        ),
        check_metric(
            "backend_platform_client_dx_style_compatibility_present",
            dx_style_compatibility_present,
        ),
        check_metric(
            "backend_platform_client_dx_style_compatibility_missing",
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
        "backend-platform-client-missing-package-status",
        format!(
            "{BACKEND_PLATFORM_CLIENT_OFFICIAL_NAME} is missing from package-status visibility"
        ),
        Some(BACKEND_PLATFORM_CLIENT_PACKAGE_STATUS.to_string()),
        "Regenerate .dx/forge/package-status.json so dx-check can report Backend Platform Client visibility states.",
    )
}

fn missing_receipt_finding(receipt_path: &str) -> DxCheckFinding {
    check_finding(
        DxSupplyChainSeverity::Medium,
        "backend-platform-client-missing-receipt",
        format!("{BACKEND_PLATFORM_CLIENT_OFFICIAL_NAME} is missing its package-lane receipt"),
        Some(receipt_path.to_string()),
        "Regenerate or restore the Backend Platform Client dashboard workflow receipt so dx-check can report source-owned package visibility without claiming hosted Supabase runtime proof.",
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

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use sha2::{Digest, Sha256};

    use super::*;

    #[test]
    fn backend_platform_client_hash_mismatch_metric_and_finding_are_byte_derived() {
        let dir = tempfile::tempdir().expect("tempdir");
        let receipt_path = dir.path().join(BACKEND_PLATFORM_CLIENT_DASHBOARD_RECEIPT);
        let receipt_dir = receipt_path.parent().expect("receipt dir");
        fs::create_dir_all(receipt_dir).expect("receipt dir");
        fs::write(&receipt_path, "{}").expect("backend platform client receipt");

        let surface_path = dir
            .path()
            .join("examples/template/supabase-profile-workflow.tsx");
        let surface_dir = surface_path.parent().expect("surface dir");
        fs::create_dir_all(surface_dir).expect("surface dir");
        fs::write(
            &surface_path,
            "export function LaunchSupabaseProfileWorkflow() { return null; }\n",
        )
        .expect("backend platform client surface");

        let expected_hash = sha256_project_file(
            dir.path(),
            "examples/template/supabase-profile-workflow.tsx",
        )
        .expect("backend platform client hash");
        write_backend_platform_client_package_status(dir.path(), &expected_hash, true);

        let manifest = DxSourceManifest::default();
        let (metrics, findings) =
            forge_backend_platform_client_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(&metrics, "backend_platform_client_hash_manifest_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(&metrics, "backend_platform_client_receipt_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(&metrics, "backend_platform_client_receipt_stale"),
            Some(0)
        );
        assert_eq!(
            metric_value(&metrics, "backend_platform_client_hash_mismatch"),
            Some(0)
        );
        assert!(
            !findings
                .iter()
                .any(|finding| finding.code == "backend-platform-client-hash-mismatch")
        );

        fs::write(
            &surface_path,
            "export function LaunchSupabaseProfileWorkflow() { return <section />; }\n",
        )
        .expect("mutate backend platform client surface");
        let (metrics, findings) =
            forge_backend_platform_client_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(&metrics, "backend_platform_client_receipt_stale"),
            Some(1)
        );
        assert_eq!(
            metric_value(&metrics, "backend_platform_client_hash_mismatch"),
            Some(1)
        );
        assert!(
            findings
                .iter()
                .any(|finding| finding.code == "backend-platform-client-hash-mismatch")
        );
    }

    #[test]
    fn backend_platform_client_dx_style_missing_metric_and_finding_flip() {
        let dir = tempfile::tempdir().expect("tempdir");
        let receipt_path = dir.path().join(BACKEND_PLATFORM_CLIENT_DASHBOARD_RECEIPT);
        let receipt_dir = receipt_path.parent().expect("receipt dir");
        fs::create_dir_all(receipt_dir).expect("receipt dir");
        fs::write(&receipt_path, "{}").expect("backend platform client receipt");

        let surface_path = dir
            .path()
            .join("examples/template/supabase-profile-workflow.tsx");
        let surface_dir = surface_path.parent().expect("surface dir");
        fs::create_dir_all(surface_dir).expect("surface dir");
        fs::write(
            &surface_path,
            "export function LaunchSupabaseProfileWorkflow() { return null; }\n",
        )
        .expect("backend platform client surface");

        let expected_hash = sha256_project_file(
            dir.path(),
            "examples/template/supabase-profile-workflow.tsx",
        )
        .expect("backend platform client hash");
        write_backend_platform_client_package_status(dir.path(), &expected_hash, true);

        let manifest = DxSourceManifest::default();
        let (metrics, findings) =
            forge_backend_platform_client_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(
                &metrics,
                "backend_platform_client_dx_style_compatibility_present"
            ),
            Some(1)
        );
        assert_eq!(
            metric_value(
                &metrics,
                "backend_platform_client_dx_style_compatibility_missing"
            ),
            Some(0)
        );
        assert!(!findings.iter().any(|finding| {
            finding.code == "backend-platform-client-missing-dx-style-compatibility"
        }));

        write_backend_platform_client_package_status(dir.path(), &expected_hash, false);
        let (missing_metrics, missing_findings) =
            forge_backend_platform_client_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(
                &missing_metrics,
                "backend_platform_client_dx_style_compatibility_present"
            ),
            Some(0)
        );
        assert_eq!(
            metric_value(
                &missing_metrics,
                "backend_platform_client_dx_style_compatibility_missing"
            ),
            Some(1)
        );
        assert!(missing_findings.iter().any(|finding| {
            finding.code == "backend-platform-client-missing-dx-style-compatibility"
        }));
    }

    #[test]
    fn backend_platform_client_hash_refresh_stale_helper_keeps_source_hash_clean() {
        let dir = tempfile::tempdir().expect("tempdir");
        let receipt_path = dir.path().join(BACKEND_PLATFORM_CLIENT_DASHBOARD_RECEIPT);
        let receipt_dir = receipt_path.parent().expect("receipt dir");
        fs::create_dir_all(receipt_dir).expect("receipt dir");
        fs::write(&receipt_path, "{}").expect("backend platform client receipt");

        let surface_path = dir
            .path()
            .join("examples/template/supabase-profile-workflow.tsx");
        let surface_dir = surface_path.parent().expect("surface dir");
        fs::create_dir_all(surface_dir).expect("surface dir");
        fs::write(
            &surface_path,
            "export function LaunchSupabaseProfileWorkflow() { return null; }\n",
        )
        .expect("backend platform client surface");

        let expected_hash = sha256_project_file(
            dir.path(),
            "examples/template/supabase-profile-workflow.tsx",
        )
        .expect("backend platform client hash");
        write_backend_platform_client_package_status(dir.path(), &expected_hash, true);

        let manifest = DxSourceManifest::default();
        let (current_metrics, current_findings) =
            forge_backend_platform_client_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(
                &current_metrics,
                "backend_platform_client_receipt_hash_refresh_current"
            ),
            Some(1)
        );
        assert_eq!(
            metric_value(
                &current_metrics,
                "backend_platform_client_receipt_hash_refresh_stale"
            ),
            Some(0)
        );
        assert_eq!(
            metric_value(
                &current_metrics,
                "backend_platform_client_receipt_hash_refresh_missing"
            ),
            Some(0)
        );
        assert_eq!(
            metric_value(&current_metrics, "backend_platform_client_hash_mismatch"),
            Some(0)
        );
        assert_eq!(
            metric_value(&current_metrics, "backend_platform_client_receipt_stale"),
            Some(0)
        );
        assert!(
            !current_findings
                .iter()
                .any(|finding| { finding.code == "backend-platform-client-stale-receipt" })
        );

        write_backend_platform_client_package_status_with_refresh(
            dir.path(),
            &expected_hash,
            true,
            "stale",
            1,
            0,
        );
        let (stale_metrics, stale_findings) =
            forge_backend_platform_client_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(
                &stale_metrics,
                "backend_platform_client_receipt_hash_refresh_current"
            ),
            Some(0)
        );
        assert_eq!(
            metric_value(
                &stale_metrics,
                "backend_platform_client_receipt_hash_refresh_stale"
            ),
            Some(1)
        );
        assert_eq!(
            metric_value(
                &stale_metrics,
                "backend_platform_client_receipt_hash_refresh_missing"
            ),
            Some(0)
        );
        assert_eq!(
            metric_value(&stale_metrics, "backend_platform_client_hash_mismatch"),
            Some(0)
        );
        assert_eq!(
            metric_value(&stale_metrics, "backend_platform_client_receipt_stale"),
            Some(1)
        );
        assert!(
            stale_findings
                .iter()
                .any(|finding| finding.code == "backend-platform-client-stale-receipt")
        );
        assert!(
            !stale_findings
                .iter()
                .any(|finding| finding.code == "backend-platform-client-hash-mismatch")
        );
    }

    fn write_backend_platform_client_package_status(
        root: &Path,
        expected_hash: &str,
        include_dx_style: bool,
    ) {
        write_backend_platform_client_package_status_with_refresh(
            root,
            expected_hash,
            include_dx_style,
            "current",
            0,
            0,
        );
    }

    fn write_backend_platform_client_package_status_with_refresh(
        root: &Path,
        expected_hash: &str,
        include_dx_style: bool,
        refresh_status: &str,
        stale_file_count: u64,
        missing_file_count: u64,
    ) {
        let package_status_path = root.join(BACKEND_PLATFORM_CLIENT_PACKAGE_STATUS);
        fs::create_dir_all(package_status_path.parent().expect("package status dir"))
            .expect("package status dir");
        let mut visibility = serde_json::json!({
            "official_package_name": BACKEND_PLATFORM_CLIENT_OFFICIAL_NAME,
            "package_id": BACKEND_PLATFORM_CLIENT_PACKAGE_ID,
            "upstream_package": "@supabase/ssr + @supabase/supabase-js",
            "upstream_version": "@supabase/ssr latest; @supabase/supabase-js ^2",
            "source_mirror": "G:/WWW/inspirations/supabase",
            "status": "present",
            "receipt_status": "present",
            "package_receipt_path": BACKEND_PLATFORM_CLIENT_DASHBOARD_RECEIPT,
            "selected_surfaces": [
                {
                    "surface_id": "supabase-profile-workflow",
                    "status": "present",
                    "hash_algorithm": "sha256",
                    "file_hashes": {
                        "examples/template/supabase-profile-workflow.tsx": format!("sha256:{expected_hash}")
                    }
                }
            ],
            "receipt_hash_refresh": {
                "schema": "dx.forge.package.receipt_hash_refresh",
                "status": refresh_status,
                "helper_path": "examples/template/backend-platform-client-receipt-hashes.ts",
                "check_command": "node examples/template/backend-platform-client-receipt-hashes.ts --check",
                "write_command": "node examples/template/backend-platform-client-receipt-hashes.ts --write",
                "json_check_command": "node examples/template/backend-platform-client-receipt-hashes.ts --check --json",
                "receipt_path": BACKEND_PLATFORM_CLIENT_DASHBOARD_RECEIPT,
                "hash_algorithm": "sha256",
                "tracked_file_count": 1,
                "stale_file_count": stale_file_count,
                "missing_file_count": missing_file_count,
                "runtime_execution": false,
                "secret_access": false,
                "zed_visibility": "backend-platform-client:receipt-hash-refresh"
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
                "visible_surfaces": [
                    "supabase-profile-workflow",
                    "supabase-schema-query-workflow"
                ],
                "source_files": [
                    "examples/template/supabase-profile-workflow.tsx",
                    "examples/template/data-status.tsx"
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
        .expect("package status");
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
