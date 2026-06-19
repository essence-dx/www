#![allow(clippy::too_many_arguments)]
use std::path::Path;

use super::super::forge_security::{DxSourceManifest, DxSupplyChainSeverity};
use super::file_hashes::{count_materialized_sha256_file_hash_mismatches, has_sha256_file_hashes};
use super::{
    DxCheckFinding, DxCheckMetric, check_finding, check_metric, json_array_entries, json_text,
    read_optional_forge_json, resolve_dx_check_relative_path,
};

const AI_SDK_PACKAGE_ID: &str = "ai/vercel-ai";
const AI_SDK_OFFICIAL_NAME: &str = "AI SDK";
const AI_SDK_PACKAGE_STATUS: &str = ".dx/forge/package-status.json";
const AI_SDK_LAUNCH_ASSISTANT_RECEIPT: &str =
    ".dx/forge/receipts/2026-05-22-ai-vercel-ai-launch-assistant.json";

pub(super) fn forge_ai_sdk_package_metrics(
    root: &Path,
    manifest: &DxSourceManifest,
) -> (Vec<DxCheckMetric>, Vec<DxCheckFinding>) {
    let mut findings = Vec::new();
    let manifest_package_present = manifest
        .packages
        .iter()
        .any(|package| package.package_id == AI_SDK_PACKAGE_ID);
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
        AI_SDK_PACKAGE_STATUS,
        "ai-sdk-package-status-invalid",
        &mut findings,
    ) else {
        if manifest_package_present || package_receipt_exists(root, AI_SDK_LAUNCH_ASSISTANT_RECEIPT)
        {
            package_present = 1;
            missing_receipt = 1;
            findings.push(missing_package_status_finding());
        }
        return (
            ai_sdk_metrics(
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
        .find(|entry| json_text(entry, &["package_id"]) == Some(AI_SDK_PACKAGE_ID))
    else {
        if manifest_package_present || package_receipt_exists(root, AI_SDK_LAUNCH_ASSISTANT_RECEIPT)
        {
            package_present = 1;
            missing_receipt = 1;
            findings.push(missing_package_status_finding());
        }
        return (
            ai_sdk_metrics(
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

    let package_receipt_path =
        json_text(visibility, &["package_receipt_path"]).unwrap_or(AI_SDK_LAUNCH_ASSISTANT_RECEIPT);
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

        hash_mismatches += count_materialized_sha256_file_hash_mismatches(root, surface);

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

    if dx_style_compatibility_is_present(visibility) {
        dx_style_compatibility_present = 1;
    } else {
        dx_style_compatibility_missing = 1;
    }

    if stale_receipt > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "ai-sdk-stale-receipt",
            format!("{AI_SDK_OFFICIAL_NAME} package-status visibility is stale"),
            Some(AI_SDK_PACKAGE_STATUS.to_string()),
            "Regenerate the AI SDK package-status row from the launch assistant receipt before claiming source freshness.",
        ));
    }
    if blocked_surfaces > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "ai-sdk-blocked-surface",
            format!("{AI_SDK_OFFICIAL_NAME} has {blocked_surfaces} blocked selected surface(s)"),
            Some(AI_SDK_PACKAGE_STATUS.to_string()),
            "Resolve the ADAPTER-BOUNDARY AI SDK provider credentials, gateway routing, model safety, persistence, rate limits, and billing controls before claiming the selected surface is release-ready.",
        ));
    }
    if unsupported_surfaces > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "ai-sdk-unsupported-surface",
            format!(
                "{AI_SDK_OFFICIAL_NAME} has {unsupported_surfaces} unsupported requested surface(s)"
            ),
            Some(AI_SDK_PACKAGE_STATUS.to_string()),
            "Request only supported AI SDK surfaces or add a real upstream-backed Forge surface first.",
        ));
    }
    if hash_mismatches > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "ai-sdk-hash-mismatch",
            format!(
                "{AI_SDK_OFFICIAL_NAME} has {hash_mismatches} missing or stale hash-backed materialized file(s)"
            ),
            Some(AI_SDK_PACKAGE_STATUS.to_string()),
            "Regenerate the AI SDK receipt after reviewing the changed front-facing launch assistant files.",
        ));
    }
    if dx_style_compatibility_missing > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "ai-sdk-missing-dx-style-compatibility",
            format!("{AI_SDK_OFFICIAL_NAME} is missing dx-style compatibility evidence"),
            Some(AI_SDK_PACKAGE_STATUS.to_string()),
            "Regenerate the AI SDK dx-style compatibility row from the launch assistant receipt and verify source-owned style markers without claiming live model streaming or browser visual proof.",
        ));
    }

    (
        ai_sdk_metrics(
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

fn ai_sdk_metrics(
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
        check_metric("ai_sdk_package_present", package_present),
        check_metric("ai_sdk_receipt_present", receipt_present),
        check_metric("ai_sdk_receipt_stale", stale_receipt),
        check_metric("ai_sdk_missing_receipt", missing_receipt),
        check_metric("ai_sdk_blocked_surface", blocked_surfaces),
        check_metric("ai_sdk_unsupported_surface", unsupported_surfaces),
        check_metric("ai_sdk_hash_manifest_present", hash_manifest_present),
        check_metric("ai_sdk_hash_mismatch", hash_mismatches),
        check_metric(
            "ai_sdk_dx_style_compatibility_present",
            dx_style_compatibility_present,
        ),
        check_metric(
            "ai_sdk_dx_style_compatibility_missing",
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
        "ai-sdk-missing-package-status",
        format!("{AI_SDK_OFFICIAL_NAME} is missing from package-status visibility"),
        Some(AI_SDK_PACKAGE_STATUS.to_string()),
        "Regenerate .dx/forge/package-status.json so dx-check can report AI SDK visibility states.",
    )
}

fn missing_receipt_finding(receipt_path: &str) -> DxCheckFinding {
    check_finding(
        DxSupplyChainSeverity::Medium,
        "ai-sdk-missing-receipt",
        format!("{AI_SDK_OFFICIAL_NAME} is missing its package-lane receipt"),
        Some(receipt_path.to_string()),
        "Regenerate or restore the AI SDK launch assistant receipt so dx-check can report source-owned package visibility.",
    )
}

fn package_receipt_exists(root: &Path, receipt_path: &str) -> bool {
    if resolve_dx_check_relative_path(root, receipt_path).is_some_and(|path| path.is_file()) {
        return true;
    }

    receipt_path
        .strip_prefix("examples/template/")
        .and_then(|template_relative| resolve_dx_check_relative_path(root, template_relative))
        .filter(|path| path.is_file())
        .is_some()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use sha2::{Digest, Sha256};

    use super::*;

    #[test]
    fn ai_sdk_hash_mismatch_metric_and_finding_are_byte_derived() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_ai_sdk_receipt(dir.path());

        let assistant_path = dir.path().join("components/launch/ai-chat-status.tsx");
        fs::create_dir_all(assistant_path.parent().expect("assistant dir")).expect("assistant dir");
        fs::write(
            &assistant_path,
            "export function LaunchAiChatStatus() { return null; }\n",
        )
        .expect("assistant surface");
        let expected_hash = sha256_file(&assistant_path);
        write_ai_sdk_package_status(dir.path(), &expected_hash, true);

        let manifest = DxSourceManifest::default();
        let (metrics, findings) = forge_ai_sdk_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(&metrics, "ai_sdk_hash_manifest_present"),
            Some(1)
        );
        assert_eq!(metric_value(&metrics, "ai_sdk_receipt_present"), Some(1));
        assert_eq!(metric_value(&metrics, "ai_sdk_receipt_stale"), Some(0));
        assert_eq!(metric_value(&metrics, "ai_sdk_hash_mismatch"), Some(0));
        assert!(
            !findings
                .iter()
                .any(|finding| finding.code == "ai-sdk-hash-mismatch")
        );

        fs::write(
            &assistant_path,
            "export function LaunchAiChatStatus() { return <section />; }\n",
        )
        .expect("mutate assistant surface");
        let (stale_metrics, stale_findings) = forge_ai_sdk_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(&stale_metrics, "ai_sdk_receipt_stale"),
            Some(1)
        );
        assert_eq!(
            metric_value(&stale_metrics, "ai_sdk_hash_mismatch"),
            Some(1)
        );
        assert!(
            stale_findings
                .iter()
                .any(|finding| finding.code == "ai-sdk-hash-mismatch")
        );
    }

    #[test]
    fn ai_sdk_dx_style_compatibility_missing_is_reported() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_ai_sdk_receipt(dir.path());

        let assistant_path = dir.path().join("components/launch/ai-chat-status.tsx");
        fs::create_dir_all(assistant_path.parent().expect("assistant dir")).expect("assistant dir");
        fs::write(
            &assistant_path,
            "export function LaunchAiChatStatus() { return null; }\n",
        )
        .expect("assistant surface");
        let expected_hash = sha256_file(&assistant_path);

        write_ai_sdk_package_status(dir.path(), &expected_hash, false);
        let manifest = DxSourceManifest::default();
        let (missing_metrics, missing_findings) =
            forge_ai_sdk_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(&missing_metrics, "ai_sdk_dx_style_compatibility_present"),
            Some(0)
        );
        assert_eq!(
            metric_value(&missing_metrics, "ai_sdk_dx_style_compatibility_missing"),
            Some(1)
        );
        assert!(
            missing_findings
                .iter()
                .any(|finding| finding.code == "ai-sdk-missing-dx-style-compatibility")
        );

        write_ai_sdk_package_status(dir.path(), &expected_hash, true);
        let (present_metrics, present_findings) =
            forge_ai_sdk_package_metrics(dir.path(), &manifest);
        assert_eq!(
            metric_value(&present_metrics, "ai_sdk_dx_style_compatibility_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(&present_metrics, "ai_sdk_dx_style_compatibility_missing"),
            Some(0)
        );
        assert!(
            !present_findings
                .iter()
                .any(|finding| finding.code == "ai-sdk-missing-dx-style-compatibility")
        );
    }

    fn write_ai_sdk_receipt(root: &Path) {
        let receipt_path = root.join(AI_SDK_LAUNCH_ASSISTANT_RECEIPT);
        fs::create_dir_all(receipt_path.parent().expect("receipt parent")).expect("receipt dir");
        fs::write(
            receipt_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.forge.package_dashboard_workflow_receipt",
                "official_package_name": AI_SDK_OFFICIAL_NAME,
                "package_id": AI_SDK_PACKAGE_ID,
                "honesty_label": "ADAPTER-BOUNDARY"
            }))
            .expect("receipt json"),
        )
        .expect("write receipt");
    }

    fn write_ai_sdk_package_status(root: &Path, expected_hash: &str, include_dx_style: bool) {
        let package_status_path = root.join(AI_SDK_PACKAGE_STATUS);
        fs::create_dir_all(package_status_path.parent().expect("package status parent"))
            .expect("package status dir");
        let mut visibility = serde_json::json!({
            "official_package_name": AI_SDK_OFFICIAL_NAME,
            "package_id": AI_SDK_PACKAGE_ID,
            "upstream_package": "ai",
            "upstream_version": "7.0.0-canary.146",
            "source_mirror": "G:/WWW/inspirations/vercel-ai",
            "status": "present",
            "receipt_status": "present",
            "package_receipt_path": AI_SDK_LAUNCH_ASSISTANT_RECEIPT,
            "selected_surfaces": [
                {
                    "surface_id": "ai-launch-assistant-dashboard-workflow",
                    "status": "present",
                    "hash_algorithm": "sha256",
                    "files": [
                        "components/launch/ai-chat-status.tsx"
                    ],
                    "file_hashes": {
                        "components/launch/ai-chat-status.tsx": format!("sha256:{expected_hash}"),
                        "core/src/ecosystem/forge_vercel_ai.rs": "sha256:0000000000000000000000000000000000000000000000000000000000000000",
                        "docs/packages/ai-vercel-ai.md": "sha256:0000000000000000000000000000000000000000000000000000000000000000",
                        "upstream:packages/ai/src/generate-text/stream-text.ts": "sha256:0000000000000000000000000000000000000000000000000000000000000000"
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
                "generated_css": "styles/generated.css",
                "visible_surfaces": [
                    "launch-ai-assistant-dashboard-workflow"
                ],
                "source_files": [
                    "components/launch/ai-chat-status.tsx"
                ],
                "runtime_proof": false,
                "data_dx_markers": [
                    "data-dx-style-surface=\"ai-sdk\"",
                    "data-dx-token-scope=\"ai/vercel-ai\""
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
