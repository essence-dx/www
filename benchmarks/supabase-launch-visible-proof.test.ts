const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  const fullPath = path.join(root, relativePath);
  assert.ok(fs.existsSync(fullPath), `expected ${relativePath} to exist`);
  return fs.readFileSync(fullPath, "utf8");
}

function escapeRegExp(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

test("supabase/client is visibly readiness-gated in generated launch template", () => {
  const receiptPath =
    "examples/template/.dx/forge/receipts/2026-05-22-supabase-client-dashboard-workflow.json";
  const receiptMarker = `data-dx-supabase-receipt-path="${receiptPath}"`;
  const launchShell = read("examples/template/template-shell.tsx");
  const dataStatus = read("examples/template/data-status.tsx");
  const editContract = read("examples/template/dx-studio-edit-contract.ts");
  const runtimeLaunch = read("tools/launch/runtime-template/pages/index.html");
  const runtimeScript = read("tools/launch/runtime-template/assets/launch-runtime.ts");

  assert.match(launchShell, /<LaunchDataStatus \/>/);
  assert.match(launchShell, /data-dx-package="supabase\/client"/);

  assert.match(dataStatus, /data-dx-package="supabase\/client"/);
  assert.match(dataStatus, /data-dx-component="supabase-schema-query-workflow"/);
  assert.doesNotMatch(dataStatus, /supabase-client-live-demo/);
  assert.match(dataStatus, /data-dx-dashboard-card="database-supabase-client"/);
  assert.match(dataStatus, /data-dx-dashboard-workflow="supabase-schema-query"/);
  assert.match(dataStatus, /data-dx-edit-id="launch\.database\.supabase-client"/);
  assert.match(dataStatus, /data-dx-product-surface="launch-data-dashboard"/);
  assert.match(dataStatus, /data-dx-supabase-readiness="client-readiness"/);
  assert.doesNotMatch(dataStatus, /data-dx-supabase-demo=/);
  assert.match(dataStatus, /data-dx-supabase-interaction="config-refresh"/);
  assert.match(dataStatus, /data-dx-supabase-action="run-local-schema-query"/);
  assert.match(dataStatus, new RegExp(escapeRegExp(receiptMarker)));
  assert.match(dataStatus, /data-dx-supabase-query-state=\{/);
  assert.match(dataStatus, /data-dx-supabase-query-operation/);
  assert.match(dataStatus, /setSupabaseQuery/);
  assert.doesNotMatch(dataStatus, /SUPABASE_SERVICE_ROLE_KEY/);
  assert.match(editContract, /id: "supabase-schema-query-workflow"/);
  assert.match(
    editContract,
    /id: "supabase-schema-query-workflow"[\s\S]*selector: '\[data-dx-component="supabase-schema-query-workflow"\]'[\s\S]*sourceFile: "examples\/template\/data-status\.tsx"/,
  );

  assert.match(runtimeLaunch, /data-dx-component="supabase-schema-query-workflow"/);
  assert.doesNotMatch(runtimeLaunch, /supabase-client-live-demo/);
  assert.match(runtimeLaunch, /data-dx-package="supabase\/client"/);
  assert.match(runtimeLaunch, /data-dx-supabase-readiness="client-readiness"/);
  assert.doesNotMatch(runtimeLaunch, /data-dx-supabase-demo=/);
  assert.match(runtimeLaunch, /data-dx-supabase-interaction="config-readiness"/);
  assert.match(runtimeLaunch, /data-dx-supabase-action="run-local-schema-query"/);
  assert.match(runtimeLaunch, new RegExp(escapeRegExp(receiptMarker)));
  assert.match(runtimeLaunch, /data-dx-supabase-config-status="missing-config"/);
  assert.match(runtimeLaunch, /data-dx-supabase-query-state="idle"/);
  assert.match(
    runtimeLaunch,
    /data-dx-supabase-query-operation="supabase\.from\('profiles'\)\.select\('id, full_name, username, website'\)"/,
  );

  assert.match(runtimeScript, /function bindSupabaseSchemaQueryWorkflow\(\)/);
  assert.doesNotMatch(runtimeScript, /supabase-client-live-demo/);
  assert.match(runtimeScript, /dataset\.dxSupabaseQueryState = "ready"/);
  assert.match(runtimeScript, /dataset\.dxSupabaseQueryOperation/);
  assert.match(runtimeScript, /essencedx/);
  assert.match(runtimeScript, /local schema query prepared/);
});
