const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function escaped(marker) {
  return new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"));
}

test("Authentication dx-check output is wired into the Rust forge section", () => {
  const projectCheck = read("core/src/ecosystem/project_check.rs");
  const authDxCheckPath = path.join(
    root,
    "core",
    "src",
    "ecosystem",
    "project_check",
    "authentication_dx_check.rs",
  );

  assert.ok(fs.existsSync(authDxCheckPath), "missing Authentication dx-check module");

  const authDxCheck = fs.readFileSync(authDxCheckPath, "utf8");

  for (const marker of [
    "mod authentication_dx_check;",
    "use authentication_dx_check::forge_authentication_package_metrics;",
    "forge_authentication_package_metrics(root, &manifest)",
    "metrics.extend(authentication_metrics);",
    "findings.extend(authentication_findings);",
  ]) {
    assert.match(projectCheck, escaped(marker), `missing project_check marker ${marker}`);
  }

  for (const marker of [
    'AUTHENTICATION_PACKAGE_ID: &str = "auth/better-auth"',
    'AUTHENTICATION_OFFICIAL_NAME: &str = "Authentication"',
    'AUTHENTICATION_PACKAGE_STATUS: &str = ".dx/forge/package-status.json"',
    'AUTHENTICATION_PACKAGE_RECEIPT: &str = ".dx/forge/receipts/auth-better-auth.json"',
    "use super::file_hashes::count_sha256_file_hash_mismatches;",
    "pub(super) fn forge_authentication_package_metrics",
    "json_array_entries(&package_status, &[\"package_lane_visibility\"])",
    "package_receipt_exists(root, package_receipt_path)",
    'check_metric("authentication_package_present", package_present)',
    'check_metric("authentication_receipt_present", receipt_present)',
    'check_metric("authentication_receipt_stale", stale_receipt)',
    'check_metric("authentication_missing_receipt", missing_receipt)',
    'check_metric("authentication_blocked_surface", blocked_surfaces)',
    'check_metric("authentication_unsupported_surface", unsupported_surfaces)',
    '"authentication_hash_manifest_present"',
    "hash_manifest_present",
    'check_metric("authentication_hash_mismatch", surface_hash_mismatches)',
    "authentication_dx_style_compatibility_present",
    "authentication_dx_style_compatibility_missing",
    "authentication_receipt_hash_refresh_current",
    "authentication_receipt_hash_refresh_stale",
    "authentication_receipt_hash_refresh_missing",
    "let (refresh_current, refresh_stale, refresh_missing) = receipt_hash_refresh_counts(visibility);",
    "stale_files",
    "stale_mirror_files",
    "missing_files",
    "missing_mirror_files",
    "authentication_package_metrics_reports_helper_freshness_from_path_arrays",
    'json_text(surface, &["hash_algorithm"]) == Some("sha256")',
    "count_sha256_file_hash_mismatches(root, surface)",
    "dx_style_compatibility_is_present",
    "authentication_package_metrics_reports_missing_dx_style_compatibility",
    "write_authentication_package_status(dir.path(), false)",
    "write_authentication_package_status(dir.path(), true)",
    '"authentication-missing-package-status"',
    '"authentication-stale-receipt"',
    '"authentication-missing-receipt"',
    '"authentication-blocked-surface"',
    '"authentication-unsupported-surface"',
    '"authentication-hash-mismatch"',
    '"authentication-missing-dx-style-compatibility"',
    "Hash the selected Authentication files",
    "data-dx-style-surface",
    "ADAPTER-BOUNDARY",
  ]) {
    assert.match(authDxCheck, escaped(marker), `missing module marker ${marker}`);
  }
});

test("Authentication docs record Rust dx-check consumption", () => {
  const packageDoc = read("docs/packages/authentication.md");

  for (const marker of [
    "Rust dx-check output",
    "`core/src/ecosystem/project_check/authentication_dx_check.rs`",
    "`authentication_*`",
    "`authentication-stale-receipt`",
    "`authentication-missing-receipt`",
    "`authentication_hash_manifest_present`",
    "`authentication_hash_mismatch`",
    "`authentication-hash-mismatch`",
    "`authentication_receipt_hash_refresh_current`",
    "`authentication_receipt_hash_refresh_stale`",
    "`authentication_receipt_hash_refresh_missing`",
    "`authentication_dx_style_compatibility_present`",
    "`authentication_dx_style_compatibility_missing`",
    "`receipt_hash_refresh.stale_files`",
    "`authentication-missing-dx-style-compatibility`",
    "`authentication_package_metrics_reports_missing_dx_style_compatibility`",
    "`authentication_package_metrics_reports_helper_freshness_from_path_arrays`",
    "`data-dx-style-surface=\"authentication-session-status\"`",
    "`hash_algorithm: sha256`",
    "`project_check/file_hashes.rs`",
    "without claiming live OAuth or session runtime proof",
  ]) {
    assert.match(packageDoc, escaped(marker), `missing package doc marker ${marker}`);
  }
});
