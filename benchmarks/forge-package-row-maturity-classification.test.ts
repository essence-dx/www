const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const statusPath = "examples/template/.dx/forge/package-status.json";

const expectedPackageIds = [
  "shadcn/ui/button",
  "state/zustand",
  "tanstack/query",
  "reactive/store",
  "db/drizzle-sqlite",
  "validation/zod",
  "forms/react-hook-form",
  "instantdb/react",
  "content/react-markdown",
  "content/fumadocs-next",
  "auth/better-auth",
  "i18n/next-intl",
  "api/trpc",
  "ai/vercel-ai",
  "payments/stripe-js",
  "supabase/client",
  "automations/n8n",
  "wasm/bindgen",
  "3d/launch-scene",
  "animation/motion",
];

const providerGatedPackages = new Set([
  "auth/better-auth",
  "instantdb/react",
  "ai/vercel-ai",
  "payments/stripe-js",
  "supabase/client",
  "automations/n8n",
]);

function readJson(relativePath) {
  return JSON.parse(fs.readFileSync(path.join(root, relativePath), "utf8"));
}

function rowText(row) {
  return [
    ...(row.runtime_limitations ?? []),
    ...(row.blocked_surfaces ?? []),
    ...(row.app_owned_boundaries ?? []),
  ].join("\n");
}

test("all visible Forge package rows carry honest maturity classification signals", () => {
  const status = readJson(statusPath);
  const rows = status.package_lane_visibility ?? [];
  const packageIds = rows.map((row) => row.package_id);

  assert.deepEqual(
    packageIds,
    expectedPackageIds,
    "package-status row order should stay aligned with the launch package reality panel",
  );
  assert.equal(status.package_count, expectedPackageIds.length);
  assert.equal(status.locked_package_count, expectedPackageIds.length);
  assert.equal(status.cache.current_manifest_count, expectedPackageIds.length);
  assert.equal(status.cache.physical_manifest_count, expectedPackageIds.length);
  assert.equal(status.cache.stale_physical_manifest_count, 0);

  for (const row of rows) {
    const text = rowText(row);

    assert.equal(row.status, "present", `${row.package_id} should be present`);
    assert.equal(
      row.receipt_status,
      "present",
      `${row.package_id} should have a present receipt`,
    );
    assert.ok(
      row.selected_surfaces?.length > 0,
      `${row.package_id} should not be catalog-only`,
    );
    assert.equal(
      row.receipt_hash_refresh?.status,
      "current",
      `${row.package_id} should expose current receipt hash freshness`,
    );
    assert.equal(
      row.receipt_hash_refresh?.runtime_execution,
      false,
      `${row.package_id} should not claim runtime execution from receipt freshness`,
    );
    assert.ok(
      /SOURCE-ONLY|ADAPTER-BOUNDARY|LOCK-BACKED SOURCE-OWNED/.test(text),
      `${row.package_id} should publish a maturity classification signal`,
    );
    assert.doesNotMatch(
      text,
      /live .*proof is complete|production-proven|no-node-modules replacement/i,
      `${row.package_id} should not overclaim production runtime proof`,
    );

    if (providerGatedPackages.has(row.package_id)) {
      assert.match(
        text,
        /ADAPTER-BOUNDARY/i,
        `${row.package_id} should stay classified as a provider boundary`,
      );
      assert.match(
        text,
        /credential|provider|hosted|OAuth|Stripe|n8n|API key|webhook|runtime proof/i,
        `${row.package_id} should name the gated provider/runtime boundary`,
      );
    }
  }
});
