import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";

const repoRoot = join(import.meta.dirname, "..");

function read(path: string): string {
  return readFileSync(join(repoRoot, path), "utf8");
}

test("media-icon runtime adopts lowercase-cache.machine only after lowercase-name validation", () => {
  const builder = read("related-crates/media-icon/src/builder.rs");
  const machine = read("related-crates/media-icon/src/machine_precomputed.rs");
  const perfectHash = read("related-crates/media-icon/src/perfect_hash.rs");
  const precomputed = read("related-crates/media-icon/src/precomputed.rs");
  const engine = read("related-crates/media-icon/src/engine.rs");

  assert.match(builder, /write_icon_lowercase_cache_machine_cache_for_index_output/);
  assert.match(
    builder,
    /write_icon_lowercase_cache_machine_cache_for_index_output\(data_dir, output_dir, &icons\)/,
  );

  assert.match(machine, /const ICON_LOWERCASE_CACHE_SCHEMA: &str = "dx\.icon\.lowercase_cache\.v1"/);
  assert.match(machine, /pub struct IconLowercaseCacheMachineV1/);
  assert.match(machine, /pub struct IconLowercaseCacheEntryV1/);
  assert.match(machine, /pub struct IconLowercaseCacheMachineRuntimeRead/);
  assert.match(machine, /pub fn write_icon_lowercase_cache_machine_cache_for_index_output\(/);
  assert.match(machine, /pub fn read_icon_lowercase_cache_machine_cache_for_index_output\(/);
  assert.match(machine, /pub fn lowercase_cache_from_machine\(/);
  assert.match(machine, /pub fn validate_lowercase_cache_machine_for_metadata\(/);
  assert.match(machine, /open_typed_machine_cache::<IconLowercaseCacheMachineV1>/);
  assert.match(machine, /access_typed_machine_cache::<IconLowercaseCacheMachineV1>/);
  assert.match(machine, /RkyvDeserialize::deserialize/);
  assert.match(machine, /ValidatedLowercaseCache::from_validated_cache\(/);
  assert.match(machine, /LowercaseCache::from_lowercase_names\(/);
  assert.match(machine, /machine\.icon_count as usize != metadata\.len\(\)/);
  assert.match(machine, /machine\.entries\.len\(\) != metadata\.len\(\)/);
  assert.match(machine, /entry\.icon_index as usize != position/);
  assert.match(machine, /entry\.lowercase_name != icon\.name\.to_lowercase\(\)/);
  assert.match(machine, /icon_machine_paths\(/);
  assert.match(machine, /lowercase-cache\.machine/);
  assert.match(machine, /lowercase-cache\.machine\.meta\.json/);

  assert.match(perfectHash, /pub fn from_lowercase_names\(lowercase_names: Vec<String>\)/);
  assert.match(perfectHash, /pub fn len\(&self\) -> usize/);
  assert.match(perfectHash, /pub fn as_slice\(&self\) -> &\[String\]/);
  assert.match(perfectHash, /pub struct ValidatedLowercaseCache/);
  assert.match(perfectHash, /pub\(crate\) fn from_validated_cache\(inner: LowercaseCache\) -> Self/);

  assert.match(precomputed, /pub lowercase_names_from_machine_cache: bool/);
  assert.match(precomputed, /Option<ValidatedLowercaseCache>/);
  assert.match(precomputed, /enum LowercaseNamesSource<'a>/);
  assert.match(precomputed, /fn from_machine_cache\(&self\) -> bool/);
  assert.match(precomputed, /pub struct LowercaseCacheMachineAdoptionSummary/);
  assert.match(precomputed, /pub runtime_lowercase_cache_machine_available: bool/);
  assert.match(precomputed, /pub runtime_lowercase_cache_machine_adopted: bool/);
  assert.match(precomputed, /pub lowercase_cache_machine_names_validated: bool/);
  assert.match(precomputed, /pub lowercase_cache_machine_fallback_reason: Option<String>/);
  assert.match(precomputed, /lowercase_cache_source: &'static str/);
  assert.match(precomputed, /lowercase_cache_source: "lowercase_cache_machine"/);
  assert.match(precomputed, /lowercase_cache_source: "lowercase_names_rebuild"/);
  assert.match(precomputed, /LowercaseCacheMachineAdoptionSummary::adopted/);
  assert.match(precomputed, /LowercaseCache::from_lowercase_names\(lowercase_names\)/);
  assert.match(precomputed, /Some\(lowercase_cache\) if lowercase_cache\.len\(\) == metadata\.len\(\) => \{[\s\S]*adopted_lowercase_cache = Some\(lowercase_cache\);[\s\S]*LowercaseCacheMachineAdoptionSummary::adopted\(\)/);
  assert.match(precomputed, /Some\(lowercase_cache\) => LowercaseNamesSource::Borrowed\(lowercase_cache\.as_slice\(\)\)/);
  assert.match(precomputed, /let lowercase_names_from_machine_cache = lowercase_names\.from_machine_cache\(\);/);
  assert.match(precomputed, /PerfectHashIndex::build_from_lowercase_names\(lowercase_names\.as_slice\(\)\)/);
  assert.match(precomputed, /IconBloomFilters::build\(lowercase_names\.as_slice\(\)\)/);
  assert.match(precomputed, /PrefixIndex::build\(lowercase_names\.as_slice\(\)\)/);
  assert.doesNotMatch(precomputed, /PerfectHashIndex::build_from_lowercase_names\(&lowercase_names\)/);
  assert.doesNotMatch(precomputed, /IconBloomFilters::build\(&lowercase_names\)/);

  assert.match(engine, /IconLowercaseCacheMachineV1/);
  assert.match(engine, /read_icon_lowercase_cache_machine_cache_with_source_fingerprint/);
  assert.match(
    engine,
    /fn read_runtime_lowercase_cache_machine\(\s*context: std::result::Result<&RuntimeMachineReadContext, &RuntimeMachineReadContextError>,\s*\)/,
  );
  assert.match(engine, /struct RuntimeLowercaseCacheMachine/);
  assert.match(engine, /fn adopt_runtime_lowercase_cache_machine\(/);
  assert.match(engine, /lowercase_cache_from_machine\(machine, metadata\)/);
  assert.match(engine, /runtime_lowercase_cache_machine_available/);
  assert.match(engine, /runtime_lowercase_cache_machine_adopted/);
  assert.match(engine, /lowercase_cache_machine_names_validated/);
  assert.match(engine, /lowercase_cache_machine_fallback_reason/);
  assert.match(engine, /runtime_lowercase_cache_machine_read_mode/);
  assert.match(engine, /runtime_lowercase_cache_source/);
  assert.match(engine, /runtime_rebuilds_lowercase_cache/);
  assert.match(engine, /runtime_precomputed_cache_adopted: runtime_all_precomputed_machines_adopted/);
  assert.match(engine, /search_behavior_changed: false/);
  assert.match(engine, /faster_than_upstream_claimed: false/);
  assert.match(engine, /upstream_baseline_measured: false/);

  const receiptJsonStart = engine.indexOf("fn engine_startup_receipt_json(");
  const receiptJson = engine.slice(receiptJsonStart);
  assert.match(receiptJson, /"runtime_lowercase_cache_machine_available"/);
  assert.match(receiptJson, /"runtime_lowercase_cache_machine_adopted"/);
  assert.match(receiptJson, /"lowercase_cache_machine_names_validated"/);
  assert.match(receiptJson, /"lowercase_cache_machine_fallback_reason"/);
  assert.match(receiptJson, /"runtime_lowercase_cache_machine_read_mode"/);
  assert.match(receiptJson, /"runtime_lowercase_cache_source"/);
  assert.match(receiptJson, /"lowercase_cache_machine_requires_name_validation"/);
  assert.match(receiptJson, /lowercase-cache\.machine is missing, stale, unreadable, fails count validation, or fails lowercase-name validation/);
  assert.match(receiptJson, /"runtime_rebuilds_lowercase_cache"/);

  assert.doesNotMatch(engine, /runtime_precomputed_cache_adopted:\s*false/);
  assert.doesNotMatch(receiptJson, /"runtime_full_precomputed_cache_adopted"\.to_string\(\),\s*serde_json::Value::Bool\(false\)/);
  assert.doesNotMatch(receiptJson, /"runtime_rebuilds_full_precomputed_index"\.to_string\(\),\s*serde_json::Value::Bool\(true\)/);
  assert.doesNotMatch(receiptJson, /"runtime_still_rebuilds_precomputed_index": true/);
  assert.doesNotMatch(engine, /search_behavior_changed:\s*true/);
  assert.doesNotMatch(engine, /faster_than_upstream_claimed:\s*true/);
});
