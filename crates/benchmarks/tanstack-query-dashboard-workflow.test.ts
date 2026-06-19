const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const sourceMirror = "G:/WWW/inspirations/tanstack-query";
const forbiddenColorPattern =
  /#[0-9a-fA-F]{3,8}|rgb\(|hsl\(|bg-[a-z]+-[0-9]|text-[a-z]+-[0-9]|border-[a-z]+-[0-9]/;

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readMirror(relativePath) {
  return fs.readFileSync(path.join(sourceMirror, relativePath), "utf8");
}

test("TanStack Query dashboard workflow uses upstream-shaped APIs and is visible in the starter", () => {
  const upstreamQueryClient = readMirror("packages/query-core/src/queryClient.ts");
  const upstreamReactUseQuery = readMirror("packages/react-query/src/useQuery.ts");
  const forge = read("core/src/ecosystem/forge_tanstack_query.rs");
  const dashboard = read("examples/dashboard/src/pages/Dashboard.tsx");
  const workflow = read(
    "examples/dashboard/src/components/QueryDashboardWorkflow.tsx",
  );
  const model = read("examples/dashboard/src/lib/queryDashboardWorkflow.ts");
  const launchStatus = read("examples/template/query-cache-status.tsx");
  const launchReadModel = read("examples/template/query-dashboard-read-model.ts");
  const packageDoc = read("docs/packages/tanstack-query.md");
  const readme = read("examples/dashboard/README.md");
  const catalog = read("examples/template/package-catalog.ts");
  const receipt = read(
    "examples/template/.dx/forge/receipts/2026-05-22-tanstack-query-dashboard-data.json",
  );
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");

  assert.match(upstreamQueryClient, /setQueryDefaults/);
  assert.match(upstreamQueryClient, /getQueryDefaults/);
  assert.match(upstreamQueryClient, /invalidateQueries/);
  assert.match(upstreamReactUseQuery, /export function useQuery/);

  assert.match(forge, /"js\/query\/dashboard-workflow\.ts"/);
  assert.match(forge, /const TANSTACK_QUERY_DASHBOARD_WORKFLOW_TS/);
  assert.match(forge, /createDxDashboardQueryClient/);
  assert.match(forge, /applyDxDashboardQueryProfile/);
  assert.match(forge, /refreshDxDashboardQuery/);
  assert.match(forge, /cacheDefaults: \{/);
  assert.match(forge, /staleTime: number/);
  assert.match(forge, /getDxDashboardQueryProfile\(readiness\.profileId\)\.staleTime/);
  assert.match(forge, /export type DxDashboardQueryPublicApi =/);
  assert.match(forge, /publicApi: DxDashboardQueryPublicApi/);
  assert.match(forge, /publicApi: getDxDashboardQueryProfile\(readiness\.profileId\)\.publicApi/);
  assert.match(forge, /DX_DASHBOARD_QUERY_RECEIPT_PATH/);
  assert.match(forge, /receiptPath: typeof DX_DASHBOARD_QUERY_RECEIPT_PATH/);
  assert.match(forge, /receiptPath: DX_DASHBOARD_QUERY_RECEIPT_PATH/);
  assert.match(forge, /runtimeExecution: false/);
  assert.match(forge, /nodeModulesRequired: false/);
  assert.match(forge, /setDxQueryDefaults/);
  assert.match(forge, /invalidateDxQueries/);
  assert.match(forge, /summarizeDxQueryClientCaches/);
  assert.match(forge, /dashboardUsage: \{/);
  assert.match(forge, /dxIcon: "pack:tanstack-query"/);
  assert.match(forge, /"query\/dashboard-workflow\.ts"/);
  assert.match(forge, /"examples\/dashboard\/src\/components\/QueryDashboardWorkflow\.tsx"/);
  assert.match(forge, /officialName: "Data Fetching & Cache"/);
  assert.match(forge, /upstreamPackage: "@tanstack\/react-query"/);
  assert.match(forge, /dxAdd: "dx add data-fetching-cache --write"/);
  assert.match(forge, /dxCheckVisibility: \{/);
  assert.match(forge, /schema: "dx\.forge\.package\.dx_check_visibility"/);
  assert.match(forge, /"missing-receipt"/);
  assert.match(forge, /"unsupported-surface"/);

  assert.match(model, /packageId: 'tanstack\/query'/);
  assert.match(model, /officialName: 'Data Fetching & Cache'/);
  assert.match(model, /upstreamPackage: '@tanstack\/react-query'/);
  assert.match(model, /sourceMirror: 'G:\/WWW\/inspirations\/tanstack-query'/);
  assert.match(model, /queryDashboardReceiptPath/);
  assert.match(model, /export type QueryDashboardRetry = number \| false/);
  assert.match(model, /export type QueryDashboardPublicApi =/);
  assert.match(model, /publicApi: QueryDashboardPublicApi/);
  assert.match(model, /publicApis: \[/);
  assert.match(model, /'QueryClient'/);
  assert.match(model, /'setQueryDefaults'/);
  assert.match(model, /'invalidateQueries'/);
  assert.match(
    model,
    /'examples\/template\/\.dx\/forge\/receipts\/2026-05-22-tanstack-query-dashboard-data\.json'/,
  );
  assert.match(model, /'examples\/template\/query-dashboard-read-model\.ts'/);
  assert.doesNotMatch(model, /\.dx\/forge\/receipts\/tanstack-query\.json/);
  assert.match(model, /dashboardWorkflow: 'query-cache-refresh'/);
  assert.match(model, /nodeModulesRequired: false/);
  assert.match(model, /receiptPath: typeof queryDashboardReceiptPath/);
  assert.match(model, /receiptPath: queryDashboardReceiptPath/);
  assert.match(model, /runtimeExecution: false/);
  assert.match(model, /staleTimeMs: 60_000/);
  assert.match(model, /gcTimeMs: 5 \* 60_000/);
  assert.match(model, /cacheDefaults: \{/);
  assert.match(model, /publicApi: profile\.publicApi/);
  assert.match(model, /queryDashboardProfiles/);
  assert.match(model, /createQueryDashboardReceipt/);
  assert.match(model, /dataFreshness:/);
  assert.match(model, /queryDashboardDxCheckVisibility/);
  assert.match(model, /dxCheckVisibility: QueryDashboardDxCheckVisibility/);
  assert.match(model, /'dx\.forge\.package\.dx_check_visibility'/);

  assert.match(workflow, /data-dx-package="tanstack\/query"/);
  assert.match(workflow, /data-dx-component="dashboard-tanstack-query-workflow"/);
  assert.match(workflow, /data-dx-dashboard-workflow="query-cache-refresh"/);
  assert.match(workflow, /data-dx-query-check-visibility=/);
  assert.match(workflow, /data-dx-style-surface="theme-token"/);
  assert.match(workflow, /data-dx-query-profile=/);
  assert.match(workflow, /data-dx-query-refresh-state=/);
  assert.match(workflow, /data-dx-query-stale-time-ms=/);
  assert.match(workflow, /data-dx-query-gc-time-ms=/);
  assert.match(workflow, /data-dx-query-retry=/);
  assert.match(workflow, /data-dx-query-dashboard-receipt-path=/);
  assert.match(workflow, /data-dx-query-runtime-execution=/);
  assert.match(workflow, /data-dx-node-modules="forbidden"/);
  assert.match(workflow, /data-dx-official-package="Data Fetching & Cache"/);
  assert.match(workflow, /<dx-icon name="pack:tanstack-query" aria-label="Data Fetching & Cache" \/>/);
  assert.match(workflow, /<h2>Data Fetching &amp; Cache workflow<\/h2>/);
  assert.doesNotMatch(workflow, /<h2>TanStack Query cache workflow<\/h2>/);
  assert.match(workflow, /data-dx-query-action="select-cache-profile"/);
  assert.match(workflow, /data-dx-query-action="prepare-refresh-receipt"/);
  assert.match(workflow, /data-dx-query-public-api=/);
  assert.match(workflow, /data-dx-query-receipt-public-api=/);
  assert.match(workflow, /data-dx-query-cache-defaults=/);
  assert.match(workflow, /data-dx-query-receipt-path=/);
  assert.match(workflow, /data-dx-query-receipt-state=/);
  assert.match(workflow, /class="panel-header"/);
  assert.match(workflow, /class="provider-options"/);
  assert.match(workflow, /class="readiness-list"/);
  assert.match(workflow, /class="primary-action"/);
  assert.doesNotMatch(workflow, forbiddenColorPattern);
  assert.doesNotMatch(workflow, /simple-icons:|lucide:|brand:/);

  assert.match(catalog, /officialName: "Data Fetching & Cache"/);
  assert.match(catalog, /upstreamPackage: "@tanstack\/react-query"/);
  assert.match(catalog, /command: "dx add data-fetching-cache --write"/);
  assert.match(catalog, /"data-fetching-cache"/);
  assert.match(catalog, /dxCheckVisibility: \{/);
  assert.match(catalog, /schema: "dx\.forge\.package\.dx_check_visibility"/);

  assert.match(launchReadModel, /displayName: string/);
  assert.match(launchReadModel, /displayName: item\.officialName \?\? item\.packageId/);
  assert.match(launchStatus, /data-dx-query-package-name=/);
  assert.match(launchStatus, /\{item\.displayName\}/);
  assert.match(launchStatus, /Data Fetching &amp; Cache model/);
  assert.doesNotMatch(launchStatus, /cached TanStack Query model/);

  assert.match(receipt, /"package_name": "Data Fetching & Cache"/);
  assert.match(receipt, /"upstream_package": "@tanstack\/react-query"/);
  assert.match(receipt, /"source_mirror": "G:\/WWW\/inspirations\/tanstack-query"/);
  assert.match(receipt, /"dx_check_visibility": \{/);
  assert.match(receipt, /"schema": "dx\.forge\.package\.dx_check_visibility"/);
  assert.match(receipt, /"present"/);
  assert.match(receipt, /"stale"/);
  assert.match(receipt, /"missing-receipt"/);
  assert.match(receipt, /"blocked"/);
  assert.match(receipt, /"unsupported-surface"/);

  assert.match(dashboard, /QueryDashboardWorkflow/);
  assert.match(dashboard, /<QueryDashboardWorkflow \/>/);
  assert.doesNotMatch(dashboard, /QueryDashboardWorkflow\.sr/);

  assert.match(packageDoc, /^# Data Fetching & Cache Forge Slice/m);
  assert.match(packageDoc, /Upstream package: `@tanstack\/react-query`/);
  assert.doesNotMatch(packageDoc, /^# TanStack Query Forge Slice/m);
  assert.match(packageDoc, /Dashboard Workflow/);
  assert.match(packageDoc, /## dx-check Visibility/);
  assert.match(
    packageDoc,
    /present, stale, missing receipt, blocked, and unsupported surface/,
  );
  assert.match(packageDoc, /QueryDashboardWorkflow/);
  assert.match(packageDoc, /query\/dashboard-workflow\.ts/);
  assert.match(packageDoc, /dx run --test \.\\benchmarks\\tanstack-query-dashboard-workflow\.test\.ts/);
  assert.doesNotMatch(packageDoc, /\/api\/health/);
  assert.match(readme, /Data Fetching & Cache dashboard workflow/);
  assert.match(dx, /dashboard starter consumes `tanstack\/query`/);
  assert.match(todo, /TanStack Query starter dashboard workflow/);
  assert.match(changelog, /TanStack Query starter dashboard workflow/);
});
