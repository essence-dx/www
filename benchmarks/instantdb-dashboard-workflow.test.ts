const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "instantdb");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("InstantDB dashboard workflow is visible, source-owned, and guarded", () => {
  const upstreamPackage = read(
    path.join(mirror, "client", "packages", "react", "package.json"),
  );
  const upstreamReact = read(
    path.join(mirror, "client", "packages", "react", "src", "index.ts"),
  );
  const upstreamRoom = read(
    path.join(
      mirror,
      "client",
      "packages",
      "react-common",
      "src",
      "InstantReactRoom.ts",
    ),
  );
  const launchProof = read(
    path.join(root, "examples", "template", "instantdb-status.tsx"),
  );
  const dashboardProof = read(
    path.join(
      root,
      "examples",
      "dashboard",
      "src",
      "components",
      "InstantDbDashboardWorkflow.tsx",
    ),
  );
  const dashboardModel = read(
    path.join(
      root,
      "examples",
      "dashboard",
      "src",
      "lib",
      "instantdbDashboard.ts",
    ),
  );
  const dashboardPage = read(
    path.join(root, "examples", "dashboard", "src", "pages", "Dashboard.tsx"),
  );
  const dashboardReadme = read(
    path.join(root, "examples", "dashboard", "README.md"),
  );
  const packageDoc = read(path.join(root, "docs", "packages", "instantdb-react.md"));
  const launchShell = read(
    path.join(root, "examples", "template", "template-shell.tsx"),
  );
  const runtimePage = read(
    path.join(root, "tools", "launch", "runtime-template", "pages", "index.html"),
  );
  const runtimeScript = read(
    path.join(root, "tools", "launch", "runtime-template", "assets", "launch-runtime.ts"),
  );
  const editContract = read(
    path.join(root, "examples", "template", "dx-studio-edit-contract.ts"),
  );
  const runtimeMaterializer = read(
    path.join(root, "tools", "launch", "materialize-www-template.ts"),
  );
  const routeContract = read(
    path.join(root, "examples", "template", "template-route-contract.ts"),
  );
  const cli = read(path.join(root, "dx-www", "src", "cli", "mod.rs"));
  const newCommand = read(path.join(root, "dx-www", "src", "cli", "new_command.rs"));
  const workflowReceipt = read(
    path.join(
      root,
      "examples",
      "www-template",
      ".dx",
      "forge",
      "receipts",
      "2026-05-22-instantdb-realtime-dashboard.json",
    ),
  );
  const catalog = read(
    path.join(root, "examples", "template", "package-catalog.ts"),
  );
  const slice = read(
    path.join(root, "core", "src", "ecosystem", "forge_instantdb.rs"),
  );
  const registry = read(
    path.join(root, "core", "src", "ecosystem", "forge_registry.rs"),
  );

  assert.match(upstreamPackage, /"name": "@instantdb\/react"/);
  assert.match(upstreamPackage, /"\.\/nextjs"/);
  assert.match(upstreamReact, /init/);
  assert.match(upstreamReact, /createInstantRouteHandler/);
  assert.match(upstreamReact, /InstantWritableStream/);
  assert.match(upstreamRoom, /usePresence/);
  assert.match(upstreamRoom, /useTypingIndicator/);

  assert.match(launchShell, /<LaunchInstantStatus \/>/);
  assert.match(launchShell, /data-dx-component="launch-instantdb-dashboard-workflow"/);
  assert.match(launchShell, /data-dx-dashboard-workflow="realtime-data-readiness"/);
  assert.match(launchShell, /data-dx-product-surface="launch-dashboard"/);
  assert.match(launchShell, /data-dx-edit-id="launch\.instantdb-ops"/);
  assert.match(launchShell, /data-dx-icon-search="pack:database"/);
  assert.match(launchShell, /InstantDB powers realtime todos/);
  assert.match(launchShell, /Safe local receipt/);
  assert.match(launchProof, /data-dx-component="instantdb-dashboard-workflow"/);
  assert.match(launchProof, /data-dx-package="instantdb\/react"/);
  assert.match(launchProof, /data-dx-instant-readiness="dashboard-readiness"/);
  assert.doesNotMatch(launchProof, /data-dx-instant-demo=/);
  assert.match(launchProof, /data-dx-instant-required-env="NEXT_PUBLIC_INSTANT_APP_ID"/);
  assert.match(launchProof, /data-dx-instant-action="prepare-local-schema-receipt"/);
  assert.match(launchProof, /data-dx-instant-local-receipt/);
  assert.match(launchProof, /data-dx-instant-local-schema/);
  assert.match(launchProof, /<dx-icon \{\.\.\.props\} \/>/);
  assert.match(launchProof, /name="pack:database"/);
  assert.match(launchProof, /React\.useState<InstantLocalReceipt \| null>/);
  assert.match(launchProof, /db\.useQuery\(\{ todos: \{\} \}\)/);
  assert.match(launchProof, /db\.rooms\.usePresence\(room\)/);
  assert.doesNotMatch(launchProof, /fetch\(/);
  assert.doesNotMatch(launchProof, /process\.env/);

  assert.match(runtimePage, /data-dx-component="launch-instantdb-runtime-dashboard-workflow"/);
  assert.match(runtimePage, /data-dx-dashboard-workflow="realtime-data-readiness"/);
  assert.match(runtimePage, /data-dx-component="instantdb-runtime-dashboard-workflow"/);
  assert.match(runtimePage, /data-dx-package="instantdb\/react"/);
  assert.match(runtimePage, /data-dx-instant-readiness="runtime-dashboard-readiness"/);
  assert.doesNotMatch(runtimePage, /data-dx-instant-demo=/);
  assert.match(runtimePage, /data-dx-instant-action="prepare-local-schema-receipt"/);
  assert.match(runtimePage, /data-dx-instant-local-receipt="idle"/);
  assert.match(runtimePage, /data-dx-instant-required-env="NEXT_PUBLIC_INSTANT_APP_ID"/);
  assert.match(runtimePage, /db\.useQuery\(\{ todos: \{\} \}\)/);
  assert.match(runtimePage, /db\.rooms\.usePresence\(room\)/);
  assert.match(runtimePage, /data-dx-node-modules="forbidden"/);
  assert.match(runtimeScript, /instantdbReceiptRuns: 0/);
  assert.match(runtimeScript, /function bindInstantDbRuntimeProof\(\)/);
  assert.match(runtimeScript, /dx-instantdb-local-/);
  assert.match(runtimeScript, /NEXT_PUBLIC_INSTANT_APP_ID/);
  assert.match(runtimeScript, /bindInstantDbRuntimeProof\(\)/);
  assert.match(runtimeMaterializer, /launch-runtime-instantdb-dashboard/);
  assert.match(runtimeMaterializer, /\[data-dx-component="instantdb-runtime-dashboard-workflow"\]/);
  assert.match(runtimeMaterializer, /instantdb\/react/);
  assert.match(routeContract, /instantDbRealtimeDashboard/);
  assert.match(routeContract, /packageId: "instantdb\/react"/);
  assert.match(routeContract, /component: "instantdb-runtime-dashboard-workflow"/);
  assert.match(routeContract, /dashboardWorkflow: "realtime-data-readiness"/);
  assert.match(routeContract, /runtimeSourceFile: "examples\/template\/app\/page\.tsx"/);
  assert.match(routeContract, /runtimeScriptFile: "tools\/launch\/runtime-template\/assets\/launch-runtime\.ts"/);
  assert.match(routeContract, /2026-05-22-instantdb-realtime-dashboard\.json/);
  assert.match(routeContract, /instantdb-dashboard-workflow\.test\.ts/);
  assert.match(cli, /NEXT_FAMILIAR_INSTANT_DASHBOARD_RECEIPT_JSON/);
  assert.match(
    cli,
    /NEXT_FAMILIAR_INSTANT_DASHBOARD_RECEIPT_JSON[\s\S]{0,180}examples\/template\/\.dx\/forge\/receipts\/2026-05-22-instantdb-realtime-dashboard\.json/,
  );
  assert.match(
    cli,
    /\.dx\/forge\/receipts\/2026-05-22-instantdb-realtime-dashboard\.json/,
  );
  assert.match(newCommand, /instant_dashboard_receipt/);
  assert.match(cli, /"aliases": \["@instantdb\/react", "instantdb", "db\/instantdb"\]/);
  assert.match(cli, /"source_mirror": "G:\/WWW\/inspirations\/instantdb"/);
  assert.match(cli, /"dashboard_usage": "\/launch uses LaunchInstantStatus/);
  assert.match(cli, /data-dx-component=\\"instantdb-runtime-dashboard-workflow\\"/);
  assert.match(cli, /"dx_icon": "pack:database"/);
  assert.match(cli, /"lib\/instant\/dashboard-workflow\.ts"/);
  assert.match(cli, /"lib\/instant\/next-client\.tsx"/);
  assert.match(cli, /"lib\/instant\/mutations\.ts"/);
  assert.match(cli, /"lib\/instant\/metadata\.ts"/);
  assert.match(cli, /"components\/instant\/instant-auth-boundary\.tsx"/);
  assert.match(cli, /"app\/api\/instant\/route\.ts"/);
  assert.match(cli, /"components\/dashboard\/instantdb-dashboard-workflow\.tsx"/);
  assert.match(cli, /"examples\/dashboard\/src\/components\/InstantDbDashboardWorkflow\.tsx"/);
  assert.match(workflowReceipt, /"schema": "dx\.forge\.package_dashboard_workflow_receipt"/);
  assert.match(workflowReceipt, /"package_id": "instantdb\/react"/);
  assert.match(workflowReceipt, /"component": "instantdb-runtime-dashboard-workflow"/);
  assert.match(workflowReceipt, /"workflow": "realtime-data-readiness"/);
  assert.match(workflowReceipt, /"examples\/dashboard\/src\/lib\/instantdbDashboard\.ts"/);
  assert.match(
    workflowReceipt,
    /"examples\/dashboard\/src\/components\/InstantDbDashboardWorkflow\.tsx"/,
  );
  assert.match(workflowReceipt, /"data-dx-instant-action=\\"prepare-local-schema-receipt\\""/);
  assert.match(workflowReceipt, /"NEXT_PUBLIC_INSTANT_APP_ID"/);
  assert.match(workflowReceipt, /"lib\/instant\/dashboard-workflow\.ts"/);
  assert.match(
    workflowReceipt,
    /"components\/dashboard\/instantdb-dashboard-workflow\.tsx"/,
  );
  assert.match(workflowReceipt, /"lib\/instant\/next-client\.tsx"/);
  assert.match(workflowReceipt, /"lib\/instant\/mutations\.ts"/);
  assert.match(workflowReceipt, /"components\/instant\/instant-cursors\.tsx"/);
  assert.match(workflowReceipt, /"components\/instant\/instant-auth-boundary\.tsx"/);
  assert.match(workflowReceipt, /"app\/api\/instant\/route\.ts"/);
  assert.match(
    workflowReceipt,
    /"data-dx-instant-readiness=\\"runtime-dashboard-readiness\\""/,
  );
  assert.match(workflowReceipt, /"local_readiness_interactions": \[/);
  assert.doesNotMatch(workflowReceipt, /data-dx-instant-demo|local_demo_interactions/);
  assert.match(workflowReceipt, /"no_runtime_execution": true/);

  assert.match(dashboardPage, /import \{ InstantDbDashboardWorkflow \}/);
  assert.match(dashboardPage, /<InstantDbDashboardWorkflow \/>/);
  assert.match(dashboardProof, /data-dx-component="dashboard-instantdb-workflow"/);
  assert.match(dashboardProof, /data-dx-package="instantdb\/react"/);
  assert.match(dashboardProof, /data-dx-instant-dashboard-workflow="realtime-boundary"/);
  assert.match(dashboardProof, /data-dx-instant-dashboard-action="select-surface"/);
  assert.match(dashboardProof, /data-dx-instant-dashboard-action="prepare-local-receipt"/);
  assert.match(dashboardProof, /<dx-icon name="pack:database"/);
  assert.match(dashboardProof, /data-dx-instant-dashboard-receipt-paths/);
  assert.match(dashboardProof, /data-dx-instant-dashboard-provenance/);
  assert.match(dashboardProof, /data-dx-node-modules="forbidden"/);
  assert.doesNotMatch(dashboardProof, /fetch\(/);
  assert.doesNotMatch(dashboardProof, /process\.env/);

  assert.match(dashboardModel, /packageId: 'instantdb\/react'/);
  assert.match(
    dashboardModel,
    /aliases: \['@instantdb\/react', 'instantdb', 'db\/instantdb'\]/,
  );
  assert.match(dashboardModel, /sourceMirror: 'G:\/WWW\/inspirations\/instantdb'/);
  assert.match(dashboardModel, /requiredEnv: \['NEXT_PUBLIC_INSTANT_APP_ID'\]/);
  assert.match(dashboardModel, /exportedFiles: \[/);
  assert.match(dashboardModel, /lib\/instant\/dashboard-workflow\.ts/);
  assert.match(dashboardModel, /lib\/instant\/next-client\.tsx/);
  assert.match(dashboardModel, /lib\/instant\/mutations\.ts/);
  assert.match(dashboardModel, /components\/instant\/instant-cursors\.tsx/);
  assert.match(dashboardModel, /components\/instant\/instant-auth-boundary\.tsx/);
  assert.match(dashboardModel, /app\/api\/instant\/route\.ts/);
  assert.match(dashboardModel, /components\/dashboard\/instantdb-dashboard-workflow\.tsx/);
  assert.match(
    dashboardModel,
    /examples\/template\/\.dx\/forge\/receipts\/2026-05-22-instantdb-realtime-dashboard\.json/,
  );
  assert.match(dashboardModel, /db\.useQuery/);
  assert.match(dashboardModel, /db\.streams\.createWriteStream/);
  assert.match(dashboardReadme, /### Realtime App Database dashboard workflow/);
  assert.match(packageDoc, /# Realtime App Database/);
  assert.match(packageDoc, /Package id: `instantdb\/react`/);
  assert.match(packageDoc, /G:\\WWW\\inspirations\\instantdb/);
  assert.match(packageDoc, /data-dx-component="instantdb-runtime-dashboard-workflow"/);
  assert.match(packageDoc, /data-dx-instant-action="prepare-local-schema-receipt"/);
  assert.match(packageDoc, /NEXT_PUBLIC_INSTANT_APP_ID/);
  assert.match(
    packageDoc,
    /examples\/template\/\.dx\/forge\/receipts\/2026-05-22-instantdb-realtime-dashboard\.json/,
  );
  assert.match(packageDoc, /No Node Modules Path/);
  assert.match(packageDoc, /Intentionally Deferred/);

  assert.match(editContract, /id: "instantdb-dashboard-workflow"/);
  assert.match(editContract, /selector: '\[data-dx-component="launch-instantdb-dashboard-workflow"\]'/);
  assert.match(editContract, /packageIds: \["instantdb\/react"\]/);

  assert.match(catalog, /aliases: \["@instantdb\/react", "instantdb", "db\/instantdb"\]/);
  assert.match(catalog, /requiredEnv: \["NEXT_PUBLIC_INSTANT_APP_ID"\]/);
  assert.match(catalog, /sourceMirror: "G:\/WWW\/inspirations\/instantdb"/);
  assert.match(catalog, /exportedFiles: \[/);
  assert.match(catalog, /lib\/instant\/dashboard-workflow\.ts/);
  assert.match(catalog, /lib\/instant\/next-client\.tsx/);
  assert.match(catalog, /lib\/instant\/mutations\.ts/);
  assert.match(catalog, /lib\/instant\/metadata\.ts/);
  assert.match(catalog, /components\/instant\/instant-auth-boundary\.tsx/);
  assert.match(catalog, /app\/api\/instant\/route\.ts/);
  assert.match(catalog, /components\/dashboard\/instantdb-dashboard-workflow\.tsx/);
  assert.match(catalog, /examples\/dashboard\/src\/components\/InstantDbDashboardWorkflow\.tsx/);
  assert.match(catalog, /receiptPaths: \[/);
  assert.match(catalog, /2026-05-22-instantdb-realtime-dashboard\.json/);
  assert.match(catalog, /dxIcon: "pack:database"/);
  assert.match(catalog, /components\/template-app\/instantdb-status\.tsx/);
  assert.match(catalog, /tools\\launch\\runtime-template\\pages\\/index\.html#instantdb-runtime-dashboard-workflow/);
  assert.match(catalog, /tools\\launch\\runtime-template\\assets\\/launch-runtime\.ts#bindInstantDbRuntimeProof/);
  assert.match(catalog, /docs\/packages\/instantdb-react\.md/);
  assert.match(catalog, /db\.storage/);
  assert.match(catalog, /db\.streams/);
  assert.match(catalog, /Realtime App Database/);
  assert.match(catalog, /InstantDbDashboardWorkflow/);
  assert.match(catalog, /data-dx-component="instantdb-runtime-dashboard-workflow"/);
  assert.match(catalog, /data-dx-instant-action="prepare-local-schema-receipt"/);

  assert.match(slice, /aliases: \["@instantdb\/react", "instantdb", "db\/instantdb"\]/);
  assert.match(slice, /requiredEnv: \["NEXT_PUBLIC_INSTANT_APP_ID"\]/);
  assert.match(slice, /sourceMirror: "G:\/WWW\/inspirations\/instantdb"/);
  assert.match(slice, /exportedFiles: \[/);
  assert.match(slice, /"js\/instant\/dashboard-workflow\.ts"/);
  assert.match(slice, /INSTANTDB_DASHBOARD_WORKFLOW_TS/);
  assert.match(slice, /"js\/components\/dashboard\/instantdb-dashboard-workflow\.tsx"/);
  assert.match(slice, /INSTANTDB_DASHBOARD_WORKFLOW_TSX/);
  assert.match(slice, /lib\/instant\/dashboard-workflow\.ts/);
  assert.match(slice, /components\/dashboard\/instantdb-dashboard-workflow\.tsx/);
  assert.match(slice, /data-dx-component="dashboard-instantdb-workflow"/);
  assert.match(slice, /components\/template-app\/instantdb-status\.tsx/);
  assert.match(slice, /examples\/dashboard\/src\/components\/InstantDbDashboardWorkflow\.tsx/);
  assert.match(slice, /tools\\launch\\runtime-template\\pages\\/index\.html#instantdb-runtime-dashboard-workflow/);
  assert.match(slice, /tools\\launch\\runtime-template\\assets\\/launch-runtime\.ts#bindInstantDbRuntimeProof/);
  assert.match(slice, /docs\/packages\/instantdb-react\.md/);
  assert.match(slice, /data-dx-component="instantdb-runtime-dashboard-workflow"/);
  assert.match(slice, /data-dx-instant-action="prepare-local-schema-receipt"/);
  assert.match(slice, /appOwnedBoundaries: \[/);
  assert.match(slice, /receiptPaths: \[/);
  assert.match(slice, /dashboard proof/);
  assert.match(slice, /reusable app-local realtime-boundary dashboard surface/);
  assert.match(registry, /instantdb\/react/);
  assert.match(registry, /Source-owned Realtime App Database launch slice/);
  assert.match(registry, /Sync Table events/);
});
