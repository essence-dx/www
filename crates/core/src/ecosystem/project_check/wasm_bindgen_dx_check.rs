#![allow(clippy::too_many_arguments)]
use std::path::Path;

use super::super::forge_security::{DxSourceManifest, DxSupplyChainSeverity};
use super::file_hashes::count_sha256_file_hash_mismatches;
use super::{
    DxCheckFinding, DxCheckMetric, check_finding, check_metric, json_array_entries, json_text,
    read_optional_forge_json, resolve_dx_check_relative_path,
};

const WEBASSEMBLY_BRIDGE_PACKAGE_ID: &str = "wasm/bindgen";
const WEBASSEMBLY_BRIDGE_OFFICIAL_NAME: &str = "WebAssembly Bridge";
const WEBASSEMBLY_BRIDGE_PACKAGE_STATUS: &str = ".dx/forge/package-status.json";
const WEBASSEMBLY_BRIDGE_DASHBOARD_RECEIPT: &str =
    ".dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json";

pub(super) fn forge_webassembly_bridge_package_metrics(
    root: &Path,
    manifest: &DxSourceManifest,
) -> (Vec<DxCheckMetric>, Vec<DxCheckFinding>) {
    let mut findings = Vec::new();
    let manifest_package_present = manifest
        .packages
        .iter()
        .any(|package| package.package_id == WEBASSEMBLY_BRIDGE_PACKAGE_ID);
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
        WEBASSEMBLY_BRIDGE_PACKAGE_STATUS,
        "webassembly-bridge-package-status-invalid",
        &mut findings,
    ) else {
        if manifest_package_present
            || package_receipt_exists(root, WEBASSEMBLY_BRIDGE_DASHBOARD_RECEIPT)
        {
            package_present = 1;
            missing_receipt = 1;
            receipt_hash_refresh_missing = 1;
            findings.push(missing_package_status_finding());
        }
        return (
            webassembly_bridge_metrics(
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
        .find(|entry| json_text(entry, &["package_id"]) == Some(WEBASSEMBLY_BRIDGE_PACKAGE_ID))
    else {
        if manifest_package_present
            || package_receipt_exists(root, WEBASSEMBLY_BRIDGE_DASHBOARD_RECEIPT)
        {
            package_present = 1;
            missing_receipt = 1;
            receipt_hash_refresh_missing = 1;
            findings.push(missing_package_status_finding());
        }
        return (
            webassembly_bridge_metrics(
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

    let package_receipt_path = json_text(visibility, &["package_receipt_path"])
        .unwrap_or(WEBASSEMBLY_BRIDGE_DASHBOARD_RECEIPT);
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

    if dx_style_compatibility_is_present(visibility) {
        dx_style_compatibility_present = 1;
    } else {
        dx_style_compatibility_missing = 1;
    }

    if stale_receipt > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "webassembly-bridge-stale-receipt",
            format!("{WEBASSEMBLY_BRIDGE_OFFICIAL_NAME} package-status visibility is stale"),
            Some(WEBASSEMBLY_BRIDGE_PACKAGE_STATUS.to_string()),
            "Regenerate the WebAssembly Bridge package-status row from the dashboard workflow receipt before claiming source freshness.",
        ));
    }
    if blocked_surfaces > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "webassembly-bridge-blocked-surface",
            format!(
                "{WEBASSEMBLY_BRIDGE_OFFICIAL_NAME} has {blocked_surfaces} blocked selected surface(s)"
            ),
            Some(WEBASSEMBLY_BRIDGE_PACKAGE_STATUS.to_string()),
            "Resolve the ADAPTER-BOUNDARY WebAssembly Bridge runtime ownership before claiming the selected surface is release-ready.",
        ));
    }
    if unsupported_surfaces > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "webassembly-bridge-unsupported-surface",
            format!(
                "{WEBASSEMBLY_BRIDGE_OFFICIAL_NAME} has {unsupported_surfaces} unsupported requested surface(s)"
            ),
            Some(WEBASSEMBLY_BRIDGE_PACKAGE_STATUS.to_string()),
            "Request only supported WebAssembly Bridge surfaces or add a real upstream-backed Forge surface first.",
        ));
    }
    if hash_mismatches > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "webassembly-bridge-hash-mismatch",
            format!(
                "{WEBASSEMBLY_BRIDGE_OFFICIAL_NAME} has {hash_mismatches} missing or stale hash-backed source file(s)"
            ),
            Some(WEBASSEMBLY_BRIDGE_PACKAGE_STATUS.to_string()),
            "Regenerate the WebAssembly Bridge dashboard receipt after reviewing the changed front-facing WebAssembly files.",
        ));
    }
    if dx_style_compatibility_missing > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "webassembly-bridge-missing-dx-style-compatibility",
            format!(
                "{WEBASSEMBLY_BRIDGE_OFFICIAL_NAME} is missing dx-style compatibility evidence"
            ),
            Some(WEBASSEMBLY_BRIDGE_PACKAGE_STATUS.to_string()),
            "Regenerate the WebAssembly Bridge dx-style compatibility row from the dashboard workflow receipt and verify source-owned style markers without claiming live generated-Wasm browser proof.",
        ));
    }

    (
        webassembly_bridge_metrics(
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

fn webassembly_bridge_metrics(
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
        check_metric("webassembly_bridge_package_present", package_present),
        check_metric("webassembly_bridge_receipt_present", receipt_present),
        check_metric("webassembly_bridge_receipt_stale", stale_receipt),
        check_metric("webassembly_bridge_missing_receipt", missing_receipt),
        check_metric("webassembly_bridge_blocked_surface", blocked_surfaces),
        check_metric(
            "webassembly_bridge_unsupported_surface",
            unsupported_surfaces,
        ),
        check_metric(
            "webassembly_bridge_hash_manifest_present",
            hash_manifest_present,
        ),
        check_metric("webassembly_bridge_hash_mismatch", hash_mismatches),
        check_metric(
            "webassembly_bridge_receipt_hash_refresh_current",
            receipt_hash_refresh_current,
        ),
        check_metric(
            "webassembly_bridge_receipt_hash_refresh_stale",
            receipt_hash_refresh_stale,
        ),
        check_metric(
            "webassembly_bridge_receipt_hash_refresh_missing",
            receipt_hash_refresh_missing,
        ),
        check_metric(
            "webassembly_bridge_dx_style_compatibility_present",
            dx_style_compatibility_present,
        ),
        check_metric(
            "webassembly_bridge_dx_style_compatibility_missing",
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

fn receipt_hash_refresh_counts(visibility: &serde_json::Value) -> (u64, u64, u64) {
    let Some(refresh) = visibility.get("receipt_hash_refresh") else {
        return (0, 0, 1);
    };

    if json_text(refresh, &["schema"]) != Some("dx.forge.package.receipt_hash_refresh") {
        return (0, 0, 1);
    }

    let stale_file_count = json_u64(refresh, "stale_file_count");
    let missing_file_count = json_u64(refresh, "missing_file_count");
    let stale_path_count = json_string_array(refresh, "stale_files").len() as u64
        + json_string_array(refresh, "stale_mirror_files").len() as u64;
    let missing_path_count = json_string_array(refresh, "missing_files").len() as u64
        + json_string_array(refresh, "missing_mirror_files").len() as u64;
    let status = json_text(refresh, &["status"]).unwrap_or("missing");
    let stale = u64::from(status == "stale" || stale_file_count > 0 || stale_path_count > 0);
    let missing =
        u64::from(status == "missing" || missing_file_count > 0 || missing_path_count > 0);
    let current = u64::from(status == "current" && stale == 0 && missing == 0);

    (current, stale, missing)
}

fn json_u64(value: &serde_json::Value, key: &str) -> u64 {
    value
        .get(key)
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0)
}

fn json_string_array(value: &serde_json::Value, key: &str) -> Vec<String> {
    value
        .get(key)
        .and_then(serde_json::Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(serde_json::Value::as_str)
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

fn missing_package_status_finding() -> DxCheckFinding {
    check_finding(
        DxSupplyChainSeverity::Medium,
        "webassembly-bridge-missing-package-status",
        format!("{WEBASSEMBLY_BRIDGE_OFFICIAL_NAME} is missing from package-status visibility"),
        Some(WEBASSEMBLY_BRIDGE_PACKAGE_STATUS.to_string()),
        "Regenerate .dx/forge/package-status.json so dx-check can report WebAssembly Bridge visibility states.",
    )
}

fn missing_receipt_finding(receipt_path: &str) -> DxCheckFinding {
    check_finding(
        DxSupplyChainSeverity::Medium,
        "webassembly-bridge-missing-receipt",
        format!("{WEBASSEMBLY_BRIDGE_OFFICIAL_NAME} is missing its package-lane receipt"),
        Some(receipt_path.to_string()),
        "Regenerate or restore the WebAssembly Bridge dashboard workflow receipt so dx-check can report source-owned package visibility.",
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
    use std::{fs, path::Path};

    use sha2::{Digest, Sha256};

    use super::*;

    #[test]
    fn webassembly_bridge_hash_mismatch_metric_and_finding_are_byte_derived() {
        let dir = tempfile::tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join(".dx/forge/receipts")).expect("receipt dir");
        fs::write(
            dir.path()
                .join(".dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json"),
            "{}",
        )
        .expect("dashboard receipt");
        fs::write(
            dir.path().join("wasm-interop-status.tsx"),
            "export const wasm = 18;\n",
        )
        .expect("wasm surface");

        let expected_hash = format!(
            "{:x}",
            Sha256::digest(fs::read(dir.path().join("wasm-interop-status.tsx")).expect("bytes"))
        );
        fs::write(
            dir.path().join(".dx/forge/package-status.json"),
            serde_json::to_vec_pretty(&serde_json::json!({
                "package_lane_visibility": [
                    {
                        "official_package_name": "WebAssembly Bridge",
                        "package_id": "wasm/bindgen",
                        "upstream_package": "wasm-bindgen",
                        "upstream_version": "0.2.121",
                        "source_mirror": "G:/WWW/inspirations/wasm-bindgen",
                        "status": "present",
                        "receipt_status": "present",
                        "package_receipt_path": ".dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json",
                        "selected_surfaces": [
                            {
                                "surface_id": "launch-wasm-compute-dashboard-workflow",
                                "status": "present",
                                "hash_algorithm": "sha256",
                                "file_hashes": {
                                    "examples/template/wasm-interop-status.tsx": expected_hash
                                }
                            }
                        ],
                        "dx_style_compatibility": {
                            "schema": "dx.forge.package.dx_style_compatibility",
                            "status": "present",
                            "token_source": "styles/theme.css",
                            "generated_css": "styles/generated.css",
                            "visible_surfaces": [
                                "launch-wasm-compute-dashboard-workflow"
                            ],
                            "source_files": [
                                "examples/template/wasm-interop-status.tsx"
                            ],
                            "receipt_path": ".dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json",
                            "runtime_proof": false,
                            "runtime_limitations": [
                                "SOURCE-ONLY: no live governed browser style proof is claimed."
                            ]
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
        let (metrics, findings) = forge_webassembly_bridge_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(&metrics, "webassembly_bridge_hash_manifest_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(&metrics, "webassembly_bridge_hash_mismatch"),
            Some(0)
        );
        assert_eq!(
            metric_value(
                &metrics,
                "webassembly_bridge_dx_style_compatibility_present"
            ),
            Some(1)
        );
        assert_eq!(
            metric_value(
                &metrics,
                "webassembly_bridge_dx_style_compatibility_missing"
            ),
            Some(0)
        );
        assert!(
            !findings
                .iter()
                .any(|finding| finding.code == "webassembly-bridge-hash-mismatch")
        );

        fs::write(
            dir.path().join("wasm-interop-status.tsx"),
            "export const wasm = 19;\n",
        )
        .expect("mutate wasm surface");
        let (metrics, findings) = forge_webassembly_bridge_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(&metrics, "webassembly_bridge_receipt_stale"),
            Some(1)
        );
        assert_eq!(
            metric_value(&metrics, "webassembly_bridge_hash_mismatch"),
            Some(1)
        );
        assert!(
            findings
                .iter()
                .any(|finding| finding.code == "webassembly-bridge-hash-mismatch")
        );
    }

    #[test]
    fn webassembly_bridge_dx_style_missing_metric_and_finding_flip() {
        let dir = tempfile::tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join(".dx/forge/receipts")).expect("receipt dir");
        fs::write(
            dir.path()
                .join(".dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json"),
            "{}",
        )
        .expect("dashboard receipt");

        write_dx_style_fixture_package_status(dir.path(), true);
        let manifest = DxSourceManifest::default();
        let (metrics, findings) = forge_webassembly_bridge_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(
                &metrics,
                "webassembly_bridge_dx_style_compatibility_present"
            ),
            Some(1)
        );
        assert_eq!(
            metric_value(
                &metrics,
                "webassembly_bridge_dx_style_compatibility_missing"
            ),
            Some(0)
        );
        assert!(!findings.iter().any(|finding| {
            finding.code == "webassembly-bridge-missing-dx-style-compatibility"
        }));

        write_dx_style_fixture_package_status(dir.path(), false);
        let (metrics, findings) = forge_webassembly_bridge_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(
                &metrics,
                "webassembly_bridge_dx_style_compatibility_present"
            ),
            Some(0)
        );
        assert_eq!(
            metric_value(
                &metrics,
                "webassembly_bridge_dx_style_compatibility_missing"
            ),
            Some(1)
        );
        assert_eq!(
            metric_value(&metrics, "webassembly_bridge_receipt_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(&metrics, "webassembly_bridge_missing_receipt"),
            Some(0)
        );
        assert!(findings.iter().any(|finding| {
            finding.code == "webassembly-bridge-missing-dx-style-compatibility"
        }));
    }

    #[test]
    fn webassembly_bridge_package_metrics_reports_helper_freshness_from_path_arrays() {
        let dir = tempfile::tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join(".dx/forge/receipts")).expect("receipt dir");
        fs::write(dir.path().join(WEBASSEMBLY_BRIDGE_DASHBOARD_RECEIPT), "{}")
            .expect("dashboard receipt");

        let package_status_path = dir.path().join(WEBASSEMBLY_BRIDGE_PACKAGE_STATUS);
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.www.template.forge_package_status",
                "package_lane_visibility": [
                    {
                        "official_package_name": WEBASSEMBLY_BRIDGE_OFFICIAL_NAME,
                        "package_id": WEBASSEMBLY_BRIDGE_PACKAGE_ID,
                        "upstream_package": "wasm-bindgen",
                        "upstream_version": "0.2.121",
                        "source_mirror": "G:/WWW/inspirations/wasm-bindgen",
                        "status": "present",
                        "receipt_status": "present",
                        "package_receipt_path": WEBASSEMBLY_BRIDGE_DASHBOARD_RECEIPT,
                        "receipt_hash_refresh": {
                            "schema": "dx.forge.package.receipt_hash_refresh",
                            "status": "current",
                            "helper_path": "examples/template/webassembly-bridge-receipt-hashes.ts",
                            "check_command": "node examples/template/webassembly-bridge-receipt-hashes.ts --check",
                            "write_command": "node examples/template/webassembly-bridge-receipt-hashes.ts --write",
                            "json_check_command": "node examples/template/webassembly-bridge-receipt-hashes.ts --check --json",
                            "source_guard_runbook_fixture": "docs/packages/wasm-bindgen.source-guard-runbook.json",
                            "receipt_path": WEBASSEMBLY_BRIDGE_DASHBOARD_RECEIPT,
                            "hash_algorithm": "sha256",
                            "tracked_file_count": 4,
                            "tracked_files": [
                                "examples/template/wasm-interop-status.tsx",
                                "tools/launch/materialize-www-template.ts",
                                "docs/packages/wasm-bindgen.md",
                                "dx-www/src/cli/studio_manifest.rs"
                            ],
                            "current_files": [
                                "examples/template/wasm-interop-status.tsx",
                                "tools/launch/materialize-www-template.ts",
                                "docs/packages/wasm-bindgen.md",
                                "dx-www/src/cli/studio_manifest.rs"
                            ],
                            "stale_files": [],
                            "missing_files": [],
                            "stale_mirror_files": [],
                            "missing_mirror_files": [],
                            "stale_file_count": 0,
                            "missing_file_count": 0,
                            "runtime_execution": false,
                            "secret_access": false,
                            "zed_visibility": "webassembly-bridge:receipt-hash-refresh"
                        },
                        "selected_surfaces": [],
                        "blocked_surfaces": [],
                        "unsupported_surfaces": [],
                        "dx_style_compatibility": {
                            "schema": "dx.forge.package.dx_style_compatibility",
                            "status": "present",
                            "token_source": "styles/theme.css",
                            "generated_css": "styles/generated.css",
                            "visible_surfaces": [
                                "launch-wasm-compute-dashboard-workflow"
                            ],
                            "source_files": [
                                "examples/template/wasm-interop-status.tsx"
                            ],
                            "receipt_path": WEBASSEMBLY_BRIDGE_DASHBOARD_RECEIPT,
                            "runtime_proof": false
                        }
                    }
                ]
            }))
            .expect("package status json"),
        )
        .expect("write package status");

        let manifest = DxSourceManifest::default();
        let (current_metrics, current_findings) =
            forge_webassembly_bridge_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(
                &current_metrics,
                "webassembly_bridge_receipt_hash_refresh_current"
            ),
            Some(1)
        );
        assert_eq!(
            metric_value(
                &current_metrics,
                "webassembly_bridge_receipt_hash_refresh_stale"
            ),
            Some(0)
        );
        assert_eq!(
            metric_value(
                &current_metrics,
                "webassembly_bridge_receipt_hash_refresh_missing"
            ),
            Some(0)
        );
        assert_eq!(
            metric_value(&current_metrics, "webassembly_bridge_receipt_stale"),
            Some(0)
        );
        assert!(
            !current_findings
                .iter()
                .any(|finding| finding.code == "webassembly-bridge-stale-receipt")
        );

        let mut package_status: serde_json::Value =
            serde_json::from_slice(&fs::read(&package_status_path).expect("package status bytes"))
                .expect("package status json");
        package_status["package_lane_visibility"][0]["receipt_hash_refresh"]["stale_files"] =
            serde_json::json!(["tools/launch/materialize-www-template.ts"]);
        package_status["package_lane_visibility"][0]["receipt_hash_refresh"]["stale_mirror_files"] =
            serde_json::json!(["dx-www/src/cli/studio_manifest.rs"]);
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&package_status).expect("stale package status json"),
        )
        .expect("write stale package status");

        let (stale_metrics, stale_findings) =
            forge_webassembly_bridge_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(
                &stale_metrics,
                "webassembly_bridge_receipt_hash_refresh_current"
            ),
            Some(0)
        );
        assert_eq!(
            metric_value(
                &stale_metrics,
                "webassembly_bridge_receipt_hash_refresh_stale"
            ),
            Some(1)
        );
        assert_eq!(
            metric_value(
                &stale_metrics,
                "webassembly_bridge_receipt_hash_refresh_missing"
            ),
            Some(0)
        );
        assert_eq!(
            metric_value(&stale_metrics, "webassembly_bridge_hash_mismatch"),
            Some(0)
        );
        assert_eq!(
            metric_value(&stale_metrics, "webassembly_bridge_receipt_stale"),
            Some(1)
        );
        assert!(
            stale_findings
                .iter()
                .any(|finding| finding.code == "webassembly-bridge-stale-receipt")
        );

        package_status["package_lane_visibility"][0]["receipt_hash_refresh"]["stale_files"] =
            serde_json::json!([]);
        package_status["package_lane_visibility"][0]["receipt_hash_refresh"]["stale_mirror_files"] =
            serde_json::json!([]);
        package_status["package_lane_visibility"][0]["receipt_hash_refresh"]["missing_files"] =
            serde_json::json!(["docs/packages/wasm-bindgen.source-guard-runbook.json"]);
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&package_status).expect("missing package status json"),
        )
        .expect("write missing package status");

        let (missing_metrics, _) = forge_webassembly_bridge_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(
                &missing_metrics,
                "webassembly_bridge_receipt_hash_refresh_current"
            ),
            Some(0)
        );
        assert_eq!(
            metric_value(
                &missing_metrics,
                "webassembly_bridge_receipt_hash_refresh_stale"
            ),
            Some(0)
        );
        assert_eq!(
            metric_value(
                &missing_metrics,
                "webassembly_bridge_receipt_hash_refresh_missing"
            ),
            Some(1)
        );
    }

    fn write_dx_style_fixture_package_status(root: &Path, include_dx_style: bool) {
        let mut visibility = serde_json::json!({
            "official_package_name": "WebAssembly Bridge",
            "package_id": "wasm/bindgen",
            "upstream_package": "wasm-bindgen",
            "upstream_version": "0.2.121",
            "source_mirror": "G:/WWW/inspirations/wasm-bindgen",
            "status": "present",
            "receipt_status": "present",
            "package_receipt_path": ".dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json",
            "selected_surfaces": [
                {
                    "surface_id": "launch-wasm-compute-dashboard-workflow",
                    "status": "present"
                }
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
                            "launch-wasm-compute-dashboard-workflow"
                        ],
                        "source_files": [
                            "examples/template/wasm-interop-status.tsx"
                        ],
                        "receipt_path": ".dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json",
                        "runtime_proof": false,
                        "runtime_limitations": [
                            "SOURCE-ONLY: no live governed browser style proof is claimed."
                        ]
                    }),
                );
        }

        fs::write(
            root.join(".dx/forge/package-status.json"),
            serde_json::to_vec_pretty(&serde_json::json!({
                "package_lane_visibility": [visibility]
            }))
            .expect("package status json"),
        )
        .expect("package status");
    }

    fn metric_value(metrics: &[DxCheckMetric], name: &str) -> Option<u64> {
        metrics
            .iter()
            .find(|metric| metric.name == name)
            .map(|metric| metric.value)
    }
}
