import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";

const repoRoot = join(import.meta.dirname, "..");

function read(path: string): string {
  return readFileSync(join(repoRoot, path), "utf8");
}

function extractFunction(source: string, signature: string): string {
  const start = source.indexOf(signature);
  assert.notEqual(start, -1, `missing function signature: ${signature}`);

  const bodyStart = source.indexOf("{", start);
  assert.notEqual(bodyStart, -1, `missing function body: ${signature}`);

  let depth = 0;
  for (let index = bodyStart; index < source.length; index += 1) {
    const char = source[index];
    if (char === "{") depth += 1;
    if (char === "}") depth -= 1;
    if (depth === 0) {
      return source.slice(start, index + 1);
    }
  }

  assert.fail(`unterminated function body: ${signature}`);
}

test("media-icon writes local body-resolution measurement receipt without upstream claims", () => {
  const engine = read("related-crates/media-icon/src/engine.rs");
  const iconCli = read("related-crates/media-icon/src/bin/icon.rs");

  assert.match(
    engine,
    /const BODY_RESOLUTION_RECEIPT_SCHEMA: &str =\s*"dx\.performance\.json_machine_cache_receipt\.media_icon_body_resolution\.v1"/,
  );
  assert.match(
    engine,
    /const BODY_RESOLUTION_RECEIPT_PATH: &str =\s*"\.dx\/performance\/json-machine-cache-receipts\/media-icon-body-resolution\.json"/,
  );
  assert.match(engine, /pub struct IconBodyResolutionReceipt/);
  assert.match(
    engine,
    /pub fn write_body_resolution_performance_receipt\(\s*index_dir: &Path,\s*data_dir: &Path,\s*pack: &str,\s*name: &str,\s*\) -> Result<PathBuf>/,
  );

  const writer = extractFunction(
    engine,
    "pub fn write_body_resolution_performance_receipt(",
  );
  assert.match(writer, /Self::load_fast_with_pack_body_cache\(index_dir\)\?/);
  assert.match(writer, /engine\.resolve_icon_body\(pack, name\)/);
  assert.match(writer, /resolve_icon_body_from_json\(data_dir, pack, name\)/);
  assert.match(writer, /body_resolution_receipt_json\(&receipt, &receipt_path\)/);
  assert.match(writer, /write_atomic\(&receipt_path/);
  assert.doesNotMatch(writer, /load_fast\(index_dir\)/);
  assert.doesNotMatch(writer, /search\(pack|search\(name|engine\.search/);

  assert.match(engine, /fn resolve_icon_body_from_json\(/);
  assert.match(engine, /serde_json::from_str::<IconPack>/);
  assert.match(engine, /fn render_resolved_icon_svg\(/);
  assert.match(engine, /fn body_resolution_receipt_json\(/);
  assert.match(engine, /"cache_name": "media-icon-body-resolution"/);
  assert.match(engine, /"cache_kind": "body-resolution-machine-vs-json-receipt"/);
  assert.match(engine, /"measurement_scope": "local one-icon body-resolution timing/);
  assert.match(engine, /"timing_order": \[/);
  assert.match(engine, /"engine_load_with_pack_body_cache_ns"/);
  assert.match(engine, /"machine_body_resolution_ns"/);
  assert.match(engine, /"json_fallback_resolution_ns"/);
  assert.match(engine, /"machine_svg_render_ns"/);
  assert.match(engine, /"json_svg_render_ns"/);
  assert.match(engine, /"machine_body_resolution_hit": receipt\.machine_body_resolution_hit/);
  assert.match(engine, /"json_fallback_ok": receipt\.json_fallback_ok/);
  assert.match(engine, /"machine_json_svg_match": receipt\.machine_json_svg_match/);
  assert.match(engine, /"pack_body_fast_cache_hit_read_adopted": true/);
  assert.match(engine, /"pack_body_fast_read_validation_scope": "typed envelope \+ source fingerprint \+ machine shape \+ runtime metadata; no deep JSON body equality audit"/);
  assert.match(engine, /"runtime_pack_body_machine_read_mode": serde_json::Value::Null/);
  assert.match(engine, /"runtime_pack_body_machine_read_mode_recorded": false/);
  assert.match(engine, /"local_body_resolution_receipt_only": true/);
  assert.match(engine, /"single_icon_measurement_only": true/);
  assert.match(engine, /"query_latency_measured": false/);
  assert.match(engine, /"full_render_proof_claimed": false/);
  assert.match(engine, /"same_name_body_drift_without_source_audit_proven": false/);
  assert.match(engine, /"full_icon_search_speed_claimed": false/);
  assert.match(engine, /"faster_than_upstream_claimed": false/);
  assert.match(engine, /"upstream_baseline_measured": false/);
  assert.match(engine, /"upstream_baseline_command": serde_json::Value::Null/);
  assert.match(engine, /"same_machine_benchmark_required": true/);

  const loadFastSection = extractFunction(engine, "pub fn load_fast(index_dir: &Path) -> Result<Self>");
  assert.doesNotMatch(loadFastSection, /read_runtime_pack_body_machine/);
  assert.doesNotMatch(loadFastSection, /write_body_resolution_performance_receipt/);

  assert.doesNotMatch(iconCli, /write_body_resolution_performance_receipt/);
});
