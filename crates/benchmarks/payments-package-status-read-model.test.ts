const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const { spawnSync } = require("node:child_process");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const receiptPath =
  "examples/template/.dx/forge/receipts/2026-05-22-payments-stripe-js-billing-workflow.json";
const statusVocabulary = [
  "present",
  "stale",
  "missing-receipt",
  "blocked",
  "unsupported-surface",
];

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

test("Payments receipt visibility is consumed by the shared package-status read model", () => {
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const receipt = readJson(receiptPath);
  const readModel = read(
    "examples/template/forge-package-status-read-model.ts",
  );
  const statusSource = read("examples/template/forge-package-status.ts");
  const packageDoc = read("docs/packages/payments-stripe-js.md");

  const paymentsVisibility = status.package_lane_visibility.find(
    (entry) => entry.package_id === "payments/stripe-js",
  );

  assert.ok(paymentsVisibility, "Payments visibility row is missing");
  assert.equal(paymentsVisibility.official_package_name, "Payments");
  assert.equal(paymentsVisibility.upstream_package, "@stripe/stripe-js");
  assert.equal(paymentsVisibility.upstream_version, "9.6.0");
  assert.equal(
    paymentsVisibility.source_mirror,
    "G:/WWW/inspirations/stripe-js",
  );
  assert.equal(paymentsVisibility.status, "present");
  assert.equal(paymentsVisibility.receipt_status, "present");
  assert.equal(paymentsVisibility.package_receipt_path, receiptPath);
  assert.deepEqual(paymentsVisibility.status_vocabulary, statusVocabulary);
  assert.deepEqual(
    receipt.dx_check_visibility.status_legend.map((entry) => entry.status),
    statusVocabulary,
  );

  for (const surfaceId of [
    "payments-launch-billing-checkout-workflow",
    "payments-checkout-session-route",
    "payments-webhook-route",
  ]) {
    assert.ok(
      paymentsVisibility.selected_surfaces.some(
        (surface) =>
          surface.surface_id === surfaceId &&
          surface.receipt_path === receiptPath,
      ),
      `${surfaceId} missing from Payments visibility row`,
    );
  }

  for (const marker of [
    'data-dx-package="payments/stripe-js"',
    'data-dx-component="launch-billing-checkout-workflow"',
    'data-dx-stripe-action="request-checkout-intent"',
    "app/api/checkout/route.ts",
    "app/api/stripe/webhook/route.ts",
  ]) {
    const markers = paymentsVisibility.selected_surfaces.flatMap((surface) =>
      surface.source_markers.concat(surface.files),
    );

    assert.ok(markers.includes(marker), `${marker} missing from Payments row`);
  }

  for (const metric of [
    "payments_receipt_present",
    "payments_receipt_stale",
    "payments_missing_receipt",
    "payments_blocked_surface",
    "payments_unsupported_surface",
    "payments_receipt_hash_refresh_current",
    "payments_receipt_hash_refresh_stale",
    "payments_receipt_hash_refresh_missing",
  ]) {
    assert.ok(
      paymentsVisibility.dx_check_metrics.includes(metric),
      `${metric} missing from Payments visibility row`,
    );
    assert.ok(
      status.dx_check_metrics.includes(metric),
      `${metric} missing from package-status dx_check_metrics`,
    );
    assert.match(readModel, new RegExp(metric));
  }

  const hashRefresh = paymentsVisibility.receipt_hash_refresh;
  assert.ok(hashRefresh, "Payments receipt hash refresh status is missing");
  assert.equal(hashRefresh.schema, "dx.forge.package.receipt_hash_refresh");
  assert.equal(hashRefresh.status, "current");
  assert.equal(
    hashRefresh.helper_path,
    "examples/template/payments-receipt-hashes.ts",
  );
  assert.equal(
    hashRefresh.check_command,
    "node tools/launch/run-template-receipt-helper.js examples/template/payments-receipt-hashes.ts --check",
  );
  assert.equal(
    hashRefresh.write_command,
    "node tools/launch/run-template-receipt-helper.js examples/template/payments-receipt-hashes.ts --write",
  );
  assert.equal(
    hashRefresh.json_check_command,
    "node tools/launch/run-template-receipt-helper.js examples/template/payments-receipt-hashes.ts --check --json",
  );
  assert.equal(
    hashRefresh.source_guard_runbook_fixture,
    "docs/packages/payments.source-guard-runbook.json",
  );
  assert.equal(
    hashRefresh.studio_manifest_source,
    "dx-www/src/cli/studio_manifest.rs",
  );
  assert.equal(hashRefresh.receipt_path, receiptPath);
  assert.equal(hashRefresh.hash_algorithm, "sha256");
  assert.equal(hashRefresh.tracked_file_count, Object.keys(receipt.file_hashes).length);
  assert.equal(hashRefresh.stale_file_count, 0);
  assert.equal(hashRefresh.missing_file_count, 0);
  assert.equal(hashRefresh.runtime_execution, false);
  assert.equal(hashRefresh.secret_access, false);
  assert.equal(hashRefresh.zed_visibility, "payments:receipt-hash-refresh");

  const helper = spawnSync(
    process.execPath,
    [
      "examples/template/payments-receipt-hashes.ts",
      "--check",
      "--json",
    ],
    {
      cwd: root,
      encoding: "utf8",
    },
  );
  assert.equal(helper.status, 0, helper.stdout + helper.stderr);
  const helperReport = JSON.parse(helper.stdout);
  assert.equal(helperReport.schema, hashRefresh.schema);
  assert.equal(helperReport.package_id, "payments/stripe-js");
  assert.equal(helperReport.official_package_name, "Payments");
  assert.equal(helperReport.status, hashRefresh.status);
  assert.equal(helperReport.tracked_file_count, hashRefresh.tracked_file_count);
  assert.equal(helperReport.stale_file_count, hashRefresh.stale_file_count);
  assert.equal(helperReport.missing_file_count, hashRefresh.missing_file_count);
  assert.equal(helperReport.runtime_execution, false);
  assert.equal(helperReport.secret_access, false);

  assert.match(readModel, /export const paymentsPackageVisibility/);
  assert.match(
    readModel,
    /packageLaneVisibility:\s*\[[\s\S]*paymentsPackageVisibility[\s\S]*\]/,
  );
  assert.match(readModel, /receiptHashRefresh/);
  assert.match(
    readModel,
    /studioManifestSource:\s*"dx-www\/src\/cli\/studio_manifest\.rs"/,
  );
  assert.match(readModel, /payments:receipt-hash-refresh/);
  assert.match(statusSource, /paymentsPackageVisibility/);
  assert.match(statusSource, /paymentsVisibility: paymentsPackageVisibility/);
  assert.ok(
    status.zed_receipt_surfaces.includes(
      "payments:launch-billing-checkout-workflow",
    ),
    "Payments Zed launch workflow receipt surface is missing",
  );
  assert.ok(
    status.zed_receipt_surfaces.includes("payments:checkout-session-route"),
    "Payments Zed checkout route receipt surface is missing",
  );
  assert.ok(
    status.zed_receipt_surfaces.includes("payments:receipt-hash-refresh"),
    "Payments Zed receipt hash refresh surface is missing",
  );
  assert.match(packageDoc, /shared package-status read model/i);
  assert.match(packageDoc, /receipt_hash_refresh/);
});
