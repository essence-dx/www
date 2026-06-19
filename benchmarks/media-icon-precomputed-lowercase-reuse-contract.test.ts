import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";

const repoRoot = join(import.meta.dirname, "..");

function read(path: string): string {
  return readFileSync(join(repoRoot, path), "utf8");
}

test("media-icon precomputed startup reuses one lowercase-name vector", () => {
  const precomputed = read("related-crates/media-icon/src/precomputed.rs");
  const perfectHash = read("related-crates/media-icon/src/perfect_hash.rs");

  assert.match(perfectHash, /pub fn build_from_lowercase_names\(lowercase_names: &\[String\]\) -> Self/);
  assert.match(perfectHash, /fn find_perfect_seed_for_names\(lowercase_names: &\[String\], table_size: usize\) -> Option<u64>/);
  assert.match(perfectHash, /for \(idx, name\) in lowercase_names\.iter\(\)\.enumerate\(\)/);
  assert.match(perfectHash, /pub fn from_lowercase_names\(lowercase_names: Vec<String>\) -> Self/);
  assert.match(perfectHash, /pub fn build\(metadata: &\[IconMetadata\]\) -> Self[\s\S]*Self::build_from_lowercase_names\(&lowercase_names\)/);
  assert.match(perfectHash, /pub fn build\(metadata: &\[IconMetadata\]\) -> Self[\s\S]*metadata[\s\S]*\.iter\(\)[\s\S]*\.map\(\|icon\| icon\.name\.to_lowercase\(\)\)[\s\S]*\.collect\(\)/);
  assert.match(perfectHash, /pub fn lookup_exact\(&self, query: &str\) -> Option<u32>[\s\S]*let query = query\.to_lowercase\(\);/);

  assert.match(precomputed, /let lowercase_names: Vec<String> = metadata\.iter\(\)\.map\(\|m\| m\.name\.to_lowercase\(\)\)\.collect\(\);/);
  assert.match(precomputed, /PerfectHashIndex::build_from_lowercase_names\(&lowercase_names\)/);
  assert.match(precomputed, /IconBloomFilters::build\(&lowercase_names\)/);
  assert.match(precomputed, /PrefixIndex::build\(&lowercase_names\)/);
  assert.match(precomputed, /LowercaseCache::from_lowercase_names\(lowercase_names\)/);

  assert.doesNotMatch(precomputed, /PerfectHashIndex::build\(&metadata\)/);
  assert.doesNotMatch(precomputed, /LowercaseCache::build\(&metadata\)/);
});
