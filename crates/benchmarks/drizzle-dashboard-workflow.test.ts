const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const sourceMirror = "G:/WWW/inspirations/drizzle-orm";
const forbiddenColorPattern = /#[0-9a-fA-F]{3,8}|rgb\(|hsl\(|bg-[a-z]+-[0-9]|text-[a-z]+-[0-9]|border-[a-z]+-[0-9]/;

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readMirror(relativePath) {
  return fs.readFileSync(path.join(sourceMirror, relativePath), "utf8");
}

test("Drizzle dashboard workflow uses upstream-shaped APIs and is visible in the starter", () => {
  const upstreamDriver = readMirror("drizzle-orm/src/better-sqlite3/driver.ts");
  const upstreamMigrator = readMirror("drizzle-orm/src/better-sqlite3/migrator.ts");
  const upstreamSelectBuilder = readMirror("drizzle-orm/src/sqlite-core/query-builders/select.ts");
  const forge = read("core/src/ecosystem/forge_drizzle.rs");
  const dashboard = read("examples/dashboard/src/pages/Dashboard.tsx");
  const workflow = read("examples/dashboard/src/components/DrizzleDashboardWorkflow.tsx");
  const model = read("examples/dashboard/src/lib/drizzleDashboard.ts");
  const packageDoc = read("docs/packages/db-drizzle-sqlite.md");
  const readme = read("examples/dashboard/README.md");
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");

  assert.match(upstreamDriver, /export function drizzle/);
  assert.match(upstreamMigrator, /export function migrate/);
  assert.match(upstreamSelectBuilder, /toSQL\(\): Query/);

  assert.match(forge, /"js\/db\/drizzle\/dashboard-workflow\.ts"/);
  assert.match(forge, /const DRIZZLE_DASHBOARD_WORKFLOW_TS/);
  assert.match(forge, /readDrizzleDashboardOverview/);
  assert.match(forge, /readDrizzleDashboardQueryPlan/);
  assert.match(forge, /getDrizzleDashboardQueryPlan/);
  assert.match(forge, /readDrizzleDashboardQueryPlanById/);
  assert.match(forge, /DrizzleDashboardQueryPlanId = "overview" \| "published-posts" \| "author-counts"/);
  assert.match(forge, /id: "overview"/);
  assert.match(forge, /helper: "readLaunchDatabaseStats"/);
  assert.match(forge, /plans\.find\(\(plan\) => plan\.id === id\)/);
  assert.match(forge, /\.toSQL\(\)/);
  assert.match(forge, /aliases: \[/);
  assert.match(forge, /sourceMirror: "G:\\\\WWW\\\\inspirations\\\\drizzle-orm"/);
  assert.match(forge, /provenance: \{/);
  assert.match(forge, /exportedFiles: \[/);
  assert.match(forge, /requiredEnv: \[\]/);
  assert.match(forge, /receiptPaths: \[/);
  assert.match(forge, /dashboardUsage: \{/);
  assert.match(forge, /dxIcon: "pack:database"/);
  assert.match(forge, /runtimeDependenciesMarker: 'data-dx-drizzle-runtime-dependencies'/);

  assert.match(model, /packageId: 'db\/drizzle-sqlite'/);
  assert.match(model, /sourceMirror: 'G:\/WWW\/inspirations\/drizzle-orm'/);
  assert.match(model, /2026-05-22-db-drizzle-sqlite-dashboard-workflow\.json/);
  assert.match(model, /drizzleDashboardWorkflowReceiptPath/);
  assert.match(model, /receiptPath: typeof drizzleDashboardWorkflowReceiptPath/);
  assert.match(model, /type DrizzleDashboardRuntimeStatus = 'missing-runtime'/);
  assert.match(model, /drizzleDashboardRuntimeDependencies = \['drizzle-orm', 'better-sqlite3'\] as const/);
  assert.match(model, /readDrizzleDashboardRuntimeReadiness/);
  assert.match(model, /runtimeDependencies: typeof drizzleDashboardRuntimeDependencies/);
  assert.match(model, /readDrizzleDashboardOverview/);
  assert.match(model, /readDrizzleDashboardQueryPlan/);
  assert.match(model, /readDrizzleDashboardQueryPlanById/);
  assert.match(model, /dashboardQueryPlanByIdEntryPoint: 'readDrizzleDashboardQueryPlanById'/);
  assert.match(model, /createDrizzleDashboardReceipt/);
  assert.match(packageDoc, /# Database ORM/);
  assert.match(packageDoc, /Package id: `db\/drizzle-sqlite`/);
  assert.match(packageDoc, /G:\\WWW\\inspirations\\drizzle-orm/);
  assert.match(packageDoc, /readDrizzleDashboardOverview/);
  assert.match(packageDoc, /readDrizzleDashboardQueryPlan/);
  assert.match(packageDoc, /readDrizzleDashboardQueryPlanById/);
  assert.match(packageDoc, /DrizzleDashboardWorkflow/);
  assert.match(packageDoc, /<dx-icon name="pack:database" \/>/);
  assert.match(packageDoc, /data-dx-drizzle-receipt-path/);
  assert.match(packageDoc, /data-dx-drizzle-runtime-dependencies/);
  assert.match(packageDoc, /no `node_modules` workflow/);
  assert.match(packageDoc, /App-owned boundaries/);
  assert.match(packageDoc, /## Source Guard/);
  assert.match(packageDoc, /dx run --test \.\\benchmarks\\drizzle-dashboard-workflow\.test\.ts/);
  assert.match(packageDoc, /## Intentionally Deferred/);
  assert.match(workflow, /data-dx-package="db\/drizzle-sqlite"/);
  assert.match(workflow, /data-dx-component="dashboard-drizzle-workflow"/);
  assert.match(workflow, /data-dx-drizzle-dashboard-workflow="content-readiness"/);
  assert.match(workflow, /<dx-icon name="pack:database"/);
  assert.match(workflow, /data-dx-drizzle-action="prepare-dashboard-query"/);
  assert.match(workflow, /data-dx-drizzle-receipt-path=\{drizzleDashboardWorkflowReceiptPath\}/);
  assert.match(workflow, /readDrizzleDashboardRuntimeReadiness/);
  assert.match(workflow, /data-dx-drizzle-runtime=\{runtimeReadiness\.status\}/);
  assert.match(workflow, /data-dx-drizzle-runtime-dependencies=\{\s*runtimeReadiness\.runtimeDependencies\.join\(','\)\s*\}/);
  assert.match(workflow, /data-dx-drizzle-sql-preview/);
  assert.match(workflow, /data-dx-node-modules="forbidden"/);
  assert.match(workflow, /class="panel-header"/);
  assert.match(workflow, /class="primary-action"/);
  assert.doesNotMatch(workflow, forbiddenColorPattern);
  assert.doesNotMatch(workflow, /simple-icons:|lucide:|brand:/);
  assert.match(dashboard, /DrizzleDashboardWorkflow/);
  assert.doesNotMatch(dashboard, /DrizzleDashboardWorkflow\.sr/);

  assert.match(readme, /Drizzle dashboard workflow/);
  assert.match(dx, /dashboard starter consumes `db\/drizzle-sqlite`/);
  assert.match(todo, /Drizzle dashboard workflow/);
  assert.match(changelog, /Drizzle dashboard workflow/);
  assert.doesNotMatch(
    fs.readdirSync(path.join(root, "examples", "dashboard", "src", "components")).join("\n"),
    /DrizzleDashboardWorkflow\.sr/,
  );
});
