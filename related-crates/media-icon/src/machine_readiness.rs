use crate::machine_catalog::{
    project_root_for_index_output, read_icon_catalog_machine_cache, read_icon_prefix_machine_cache,
    write_icon_catalog_prefix_machine_caches,
};
use crate::machine_manifest::{
    read_icon_manifest_machine_cache_summary, write_icon_manifest_machine_cache,
};
use crate::machine_pack_body::{
    read_icon_pack_body_machine_cache, write_icon_pack_body_machine_cache,
};
use crate::machine_precomputed::{
    read_icon_bloom_machine_cache, read_icon_lowercase_cache_machine_cache,
    read_icon_perfect_hash_machine_cache, write_icon_bloom_machine_cache,
    write_icon_lowercase_cache_machine_cache, write_icon_perfect_hash_machine_cache,
};
use crate::parser::parse_icon_files;
use crate::types::IconMetadata;
use anyhow::{Context, Result, anyhow};
use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

pub const ICON_MACHINE_CACHE_READINESS_SCHEMA: &str = "dx.icon.machine_cache_readiness.v1";
pub const ICON_MACHINE_CACHE_READINESS_RECEIPT_SCHEMA: &str =
    "dx.performance.json_machine_cache_receipt.media_icon_existing_cache_readiness.v1";
pub const ICON_MACHINE_CACHE_READINESS_RECEIPT_PATH: &str =
    ".dx/performance/json-machine-cache-receipts/media-icon-existing-cache-readiness.json";
pub const REQUIRED_ICON_MACHINE_CACHE_NAMES: [&str; 7] = [
    "manifest",
    "catalog",
    "prefix",
    "perfect-hash",
    "bloom",
    "lowercase-cache",
    "pack-body",
];

static ATOMIC_WRITE_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IconMachineCacheReadinessReport {
    pub schema: &'static str,
    pub project_root: PathBuf,
    pub data_dir: PathBuf,
    pub receipt_path: PathBuf,
    pub parsed_icon_count: usize,
    pub required_cache_count: usize,
    pub all_required_caches_ready: bool,
    pub stale_or_missing_caches_regenerated: bool,
    pub write_errors: Vec<String>,
    pub entries: Vec<IconMachineCacheReadinessEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IconMachineCacheReadinessEntry {
    pub cache_name: &'static str,
    pub machine_path: PathBuf,
    pub before_ready: bool,
    pub ready: bool,
    pub regenerated: bool,
    pub read_mode: Option<&'static str>,
    pub fallback_reason: Option<String>,
}

pub fn ensure_icon_machine_caches_for_index_output(
    data_dir: &Path,
    output_dir: &Path,
    icons: &[IconMetadata],
) -> Result<IconMachineCacheReadinessReport> {
    let project_root = project_root_for_index_output(output_dir)?;
    ensure_icon_machine_caches(&project_root, data_dir, icons)
}

pub fn ensure_icon_machine_caches_from_dir(
    project_root: &Path,
    data_dir: &Path,
) -> Result<IconMachineCacheReadinessReport> {
    let icons = parse_icon_files(data_dir)?;
    ensure_icon_machine_caches(project_root, data_dir, &icons)
}

pub fn ensure_icon_machine_caches(
    project_root: &Path,
    data_dir: &Path,
    icons: &[IconMetadata],
) -> Result<IconMachineCacheReadinessReport> {
    let before = collect_icon_machine_cache_statuses(project_root, data_dir);

    let mut write_errors = Vec::new();
    record_write_result(
        &mut write_errors,
        "manifest",
        write_icon_manifest_machine_cache(project_root, data_dir),
    );
    record_write_result(
        &mut write_errors,
        "catalog-prefix",
        write_icon_catalog_prefix_machine_caches(project_root, data_dir, icons)
            .map(|_| icon_machine_root(project_root).join("catalog.machine")),
    );
    record_write_result(
        &mut write_errors,
        "perfect-hash",
        write_icon_perfect_hash_machine_cache(project_root, data_dir, icons),
    );
    record_write_result(
        &mut write_errors,
        "bloom",
        write_icon_bloom_machine_cache(project_root, data_dir, icons),
    );
    record_write_result(
        &mut write_errors,
        "lowercase-cache",
        write_icon_lowercase_cache_machine_cache(project_root, data_dir, icons),
    );
    record_write_result(
        &mut write_errors,
        "pack-body",
        write_icon_pack_body_machine_cache(project_root, data_dir),
    );

    let after = collect_icon_machine_cache_statuses(project_root, data_dir);
    let entries = merge_readiness_statuses(&before, &after);
    let stale_or_missing_caches_regenerated = entries.iter().any(|entry| entry.regenerated);
    let report = IconMachineCacheReadinessReport {
        schema: ICON_MACHINE_CACHE_READINESS_SCHEMA,
        project_root: project_root.to_path_buf(),
        data_dir: data_dir.to_path_buf(),
        receipt_path: machine_cache_readiness_receipt_path(project_root),
        parsed_icon_count: icons.len(),
        required_cache_count: REQUIRED_ICON_MACHINE_CACHE_NAMES.len(),
        all_required_caches_ready: after.iter().all(|entry| entry.ready),
        stale_or_missing_caches_regenerated,
        write_errors,
        entries,
    };

    write_machine_cache_readiness_receipt(&report)?;
    Ok(report)
}

fn collect_icon_machine_cache_statuses(
    project_root: &Path,
    data_dir: &Path,
) -> Vec<IconMachineCacheReadinessEntry> {
    vec![
        cache_status(
            project_root,
            "manifest",
            "manifest.machine",
            read_icon_manifest_machine_cache_summary(project_root, data_dir)
                .map(|summary| summary.mode)
                .ok_or_else(|| anyhow!("manifest machine cache missing, stale, or rejected")),
        ),
        cache_status(
            project_root,
            "catalog",
            "catalog.machine",
            read_icon_catalog_machine_cache(project_root, data_dir).map(|read| read.mode),
        ),
        cache_status(
            project_root,
            "prefix",
            "prefix.machine",
            read_icon_prefix_machine_cache(project_root, data_dir).map(|read| read.mode),
        ),
        cache_status(
            project_root,
            "perfect-hash",
            "perfect-hash.machine",
            read_icon_perfect_hash_machine_cache(project_root, data_dir).map(|read| read.mode),
        ),
        cache_status(
            project_root,
            "bloom",
            "bloom.machine",
            read_icon_bloom_machine_cache(project_root, data_dir).map(|read| read.mode),
        ),
        cache_status(
            project_root,
            "lowercase-cache",
            "lowercase-cache.machine",
            read_icon_lowercase_cache_machine_cache(project_root, data_dir).map(|read| read.mode),
        ),
        cache_status(
            project_root,
            "pack-body",
            "pack-body.machine",
            read_icon_pack_body_machine_cache(project_root, data_dir).map(|read| read.mode),
        ),
    ]
}

fn cache_status(
    project_root: &Path,
    cache_name: &'static str,
    machine_file: &str,
    read_result: Result<&'static str>,
) -> IconMachineCacheReadinessEntry {
    let machine_path = icon_machine_root(project_root).join(machine_file);
    match read_result {
        Ok(read_mode) => IconMachineCacheReadinessEntry {
            cache_name,
            machine_path,
            before_ready: false,
            ready: true,
            regenerated: false,
            read_mode: Some(read_mode),
            fallback_reason: None,
        },
        Err(error) => {
            let fallback_reason = if machine_path.exists() {
                format!("machine_cache_rejected_or_stale: {error:#}")
            } else {
                "machine_cache_missing".to_string()
            };
            IconMachineCacheReadinessEntry {
                cache_name,
                machine_path,
                before_ready: false,
                ready: false,
                regenerated: false,
                read_mode: None,
                fallback_reason: Some(fallback_reason),
            }
        }
    }
}

fn merge_readiness_statuses(
    before: &[IconMachineCacheReadinessEntry],
    after: &[IconMachineCacheReadinessEntry],
) -> Vec<IconMachineCacheReadinessEntry> {
    after
        .iter()
        .cloned()
        .map(|mut entry| {
            if let Some(before_entry) = before
                .iter()
                .find(|before_entry| before_entry.cache_name == entry.cache_name)
            {
                entry.before_ready = before_entry.ready;
                entry.regenerated = !before_entry.ready && entry.ready;
            }
            entry
        })
        .collect()
}

fn record_write_result(
    write_errors: &mut Vec<String>,
    cache_name: &'static str,
    write_result: Result<PathBuf>,
) {
    if let Err(error) = write_result {
        write_errors.push(format!("{cache_name}: {error:#}"));
    }
}

fn write_machine_cache_readiness_receipt(report: &IconMachineCacheReadinessReport) -> Result<()> {
    let entries = report
        .entries
        .iter()
        .map(|entry| {
            json!({
                "cache_name": entry.cache_name,
                "machine_path": normalize_path(&entry.machine_path),
                "before_ready": entry.before_ready,
                "ready": entry.ready,
                "regenerated": entry.regenerated,
                "read_mode": entry.read_mode,
                "fallback_reason": entry.fallback_reason,
            })
        })
        .collect::<Vec<_>>();
    let receipt = json!({
        "schema": ICON_MACHINE_CACHE_READINESS_RECEIPT_SCHEMA,
        "cache_name": "media-icon-existing-cache-readiness",
        "cache_kind": "existing-machine-cache-readiness",
        "readiness_schema": report.schema,
        "source_data_root": normalize_path(&report.data_dir),
        "machine_cache_root": normalize_path(&icon_machine_root(&report.project_root)),
        "parsed_icon_count": report.parsed_icon_count,
        "required_cache_count": report.required_cache_count,
        "required_cache_names": REQUIRED_ICON_MACHINE_CACHE_NAMES,
        "all_required_caches_ready": report.all_required_caches_ready,
        "stale_or_missing_caches_regenerated": report.stale_or_missing_caches_regenerated,
        "cache_migration_performed": report.stale_or_missing_caches_regenerated,
        "write_errors": report.write_errors,
        "entries": entries,
        "json_source_authoritative": true,
        "generated_machine_cache_only": true,
        "normal_search_behavior_changed": false,
        "full_startup_search_render_proof": false,
        "upstream_baseline_measured": false,
        "faster_than_upstream_claimed": false,
        "same_machine_benchmark_required": true,
        "test_command": null,
        "test_command_recorded": false,
        "suggested_test_command": "cargo check --manifest-path G:\\Dx\\www\\Cargo.toml -p dx-icons --locked --no-default-features --features rayon --lib --bin build_index -j 1 --message-format=short --color never",
        "generated_at_unix_ms": current_unix_ms(),
        "machine": {
            "os": std::env::consts::OS,
            "arch": std::env::consts::ARCH
        }
    });

    if let Some(parent) = report.receipt_path.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!(
                "create media-icon machine readiness receipt directory {}",
                parent.display()
            )
        })?;
    }
    write_atomic(
        &report.receipt_path,
        &serde_json::to_vec_pretty(&receipt)
            .context("serialize media-icon machine readiness receipt")?,
    )
    .with_context(|| {
        format!(
            "write media-icon machine readiness receipt {}",
            report.receipt_path.display()
        )
    })
}

fn machine_cache_readiness_receipt_path(project_root: &Path) -> PathBuf {
    project_root.join(ICON_MACHINE_CACHE_READINESS_RECEIPT_PATH)
}

fn icon_machine_root(project_root: &Path) -> PathBuf {
    project_root.join(".dx/icon/machine/v1")
}

fn normalize_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn current_unix_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .and_then(|duration| u64::try_from(duration.as_millis()).ok())
        .unwrap_or_default()
}

fn write_atomic(path: &Path, bytes: &[u8]) -> Result<()> {
    let tmp = atomic_temp_path(path);
    fs::write(&tmp, bytes).with_context(|| format!("write temp {}", tmp.display()))?;
    fs::rename(&tmp, path)
        .with_context(|| format!("replace {} with {}", path.display(), tmp.display()))?;
    Ok(())
}

fn atomic_temp_path(path: &Path) -> PathBuf {
    let file_name = path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("media-icon-existing-cache-readiness.json");
    let nonce = ATOMIC_WRITE_COUNTER.fetch_add(1, Ordering::Relaxed);
    path.with_file_name(format!(
        "{}.tmp.{}.{}.{}",
        file_name,
        std::process::id(),
        current_unix_ms(),
        nonce
    ))
}
