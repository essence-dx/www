#![allow(clippy::too_many_arguments)]
use std::{fmt::Write as _, fs, path::Path};

use sha2::{Digest, Sha256};

use super::super::forge_security::{DxSourceManifest, DxSupplyChainSeverity};
use super::{
    DxCheckFinding, DxCheckMetric, check_finding, check_metric, json_array_entries, json_text,
    read_optional_forge_json, resolve_dx_check_relative_path,
};

const AUTOMATION_CONNECTORS_PACKAGE_ID: &str = "automations/n8n";
const AUTOMATION_CONNECTORS_OFFICIAL_NAME: &str = "Automation Connectors";
const AUTOMATION_CONNECTORS_PACKAGE_STATUS: &str = ".dx/forge/package-status.json";
const AUTOMATION_CONNECTORS_DASHBOARD_RECEIPT: &str =
    ".dx/forge/receipts/2026-05-22-automation-connectors-launch-workflow.json";
const REQUIRED_UPSTREAM_FILES: &[&str] = &[
    "packages/nodes-base/package.json",
    "packages/nodes-base/nodes/ManualTrigger/ManualTrigger.node.ts",
    "packages/nodes-base/nodes/Slack/Slack.node.ts",
    "packages/nodes-base/nodes/Slack/V2/SlackV2.node.ts",
    "packages/nodes-base/nodes/Webhook/Webhook.node.ts",
    "packages/nodes-base/nodes/Notion/Notion.node.ts",
    "packages/nodes-base/credentials/SlackApi.credentials.ts",
    "packages/nodes-base/credentials/SlackOAuth2Api.credentials.ts",
    "packages/nodes-base/credentials/NotionApi.credentials.ts",
];
const REQUIRED_UPSTREAM_PUBLIC_APIS: &[&str] = &[
    "VersionedNodeType",
    "INodeType",
    "INodeTypeDescription",
    "ITriggerFunctions",
    "IExecuteFunctions",
    "IWebhookFunctions",
    "ICredentialType",
    "IAuthenticateGeneric",
    "ICredentialTestRequest",
];

pub(super) fn forge_automation_connectors_package_metrics(
    root: &Path,
    manifest: &DxSourceManifest,
) -> (Vec<DxCheckMetric>, Vec<DxCheckFinding>) {
    let mut findings = Vec::new();
    let manifest_package_present = manifest
        .packages
        .iter()
        .any(|package| package.package_id == AUTOMATION_CONNECTORS_PACKAGE_ID);
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
    let mut upstream_runtime_boundary_present = 0u64;
    let mut upstream_runtime_boundary_missing = 0u64;
    let mut receipt_hash_refresh_current = 0u64;
    let mut receipt_hash_refresh_stale = 0u64;
    let mut receipt_hash_refresh_missing = 0u64;

    let Some(package_status) = read_optional_forge_json(
        root,
        AUTOMATION_CONNECTORS_PACKAGE_STATUS,
        "automation-connectors-package-status-invalid",
        &mut findings,
    ) else {
        if manifest_package_present
            || package_receipt_exists(root, AUTOMATION_CONNECTORS_DASHBOARD_RECEIPT)
        {
            package_present = 1;
            missing_receipt = 1;
            receipt_hash_refresh_missing = 1;
            findings.push(missing_package_status_finding());
        }
        return (
            automation_connectors_metrics(
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
                upstream_runtime_boundary_present,
                upstream_runtime_boundary_missing,
                receipt_hash_refresh_current,
                receipt_hash_refresh_stale,
                receipt_hash_refresh_missing,
            ),
            findings,
        );
    };

    let Some(visibility) = json_array_entries(&package_status, &["package_lane_visibility"])
        .into_iter()
        .find(|entry| json_text(entry, &["package_id"]) == Some(AUTOMATION_CONNECTORS_PACKAGE_ID))
    else {
        if manifest_package_present
            || package_receipt_exists(root, AUTOMATION_CONNECTORS_DASHBOARD_RECEIPT)
        {
            package_present = 1;
            missing_receipt = 1;
            receipt_hash_refresh_missing = 1;
            findings.push(missing_package_status_finding());
        }
        return (
            automation_connectors_metrics(
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
                upstream_runtime_boundary_present,
                upstream_runtime_boundary_missing,
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
    if refresh_stale > 0 || refresh_missing > 0 {
        stale_receipt = 1;
    }

    if has_dx_style_compatibility(visibility) {
        dx_style_compatibility_present = 1;
    } else {
        dx_style_compatibility_missing = 1;
    }

    if has_upstream_runtime_boundary(visibility) {
        upstream_runtime_boundary_present = 1;
    } else {
        upstream_runtime_boundary_missing = 1;
    }

    let package_receipt_path = json_text(visibility, &["package_receipt_path"])
        .unwrap_or(AUTOMATION_CONNECTORS_DASHBOARD_RECEIPT);
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
    if matches!(
        visibility_status.or(receipt_status),
        Some("unsupported-surface")
    ) {
        unsupported_surfaces += 1;
    }

    blocked_surfaces += json_array_entries(visibility, &["blocked_surfaces"]).len() as u64;
    unsupported_surfaces += json_array_entries(visibility, &["unsupported_surfaces"]).len() as u64;

    let mut selected_surface_hash_manifest_present = false;
    for surface in json_array_entries(visibility, &["selected_surfaces"]) {
        if has_sha256_hash_manifest(surface) {
            hash_manifest_present = 1;
            selected_surface_hash_manifest_present = true;
        }

        hash_mismatches += count_hash_mismatches(root, surface);

        match json_text(surface, &["status"]) {
            Some("stale") => stale_receipt = 1,
            Some("missing-receipt") => missing_receipt = 1,
            Some("blocked") => blocked_surfaces += 1,
            Some("unsupported-surface") => unsupported_surfaces += 1,
            _ => {}
        }
    }

    if !selected_surface_hash_manifest_present {
        if let Some(receipt) = read_package_receipt(root, package_receipt_path) {
            if has_sha256_hash_manifest(&receipt) {
                hash_manifest_present = 1;
            }
            hash_mismatches += count_hash_mismatches(root, &receipt);
        }
    }

    if hash_mismatches > 0 {
        stale_receipt = 1;
    }

    if stale_receipt > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "automation-connectors-stale-receipt",
            format!("{AUTOMATION_CONNECTORS_OFFICIAL_NAME} package-status visibility is stale"),
            Some(AUTOMATION_CONNECTORS_PACKAGE_STATUS.to_string()),
            "Regenerate the Automation Connectors package-status row from the dashboard workflow receipt before claiming source freshness.",
        ));
    }
    if blocked_surfaces > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "automation-connectors-blocked-surface",
            format!(
                "{AUTOMATION_CONNECTORS_OFFICIAL_NAME} has {blocked_surfaces} blocked selected surface(s)"
            ),
            Some(AUTOMATION_CONNECTORS_PACKAGE_STATUS.to_string()),
            "Resolve the app-owned Automation Connectors credential or runtime boundary before claiming the selected surface is release-ready.",
        ));
    }
    if unsupported_surfaces > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "automation-connectors-unsupported-surface",
            format!(
                "{AUTOMATION_CONNECTORS_OFFICIAL_NAME} has {unsupported_surfaces} unsupported requested surface(s)"
            ),
            Some(AUTOMATION_CONNECTORS_PACKAGE_STATUS.to_string()),
            "Request only supported Automation Connectors surfaces or add a real upstream-backed Forge surface first.",
        ));
    }
    if hash_mismatches > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "automation-connectors-hash-mismatch",
            format!(
                "{AUTOMATION_CONNECTORS_OFFICIAL_NAME} has {hash_mismatches} missing or stale hash-backed selected file(s)"
            ),
            Some(AUTOMATION_CONNECTORS_PACKAGE_STATUS.to_string()),
            "Regenerate the Automation Connectors launch workflow receipt after reviewing the changed dashboard, Zed handoff, or connector readiness source files.",
        ));
    }
    if dx_style_compatibility_missing > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "automation-connectors-missing-dx-style-compatibility",
            format!(
                "{AUTOMATION_CONNECTORS_OFFICIAL_NAME} is missing source-owned dx-style compatibility evidence"
            ),
            Some(AUTOMATION_CONNECTORS_PACKAGE_STATUS.to_string()),
            "Restore the Automation Connectors dx_style_compatibility package-status row with source markers, token scope, generated CSS evidence, and runtime-proof limitations before claiming style compatibility.",
        ));
    }
    if upstream_runtime_boundary_missing > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "automation-connectors-missing-upstream-runtime-boundary",
            format!(
                "{AUTOMATION_CONNECTORS_OFFICIAL_NAME} is missing inspected upstream runtime-boundary evidence"
            ),
            Some(AUTOMATION_CONNECTORS_PACKAGE_STATUS.to_string()),
            "Restore inspected_upstream_files and upstream_public_apis for the Manual Trigger, Slack V2 execute, Webhook, Notion, and credential boundaries before claiming upstream provenance visibility.",
        ));
    }

    (
        automation_connectors_metrics(
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
            upstream_runtime_boundary_present,
            upstream_runtime_boundary_missing,
            receipt_hash_refresh_current,
            receipt_hash_refresh_stale,
            receipt_hash_refresh_missing,
        ),
        findings,
    )
}

fn automation_connectors_metrics(
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
    upstream_runtime_boundary_present: u64,
    upstream_runtime_boundary_missing: u64,
    receipt_hash_refresh_current: u64,
    receipt_hash_refresh_stale: u64,
    receipt_hash_refresh_missing: u64,
) -> Vec<DxCheckMetric> {
    vec![
        check_metric("automation_connectors_package_present", package_present),
        check_metric("automation_connectors_receipt_present", receipt_present),
        check_metric("automation_connectors_receipt_stale", stale_receipt),
        check_metric("automation_connectors_missing_receipt", missing_receipt),
        check_metric("automation_connectors_blocked_surface", blocked_surfaces),
        check_metric(
            "automation_connectors_unsupported_surface",
            unsupported_surfaces,
        ),
        check_metric(
            "automation_connectors_hash_manifest_present",
            hash_manifest_present,
        ),
        check_metric("automation_connectors_hash_mismatch", hash_mismatches),
        check_metric(
            "automation_connectors_dx_style_compatibility_present",
            dx_style_compatibility_present,
        ),
        check_metric(
            "automation_connectors_dx_style_compatibility_missing",
            dx_style_compatibility_missing,
        ),
        check_metric(
            "automation_connectors_upstream_runtime_boundary_present",
            upstream_runtime_boundary_present,
        ),
        check_metric(
            "automation_connectors_upstream_runtime_boundary_missing",
            upstream_runtime_boundary_missing,
        ),
        check_metric(
            "automation_connectors_receipt_hash_refresh_current",
            receipt_hash_refresh_current,
        ),
        check_metric(
            "automation_connectors_receipt_hash_refresh_stale",
            receipt_hash_refresh_stale,
        ),
        check_metric(
            "automation_connectors_receipt_hash_refresh_missing",
            receipt_hash_refresh_missing,
        ),
    ]
}

fn has_upstream_runtime_boundary(value: &serde_json::Value) -> bool {
    json_array_contains_all(value, "inspected_upstream_files", REQUIRED_UPSTREAM_FILES)
        && json_array_contains_all(value, "upstream_public_apis", REQUIRED_UPSTREAM_PUBLIC_APIS)
}

fn json_array_contains_all(value: &serde_json::Value, key: &str, required: &[&str]) -> bool {
    let entries = json_array_entries(value, &[key]);
    required.iter().all(|required| {
        entries
            .iter()
            .any(|entry| entry.as_str() == Some(*required))
    })
}

fn has_dx_style_compatibility(value: &serde_json::Value) -> bool {
    let Some(compatibility) = value.get("dx_style_compatibility") else {
        return false;
    };

    json_text(compatibility, &["schema"]) == Some("dx.forge.package.dx_style_compatibility")
        && json_text(compatibility, &["status"]) == Some("present")
        && json_text(compatibility, &["token_source"]).is_some()
        && json_text(compatibility, &["generated_css"]).is_some()
        && json_text(compatibility, &["receipt_path"]).is_some()
        && compatibility
            .get("visible_surfaces")
            .and_then(serde_json::Value::as_array)
            .is_some_and(|surfaces| !surfaces.is_empty())
        && compatibility
            .get("source_files")
            .and_then(serde_json::Value::as_array)
            .is_some_and(|files| !files.is_empty())
}

fn has_sha256_hash_manifest(value: &serde_json::Value) -> bool {
    json_text(value, &["hash_algorithm"]) == Some("sha256")
        && value
            .get("file_hashes")
            .and_then(serde_json::Value::as_object)
            .is_some_and(|hashes| !hashes.is_empty())
}

fn count_hash_mismatches(root: &Path, value: &serde_json::Value) -> u64 {
    let mut mismatches = u64::from(json_text(value, &["status"]) == Some("stale"));
    if json_text(value, &["hash_algorithm"]) != Some("sha256") {
        return mismatches;
    }

    let Some(file_hashes) = value
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

        match hash_project_file_sha256(root, relative_path) {
            Some(actual_hash) if normalize_sha256_hash(expected_hash) == actual_hash => {}
            _ => mismatches += 1,
        }
    }

    mismatches
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

fn read_package_receipt(root: &Path, receipt_path: &str) -> Option<serde_json::Value> {
    let path = project_file_path(root, receipt_path)?;
    let bytes = fs::read(path).ok()?;
    serde_json::from_slice(&bytes).ok()
}

fn hash_project_file_sha256(root: &Path, relative_path: &str) -> Option<String> {
    let path = project_file_path(root, relative_path)?;
    let bytes = fs::read(path).ok()?;
    let digest = Sha256::digest(bytes);
    Some(hex_digest(&digest))
}

fn normalize_sha256_hash(value: &str) -> String {
    value
        .trim()
        .strip_prefix("sha256:")
        .unwrap_or(value.trim())
        .to_ascii_lowercase()
}

fn hex_digest(bytes: &[u8]) -> String {
    let mut hex = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        let _ = write!(&mut hex, "{byte:02x}");
    }
    hex
}

fn missing_package_status_finding() -> DxCheckFinding {
    check_finding(
        DxSupplyChainSeverity::Medium,
        "automation-connectors-missing-package-status",
        format!("{AUTOMATION_CONNECTORS_OFFICIAL_NAME} is missing from package-status visibility"),
        Some(AUTOMATION_CONNECTORS_PACKAGE_STATUS.to_string()),
        "Regenerate .dx/forge/package-status.json so dx-check can report Automation Connectors visibility states.",
    )
}

fn missing_receipt_finding(receipt_path: &str) -> DxCheckFinding {
    check_finding(
        DxSupplyChainSeverity::Medium,
        "automation-connectors-missing-receipt",
        format!("{AUTOMATION_CONNECTORS_OFFICIAL_NAME} is missing its package-lane receipt"),
        Some(receipt_path.to_string()),
        "Regenerate or restore the Automation Connectors dashboard workflow receipt so dx-check can report source-owned package visibility.",
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

    #[test]
    fn automation_connectors_package_metrics_reports_helper_freshness_from_path_arrays() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_automation_connectors_receipt(dir.path());
        write_automation_connectors_package_status(dir.path());

        let manifest = DxSourceManifest::default();
        let (current_metrics, current_findings) =
            forge_automation_connectors_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(
                &current_metrics,
                "automation_connectors_receipt_hash_refresh_current"
            ),
            Some(1)
        );
        assert_eq!(
            metric_value(
                &current_metrics,
                "automation_connectors_receipt_hash_refresh_stale"
            ),
            Some(0)
        );
        assert_eq!(
            metric_value(
                &current_metrics,
                "automation_connectors_receipt_hash_refresh_missing"
            ),
            Some(0)
        );
        assert_eq!(
            metric_value(&current_metrics, "automation_connectors_receipt_stale"),
            Some(0)
        );
        assert!(
            !current_findings
                .iter()
                .any(|finding| { finding.code == "automation-connectors-stale-receipt" })
        );

        let package_status_path = dir.path().join(AUTOMATION_CONNECTORS_PACKAGE_STATUS);
        let mut package_status: serde_json::Value =
            serde_json::from_slice(&fs::read(&package_status_path).expect("package status bytes"))
                .expect("package status json");
        package_status["package_lane_visibility"][0]["receipt_hash_refresh"]["stale_files"] =
            serde_json::json!(["tools/launch/materialize-www-template.ts"]);
        package_status["package_lane_visibility"][0]["receipt_hash_refresh"]["stale_mirror_files"] =
            serde_json::json!(["examples/template/forge-package-status-read-model.ts"]);
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&package_status).expect("stale package status json"),
        )
        .expect("write stale package status");

        let (stale_metrics, stale_findings) =
            forge_automation_connectors_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(
                &stale_metrics,
                "automation_connectors_receipt_hash_refresh_current"
            ),
            Some(0)
        );
        assert_eq!(
            metric_value(
                &stale_metrics,
                "automation_connectors_receipt_hash_refresh_stale"
            ),
            Some(1)
        );
        assert_eq!(
            metric_value(
                &stale_metrics,
                "automation_connectors_receipt_hash_refresh_missing"
            ),
            Some(0)
        );
        assert_eq!(
            metric_value(&stale_metrics, "automation_connectors_hash_mismatch"),
            Some(0)
        );
        assert_eq!(
            metric_value(&stale_metrics, "automation_connectors_receipt_stale"),
            Some(1)
        );
        assert!(
            stale_findings
                .iter()
                .any(|finding| { finding.code == "automation-connectors-stale-receipt" })
        );

        package_status["package_lane_visibility"][0]["receipt_hash_refresh"]["stale_files"] =
            serde_json::json!([]);
        package_status["package_lane_visibility"][0]["receipt_hash_refresh"]["stale_mirror_files"] =
            serde_json::json!([]);
        package_status["package_lane_visibility"][0]["receipt_hash_refresh"]["missing_files"] =
            serde_json::json!(["dx-www/src/cli/studio_manifest.rs"]);
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&package_status).expect("missing package status json"),
        )
        .expect("write missing package status");

        let (missing_metrics, _) =
            forge_automation_connectors_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(
                &missing_metrics,
                "automation_connectors_receipt_hash_refresh_current"
            ),
            Some(0)
        );
        assert_eq!(
            metric_value(
                &missing_metrics,
                "automation_connectors_receipt_hash_refresh_stale"
            ),
            Some(0)
        );
        assert_eq!(
            metric_value(
                &missing_metrics,
                "automation_connectors_receipt_hash_refresh_missing"
            ),
            Some(1)
        );
    }

    fn write_automation_connectors_receipt(root: &Path) {
        let receipt_path = root.join(AUTOMATION_CONNECTORS_DASHBOARD_RECEIPT);
        fs::create_dir_all(receipt_path.parent().expect("receipt parent"))
            .expect("receipt directory");
        fs::write(
            receipt_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.forge.package_dashboard_workflow_receipt",
                "official_package_name": AUTOMATION_CONNECTORS_OFFICIAL_NAME,
                "package_id": AUTOMATION_CONNECTORS_PACKAGE_ID,
                "surface": "automation-launch-dashboard-workflow"
            }))
            .expect("receipt json"),
        )
        .expect("write receipt");
    }

    fn write_automation_connectors_package_status(root: &Path) {
        let package_status_path = root.join(AUTOMATION_CONNECTORS_PACKAGE_STATUS);
        fs::create_dir_all(package_status_path.parent().expect("package status parent"))
            .expect("package status directory");
        fs::write(
            package_status_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.www.template.forge_package_status",
                "package_lane_visibility": [
                    {
                        "official_package_name": AUTOMATION_CONNECTORS_OFFICIAL_NAME,
                        "package_id": AUTOMATION_CONNECTORS_PACKAGE_ID,
                        "upstream_package": "n8n-nodes-base",
                        "upstream_version": "2.22.0",
                        "source_mirror": "G:/WWW/inspirations/n8n/packages/nodes-base",
                        "status": "present",
                        "receipt_status": "present",
                        "package_receipt_path": AUTOMATION_CONNECTORS_DASHBOARD_RECEIPT,
                        "receipt_hash_refresh": {
                            "schema": "dx.forge.package.receipt_hash_refresh",
                            "status": "current",
                            "helper_path": "examples/template/automation-connectors-receipt-hashes.ts",
                            "check_command": "node examples/template/automation-connectors-receipt-hashes.ts --check",
                            "write_command": "node examples/template/automation-connectors-receipt-hashes.ts --write",
                            "json_check_command": "node examples/template/automation-connectors-receipt-hashes.ts --check --json",
                            "source_guard_runbook_fixture": "docs/packages/automation-connectors.source-guard-runbook.json",
                            "preview_manifest_materializer": "tools/launch/materialize-www-template.ts",
                            "studio_manifest_source": "dx-www/src/cli/studio_manifest.rs",
                            "receipt_path": AUTOMATION_CONNECTORS_DASHBOARD_RECEIPT,
                            "hash_algorithm": "sha256",
                            "tracked_file_count": 13,
                            "tracked_files": [
                                "examples/template/automations-status.tsx",
                                "tools/launch/materialize-www-template.ts",
                                "dx-www/src/cli/studio_manifest.rs"
                            ],
                            "current_files": [
                                "examples/template/automations-status.tsx",
                                "tools/launch/materialize-www-template.ts",
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
                            "zed_visibility": "automation-connectors:receipt-hash-refresh"
                        },
                        "selected_surfaces": [],
                        "blocked_surfaces": [],
                        "unsupported_surfaces": [],
                        "dx_style_compatibility": {
                            "schema": "dx.forge.package.dx_style_compatibility",
                            "status": "present",
                            "token_source": "styles/theme.css",
                            "generated_css": "styles/generated.css",
                            "visible_surfaces": ["automation-connectors"],
                            "source_files": ["examples/template/automations-status.tsx"],
                            "receipt_path": AUTOMATION_CONNECTORS_DASHBOARD_RECEIPT
                        },
                        "inspected_upstream_files": REQUIRED_UPSTREAM_FILES,
                        "upstream_public_apis": REQUIRED_UPSTREAM_PUBLIC_APIS
                    }
                ]
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
