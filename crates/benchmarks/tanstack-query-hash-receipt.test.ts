const assert = require("node:assert/strict");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const receiptPath =
  "examples/template/.dx/forge/receipts/2026-05-22-tanstack-query-dashboard-data.json";
const trackedFiles = [
  "core/src/ecosystem/forge_tanstack_query.rs",
  "examples/template/query-cache-status.tsx",
  "examples/template/query-dashboard-read-model.ts",
  "examples/template/template-shell.tsx",
  "examples/template/dx-studio-edit-contract.ts",
  "examples/template/template-route-contract.ts",
  "examples/template/package-catalog.ts",
  "tools/launch/runtime-template/pages/index.html",
  "tools/launch/runtime-template/assets/launch-runtime.ts",
  "examples/dashboard/src/lib/queryDashboardWorkflow.ts",
  "examples/dashboard/src/components/QueryDashboardWorkflow.tsx",
  "docs/packages/data-fetching-cache.source-guard-runbook.json",
  "tools/launch/materialize-www-template.ts",
  "examples/template/components/template-app/dashboard-query-cache.ts",
  "examples/template/server/query-cache/readiness.ts",
  "examples/template/app/api/query-cache/readiness/route.ts",
];

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function sha256(relativePath) {
  return crypto
    .createHash("sha256")
    .update(fs.readFileSync(path.join(root, relativePath)))
    .digest("hex");
}

test("Data Fetching & Cache receipt hashes drive package-status and dx-check staleness", () => {
  const upstreamQueryClient = fs.readFileSync(
    path.resolve(
      root,
      "..",
      "..",
      "WWW/inspirations/tanstack-query/packages/query-core/src/queryClient.ts",
    ),
    "utf8",
  );
  const receipt = readJson(receiptPath);
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const readModel = read("examples/template/forge-package-status-read-model.ts");
  const packageCatalog = read("examples/template/package-catalog.ts");
  const packageDoc = read("docs/packages/tanstack-query.md");
  const rustHelper = read(
    "core/src/ecosystem/project_check/data_fetching_cache_dx_check.rs",
  );
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");

  assert.match(upstreamQueryClient, /setQueryDefaults/);
  assert.match(upstreamQueryClient, /getQueryDefaults/);
  assert.match(upstreamQueryClient, /invalidateQueries/);
  assert.match(upstreamQueryClient, /ensureQueryData/);

  assert.equal(receipt.package_id, "tanstack/query");
  assert.equal(receipt.package_name, "Data Fetching & Cache");
  assert.equal(receipt.hash_algorithm, "sha256");
  assert.deepEqual(receipt.files, trackedFiles);
  assert.ok(receipt.file_hashes, "receipt file_hashes manifest is missing");

  for (const filePath of trackedFiles) {
    assert.equal(
      receipt.file_hashes[filePath],
      sha256(filePath),
      `${filePath} hash is stale in the Data Fetching & Cache receipt`,
    );
  }

  const visibility = status.package_lane_visibility.find(
    (entry) => entry.package_id === "tanstack/query",
  );
  assert.ok(visibility, "Data Fetching & Cache package-status row is missing");

  for (const surfaceId of [
    "data-fetching-cache-query-dashboard-workflow",
    "data-fetching-cache-starter-dashboard-workflow",
  ]) {
    const surface = visibility.selected_surfaces.find(
      (candidate) => candidate.surface_id === surfaceId,
    );
    assert.ok(surface, `${surfaceId} is missing from package-status`);
    assert.equal(surface.hash_algorithm, "sha256");
    assert.deepEqual(surface.file_hashes, receipt.file_hashes);
  }

  for (const metric of [
    "data_fetching_cache_hash_manifest_present",
    "data_fetching_cache_hash_mismatch",
  ]) {
    assert.ok(
      visibility.dx_check_metrics.includes(metric),
      `${metric} missing from Data Fetching & Cache package-status row`,
    );
    assert.ok(
      status.dx_check_metrics.includes(metric),
      `${metric} missing from package-status dx_check_metrics`,
    );
    assert.match(readModel, new RegExp(metric));
    assert.match(rustHelper, new RegExp(metric));
    assert.match(packageDoc, new RegExp(metric));
  }

  assert.match(packageCatalog, /receiptIntegrity: \{/);
  assert.match(packageCatalog, /trackedFiles: \[/);
  assert.match(packageCatalog, /hashAlgorithm: "sha256"/);
  assert.match(rustHelper, /hash_manifest_present = 1/);
  assert.match(rustHelper, /use super::file_hashes::count_sha256_file_hash_mismatches;/);
  assert.match(
    rustHelper,
    /hash_mismatches \+= count_sha256_file_hash_mismatches\(root, surface\)/,
  );
  assert.doesNotMatch(rustHelper, /fn count_hash_mismatches\(/);
  assert.match(rustHelper, /data-fetching-cache-hash-mismatch/);

  for (const source of [dx, todo, changelog]) {
    assert.match(source, /hash_algorithm: sha256/);
    assert.match(source, /data_fetching_cache_hash_mismatch/);
    assert.match(source, /without claiming live QueryClient runtime proof/);
  }
});
