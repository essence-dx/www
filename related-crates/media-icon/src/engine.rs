use crate::bloom::IconBloomFilters;
use crate::index::{IconIndex, MappedIconIndex};
use crate::machine_catalog::{
    IconCatalogMachineV1, IconCatalogSourceFingerprint, IconPrefixMachineV1,
    icon_catalog_source_fingerprint, icon_metadata_from_catalog_machine,
    read_icon_catalog_machine_cache_with_source_fingerprint,
    read_icon_prefix_machine_cache_with_source_fingerprint,
    validate_catalog_machine_metadata_parity,
};
use crate::machine_manifest::select_icon_data_dir;
use crate::machine_pack_body::{
    IconPackBodyMachineV1, ResolvedIconPackBody,
    read_icon_pack_body_machine_cache_with_source_fingerprint, resolve_icon_pack_body,
    validate_icon_pack_body_runtime_metadata,
};
use crate::machine_precomputed::{
    IconBloomMachineV1, IconLowercaseCacheMachineV1, IconPerfectHashMachineV1,
    bloom_filters_from_machine, lowercase_cache_from_machine, perfect_hash_index_from_machine,
    read_icon_bloom_machine_cache_with_source_fingerprint,
    read_icon_lowercase_cache_machine_cache_with_source_fingerprint,
    read_icon_perfect_hash_machine_cache_with_source_fingerprint,
};
use crate::perfect_hash::{PerfectHashIndex, ValidatedLowercaseCache};
use crate::precomputed::{
    BloomMachineAdoptionSummary, LowercaseCacheMachineAdoptionSummary,
    PerfectHashMachineAdoptionSummary, PrecomputedBuildTimings, PrecomputedIndex,
    PrefixMachineAdoptionSummary,
};
use crate::search::{MatchType, SearchResult, calculate_score, fuzzy_match};
use crate::types::{IconMetadata, IconPack};
use anyhow::{Context, Result, anyhow};
use dashmap::DashMap;
use rayon::prelude::*;
use rkyv::Archived;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

// Optimal chunk size for cache locality (64KB L1 cache / ~200 bytes per icon = ~320 icons)
const RAYON_CHUNK_SIZE: usize = 256;
const ENGINE_STARTUP_RECEIPT_SCHEMA: &str =
    "dx.performance.json_machine_cache_receipt.media_icon_engine_startup.v1";
const ENGINE_STARTUP_RECEIPT_PATH: &str =
    ".dx/performance/json-machine-cache-receipts/media-icon-engine-startup.json";
const BODY_RESOLUTION_RECEIPT_SCHEMA: &str =
    "dx.performance.json_machine_cache_receipt.media_icon_body_resolution.v1";
const BODY_RESOLUTION_RECEIPT_PATH: &str =
    ".dx/performance/json-machine-cache-receipts/media-icon-body-resolution.json";
const QUERY_LATENCY_RECEIPT_SCHEMA: &str =
    "dx.performance.json_machine_cache_receipt.media_icon_query_latency.v1";
const QUERY_LATENCY_RECEIPT_PATH: &str =
    ".dx/performance/json-machine-cache-receipts/media-icon-query-latency.json";
const QUERY_LATENCY_MAX_QUERIES: usize = 12;
const QUERY_LATENCY_MAX_LIMIT: usize = 32;
const QUERY_LATENCY_MAX_WARMUP_RUNS: usize = 3;
const QUERY_LATENCY_MAX_MEASURED_RUNS: usize = 5;

#[derive(Debug, Clone)]
struct IconEngineStartupContext {
    index_dir: PathBuf,
    load_mode: &'static str,
    startup_started: Instant,
    raw_mmap_validate_load_ns: u64,
    raw_mmap_ok: bool,
    raw_mmap_error: Option<String>,
    owned_fallback_used: bool,
    owned_fallback_load_ns: u64,
    owned_fallback_error: Option<String>,
}

#[derive(Debug, Clone)]
struct RuntimeCatalogMachine {
    catalog_machine: Option<IconCatalogMachineV1>,
    runtime_catalog_machine_available: bool,
    runtime_catalog_machine_read_mode: Option<&'static str>,
    catalog_machine_fallback_reason: Option<String>,
}

#[derive(Debug, Clone)]
struct RuntimePerfectHashMachine {
    perfect_hash_machine: Option<IconPerfectHashMachineV1>,
    runtime_perfect_hash_machine_available: bool,
    runtime_perfect_hash_machine_read_mode: Option<&'static str>,
    perfect_hash_machine_fallback_reason: Option<String>,
}

#[derive(Debug, Clone)]
struct RuntimeBloomMachine {
    bloom_machine: Option<IconBloomMachineV1>,
    runtime_bloom_machine_available: bool,
    runtime_bloom_machine_read_mode: Option<&'static str>,
    bloom_machine_fallback_reason: Option<String>,
}

#[derive(Debug, Clone)]
struct RuntimeLowercaseCacheMachine {
    lowercase_cache_machine: Option<IconLowercaseCacheMachineV1>,
    runtime_lowercase_cache_machine_available: bool,
    runtime_lowercase_cache_machine_read_mode: Option<&'static str>,
    lowercase_cache_machine_fallback_reason: Option<String>,
}

#[derive(Debug, Clone)]
struct RuntimePackBodyMachine {
    pack_body_machine: Option<Arc<IconPackBodyMachineV1>>,
    runtime_pack_body_machine_available: bool,
    runtime_pack_body_machine_read_mode: Option<&'static str>,
    pack_body_machine_fallback_reason: Option<String>,
}

struct RuntimeMachineReadContext {
    project_root: PathBuf,
    data_dir: PathBuf,
    source_fingerprint: IconCatalogSourceFingerprint,
}

enum RuntimeMachineReadContextError {
    ProjectRoot(anyhow::Error),
    DataDirNotFound,
    SourceFingerprint(anyhow::Error),
}

impl RuntimeBloomMachine {
    fn available(bloom_machine: IconBloomMachineV1, mode: &'static str) -> Self {
        Self {
            bloom_machine: Some(bloom_machine),
            runtime_bloom_machine_available: true,
            runtime_bloom_machine_read_mode: Some(mode),
            bloom_machine_fallback_reason: None,
        }
    }

    fn unavailable(bloom_machine_fallback_reason: String) -> Self {
        Self {
            bloom_machine: None,
            runtime_bloom_machine_available: false,
            runtime_bloom_machine_read_mode: None,
            bloom_machine_fallback_reason: Some(bloom_machine_fallback_reason),
        }
    }
}

impl RuntimeLowercaseCacheMachine {
    fn available(lowercase_cache_machine: IconLowercaseCacheMachineV1, mode: &'static str) -> Self {
        Self {
            lowercase_cache_machine: Some(lowercase_cache_machine),
            runtime_lowercase_cache_machine_available: true,
            runtime_lowercase_cache_machine_read_mode: Some(mode),
            lowercase_cache_machine_fallback_reason: None,
        }
    }

    fn unavailable(lowercase_cache_machine_fallback_reason: String) -> Self {
        Self {
            lowercase_cache_machine: None,
            runtime_lowercase_cache_machine_available: false,
            runtime_lowercase_cache_machine_read_mode: None,
            lowercase_cache_machine_fallback_reason: Some(lowercase_cache_machine_fallback_reason),
        }
    }
}

impl RuntimePackBodyMachine {
    fn available(pack_body_machine: IconPackBodyMachineV1, mode: &'static str) -> Self {
        Self {
            pack_body_machine: Some(Arc::new(pack_body_machine)),
            runtime_pack_body_machine_available: true,
            runtime_pack_body_machine_read_mode: Some(mode),
            pack_body_machine_fallback_reason: None,
        }
    }

    fn unavailable(pack_body_machine_fallback_reason: String) -> Self {
        Self {
            pack_body_machine: None,
            runtime_pack_body_machine_available: false,
            runtime_pack_body_machine_read_mode: None,
            pack_body_machine_fallback_reason: Some(pack_body_machine_fallback_reason),
        }
    }
}

impl RuntimePerfectHashMachine {
    fn available(perfect_hash_machine: IconPerfectHashMachineV1, mode: &'static str) -> Self {
        Self {
            perfect_hash_machine: Some(perfect_hash_machine),
            runtime_perfect_hash_machine_available: true,
            runtime_perfect_hash_machine_read_mode: Some(mode),
            perfect_hash_machine_fallback_reason: None,
        }
    }

    fn unavailable(perfect_hash_machine_fallback_reason: String) -> Self {
        Self {
            perfect_hash_machine: None,
            runtime_perfect_hash_machine_available: false,
            runtime_perfect_hash_machine_read_mode: None,
            perfect_hash_machine_fallback_reason: Some(perfect_hash_machine_fallback_reason),
        }
    }
}

impl RuntimeCatalogMachine {
    fn available(catalog_machine: IconCatalogMachineV1, mode: &'static str) -> Self {
        Self {
            catalog_machine: Some(catalog_machine),
            runtime_catalog_machine_available: true,
            runtime_catalog_machine_read_mode: Some(mode),
            catalog_machine_fallback_reason: None,
        }
    }

    fn unavailable(catalog_machine_fallback_reason: String) -> Self {
        Self {
            catalog_machine: None,
            runtime_catalog_machine_available: false,
            runtime_catalog_machine_read_mode: None,
            catalog_machine_fallback_reason: Some(catalog_machine_fallback_reason),
        }
    }
}

#[derive(Debug, Clone)]
struct RuntimePrefixMachine {
    prefix_machine: Option<IconPrefixMachineV1>,
    runtime_prefix_machine_available: bool,
    runtime_prefix_machine_read_mode: Option<&'static str>,
    prefix_machine_fallback_reason: Option<String>,
}

impl RuntimePrefixMachine {
    fn available(prefix_machine: IconPrefixMachineV1, mode: &'static str) -> Self {
        Self {
            prefix_machine: Some(prefix_machine),
            runtime_prefix_machine_available: true,
            runtime_prefix_machine_read_mode: Some(mode),
            prefix_machine_fallback_reason: None,
        }
    }

    fn unavailable(prefix_machine_fallback_reason: String) -> Self {
        Self {
            prefix_machine: None,
            runtime_prefix_machine_available: false,
            runtime_prefix_machine_read_mode: None,
            prefix_machine_fallback_reason: Some(prefix_machine_fallback_reason),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CatalogMachineMetadataAdoptionSummary {
    runtime_catalog_machine_available: bool,
    runtime_catalog_machine_adopted: bool,
    catalog_machine_metadata_parity_validated: bool,
    catalog_machine_fallback_reason: Option<String>,
    runtime_metadata_source: &'static str,
}

impl CatalogMachineMetadataAdoptionSummary {
    fn unavailable() -> Self {
        Self {
            runtime_catalog_machine_available: false,
            runtime_catalog_machine_adopted: false,
            catalog_machine_metadata_parity_validated: false,
            catalog_machine_fallback_reason: Some("catalog_machine_not_available".to_string()),
            runtime_metadata_source: "rkyv_metadata_index",
        }
    }

    fn adopted() -> Self {
        Self {
            runtime_catalog_machine_available: true,
            runtime_catalog_machine_adopted: true,
            catalog_machine_metadata_parity_validated: true,
            catalog_machine_fallback_reason: None,
            runtime_metadata_source: "catalog_machine",
        }
    }

    fn fallback(error: anyhow::Error) -> Self {
        Self {
            runtime_catalog_machine_available: true,
            runtime_catalog_machine_adopted: false,
            catalog_machine_metadata_parity_validated: false,
            catalog_machine_fallback_reason: Some(error.to_string()),
            runtime_metadata_source: "rkyv_metadata_index",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PackBodyMachineEvidenceSummary {
    runtime_pack_body_machine_available: bool,
    runtime_pack_body_machine_evidence_validated: bool,
    pack_body_machine_runtime_metadata_validated: bool,
    pack_body_machine_fallback_reason: Option<String>,
    runtime_pack_body_source: &'static str,
    pack_body_machine_pack_count: usize,
    pack_body_machine_icon_count: usize,
    pack_body_machine_source_total_bytes: u64,
}

impl PackBodyMachineEvidenceSummary {
    fn unavailable() -> Self {
        Self {
            runtime_pack_body_machine_available: false,
            runtime_pack_body_machine_evidence_validated: false,
            pack_body_machine_runtime_metadata_validated: false,
            pack_body_machine_fallback_reason: Some("pack_body_machine_not_available".to_string()),
            runtime_pack_body_source: "pack_body_machine_unavailable",
            pack_body_machine_pack_count: 0,
            pack_body_machine_icon_count: 0,
            pack_body_machine_source_total_bytes: 0,
        }
    }

    fn validated(machine: &IconPackBodyMachineV1) -> Self {
        Self {
            runtime_pack_body_machine_available: true,
            runtime_pack_body_machine_evidence_validated: true,
            pack_body_machine_runtime_metadata_validated: true,
            pack_body_machine_fallback_reason: None,
            runtime_pack_body_source: "pack_body_machine",
            pack_body_machine_pack_count: machine.packs.len(),
            pack_body_machine_icon_count: machine.icon_count as usize,
            pack_body_machine_source_total_bytes: machine.source_total_bytes,
        }
    }

    fn fallback(machine: &IconPackBodyMachineV1, error: anyhow::Error) -> Self {
        Self {
            runtime_pack_body_machine_available: true,
            runtime_pack_body_machine_evidence_validated: false,
            pack_body_machine_runtime_metadata_validated: false,
            pack_body_machine_fallback_reason: Some(format!(
                "pack_body_machine_runtime_metadata_mismatch: {error:#}"
            )),
            runtime_pack_body_source: "pack_body_machine_unvalidated",
            pack_body_machine_pack_count: machine.packs.len(),
            pack_body_machine_icon_count: machine.icon_count as usize,
            pack_body_machine_source_total_bytes: machine.source_total_bytes,
        }
    }
}

/// Local startup timing receipt for the media-icon search engine.
#[derive(Debug, Clone)]
pub struct IconEngineStartupReceipt {
    pub schema: &'static str,
    pub index_dir: String,
    pub load_mode: &'static str,
    pub metadata_bytes: u64,
    pub icon_count: usize,
    pub startup_total_ns: u64,
    pub raw_mmap_validate_load_ns: u64,
    pub owned_fallback_load_ns: u64,
    pub metadata_bytecheck_ns: u64,
    pub metadata_materialize_ns: u64,
    pub from_metadata_bytes_total_ns: u64,
    pub precomputed_build_total_ns: u64,
    pub perfect_hash_build_ns: u64,
    pub lowercase_cache_build_ns: u64,
    pub lowercase_names_build_ns: u64,
    pub lowercase_names_from_machine_cache: bool,
    pub bloom_filters_build_ns: u64,
    pub prefix_index_build_ns: u64,
    pub engine_cache_init_ns: u64,
    pub receipt_write_ns: u64,
    pub raw_mmap_ok: bool,
    pub raw_mmap_error: Option<String>,
    pub owned_fallback_used: bool,
    pub owned_fallback_error: Option<String>,
    pub engine_startup_receipt_only: bool,
    pub runtime_machine_source_fingerprint_reused: bool,
    pub runtime_precomputed_cache_adopted: bool,
    pub runtime_perfect_hash_machine_available: bool,
    pub runtime_perfect_hash_machine_adopted: bool,
    pub perfect_hash_machine_lookup_validated: bool,
    pub perfect_hash_machine_fallback_reason: Option<String>,
    pub runtime_perfect_hash_source: &'static str,
    pub runtime_perfect_hash_machine_read_mode: Option<&'static str>,
    pub runtime_bloom_machine_available: bool,
    pub runtime_bloom_machine_adopted: bool,
    pub bloom_machine_no_false_negatives_validated: bool,
    pub bloom_machine_fallback_reason: Option<String>,
    pub runtime_bloom_source: &'static str,
    pub runtime_bloom_machine_read_mode: Option<&'static str>,
    pub runtime_lowercase_cache_machine_available: bool,
    pub runtime_lowercase_cache_machine_adopted: bool,
    pub lowercase_cache_machine_names_validated: bool,
    pub lowercase_cache_machine_fallback_reason: Option<String>,
    pub runtime_lowercase_cache_source: &'static str,
    pub runtime_lowercase_cache_machine_read_mode: Option<&'static str>,
    pub runtime_catalog_machine_available: bool,
    pub runtime_catalog_machine_adopted: bool,
    pub catalog_machine_metadata_parity_validated: bool,
    pub catalog_machine_fallback_reason: Option<String>,
    pub runtime_metadata_source: &'static str,
    pub runtime_catalog_machine_read_mode: Option<&'static str>,
    pub runtime_prefix_machine_available: bool,
    pub runtime_prefix_machine_adopted: bool,
    pub catalog_prefix_machine_consumed_at_runtime: bool,
    pub prefix_machine_id_to_position_validated: bool,
    pub prefix_machine_fallback_reason: Option<String>,
    pub runtime_prefix_index_source: &'static str,
    pub runtime_prefix_machine_read_mode: Option<&'static str>,
    pub runtime_pack_body_machine_available: bool,
    pub runtime_pack_body_machine_evidence_validated: bool,
    pub runtime_pack_body_machine_adopted: bool,
    pub pack_body_machine_runtime_metadata_validated: bool,
    pub pack_body_machine_fallback_reason: Option<String>,
    pub runtime_pack_body_source: &'static str,
    pub runtime_pack_body_machine_read_mode: Option<&'static str>,
    pub pack_body_machine_pack_count: usize,
    pub pack_body_machine_icon_count: usize,
    pub pack_body_machine_source_total_bytes: u64,
    pub pack_body_machine_consumed_for_body_resolution: bool,
    pub pack_body_machine_consumed_by_search: bool,
    pub pack_body_machine_evidence_only: bool,
    pub runtime_rebuilds_non_prefix_precomputed_structures: bool,
    pub search_behavior_changed: bool,
    pub full_icon_runtime_baseline_measured: bool,
    pub full_icon_search_speed_claimed: bool,
    pub faster_than_upstream_claimed: bool,
    pub upstream_baseline_measured: bool,
    pub same_machine_benchmark_required: bool,
}

/// Local one-icon timing receipt for machine-backed body resolution versus JSON fallback.
#[derive(Debug, Clone)]
pub struct IconBodyResolutionReceipt {
    pub schema: &'static str,
    pub index_dir: String,
    pub data_dir: String,
    pub pack: String,
    pub name: String,
    pub engine_load_with_pack_body_cache_ns: u64,
    pub machine_body_resolution_ns: u64,
    pub json_fallback_resolution_ns: u64,
    pub machine_svg_render_ns: u64,
    pub json_svg_render_ns: u64,
    pub receipt_write_ns: u64,
    pub engine_has_pack_body_cache: bool,
    pub machine_body_resolution_hit: bool,
    pub machine_body_resolution_has_dimensions: bool,
    pub json_fallback_ok: bool,
    pub machine_json_svg_match: Option<bool>,
    pub machine_body_bytes: Option<usize>,
    pub json_body_bytes: Option<usize>,
    pub rendered_machine_svg_bytes: Option<usize>,
    pub rendered_json_svg_bytes: Option<usize>,
    pub json_fallback_error: Option<String>,
}

/// Local bounded warm-cache timing receipt for search plus top-result body/render work.
#[derive(Debug, Clone)]
pub struct IconQueryLatencyReceipt {
    pub schema: &'static str,
    pub index_dir: String,
    pub data_dir: String,
    pub requested_query_count: usize,
    pub effective_query_count: usize,
    pub requested_limit: usize,
    pub effective_limit: usize,
    pub requested_warmup_runs: usize,
    pub effective_warmup_runs: usize,
    pub requested_measured_runs: usize,
    pub effective_measured_runs: usize,
    pub engine_load_with_pack_body_cache_ns: u64,
    pub receipt_write_ns: u64,
    pub engine_has_pack_body_cache: bool,
    pub samples: Vec<IconQueryLatencySample>,
}

#[derive(Debug, Clone)]
pub struct IconQueryLatencySample {
    pub query: String,
    pub run_index: usize,
    pub search_ns: u64,
    pub result_count: usize,
    pub top_result_pack: Option<String>,
    pub top_result_name: Option<String>,
    pub top_result_score: Option<f32>,
    pub top_result_match_type: Option<&'static str>,
    pub body_resolution_ns: u64,
    pub body_resolution_hit: bool,
    pub body_resolution_source: &'static str,
    pub export_like_source: &'static str,
    pub svg_render_ns: u64,
    pub svg_rendered: bool,
    pub rendered_svg_bytes: Option<usize>,
    pub export_like_top_result_ns: u64,
}

struct BoundedQueryLatencyInputs {
    queries: Vec<String>,
    effective_limit: usize,
    effective_warmup_runs: usize,
    effective_measured_runs: usize,
}

/// Receipt-backed icon search engine.
/// - O(1) exact match via perfect hashing
/// - 90%+ rejection via bloom filters
/// - Zero-allocation search
/// - Pre-computed indices
/// - Lock-free caching
#[repr(align(64))] // Cache-line aligned for better performance
pub struct IconSearchEngine {
    /// Pre-computed indices (built once at startup)
    precomputed: Arc<PrecomputedIndex>,
    cache: Arc<DashMap<String, Vec<SearchResult>>>,
    pack_body_machine: Option<Arc<IconPackBodyMachineV1>>,
}

impl IconSearchEngine {
    /// Load a search engine from disk, preferring validated raw mmap index files.
    pub fn load_fast(index_dir: &Path) -> Result<Self> {
        let runtime_machine_context = read_runtime_machine_context(index_dir);
        let runtime_catalog_machine =
            read_runtime_catalog_machine(runtime_machine_context.as_ref());
        let runtime_prefix_machine = read_runtime_prefix_machine(runtime_machine_context.as_ref());
        let runtime_perfect_hash_machine =
            read_runtime_perfect_hash_machine(runtime_machine_context.as_ref());
        let runtime_bloom_machine = read_runtime_bloom_machine(runtime_machine_context.as_ref());
        let runtime_lowercase_cache_machine =
            read_runtime_lowercase_cache_machine(runtime_machine_context.as_ref());
        let catalog_machine = runtime_catalog_machine.catalog_machine.as_ref();
        let prefix_machine = runtime_prefix_machine.prefix_machine.as_ref();
        let perfect_hash_machine = runtime_perfect_hash_machine.perfect_hash_machine.as_ref();
        let bloom_machine = runtime_bloom_machine.bloom_machine.as_ref();
        let lowercase_cache_machine = runtime_lowercase_cache_machine
            .lowercase_cache_machine
            .as_ref();

        IconIndex::load_uncompressed_mmap(index_dir)
            .and_then(|index| {
                Self::from_mapped_index_with_catalog_and_prefix(
                    &index,
                    catalog_machine,
                    prefix_machine,
                    perfect_hash_machine,
                    bloom_machine,
                    lowercase_cache_machine,
                )
            })
            .or_else(|_| {
                let index = IconIndex::load_fast(index_dir)?;
                Self::from_index_with_catalog_and_prefix(
                    index,
                    catalog_machine,
                    prefix_machine,
                    perfect_hash_machine,
                    bloom_machine,
                    lowercase_cache_machine,
                )
            })
    }

    /// Load a search engine and attach a validated pack-body machine for render/body resolution.
    pub fn load_fast_with_pack_body_cache(index_dir: &Path) -> Result<Self> {
        let runtime_machine_context = read_runtime_machine_context(index_dir);
        let runtime_catalog_machine =
            read_runtime_catalog_machine(runtime_machine_context.as_ref());
        let runtime_prefix_machine = read_runtime_prefix_machine(runtime_machine_context.as_ref());
        let runtime_perfect_hash_machine =
            read_runtime_perfect_hash_machine(runtime_machine_context.as_ref());
        let runtime_bloom_machine = read_runtime_bloom_machine(runtime_machine_context.as_ref());
        let runtime_lowercase_cache_machine =
            read_runtime_lowercase_cache_machine(runtime_machine_context.as_ref());
        let runtime_pack_body_machine =
            read_runtime_pack_body_machine(runtime_machine_context.as_ref());
        let catalog_machine = runtime_catalog_machine.catalog_machine.as_ref();
        let prefix_machine = runtime_prefix_machine.prefix_machine.as_ref();
        let perfect_hash_machine = runtime_perfect_hash_machine.perfect_hash_machine.as_ref();
        let bloom_machine = runtime_bloom_machine.bloom_machine.as_ref();
        let lowercase_cache_machine = runtime_lowercase_cache_machine
            .lowercase_cache_machine
            .as_ref();
        let pack_body_machine = runtime_pack_body_machine.pack_body_machine.clone();

        IconIndex::load_uncompressed_mmap(index_dir)
            .and_then(|index| {
                Self::from_mapped_index_with_catalog_prefix_and_pack_body(
                    &index,
                    catalog_machine,
                    prefix_machine,
                    perfect_hash_machine,
                    bloom_machine,
                    lowercase_cache_machine,
                    pack_body_machine.clone(),
                )
            })
            .or_else(|_| {
                let index = IconIndex::load_fast(index_dir)?;
                Self::from_index_with_catalog_prefix_and_pack_body(
                    index,
                    catalog_machine,
                    prefix_machine,
                    perfect_hash_machine,
                    bloom_machine,
                    lowercase_cache_machine,
                    pack_body_machine,
                )
            })
    }

    /// Load a search engine and return a local startup timing receipt.
    pub fn load_fast_with_startup_receipt(
        index_dir: &Path,
    ) -> Result<(Self, IconEngineStartupReceipt)> {
        let startup_started = Instant::now();
        let runtime_machine_context = read_runtime_machine_context(index_dir);
        let runtime_catalog_machine =
            read_runtime_catalog_machine(runtime_machine_context.as_ref());
        let runtime_prefix_machine = read_runtime_prefix_machine(runtime_machine_context.as_ref());
        let runtime_perfect_hash_machine =
            read_runtime_perfect_hash_machine(runtime_machine_context.as_ref());
        let runtime_bloom_machine = read_runtime_bloom_machine(runtime_machine_context.as_ref());
        let runtime_lowercase_cache_machine =
            read_runtime_lowercase_cache_machine(runtime_machine_context.as_ref());
        let runtime_pack_body_machine =
            read_runtime_pack_body_machine(runtime_machine_context.as_ref());
        let raw_mmap_validate_load_started = Instant::now();

        match IconIndex::load_uncompressed_mmap(index_dir) {
            Ok(index) => {
                let context = IconEngineStartupContext {
                    index_dir: index_dir.to_path_buf(),
                    load_mode: "raw_mmap",
                    startup_started,
                    raw_mmap_validate_load_ns: duration_nanos_u64(
                        raw_mmap_validate_load_started.elapsed(),
                    ),
                    raw_mmap_ok: true,
                    raw_mmap_error: None,
                    owned_fallback_used: false,
                    owned_fallback_load_ns: 0,
                    owned_fallback_error: None,
                };
                Self::from_metadata_bytes_with_startup_context(
                    index.metadata_bytes(),
                    context,
                    runtime_catalog_machine,
                    runtime_prefix_machine,
                    runtime_perfect_hash_machine,
                    runtime_bloom_machine,
                    runtime_lowercase_cache_machine,
                    runtime_pack_body_machine,
                )
            }
            Err(raw_error) => {
                let raw_mmap_validate_load_ns =
                    duration_nanos_u64(raw_mmap_validate_load_started.elapsed());
                let raw_mmap_error = format!("{raw_error:#}");
                let owned_fallback_load_started = Instant::now();
                let owned_index_result = IconIndex::load_fast(index_dir);
                let owned_fallback_load_ns =
                    duration_nanos_u64(owned_fallback_load_started.elapsed());

                match owned_index_result {
                    Ok(index) => {
                        let context = IconEngineStartupContext {
                            index_dir: index_dir.to_path_buf(),
                            load_mode: "owned_fallback",
                            startup_started,
                            raw_mmap_validate_load_ns,
                            raw_mmap_ok: false,
                            raw_mmap_error: Some(raw_mmap_error),
                            owned_fallback_used: true,
                            owned_fallback_load_ns,
                            owned_fallback_error: None,
                        };
                        Self::from_metadata_bytes_with_startup_context(
                            &index.metadata_bytes,
                            context,
                            runtime_catalog_machine,
                            runtime_prefix_machine,
                            runtime_perfect_hash_machine,
                            runtime_bloom_machine,
                            runtime_lowercase_cache_machine,
                            runtime_pack_body_machine,
                        )
                    }
                    Err(owned_error) => Err(owned_error)
                        .with_context(|| format!("raw mmap load failed first: {raw_mmap_error}")),
                }
            }
        }
    }

    /// Write a local engine-startup receipt without changing normal load behavior.
    pub fn write_startup_performance_receipt(index_dir: &Path) -> Result<PathBuf> {
        let receipt_path = engine_startup_receipt_path(index_dir)?;
        let (_engine, mut receipt) = Self::load_fast_with_startup_receipt(index_dir)?;

        if let Some(parent) = receipt_path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!(
                    "create engine startup performance receipt directory {}",
                    parent.display()
                )
            })?;
        }

        let receipt_write_started = Instant::now();
        let receipt_json = engine_startup_receipt_json(&receipt, &receipt_path);
        write_atomic(&receipt_path, &serde_json::to_vec_pretty(&receipt_json)?).with_context(
            || {
                format!(
                    "write engine startup performance receipt {}",
                    receipt_path.display()
                )
            },
        )?;
        receipt.receipt_write_ns = duration_nanos_u64(receipt_write_started.elapsed());
        let receipt_json = engine_startup_receipt_json(&receipt, &receipt_path);
        write_atomic(&receipt_path, &serde_json::to_vec_pretty(&receipt_json)?).with_context(
            || {
                format!(
                    "write engine startup performance receipt {}",
                    receipt_path.display()
                )
            },
        )?;

        Ok(receipt_path)
    }

    /// Write a local one-icon body-resolution receipt without changing normal search/export behavior.
    pub fn write_body_resolution_performance_receipt(
        index_dir: &Path,
        data_dir: &Path,
        pack: &str,
        name: &str,
    ) -> Result<PathBuf> {
        let receipt_path = body_resolution_receipt_path(index_dir)?;

        if let Some(parent) = receipt_path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!(
                    "create body-resolution performance receipt directory {}",
                    parent.display()
                )
            })?;
        }

        let engine_load_started = Instant::now();
        let engine = Self::load_fast_with_pack_body_cache(index_dir)?;
        let engine_load_with_pack_body_cache_ns = duration_nanos_u64(engine_load_started.elapsed());
        let engine_has_pack_body_cache = engine.has_pack_body_cache();

        let machine_resolve_started = Instant::now();
        let machine_resolved = engine.resolve_icon_body(pack, name);
        let machine_body_resolution_ns = duration_nanos_u64(machine_resolve_started.elapsed());

        let machine_svg_started = Instant::now();
        let machine_svg = machine_resolved.as_ref().and_then(render_resolved_icon_svg);
        let machine_svg_render_ns = duration_nanos_u64(machine_svg_started.elapsed());

        let json_resolve_started = Instant::now();
        let json_resolved = resolve_icon_body_from_json(data_dir, pack, name);
        let json_fallback_resolution_ns = duration_nanos_u64(json_resolve_started.elapsed());

        let json_svg_started = Instant::now();
        let json_svg = json_resolved
            .as_ref()
            .ok()
            .and_then(render_resolved_icon_svg);
        let json_svg_render_ns = duration_nanos_u64(json_svg_started.elapsed());

        let mut receipt = IconBodyResolutionReceipt {
            schema: BODY_RESOLUTION_RECEIPT_SCHEMA,
            index_dir: normalize_path(index_dir),
            data_dir: normalize_path(data_dir),
            pack: pack.to_string(),
            name: name.to_string(),
            engine_load_with_pack_body_cache_ns,
            machine_body_resolution_ns,
            json_fallback_resolution_ns,
            machine_svg_render_ns,
            json_svg_render_ns,
            receipt_write_ns: 0,
            engine_has_pack_body_cache,
            machine_body_resolution_hit: machine_resolved.is_some(),
            machine_body_resolution_has_dimensions: machine_resolved
                .as_ref()
                .is_some_and(|resolved| resolved.width.is_some() && resolved.height.is_some()),
            json_fallback_ok: json_resolved.is_ok(),
            machine_json_svg_match: machine_svg
                .as_ref()
                .zip(json_svg.as_ref())
                .map(|(machine, json)| machine == json),
            machine_body_bytes: machine_resolved
                .as_ref()
                .map(|resolved| resolved.body.len()),
            json_body_bytes: json_resolved
                .as_ref()
                .ok()
                .map(|resolved| resolved.body.len()),
            rendered_machine_svg_bytes: machine_svg.as_ref().map(String::len),
            rendered_json_svg_bytes: json_svg.as_ref().map(String::len),
            json_fallback_error: json_resolved.err().map(|error| format!("{error:#}")),
        };

        let receipt_write_started = Instant::now();
        let receipt_json = body_resolution_receipt_json(&receipt, &receipt_path);
        write_atomic(&receipt_path, &serde_json::to_vec_pretty(&receipt_json)?).with_context(
            || {
                format!(
                    "write body-resolution performance receipt {}",
                    receipt_path.display()
                )
            },
        )?;
        receipt.receipt_write_ns = duration_nanos_u64(receipt_write_started.elapsed());
        let receipt_json = body_resolution_receipt_json(&receipt, &receipt_path);
        write_atomic(&receipt_path, &serde_json::to_vec_pretty(&receipt_json)?).with_context(
            || {
                format!(
                    "write body-resolution performance receipt {}",
                    receipt_path.display()
                )
            },
        )?;

        Ok(receipt_path)
    }

    /// Write a local bounded warm-cache query latency receipt without exporting files.
    pub fn write_query_latency_performance_receipt(
        index_dir: &Path,
        data_dir: &Path,
        queries: &[&str],
        limit: usize,
        warmup_runs: usize,
        measured_runs: usize,
    ) -> Result<PathBuf> {
        let receipt_path = query_latency_receipt_path(index_dir)?;
        let bounded = bounded_query_latency_inputs(queries, limit, warmup_runs, measured_runs)?;

        if let Some(parent) = receipt_path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!(
                    "create query-latency performance receipt directory {}",
                    parent.display()
                )
            })?;
        }

        let engine_load_started = Instant::now();
        let engine = Self::load_fast_with_pack_body_cache(index_dir)?;
        let engine_load_with_pack_body_cache_ns = duration_nanos_u64(engine_load_started.elapsed());
        let engine_has_pack_body_cache = engine.has_pack_body_cache();
        let effective_limit = bounded.effective_limit;
        let effective_warmup_runs = bounded.effective_warmup_runs;
        let effective_measured_runs = bounded.effective_measured_runs;
        let mut samples = Vec::with_capacity(bounded.queries.len() * effective_measured_runs);

        for query in &bounded.queries {
            for _ in 0..effective_warmup_runs {
                let _ = engine.search(query, effective_limit);
            }

            for run_index in 0..effective_measured_runs {
                let search_started = Instant::now();
                let results = engine.search(query, effective_limit);
                let search_ns = duration_nanos_u64(search_started.elapsed());
                let result_count = results.len();

                let mut top_result_pack = None;
                let mut top_result_name = None;
                let mut top_result_score = None;
                let mut top_result_match_type = None;
                let mut body_resolution_ns = 0;
                let mut body_resolution_hit = false;
                let mut body_resolution_source = "no_result";
                let mut svg_render_ns = 0;
                let mut svg_rendered = false;
                let mut rendered_svg_bytes = None;

                if let Some(top_result) = results.first() {
                    let top_icon = top_result.icon.clone();
                    top_result_pack = Some(top_icon.pack.clone());
                    top_result_name = Some(top_icon.name.clone());
                    top_result_score = Some(top_result.score);
                    top_result_match_type = Some(match_type_label(top_result.match_type));

                    let body_started = Instant::now();
                    let machine_resolved = engine.resolve_icon_body(&top_icon.pack, &top_icon.name);
                    body_resolution_hit = machine_resolved.is_some();
                    let mut resolved = machine_resolved
                        .filter(|body| body.width.is_some() && body.height.is_some());

                    if resolved.is_some() {
                        body_resolution_source = "pack_body_machine";
                    } else {
                        resolved =
                            resolve_icon_body_from_json(data_dir, &top_icon.pack, &top_icon.name)
                                .ok();
                        body_resolution_source = if resolved.is_some() {
                            "json_fallback"
                        } else {
                            "unavailable"
                        };
                    }
                    body_resolution_ns = duration_nanos_u64(body_started.elapsed());

                    let render_started = Instant::now();
                    let rendered = resolved.as_ref().and_then(render_resolved_icon_svg);
                    svg_render_ns = duration_nanos_u64(render_started.elapsed());
                    svg_rendered = rendered.is_some();
                    rendered_svg_bytes = rendered.as_ref().map(String::len);
                }

                let export_like_top_result_ns = search_ns
                    .saturating_add(body_resolution_ns)
                    .saturating_add(svg_render_ns);
                samples.push(IconQueryLatencySample {
                    query: query.clone(),
                    run_index,
                    search_ns,
                    result_count,
                    top_result_pack,
                    top_result_name,
                    top_result_score,
                    top_result_match_type,
                    body_resolution_ns,
                    body_resolution_hit,
                    body_resolution_source,
                    export_like_source: body_resolution_source,
                    svg_render_ns,
                    svg_rendered,
                    rendered_svg_bytes,
                    export_like_top_result_ns,
                });
            }
        }

        let mut receipt = IconQueryLatencyReceipt {
            schema: QUERY_LATENCY_RECEIPT_SCHEMA,
            index_dir: normalize_path(index_dir),
            data_dir: normalize_path(data_dir),
            requested_query_count: queries.len(),
            effective_query_count: bounded.queries.len(),
            requested_limit: limit,
            effective_limit,
            requested_warmup_runs: warmup_runs,
            effective_warmup_runs,
            requested_measured_runs: measured_runs,
            effective_measured_runs,
            engine_load_with_pack_body_cache_ns,
            receipt_write_ns: 0,
            engine_has_pack_body_cache,
            samples,
        };

        let receipt_write_started = Instant::now();
        let receipt_json = query_latency_receipt_json(&receipt, &receipt_path);
        write_atomic(&receipt_path, &serde_json::to_vec_pretty(&receipt_json)?).with_context(
            || {
                format!(
                    "write query-latency performance receipt {}",
                    receipt_path.display()
                )
            },
        )?;
        receipt.receipt_write_ns = duration_nanos_u64(receipt_write_started.elapsed());
        let receipt_json = query_latency_receipt_json(&receipt, &receipt_path);
        write_atomic(&receipt_path, &serde_json::to_vec_pretty(&receipt_json)?).with_context(
            || {
                format!(
                    "write query-latency performance receipt {}",
                    receipt_path.display()
                )
            },
        )?;

        Ok(receipt_path)
    }

    /// Create engine from index with pre-computed indices
    pub fn from_index(index: IconIndex) -> Result<Self> {
        Self::from_metadata_bytes(&index.metadata_bytes)
    }

    fn from_index_with_catalog_and_prefix(
        index: IconIndex,
        catalog_machine: Option<&IconCatalogMachineV1>,
        prefix_machine: Option<&IconPrefixMachineV1>,
        perfect_hash_machine: Option<&IconPerfectHashMachineV1>,
        bloom_machine: Option<&IconBloomMachineV1>,
        lowercase_cache_machine: Option<&IconLowercaseCacheMachineV1>,
    ) -> Result<Self> {
        Self::from_metadata_bytes_with_catalog_and_prefix(
            &index.metadata_bytes,
            catalog_machine,
            prefix_machine,
            perfect_hash_machine,
            bloom_machine,
            lowercase_cache_machine,
        )
    }

    fn from_index_with_catalog_prefix_and_pack_body(
        index: IconIndex,
        catalog_machine: Option<&IconCatalogMachineV1>,
        prefix_machine: Option<&IconPrefixMachineV1>,
        perfect_hash_machine: Option<&IconPerfectHashMachineV1>,
        bloom_machine: Option<&IconBloomMachineV1>,
        lowercase_cache_machine: Option<&IconLowercaseCacheMachineV1>,
        pack_body_machine: Option<Arc<IconPackBodyMachineV1>>,
    ) -> Result<Self> {
        Self::from_metadata_bytes_with_catalog_prefix_and_pack_body(
            &index.metadata_bytes,
            catalog_machine,
            prefix_machine,
            perfect_hash_machine,
            bloom_machine,
            lowercase_cache_machine,
            pack_body_machine,
        )
    }

    /// Create engine from a memory-mapped uncompressed index.
    pub fn from_mapped_index(index: &MappedIconIndex) -> Result<Self> {
        Self::from_metadata_bytes(index.metadata_bytes())
    }

    fn from_mapped_index_with_catalog_and_prefix(
        index: &MappedIconIndex,
        catalog_machine: Option<&IconCatalogMachineV1>,
        prefix_machine: Option<&IconPrefixMachineV1>,
        perfect_hash_machine: Option<&IconPerfectHashMachineV1>,
        bloom_machine: Option<&IconBloomMachineV1>,
        lowercase_cache_machine: Option<&IconLowercaseCacheMachineV1>,
    ) -> Result<Self> {
        Self::from_metadata_bytes_with_catalog_and_prefix(
            index.metadata_bytes(),
            catalog_machine,
            prefix_machine,
            perfect_hash_machine,
            bloom_machine,
            lowercase_cache_machine,
        )
    }

    fn from_mapped_index_with_catalog_prefix_and_pack_body(
        index: &MappedIconIndex,
        catalog_machine: Option<&IconCatalogMachineV1>,
        prefix_machine: Option<&IconPrefixMachineV1>,
        perfect_hash_machine: Option<&IconPerfectHashMachineV1>,
        bloom_machine: Option<&IconBloomMachineV1>,
        lowercase_cache_machine: Option<&IconLowercaseCacheMachineV1>,
        pack_body_machine: Option<Arc<IconPackBodyMachineV1>>,
    ) -> Result<Self> {
        Self::from_metadata_bytes_with_catalog_prefix_and_pack_body(
            index.metadata_bytes(),
            catalog_machine,
            prefix_machine,
            perfect_hash_machine,
            bloom_machine,
            lowercase_cache_machine,
            pack_body_machine,
        )
    }

    fn from_metadata_bytes(metadata_bytes: &[u8]) -> Result<Self> {
        Self::from_metadata_bytes_with_prefix(metadata_bytes, None)
    }

    fn from_metadata_bytes_with_prefix(
        metadata_bytes: &[u8],
        prefix_machine: Option<&IconPrefixMachineV1>,
    ) -> Result<Self> {
        Self::from_metadata_bytes_with_catalog_and_prefix(
            metadata_bytes,
            None,
            prefix_machine,
            None,
            None,
            None,
        )
    }

    fn from_metadata_bytes_with_catalog_and_prefix(
        metadata_bytes: &[u8],
        catalog_machine: Option<&IconCatalogMachineV1>,
        prefix_machine: Option<&IconPrefixMachineV1>,
        perfect_hash_machine: Option<&IconPerfectHashMachineV1>,
        bloom_machine: Option<&IconBloomMachineV1>,
        lowercase_cache_machine: Option<&IconLowercaseCacheMachineV1>,
    ) -> Result<Self> {
        Self::from_metadata_bytes_with_catalog_prefix_and_pack_body(
            metadata_bytes,
            catalog_machine,
            prefix_machine,
            perfect_hash_machine,
            bloom_machine,
            lowercase_cache_machine,
            None,
        )
    }

    fn from_metadata_bytes_with_catalog_prefix_and_pack_body(
        metadata_bytes: &[u8],
        catalog_machine: Option<&IconCatalogMachineV1>,
        prefix_machine: Option<&IconPrefixMachineV1>,
        perfect_hash_machine: Option<&IconPerfectHashMachineV1>,
        bloom_machine: Option<&IconBloomMachineV1>,
        lowercase_cache_machine: Option<&IconLowercaseCacheMachineV1>,
        pack_body_machine: Option<Arc<IconPackBodyMachineV1>>,
    ) -> Result<Self> {
        let archived =
            rkyv::access::<Archived<Vec<IconMetadata>>, rkyv::rancor::Error>(metadata_bytes)
                .map_err(|error| anyhow!("icon metadata rkyv bytecheck failed: {error}"))?;

        // Deserialize metadata
        let metadata: Vec<IconMetadata> = archived
            .iter()
            .map(|item| IconMetadata {
                id: item.id.into(),
                name: item.name.to_string(),
                pack: item.pack.to_string(),
                category: item.category.to_string(),
                tags: item.tags.iter().map(|t| t.to_string()).collect(),
                popularity: item.popularity.into(),
            })
            .collect();
        let (metadata, _catalog_adoption) =
            adopt_catalog_machine_metadata(metadata, catalog_machine);
        let pack_body_machine_for_engine = pack_body_machine.filter(|machine| {
            validate_icon_pack_body_runtime_metadata(machine.as_ref(), metadata.as_slice()).is_ok()
        });
        let (perfect_hash_index, _perfect_hash_adoption) =
            adopt_runtime_perfect_hash_machine(metadata.as_slice(), perfect_hash_machine);
        let (bloom_filters, _bloom_adoption) =
            adopt_runtime_bloom_machine(metadata.as_slice(), bloom_machine);
        let (lowercase_cache, _lowercase_cache_adoption) =
            adopt_runtime_lowercase_cache_machine(metadata.as_slice(), lowercase_cache_machine);

        // Build all pre-computed indices silently
        let (
            precomputed,
            _timings,
            _prefix_adoption,
            _perfect_hash_adoption,
            _bloom_adoption,
            _lowercase_cache_adoption,
        ) = PrecomputedIndex::build_with_timings_and_optional_precomputed_machines(
            metadata,
            prefix_machine,
            perfect_hash_index,
            bloom_filters,
            lowercase_cache,
        );
        let precomputed = Arc::new(precomputed);

        Ok(Self {
            precomputed,
            cache: Arc::new(DashMap::new()),
            pack_body_machine: pack_body_machine_for_engine,
        })
    }

    pub fn from_metadata_bytes_with_startup_receipt(
        metadata_bytes: &[u8],
    ) -> Result<(Self, IconEngineStartupReceipt)> {
        let context = IconEngineStartupContext {
            index_dir: PathBuf::new(),
            load_mode: "metadata_bytes",
            startup_started: Instant::now(),
            raw_mmap_validate_load_ns: 0,
            raw_mmap_ok: false,
            raw_mmap_error: None,
            owned_fallback_used: false,
            owned_fallback_load_ns: 0,
            owned_fallback_error: None,
        };
        Self::from_metadata_bytes_with_startup_context(
            metadata_bytes,
            context,
            RuntimeCatalogMachine::unavailable(
                "metadata_bytes_load_has_no_catalog_machine_context".to_string(),
            ),
            RuntimePrefixMachine::unavailable(
                "metadata_bytes_load_has_no_prefix_machine_context".to_string(),
            ),
            RuntimePerfectHashMachine::unavailable(
                "metadata_bytes_load_has_no_perfect_hash_machine_context".to_string(),
            ),
            RuntimeBloomMachine::unavailable(
                "metadata_bytes_load_has_no_bloom_machine_context".to_string(),
            ),
            RuntimeLowercaseCacheMachine::unavailable(
                "metadata_bytes_load_has_no_lowercase_cache_machine_context".to_string(),
            ),
            RuntimePackBodyMachine::unavailable(
                "metadata_bytes_load_has_no_pack_body_machine_context".to_string(),
            ),
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn from_metadata_bytes_with_startup_context(
        metadata_bytes: &[u8],
        context: IconEngineStartupContext,
        runtime_catalog_machine: RuntimeCatalogMachine,
        runtime_prefix_machine: RuntimePrefixMachine,
        runtime_perfect_hash_machine: RuntimePerfectHashMachine,
        runtime_bloom_machine: RuntimeBloomMachine,
        runtime_lowercase_cache_machine: RuntimeLowercaseCacheMachine,
        runtime_pack_body_machine: RuntimePackBodyMachine,
    ) -> Result<(Self, IconEngineStartupReceipt)> {
        let from_metadata_bytes_started = Instant::now();
        let metadata_bytecheck_started = Instant::now();
        let archived =
            rkyv::access::<Archived<Vec<IconMetadata>>, rkyv::rancor::Error>(metadata_bytes)
                .map_err(|error| anyhow!("icon metadata rkyv bytecheck failed: {error}"))?;
        let metadata_bytecheck_ns = duration_nanos_u64(metadata_bytecheck_started.elapsed());

        let metadata_materialize_started = Instant::now();
        let metadata: Vec<IconMetadata> = archived
            .iter()
            .map(|item| IconMetadata {
                id: item.id.into(),
                name: item.name.to_string(),
                pack: item.pack.to_string(),
                category: item.category.to_string(),
                tags: item.tags.iter().map(|t| t.to_string()).collect(),
                popularity: item.popularity.into(),
            })
            .collect();
        let metadata_materialize_ns = duration_nanos_u64(metadata_materialize_started.elapsed());
        let metadata_bytes_len = metadata_bytes.len() as u64;
        let (metadata, catalog_adoption) = adopt_catalog_machine_metadata(
            metadata,
            runtime_catalog_machine.catalog_machine.as_ref(),
        );
        let icon_count = metadata.len();
        let pack_body_evidence = validate_runtime_pack_body_machine_evidence(
            metadata.as_slice(),
            runtime_pack_body_machine.pack_body_machine.as_deref(),
        );
        let pack_body_machine_for_engine =
            if pack_body_evidence.runtime_pack_body_machine_evidence_validated {
                runtime_pack_body_machine.pack_body_machine.clone()
            } else {
                None
            };
        let (perfect_hash_index, perfect_hash_adoption) = adopt_runtime_perfect_hash_machine(
            metadata.as_slice(),
            runtime_perfect_hash_machine.perfect_hash_machine.as_ref(),
        );
        let (bloom_filters, bloom_adoption) = adopt_runtime_bloom_machine(
            metadata.as_slice(),
            runtime_bloom_machine.bloom_machine.as_ref(),
        );
        let (lowercase_cache, lowercase_cache_adoption) = adopt_runtime_lowercase_cache_machine(
            metadata.as_slice(),
            runtime_lowercase_cache_machine
                .lowercase_cache_machine
                .as_ref(),
        );

        let (
            precomputed,
            precomputed_timings,
            prefix_adoption,
            _precomputed_hash_adoption,
            _bloom_adoption,
            _lowercase_cache_adoption,
        ) = PrecomputedIndex::build_with_timings_and_optional_precomputed_machines(
            metadata,
            runtime_prefix_machine.prefix_machine.as_ref(),
            perfect_hash_index,
            bloom_filters,
            lowercase_cache,
        );

        let engine_cache_init_started = Instant::now();
        let engine = Self {
            precomputed: Arc::new(precomputed),
            cache: Arc::new(DashMap::new()),
            pack_body_machine: pack_body_machine_for_engine,
        };
        let engine_cache_init_ns = duration_nanos_u64(engine_cache_init_started.elapsed());

        let receipt = build_engine_startup_receipt(
            context,
            metadata_bytes_len,
            icon_count,
            metadata_bytecheck_ns,
            metadata_materialize_ns,
            duration_nanos_u64(from_metadata_bytes_started.elapsed()),
            precomputed_timings,
            catalog_adoption,
            runtime_catalog_machine,
            perfect_hash_adoption,
            runtime_perfect_hash_machine,
            bloom_adoption,
            runtime_bloom_machine,
            lowercase_cache_adoption,
            runtime_lowercase_cache_machine,
            prefix_adoption,
            runtime_prefix_machine,
            pack_body_evidence,
            runtime_pack_body_machine,
            engine_cache_init_ns,
        );

        Ok((engine, receipt))
    }

    /// Search icons with the configured index, filter, and cache paths.
    pub fn search(&self, query: &str, limit: usize) -> Vec<SearchResult> {
        // Check lock-free cache first
        if let Some(cached) = self.cache.get(query) {
            return cached.value().iter().take(limit).cloned().collect();
        }

        // Use optimized search path
        self.search_optimized(query, limit)
    }

    /// Check if query is cached
    pub fn is_cached(&self, query: &str) -> bool {
        self.cache.contains_key(query)
    }

    pub fn has_pack_body_cache(&self) -> bool {
        self.pack_body_machine.is_some()
    }

    pub fn resolve_icon_body(&self, pack: &str, name: &str) -> Option<ResolvedIconPackBody> {
        let machine = self.pack_body_machine.as_ref()?;
        resolve_icon_pack_body(machine.as_ref(), pack, name)
    }

    /// Optimized search with all 5 improvements
    fn search_optimized(&self, query: &str, limit: usize) -> Vec<SearchResult> {
        let query_lower = query.to_lowercase();

        // OPTIMIZATION 1: Start with exact match via perfect hashing (if exists)
        let mut results: smallvec::SmallVec<[SearchResult; 128]> = smallvec::SmallVec::new();

        if let Some(idx) = self.precomputed.perfect_hash.lookup_exact(&query_lower) {
            let icon = &self.precomputed.metadata[idx as usize];
            let score = calculate_score(
                &query_lower,
                &query_lower,
                MatchType::Exact,
                icon.popularity,
            );
            results.push(SearchResult::new(icon.clone(), score, MatchType::Exact));
        }

        // OPTIMIZATION 2 & 3: Use prefix index + bloom filters for fast candidate selection
        let candidates = if query_lower.len() <= 3 {
            // Use prefix index for short queries
            self.precomputed
                .prefix_index
                .get_candidates(&query_lower)
                .map(|c| c.to_vec())
                .unwrap_or_else(|| (0..self.precomputed.metadata.len() as u32).collect())
        } else {
            // Use prefix index with first 3 chars
            self.precomputed
                .prefix_index
                .get_candidates(&query_lower[..3])
                .map(|c| c.to_vec())
                .unwrap_or_else(|| (0..self.precomputed.metadata.len() as u32).collect())
        };

        // OPTIMIZATION 4: Zero-allocation search with pre-computed lowercase
        let query_bytes = query_lower.as_bytes();

        // OPTIMIZATION 5: Single-threaded for small candidate sets, parallel for large
        if candidates.len() < 1000 {
            // Single-threaded path (no overhead)
            for &idx in &candidates {
                let idx = idx as usize;

                // Bloom filter rejection (90%+ filtered out)
                if !self
                    .precomputed
                    .bloom_filters
                    .might_match(idx, &query_lower)
                {
                    continue;
                }

                let icon = &self.precomputed.metadata[idx];
                let icon_name_lower = self.precomputed.lowercase_cache.get(idx);
                let icon_bytes = icon_name_lower.as_bytes();

                // Fast exact/prefix/substring matching
                if icon_bytes == query_bytes {
                    // Skip if already added via perfect hash
                    if results.iter().any(|r| r.icon.id == icon.id) {
                        continue;
                    }
                    let score = calculate_score(
                        &query_lower,
                        icon_name_lower,
                        MatchType::Exact,
                        icon.popularity,
                    );
                    results.push(SearchResult::new(icon.clone(), score, MatchType::Exact));
                } else if memchr::memmem::find(icon_bytes, query_bytes).is_some() {
                    let (match_type, multiplier) = if icon_name_lower.starts_with(&query_lower) {
                        (MatchType::Prefix, 0.8)
                    } else {
                        (MatchType::Prefix, 0.7)
                    };
                    let score =
                        calculate_score(&query_lower, icon_name_lower, match_type, icon.popularity)
                            * multiplier;
                    results.push(SearchResult::new(icon.clone(), score, match_type));
                }
            }
        } else {
            // Parallel path for large candidate sets
            let parallel_results: Vec<SearchResult> = candidates
                .par_chunks(256)
                .flat_map(|chunk| {
                    let mut local_results = smallvec::SmallVec::<[SearchResult; 128]>::new();

                    for &idx in chunk {
                        let idx = idx as usize;

                        if !self
                            .precomputed
                            .bloom_filters
                            .might_match(idx, &query_lower)
                        {
                            continue;
                        }

                        let icon = &self.precomputed.metadata[idx];
                        let icon_name_lower = self.precomputed.lowercase_cache.get(idx);
                        let icon_bytes = icon_name_lower.as_bytes();

                        if icon_bytes == query_bytes {
                            let score = calculate_score(
                                &query_lower,
                                icon_name_lower,
                                MatchType::Exact,
                                icon.popularity,
                            );
                            local_results.push(SearchResult::new(
                                icon.clone(),
                                score,
                                MatchType::Exact,
                            ));
                        } else if memchr::memmem::find(icon_bytes, query_bytes).is_some() {
                            let (match_type, multiplier) =
                                if icon_name_lower.starts_with(&query_lower) {
                                    (MatchType::Prefix, 0.8)
                                } else {
                                    (MatchType::Prefix, 0.7)
                                };
                            let score = calculate_score(
                                &query_lower,
                                icon_name_lower,
                                match_type,
                                icon.popularity,
                            ) * multiplier;
                            local_results.push(SearchResult::new(icon.clone(), score, match_type));
                        }
                    }

                    local_results.into_vec()
                })
                .collect();

            results.extend(parallel_results);
        }

        // Fallback to fuzzy search if no results
        let mut final_results = if results.is_empty() {
            self.fallback_search(&query_lower, limit)
        } else {
            results.into_vec()
        };

        // Sort and deduplicate
        final_results.par_sort_unstable_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        final_results.dedup_by(|a, b| a.icon.id == b.icon.id);

        let final_results: Vec<_> = final_results.into_iter().take(limit).collect();

        // Cache results
        self.cache.insert(query.to_string(), final_results.clone());

        final_results
    }

    /// Fallback search - parallel fuzzy matching with SIMD acceleration
    fn fallback_search(&self, query: &str, limit: usize) -> Vec<SearchResult> {
        let threshold = 0.5;

        let mut results: Vec<SearchResult> = self
            .precomputed
            .metadata
            .par_chunks(RAYON_CHUNK_SIZE) // Cache-friendly chunking
            .take(100000 / RAYON_CHUNK_SIZE)
            .flat_map(|chunk| {
                chunk
                    .iter()
                    .filter_map(|icon| {
                        let icon_name_lower = icon.name.to_lowercase();

                        if let Some(similarity) = fuzzy_match(query, &icon_name_lower, threshold) {
                            let score = calculate_score(
                                query,
                                &icon_name_lower,
                                MatchType::Fuzzy,
                                icon.popularity,
                            ) * similarity;
                            Some(SearchResult::new(icon.clone(), score, MatchType::Fuzzy))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        results.par_sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        results.into_iter().take(limit).collect()
    }

    /// Get total icon count
    pub fn total_icons(&self) -> usize {
        self.precomputed.metadata.len()
    }

    /// Clear cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// Check if GPU is being used
    pub fn is_gpu_enabled(&self) -> bool {
        false
    }
}

fn read_runtime_machine_context(
    index_dir: &Path,
) -> std::result::Result<RuntimeMachineReadContext, RuntimeMachineReadContextError> {
    let project_root = match engine_startup_project_root(index_dir) {
        Ok(project_root) => project_root,
        Err(error) => return Err(RuntimeMachineReadContextError::ProjectRoot(error)),
    };
    let data_dir = match select_icon_data_dir(&project_root) {
        Some(data_dir) => data_dir,
        None => return Err(RuntimeMachineReadContextError::DataDirNotFound),
    };
    let source_fingerprint = icon_catalog_source_fingerprint(&data_dir)
        .map_err(RuntimeMachineReadContextError::SourceFingerprint)?;

    Ok(RuntimeMachineReadContext {
        project_root,
        data_dir,
        source_fingerprint,
    })
}

fn runtime_machine_context_unavailable_reason(
    machine_name: &str,
    error: &RuntimeMachineReadContextError,
) -> String {
    match error {
        RuntimeMachineReadContextError::ProjectRoot(error) => {
            format!("{machine_name}_machine_project_root_unavailable: {error:#}")
        }
        RuntimeMachineReadContextError::DataDirNotFound => {
            format!("{machine_name}_machine_data_dir_not_found")
        }
        RuntimeMachineReadContextError::SourceFingerprint(error) => {
            format!("{machine_name}_machine_source_fingerprint_unavailable: {error:#}")
        }
    }
}

fn read_runtime_catalog_machine(
    context: std::result::Result<&RuntimeMachineReadContext, &RuntimeMachineReadContextError>,
) -> RuntimeCatalogMachine {
    let context = match context {
        Ok(context) => context,
        Err(error) => {
            return RuntimeCatalogMachine::unavailable(runtime_machine_context_unavailable_reason(
                "catalog", error,
            ));
        }
    };

    match read_icon_catalog_machine_cache_with_source_fingerprint(
        &context.project_root,
        &context.data_dir,
        &context.source_fingerprint,
    ) {
        Ok(read) => RuntimeCatalogMachine::available(read.catalog_machine, read.mode),
        Err(error) => {
            RuntimeCatalogMachine::unavailable(format!("catalog_machine_not_available: {error:#}"))
        }
    }
}

fn read_runtime_prefix_machine(
    context: std::result::Result<&RuntimeMachineReadContext, &RuntimeMachineReadContextError>,
) -> RuntimePrefixMachine {
    let context = match context {
        Ok(context) => context,
        Err(error) => {
            return RuntimePrefixMachine::unavailable(runtime_machine_context_unavailable_reason(
                "prefix", error,
            ));
        }
    };

    match read_icon_prefix_machine_cache_with_source_fingerprint(
        &context.project_root,
        &context.data_dir,
        &context.source_fingerprint,
    ) {
        Ok(read) => RuntimePrefixMachine::available(read.prefix_machine, read.mode),
        Err(error) => {
            RuntimePrefixMachine::unavailable(format!("prefix_machine_not_available: {error:#}"))
        }
    }
}

fn read_runtime_perfect_hash_machine(
    context: std::result::Result<&RuntimeMachineReadContext, &RuntimeMachineReadContextError>,
) -> RuntimePerfectHashMachine {
    let context = match context {
        Ok(context) => context,
        Err(error) => {
            return RuntimePerfectHashMachine::unavailable(
                runtime_machine_context_unavailable_reason("perfect_hash", error),
            );
        }
    };

    match read_icon_perfect_hash_machine_cache_with_source_fingerprint(
        &context.project_root,
        &context.data_dir,
        &context.source_fingerprint,
    ) {
        Ok(read) => RuntimePerfectHashMachine::available(read.perfect_hash_machine, read.mode),
        Err(error) => RuntimePerfectHashMachine::unavailable(format!(
            "perfect_hash_machine_not_available: {error:#}"
        )),
    }
}

fn read_runtime_bloom_machine(
    context: std::result::Result<&RuntimeMachineReadContext, &RuntimeMachineReadContextError>,
) -> RuntimeBloomMachine {
    let context = match context {
        Ok(context) => context,
        Err(error) => {
            return RuntimeBloomMachine::unavailable(runtime_machine_context_unavailable_reason(
                "bloom", error,
            ));
        }
    };

    match read_icon_bloom_machine_cache_with_source_fingerprint(
        &context.project_root,
        &context.data_dir,
        &context.source_fingerprint,
    ) {
        Ok(read) => RuntimeBloomMachine::available(read.bloom_machine, read.mode),
        Err(error) => {
            RuntimeBloomMachine::unavailable(format!("bloom_machine_not_available: {error:#}"))
        }
    }
}

fn read_runtime_lowercase_cache_machine(
    context: std::result::Result<&RuntimeMachineReadContext, &RuntimeMachineReadContextError>,
) -> RuntimeLowercaseCacheMachine {
    let context = match context {
        Ok(context) => context,
        Err(error) => {
            return RuntimeLowercaseCacheMachine::unavailable(
                runtime_machine_context_unavailable_reason("lowercase_cache", error),
            );
        }
    };

    match read_icon_lowercase_cache_machine_cache_with_source_fingerprint(
        &context.project_root,
        &context.data_dir,
        &context.source_fingerprint,
    ) {
        Ok(read) => {
            RuntimeLowercaseCacheMachine::available(read.lowercase_cache_machine, read.mode)
        }
        Err(error) => RuntimeLowercaseCacheMachine::unavailable(format!(
            "lowercase_cache_machine_not_available: {error:#}"
        )),
    }
}

fn read_runtime_pack_body_machine(
    context: std::result::Result<&RuntimeMachineReadContext, &RuntimeMachineReadContextError>,
) -> RuntimePackBodyMachine {
    let context = match context {
        Ok(context) => context,
        Err(error) => {
            return RuntimePackBodyMachine::unavailable(
                runtime_machine_context_unavailable_reason("pack_body", error),
            );
        }
    };

    match read_icon_pack_body_machine_cache_with_source_fingerprint(
        &context.project_root,
        &context.data_dir,
        &context.source_fingerprint,
    ) {
        Ok(read) => RuntimePackBodyMachine::available(read.pack_body_machine, read.mode),
        Err(error) => RuntimePackBodyMachine::unavailable(format!(
            "pack_body_machine_not_available: {error:#}"
        )),
    }
}

fn adopt_catalog_machine_metadata(
    runtime_metadata: Vec<IconMetadata>,
    catalog_machine: Option<&IconCatalogMachineV1>,
) -> (Vec<IconMetadata>, CatalogMachineMetadataAdoptionSummary) {
    let Some(catalog_machine) = catalog_machine else {
        return (
            runtime_metadata,
            CatalogMachineMetadataAdoptionSummary::unavailable(),
        );
    };

    match icon_metadata_from_catalog_machine(catalog_machine).and_then(|catalog_metadata| {
        validate_catalog_machine_metadata_parity(&catalog_metadata, &runtime_metadata)?;
        Ok(catalog_metadata)
    }) {
        Ok(catalog_metadata) => (
            catalog_metadata,
            CatalogMachineMetadataAdoptionSummary::adopted(),
        ),
        Err(error) => (
            runtime_metadata,
            CatalogMachineMetadataAdoptionSummary::fallback(error),
        ),
    }
}

fn adopt_runtime_perfect_hash_machine(
    metadata: &[IconMetadata],
    machine: Option<&IconPerfectHashMachineV1>,
) -> (Option<PerfectHashIndex>, PerfectHashMachineAdoptionSummary) {
    let Some(machine) = machine else {
        return (None, PerfectHashMachineAdoptionSummary::unavailable());
    };

    match perfect_hash_index_from_machine(machine, metadata) {
        Ok(index) => (Some(index), PerfectHashMachineAdoptionSummary::adopted()),
        Err(error) => (
            None,
            PerfectHashMachineAdoptionSummary::fallback(error.to_string()),
        ),
    }
}

fn adopt_runtime_bloom_machine(
    metadata: &[IconMetadata],
    machine: Option<&IconBloomMachineV1>,
) -> (Option<IconBloomFilters>, BloomMachineAdoptionSummary) {
    let Some(machine) = machine else {
        return (None, BloomMachineAdoptionSummary::unavailable());
    };

    match bloom_filters_from_machine(machine, metadata) {
        Ok(filters) => (Some(filters), BloomMachineAdoptionSummary::adopted()),
        Err(error) => (
            None,
            BloomMachineAdoptionSummary::fallback(error.to_string()),
        ),
    }
}

fn adopt_runtime_lowercase_cache_machine(
    metadata: &[IconMetadata],
    machine: Option<&IconLowercaseCacheMachineV1>,
) -> (
    Option<ValidatedLowercaseCache>,
    LowercaseCacheMachineAdoptionSummary,
) {
    let Some(machine) = machine else {
        return (None, LowercaseCacheMachineAdoptionSummary::unavailable());
    };

    match lowercase_cache_from_machine(machine, metadata) {
        Ok(cache) => (Some(cache), LowercaseCacheMachineAdoptionSummary::adopted()),
        Err(error) => (
            None,
            LowercaseCacheMachineAdoptionSummary::fallback(error.to_string()),
        ),
    }
}

fn validate_runtime_pack_body_machine_evidence(
    metadata: &[IconMetadata],
    machine: Option<&IconPackBodyMachineV1>,
) -> PackBodyMachineEvidenceSummary {
    let Some(machine) = machine else {
        return PackBodyMachineEvidenceSummary::unavailable();
    };

    match validate_icon_pack_body_runtime_metadata(machine, metadata) {
        Ok(()) => PackBodyMachineEvidenceSummary::validated(machine),
        Err(error) => PackBodyMachineEvidenceSummary::fallback(machine, error),
    }
}

#[allow(clippy::too_many_arguments)]
fn build_engine_startup_receipt(
    context: IconEngineStartupContext,
    metadata_bytes: u64,
    icon_count: usize,
    metadata_bytecheck_ns: u64,
    metadata_materialize_ns: u64,
    from_metadata_bytes_total_ns: u64,
    precomputed_timings: PrecomputedBuildTimings,
    catalog_adoption: CatalogMachineMetadataAdoptionSummary,
    runtime_catalog_machine: RuntimeCatalogMachine,
    perfect_hash_adoption: PerfectHashMachineAdoptionSummary,
    runtime_perfect_hash_machine: RuntimePerfectHashMachine,
    bloom_adoption: BloomMachineAdoptionSummary,
    runtime_bloom_machine: RuntimeBloomMachine,
    lowercase_cache_adoption: LowercaseCacheMachineAdoptionSummary,
    runtime_lowercase_cache_machine: RuntimeLowercaseCacheMachine,
    prefix_adoption: PrefixMachineAdoptionSummary,
    runtime_prefix_machine: RuntimePrefixMachine,
    pack_body_evidence: PackBodyMachineEvidenceSummary,
    runtime_pack_body_machine: RuntimePackBodyMachine,
    engine_cache_init_ns: u64,
) -> IconEngineStartupReceipt {
    let runtime_catalog_machine_available = runtime_catalog_machine
        .runtime_catalog_machine_available
        || catalog_adoption.runtime_catalog_machine_available;
    let catalog_machine_fallback_reason = if catalog_adoption.runtime_catalog_machine_adopted {
        None
    } else if runtime_catalog_machine.runtime_catalog_machine_available {
        catalog_adoption.catalog_machine_fallback_reason.clone()
    } else {
        runtime_catalog_machine
            .catalog_machine_fallback_reason
            .clone()
            .or(catalog_adoption.catalog_machine_fallback_reason.clone())
    };
    let runtime_prefix_machine_available = runtime_prefix_machine.runtime_prefix_machine_available
        || prefix_adoption.runtime_prefix_machine_available;
    let prefix_machine_fallback_reason = if prefix_adoption.runtime_prefix_machine_adopted {
        None
    } else if runtime_prefix_machine.runtime_prefix_machine_available {
        prefix_adoption.prefix_machine_fallback_reason.clone()
    } else {
        runtime_prefix_machine
            .prefix_machine_fallback_reason
            .clone()
            .or(prefix_adoption.prefix_machine_fallback_reason.clone())
    };
    let runtime_prefix_index_source = if prefix_adoption.runtime_prefix_machine_adopted {
        "prefix_machine"
    } else {
        "lowercase_names_rebuild"
    };
    let runtime_perfect_hash_machine_available = runtime_perfect_hash_machine
        .runtime_perfect_hash_machine_available
        || perfect_hash_adoption.runtime_perfect_hash_machine_available;
    let perfect_hash_machine_fallback_reason =
        if perfect_hash_adoption.runtime_perfect_hash_machine_adopted {
            None
        } else if runtime_perfect_hash_machine.runtime_perfect_hash_machine_available {
            perfect_hash_adoption
                .perfect_hash_machine_fallback_reason
                .clone()
        } else {
            runtime_perfect_hash_machine
                .perfect_hash_machine_fallback_reason
                .clone()
                .or(perfect_hash_adoption
                    .perfect_hash_machine_fallback_reason
                    .clone())
        };
    let runtime_bloom_machine_available = runtime_bloom_machine.runtime_bloom_machine_available
        || bloom_adoption.runtime_bloom_machine_available;
    let bloom_machine_fallback_reason = if bloom_adoption.runtime_bloom_machine_adopted {
        None
    } else if runtime_bloom_machine.runtime_bloom_machine_available {
        bloom_adoption.bloom_machine_fallback_reason.clone()
    } else {
        runtime_bloom_machine
            .bloom_machine_fallback_reason
            .clone()
            .or(bloom_adoption.bloom_machine_fallback_reason.clone())
    };
    let runtime_lowercase_cache_machine_available = runtime_lowercase_cache_machine
        .runtime_lowercase_cache_machine_available
        || lowercase_cache_adoption.runtime_lowercase_cache_machine_available;
    let lowercase_cache_machine_fallback_reason =
        if lowercase_cache_adoption.runtime_lowercase_cache_machine_adopted {
            None
        } else if runtime_lowercase_cache_machine.runtime_lowercase_cache_machine_available {
            lowercase_cache_adoption
                .lowercase_cache_machine_fallback_reason
                .clone()
        } else {
            runtime_lowercase_cache_machine
                .lowercase_cache_machine_fallback_reason
                .clone()
                .or(lowercase_cache_adoption
                    .lowercase_cache_machine_fallback_reason
                    .clone())
        };
    let runtime_non_prefix_precomputed_machines_adopted = perfect_hash_adoption
        .runtime_perfect_hash_machine_adopted
        && bloom_adoption.runtime_bloom_machine_adopted
        && lowercase_cache_adoption.runtime_lowercase_cache_machine_adopted;
    let runtime_all_precomputed_machines_adopted = runtime_non_prefix_precomputed_machines_adopted
        && prefix_adoption.runtime_prefix_machine_adopted;
    let runtime_pack_body_machine_available = runtime_pack_body_machine
        .runtime_pack_body_machine_available
        || pack_body_evidence.runtime_pack_body_machine_available;
    let pack_body_machine_fallback_reason =
        if pack_body_evidence.runtime_pack_body_machine_evidence_validated {
            None
        } else if runtime_pack_body_machine.runtime_pack_body_machine_available {
            pack_body_evidence.pack_body_machine_fallback_reason.clone()
        } else {
            runtime_pack_body_machine
                .pack_body_machine_fallback_reason
                .clone()
                .or(pack_body_evidence.pack_body_machine_fallback_reason.clone())
        };
    let pack_body_machine_consumed_for_body_resolution =
        pack_body_evidence.runtime_pack_body_machine_evidence_validated;

    IconEngineStartupReceipt {
        schema: ENGINE_STARTUP_RECEIPT_SCHEMA,
        index_dir: normalize_path(&context.index_dir),
        load_mode: context.load_mode,
        metadata_bytes,
        icon_count,
        startup_total_ns: duration_nanos_u64(context.startup_started.elapsed()),
        raw_mmap_validate_load_ns: context.raw_mmap_validate_load_ns,
        owned_fallback_load_ns: context.owned_fallback_load_ns,
        metadata_bytecheck_ns,
        metadata_materialize_ns,
        from_metadata_bytes_total_ns,
        precomputed_build_total_ns: precomputed_timings.precomputed_total_build_ns,
        perfect_hash_build_ns: precomputed_timings.perfect_hash_build_ns,
        lowercase_cache_build_ns: precomputed_timings.lowercase_cache_build_ns,
        lowercase_names_build_ns: precomputed_timings.lowercase_names_build_ns,
        lowercase_names_from_machine_cache: precomputed_timings.lowercase_names_from_machine_cache,
        bloom_filters_build_ns: precomputed_timings.bloom_filters_build_ns,
        prefix_index_build_ns: precomputed_timings.prefix_index_build_ns,
        engine_cache_init_ns,
        receipt_write_ns: 0,
        raw_mmap_ok: context.raw_mmap_ok,
        raw_mmap_error: context.raw_mmap_error,
        owned_fallback_used: context.owned_fallback_used,
        owned_fallback_error: context.owned_fallback_error,
        engine_startup_receipt_only: true,
        runtime_machine_source_fingerprint_reused: true,
        runtime_precomputed_cache_adopted: runtime_all_precomputed_machines_adopted,
        runtime_perfect_hash_machine_available,
        runtime_perfect_hash_machine_adopted: perfect_hash_adoption
            .runtime_perfect_hash_machine_adopted,
        perfect_hash_machine_lookup_validated: perfect_hash_adoption
            .perfect_hash_machine_lookup_validated,
        perfect_hash_machine_fallback_reason,
        runtime_perfect_hash_source: perfect_hash_adoption.perfect_hash_source,
        runtime_perfect_hash_machine_read_mode: runtime_perfect_hash_machine
            .runtime_perfect_hash_machine_read_mode,
        runtime_bloom_machine_available,
        runtime_bloom_machine_adopted: bloom_adoption.runtime_bloom_machine_adopted,
        bloom_machine_no_false_negatives_validated: bloom_adoption
            .bloom_machine_no_false_negatives_validated,
        bloom_machine_fallback_reason,
        runtime_bloom_source: bloom_adoption.bloom_source,
        runtime_bloom_machine_read_mode: runtime_bloom_machine.runtime_bloom_machine_read_mode,
        runtime_lowercase_cache_machine_available,
        runtime_lowercase_cache_machine_adopted: lowercase_cache_adoption
            .runtime_lowercase_cache_machine_adopted,
        lowercase_cache_machine_names_validated: lowercase_cache_adoption
            .lowercase_cache_machine_names_validated,
        lowercase_cache_machine_fallback_reason,
        runtime_lowercase_cache_source: lowercase_cache_adoption.lowercase_cache_source,
        runtime_lowercase_cache_machine_read_mode: runtime_lowercase_cache_machine
            .runtime_lowercase_cache_machine_read_mode,
        runtime_catalog_machine_available,
        runtime_catalog_machine_adopted: catalog_adoption.runtime_catalog_machine_adopted,
        catalog_machine_metadata_parity_validated: catalog_adoption
            .catalog_machine_metadata_parity_validated,
        catalog_machine_fallback_reason,
        runtime_metadata_source: catalog_adoption.runtime_metadata_source,
        runtime_catalog_machine_read_mode: runtime_catalog_machine
            .runtime_catalog_machine_read_mode,
        runtime_prefix_machine_available,
        runtime_prefix_machine_adopted: prefix_adoption.runtime_prefix_machine_adopted,
        catalog_prefix_machine_consumed_at_runtime: prefix_adoption.runtime_prefix_machine_adopted,
        prefix_machine_id_to_position_validated: prefix_adoption
            .prefix_machine_id_to_position_validated,
        prefix_machine_fallback_reason,
        runtime_prefix_index_source,
        runtime_prefix_machine_read_mode: runtime_prefix_machine.runtime_prefix_machine_read_mode,
        runtime_pack_body_machine_available,
        runtime_pack_body_machine_evidence_validated: pack_body_evidence
            .runtime_pack_body_machine_evidence_validated,
        runtime_pack_body_machine_adopted: pack_body_evidence
            .runtime_pack_body_machine_evidence_validated,
        pack_body_machine_runtime_metadata_validated: pack_body_evidence
            .pack_body_machine_runtime_metadata_validated,
        pack_body_machine_fallback_reason,
        runtime_pack_body_source: pack_body_evidence.runtime_pack_body_source,
        runtime_pack_body_machine_read_mode: runtime_pack_body_machine
            .runtime_pack_body_machine_read_mode,
        pack_body_machine_pack_count: pack_body_evidence.pack_body_machine_pack_count,
        pack_body_machine_icon_count: pack_body_evidence.pack_body_machine_icon_count,
        pack_body_machine_source_total_bytes: pack_body_evidence
            .pack_body_machine_source_total_bytes,
        pack_body_machine_consumed_for_body_resolution,
        pack_body_machine_consumed_by_search: false,
        pack_body_machine_evidence_only: !pack_body_machine_consumed_for_body_resolution,
        runtime_rebuilds_non_prefix_precomputed_structures:
            !runtime_non_prefix_precomputed_machines_adopted,
        search_behavior_changed: false,
        full_icon_runtime_baseline_measured: false,
        full_icon_search_speed_claimed: false,
        faster_than_upstream_claimed: false,
        upstream_baseline_measured: false,
        same_machine_benchmark_required: true,
    }
}

fn engine_startup_receipt_json(
    receipt: &IconEngineStartupReceipt,
    receipt_path: &Path,
) -> serde_json::Value {
    let mut receipt_json = serde_json::json!({
        "schema": receipt.schema,
        "cache_name": "media-icon-engine-startup",
        "cache_kind": "engine-startup-machine-adoption-receipt",
        "measurement_scope": "local media-icon engine startup timing: raw index load, rkyv validation/materialization, optional catalog.machine metadata adoption after runtime metadata parity validation, optional perfect-hash.machine adoption after exact lookup validation, optional bloom.machine adoption after no-false-negative validation, optional lowercase-cache.machine adoption after lowercase-name validation, optional prefix.machine adoption after icon-ID to metadata-position validation, and optional pack-body.machine body-resolution adoption after typed source-fingerprint, machine-shape, and runtime metadata validation. Records whether validated perfect-hash.machine, bloom.machine, lowercase-cache.machine, and prefix.machine caches were consumed together as the full precomputed search-structure set; records pack-body.machine as body-resolution cache adoption only; excludes query latency, pack-body engine search adoption, full render proof, and upstream comparison",
        "timing_order": [
            "raw_mmap_validate_load",
            "owned_fallback_load",
            "metadata_bytecheck",
            "metadata_materialize",
            "lowercase_names_build",
            "perfect_hash_build",
            "bloom_filters_build",
            "prefix_index_build",
            "lowercase_cache_build",
            "precomputed_total_build",
            "engine_cache_init",
            "receipt_write"
        ],
        "index_dir": receipt.index_dir,
        "receipt_path": normalize_path(receipt_path),
        "load_mode": receipt.load_mode,
        "metadata_bytes": receipt.metadata_bytes,
        "icon_count": receipt.icon_count,
        "startup_total_ns": receipt.startup_total_ns,
        "raw_mmap_validate_load_ns": receipt.raw_mmap_validate_load_ns,
        "owned_fallback_load_ns": receipt.owned_fallback_load_ns,
        "metadata_bytecheck_ns": receipt.metadata_bytecheck_ns,
        "metadata_materialize_ns": receipt.metadata_materialize_ns,
        "from_metadata_bytes_total_ns": receipt.from_metadata_bytes_total_ns,
        "precomputed_build_total_ns": receipt.precomputed_build_total_ns,
        "perfect_hash_build_ns": receipt.perfect_hash_build_ns,
        "lowercase_cache_build_ns": receipt.lowercase_cache_build_ns,
        "lowercase_names_build_ns": receipt.lowercase_names_build_ns,
        "lowercase_names_from_machine_cache": receipt.lowercase_names_from_machine_cache,
        "bloom_filters_build_ns": receipt.bloom_filters_build_ns,
        "prefix_index_build_ns": receipt.prefix_index_build_ns,
        "engine_cache_init_ns": receipt.engine_cache_init_ns,
        "receipt_write_ns": receipt.receipt_write_ns,
        "raw_mmap_ok": receipt.raw_mmap_ok,
        "raw_mmap_error": receipt.raw_mmap_error,
        "owned_fallback_used": receipt.owned_fallback_used,
        "owned_fallback_error": receipt.owned_fallback_error,
        "startup_optimization_recorded": "single_lowercase_name_vector_reuse",
        "single_lowercase_name_vector_reused": true,
        "lowercase_name_reuse_consumers": [
            "perfect_hash",
            "bloom_filters",
            "prefix_index",
            "lowercase_cache"
        ],
        "precomputed_lowercase_passes": if receipt.lowercase_names_from_machine_cache { 0 } else { 1 },
        "lowercase_cache_build_includes_lowercasing": false,
        "startup_delta_claimed": false,
        "same_machine_previous_startup_baseline_measured": false,
        "runtime_still_rebuilds_precomputed_index": !receipt.runtime_precomputed_cache_adopted,
        "engine_startup_receipt_only": receipt.engine_startup_receipt_only,
        "runtime_machine_source_fingerprint_reused": receipt.runtime_machine_source_fingerprint_reused,
        "runtime_precomputed_cache_adopted": receipt.runtime_precomputed_cache_adopted,
        "runtime_catalog_machine_available": receipt.runtime_catalog_machine_available,
        "runtime_catalog_machine_adopted": receipt.runtime_catalog_machine_adopted,
        "catalog_machine_consumed_at_runtime": receipt.runtime_catalog_machine_adopted,
        "catalog_machine_metadata_parity_validated": receipt.catalog_machine_metadata_parity_validated,
        "catalog_machine_fallback_reason": receipt.catalog_machine_fallback_reason,
        "runtime_metadata_source": receipt.runtime_metadata_source,
        "runtime_catalog_machine_read_mode": receipt.runtime_catalog_machine_read_mode,
        "catalog_machine_requires_runtime_metadata_parity_check": true,
        "runtime_prefix_machine_available": receipt.runtime_prefix_machine_available,
        "runtime_prefix_machine_adopted": receipt.runtime_prefix_machine_adopted,
        "catalog_prefix_machine_consumed_at_runtime": receipt.catalog_prefix_machine_consumed_at_runtime,
        "prefix_machine_id_to_position_validated": receipt.prefix_machine_id_to_position_validated,
        "prefix_machine_fallback_reason": receipt.prefix_machine_fallback_reason,
        "runtime_prefix_index_source": receipt.runtime_prefix_index_source,
        "runtime_prefix_machine_read_mode": receipt.runtime_prefix_machine_read_mode,
        "runtime_rebuilds_non_prefix_precomputed_structures": receipt.runtime_rebuilds_non_prefix_precomputed_structures,
        "runtime_pack_body_machine_available": receipt.runtime_pack_body_machine_available,
        "runtime_pack_body_machine_evidence_validated": receipt.runtime_pack_body_machine_evidence_validated,
        "runtime_pack_body_machine_adopted": receipt.runtime_pack_body_machine_adopted,
        "pack_body_machine_consumed_for_body_resolution": receipt.pack_body_machine_consumed_for_body_resolution,
        "pack_body_machine_consumed_by_engine_search": false,
        "pack_body_machine_runtime_metadata_validated": receipt.pack_body_machine_runtime_metadata_validated,
        "pack_body_machine_fallback_reason": receipt.pack_body_machine_fallback_reason,
        "runtime_pack_body_source": receipt.runtime_pack_body_source,
        "runtime_pack_body_machine_read_mode": receipt.runtime_pack_body_machine_read_mode,
        "pack_body_machine_pack_count": receipt.pack_body_machine_pack_count,
        "pack_body_machine_icon_count": receipt.pack_body_machine_icon_count,
        "pack_body_machine_source_total_bytes": receipt.pack_body_machine_source_total_bytes,
        "pack_body_machine_requires_runtime_metadata_validation": true,
        "pack_body_machine_evidence_only": receipt.pack_body_machine_evidence_only,
        "runtime_pack_body_adoption_scope": "body-resolution only: pack-body.machine is validated against the typed machine envelope, current source fingerprint, machine shape, and runtime metadata before engine resolve_icon_body can consume it; engine search does not consume pack-body.machine in this wave",
        "pack_body_machine_fallback_behavior": "when pack-body.machine is missing, stale, unreadable, fails typed source-fingerprint validation, fails machine-shape validation, or fails runtime metadata validation, the engine leaves body resolution unavailable for JSON fallback callers and continues using validated index metadata plus rebuilt/adopted search structures; search behavior is unchanged",
        "catalog_machine_fallback_behavior": "when catalog.machine is missing, stale, unreadable, fails catalog validation, or does not match runtime rkyv metadata, the engine falls back to validated raw index metadata bytes and preserves existing search behavior",
        "fallback_behavior": "when catalog.machine is missing, stale, unreadable, fails catalog validation, or does not match runtime rkyv metadata, when perfect-hash.machine is missing, stale, unreadable, fails table validation, or fails exact lookup validation, when bloom.machine is missing, stale, unreadable, fails shape validation, or fails no-false-negative validation, when lowercase-cache.machine is missing, stale, unreadable, fails count validation, or fails lowercase-name validation, and when prefix.machine is missing, stale, unreadable, or fails icon-id to metadata-position validation, the engine falls back to validated raw index metadata bytes or rebuilds in-memory search structures from runtime metadata and preserves existing search behavior",
        "search_behavior_changed": receipt.search_behavior_changed,
        "query_latency_measured": false,
        "full_icon_runtime_baseline_measured": receipt.full_icon_runtime_baseline_measured,
        "full_icon_search_speed_claimed": receipt.full_icon_search_speed_claimed,
        "faster_than_upstream_claimed": receipt.faster_than_upstream_claimed,
        "upstream_baseline_measured": receipt.upstream_baseline_measured,
        "upstream_baseline_command": serde_json::Value::Null,
        "upstream_baseline_checkout": serde_json::Value::Null,
        "same_machine_benchmark_required": receipt.same_machine_benchmark_required,
        "test_command": serde_json::Value::Null,
        "test_command_recorded": false,
        "suggested_test_command": "cargo check --manifest-path G:\\Dx\\www\\Cargo.toml --package dx-icons --locked --no-default-features --features rayon --lib --bin build_index --bin search_cli --bin icon -j1 --color never --message-format=short",
        "generated_at_unix_ms": current_unix_ms(),
        "machine": {
            "os": std::env::consts::OS,
            "arch": std::env::consts::ARCH
        }
    });

    if let Some(object) = receipt_json.as_object_mut() {
        object.insert(
            "runtime_full_precomputed_cache_adopted".to_string(),
            serde_json::Value::Bool(receipt.runtime_precomputed_cache_adopted),
        );
        object.insert(
            "runtime_perfect_hash_machine_available".to_string(),
            serde_json::Value::Bool(receipt.runtime_perfect_hash_machine_available),
        );
        object.insert(
            "runtime_perfect_hash_machine_adopted".to_string(),
            serde_json::Value::Bool(receipt.runtime_perfect_hash_machine_adopted),
        );
        object.insert(
            "perfect_hash_machine_consumed_at_runtime".to_string(),
            serde_json::Value::Bool(receipt.runtime_perfect_hash_machine_adopted),
        );
        object.insert(
            "perfect_hash_machine_lookup_validated".to_string(),
            serde_json::Value::Bool(receipt.perfect_hash_machine_lookup_validated),
        );
        object.insert(
            "perfect_hash_machine_fallback_reason".to_string(),
            receipt
                .perfect_hash_machine_fallback_reason
                .as_ref()
                .map(|reason| serde_json::Value::String(reason.clone()))
                .unwrap_or(serde_json::Value::Null),
        );
        object.insert(
            "runtime_perfect_hash_source".to_string(),
            serde_json::Value::String(receipt.runtime_perfect_hash_source.to_string()),
        );
        object.insert(
            "runtime_perfect_hash_machine_read_mode".to_string(),
            receipt
                .runtime_perfect_hash_machine_read_mode
                .map(|mode| serde_json::Value::String(mode.to_string()))
                .unwrap_or(serde_json::Value::Null),
        );
        object.insert(
            "perfect_hash_machine_requires_exact_lookup_validation".to_string(),
            serde_json::Value::Bool(true),
        );
        object.insert(
            "runtime_rebuilds_perfect_hash".to_string(),
            serde_json::Value::Bool(!receipt.runtime_perfect_hash_machine_adopted),
        );
        object.insert(
            "perfect_hash_machine_adoption_scope".to_string(),
            serde_json::Value::String("perfect-hash.machine adoption only replaces runtime perfect-hash table construction after table validation and exact lookup validation; it does not adopt a full persisted precomputed cache.".to_string()),
        );
        object.insert(
            "runtime_rebuilds_lowercase_cache".to_string(),
            serde_json::Value::Bool(!receipt.runtime_lowercase_cache_machine_adopted),
        );
        object.insert(
            "runtime_rebuilds_full_precomputed_index".to_string(),
            serde_json::Value::Bool(!receipt.runtime_precomputed_cache_adopted),
        );
        object.insert(
            "perfect_hash_machine_fallback_behavior".to_string(),
            serde_json::Value::String("when perfect-hash.machine is missing, stale, unreadable, fails table validation, or fails exact lookup validation, the engine rebuilds the perfect hash from the runtime lowercase-name vector and preserves existing exact-match behavior".to_string()),
        );
        object.insert(
            "runtime_bloom_machine_available".to_string(),
            serde_json::Value::Bool(receipt.runtime_bloom_machine_available),
        );
        object.insert(
            "runtime_bloom_machine_adopted".to_string(),
            serde_json::Value::Bool(receipt.runtime_bloom_machine_adopted),
        );
        object.insert(
            "bloom_machine_consumed_at_runtime".to_string(),
            serde_json::Value::Bool(receipt.runtime_bloom_machine_adopted),
        );
        object.insert(
            "bloom_machine_no_false_negatives_validated".to_string(),
            serde_json::Value::Bool(receipt.bloom_machine_no_false_negatives_validated),
        );
        object.insert(
            "bloom_machine_fallback_reason".to_string(),
            receipt
                .bloom_machine_fallback_reason
                .as_ref()
                .map(|reason| serde_json::Value::String(reason.clone()))
                .unwrap_or(serde_json::Value::Null),
        );
        object.insert(
            "runtime_bloom_source".to_string(),
            serde_json::Value::String(receipt.runtime_bloom_source.to_string()),
        );
        object.insert(
            "runtime_bloom_machine_read_mode".to_string(),
            receipt
                .runtime_bloom_machine_read_mode
                .map(|mode| serde_json::Value::String(mode.to_string()))
                .unwrap_or(serde_json::Value::Null),
        );
        object.insert(
            "bloom_machine_requires_no_false_negative_validation".to_string(),
            serde_json::Value::Bool(true),
        );
        object.insert(
            "runtime_rebuilds_bloom_filters".to_string(),
            serde_json::Value::Bool(!receipt.runtime_bloom_machine_adopted),
        );
        object.insert(
            "bloom_machine_fallback_behavior".to_string(),
            serde_json::Value::String("when bloom.machine is missing, stale, unreadable, fails shape validation, or fails no-false-negative validation, the engine rebuilds bloom filters from the runtime lowercase-name vector and preserves existing search behavior".to_string()),
        );
        object.insert(
            "runtime_lowercase_cache_machine_available".to_string(),
            serde_json::Value::Bool(receipt.runtime_lowercase_cache_machine_available),
        );
        object.insert(
            "runtime_lowercase_cache_machine_adopted".to_string(),
            serde_json::Value::Bool(receipt.runtime_lowercase_cache_machine_adopted),
        );
        object.insert(
            "lowercase_cache_machine_consumed_at_runtime".to_string(),
            serde_json::Value::Bool(receipt.runtime_lowercase_cache_machine_adopted),
        );
        object.insert(
            "lowercase_cache_machine_names_validated".to_string(),
            serde_json::Value::Bool(receipt.lowercase_cache_machine_names_validated),
        );
        object.insert(
            "lowercase_cache_machine_fallback_reason".to_string(),
            receipt
                .lowercase_cache_machine_fallback_reason
                .as_ref()
                .map(|reason| serde_json::Value::String(reason.clone()))
                .unwrap_or(serde_json::Value::Null),
        );
        object.insert(
            "runtime_lowercase_cache_source".to_string(),
            serde_json::Value::String(receipt.runtime_lowercase_cache_source.to_string()),
        );
        object.insert(
            "runtime_lowercase_cache_machine_read_mode".to_string(),
            receipt
                .runtime_lowercase_cache_machine_read_mode
                .map(|mode| serde_json::Value::String(mode.to_string()))
                .unwrap_or(serde_json::Value::Null),
        );
        object.insert(
            "lowercase_cache_machine_requires_name_validation".to_string(),
            serde_json::Value::Bool(true),
        );
        object.insert(
            "runtime_rebuilds_lowercase_cache".to_string(),
            serde_json::Value::Bool(!receipt.runtime_lowercase_cache_machine_adopted),
        );
        object.insert(
            "lowercase_cache_machine_fallback_behavior".to_string(),
            serde_json::Value::String("when lowercase-cache.machine is missing, stale, unreadable, fails count validation, or fails lowercase-name validation, the engine rebuilds the lowercase cache from the runtime lowercase-name vector and preserves existing search behavior".to_string()),
        );
    }

    receipt_json
}

fn engine_startup_receipt_path(index_dir: &Path) -> Result<PathBuf> {
    let project_root = engine_startup_project_root(index_dir)?;
    Ok(project_root.join(ENGINE_STARTUP_RECEIPT_PATH))
}

fn body_resolution_receipt_path(index_dir: &Path) -> Result<PathBuf> {
    let project_root = engine_startup_project_root(index_dir)?;
    Ok(project_root.join(BODY_RESOLUTION_RECEIPT_PATH))
}

fn query_latency_receipt_path(index_dir: &Path) -> Result<PathBuf> {
    let project_root = engine_startup_project_root(index_dir)?;
    Ok(project_root.join(QUERY_LATENCY_RECEIPT_PATH))
}

fn bounded_query_latency_inputs(
    queries: &[&str],
    limit: usize,
    warmup_runs: usize,
    measured_runs: usize,
) -> Result<BoundedQueryLatencyInputs> {
    let queries = queries
        .iter()
        .map(|query| query.trim())
        .filter(|query| !query.is_empty())
        .take(QUERY_LATENCY_MAX_QUERIES)
        .map(str::to_string)
        .collect::<Vec<_>>();

    if queries.is_empty() {
        return Err(anyhow!(
            "query latency receipt requires at least one non-empty query"
        ));
    }

    Ok(BoundedQueryLatencyInputs {
        queries,
        effective_limit: limit.clamp(1, QUERY_LATENCY_MAX_LIMIT),
        effective_warmup_runs: warmup_runs.clamp(1, QUERY_LATENCY_MAX_WARMUP_RUNS),
        effective_measured_runs: measured_runs.clamp(1, QUERY_LATENCY_MAX_MEASURED_RUNS),
    })
}

fn resolve_icon_body_from_json(
    data_dir: &Path,
    pack: &str,
    name: &str,
) -> Result<ResolvedIconPackBody> {
    let pack_file = data_dir.join(format!("{pack}.json"));
    let content = fs::read_to_string(&pack_file)
        .with_context(|| format!("read icon pack JSON {}", pack_file.display()))?;
    let pack_data = serde_json::from_str::<IconPack>(&content)
        .with_context(|| format!("parse icon pack JSON {}", pack_file.display()))?;
    let pack_json = serde_json::from_str::<serde_json::Value>(&content)
        .with_context(|| format!("parse icon pack JSON defaults {}", pack_file.display()))?;
    let icon = pack_data
        .icons
        .get(name)
        .ok_or_else(|| anyhow!("Icon '{}' not found in '{}'", name, pack))?;
    let width = icon
        .width
        .or_else(|| pack_json["width"].as_f64().map(|value| value as f32))
        .or(Some(24.0));
    let height = icon
        .height
        .or_else(|| pack_json["height"].as_f64().map(|value| value as f32))
        .or(Some(24.0));

    Ok(ResolvedIconPackBody {
        body: icon.body.clone(),
        width,
        height,
    })
}

fn render_resolved_icon_svg(resolved: &ResolvedIconPackBody) -> Option<String> {
    let (Some(width), Some(height)) = (resolved.width, resolved.height) else {
        return None;
    };

    Some(format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">{}</svg>"#,
        resolved.body
    ))
}

fn body_resolution_receipt_json(
    receipt: &IconBodyResolutionReceipt,
    receipt_path: &Path,
) -> serde_json::Value {
    serde_json::json!({
        "schema": receipt.schema,
        "cache_name": "media-icon-body-resolution",
        "cache_kind": "body-resolution-machine-vs-json-receipt",
        "measurement_scope": "local one-icon body-resolution timing: loads the engine with validated pack-body cache support, times engine resolve_icon_body for one pack/name, times current JSON fallback resolution for the same pack/name, and times SVG string rendering for each available body source; excludes query latency, full render proof, source-audit proof, and upstream comparison",
        "timing_order": [
            "engine_load_with_pack_body_cache",
            "machine_body_resolution",
            "machine_svg_render",
            "json_fallback_resolution",
            "json_svg_render",
            "receipt_write"
        ],
        "index_dir": receipt.index_dir,
        "data_dir": receipt.data_dir,
        "receipt_path": normalize_path(receipt_path),
        "pack": receipt.pack,
        "name": receipt.name,
        "engine_load_with_pack_body_cache_ns": receipt.engine_load_with_pack_body_cache_ns,
        "pack_body_cache_validation_included_in_engine_load": true,
        "pack_body_cache_validation_timing_scope": "included_in_engine_load_with_pack_body_cache_ns",
        "machine_body_resolution_ns": receipt.machine_body_resolution_ns,
        "json_fallback_resolution_ns": receipt.json_fallback_resolution_ns,
        "machine_svg_render_ns": receipt.machine_svg_render_ns,
        "json_svg_render_ns": receipt.json_svg_render_ns,
        "receipt_write_ns": receipt.receipt_write_ns,
        "engine_has_pack_body_cache": receipt.engine_has_pack_body_cache,
        "machine_body_resolution_hit": receipt.machine_body_resolution_hit,
        "machine_body_resolution_has_dimensions": receipt.machine_body_resolution_has_dimensions,
        "json_fallback_ok": receipt.json_fallback_ok,
        "machine_json_svg_match": receipt.machine_json_svg_match,
        "machine_body_bytes": receipt.machine_body_bytes,
        "json_body_bytes": receipt.json_body_bytes,
        "rendered_machine_svg_bytes": receipt.rendered_machine_svg_bytes,
        "rendered_json_svg_bytes": receipt.rendered_json_svg_bytes,
        "json_fallback_error": receipt.json_fallback_error,
        "pack_body_fast_cache_hit_read_adopted": true,
        "pack_body_fast_read_validation_scope": "typed envelope + source fingerprint + machine shape + runtime metadata; no deep JSON body equality audit",
        "runtime_pack_body_machine_read_mode": serde_json::Value::Null,
        "runtime_pack_body_machine_read_mode_recorded": false,
        "json_source_authoritative": true,
        "deep_source_audit_run": false,
        "local_body_resolution_receipt_only": true,
        "single_icon_measurement_only": true,
        "search_behavior_changed": false,
        "query_latency_measured": false,
        "full_render_proof_claimed": false,
        "same_name_body_drift_without_source_audit_proven": false,
        "full_icon_runtime_baseline_measured": false,
        "full_icon_search_speed_claimed": false,
        "faster_than_upstream_claimed": false,
        "upstream_baseline_measured": false,
        "upstream_baseline_command": serde_json::Value::Null,
        "upstream_baseline_checkout": serde_json::Value::Null,
        "same_machine_benchmark_required": true,
        "fallback_behavior": "when machine body resolution misses, lacks dimensions, or the validated pack-body cache is unavailable, callers continue using the existing JSON fallback path; this receipt records that local fallback timing but does not change search or export behavior",
        "machine_body_trust_scope": "machine body resolution trusts the typed cache envelope, current source fingerprint, machine-shape validation, and runtime metadata validation; because the fast cache-hit path does not run the deep JSON body audit, same-name body drift is not proven impossible by this receipt",
        "test_command": serde_json::Value::Null,
        "test_command_recorded": false,
        "suggested_test_command": "node --test G:\\Dx\\www\\benchmarks\\media-icon-body-resolution-receipt-contract.test.ts",
        "generated_at_unix_ms": current_unix_ms(),
        "machine": {
            "os": std::env::consts::OS,
            "arch": std::env::consts::ARCH
        }
    })
}

fn query_latency_receipt_json(
    receipt: &IconQueryLatencyReceipt,
    receipt_path: &Path,
) -> serde_json::Value {
    let total_search_ns = receipt.samples.iter().fold(0_u64, |total, sample| {
        total.saturating_add(sample.search_ns)
    });
    let total_body_resolution_ns = receipt.samples.iter().fold(0_u64, |total, sample| {
        total.saturating_add(sample.body_resolution_ns)
    });
    let total_svg_render_ns = receipt.samples.iter().fold(0_u64, |total, sample| {
        total.saturating_add(sample.svg_render_ns)
    });
    let total_export_like_top_result_ns = receipt.samples.iter().fold(0_u64, |total, sample| {
        total.saturating_add(sample.export_like_top_result_ns)
    });

    serde_json::json!({
        "schema": receipt.schema,
        "cache_name": "media-icon-query-latency",
        "cache_kind": "bounded-query-latency-receipt",
        "measurement_scope": "local bounded warm-cache query latency: loads the engine once with validated pack-body cache support, performs bounded warmup searches, times bounded measured search calls, and times top-result body resolution plus SVG string rendering without disk export writes; excludes cold-start comparison, broad benchmark sweeps, full render proof, and upstream comparison",
        "timing_order": [
            "engine_load_with_pack_body_cache",
            "warmup_search",
            "measured_search",
            "top_result_body_resolution",
            "top_result_svg_render",
            "receipt_write"
        ],
        "index_dir": receipt.index_dir,
        "data_dir": receipt.data_dir,
        "receipt_path": normalize_path(receipt_path),
        "requested_query_count": receipt.requested_query_count,
        "effective_query_count": receipt.effective_query_count,
        "requested_limit": receipt.requested_limit,
        "effective_limit": receipt.effective_limit,
        "requested_warmup_runs": receipt.requested_warmup_runs,
        "effective_warmup_runs": receipt.effective_warmup_runs,
        "requested_measured_runs": receipt.requested_measured_runs,
        "effective_measured_runs": receipt.effective_measured_runs,
        "max_query_count": QUERY_LATENCY_MAX_QUERIES,
        "max_limit": QUERY_LATENCY_MAX_LIMIT,
        "max_warmup_runs": QUERY_LATENCY_MAX_WARMUP_RUNS,
        "max_measured_runs": QUERY_LATENCY_MAX_MEASURED_RUNS,
        "engine_load_with_pack_body_cache_ns": receipt.engine_load_with_pack_body_cache_ns,
        "engine_has_pack_body_cache": receipt.engine_has_pack_body_cache,
        "receipt_write_ns": receipt.receipt_write_ns,
        "sample_count": receipt.samples.len(),
        "total_search_ns": total_search_ns,
        "total_body_resolution_ns": total_body_resolution_ns,
        "total_svg_render_ns": total_svg_render_ns,
        "total_export_like_top_result_ns": total_export_like_top_result_ns,
        "samples": receipt.samples.iter().map(|sample| serde_json::json!({
            "query": sample.query,
            "run_index": sample.run_index,
            "search_ns": sample.search_ns,
            "result_count": sample.result_count,
            "top_result_pack": sample.top_result_pack,
            "top_result_name": sample.top_result_name,
            "top_result_score": sample.top_result_score,
            "top_result_match_type": sample.top_result_match_type,
            "body_resolution_ns": sample.body_resolution_ns,
            "body_resolution_hit": sample.body_resolution_hit,
            "body_resolution_source": sample.body_resolution_source,
            "export_like_source": sample.export_like_source,
            "svg_render_ns": sample.svg_render_ns,
            "svg_rendered": sample.svg_rendered,
            "rendered_svg_bytes": sample.rendered_svg_bytes,
            "export_like_top_result_ns": sample.export_like_top_result_ns
        })).collect::<Vec<_>>(),
        "pack_body_fast_cache_hit_read_adopted": true,
        "pack_body_fast_read_validation_scope": "typed envelope + source fingerprint + machine shape + runtime metadata; no deep JSON body equality audit",
        "pack_body_cache_validation_included_in_engine_load": true,
        "json_source_authoritative": true,
        "deep_source_audit_run": false,
        "disk_export_writes_performed": false,
        "bounded_query_latency_receipt_only": true,
        "warm_cache_only": true,
        "search_behavior_changed": false,
        "query_latency_measured": true,
        "full_startup_search_render_proof_claimed": false,
        "full_render_proof_claimed": false,
        "same_name_body_drift_without_source_audit_proven": false,
        "full_icon_runtime_baseline_measured": false,
        "full_icon_search_speed_claimed": false,
        "faster_than_upstream_claimed": false,
        "upstream_baseline_measured": false,
        "upstream_baseline_command": serde_json::Value::Null,
        "upstream_baseline_checkout": serde_json::Value::Null,
        "same_machine_benchmark_required": true,
        "fallback_behavior": "when the top result cannot be resolved through the validated pack-body machine with dimensions, the receipt times the existing JSON fallback for that top result; it does not change normal search, export, or desktop behavior",
        "test_command": serde_json::Value::Null,
        "test_command_recorded": false,
        "suggested_test_command": "node --test G:\\Dx\\www\\benchmarks\\media-icon-query-latency-receipt-contract.test.ts",
        "generated_at_unix_ms": current_unix_ms(),
        "machine": {
            "os": std::env::consts::OS,
            "arch": std::env::consts::ARCH
        }
    })
}

fn match_type_label(match_type: MatchType) -> &'static str {
    match match_type {
        MatchType::Exact => "exact",
        MatchType::Prefix => "prefix",
        MatchType::Fuzzy => "fuzzy",
        MatchType::Semantic => "semantic",
    }
}

fn engine_startup_project_root(index_dir: &Path) -> Result<PathBuf> {
    if index_dir.file_name().and_then(|value| value.to_str()) != Some("index") {
        return Err(anyhow!(
            "engine startup receipt expects an index output directory, got {}",
            index_dir.display()
        ));
    }

    if let Some(parent) = index_dir.parent()
        && !parent.as_os_str().is_empty()
    {
        return Ok(parent.to_path_buf());
    }

    std::env::current_dir().context("resolve current directory for engine startup receipt")
}

fn duration_nanos_u64(duration: Duration) -> u64 {
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
    fs::write(&tmp, bytes)?;
    if path.exists() {
        fs::remove_file(path)?;
    }
    fs::rename(tmp, path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::machine_pack_body::{IconPackBodyEntryV1, IconPackBodyPackV1};

    fn metadata_bytes(metadata: &[IconMetadata]) -> Vec<u8> {
        rkyv::to_bytes::<rkyv::rancor::Error>(&metadata.to_vec())
            .unwrap()
            .to_vec()
    }

    fn metadata() -> Vec<IconMetadata> {
        vec![IconMetadata {
            id: 0,
            name: "home".to_string(),
            pack: "test".to_string(),
            category: String::new(),
            tags: Vec::new(),
            popularity: 10,
        }]
    }

    fn pack_body_machine(pack: &str, name: &str, body: &str) -> Arc<IconPackBodyMachineV1> {
        Arc::new(IconPackBodyMachineV1 {
            selected_data_root: "test".to_string(),
            generated_at_unix_ms: 0,
            source_file_count: 1,
            source_total_bytes: 10,
            source_blake3: [0; 32],
            pack_count: 1,
            icon_count: 1,
            packs: vec![IconPackBodyPackV1 {
                pack: pack.to_string(),
                rel_path: "test.json".to_string(),
                source_bytes: 10,
                source_blake3: [1; 32],
                icon_count: 1,
                icons: vec![IconPackBodyEntryV1 {
                    name: name.to_string(),
                    body: body.to_string(),
                    width: Some(24.0),
                    height: Some(24.0),
                }],
            }],
        })
    }

    #[test]
    fn engine_resolves_icon_body_from_validated_pack_body_machine() {
        let metadata = metadata();
        let metadata_bytes = metadata_bytes(&metadata);
        let engine = IconSearchEngine::from_metadata_bytes_with_catalog_prefix_and_pack_body(
            &metadata_bytes,
            None,
            None,
            None,
            None,
            None,
            Some(pack_body_machine("test", "home", "<path id=\"machine\" />")),
        )
        .unwrap();

        let resolved = engine.resolve_icon_body("test", "home").unwrap();

        assert!(engine.has_pack_body_cache());
        assert_eq!(resolved.body, "<path id=\"machine\" />");
        assert_eq!(resolved.width, Some(24.0));
        assert_eq!(resolved.height, Some(24.0));
        assert_eq!(engine.search("home", 10)[0].icon.name, "home");
    }

    #[test]
    fn engine_rejects_pack_body_machine_when_runtime_metadata_mismatches() {
        let metadata = metadata();
        let metadata_bytes = metadata_bytes(&metadata);
        let engine = IconSearchEngine::from_metadata_bytes_with_catalog_prefix_and_pack_body(
            &metadata_bytes,
            None,
            None,
            None,
            None,
            None,
            Some(pack_body_machine("wrong", "home", "<path />")),
        )
        .unwrap();

        assert!(!engine.has_pack_body_cache());
        assert!(engine.resolve_icon_body("wrong", "home").is_none());
    }
}
