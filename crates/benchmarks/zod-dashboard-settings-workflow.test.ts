const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const inspirationRoot = path.resolve(root, "..", "..", "WWW", "inspirations", "zod");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readInspiration(relativePath) {
  return fs.readFileSync(path.join(inspirationRoot, relativePath), "utf8");
}

function readRequired(relativePath) {
  const file = path.join(root, relativePath);
  assert.ok(fs.existsSync(file), `${relativePath} should exist`);
  return fs.readFileSync(file, "utf8");
}

test("validation/zod exposes a dashboard settings workflow, not only a status card", () => {
  const upstreamExports = readInspiration("packages/zod/src/v4/classic/external.ts");
  const upstreamSchemas = readInspiration("packages/zod/src/v4/classic/schemas.ts");
  const forgeZod = read("core/src/ecosystem/forge_zod.rs");
  const catalog = read("examples/template/package-catalog.ts");
  const shell = read("examples/template/template-shell.tsx");
  const runtimePage = read("tools/launch/runtime-template/pages/index.html");
  const runtimeScript = read("tools/launch/runtime-template/assets/launch-runtime.ts");
  const routeContract = read("examples/template/template-route-contract.ts");
  const templateSurfaceRegistry = read("examples/template/template-surface-registry.ts");
  const studioEditContract = read("examples/template/dx-studio-edit-contract.ts");
  const liveRuntimeMaterializer = read("tools/launch/materialize-www-template.ts");
  const cli = read("dx-www/src/cli/mod.rs");
  const dashboardStarterSettings = readRequired(
    "examples/dashboard/src/pages/Settings.tsx",
  );
  const dashboardStarterComponent = readRequired(
    "examples/dashboard/src/components/ZodSettingsValidator.tsx",
  );
  const dashboardStarterPackage = readRequired(
    "examples/dashboard/src/lib/forge/validation/zod/dashboard-settings.ts",
  );
  const dashboardStarterWorkflow = readRequired(
    "examples/dashboard/src/lib/zodDashboardSettings.ts",
  );
  const packageDocs = readRequired("docs/packages/validation-zod.md");
  const dashboardSettings = readRequired(
    "examples/template/zod-dashboard-settings.tsx",
  );
  const templateFormsValidation = readRequired(
    "examples/template/lib/validation/zod/template-forms.ts",
  );
  const dashboardReceipt = readRequired(
    "examples/template/.dx/forge/receipts/2026-05-22-validation-zod-dashboard-settings.json",
  );
  const zodCatalogEntryStart = catalog.indexOf('packageId: "validation/zod"');
  const zodCatalogEntryEnd = catalog.indexOf(
    'packageId: "payments/stripe-js"',
    zodCatalogEntryStart,
  );
  assert.notEqual(zodCatalogEntryStart, -1, "Validation & Schemas catalog entry should exist");
  assert.notEqual(zodCatalogEntryEnd, -1, "Validation & Schemas catalog entry should remain scoped");
  const zodCatalogEntry = catalog.slice(zodCatalogEntryStart, zodCatalogEntryEnd);

  assert.match(upstreamExports, /flattenError/);
  assert.match(upstreamExports, /treeifyError/);
  assert.match(upstreamExports, /globalRegistry/);
  assert.match(upstreamSchemas, /safeParse\(data: unknown/);
  assert.match(upstreamSchemas, /meta\(data:/);

  assert.match(forgeZod, /ZOD_DASHBOARD_SETTINGS_TS/);
  assert.match(forgeZod, /"js\/validation\/zod\/dashboard-settings\.ts"/);
  assert.match(forgeZod, /dxDashboardSettingsSchema/);
  assert.match(forgeZod, /safeParseDxDashboardSettingsForm/);
  assert.match(forgeZod, /z\.strictObject/);
  assert.match(forgeZod, /z\.flattenError/);
  assert.match(forgeZod, /aliases: \[/);
  assert.match(forgeZod, /sourceMirror: "G:\/WWW\/inspirations\/zod"/);
  assert.match(forgeZod, /provenance:/);
  assert.match(forgeZod, /receiptPaths: \[/);
  assert.match(forgeZod, /dxLaunchPackageDxCheckStatusSchema/);
  assert.match(forgeZod, /dxCheckVisibility: \{/);
  assert.match(forgeZod, /schema: z\.literal\("dx\.forge\.package\.dx_check_visibility"\)/);
  assert.match(forgeZod, /monitoredSurfaces: z\.array\(z\.string\(\)\.min\(1\)\)\.min\(1\)\.readonly\(\)/);
  assert.match(forgeZod, /currentStatus: "present"/);
  assert.match(forgeZod, /"missing receipt"/);
  assert.match(forgeZod, /"unsupported surface"/);
  assert.match(forgeZod, /appOwnedBoundaries: \[/);
  assert.match(forgeZod, /dashboardUsage: \{/);
  assert.match(forgeZod, /examples\/dashboard\/src\/components\/ZodSettingsValidator\.tsx/);
  assert.match(forgeZod, /launchRuntimeUsage: \{/);
  assert.match(forgeZod, /tools\/launch\/runtime-template\/pages\/index\.html/);
  assert.match(forgeZod, /mission-settings-status/);
  assert.match(forgeZod, /schema: "dxDashboardSettingsSchema"/);
  assert.match(forgeZod, /publicApi: "safeParseDxDashboardSettingsForm"/);
  assert.match(forgeZod, /fieldErrorsApi: "z\.flattenError"/);
  assert.match(forgeZod, /receiptTarget: "mission-settings-receipt-json"/);
  assert.match(forgeZod, /form: "dashboard-settings"/);
  assert.match(forgeZod, /data-dx-zod-form=\\"dashboard-settings\\"/);
  assert.match(forgeZod, /data-dx-form-package=\\"forms\/react-hook-form\\"/);
  assert.match(forgeZod, /data-dx-zod-dashboard-receipt-api=\\"createDxDashboardSettingsReceipt\\"/);
  assert.match(forgeZod, /data-dx-zod-dashboard-receipt-json=\\"idle\\"/);
  assert.match(forgeZod, /dx add validation-schemas --write/);
  assert.match(forgeZod, /"lib\/validation\/zod\/dashboard-settings\.ts"/);

  assert.match(catalog, /packageId: "validation\/zod"/);
  assert.match(catalog, /officialName: "Validation & Schemas"/);
  assert.match(catalog, /upstreamPackage: "zod"/);
  assert.match(catalog, /upstreamVersion: "4\.4\.3"/);
  assert.match(catalog, /command: "dx add validation-schemas --write"/);
  assert.match(catalog, /aliases: \["zod", "zod\/v4", "schema\/zod"/);
  assert.match(catalog, /sourceMirror: "G:\/WWW\/inspirations\/zod"/);
  assert.match(catalog, /"lib\/validation\/zod\/dashboard-settings\.ts"/);
  assert.match(catalog, /"components\/template-app\/zod-dashboard-settings\.tsx"/);
  assert.match(catalog, /"tools\/launch\/runtime-template\/pages\/index\.html"/);
  assert.match(catalog, /"tools\/launch\/runtime-template\/assets\/launch-runtime\.ts"/);
  assert.match(catalog, /"examples\/dashboard\/src\/components\/ZodSettingsValidator\.tsx"/);
  assert.match(
    catalog,
    /"examples\/template\/\.dx\/forge\/receipts\/2026-05-22-validation-zod-dashboard-settings\.json"/,
  );
  assert.match(catalog, /"dxDashboardSettingsSchema"/);
  assert.match(catalog, /"safeParseDxDashboardSettingsForm"/);
  assert.match(catalog, /dashboardUsage: \{/);
  assert.match(catalog, /mission-control-receipt-json/);
  assert.match(catalog, /data-dx-zod-dashboard-receipt-api="createDxDashboardSettingsReceipt"/);
  assert.match(catalog, /"z\.flattenError"/);
  assert.match(zodCatalogEntry, /dxCheckVisibility: \{/);
  assert.match(
    zodCatalogEntry,
    /schema: "dx\.forge\.package\.dx_check_visibility"/,
  );
  assert.match(
    zodCatalogEntry,
    /receiptPath:\s*"examples\/template\/\.dx\/forge\/receipts\/2026-05-22-validation-zod-dashboard-settings\.json"/,
  );
  assert.match(zodCatalogEntry, /currentStatus: "present"/);
  assert.match(zodCatalogEntry, /"missing receipt"/);
  assert.match(zodCatalogEntry, /"unsupported surface"/);
  assert.match(zodCatalogEntry, /monitoredSurfaces: \[/);
  assert.match(zodCatalogEntry, /"dashboard-settings-validation"/);

  assert.match(dashboardSettings, /"use client";/);
  assert.match(
    dashboardSettings,
    /@\/lib\/validation\/zod\/dashboard-settings/,
  );
  assert.match(dashboardSettings, /data-dx-package="validation\/zod"/);
  assert.match(
    dashboardSettings,
    /data-dx-component="zod-dashboard-settings-form"/,
  );
  assert.match(dashboardSettings, /<dx-icon name="pack:validation-zod"/);
  assert.match(dashboardSettings, /data-dx-zod-settings-field="workspaceName"/);
  assert.match(dashboardSettings, /data-dx-zod-settings-field="contactEmail"/);
  assert.match(dashboardSettings, /data-dx-zod-settings-action="load-invalid"/);
  assert.match(dashboardSettings, /data-dx-zod-settings-action="validate"/);
  assert.match(dashboardSettings, /data-dx-zod-settings-issues/);
  assert.match(dashboardSettings, /data-dx-zod-settings-output/);
  assert.match(dashboardSettings, /safeParseDxDashboardSettingsForm/);
  assert.match(dashboardSettings, /formatDxDashboardSettingsIssues/);
  assert.doesNotMatch(dashboardSettings, /#[0-9a-fA-F]{3,8}/);
  assert.doesNotMatch(dashboardSettings, /\b(?:bg|text|border)-(?:neutral|slate|zinc|stone|gray)-/);
  assert.match(templateFormsValidation, /schema: "dx\.validation\.template_forms"/);
  assert.match(templateFormsValidation, /export const templateLoginSchema/);
  assert.match(templateFormsValidation, /export const templateWorkspaceSettingsSchema/);
  assert.match(templateFormsValidation, /export const templateProfileSchema/);
  assert.match(templateFormsValidation, /export const templateBillingContactSchema/);
  assert.match(templateFormsValidation, /export const templateFormValidationMetadata/);
  assert.match(templateFormsValidation, /export function safeParseTemplateForm/);
  assert.match(templateFormsValidation, /z\.strictObject/);
  assert.match(templateFormsValidation, /z\.flattenError/);

  assert.match(shell, /import \{ LaunchZodDashboardSettings \}/);
  assert.match(shell, /<LaunchZodDashboardSettings \/>/);
  assert.match(shell, /data-dx-section="forms-validation"/);
  assert.match(shell, /data-dx-dashboard-workflow="account-settings-validation"/);
  assert.match(shell, /data-dx-product-surface="account-settings"/);
  assert.match(routeContract, /"components\/template-app\/zod-dashboard-settings\.tsx"/);
  assert.match(templateSurfaceRegistry, /id: "settings-validation"/);
  assert.match(
    templateSurfaceRegistry,
    /componentSelector: '\[data-dx-component="zod-dashboard-settings-form"\]'/,
  );
  assert.match(
    templateSurfaceRegistry,
    /2026-05-22-validation-zod-dashboard-settings\.json/,
  );
  assert.match(templateSurfaceRegistry, /data-dx-zod-settings-field/);
  assert.match(templateSurfaceRegistry, /createDxDashboardSettingsReceipt output/);
  assert.match(studioEditContract, /id: "form-validation-proof"/);
  assert.match(
    studioEditContract,
    /data-dx-zod-settings-action="validate"/,
  );
  assert.match(studioEditContract, /data-dx-zod-settings-field/);
  assert.match(studioEditContract, /data-dx-zod-settings-submitted/);
  assert.match(
    studioEditContract,
    /2026-05-22-validation-zod-dashboard-settings\.json/,
  );
  assert.match(liveRuntimeMaterializer, /launch-runtime-settings-validation/);
  assert.match(
    liveRuntimeMaterializer,
    /data-dx-component="launch-settings-validation-summary"/,
  );
  assert.match(liveRuntimeMaterializer, /data-dx-zod-dashboard-field/);
  assert.match(liveRuntimeMaterializer, /2026-05-22-validation-zod-dashboard-settings\.json/);
  assert.match(cli, /NEXT_FAMILIAR_ZOD_DASHBOARD_SETTINGS_TSX/);
  assert.match(cli, /examples\/template\/zod-dashboard-settings\.tsx/);
  assert.match(cli, /"components\/template-app\/zod-dashboard-settings\.tsx"/);

  assert.match(
    dashboardStarterSettings,
    /import \{ ZodSettingsValidator \} from '\.\.\/components\/ZodSettingsValidator';/,
  );
  assert.match(dashboardStarterSettings, /<ZodSettingsValidator \/>/);
  assert.match(dashboardStarterPackage, /dxDashboardSettingsSchema/);
  assert.match(dashboardStarterPackage, /import \{ z \} from ['"]zod['"];/);
  assert.match(dashboardStarterPackage, /safeParseDxDashboardSettingsForm/);
  assert.match(dashboardStarterPackage, /createDxDashboardSettingsReceipt/);
  assert.match(dashboardStarterPackage, /DxDashboardSettingsIssue/);
  assert.match(dashboardStarterPackage, /z\.strictObject/);
  assert.match(dashboardStarterPackage, /z\.flattenError/);
  assert.match(dashboardStarterPackage, /path: issue\.path\.join\('\.'\) \|\| 'settings'/);
  assert.match(dashboardStarterPackage, /message: issue\.message/);
  assert.doesNotMatch(dashboardStarterPackage, /const z = \{/);
  assert.match(dashboardStarterWorkflow, /packageId: 'validation\/zod'/);
  assert.match(dashboardStarterWorkflow, /officialName: 'Validation & Schemas'/);
  assert.match(dashboardStarterWorkflow, /upstreamPackage: 'zod'/);
  assert.match(dashboardStarterWorkflow, /upstreamVersion: '4\.4\.3'/);
  assert.match(dashboardStarterWorkflow, /sourceMirror: 'G:\/WWW\/inspirations\/zod'/);
  assert.match(
    dashboardStarterWorkflow,
    /2026-05-22-validation-zod-dashboard-settings\.json/,
  );
  assert.match(dashboardStarterWorkflow, /dxCheckVisibility: \{/);
  assert.match(
    dashboardStarterWorkflow,
    /schema: 'dx\.forge\.package\.dx_check_visibility'/,
  );
  assert.match(dashboardStarterWorkflow, /currentStatus: 'present'/);
  assert.match(dashboardStarterWorkflow, /'missing receipt'/);
  assert.match(dashboardStarterWorkflow, /'unsupported surface'/);
  assert.match(dashboardStarterWorkflow, /monitoredSurfaces: \[/);
  assert.match(dashboardStarterWorkflow, /'dashboard-settings-validation'/);
  assert.match(dashboardStarterWorkflow, /dashboardUsage: \{/);
  assert.match(
    dashboardStarterWorkflow,
    /starterComponent: 'dashboard-zod-settings-validator'/,
  );
  assert.match(dashboardStarterWorkflow, /safeParseDxDashboardSettingsForm/);
  assert.match(dashboardStarterWorkflow, /createDxDashboardSettingsReceipt/);
  assert.match(dashboardStarterWorkflow, /DxDashboardSettingsIssue/);
  assert.match(dashboardStarterWorkflow, /fieldErrors:/);
  assert.match(dashboardStarterWorkflow, /settings:/);
  assert.match(dashboardStarterComponent, /data-dx-package="validation\/zod"/);
  assert.match(dashboardStarterComponent, /function hasFieldError/);
  assert.match(dashboardStarterComponent, /function fieldState/);
  assert.match(dashboardStarterComponent, /function firstFieldError/);
  assert.match(
    dashboardStarterComponent,
    /data-dx-component="dashboard-zod-settings-validator"/,
  );
  assert.match(
    dashboardStarterComponent,
    /data-dx-dashboard-workflow="settings-validation"/,
  );
  assert.match(dashboardStarterComponent, /<dx-icon name="pack:validation-zod"/);
  assert.match(
    dashboardStarterComponent,
    /data-dx-zod-dashboard-field="workspaceName"/,
  );
  assert.match(
    dashboardStarterComponent,
    /data-dx-zod-dashboard-action="load-invalid"/,
  );
  assert.match(
    dashboardStarterComponent,
    /data-dx-zod-dashboard-action="validate"/,
  );
  assert.match(dashboardStarterComponent, /aria-invalid=\{hasFieldError\('workspaceName'\)\}/);
  assert.match(dashboardStarterComponent, /data-dx-zod-dashboard-field-state=\{fieldState\('workspaceName'\)\}/);
  assert.match(dashboardStarterComponent, /data-dx-zod-dashboard-field-error=\{firstFieldError\('workspaceName'\)\}/);
  assert.match(dashboardStarterComponent, /data-dx-zod-dashboard-receipt-api="createDxDashboardSettingsReceipt"/);
  assert.match(dashboardStarterComponent, /data-dx-zod-dashboard-issues/);
  assert.match(dashboardStarterComponent, /data-dx-zod-dashboard-field-errors/);
  assert.match(dashboardStarterComponent, /data-dx-zod-dashboard-settings-json/);
  assert.match(dashboardStarterComponent, /\{issue\.path\}:\s*\{issue\.message\}/);
  assert.match(dashboardStarterComponent, /data-dx-zod-dashboard-receipt/);
  assert.match(packageDocs, /^# Validation & Schemas/m);
  assert.match(packageDocs, /Official DX package: Validation & Schemas/);
  assert.match(packageDocs, /Honesty label: SOURCE-ONLY/);
  assert.match(packageDocs, /Official CLI: `dx add validation-schemas --write`/);
  assert.match(packageDocs, /packageId: validation\/zod/);
  assert.match(packageDocs, /upstream_package: zod/);
  assert.match(packageDocs, /upstream_version: 4\.4\.3/);
  assert.match(packageDocs, /launch-settings-validation-summary/);
  assert.match(packageDocs, /data-dx-dashboard-card="settings"/);
  assert.match(packageDocs, /mission-settings-receipt-json/);
  assert.match(packageDocs, /data-dx-zod-dashboard-receipt-api="createDxDashboardSettingsReceipt"/);
  assert.match(packageDocs, /data-dx-zod-dashboard-field-state/);
  assert.match(packageDocs, /2026-05-22-validation-zod-dashboard-settings\.json/);
  assert.match(packageDocs, /data-dx-zod-form="dashboard-settings"/);
  assert.match(packageDocs, /data-dx-zod-schema="dxDashboardSettingsSchema"/);
  assert.match(packageDocs, /## dx-check Visibility/);
  assert.match(packageDocs, /dx\.forge\.package\.dx_check_visibility/);
  assert.match(packageDocs, /`present`/);
  assert.match(packageDocs, /`stale`/);
  assert.match(packageDocs, /`missing receipt`/);
  assert.match(packageDocs, /`blocked`/);
  assert.match(packageDocs, /`unsupported surface`/);
  assert.match(packageDocs, /app-owned boundaries/i);
  assert.match(runtimePage, /data-dx-component="launch-settings-validation-summary"/);
  assert.match(runtimePage, /data-dx-package="validation\/zod"/);
  assert.match(runtimePage, /data-dx-form-package="forms\/react-hook-form"/);
  assert.match(runtimePage, /data-dx-dashboard-card="settings"/);
  assert.match(runtimePage, /data-dx-dashboard-workflow="settings-validation"/);
  assert.match(runtimePage, /data-dx-product-surface="account-settings"/);
  assert.match(runtimePage, /data-dx-rhf-boundary="runtime-safe-form"/);
  assert.match(runtimePage, /data-dx-zod-schema="dxDashboardSettingsSchema"/);
  assert.match(runtimePage, /data-dx-zod-public-api="safeParseDxDashboardSettingsForm"/);
  assert.match(runtimePage, /data-dx-source-owned-api="lib\/validation\/zod\/dashboard-settings\.ts"/);
  assert.match(runtimePage, /data-dx-zod-validation-summary-target="mission-control"/);
  assert.match(runtimePage, /data-dx-zod-dashboard-controls="mission-control"/);
  assert.match(runtimePage, /data-dx-zod-dashboard-fieldset="editable-settings"/);
  assert.match(runtimePage, /id="mission-settings-workspace"/);
  assert.match(runtimePage, /data-dx-zod-dashboard-field="workspaceName"/);
  assert.match(runtimePage, /id="mission-settings-email"/);
  assert.match(runtimePage, /data-dx-zod-dashboard-field="contactEmail"/);
  assert.match(runtimePage, /id="mission-settings-score"/);
  assert.match(runtimePage, /data-dx-zod-dashboard-field="launchScoreTarget"/);
  assert.match(runtimePage, /id="mission-settings-locale"/);
  assert.match(runtimePage, /data-dx-zod-dashboard-field="defaultLocale"/);
  assert.match(runtimePage, /id="mission-settings-theme"/);
  assert.match(runtimePage, /data-dx-zod-dashboard-field="theme"/);
  assert.match(runtimePage, /id="mission-settings-preview-mode"/);
  assert.match(runtimePage, /data-dx-zod-dashboard-field="previewMode"/);
  assert.match(runtimePage, /id="mission-settings-receipts-required"/);
  assert.match(runtimePage, /data-dx-zod-dashboard-field="packageReceiptsRequired"/);
  assert.match(runtimePage, /id="mission-settings-show-errors"/);
  assert.match(runtimePage, /data-dx-zod-dashboard-action="load-invalid-settings"/);
  assert.match(runtimePage, /id="mission-settings-validate"/);
  assert.match(runtimePage, /data-dx-zod-dashboard-action="load-valid-settings"/);
  assert.match(runtimePage, /id="mission-settings-receipt"/);
  assert.match(runtimePage, /data-dx-zod-dashboard-receipt="idle"/);
  assert.match(runtimePage, /id="mission-settings-receipt-json"/);
  assert.match(runtimePage, /data-dx-zod-dashboard-receipt-json="idle"/);
  assert.match(runtimePage, /data-dx-zod-dashboard-receipt-api="createDxDashboardSettingsReceipt"/);
  assert.match(runtimePage, /data-dx-zod-form="dashboard-settings"/);
  assert.match(runtimePage, /data-dx-zod-settings-field="workspaceName"/);
  assert.match(runtimePage, /data-dx-zod-settings-field="contactEmail"/);
  assert.match(runtimePage, /data-dx-zod-settings-field="defaultLocale"/);
  assert.match(runtimePage, /data-dx-zod-settings-field="theme"/);
  assert.match(runtimePage, /data-dx-zod-settings-field="previewMode"/);
  assert.match(runtimePage, /data-dx-zod-settings-field="launchScoreTarget"/);
  assert.match(runtimePage, /data-dx-zod-settings-field="packageReceiptsRequired"/);
  assert.match(runtimePage, /id="form-field-errors"/);
  assert.match(runtimePage, /data-dx-zod-field-errors-api="z\.flattenError"/);
  assert.match(runtimePage, /data-dx-zod-validation-field-errors="idle"/);
  assert.match(runtimePage, /id="form-settings-summary"/);
  assert.match(runtimePage, /data-dx-zod-settings-summary="idle"/);
  assert.match(runtimeScript, /zodSettingsValidationContract/);
  assert.match(runtimeScript, /function bindSettingsForm/);
  assert.match(runtimeScript, /function bindMissionSettingsShortcuts/);
  assert.match(runtimeScript, /function readMissionSettingsPayload/);
  assert.match(runtimeScript, /function writeMissionSettingsControls/);
  assert.match(runtimeScript, /function createMissionSettingsReceipt/);
  assert.match(runtimeScript, /function renderMissionSettingsReceipt/);
  assert.match(runtimeScript, /mission-settings-show-errors/);
  assert.match(runtimeScript, /mission-settings-validate/);
  assert.match(runtimeScript, /mission-settings-receipt-json/);
  assert.match(runtimeScript, /mission-settings-workspace/);
  assert.match(runtimeScript, /mission-settings-email/);
  assert.match(runtimeScript, /mission-settings-score/);
  assert.match(runtimeScript, /mission-settings-locale/);
  assert.match(runtimeScript, /mission-settings-theme/);
  assert.match(runtimeScript, /mission-settings-preview-mode/);
  assert.match(runtimeScript, /mission-settings-receipts-required/);
  assert.match(runtimeScript, /dxZodDashboardActionState/);
  assert.match(runtimeScript, /dxZodDashboardReceipt/);
  assert.match(runtimeScript, /dxZodDashboardReceiptJson/);
  assert.match(runtimeScript, /dxZodDashboardReceiptApi/);
  assert.match(runtimeScript, /function createZodFieldErrors/);
  assert.match(runtimeScript, /dxZodFieldErrors/);
  assert.match(runtimeScript, /dxZodSettingsSummary/);
  assert.match(runtimeScript, /zodSettingsValidation/);
  assert.match(runtimeScript, /zodSettingsIssueCount/);
  assert.match(runtimeScript, /zodSettingsWorkspaceName/);
  assert.match(runtimeScript, /zodSettingsLaunchScoreTarget/);
  assert.match(runtimeScript, /workspaceName/);
  assert.match(runtimeScript, /contactEmail/);
  assert.match(runtimeScript, /launchScoreTarget/);
  assert.match(runtimeScript, /packageReceiptsRequired/);
  assert.match(runtimeScript, /mission-settings-status/);
  assert.match(runtimeScript, /dxDashboardSettingsValidation/);
  assert.match(runtimeScript, /dxDashboardSettingsIssueCount/);
  assert.match(runtimeScript, /fieldErrorsApi: "z\.flattenError"/);
  assert.match(runtimeScript, /Settings validation updated the launch dashboard/);
  const receipt = JSON.parse(dashboardReceipt);
  assert.equal(receipt.schema, "dx.forge.package_dashboard_workflow_receipt");
  assert.equal(receipt.package_id, "validation/zod");
  assert.equal(receipt.official_dx_package_name, "Validation & Schemas");
  assert.equal(receipt.upstream_package, "zod");
  assert.equal(receipt.upstream_version, "4.4.3");
  assert.equal(receipt.honesty_label, "SOURCE-ONLY");
  assert.equal(
    receipt.dx_check_visibility.schema,
    "dx.forge.package.dx_check_visibility",
  );
  assert.equal(receipt.dx_check_visibility.current_status, "present");
  assert.ok(receipt.dx_check_visibility.statuses.includes("missing receipt"));
  assert.ok(receipt.dx_check_visibility.statuses.includes("unsupported surface"));
  assert.ok(
    receipt.dx_check_visibility.monitored_surfaces.includes(
      "dashboard-settings-validation",
    ),
  );
  assert.equal(receipt.component, "launch-settings-validation-summary");
  assert.equal(receipt.node_modules_required, false);
  assert.equal(receipt.no_runtime_execution, true);
  assert.ok(receipt.upstream_public_apis.includes("safeParse"));
  assert.ok(receipt.upstream_public_apis.includes("z.flattenError"));
  assert.ok(receipt.local_readiness_interactions.includes("load-invalid-settings"));
  assert.ok(receipt.local_readiness_interactions.includes("load-valid-settings"));
  assert.equal(receipt.local_demo_interactions, undefined);
  assert.ok(receipt.stable_markers.includes('data-dx-package="validation/zod"'));
  assert.ok(
    receipt.stable_markers.includes(
      'data-dx-zod-dashboard-receipt-api="createDxDashboardSettingsReceipt"',
    ),
  );
  assert.ok(receipt.stable_markers.includes("data-dx-zod-dashboard-field-state"));
  assert.ok(receipt.stable_markers.includes("data-dx-zod-dashboard-field-error"));
  assert.ok(
    receipt.guards.includes(
      "dx run --test .\\benchmarks\\zod-dashboard-settings-workflow.test.ts",
    ),
  );
  assert.doesNotMatch(dashboardStarterComponent, /#[0-9a-fA-F]{3,8}/);
  assert.doesNotMatch(
    dashboardStarterComponent,
    /\b(?:bg|text|border)-(?:neutral|slate|zinc|stone|gray)-/,
  );
  assert.ok(
    !fs.existsSync(path.join(root, "examples", "template", "node_modules")),
    "launch template must not add a local node_modules workflow",
  );
});
