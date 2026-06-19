const assert = require("node:assert/strict");
const { execFileSync } = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const hubRoot = path.dirname(root);
const launchAutomationReceiptPath = path.join(
  hubRoot,
  ".dx",
  "receipts",
  "automations",
  "launch-release-notification.json",
);
const zedRunReceiptPath = path.join(
  hubRoot,
  ".dx",
  "receipts",
  "automations",
  "run-latest.json",
);

function read(relativePath) {
  const fullPath = path.join(root, relativePath);
  assert.ok(fs.existsSync(fullPath), `expected ${relativePath} to exist`);
  return fs.readFileSync(fullPath, "utf8");
}

test("automations/n8n is visibly demoable in generated launch template", () => {
  const launchShell = read("examples/template/template-shell.tsx");
  const automationMissionSummary = read(
    "examples/template/automation-mission-summary.tsx",
  );
  const launchStatus = read("examples/template/automations-status.tsx");
  const runtimeLaunch = read("tools/launch/runtime-template/pages/index.html");
  const runtimeScript = read("tools/launch/runtime-template/assets/launch-runtime.ts");
  const editContract = read("examples/template/dx-studio-edit-contract.ts");
  const packageCatalog = read("examples/template/package-catalog.ts");
  const routeContract = read("examples/template/template-route-contract.ts");
  const cli = read("dx-www/src/cli/mod.rs");

  assert.match(
    launchShell,
    /import \{\s*LaunchAutomationBridgeStatus,\s*type LaunchAutomationDashboardState,\s*\} from "\.\/automations-status";/s,
  );
  assert.match(
    launchShell,
    /import \{\s*LaunchAutomationMissionSummary\s*\} from "\.\/automation-mission-summary";/,
  );
  assert.match(launchShell, /data-dx-package="automations\/n8n"/);
  assert.match(launchShell, /data-dx-section="launch-automation-ops"/);
  assert.match(launchShell, /data-dx-component="launch-automation-dashboard-workflow"/);
  assert.match(launchShell, /data-dx-dashboard-workflow="automation-release-receipt"/);
  assert.match(launchShell, /React\.useState<LaunchAutomationDashboardState>/);
  assert.match(
    launchShell,
    /<LaunchAutomationBridgeStatus\s+onWorkflowChange=\{setAutomationDashboardState\}\s+surface="dashboard"\s+\/>/s,
  );
  assert.doesNotMatch(launchShell, /function LaunchAutomationMissionSummary/);
  assert.match(automationMissionSummary, /"use client";/);
  assert.match(
    automationMissionSummary,
    /import type \{\s*LaunchAutomationDashboardState\s*\} from "\.\/automations-status";/s,
  );
  assert.match(automationMissionSummary, /type LaunchAutomationMissionSummaryProps = \{/);
  assert.match(automationMissionSummary, /export function LaunchAutomationMissionSummary/);
  assert.match(automationMissionSummary, /data-dx-component="launch-automation-mission-summary"/);
  assert.match(automationMissionSummary, /data-dx-dashboard-card="automation"/);
  assert.match(
    automationMissionSummary,
    /data-dx-automation-dashboard-state=\{state\.receiptState\}/,
  );
  assert.match(
    automationMissionSummary,
    /data-dx-automation-selected-connector=\{state\.connectorId\}/,
  );
  assert.match(
    automationMissionSummary,
    /data-dx-automation-workflow-node-readiness=\{state\.workflowNodeReadiness\}/,
  );
  assert.match(
    launchShell,
    /selector: '\[data-dx-component="launch-automation-dashboard-workflow"\]'/,
  );
  assert.doesNotMatch(launchShell, /automations-n8n-proof/);
  assert.doesNotMatch(launchShell, /data-dx-component="automations-n8n-launch-card"/);

  assert.match(launchStatus, /"use client";/);
  assert.match(launchStatus, /export type LaunchAutomationDashboardState = \{/);
  assert.match(launchStatus, /onWorkflowChange\?: \(state: LaunchAutomationDashboardState\) => void/);
  assert.match(launchStatus, /const dashboardState = React\.useMemo<LaunchAutomationDashboardState>/);
  assert.match(launchStatus, /onWorkflowChange\?\.\(dashboardState\)/);
  assert.match(launchStatus, /data-dx-package="automations\/n8n"/);
  assert.match(launchStatus, /data-dx-component="launch-automation-connector-workflow"/);
  assert.match(launchStatus, /data-dx-component="launch-automation-catalog-summary"/);
  assert.match(launchStatus, /data-dx-dashboard-workflow="automation-release-receipt"/);
  assert.match(launchStatus, /data-dx-automation-dashboard-state=\{dashboardState\.receiptState\}/);
  assert.match(launchStatus, /data-dx-automation-dashboard-connector=\{dashboardState\.connectorId\}/);
  assert.match(
    launchStatus,
    /data-dx-automation-dashboard-node-readiness=\{dashboardState\.workflowNodeReadiness\}/,
  );
  assert.match(launchStatus, /data-dx-automation-dashboard-card="launch-release-notification"/);
  assert.match(launchStatus, /data-dx-automation-intent-input="release-notification"/);
  assert.match(launchStatus, /data-dx-automation-receipt-path=/);
  assert.match(launchStatus, /data-dx-automation-receipt-intent=/);
  assert.match(launchStatus, /data-dx-automation-run-receipt-intent=/);
  assert.match(launchStatus, /data-dx-automation-required-env=\{readiness\.requiredEnv\.join\(","\)\}/);
  assert.match(launchStatus, /data-dx-automation-workflow="connector-readiness"/);
  assert.doesNotMatch(launchStatus, /automations-n8n-live-demo/);
  assert.doesNotMatch(launchStatus, /data-dx-automation-demo=/);
  assert.match(launchStatus, /data-dx-automation-interaction="connector-filter"/);
  assert.match(launchStatus, /data-dx-automation-interaction="connector-picker"/);
  assert.match(launchStatus, /data-dx-automation-interaction="workflow-readiness"/);
  assert.match(launchStatus, /data-dx-automation-connector=\{connector\.id\}/);
  assert.match(
    launchStatus,
    /data-dx-automation-readiness-card=\{workflowStatus\}/,
  );
  assert.match(launchStatus, /data-dx-automation-credential-schema=\{selectedConnector\.id\}/);
  assert.match(launchStatus, /data-dx-automation-auth-kind=\{authKind\}/);
  assert.match(launchStatus, /data-dx-automation-credential-type=\{credentialName\}/);
  assert.match(
    launchStatus,
    /data-dx-automation-workflow-node-readiness=\{\s*selectedConnector\.workflowNode\.runMode\s*\}/,
  );
  assert.match(
    launchStatus,
    /data-dx-automation-usable-as-tool=\{\s*selectedConnector\.workflowNode\.usableAsTool \? "true" : "false"\s*\}/,
  );
  assert.match(
    launchStatus,
    /data-dx-automation-missing-config=\{\s*readiness\.missingCredentials\.length > 0 \? "true" : "false"\s*\}/,
  );
  assert.match(
    launchStatus,
    /data-dx-automation-local-receipt="draft-workflow-receipt"/,
  );
  assert.doesNotMatch(launchStatus, /data-dx-automation-local-demo=/);
  assert.match(
    launchStatus,
    /data-dx-automation-safe-action="prepare-dry-run-receipt"/,
  );
  assert.match(launchStatus, /launchAutomationRunReceiptPath/);
  assert.match(launchStatus, /data-dx-automation-handoff="zed-run-receipt"/);
  assert.match(
    launchStatus,
    /data-dx-automation-safe-action="prepare-zed-run-handoff"/,
  );
  assert.match(launchStatus, /setZedHandoffReceipt/);
  assert.match(launchStatus, /mode: "run"/);
  assert.match(launchStatus, /setDraftReceipt/);
  assert.doesNotMatch(launchStatus, /fetch\(/);
  assert.doesNotMatch(launchStatus, /process\.env/);
  assert.match(
    editContract,
    /selector: '\[data-dx-component="launch-automation-dashboard-workflow"\]'/,
  );
  assert.match(editContract, /data-dx-automation-receipt-intent/);
  assert.match(editContract, /data-dx-automation-run-receipt-intent/);
  assert.match(editContract, /data-dx-automation-required-env/);
  assert.doesNotMatch(
    editContract,
    /selector: '\[data-dx-component="automations-n8n-status"\]'/,
  );

  assert.match(runtimeLaunch, /data-dx-package="automations\/n8n"/);
  assert.match(runtimeLaunch, /data-dx-component="launch-automation-mission-summary"/);
  assert.match(runtimeLaunch, /data-dx-automation-dashboard-state="idle"/);
  assert.match(runtimeLaunch, /data-dx-automation-selected-connector="n8n-nodes-base\.manualTrigger"/);
  assert.match(runtimeLaunch, /data-dx-automation-workflow-node-readiness="metadata-ready"/);
  assert.match(runtimeLaunch, /id="mission-automation-schema"/);
  assert.match(runtimeLaunch, /data-dx-component="launch-automation-connector-workflow"/);
  assert.match(runtimeLaunch, /data-dx-component="launch-automation-catalog-summary"/);
  assert.match(runtimeLaunch, /data-dx-component="launch-automation-dashboard-workflow"/);
  assert.match(runtimeLaunch, /data-dx-dashboard-workflow="automation-release-receipt"/);
  assert.match(runtimeLaunch, /data-dx-automation-dashboard-card="launch-release-notification"/);
  assert.match(runtimeLaunch, /data-dx-automation-intent-input="release-notification"/);
  assert.match(runtimeLaunch, /data-dx-automation-receipt-path="G:\/Dx\/\.dx\/receipts\/automations\/launch-release-notification\.json"/);
  assert.ok(
    runtimeLaunch.indexOf('data-dx-component="launch-automation-dashboard-workflow"') <
      runtimeLaunch.indexOf('id="proof-grid"'),
    "automation dashboard workflow should appear before the package proof grid",
  );
  assert.match(runtimeLaunch, /data-dx-automation-workflow="connector-readiness"/);
  assert.doesNotMatch(runtimeLaunch, /automations-n8n-live-demo/);
  assert.doesNotMatch(runtimeLaunch, /data-dx-automation-demo=/);
  assert.match(runtimeLaunch, /data-dx-automation-interaction="connector-filter"/);
  assert.match(runtimeLaunch, /data-dx-automation-filter="metadata-ready"/);
  assert.match(runtimeLaunch, /data-dx-automation-filter="missing-config"/);
  assert.match(runtimeLaunch, /data-dx-automation-interaction="connector-picker"/);
  assert.match(runtimeLaunch, /data-dx-automation-connector="n8n-nodes-base\.manualTrigger"/);
  assert.match(runtimeLaunch, /data-dx-automation-connector="n8n-nodes-base\.slack"/);
  assert.match(runtimeLaunch, /data-dx-automation-interaction="workflow-readiness"/);
  assert.match(runtimeLaunch, /data-dx-automation-missing-config="false"/);
  assert.match(runtimeLaunch, /id="automation-schema-summary"/);
  assert.match(runtimeLaunch, /data-dx-automation-credential-schema="n8n-nodes-base\.manualTrigger"/);
  assert.match(runtimeLaunch, /data-dx-automation-auth-kinds="none"/);
  assert.match(runtimeLaunch, /data-dx-automation-required-env="DX_AUTOMATIONS_OPERATOR_APPROVAL"/);
  assert.match(runtimeLaunch, /data-dx-automation-workflow-node-readiness="metadata-ready"/);
  assert.match(runtimeLaunch, /data-dx-automation-usable-as-tool="false"/);
  assert.match(runtimeLaunch, /data-dx-automation-auth-kinds="bearer_token,oauth2"/);
  assert.match(runtimeLaunch, /data-dx-automation-credential-types="slackApi,slackOAuth2Api"/);
  assert.match(runtimeLaunch, /data-dx-automation-local-receipt="draft-workflow-receipt"/);
  assert.doesNotMatch(runtimeLaunch, /data-dx-automation-local-demo=/);
  assert.match(runtimeLaunch, /data-dx-automation-safe-action="prepare-dry-run-receipt"/);
  assert.match(runtimeLaunch, /data-dx-automation-safe-action-state="idle"/);
  assert.match(runtimeLaunch, /data-dx-automation-receipt-output="local-draft"/);
  assert.match(runtimeLaunch, /data-dx-automation-receipt-intent=/);
  assert.match(runtimeLaunch, /id="automation-zed-run-handoff"/);
  assert.match(runtimeLaunch, /data-dx-automation-handoff="zed-run-receipt"/);
  assert.match(runtimeLaunch, /data-dx-automation-safe-action="prepare-zed-run-handoff"/);
  assert.match(runtimeLaunch, /data-dx-automation-run-receipt-path="G:\/Dx\/\.dx\/receipts\/automations\/run-latest\.json"/);
  assert.match(runtimeLaunch, /id="automation-zed-output"/);
  assert.match(runtimeLaunch, /data-dx-automation-run-receipt-output="zed-handoff"/);
  assert.match(runtimeLaunch, /data-dx-automation-run-receipt-intent=/);

  assert.match(runtimeScript, /function bindAutomations\(\)/);
  assert.match(
    runtimeScript,
    /const automationMission = \$\('\[data-dx-component="launch-automation-mission-summary"\]'\)/,
  );
  assert.match(runtimeScript, /automationMission\.dataset\.dxAutomationSelectedConnector = state\.automationConnector/);
  assert.match(runtimeScript, /automationMission\.dataset\.dxAutomationWorkflowNodeReadiness/);
  assert.match(runtimeScript, /#mission-automation-schema/);
  assert.match(runtimeScript, /const intentInput = \$\("#automation-intent"\)/);
  assert.match(runtimeScript, /const zedRun = \$\("#automation-zed-run-handoff"\)/);
  assert.match(runtimeScript, /const zedOutput = \$\("#automation-zed-output"\)/);
  assert.match(runtimeScript, /const schemaSummary = \$\("#automation-schema-summary"\)/);
  assert.match(runtimeScript, /data-dx-automation-connector/);
  assert.match(runtimeScript, /data-dx-automation-filter/);
  assert.match(runtimeScript, /dxAutomationCredentialSchema = connector/);
  assert.match(runtimeScript, /dxAutomationRequiredEnv = requiredEnv/);
  assert.match(runtimeScript, /dxAutomationWorkflowNodeReadiness = workflowMode/);
  assert.match(runtimeScript, /dxAutomationUsableAsTool = usableAsTool/);
  assert.match(runtimeScript, /applyFilter/);
  assert.match(runtimeScript, /automation-receipt-output/);
  assert.match(runtimeScript, /receipt\.dataset\.dxAutomationSafeActionState = "created"/);
  assert.match(runtimeScript, /dxAutomationReceiptIntent/);
  assert.match(runtimeScript, /dxAutomationDraftState = "created"/);
  assert.match(runtimeScript, /dxAutomationZedRunState = "created"/);
  assert.match(runtimeScript, /dxAutomationRunReceiptIntent/);
  assert.match(runtimeScript, /dx automations run --json/);
  assert.match(runtimeScript, /G:\/Dx\/\.dx\/receipts\/automations\/run-latest\.json/);
  assert.match(runtimeScript, /dxDashboardAutomationWorkflow/);

  assert.match(packageCatalog, /packageId: "automations\/n8n"/);
  assert.match(packageCatalog, /sourceMirror: "G:\/WWW\/inspirations\/n8n\/packages\/nodes-base"/);
  assert.match(packageCatalog, /"components\/template-app\/template-shell\.tsx"/);
  assert.match(packageCatalog, /"components\/template-app\/automations-status\.tsx"/);
  assert.match(packageCatalog, /"components\/template-app\/automation-mission-summary\.tsx"/);
  assert.match(packageCatalog, /"pages\/index\.html"/);
  assert.match(packageCatalog, /"public\/launch-runtime\.js"/);
  assert.match(packageCatalog, /"examples\/dashboard\/src\/lib\/n8nAutomationBridge\.ts"/);
  assert.match(packageCatalog, /"examples\/dashboard\/src\/components\/AutomationWorkflowPanel\.tsx"/);
  assert.match(packageCatalog, /"G:\/Dx\/\.dx\/receipts\/automations\/launch-release-notification\.json"/);
  assert.match(packageCatalog, /"G:\/Dx\/\.dx\/receipts\/automations\/run-latest\.json"/);
  assert.match(packageCatalog, /data-dx-automation-credential-schema/);
  assert.match(packageCatalog, /data-dx-automation-required-env/);
  assert.match(packageCatalog, /data-dx-automation-receipt-intent/);
  assert.match(packageCatalog, /data-dx-automation-run-receipt-intent/);
  assert.match(packageCatalog, /data-dx-automation-workflow-node-readiness/);
  assert.match(packageCatalog, /data-dx-component="launch-automation-connector-workflow"/);
  assert.match(packageCatalog, /data-dx-component="launch-automation-catalog-summary"/);
  assert.match(packageCatalog, /data-dx-automation-workflow="connector-readiness"/);
  assert.match(packageCatalog, /data-dx-component="launch-automation-mission-summary"/);
  assert.match(packageCatalog, /data-dx-automation-dashboard-state/);

  assert.ok(
    fs.existsSync(launchAutomationReceiptPath),
    "launch automation dashboard receipt seed should exist under G:/Dx/.dx/receipts/automations",
  );
  const receipt = JSON.parse(fs.readFileSync(launchAutomationReceiptPath, "utf8"));
  assert.equal(receipt.package_id, "automations/n8n");
  assert.equal(receipt.dashboard_workflow, "automation-release-receipt");
  assert.equal(receipt.redacted, true);
  assert.equal(receipt.runtime_execution, false);
  assert.equal(receipt.receipt_path, "G:/Dx/.dx/receipts/automations/launch-release-notification.json");
  assert.match(receipt.source_selector, /launch-automation-dashboard-workflow/);
  assert.deepEqual(receipt.secret_values, []);
  assert.ok(
    receipt.zed_preview_markers.includes("data-dx-automation-credential-schema"),
    "receipt seed should advertise credential schema marker",
  );
  assert.ok(
    receipt.zed_preview_markers.includes("data-dx-automation-required-env"),
    "receipt seed should advertise required env marker",
  );
  assert.ok(
    receipt.zed_preview_markers.includes("data-dx-automation-workflow-node-readiness"),
    "receipt seed should advertise workflow node readiness marker",
  );
  assert.ok(
    receipt.zed_preview_markers.includes('data-dx-component="launch-automation-mission-summary"'),
    "receipt seed should advertise dashboard mission summary marker",
  );
  assert.doesNotMatch(
    JSON.stringify(receipt),
    /(xox[baprs]-|sk_live|sk_test|ghp_|api[_-]?key":\s*"[^"]|token":\s*"[^"]|secret":\s*"[^"])/i,
  );
  assert.ok(
    fs.existsSync(zedRunReceiptPath),
    "Zed run handoff receipt seed should exist under G:/Dx/.dx/receipts/automations",
  );
  const zedReceipt = JSON.parse(fs.readFileSync(zedRunReceiptPath, "utf8"));
  assert.equal(zedReceipt.schema, "dx.automations.zed.run_receipt");
  assert.equal(zedReceipt.package_id, "automations/n8n");
  assert.equal(zedReceipt.dashboard_workflow, "automation-release-receipt");
  assert.equal(zedReceipt.runtime_execution, false);
  assert.deepEqual(zedReceipt.secret_values, []);
  assert.ok(
    zedReceipt.zed_preview_markers.includes("data-dx-automation-dashboard-state"),
    "Zed run receipt should advertise dashboard state marker",
  );
  assert.deepEqual(zedReceipt.required_env, [
    "SLACK_BOT_TOKEN",
    "SLACK_SIGNING_SECRET",
    "NOTION_API_KEY",
    "DX_AUTOMATIONS_OPERATOR_APPROVAL",
  ]);
  assert.doesNotMatch(
    JSON.stringify(zedReceipt),
    /(xox[baprs]-|sk_live|sk_test|ghp_|api[_-]?key":\s*"[^"]|token":\s*"[^"]|secret":\s*"[^"])/i,
  );

  assert.match(routeContract, /"components\/template-app\/automations-status\.tsx"/);
  assert.match(routeContract, /"components\/template-app\/automation-mission-summary\.tsx"/);
  assert.match(
    routeContract,
    /"components\/template-app\/automations\/automations-metadata\.ts"/,
  );
  assert.match(cli, /NEXT_FAMILIAR_AUTOMATIONS_STATUS_TSX/);
  assert.match(cli, /NEXT_FAMILIAR_AUTOMATIONS_METADATA_TS/);
  assert.match(cli, /"components\/template-app\/template-shell\.tsx"/);
  assert.match(cli, /"components\/template-app\/automations-status\.tsx"/);
  assert.match(cli, /"components\/template-app\/automation-mission-summary\.tsx"/);
  assert.match(cli, /"public\/launch-runtime\.js"/);
  assert.match(cli, /"examples\/dashboard\/src\/lib\/n8nAutomationBridge\.ts"/);
  assert.match(cli, /"examples\/dashboard\/src\/components\/AutomationWorkflowPanel\.tsx"/);
  assert.match(cli, /"docs\/packages\/automations-n8n\.md"/);
  assert.match(cli, /"dashboard_usage": "\/launch imports LaunchAutomationBridgeStatus/);
  assert.match(cli, /credential schema and workflow-node readiness/);
  assert.match(cli, /"G:\/Dx\/\.dx\/receipts\/automations\/run-latest\.json"/);
  assert.match(
    cli,
    /"components\/template-app\/automations\/automations-metadata\.ts"/,
  );
});

test("automations/n8n materializes into live /launch runtime page", () => {
  const materializer = path.join(
    root,
    "tools",
    "launch",
    "materialize-www-template.ts",
  );
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-automation-launch-"));
  fs.mkdirSync(path.join(dir, "app", "launch"), { recursive: true });
  fs.writeFileSync(
    path.join(dir, "app", "launch", "page.tsx"),
    "export default function Page(){ return <>{children}</>; }\n",
  );

  const output = execFileSync(process.execPath, [materializer, dir], {
    cwd: root,
    encoding: "utf8",
  });
  const result = JSON.parse(output);
  const launch = fs.readFileSync(path.join(dir, "pages", "index.html"), "utf8");
  const runtime = fs.readFileSync(path.join(dir, "public", "launch-runtime.js"), "utf8");

  assert.equal(result.ok, true);
  assert.equal(result.noNodeModules, true);
  assert.match(launch, /data-dx-route="\/"/);
  assert.match(launch, /data-dx-package="automations\/n8n"/);
  assert.match(launch, /data-dx-component="launch-automation-mission-summary"/);
  assert.match(launch, /data-dx-automation-dashboard-state="idle"/);
  assert.match(launch, /id="mission-automation-schema"/);
  assert.match(launch, /data-dx-component="launch-automation-connector-workflow"/);
  assert.match(launch, /data-dx-component="launch-automation-catalog-summary"/);
  assert.match(launch, /data-dx-component="launch-automation-dashboard-workflow"/);
  assert.match(launch, /data-dx-dashboard-workflow="automation-release-receipt"/);
  assert.match(launch, /data-dx-automation-intent-input="release-notification"/);
  assert.match(launch, /data-dx-automation-interaction="connector-filter"/);
  assert.match(launch, /data-dx-automation-filter="metadata-ready"/);
  assert.match(launch, /data-dx-automation-filter="missing-config"/);
  assert.match(launch, /data-dx-automation-interaction="connector-picker"/);
  assert.match(launch, /data-dx-automation-interaction="workflow-readiness"/);
  assert.match(launch, /id="automation-schema-summary"/);
  assert.match(launch, /data-dx-automation-credential-schema="n8n-nodes-base\.manualTrigger"/);
  assert.match(launch, /data-dx-automation-required-env="DX_AUTOMATIONS_OPERATOR_APPROVAL"/);
  assert.match(launch, /data-dx-automation-workflow-node-readiness="metadata-ready"/);
  assert.match(launch, /data-dx-automation-credential-types="slackApi,slackOAuth2Api"/);
  assert.match(launch, /data-dx-automation-safe-action="prepare-dry-run-receipt"/);
  assert.match(launch, /data-dx-automation-safe-action-state="idle"/);
  assert.match(launch, /data-dx-automation-receipt-output="local-draft"/);
  assert.match(launch, /id="automation-zed-run-handoff"/);
  assert.match(launch, /data-dx-automation-handoff="zed-run-receipt"/);
  assert.match(launch, /data-dx-automation-safe-action="prepare-zed-run-handoff"/);
  assert.match(launch, /data-dx-automation-run-receipt-output="zed-handoff"/);
  assert.match(runtime, /function bindAutomations\(\)/);
  assert.match(runtime, /launch-automation-mission-summary/);
  assert.match(runtime, /dxAutomationDashboardState/);
  assert.match(runtime, /mission-automation-schema/);
  assert.match(runtime, /applyFilter/);
  assert.match(runtime, /automation-schema-summary/);
  assert.match(runtime, /dxAutomationRequiredEnv = requiredEnv/);
  assert.match(runtime, /dxAutomationWorkflowNodeReadiness = workflowMode/);
  assert.match(runtime, /automation-receipt-output/);
  assert.match(runtime, /automation-zed-run-handoff/);
  assert.match(runtime, /dxAutomationZedRunState = "created"/);
  assert.match(runtime, /receipt\.dataset\.dxAutomationSafeActionState = "created"/);
  assert.match(runtime, /dxAutomationDraftState = "created"/);
});
