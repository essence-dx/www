use std::fs;
use std::path::{Component, Path, PathBuf};

use anyhow::Context;
use chrono::Utc;
use serde::Serialize;

use crate::error::{DxError, DxResult};

use super::forge_error;
use super::forge_static_page_assets::{DxForgeStaticPageAsset, DxForgeStaticPageAssetManifest};
use super::markdown_table_cell;
use super::options::{DxOutputFormat, parse_score_threshold, resolve_cli_path};

const LONG_LIVED_CACHE_HEADER: &str = "public, max-age=31536000, immutable";

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeStaticAssetMaterializationReport {
    version: u32,
    generated_at: String,
    pub(super) passed: bool,
    status: String,
    pub(super) score: u8,
    fail_under: u8,
    project: PathBuf,
    manifest_path: PathBuf,
    public_dir: PathBuf,
    mode: String,
    asset_count: u64,
    copied_asset_count: u64,
    planned_asset_count: u64,
    unresolved_asset_count: u64,
    hash_verified_count: u64,
    cache_policy: DxForgeStaticAssetCachePolicy,
    no_node_modules: bool,
    package_installs_run: bool,
    assets: Vec<DxForgeStaticAssetMaterializationAsset>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

pub(super) fn cmd_forge_materialize_static_assets(cwd: &Path, args: &[String]) -> DxResult<()> {
    let mut project: Option<PathBuf> = None;
    let mut manifest: Option<PathBuf> = None;
    let mut public_dir: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut fail_under = 90u8;
    let mut write = false;
    let mut dry_run = false;
    let mut quiet = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--project" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    static_asset_materialization_error("--project requires a path", "project")
                })?;
                project = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--manifest" | "--asset-manifest" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    static_asset_materialization_error("--manifest requires a path", "manifest")
                })?;
                manifest = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--public-dir" | "--out-dir" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    static_asset_materialization_error(
                        "--public-dir requires a directory",
                        "public-dir",
                    )
                })?;
                public_dir = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--output" | "--out" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    static_asset_materialization_error("--output requires a path", "output")
                })?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--format" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    static_asset_materialization_error("--format requires a value", "format")
                })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--fail-under" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    static_asset_materialization_error(
                        "--fail-under requires a score",
                        "fail-under",
                    )
                })?;
                fail_under = parse_score_threshold(value)?;
                index += 2;
            }
            "--write" => {
                write = true;
                index += 1;
            }
            "--dry-run" => {
                dry_run = true;
                index += 1;
            }
            "--quiet" => {
                quiet = true;
                index += 1;
            }
            value if value.starts_with('-') => {
                return Err(static_asset_materialization_error(
                    format!("Unknown forge materialize-static-assets option: {value}"),
                    "forge materialize-static-assets",
                ));
            }
            value => {
                if manifest.is_none() {
                    manifest = Some(resolve_cli_path(cwd, value));
                } else {
                    return Err(static_asset_materialization_error(
                        format!("Unexpected forge materialize-static-assets argument: {value}"),
                        "forge materialize-static-assets",
                    ));
                }
                index += 1;
            }
        }
    }

    if write && dry_run {
        return Err(static_asset_materialization_error(
            "Choose either --dry-run or --write, not both",
            "forge materialize-static-assets",
        ));
    }

    let project = project.unwrap_or_else(|| cwd.to_path_buf());
    let manifest = manifest.ok_or_else(|| {
        static_asset_materialization_error(
            "dx forge materialize-static-assets requires --manifest <asset-manifest.json>",
            "manifest",
        )
    })?;
    let public_dir = public_dir.unwrap_or_else(|| project.join("public"));
    let report = build_forge_static_asset_materialization_report(
        &project,
        &manifest,
        &public_dir,
        write && !dry_run,
        fail_under,
    )
    .map_err(forge_error)?;
    let rendered = match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
        DxOutputFormat::Terminal => forge_static_asset_materialization_terminal(&report),
        DxOutputFormat::Markdown => forge_static_asset_materialization_markdown(&report),
    };

    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).map_err(forge_error)?;
        }
        fs::write(&output, &rendered).map_err(forge_error)?;
    }

    if !quiet {
        println!("{rendered}");
    }

    if !report.passed {
        return Err(static_asset_materialization_error(
            forge_static_asset_materialization_failure_summary(&report),
            "forge materialize-static-assets",
        ));
    }

    if report.score < fail_under {
        return Err(static_asset_materialization_error(
            format!(
                "DX Forge materialize-static-assets score {} is below fail-under threshold {}",
                report.score, fail_under
            ),
            "forge materialize-static-assets",
        ));
    }

    Ok(())
}

fn static_asset_materialization_error(message: impl Into<String>, field: &str) -> DxError {
    DxError::ConfigValidationError {
        message: message.into(),
        field: Some(field.to_string()),
    }
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeStaticAssetCachePolicy {
    long_lived_header: String,
    immutable_required: bool,
    reviewed_cache_hint: String,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeStaticAssetMaterializationAsset {
    source_url: String,
    copied_target_path: String,
    source_file_path: Option<PathBuf>,
    target_path: PathBuf,
    write_state: String,
    hash_verified: bool,
    source_hash: Option<String>,
    copied_hash: Option<String>,
    byte_size: Option<u64>,
    copied_byte_size: Option<u64>,
    cache_hint: String,
    cache_header: String,
    optimization_state: String,
    alt_text_review_state: String,
    note: String,
}

pub(super) fn build_forge_static_asset_materialization_report(
    project: &Path,
    manifest_path: &Path,
    public_dir: &Path,
    write: bool,
    fail_under: u8,
) -> anyhow::Result<DxForgeStaticAssetMaterializationReport> {
    let manifest = read_static_asset_manifest(manifest_path)?;
    let mode = if write { "write" } else { "dry-run" }.to_string();
    let mut assets = Vec::new();
    let mut findings = Vec::new();

    for asset in &manifest.assets {
        let materialized =
            materialize_static_asset(public_dir, asset, write).with_context(|| {
                format!(
                    "materialize static asset `{}` from `{}`",
                    asset.source_url,
                    manifest_path.display()
                )
            })?;
        if materialized.write_state == "blocked" {
            findings.push(format!(
                "`{}` target path is unsafe: {}",
                materialized.source_url,
                materialized.target_path.display()
            ));
        }
        if materialized.write_state == "hash-mismatch" {
            findings.push(format!(
                "`{}` copied hash did not match the source manifest",
                materialized.source_url
            ));
        }
        assets.push(materialized);
    }

    let no_node_modules =
        !project.join("node_modules").exists() && !public_dir.join("node_modules").exists();
    let package_installs_run = false;
    if !no_node_modules {
        findings.push(
            "node_modules exists in the asset materialization project or public output."
                .to_string(),
        );
    }

    let copied_asset_count = assets
        .iter()
        .filter(|asset| asset.write_state == "copied")
        .count() as u64;
    let planned_asset_count = assets
        .iter()
        .filter(|asset| asset.write_state == "planned")
        .count() as u64;
    let unresolved_asset_count = assets
        .iter()
        .filter(|asset| asset.write_state == "unresolved")
        .count() as u64;
    let hash_verified_count = assets.iter().filter(|asset| asset.hash_verified).count() as u64;
    let mut score = if assets.is_empty() { 0 } else { 100u8 };
    let unresolved_penalty = unresolved_asset_count.saturating_mul(5).min(100) as u8;
    score = score.saturating_sub(unresolved_penalty);
    if !no_node_modules {
        score = score.saturating_sub(40);
    }
    if findings
        .iter()
        .any(|finding| finding.contains("unsafe") || finding.contains("hash"))
    {
        score = score.saturating_sub(35);
    }
    let passed = findings.is_empty()
        && !assets.is_empty()
        && no_node_modules
        && !package_installs_run
        && score >= fail_under
        && (write || planned_asset_count > 0)
        && (!write || copied_asset_count > 0 || unresolved_asset_count == assets.len() as u64);
    let status = if !passed {
        "blocked"
    } else if unresolved_asset_count > 0 {
        "needs-review"
    } else if write {
        "materialized"
    } else {
        "planned"
    }
    .to_string();

    Ok(DxForgeStaticAssetMaterializationReport {
        version: 1,
        generated_at: Utc::now().to_rfc3339(),
        passed,
        status,
        score,
        fail_under,
        project: project.to_path_buf(),
        manifest_path: manifest_path.to_path_buf(),
        public_dir: public_dir.to_path_buf(),
        mode,
        asset_count: assets.len() as u64,
        copied_asset_count,
        planned_asset_count,
        unresolved_asset_count,
        hash_verified_count,
        cache_policy: DxForgeStaticAssetCachePolicy {
            long_lived_header: LONG_LIVED_CACHE_HEADER.to_string(),
            immutable_required: true,
            reviewed_cache_hint: "copy-optimize-cache-long".to_string(),
        },
        no_node_modules,
        package_installs_run,
        assets,
        findings,
        next_commands: vec![
            "dx forge materialize-static-assets --manifest <asset-manifest.json> --public-dir public --write --format markdown".to_string(),
            "Review unresolved media gaps before production publish.".to_string(),
            "Serve `/assets/migrated/**` with `Cache-Control: public, max-age=31536000, immutable` after fingerprint review.".to_string(),
        ],
    })
}

pub(super) fn forge_static_asset_materialization_terminal(
    report: &DxForgeStaticAssetMaterializationReport,
) -> String {
    let mut output = String::new();
    output.push_str("DX Forge Static Asset Materialization\n");
    output.push_str(&format!(
        "Status: {} | Score: {} / 100 | Passed: {}\n",
        report.status, report.score, report.passed
    ));
    output.push_str(&format!(
        "Mode: {} | Assets: {} | Copied: {} | Unresolved: {} | no node_modules: {}\n",
        report.mode,
        report.asset_count,
        report.copied_asset_count,
        report.unresolved_asset_count,
        report.no_node_modules
    ));
    for asset in &report.assets {
        output.push_str(&format!(
            "- {} -> {} ({}, hash verified={})\n",
            asset.source_url,
            asset.target_path.display(),
            asset.write_state,
            asset.hash_verified
        ));
    }
    if !report.findings.is_empty() {
        output.push_str("Findings:\n");
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }
    output
}

pub(super) fn forge_static_asset_materialization_markdown(
    report: &DxForgeStaticAssetMaterializationReport,
) -> String {
    let mut output = format!(
        "# DX Forge Static Asset Materialization\n\n- Generated: `{}`\n- Passed: `{}`\n- Status: `{}`\n- Score: `{}` / `100`\n- Required score: `{}` / `100`\n- Mode: `{}`\n- Manifest: `{}`\n- Public dir: `{}`\n- Assets: `{}`\n- Copied: `{}`\n- Unresolved: `{}`\n- No `node_modules`: `{}`\n- Package installs run: `{}`\n- Cache-Control: `{}`\n\n",
        report.generated_at,
        report.passed,
        report.status,
        report.score,
        report.fail_under,
        report.mode,
        markdown_table_cell(&report.manifest_path.display().to_string()),
        markdown_table_cell(&report.public_dir.display().to_string()),
        report.asset_count,
        report.copied_asset_count,
        report.unresolved_asset_count,
        report.no_node_modules,
        report.package_installs_run,
        report.cache_policy.long_lived_header
    );

    output.push_str("| Source | State | Target | Hash | Cache | Optimization |\n");
    output.push_str("| --- | --- | --- | --- | --- | --- |\n");
    for asset in &report.assets {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
            markdown_table_cell(&asset.source_url),
            asset.write_state,
            markdown_table_cell(&asset.target_path.display().to_string()),
            asset.hash_verified,
            markdown_table_cell(&asset.cache_header),
            markdown_table_cell(&asset.optimization_state)
        ));
    }

    if !report.findings.is_empty() {
        output.push_str("\n## Findings\n\n");
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }

    output.push_str("\n## Next Commands\n\n");
    for command in &report.next_commands {
        output.push_str(&format!("- `{}`\n", markdown_table_cell(command)));
    }

    output
}

pub(super) fn forge_static_asset_materialization_failure_summary(
    report: &DxForgeStaticAssetMaterializationReport,
) -> String {
    if report.findings.is_empty() {
        format!(
            "DX Forge static asset materialization failed with score {}",
            report.score
        )
    } else {
        report.findings.join("; ")
    }
}

fn read_static_asset_manifest(path: &Path) -> anyhow::Result<DxForgeStaticPageAssetManifest> {
    serde_json::from_slice(&fs::read(path).with_context(|| format!("read `{}`", path.display()))?)
        .with_context(|| format!("parse static page asset manifest `{}`", path.display()))
}

fn materialize_static_asset(
    public_dir: &Path,
    asset: &DxForgeStaticPageAsset,
    write: bool,
) -> anyhow::Result<DxForgeStaticAssetMaterializationAsset> {
    let target_path = safe_public_asset_target(public_dir, &asset.copied_target_path)?;
    let cache_header = cache_header_for_hint(&asset.cache_hint).to_string();
    if asset.unresolved || asset.source_file_path.is_none() {
        return Ok(DxForgeStaticAssetMaterializationAsset {
            source_url: asset.source_url.clone(),
            copied_target_path: asset.copied_target_path.clone(),
            source_file_path: asset.source_file_path.clone(),
            target_path,
            write_state: "unresolved".to_string(),
            hash_verified: false,
            source_hash: asset.hash.clone(),
            copied_hash: None,
            byte_size: asset.byte_size,
            copied_byte_size: None,
            cache_hint: asset.cache_hint.clone(),
            cache_header,
            optimization_state: "blocked-unresolved-source".to_string(),
            alt_text_review_state: asset.alt_text_review_state.clone(),
            note: asset.note.clone(),
        });
    }

    let source_path = asset
        .source_file_path
        .clone()
        .expect("checked source file path");
    let bytes = fs::read(&source_path)
        .with_context(|| format!("read source asset `{}`", source_path.display()))?;
    let copied_hash = format!("blake3:{}", blake3::hash(&bytes).to_hex());
    let hash_verified = asset
        .hash
        .as_deref()
        .map(|hash| hash == copied_hash)
        .unwrap_or(false);
    let write_state = if !hash_verified {
        "hash-mismatch"
    } else if write {
        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent).with_context(|| format!("create `{}`", parent.display()))?;
        }
        fs::write(&target_path, &bytes)
            .with_context(|| format!("write asset `{}`", target_path.display()))?;
        "copied"
    } else {
        "planned"
    };
    let copied_byte_size = if write_state == "copied" {
        Some(bytes.len() as u64)
    } else {
        None
    };
    let optimization_state = match write_state {
        "copied" => "copied-lossless-original-with-cache-policy",
        "planned" => "planned-lossless-original-with-cache-policy",
        "hash-mismatch" => "blocked-hash-mismatch",
        _ => "blocked-review-required",
    };

    Ok(DxForgeStaticAssetMaterializationAsset {
        source_url: asset.source_url.clone(),
        copied_target_path: asset.copied_target_path.clone(),
        source_file_path: Some(source_path),
        target_path,
        write_state: write_state.to_string(),
        hash_verified,
        source_hash: asset.hash.clone(),
        copied_hash: Some(copied_hash),
        byte_size: asset.byte_size,
        copied_byte_size,
        cache_hint: asset.cache_hint.clone(),
        cache_header,
        optimization_state: optimization_state.to_string(),
        alt_text_review_state: asset.alt_text_review_state.clone(),
        note: asset.note.clone(),
    })
}

fn safe_public_asset_target(
    public_dir: &Path,
    copied_target_path: &str,
) -> anyhow::Result<PathBuf> {
    let mut target = public_dir.to_path_buf();
    for component in Path::new(copied_target_path.trim_start_matches(['/', '\\'])).components() {
        match component {
            Component::Normal(segment) => target.push(segment),
            Component::CurDir => {}
            _ => anyhow::bail!("unsafe copied asset target `{copied_target_path}`"),
        }
    }
    if target == public_dir {
        anyhow::bail!("empty copied asset target `{copied_target_path}`");
    }
    Ok(target)
}

fn cache_header_for_hint(cache_hint: &str) -> &'static str {
    if cache_hint == "copy-optimize-cache-long" {
        LONG_LIVED_CACHE_HEADER
    } else {
        "public, max-age=3600"
    }
}
