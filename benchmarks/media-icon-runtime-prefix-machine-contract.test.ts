import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";

const repoRoot = join(import.meta.dirname, "..");

function read(path: string): string {
  return readFileSync(join(repoRoot, path), "utf8");
}

test("media-icon runtime adopts prefix.machine only after validating icon IDs to metadata positions", () => {
  const engine = read("related-crates/media-icon/src/engine.rs");
  const precomputed = read("related-crates/media-icon/src/precomputed.rs");
  const catalog = read("related-crates/media-icon/src/machine_catalog.rs");

  assert.match(catalog, /pub struct IconPrefixMachineRuntimeRead/);
  assert.match(catalog, /pub fn read_icon_prefix_machine_cache_for_index_output\(/);
  assert.match(catalog, /open_typed_machine_cache::<IconPrefixMachineV1>/);
  assert.match(catalog, /access_typed_machine_cache::<IconPrefixMachineV1>/);
  assert.match(catalog, /RkyvDeserialize::deserialize/);
  assert.match(catalog, /prefix\.machine/);
  assert.match(catalog, /\.dx\/icon\/machine\/v1/);

  assert.match(precomputed, /pub struct PrefixMachineAdoptionSummary/);
  assert.match(precomputed, /pub runtime_prefix_machine_available: bool/);
  assert.match(precomputed, /pub runtime_prefix_machine_adopted: bool/);
  assert.match(precomputed, /pub prefix_machine_id_to_position_validated: bool/);
  assert.match(precomputed, /pub prefix_machine_fallback_reason: Option<String>/);
  assert.match(
    precomputed,
    /pub fn build_with_optional_prefix_machine\(\s*metadata: Vec<IconMetadata>,\s*prefix_machine: Option<&IconPrefixMachineV1>,\s*\) -> \(Self, PrefixMachineAdoptionSummary\)/,
  );
  assert.match(
    precomputed,
    /pub fn from_icon_id_prefix_machine\(\s*prefix_machine: &IconPrefixMachineV1,\s*metadata: &\[IconMetadata\],\s*\) -> Result<Self, PrefixMachineAdoptionError>/,
  );
  assert.match(precomputed, /let mut id_to_position: HashMap<u32, u32>/);
  assert.match(precomputed, /id_to_position\.insert\(icon\.id, position as u32\)/);
  assert.match(precomputed, /\.get\(&icon_id\)/);
  assert.match(precomputed, /checked_add\(entry\.len\)/);
  assert.match(precomputed, /prefix_machine\.icon_count as usize != metadata\.len\(\)/);
  assert.match(precomputed, /build_prefix_map\(lowercase_names\)/);
  assert.match(precomputed, /if prefix_map != expected_prefix_map/);
  assert.match(precomputed, /PrefixMapMismatch/);
  assert.match(precomputed, /NonCanonicalPrefixRange/);
  assert.match(precomputed, /PrefixNotLowercase/);
  assert.match(precomputed, /runtime_prefix_machine_adopted: true/);
  assert.match(precomputed, /prefix_machine_id_to_position_validated: true/);
  assert.match(precomputed, /prefix_machine_fallback_reason: Some/);

  const fromMachineStart = precomputed.indexOf("pub fn from_icon_id_prefix_machine(");
  const fromMachineEnd = precomputed.indexOf("/// Get candidates for prefix", fromMachineStart);
  const fromMachineSection = precomputed.slice(fromMachineStart, fromMachineEnd);

  assert.doesNotMatch(fromMachineSection, /\.push\(icon_id\)/);
  assert.doesNotMatch(fromMachineSection, /entry\.start \+ entry\.len/);
  assert.doesNotMatch(fromMachineSection, /unwrap_or(?:_default)?\(/);

  assert.match(engine, /read_icon_prefix_machine_cache_with_source_fingerprint/);
  assert.match(engine, /use crate::machine_manifest::select_icon_data_dir/);
  assert.match(
    engine,
    /fn read_runtime_prefix_machine\(\s*context: std::result::Result<&RuntimeMachineReadContext, &RuntimeMachineReadContextError>,\s*\)/,
  );
  assert.match(
    engine,
    /PrecomputedIndex::build_with_timings_and_optional_precomputed_machines\(\s*metadata,\s*runtime_prefix_machine\.prefix_machine\.as_ref\(\),/,
  );
  assert.match(engine, /runtime_prefix_machine_available/);
  assert.match(engine, /runtime_prefix_machine_adopted/);
  assert.match(engine, /prefix_machine_id_to_position_validated/);
  assert.match(engine, /prefix_machine_fallback_reason/);
  assert.match(engine, /catalog_prefix_machine_consumed_at_runtime: prefix_adoption\s*\.runtime_prefix_machine_adopted/);
  assert.match(engine, /search_behavior_changed: false/);
  assert.doesNotMatch(engine, /catalog_prefix_machine_consumed_at_runtime:\s*true/);
  assert.doesNotMatch(engine, /prefix_machine_id_to_position_validated:\s*true/);
  assert.doesNotMatch(engine, /search_behavior_changed:\s*true/);
});
