const assert = require("node:assert/strict");
const { execFileSync } = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("validation/zod stays visible and interactive in the launch template", () => {
  const shell = read("examples/template/template-shell.tsx");
  const zodStatus = read("examples/template/zod-validation-status.tsx");
  const runtimePage = read("tools/launch/runtime-template/pages/index.html");
  const runtimeJs = read("tools/launch/runtime-template/assets/launch-runtime.ts");
  const cli = read("dx-www/src/cli/mod.rs");

  assert.match(shell, /import \{ LaunchZodValidationStatus \}/);
  assert.match(shell, /<LaunchZodValidationStatus \/>/);
  assert.match(zodStatus, /"use client";/);
  assert.match(zodStatus, /import \{ Button \} from "@\/components\/ui\/button";/);
  assert.match(zodStatus, /React\.useState\(invalidZodValidationInput\)/);
  assert.match(zodStatus, /safeParseDxLaunchSignupForDisplay\(validationInput\)/);
  assert.match(zodStatus, /data-dx-zod-validation-input="email"/);
  assert.match(zodStatus, /data-dx-zod-validation-input="name"/);
  assert.match(zodStatus, /data-dx-zod-validation-input="intent"/);
  assert.match(zodStatus, /data-dx-package="validation\/zod"/);
  assert.match(zodStatus, /data-dx-component="zod-validation-readiness"/);
  assert.match(zodStatus, /data-dx-zod-validation-state/);
  assert.match(zodStatus, /data-dx-zod-validation-result/);
  assert.match(zodStatus, /data-dx-zod-validation-email/);
  assert.match(zodStatus, /data-dx-zod-validation-error/);
  assert.match(zodStatus, /onChange=\{\(event\) =>/);
  assert.match(zodStatus, /data-dx-zod-validation-action="load-invalid"/);
  assert.match(zodStatus, /data-dx-zod-validation-action="load-valid"/);
  assert.match(zodStatus, /Show error/);
  assert.match(zodStatus, /Validate sample/);
  assert.match(runtimePage, /data-dx-component="zod-form-card"/);
  assert.match(runtimePage, /data-dx-package="validation\/zod"/);
  assert.match(runtimePage, /data-dx-zod-validation="runtime-settings-validation"/);
  assert.match(runtimePage, /data-dx-zod-form="dashboard-settings"/);
  assert.match(runtimePage, /data-dx-zod-settings-field="workspaceName"/);
  assert.match(runtimePage, /data-dx-zod-settings-field="contactEmail"/);
  assert.match(runtimePage, /data-dx-zod-settings-field="defaultLocale"/);
  assert.match(runtimePage, /data-dx-zod-settings-field="theme"/);
  assert.match(runtimePage, /data-dx-zod-settings-field="previewMode"/);
  assert.match(runtimePage, /data-dx-zod-settings-field="launchScoreTarget"/);
  assert.match(runtimePage, /data-dx-zod-settings-field="packageReceiptsRequired"/);
  assert.match(runtimePage, /data-dx-zod-validation-action="validate-settings-form"/);
  assert.match(runtimePage, /data-dx-zod-validation-action="load-invalid-settings"/);
  assert.match(runtimePage, /data-dx-zod-validation-action="load-valid-settings"/);
  assert.match(runtimePage, /data-dx-zod-dashboard-controls="mission-control"/);
  assert.match(runtimePage, /data-dx-zod-dashboard-fieldset="editable-settings"/);
  assert.match(runtimePage, /id="mission-settings-workspace"/);
  assert.match(runtimePage, /id="mission-settings-email"/);
  assert.match(runtimePage, /id="mission-settings-score"/);
  assert.match(runtimePage, /id="mission-settings-locale"/);
  assert.match(runtimePage, /id="mission-settings-theme"/);
  assert.match(runtimePage, /id="mission-settings-preview-mode"/);
  assert.match(runtimePage, /id="mission-settings-receipts-required"/);
  assert.match(runtimePage, /id="mission-settings-show-errors"/);
  assert.match(runtimePage, /id="mission-settings-validate"/);
  assert.match(runtimePage, /data-dx-zod-dashboard-receipt="idle"/);
  assert.match(runtimePage, /id="mission-settings-receipt-json"/);
  assert.match(runtimePage, /data-dx-zod-dashboard-receipt-api="createDxDashboardSettingsReceipt"/);
  assert.match(runtimePage, /data-dx-zod-validation-issues="\[\]"/);
  assert.match(runtimePage, /data-dx-zod-validation-issue-list/);
  assert.match(runtimePage, /data-dx-zod-field-errors-api="z\.flattenError"/);
  assert.match(runtimePage, /data-dx-zod-validation-field-errors="idle"/);
  assert.match(runtimePage, /data-dx-zod-settings-summary="idle"/);
  assert.match(runtimePage, /data-dx-zod-validation-payload="idle"/);
  assert.match(runtimePage, /data-dx-zod-validation-output="pending"/);
  assert.match(runtimePage, /data-dx-zod-validation-result="idle"/);
  assert.match(runtimePage, /data-dx-zod-validation-state="ready"/);
  assert.match(runtimeJs, /function renderZodIssues/);
  assert.match(runtimeJs, /function createZodFieldErrors/);
  assert.match(runtimeJs, /dxZodFieldErrors/);
  assert.match(runtimeJs, /dxZodSettingsSummary/);
  assert.match(runtimeJs, /function renderZodPayload/);
  assert.match(runtimeJs, /function bindSettingsForm/);
  assert.match(runtimeJs, /function bindMissionSettingsShortcuts/);
  assert.match(runtimeJs, /function readMissionSettingsPayload/);
  assert.match(runtimeJs, /function writeMissionSettingsControls/);
  assert.match(runtimeJs, /function createMissionSettingsReceipt/);
  assert.match(runtimeJs, /function renderMissionSettingsReceipt/);
  assert.match(runtimeJs, /mission-settings-show-errors/);
  assert.match(runtimeJs, /mission-settings-validate/);
  assert.match(runtimeJs, /function setZodSample/);
  assert.match(runtimeJs, /workspaceName/);
  assert.match(runtimeJs, /contactEmail/);
  assert.match(runtimeJs, /launchScoreTarget/);
  assert.match(runtimeJs, /packageReceiptsRequired/);
  assert.match(runtimeJs, /code: "too_small"/);
  assert.match(runtimeJs, /code: "invalid_string"/);
  assert.match(runtimeJs, /dxZodValidationOutput = "rejected"/);
  assert.match(runtimeJs, /dxZodValidationOutput = "accepted"/);
  assert.match(runtimeJs, /dxZodValidationState = "invalid"/);
  assert.match(runtimeJs, /dxZodValidationState = "valid"/);
  assert.match(runtimeJs, /dxZodValidationResult = "error"/);
  assert.match(runtimeJs, /dxZodValidationResult = "success"/);
  assert.match(cli, /NEXT_FAMILIAR_ZOD_STATUS_TSX/);
  assert.match(cli, /"components\/template-app\/zod-validation-status\.tsx"/);

  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-zod-launch-"));
  fs.mkdirSync(path.join(dir, "app", "launch"), { recursive: true });
  fs.writeFileSync(
    path.join(dir, "app", "launch", "page.tsx"),
    "export default function Page(){ return null; }\n",
  );
  const materializer = path.join(root, "tools", "launch", "materialize-www-template.ts");
  const result = JSON.parse(execFileSync(process.execPath, [materializer, dir], {
    cwd: root,
    encoding: "utf8",
  }));
  const materializedLaunch = fs.readFileSync(path.join(dir, "pages", "index.html"), "utf8");
  const materializedRuntime = fs.readFileSync(path.join(dir, "public", "launch-runtime.js"), "utf8");
  const manifest = JSON.parse(fs.readFileSync(path.join(dir, "public", "preview-.dx/build-cache/manifest.json"), "utf8"));

  assert.equal(result.ok, true);
  assert.equal(result.noNodeModules, true);
  assert.ok(!fs.existsSync(path.join(dir, "node_modules")));
  assert.match(materializedLaunch, /data-dx-package="validation\/zod"/);
  assert.match(materializedLaunch, /data-dx-zod-form="dashboard-settings"/);
  assert.match(materializedLaunch, /data-dx-zod-dashboard-controls="mission-control"/);
  assert.match(materializedLaunch, /data-dx-zod-dashboard-fieldset="editable-settings"/);
  assert.match(materializedLaunch, /id="mission-settings-validate"/);
  assert.match(materializedLaunch, /id="mission-settings-receipt-json"/);
  assert.match(materializedLaunch, /data-dx-zod-field-errors-api="z\.flattenError"/);
  assert.match(materializedLaunch, /data-dx-zod-validation-action="load-invalid-settings"/);
  assert.match(materializedLaunch, /data-dx-zod-validation-payload="idle"/);
  assert.match(materializedRuntime, /function createZodFieldErrors/);
  assert.match(materializedRuntime, /function renderZodPayload/);
  assert.match(materializedRuntime, /function bindMissionSettingsShortcuts/);
  assert.match(materializedRuntime, /function readMissionSettingsPayload/);
  assert.match(materializedRuntime, /function renderMissionSettingsReceipt/);
  assert.match(materializedRuntime, /function bindSettingsForm/);
  assert.match(materializedRuntime, /dxZodValidationOutput = "accepted"/);
  assert.ok(
    manifest.routes.some((route) =>
      route.route === "/" && route.forgePackages.includes("validation/zod"),
    ),
  );
});
