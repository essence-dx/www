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
    if (char === "{") {
      depth += 1;
    } else if (char === "}") {
      depth -= 1;
      if (depth === 0) {
        return source.slice(start, index + 1);
      }
    }
  }

  assert.fail(`unterminated function body: ${signature}`);
}

test("pack-body machine default reads avoid deep JSON body validation on cache hits", () => {
  const packBody = read("related-crates/media-icon/src/machine_pack_body.rs");
  const iconCli = read("related-crates/media-icon/src/bin/icon.rs");
  const engine = read("related-crates/media-icon/src/engine.rs");

  assert.match(packBody, /PACK_BODY_MACHINE_FAST_CACHE_HIT_READ_ADOPTED: bool = true/);
  assert.match(packBody, /PACK_BODY_MACHINE_DEEP_SOURCE_AUDIT_AVAILABLE: bool = true/);
  assert.match(packBody, /PACK_BODY_MACHINE_FAST_READ_STILL_USES_SOURCE_FINGERPRINT: bool = true/);

  const summaryReader = extractFunction(
    packBody,
    "pub fn read_icon_pack_body_machine_cache_summary(",
  );
  const runtimeReader = extractFunction(
    packBody,
    "pub fn read_icon_pack_body_machine_cache(",
  );
  const auditReader = extractFunction(
    packBody,
    "pub fn read_icon_pack_body_machine_cache_with_source_audit(",
  );
  const sourceAudit = extractFunction(packBody, "fn validate_icon_pack_body_source_files(");

  assert.match(summaryReader, /icon_catalog_source_fingerprint\(data_dir\)/);
  assert.match(runtimeReader, /icon_catalog_source_fingerprint\(data_dir\)/);
  assert.match(summaryReader, /validate_icon_pack_body_machine\(&pack_body_machine\)/);
  assert.match(runtimeReader, /validate_icon_pack_body_machine\(&pack_body_machine\)/);

  assert.doesNotMatch(summaryReader, /validate_icon_pack_body_source_files/);
  assert.doesNotMatch(runtimeReader, /validate_icon_pack_body_source_files/);
  assert.doesNotMatch(summaryReader, /serde_json::from_str::<IconPack>/);
  assert.doesNotMatch(runtimeReader, /serde_json::from_str::<IconPack>/);
  assert.doesNotMatch(summaryReader, /source_icon\.body != icon\.body/);
  assert.doesNotMatch(runtimeReader, /source_icon\.body != icon\.body/);

  assert.match(auditReader, /read_icon_pack_body_machine_cache\(project_root, data_dir\)\?/);
  assert.match(auditReader, /validate_icon_pack_body_source_files\(&read\.pack_body_machine, data_dir\)\?/);
  assert.match(sourceAudit, /serde_json::from_str::<IconPack>/);
  assert.match(sourceAudit, /source_icon\.body != icon\.body/);
  assert.match(sourceAudit, /source_icon\.width != icon\.width \|\| source_icon\.height != icon\.height/);

  assert.match(iconCli, /read_icon_pack_body_machine_cache\(&project_root, &data_dir\)/);
  assert.doesNotMatch(iconCli, /read_icon_pack_body_machine_cache_with_source_audit/);
  assert.match(engine, /read_icon_pack_body_machine_cache_for_index_output\(index_dir, &data_dir\)/);
  assert.doesNotMatch(engine, /read_icon_pack_body_machine_cache_with_source_audit/);
});
