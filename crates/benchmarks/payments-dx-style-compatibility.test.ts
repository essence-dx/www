const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const receiptPath =
  "examples/template/.dx/forge/receipts/2026-05-22-payments-stripe-js-billing-workflow.json";

function read(relativePath) {
  const filePath = path.join(root, relativePath);
  assert.ok(fs.existsSync(filePath), `missing ${relativePath}`);
  return fs.readFileSync(filePath, "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function escaped(marker) {
  return new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"));
}

test("Payments launch billing workflow declares dx-style compatibility", () => {
  const paymentsStatus = read("examples/template/payments-status.tsx");
  const receipt = readJson(receiptPath);
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const readModel = read("examples/template/forge-package-status-read-model.ts");
  const paymentsDxCheck = read("core/src/ecosystem/project_check/payments_dx_check.rs");
  const packageDoc = read("docs/packages/payments-stripe-js.md");

  assert.match(paymentsStatus, /data-dx-style-surface="payments"/);
  assert.doesNotMatch(paymentsStatus, /style=\{\{/);
  assert.doesNotMatch(paymentsStatus, /#[0-9a-fA-F]{3,8}/);

  const compatibility = receipt.dx_style_compatibility;
  assert.equal(compatibility.schema, "dx.forge.package.dx_style_compatibility");
  assert.equal(compatibility.status, "present");
  assert.equal(compatibility.token_source, "styles/globals.css");
  assert.equal(compatibility.generated_css, "styles/globals.css");
  assert.equal(compatibility.runtime_proof, false);
  assert.ok(
    compatibility.visible_surfaces.includes(
      "launch-billing-checkout-workflow",
    ),
  );
  assert.ok(
    compatibility.source_files.includes(
      "examples/template/payments-status.tsx",
    ),
  );
  assert.ok(
    compatibility.data_dx_markers.includes(
      'data-dx-style-surface="payments"',
    ),
  );
  assert.ok(
    compatibility.style_boundaries.includes("no inline style objects"),
  );
  assert.match(
    compatibility.runtime_limitations.join(" "),
    /live Stripe Checkout visual proof/,
  );

  const paymentsVisibility = status.package_lane_visibility.find(
    (entry) => entry.package_id === "payments/stripe-js",
  );
  assert.ok(paymentsVisibility, "Payments visibility row is missing");
  assert.equal(
    paymentsVisibility.dx_style_compatibility.schema,
    "dx.forge.package.dx_style_compatibility",
  );
  assert.equal(paymentsVisibility.dx_style_compatibility.status, "present");
  assert.equal(
    paymentsVisibility.dx_style_compatibility.token_source,
    "styles/globals.css",
  );
  assert.ok(
    paymentsVisibility.selected_surfaces
      .find(
        (surface) =>
          surface.surface_id === "payments-launch-billing-checkout-workflow",
      )
      .source_markers.includes('data-dx-style-surface="payments"'),
  );

  for (const metric of [
    "payments_dx_style_compatibility_present",
    "payments_dx_style_compatibility_missing",
  ]) {
    assert.ok(
      paymentsVisibility.dx_check_metrics.includes(metric),
      `${metric} missing from Payments visibility row`,
    );
    assert.ok(
      status.dx_check_metrics.includes(metric),
      `${metric} missing from package-status dx_check_metrics`,
    );
    assert.match(readModel, escaped(metric));
    assert.match(paymentsDxCheck, escaped(metric));
  }

  for (const marker of [
    "dxStyleCompatibility: {",
    "visibleSurfaces: [\"launch-billing-checkout-workflow\"]",
    "data-dx-style-surface=\"payments\"",
    "payments-missing-dx-style-compatibility",
    "dx_style_compatibility_is_present",
  ]) {
    assert.match(readModel + paymentsDxCheck, escaped(marker));
  }

  assert.match(packageDoc, /## DX-Style Compatibility/);
  assert.match(packageDoc, /payments_dx_style_compatibility_present/);
  assert.match(packageDoc, /payments-missing-dx-style-compatibility/);
});
