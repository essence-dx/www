import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";

const repoRoot = join(import.meta.dirname, "..");

function read(path: string): string {
  return readFileSync(join(repoRoot, path), "utf8");
}

test("media-icon pack body cache uses typed dx-serializer machine files without parser adoption", () => {
  const lib = read("related-crates/media-icon/src/lib.rs");
  const builder = read("related-crates/media-icon/src/builder.rs");
  const parser = read("related-crates/media-icon/src/parser.rs");
  const packBody = read("related-crates/media-icon/src/machine_pack_body.rs");

  assert.match(lib, /pub mod machine_pack_body;/);
  assert.match(builder, /write_icon_pack_body_machine_cache_for_index_output/);
  assert.match(
    builder,
    /write_icon_pack_body_machine_cache_for_index_output\(data_dir, output_dir\)/,
  );

  assert.match(packBody, /const ICON_PACK_BODY_CACHE_SCHEMA: &str = "dx\.icon\.pack_body\.v1"/);
  assert.match(packBody, /pub struct IconPackBodyMachineV1/);
  assert.match(packBody, /pub struct IconPackBodyPackV1/);
  assert.match(packBody, /pub struct IconPackBodyEntryV1/);
  assert.match(packBody, /pub struct IconPackBodyReadSummary/);
  assert.match(packBody, /pub fn write_icon_pack_body_machine_cache_for_index_output\(/);
  assert.match(packBody, /pub fn write_icon_pack_body_machine_cache\(/);
  assert.match(packBody, /pub fn read_icon_pack_body_machine_cache_summary\(/);
  assert.match(packBody, /pub fn validate_icon_pack_body_machine\(/);
  assert.match(packBody, /MachineCacheCodec::None/);
  assert.match(packBody, /write_typed_machine_cache/);
  assert.match(packBody, /open_typed_machine_cache::<IconPackBodyMachineV1>/);
  assert.match(packBody, /access_typed_machine_cache::<IconPackBodyMachineV1>/);
  assert.match(packBody, /RkyvDeserialize::deserialize/);
  assert.match(packBody, /serde_json::from_str::<IconPack>/);
  assert.match(packBody, /blake3::hash\(&bytes\)/);
  assert.match(packBody, /validate_icon_pack_body_source_files/);
  assert.match(packBody, /source_icon\.body != icon\.body/);
  assert.match(packBody, /source_icon\.width != icon\.width \|\| source_icon\.height != icon\.height/);
  assert.match(packBody, /machine\.pack_count as usize != machine\.packs\.len\(\)/);
  assert.match(packBody, /machine\.source_file_count as usize != machine\.packs\.len\(\)/);
  assert.match(packBody, /pack\.icon_count as usize != pack\.icons\.len\(\)/);
  assert.match(packBody, /computed_icon_count != machine\.icon_count as usize/);
  assert.match(packBody, /icon_machine_paths\(/);
  assert.match(packBody, /pack-body\.machine/);
  assert.match(packBody, /pack-body\.machine\.meta\.json/);
  assert.match(packBody, /PACK_BODY_MACHINE_JSON_SOURCE_AUTHORITATIVE: bool = true/);
  assert.match(packBody, /PACK_BODY_MACHINE_CACHE_ONLY: bool = true/);
  assert.match(packBody, /PACK_BODY_MACHINE_PARSER_ADOPTION_DEFERRED: bool = true/);
  assert.match(packBody, /PACK_BODY_MACHINE_RUNTIME_ADOPTED: bool = false/);
  assert.match(packBody, /PACK_BODY_MACHINE_FULL_ICON_SEARCH_SPEED_CLAIMED: bool = false/);
  assert.match(packBody, /PACK_BODY_MACHINE_FASTER_THAN_UPSTREAM_CLAIMED: bool = false/);
  assert.match(packBody, /PACK_BODY_MACHINE_UPSTREAM_BASELINE_MEASURED: bool = false/);
  assert.match(packBody, /PACK_BODY_MACHINE_SAME_MACHINE_BENCHMARK_REQUIRED: bool = true/);

  assert.doesNotMatch(parser, /read_icon_pack_body_machine_cache/);
  assert.doesNotMatch(parser, /IconPackBodyMachineV1/);
});

test("generated pack body machine cache paths stay local-only", () => {
  const generatedGuard = read("benchmarks/generated-artifact-ignore-contract.test.ts");
  const tempGuard = read("benchmarks/temp-cache-ignore-hygiene.test.ts");

  assert.match(generatedGuard, /\.dx\/icon\/machine\/v1\/pack-body\.machine/);
  assert.match(generatedGuard, /\.dx\/icon\/machine\/v1\/pack-body\.machine\.meta\.json/);
  assert.match(tempGuard, /\.dx\/icon\/machine\/v1\/pack-body\.machine/);
  assert.match(tempGuard, /\.dx\/icon\/machine\/v1\/pack-body\.machine\.meta\.json/);
});
