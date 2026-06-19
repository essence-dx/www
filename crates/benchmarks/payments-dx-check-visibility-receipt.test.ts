const assert = require("node:assert");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const receiptPath =
  "examples/template/.dx/forge/receipts/2026-05-22-payments-stripe-js-billing-workflow.json";
const visibilitySchema = "dx.forge.package.dx_check_visibility";
const statusLegend = [
  "present",
  "stale",
  "missing-receipt",
  "blocked",
  "unsupported-surface",
];

function read(relativePath) {
  const filePath = path.join(root, relativePath);
  assert.ok(fs.existsSync(filePath), `missing ${relativePath}`);
  return fs.readFileSync(filePath, "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

test("Payments receipt exposes structured dx-check visibility states", () => {
  const receipt = readJson(receiptPath);
  const forge = read("core/src/ecosystem/forge_stripe_js.rs");
  const catalog = read("examples/template/package-catalog.ts");
  const docs = read("docs/packages/payments-stripe-js.md");

  assert.equal(receipt.package_id, "payments/stripe-js");
  assert.equal(receipt.package_name, "Payments");
  assert.equal(receipt.upstream_package, "@stripe/stripe-js");
  assert.equal(receipt.upstream_version, "9.6.0");
  assert.equal(receipt.honesty_label, "ADAPTER-BOUNDARY");

  assert.equal(receipt.dx_check_visibility.schema, visibilitySchema);
  assert.equal(receipt.dx_check_visibility.package_id, "payments/stripe-js");
  assert.equal(receipt.dx_check_visibility.official_package_name, "Payments");
  assert.equal(receipt.dx_check_visibility.current_status, "present");
  assert.deepEqual(
    receipt.dx_check_visibility.status_legend.map((entry) => entry.status),
    statusLegend,
  );
  assert.ok(
    receipt.dx_check_visibility.monitored_surfaces.some(
      (surface) =>
        surface.id === "payments-launch-billing-checkout-workflow" &&
        surface.status === "present" &&
        surface.source_file === "examples/template/payments-status.tsx" &&
        surface.materialized_file === "components/template-app/payments-status.tsx" &&
        surface.receipt_path === receiptPath,
    ),
  );
  assert.ok(
    receipt.dx_check_visibility.monitored_surfaces.some(
      (surface) =>
        surface.id === "payments-checkout-session-route" &&
        surface.status === "present" &&
        surface.source_file === "core/src/ecosystem/forge_stripe_js.rs" &&
        surface.materialized_file === "app/api/checkout/route.ts" &&
        surface.receipt_path === receiptPath,
    ),
  );
  assert.ok(
    receipt.dx_check_visibility.monitored_surfaces.some(
      (surface) =>
        surface.id === "payments-webhook-route" &&
        surface.status === "present" &&
        surface.materialized_file === "app/api/stripe/webhook/route.ts",
    ),
  );

  assert.match(forge, /dxCheckVisibility: \{/);
  assert.match(forge, /schema: "dx\.forge\.package\.dx_check_visibility"/);
  assert.match(forge, /currentStatus: "present"/);
  assert.match(forge, /"missing-receipt"/);
  assert.match(forge, /"unsupported-surface"/);
  assert.match(forge, /"payments-launch-billing-checkout-workflow"/);
  assert.match(forge, /"payments-checkout-session-route"/);

  assert.match(catalog, /packageId: "payments\/stripe-js"[\s\S]*?dxCheckVisibility: \{/);
  assert.match(catalog, /schema: "dx\.forge\.package\.dx_check_visibility"/);
  assert.match(catalog, /receiptPath:\s*"examples\/template\/\.dx\/forge\/receipts\/2026-05-22-payments-stripe-js-billing-workflow\.json"/);
  assert.match(catalog, /"payments-launch-billing-checkout-workflow"/);
  assert.match(catalog, /"payments-checkout-session-route"/);
  assert.match(catalog, /"payments-webhook-route"/);

  assert.match(docs, /Schema: `dx\.forge\.package\.dx_check_visibility`/);
  assert.match(docs, /present, stale, missing-receipt, blocked, and unsupported-surface/);
});
