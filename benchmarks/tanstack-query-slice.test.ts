const assert = require("assert");
const { execFileSync } = require("child_process");
const fs = require("fs");
const os = require("os");
const path = require("path");
const test = require("node:test");
const vm = require("vm");

const root = path.resolve(__dirname, "..");
const launchMaterializer = path.join(
  root,
  "tools",
  "launch",
  "materialize-www-template.ts",
);

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("tanstack query slice materializes mutation and invalidation helpers", () => {
  const source = read("core/src/ecosystem/forge_tanstack_query.rs");

  assert.match(source, /"js\/query\/mutation\.ts", TANSTACK_QUERY_MUTATION_TS/);
  assert.match(source, /const TANSTACK_QUERY_MUTATION_TS: &str = r#"/);
  assert.match(source, /mutationOptions/);
  assert.match(source, /UseMutationOptions/);
  assert.match(source, /InvalidateQueryFilters/);
  assert.match(source, /invalidateDxQueries/);
  assert.match(source, /"query\/mutation\.ts"/);
  assert.match(source, /`query\/mutation\.ts` adds typed mutation options and cache invalidation helpers/);
});

test("tanstack query launch companion proves mutation invalidation wiring", () => {
  const status = read("examples/template/query-cache-status.tsx");

  assert.match(status, /useMutation/);
  assert.match(status, /useDxRequiredQueryClient/);
  assert.match(status, /dxMutationOptions/);
  assert.match(status, /invalidateDxQueries/);
  assert.match(status, /mutationKey: \["dx", "launch", "query-cache-refresh"\] as const/);
  assert.match(status, /launchCacheStatusQuery\.queryKey/);
  assert.match(status, /data-query-refresh-state=/);
});

test("tanstack query launch companion powers dashboard data reads", () => {
  const status = read("examples/template/query-cache-status.tsx");
  const readModel = read("examples/template/query-dashboard-read-model.ts");
  const shell = read("examples/template/template-shell.tsx");
  const catalog = read("examples/template/package-catalog.ts");
  const editContract = read("examples/template/dx-studio-edit-contract.ts");
  const routeContract = read("examples/template/template-route-contract.ts");
  const cli = read("dx-www/src/cli/mod.rs");
  const studioManifest = read("dx-www/src/cli/studio_manifest.rs");
  const liveGuard = read("benchmarks/launch-live-runtime-guard.test.ts");
  const receipt = read(
    "examples/template/.dx/forge/receipts/2026-05-22-tanstack-query-dashboard-data.json",
  );

  assert.match(status, /from "\.\/query-dashboard-read-model"/);
  assert.match(status, /readLaunchQueryDashboardData/);
  assert.doesNotMatch(status, /from "\.\/package-catalog"/);
  assert.doesNotMatch(status, /function readLaunchQueryDashboardData/);
  assert.doesNotMatch(status, /function readLaunchQueryDashboardPackages/);
  assert.doesNotMatch(status, /type LaunchQueryDashboardPackage =/);
  assert.match(status, /queryKey: \["dx", "launch", "dashboard", "read-model"\] as const/);
  assert.match(status, /data-dx-component="tanstack-query-dashboard-data-workflow"/);
  assert.match(status, /data-dx-dashboard-workflow="query-backed-dashboard-data"/);
  assert.match(status, /data-dx-product-surface="launch-dashboard"/);
  assert.match(status, /data-dx-query-dashboard-queue="package-readiness"/);
  assert.match(status, /data-dx-query-dashboard-package-count=/);
  assert.match(status, /data-dx-query-dashboard-role-count=/);
  assert.match(status, /data-dx-query-dashboard-required-env-count=/);
  assert.match(status, /data-dx-query-dashboard-source=/);
  assert.match(status, /data-dx-query-dashboard-row=/);
  assert.match(status, /data-dx-query-package-id=/);
  assert.match(status, /data-dx-query-package-role=/);
  assert.match(status, /data-dx-query-package-status=/);
  assert.match(status, /data-dx-query-action="refresh-dashboard-data"/);
  assert.match(status, /data-dx-query-action="fetch-dashboard-data-now"/);
  assert.doesNotMatch(status, /Fetching \/api\/health/);

  assert.match(readModel, /launchPackageCatalog/);
  assert.match(readModel, /launchPackageRoleSummary/);
  assert.match(readModel, /requiredLaunchEnv/);
  assert.match(readModel, /export type LaunchQueryDashboardData =/);
  assert.match(readModel, /export type LaunchQueryDashboardPackage =/);
  assert.match(readModel, /export function readLaunchQueryDashboardData/);
  assert.match(readModel, /export function readLaunchQueryDashboardPackages/);
  assert.match(readModel, /dashboardPackages: readonly LaunchQueryDashboardPackage\[\]/);
  assert.match(readModel, /preferredPackageOrder/);
  assert.match(readModel, /source: "launch-package-catalog"/);
  assert.match(readModel, /requiredEnv\.size > 0 \? "needs-env" : "ready"/);

  assert.match(shell, /selector: '\[data-dx-component="tanstack-query-dashboard-data-workflow"\]'/);
  assert.match(shell, /Query-backed dashboard data/);
  assert.match(shell, /data-dx-dashboard-workflow="query-backed-dashboard-data"/);
  assert.match(catalog, /dashboardUsage: \{/);
  assert.match(catalog, /query-backed-dashboard-data/);
  assert.match(catalog, /"query\/dashboard-workflow\.ts"/);
  assert.match(catalog, /"examples\/template\/query-dashboard-read-model\.ts"/);
  assert.match(catalog, /2026-05-22-tanstack-query-dashboard-data\.json/);

  assert.match(cli, /const NEXT_FAMILIAR_QUERY_DASHBOARD_READ_MODEL_TS: &str =/);
  assert.match(
    cli,
    /include_str!\("..\/..\/..\/examples\/template\/query-dashboard-read-model\.ts"\)/,
  );
  assert.match(cli, /"components\/template-app\/query-dashboard-read-model\.ts"/);
  assert.match(
    studioManifest,
    /"read_model_source_file": "examples\/template\/query-dashboard-read-model\.ts"/,
  );
  assert.match(
    studioManifest,
    /"read_model_materialized_file": "components\/template-app\/query-dashboard-read-model\.ts"/,
  );

  assert.match(editContract, /id: "tanstack-query-dashboard-data"/);
  assert.match(
    editContract,
    /selector: '\[data-dx-component="tanstack-query-dashboard-data-workflow"\]'/,
  );
  assert.match(
    editContract,
    /sourceFile: "examples\/template\/query-cache-status\.tsx"/,
  );
  assert.match(
    editContract,
    /materializedFile: "components\/template-app\/query-cache-status\.tsx"/,
  );
  assert.match(editContract, /packageIds: \["tanstack\/query"\]/);

  assert.match(routeContract, /tanstackQueryDashboardData:/);
  assert.match(routeContract, /packageId: "tanstack\/query"/);
  assert.match(routeContract, /component: "tanstack-query-dashboard-data-workflow"/);
  assert.match(routeContract, /dashboardWorkflow: "query-backed-dashboard-data"/);
  assert.match(routeContract, /readModelSourceFile: "examples\/template\/query-dashboard-read-model\.ts"/);
  assert.match(routeContract, /materializedReceiptFile:\s*\r?\n\s*"\.dx\/forge\/receipts\/2026-05-22-tanstack-query-dashboard-data\.json"/);
  assert.match(
    routeContract,
    /sourceGuard: "dx run --test \.\\\\benchmarks\\\\tanstack-query-slice\.test\.ts"/,
  );

  assert.match(liveGuard, /data-dx-package="tanstack\\\/query"/);
  assert.match(liveGuard, /data-dx-dashboard-workflow="query-backed-dashboard-data"/);
  assert.match(liveGuard, /data-dx-query-dashboard-source="launch-runtime-catalog"/);
  assert.match(liveGuard, /data-dx-query-dashboard-queue="package-readiness"/);
  assert.match(liveGuard, /data-dx-query-package-id="tanstack\\\/query"/);
  assert.match(liveGuard, /data-dx-query-package-status="ready"/);
  assert.match(liveGuard, /data-dx-query-action="refresh-dashboard-data"/);
  assert.match(liveGuard, /data-dx-query-safe-action="read-dashboard-catalog"/);
  assert.match(liveGuard, /id="query-package-count"/);
  assert.match(liveGuard, /id="mission-query-status"/);
  assert.match(liveGuard, /live preview manifest exposes TanStack Query dashboard workflow/);
  assert.match(liveGuard, /await get\("\/public\/preview-manifest\.json"\)/);
  assert.match(liveGuard, /launch-runtime-query-dashboard-data/);
  assert.match(liveGuard, /data-dx-query-dashboard-source/);
  assert.match(liveGuard, /2026-05-22-tanstack-query-dashboard-data\.json/);

  assert.match(receipt, /"package_id": "tanstack\/query"/);
  assert.match(receipt, /"component": "tanstack-query-dashboard-data-workflow"/);
  assert.match(receipt, /"workflow": "query-backed-dashboard-data"/);
  assert.match(receipt, /"product_surface": "launch-dashboard"/);
  assert.match(receipt, /"coding_score": 99/);
  assert.match(receipt, /"examples\/template\/query-dashboard-read-model\.ts"/);
  assert.match(receipt, /"\.dx\/forge\/receipts\/2026-05-22-tanstack-query-dashboard-data\.json"/);
  assert.match(receipt, /"data-dx-component=\\"tanstack-query-dashboard-data-workflow\\""/);
  assert.match(receipt, /"data-dx-dashboard-workflow=\\"query-backed-dashboard-data\\""/);
  assert.match(receipt, /"data-dx-query-dashboard-queue=\\"package-readiness\\""/);
  assert.match(receipt, /"data-dx-query-package-id"/);
  assert.match(receipt, /"no_runtime_execution": true/);
});

test("tanstack query launch runtime proof is browser-visible without node_modules", () => {
  const projectDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-query-launch-"));
  fs.mkdirSync(path.join(projectDir, "app", "launch"), { recursive: true });
  fs.writeFileSync(
    path.join(projectDir, "app", "launch", "page.tsx"),
    "export default function Page(){ return <>{children}</>; }\n",
  );

  const output = execFileSync(process.execPath, [launchMaterializer, projectDir], {
    cwd: root,
    encoding: "utf8",
  });
  const result = JSON.parse(output);
  const launch = fs.readFileSync(path.join(projectDir, "pages", "index.html"), "utf8");
  const runtime = fs.readFileSync(
    path.join(projectDir, "public", "launch-runtime.js"),
    "utf8",
  );
  const manifest = JSON.parse(
    fs.readFileSync(path.join(projectDir, "public", "preview-manifest.json"), "utf8"),
  );

  assert.equal(result.ok, true);
  assert.equal(result.noNodeModules, true);
  assert.ok(!fs.existsSync(path.join(projectDir, "node_modules")));
  assert.ok(
    manifest.routes.some((route) =>
      route.route === "/" && route.forgePackages.includes("tanstack/query"),
    ),
  );
  assert.ok(result.files.includes(".dx/forge/receipts/2026-05-22-tanstack-query-dashboard-data.json"));
  assert.ok(
    fs.existsSync(
      path.join(
        projectDir,
        ".dx",
        "forge",
        "receipts",
        "2026-05-22-tanstack-query-dashboard-data.json",
      ),
    ),
  );
  const launchRoute = manifest.routes.find((route) => route.route === "/");
  assert.ok(launchRoute.dataDxMarkers.includes("data-dx-query-dashboard-source"));
  assert.ok(launchRoute.dataDxMarkers.includes("data-dx-query-dashboard-queue"));
  assert.ok(launchRoute.dataDxMarkers.includes("data-dx-query-package-id"));
  assert.ok(launchRoute.dataDxMarkers.includes("data-dx-query-package-status"));
  assert.ok(launchRoute.dataDxMarkers.includes("data-dx-query-action"));
  assert.ok(
    manifest.editContract.editableSurfaces.some(
      (surface) =>
        surface.id === "launch-runtime-query-dashboard-data" &&
        surface.selector === '[data-dx-component="tanstack-query-dashboard-data-workflow"]' &&
        surface.packageIds.includes("tanstack/query") &&
        surface.receiptPath ===
          ".dx/forge/receipts/2026-05-22-tanstack-query-dashboard-data.json",
    ),
  );
  assert.match(launch, /data-dx-component="tanstack-query-dashboard-data-workflow"/);
  assert.match(launch, /data-dx-package="tanstack\/query"/);
  assert.match(launch, /data-dx-node-modules="forbidden"/);
  assert.match(launch, /data-dx-dashboard-workflow="query-backed-dashboard-data"/);
  assert.match(launch, /data-dx-product-surface="launch-dashboard"/);
  assert.match(launch, /data-dx-query-dashboard-source="launch-runtime-catalog"/);
  assert.match(launch, /data-dx-query-dashboard-queue="package-readiness"/);
  assert.match(launch, /id="query-package-queue"/);
  assert.match(launch, /data-dx-query-dashboard-row="tanstack\/query"/);
  assert.match(launch, /data-dx-query-package-id="tanstack\/query"/);
  assert.match(launch, /data-dx-query-package-status="ready"/);
  assert.match(launch, /data-dx-query-key="\[&quot;dx&quot;,&quot;launch&quot;,&quot;dashboard&quot;,&quot;read-model&quot;\]"/);
  assert.match(launch, /data-dx-query-cache-state="idle"/);
  assert.match(launch, /data-dx-query-refresh-state="idle"/);
  assert.match(launch, /data-dx-query-interaction="refresh-dashboard-data"/);
  assert.match(launch, /data-dx-query-safe-action="read-dashboard-catalog"/);
  assert.match(launch, /data-dx-query-safe-action-state="idle"/);
  assert.match(launch, /data-dx-query-result-status="idle"/);
  assert.match(launch, /id="query-cache-runs"/);
  assert.match(launch, /id="query-cache-updated"/);
  assert.match(launch, /id="query-package-count"/);
  assert.match(launch, /id="mission-query-status"/);
  assert.doesNotMatch(launch, /data-dx-query-endpoint="\/api\/health"/);

  assert.match(runtime, /const \$\$ = \(selector\) => Array\.from\(document\.querySelectorAll\(selector\)\)/);
  assert.match(runtime, /readLaunchQueryDashboardData/);
  assert.match(runtime, /launchQueryDashboardPackages/);
  assert.match(runtime, /renderQueryPackageQueue/);
  assert.match(runtime, /data-dx-query-package-status/);
  assert.match(runtime, /queryCard\.setAttribute\("data-dx-query-cache-state", "fetching"\)/);
  assert.match(runtime, /queryButton\.setAttribute\("data-dx-query-refresh-state", "fetching"\)/);
  assert.match(runtime, /queryButton\.setAttribute\("data-dx-query-safe-action-state", "fetching"\)/);
  assert.match(runtime, /queryButton\.setAttribute\("data-dx-query-safe-action-state", "completed"\)/);
  assert.match(runtime, /queryStatus\.setAttribute\("data-dx-query-result-status", "success"\)/);
  assert.match(runtime, /queryStatus\.setAttribute\("data-dx-query-result-status", "error"\)/);
  assert.match(runtime, /state\.queryDashboardData = readLaunchQueryDashboardData\(\)/);
  assert.doesNotMatch(runtime, /Fetching \/api\/health/);
});

test("tanstack query launch runtime refresh mutates visible browser state", async () => {
  function createElement() {
    return {
      attributes: {},
      dataset: {},
      innerHTML: "",
      listeners: {},
      textContent: "",
      addEventListener(event, handler) {
        this.listeners[event] = handler;
      },
      setAttribute(name, value) {
        this.attributes[name] = String(value);
      },
    };
  }

  const queryCard = createElement();
  const queryButton = createElement();
  const queryStatus = createElement();
  const queryRuns = createElement();
  const queryUpdated = createElement();
  const queryPackageCount = createElement();
  const queryRoleCount = createElement();
  const queryRequiredEnvCount = createElement();
  const queryPackageQueue = createElement();
  const missionQueryStatus = createElement();
  const missionQueryDetail = createElement();
  const elements = new Map([
    ['[data-dx-component="tanstack-query-dashboard-data-workflow"]', queryCard],
    ["#query-refresh", queryButton],
    ["#query-status", queryStatus],
    ["#query-cache-runs", queryRuns],
    ["#query-cache-updated", queryUpdated],
    ["#query-package-count", queryPackageCount],
    ["#query-role-count", queryRoleCount],
    ["#query-required-env-count", queryRequiredEnvCount],
    ["#query-package-queue", queryPackageQueue],
    ["#mission-query-status", missionQueryStatus],
    ["#mission-query-detail", missionQueryDetail],
  ]);

  vm.runInNewContext(read("tools/launch/runtime-template/assets/launch-runtime.ts"), {
    HTMLCanvasElement: class {},
    WebAssembly,
    document: {
      documentElement: createElement(),
      addEventListener(event, handler) {
        if (event === "DOMContentLoaded") handler();
      },
      querySelector(selector) {
        return elements.get(selector) ?? null;
      },
      querySelectorAll() {
        return [];
      },
    },
    fetch: async () => ({
      json: async () => ({ message: "health ok" }),
      ok: true,
    }),
    localStorage: {
      getItem: () => null,
      setItem: () => undefined,
    },
  });

  assert.equal(typeof queryButton.listeners.click, "function");

  await queryButton.listeners.click();

  assert.equal(queryCard.attributes["data-dx-query-cache-state"], "fresh");
  assert.equal(queryButton.attributes["data-dx-query-refresh-state"], "success");
  assert.equal(queryButton.attributes["data-dx-query-safe-action-state"], "completed");
  assert.equal(queryStatus.attributes["data-dx-query-result-status"], "success");
  assert.equal(queryStatus.attributes["data-dx-query-cache-entry"], "fresh");
  assert.equal(queryCard.attributes["data-dx-query-dashboard-source"], "launch-runtime-catalog");
  assert.equal(queryCard.attributes["data-dx-query-dashboard-package-count"], "30");
  assert.equal(queryCard.attributes["data-dx-query-dashboard-queue"], "package-readiness");
  assert.match(queryPackageQueue.innerHTML, /data-dx-query-package-id="tanstack\/query"/);
  assert.match(queryPackageQueue.innerHTML, /data-dx-query-package-status="ready"/);
  assert.equal(queryRuns.textContent, "1");
  assert.equal(queryPackageCount.textContent, "30");
  assert.match(queryStatus.textContent, /Dashboard data refreshed from launch-runtime-catalog/);
  assert.match(missionQueryStatus.textContent, /30 packages/);
});

test("tanstack query slice materializes mutation result helpers", () => {
  const source = read("core/src/ecosystem/forge_tanstack_query.rs");
  const status = read("examples/template/query-cache-status.tsx");

  assert.match(source, /"js\/query\/mutation-result\.ts",\s*TANSTACK_QUERY_MUTATION_RESULT_TS/);
  assert.match(source, /const TANSTACK_QUERY_MUTATION_RESULT_TS: &str = r#"/);
  assert.match(source, /UseMutationResult/);
  assert.match(source, /UseMutateFunction/);
  assert.match(source, /UseMutateAsyncFunction/);
  assert.match(source, /MutateFunction/);
  assert.match(source, /MutateOptions/);
  assert.match(source, /MutationFunction/);
  assert.match(source, /MutationFunctionContext/);
  assert.match(source, /MutationObserverBaseResult/);
  assert.match(source, /summarizeDxMutationResult/);
  assert.match(source, /formatDxMutationResultError/);
  assert.match(source, /canReset/);
  assert.match(source, /"UseMutationResult"/);
  assert.match(source, /"UseMutateAsyncFunction"/);
  assert.match(source, /"MutationFunctionContext"/);
  assert.match(source, /"query\/mutation-result\.ts"/);
  assert.match(source, /`query\/mutation-result\.ts` adds mutation result helpers/);

  assert.match(status, /summarizeDxMutationResult/);
  assert.match(status, /formatDxMutationResultError/);
  assert.match(status, /from "@\/lib\/query\/mutation-result"/);
  assert.match(status, /launchRefreshMutationSummary/);
  assert.match(status, /launchRefreshMutationErrorMessage/);
  assert.match(status, /data-mutation-result-status=/);
  assert.match(status, /data-mutation-result-can-reset=/);
  assert.match(status, /data-mutation-result-failure-count=/);
});

test("tanstack query slice materializes query result helpers", () => {
  const source = read("core/src/ecosystem/forge_tanstack_query.rs");
  const status = read("examples/template/query-cache-status.tsx");

  assert.match(source, /"js\/query\/query-result\.ts",\s*TANSTACK_QUERY_RESULT_TS/);
  assert.match(source, /const TANSTACK_QUERY_RESULT_TS: &str = r#"/);
  assert.match(source, /UseBaseQueryResult/);
  assert.match(source, /UseQueryResult/);
  assert.match(source, /DefinedUseQueryResult/);
  assert.match(source, /UseInfiniteQueryResult/);
  assert.match(source, /DefinedUseInfiniteQueryResult/);
  assert.match(source, /UseSuspenseQueryResult/);
  assert.match(source, /UseSuspenseInfiniteQueryResult/);
  assert.match(source, /QueryObserverBaseResult/);
  assert.match(source, /InfiniteQueryObserverBaseResult/);
  assert.match(source, /summarizeDxQueryResult/);
  assert.match(source, /summarizeDxInfiniteQueryResult/);
  assert.match(source, /formatDxQueryResultError/);
  assert.match(source, /canRefetch/);
  assert.match(source, /"UseQueryResult"/);
  assert.match(source, /"UseInfiniteQueryResult"/);
  assert.match(source, /"UseSuspenseQueryResult"/);
  assert.match(source, /"query\/query-result\.ts"/);
  assert.match(source, /`query\/query-result\.ts` adds query result helpers/);

  assert.match(status, /summarizeDxQueryResult/);
  assert.match(status, /formatDxQueryResultError/);
  assert.match(status, /from "@\/lib\/query\/query-result"/);
  assert.match(status, /launchStatusQuerySummary/);
  assert.match(status, /launchStatusQueryErrorMessage/);
  assert.match(status, /data-query-result-status=/);
  assert.match(status, /data-query-result-fetch-status=/);
  assert.match(status, /data-query-result-can-refetch=/);
});

test("tanstack query slice materializes React context and option type helpers", () => {
  const source = read("core/src/ecosystem/forge_tanstack_query.rs");
  const status = read("examples/template/query-cache-status.tsx");

  assert.match(source, /"js\/query\/react-context\.tsx",\s*TANSTACK_QUERY_REACT_CONTEXT_TSX/);
  assert.match(source, /const TANSTACK_QUERY_REACT_CONTEXT_TSX: &str = r#"/);
  assert.match(source, /QueryClientContext/);
  assert.match(source, /useQueryClient/);
  assert.match(source, /QueryClientProviderProps/);
  assert.match(source, /HydrationBoundaryProps/);
  assert.match(source, /DefinedInitialDataOptions/);
  assert.match(source, /UndefinedInitialDataOptions/);
  assert.match(source, /UnusedSkipTokenOptions/);
  assert.match(source, /DefinedInitialDataInfiniteOptions/);
  assert.match(source, /UndefinedInitialDataInfiniteOptions/);
  assert.match(source, /UnusedSkipTokenInfiniteOptions/);
  assert.match(source, /DxQueryClientContextBridge/);
  assert.match(source, /useDxRequiredQueryClient/);
  assert.match(source, /readDxQueryClientContextStatus/);
  assert.match(source, /DxHydrationBoundary/);
  assert.match(source, /"QueryClientContext"/);
  assert.match(source, /"useQueryClient"/);
  assert.match(source, /"QueryClientProviderProps"/);
  assert.match(source, /"HydrationBoundaryProps"/);
  assert.match(source, /"DefinedInitialDataOptions"/);
  assert.match(source, /"UnusedSkipTokenInfiniteOptions"/);
  assert.match(source, /"query\/react-context\.tsx"/);
  assert.match(source, /`query\/react-context\.tsx` exposes React context helpers/);

  assert.match(status, /useDxRequiredQueryClient/);
  assert.match(status, /readDxQueryClientContextStatus/);
  assert.match(status, /from "@\/lib\/query\/react-context"/);
  assert.match(status, /launchQueryClientContextStatus/);
  assert.match(status, /data-query-client-context-state=/);
  assert.match(status, /data-query-client-context-provider=/);
});

test("tanstack query slice materializes error reset boundary helpers", () => {
  const source = read("core/src/ecosystem/forge_tanstack_query.rs");

  assert.match(source, /"js\/query\/error-boundary\.tsx",\s*TANSTACK_QUERY_ERROR_BOUNDARY_TSX/);
  assert.match(source, /const TANSTACK_QUERY_ERROR_BOUNDARY_TSX: &str = r#"/);
  assert.match(source, /QueryErrorResetBoundary/);
  assert.match(source, /useQueryErrorResetBoundary/);
  assert.match(source, /DxQueryErrorResetBoundary/);
  assert.match(source, /useDxQueryErrorResetBoundary/);
  assert.match(source, /DxQueryErrorFallback/);
  assert.match(source, /formatDxQueryErrorMessage/);
  assert.match(source, /role="alert"/);
  assert.match(source, /Retry query/);
  assert.match(source, /"query\/error-boundary\.tsx"/);
  assert.match(source, /`query\/error-boundary\.tsx` exposes a tiny reset boundary/);
});

test("tanstack query slice materializes cancellation error helpers", () => {
  const source = read("core/src/ecosystem/forge_tanstack_query.rs");
  const status = read("examples/template/query-cache-status.tsx");

  assert.match(source, /"js\/query\/errors\.ts",\s*TANSTACK_QUERY_ERRORS_TS/);
  assert.match(source, /const TANSTACK_QUERY_ERRORS_TS: &str = r#"/);
  assert.match(source, /CancelledError/);
  assert.match(source, /isCancelledError/);
  assert.match(source, /shouldThrowError/);
  assert.match(source, /CancelOptions/);
  assert.match(source, /ThrowOnError/);
  assert.match(source, /createDxCancelledQueryError/);
  assert.match(source, /isDxCancelledQueryError/);
  assert.match(source, /getDxQueryErrorKind/);
  assert.match(source, /shouldThrowDxQueryError/);
  assert.match(source, /formatDxQueryErrorMessage/);
  assert.match(source, /"CancelledError"/);
  assert.match(source, /"isCancelledError"/);
  assert.match(source, /"shouldThrowError"/);
  assert.match(source, /"query\/errors\.ts"/);
  assert.match(source, /`query\/errors\.ts` adds cancellation and throw-policy helpers/);

  assert.match(status, /getDxQueryErrorKind/);
  assert.match(status, /isDxCancelledQueryError/);
  assert.match(status, /from "@\/lib\/query\/errors"/);
  assert.match(status, /launchQueryErrorKind/);
  assert.match(status, /data-query-error-kind=/);
});

test("tanstack query slice materializes infinite query helpers", () => {
  const source = read("core/src/ecosystem/forge_tanstack_query.rs");

  assert.match(source, /"js\/query\/infinite\.ts",\s*TANSTACK_QUERY_INFINITE_TS/);
  assert.match(source, /const TANSTACK_QUERY_INFINITE_TS: &str = r#"/);
  assert.match(source, /infiniteQueryOptions/);
  assert.match(source, /prefetchInfiniteQuery/);
  assert.match(source, /flattenDxInfiniteItems/);
  assert.match(source, /getDxNextPageParam/);
  assert.match(source, /"useInfiniteQuery"/);
  assert.match(source, /"query\/infinite\.ts"/);
  assert.match(source, /`query\/infinite\.ts` adds typed infinite-query helpers/);
});

test("tanstack query slice materializes query activity helpers", () => {
  const source = read("core/src/ecosystem/forge_tanstack_query.rs");
  const status = read("examples/template/query-cache-status.tsx");

  assert.match(source, /"js\/query\/activity\.tsx",\s*TANSTACK_QUERY_ACTIVITY_TSX/);
  assert.match(source, /const TANSTACK_QUERY_ACTIVITY_TSX: &str = r#"/);
  assert.match(source, /useIsFetching/);
  assert.match(source, /useIsMutating/);
  assert.match(source, /useMutationState/);
  assert.match(source, /DxQueryActivityStatus/);
  assert.match(source, /useDxQueryActivity/);
  assert.match(source, /getDxLatestMutationState/);
  assert.match(source, /"query\/activity\.tsx"/);
  assert.match(source, /`query\/activity\.tsx` adds launch-ready cache activity helpers/);

  assert.match(status, /DxQueryActivityStatus/);
  assert.match(status, /useDxQueryActivity/);
  assert.match(status, /useDxPendingMutationState/);
  assert.match(status, /getDxLatestMutationState/);
  assert.match(status, /from "@\/lib\/query\/activity"/);
  assert.match(status, /launchQueryActivity/);
  assert.match(status, /launchPendingMutationStates/);
  assert.match(status, /launchLatestPendingMutationState/);
  assert.match(status, /data-query-activity-state=/);
  assert.match(status, /data-query-pending-mutation-count=/);
  assert.match(status, /data-query-latest-pending-mutation-status=/);
});

test("tanstack query slice materializes observer bridge helpers", () => {
  const source = read("core/src/ecosystem/forge_tanstack_query.rs");
  const status = read("examples/template/query-cache-status.tsx");

  assert.match(source, /"js\/query\/observers\.ts",\s*TANSTACK_QUERY_OBSERVERS_TS/);
  assert.match(source, /const TANSTACK_QUERY_OBSERVERS_TS: &str = r#"/);
  assert.match(source, /QueryObserver/);
  assert.match(source, /InfiniteQueryObserver/);
  assert.match(source, /MutationObserver/);
  assert.match(source, /QueriesObserver/);
  assert.match(source, /QueryObserverOptions/);
  assert.match(source, /InfiniteQueryObserverOptions/);
  assert.match(source, /MutationObserverOptions/);
  assert.match(source, /QueryObserverResult/);
  assert.match(source, /MutationObserverResult/);
  assert.match(source, /createDxQueryObserver/);
  assert.match(source, /createDxInfiniteQueryObserver/);
  assert.match(source, /createDxMutationObserver/);
  assert.match(source, /createDxQueriesObserver/);
  assert.match(source, /subscribeDxQueryObserver/);
  assert.match(source, /readDxQueryObserverSnapshot/);
  assert.match(source, /readDxInfiniteQueryObserverSnapshot/);
  assert.match(source, /readDxMutationObserverSnapshot/);
  assert.match(source, /"QueryObserver"/);
  assert.match(source, /"InfiniteQueryObserver"/);
  assert.match(source, /"MutationObserver"/);
  assert.match(source, /"query\/observers\.ts"/);
  assert.match(source, /`query\/observers\.ts` adds non-React observer bridge helpers/);

  assert.match(status, /readDxQueryObserverSnapshot/);
  assert.match(status, /from "@\/lib\/query\/observers"/);
  assert.match(status, /launchQueryObserverSnapshot/);
  assert.match(status, /data-query-observer-status=/);
  assert.match(status, /data-query-observer-fetch-status=/);
});

test("tanstack query slice materializes cache control helpers", () => {
  const source = read("core/src/ecosystem/forge_tanstack_query.rs");
  const status = read("examples/template/query-cache-status.tsx");

  assert.match(source, /"js\/query\/cache\.ts",\s*TANSTACK_QUERY_CACHE_TS/);
  assert.match(source, /const TANSTACK_QUERY_CACHE_TS: &str = r#"/);
  assert.match(source, /ensureDxQueryData/);
  assert.match(source, /readDxQueryData/);
  assert.match(source, /writeDxQueryData/);
  assert.match(source, /readDxQueryState/);
  assert.match(source, /countDxFetchingQueries/);
  assert.match(source, /countDxMutatingRequests/);
  assert.match(source, /readDxQueriesData/);
  assert.match(source, /writeDxQueriesData/);
  assert.match(source, /refetchDxQueries/);
  assert.match(source, /resetDxQueries/);
  assert.match(source, /cancelDxQueries/);
  assert.match(source, /removeDxQueries/);
  assert.match(source, /dxExactQueryFilter/);
  assert.match(source, /"getQueriesData"/);
  assert.match(source, /"setQueriesData"/);
  assert.match(source, /"refetchQueries"/);
  assert.match(source, /"resetQueries"/);
  assert.match(source, /"getQueryState"/);
  assert.match(source, /"isFetching"/);
  assert.match(source, /"isMutating"/);
  assert.match(source, /"ensureQueryData"/);
  assert.match(source, /"query\/cache\.ts"/);
  assert.match(source, /`query\/cache\.ts` adds typed cache control helpers/);

  assert.match(status, /readDxQueriesData/);
  assert.match(status, /readDxQueryState/);
  assert.match(status, /countDxFetchingQueries/);
  assert.match(status, /countDxMutatingRequests/);
  assert.match(status, /launchCacheSnapshotCount/);
  assert.match(status, /launchCacheQueryState/);
  assert.match(status, /data-query-cache-matches=/);
  assert.match(status, /data-query-fetching-count=/);
  assert.match(status, /data-query-mutating-count=/);
  assert.match(status, /data-query-data-updated-at=/);
});

test("tanstack query slice materializes suspense query helpers", () => {
  const source = read("core/src/ecosystem/forge_tanstack_query.rs");

  assert.match(source, /"js\/query\/suspense\.tsx",\s*TANSTACK_QUERY_SUSPENSE_TSX/);
  assert.match(source, /const TANSTACK_QUERY_SUSPENSE_TSX: &str = r#"/);
  assert.match(source, /useSuspenseQuery/);
  assert.match(source, /useSuspenseInfiniteQuery/);
  assert.match(source, /useSuspenseQueries/);
  assert.match(source, /useDxSuspenseQuery/);
  assert.match(source, /useDxSuspenseInfiniteQuery/);
  assert.match(source, /useDxSuspenseQueries/);
  assert.match(source, /SuspenseQueriesOptions/);
  assert.match(source, /"query\/suspense\.tsx"/);
  assert.match(source, /`query\/suspense\.tsx` adds typed Suspense query helpers/);
});

test("tanstack query slice materializes render prefetch hooks", () => {
  const source = read("core/src/ecosystem/forge_tanstack_query.rs");

  assert.match(source, /"js\/query\/prefetch-hooks\.tsx",\s*TANSTACK_QUERY_PREFETCH_HOOKS_TSX/);
  assert.match(source, /const TANSTACK_QUERY_PREFETCH_HOOKS_TSX: &str = r#"/);
  assert.match(source, /usePrefetchQuery/);
  assert.match(source, /usePrefetchInfiniteQuery/);
  assert.match(source, /useDxPrefetchQuery/);
  assert.match(source, /useDxPrefetchInfiniteQuery/);
  assert.match(source, /DxPrefetchOnRender/);
  assert.match(source, /FetchInfiniteQueryOptions/);
  assert.match(source, /"query\/prefetch-hooks\.tsx"/);
  assert.match(source, /`query\/prefetch-hooks\.tsx` adds render-time prefetch hooks/);
});

test("tanstack query slice materializes parallel query helpers", () => {
  const source = read("core/src/ecosystem/forge_tanstack_query.rs");

  assert.match(source, /"js\/query\/queries\.tsx",\s*TANSTACK_QUERY_QUERIES_TSX/);
  assert.match(source, /const TANSTACK_QUERY_QUERIES_TSX: &str = r#"/);
  assert.match(source, /useQueries/);
  assert.match(source, /QueriesOptions/);
  assert.match(source, /QueriesResults/);
  assert.match(source, /UseQueryResult/);
  assert.match(source, /useDxQueries/);
  assert.match(source, /getDxQueriesSummary/);
  assert.match(source, /DxQueriesStatus/);
  assert.match(source, /data-query-batch-state/);
  assert.match(source, /"QueriesObserver"/);
  assert.match(source, /"query\/queries\.tsx"/);
  assert.match(source, /`query\/queries\.tsx` adds typed parallel query helpers/);
});

test("tanstack query slice materializes lifecycle manager helpers", () => {
  const source = read("core/src/ecosystem/forge_tanstack_query.rs");

  assert.match(source, /"js\/query\/lifecycle\.tsx",\s*TANSTACK_QUERY_LIFECYCLE_TSX/);
  assert.match(source, /const TANSTACK_QUERY_LIFECYCLE_TSX: &str = r#"/);
  assert.match(source, /focusManager/);
  assert.match(source, /onlineManager/);
  assert.match(source, /setDxQueryFocused/);
  assert.match(source, /setDxQueryOnline/);
  assert.match(source, /installDxQueryLifecycleBridge/);
  assert.match(source, /subscribeDxQueryLifecycle/);
  assert.match(source, /useDxQueryLifecycleStatus/);
  assert.match(source, /DxQueryLifecycleStatus/);
  assert.match(source, /data-query-online-state/);
  assert.match(source, /"focusManager"/);
  assert.match(source, /"onlineManager"/);
  assert.match(source, /"query\/lifecycle\.tsx"/);
  assert.match(source, /`query\/lifecycle\.tsx` adds focus and online manager helpers/);
});

test("tanstack query slice materializes runtime manager helpers", () => {
  const source = read("core/src/ecosystem/forge_tanstack_query.rs");
  const status = read("examples/template/query-cache-status.tsx");

  assert.match(source, /"js\/query\/runtime\.ts",\s*TANSTACK_QUERY_RUNTIME_TS/);
  assert.match(source, /const TANSTACK_QUERY_RUNTIME_TS: &str = r#"/);
  assert.match(source, /environmentManager/);
  assert.match(source, /timeoutManager/);
  assert.match(source, /defaultScheduler/);
  assert.match(source, /isServer/);
  assert.match(source, /ManagedTimerId/);
  assert.match(source, /TimeoutProvider/);
  assert.match(source, /readDxQueryRuntimeStatus/);
  assert.match(source, /setDxQueryServerEnvironment/);
  assert.match(source, /installDxQueryTimeoutProvider/);
  assert.match(source, /scheduleDxQueryTick/);
  assert.match(source, /"environmentManager"/);
  assert.match(source, /"timeoutManager"/);
  assert.match(source, /"defaultScheduler"/);
  assert.match(source, /"query\/runtime\.ts"/);
  assert.match(source, /`query\/runtime\.ts` adds runtime manager helpers/);

  assert.match(status, /readDxQueryRuntimeStatus/);
  assert.match(status, /from "@\/lib\/query\/runtime"/);
  assert.match(status, /launchQueryRuntimeStatus/);
  assert.match(status, /data-query-runtime-env=/);
  assert.match(status, /data-query-server-env=/);
});

test("tanstack query slice materializes streamed query helpers", () => {
  const source = read("core/src/ecosystem/forge_tanstack_query.rs");

  assert.match(source, /"js\/query\/stream\.ts",\s*TANSTACK_QUERY_STREAM_TS/);
  assert.match(source, /const TANSTACK_QUERY_STREAM_TS: &str = r#"/);
  assert.match(source, /experimental_streamedQuery/);
  assert.match(source, /dxStreamedQuery/);
  assert.match(source, /dxStreamedArrayQuery/);
  assert.match(source, /dxStreamedTextQuery/);
  assert.match(source, /decodeDxUtf8Stream/);
  assert.match(source, /QueryFunctionContext/);
  assert.match(source, /refetchMode/);
  assert.match(source, /"append"/);
  assert.match(source, /"replace"/);
  assert.match(source, /"experimental_streamedQuery"/);
  assert.match(source, /"query\/stream\.ts"/);
  assert.match(source, /`query\/stream\.ts` adds streamed query helpers/);
});

test("tanstack query slice materializes skip-token dependent query helpers", () => {
  const source = read("core/src/ecosystem/forge_tanstack_query.rs");
  const status = read("examples/template/query-cache-status.tsx");

  assert.match(source, /"js\/query\/disabled\.ts",\s*TANSTACK_QUERY_DISABLED_TS/);
  assert.match(source, /const TANSTACK_QUERY_DISABLED_TS: &str = r#"/);
  assert.match(source, /skipToken/);
  assert.match(source, /dxSkipToken/);
  assert.match(source, /dxConditionalQueryOptions/);
  assert.match(source, /dxMaybeQueryFn/);
  assert.match(source, /isDxQuerySkipped/);
  assert.match(source, /"query\/disabled\.ts"/);
  assert.match(source, /`query\/disabled\.ts` adds typed dependent-query helpers/);

  assert.match(status, /dxConditionalQueryOptions/);
  assert.match(status, /dxMaybeQueryFn/);
  assert.match(status, /launchCacheDiagnosticsQuery/);
  assert.match(status, /data-query-dependent-state=/);
});

test("tanstack query slice materializes placeholder data helpers", () => {
  const source = read("core/src/ecosystem/forge_tanstack_query.rs");
  const status = read("examples/template/query-cache-status.tsx");

  assert.match(source, /"js\/query\/placeholder\.ts",\s*TANSTACK_QUERY_PLACEHOLDER_TS/);
  assert.match(source, /const TANSTACK_QUERY_PLACEHOLDER_TS: &str = r#"/);
  assert.match(source, /keepPreviousData/);
  assert.match(source, /replaceEqualDeep/);
  assert.match(source, /PlaceholderDataFunction/);
  assert.match(source, /dxKeepPreviousData/);
  assert.match(source, /createDxPlaceholderData/);
  assert.match(source, /shareDxQueryData/);
  assert.match(source, /readDxPlaceholderState/);
  assert.match(source, /"keepPreviousData"/);
  assert.match(source, /"replaceEqualDeep"/);
  assert.match(source, /"query\/placeholder\.ts"/);
  assert.match(source, /`query\/placeholder\.ts` adds placeholder and structural-sharing helpers/);

  assert.match(status, /dxKeepPreviousData/);
  assert.match(status, /shareDxQueryData/);
  assert.match(status, /readDxPlaceholderState/);
  assert.match(status, /from "@\/lib\/query\/placeholder"/);
  assert.match(status, /launchQueryPlaceholderState/);
  assert.match(status, /data-query-placeholder-state=/);
  assert.match(status, /data-query-placeholder-source=/);
});

test("tanstack query slice materializes restore-state helpers", () => {
  const source = read("core/src/ecosystem/forge_tanstack_query.rs");
  const status = read("examples/template/query-cache-status.tsx");

  assert.match(source, /"js\/query\/restoring\.tsx",\s*TANSTACK_QUERY_RESTORING_TSX/);
  assert.match(source, /const TANSTACK_QUERY_RESTORING_TSX: &str = r#"/);
  assert.match(source, /useIsRestoring/);
  assert.match(source, /IsRestoringProvider/);
  assert.match(source, /DxQueryRestoringProvider/);
  assert.match(source, /useDxQueryRestoreStatus/);
  assert.match(source, /DxQueryRestoreStatus/);
  assert.match(source, /data-query-restore-state=/);
  assert.match(source, /"useIsRestoring"/);
  assert.match(source, /"IsRestoringProvider"/);
  assert.match(source, /"query\/restoring\.tsx"/);
  assert.match(source, /`query\/restoring\.tsx` adds cache restore-state helpers/);

  assert.match(status, /DxQueryRestoreStatus/);
  assert.match(status, /from "@\/lib\/query\/restoring"/);
  assert.match(status, /data-query-restore-slot="launch-cache"/);
});

test("tanstack query slice materializes query key helpers", () => {
  const source = read("core/src/ecosystem/forge_tanstack_query.rs");
  const status = read("examples/template/query-cache-status.tsx");

  assert.match(source, /"js\/query\/keys\.ts",\s*TANSTACK_QUERY_KEYS_TS/);
  assert.match(source, /const TANSTACK_QUERY_KEYS_TS: &str = r#"/);
  assert.match(source, /hashKey/);
  assert.match(source, /partialMatchKey/);
  assert.match(source, /dxQueryKey/);
  assert.match(source, /hashDxQueryKey/);
  assert.match(source, /isDxPartialQueryKeyMatch/);
  assert.match(source, /"hashKey"/);
  assert.match(source, /"partialMatchKey"/);
  assert.match(source, /"query\/keys\.ts"/);
  assert.match(source, /`query\/keys\.ts` adds deterministic key helpers/);

  assert.match(status, /hashDxQueryKey/);
  assert.match(status, /data-query-key-hash=/);
  assert.match(status, /launchCacheKeyHash/);
});

test("tanstack query slice materializes core query and mutation state helpers", () => {
  const source = read("core/src/ecosystem/forge_tanstack_query.rs");
  const status = read("examples/template/query-cache-status.tsx");

  assert.match(source, /"js\/query\/state\.ts",\s*TANSTACK_QUERY_STATE_TS/);
  assert.match(source, /const TANSTACK_QUERY_STATE_TS: &str = r#"/);
  assert.match(source, /type Query,/);
  assert.match(source, /type Mutation,/);
  assert.match(source, /type QueryState,/);
  assert.match(source, /type MutationState,/);
  assert.match(source, /type QueryStatus,/);
  assert.match(source, /type FetchStatus,/);
  assert.match(source, /type MutationStatus,/);
  assert.match(source, /type Updater,/);
  assert.match(source, /readDxQueryCoreState/);
  assert.match(source, /summarizeDxQueryCoreState/);
  assert.match(source, /readDxMutationCoreState/);
  assert.match(source, /summarizeDxMutationCoreState/);
  assert.match(source, /resolveDxStateUpdater/);
  assert.match(source, /findDxCachedQuery/);
  assert.match(source, /findDxCachedMutation/);
  assert.match(source, /getObserversCount/);
  assert.match(source, /"QueryState"/);
  assert.match(source, /"MutationState"/);
  assert.match(source, /"Updater"/);
  assert.match(source, /"query\/state\.ts"/);
  assert.match(source, /`query\/state\.ts` adds core query and mutation state helpers/);

  assert.match(status, /summarizeDxQueryCoreState/);
  assert.match(status, /summarizeDxMutationCoreState/);
  assert.match(status, /findDxCachedQuery/);
  assert.match(status, /findDxCachedMutation/);
  assert.match(status, /from "@\/lib\/query\/state"/);
  assert.match(status, /launchQueryCoreStateSummary/);
  assert.match(status, /launchMutationCoreStateSummary/);
  assert.match(status, /data-query-core-state=/);
  assert.match(status, /data-query-core-observers=/);
  assert.match(status, /data-mutation-core-state=/);
  assert.match(status, /data-mutation-core-failures=/);
});

test("tanstack query slice materializes cache event helpers", () => {
  const source = read("core/src/ecosystem/forge_tanstack_query.rs");
  const status = read("examples/template/query-cache-status.tsx");

  assert.match(source, /"js\/query\/cache-events\.ts",\s*TANSTACK_QUERY_CACHE_EVENTS_TS/);
  assert.match(source, /const TANSTACK_QUERY_CACHE_EVENTS_TS: &str = r#"/);
  assert.match(source, /QueryCache/);
  assert.match(source, /MutationCache/);
  assert.match(source, /QueryCacheNotifyEvent/);
  assert.match(source, /MutationCacheNotifyEvent/);
  assert.match(source, /notifyManager/);
  assert.match(source, /createDxQueryCache/);
  assert.match(source, /createDxMutationCache/);
  assert.match(source, /subscribeDxQueryCacheEvents/);
  assert.match(source, /subscribeDxMutationCacheEvents/);
  assert.match(source, /dxBatchQueryCacheNotifications/);
  assert.match(source, /summarizeDxQueryCache/);
  assert.match(source, /"QueryCache"/);
  assert.match(source, /"MutationCache"/);
  assert.match(source, /"notifyManager"/);
  assert.match(source, /"query\/cache-events\.ts"/);
  assert.match(source, /`query\/cache-events\.ts` adds cache event instrumentation helpers/);

  assert.match(status, /summarizeDxQueryCache/);
  assert.match(status, /from "@\/lib\/query\/cache-events"/);
  assert.match(status, /data-query-cache-size=/);
  assert.match(status, /launchCacheEventSummary/);
});

test("tanstack query slice materializes cache matching helpers", () => {
  const source = read("core/src/ecosystem/forge_tanstack_query.rs");
  const status = read("examples/template/query-cache-status.tsx");

  assert.match(source, /"js\/query\/matches\.ts",\s*TANSTACK_QUERY_MATCHES_TS/);
  assert.match(source, /const TANSTACK_QUERY_MATCHES_TS: &str = r#"/);
  assert.match(source, /matchQuery/);
  assert.match(source, /matchMutation/);
  assert.match(source, /QueryFilters/);
  assert.match(source, /MutationFilters/);
  assert.match(source, /getDxMatchingQueries/);
  assert.match(source, /getDxMatchingMutations/);
  assert.match(source, /countDxMatchingQueries/);
  assert.match(source, /countDxMatchingMutations/);
  assert.match(source, /summarizeDxCacheMatches/);
  assert.match(source, /"matchQuery"/);
  assert.match(source, /"matchMutation"/);
  assert.match(source, /"query\/matches\.ts"/);
  assert.match(source, /`query\/matches\.ts` adds cache matching helpers/);

  assert.match(status, /summarizeDxCacheMatches/);
  assert.match(status, /from "@\/lib\/query\/matches"/);
  assert.match(status, /launchCacheMatchSummary/);
  assert.match(status, /data-query-match-count=/);
  assert.match(status, /data-mutation-match-count=/);
});

test("tanstack query slice materializes persisted cache helpers", () => {
  const source = read("core/src/ecosystem/forge_tanstack_query.rs");
  const status = read("examples/template/query-cache-status.tsx");

  assert.match(source, /"js\/query\/persist\.tsx",\s*TANSTACK_QUERY_PERSIST_TSX/);
  assert.match(source, /const TANSTACK_QUERY_PERSIST_TSX: &str = r#"/);
  assert.match(source, /PersistQueryClientProvider/);
  assert.match(source, /persistQueryClientRestore/);
  assert.match(source, /persistQueryClientSave/);
  assert.match(source, /persistQueryClientSubscribe/);
  assert.match(source, /persistQueryClient/);
  assert.match(source, /createAsyncStoragePersister/);
  assert.match(source, /experimental_createQueryPersister/);
  assert.match(source, /removeOldestQuery/);
  assert.match(source, /createDxBrowserQueryPersister/);
  assert.match(source, /DxPersistQueryClientProvider/);
  assert.match(source, /mountDxPersistedQueryClient/);
  assert.match(source, /restoreDxPersistedQueryClient/);
  assert.match(source, /saveDxPersistedQueryClient/);
  assert.match(source, /subscribeDxPersistedQueryClient/);
  assert.match(source, /createDxFineGrainedQueryPersister/);
  assert.match(source, /readDxPersistedQueryStatus/);
  assert.match(source, /"PersistQueryClientProvider"/);
  assert.match(source, /"persistQueryClientRestore"/);
  assert.match(source, /"createAsyncStoragePersister"/);
  assert.match(source, /"experimental_createQueryPersister"/);
  assert.match(source, /"query\/persist\.tsx"/);
  assert.match(source, /`query\/persist\.tsx` adds persisted cache helpers/);

  assert.match(status, /createDxBrowserQueryPersister/);
  assert.match(status, /readDxPersistedQueryStatus/);
  assert.match(status, /from "@\/lib\/query\/persist"/);
  assert.match(status, /launchQueryPersistenceStatus/);
  assert.match(status, /data-query-persist-storage=/);
  assert.match(status, /data-query-persist-key=/);
  assert.match(status, /data-query-persist-restoring=/);
});

test("tanstack query slice materializes sync storage persister helpers", () => {
  const source = read("core/src/ecosystem/forge_tanstack_query.rs");
  const status = read("examples/template/query-cache-status.tsx");

  assert.match(source, /"js\/query\/sync-persist\.ts",\s*TANSTACK_QUERY_SYNC_PERSIST_TS/);
  assert.match(source, /const TANSTACK_QUERY_SYNC_PERSIST_TS: &str = r#"/);
  assert.match(source, /createSyncStoragePersister/);
  assert.match(source, /PersistRetryer/);
  assert.match(source, /PersistedClient/);
  assert.match(source, /Persister/);
  assert.match(source, /removeOldestQuery/);
  assert.match(source, /createDxBrowserSyncStoragePersister/);
  assert.match(source, /createDxMemorySyncStorage/);
  assert.match(source, /readDxSyncStoragePersisterStatus/);
  assert.match(source, /"createSyncStoragePersister"/);
  assert.match(source, /"@tanstack\/query-sync-storage-persister"/);
  assert.match(source, /"query\/sync-persist\.ts"/);
  assert.match(source, /`query\/sync-persist\.ts` adds legacy sync storage persister helpers/);

  assert.match(status, /createDxBrowserSyncStoragePersister/);
  assert.match(status, /readDxSyncStoragePersisterStatus/);
  assert.match(status, /from "@\/lib\/query\/sync-persist"/);
  assert.match(status, /launchQuerySyncPersister/);
  assert.match(status, /launchQuerySyncPersistenceStatus/);
  assert.match(status, /data-query-sync-persist-storage=/);
  assert.match(status, /data-query-sync-persist-key=/);
  assert.match(status, /data-query-sync-persist-deprecated=/);
});

test("tanstack query slice materializes devtools helpers", () => {
  const source = read("core/src/ecosystem/forge_tanstack_query.rs");
  const status = read("examples/template/query-cache-status.tsx");

  assert.match(source, /"js\/query\/devtools\.tsx",\s*TANSTACK_QUERY_DEVTOOLS_TSX/);
  assert.match(source, /const TANSTACK_QUERY_DEVTOOLS_TSX: &str = r#"/);
  assert.match(source, /ReactQueryDevtools/);
  assert.match(source, /ReactQueryDevtoolsPanel/);
  assert.match(source, /DevtoolsPanelOptions/);
  assert.match(source, /DxQueryDevtools/);
  assert.match(source, /DxQueryDevtoolsPanel/);
  assert.match(source, /readDxQueryDevtoolsStatus/);
  assert.match(source, /"ReactQueryDevtools"/);
  assert.match(source, /"ReactQueryDevtoolsPanel"/);
  assert.match(source, /"DevtoolsPanelOptions"/);
  assert.match(source, /"@tanstack\/react-query-devtools"/);
  assert.match(source, /"query\/devtools\.tsx"/);
  assert.match(source, /`query\/devtools\.tsx` adds opt-in React Query Devtools helpers/);

  assert.match(status, /DxQueryDevtools/);
  assert.match(status, /readDxQueryDevtoolsStatus/);
  assert.match(status, /from "@\/lib\/query\/devtools"/);
  assert.match(status, /showQueryDevtools/);
  assert.match(status, /data-query-devtools-enabled=/);
  assert.match(status, /data-query-devtools-env=/);
  assert.match(status, /data-query-devtools-panel=/);
});

test("tanstack query slice materializes broadcast cache sync helpers", () => {
  const source = read("core/src/ecosystem/forge_tanstack_query.rs");
  const status = read("examples/template/query-cache-status.tsx");

  assert.match(source, /"js\/query\/broadcast\.ts",\s*TANSTACK_QUERY_BROADCAST_TS/);
  assert.match(source, /const TANSTACK_QUERY_BROADCAST_TS: &str = r#"/);
  assert.match(source, /broadcastQueryClient/);
  assert.match(source, /BroadcastChannelOptions/);
  assert.match(source, /DX_QUERY_BROADCAST_CHANNEL/);
  assert.match(source, /mountDxQueryBroadcastClient/);
  assert.match(source, /readDxQueryBroadcastStatus/);
  assert.match(source, /createDxQueryBroadcastDisabledHandle/);
  assert.match(source, /"broadcastQueryClient"/);
  assert.match(source, /"@tanstack\/query-broadcast-client-experimental"/);
  assert.match(source, /"broadcast-channel"/);
  assert.match(source, /"query\/broadcast\.ts"/);
  assert.match(source, /`query\/broadcast\.ts` adds cross-tab cache sync helpers/);

  assert.match(status, /mountDxQueryBroadcastClient/);
  assert.match(status, /readDxQueryBroadcastStatus/);
  assert.match(status, /from "@\/lib\/query\/broadcast"/);
  assert.match(status, /launchQueryBroadcastStatus/);
  assert.match(status, /data-query-broadcast-channel=/);
  assert.match(status, /data-query-broadcast-connected=/);
  assert.match(status, /data-query-broadcast-transport=/);
});

test("tanstack query slice materializes next app router streaming helpers", () => {
  const source = read("core/src/ecosystem/forge_tanstack_query.rs");
  const status = read("examples/template/query-cache-status.tsx");

  assert.match(source, /"js\/query\/next-streaming\.tsx",\s*TANSTACK_QUERY_NEXT_STREAMING_TSX/);
  assert.match(source, /const TANSTACK_QUERY_NEXT_STREAMING_TSX: &str = r#"/);
  assert.match(source, /ReactQueryStreamedHydration/);
  assert.match(source, /DxReactQueryStreamedHydration/);
  assert.match(source, /readDxQueryNextStreamingStatus/);
  assert.match(source, /DehydratedState/);
  assert.match(source, /DehydrateOptions/);
  assert.match(source, /HydrateOptions/);
  assert.match(source, /"ReactQueryStreamedHydration"/);
  assert.match(source, /"@tanstack\/react-query-next-experimental"/);
  assert.match(source, /"query\/next-streaming\.tsx"/);
  assert.match(source, /`query\/next-streaming\.tsx` adds Next App Router streamed hydration helpers/);

  assert.match(status, /DxReactQueryStreamedHydration/);
  assert.match(status, /readDxQueryNextStreamingStatus/);
  assert.match(status, /from "@\/lib\/query\/next-streaming"/);
  assert.match(status, /launchQueryNextStreamingStatus/);
  assert.match(status, /data-query-next-streaming-mode=/);
  assert.match(status, /data-query-next-streaming-runtime=/);
  assert.match(status, /data-query-next-streaming-transformer=/);
});

test("tanstack query slice materializes query client lifecycle helpers", () => {
  const source = read("core/src/ecosystem/forge_tanstack_query.rs");
  const status = read("examples/template/query-cache-status.tsx");

  assert.match(source, /"js\/query\/client-lifecycle\.ts",\s*TANSTACK_QUERY_CLIENT_LIFECYCLE_TS/);
  assert.match(source, /const TANSTACK_QUERY_CLIENT_LIFECYCLE_TS: &str = r#"/);
  assert.match(source, /mount\(\)/);
  assert.match(source, /unmount\(\)/);
  assert.match(source, /clear\(\)/);
  assert.match(source, /resumePausedMutations/);
  assert.match(source, /getQueryCache/);
  assert.match(source, /getMutationCache/);
  assert.match(source, /QueryCache/);
  assert.match(source, /MutationCache/);
  assert.match(source, /mountDxQueryClient/);
  assert.match(source, /unmountDxQueryClient/);
  assert.match(source, /clearDxQueryClient/);
  assert.match(source, /resumeDxPausedMutations/);
  assert.match(source, /readDxQueryClientCaches/);
  assert.match(source, /summarizeDxQueryClientCaches/);
  assert.match(source, /"query\/client-lifecycle\.ts"/);
  assert.match(source, /`query\/client-lifecycle\.ts` adds QueryClient lifecycle helpers/);

  assert.match(status, /summarizeDxQueryClientCaches/);
  assert.match(status, /resumeDxPausedMutations/);
  assert.match(status, /from "@\/lib\/query\/client-lifecycle"/);
  assert.match(status, /launchQueryClientCaches/);
  assert.match(status, /data-query-client-cache-count=/);
  assert.match(status, /data-mutation-client-cache-count=/);
  assert.match(status, /data-paused-mutation-resume-state=/);
});

test("tanstack query dashboard consumes lifecycle managers with dx icon controls", () => {
  const source = read("core/src/ecosystem/forge_tanstack_query.rs");
  const status = read("examples/template/query-cache-status.tsx");
  const catalog = read("examples/template/package-catalog.ts");

  assert.match(source, /setDxQueryFocused/);
  assert.match(source, /setDxQueryOnline/);
  assert.match(source, /useDxQueryLifecycleStatus/);
  assert.match(source, /DxQueryLifecycleStatus/);
  assert.match(status, /from "@\/lib\/query\/lifecycle"/);
  assert.match(status, /useDxQueryLifecycleStatus/);
  assert.match(status, /setDxQueryFocused/);
  assert.match(status, /setDxQueryOnline/);
  assert.match(status, /DxQueryLifecycleStatus/);
  assert.match(status, /data-dx-component="tanstack-query-dashboard-settings"/);
  assert.match(status, /data-query-dashboard-workflow="settings-state-refresh"/);
  assert.match(status, /data-query-dashboard-action="set-offline"/);
  assert.match(status, /data-query-dashboard-action="set-background"/);
  assert.match(status, /<dx-icon name="pack:tanstack-query"/);
  assert.match(status, /border-border bg-card/);
  assert.match(status, /hover:bg-accent hover:text-accent-foreground/);
  assert.match(catalog, /"setDxQueryOnline"/);
  assert.match(catalog, /"DxQueryLifecycleStatus"/);
});

test("tanstack query dashboard applies cache policy defaults visibly", () => {
  const source = read("core/src/ecosystem/forge_tanstack_query.rs");
  const status = read("examples/template/query-cache-status.tsx");
  const catalog = read("examples/template/package-catalog.ts");

  assert.match(source, /setDxQueryDefaults/);
  assert.match(source, /readDxQueryDefaults/);
  assert.match(status, /from "@\/lib\/query\/defaults"/);
  assert.match(status, /readDxQueryDefaults/);
  assert.match(status, /setDxQueryDefaults/);
  assert.match(status, /type LaunchQueryPolicyId =/);
  assert.match(status, /launchQueryPolicies/);
  assert.match(status, /handleApplyQueryPolicy/);
  assert.match(status, /data-query-dashboard-policy=/);
  assert.match(status, /data-query-dashboard-policy-stale-time=/);
  assert.match(status, /data-query-dashboard-policy-gc-time=/);
  assert.match(status, /data-query-dashboard-policy-retry=/);
  assert.match(status, /data-query-dashboard-action="set-balanced-cache"/);
  assert.match(status, /data-query-dashboard-action="set-fast-cache"/);
  assert.match(status, /data-query-dashboard-action="set-durable-cache"/);
  assert.match(status, /border-border bg-muted\/40/);
  assert.match(status, /hover:bg-accent hover:text-accent-foreground/);
  assert.match(catalog, /"setDxQueryDefaults"/);
  assert.match(catalog, /"readDxQueryDefaults"/);
});

test("tanstack query metadata exposes Forge provenance and app-owned boundaries", () => {
  const source = read("core/src/ecosystem/forge_tanstack_query.rs");

  assert.match(source, /aliases: \[/);
  assert.match(source, /"tanstack-query"/);
  assert.match(source, /"query\/tanstack"/);
  assert.match(source, /sourceMirror: "G:\\\\WWW\\\\inspirations\\\\tanstack-query"/);
  assert.match(source, /provenance: \{/);
  assert.match(source, /upstreamRepository: "https:\/\/github\.com\/TanStack\/query"/);
  assert.match(source, /exportedFiles: \[/);
  assert.match(source, /"query\/lifecycle\.tsx"/);
  assert.match(source, /requiredEnv: \[\]/);
  assert.match(source, /appOwnedBoundaries: \[/);
  assert.match(source, /Query keys, fetchers, cache invalidation/);
  assert.match(source, /receiptPaths: \[/);
  assert.doesNotMatch(source, /\.dx\/forge\/receipts\/tanstack-query\.json/);
  assert.match(
    source,
    /examples\/template\/\.dx\/forge\/receipts\/2026-05-22-tanstack-query-dashboard-data\.json/,
  );
  assert.match(source, /"docs\/packages\/tanstack-query\.md"/);
  assert.match(source, /"examples\/template\/query-cache-status\.tsx"/);
  assert.match(source, /packageQueueMarker: "data-dx-query-dashboard-queue=\\"package-readiness\\""/);
  assert.match(source, /"data-dx-query-package-id"/);
  assert.match(source, /"data-dx-query-package-status"/);
});

test("tanstack query slice materializes imperative fetch helpers", () => {
  const source = read("core/src/ecosystem/forge_tanstack_query.rs");
  const status = read("examples/template/query-cache-status.tsx");

  assert.match(source, /"js\/query\/fetch\.ts",\s*TANSTACK_QUERY_FETCH_TS/);
  assert.match(source, /const TANSTACK_QUERY_FETCH_TS: &str = r#"/);
  assert.match(source, /FetchQueryOptions/);
  assert.match(source, /FetchInfiniteQueryOptions/);
  assert.match(source, /EnsureQueryDataOptions/);
  assert.match(source, /EnsureInfiniteQueryDataOptions/);
  assert.match(source, /InfiniteData/);
  assert.match(source, /fetchQuery/);
  assert.match(source, /prefetchQuery/);
  assert.match(source, /fetchInfiniteQuery/);
  assert.match(source, /prefetchInfiniteQuery/);
  assert.match(source, /ensureQueryData/);
  assert.match(source, /ensureInfiniteQueryData/);
  assert.match(source, /fetchDxQueryData/);
  assert.match(source, /prefetchDxQueryData/);
  assert.match(source, /ensureDxQueryDataFresh/);
  assert.match(source, /revalidateIfStale:\s*true/);
  assert.match(source, /fetchDxInfiniteQueryData/);
  assert.match(source, /prefetchDxInfiniteQueryData/);
  assert.match(source, /ensureDxInfiniteQueryData/);
  assert.match(source, /createDxFetchSummary/);
  assert.match(source, /"query\/fetch\.ts"/);
  assert.match(source, /`query\/fetch\.ts` adds imperative fetch helpers/);

  assert.match(status, /fetchDxQueryData/);
  assert.match(status, /from "@\/lib\/query\/fetch"/);
  assert.match(status, /launchCacheFetchState/);
  assert.match(status, /handleFetchQueryNow/);
  assert.match(status, /data-query-fetch-action-state=/);
});

test("tanstack query slice materializes hydration policy helpers", () => {
  const source = read("core/src/ecosystem/forge_tanstack_query.rs");
  const status = read("examples/template/query-cache-status.tsx");

  assert.match(source, /"js\/query\/hydration\.ts",\s*TANSTACK_QUERY_HYDRATION_TS/);
  assert.match(source, /const TANSTACK_QUERY_HYDRATION_TS: &str = r#"/);
  assert.match(source, /hydrate/);
  assert.match(source, /dehydrate/);
  assert.match(source, /defaultShouldDehydrateQuery/);
  assert.match(source, /defaultShouldDehydrateMutation/);
  assert.match(source, /DehydratedState/);
  assert.match(source, /DehydrateOptions/);
  assert.match(source, /HydrateOptions/);
  assert.match(source, /createDxDehydrateOptions/);
  assert.match(source, /createDxHydrateOptions/);
  assert.match(source, /dehydrateDxQueryClient/);
  assert.match(source, /hydrateDxQueryClient/);
  assert.match(source, /summarizeDxHydrationState/);
  assert.match(source, /"hydrate"/);
  assert.match(source, /"defaultShouldDehydrateQuery"/);
  assert.match(source, /"defaultShouldDehydrateMutation"/);
  assert.match(source, /"query\/hydration\.ts"/);
  assert.match(source, /`query\/hydration\.ts` adds hydration policy helpers/);

  assert.match(status, /summarizeDxHydrationState/);
  assert.match(status, /from "@\/lib\/query\/hydration"/);
  assert.match(status, /launchHydrationSummary/);
  assert.match(status, /data-query-hydration-queries=/);
  assert.match(status, /data-query-hydration-state=/);
});

test("tanstack query slice materializes default policy helpers", () => {
  const source = read("core/src/ecosystem/forge_tanstack_query.rs");
  const status = read("examples/template/query-cache-status.tsx");

  assert.match(source, /"js\/query\/defaults\.ts",\s*TANSTACK_QUERY_DEFAULTS_TS/);
  assert.match(source, /const TANSTACK_QUERY_DEFAULTS_TS: &str = r#"/);
  assert.match(source, /DefaultOptions/);
  assert.match(source, /QueryObserverOptions/);
  assert.match(source, /MutationObserverOptions/);
  assert.match(source, /getDefaultOptions/);
  assert.match(source, /setDefaultOptions/);
  assert.match(source, /getQueryDefaults/);
  assert.match(source, /setQueryDefaults/);
  assert.match(source, /getMutationDefaults/);
  assert.match(source, /setMutationDefaults/);
  assert.match(source, /readDxDefaultOptions/);
  assert.match(source, /applyDxDefaultOptions/);
  assert.match(source, /setDxQueryDefaults/);
  assert.match(source, /readDxQueryDefaults/);
  assert.match(source, /setDxMutationDefaults/);
  assert.match(source, /readDxMutationDefaults/);
  assert.match(source, /summarizeDxDefaultPolicies/);
  assert.match(source, /"query\/defaults\.ts"/);
  assert.match(source, /`query\/defaults\.ts` adds default policy helpers/);

  assert.match(status, /summarizeDxDefaultPolicies/);
  assert.match(status, /from "@\/lib\/query\/defaults"/);
  assert.match(status, /launchQueryDefaultPolicies/);
  assert.match(status, /data-query-default-retry=/);
  assert.match(status, /data-query-default-stale-time=/);
});
