const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const n8nMirror = "G:\\WWW\\inspirations\\n8n";

function read(relativePath) {
  const fullPath = path.join(root, relativePath);
  assert.ok(fs.existsSync(fullPath), `expected ${relativePath} to exist`);
  return fs.readFileSync(fullPath, "utf8");
}

function readMirror(relativePath) {
  const fullPath = path.join(n8nMirror, relativePath);
  assert.ok(fs.existsSync(fullPath), `expected n8n mirror file ${relativePath}`);
  return fs.readFileSync(fullPath, "utf8");
}

test("automations/n8n dashboard workflow uses real source-owned package APIs", () => {
  const slackNode = readMirror("packages/nodes-base/nodes/Slack/Slack.node.ts");
  const manualTrigger = readMirror(
    "packages/nodes-base/nodes/ManualTrigger/ManualTrigger.node.ts",
  );
  const slackCredential = readMirror(
    "packages/nodes-base/credentials/SlackApi.credentials.ts",
  );
  const bridge = read("examples/dashboard/src/lib/n8nAutomationBridge.ts");
  const panel = read("examples/dashboard/src/components/AutomationWorkflowPanel.tsx");
  const dashboard = read("examples/dashboard/src/pages/Dashboard.tsx");
  const forgeSlice = read("core/src/ecosystem/forge_n8n_automations.rs");
  const packageDoc = read("docs/packages/automations-n8n.md");

  assert.match(slackNode, /extends VersionedNodeType/);
  assert.match(slackNode, /displayName: 'Slack'/);
  assert.match(manualTrigger, /implements INodeType/);
  assert.match(manualTrigger, /outputs: \[NodeConnectionTypes\.Main\]/);
  assert.match(slackCredential, /implements ICredentialType/);
  assert.match(slackCredential, /Authorization: '=Bearer \{\{\$credentials\.accessToken\}\}'/);

  assert.match(bridge, /packageId: "automations\/n8n"/);
  assert.match(bridge, /sourceMirror: "G:\\\\WWW\\\\inspirations\\\\n8n\\\\packages\\\\nodes-base"/);
  assert.match(bridge, /aliases: \[/);
  assert.match(bridge, /"@n8n\/nodes-base"/);
  assert.match(bridge, /"n8n-nodes-base"/);
  assert.match(bridge, /exportedFiles: \[/);
  assert.match(bridge, /requiredEnv: \[/);
  assert.match(bridge, /appOwnedBoundaries: \[/);
  assert.match(bridge, /receiptPaths: \[/);
  assert.match(bridge, /INodeTypeDescription/);
  assert.match(bridge, /ICredentialType/);
  assert.match(bridge, /IAuthenticateGeneric/);
  assert.match(bridge, /createN8nDashboardWorkflow/);
  assert.match(bridge, /selectN8nConnector/);
  assert.match(bridge, /buildN8nWorkflowReadiness/);
  assert.match(bridge, /createRedactedN8nReceipt/);
  assert.match(bridge, /workflowIntent: string/);
  assert.match(bridge, /workflowIntent: trimmedIntent/);
  assert.match(bridge, /requiredEnv: \['SLACK_BOT_TOKEN', 'SLACK_SIGNING_SECRET'\]/);
  assert.match(bridge, /requiredEnv: \['NOTION_API_KEY'\]/);
  assert.match(bridge, /formatN8nCredentialBoundary/);
  assert.match(bridge, /secretFields: \[['"]accessToken['"], ['"]signatureSecret['"]\]/);

  assert.match(panel, /data-dx-package="automations\/n8n"/);
  assert.match(panel, /data-dx-component="dashboard-automation-workflow"/);
  assert.match(panel, /data-dx-automation-dashboard-workflow="connector-readiness"/);
  assert.match(panel, /data-dx-automation-action="prepare-redacted-receipt"/);
  assert.match(panel, /data-dx-automation-local-receipt="dashboard-receipt"/);
  assert.doesNotMatch(panel, /data-dx-automation-local-demo=/);
  assert.match(panel, /data-dx-automation-intent-preview=\{receipt\?\.workflowIntent \|\| intent\}/);
  assert.match(panel, /data-dx-automation-required-env=\{readiness\.missingEnv\.join\(','\)\}/);
  assert.match(panel, /data-dx-icon-search="automation:workflow"/);
  assert.match(panel, /<dx-icon name="pack:workflow"/);
  assert.match(panel, /createRedactedN8nReceipt/);
  assert.doesNotMatch(panel, /#[0-9A-Fa-f]{3,8}|rgb\(|rgba\(|hsl\(|oklch\(/);
  assert.doesNotMatch(panel, /slack\.svg|notion\.svg|n8n\.svg/i);

  assert.match(
    dashboard,
    /import \{ AutomationWorkflowPanel \} from '\.\.\/components\/AutomationWorkflowPanel';/,
  );
  assert.match(dashboard, /<AutomationWorkflowPanel \/>/);

  assert.match(forgeSlice, /nodeSourceMirror: "G:\\\\WWW\\\\inspirations\\\\n8n\\\\packages\\\\nodes-base"/);
  assert.match(forgeSlice, /dashboard\/src\/components\/AutomationWorkflowPanel\.tsx/);
  assert.match(forgeSlice, /dashboard\/src\/lib\/n8nAutomationBridge\.ts/);
  assert.match(packageDoc, /Dashboard Usage/);
  assert.match(packageDoc, /G:\\\\WWW\\\\inspirations\\\\n8n/);
});
