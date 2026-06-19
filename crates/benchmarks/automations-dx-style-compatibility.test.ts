const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const receiptPath =
  "examples/template/.dx/forge/receipts/2026-05-22-automation-connectors-launch-workflow.json";

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

test("Automation Connectors publishes source-owned dx-style compatibility evidence", () => {
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const receipt = readJson(receiptPath);
  const launchShell = read("examples/template/template-shell.tsx");
  const launchWorkflow = read("examples/template/automations-status.tsx");
  const missionSummary = read("examples/template/automation-mission-summary.tsx");
  const readModel = read("examples/template/forge-package-status-read-model.ts");
  const packageCatalog = read("examples/template/package-catalog.ts");
  const rustDxCheck = read(
    "core/src/ecosystem/project_check/automation_connectors_dx_check.rs",
  );
  const packageDoc = read("docs/packages/automations-n8n.md");

  const visibility = status.package_lane_visibility.find(
    (entry) => entry.package_id === "automations/n8n",
  );
  assert.ok(visibility, "Automation Connectors package-status row is missing");

  for (const source of [launchShell, launchWorkflow, missionSummary]) {
    assert.match(source, /data-dx-style-surface="automation-connectors"/);
    assert.match(source, /data-dx-token-scope="automations\/n8n"/);
  }

  const compatibility = visibility.dx_style_compatibility;
  assert.ok(compatibility, "Automation Connectors dx-style compatibility is missing");
  assert.equal(
    compatibility.schema,
    "dx.forge.package.dx_style_compatibility",
  );
  assert.equal(compatibility.status, "present");
  assert.equal(compatibility.token_source, "examples/template/automations-status.tsx");
  assert.equal(
    compatibility.generated_css,
    "examples/template/styles/globals.css",
  );
  assert.equal(compatibility.receipt_path, receiptPath);
  assert.equal(compatibility.runtime_proof, false);
  assert.deepEqual(compatibility.visible_surfaces, [
    "launch-automation-dashboard-workflow",
    "launch-automation-connector-workflow",
    "launch-automation-mission-summary",
  ]);
  assert.ok(
    compatibility.source_files.includes("examples/template/template-shell.tsx"),
  );
  assert.ok(
    compatibility.source_files.includes(
      "examples/template/automations-status.tsx",
    ),
  );
  assert.ok(
    compatibility.source_files.includes(
      "examples/template/automation-mission-summary.tsx",
    ),
  );
  assert.ok(
    compatibility.source_markers.includes(
      'data-dx-style-surface="automation-connectors"',
    ),
  );
  assert.ok(
    compatibility.source_markers.includes('data-dx-token-scope="automations/n8n"'),
  );
  assert.ok(
    compatibility.runtime_limitations.some((limitation) =>
      /no browser visual proof/i.test(limitation),
    ),
  );

  const dashboardSurface = visibility.selected_surfaces.find(
    (surface) => surface.surface_id === "automation-launch-dashboard-workflow",
  );
  assert.ok(dashboardSurface, "Automation Connectors dashboard surface missing");
  assert.ok(
    dashboardSurface.source_markers.includes(
      'data-dx-style-surface="automation-connectors"',
    ),
  );
  assert.ok(
    dashboardSurface.source_markers.includes('data-dx-token-scope="automations/n8n"'),
  );

  for (const metric of [
    "automation_connectors_dx_style_compatibility_present",
    "automation_connectors_dx_style_compatibility_missing",
  ]) {
    assert.ok(visibility.dx_check_metrics.includes(metric), `${metric} missing from row`);
    assert.ok(status.dx_check_metrics.includes(metric), `${metric} missing from top-level status`);
    assert.ok(
      receipt.dx_check_visibility.dx_check_metrics.includes(metric),
      `${metric} missing from receipt visibility`,
    );
    assert.match(readModel, new RegExp(metric));
    assert.match(rustDxCheck, new RegExp(metric));
  }

  assert.deepEqual(receipt.dx_style_compatibility, compatibility);
  assert.match(readModel, /dxStyleCompatibility:\s*\{/);
  assert.match(readModel, /data-dx-style-surface="automation-connectors"/);
  assert.match(packageCatalog, /dxStyleCompatibility:\s*\{/);
  assert.match(packageCatalog, /data-dx-style-surface="automation-connectors"/);
  assert.match(
    rustDxCheck,
    /automation-connectors-missing-dx-style-compatibility/,
  );
  assert.match(packageDoc, /dx-style compatibility/i);
  assert.match(packageDoc, /data-dx-style-surface="automation-connectors"/);
});
