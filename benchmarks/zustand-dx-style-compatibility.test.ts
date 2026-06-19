const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function requireMetric(metrics, metric) {
  assert.ok(metrics.includes(metric), `missing metric ${metric}`);
}

function assertDxStyleCompatibility(compatibility) {
  assert.equal(
    compatibility.schema,
    "dx.forge.package.dx_style_compatibility",
  );
  assert.equal(compatibility.status, "present");
  assert.equal(compatibility.token_source, "styles/globals.css");
  assert.equal(compatibility.generated_css, "styles/globals.css");
  assert.equal(compatibility.runtime_proof, false);
  assert.ok(
    compatibility.visible_surfaces.includes(
      "launch-dashboard-state-workflow",
    ),
  );
  assert.ok(
    compatibility.visible_surfaces.includes("launch-dashboard-state-shell"),
  );
  assert.ok(
    compatibility.source_files.includes(
      "examples/template/state-zustand-dashboard.tsx",
    ),
  );
  assert.ok(
    compatibility.source_files.includes(
      "examples/template/template-shell.tsx",
    ),
  );
  assert.ok(
    compatibility.runtime_limitations.some((limitation) =>
      limitation.startsWith("SOURCE-ONLY:"),
    ),
  );
}

test("State Management exposes dx-style compatibility for visible UI surfaces", () => {
  const dashboard = read("examples/template/state-zustand-dashboard.tsx");
  const launchShell = read("examples/template/template-shell.tsx");
  const runtimeLaunch = read("tools/launch/runtime-template/pages/index.html");
  const workflowReceipt = readJson(
    "examples/template/.dx/forge/receipts/2026-05-22-state-zustand-dashboard-workflow.json",
  );
  const packageReceipt = readJson(
    "examples/template/.dx/forge/receipts/packages/state-zustand.json",
  );
  const packageStatus = readJson(
    "examples/template/.dx/forge/package-status.json",
  );
  const readModel = read(
    "examples/template/forge-package-status-read-model.ts",
  );
  const checker = read(
    "core/src/ecosystem/project_check/state_management_dx_check.rs",
  );
  const docs = read("docs/packages/state-zustand.md");

  for (const source of [dashboard, launchShell, runtimeLaunch]) {
    assert.match(source, /data-dx-style-surface="state-management"/);
  }

  assertDxStyleCompatibility(workflowReceipt.dx_style_compatibility);
  assert.ok(
    workflowReceipt.dx_style_compatibility.data_dx_markers.includes(
      'data-dx-style-surface="state-management"',
    ),
  );

  const packageVisibility = packageReceipt.package.dx_check_visibility;
  assertDxStyleCompatibility(packageVisibility.dx_style_compatibility);
  requireMetric(
    packageVisibility.metrics,
    "state_management_dx_style_compatibility_present",
  );
  requireMetric(
    packageVisibility.metrics,
    "state_management_dx_style_compatibility_missing",
  );

  const packageLane = packageStatus.package_lane_visibility.find(
    (lane) => lane.package_id === "state/zustand",
  );
  assert.ok(packageLane, "missing State Management package lane row");
  assertDxStyleCompatibility(packageLane.dx_style_compatibility);
  requireMetric(
    packageLane.dx_check_metrics,
    "state_management_dx_style_compatibility_present",
  );
  requireMetric(
    packageLane.dx_check_metrics,
    "state_management_dx_style_compatibility_missing",
  );
  requireMetric(
    packageStatus.dx_check_metrics,
    "state_management_dx_style_compatibility_present",
  );
  requireMetric(
    packageStatus.dx_check_metrics,
    "state_management_dx_style_compatibility_missing",
  );

  assert.match(
    readModel,
    /dxStyleCompatibility:\s*\{\s*schema:\s*"dx\.forge\.package\.dx_style_compatibility"/,
  );
  assert.match(
    readModel,
    /"state_management_dx_style_compatibility_present"/,
  );
  assert.match(
    readModel,
    /"state_management_dx_style_compatibility_missing"/,
  );

  assert.match(checker, /state_management_dx_style_compatibility_present/);
  assert.match(checker, /state_management_dx_style_compatibility_missing/);
  assert.match(checker, /state-management-missing-dx-style-compatibility/);
  assert.match(checker, /fn dx_style_compatibility_is_present/);
  assert.match(
    checker,
    /fn state_management_dx_style_missing_metric_and_finding_flip/,
  );

  assert.match(docs, /dx\.forge\.package\.dx_style_compatibility/);
  assert.match(docs, /data-dx-style-surface="state-management"/);
  assert.match(docs, /state_management_dx_style_compatibility_present/);
  assert.match(
    docs,
    /cargo test -p dx-www-compiler state_management_dx_style_missing_metric_and_finding_flip --lib/,
  );
});
