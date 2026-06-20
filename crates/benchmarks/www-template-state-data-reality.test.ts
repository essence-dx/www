import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { spawnSync } from "node:child_process";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";
import { pathToFileURL } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const materializer = path.join(root, "tools", "launch", "materialize-www-template.ts");

function read(relativePath: string) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath: string) {
  return JSON.parse(read(relativePath));
}

function runReceiptHelper(relativePath: string) {
  const result = spawnSync(process.execPath, [relativePath, "--check", "--json"], {
    cwd: root,
    encoding: "utf8",
  });

  assert.equal(result.stderr, "");
  assert.equal(result.status, 0, result.stdout);
  return JSON.parse(result.stdout);
}

test("lane 3 packages are source-owned and honestly classified", async () => {
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const lock = readJson("examples/template/.dx/forge/package-lock.json");
  const realitySource = read("examples/template/components/template-app/package-reality.ts");
  const packageReality = await import(
    pathToFileURL(
      path.join(root, "examples/template/components/template-app/package-reality.ts"),
    ).href
  );
  const laneScores = Object.fromEntries(
    packageReality.forgeRealityRows.map((row: { packageId: string; score: number }) => [
      row.packageId,
      row.score,
    ]),
  );

  assert.ok(status.locked_package_names.includes("state/zustand"));
  assert.ok(status.locked_package_names.includes("tanstack/query"));
  assert.ok(status.locked_package_names.includes("reactive/store"));
  assert.ok(lock.packages.some((entry: { name: string }) => entry.name === "reactive/store"));
  assert.ok(
    fs.existsSync(
      path.join(
        root,
        "examples/template/.dx/forge/cache/reactive-store/0.11.0-dx.1/.dx/build-cache/manifest.json",
      ),
    ),
  );

  const reactiveLane = status.package_lane_visibility.find(
    (entry: { package_id: string }) => entry.package_id === "reactive/store",
  );
  assert.equal(reactiveLane?.official_package_name, "Reactive Store");
  assert.equal(reactiveLane?.receipt_status, "present");
  assert.equal(reactiveLane?.receipt_hash_refresh?.status, "current");

  assert.match(realitySource, /lock-backed adapter-boundary/);
  assert.match(realitySource, /packageId === "tanstack\/query"/);
  assert.match(
    realitySource,
    /"tanstack\/query": \{[\s\S]*controlId: "query-refresh"/,
  );
  assert.match(realitySource, /queryclient-adapter-required/);
  assert.equal(laneScores["state/zustand"], 86);
  assert.equal(laneScores["tanstack/query"], 86);
  assert.equal(laneScores["reactive/store"], 86);
  assert.ok(laneScores["tanstack/query"] < 90);
});

test("default dashboard exposes real lane 3 state, filters, optimistic readiness, and cache status", () => {
  const dashboardSource = read("examples/template/components/template-app/dashboard-page.tsx");
  const dashboardStateSource = read("examples/template/components/template-app/dashboard-state.ts");
  const queryCacheSource = read("examples/template/components/template-app/dashboard-query-cache.ts");
  const runtimeSource = read("tools/launch/runtime-template/assets/launch-runtime.ts");

  assert.match(dashboardSource, /createStoreContext<DashboardReactiveStoreSnapshot>/);
  assert.match(dashboardSource, /DashboardReactiveStoreProvider/);
  assert.match(dashboardSource, /data-dx-lane-three-state="state-data-fetching"/);
  assert.match(dashboardSource, /data-dx-dashboard-filter-state=\{filter\}/);
  assert.match(dashboardSource, /data-dx-optimistic-ui-state=\{optimisticReceipt\.state\}/);
  assert.match(dashboardSource, /data-dx-query-cache-status=\{queryCacheStatus\.status\}/);
  assert.match(dashboardSource, /data-dx-query-cache-runtime=\{queryCacheStatus\.runtimeBoundary\}/);
  assert.match(dashboardSource, /data-dx-query-cache-key=\{queryCacheStatus\.queryKey\}/);
  assert.match(dashboardSource, /data-dx-query-cache-optimistic-count=\{queryCacheStatus\.optimisticEntryCount\}/);
  assert.match(dashboardSource, /data-dx-reactive-store-provider="dashboard-context"/);
  assert.match(
    dashboardSource,
    /data-dx-tanstack-query-runtime-boundary=\{queryCacheStatus\.upstreamAdapterBoundary\}/,
  );

  assert.match(dashboardStateSource, /create<ForgeDashboardStore>/);
  assert.match(dashboardStateSource, /setFilter: \(filter\) => set\(\{ filter \}\)/);
  assert.match(dashboardStateSource, /markOptimisticReceipt/);

  assert.match(queryCacheSource, /source-owned-cache-readiness/);
  assert.match(queryCacheSource, /source-owned-template-cache/);
  assert.match(queryCacheSource, /queryclient-adapter-required/);
  assert.doesNotMatch(queryCacheSource, /from ["']@tanstack\/react-query["']/);
  assert.match(runtimeSource, /source-owned-cache-action-dry-run/);
  assert.match(runtimeSource, /dxQueryCacheActionState/);
  assert.match(runtimeSource, /dxQueryCacheLastAction/);
  assert.match(runtimeSource, /dxQueryCacheInvalidatedCount/);
  assert.match(runtimeSource, /lane-three-cache-meter/);

  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-lane3-state-data-"));
  execFileSync(process.execPath, [materializer, dir], { cwd: root, stdio: "pipe" });
  const materializedDashboard = fs.readFileSync(
    path.join(dir, "pages", "dashboard.html"),
    "utf8",
  );

  assert.match(materializedDashboard, /data-dx-lane-three-state="state-data-fetching"/);
  assert.match(materializedDashboard, /data-dx-reactive-store-provider="dashboard-context"/);
  assert.match(materializedDashboard, /data-dx-query-cache-status="source-owned-cache-readiness"/);
  assert.match(
    materializedDashboard,
    /data-dx-query-cache-runtime="source-owned-template-cache"/,
  );
  assert.match(
    materializedDashboard,
    /data-dx-query-cache-optimistic-count="\d+"/,
  );
  assert.match(
    materializedDashboard,
    /data-dx-query-cache-action-state="no-cache-action-requested"/,
  );
  assert.match(
    materializedDashboard,
    /data-dx-query-cache-last-action="none"/,
  );
  assert.match(
    materializedDashboard,
    /data-dx-tanstack-query-runtime-boundary="queryclient-adapter-required"/,
  );
});

test("data fetching cache has a source-owned template cache model tracked by receipts", async () => {
  const queryCachePath =
    "examples/template/components/template-app/dashboard-query-cache.ts";
  const queryCacheSource = read(queryCachePath);
  const receipt = readJson(
    "examples/template/.dx/forge/receipts/2026-05-22-tanstack-query-dashboard-data.json",
  );
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const dataFetchingLane = status.package_lane_visibility.find(
    (entry: { package_id: string }) => entry.package_id === "tanstack/query",
  );
  const dashboardSurface = dataFetchingLane?.selected_surfaces.find(
    (surface: { surface_id: string }) =>
      surface.surface_id === "data-fetching-cache-query-dashboard-workflow",
  );

  assert.match(queryCacheSource, /createDashboardQueryCache/);
  assert.match(queryCacheSource, /invalidateDashboardQueryCache/);
  assert.match(queryCacheSource, /summarizeDashboardQueryCache/);
  assert.match(queryCacheSource, /source-owned-template-cache/);
  assert.doesNotMatch(queryCacheSource, /from ["']@tanstack\/react-query["']/);
  assert.ok(receipt.source_files.includes(queryCachePath));
  assert.ok(receipt.file_hashes[queryCachePath]);
  assert.ok(dashboardSurface?.file_hashes?.[queryCachePath]);

  const cacheModule = await import(
    pathToFileURL(path.join(root, queryCachePath)).href
  );
  const packageLockReality = await import(
    pathToFileURL(
      path.join(
        root,
        "examples/template/components/template-app/package-lock-reality.ts",
      ),
    ).href
  );
  const cache = cacheModule.createDashboardQueryCache({
    filter: "ready",
    lastReceiptState: "Local cache queued",
    optimisticState: "queued",
    visibleProjectCount: 2,
  });
  assert.equal(cache.runtimeBoundary, "source-owned-template-cache");
  assert.equal(cache.upstreamAdapterBoundary, "queryclient-adapter-required");
  assert.equal(cache.entries[0].queryKey, "dx:dashboard:projects:ready");
  const defaultCountCache = cacheModule.createDashboardQueryCache({
    filter: "all",
    lastReceiptState: "Forge package-status receipt",
    optimisticState: "idle",
    visibleProjectCount: 4,
  });
  assert.equal(
    defaultCountCache.entries[1].itemCount,
    packageLockReality.defaultTemplateLockReality.packageIds.length,
  );

  const invalidated = cacheModule.invalidateDashboardQueryCache(
    cache,
    "dx:dashboard:projects:ready",
  );
  const summary = cacheModule.summarizeDashboardQueryCache(invalidated);
  assert.equal(summary.cacheEntryCount, 2);
  assert.equal(summary.optimisticEntryCount, 1);
  assert.equal(summary.readyEntryCount, 1);
  assert.equal(summary.invalidatedEntryCount, 1);

  const statusResult = cacheModule.createDashboardQueryCacheStatus({
    filter: "ready",
    lastReceiptState: "Local cache queued",
    optimisticState: "queued",
    visibleProjectCount: 2,
  });
  assert.equal(statusResult.queryKey, "dx:dashboard:projects:ready");
  assert.equal(statusResult.runtimeBoundary, "source-owned-template-cache");
  assert.equal(statusResult.upstreamAdapterBoundary, "queryclient-adapter-required");
  assert.equal(statusResult.optimisticEntryCount, 2);

  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-lane3-query-cache-"));
  execFileSync(process.execPath, [materializer, dir], { cwd: root, stdio: "pipe" });
  const materializedDashboard = fs.readFileSync(
    path.join(dir, "pages", "dashboard.html"),
    "utf8",
  );

  assert.match(
    materializedDashboard,
    /data-dx-query-cache-runtime="source-owned-template-cache"/,
  );
  assert.match(
    materializedDashboard,
    /data-dx-tanstack-query-runtime-boundary="queryclient-adapter-required"/,
  );
  assert.match(
    materializedDashboard,
    /data-dx-query-cache-key="dx:dashboard:projects:all"/,
  );
});

test("data fetching cache exposes an App Router readiness route without QueryClient overclaim", async () => {
  const serverPath = "examples/template/server/query-cache/readiness.ts";
  const routePath = "examples/template/app/api/query-cache/readiness/route.ts";
  const serverSource = read(serverPath);
  const routeSource = read(routePath);
  const receipt = readJson(
    "examples/template/.dx/forge/receipts/2026-05-22-tanstack-query-dashboard-data.json",
  );
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const dataFetchingLane = status.package_lane_visibility.find(
    (entry: { package_id: string }) => entry.package_id === "tanstack/query",
  );
  const routeSurface = dataFetchingLane?.selected_surfaces.find(
    (surface: { surface_id: string }) =>
      surface.surface_id === "data-fetching-cache-readiness-route",
  );

  assert.match(serverSource, /readDataFetchingCacheReadiness/);
  assert.match(serverSource, /createDataFetchingCacheReadinessResponse/);
  assert.match(serverSource, /createDataFetchingCacheActionResponse/);
  assert.match(serverSource, /createDataFetchingCacheActionErrorResponse/);
  assert.match(serverSource, /unsupported-cache-action/);
  assert.match(serverSource, /"source-owned-cache-readiness"/);
  assert.match(serverSource, /"source-owned-cache-action-dry-run"/);
  assert.match(serverSource, /"queryclient-adapter-required"/);
  assert.doesNotMatch(serverSource, /from ["']@tanstack\/react-query["']/);
  assert.match(routeSource, /@\/server\/query-cache\/readiness/);
  assert.match(routeSource, /export function GET/);
  assert.match(routeSource, /export async function POST/);
  assert.ok(receipt.source_files.includes(serverPath));
  assert.ok(receipt.source_files.includes(routePath));
  assert.ok(receipt.file_hashes[serverPath]);
  assert.ok(receipt.file_hashes[routePath]);
  assert.ok(routeSurface?.file_hashes?.[serverPath]);
  assert.ok(routeSurface?.file_hashes?.[routePath]);
  assert.ok(routeSurface?.source_markers?.includes("createDataFetchingCacheActionResponse"));
  assert.ok(routeSurface?.source_markers?.includes("createDataFetchingCacheActionErrorResponse"));
  assert.ok(routeSurface?.source_markers?.includes("source-owned-cache-action-dry-run"));
  assert.ok(routeSurface?.source_markers?.includes("unsupported-cache-action"));
  assert.ok(routeSurface?.source_markers?.includes("export async function POST"));

  const serverModule = await import(pathToFileURL(path.join(root, serverPath)).href);
  const readiness = serverModule.readDataFetchingCacheReadiness({
    filter: "review",
    optimisticState: "queued",
    visibleProjectCount: 2,
  });
  assert.equal(readiness.route, "/api/query-cache/readiness");
  assert.equal(readiness.packageId, "tanstack/query");
  assert.equal(readiness.officialPackageName, "Data Fetching & Cache");
  assert.equal(readiness.status, "source-owned-cache-readiness");
  assert.equal(readiness.runtimeProof, false);
  assert.equal(readiness.networkCalls, false);
  assert.equal(readiness.adapterBoundary, "queryclient-adapter-required");
  assert.equal(readiness.cache.queryKey, "dx:dashboard:projects:review");
  assert.equal(readiness.cache.readyEntryCount, 2);
  assert.equal(readiness.cache.optimisticEntryCount, 2);

  const response = serverModule.createDataFetchingCacheReadinessResponse(
    new Request("https://dx.local/api/query-cache/readiness?filter=review&optimistic=queued&visible=2"),
  );
  assert.equal(response.status, 200);
  const payload = await response.json();
  assert.equal(payload.cache.queryKey, "dx:dashboard:projects:review");
  assert.equal(payload.cache.optimisticEntryCount, 2);
  assert.equal(payload.adapterBoundary, "queryclient-adapter-required");

  const actionResponse = await serverModule.createDataFetchingCacheActionResponse(
    new Request("https://dx.local/api/query-cache/readiness", {
      method: "POST",
      body: JSON.stringify({
        action: "invalidate",
        queryKey: "dx:dashboard:projects:review",
        filter: "review",
        optimisticState: "queued",
        visibleProjectCount: 2,
      }),
    }),
  );
  assert.equal(actionResponse.status, 200);
  const actionPayload = await actionResponse.json();
  assert.equal(actionPayload.schema, "dx.www.template.data_fetching_cache_action_receipt");
  assert.equal(actionPayload.status, "source-owned-cache-action-dry-run");
  assert.equal(actionPayload.packageId, "tanstack/query");
  assert.equal(actionPayload.action, "invalidate");
  assert.equal(actionPayload.queryKey, "dx:dashboard:projects:review");
  assert.equal(actionPayload.cache.invalidatedEntryCount, 1);
  assert.equal(actionPayload.runtimeProof, false);
  assert.equal(actionPayload.networkCalls, false);
  assert.equal(actionPayload.queryClientExecution, false);
  assert.deepEqual(actionPayload.secretValues, []);

  const invalidActionResponse = await serverModule.createDataFetchingCacheActionResponse(
    new Request("https://dx.local/api/query-cache/readiness", {
      method: "POST",
      body: JSON.stringify({
        action: "delete",
        queryKey: "dx:dashboard:projects:review",
      }),
    }),
  );
  assert.equal(invalidActionResponse.status, 400);
  const invalidActionPayload = await invalidActionResponse.json();
  assert.equal(
    invalidActionPayload.schema,
    "dx.www.template.data_fetching_cache_action_error",
  );
  assert.equal(invalidActionPayload.status, "unsupported-cache-action");
  assert.equal(invalidActionPayload.action, "delete");
  assert.equal(invalidActionPayload.queryKey, "dx:dashboard:projects:review");
  assert.deepEqual(invalidActionPayload.allowedActions, ["invalidate", "refresh"]);
  assert.equal(invalidActionPayload.runtimeProof, false);
  assert.equal(invalidActionPayload.networkCalls, false);
  assert.equal(invalidActionPayload.queryClientExecution, false);
  assert.deepEqual(invalidActionPayload.secretValues, []);

  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-lane3-query-route-"));
  execFileSync(process.execPath, [materializer, dir], { cwd: root, stdio: "pipe" });
  assert.ok(
    fs.existsSync(path.join(dir, "app", "api", "query-cache", "readiness", "route.ts")),
  );
  assert.ok(fs.existsSync(path.join(dir, "server", "query-cache", "readiness.ts")));
  assert.ok(
    fs.existsSync(
      path.join(dir, "components", "template-app", "dashboard-query-cache.ts"),
    ),
  );
  const materializedDashboard = fs.readFileSync(
    path.join(dir, "pages", "dashboard.html"),
    "utf8",
  );
  assert.match(
    materializedDashboard,
    /data-dx-query-cache-readiness-route="\/api\/query-cache\/readiness"/,
  );
  assert.match(
    materializedDashboard,
    /data-dx-package-id="tanstack\/query"[\s\S]*data-template-module="query-refresh"/,
  );
  assert.match(
    materializedDashboard,
    /data-dx-package-id="tanstack\/query"[\s\S]*data-dx-forge-reality-level="lock-backed-adapter-boundary"/,
  );
  assert.match(
    materializedDashboard,
    /data-dx-package-id="tanstack\/query"[\s\S]*data-dx-package-maturity="adapter-boundary-readiness"/,
  );
  const materializedRoute = fs.readFileSync(
    path.join(dir, "app", "api", "query-cache", "readiness", "route.ts"),
    "utf8",
  );
  assert.match(materializedRoute, /export async function POST/);
  assert.match(materializedRoute, /createDataFetchingCacheActionResponse/);
});

test("reactive store has a source-owned template store tracked by receipts", async () => {
  const reactiveStorePath =
    "examples/template/components/template-app/dashboard-reactive-store.ts";
  const reactiveStoreSource = read(reactiveStorePath);
  const receipt = readJson(
    "examples/template/.dx/forge/receipts/packages/reactive-store.json",
  );
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const reactiveLane = status.package_lane_visibility.find(
    (entry: { package_id: string }) => entry.package_id === "reactive/store",
  );
  const dashboardSurface = reactiveLane?.selected_surfaces.find(
    (surface: { surface_id: string }) =>
      surface.surface_id === "template-reactive-dashboard-store",
  );

  assert.match(reactiveStoreSource, /createDashboardReactiveStore/);
  assert.match(reactiveStoreSource, /summarizeDashboardReactiveStore/);
  assert.match(reactiveStoreSource, /source-owned-template-store/);
  assert.match(reactiveStoreSource, /from ["']..\/..\/lib\/forge\/state\/reactive-store\/store\.ts["']/);
  assert.doesNotMatch(reactiveStoreSource, /from ["']@tanstack\/react-store["']/);
  assert.ok(receipt.files.includes(reactiveStorePath));
  assert.ok(receipt.file_hashes[reactiveStorePath]);
  assert.ok(dashboardSurface?.file_hashes?.[reactiveStorePath]);

  const reactiveModule = await import(
    pathToFileURL(path.join(root, reactiveStorePath)).href
  );
  const store = reactiveModule.createDashboardReactiveStore({
    activeModule: "reactive-context",
    filter: "all",
    optimisticState: "idle",
    queryCacheStatus: "source-owned-cache-readiness",
    theme: "dark",
    visibleProjectCount: 3,
  });
  const emissions: Array<{ filter: string; optimisticState: string }> = [];
  const subscription = store.subscribe((snapshot: { filter: string; optimisticState: string }) => {
    emissions.push({
      filter: snapshot.filter,
      optimisticState: snapshot.optimisticState,
    });
  });
  store.actions.setFilter("ready");
  store.actions.setOptimisticState("queued");
  subscription.unsubscribe();

  const summary = reactiveModule.summarizeDashboardReactiveStore(store);
  assert.equal(summary.packageId, "reactive/store");
  assert.equal(summary.runtimeBoundary, "source-owned-template-store");
  assert.equal(summary.providerBoundary, "react-context-template-provider");
  assert.equal(summary.filter, "ready");
  assert.equal(summary.optimisticState, "queued");
  assert.equal(summary.visibleProjectCount, 2);
  assert.equal(emissions.length, 2);

  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-lane3-reactive-store-"));
  execFileSync(process.execPath, [materializer, dir], { cwd: root, stdio: "pipe" });
  const materializedDashboard = fs.readFileSync(
    path.join(dir, "pages", "dashboard.html"),
    "utf8",
  );

  assert.match(
    materializedDashboard,
    /data-dx-reactive-store-runtime="source-owned-template-store"/,
  );
  assert.match(
    materializedDashboard,
    /data-dx-reactive-store-provider-boundary="react-context-template-provider"/,
  );
  assert.match(
    materializedDashboard,
    /data-dx-reactive-store-snapshot-key="dashboard:all:idle:dark"/,
  );
});

test("lane 3 receipt helpers track current source-owned runtime files", () => {
  for (const helper of [
    "examples/template/state-management-receipt-hashes.ts",
    "examples/template/data-fetching-cache-receipt-hashes.ts",
  ]) {
    const report = runReceiptHelper(helper);

    assert.equal(report.status, "current");
    assert.equal(report.stale_file_count, 0);
    assert.equal(report.missing_file_count, 0);
    assert.ok(report.tracked_files.includes("tools/launch/runtime-template/assets/launch-runtime.ts"));
    assert.equal(
      report.tracked_files.includes("tools/launch/runtime-template/assets/launch-runtime.js"),
      false,
    );
  }

  const reactiveReport = runReceiptHelper(
    "examples/template/reactive-store-receipt-hashes.ts",
  );
  assert.equal(reactiveReport.status, "current");
  assert.equal(reactiveReport.stale_file_count, 0);
  assert.equal(reactiveReport.missing_file_count, 0);
  assert.ok(
    reactiveReport.tracked_files.includes(
      "examples/template/components/template-app/dashboard-reactive-store.ts",
    ),
  );
});
