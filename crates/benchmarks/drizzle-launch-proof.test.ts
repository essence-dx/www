const assert = require("assert");
const { execFileSync } = require("child_process");
const fs = require("fs");
const os = require("os");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const materializer = path.join(root, "tools", "launch", "materialize-www-template.ts");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("Drizzle launch dashboard data workflow is visible and materialized without node_modules", () => {
  const shell = read("examples/template/template-shell.tsx");
  const dataStatus = read("examples/template/data-status.tsx");
  const drizzleProof = read("examples/template/drizzle-query-proof.tsx");
  const routeContract = read("examples/template/template-route-contract.ts");
  const editContract = read("examples/template/dx-studio-edit-contract.ts");
  const packageCatalog = read("examples/template/package-catalog.ts");
  const templateSurfaceRegistry = read("examples/template/template-surface-registry.ts");
  const packageDocs = read("docs/packages/db-drizzle-sqlite.md");
  const forgeDrizzle = read("core/src/ecosystem/forge_drizzle.rs");
  const dashboardReceipt = JSON.parse(
    read("examples/template/.dx/forge/receipts/2026-05-22-db-drizzle-sqlite-dashboard-workflow.json"),
  );
  const runtimeLaunch = read("tools/launch/runtime-template/pages/index.html");
  const runtimeScript = read("tools/launch/runtime-template/assets/launch-runtime.ts");
  const runtimeStyles = read("examples/template/styles/globals.css");
  const cli = read("dx-www/src/cli/mod.rs");
  const studioManifest = read("dx-www/src/cli/studio_manifest.rs");

  assert.match(shell, /LaunchDataStatus/);
  assert.match(shell, /data-dx-dashboard-workflow="data-read-model"/);
  assert.match(shell, /data-dx-edit-kind="dashboard-workflow"/);
  assert.match(shell, /data-dx-backend-status="read-model-ready"/);
  assert.match(shell, /data-dx-backend-detail="Drizzle read model ready/);
  assert.match(dataStatus, /LaunchDrizzleDashboardData/);
  assert.match(drizzleProof, /@\/db\/drizzle\/metadata/);
  assert.match(drizzleProof, /data-dx-package="db\/drizzle-sqlite"/);
  assert.match(drizzleProof, /data-dx-component="launch-drizzle-data-workflow"/);
  assert.match(drizzleProof, /data-dx-dashboard-workflow="sqlite-read-model"/);
  assert.match(drizzleProof, /data-dx-product-surface="launch-data-dashboard"/);
  assert.match(drizzleProof, /data-dx-dashboard-target="mission-control-database"/);
  assert.match(drizzleProof, /data-dx-source="examples\/template\/drizzle-query-proof\.tsx"/);
  assert.match(
    drizzleProof,
    /data-dx-drizzle-receipt-path="examples\/template\/\.dx\/forge\/receipts\/2026-05-22-db-drizzle-sqlite-dashboard-workflow\.json"/,
  );
  assert.match(drizzleProof, /data-dx-drizzle-mission-control="database"/);
  assert.match(drizzleProof, /data-dx-drizzle-status/);
  assert.match(drizzleProof, /data-dx-drizzle-read-model/);
  assert.match(drizzleProof, /data-dx-drizzle-runtime-dependencies=\{dxDrizzlePackage\.runtimeDependencies\.join\(","\)\}/);
  assert.match(packageDocs, /data-dx-drizzle-runtime-dependencies/);
  assert.match(drizzleProof, /data-dx-drizzle-action="apply-read-model"/);
  assert.match(drizzleProof, /data-dx-drizzle-action="select-read-model"/);
  assert.match(drizzleProof, /data-dx-drizzle-action="preview-query-plan"/);
  assert.match(drizzleProof, /data-dx-drizzle-query-plan-id=\{activeModel\.queryPlanId\}/);
  assert.match(drizzleProof, /queryPlanByIdEntryPoint/);
  assert.match(drizzleProof, /data-dx-drizzle-sql-preview/);
  assert.match(drizzleProof, /data-dx-drizzle-fixture-row/);
  assert.match(drizzleProof, /data-dx-drizzle-receipt-state/);
  assert.match(drizzleProof, /<dx-icon name="pack:database"/);
  assert.match(drizzleProof, /React\.useState/);
  assert.match(drizzleProof, /no node_modules/i);
  assert.match(drizzleProof, /runtimeDependencies\.join/);
  assert.match(routeContract, /"components\/template-app\/drizzle-query-proof\.tsx"/);
  assert.match(routeContract, /2026-05-22-db-drizzle-sqlite-dashboard-workflow\.json/);
  assert.match(editContract, /id: "launch-drizzle-data-workflow"/);
  assert.match(editContract, /interactionSelectors: \[/);
  assert.match(editContract, /\[data-dx-drizzle-action="select-read-model"\]/);
  assert.match(editContract, /\[data-dx-drizzle-action="preview-query-plan"\]/);
  assert.match(editContract, /\[data-dx-drizzle-action="apply-read-model"\]/);
  assert.match(editContract, /\[data-dx-dashboard-target="mission-control-database"\]/);
  assert.match(editContract, /stateMarkers: \[/);
  assert.match(editContract, /data-dx-backend-status/);
  assert.match(editContract, /data-dx-backend-detail/);
  assert.match(editContract, /data-dx-drizzle-receipt-path/);
  assert.match(
    editContract,
    /receiptPath: "examples\/template\/\.dx\/forge\/receipts\/2026-05-22-db-drizzle-sqlite-dashboard-workflow\.json"/,
  );
  assert.match(packageCatalog, /mission-control database card/i);
  assert.match(packageCatalog, /2026-05-22-db-drizzle-sqlite-dashboard-workflow\.json/);
  assert.match(templateSurfaceRegistry, /data-dx-drizzle-receipt-path/);
  assert.match(templateSurfaceRegistry, /data-dx-drizzle-runtime-dependencies/);
  assert.match(packageDocs, /mission-control database card/i);
  assert.match(packageDocs, /2026-05-22-db-drizzle-sqlite-dashboard-workflow\.json/);
  assert.match(forgeDrizzle, /missionControlTarget: "mission-control-database"/);
  assert.match(forgeDrizzle, /backendStatusMarker: 'data-dx-backend-status'/);
  assert.match(forgeDrizzle, /2026-05-22-db-drizzle-sqlite-dashboard-workflow\.json/);
  assert.match(cli, /NEXT_FAMILIAR_DRIZZLE_QUERY_PROOF_TSX/);
  assert.match(cli, /examples\/template\/drizzle-query-proof\.tsx/);
  assert.match(cli, /"components\/template-app\/drizzle-query-proof\.tsx"/);
  assert.match(cli, /LaunchDrizzleDashboardData/);
  assert.match(studioManifest, /data-dx-drizzle-action/);
  assert.match(studioManifest, /data-dx-backend-status/);
  assert.match(studioManifest, /launch-drizzle-data-workflow/);
  assert.match(studioManifest, /examples\/template\/drizzle-query-proof\.tsx/);
  assert.match(studioManifest, /fn studio_drizzle_edit_surface\(/);
  assert.match(studioManifest, /studio_drizzle_edit_surface\(\s*"launch-drizzle-data-workflow"/);
  assert.ok(
    studioManifest.includes('"[data-dx-drizzle-action=\\"select-read-model\\"]"'),
    "Studio manifest must expose Drizzle read-model selection as an interaction selector",
  );
  assert.ok(
    studioManifest.includes('"[data-dx-drizzle-action=\\"preview-query-plan\\"]"'),
    "Studio manifest must expose Drizzle query-plan preview as an interaction selector",
  );
  assert.ok(
    studioManifest.includes('"[data-dx-drizzle-action=\\"apply-read-model\\"]"'),
    "Studio manifest must expose Drizzle apply action as an interaction selector",
  );
  assert.ok(
    studioManifest.includes('"data-dx-drizzle-read-model"'),
    "Studio manifest must expose the Drizzle read-model state marker",
  );
  assert.ok(
    studioManifest.includes('"data-dx-backend-detail"'),
    "Studio manifest must expose the mission-control backend detail state marker",
  );
  assert.ok(
    studioManifest.includes('"data-dx-drizzle-receipt-path"'),
    "Studio manifest must expose the Drizzle receipt path marker",
  );
  assert.ok(
    studioManifest.includes('"data-dx-drizzle-runtime-dependencies"'),
    "Studio manifest must expose the Drizzle runtime dependency marker",
  );
  assert.ok(
    studioManifest.includes('"examples/template/.dx/forge/receipts/2026-05-22-db-drizzle-sqlite-dashboard-workflow.json"'),
    "Studio manifest must expose the source-owned Drizzle dashboard receipt",
  );
  assert.match(studioManifest, /"front_facing_name": "Database ORM"/);
  assert.match(studioManifest, /"data-dx-check-package-lane-row"/);
  assert.match(studioManifest, /"data-dx-check-package-lane-dx-style-status"/);
  assert.match(studioManifest, /"dashboard_workflow": "sqlite-read-model"/);
  assert.match(studioManifest, /"source_file": "examples\/template\/drizzle-query-proof\.tsx"/);
  assert.match(studioManifest, /"materialized_file": "components\/template-app\/drizzle-query-proof\.tsx"/);
  assert.match(studioManifest, /"readDrizzleDashboardOverview"/);
  assert.match(studioManifest, /"readDrizzleDashboardQueryPlan"/);
  assert.match(studioManifest, /"readDrizzleDashboardQueryPlanById"/);
  assert.match(studioManifest, /data-dx-drizzle-query-plan-id/);
  assert.equal(dashboardReceipt.schema, "dx.forge.package_dashboard_workflow_receipt");
  assert.equal(dashboardReceipt.package_id, "db/drizzle-sqlite");
  assert.equal(dashboardReceipt.component, "launch-drizzle-data-workflow");
  assert.equal(dashboardReceipt.workflow, "sqlite-read-model");
  assert.equal(dashboardReceipt.mission_control_target, "mission-control-database");
  assert.equal(dashboardReceipt.no_runtime_execution, true);
  assert.ok(
    dashboardReceipt.forge_public_apis.includes("readDrizzleDashboardQueryPlanById"),
    "Drizzle receipt must expose the package-owned by-id query-plan API",
  );
  assert.ok(
    dashboardReceipt.stable_markers.includes('data-dx-backend-status'),
    "Drizzle receipt must expose the mission-control backend status marker",
  );
  assert.ok(
    dashboardReceipt.stable_markers.includes('data-dx-drizzle-receipt-path'),
    "Drizzle receipt must expose its source-owned receipt path marker",
  );
  assert.ok(
    dashboardReceipt.stable_markers.includes('data-dx-drizzle-runtime-dependencies'),
    "Drizzle receipt must expose the runtime dependency marker",
  );
  assert.ok(
    dashboardReceipt.local_readiness_interactions.includes("apply-read-model"),
    "Drizzle receipt must name the visible apply action",
  );
  assert.equal(dashboardReceipt.local_demo_interactions, undefined);
  assert.match(runtimeLaunch, /data-dx-component="launch-drizzle-data-workflow"/);
  assert.match(runtimeLaunch, /data-dx-package="db\/drizzle-sqlite"/);
  assert.match(runtimeLaunch, /data-dx-dashboard-workflow="sqlite-read-model"/);
  assert.match(runtimeLaunch, /data-dx-product-surface="launch-data-dashboard"/);
  assert.match(runtimeLaunch, /data-dx-dashboard-target="mission-control-database"/);
  assert.match(runtimeLaunch, /data-dx-source="examples\/template\/drizzle-query-proof\.tsx"/);
  assert.match(
    runtimeLaunch,
    /data-dx-drizzle-receipt-path="\.dx\/forge\/receipts\/2026-05-22-db-drizzle-sqlite-dashboard-workflow\.json"/,
  );
  assert.match(runtimeLaunch, /data-dx-drizzle-mission-control="database"/);
  assert.match(runtimeLaunch, /data-dx-drizzle-runtime-dependencies="drizzle-orm,better-sqlite3"/);
  assert.match(runtimeLaunch, /data-dx-backend-status="read-model-ready"/);
  assert.match(runtimeLaunch, /data-dx-backend-detail="Drizzle read model ready/);
  assert.match(runtimeLaunch, /data-dx-drizzle-action="apply-read-model"/);
  assert.match(runtimeLaunch, /data-dx-drizzle-action="select-read-model"/);
  assert.match(runtimeLaunch, /data-dx-drizzle-action="preview-query-plan"/);
  assert.match(runtimeLaunch, /data-dx-drizzle-query-plan-id="overview"/);
  assert.match(runtimeLaunch, /data-dx-drizzle-query-plan-export="readDrizzleDashboardQueryPlanById"/);
  assert.match(runtimeLaunch, /data-dx-drizzle-sql-preview/);
  assert.match(runtimeLaunch, /data-dx-drizzle-fixture-row/);
  assert.match(runtimeLaunch, /data-dx-drizzle-receipt-state="idle"/);
  assert.match(runtimeLaunch, /id="drizzle-status"/);
  assert.match(runtimeLaunch, /id="drizzle-apply-read-model"/);
  assert.match(runtimeLaunch, /id="drizzle-query-plan"/);
  assert.match(runtimeScript, /function bindDrizzleDashboardData\(\)/);
  assert.match(runtimeScript, /function syncDatabaseDashboardState/);
  assert.match(runtimeScript, /databaseDashboard\.dataset\.dxBackendStatus = status/);
  assert.match(runtimeScript, /databaseDashboard\.dataset\.dxBackendDetail = detail/);
  assert.match(runtimeScript, /const databaseSurface = \$\('\[data-dx-component="database-backend-proof"\], \[data-dx-component="database-backend-card"\]'\)/);
  assert.match(runtimeScript, /const databaseStatus = databaseSurface\?\.dataset\.dxBackendStatus/);
  assert.match(runtimeScript, /syncDatabaseDashboardState\(\s*"read-model-applied"/);
  assert.match(runtimeScript, /card\.dataset\.dxDrizzleStatus = "read-model-applied"/);
  assert.match(runtimeScript, /card\.dataset\.dxDrizzleReceiptState = "ready"/);
  assert.match(runtimeScript, /button\.dataset\.dxDrizzleReadModelOption/);
  assert.match(runtimeScript, /queryBox\.dataset\.dxDrizzleQueryPlanId = model\.queryPlanId/);
  assert.match(runtimeScript, /state\.drizzleRuns \+= 1/);
  assert.match(runtimeStyles, /\[data-dx-drizzle-action="select-read-model"\]\[data-active="true"\]/);
  assert.ok(
    !fs.existsSync(path.join(root, "examples", "template", "node_modules")),
    "launch template must not contain node_modules",
  );
});

test("Drizzle launch dashboard workflow survives live runtime page materialization", () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-drizzle-runtime-"));
  execFileSync(process.execPath, [materializer, dir], {
    cwd: root,
    encoding: "utf8",
  });

  const launch = fs.readFileSync(path.join(dir, "pages", "index.html"), "utf8");
  assert.match(launch, /data-dx-component="launch-drizzle-data-workflow"/);
  assert.match(launch, /data-dx-package="db\/drizzle-sqlite"/);
  assert.match(launch, /data-dx-drizzle-status="read-model-ready"/);
  assert.match(launch, /data-dx-drizzle-runtime-dependencies="drizzle-orm,better-sqlite3"/);
  assert.match(launch, /data-dx-dashboard-target="mission-control-database"/);
  assert.match(launch, /data-dx-drizzle-receipt-path="\.dx\/forge\/receipts\/2026-05-22-db-drizzle-sqlite-dashboard-workflow\.json"/);
  assert.match(launch, /data-dx-drizzle-mission-control="database"/);
  assert.match(launch, /data-dx-backend-status="read-model-ready"/);
  assert.match(launch, /data-dx-drizzle-action="apply-read-model"/);
  assert.match(launch, /data-dx-drizzle-action="select-read-model"/);
  assert.match(launch, /data-dx-drizzle-action="preview-query-plan"/);
  assert.match(launch, /data-dx-drizzle-query-plan-id="overview"/);
  assert.match(launch, /data-dx-drizzle-query-plan-export="readDrizzleDashboardQueryPlanById"/);
  assert.match(launch, /data-dx-drizzle-fixture-row="dx-preview"/);
  assert.match(launch, /id="drizzle-status"/);
  assert.match(launch, /id="drizzle-apply-read-model"/);
  assert.match(launch, /data-dx-drizzle-receipt-state="idle"/);
  const receiptPath = path.join(
    dir,
    ".dx",
    "forge",
    "receipts",
    "2026-05-22-db-drizzle-sqlite-dashboard-workflow.json",
  );
  assert.ok(fs.existsSync(receiptPath), "materialized launch route must carry the Drizzle dashboard receipt");
  const receipt = JSON.parse(fs.readFileSync(receiptPath, "utf8"));
  assert.equal(receipt.package_id, "db/drizzle-sqlite");
  assert.equal(receipt.no_runtime_execution, true);
  assert.ok(!fs.existsSync(path.join(dir, "node_modules")));

  const previewManifest = JSON.parse(fs.readFileSync(path.join(dir, "public", "preview-manifest.json"), "utf8"));
  const launchRoute = previewManifest.routes.find((route) => route.route === "/");
  assert.ok(launchRoute, "preview manifest must include /launch");
  assert.ok(
    launchRoute.dataDxMarkers.includes("data-dx-drizzle-action"),
    "preview manifest must expose Drizzle action markers for source selection",
  );
  assert.ok(
    launchRoute.dataDxMarkers.includes("data-dx-backend-status"),
    "preview manifest must expose backend status markers for mission-control sync",
  );
  assert.ok(
    launchRoute.dataDxMarkers.includes("data-dx-drizzle-receipt-path"),
    "preview manifest must expose the Drizzle receipt path marker",
  );
  assert.ok(
    launchRoute.dataDxMarkers.includes("data-dx-drizzle-runtime-dependencies"),
    "preview manifest must expose the Drizzle runtime dependency marker",
  );
  const drizzleSurface = previewManifest.editContract.editableSurfaces.find(
    (surface) => surface.id === "launch-runtime-drizzle-data-workflow",
  );
  assert.ok(drizzleSurface, "materialized edit contract must include the Drizzle workflow surface");
  assert.equal(drizzleSurface.selector, '[data-dx-component="launch-drizzle-data-workflow"]');
  assert.deepEqual(drizzleSurface.packageIds, ["db/drizzle-sqlite"]);
  assert.ok(
    drizzleSurface.interactionSelectors.includes('[data-dx-drizzle-action="select-read-model"]'),
    "materialized Drizzle surface must include read-model selector actions",
  );
  assert.ok(
    drizzleSurface.stateMarkers.includes("data-dx-backend-status"),
    "materialized Drizzle surface must include backend state markers",
  );
  assert.ok(
    drizzleSurface.stateMarkers.includes("data-dx-drizzle-receipt-path"),
    "materialized Drizzle surface must include the receipt path marker",
  );
  assert.ok(
    drizzleSurface.stateMarkers.includes("data-dx-drizzle-runtime-dependencies"),
    "materialized Drizzle surface must include runtime dependency markers",
  );
});
