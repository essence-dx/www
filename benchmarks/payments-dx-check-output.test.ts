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

test("Payments dx-check output is wired into the Rust forge section", () => {
  const projectCheck = read("core/src/ecosystem/project_check.rs");
  const paymentsDxCheckPath = path.join(
    root,
    "core",
    "src",
    "ecosystem",
    "project_check",
    "payments_dx_check.rs",
  );

  assert.ok(fs.existsSync(paymentsDxCheckPath), "missing Payments dx-check module");

  const paymentsDxCheck = fs.readFileSync(paymentsDxCheckPath, "utf8");

  for (const marker of [
    "mod payments_dx_check;",
    "use payments_dx_check::forge_payments_package_metrics;",
    "forge_payments_package_metrics(root, &manifest)",
    "metrics.extend(payments_metrics);",
    "findings.extend(payments_findings);",
  ]) {
    assertContains(projectCheck, marker, `missing project_check marker ${marker}`);
  }

  for (const marker of [
    'PAYMENTS_PACKAGE_ID: &str = "payments/stripe-js"',
    'PAYMENTS_OFFICIAL_NAME: &str = "Payments"',
    'PAYMENTS_PACKAGE_STATUS: &str = ".dx/forge/package-status.json"',
    "PAYMENTS_BILLING_WORKFLOW_RECEIPT",
    '"examples/template/.dx/forge/receipts/2026-05-22-payments-stripe-js-billing-workflow.json"',
    "pub(super) fn forge_payments_package_metrics",
    "json_array_entries(&package_status, &[\"package_lane_visibility\"])",
    "package_receipt_exists(root, package_receipt_path)",
    'check_metric("payments_package_present", package_present)',
    'check_metric("payments_receipt_present", receipt_present)',
    'check_metric("payments_receipt_stale", stale_receipt)',
    'check_metric("payments_missing_receipt", missing_receipt)',
    'check_metric("payments_blocked_surface", blocked_surfaces)',
    'check_metric("payments_unsupported_surface", unsupported_surfaces)',
    '"payments_receipt_hash_refresh_current"',
    '"payments_receipt_hash_refresh_stale"',
    '"payments_receipt_hash_refresh_missing"',
    "receipt_hash_refresh_counts(visibility)",
    '"payments-missing-package-status"',
    '"payments-stale-receipt"',
    '"payments-missing-receipt"',
    '"payments-blocked-surface"',
    '"payments-unsupported-surface"',
    "payments_package_metrics_reports_missing_dx_style_compatibility",
    "payments_hash_refresh_stale_helper_keeps_source_hash_clean",
    "ADAPTER-BOUNDARY",
    "Stripe credentials",
    "Checkout",
    "app-owned",
  ]) {
    assertContains(paymentsDxCheck, marker, `missing module marker ${marker}`);
  }
});

test("Payments docs record Rust dx-check consumption", () => {
  const packageDoc = read("docs/packages/payments-stripe-js.md");

  for (const marker of [
    "Rust dx-check output",
    "`core/src/ecosystem/project_check/payments_dx_check.rs`",
    "`payments_*`",
    "`payments-stale-receipt`",
    "`payments-missing-receipt`",
    "`payments_receipt_hash_refresh_current`",
    "`payments_receipt_hash_refresh_stale`",
    "`payments_receipt_hash_refresh_missing`",
    "cargo test -q -p dx-www-compiler payments_package_metrics_reports_missing_dx_style_compatibility --lib",
    "cargo test -q -p dx-www-compiler payments_hash_refresh_stale_helper_keeps_source_hash_clean --lib",
    "without claiming live Stripe payment execution",
  ]) {
    assertContains(packageDoc, marker, `missing package doc marker ${marker}`);
  }
});
