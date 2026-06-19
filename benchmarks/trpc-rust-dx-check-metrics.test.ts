const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("Type-Safe API package-status metrics are wired into Rust dx-check", () => {
  const projectCheck = read("core/src/ecosystem/project_check.rs");
  const typeSafeApiCheck = read(
    "core/src/ecosystem/project_check/type_safe_api_dx_check.rs",
  );
  const dxCheckReceipt = read("core/src/ecosystem/dx_check_receipt.rs");
  const packageDoc = read("docs/packages/api-trpc.md");

  assert.match(projectCheck, /mod type_safe_api_dx_check;/);
  assert.match(
    projectCheck,
    /use type_safe_api_dx_check::forge_type_safe_api_package_metrics;/,
  );
  assert.match(
    projectCheck,
    /forge_type_safe_api_package_metrics\(root, &manifest\)/,
  );
  assert.match(
    projectCheck,
    /fn dx_check_reports_type_safe_api_package_status_visibility\(\)/,
  );

  for (const expected of [
    'const TYPE_SAFE_API_PACKAGE_ID: &str = "api/trpc"',
    'const TYPE_SAFE_API_OFFICIAL_NAME: &str = "Type-Safe API"',
    'const TYPE_SAFE_API_DASHBOARD_RECEIPT: &str =',
    "type_safe_api_package_present",
    "type_safe_api_receipt_present",
    "type_safe_api_receipt_stale",
    "type_safe_api_missing_receipt",
    "type_safe_api_blocked_surface",
    "type_safe_api_unsupported_surface",
    "type_safe_api_hash_manifest_present",
    "type_safe_api_hash_mismatch",
    "type-safe-api-missing-package-status",
    "type-safe-api-missing-receipt",
    "type-safe-api-stale-receipt",
    "type-safe-api-blocked-surface",
    "type-safe-api-unsupported-surface",
    "type-safe-api-hash-mismatch",
    "use super::file_hashes::count_sha256_file_hash_mismatches;",
    "count_sha256_file_hash_mismatches(root, surface)",
    "hash_manifest_present",
    "hash_mismatches",
    "package_receipt_exists",
    "type_safe_api_hash_mismatch_flips_when_selected_file_changes",
    'strip_prefix("examples/template/")',
  ]) {
    assert.ok(
      typeSafeApiCheck.includes(expected),
      `${expected} missing from Type-Safe API dx-check module`,
    );
  }

  assert.doesNotMatch(typeSafeApiCheck, /fn count_hash_mismatches\(/);
  assert.match(
    dxCheckReceipt,
    /fn dx_check_latest_panel_exposes_type_safe_api_package_lane_hash_refresh_row\(\)/,
  );
  assert.match(dxCheckReceipt, /type_safe_api_receipt_hash_refresh_stale/);
  assert.match(dxCheckReceipt, /type-safe-api:receipt-hash-refresh/);
  assert.match(
    dxCheckReceipt,
    /type_safe_api\["receipt_hash_refresh"\]\["current_files"\]/,
  );
  assert.match(
    dxCheckReceipt,
    /stale_type_safe_api\["receipt_hash_refresh"\]\["stale_files"\]/,
  );
  assert.match(packageDoc, /Rust `dx check` now consumes this shared row/);
  assert.match(packageDoc, /hash_algorithm: sha256/);
  assert.match(packageDoc, /type_safe_api_hash_mismatch/);
  assert.match(packageDoc, /type_safe_api_receipt_hash_refresh_stale/);
  assert.match(packageDoc, /type-safe-api-missing-receipt/);
  assert.match(packageDoc, /cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_type_safe_api_package_lane_hash_refresh_row --lib/);
});
