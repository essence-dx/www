import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";

const repoRoot = join(import.meta.dirname, "..");

function read(path: string): string {
  return readFileSync(join(repoRoot, path), "utf8");
}

test("media-icon engine exposes startup timing receipts without changing normal load behavior", () => {
  const engine = read("related-crates/media-icon/src/engine.rs");
  const precomputed = read("related-crates/media-icon/src/precomputed.rs");

  assert.match(engine, /const ENGINE_STARTUP_RECEIPT_SCHEMA: &str =\s*"dx\.performance\.json_machine_cache_receipt\.media_icon_engine_startup\.v1"/);
  assert.match(engine, /const ENGINE_STARTUP_RECEIPT_PATH: &str =\s*"\.dx\/performance\/json-machine-cache-receipts\/media-icon-engine-startup\.json"/);
  assert.match(engine, /pub struct IconEngineStartupReceipt/);
  assert.match(engine, /pub fn load_fast_with_startup_receipt\(\s*index_dir: &Path,\s*\) -> Result<\(Self, IconEngineStartupReceipt\)>/);
  assert.match(engine, /pub fn write_startup_performance_receipt\(index_dir: &Path\) -> Result<PathBuf>/);
  assert.match(engine, /fn from_metadata_bytes_with_startup_receipt\(/);
  assert.match(engine, /fn from_metadata_bytes_with_startup_context\(/);
  assert.match(engine, /PrecomputedIndex::build_with_timings_and_optional_precomputed_machines\(\s*metadata,\s*runtime_prefix_machine\.prefix_machine\.as_ref\(\),\s*perfect_hash_index,\s*bloom_filters,\s*lowercase_cache,\s*\)/);

  assert.match(precomputed, /pub struct PrecomputedBuildTimings/);
  assert.match(precomputed, /pub perfect_hash_build_ns: u64/);
  assert.match(precomputed, /pub lowercase_cache_build_ns: u64/);
  assert.match(precomputed, /pub lowercase_names_build_ns: u64/);
  assert.match(precomputed, /pub lowercase_names_from_machine_cache: bool/);
  assert.match(precomputed, /pub bloom_filters_build_ns: u64/);
  assert.match(precomputed, /pub prefix_index_build_ns: u64/);
  assert.match(precomputed, /pub precomputed_total_build_ns: u64/);
  assert.match(precomputed, /pub fn build_with_timings\(metadata: Vec<IconMetadata>\) -> \(Self, PrecomputedBuildTimings\)/);
  assert.match(precomputed, /Instant::now\(\)/);

  assert.match(engine, /"metadata_bytecheck"/);
  assert.match(engine, /"metadata_materialize"/);
  assert.match(engine, /"perfect_hash_build"/);
  assert.match(engine, /"lowercase_cache_build"/);
  assert.match(engine, /"lowercase_names_build"/);
  assert.match(engine, /"bloom_filters_build"/);
  assert.match(engine, /"prefix_index_build"/);
  assert.match(engine, /"precomputed_total_build"/);
  assert.match(engine, /"startup_optimization_recorded": "single_lowercase_name_vector_reuse"/);
  assert.match(engine, /"single_lowercase_name_vector_reused": true/);
  assert.match(engine, /"lowercase_name_reuse_consumers": \[/);
  assert.match(engine, /"lowercase_names_from_machine_cache": receipt\.lowercase_names_from_machine_cache/);
  assert.match(engine, /"precomputed_lowercase_passes": if receipt\.lowercase_names_from_machine_cache \{ 0 \} else \{ 1 \}/);
  assert.match(engine, /"lowercase_cache_build_includes_lowercasing": false/);
  assert.match(engine, /"startup_delta_claimed": false/);
  assert.match(engine, /"same_machine_previous_startup_baseline_measured": false/);
  assert.match(engine, /engine_startup_receipt_only: true/);
  assert.match(engine, /runtime_all_precomputed_machines_adopted/);
  assert.match(engine, /runtime_precomputed_cache_adopted: runtime_all_precomputed_machines_adopted/);
  assert.match(engine, /runtime_catalog_machine_available/);
  assert.match(engine, /runtime_catalog_machine_adopted/);
  assert.match(engine, /catalog_machine_metadata_parity_validated/);
  assert.match(engine, /catalog_machine_fallback_reason/);
  assert.match(engine, /runtime_metadata_source/);
  assert.match(engine, /runtime_catalog_machine_read_mode/);
  assert.match(engine, /"catalog_machine_requires_runtime_metadata_parity_check": true/);
  assert.match(engine, /runtime_prefix_machine_available/);
  assert.match(engine, /runtime_prefix_machine_adopted/);
  assert.match(engine, /catalog_prefix_machine_consumed_at_runtime: prefix_adoption\.runtime_prefix_machine_adopted/);
  assert.match(engine, /prefix_machine_id_to_position_validated/);
  assert.match(engine, /runtime_prefix_index_source/);
  assert.match(engine, /runtime_rebuilds_non_prefix_precomputed_structures:\s*!runtime_non_prefix_precomputed_machines_adopted/);
  assert.match(engine, /runtime_lowercase_cache_machine_available/);
  assert.match(engine, /runtime_lowercase_cache_machine_adopted/);
  assert.match(engine, /lowercase_cache_machine_names_validated/);
  assert.match(engine, /runtime_lowercase_cache_source/);
  assert.match(engine, /"lowercase_cache_machine_requires_name_validation"/);
  assert.match(engine, /"catalog_machine_fallback_behavior": "when catalog\.machine is missing, stale, unreadable, fails catalog validation, or does not match runtime rkyv metadata, the engine falls back to validated raw index metadata bytes and preserves existing search behavior"/);
  assert.match(engine, /"fallback_behavior": "when catalog\.machine is missing, stale, unreadable, fails catalog validation, or does not match runtime rkyv metadata, when perfect-hash\.machine is missing, stale, unreadable, fails table validation, or fails exact lookup validation, when bloom\.machine is missing, stale, unreadable, fails shape validation, or fails no-false-negative validation, when lowercase-cache\.machine is missing, stale, unreadable, fails count validation, or fails lowercase-name validation, and when prefix\.machine is missing, stale, unreadable, or fails icon-id to metadata-position validation, the engine falls back to validated raw index metadata bytes or rebuilds in-memory search structures from runtime metadata and preserves existing search behavior"/);
  assert.match(engine, /search_behavior_changed: false/);
  assert.match(engine, /full_icon_runtime_baseline_measured: false/);
  assert.match(engine, /full_icon_search_speed_claimed: false/);
  assert.match(engine, /faster_than_upstream_claimed: false/);
  assert.match(engine, /upstream_baseline_measured: false/);
  assert.match(engine, /same_machine_benchmark_required: true/);

  const loadFastStart = engine.indexOf("pub fn load_fast(index_dir: &Path) -> Result<Self>");
  const loadFastEnd = engine.indexOf("/// Load a search engine and return a local startup timing receipt.", loadFastStart);
  const loadFastSection = engine.slice(loadFastStart, loadFastEnd);

  assert.match(loadFastSection, /Self::from_mapped_index_with_catalog_and_prefix\(\s*&index,\s*catalog_machine,\s*prefix_machine,\s*perfect_hash_machine,\s*bloom_machine,\s*lowercase_cache_machine,\s*\)/);
  assert.match(loadFastSection, /Self::from_index_with_catalog_and_prefix\(\s*index,\s*catalog_machine,\s*prefix_machine,\s*perfect_hash_machine,\s*bloom_machine,\s*lowercase_cache_machine,\s*\)/);
  assert.doesNotMatch(loadFastSection, /write_startup_performance_receipt/);
});
