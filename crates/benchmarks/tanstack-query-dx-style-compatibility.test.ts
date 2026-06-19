const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function assertDxStyleCompatibility(value) {
  assert.equal(value.schema, "dx.forge.package.dx_style_compatibility");
  assert.equal(value.status, "present");
  assert.equal(value.token_source ?? value.tokenSource, "styles/globals.css");
  assert.equal(value.generated_css ?? value.generatedCss, "styles/globals.css");
  assert.equal(value.runtime_proof ?? value.runtimeProof, false);
}

function assertIncludesAll(values, expectedValues) {
  for (const expectedValue of expectedValues) {
    assert.ok(
      values.includes(expectedValue),
      `expected ${JSON.stringify(values)} to include ${expectedValue}`,
    );
  }
}

test("Data Fetching & Cache publishes dx-style compatibility for query dashboard surfaces", () => {
  const upstreamPackage = fs.readFileSync(
    path.resolve(
      root,
      "..",
      "..",
      "WWW/inspirations/tanstack-query/packages/react-query/package.json",
    ),
    "utf8",
  );
  const upstreamQueryClient = fs.readFileSync(
    path.resolve(
      root,
      "..",
      "..",
      "WWW/inspirations/tanstack-query/packages/query-core/src/queryClient.ts",
    ),
    "utf8",
  );

  assert.match(upstreamPackage, /"name": "@tanstack\/react-query"/);
  assert.match(upstreamPackage, /"version": "5\.100\.10"/);
  for (const api of [
    "setQueryDefaults",
    "getQueryDefaults",
    "invalidateQueries",
    "ensureQueryData",
  ]) {
    assert.match(upstreamQueryClient, new RegExp(api));
  }

  const launchWorkflow = read("examples/template/query-cache-status.tsx");
  const starterWorkflow = read(
    "examples/dashboard/src/components/QueryDashboardWorkflow.tsx",
  );

  assert.match(launchWorkflow, /data-dx-style-surface="data-fetching-cache"/);
  assert.match(launchWorkflow, /border-border/);
  assert.match(launchWorkflow, /bg-card/);
  assert.match(launchWorkflow, /text-card-foreground/);
  assert.doesNotMatch(launchWorkflow, /style=\{\{/);
  assert.doesNotMatch(launchWorkflow, /#[0-9a-fA-F]{3,8}/);
  assert.match(starterWorkflow, /data-dx-style-surface="theme-token"/);

  const receipt = readJson(
    "examples/template/.dx/forge/receipts/2026-05-22-tanstack-query-dashboard-data.json",
  );
  assertDxStyleCompatibility(receipt.dx_style_compatibility);
  assertIncludesAll(receipt.dx_style_compatibility.visible_surfaces, [
    "data-fetching-cache-query-dashboard-workflow",
    "data-fetching-cache-starter-dashboard-workflow",
  ]);
  assertIncludesAll(receipt.dx_style_compatibility.source_files, [
    "examples/template/query-cache-status.tsx",
    "examples/dashboard/src/components/QueryDashboardWorkflow.tsx",
  ]);
  assertIncludesAll(receipt.dx_style_compatibility.data_dx_markers, [
    'data-dx-style-surface="data-fetching-cache"',
    'data-dx-style-surface="theme-token"',
  ]);
  assert.match(
    receipt.dx_style_compatibility.runtime_limitations.join("\n"),
    /SOURCE-ONLY/,
  );
  assert.match(
    receipt.dx_style_compatibility.runtime_limitations.join("\n"),
    /No live browser style proof was run/,
  );

  const status = readJson("examples/template/.dx/forge/package-status.json");
  const dataFetchingRow = status.package_lane_visibility.find(
    (entry) => entry.package_id === "tanstack/query",
  );
  assert.ok(dataFetchingRow, "missing Data Fetching & Cache package-status row");
  assertDxStyleCompatibility(dataFetchingRow.dx_style_compatibility);
  assertIncludesAll(dataFetchingRow.dx_check_metrics, [
    "data_fetching_cache_dx_style_compatibility_present",
    "data_fetching_cache_dx_style_compatibility_missing",
  ]);
  assertIncludesAll(status.dx_check_metrics, [
    "data_fetching_cache_dx_style_compatibility_present",
    "data_fetching_cache_dx_style_compatibility_missing",
  ]);
  const querySurface = dataFetchingRow.selected_surfaces.find(
    (surface) =>
      surface.surface_id === "data-fetching-cache-query-dashboard-workflow",
  );
  const starterSurface = dataFetchingRow.selected_surfaces.find(
    (surface) =>
      surface.surface_id === "data-fetching-cache-starter-dashboard-workflow",
  );
  assertIncludesAll(querySurface.source_markers, [
    'data-dx-style-surface="data-fetching-cache"',
  ]);
  assertIncludesAll(starterSurface.source_markers, [
    'data-dx-style-surface="theme-token"',
  ]);

  const readModel = read("examples/template/forge-package-status-read-model.ts");
  assert.match(readModel, /dxStyleCompatibility: \{/);
  assert.match(readModel, /data_fetching_cache_dx_style_compatibility_present/);
  assert.match(readModel, /data_fetching_cache_dx_style_compatibility_missing/);
  assert.match(readModel, /data-dx-style-surface="data-fetching-cache"/);
  assert.match(readModel, /data-dx-style-surface="theme-token"/);

  const checker = read(
    "core/src/ecosystem/project_check/data_fetching_cache_dx_check.rs",
  );
  assert.match(checker, /data_fetching_cache_dx_style_compatibility_present/);
  assert.match(checker, /data_fetching_cache_dx_style_compatibility_missing/);
  assert.match(checker, /data-fetching-cache-missing-dx-style-compatibility/);
  assert.match(checker, /fn dx_style_compatibility_is_present/);

  const docs = read("docs/packages/tanstack-query.md");
  assert.match(docs, /## DX-Style Compatibility/);
  assert.match(docs, /data_fetching_cache_dx_style_compatibility_present/);
  assert.match(docs, /data-fetching-cache-missing-dx-style-compatibility/);
  assert.match(docs, /data-dx-style-surface="data-fetching-cache"/);
});
