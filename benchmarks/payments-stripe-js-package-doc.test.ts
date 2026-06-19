const assert = require("node:assert");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  const filePath = path.join(root, relativePath);
  assert.ok(fs.existsSync(filePath), `missing ${relativePath}`);
  return fs.readFileSync(filePath, "utf8");
}

test("Payments package docs and metadata expose official lane naming", () => {
  const docs = read("docs/packages/payments-stripe-js.md");
  const catalog = read("examples/template/package-catalog.ts");
  const forge = read("core/src/ecosystem/forge_stripe_js.rs");
  const studioManifest = read("dx-www/src/cli/studio_manifest.rs");
  const receipt = JSON.parse(
    read(
      "examples/template/.dx/forge/receipts/2026-05-22-payments-stripe-js-billing-workflow.json",
    ),
  );

  assert.match(docs, /^# Payments/m);
  assert.match(docs, /Official DX package: Payments/);
  assert.match(docs, /package_id: payments\/stripe-js/);
  assert.match(docs, /upstream_package: @stripe\/stripe-js/);
  assert.match(docs, /source_mirror: G:\/WWW\/inspirations\/stripe-js/);
  assert.match(docs, /upstream_version: 9\.6\.0/);
  assert.match(docs, /honesty_label: ADAPTER-BOUNDARY/);
  assert.match(docs, /src\/pure\.ts/);
  assert.match(docs, /types\/shared\.d\.ts/);
  assert.match(docs, /types\/stripe-js\/stripe\.d\.ts/);
  assert.match(docs, /types\/stripe-js\/checkout\.d\.ts/);
  assert.match(docs, /loadStripe/);
  assert.match(docs, /loadStripe\.setLoadParameters/);
  assert.match(docs, /stripe\.confirmPayment/);
  assert.match(docs, /stripe\.retrievePaymentIntent/);
  assert.match(docs, /stripe\.createEmbeddedCheckoutPage/);
  assert.match(docs, /StripeEmbeddedCheckoutOptions\.fetchClientSecret/);
  assert.match(docs, /Checkout Sessions/);
  assert.match(docs, /Payment Element/);
  assert.match(docs, /Billing Portal/);
  assert.match(docs, /data-dx-component="launch-billing-checkout-workflow"/);
  assert.match(docs, /data-dx-stripe-action="request-checkout-intent"/);
  assert.match(docs, /data-dx-stripe-receipt-path/);
  assert.match(docs, /present/);
  assert.match(docs, /stale/);
  assert.match(docs, /missing receipt/);
  assert.match(docs, /blocked/);
  assert.match(docs, /unsupported surface/);
  assert.doesNotMatch(docs, /^# Stripe/m);

  assert.match(
    catalog,
    /packageId: "payments\/stripe-js",[\s\S]*?sourceMirror: "G:\/WWW\/inspirations\/stripe-js"/,
  );
  assert.match(
    catalog,
    /packageId: "payments\/stripe-js",[\s\S]*?"docs\/packages\/payments-stripe-js\.md"/,
  );
  assert.match(catalog, /"payments\/stripe-js": \{\s*name: "Payments"/);
  assert.doesNotMatch(catalog, /name: "Stripe Billing Workflow"/);

  assert.match(forge, /officialPackageName: "Payments"/);
  assert.match(forge, /sourceMirror: "G:\/WWW\/inspirations\/stripe-js"/);
  assert.match(forge, /inspectedSourceFiles: \[/);
  assert.match(forge, /"src\/pure\.ts"/);
  assert.match(forge, /"types\/stripe-js\/checkout\.d\.ts"/);
  assert.match(forge, /honestyLabel: "ADAPTER-BOUNDARY"/);

  assert.match(studioManifest, /"front_facing_name": "Payments Billing Workflow"/);
  assert.doesNotMatch(studioManifest, /"front_facing_name": "Stripe Billing Workflow"/);

  assert.equal(receipt.package_name, "Payments");
  assert.equal(receipt.package_id, "payments/stripe-js");
  assert.equal(receipt.upstream_package, "@stripe/stripe-js");
  assert.equal(receipt.source_mirror, "G:/WWW/inspirations/stripe-js");
  assert.equal(receipt.honesty_label, "ADAPTER-BOUNDARY");
  assert.ok(receipt.selected_surfaces.includes("launch-billing-checkout-workflow"));
  assert.equal(
    receipt.dx_check_visibility.schema,
    "dx.forge.package.dx_check_visibility",
  );
  assert.deepEqual(
    receipt.dx_check_visibility.status_legend.map((entry) => entry.status),
    ["present", "stale", "missing-receipt", "blocked", "unsupported-surface"],
  );
  assert.ok(receipt.source_files.includes("docs/packages/payments-stripe-js.md"));
  assert.equal(receipt.runtime_execution, false);
});
