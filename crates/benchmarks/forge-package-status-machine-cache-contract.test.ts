import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

test("Forge package-status uses typed machine cache before JSON fallbacks", () => {
  const machine = read("core/src/ecosystem/forge_package_status_machine.rs");
  const lanes = read(
    "core/src/ecosystem/dx_check_receipt/panel_parts/package_lanes.rs",
  );
  const cli = read("dx-www/src/cli/new_command.rs");
  const gitignore = read(".gitignore");
  const writerStart = machine.indexOf(
    "pub fn write_forge_package_status_machine_cache_with_performance_receipt",
  );
  const writerEnd = machine.indexOf(
    "pub(crate) fn read_forge_package_status_machine_cache",
  );
  assert.notEqual(writerStart, -1);
  assert.notEqual(writerEnd, -1);
  const performanceReceiptWriter = machine.slice(writerStart, writerEnd);

  assert.match(machine, /struct ForgePackageStatusMachineCache/);
  assert.match(machine, /struct ForgePackageStatusMachineCacheWrite/);
  assert.match(machine, /source:\s*MachineCacheSource/);
  assert.match(machine, /receipt:\s*MachineCacheReceipt/);
  assert.match(machine, /derive\(Archive,\s*RkyvSerialize,\s*RkyvDeserialize/);
  assert.match(machine, /write_typed_machine_cache\(/);
  assert.match(machine, /fn write_forge_package_status_machine_cache_with_source/);
  assert.match(machine, /write_forge_package_status_machine_cache_with_performance_receipt/);
  assert.match(
    performanceReceiptWriter,
    /let\s+machine_cache_write\s*=\s*write_forge_package_status_machine_cache_with_source\(/,
  );
  assert.match(machine, /FORGE_PACKAGE_STATUS_PERFORMANCE_RECEIPT_PATH:\s*&str\s*=/);
  assert.match(
    machine,
    /\.dx\/performance\/json-machine-cache-receipts\/forge-package-status\.json/,
  );
  assert.match(machine, /source_parse_ns/);
  assert.match(machine, /cache_generation_ns/);
  assert.match(machine, /machine_validate_read_ns/);
  assert.match(machine, /machine_validate_mmap_read_ns/);
  assert.match(machine, /machine_read_mode/);
  assert.match(machine, /ForgePackageStatusMachineJsonValue/);
  assert.match(machine, /ForgePackageStatusMachineObjectField/);
  assert.match(machine, /let\s+cache_schema\s*=\s*forge_package_status_schema\(\);/);
  assert.match(machine, /let\s+cache_schema_kind\s*=\s*match\s+cache_schema\.kind/);
  assert.match(machine, /"cache_schema":\s*cache_schema\.name/);
  assert.match(machine, /"cache_version":\s*cache_schema\.version/);
  assert.match(machine, /"machine_cache_schema"/);
  assert.match(machine, /"name":\s*cache_schema\.name/);
  assert.match(machine, /"version":\s*cache_schema\.version/);
  assert.match(machine, /"kind":\s*cache_schema_kind/);
  assert.match(machine, /"machine_cache_provenance"/);
  assert.match(machine, /"cache_encoding":\s*"typed-rkyv"/);
  assert.match(machine, /"invalidates_schema_versions":\s*\[1\]/);
  assert.match(machine, /"stale_v1_cache_accepted":\s*false/);
  assert.match(
    performanceReceiptWriter,
    /let\s+source_fingerprint\s*=\s*&machine_cache_write\.source;/,
  );
  assert.doesNotMatch(performanceReceiptWriter, /source_fingerprint\(&source_path\)/);
  assert.match(performanceReceiptWriter, /"source_fingerprint"/);
  assert.match(performanceReceiptWriter, /"path":\s*FORGE_PACKAGE_STATUS_SOURCE_PATH/);
  assert.match(performanceReceiptWriter, /"bytes":\s*source_fingerprint\.bytes/);
  assert.match(
    performanceReceiptWriter,
    /"modified_unix_ms":\s*source_fingerprint\.modified_unix_ms/,
  );
  assert.match(performanceReceiptWriter, /"blake3":\s*hex32\(source_fingerprint\.blake3\)/);
  assert.match(performanceReceiptWriter, /"source_bytes":\s*source_fingerprint\.bytes/);
  assert.doesNotMatch(
    performanceReceiptWriter,
    /"source_bytes":\s*source_bytes\.len\(\)\s+as\s+u64/,
  );
  assert.match(performanceReceiptWriter, /"source_modified_unix_ms"/);
  assert.match(performanceReceiptWriter, /"source_blake3":\s*hex32\(source_fingerprint\.blake3\)/);
  assert.match(performanceReceiptWriter, /"source_fingerprint_provenance"/);
  assert.match(
    performanceReceiptWriter,
    /"provider":\s*"serializer::machine::source_fingerprint"/,
  );
  assert.match(performanceReceiptWriter, /"hash_algorithm":\s*"blake3"/);
  assert.match(performanceReceiptWriter, /"modified_time_unit":\s*"unix_ms"/);
  assert.match(performanceReceiptWriter, /"reused_for_machine_cache_write":\s*true/);
  assert.match(performanceReceiptWriter, /"machine_bytes":\s*machine_cache_write\.receipt\.machine_bytes/);
  assert.match(
    performanceReceiptWriter,
    /"machine_archive_bytes":\s*machine_cache_write\.receipt\.archive_bytes/,
  );
  assert.match(
    performanceReceiptWriter,
    /"machine_archive_blake3":\s*hex32\(machine_cache_write\.receipt\.archive_blake3\)/,
  );
  assert.match(performanceReceiptWriter, /"machine_receipt_provenance"/);
  assert.match(
    performanceReceiptWriter,
    /"provider":\s*"serializer::machine::write_typed_machine_cache"/,
  );
  assert.match(performanceReceiptWriter, /"archive_hash_algorithm":\s*"blake3"/);
  assert.match(
    performanceReceiptWriter,
    /"reused_from_write_typed_machine_cache_receipt":\s*true/,
  );
  assert.match(machine, /fn\s+hex32\(bytes:\s*\[u8;\s*32\]\)\s*->\s*String/);
  assert.match(machine, /String::with_capacity\(64\)/);
  assert.match(machine, /machine_visibility_entry_materialization/);
  assert.match(machine, /typed archived JSON value materialized directly from the machine cache/);
  assert.doesNotMatch(machine, /typed_cache_still_parses_visibility_entry_raw_json/);
  assert.doesNotMatch(machine, /\braw_json\b/);
  assert.doesNotMatch(machine, /raw JSON string parsing/i);
  assert.doesNotMatch(machine, /serde_json::to_string\(entry\)/);
  assert.doesNotMatch(machine, /serde_json::from_str/);
  assert.match(machine, /faster_than_upstream_claimed/);
  assert.match(machine, /upstream_baseline_measured/);
  assert.match(machine, /"test_command": null/);
  assert.match(machine, /test_command_recorded/);
  assert.match(machine, /suggested_test_command/);
  assert.match(machine, /open_typed_machine_cache::<ForgePackageStatusMachineCache>/);
  assert.match(machine, /access_typed_machine_cache::<ForgePackageStatusMachineCache>/);
  assert.match(machine, /MachineCacheKind::Receipt/);
  assert.match(machine, /name:\s*"dx\.www\.forge_package_status"/);
  assert.doesNotMatch(machine, /name:\s*"dx\.www\.forge_package_status\.v2"/);
  assert.match(machine, /version:\s*2/);
  assert.match(
    machine,
    /FORGE_PACKAGE_STATUS_CACHE_NAME:\s*&str\s*=\s*"forge-package-status"/,
  );
  assert.match(
    machine,
    /FORGE_PACKAGE_STATUS_SOURCE_PATH:\s*&str\s*=\s*"\.dx\/forge\/package-status\.json"/,
  );

  assert.match(lanes, /read_forge_package_status_machine_cache\(root\)/);
  assert.match(lanes, /read_json_receipt_machine_alias\(/);
  assert.ok(
    lanes.indexOf("read_forge_package_status_machine_cache(root)") <
      lanes.indexOf("read_json_receipt_machine_alias("),
    "typed cache must be attempted before the JSON document alias fallback",
  );
  assert.match(
    lanes,
    /package_lane_visibility_by_id:\s*std::collections::BTreeMap<String,\s*usize>/,
  );
  assert.match(lanes, /from_visibility_entries\(package_lane_visibility/);

  assert.match(
    cli,
    /write_forge_package_status_machine_cache_with_performance_receipt\(\s*project_dir,\s*&package_status/,
  );
  assert.match(cli, /write_json_receipt_machine_alias_best_effort\(/);

  assert.match(gitignore, /^\.dx\/www\/\*\.machine$/m);
  assert.match(gitignore, /^\.dx\/www\/\*\.machine\.meta\.json$/m);
  assert.match(gitignore, /^\.dx\/performance\/$/m);
});
