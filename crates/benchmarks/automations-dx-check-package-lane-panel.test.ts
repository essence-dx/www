const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const upstream = path.resolve(
  root,
  "..",
  "..",
  "WWW/inspirations/n8n/packages/nodes-base",
);
const receiptPath =
  "examples/template/.dx/forge/receipts/2026-05-22-automation-connectors-launch-workflow.json";

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readUpstream(relativePath) {
  return fs.readFileSync(path.join(upstream, relativePath), "utf8");
}

test("Automation Connectors dx-check package-lane row is Studio and runtime discoverable", () => {
  const upstreamPackage = JSON.parse(readUpstream("package.json"));
  const manualTrigger = readUpstream("nodes/ManualTrigger/ManualTrigger.node.ts");
  const slackNode = readUpstream("nodes/Slack/Slack.node.ts");
  const slackV2 = readUpstream("nodes/Slack/V2/SlackV2.node.ts");
  const launchShell = read("examples/template/template-shell.tsx");
  const runtimeLaunch = read("tools/launch/runtime-template/pages/index.html");
  const editContract = read("examples/template/dx-studio-edit-contract.ts");
  const materializer = read("tools/launch/materialize-www-template.ts");
  const studioManifest = read("dx-www/src/cli/studio_manifest.rs");
  const packageStatusReadModel = read(
    "examples/template/forge-package-status-read-model.ts",
  );
  const packageStatus = read("examples/template/.dx/forge/package-status.json");
  const automationDxCheck = read(
    "core/src/ecosystem/project_check/automation_connectors_dx_check.rs",
  );
  const dxCheckReceipt = read("core/src/ecosystem/dx_check_receipt.rs");
  const packageDocs = read("docs/packages/automations-n8n.md");
  const sourceGuardRunbook = read(
    "docs/packages/automation-connectors.source-guard-runbook.json",
  );

  assert.equal(upstreamPackage.name, "n8n-nodes-base");
  assert.equal(upstreamPackage.version, "2.22.0");
  assert.match(manualTrigger, /export class ManualTrigger implements INodeType/);
  assert.match(manualTrigger, /description: INodeTypeDescription/);
  assert.match(slackNode, /IVersionedNodeType/);
  assert.match(slackNode, /description: 'Consume Slack API'/);
  assert.match(slackV2, /async execute\(this: IExecuteFunctions\)/);

  assert.match(launchShell, /data-dx-check-package-lane-row=\{packageLane\.package_id\}/);
  assert.match(
    launchShell,
    /data-dx-check-package-lane-dx-style-status=\{dxStyleCompatibilityStatus\(packageLane\)\}/,
  );

  assert.match(runtimeLaunch, /data-dx-check-package-lane-row="automations\/n8n"/);
  assert.match(runtimeLaunch, /data-dx-check-package-lane-name="Automation Connectors"/);
  assert.match(runtimeLaunch, /data-dx-check-package-lane-status="missing"/);
  assert.match(
    runtimeLaunch,
    /data-dx-check-package-lane-receipt-status="missing-receipt"/,
  );
  assert.match(
    runtimeLaunch,
    /data-dx-check-package-lane-upstream-package="n8n-nodes-base"/,
  );
  assert.match(
    runtimeLaunch,
    /data-dx-check-package-lane-source-mirror="G:\/WWW\/inspirations\/n8n\/packages\/nodes-base"/,
  );
  assert.match(
    runtimeLaunch,
    /data-dx-check-package-lane-receipt-path="examples\/template\/\.dx\/forge\/receipts\/2026-05-22-automation-connectors-launch-workflow\.json"/,
  );
  assert.match(
    runtimeLaunch,
    /data-dx-check-package-lane-dx-style-status="present"/,
  );
  assert.match(runtimeLaunch, /data-dx-style-surface="automation-connectors"/);
  assert.match(runtimeLaunch, /data-dx-token-scope="automations\/n8n"/);

  assert.match(
    editContract,
    /id: "dx-check-health-panel"[\s\S]*packageIds: \[[\s\S]*"automations\/n8n"/,
  );
  assert.match(
    materializer,
    /"launch-runtime-dx-check-panel"[\s\S]*\[[\s\S]*"automations\/n8n"/,
  );
  assert.match(
    materializer,
    /const AUTOMATION_CONNECTORS_SOURCE_GUARD_RUNBOOK_FIXTURE = \{[\s\S]*packageId: "automations\/n8n"[\s\S]*officialPackageName: "Automation Connectors"[\s\S]*fixture: "docs\/packages\/automation-connectors\.source-guard-runbook\.json"[\s\S]*guardId: "automation-connectors-package-lane-panel"[\s\S]*honestyLabel: "ADAPTER-BOUNDARY"[\s\S]*runtimeProof: false[\s\S]*zedVisibility: "automation-connectors:receipt-hash-refresh"/,
  );
  assert.match(
    materializer,
    /sourceGuardRunbookFixtures: \[[\s\S]*AUTOMATION_CONNECTORS_SOURCE_GUARD_RUNBOOK_FIXTURE[\s\S]*\]/,
  );
  assert.match(
    materializer,
    /sourceGuardRunbookFixtures: \[[\s\S]*AUTOMATION_CONNECTORS_SOURCE_GUARD_RUNBOOK_FIXTURE\.fixture[\s\S]*\]/,
  );
  assert.match(
    studioManifest,
    /"dx-check-health-panel"[\s\S]*&\[[\s\S]*"automations\/n8n"/,
  );
  assert.match(
    studioManifest,
    /studio_source_guard_with_fixture\([\s\S]*"automation-connectors-package-lane-panel"[\s\S]*"benchmarks\/automations-dx-check-package-lane-panel\.test\.ts"[\s\S]*"docs\/packages\/automation-connectors\.source-guard-runbook\.json"/,
  );
  assert.match(
    studioManifest,
    /"source_guard_id": "automation-connectors-package-lane-panel"[\s\S]*"package_id": "automations\/n8n"[\s\S]*"fixture_path": "docs\/packages\/automation-connectors\.source-guard-runbook\.json"/,
  );
  assert.match(
    studioManifest,
    /source_guard_contract_with_fixture\([\s\S]*"automation-connectors-package-lane-panel"[\s\S]*"benchmarks\/automations-dx-check-package-lane-panel\.test\.ts"[\s\S]*"docs\/packages\/automation-connectors\.source-guard-runbook\.json"/,
  );
  assert.match(
    studioManifest,
    /source_guard_command_with_fixture\([\s\S]*automations-dx-check-package-lane-panel\.test\.ts[\s\S]*"docs\/packages\/automation-connectors\.source-guard-runbook\.json"/,
  );
  assert.match(
    studioManifest,
    /"package": "automations\/n8n"[\s\S]*"front_facing_name": "Automation Connectors"[\s\S]*"data-dx-check-package-lane-row"/,
  );
  assert.match(studioManifest, /"data-dx-check-package-lane-dx-style-status"/);
  assert.match(studioManifest, /"data-dx-style-surface=\\\"automation-connectors\\\""/);

  for (const metric of [
    "automation_connectors_dx_style_compatibility_present",
    "automation_connectors_dx_style_compatibility_missing",
    "automation_connectors_upstream_runtime_boundary_present",
    "automation_connectors_upstream_runtime_boundary_missing",
    "automation_connectors_receipt_hash_refresh_current",
    "automation_connectors_receipt_hash_refresh_stale",
    "automation_connectors_receipt_hash_refresh_missing",
  ]) {
    assert.match(packageStatusReadModel, new RegExp(metric));
    assert.match(packageStatus, new RegExp(metric));
    assert.match(automationDxCheck, new RegExp(metric));
  }
  assert.match(automationDxCheck, /has_upstream_runtime_boundary/);
  assert.match(
    automationDxCheck,
    /packages\/nodes-base\/nodes\/Notion\/Notion\.node\.ts/,
  );
  assert.match(
    automationDxCheck,
    /automation-connectors-missing-upstream-runtime-boundary/,
  );
  assert.match(automationDxCheck, /receipt_hash_refresh_counts/);
  assert.match(automationDxCheck, /stale_mirror_files/);
  assert.match(automationDxCheck, /missing_mirror_files/);

  assert.match(dxCheckReceipt, /fn automation_connectors_package_lane_row/);
  assert.match(
    dxCheckReceipt,
    /rows\.extend\(automation_connectors_package_lane_row\(root, package_status\)\)/,
  );
  assert.match(dxCheckReceipt, /AUTOMATION_CONNECTORS_METRICS/);
  assert.match(dxCheckReceipt, /automation_connectors_upstream_runtime_boundary_present/);
  assert.match(dxCheckReceipt, /automation_connectors_has_upstream_runtime_boundary/);
  assert.match(dxCheckReceipt, /studio_manifest_source/);
  assert.match(dxCheckReceipt, /lower_dx_check_source/);
  assert.match(dxCheckReceipt, /check_panel_source/);
  assert.match(
    dxCheckReceipt,
    /fn dx_check_latest_panel_exposes_automation_connectors_package_lane_hash_refresh_row/,
  );
  assert.match(packageStatusReadModel, /checkPanelSource:\s*"core\/src\/ecosystem\/dx_check_receipt\.rs"/);
  assert.match(packageStatus, /"check_panel_source": "core\/src\/ecosystem\/dx_check_receipt\.rs"/);
  assert.match(packageStatusReadModel, /automation-connectors-check-panel-source/);
  assert.match(packageStatus, /automation-connectors-check-panel-source/);
  assert.match(sourceGuardRunbook, /"check_panel_source": "core\/src\/ecosystem\/dx_check_receipt\.rs"/);
  assert.match(sourceGuardRunbook, /automation-connectors-check-panel-source/);

  assert.match(packageDocs, /DX Studio\/check-panel Automation Connectors package row/);
  assert.match(packageDocs, /automation-connectors-check-panel-source/);
  assert.match(packageDocs, /automation_connectors_upstream_runtime_boundary_present/);
  assert.match(packageDocs, /automation_connectors_receipt_hash_refresh_current/);
  assert.match(packageDocs, /data-dx-check-package-lane-row="automations\/n8n"/);
  assert.match(packageDocs, /data-dx-check-package-lane-dx-style-status="present"/);
  assert.match(packageDocs, /generated `public\/preview-manifest\.json`/);
  assert.match(packageDocs, /source_guard_runbook_index/);
  assert.match(packageDocs, /without claiming live n8n runtime proof/);
  assert.match(packageDocs, new RegExp(receiptPath.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
});
