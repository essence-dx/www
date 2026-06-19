import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";

const repoRoot = join(import.meta.dirname, "..");

function read(path: string): string {
  return readFileSync(join(repoRoot, path), "utf8");
}

test("media-icon parser has deterministic order before pack-body runtime adoption", () => {
  const parser = read("related-crates/media-icon/src/parser.rs");
  const packBody = read("related-crates/media-icon/src/machine_pack_body.rs");
  const engine = read("related-crates/media-icon/src/engine.rs");

  assert.match(parser, /fn sorted_json_icon_files\(data_dir: &Path\) -> Result<Vec<PathBuf>>/);
  assert.match(parser, /json_files\.sort_by/);
  assert.match(parser, /fn sorted_icon_names\(pack: &IconPack\) -> Vec<&String>/);
  assert.match(parser, /let mut icon_names = pack\.icons\.keys\(\)\.collect::<Vec<_>>\(\)/);
  assert.match(parser, /icon_names\.sort\(\)/);
  assert.match(parser, /for path in sorted_json_icon_files\(data_dir\)\?/);
  assert.match(parser, /for icon_name in sorted_icon_names\(&pack\)/);
  assert.match(parser, /id: icon_id[\s\S]*icon_id \+= 1/);

  assert.match(packBody, /use crate::types::\{IconMetadata, IconPack\}/);
  assert.match(packBody, /fn build_icon_pack_body_machine_with_source\(data_dir: &Path\) -> Result<IconPackBodyMachineBuild>/);
  assert.match(packBody, /json_files\.sort_by_key\(\|entry\| entry\.file_name\(\)\)/);
  assert.match(packBody, /path\.extension\(\)\.and_then\(\|value\| value\.to_str\(\)\) != Some\("json"\)/);
  assert.match(packBody, /\.icons[\s\S]*\.into_iter\(\)[\s\S]*IconPackBodyEntryV1/);
  assert.match(packBody, /icons\.sort_by\(\|left, right\| left\.name\.cmp\(&right\.name\)\)/);
  assert.match(packBody, /packs\.sort_by\(\|left, right\| left\.rel_path\.cmp\(&right\.rel_path\)\)/);
  assert.match(packBody, /pub fn validate_icon_pack_body_parser_metadata\(/);
  assert.match(packBody, /validate_icon_pack_body_machine\(machine\)\?/);
  assert.match(packBody, /metadata\.len\(\) != machine\.icon_count as usize/);
  assert.match(packBody, /let mut expected_id = 0u32/);
  assert.match(packBody, /for pack in &machine\.packs[\s\S]*for icon in &pack\.icons/);
  assert.match(packBody, /icon_metadata\.id != expected_id/);
  assert.match(packBody, /icon_metadata\.pack != pack\.pack/);
  assert.match(packBody, /icon_metadata\.name != icon\.name/);
  assert.match(packBody, /expected_id = expected_id\.saturating_add\(1\)/);
  assert.match(packBody, /validate_icon_pack_body_source_files/);
  assert.match(packBody, /source_icon\.body != icon\.body/);
  assert.match(packBody, /source_icon\.width != icon\.width \|\| source_icon\.height != icon\.height/);
  assert.match(packBody, /PACK_BODY_MACHINE_CACHE_ONLY: bool = true/);
  assert.match(packBody, /PACK_BODY_MACHINE_PARSER_ADOPTION_DEFERRED: bool = true/);
  assert.match(packBody, /PACK_BODY_MACHINE_PARSER_PARITY_GATE: bool = true/);
  assert.match(packBody, /PACK_BODY_MACHINE_RUNTIME_ADOPTED: bool = false/);
  assert.match(packBody, /PACK_BODY_MACHINE_RUNTIME_ADOPTION_DEFERRED: bool = true/);
  assert.match(packBody, /PACK_BODY_MACHINE_SAME_MACHINE_BENCHMARK_REQUIRED: bool = true/);

  assert.doesNotMatch(parser, /IconPackBodyMachineV1/);
  assert.doesNotMatch(parser, /read_icon_pack_body_machine_cache(?:_summary)?/);
  assert.match(engine, /IconPackBodyMachineV1/);
  assert.match(engine, /read_icon_pack_body_machine_cache_for_index_output/);
  assert.match(engine, /validate_icon_pack_body_runtime_metadata/);
  assert.match(engine, /pack_body_machine_consumed_by_engine_search": false/);
  assert.match(engine, /receipt evidence only/);
  assert.doesNotMatch(engine, /resolve_icon_pack_body/);
  assert.doesNotMatch(engine, /runtime_pack_body_machine_adopted:\s*true/);
  assert.match(engine, /search_behavior_changed: false/);
  assert.match(engine, /full_icon_search_speed_claimed: false/);
  assert.match(engine, /faster_than_upstream_claimed: false/);
  assert.match(engine, /upstream_baseline_measured: false/);
});
