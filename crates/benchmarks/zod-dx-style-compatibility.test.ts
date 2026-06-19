const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const upstreamRoot = path.resolve(root, "..", "..", "WWW/inspirations/zod");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readUpstream(relativePath) {
  return fs.readFileSync(path.join(upstreamRoot, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function assertIncludesAll(values, expectedValues) {
  for (const expectedValue of expectedValues) {
    assert.ok(
      values.includes(expectedValue),
      `expected ${JSON.stringify(values)} to include ${expectedValue}`,
    );
  }
}

function assertDxStyleCompatibility(value) {
  assert.equal(value.schema, "dx.forge.package.dx_style_compatibility");
  assert.equal(value.status, "present");
  assert.equal(value.token_source ?? value.tokenSource, "styles/globals.css");
  assert.equal(value.generated_css ?? value.generatedCss, "styles/globals.css");
  assert.equal(value.receipt_path ?? value.receiptPath, "examples/template/.dx/forge/receipts/2026-05-22-validation-zod-dashboard-settings.json");
  assert.equal(value.runtime_proof ?? value.runtimeProof, false);
  assertIncludesAll(value.visible_surfaces ?? value.visibleSurfaces, [
    "launch-settings-validation-summary",
    "zod-dashboard-settings-form",
    "dashboard-zod-settings-validator",
  ]);
  assertIncludesAll(value.source_files ?? value.sourceFiles, [
    "examples/template/zod-dashboard-settings.tsx",
    "examples/dashboard/src/components/ZodSettingsValidator.tsx",
    "tools/launch/runtime-template/pages/index.html",
    "styles/globals.css",
    "styles/globals.css",
  ]);
  assert.match(
    (value.runtime_limitations ?? value.runtimeLimitations).join("\n"),
    /SOURCE-ONLY/,
  );
  assert.match(
    (value.runtime_limitations ?? value.runtimeLimitations).join("\n"),
    /No browser style proof was run/,
  );
}

test("Validation & Schemas exposes dx-style compatibility for Zod dashboard surfaces", () => {
  const upstreamPackage = readUpstream("packages/zod/package.json");
  const upstreamSchemas = readUpstream("packages/zod/src/v4/classic/schemas.ts");
  const upstreamParse = readUpstream("packages/zod/src/v4/classic/parse.ts");
  const upstreamErrors = readUpstream("packages/zod/src/v4/core/errors.ts");

  assert.match(upstreamPackage, /"name": "zod"/);
  assert.match(upstreamPackage, /"version": "4\.4\.3"/);
  assert.match(upstreamSchemas, /strictObject/);
  assert.match(upstreamParse, /safeParse/);
  assert.match(upstreamErrors, /flattenError/);

  const launchWorkflow = read("examples/template/zod-dashboard-settings.tsx");
  const starterWorkflow = read(
    "examples/dashboard/src/components/ZodSettingsValidator.tsx",
  );
  const runtimeLaunch = read("tools/launch/runtime-template/pages/index.html");
  const packageCatalog = read("examples/template/package-catalog.ts");
  const receipt = readJson(
    "examples/template/.dx/forge/receipts/2026-05-22-validation-zod-dashboard-settings.json",
  );
  const packageStatus = readJson(
    "examples/template/.dx/forge/package-status.json",
  );
  const readModel = read(
    "examples/template/forge-package-status-read-model.ts",
  );
  const checker = read(
    "core/src/ecosystem/project_check/validation_schemas_dx_check.rs",
  );
  const docs = read("docs/packages/validation-zod.md");

  assert.match(launchWorkflow, /data-dx-style-surface="validation-schemas"/);
  assert.match(launchWorkflow, /bg-card/);
  assert.match(launchWorkflow, /text-card-foreground/);
  assert.doesNotMatch(launchWorkflow, /style=\{\{/);
  assert.doesNotMatch(launchWorkflow, /#[0-9a-fA-F]{3,8}/);
  assert.match(starterWorkflow, /data-dx-style-surface="validation-schemas"/);
  assert.match(runtimeLaunch, /data-dx-check-package-lane-dx-style-status="present"/);
  assert.match(runtimeLaunch, /data-dx-style-surface="validation-schemas"/);
  assert.match(runtimeLaunch, /data-dx-token-scope="validation\/zod"/);

  assert.match(packageCatalog, /dxStyleCompatibility:\s*\{/);
  assert.match(packageCatalog, /data-dx-style-surface="validation-schemas"/);

  assertDxStyleCompatibility(receipt.dx_style_compatibility);
  assertIncludesAll(receipt.dx_style_compatibility.data_dx_markers, [
    'data-dx-style-surface="validation-schemas"',
    'data-dx-token-scope="validation/zod"',
  ]);
  assertIncludesAll(receipt.stable_markers, [
    'data-dx-style-surface="validation-schemas"',
    'data-dx-token-scope="validation/zod"',
  ]);

  const packageLane = packageStatus.package_lane_visibility.find(
    (entry) => entry.package_id === "validation/zod",
  );
  assert.ok(packageLane, "missing Validation & Schemas package-status row");
  assertDxStyleCompatibility(packageLane.dx_style_compatibility);
  assertIncludesAll(packageLane.dx_check_metrics, [
    "validation_schemas_dx_style_compatibility_present",
    "validation_schemas_dx_style_compatibility_missing",
  ]);
  assertIncludesAll(packageStatus.dx_check_metrics, [
    "validation_schemas_dx_style_compatibility_present",
    "validation_schemas_dx_style_compatibility_missing",
  ]);

  const dashboardSurface = packageLane.selected_surfaces.find(
    (surface) => surface.surface_id === "dashboard-settings-validation",
  );
  const starterSurface = packageLane.selected_surfaces.find(
    (surface) => surface.surface_id === "starter-dashboard-settings-validator",
  );
  assert.ok(dashboardSurface, "missing dashboard settings surface");
  assert.ok(starterSurface, "missing starter dashboard validator surface");
  assertIncludesAll(dashboardSurface.source_markers, [
    'data-dx-style-surface="validation-schemas"',
  ]);
  assertIncludesAll(starterSurface.source_markers, [
    'data-dx-style-surface="validation-schemas"',
  ]);

  assert.match(readModel, /dxStyleCompatibility:\s*\{/);
  assert.match(readModel, /validation_schemas_dx_style_compatibility_present/);
  assert.match(readModel, /validation_schemas_dx_style_compatibility_missing/);
  assert.match(readModel, /data-dx-style-surface="validation-schemas"/);
  assert.match(readModel, /data-dx-token-scope="validation\/zod"/);

  assert.match(checker, /validation_schemas_dx_style_compatibility_present/);
  assert.match(checker, /validation_schemas_dx_style_compatibility_missing/);
  assert.match(checker, /validation-schemas-missing-dx-style-compatibility/);
  assert.match(checker, /fn dx_style_compatibility_is_present/);
  assert.match(
    checker,
    /fn validation_schemas_dx_style_missing_metric_and_finding_flip/,
  );

  assert.match(docs, /## DX-Style Compatibility/);
  assert.match(docs, /dx\.forge\.package\.dx_style_compatibility/);
  assert.match(docs, /data-dx-style-surface="validation-schemas"/);
  assert.match(docs, /validation_schemas_dx_style_compatibility_present/);
  assert.match(docs, /validation-schemas-missing-dx-style-compatibility/);
});
