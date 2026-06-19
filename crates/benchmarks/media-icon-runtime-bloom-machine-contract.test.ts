import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";

const repoRoot = join(import.meta.dirname, "..");

function read(path: string): string {
  return readFileSync(join(repoRoot, path), "utf8");
}

test("media-icon runtime adopts bloom.machine only after no-false-negative validation", () => {
  const lib = read("related-crates/media-icon/src/lib.rs");
  const builder = read("related-crates/media-icon/src/builder.rs");
  const bloom = read("related-crates/media-icon/src/bloom.rs");
  const machine = read("related-crates/media-icon/src/machine_precomputed.rs");
  const precomputed = read("related-crates/media-icon/src/precomputed.rs");
  const engine = read("related-crates/media-icon/src/engine.rs");

  assert.match(lib, /pub mod machine_precomputed;/);
  assert.match(builder, /write_icon_bloom_machine_cache_for_index_output/);
  assert.match(
    builder,
    /write_icon_bloom_machine_cache_for_index_output\(data_dir, output_dir, &icons\)/,
  );

  assert.match(bloom, /pub fn to_machine_parts\(&self\)/);
  assert.match(bloom, /pub fn from_machine_parts\(/);
  assert.match(bloom, /pub fn filters_len\(&self\) -> usize/);
  assert.match(bloom, /pub fn from_filters\(/);
  assert.match(bloom, /bits\.len\(\) != expected_words/);
  assert.match(bloom, /filter size must be greater than zero/);

  assert.match(machine, /const ICON_BLOOM_CACHE_SCHEMA: &str = "dx\.icon\.bloom\.v1"/);
  assert.match(machine, /pub struct IconBloomMachineV1/);
  assert.match(machine, /pub struct IconBloomFilterMachineV1/);
  assert.match(machine, /pub struct IconBloomMachineRuntimeRead/);
  assert.match(machine, /pub fn write_icon_bloom_machine_cache_for_index_output\(/);
  assert.match(machine, /pub fn read_icon_bloom_machine_cache_for_index_output\(/);
  assert.match(machine, /pub fn bloom_filters_from_machine\(/);
  assert.match(machine, /pub fn validate_bloom_machine_for_metadata\(/);
  assert.match(machine, /open_typed_machine_cache::<IconBloomMachineV1>/);
  assert.match(machine, /access_typed_machine_cache::<IconBloomMachineV1>/);
  assert.match(machine, /RkyvDeserialize::deserialize/);
  assert.match(machine, /IconBloomFilters::from_filters\(/);
  assert.match(machine, /machine\.icon_count as usize != metadata\.len\(\)/);
  assert.match(machine, /filter\.bits\.iter\(\)\.all\(\|word\| \*word == 0\)/);
  assert.match(machine, /might_match\(position, &needle\)/);
  assert.match(machine, /bloom machine false negative/);
  assert.match(machine, /icon_machine_paths\(/);
  assert.match(machine, /bloom\.machine/);
  assert.match(machine, /bloom\.machine\.meta\.json/);

  assert.match(precomputed, /pub struct BloomMachineAdoptionSummary/);
  assert.match(precomputed, /pub runtime_bloom_machine_available: bool/);
  assert.match(precomputed, /pub runtime_bloom_machine_adopted: bool/);
  assert.match(precomputed, /pub bloom_machine_no_false_negatives_validated: bool/);
  assert.match(precomputed, /pub bloom_machine_fallback_reason: Option<String>/);
  assert.match(precomputed, /bloom_source: &'static str/);
  assert.match(precomputed, /bloom_source: "bloom_machine"/);
  assert.match(precomputed, /bloom_source: "lowercase_names_rebuild"/);
  assert.match(precomputed, /BloomMachineAdoptionSummary::adopted/);
  assert.match(precomputed, /IconBloomFilters::build\(lowercase_names\.as_slice\(\)\)/);

  assert.match(engine, /IconBloomMachineV1/);
  assert.match(engine, /read_icon_bloom_machine_cache_with_source_fingerprint/);
  assert.match(
    engine,
    /fn read_runtime_bloom_machine\(\s*context: std::result::Result<&RuntimeMachineReadContext, &RuntimeMachineReadContextError>,\s*\)/,
  );
  assert.match(engine, /struct RuntimeBloomMachine/);
  assert.match(engine, /fn adopt_runtime_bloom_machine\(/);
  assert.match(engine, /bloom_filters_from_machine\(machine, metadata\)/);
  assert.match(engine, /runtime_bloom_machine_available/);
  assert.match(engine, /runtime_bloom_machine_adopted/);
  assert.match(engine, /bloom_machine_no_false_negatives_validated/);
  assert.match(engine, /bloom_machine_fallback_reason/);
  assert.match(engine, /runtime_bloom_machine_read_mode/);
  assert.match(engine, /runtime_bloom_source/);
  assert.match(engine, /runtime_precomputed_cache_adopted: runtime_all_precomputed_machines_adopted/);
  assert.match(
    engine,
    /runtime_rebuilds_non_prefix_precomputed_structures:\s*!runtime_non_prefix_precomputed_machines_adopted/,
  );
  assert.match(engine, /search_behavior_changed: false/);
  assert.match(engine, /faster_than_upstream_claimed: false/);
  assert.match(engine, /upstream_baseline_measured: false/);

  const receiptJsonStart = engine.indexOf("fn engine_startup_receipt_json(");
  const receiptJson = engine.slice(receiptJsonStart);
  assert.match(receiptJson, /"runtime_bloom_machine_available"/);
  assert.match(receiptJson, /"runtime_bloom_machine_adopted"/);
  assert.match(receiptJson, /"bloom_machine_no_false_negatives_validated"/);
  assert.match(receiptJson, /"bloom_machine_fallback_reason"/);
  assert.match(receiptJson, /"runtime_bloom_machine_read_mode"/);
  assert.match(receiptJson, /"runtime_bloom_source"/);
  assert.match(receiptJson, /"bloom_machine_requires_no_false_negative_validation"/);
  assert.match(receiptJson, /bloom\.machine is missing, stale, unreadable, fails shape validation, or fails no-false-negative validation/);

  assert.doesNotMatch(engine, /runtime_precomputed_cache_adopted:\s*true/);
  assert.doesNotMatch(engine, /search_behavior_changed:\s*true/);
  assert.doesNotMatch(engine, /faster_than_upstream_claimed:\s*true/);
});
