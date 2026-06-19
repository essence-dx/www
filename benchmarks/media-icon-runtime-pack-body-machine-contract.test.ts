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

test("media-icon startup receipt validates pack-body machine for body resolution only", () => {
  const packBody = read("related-crates/media-icon/src/machine_pack_body.rs");
  const engine = read("related-crates/media-icon/src/engine.rs");

  assert.match(packBody, /pub fn validate_icon_pack_body_runtime_metadata\(/);
  assert.match(packBody, /validate_icon_pack_body_parser_metadata\(machine, metadata\)\?/);
  assert.match(packBody, /PACK_BODY_MACHINE_CLI_EXPORT_ADOPTED: bool = true/);
  assert.match(packBody, /PACK_BODY_MACHINE_STARTUP_RECEIPT_EVIDENCE_ADOPTED: bool = true/);
  assert.match(packBody, /PACK_BODY_MACHINE_FAST_CACHE_HIT_READ_ADOPTED: bool = true/);
  assert.match(packBody, /PACK_BODY_MACHINE_DEEP_SOURCE_AUDIT_AVAILABLE: bool = true/);
  assert.match(packBody, /PACK_BODY_MACHINE_FAST_READ_STILL_USES_SOURCE_FINGERPRINT: bool = true/);
  assert.match(packBody, /PACK_BODY_MACHINE_ENGINE_BODY_RESOLUTION_ADOPTED: bool = true/);
  assert.match(packBody, /PACK_BODY_MACHINE_ENGINE_SEARCH_ADOPTED: bool = false/);
  assert.match(packBody, /PACK_BODY_MACHINE_FASTER_THAN_UPSTREAM_CLAIMED: bool = false/);

  assert.match(engine, /use crate::machine_pack_body::\{/);
  assert.match(engine, /IconPackBodyMachineV1/);
  assert.match(engine, /read_icon_pack_body_machine_cache_with_source_fingerprint/);
  assert.match(engine, /validate_icon_pack_body_runtime_metadata/);
  assert.match(engine, /struct RuntimePackBodyMachine/);
  assert.match(engine, /struct PackBodyMachineEvidenceSummary/);
  assert.match(
    engine,
    /fn read_runtime_pack_body_machine\(\s*context: std::result::Result<&RuntimeMachineReadContext, &RuntimeMachineReadContextError>,\s*\) -> RuntimePackBodyMachine/,
  );
  assert.match(engine, /fn validate_runtime_pack_body_machine_evidence\(/);
  assert.match(engine, /validate_icon_pack_body_runtime_metadata\(machine, metadata\)/);

  const loadFastSection = extractFunction(engine, "pub fn load_fast(index_dir: &Path) -> Result<Self>");
  assert.doesNotMatch(loadFastSection, /read_runtime_pack_body_machine/);
  assert.doesNotMatch(loadFastSection, /load_fast_with_pack_body_cache/);

  assert.match(engine, /let runtime_pack_body_machine =\s*read_runtime_pack_body_machine\(runtime_machine_context\.as_ref\(\)\);/);
  assert.match(engine, /runtime_pack_body_machine: RuntimePackBodyMachine/);
  assert.match(
    engine,
    /validate_runtime_pack_body_machine_evidence\(\s*metadata\.as_slice\(\),\s*runtime_pack_body_machine\s*\.pack_body_machine\s*\.as_deref\(\),\s*\)/,
  );
  assert.match(engine, /pub fn load_fast_with_pack_body_cache\(index_dir: &Path\) -> Result<Self>/);
  assert.match(engine, /pub fn resolve_icon_body\(&self, pack: &str, name: &str\) -> Option<ResolvedIconPackBody>/);
  assert.match(engine, /resolve_icon_pack_body\(machine\.as_ref\(\), pack, name\)/);

  assert.match(engine, /pub runtime_pack_body_machine_available: bool/);
  assert.match(engine, /pub runtime_pack_body_machine_evidence_validated: bool/);
  assert.match(engine, /pub runtime_pack_body_machine_adopted: bool/);
  assert.match(engine, /pub pack_body_machine_runtime_metadata_validated: bool/);
  assert.match(engine, /pub pack_body_machine_fallback_reason: Option<String>/);
  assert.match(engine, /pub runtime_pack_body_source: &'static str/);
  assert.match(engine, /pub runtime_pack_body_machine_read_mode: Option<&'static str>/);
  assert.match(engine, /pub pack_body_machine_consumed_for_body_resolution: bool/);

  assert.match(engine, /"runtime_pack_body_machine_available"/);
  assert.match(engine, /"runtime_pack_body_machine_evidence_validated"/);
  assert.match(engine, /"runtime_pack_body_machine_adopted": receipt\.runtime_pack_body_machine_adopted/);
  assert.match(engine, /"pack_body_machine_consumed_for_body_resolution": receipt\.pack_body_machine_consumed_for_body_resolution/);
  assert.match(engine, /"pack_body_machine_consumed_by_engine_search": false/);
  assert.match(engine, /"pack_body_machine_runtime_metadata_validated"/);
  assert.match(engine, /"runtime_pack_body_source"/);
  assert.match(engine, /"pack_body_machine_requires_runtime_metadata_validation": true/);
  assert.match(
    engine,
    /optional pack-body\.machine body-resolution adoption after typed source-fingerprint, machine-shape, and runtime metadata validation/,
  );
  assert.match(
    engine,
    /pack-body\.machine is missing, stale, unreadable, fails typed source-fingerprint validation, fails machine-shape validation, or fails runtime metadata validation/,
  );
  assert.match(engine, /"runtime_pack_body_adoption_scope"/);
  assert.match(engine, /body-resolution only/);

  assert.match(engine, /search_behavior_changed: false/);
  assert.match(engine, /"query_latency_measured": false/);
  assert.match(engine, /full_icon_search_speed_claimed: false/);
  assert.match(engine, /faster_than_upstream_claimed: false/);
  assert.match(engine, /upstream_baseline_measured: false/);
});
