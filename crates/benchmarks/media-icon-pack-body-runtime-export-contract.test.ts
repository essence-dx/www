import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";

const repoRoot = join(import.meta.dirname, "..");

function read(path: string): string {
  return readFileSync(join(repoRoot, path), "utf8");
}

test("media-icon CLI export resolves SVG bodies from validated pack-body machine before JSON fallback", () => {
  const packBody = read("related-crates/media-icon/src/machine_pack_body.rs");
  const iconCli = read("related-crates/media-icon/src/bin/icon.rs");
  const engine = read("related-crates/media-icon/src/engine.rs");

  assert.match(packBody, /pub struct IconPackBodyMachineRuntimeRead/);
  assert.match(packBody, /pub struct ResolvedIconPackBody/);
  assert.match(packBody, /pub fn read_icon_pack_body_machine_cache_for_index_output\(/);
  assert.match(packBody, /pub fn read_icon_pack_body_machine_cache\(/);
  assert.match(packBody, /pub fn read_icon_pack_body_machine_cache_with_source_audit\(/);
  assert.match(packBody, /pub fn resolve_icon_pack_body\(/);
  assert.match(packBody, /deserialize_icon_pack_body_machine_archive\(machine\.archived\(\)\)\?/);
  assert.match(packBody, /validate_icon_pack_body_machine\(&pack_body_machine\)\?/);
  assert.match(packBody, /validate_icon_pack_body_source_files\(&read\.pack_body_machine, data_dir\)\?/);
  assert.match(packBody, /validate_icon_pack_body_parser_metadata\(machine, metadata\)\?/);
  assert.match(packBody, /PACK_BODY_MACHINE_CLI_EXPORT_ADOPTED: bool = true/);
  assert.match(packBody, /PACK_BODY_MACHINE_FAST_CACHE_HIT_READ_ADOPTED: bool = true/);
  assert.match(packBody, /PACK_BODY_MACHINE_DEEP_SOURCE_AUDIT_AVAILABLE: bool = true/);
  assert.match(packBody, /PACK_BODY_MACHINE_ENGINE_SEARCH_ADOPTED: bool = false/);

  assert.match(iconCli, /IconSearchEngine::load_fast_with_pack_body_cache\(index_dir\)/);
  assert.match(iconCli, /fn with_body_engine<F, R>\(f: F\) -> anyhow::Result<R>/);
  assert.match(iconCli, /fn generate_svg_from_engine_pack_body\(/);
  assert.match(iconCli, /fn generate_svg_from_json\(/);
  assert.match(
    iconCli,
    /if let Some\(svg\) = generate_svg_from_engine_pack_body\(name, pack\)\?[\s\S]*return Ok\(svg\);/,
  );
  assert.match(iconCli, /engine\.resolve_icon_body\(pack, name\)/);
  assert.doesNotMatch(iconCli, /static PACK_BODY_MACHINE/);
  assert.doesNotMatch(iconCli, /read_icon_pack_body_machine_cache/);
  assert.match(iconCli, /generate_svg_from_json\(name, pack\)/);

  assert.match(engine, /excludes query latency, pack-body engine search adoption, full render proof, and upstream comparison/);
  assert.match(engine, /IconPackBodyMachineV1/);
  assert.match(engine, /read_icon_pack_body_machine_cache_with_source_fingerprint/);
  assert.match(engine, /"pack_body_machine_consumed_for_body_resolution": receipt\.pack_body_machine_consumed_for_body_resolution/);
  assert.match(engine, /"pack_body_machine_consumed_by_engine_search": false/);
  assert.match(engine, /pub fn resolve_icon_body\(&self, pack: &str, name: &str\) -> Option<ResolvedIconPackBody>/);
  assert.match(engine, /resolve_icon_pack_body\(machine\.as_ref\(\), pack, name\)/);
});
