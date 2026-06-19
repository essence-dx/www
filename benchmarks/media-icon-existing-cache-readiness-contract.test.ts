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

test("media-icon exposes explicit existing-cache readiness proof before default readiness", () => {
  const lib = read("related-crates/media-icon/src/lib.rs");
  const builder = read("related-crates/media-icon/src/builder.rs");
  const manifest = read("related-crates/media-icon/src/machine_manifest.rs");
  const readiness = read("related-crates/media-icon/src/machine_readiness.rs");
  const engine = read("related-crates/media-icon/src/engine.rs");
  const iconCli = read("related-crates/media-icon/src/bin/icon.rs");

  assert.match(lib, /pub mod machine_readiness;/);
  assert.match(
    builder,
    /use crate::machine_readiness::ensure_icon_machine_caches_for_index_output;/,
  );
  assert.match(
    builder,
    /let _ = ensure_icon_machine_caches_for_index_output\(data_dir, output_dir, &icons\);/,
  );

  assert.match(readiness, /pub const ICON_MACHINE_CACHE_READINESS_SCHEMA: &str = "dx\.icon\.machine_cache_readiness\.v1"/);
  assert.match(
    readiness,
    /pub const ICON_MACHINE_CACHE_READINESS_RECEIPT_SCHEMA: &str =\s*"dx\.performance\.json_machine_cache_receipt\.media_icon_existing_cache_readiness\.v1"/,
  );
  assert.match(
    readiness,
    /pub const ICON_MACHINE_CACHE_READINESS_RECEIPT_PATH: &str =\s*"\.dx\/performance\/json-machine-cache-receipts\/media-icon-existing-cache-readiness\.json"/,
  );
  assert.match(readiness, /pub struct IconMachineCacheReadinessReport/);
  assert.match(readiness, /pub struct IconMachineCacheReadinessEntry/);
  assert.match(readiness, /pub const REQUIRED_ICON_MACHINE_CACHE_NAMES: \[&str; 7\] = \[/);

  for (const cacheName of [
    "manifest",
    "catalog",
    "prefix",
    "perfect-hash",
    "bloom",
    "lowercase-cache",
    "pack-body",
  ]) {
    assert.match(readiness, new RegExp(`"${cacheName}"`));
  }

  assert.match(
    readiness,
    /pub fn ensure_icon_machine_caches_for_index_output\(\s*data_dir: &Path,\s*output_dir: &Path,\s*icons: &\[IconMetadata\],\s*\) -> Result<IconMachineCacheReadinessReport>/,
  );
  assert.match(
    readiness,
    /pub fn ensure_icon_machine_caches_from_dir\(\s*project_root: &Path,\s*data_dir: &Path,\s*\) -> Result<IconMachineCacheReadinessReport>/,
  );
  assert.match(
    readiness,
    /pub fn ensure_icon_machine_caches\(\s*project_root: &Path,\s*data_dir: &Path,\s*icons: &\[IconMetadata\],\s*\) -> Result<IconMachineCacheReadinessReport>/,
  );

  const ensureBody = extractFunction(
    readiness,
    "pub fn ensure_icon_machine_caches(",
  );
  assert.match(ensureBody, /let before = collect_icon_machine_cache_statuses\(project_root, data_dir\);/);
  assert.match(ensureBody, /write_icon_manifest_machine_cache\(project_root, data_dir\)/);
  assert.match(ensureBody, /write_icon_catalog_prefix_machine_caches\(project_root, data_dir, icons\)/);
  assert.match(ensureBody, /write_icon_perfect_hash_machine_cache\(project_root, data_dir, icons\)/);
  assert.match(ensureBody, /write_icon_bloom_machine_cache\(project_root, data_dir, icons\)/);
  assert.match(ensureBody, /write_icon_lowercase_cache_machine_cache\(project_root, data_dir, icons\)/);
  assert.match(ensureBody, /write_icon_pack_body_machine_cache\(project_root, data_dir\)/);
  assert.match(ensureBody, /let after = collect_icon_machine_cache_statuses\(project_root, data_dir\);/);
  assert.match(ensureBody, /all_required_caches_ready: after\.iter\(\)\.all\(\|entry\| entry\.ready\)/);
  assert.match(ensureBody, /stale_or_missing_caches_regenerated/);
  assert.match(ensureBody, /write_machine_cache_readiness_receipt\(&report\)\?/);

  assert.match(readiness, /read_icon_manifest_machine_cache_summary\(project_root, data_dir\)/);
  assert.match(readiness, /read_icon_catalog_machine_cache\(project_root, data_dir\)/);
  assert.match(readiness, /read_icon_prefix_machine_cache\(project_root, data_dir\)/);
  assert.match(readiness, /read_icon_perfect_hash_machine_cache\(project_root, data_dir\)/);
  assert.match(readiness, /read_icon_bloom_machine_cache\(project_root, data_dir\)/);
  assert.match(readiness, /read_icon_lowercase_cache_machine_cache\(project_root, data_dir\)/);
  assert.match(readiness, /read_icon_pack_body_machine_cache\(project_root, data_dir\)/);
  assert.match(readiness, /"cache_name": "media-icon-existing-cache-readiness"/);
  assert.match(readiness, /"cache_kind": "existing-machine-cache-readiness"/);
  assert.match(readiness, /"json_source_authoritative": true/);
  assert.match(readiness, /"generated_machine_cache_only": true/);
  assert.match(readiness, /"normal_search_behavior_changed": false/);
  assert.match(readiness, /"full_startup_search_render_proof": false/);
  assert.match(readiness, /"upstream_baseline_measured": false/);
  assert.match(readiness, /"faster_than_upstream_claimed": false/);
  assert.match(readiness, /"same_machine_benchmark_required": true/);

  const manifestFingerprint = extractFunction(
    manifest,
    "pub fn icon_manifest_source_fingerprint(data_dir: &Path) -> Result<MachineCacheSource>",
  );
  assert.match(manifestFingerprint, /let bytes =\s*fs::read\(&path\)/);
  assert.match(manifestFingerprint, /let file_hash = blake3::hash\(&bytes\)/);
  assert.match(manifestFingerprint, /source_hasher\.update\(file_hash\.as_bytes\(\)\)/);

  const readinessWriteAtomic = extractFunction(readiness, "fn write_atomic(");
  assert.match(readinessWriteAtomic, /atomic_temp_path\(path\)/);
  assert.match(readinessWriteAtomic, /fs::rename\(&tmp,\s*path\)/);
  const readinessAtomicTempPath = extractFunction(readiness, "fn atomic_temp_path(");
  assert.match(readiness, /static ATOMIC_WRITE_COUNTER: AtomicU64 = AtomicU64::new\(0\);/);
  assert.match(readinessAtomicTempPath, /std::process::id\(\)/);
  assert.match(readinessAtomicTempPath, /current_unix_ms\(\)/);
  assert.match(readinessAtomicTempPath, /ATOMIC_WRITE_COUNTER\.fetch_add\(1, Ordering::Relaxed\)/);
  assert.match(readinessAtomicTempPath, /\.tmp\.\{\}\.\{\}\.\{\}/);
  assert.doesNotMatch(readinessWriteAtomic, /with_extension\("tmp"\)/);
  assert.doesNotMatch(readinessWriteAtomic, /remove_file\(\s*&?path\s*\)/);

  const loadFast = extractFunction(
    engine,
    "pub fn load_fast(index_dir: &Path) -> Result<Self>",
  );
  assert.doesNotMatch(loadFast, /ensure_icon_machine_caches/);
  assert.doesNotMatch(iconCli, /ensure_icon_machine_caches/);
});
