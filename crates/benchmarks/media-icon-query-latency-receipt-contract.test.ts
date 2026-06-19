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

test("media-icon writes bounded local query-latency receipt without upstream claims", () => {
  const engine = read("related-crates/media-icon/src/engine.rs");
  const iconCli = read("related-crates/media-icon/src/bin/icon.rs");

  assert.match(
    engine,
    /const QUERY_LATENCY_RECEIPT_SCHEMA: &str =\s*"dx\.performance\.json_machine_cache_receipt\.media_icon_query_latency\.v1"/,
  );
  assert.match(
    engine,
    /const QUERY_LATENCY_RECEIPT_PATH: &str =\s*"\.dx\/performance\/json-machine-cache-receipts\/media-icon-query-latency\.json"/,
  );
  assert.match(engine, /const QUERY_LATENCY_MAX_QUERIES: usize = 12/);
  assert.match(engine, /const QUERY_LATENCY_MAX_LIMIT: usize = 32/);
  assert.match(engine, /const QUERY_LATENCY_MAX_WARMUP_RUNS: usize = 3/);
  assert.match(engine, /const QUERY_LATENCY_MAX_MEASURED_RUNS: usize = 5/);
  assert.match(engine, /pub struct IconQueryLatencyReceipt/);
  assert.match(engine, /pub struct IconQueryLatencySample/);
  assert.match(
    engine,
    /pub fn write_query_latency_performance_receipt\(\s*index_dir: &Path,\s*data_dir: &Path,\s*queries: &\[&str\],\s*limit: usize,\s*warmup_runs: usize,\s*measured_runs: usize,\s*\) -> Result<PathBuf>/,
  );

  const writer = extractFunction(
    engine,
    "pub fn write_query_latency_performance_receipt(",
  );
  assert.match(writer, /Self::load_fast_with_pack_body_cache\(index_dir\)\?/);
  assert.match(writer, /bounded_query_latency_inputs\(queries, limit, warmup_runs, measured_runs\)\?/);
  assert.match(writer, /for _ in 0\.\.effective_warmup_runs/);
  assert.match(writer, /for run_index in 0\.\.effective_measured_runs/);
  assert.match(writer, /engine\.search\(query, effective_limit\)/);
  assert.match(writer, /engine\.resolve_icon_body\(&top_icon\.pack, &top_icon\.name\)/);
  assert.match(writer, /resolve_icon_body_from_json\(data_dir, &top_icon\.pack, &top_icon\.name\)/);
  assert.match(writer, /render_resolved_icon_svg/);
  assert.match(writer, /query_latency_receipt_json\(&receipt, &receipt_path\)/);
  assert.match(writer, /write_atomic\(&receipt_path/);
  assert.doesNotMatch(writer, /fs::write\(&.*svg|create_dir_all\(.*output|export_icons|export_to_desktop/);

  assert.match(engine, /fn query_latency_receipt_path\(/);
  assert.match(engine, /fn bounded_query_latency_inputs\(/);
  assert.match(engine, /effective_limit: limit\.max\(1\)\.min\(QUERY_LATENCY_MAX_LIMIT\)/);
  assert.match(engine, /effective_warmup_runs: warmup_runs\.max\(1\)\.min\(QUERY_LATENCY_MAX_WARMUP_RUNS\)/);
  assert.match(engine, /effective_measured_runs: measured_runs\.max\(1\)\.min\(QUERY_LATENCY_MAX_MEASURED_RUNS\)/);
  assert.match(engine, /fn query_latency_receipt_json\(/);
  assert.match(engine, /fn match_type_label\(/);
  assert.match(engine, /"cache_name": "media-icon-query-latency"/);
  assert.match(engine, /"cache_kind": "bounded-query-latency-receipt"/);
  assert.match(engine, /"measurement_scope": "local bounded warm-cache query latency/);
  assert.match(engine, /"requested_query_count": receipt\.requested_query_count/);
  assert.match(engine, /"effective_query_count": receipt\.effective_query_count/);
  assert.match(engine, /"requested_limit": receipt\.requested_limit/);
  assert.match(engine, /"effective_limit": receipt\.effective_limit/);
  assert.match(engine, /"requested_warmup_runs": receipt\.requested_warmup_runs/);
  assert.match(engine, /"effective_warmup_runs": receipt\.effective_warmup_runs/);
  assert.match(engine, /"requested_measured_runs": receipt\.requested_measured_runs/);
  assert.match(engine, /"effective_measured_runs": receipt\.effective_measured_runs/);
  assert.match(engine, /"engine_load_with_pack_body_cache_ns": receipt\.engine_load_with_pack_body_cache_ns/);
  assert.match(engine, /"samples": receipt\.samples\.iter\(\)/);
  assert.match(engine, /"search_ns": sample\.search_ns/);
  assert.match(engine, /"body_resolution_ns": sample\.body_resolution_ns/);
  assert.match(engine, /"svg_render_ns": sample\.svg_render_ns/);
  assert.match(engine, /"export_like_top_result_ns": sample\.export_like_top_result_ns/);
  assert.match(engine, /"export_like_source": sample\.export_like_source/);
  assert.match(engine, /"disk_export_writes_performed": false/);
  assert.match(engine, /"bounded_query_latency_receipt_only": true/);
  assert.match(engine, /"warm_cache_only": true/);
  assert.match(engine, /"query_latency_measured": true/);
  assert.match(engine, /"full_startup_search_render_proof_claimed": false/);
  assert.match(engine, /"full_render_proof_claimed": false/);
  assert.match(engine, /"same_name_body_drift_without_source_audit_proven": false/);
  assert.match(engine, /"faster_than_upstream_claimed": false/);
  assert.match(engine, /"upstream_baseline_measured": false/);
  assert.match(engine, /"upstream_baseline_command": serde_json::Value::Null/);
  assert.match(engine, /"same_machine_benchmark_required": true/);

  const loadFastSection = extractFunction(engine, "pub fn load_fast(index_dir: &Path) -> Result<Self>");
  assert.doesNotMatch(loadFastSection, /read_runtime_pack_body_machine/);
  assert.doesNotMatch(loadFastSection, /write_query_latency_performance_receipt/);

  assert.doesNotMatch(iconCli, /write_query_latency_performance_receipt/);
});
