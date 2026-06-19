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

test("Payments package docs and metadata use official DX naming with Stripe.js as provenance", () => {
  const docs = read("docs/packages/payments-stripe-js.md");
  const catalog = read("examples/template/package-catalog.ts");
  const forge = read("core/src/ecosystem/forge_stripe_js.rs");
  const cli = read("dx-www/src/cli/mod.rs");
  const studioManifest = read("dx-www/src/cli/studio_manifest.rs");

  assert.match(docs, /^# Payments/m);
  assert.match(docs, /official_package_name: Payments/);
  assert.match(docs, /package_id: payments\/stripe-js/);
  assert.match(docs, /upstream_package: @stripe\/stripe-js/);
  assert.match(docs, /source_mirror: G:\/WWW\/inspirations\/stripe-js/);
  assert.match(docs, /upstream_version: 9\.6\.0/);
  assert.match(docs, /honesty_label: ADAPTER-BOUNDARY/);
  assert.match(docs, /src\/pure\.ts/);
  assert.match(docs, /src\/shared\.ts/);
  assert.match(docs, /types\/stripe-js\/stripe\.d\.ts/);
  assert.match(docs, /types\/stripe-js\/checkout\.d\.ts/);
  assert.match(docs, /loadStripe/);
  assert.match(docs, /loadStripe\.setLoadParameters/);
  assert.match(docs, /stripe\.confirmPayment/);
  assert.match(docs, /stripe\.retrievePaymentIntent/);
  assert.match(docs, /stripe\.createEmbeddedCheckoutPage/);
  assert.match(docs, /StripeEmbeddedCheckoutOptions\.fetchClientSecret/);
  assert.match(docs, /data-dx-component="launch-billing-checkout-workflow"/);
  assert.match(docs, /data-dx-component="dashboard-stripe-plan-checkout"/);
  assert.match(docs, /present/);
  assert.match(docs, /stale/);
  assert.match(docs, /missing receipt/);
  assert.match(docs, /blocked/);
  assert.match(docs, /unsupported surface/);
  assert.doesNotMatch(docs, /^# Stripe/m);

  assert.match(
    catalog,
    /packageId: "payments\/stripe-js",[\s\S]*?officialName: "Payments"/,
  );
  assert.match(
    catalog,
    /packageId: "payments\/stripe-js",[\s\S]*?upstreamPackage: "@stripe\/stripe-js"/,
  );
  assert.match(
    catalog,
    /packageId: "payments\/stripe-js",[\s\S]*?upstreamVersion: "9\.6\.0"/,
  );
  assert.match(
    catalog,
    /packageId: "payments\/stripe-js",[\s\S]*?"docs\/packages\/payments-stripe-js\.md"/,
  );
  assert.match(catalog, /"payments\/stripe-js": \{\s*name: "Payments"/);

  assert.match(forge, /officialPackageName: "Payments"/);
  assert.match(forge, /upstreamPackage: "@stripe\/stripe-js"/);
  assert.match(forge, /sourceMirror: "G:\/WWW\/inspirations\/stripe-js"/);
  assert.match(forge, /docsPath: "docs\/packages\/payments-stripe-js\.md"/);
  assert.match(forge, /honestyLabel: "ADAPTER-BOUNDARY"/);
  assert.match(forge, /inspectedSourceFiles: \[/);
  assert.match(forge, /"src\/pure\.ts"/);
  assert.match(forge, /"src\/shared\.ts"/);
  assert.match(forge, /surfaces: \[/);
  assert.match(forge, /"launch-billing-checkout-workflow"/);
  assert.match(forge, /"dashboard-stripe-plan-checkout"/);

  assert.match(cli, /"official_package_name": "Payments"/);
  assert.match(cli, /"upstream_package": "@stripe\/stripe-js"/);
  assert.match(cli, /"docs\/packages\/payments-stripe-js\.md"/);
  assert.match(studioManifest, /"front_facing_name": "Payments Billing Workflow"/);
  assert.doesNotMatch(studioManifest, /"front_facing_name": "Stripe Billing Workflow"/);
});
