use std::collections::BTreeSet;
use std::path::{Component, Path, PathBuf};

use anyhow::{Context, bail};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxBuildRollbackVerificationReport {
    pub passed: bool,
    pub previous_build_dir: String,
    pub current_build_dir: String,
    pub strategy: String,
    pub previous_manifest_hash: String,
    pub current_manifest_hash: String,
    pub manifests_differ: bool,
    pub current_rollback_metadata_present: bool,
    pub previous_release_required: bool,
    pub previous_immutable_assets_total: usize,
    pub previous_immutable_assets_restorable: usize,
    pub previous_immutable_assets: Vec<DxBuildRollbackAsset>,
    pub missing_previous_assets: Vec<String>,
    pub restore_order: Vec<String>,
    pub findings: Vec<DxBuildRollbackFinding>,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxBuildRollbackAsset {
    pub path: String,
    pub exists: bool,
    pub bytes: u64,
    pub hash: Option<String>,
    pub cache_control: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxBuildRollbackFinding {
    pub severity: String,
    pub code: String,
    pub message: String,
}

pub(super) fn verify_build_rollback(
    previous_build_dir: &Path,
    current_build_dir: &Path,
) -> anyhow::Result<DxBuildRollbackVerificationReport> {
    let previous_deploy = read_json(previous_build_dir.join(".dx/build-cache/deploy-adapter.json"))?;
    let current_deploy = read_json(current_build_dir.join(".dx/build-cache/deploy-adapter.json"))?;
    let previous_manifest_path = previous_build_dir.join(".dx/build-cache/manifest.json");
    let current_manifest_path = current_build_dir.join(".dx/build-cache/manifest.json");
    let previous_manifest_hash = blake3_file_hash(&previous_manifest_path)?;
    let current_manifest_hash = blake3_file_hash(&current_manifest_path)?;
    let current_rollback_path = current_build_dir.join(
        current_deploy["rollback"]["metadata_path"]
            .as_str()
            .unwrap_or(".dx/build-cache/rollback.json"),
    );
    let current_rollback_metadata_present = current_rollback_path.exists();
    let current_rollback = if current_rollback_metadata_present {
        read_json(current_rollback_path)?
    } else {
        serde_json::Value::Null
    };
    let strategy = json_string(&current_rollback["strategy"])
        .or_else(|| json_string(&current_deploy["rollback"]["strategy"]))
        .unwrap_or_else(|| "manifest-pinned-asset-rollback".to_string());
    let previous_release_required = current_rollback["previous_release_required"]
        .as_bool()
        .unwrap_or(false);
    let restore_order = current_rollback["restore_order"]
        .as_array()
        .map(|steps| {
            steps
                .iter()
                .filter_map(json_string)
                .collect::<Vec<String>>()
        })
        .filter(|steps| !steps.is_empty())
        .unwrap_or_else(|| {
            vec![
                "immutable_assets".to_string(),
                ".dx/build-cache/manifest.json".to_string(),
                ".dx/build-cache/deploy-adapter.json".to_string(),
            ]
        });

    let mut previous_immutable_assets = Vec::new();
    let mut missing_previous_assets = Vec::new();
    let mut seen_assets = BTreeSet::new();
    for asset in previous_deploy["immutable_assets"]
        .as_array()
        .into_iter()
        .flatten()
    {
        let Some(path) = asset["path"].as_str() else {
            continue;
        };
        validate_relative_artifact_path(path)?;
        if !seen_assets.insert(path.to_string()) {
            continue;
        }
        let artifact_path = previous_build_dir.join(path);
        let exists = artifact_path.is_file();
        let (bytes, hash) = if exists {
            let metadata = std::fs::metadata(&artifact_path)
                .with_context(|| format!("read metadata for {}", artifact_path.display()))?;
            (metadata.len(), Some(blake3_file_hash(&artifact_path)?))
        } else {
            missing_previous_assets.push(path.to_string());
            (0, None)
        };
        previous_immutable_assets.push(DxBuildRollbackAsset {
            path: path.to_string(),
            exists,
            bytes,
            hash,
            cache_control: json_string(&asset["cache_control"]),
        });
    }

    let previous_immutable_assets_total = previous_immutable_assets.len();
    let previous_immutable_assets_restorable = previous_immutable_assets
        .iter()
        .filter(|asset| asset.exists)
        .count();
    let previous_manifest_matches_deploy =
        previous_deploy["build_manifest"]["hash"].as_str() == Some(previous_manifest_hash.as_str());
    let current_manifest_matches_deploy =
        current_deploy["build_manifest"]["hash"].as_str() == Some(current_manifest_hash.as_str());

    let mut findings = Vec::new();
    push_finding(
        &mut findings,
        previous_manifest_matches_deploy,
        "previous-manifest-hash",
        "Previous manifest hash matches .dx/build-cache/deploy-adapter.json.",
        "Previous manifest hash does not match .dx/build-cache/deploy-adapter.json.",
    );
    push_finding(
        &mut findings,
        current_manifest_matches_deploy,
        "current-manifest-hash",
        "Current manifest hash matches .dx/build-cache/deploy-adapter.json.",
        "Current manifest hash does not match .dx/build-cache/deploy-adapter.json.",
    );
    push_finding(
        &mut findings,
        current_rollback_metadata_present,
        "current-rollback-metadata",
        "Current build includes rollback metadata.",
        "Current build is missing rollback metadata.",
    );
    push_finding(
        &mut findings,
        previous_release_required,
        "previous-release-required",
        "Current rollback metadata requires a previous release.",
        "Current rollback metadata does not require a previous release.",
    );
    push_finding(
        &mut findings,
        missing_previous_assets.is_empty(),
        "previous-immutable-assets",
        "All previous immutable assets are present and restorable.",
        "One or more previous immutable assets are missing.",
    );

    let passed = previous_manifest_matches_deploy
        && current_manifest_matches_deploy
        && current_rollback_metadata_present
        && previous_release_required
        && missing_previous_assets.is_empty();

    let manifests_differ = previous_manifest_hash != current_manifest_hash;

    Ok(DxBuildRollbackVerificationReport {
        passed,
        previous_build_dir: previous_build_dir.display().to_string(),
        current_build_dir: current_build_dir.display().to_string(),
        strategy,
        previous_manifest_hash,
        current_manifest_hash,
        manifests_differ,
        current_rollback_metadata_present,
        previous_release_required,
        previous_immutable_assets_total,
        previous_immutable_assets_restorable,
        previous_immutable_assets,
        missing_previous_assets,
        restore_order,
        findings,
    })
}

pub(super) fn build_rollback_verification_terminal(
    report: &DxBuildRollbackVerificationReport,
) -> String {
    let status = if report.passed { "PASS" } else { "FAIL" };
    format!(
        "DX build rollback verification: {status}\nPrevious build: {}\nCurrent build: {}\nStrategy: {}\nPrevious assets restorable: {}/{}\nMissing previous assets: {}\n",
        report.previous_build_dir,
        report.current_build_dir,
        report.strategy,
        report.previous_immutable_assets_restorable,
        report.previous_immutable_assets_total,
        report.missing_previous_assets.len()
    )
}

pub(super) fn build_rollback_verification_markdown(
    report: &DxBuildRollbackVerificationReport,
) -> String {
    let mut output = format!(
        "# DX Build Rollback Verification\n\n- Passed: `{}`\n- Previous build: `{}`\n- Current build: `{}`\n- Strategy: `{}`\n- Previous assets restorable: `{}` / `{}`\n- Missing previous assets: `{}`\n\n",
        report.passed,
        report.previous_build_dir,
        report.current_build_dir,
        report.strategy,
        report.previous_immutable_assets_restorable,
        report.previous_immutable_assets_total,
        report.missing_previous_assets.len()
    );
    output.push_str("## Restore Order\n\n");
    for step in &report.restore_order {
        output.push_str(&format!("- `{step}`\n"));
    }
    output.push_str("\n## Findings\n\n");
    for finding in &report.findings {
        output.push_str(&format!(
            "- `{}` `{}` {}\n",
            finding.severity, finding.code, finding.message
        ));
    }
    output
}

pub(super) fn build_rollback_verification_failure_summary(
    report: &DxBuildRollbackVerificationReport,
) -> String {
    if !report.missing_previous_assets.is_empty() {
        return format!(
            "rollback verify failed: missing previous immutable assets: {}",
            report.missing_previous_assets.join(", ")
        );
    }
    let failed = report
        .findings
        .iter()
        .filter(|finding| finding.severity == "error")
        .map(|finding| finding.code.as_str())
        .collect::<Vec<_>>();
    format!("rollback verify failed: {}", failed.join(", "))
}

fn read_json(path: PathBuf) -> anyhow::Result<serde_json::Value> {
    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("read rollback verification JSON {}", path.display()))?;
    serde_json::from_str(&text)
        .with_context(|| format!("parse rollback verification JSON {}", path.display()))
}

fn blake3_file_hash(path: &Path) -> anyhow::Result<String> {
    let bytes = std::fs::read(path).with_context(|| format!("read {}", path.display()))?;
    Ok(format!("blake3:{}", blake3::hash(&bytes).to_hex()))
}

fn validate_relative_artifact_path(path: &str) -> anyhow::Result<()> {
    let artifact = Path::new(path);
    if artifact.components().any(|component| {
        matches!(
            component,
            Component::Prefix(_) | Component::RootDir | Component::ParentDir
        )
    }) {
        bail!("rollback artifact path must be relative and stay inside build dir: {path}");
    }
    Ok(())
}

fn json_string(value: &serde_json::Value) -> Option<String> {
    value.as_str().map(ToOwned::to_owned)
}

fn push_finding(
    findings: &mut Vec<DxBuildRollbackFinding>,
    passed: bool,
    code: &str,
    pass_message: &str,
    fail_message: &str,
) {
    findings.push(DxBuildRollbackFinding {
        severity: if passed { "pass" } else { "error" }.to_string(),
        code: code.to_string(),
        message: if passed { pass_message } else { fail_message }.to_string(),
    });
}
