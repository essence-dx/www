use crate::types::IconMetadata;
use anyhow::{Context, Result, bail};
use memmap2::Mmap;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

const RAW_FST_FILE: &str = "index.fst";
const RAW_METADATA_FILE: &str = "index.meta.machine";
const RAW_INTEGRITY_FILE: &str = "index.raw.integrity.json";
const COMPRESSED_FST_FILE: &str = "index.fst.lz4";
const COMPRESSED_METADATA_FILE: &str = "index.meta.lz4";
const RAW_INDEX_SCHEMA: &str = "dx.media-icon.raw-index.v1";
const RAW_INDEX_VERSION: u32 = 1;
const RAW_INDEX_PERFORMANCE_RECEIPT_SCHEMA: &str =
    "dx.performance.json_machine_cache_receipt.media_icon_raw_index.v1";
const RAW_INDEX_PERFORMANCE_RECEIPT_PATH: &str =
    ".dx/performance/json-machine-cache-receipts/media-icon-raw-index.json";

#[derive(Debug, Deserialize, Serialize)]
struct RawIndexIntegrity {
    schema: String,
    version: u32,
    fst: RawIndexFileIntegrity,
    metadata: RawIndexFileIntegrity,
    compressed: Option<CompressedIndexIntegrity>,
}

#[derive(Debug, Deserialize, Serialize)]
struct CompressedIndexIntegrity {
    fst: RawIndexFileIntegrity,
    metadata: RawIndexFileIntegrity,
}

#[derive(Debug, Deserialize, Serialize)]
struct RawIndexFileIntegrity {
    file: String,
    bytes: u64,
    blake3: String,
}

/// Serialized icon index with FST and rkyv metadata
/// Optimized with memory-mapped files for instant loading
pub struct IconIndex {
    pub fst_bytes: Vec<u8>,
    pub metadata_bytes: Vec<u8>,
}

/// Memory-mapped uncompressed icon index.
pub struct MappedIconIndex {
    fst_mmap: Mmap,
    metadata_mmap: Mmap,
}

impl MappedIconIndex {
    /// Raw FST bytes mapped from `index.fst`.
    pub fn fst_bytes(&self) -> &[u8] {
        &self.fst_mmap
    }

    /// Raw rkyv metadata bytes mapped from `index.meta.machine`.
    pub fn metadata_bytes(&self) -> &[u8] {
        &self.metadata_mmap
    }

    /// Convert the mapped index to the legacy owned index shape.
    pub fn to_owned_index(&self) -> IconIndex {
        IconIndex {
            fst_bytes: self.fst_bytes().to_vec(),
            metadata_bytes: self.metadata_bytes().to_vec(),
        }
    }
}

impl IconIndex {
    /// Build index from icon metadata
    pub fn build(icons: Vec<IconMetadata>) -> Result<Self> {
        // Build FST for name -> id mapping
        let mut builder = fst::MapBuilder::memory();
        let mut sorted_icons: Vec<_> = icons
            .iter()
            .enumerate()
            .map(|(idx, icon)| {
                // Create unique key: pack:name
                let key = format!("{}:{}", icon.pack, icon.name);
                (key, idx as u64)
            })
            .collect();
        sorted_icons.sort_by(|a, b| a.0.cmp(&b.0));

        for (name, id) in sorted_icons {
            builder.insert(name.as_bytes(), id)?;
        }

        let fst_bytes = builder.into_inner()?;

        // Serialize metadata with rkyv (zero-copy)
        let metadata_bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&icons)?.to_vec();

        Ok(IconIndex {
            fst_bytes,
            metadata_bytes,
        })
    }

    /// Save index to disk with legacy LZ4 compression.
    pub fn save(&self, path: &Path) -> Result<()> {
        self.save_compressed(path)
    }

    /// Save both legacy compressed files and uncompressed hot-cache files.
    pub fn save_all(&self, path: &Path) -> Result<()> {
        self.save(path)?;
        self.save_uncompressed(path)?;
        let _ = Self::write_raw_index_performance_receipt(path);
        Ok(())
    }

    /// Write a local source-vs-machine receipt for raw-index load paths.
    pub fn write_raw_index_performance_receipt(path: &Path) -> Result<PathBuf> {
        let compressed_load_started = Instant::now();
        let compressed_load_result = Self::load(path);
        let compressed_load_ns = duration_nanos_u64(compressed_load_started.elapsed());

        let raw_owned_load_started = Instant::now();
        let raw_owned_load_result = Self::load_uncompressed(path);
        let raw_owned_load_ns = duration_nanos_u64(raw_owned_load_started.elapsed());

        let raw_mmap_validate_load_started = Instant::now();
        let raw_mmap_result = Self::load_uncompressed_mmap(path);
        let raw_mmap_validate_load_ns =
            duration_nanos_u64(raw_mmap_validate_load_started.elapsed());

        let receipt_path = raw_index_performance_receipt_path(path)?;
        if let Some(parent) = receipt_path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!(
                    "create raw index performance receipt directory {}",
                    parent.display()
                )
            })?;
        }

        let receipt = serde_json::json!({
            "schema": RAW_INDEX_PERFORMANCE_RECEIPT_SCHEMA,
                "cache_name": "media-icon-raw-index",
                "cache_kind": "raw-fst-rkyv-metadata-index",
                "measurement_scope": "post-save local validation timing; OS page cache may be warm",
                "timing_order": ["compressed_load", "raw_owned_load", "raw_mmap_validate_load"],
                "post_save_page_cache_may_be_warm": true,
                "raw_owned_load_includes_validation": true,
                "raw_mmap_validate_load_hashes_full_files": true,
                "index_dir": normalize_path(path),
                "receipt_path": normalize_path(&receipt_path),
                "compressed_fst_file_path": normalize_path(&path.join(COMPRESSED_FST_FILE)),
            "compressed_metadata_file_path": normalize_path(&path.join(COMPRESSED_METADATA_FILE)),
            "raw_fst_file_path": normalize_path(&path.join(RAW_FST_FILE)),
            "raw_metadata_file_path": normalize_path(&path.join(RAW_METADATA_FILE)),
            "raw_integrity_file_path": normalize_path(&path.join(RAW_INTEGRITY_FILE)),
            "compressed_fst_bytes": file_len_json(&path.join(COMPRESSED_FST_FILE)),
            "compressed_metadata_bytes": file_len_json(&path.join(COMPRESSED_METADATA_FILE)),
            "raw_fst_bytes": file_len_json(&path.join(RAW_FST_FILE)),
            "raw_metadata_bytes": file_len_json(&path.join(RAW_METADATA_FILE)),
            "raw_integrity_bytes": file_len_json(&path.join(RAW_INTEGRITY_FILE)),
            "compressed_load_ns": compressed_load_ns,
            "raw_owned_load_ns": raw_owned_load_ns,
            "raw_mmap_validate_load_ns": raw_mmap_validate_load_ns,
            "compressed_load_ok": compressed_load_result.is_ok(),
            "raw_owned_load_ok": raw_owned_load_result.is_ok(),
            "raw_mmap_available": raw_mmap_result.is_ok(),
            "compressed_load_error": result_error(&compressed_load_result),
            "raw_owned_load_error": result_error(&raw_owned_load_result),
            "raw_mmap_error": result_error(&raw_mmap_result),
            "fallback_behavior": "raw index receipt or mmap miss falls back to raw owned load and then legacy compressed LZ4 load through IconIndex::load_fast",
            "json_source_authoritative": true,
            "raw_index_cache_only": true,
            "full_icon_runtime_baseline_measured": false,
            "full_icon_search_speed_claimed": false,
            "faster_than_upstream_claimed": false,
            "upstream_baseline_measured": false,
            "upstream_baseline_command": serde_json::Value::Null,
            "upstream_baseline_checkout": serde_json::Value::Null,
            "same_machine_benchmark_required": true,
            "test_command": serde_json::Value::Null,
            "test_command_recorded": false,
            "suggested_test_command": "cargo check --manifest-path G:\\Dx\\www\\Cargo.toml --package dx-icons --locked --no-default-features --features rayon --lib --bin search_cli --bin icon -j1 --color never",
            "generated_at_unix_ms": current_unix_ms(),
            "machine": {
                "os": std::env::consts::OS,
                "arch": std::env::consts::ARCH
            }
        });

        write_atomic(&receipt_path, &serde_json::to_vec_pretty(&receipt)?).with_context(|| {
            format!(
                "write raw index performance receipt {}",
                receipt_path.display()
            )
        })?;

        Ok(receipt_path)
    }

    /// Save index to disk with legacy LZ4 compression.
    pub fn save_compressed(&self, path: &Path) -> Result<()> {
        std::fs::create_dir_all(path)?;
        let compressed_fst = lz4_flex::compress_prepend_size(&self.fst_bytes);
        let compressed_metadata = lz4_flex::compress_prepend_size(&self.metadata_bytes);

        write_atomic(&path.join(COMPRESSED_FST_FILE), &compressed_fst)?;
        write_atomic(&path.join(COMPRESSED_METADATA_FILE), &compressed_metadata)?;

        Ok(())
    }

    /// Save uncompressed hot-cache files for mmap-friendly reads.
    pub fn save_uncompressed(&self, path: &Path) -> Result<()> {
        std::fs::create_dir_all(path)?;
        write_atomic(&path.join(RAW_FST_FILE), &self.fst_bytes)?;
        write_atomic(&path.join(RAW_METADATA_FILE), &self.metadata_bytes)?;
        write_raw_index_integrity(path, &self.fst_bytes, &self.metadata_bytes)?;

        Ok(())
    }

    /// Load index from disk, preferring uncompressed hot-cache files.
    pub fn load_fast(path: &Path) -> Result<Self> {
        Self::load_uncompressed(path).or_else(|_| Self::load(path))
    }

    /// Load index from legacy compressed files.
    pub fn load(path: &Path) -> Result<Self> {
        let compressed_fst = std::fs::read(path.join(COMPRESSED_FST_FILE))?;
        let compressed_metadata = std::fs::read(path.join(COMPRESSED_METADATA_FILE))?;

        let fst_bytes = lz4_flex::decompress_size_prepended(&compressed_fst)?;
        let metadata_bytes = lz4_flex::decompress_size_prepended(&compressed_metadata)?;

        Ok(IconIndex {
            fst_bytes,
            metadata_bytes,
        })
    }

    /// Load index from uncompressed hot-cache files.
    pub fn load_uncompressed(path: &Path) -> Result<Self> {
        let integrity = read_raw_index_integrity(path)?;
        validate_current_compressed_index(path, integrity.compressed.as_ref())?;
        let fst_bytes = std::fs::read(path.join(RAW_FST_FILE))?;
        let metadata_bytes = std::fs::read(path.join(RAW_METADATA_FILE))?;
        validate_index_file_bytes(RAW_FST_FILE, &fst_bytes, &integrity.fst)?;
        validate_index_file_bytes(RAW_METADATA_FILE, &metadata_bytes, &integrity.metadata)?;
        validate_raw_fst_bytes(&fst_bytes)?;

        Ok(IconIndex {
            fst_bytes,
            metadata_bytes,
        })
    }

    /// Memory-map uncompressed hot-cache files.
    pub fn load_uncompressed_mmap(path: &Path) -> Result<MappedIconIndex> {
        let integrity = read_raw_index_integrity(path)?;
        validate_current_compressed_index(path, integrity.compressed.as_ref())?;
        let fst_file = File::open(path.join(RAW_FST_FILE))?;
        let meta_file = File::open(path.join(RAW_METADATA_FILE))?;

        // SAFETY: Files are opened read-only and the maps stay owned by MappedIconIndex.
        let fst_mmap = unsafe { Mmap::map(&fst_file)? };
        let metadata_mmap = unsafe { Mmap::map(&meta_file)? };
        validate_index_file_bytes(RAW_FST_FILE, &fst_mmap, &integrity.fst)?;
        validate_index_file_bytes(RAW_METADATA_FILE, &metadata_mmap, &integrity.metadata)?;
        validate_raw_fst_bytes(&fst_mmap)?;

        Ok(MappedIconIndex {
            fst_mmap,
            metadata_mmap,
        })
    }

    /// Load index using memory-mapped compressed files.
    ///
    /// This is still a compatibility path because compressed bytes must be
    /// decompressed into owned memory. Prefer [`IconIndex::load_uncompressed_mmap`]
    /// when raw hot-cache files are available.
    #[allow(dead_code)]
    pub fn load_mmap(path: &Path) -> Result<Self> {
        let fst_file = File::open(path.join(COMPRESSED_FST_FILE))?;
        let meta_file = File::open(path.join(COMPRESSED_METADATA_FILE))?;

        // SAFETY: Files are read-only and won't be modified
        let fst_mmap = unsafe { Mmap::map(&fst_file)? };
        let meta_mmap = unsafe { Mmap::map(&meta_file)? };

        // Decompress (still needed, but OS handles paging)
        let fst_bytes = lz4_flex::decompress_size_prepended(&fst_mmap)?;
        let metadata_bytes = lz4_flex::decompress_size_prepended(&meta_mmap)?;

        Ok(IconIndex {
            fst_bytes,
            metadata_bytes,
        })
    }
}

fn write_raw_index_integrity(path: &Path, fst_bytes: &[u8], metadata_bytes: &[u8]) -> Result<()> {
    let integrity = RawIndexIntegrity {
        schema: RAW_INDEX_SCHEMA.to_string(),
        version: RAW_INDEX_VERSION,
        fst: raw_index_file_integrity(RAW_FST_FILE, fst_bytes),
        metadata: raw_index_file_integrity(RAW_METADATA_FILE, metadata_bytes),
        compressed: current_compressed_index_integrity(path)?,
    };
    let bytes = serde_json::to_vec_pretty(&integrity)?;
    write_atomic(&path.join(RAW_INTEGRITY_FILE), &bytes)?;
    Ok(())
}

fn read_raw_index_integrity(path: &Path) -> Result<RawIndexIntegrity> {
    let integrity_path = path.join(RAW_INTEGRITY_FILE);
    let bytes = std::fs::read(&integrity_path)
        .with_context(|| format!("failed to read {}", integrity_path.display()))?;
    let integrity: RawIndexIntegrity = serde_json::from_slice(&bytes)
        .with_context(|| format!("failed to parse {}", integrity_path.display()))?;

    if integrity.schema != RAW_INDEX_SCHEMA {
        bail!(
            "raw icon index schema mismatch: expected {RAW_INDEX_SCHEMA}, got {}",
            integrity.schema
        );
    }
    if integrity.version != RAW_INDEX_VERSION {
        bail!(
            "raw icon index version mismatch: expected {RAW_INDEX_VERSION}, got {}",
            integrity.version
        );
    }

    Ok(integrity)
}

fn current_compressed_index_integrity(path: &Path) -> Result<Option<CompressedIndexIntegrity>> {
    let fst_path = path.join(COMPRESSED_FST_FILE);
    let metadata_path = path.join(COMPRESSED_METADATA_FILE);
    let fst_exists = fst_path.exists();
    let metadata_exists = metadata_path.exists();

    match (fst_exists, metadata_exists) {
        (false, false) => Ok(None),
        (true, true) => {
            let fst_bytes = std::fs::read(&fst_path)
                .with_context(|| format!("failed to read {}", fst_path.display()))?;
            let metadata_bytes = std::fs::read(&metadata_path)
                .with_context(|| format!("failed to read {}", metadata_path.display()))?;
            Ok(Some(CompressedIndexIntegrity {
                fst: raw_index_file_integrity(COMPRESSED_FST_FILE, &fst_bytes),
                metadata: raw_index_file_integrity(COMPRESSED_METADATA_FILE, &metadata_bytes),
            }))
        }
        _ => bail!(
            "partial legacy compressed icon index beside raw cache: expected both {COMPRESSED_FST_FILE} and {COMPRESSED_METADATA_FILE}"
        ),
    }
}

fn validate_current_compressed_index(
    path: &Path,
    expected: Option<&CompressedIndexIntegrity>,
) -> Result<()> {
    let Some(expected) = expected else {
        if current_compressed_index_integrity(path)?.is_some() {
            bail!("raw icon index integrity is missing compressed source fingerprints");
        }
        return Ok(());
    };

    let current = current_compressed_index_integrity(path)?.context(
        "raw icon index integrity expects legacy compressed files, but they are missing",
    )?;

    validate_integrity_record(COMPRESSED_FST_FILE, &current.fst, &expected.fst)?;
    validate_integrity_record(
        COMPRESSED_METADATA_FILE,
        &current.metadata,
        &expected.metadata,
    )?;
    Ok(())
}

fn validate_integrity_record(
    expected_file: &str,
    current: &RawIndexFileIntegrity,
    expected: &RawIndexFileIntegrity,
) -> Result<()> {
    if expected.file != expected_file || current.file != expected_file {
        bail!("icon index integrity file mismatch for {expected_file}");
    }
    if expected.bytes != current.bytes {
        bail!(
            "icon index compressed byte length mismatch for {expected_file}: expected {}, got {}",
            expected.bytes,
            current.bytes
        );
    }
    if expected.blake3 != current.blake3 {
        bail!("icon index compressed hash mismatch for {expected_file}");
    }
    Ok(())
}

fn raw_index_file_integrity(file: &str, bytes: &[u8]) -> RawIndexFileIntegrity {
    RawIndexFileIntegrity {
        file: file.to_string(),
        bytes: bytes.len() as u64,
        blake3: blake3::hash(bytes).to_hex().to_string(),
    }
}

fn validate_index_file_bytes(
    expected_file: &str,
    bytes: &[u8],
    integrity: &RawIndexFileIntegrity,
) -> Result<()> {
    if integrity.file != expected_file {
        bail!(
            "raw icon index receipt points to {}, expected {expected_file}",
            integrity.file
        );
    }

    if integrity.bytes != bytes.len() as u64 {
        bail!(
            "raw icon index byte length mismatch for {expected_file}: expected {}, got {}",
            integrity.bytes,
            bytes.len()
        );
    }

    let actual_hash = blake3::hash(bytes).to_hex().to_string();
    if integrity.blake3 != actual_hash {
        bail!("raw icon index hash mismatch for {expected_file}");
    }

    Ok(())
}

fn validate_raw_fst_bytes(bytes: &[u8]) -> Result<()> {
    let _ = fst::Map::new(bytes).context("raw icon index FST validation failed")?;
    Ok(())
}

fn raw_index_performance_receipt_path(index_dir: &Path) -> Result<PathBuf> {
    let project_root = raw_index_project_root(index_dir)?;
    Ok(project_root.join(RAW_INDEX_PERFORMANCE_RECEIPT_PATH))
}

fn raw_index_project_root(index_dir: &Path) -> Result<PathBuf> {
    if index_dir.file_name().and_then(|value| value.to_str()) != Some("index") {
        bail!(
            "raw index performance receipt expects an index output directory, got {}",
            index_dir.display()
        );
    }

    if let Some(parent) = index_dir.parent()
        && !parent.as_os_str().is_empty()
    {
        return Ok(parent.to_path_buf());
    }

    std::env::current_dir().context("resolve current directory for relative index output directory")
}

fn file_len_json(path: &Path) -> serde_json::Value {
    fs::metadata(path)
        .map(|metadata| serde_json::Value::from(metadata.len()))
        .unwrap_or(serde_json::Value::Null)
}

fn result_error<T>(result: &Result<T>) -> serde_json::Value {
    match result {
        Ok(_) => serde_json::Value::Null,
        Err(error) => serde_json::Value::String(format!("{error:#}")),
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

fn write_atomic(path: &Path, bytes: &[u8]) -> Result<()> {
    let tmp = path.with_extension(format!(
        "{}.tmp.{}",
        path.extension()
            .and_then(|extension| extension.to_str())
            .unwrap_or("tmp"),
        std::process::id()
    ));
    std::fs::write(&tmp, bytes)?;
    if path.exists() {
        std::fs::remove_file(path)?;
    }
    std::fs::rename(tmp, path)?;
    Ok(())
}
