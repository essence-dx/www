const assert = require("assert");
const { execFileSync } = require("node:child_process");
const fs = require("fs");
const os = require("node:os");
const path = require("path");
const test = require("node:test");

const wwwRoot = path.resolve(__dirname, "..");
const dxRoot = path.resolve(wwwRoot, "..");
const cliRoot = path.join(dxRoot, "cli");
const runtimeMaterializer = path.join(
  wwwRoot,
  "tools",
  "launch",
  "materialize-www-template.ts",
);

function readJson(relativePath) {
  const fullPath = path.join(wwwRoot, relativePath);
  assert.ok(fs.existsSync(fullPath), `expected ${relativePath} to exist`);
  return JSON.parse(fs.readFileSync(fullPath, "utf8"));
}

function readText(fullPath) {
  assert.ok(fs.existsSync(fullPath), `expected ${fullPath} to exist`);
  return fs.readFileSync(fullPath, "utf8");
}

test("automations bridge manifests are generated from the copied n8n source", () => {
  const connectors = readJson(
    "integrations/n8n-nodes-base/generated/dx-automations-connectors.json",
  );
  const credentials = readJson(
    "integrations/n8n-nodes-base/generated/dx-automations-credentials.json",
  );
  const readiness = readJson(
    "integrations/n8n-nodes-base/generated/dx-automations-readiness.json",
  );

  assert.equal(connectors.schema, "dx.automations.connectors");
  assert.equal(credentials.schema, "dx.automations.credentials");
  assert.equal(readiness.schema, "dx.automations.workflow_readiness");
  assert.ok(connectors.summary.connector_count >= 500);
  assert.ok(credentials.summary.credential_count >= 350);
  assert.equal(connectors.source.provenance, "n8n-nodes-base");
  assert.match(connectors.source.manifest, /dx-node-source-manifest\.json$/);
  assert.match(readiness.bridge.commands.connectors, /dx automations connectors --json/);

  const slack = connectors.connectors.find((item) => item.id === "n8n-nodes-base.slack");
  assert.ok(slack, "Slack connector should be present");
  assert.equal(slack.display_name, "Slack");
  assert.ok(slack.credential_type_names.includes("slackApi"));
  assert.ok(slack.credential_type_names.includes("slackOAuth2Api"));
  assert.ok(slack.auth_kinds.includes("bearer_token"));
  assert.ok(slack.auth_kinds.includes("oauth2"));
  assert.ok(slack.resources.some((resource) => resource.value === "message"));
  assert.ok(slack.operations.some((operation) => operation.value === "post"));
  assert.equal(slack.status, "needs_credential");

  const slackApi = credentials.credentials.find((item) => item.id === "slackApi");
  assert.ok(slackApi, "Slack API credential should be present");
  assert.equal(slackApi.auth_kind, "bearer_token");
  assert.ok(slackApi.secret_fields.includes("accessToken"));
  assert.ok(slackApi.fields.every((field) => !String(field.default ?? "").includes("{{$credentials")));
});

test("automations routes and CLI bridge are source-owned and Zed-readable", () => {
  const routeFiles = [
    "examples/template/app/automations/page.tsx",
    "examples/template/app/automations/connectors/page.tsx",
    "examples/template/app/automations/credentials/page.tsx",
    "examples/template/app/automations/workflows/page.tsx",
    "examples/template/automations-status.tsx",
    "examples/template/automations/automations-shell.tsx",
    "examples/template/automations/automations-metadata.ts",
  ];

  for (const relativePath of routeFiles) {
    const source = readText(path.join(wwwRoot, relativePath));
    assert.doesNotMatch(source, /n8n-icon|NodeCreator|CanvasNode/);
    assert.match(source, /DX Automations|automation|Automations/);
  }

  const cli = readText(path.join(cliRoot, "src", "cli.rs"));
  const main = readText(path.join(cliRoot, "src", "main.rs"));
  const commandDispatch = readText(path.join(cliRoot, "src", "command_dispatch.rs"));
  const ecosystemCommandFlow = readText(
    path.join(cliRoot, "src", "ecosystem_command_flow.rs"),
  );
  const lib = readText(path.join(cliRoot, "src", "lib.rs"));
  const bridge = readText(path.join(cliRoot, "src", "automations_bridge.rs"));
  const launchStatus = readText(
    path.join(wwwRoot, "examples/template/automations-status.tsx"),
  );
  const automationMissionSummary = readText(
    path.join(wwwRoot, "examples/template/automation-mission-summary.tsx"),
  );
  const launchShell = readText(
    path.join(wwwRoot, "examples/template/template-shell.tsx"),
  );
  const runtimeLaunch = readText(
    path.join(wwwRoot, "tools/launch/runtime-template/pages/index.html"),
  );
  const runtimeScript = readText(
    path.join(wwwRoot, "tools/launch/runtime-template/assets/launch-runtime.ts"),
  );
  const wwwCli = readText(path.join(wwwRoot, "dx-www", "src", "cli", "mod.rs"));

  assert.match(cli, /Automations\(AutomationsArgs\)/);
  assert.match(cli, /dx automations connectors --json/);
  assert.match(cli, /dx automations credentials --json/);
  assert.match(cli, /dx automations run --json/);
  assert.match(main, /command_dispatch::run_command/);
  assert.match(main, /run_command\(registry, invocation_dir, command\)/);
  assert.match(commandDispatch, /Command::Automations\(args\) => run_automations/);
  assert.match(ecosystemCommandFlow, /automations_bridge::run\(registry, &args\)/);
  assert.match(lib, /pub mod automations_bridge;/);
  assert.match(bridge, /dx\.automations\.zed\.run_receipt/);
  assert.match(bridge, /\.dx.+receipts.+automations/);
  assert.match(bridge, /launch-release-notification\.json/);
  assert.match(bridge, /dashboard_workflow/);
  assert.match(bridge, /automation-release-receipt/);
  assert.match(bridge, /source_selector/);
  assert.match(bridge, /launch-automation-dashboard-workflow/);
  assert.match(bridge, /receipt_seed_path/);
  assert.match(bridge, /workflow_intent/);
  assert.match(bridge, /read_launch_workflow_intent/);
  assert.match(bridge, /required_env/);
  assert.match(bridge, /read_launch_required_env/);
  assert.match(bridge, /secret_values/);
  assert.match(launchStatus, /"use client";/);
  assert.match(launchStatus, /surface\?: "dashboard" \| "connector-readiness"/);
  assert.match(launchStatus, /surface = "connector-readiness"/);
  assert.match(
    launchStatus,
    /workflowId:\s*surface === "dashboard"\s*\?\s*"launch-release-notification"\s*:\s*"connector-readiness"/,
  );
  assert.doesNotMatch(launchStatus, /surface\?: "dashboard" \| "proof"/);
  assert.doesNotMatch(launchStatus, /surface = "proof"/);
  assert.doesNotMatch(launchStatus, /launch-dashboard-preview/);
  assert.match(launchStatus, /data-dx-package="automations\/n8n"/);
  assert.match(launchStatus, /data-dx-component="launch-automation-connector-workflow"/);
  assert.match(launchStatus, /data-dx-component="launch-automation-catalog-summary"/);
  assert.match(launchStatus, /data-dx-automation-workflow="connector-readiness"/);
  assert.doesNotMatch(launchStatus, /automations-n8n-live-demo/);
  assert.doesNotMatch(launchStatus, /data-dx-automation-demo=/);
  assert.match(launchStatus, /<dx-icon name="pack:n8n"/);
  assert.match(launchStatus, /data-dx-icon-search="pack:n8n"/);
  assert.match(launchStatus, /data-dx-automation-interaction="connector-picker"/);
  assert.match(launchStatus, /data-dx-automation-interaction="connector-filter"/);
  assert.match(launchStatus, /data-dx-automation-filter=\{filter\.id\}/);
  assert.match(
    launchStatus,
    /data-dx-automation-filter-active=\{selected \? "true" : "false"\}/,
  );
  assert.match(launchStatus, /filter\(\(connector\) => connector\.status === "ready"\)/);
  assert.match(
    launchStatus,
    /filter\(\(connector\) => connector\.status === "needs_credential"\)/,
  );
  assert.match(launchStatus, /filterDxN8nConnectors\(normalizedConnectors, connectorFilter\)/);
  assert.match(launchStatus, /data-dx-automation-connector=\{connector\.id\}/);
  assert.match(
    launchStatus,
    /data-dx-automation-connector-status=\{workflowStatusForConnector\(connector\)\}/,
  );
  assert.match(
    launchStatus,
    /data-dx-automation-selected-connector=\{selectedConnector\.id\}/,
  );
  assert.match(
    launchStatus,
    /data-dx-automation-workflow-status=\{workflowStatus\}/,
  );
  assert.match(
    launchStatus,
    /data-dx-automation-required-env=\{readiness\.requiredEnv\.join\(","\)\}/,
  );
  assert.match(
    launchStatus,
    /data-dx-automation-readiness-card=\{workflowStatus\}/,
  );
  assert.match(
    launchStatus,
    /data-dx-automation-resource-list=\{selectedConnector\.id\}/,
  );
  assert.match(
    launchStatus,
    /data-dx-automation-operation-list=\{selectedConnector\.id\}/,
  );
  assert.match(launchStatus, /data-dx-automation-operation=\{operation\.value\}/);
  assert.match(
    launchStatus,
    /data-dx-automation-receipt-status=\{\s*draftReceipt \? draftReceipt\.status : "idle"\s*\}/,
  );
  assert.match(
    launchStatus,
    /data-dx-automation-local-receipt="draft-workflow-receipt"/,
  );
  assert.doesNotMatch(launchStatus, /data-dx-automation-local-demo=/);
  assert.match(
    launchStatus,
    /data-dx-automation-receipt-state=\{draftReceipt \? "created" : "idle"\}/,
  );
  assert.match(
    launchStatus,
    /data-dx-automation-safe-action="prepare-dry-run-receipt"/,
  );
  assert.doesNotMatch(launchStatus, /fetch\(/);
  assert.doesNotMatch(launchStatus, /process\.env/);
  assert.match(launchStatus, /setDraftReceipt/);
  assert.match(
    launchShell,
    /import \{\s*LaunchAutomationBridgeStatus,\s*type LaunchAutomationDashboardState,\s*\} from "\.\/automations-status";/s,
  );
  assert.match(
    launchShell,
    /import \{\s*LaunchAutomationMissionSummary\s*\} from "\.\/automation-mission-summary";/,
  );
  assert.match(
    launchShell,
    /data-dx-component="launch-automation-dashboard-workflow"/,
  );
  assert.match(launchShell, /React\.useState<LaunchAutomationDashboardState>/);
  assert.match(
    launchShell,
    /<LaunchAutomationBridgeStatus\s+onWorkflowChange=\{setAutomationDashboardState\}\s+surface="dashboard"\s+\/>/s,
  );
  assert.doesNotMatch(launchShell, /function LaunchAutomationMissionSummary/);
  assert.match(automationMissionSummary, /export function LaunchAutomationMissionSummary/);
  assert.match(automationMissionSummary, /data-dx-component="launch-automation-mission-summary"/);
  assert.match(
    automationMissionSummary,
    /data-dx-automation-dashboard-state=\{state\.receiptState\}/,
  );
  assert.match(launchShell, /data-dx-package="automations\/n8n"/);
  assert.doesNotMatch(runtimeLaunch, /data-dx-component="automations-n8n-summary"/);
  assert.match(
    runtimeLaunch,
    /data-dx-component="launch-automation-dashboard-workflow"/,
  );
  assert.match(runtimeLaunch, /data-dx-component="launch-automation-connector-workflow"/);
  assert.match(runtimeLaunch, /data-dx-component="launch-automation-catalog-summary"/);
  assert.match(runtimeLaunch, /data-dx-package="automations\/n8n"/);
  assert.match(runtimeLaunch, /data-dx-automation-interaction="connector-picker"/);
  assert.match(runtimeLaunch, /data-dx-automation-interaction="workflow-readiness"/);
  assert.match(runtimeLaunch, /data-dx-automation-safe-action="prepare-dry-run-receipt"/);
  assert.match(runtimeLaunch, /data-dx-automation-receipt-status="idle"/);
  assert.match(runtimeLaunch, /id="automation-receipt-output"/);
  assert.match(runtimeLaunch, /data-dx-automation-receipt-output="local-draft"/);
  assert.match(runtimeScript, /function bindAutomations\(\)/);
  assert.match(runtimeScript, /const receiptOutput = \$\("#automation-receipt-output"\)/);
  assert.match(runtimeScript, /receiptOutput\.dataset\.dxAutomationDraftState/);
  assert.match(runtimeScript, /dataset\.dxAutomationReceiptStatus/);
  assert.match(runtimeScript, /live execution remains credential-gated/);
  assert.match(wwwCli, /NEXT_FAMILIAR_AUTOMATIONS_STATUS_TSX/);
  assert.match(wwwCli, /"components\/template-app\/automations-status\.tsx"/);
});

test("automations Forge package exposes a real n8n-shaped public API", () => {
  const forgeSlice = readText(
    path.join(wwwRoot, "core", "src", "ecosystem", "forge_n8n_automations.rs"),
  );
  const launchStatus = readText(
    path.join(wwwRoot, "examples", "template", "automations-status.tsx"),
  );
  const packageCatalog = readText(
    path.join(wwwRoot, "examples", "template", "package-catalog.ts"),
  );
  const packageDoc = readText(
    path.join(wwwRoot, "docs", "packages", "automations-n8n.md"),
  );
  const studioManifest = readText(
    path.join(wwwRoot, "dx-www", "src", "cli", "studio_manifest.rs"),
  );
  const wwwCli = readText(path.join(wwwRoot, "dx-www", "src", "cli", "mod.rs"));

  for (const file of [
    "js/lib/automations/n8n/catalog.ts",
    "js/lib/automations/n8n/readiness.ts",
    "js/lib/automations/n8n/receipt.ts",
  ]) {
    assert.match(forgeSlice, new RegExp(file.replace(/[/.]/g, "\\$&")));
  }

  for (const publicApi of [
    "DxN8nUpstreamConnector",
    "normalizeDxN8nConnector",
    "filterDxN8nConnectors",
    "buildDxN8nCredentialReadiness",
    "requiredEnvForDxN8nConnector",
    "buildDxN8nWorkflowDraft",
    "DX_N8N_RUN_RECEIPT_SCHEMA",
    "createDxN8nRunReceipt",
  ]) {
    assert.match(forgeSlice, new RegExp(`export (?:type |const |function )${publicApi}`));
  }

  assert.match(forgeSlice, /aliases: \["n8n", "n8n-nodes-base", "workflows\/n8n"\]/);
  assert.match(forgeSlice, /officialName: "Automation Connectors"/);
  assert.match(forgeSlice, /upstreamPackage: "n8n-nodes-base"/);
  assert.match(forgeSlice, /upstreamVersion: "2\.22\.0"/);
  assert.match(forgeSlice, /inspectedSourceFiles:/);
  assert.match(
    forgeSlice,
    /"packages\/nodes-base\/nodes\/ManualTrigger\/ManualTrigger\.node\.ts"/,
  );
  assert.match(
    forgeSlice,
    /"packages\/nodes-base\/nodes\/Slack\/V2\/SlackV2\.node\.ts"/,
  );
  assert.match(
    forgeSlice,
    /"packages\/nodes-base\/nodes\/Webhook\/Webhook\.node\.ts"/,
  );
  assert.match(forgeSlice, /selectedSurfaces:/);
  assert.match(forgeSlice, /dxCheckVisibility:/);
  assert.match(
    forgeSlice,
    /statuses: \["present", "stale", "missing-receipt", "blocked", "unsupported-surface"\]/,
  );
  assert.match(forgeSlice, /honestyLabel: "ADAPTER-BOUNDARY"/);
  assert.match(forgeSlice, /sourceMirror: "G:\/WWW\/inspirations\/n8n"/);
  assert.match(forgeSlice, /provenance: "n8n-nodes-base"/);
  assert.match(forgeSlice, /exportedFiles:/);
  assert.match(forgeSlice, /requiredEnv: \[/);
  assert.match(forgeSlice, /"SLACK_BOT_TOKEN"/);
  assert.match(forgeSlice, /"SLACK_SIGNING_SECRET"/);
  assert.match(forgeSlice, /"NOTION_API_KEY"/);
  assert.match(forgeSlice, /"DX_AUTOMATIONS_OPERATOR_APPROVAL"/);
  assert.match(forgeSlice, /requiredEnvByCredentialType/);
  assert.match(forgeSlice, /appOwnedBoundaries:/);
  assert.match(forgeSlice, /\.dx\/forge\/receipts\/automations/);
  assert.match(forgeSlice, /launch-release-notification\.json/);
  assert.match(forgeSlice, /G:\/Dx\/\.dx\/receipts\/automations\/run-latest\.json/);
  assert.match(forgeSlice, /display_name/);
  assert.match(forgeSlice, /credential_type_names/);
  assert.match(forgeSlice, /usable_as_tool/);
  assert.match(forgeSlice, /receiptIntentMarker: "data-dx-automation-receipt-intent"/);
  assert.match(forgeSlice, /runReceiptIntentMarker: "data-dx-automation-run-receipt-intent"/);

  assert.match(launchStatus, /@\/lib\/automations\/n8n\/catalog/);
  assert.match(launchStatus, /@\/lib\/automations\/n8n\/readiness/);
  assert.match(launchStatus, /@\/lib\/automations\/n8n\/receipt/);
  assert.match(launchStatus, /buildDxN8nCredentialReadiness\(selectedConnector\)/);
  assert.match(launchStatus, /readiness\.requiredEnv/);
  assert.match(launchStatus, /createDxN8nRunReceipt\(/);
  assert.match(
    forgeSlice,
    /export type DxN8nRunReceiptInput = \{[\s\S]*intent\?: string;/,
  );
  assert.match(
    forgeSlice,
    /export type DxN8nRunReceipt = \{[\s\S]*workflowIntent: string;/,
  );
  assert.match(forgeSlice, /const workflowIntent = input\.intent\?\.trim\(\) \?\? ""/);
  assert.match(launchStatus, /intent: workflowIntent/);
  assert.match(launchStatus, /draftReceipt\.workflowIntent/);
  assert.match(launchStatus, /zedHandoffReceipt\.workflowIntent/);
  assert.match(packageCatalog, /"createDxN8nRunReceipt"/);
  assert.match(packageCatalog, /officialName: "Automation Connectors"/);
  assert.match(packageCatalog, /upstreamPackage: "n8n-nodes-base"/);
  assert.match(packageCatalog, /upstreamVersion: "2\.22\.0"/);
  assert.match(packageCatalog, /command: "dx add automation-connectors --write"/);
  assert.match(packageCatalog, /honestyLabel: "ADAPTER-BOUNDARY"/);
  assert.match(packageCatalog, /dxCheckVisibility:/);
  assert.match(packageDoc, /^# Automation Connectors/m);
  assert.match(packageDoc, /Official DX package name: `Automation Connectors`/);
  assert.match(packageDoc, /Upstream package: `n8n-nodes-base` `2\.22\.0`/);
  assert.match(packageDoc, /`ITriggerFunctions`/);
  assert.match(packageDoc, /`IExecuteFunctions`/);
  assert.match(packageDoc, /`IWebhookFunctions`/);
  assert.match(packageDoc, /nodes\/Slack\/V2\/SlackV2\.node\.ts/);
  assert.match(packageDoc, /nodes\/Webhook\/Webhook\.node\.ts/);
  assert.match(studioManifest, /"createDxN8nRunReceipt"/);
  assert.match(studioManifest, /"data-dx-automation-receipt-intent"/);
  assert.match(studioManifest, /"data-dx-automation-run-receipt-intent"/);
  assert.match(studioManifest, /"data-dx-automation-required-env"/);
  assert.doesNotMatch(packageCatalog, /createDxAutomationRunReceipt/);
  assert.doesNotMatch(studioManifest, /createDxAutomationRunReceipt/);
  assert.match(wwwCli, /"official_name": "Automation Connectors"/);
  assert.match(wwwCli, /"upstream_package": "n8n-nodes-base"/);
  assert.match(wwwCli, /"upstream_version": "2\.22\.0"/);
  assert.match(wwwCli, /"command": "dx add automation-connectors --write"/);
  assert.match(wwwCli, /"normalizeDxN8nConnector"/);
  assert.match(wwwCli, /"buildDxN8nCredentialReadiness"/);
  assert.match(wwwCli, /"requiredEnvForDxN8nConnector"/);
});

test("automations workflow materializes into the generated launch route", () => {
  const projectDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-automations-launch-"));

  execFileSync(process.execPath, [runtimeMaterializer, projectDir], {
    cwd: wwwRoot,
    encoding: "utf8",
  });

  const launch = readText(path.join(projectDir, "pages", "index.html"));
  const script = readText(path.join(projectDir, "public", "launch-runtime.js"));

  assert.match(launch, /data-dx-route="\/"/);
  assert.doesNotMatch(launch, /data-dx-component="automations-n8n-summary"/);
  assert.match(
    launch,
    /data-dx-component="launch-automation-dashboard-workflow"/,
  );
  assert.match(launch, /data-dx-component="launch-automation-connector-workflow"/);
  assert.match(launch, /data-dx-component="launch-automation-catalog-summary"/);
  assert.match(launch, /data-dx-package="automations\/n8n"/);
  assert.match(launch, /data-dx-automation-interaction="connector-picker"/);
  assert.match(launch, /data-dx-automation-interaction="workflow-readiness"/);
  assert.match(launch, /data-dx-automation-missing-config="false"/);
  assert.match(launch, /data-dx-automation-required-env="DX_AUTOMATIONS_OPERATOR_APPROVAL"/);
  assert.match(launch, /data-dx-automation-selected-connector="n8n-nodes-base\.manualTrigger"/);
  assert.match(launch, /data-dx-automation-connector-status="missing-config"/);
  assert.match(launch, /data-dx-automation-safe-action="prepare-dry-run-receipt"/);
  assert.match(launch, /id="automation-receipt-output"/);
  assert.match(launch, /data-dx-automation-receipt-output="local-draft"/);
  assert.match(script, /function bindAutomations\(\)/);
  assert.match(script, /const receiptOutput = \$\("#automation-receipt-output"\)/);
  assert.match(script, /receiptOutput\.dataset\.dxAutomationDraftState/);
  assert.match(script, /Draft workflow receipt/);
});
