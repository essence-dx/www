import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";

const repoRoot = join(import.meta.dirname, "..");

function read(path: string): string {
  return readFileSync(join(repoRoot, path), "utf8");
}

test("media-icon runtime adopts perfect-hash.machine only after exact lookup validation", () => {
  const lib = read("related-crates/media-icon/src/lib.rs");
  const builder = read("related-crates/media-icon/src/builder.rs");
  const machine = read("related-crates/media-icon/src/machine_precomputed.rs");
  const perfectHash = read("related-crates/media-icon/src/perfect_hash.rs");
  const precomputed = read("related-crates/media-icon/src/precomputed.rs");
  const engine = read("related-crates/media-icon/src/engine.rs");

  assert.match(lib, /pub mod machine_precomputed;/);
  assert.match(builder, /write_icon_perfect_hash_machine_cache_for_index_output/);
  assert.match(
    builder,
    /write_icon_perfect_hash_machine_cache_for_index_output\(data_dir, output_dir, &icons\)/,
  );

  assert.match(machine, /const ICON_PERFECT_HASH_CACHE_SCHEMA: &str = "dx\.icon\.perfect_hash\.v1"/);
  assert.match(machine, /pub struct IconPerfectHashMachineV1/);
  assert.match(machine, /pub struct IconPerfectHashSlotV1/);
  assert.match(machine, /pub struct IconPerfectHashMachineRuntimeRead/);
  assert.match(machine, /pub fn write_icon_perfect_hash_machine_cache_for_index_output\(/);
  assert.match(machine, /pub fn read_icon_perfect_hash_machine_cache_for_index_output\(/);
  assert.match(machine, /pub fn perfect_hash_index_from_machine\(/);
  assert.match(machine, /open_typed_machine_cache::<IconPerfectHashMachineV1>/);
  assert.match(machine, /access_typed_machine_cache::<IconPerfectHashMachineV1>/);
  assert.match(machine, /RkyvDeserialize::deserialize/);
  assert.match(machine, /PerfectHashIndex::from_machine_parts\(/);
  assert.match(machine, /validate_perfect_hash_machine_for_metadata\(/);
  assert.match(machine, /machine\.icon_count as usize != metadata\.len\(\)/);
  assert.match(machine, /machine\.table_size as usize != machine\.slots\.len\(\)/);
  assert.match(machine, /lookup_exact\(&icon\.name\)/);
  assert.match(machine, /perfect hash machine lookup mismatch/);
  assert.match(machine, /icon_machine_paths\(/);
  assert.match(machine, /perfect-hash\.machine/);
  assert.match(machine, /perfect-hash\.machine\.meta\.json/);

  assert.match(perfectHash, /pub fn to_machine_parts\(&self\)/);
  assert.match(perfectHash, /pub fn from_machine_parts\(/);
  assert.match(perfectHash, /hash_table\.len\(\) != table_size/);
  assert.match(perfectHash, /name_table\.len\(\) != table_size/);

  assert.match(precomputed, /pub struct PerfectHashMachineAdoptionSummary/);
  assert.match(precomputed, /pub runtime_perfect_hash_machine_available: bool/);
  assert.match(precomputed, /pub runtime_perfect_hash_machine_adopted: bool/);
  assert.match(precomputed, /pub perfect_hash_machine_lookup_validated: bool/);
  assert.match(precomputed, /pub perfect_hash_machine_fallback_reason: Option<String>/);
  assert.match(precomputed, /perfect_hash_source: &'static str/);
  assert.match(precomputed, /perfect_hash_source: "perfect_hash_machine"/);
  assert.match(precomputed, /perfect_hash_source: "lowercase_names_rebuild"/);
  assert.match(precomputed, /PerfectHashMachineAdoptionSummary::adopted/);
  assert.match(
    precomputed,
    /PerfectHashIndex::build_from_lowercase_names\(lowercase_names\.as_slice\(\)\)/,
  );

  assert.match(engine, /IconPerfectHashMachineV1/);
  assert.match(engine, /read_icon_perfect_hash_machine_cache_with_source_fingerprint/);
  assert.match(
    engine,
    /fn read_runtime_perfect_hash_machine\(\s*context: std::result::Result<&RuntimeMachineReadContext, &RuntimeMachineReadContextError>,\s*\)/,
  );
  assert.match(engine, /struct RuntimePerfectHashMachine/);
  assert.match(engine, /perfect_hash_index_from_machine\(/);
  assert.match(engine, /runtime_perfect_hash_machine_available/);
  assert.match(engine, /runtime_perfect_hash_machine_adopted/);
  assert.match(engine, /perfect_hash_machine_lookup_validated/);
  assert.match(engine, /perfect_hash_machine_fallback_reason/);
  assert.match(engine, /runtime_perfect_hash_machine_read_mode/);
  assert.match(engine, /runtime_perfect_hash_source/);
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
  assert.match(receiptJson, /"runtime_perfect_hash_machine_available"/);
  assert.match(receiptJson, /"runtime_perfect_hash_machine_adopted"/);
  assert.match(receiptJson, /"perfect_hash_machine_lookup_validated"/);
  assert.match(receiptJson, /"perfect_hash_machine_fallback_reason"/);
  assert.match(receiptJson, /"runtime_perfect_hash_machine_read_mode"/);
  assert.match(receiptJson, /"runtime_perfect_hash_source"/);
  assert.match(receiptJson, /perfect-hash\.machine is missing, stale, unreadable, fails table validation, or fails exact lookup validation/);

  assert.doesNotMatch(engine, /runtime_precomputed_cache_adopted:\s*true/);
  assert.doesNotMatch(engine, /search_behavior_changed:\s*true/);
  assert.doesNotMatch(engine, /faster_than_upstream_claimed:\s*true/);
});
