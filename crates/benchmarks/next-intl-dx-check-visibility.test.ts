const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const sourceRoot = "G:\\WWW\\inspirations\\next-intl";
const visibilitySchema = "dx.forge.package.dx_check_visibility";
const statuses = [
  "present",
  "stale",
  "missing-receipt",
  "blocked",
  "unsupported-surface",
];
const receiptPath =
  "examples/template/.dx/forge/receipts/2026-05-22-i18n-next-intl-dashboard-locale.json";

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function readMirror(relativePath) {
  return fs.readFileSync(path.join(sourceRoot, relativePath), "utf8");
}

test("Internationalization receipt exposes dx-check visibility states", () => {
  const upstreamPackage = JSON.parse(readMirror("packages/next-intl/package.json"));
  const routing = readMirror("packages/next-intl/src/routing/defineRouting.tsx");
  const navigation = readMirror(
    "packages/next-intl/src/navigation/react-client/createNavigation.tsx",
  );
  const requestConfig = readMirror(
    "packages/next-intl/src/server/react-server/getRequestConfig.tsx",
  );
  const middleware = readMirror("packages/next-intl/src/middleware/middleware.tsx");

  const receipt = readJson(receiptPath);
  const forge = read("core/src/ecosystem/forge_next_intl.rs");
  const catalog = read("examples/template/package-catalog.ts");
  const docs = read("docs/packages/next-intl.md");

  assert.equal(upstreamPackage.name, "next-intl");
  assert.equal(upstreamPackage.version, "4.12.0");
  assert.match(routing, /export default function defineRouting/);
  assert.match(navigation, /export default function createNavigation/);
  assert.match(requestConfig, /export default function getRequestConfig/);
  assert.match(middleware, /export default function createMiddleware/);

  assert.equal(receipt.package_id, "i18n/next-intl");
  assert.equal(receipt.package_name, "Internationalization");
  assert.equal(receipt.official_package_name, "Internationalization");
  assert.equal(receipt.upstream_package, "next-intl");
  assert.equal(receipt.upstream_version, "4.12.0");
  assert.equal(receipt.honesty_label, "SOURCE-ONLY");
  assert.ok(
    receipt.guards.includes(
      "dx run --test .\\benchmarks\\next-intl-dx-check-visibility.test.ts",
    ),
  );
  assert.equal(receipt.dx_check_visibility.schema, visibilitySchema);
  assert.equal(receipt.dx_check_visibility.package_id, "i18n/next-intl");
  assert.equal(
    receipt.dx_check_visibility.official_package_name,
    "Internationalization",
  );
  assert.equal(receipt.dx_check_visibility.current_status, "present");
  assert.deepEqual(
    receipt.dx_check_visibility.status_legend.map((entry) => entry.status),
    statuses,
  );
  assert.ok(
    receipt.dx_check_visibility.monitored_surfaces.some(
      (surface) =>
        surface.id === "next-intl-dashboard-locale-workflow" &&
        surface.status === "present" &&
        surface.source_file ===
          "examples/template/next-intl-dashboard-locale.tsx" &&
        surface.materialized_file ===
          "components/template-app/next-intl-dashboard-locale.tsx" &&
        surface.upstream_public_apis.includes("useTranslations") &&
        surface.upstream_public_apis.includes("useFormatter"),
    ),
  );
  assert.ok(
    receipt.dx_check_visibility.monitored_surfaces.some(
      (surface) =>
        surface.id === "next-intl-dashboard-message-contract" &&
        surface.status === "present" &&
        surface.source_file ===
          "examples/template/next-intl-dashboard-locale-contract.ts" &&
        surface.materialized_file ===
          "components/template-app/next-intl-dashboard-locale-contract.ts",
    ),
  );
  assert.deepEqual(receipt.dx_check_visibility.blocked_surfaces, []);
  assert.ok(
    receipt.dx_check_visibility.unsupported_surfaces.some(
      (surface) =>
        surface.id === "production-locale-routing-runtime" &&
        surface.status === "unsupported-surface",
    ),
  );

  assert.match(forge, /dxCheckVisibility: \{/);
  assert.match(forge, /schema: "dx\.forge\.package\.dx_check_visibility"/);
  assert.match(forge, /currentStatus: "present"/);
  assert.match(forge, /"missing-receipt"/);
  assert.match(forge, /"unsupported-surface"/);
  assert.match(forge, /id: "next-intl-dashboard-locale-workflow"/);
  assert.match(forge, /id: "next-intl-dashboard-message-contract"/);

  assert.match(catalog, /dxCheckVisibility: \{/);
  assert.match(catalog, /statuses: \["present", "stale", "missing-receipt", "blocked", "unsupported-surface"\]/);
  assert.match(catalog, /receiptPath:\s*"examples\/template\/\.dx\/forge\/receipts\/2026-05-22-i18n-next-intl-dashboard-locale\.json"/);
  assert.match(catalog, /"next-intl-dashboard-locale-workflow"/);
  assert.match(catalog, /"next-intl-dashboard-message-contract"/);
  assert.match(catalog, /honestyLabel: "SOURCE-ONLY"/);

  assert.match(docs, /## dx-check visibility/);
  assert.match(docs, new RegExp(visibilitySchema));
  assert.match(
    docs,
    /present, stale, missing-receipt, blocked, and unsupported-surface/,
  );
});
