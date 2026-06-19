const assert = require("node:assert/strict");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const sourceMirror = "G:/WWW/inspirations/supabase";

function read(relativePath) {
  const fullPath = path.join(root, relativePath);
  assert.ok(fs.existsSync(fullPath), `expected ${relativePath} to exist`);
  return fs.readFileSync(fullPath, "utf8");
}

function readMirror(relativePath) {
  const fullPath = path.join(sourceMirror, relativePath);
  assert.ok(fs.existsSync(fullPath), `expected upstream ${relativePath} to exist`);
  return fs.readFileSync(fullPath, "utf8");
}

function escapeRegExp(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

function sha256(relativePath) {
  return crypto.createHash("sha256").update(read(relativePath)).digest("hex");
}

test("Supabase dashboard workflow uses real profile APIs and is materialized into launch", () => {
  const dxCheckStatuses = [
    "present",
    "stale",
    "missing-receipt",
    "blocked",
    "unsupported-surface",
  ];
  const dashboardReceiptPath =
    "examples/template/.dx/forge/receipts/2026-05-22-supabase-client-dashboard-workflow.json";
  const materializedDashboardReceiptPath =
    ".dx/forge/receipts/2026-05-22-supabase-client-dashboard-workflow.json";
  const dashboardReceiptMarker = `data-dx-supabase-receipt-path="${dashboardReceiptPath}"`;
  const hashedReceiptFiles = [
    "docs/packages/supabase-client.md",
    "core/src/ecosystem/forge_supabase.rs",
    "examples/template/supabase-profile-workflow-state.ts",
    "examples/template/supabase-profile-workflow.tsx",
    "examples/template/data-status.tsx",
  ];
  const upstreamAccountForm = readMirror(
    "examples/user-management/nextjs-user-management/app/account/account-form.tsx",
  );
  const upstreamApiDocs = readMirror(
    "apps/studio/components/interfaces/ProjectAPIDocs/ProjectAPIDocs.constants.ts",
  );
  const upstreamExamplePackage = JSON.parse(
    readMirror("examples/user-management/nextjs-user-management/package.json"),
  );
  const workflowApi = read("examples/template/supabase-profile-workflow-state.ts");
  const workflow = read("examples/template/supabase-profile-workflow.tsx");
  const dataStatus = read("examples/template/data-status.tsx");
  const editContract = read("examples/template/dx-studio-edit-contract.ts");
  const launchShell = read("examples/template/template-shell.tsx");
  const packageCatalog = read("examples/template/package-catalog.ts");
  const runtimeLaunch = read("tools/launch/runtime-template/pages/index.html");
  const runtimeScript = read("tools/launch/runtime-template/assets/launch-runtime.ts");
  const cli = read("dx-www/src/cli/mod.rs");
  const newCommand = read("dx-www/src/cli/new_command.rs");
  const studioManifest = read("dx-www/src/cli/studio_manifest.rs");
  const forge = read("core/src/ecosystem/forge_supabase.rs");
  const packageDoc = read("docs/packages/supabase-client.md");
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");
  const dashboardReceipt = JSON.parse(read(dashboardReceiptPath));

  assert.match(upstreamAccountForm, /\.from\('profiles'\)/);
  assert.match(upstreamAccountForm, /\.upsert\(/);
  assert.match(upstreamApiDocs, /\.from\('\$\{resourceId\}'\)/);
  assert.match(upstreamApiDocs, /\.select\('\*'\)/);
  assert.equal(upstreamExamplePackage.dependencies["@supabase/ssr"], "latest");
  assert.equal(upstreamExamplePackage.dependencies["@supabase/supabase-js"], "^2");

  assert.match(workflowApi, /readDxSupabaseProfileConfigStatus/);
  assert.match(workflowApi, /createDxSupabaseProfilePreview/);
  assert.match(workflowApi, /createDxSupabaseProfileUpsertReceipt/);
  assert.match(workflowApi, /type DxSupabaseProfileField/);
  assert.match(workflowApi, /type DxSupabaseProfilesReadModel/);
  assert.match(workflowApi, /dxSupabaseProfileFields/);
  assert.match(workflowApi, /readDxSupabaseProfilesReadModel/);
  assert.match(workflowApi, /select: "id, full_name, username, website"/);
  assert.match(
    workflowApi,
    /operation: "supabase\.from\('profiles'\)\.select\('id, full_name, username, website'\)"/,
  );
  assert.match(workflowApi, /key: "fullName"/);
  assert.match(workflowApi, /key: "username"/);
  assert.match(workflowApi, /key: "website"/);
  assert.match(workflowApi, /dxSupabaseInitialProfileDraft/);
  assert.match(workflowApi, /getDxSupabaseCurrentProfile/);
  assert.match(workflowApi, /upsertDxSupabaseProfile/);
  assert.match(workflowApi, /type DxSupabaseProfileInput/);
  assert.doesNotMatch(workflowApi, /SUPABASE_SERVICE_ROLE_KEY/);

  assert.match(workflow, /data-dx-package="supabase\/client"/);
  assert.match(workflow, /data-dx-component="supabase-profile-workflow"/);
  assert.match(workflow, /data-dx-dashboard-workflow="account-profile-settings"/);
  assert.match(workflow, /data-dx-dashboard-card="account-profile"/);
  assert.match(workflow, /data-dx-supabase-workflow="profile-settings"/);
  assert.match(workflow, /data-dx-supabase-profile-field=\{field\.key\}/);
  assert.match(workflow, /dxSupabaseProfileFields\.map/);
  assert.match(workflow, /data-dx-supabase-profile-label=\{field\.key\}/);
  assert.doesNotMatch(workflow, /onChange=\{\(event\) => updateProfileDraft\("fullName"/);
  assert.match(workflow, new RegExp(escapeRegExp(dashboardReceiptMarker)));
  assert.match(workflow, /setProfileDraft/);
  assert.match(workflow, /data-dx-supabase-action="load-profile-fixture"/);
  assert.match(workflow, /data-dx-supabase-action="prepare-profile-upsert"/);
  assert.match(workflow, /<dx-icon name="database:supabase"/);
  assert.match(workflow, /readDxSupabaseProfileConfigStatus/);
  assert.match(workflow, /createDxSupabaseProfilePreview/);
  assert.match(workflow, /createDxSupabaseProfileUpsertReceipt/);
  assert.match(workflow, /type DxSupabaseProfileInput/);
  assert.doesNotMatch(workflow, /SUPABASE_SERVICE_ROLE_KEY/);
  assert.doesNotMatch(
    workflow,
    /\b(?:bg|text|border)-(?:emerald|green|red|blue|slate|neutral|zinc|stone|amber|orange|purple|cyan|sky|rose)-\d{2,3}\b/,
  );

  assert.doesNotMatch(dataStatus, /LaunchSupabaseProfileWorkflow/);
  assert.match(dataStatus, /readDxSupabaseProfilesReadModel/);
  assert.doesNotMatch(dataStatus, /function readSupabaseLocalQuery/);
  assert.match(dataStatus, /data-dx-supabase-query-operation/);
  assert.doesNotMatch(dataStatus, /role: "assistant"/);
  assert.match(dataStatus, new RegExp(escapeRegExp(dashboardReceiptMarker)));
  assert.match(launchShell, /LaunchSupabaseProfileWorkflow/);
  assert.match(launchShell, /data-dx-section="account-data-dashboard"/);
  assert.match(launchShell, /data-dx-component="launch-account-data-dashboard"/);
  assert.match(launchShell, /<LaunchSupabaseProfileWorkflow \/>/);
  assert.match(editContract, /id: "supabase-account-data-dashboard"/);
  assert.match(
    editContract,
    /id: "supabase-account-data-dashboard"[\s\S]*selector: '\[data-dx-section="account-data-dashboard"\]'[\s\S]*sourceFile: "examples\/template\/template-shell\.tsx"/,
  );
  assert.match(editContract, /id: "supabase-profile-workflow"/);
  assert.match(
    editContract,
    /id: "supabase-profile-workflow"[\s\S]*selector: '\[data-dx-component="supabase-profile-workflow"\]'[\s\S]*sourceFile: "examples\/template\/supabase-profile-workflow\.tsx"/,
  );
  assert.match(editContract, /id: "supabase-schema-query-workflow"/);
  assert.match(
    editContract,
    /id: "supabase-schema-query-workflow"[\s\S]*selector: '\[data-dx-component="supabase-schema-query-workflow"\]'[\s\S]*interactionSelectors:[\s\S]*'\[data-dx-supabase-action="run-local-schema-query"\]'/,
  );
  assert.match(
    editContract,
    /id: "supabase-schema-query-workflow"[\s\S]*stateMarkers:[\s\S]*"data-dx-supabase-query-operation"/,
  );
  assert.match(editContract, new RegExp(escapeRegExp(dashboardReceiptPath)));
  assert.match(packageCatalog, /dashboardUsage/);
  assert.match(packageCatalog, /account-data-dashboard/);
  assert.match(packageCatalog, /supabase-schema-query/);
  assert.doesNotMatch(packageCatalog, /supabase-client-live-demo/);
  assert.match(packageCatalog, /docs\/packages\/supabase-client\.md/);
  assert.match(packageCatalog, /officialName: "Backend Platform Client"/);
  assert.match(
    packageCatalog,
    /upstreamPackage: "@supabase\/ssr \+ @supabase\/supabase-js"/,
  );
  assert.match(
    packageCatalog,
    /upstreamVersion: "@supabase\/ssr latest; @supabase\/supabase-js \^2"/,
  );
  assert.match(packageCatalog, /"lib\/supabase\/metadata\.ts"/);
  assert.match(packageCatalog, /dxSupabaseForgePackage/);
  assert.match(packageCatalog, /dxCheckVisibility: \{/);
  assert.match(
    packageCatalog,
    /statuses: \["present", "stale", "missing-receipt", "blocked", "unsupported-surface"\]/,
  );
  assert.match(packageCatalog, new RegExp(escapeRegExp(dashboardReceiptPath)));
  assert.match(packageCatalog, new RegExp(escapeRegExp(materializedDashboardReceiptPath)));
  assert.match(packageCatalog, new RegExp(escapeRegExp(dashboardReceiptMarker)));
  assert.match(runtimeLaunch, /data-dx-component="supabase-profile-workflow"/);
  assert.match(runtimeLaunch, /data-dx-supabase-workflow="profile-settings"/);
  assert.match(runtimeLaunch, /data-dx-supabase-action="load-profile-fixture"/);
  assert.match(runtimeLaunch, /data-dx-supabase-action="prepare-profile-upsert"/);
  assert.match(runtimeLaunch, new RegExp(escapeRegExp(dashboardReceiptMarker)));
  assert.match(runtimeLaunch, /<dx-icon name="database:supabase"/);
  assert.match(runtimeScript, /supabase-load-profile/);
  assert.match(runtimeScript, /upsertDxSupabaseProfile\(userId, input\)/);
  assert.match(cli, /NEXT_FAMILIAR_SUPABASE_PROFILE_WORKFLOW_TSX/);
  assert.match(cli, /NEXT_FAMILIAR_SUPABASE_PROFILE_WORKFLOW_STATE_TS/);
  assert.match(cli, /NEXT_FAMILIAR_SUPABASE_DASHBOARD_RECEIPT_JSON/);
  assert.match(cli, /"components\/template-app\/supabase-profile-workflow\.tsx"/);
  assert.match(cli, /"lib\/supabase\/profile-workflow\.ts"/);
  assert.match(cli, /"lib\/supabase\/metadata\.ts"/);
  assert.match(cli, /dxSupabaseForgePackage/);
  assert.match(cli, new RegExp(escapeRegExp(`"${materializedDashboardReceiptPath}"`)));
  assert.match(newCommand, /content: NEXT_FAMILIAR_SUPABASE_DASHBOARD_RECEIPT_JSON\.to_string\(\)/);
  assert.match(cli, /supabase-schema-query/);
  assert.match(cli, /dashboard_usage/);
  assert.match(cli, /"official_name": "Backend Platform Client"/);
  assert.match(cli, /"upstream_package": "@supabase\/ssr \+ @supabase\/supabase-js"/);
  assert.match(
    cli,
    /"upstream_version": "@supabase\/ssr latest; @supabase\/supabase-js \^2"/,
  );
  assert.match(cli, /"dx_check_visibility": \{/);
  assert.match(cli, /"current_status": "present"/);
  assert.match(cli, new RegExp(escapeRegExp(dashboardReceiptPath)));
  assert.match(cli, new RegExp(escapeRegExp(materializedDashboardReceiptPath)));

  assert.match(studioManifest, /"package": "supabase\/client"/);
  assert.match(studioManifest, /"front_facing_name": "Backend Platform Client"/);
  assert.match(studioManifest, /"official_dx_package_name": "Backend Platform Client"/);
  assert.match(studioManifest, /"source_mirror": "G:\/WWW\/inspirations\/supabase"/);
  assert.match(studioManifest, /"dashboard_workflow": "account-profile-settings"/);
  assert.match(studioManifest, /"secondary_dashboard_workflow": "supabase-schema-query"/);
  assert.match(studioManifest, /"LaunchSupabaseProfileWorkflow"/);
  assert.match(studioManifest, /"getDxSupabaseCurrentProfile"/);
  assert.match(studioManifest, /"upsertDxSupabaseProfile"/);
  assert.match(studioManifest, /"readDxSupabaseProfilesReadModel"/);
  assert.match(studioManifest, /"data-dx-supabase-action"/);
  assert.match(studioManifest, /"data-dx-supabase-profile-field"/);
  assert.match(studioManifest, /"data-dx-supabase-query-operation"/);
  assert.match(studioManifest, /"data-dx-supabase-receipt-path"/);
  assert.match(studioManifest, /\[data-dx-supabase-query-operation\]/);
  assert.match(studioManifest, new RegExp(escapeRegExp(dashboardReceiptPath)));
  assert.match(studioManifest, new RegExp(escapeRegExp(materializedDashboardReceiptPath)));

  assert.match(forge, /dashboardUsage/);
  assert.match(forge, /officialName: "Backend Platform Client"/);
  assert.match(forge, /upstreamPackage: "@supabase\/ssr \+ @supabase\/supabase-js"/);
  assert.match(
    forge,
    /upstreamVersion: "@supabase\/ssr latest; @supabase\/supabase-js \^2"/,
  );
  assert.match(forge, /js\/supabase\/profile-workflow\.ts/);
  assert.match(forge, /dashboardProfileWorkflow/);
  assert.match(forge, /readDxSupabaseProfilesReadModel/);
  assert.match(forge, /dxSupabaseProfileFields/);
  assert.match(forge, /account-data-dashboard/);
  assert.match(forge, /supabase-schema-query/);
  assert.match(forge, /upsertDxSupabaseProfile\(userId, input\)/);
  assert.match(forge, /dxCheckVisibility: \{/);
  assert.match(forge, /schema: "dx\.forge\.package\.dx_check_visibility"/);
  assert.match(forge, /currentStatus: "present"/);
  assert.match(forge, /"missing-receipt"/);
  assert.match(forge, /"unsupported-surface"/);
  assert.match(forge, /receiptIntegrity: \{/);
  assert.match(forge, /hashAlgorithm: "sha256"/);
  assert.match(forge, new RegExp(escapeRegExp(dashboardReceiptPath)));
  assert.match(forge, new RegExp(escapeRegExp(materializedDashboardReceiptPath)));

  assert.equal(dashboardReceipt.schema, "dx.forge.package_dashboard_workflow_receipt");
  assert.equal(dashboardReceipt.package_id, "supabase/client");
  assert.equal(dashboardReceipt.package_name, "Backend Platform Client");
  assert.equal(dashboardReceipt.official_dx_package_name, "Backend Platform Client");
  assert.equal(
    dashboardReceipt.upstream_package,
    "@supabase/ssr + @supabase/supabase-js",
  );
  assert.equal(
    dashboardReceipt.upstream_version,
    "@supabase/ssr latest; @supabase/supabase-js ^2",
  );
  assert.equal(dashboardReceipt.route, "/");
  assert.equal(dashboardReceipt.workflow, "account-profile-settings");
  assert.equal(dashboardReceipt.secondary_workflow, "supabase-schema-query");
  assert.equal(dashboardReceipt.surface, "backend-platform-client-dashboard-workflow");
  assert.equal(dashboardReceipt.hash_algorithm, "sha256");
  assert.match(
    dashboardReceipt.hash_scope,
    /selected Backend Platform Client dashboard files/,
  );
  for (const file of hashedReceiptFiles) {
    assert.ok(dashboardReceipt.files.includes(file), `receipt files should include ${file}`);
    assert.equal(dashboardReceipt.file_hashes[file], sha256(file));
  }
  assert.ok(
    dashboardReceipt.hash_exclusions.some(
      (entry) =>
        entry.file === "dx-www/src/cli/mod.rs" && /shared CLI/.test(entry.reason),
    ),
  );
  assert.equal(dashboardReceipt.node_modules_required, false);
  assert.equal(dashboardReceipt.no_runtime_execution, true);
  assert.equal(dashboardReceipt.reality_audit.verdict, "REAL");
  assert.match(dashboardReceipt.reality_audit.classification_scope, /Forge package/);
  assert.match(dashboardReceipt.reality_audit.partial_scope, /hosted Supabase/);
  assert.equal(
    dashboardReceipt.dx_check_visibility.schema,
    "dx.forge.package.dx_check_visibility",
  );
  assert.equal(dashboardReceipt.dx_check_visibility.package_id, "supabase/client");
  assert.equal(
    dashboardReceipt.dx_check_visibility.official_package_name,
    "Backend Platform Client",
  );
  assert.equal(dashboardReceipt.dx_check_visibility.current_status, "present");
  assert.deepEqual(
    dashboardReceipt.dx_check_visibility.status_legend.map((entry) => entry.status),
    dxCheckStatuses,
  );
  assert.ok(
    dashboardReceipt.dx_check_visibility.monitored_surfaces.some(
      (surface) =>
        surface.id === "supabase-profile-workflow" &&
        surface.status === "present" &&
        surface.receipt_path === dashboardReceiptPath,
    ),
  );
  assert.ok(
    dashboardReceipt.dx_check_visibility.monitored_surfaces.some(
      (surface) =>
        surface.id === "supabase-schema-query-workflow" &&
        surface.status === "present" &&
        surface.receipt_path === dashboardReceiptPath,
    ),
  );
  assert.equal(
    dashboardReceipt.reality_audit.upstream_source.mirror,
    "G:/WWW/inspirations/supabase",
  );
  assert.ok(
    dashboardReceipt.reality_audit.upstream_source.evidence_files.includes(
      "examples/user-management/nextjs-user-management/app/account/account-form.tsx",
    ),
  );
  assert.ok(
    dashboardReceipt.reality_audit.upstream_source.public_apis.includes(
      "createBrowserClient",
    ),
  );
  assert.ok(
    dashboardReceipt.reality_audit.forge_package_files.includes(
      "core/src/ecosystem/forge_supabase.rs",
    ),
  );
  assert.ok(
    dashboardReceipt.reality_audit.dashboard_consumers.includes(
      "examples/template/supabase-profile-workflow.tsx",
    ),
  );
  assert.ok(
    dashboardReceipt.reality_audit.receipt_manifest_files.includes(
      "examples/template/dx-studio-edit-contract.ts",
    ),
  );
  assert.ok(
    dashboardReceipt.reality_audit.guard_files.includes(
      "benchmarks/supabase-dashboard-workflow.test.ts",
    ),
  );
  assert.ok(
    dashboardReceipt.reality_audit.visible_workflow_proof.includes(
      "run-local-schema-query",
    ),
  );
  assert.equal(dashboardReceipt.credential_state, "missing-config");
  assert.deepEqual(dashboardReceipt.secret_values, []);
  assert.ok(dashboardReceipt.materialized_files.includes("lib/supabase/metadata.ts"));
  assert.ok(dashboardReceipt.materialized_files.includes(materializedDashboardReceiptPath));
  assert.ok(
    dashboardReceipt.stable_markers.includes('data-dx-component="supabase-profile-workflow"'),
  );
  assert.ok(
    dashboardReceipt.stable_markers.includes(
      'data-dx-component="supabase-schema-query-workflow"',
    ),
  );
  assert.ok(dashboardReceipt.stable_markers.includes("data-dx-supabase-query-operation"));
  assert.ok(
    dashboardReceipt.dashboard_public_apis.includes("readDxSupabaseProfilesReadModel"),
  );
  assert.ok(dashboardReceipt.stable_markers.includes(dashboardReceiptMarker));
  assert.ok(
    dashboardReceipt.local_readiness_interactions.includes("prepare-profile-upsert"),
  );
  assert.equal(dashboardReceipt.local_demo_interactions, undefined);
  assert.ok(
    dashboardReceipt.guards.includes(
      "node --test .\\benchmarks\\supabase-dashboard-workflow.test.ts",
    ),
  );

  assert.match(packageDoc, /^# Backend Platform Client/m);
  assert.match(packageDoc, /Official DX package: Backend Platform Client/);
  assert.match(packageDoc, /packageId: supabase\/client/);
  assert.match(packageDoc, /upstream_package: @supabase\/ssr \+ @supabase\/supabase-js/);
  assert.match(
    packageDoc,
    /upstream_version: @supabase\/ssr latest; @supabase\/supabase-js \^2/,
  );
  assert.match(packageDoc, /data-dx-section="account-data-dashboard"/);
  assert.match(packageDoc, /lib\/supabase\/metadata\.ts/);
  assert.match(packageDoc, /data-dx-supabase-profile-field="fullName"/);
  assert.match(packageDoc, /dxSupabaseProfileFields/);
  assert.match(packageDoc, /readDxSupabaseProfilesReadModel/);
  assert.match(packageDoc, /Reality audit/);
  assert.match(packageDoc, /classifies the slice as `REAL`/);
  assert.match(packageDoc, /dx-check visibility/);
  assert.match(packageDoc, /present, stale, missing-receipt, blocked, and unsupported-surface/);
  assert.match(packageDoc, /hash_algorithm: sha256/);
  assert.match(packageDoc, /file_hashes/);
  assert.match(packageDoc, new RegExp(escapeRegExp(dashboardReceiptMarker)));
  assert.match(packageDoc, new RegExp(escapeRegExp(dashboardReceiptPath)));
  assert.match(packageDoc, new RegExp(escapeRegExp(materializedDashboardReceiptPath)));
  assert.match(packageDoc, /dx run --test \.\\benchmarks\\supabase-dashboard-workflow\.test\.ts/);
  assert.match(dx, /Supabase profile workflow/);
  assert.match(dx, /Backend Platform Client/);
  assert.match(dx, /lib\/supabase\/metadata\.ts/);
  assert.match(dx, /readDxSupabaseProfilesReadModel/);
  assert.match(todo, /Supabase profile workflow/);
  assert.match(todo, /Backend Platform Client/);
  assert.match(todo, /readDxSupabaseProfilesReadModel/);
  assert.match(todo, /lib\/supabase\/metadata\.ts/);
  assert.match(changelog, /Supabase profile workflow/);
  assert.match(changelog, /Backend Platform Client/);
  assert.match(changelog, /readDxSupabaseProfilesReadModel/);
  assert.match(changelog, /lib\/supabase\/metadata\.ts/);
});
