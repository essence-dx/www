const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const helperPath = "core/src/ecosystem/project_check/reactive_store_dx_check.rs";

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function escaped(marker) {
  return new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"));
}

test("Reactive Store dx-check output is wired into the Rust forge section", () => {
  const projectCheck = read("core/src/ecosystem/project_check.rs");
  const readiness = read("core/src/ecosystem/project_check/readiness.rs");
  const forgeReadiness = read("core/src/ecosystem/project_check/readiness_parts/forge.rs");
  const reactiveStoreDxCheckPath = path.join(root, helperPath);
  const checkPanel = [
    read("core/src/ecosystem/dx_check_receipt/panel.rs"),
    read("core/src/ecosystem/dx_check_receipt/panel_parts/package_lanes.rs"),
    read("core/src/ecosystem/dx_check_receipt/panel_parts/package_metrics.rs"),
    read("core/src/ecosystem/dx_check_receipt/panel_parts/tests_a.rs"),
  ].join("\n");

  assert.ok(
    fs.existsSync(reactiveStoreDxCheckPath),
    "missing Reactive Store dx-check module",
  );

  const reactiveStoreDxCheck = fs.readFileSync(reactiveStoreDxCheckPath, "utf8");

  for (const marker of [
    "mod reactive_store_dx_check;",
    "mod file_hashes;",
  ]) {
    assert.match(projectCheck, escaped(marker), `missing project_check marker ${marker}`);
  }

  for (const marker of [
    "reactive_store_dx_check::forge_reactive_store_package_metrics",
  ]) {
    assert.match(readiness, escaped(marker), `missing readiness marker ${marker}`);
  }

  for (const marker of [
    "forge_reactive_store_package_metrics(root, &manifest)",
    "metrics.extend(reactive_store_metrics);",
    "findings.extend(reactive_store_findings);",
  ]) {
    assert.match(forgeReadiness, escaped(marker), `missing forge readiness marker ${marker}`);
  }

  for (const marker of [
    'REACTIVE_STORE_PACKAGE_ID: &str = "reactive/store"',
    'REACTIVE_STORE_OFFICIAL_NAME: &str = "Reactive Store"',
    'REACTIVE_STORE_PACKAGE_STATUS: &str = ".dx/forge/package-status.json"',
    'REACTIVE_STORE_PACKAGE_RECEIPT: &str = ".dx/forge/receipts/packages/reactive-store.json"',
    "pub(super) fn forge_reactive_store_package_metrics",
    "json_array_entries(&package_status, &[\"package_lane_visibility\"])",
    "package_receipt_exists(root, package_receipt_path)",
    "hash_manifest_present = 1;",
    "use super::file_hashes::count_sha256_file_hash_mismatches;",
    "hash_mismatches += count_sha256_file_hash_mismatches(root, surface);",
    "let (refresh_current, refresh_stale, refresh_missing) = receipt_hash_refresh_counts(visibility);",
    "reactive_store_hash_mismatch_flips_when_selected_file_changes",
    "reactive_store_hash_refresh_stale_helper_keeps_source_hash_clean",
    "write_reactive_store_package_status",
    "metric_value(&stale_metrics, \"reactive_store_hash_mismatch\")",
    "&helper_stale_metrics,",
    "metric_value(&helper_stale_metrics, \"reactive_store_hash_mismatch\")",
    'check_metric("reactive_store_package_present", package_present)',
    'check_metric("reactive_store_receipt_present", receipt_present)',
    'check_metric("reactive_store_receipt_stale", stale_receipt)',
    'check_metric("reactive_store_missing_receipt", missing_receipt)',
    'check_metric("reactive_store_blocked_surface", blocked_surfaces)',
    '"reactive_store_unsupported_surface"',
    '"reactive_store_hash_manifest_present"',
    '"reactive_store_hash_mismatch"',
    '"reactive_store_receipt_hash_refresh_current"',
    '"reactive_store_receipt_hash_refresh_stale"',
    '"reactive_store_receipt_hash_refresh_missing"',
    '"reactive-store-missing-package-status"',
    '"reactive-store-stale-receipt"',
    '"reactive-store-missing-receipt"',
    '"reactive-store-blocked-surface"',
    '"reactive-store-unsupported-surface"',
    '"reactive-store-hash-mismatch"',
    "SOURCE-ONLY",
    "ADAPTER-BOUNDARY",
  ]) {
    assert.match(reactiveStoreDxCheck, escaped(marker), `missing module marker ${marker}`);
  }

  assert.doesNotMatch(
    reactiveStoreDxCheck,
    /fn count_hash_mismatches\(/,
    "Reactive Store should use the shared SHA-256 hash comparator",
  );
  assert.doesNotMatch(
    reactiveStoreDxCheck,
    /fn sha256_project_file\(root: &Path/,
    "Reactive Store should not carry a lane-local root-relative hasher",
  );

  for (const marker of [
    'REACTIVE_STORE_PACKAGE_ID: &str = "reactive/store"',
    'REACTIVE_STORE_OFFICIAL_NAME: &str = "Reactive Store"',
    'REACTIVE_STORE_PACKAGE_RECEIPT_PATH: &str =',
    "rows.extend(reactive_store_package_lane_row(root, package_status));",
    "fn reactive_store_package_lane_row(",
    "fn reactive_store_selected_surfaces(",
    "fn reactive_store_metric_rows(",
    "fn count_sha256_file_hash_mismatches(root: &Path, value: &serde_json::Value)",
    '"reactive_store_hash_manifest_present"',
    '"reactive_store_hash_mismatch"',
    "dx_check_latest_panel_exposes_reactive_store_package_lane_hash_row",
  ]) {
    assert.match(checkPanel, escaped(marker), `missing check-panel marker ${marker}`);
  }
});

test("Reactive Store docs record Rust dx-check consumption", () => {
  const packageDoc = read("docs/packages/reactive-store.md");

  for (const marker of [
    "Rust dx-check output",
    "`core/src/ecosystem/project_check/reactive_store_dx_check.rs`",
    "`reactive_store_*`",
    "`reactive-store-stale-receipt`",
    "`reactive-store-missing-receipt`",
    "`reactive-store-hash-mismatch`",
    "shared byte-level SHA-256 helper",
    "`reactive_store_receipt_hash_refresh_current` / `reactive_store_receipt_hash_refresh_stale` / `reactive_store_receipt_hash_refresh_missing`",
    "`reactive_store_hash_refresh_stale_helper_keeps_source_hash_clean`",
    "dx-check Panel Row",
    "`check_panel.view_model.package_lane_rows`",
    "`reactive_store_hash_manifest_present` / `reactive_store_hash_mismatch`",
    "without claiming live React runtime proof",
  ]) {
    assert.match(packageDoc, escaped(marker), `missing package doc marker ${marker}`);
  }
});
