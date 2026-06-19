const assert = require("assert");
const crypto = require("crypto");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const receiptPath =
  "examples/template/.dx/forge/receipts/2026-05-22-api-trpc-dashboard-workflow.json";

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function sha256(relativePath) {
  return crypto.createHash("sha256").update(read(relativePath)).digest("hex");
}

test("Type-Safe API receipt exposes dx-check visibility states and selected surfaces", () => {
  const receipt = readJson(receiptPath);
  const forge = read("core/src/ecosystem/forge_trpc.rs");
  const packageCatalog = read("examples/template/package-catalog.ts");
  const packageDoc = read("docs/packages/api-trpc.md");
  const cli = read("dx-www/src/cli/mod.rs");

  assert.equal(receipt.package_id, "api/trpc");
  assert.equal(receipt.package_name, "Type-Safe API");
  assert.equal(receipt.official_dx_package_name, "Type-Safe API");
  assert.equal(receipt.upstream_package, "@trpc/server");
  assert.equal(receipt.source_mirror, "G:/WWW/inspirations/trpc");
  assert.equal(receipt.node_modules_required, false);
  assert.equal(receipt.no_runtime_execution, true);
  assert.equal(receipt.hash_algorithm, "sha256");
  for (const filePath of [
    "core/src/ecosystem/forge_trpc.rs",
    "examples/template/trpc-launch-health.tsx",
    "examples/template/trpc-launch-contract.ts",
    "examples/dashboard/src/components/TrpcDashboardWorkflow.tsx",
    "examples/dashboard/src/lib/trpcDashboardWorkflow.ts",
  ]) {
    assert.equal(
      receipt.file_hashes[filePath],
      sha256(filePath),
      `${filePath} hash is stale in the Type-Safe API receipt`,
    );
  }
  assert.ok(
    receipt.guards.includes(
      "dx run --test .\\benchmarks\\trpc-dx-check-visibility-receipt.test.ts",
    ),
  );
  assert.ok(receipt.upstream_public_apis.includes("initTRPC.context().create()"));
  assert.ok(receipt.upstream_public_apis.includes("fetchRequestHandler"));
  assert.ok(receipt.upstream_public_apis.includes("trpc.launchEvent.mutationOptions()"));

  assert.equal(
    receipt.dx_check_visibility.schema,
    "dx.forge.package.dx_check_visibility",
  );
  assert.equal(receipt.dx_check_visibility.current_status, "present");
  assert.deepEqual(
    receipt.dx_check_visibility.status_legend.map((entry) => entry.status),
    ["present", "stale", "missing-receipt", "blocked", "unsupported-surface"],
  );
  assert.ok(
    receipt.dx_check_visibility.monitored_surfaces.some(
      (surface) =>
        surface.id === "trpc-launch-dashboard-workflow" &&
        surface.status === "present" &&
        surface.hash_algorithm === "sha256" &&
        Object.keys(surface.file_hashes).length >= 2 &&
        surface.materialized_file === "components/template-app/trpc-launch-health.tsx" &&
        surface.receipt_path === receiptPath,
    ),
  );
  assert.ok(
    receipt.dx_check_visibility.monitored_surfaces.some(
      (surface) =>
        surface.id === "trpc-starter-dashboard-workflow" &&
        surface.status === "present" &&
        surface.hash_algorithm === "sha256" &&
        Object.keys(surface.file_hashes).length >= 2 &&
        surface.materialized_file === "components/dashboard/trpc-dashboard-workflow.tsx",
    ),
  );
  assert.ok(
    receipt.dx_check_visibility.monitored_surfaces.some(
      (surface) =>
        surface.id === "trpc-route-handler" &&
        surface.status === "present" &&
        surface.hash_algorithm === "sha256" &&
        surface.file_hashes["core/src/ecosystem/forge_trpc.rs"] ===
          receipt.file_hashes["core/src/ecosystem/forge_trpc.rs"] &&
        surface.materialized_file === "app/api/trpc/[trpc]/route.ts",
    ),
  );
  assert.ok(
    receipt.dx_check_visibility.dx_check_metrics.includes(
      "type_safe_api_receipt_present",
    ),
  );
  assert.ok(
    receipt.dx_check_visibility.dx_check_metrics.includes(
      "type_safe_api_unsupported_surface",
    ),
  );
  assert.ok(
    receipt.dx_check_visibility.dx_check_metrics.includes(
      "type_safe_api_hash_manifest_present",
    ),
  );
  assert.ok(
    receipt.dx_check_visibility.dx_check_metrics.includes(
      "type_safe_api_hash_mismatch",
    ),
  );

  assert.match(forge, /dxCheckVisibility: \{/);
  assert.match(forge, /schema: "dx\.forge\.package\.dx_check_visibility"/);
  assert.match(forge, /currentStatus: "present"/);
  assert.match(forge, /"missing-receipt"/);
  assert.match(forge, /"unsupported-surface"/);
  assert.match(forge, /type_safe_api_receipt_present/);

  assert.match(packageCatalog, /dxCheckVisibility: \{/);
  assert.match(
    packageCatalog,
    /receiptPath:\s*"examples\/template\/\.dx\/forge\/receipts\/2026-05-22-api-trpc-dashboard-workflow\.json"/,
  );
  assert.match(packageCatalog, /trpc-launch-dashboard-workflow/);
  assert.match(packageCatalog, /trpc-starter-dashboard-workflow/);
  assert.match(packageCatalog, /trpc-route-handler/);

  assert.match(
    cli,
    /"package_id": "api\/trpc"[\s\S]*"dx_check_visibility": \{[\s\S]*"receipt_path": "examples\/template\/\.dx\/forge\/receipts\/2026-05-22-api-trpc-dashboard-workflow\.json"/,
  );
  assert.match(cli, /"type_safe_api_receipt_present"/);
  assert.match(cli, /"type_safe_api_hash_mismatch"/);
  assert.match(cli, /"trpc-launch-dashboard-workflow"/);

  assert.match(packageDoc, /## dx-check Visibility/);
  assert.match(
    packageDoc,
    /present, stale, missing-receipt, blocked, and unsupported-surface/,
  );
  assert.match(packageDoc, /hash_algorithm: sha256/);
  assert.match(packageDoc, /type_safe_api_hash_mismatch/);
});
