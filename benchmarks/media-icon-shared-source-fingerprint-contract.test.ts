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

test("media-icon runtime machine readers reuse one source fingerprint per startup", () => {
  const catalog = read("related-crates/media-icon/src/machine_catalog.rs");
  const precomputed = read("related-crates/media-icon/src/machine_precomputed.rs");
  const packBody = read("related-crates/media-icon/src/machine_pack_body.rs");
  const engine = read("related-crates/media-icon/src/engine.rs");

  assert.match(catalog, /pub\(crate\) struct IconCatalogSourceFingerprint/);
  assert.match(catalog, /pub\(crate\) fn icon_catalog_source_fingerprint\(/);
  assert.match(catalog, /read_icon_catalog_machine_cache_with_source_fingerprint/);
  assert.match(catalog, /read_icon_prefix_machine_cache_with_source_fingerprint/);
  assert.match(precomputed, /read_icon_perfect_hash_machine_cache_with_source_fingerprint/);
  assert.match(precomputed, /read_icon_bloom_machine_cache_with_source_fingerprint/);
  assert.match(precomputed, /read_icon_lowercase_cache_machine_cache_with_source_fingerprint/);
  assert.match(packBody, /read_icon_pack_body_machine_cache_with_source_fingerprint/);

  const catalogDefault = extractFunction(catalog, "pub fn read_icon_catalog_machine_cache(");
  const prefixDefault = extractFunction(catalog, "pub fn read_icon_prefix_machine_cache(");
  const perfectHashDefault = extractFunction(precomputed, "pub fn read_icon_perfect_hash_machine_cache(");
  const bloomDefault = extractFunction(precomputed, "pub fn read_icon_bloom_machine_cache(");
  const lowercaseDefault = extractFunction(precomputed, "pub fn read_icon_lowercase_cache_machine_cache(");
  const packBodyDefault = extractFunction(packBody, "pub fn read_icon_pack_body_machine_cache(");

  for (const defaultReader of [
    catalogDefault,
    prefixDefault,
    perfectHashDefault,
    bloomDefault,
    lowercaseDefault,
    packBodyDefault,
  ]) {
    assert.match(defaultReader, /let source = icon_catalog_source_fingerprint\(data_dir\)\?/);
    assert.match(defaultReader, /_with_source_fingerprint\(\s*project_root,\s*data_dir,\s*&source,?\s*\)/);
  }

  assert.match(engine, /struct RuntimeMachineReadContext/);
  assert.match(engine, /enum RuntimeMachineReadContextError/);
  assert.match(engine, /fn read_runtime_machine_context\(\s*index_dir: &Path,?\s*\)/);
  assert.match(engine, /icon_catalog_source_fingerprint\(&data_dir\)/);
  assert.match(engine, /let runtime_machine_context = read_runtime_machine_context\(index_dir\);/);
  assert.match(engine, /runtime_machine_source_fingerprint_reused: true/);
  assert.match(engine, /"runtime_machine_source_fingerprint_reused": receipt\.runtime_machine_source_fingerprint_reused/);

  const loadFast = engine.slice(
    engine.indexOf("pub fn load_fast(index_dir: &Path) -> Result<Self>"),
    engine.indexOf("/// Load a search engine and return a local startup timing receipt."),
  );
  const loadFastWithReceipt = engine.slice(
    engine.indexOf("pub fn load_fast_with_startup_receipt("),
    engine.indexOf("match IconIndex::load_uncompressed_mmap(index_dir)", engine.indexOf("pub fn load_fast_with_startup_receipt(")),
  );
  assert.match(loadFast, /read_runtime_catalog_machine\(runtime_machine_context\.as_ref\(\)\)/);
  assert.match(loadFast, /read_runtime_lowercase_cache_machine\(runtime_machine_context\.as_ref\(\)\)/);
  assert.match(loadFastWithReceipt, /read_runtime_pack_body_machine\(runtime_machine_context\.as_ref\(\)\)/);

  const runtimeCatalog = extractFunction(
    engine,
    "fn read_runtime_catalog_machine(\n    context: std::result::Result<&RuntimeMachineReadContext, &RuntimeMachineReadContextError>,",
  );
  const runtimePrefix = extractFunction(
    engine,
    "fn read_runtime_prefix_machine(\n    context: std::result::Result<&RuntimeMachineReadContext, &RuntimeMachineReadContextError>,",
  );
  const runtimePerfectHash = extractFunction(
    engine,
    "fn read_runtime_perfect_hash_machine(\n    context: std::result::Result<&RuntimeMachineReadContext, &RuntimeMachineReadContextError>,",
  );
  const runtimeBloom = extractFunction(
    engine,
    "fn read_runtime_bloom_machine(\n    context: std::result::Result<&RuntimeMachineReadContext, &RuntimeMachineReadContextError>,",
  );
  const runtimeLowercase = extractFunction(
    engine,
    "fn read_runtime_lowercase_cache_machine(\n    context: std::result::Result<&RuntimeMachineReadContext, &RuntimeMachineReadContextError>,",
  );
  const runtimePackBody = extractFunction(
    engine,
    "fn read_runtime_pack_body_machine(\n    context: std::result::Result<&RuntimeMachineReadContext, &RuntimeMachineReadContextError>,",
  );

  for (const runtimeReader of [
    runtimeCatalog,
    runtimePrefix,
    runtimePerfectHash,
    runtimeBloom,
    runtimeLowercase,
    runtimePackBody,
  ]) {
    assert.doesNotMatch(runtimeReader, /engine_startup_project_root/);
    assert.doesNotMatch(runtimeReader, /select_icon_data_dir/);
    assert.doesNotMatch(runtimeReader, /_for_index_output\(index_dir, &data_dir\)/);
    assert.match(runtimeReader, /context\.source_fingerprint/);
    assert.match(runtimeReader, /_with_source_fingerprint/);
  }
});
