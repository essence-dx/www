import assert from "node:assert/strict";
import { readFileSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";

const repoRoot = join(import.meta.dirname, "..");

function read(path: string): string {
  return readFileSync(join(repoRoot, path), "utf8");
}

function rustFilesUnder(relativeDir: string): string[] {
  return readdirSync(join(repoRoot, relativeDir)).flatMap((entry) => {
    const relativePath = `${relativeDir}/${entry}`;
    const stat = statSync(join(repoRoot, relativePath));

    if (stat.isDirectory()) return rustFilesUnder(relativePath);
    return entry.endsWith(".rs") ? [relativePath] : [];
  });
}

test("media-icon raw index cache preserves compressed compatibility and adds hot uncompressed files", () => {
  const index = read("related-crates/media-icon/src/index.rs");
  const engine = read("related-crates/media-icon/src/engine.rs");
  const builder = read("related-crates/media-icon/src/builder.rs");
  const searchCli = read("related-crates/media-icon/src/bin/search_cli.rs");
  const iconCli = read("related-crates/media-icon/src/bin/icon.rs");
  const perfTest = read("related-crates/media-icon/src/bin/perf_test.rs");
  const demandingBenchmark = read("related-crates/media-icon/src/bin/demanding_benchmark.rs");
  const gpuBenchmark = read("related-crates/media-icon/src/bin/gpu_benchmark.rs");

  assert.match(index, /const RAW_FST_FILE: &str = "index\.fst"/);
  assert.match(index, /const RAW_METADATA_FILE: &str = "index\.meta\.machine"/);
  assert.match(index, /const RAW_INTEGRITY_FILE: &str = "index\.raw\.integrity\.json"/);
  assert.match(index, /const RAW_INDEX_SCHEMA: &str = "dx\.media-icon\.raw-index\.v1"/);
  assert.match(index, /const RAW_INDEX_VERSION: u32 = 1/);
  assert.match(index, /const RAW_INDEX_PERFORMANCE_RECEIPT_SCHEMA: &str =\s*"dx\.performance\.json_machine_cache_receipt\.media_icon_raw_index\.v1"/);
  assert.match(index, /const RAW_INDEX_PERFORMANCE_RECEIPT_PATH: &str =\s*"\.dx\/performance\/json-machine-cache-receipts\/media-icon-raw-index\.json"/);
  assert.match(index, /struct RawIndexIntegrity[\s\S]*compressed: Option<CompressedIndexIntegrity>/);
  assert.match(index, /struct CompressedIndexIntegrity[\s\S]*fst: RawIndexFileIntegrity[\s\S]*metadata: RawIndexFileIntegrity/);
  assert.match(index, /struct RawIndexFileIntegrity[\s\S]*file: String[\s\S]*bytes: u64[\s\S]*blake3: String/);
  assert.match(index, /pub struct MappedIconIndex/);
  assert.match(index, /pub fn save_uncompressed/);
  assert.match(index, /pub fn save_all/);
  assert.match(index, /let _ = Self::write_raw_index_performance_receipt\(path\);/);
  assert.match(index, /pub fn write_raw_index_performance_receipt\(path: &Path\) -> Result<PathBuf>/);
  assert.match(index, /pub fn load_fast/);
  assert.match(index, /pub fn load_uncompressed_mmap/);
  assert.match(index, /COMPRESSED_FST_FILE/);
  assert.match(index, /COMPRESSED_METADATA_FILE/);
  assert.match(index, /write_raw_index_integrity\(path, &self\.fst_bytes, &self\.metadata_bytes\)\?/);
  assert.match(index, /read_raw_index_integrity\(path\)\?/);
  assert.match(index, /validate_current_compressed_index\(path, integrity\.compressed\.as_ref\(\)\)\?/);
  assert.match(index, /validate_index_file_bytes\(RAW_FST_FILE, &fst_bytes, &integrity\.fst\)\?/);
  assert.match(index, /validate_index_file_bytes\(RAW_METADATA_FILE, &metadata_bytes, &integrity\.metadata\)\?/);
  assert.match(index, /validate_index_file_bytes\(RAW_FST_FILE, &fst_mmap, &integrity\.fst\)\?/);
  assert.match(index, /validate_index_file_bytes\(RAW_METADATA_FILE, &metadata_mmap, &integrity\.metadata\)\?/);
  assert.match(index, /validate_raw_fst_bytes\(&fst_bytes\)\?/);
  assert.match(index, /validate_raw_fst_bytes\(&fst_mmap\)\?/);

  assert.match(builder, /index\.save_all\(output_dir\)\?/);
  assert.match(engine, /pub fn load_fast\(index_dir: &Path\) -> Result<Self>/);
  assert.match(engine, /IconIndex::load_uncompressed_mmap\(index_dir\)/);
  assert.match(engine, /Self::from_mapped_index_with_catalog_and_prefix\(\s*&index,\s*catalog_machine,\s*prefix_machine,\s*\)/);
  assert.match(engine, /pub fn from_mapped_index\(index: &MappedIconIndex\) -> Result<Self>/);
  assert.match(engine, /IconIndex::load_fast\(index_dir\)\?/);
  assert.match(engine, /Self::from_index_with_catalog_and_prefix\(index, catalog_machine, prefix_machine\)/);
  assert.match(searchCli, /IconSearchEngine::load_fast\(index_dir\)\?/);
  assert.doesNotMatch(searchCli, /IconIndex::load/);
  assert.match(iconCli, /IconSearchEngine::load_fast\(index_dir\)\?/);
  assert.doesNotMatch(iconCli, /IconIndex::load/);
  assert.match(perfTest, /IconIndex::load_fast\(&index_dir\)\?/);
  assert.match(demandingBenchmark, /IconIndex::load_fast\(&index_dir\)\?/);
  assert.match(gpuBenchmark, /IconIndex::load_fast\(&index_dir\)\?/);

  for (const file of rustFilesUnder("related-crates/media-icon/src")) {
    assert.doesNotMatch(
      read(file),
      /\bIconIndex::load\s*\(/,
      `${file} should not call IconIndex::load directly`,
    );
  }
});

test("media-icon engine validates rkyv metadata before materializing search state", () => {
  const engine = read("related-crates/media-icon/src/engine.rs");

  assert.match(engine, /pub fn from_mapped_index\(index: &MappedIconIndex\)/);
  assert.match(engine, /rkyv::access::<Archived<Vec<IconMetadata>>, rkyv::rancor::Error>/);
  assert.doesNotMatch(engine, /access_unchecked/);
});

test("media-icon raw index integrity uses blake3 and preserves compressed fallback", () => {
  const index = read("related-crates/media-icon/src/index.rs");

  assert.match(index, /blake3::hash\(bytes\)\.to_hex\(\)\.to_string\(\)/);
  assert.match(index, /serde_json::to_vec_pretty\(&integrity\)\?/);
  assert.match(index, /serde_json::from_slice\(&bytes\)/);
  assert.match(index, /compressed: current_compressed_index_integrity\(path\)\?/);
  assert.match(index, /raw icon index integrity is missing compressed source fingerprints/);
  assert.match(index, /raw icon index integrity expects legacy compressed files, but they are missing/);
  assert.match(index, /validate_integrity_record\(COMPRESSED_FST_FILE, &current\.fst, &expected\.fst\)\?/);
  assert.match(index, /fst::Map::new\(bytes\)/);
  assert.match(index, /Self::load_uncompressed\(path\)\.or_else\(\|_\| Self::load\(path\)\)/);
});

test("media-icon raw index writes an honest local performance receipt", () => {
  const index = read("related-crates/media-icon/src/index.rs");

  assert.match(index, /"cache_name": "media-icon-raw-index"/);
  assert.match(index, /"cache_kind": "raw-fst-rkyv-metadata-index"/);
  assert.match(index, /"compressed_load_ns": compressed_load_ns/);
  assert.match(index, /"raw_owned_load_ns": raw_owned_load_ns/);
  assert.match(index, /"raw_mmap_validate_load_ns": raw_mmap_validate_load_ns/);
  assert.match(index, /"measurement_scope": "post-save local validation timing; OS page cache may be warm"/);
  assert.match(index, /"timing_order": \["compressed_load", "raw_owned_load", "raw_mmap_validate_load"\]/);
  assert.match(index, /"post_save_page_cache_may_be_warm": true/);
  assert.match(index, /"raw_owned_load_includes_validation": true/);
  assert.match(index, /"raw_mmap_validate_load_hashes_full_files": true/);
  assert.match(index, /"raw_mmap_available": raw_mmap_result\.is_ok\(\)/);
  assert.match(index, /"compressed_load_ok": compressed_load_result\.is_ok\(\)/);
  assert.match(index, /"raw_owned_load_ok": raw_owned_load_result\.is_ok\(\)/);
  assert.match(index, /"json_source_authoritative": true/);
  assert.match(index, /"raw_index_cache_only": true/);
  assert.match(index, /"full_icon_runtime_baseline_measured": false/);
  assert.match(index, /"faster_than_upstream_claimed": false/);
  assert.match(index, /"upstream_baseline_measured": false/);
  assert.match(index, /"upstream_baseline_command": serde_json::Value::Null/);
  assert.match(index, /"upstream_baseline_checkout": serde_json::Value::Null/);
  assert.match(index, /"same_machine_benchmark_required": true/);
  assert.match(index, /"test_command_recorded": false/);
  assert.match(index, /write_atomic\(&receipt_path,\s*&serde_json::to_vec_pretty\(&receipt\)/);
  assert.match(index, /raw index performance receipt expects an index output directory/);
  assert.doesNotMatch(index, /std::env::current_dir\(\)\.context\("resolve current directory for raw index performance receipt"\)/);
  assert.match(index, /std::env::consts::OS/);
  assert.match(index, /std::env::consts::ARCH/);
});
