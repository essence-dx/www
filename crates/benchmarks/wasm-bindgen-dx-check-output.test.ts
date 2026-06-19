const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("WebAssembly Bridge dx-check output is wired into the Rust forge section", () => {
  const projectCheck = read("core/src/ecosystem/project_check.rs");
  const wasmDxCheckPath = path.join(
    root,
    "core",
    "src",
    "ecosystem",
    "project_check",
    "wasm_bindgen_dx_check.rs",
  );

  assert.ok(fs.existsSync(wasmDxCheckPath), "missing WebAssembly Bridge dx-check module");

  const wasmDxCheck = fs.readFileSync(wasmDxCheckPath, "utf8");

  for (const marker of [
    "mod wasm_bindgen_dx_check;",
    "use wasm_bindgen_dx_check::forge_webassembly_bridge_package_metrics;",
    "forge_webassembly_bridge_package_metrics(root, &manifest)",
    "metrics.extend(webassembly_bridge_metrics);",
    "findings.extend(webassembly_bridge_findings);",
  ]) {
    assert.match(projectCheck, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")), `missing project_check marker ${marker}`);
  }

  for (const marker of [
    'WEBASSEMBLY_BRIDGE_PACKAGE_ID: &str = "wasm/bindgen"',
    'WEBASSEMBLY_BRIDGE_OFFICIAL_NAME: &str = "WebAssembly Bridge"',
    'WEBASSEMBLY_BRIDGE_PACKAGE_STATUS: &str = ".dx/forge/package-status.json"',
    "WEBASSEMBLY_BRIDGE_DASHBOARD_RECEIPT",
    '".dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json"',
    "pub(super) fn forge_webassembly_bridge_package_metrics",
    "json_array_entries(&package_status, &[\"package_lane_visibility\"])",
    "package_receipt_exists(root, package_receipt_path)",
    'check_metric("webassembly_bridge_package_present", package_present)',
    'check_metric("webassembly_bridge_receipt_present", receipt_present)',
    'check_metric("webassembly_bridge_receipt_stale", stale_receipt)',
    'check_metric("webassembly_bridge_missing_receipt", missing_receipt)',
    'check_metric("webassembly_bridge_blocked_surface", blocked_surfaces)',
    '"webassembly_bridge_unsupported_surface"',
    '"webassembly_bridge_receipt_hash_refresh_current"',
    '"webassembly_bridge_receipt_hash_refresh_stale"',
    '"webassembly_bridge_receipt_hash_refresh_missing"',
    "receipt_hash_refresh_counts(visibility)",
    'json_string_array(refresh, "stale_files")',
    'json_string_array(refresh, "missing_files")',
    'json_string_array(refresh, "stale_mirror_files")',
    'json_string_array(refresh, "missing_mirror_files")',
    '"webassembly-bridge-missing-package-status"',
    '"webassembly-bridge-stale-receipt"',
    '"webassembly-bridge-missing-receipt"',
    '"webassembly-bridge-blocked-surface"',
    '"webassembly-bridge-unsupported-surface"',
    "webassembly_bridge_package_metrics_reports_helper_freshness_from_path_arrays",
    "ADAPTER-BOUNDARY",
  ]) {
    assert.match(wasmDxCheck, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")), `missing module marker ${marker}`);
  }
});

test("WebAssembly Bridge docs record Rust dx-check consumption", () => {
  const packageDoc = read("docs/packages/wasm-bindgen.md");

  for (const marker of [
    "Rust dx-check output",
    "`core/src/ecosystem/project_check/wasm_bindgen_dx_check.rs`",
    "`webassembly_bridge_*`",
    "`webassembly-bridge-stale-receipt`",
    "`webassembly-bridge-missing-receipt`",
    "`webassembly_bridge_receipt_hash_refresh_current`",
    "`webassembly_bridge_receipt_hash_refresh_stale`",
    "`webassembly_bridge_receipt_hash_refresh_missing`",
    "`receipt_hash_refresh.stale_files[]`",
    "`receipt_hash_refresh.missing_files[]`",
    "without claiming browser execution proof",
  ]) {
    assert.match(packageDoc, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")), `missing package doc marker ${marker}`);
  }
});
