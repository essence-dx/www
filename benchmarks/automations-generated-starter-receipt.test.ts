const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const upstreamRoot = "G:/WWW/inspirations/n8n/packages/nodes-base";
const receiptFile = "2026-05-22-automation-connectors-launch-workflow.json";
const sourceReceiptPath = `examples/template/.dx/forge/receipts/${receiptFile}`;
const generatedReceiptPath = `.dx/forge/receipts/${receiptFile}`;

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readUpstream(relativePath) {
  return fs.readFileSync(path.join(upstreamRoot, relativePath), "utf8");
}

test("Automation Connectors receipt is materialized into generated launch starters", () => {
  const upstreamPackage = JSON.parse(readUpstream("package.json"));
  const manualTrigger = readUpstream("nodes/ManualTrigger/ManualTrigger.node.ts");
  const slackNode = readUpstream("nodes/Slack/Slack.node.ts");
  const slackV2 = readUpstream("nodes/Slack/V2/SlackV2.node.ts");
  const webhookNode = readUpstream("nodes/Webhook/Webhook.node.ts");

  const routeContract = read("examples/template/template-route-contract.ts");
  const cli = read("dx-www/src/cli/mod.rs");
  const packageDoc = read("docs/packages/automations-n8n.md");
  const sourceReceipt = JSON.parse(read(sourceReceiptPath));

  assert.equal(upstreamPackage.name, "n8n-nodes-base");
  assert.equal(upstreamPackage.version, "2.22.0");
  assert.match(manualTrigger, /export class ManualTrigger implements INodeType/);
  assert.match(manualTrigger, /async trigger\(this: ITriggerFunctions\)/);
  assert.match(slackNode, /export class Slack extends VersionedNodeType/);
  assert.match(slackNode, /name: 'slack'/);
  assert.match(slackV2, /async execute\(this: IExecuteFunctions\)/);
  assert.match(webhookNode, /export class Webhook extends Node/);
  assert.match(webhookNode, /async webhook\(context: IWebhookFunctions\)/);

  assert.equal(sourceReceipt.package_id, "automations/n8n");
  assert.equal(sourceReceipt.official_package_name, "Automation Connectors");
  assert.equal(sourceReceipt.no_runtime_execution, true);
  assert.ok(
    sourceReceipt.upstream_public_apis.includes("ITriggerFunctions"),
    "Automation Connectors receipt should cite the Manual Trigger trigger boundary",
  );
  assert.ok(
    sourceReceipt.upstream_public_apis.includes("IExecuteFunctions"),
    "Automation Connectors receipt should cite the Slack V2 execute boundary",
  );
  assert.ok(
    sourceReceipt.upstream_public_apis.includes("IWebhookFunctions"),
    "Automation Connectors receipt should cite the Webhook runtime boundary",
  );
  assert.ok(
    sourceReceipt.inspected_upstream_files.includes(
      "packages/nodes-base/nodes/Slack/V2/SlackV2.node.ts",
    ),
    "Automation Connectors receipt should cite inspected Slack V2 source",
  );
  assert.ok(
    sourceReceipt.inspected_upstream_files.includes(
      "packages/nodes-base/nodes/Webhook/Webhook.node.ts",
    ),
    "Automation Connectors receipt should cite inspected Webhook source",
  );

  assert.match(routeContract, new RegExp(`"${generatedReceiptPath}"`));
  assert.match(routeContract, /automationConnectorsLaunchWorkflow:/);
  assert.match(routeContract, /officialPackageName: "Automation Connectors"/);
  assert.match(routeContract, /packageId: "automations\/n8n"/);
  assert.match(
    routeContract,
    new RegExp(`materializedReceiptFile:\\s*"${generatedReceiptPath}"`),
  );
  assert.match(routeContract, /data-dx-check-package-lane-row/);
  assert.match(routeContract, /data-dx-automation-run-receipt-path/);

  assert.match(cli, /NEXT_FAMILIAR_AUTOMATION_CONNECTORS_LAUNCH_RECEIPT_JSON/);
  assert.match(
    cli,
    new RegExp(sourceReceiptPath.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
  );
  assert.match(cli, new RegExp(`"${generatedReceiptPath}"`));
  assert.match(
    cli,
    new RegExp(
      `"${generatedReceiptPath.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}"[\\s\\S]*NEXT_FAMILIAR_AUTOMATION_CONNECTORS_LAUNCH_RECEIPT_JSON`,
    ),
  );

  assert.match(packageDoc, /generated-starter Automation Connectors receipt/);
  assert.match(
    packageDoc,
    new RegExp(generatedReceiptPath.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
  );
  assert.match(packageDoc, /SOURCE-ONLY for generated starter materialization/);
});
