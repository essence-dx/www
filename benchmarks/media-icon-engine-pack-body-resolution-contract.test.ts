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

test("media-icon engine adopts pack-body machine for body resolution without changing search", () => {
  const packBody = read("related-crates/media-icon/src/machine_pack_body.rs");
  const engine = read("related-crates/media-icon/src/engine.rs");
  const iconCli = read("related-crates/media-icon/src/bin/icon.rs");

  assert.match(packBody, /PACK_BODY_MACHINE_ENGINE_BODY_RESOLUTION_ADOPTED: bool = true/);
  assert.match(packBody, /PACK_BODY_MACHINE_ENGINE_SEARCH_ADOPTED: bool = false/);
  assert.match(packBody, /PACK_BODY_MACHINE_FASTER_THAN_UPSTREAM_CLAIMED: bool = false/);

  assert.match(engine, /ResolvedIconPackBody/);
  assert.match(engine, /resolve_icon_pack_body/);
  assert.match(engine, /pack_body_machine: Option<Arc<IconPackBodyMachineV1>>/);
  assert.match(engine, /pub fn load_fast_with_pack_body_cache\(index_dir: &Path\) -> Result<Self>/);
  assert.match(engine, /pub fn has_pack_body_cache\(&self\) -> bool/);
  assert.match(
    engine,
    /pub fn resolve_icon_body\(&self, pack: &str, name: &str\) -> Option<ResolvedIconPackBody>/,
  );
  assert.match(
    engine,
    /resolve_icon_pack_body\(machine\.as_ref\(\), pack, name\)/,
  );
  assert.match(
    engine,
    /validate_icon_pack_body_runtime_metadata\(machine\.as_ref\(\), metadata\.as_slice\(\)\)/,
  );
  assert.match(
    engine,
    /pack_body_machine: pack_body_machine_for_engine/,
  );

  const loadFastSection = extractFunction(engine, "pub fn load_fast(index_dir: &Path) -> Result<Self>");
  assert.doesNotMatch(loadFastSection, /read_runtime_pack_body_machine/);

  assert.match(engine, /runtime_pack_body_machine_adopted: pack_body_evidence\s*\.runtime_pack_body_machine_evidence_validated/);
  assert.match(engine, /pack_body_machine_consumed_by_search: false/);
  assert.match(engine, /pack_body_machine_evidence_only: !pack_body_machine_consumed_for_body_resolution/);
  assert.match(
    engine,
    /"pack_body_machine_consumed_for_body_resolution": receipt\.pack_body_machine_consumed_for_body_resolution/,
  );
  assert.match(engine, /"pack_body_machine_consumed_by_engine_search": false/);
  assert.match(engine, /search_behavior_changed: false/);
  assert.match(engine, /faster_than_upstream_claimed: false/);

  assert.match(iconCli, /IconSearchEngine::load_fast_with_pack_body_cache\(index_dir\)/);
  assert.match(iconCli, /fn with_body_engine<F, R>\(f: F\) -> anyhow::Result<R>/);
  assert.match(iconCli, /engine\.resolve_icon_body\(pack, name\)/);
  assert.doesNotMatch(iconCli, /static PACK_BODY_MACHINE/);
  assert.doesNotMatch(iconCli, /read_icon_pack_body_machine_cache/);
});
