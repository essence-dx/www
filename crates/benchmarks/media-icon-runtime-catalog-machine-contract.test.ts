import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";

const repoRoot = join(import.meta.dirname, "..");

function read(path: string): string {
  return readFileSync(join(repoRoot, path), "utf8");
}

test("media-icon runtime adopts catalog.machine metadata only after parity validation", () => {
  const engine = read("related-crates/media-icon/src/engine.rs");
  const catalog = read("related-crates/media-icon/src/machine_catalog.rs");

  assert.match(catalog, /pub struct IconCatalogMachineRuntimeRead/);
  assert.match(catalog, /pub fn read_icon_catalog_machine_cache_for_index_output\(/);
  assert.match(catalog, /pub fn read_icon_catalog_machine_cache\(/);
  assert.match(catalog, /open_typed_machine_cache::<IconCatalogMachineV1>/);
  assert.match(catalog, /access_typed_machine_cache::<IconCatalogMachineV1>/);
  assert.match(catalog, /fn deserialize_icon_catalog_machine_archive\(/);
  assert.match(catalog, /RkyvDeserialize::deserialize/);
  assert.match(catalog, /pub fn icon_metadata_from_catalog_machine\(/);
  assert.match(catalog, /pub fn validate_catalog_machine_metadata_parity\(/);
  assert.match(catalog, /catalog\.icon_count as usize != catalog\.entries\.len\(\)/);
  assert.match(catalog, /catalog\.pack_count as usize != catalog\.packs\.len\(\)/);
  assert.match(catalog, /entry\.pack_id as usize/);
  assert.match(catalog, /seen_ids\.insert\(entry\.id\)/);
  assert.match(catalog, /catalog machine duplicate icon id/);
  assert.match(catalog, /catalog machine metadata parity mismatch/);
  assert.match(catalog, /\.dx\/icon\/machine\/v1/);
  assert.match(catalog, /catalog\.machine/);

  assert.match(engine, /IconCatalogMachineV1/);
  assert.match(engine, /read_icon_catalog_machine_cache_with_source_fingerprint/);
  assert.match(
    engine,
    /fn read_runtime_catalog_machine\(\s*context: std::result::Result<&RuntimeMachineReadContext, &RuntimeMachineReadContextError>,\s*\)/,
  );
  assert.match(engine, /struct RuntimeCatalogMachine/);
  assert.match(engine, /struct CatalogMachineMetadataAdoptionSummary/);
  assert.match(engine, /fn adopt_catalog_machine_metadata\(/);
  assert.match(engine, /icon_metadata_from_catalog_machine\(catalog_machine\)/);
  assert.match(engine, /validate_catalog_machine_metadata_parity\(&catalog_metadata, &runtime_metadata\)/);
  assert.match(engine, /runtime_catalog_machine_available/);
  assert.match(engine, /runtime_catalog_machine_adopted/);
  assert.match(engine, /catalog_machine_metadata_parity_validated/);
  assert.match(engine, /catalog_machine_fallback_reason/);
  assert.match(engine, /runtime_metadata_source/);
  assert.match(engine, /runtime_catalog_machine_read_mode/);
  assert.match(engine, /runtime_metadata_source: "catalog_machine"/);
  assert.match(engine, /runtime_metadata_source: "rkyv_metadata_index"/);
  assert.match(engine, /runtime_precomputed_cache_adopted: runtime_all_precomputed_machines_adopted/);
  assert.match(engine, /search_behavior_changed: false/);
  assert.match(engine, /faster_than_upstream_claimed: false/);
  assert.match(engine, /upstream_baseline_measured: false/);

  const receiptJsonStart = engine.indexOf("fn engine_startup_receipt_json(");
  const receiptJson = engine.slice(receiptJsonStart);
  assert.match(receiptJson, /"runtime_catalog_machine_available"/);
  assert.match(receiptJson, /"runtime_catalog_machine_adopted"/);
  assert.match(receiptJson, /"catalog_machine_metadata_parity_validated"/);
  assert.match(receiptJson, /"catalog_machine_fallback_reason"/);
  assert.match(receiptJson, /"runtime_metadata_source"/);
  assert.match(receiptJson, /"runtime_catalog_machine_read_mode"/);
  assert.match(receiptJson, /"catalog_machine_requires_runtime_metadata_parity_check": true/);
  assert.match(
    receiptJson,
    /catalog\.machine is missing, stale, unreadable, fails catalog validation, or does not match runtime rkyv metadata/,
  );

  assert.doesNotMatch(engine, /runtime_precomputed_cache_adopted:\s*true/);
  assert.doesNotMatch(engine, /search_behavior_changed:\s*true/);
  assert.doesNotMatch(engine, /faster_than_upstream_claimed:\s*true/);
});
