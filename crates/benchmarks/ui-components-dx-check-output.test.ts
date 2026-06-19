const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function assertContains(text, marker, message) {
  assert.match(
    text,
    new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
    message || `missing marker ${marker}`,
  );
}

test("UI Components dx-check output is wired into the Rust forge section", () => {
  const projectCheck = read("core/src/ecosystem/project_check.rs");
  const uiComponentsDxCheckPath = path.join(
    root,
    "core",
    "src",
    "ecosystem",
    "project_check",
    "ui_components_dx_check.rs",
  );

  assert.ok(
    fs.existsSync(uiComponentsDxCheckPath),
    "missing UI Components dx-check module",
  );

  const uiComponentsDxCheck = fs.readFileSync(uiComponentsDxCheckPath, "utf8");

  for (const marker of [
    "mod ui_components_dx_check;",
    "use ui_components_dx_check::forge_ui_components_package_metrics;",
    "forge_ui_components_package_metrics(root, &manifest)",
    "metrics.extend(ui_components_metrics);",
    "findings.extend(ui_components_findings);",
  ]) {
    assertContains(projectCheck, marker, `missing project_check marker ${marker}`);
  }

  for (const marker of [
    'UI_COMPONENTS_PACKAGE_ID: &str = "shadcn/ui/button"',
    'UI_COMPONENTS_OFFICIAL_NAME: &str = "UI Components"',
    'UI_COMPONENTS_PACKAGE_STATUS: &str = ".dx/forge/package-status.json"',
    "UI_COMPONENTS_DASHBOARD_RECEIPT",
    '"examples/template/.dx/forge/receipts/2026-05-22-shadcn-dashboard-controls.json"',
    "pub(super) fn forge_ui_components_package_metrics",
    "count_sha256_file_hash_mismatches(root, surface)",
    "json_array_entries(&package_status, &[\"package_lane_visibility\"])",
    "package_receipt_exists(root, package_receipt_path)",
    'check_metric("ui_components_package_present", package_present)',
    'check_metric("ui_components_receipt_present", receipt_present)',
    'check_metric("ui_components_receipt_stale", stale_receipt)',
    'check_metric("ui_components_missing_receipt", missing_receipt)',
    'check_metric("ui_components_blocked_surface", blocked_surfaces)',
    'check_metric("ui_components_unsupported_surface", unsupported_surfaces)',
    'check_metric("ui_components_hash_manifest_present", hash_manifest_present)',
    'check_metric("ui_components_hash_mismatch", hash_mismatches)',
    "receipt_hash_refresh_counts(visibility)",
    '"ui_components_receipt_hash_refresh_current"',
    '"ui_components_receipt_hash_refresh_stale"',
    '"ui_components_receipt_hash_refresh_missing"',
    "receipt_hash_refresh_stale > 0",
    '"ui-components-missing-package-status"',
    '"ui-components-stale-receipt"',
    '"ui-components-missing-receipt"',
    '"ui-components-blocked-surface"',
    '"ui-components-unsupported-surface"',
    '"ui-components-hash-mismatch"',
    "SOURCE-ONLY",
    "shadcn-ui",
    "Radix",
    "browser UI runtime proof",
  ]) {
    assertContains(uiComponentsDxCheck, marker, `missing module marker ${marker}`);
  }
});

test("UI Components docs record Rust dx-check consumption", () => {
  const packageDoc = read("docs/packages/ui-components.md");

  for (const marker of [
    "Rust dx-check output",
    "`core/src/ecosystem/project_check/ui_components_dx_check.rs`",
    "`ui_components_*`",
    "`ui-components-stale-receipt`",
    "`ui-components-missing-receipt`",
    "`ui-components-hash-mismatch`",
    "`ui_components_hash_manifest_present`",
    "`ui_components_receipt_hash_refresh_current`",
    "`ui_components_receipt_hash_refresh_stale`",
    "`ui_components_receipt_hash_refresh_missing`",
    "without claiming browser UI runtime proof",
  ]) {
    assertContains(packageDoc, marker, `missing package doc marker ${marker}`);
  }
});

test("UI Components run notes record Rust dx-check status honestly", () => {
  for (const sourcePath of ["DX.md", "TODO.md", "CHANGELOG.md"]) {
    const source = read(sourcePath);

    for (const marker of [
      "UI Components",
      "Rust dx-check output",
      "ui_components_package_present",
      "ui_components_hash_manifest_present",
      "ui_components_receipt_hash_refresh_current",
      "ui_components_receipt_hash_refresh_stale",
      "ui_components_receipt_hash_refresh_missing",
      "ui-components-missing-receipt",
      "ui-components-hash-mismatch",
      "browser UI runtime proof",
    ]) {
      assertContains(source, marker, `missing ${sourcePath} marker ${marker}`);
    }
  }
});
