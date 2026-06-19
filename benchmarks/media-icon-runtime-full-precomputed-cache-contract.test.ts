import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";

const repoRoot = join(import.meta.dirname, "..");

function read(path: string): string {
  return readFileSync(join(repoRoot, path), "utf8");
}

test("runtime precomputed cache adoption reuses validated lowercase-cache names", () => {
  const perfectHash = read("related-crates/media-icon/src/perfect_hash.rs");
  const precomputed = read("related-crates/media-icon/src/precomputed.rs");
  const engine = read("related-crates/media-icon/src/engine.rs");

  assert.match(perfectHash, /pub fn as_slice\(&self\) -> &\[String\]/);
  assert.match(precomputed, /pub lowercase_names_from_machine_cache: bool/);
  assert.match(precomputed, /enum LowercaseNamesSource<'a>/);
  assert.match(precomputed, /LowercaseNamesSource::Borrowed\(lowercase_cache\.as_slice\(\)\)/);
  assert.match(precomputed, /LowercaseNamesSource::Owned\(\s*metadata\.iter\(\)[\s\S]*\.map\(\|m\| m\.name\.to_lowercase\(\)[\s\S]*\.collect\(\),\s*\)/);
  assert.match(precomputed, /PrefixIndex::from_icon_id_prefix_machine_with_lowercase_names/);
  assert.match(precomputed, /pub fn from_icon_id_prefix_machine_with_lowercase_names\(/);

  assert.match(engine, /pub lowercase_names_from_machine_cache: bool/);
  assert.match(engine, /runtime_all_precomputed_machines_adopted/);
  assert.match(engine, /runtime_non_prefix_precomputed_machines_adopted/);
  assert.match(engine, /runtime_precomputed_cache_adopted: runtime_all_precomputed_machines_adopted/);
  assert.match(engine, /runtime_rebuilds_non_prefix_precomputed_structures:\s*!runtime_non_prefix_precomputed_machines_adopted/);
  assert.match(engine, /"lowercase_names_from_machine_cache": receipt\.lowercase_names_from_machine_cache/);
  assert.match(engine, /"runtime_full_precomputed_cache_adopted"[\s\S]*receipt\.runtime_precomputed_cache_adopted/);
  assert.match(engine, /"runtime_rebuilds_full_precomputed_index"[\s\S]*!receipt\.runtime_precomputed_cache_adopted/);

  assert.match(engine, /search_behavior_changed: false/);
  assert.match(engine, /full_icon_search_speed_claimed: false/);
  assert.match(engine, /faster_than_upstream_claimed: false/);
  assert.match(engine, /upstream_baseline_measured: false/);
});
