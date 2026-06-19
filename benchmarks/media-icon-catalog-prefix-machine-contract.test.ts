import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";

const repoRoot = join(import.meta.dirname, "..");

function read(path: string): string {
  return readFileSync(join(repoRoot, path), "utf8");
}

test("media-icon catalog and prefix caches use typed dx-serializer machine files", () => {
  const lib = read("related-crates/media-icon/src/lib.rs");
  const builder = read("related-crates/media-icon/src/builder.rs");
  const catalog = read("related-crates/media-icon/src/machine_catalog.rs");

  assert.match(lib, /pub mod machine_catalog;/);
  assert.match(builder, /write_icon_catalog_prefix_machine_caches_for_index_output/);
  assert.match(builder, /let _ =\s*write_icon_catalog_prefix_machine_caches_for_index_output\(data_dir, output_dir, &icons\);/);

  assert.match(catalog, /const ICON_CATALOG_CACHE_SCHEMA: &str = "dx\.icon\.catalog\.v1"/);
  assert.match(catalog, /const ICON_PREFIX_CACHE_SCHEMA: &str = "dx\.icon\.prefix\.v1"/);
  assert.match(catalog, /pub struct IconCatalogMachineV1/);
  assert.match(catalog, /pub struct IconCatalogPackV1/);
  assert.match(catalog, /pub struct IconCatalogEntryV1/);
  assert.match(catalog, /pub struct IconPrefixMachineV1/);
  assert.match(catalog, /pub struct IconPrefixEntryV1/);
  assert.match(catalog, /source_file_count: u32/);
  assert.match(catalog, /source_total_bytes: u64/);
  assert.match(catalog, /source_blake3: \[u8; 32\]/);
  assert.match(catalog, /pub pack_id: u32/);
  assert.match(catalog, /pub icon_ids: Vec<u32>/);
  assert.match(catalog, /MachineCacheCodec::None/);
  assert.match(catalog, /write_typed_machine_cache/);
  assert.match(catalog, /open_typed_machine_cache::<IconCatalogMachineV1>/);
  assert.match(catalog, /open_typed_machine_cache::<IconPrefixMachineV1>/);
  assert.match(catalog, /access_typed_machine_cache::<IconCatalogMachineV1>/);
  assert.match(catalog, /access_typed_machine_cache::<IconPrefixMachineV1>/);
  assert.match(catalog, /\.dx\/icon\/machine\/v1/);
  assert.match(catalog, /catalog\.machine/);
  assert.match(catalog, /catalog\.machine\.meta\.json/);
  assert.match(catalog, /prefix\.machine/);
  assert.match(catalog, /prefix\.machine\.meta\.json/);
  assert.match(catalog, /fn icon_catalog_source_fingerprint/);
  assert.match(catalog, /blake3::hash\(&bytes\)/);
  assert.match(catalog, /BTreeMap<String, Vec<u32>>/);
  assert.match(catalog, /for len in 1\.\.=3\.min\(name\.len\(\)\)/);
  assert.match(catalog, /json_source_authoritative/);
  assert.match(catalog, /catalog_prefix_cache_only/);
  assert.match(catalog, /faster_than_upstream_claimed/);
  assert.match(catalog, /upstream_baseline_measured/);
  assert.match(catalog, /full_icon_runtime_baseline_measured/);
  assert.match(catalog, /same_machine_benchmark_required/);
});

test("generated catalog and prefix machine cache paths stay local-only", () => {
  const gitignore = read(".gitignore");
  const generatedGuard = read("benchmarks/generated-artifact-ignore-contract.test.ts");
  const tempGuard = read("benchmarks/temp-cache-ignore-hygiene.test.ts");

  assert.match(gitignore, /^\.dx\/icon\/$/m);
  assert.match(generatedGuard, /\.dx\/icon\/machine\/v1\/catalog\.machine/);
  assert.match(generatedGuard, /\.dx\/icon\/machine\/v1\/catalog\.machine\.meta\.json/);
  assert.match(generatedGuard, /\.dx\/icon\/machine\/v1\/prefix\.machine/);
  assert.match(generatedGuard, /\.dx\/icon\/machine\/v1\/prefix\.machine\.meta\.json/);
  assert.match(tempGuard, /\.dx\/icon\/machine\/v1\/catalog\.machine/);
  assert.match(tempGuard, /\.dx\/icon\/machine\/v1\/catalog\.machine\.meta\.json/);
  assert.match(tempGuard, /\.dx\/icon\/machine\/v1\/prefix\.machine/);
  assert.match(tempGuard, /\.dx\/icon\/machine\/v1\/prefix\.machine\.meta\.json/);
});
