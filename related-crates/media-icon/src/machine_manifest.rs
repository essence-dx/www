use anyhow::{Context, Result, bail};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serializer::machine::{
    MachineCacheKind, MachineCachePaths, MachineCacheSchema, MachineCacheSource,
    MachineCacheWriteOptions, access_typed_machine_cache, open_typed_machine_cache,
    write_typed_machine_cache,
};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

const ICON_MANIFEST_CACHE_SCHEMA: &str = "dx.icon.manifest.v1";
const ICON_MANIFEST_PERFORMANCE_RECEIPT_SCHEMA: &str =
    "dx.performance.json_machine_cache_receipt.icon_manifest.v1";
const ICON_MANIFEST_PERFORMANCE_RECEIPT_PATH: &str =
    ".dx/performance/json-machine-cache-receipts/icon-.dx/build-cache/manifest.json";

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct IconManifestMachineV1 {
    pub selected_data_root: String,
    pub generated_at_unix_ms: u64,
    pub source_file_count: u32,
    pub source_total_bytes: u64,
    pub packs: Vec<IconPackManifestEntryV1>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct IconPackManifestEntryV1 {
    pub pack: String,
    pub rel_path: String,
    pub bytes: u64,
    pub modified_unix_ms: Option<u64>,
    pub blake3: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IconManifestReadSummary {
    pub source_file_count: usize,
    pub source_total_bytes: u64,
    pub mode: &'static str,
}

pub fn select_icon_data_dir(project_root: &Path) -> Option<PathBuf> {
    candidate_icon_data_dirs(project_root)
        .into_iter()
        .find(|path| path.is_dir())
}

pub fn candidate_icon_data_dirs(project_root: &Path) -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    if let Ok(env_path) = std::env::var("DX_ICON_DATA")
        && !env_path.trim().is_empty()
    {
        candidates.push(PathBuf::from(env_path));
    }

    candidates.push(project_root.join("data"));
    candidates.push(project_root.join("related-crates/media-icon/data"));
    candidates.push(project_root.join("../icon/data"));
    candidates.push(project_root.join("../media/icon/data"));

    if let Some(home) = std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME")) {
        candidates.push(PathBuf::from(home).join(".dx/icon/data"));
    }

    candidates
}

pub fn write_icon_manifest_machine_cache(project_root: &Path, data_dir: &Path) -> Result<PathBuf> {
    let (manifest, source) = build_icon_manifest(data_dir)?;
    let paths = icon_manifest_machine_paths(project_root, data_dir)?;
    let receipt = write_typed_machine_cache(
        &manifest,
        &source,
        &paths,
        icon_manifest_schema(),
        MachineCacheWriteOptions::default(),
    )
    .with_context(|| {
        format!(
            "write icon manifest machine cache {}",
            paths.machine.display()
        )
    })?;
    Ok(receipt.machine)
}

pub fn write_icon_manifest_machine_cache_with_performance_receipt(
    project_root: &Path,
    data_dir: &Path,
) -> Result<(PathBuf, PathBuf)> {
    let source_scan_started = Instant::now();
    let (manifest, source) = build_icon_manifest(data_dir)?;
    let source_scan_ns = duration_nanos_u64(source_scan_started.elapsed());

    let paths = icon_manifest_machine_paths(project_root, data_dir)?;
    let generation_started = Instant::now();
    let receipt = write_typed_machine_cache(
        &manifest,
        &source,
        &paths,
        icon_manifest_schema(),
        MachineCacheWriteOptions::default(),
    )
    .with_context(|| {
        format!(
            "write icon manifest machine cache {}",
            paths.machine.display()
        )
    })?;
    let cache_generation_ns = duration_nanos_u64(generation_started.elapsed());

    let read_started = Instant::now();
    let read_summary = read_icon_manifest_machine_cache_summary(project_root, data_dir)
        .context("validate icon manifest machine cache after write")?;
    let machine_validate_read_ns = duration_nanos_u64(read_started.elapsed());
    let machine_validate_mmap_read_ns = if read_summary.mode == "mmap" {
        serde_json::Value::from(machine_validate_read_ns)
    } else {
        serde_json::Value::Null
    };

    let receipt_path = project_root.join(ICON_MANIFEST_PERFORMANCE_RECEIPT_PATH);
    if let Some(parent) = receipt_path.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!("create performance receipt directory {}", parent.display())
        })?;
    }

    let perf_receipt = serde_json::json!({
        "schema": ICON_MANIFEST_PERFORMANCE_RECEIPT_SCHEMA,
        "cache_name": "icon-manifest",
        "cache_kind": "typed-rkyv-directory-manifest",
        "source_data_root": normalize_path(data_dir),
        "machine_file_path": normalize_path(&paths.machine),
        "machine_metadata_file_path": normalize_path(&paths.metadata),
        "source_file_count": manifest.source_file_count,
        "source_total_bytes": manifest.source_total_bytes,
        "machine_source_file_count": read_summary.source_file_count,
        "machine_source_total_bytes": read_summary.source_total_bytes,
        "source_scan_fingerprint_ns": source_scan_ns,
        "cache_generation_ns": cache_generation_ns,
        "machine_validate_read_ns": machine_validate_read_ns,
        "machine_validate_mmap_read_ns": machine_validate_mmap_read_ns,
        "machine_read_mode": read_summary.mode,
        "machine_bytes": receipt.machine_bytes,
        "archive_bytes": receipt.archive_bytes,
        "fallback_behavior": "manifest cache miss falls back to current icon JSON directory scan and parser behavior",
        "json_source_authoritative": true,
        "manifest_cache_only": true,
        "full_icon_search_speed_claimed": false,
        "faster_than_upstream_claimed": false,
        "upstream_baseline_measured": false,
        "upstream_baseline_command": null,
        "upstream_baseline_checkout": null,
        "same_machine_benchmark_required": true,
        "test_command": null,
        "test_command_recorded": false,
        "suggested_test_command": "cargo check --manifest-path G:\\Dx\\www\\Cargo.toml --package dx-icons --no-default-features --lib -j1 --color never",
        "generated_at_unix_ms": current_unix_ms(),
        "machine": {
            "os": std::env::consts::OS,
            "arch": std::env::consts::ARCH
        }
    });
    fs::write(
        &receipt_path,
        serde_json::to_vec_pretty(&perf_receipt).context("serialize icon performance receipt")?,
    )
    .with_context(|| format!("write performance receipt {}", receipt_path.display()))?;

    Ok((receipt.machine, receipt_path))
}

pub fn read_icon_manifest_machine_cache_summary(
    project_root: &Path,
    data_dir: &Path,
) -> Option<IconManifestReadSummary> {
    let source = icon_manifest_source_fingerprint(data_dir).ok()?;
    let paths = icon_manifest_machine_paths(project_root, data_dir).ok()?;

    if let Ok(mapped) =
        open_typed_machine_cache::<IconManifestMachineV1>(&paths, &source, icon_manifest_schema())
    {
        return Some(summary_from_archive(mapped.archived(), "mmap"));
    }

    let bytes = fs::read(&paths.machine).ok()?;
    let archived = access_typed_machine_cache::<IconManifestMachineV1>(
        &bytes,
        &source,
        icon_manifest_schema(),
    )
    .ok()?;
    Some(summary_from_archive(archived, "bytes"))
}

pub fn icon_manifest_source_fingerprint(data_dir: &Path) -> Result<MachineCacheSource> {
    let mut json_files = fs::read_dir(data_dir)
        .with_context(|| format!("read icon data directory {}", data_dir.display()))?
        .collect::<std::result::Result<Vec<_>, _>>()
        .with_context(|| format!("list icon data directory {}", data_dir.display()))?;
    json_files.sort_by_key(|entry| entry.file_name());

    let mut total_bytes = 0u64;
    let mut latest_modified_unix_ms = None;
    let mut source_hasher = blake3::Hasher::new();

    for entry in json_files {
        let path = entry.path();
        if path.extension().and_then(|value| value.to_str()) != Some("json") {
            continue;
        }

        let bytes =
            fs::read(&path).with_context(|| format!("read icon pack {}", path.display()))?;
        let metadata =
            fs::metadata(&path).with_context(|| format!("stat icon pack {}", path.display()))?;
        let modified_unix_ms = modified_unix_ms(&metadata);
        let rel_path = path
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or_default()
            .to_string();
        let len = metadata.len();
        let file_hash = blake3::hash(&bytes);
        total_bytes = total_bytes.saturating_add(len);
        latest_modified_unix_ms = max_optional(latest_modified_unix_ms, modified_unix_ms);

        source_hasher.update(rel_path.as_bytes());
        source_hasher.update(&len.to_le_bytes());
        source_hasher.update(&modified_unix_ms.unwrap_or_default().to_le_bytes());
        source_hasher.update(file_hash.as_bytes());
    }

    Ok(MachineCacheSource {
        path: data_dir.to_path_buf(),
        bytes: total_bytes,
        modified_unix_ms: latest_modified_unix_ms,
        blake3: *source_hasher.finalize().as_bytes(),
    })
}

pub fn build_icon_manifest(data_dir: &Path) -> Result<(IconManifestMachineV1, MachineCacheSource)> {
    let mut json_files = fs::read_dir(data_dir)
        .with_context(|| format!("read icon data directory {}", data_dir.display()))?
        .collect::<std::result::Result<Vec<_>, _>>()
        .with_context(|| format!("list icon data directory {}", data_dir.display()))?;
    json_files.sort_by_key(|entry| entry.file_name());

    let mut packs = Vec::new();
    let mut total_bytes = 0u64;
    let mut latest_modified_unix_ms = None;

    for entry in json_files {
        let path = entry.path();
        if path.extension().and_then(|value| value.to_str()) != Some("json") {
            continue;
        }

        let bytes =
            fs::read(&path).with_context(|| format!("read icon pack {}", path.display()))?;
        let metadata =
            fs::metadata(&path).with_context(|| format!("stat icon pack {}", path.display()))?;
        let modified_unix_ms = modified_unix_ms(&metadata);
        latest_modified_unix_ms = max_optional(latest_modified_unix_ms, modified_unix_ms);
        let pack_hash = blake3::hash(&bytes);
        let rel_path = path
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or_default()
            .to_string();
        let pack = path
            .file_stem()
            .and_then(|value| value.to_str())
            .unwrap_or_default()
            .to_string();
        let len = metadata.len();
        total_bytes = total_bytes.saturating_add(len);

        packs.push(IconPackManifestEntryV1 {
            pack,
            rel_path,
            bytes: len,
            modified_unix_ms,
            blake3: *pack_hash.as_bytes(),
        });
    }

    let source_file_count = u32::try_from(packs.len()).unwrap_or(u32::MAX);
    let manifest = IconManifestMachineV1 {
        selected_data_root: normalize_path(data_dir),
        generated_at_unix_ms: current_unix_ms(),
        source_file_count,
        source_total_bytes: total_bytes,
        packs,
    };
    let source = icon_manifest_source_fingerprint(data_dir)?;

    Ok((manifest, source))
}

fn icon_manifest_machine_paths(project_root: &Path, data_dir: &Path) -> Result<MachineCachePaths> {
    let machine_root = project_root.join(".dx/icon/machine/v1");
    if !machine_root.starts_with(project_root.join(".dx")) {
        bail!(
            "icon manifest cache path escaped .dx root: {}",
            machine_root.display()
        );
    }

    Ok(MachineCachePaths {
        source: data_dir.to_path_buf(),
        machine: machine_root.join("manifest.machine"),
        metadata: machine_root.join("manifest.machine.meta.json"),
    })
}

fn icon_manifest_schema() -> MachineCacheSchema {
    MachineCacheSchema {
        name: ICON_MANIFEST_CACHE_SCHEMA,
        version: 1,
        kind: MachineCacheKind::Index,
    }
}

fn summary_from_archive(
    archived: &ArchivedIconManifestMachineV1,
    mode: &'static str,
) -> IconManifestReadSummary {
    IconManifestReadSummary {
        source_file_count: archived.packs.len(),
        source_total_bytes: archived.source_total_bytes.into(),
        mode,
    }
}

fn modified_unix_ms(metadata: &fs::Metadata) -> Option<u64> {
    metadata
        .modified()
        .ok()
        .and_then(|modified| modified.duration_since(UNIX_EPOCH).ok())
        .and_then(|duration| u64::try_from(duration.as_millis()).ok())
}

fn max_optional(left: Option<u64>, right: Option<u64>) -> Option<u64> {
    match (left, right) {
        (Some(left), Some(right)) => Some(left.max(right)),
        (Some(value), None) | (None, Some(value)) => Some(value),
        (None, None) => None,
    }
}

fn duration_nanos_u64(duration: std::time::Duration) -> u64 {
    u64::try_from(duration.as_nanos()).unwrap_or(u64::MAX)
}

fn current_unix_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .and_then(|duration| u64::try_from(duration.as_millis()).ok())
        .unwrap_or_default()
}

fn normalize_path(path: &Path) -> String {
    path.display().to_string().replace('\\', "/")
}
