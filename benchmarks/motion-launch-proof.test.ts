const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const shellPath = path.join(root, "examples", "template", "template-shell.tsx");
const proofPath = path.join(
  root,
  "examples",
  "template",
  "motion-interaction-proof.tsx",
);
const routeContractPath = path.join(
  root,
  "examples",
  "template",
  "template-route-contract.ts",
);
const runtimePagePath = path.join(
  root,
  "tools",
  "launch",
  "runtime-template",
  "pages",
  "index.html",
);
const runtimeScriptPath = path.join(
  root,
  "tools",
  "launch",
  "runtime-template",
  "assets",
  "launch-runtime.ts",
);
const runtimeStylePath = path.join(
  root,
  "tools",
  "launch",
  "runtime-template",
  "assets",
  "launch-runtime.css",
);
const cliPath = path.join(root, "dx-www", "src", "cli", "mod.rs");
const wwwTemplateNodeModulesPath = path.join(
  root,
  "examples",
  "template",
  "node_modules",
);

function readRequiredFile(filePath) {
  assert.ok(fs.existsSync(filePath), `expected ${path.relative(root, filePath)} to exist`);
  return fs.readFileSync(filePath, "utf8");
}

test("launch template has a visible Motion interaction proof", () => {
  const shell = readRequiredFile(shellPath);
  const proof = readRequiredFile(proofPath);
  const routeContract = readRequiredFile(routeContractPath);
  const runtimePage = readRequiredFile(runtimePagePath);
  const runtimeScript = readRequiredFile(runtimeScriptPath);
  const runtimeStyle = readRequiredFile(runtimeStylePath);
  const cli = readRequiredFile(cliPath);

  assert.ok(
    !fs.existsSync(wwwTemplateNodeModulesPath),
    "launch template must not contain a local node_modules folder",
  );

  assert.match(
    shell,
    /import \{ LaunchMotionInteractionProof \} from "\.\/motion-interaction-proof";/,
  );
  assert.match(shell, /<LaunchMotionInteractionProof \/>/);
  assert.match(shell, /data-dx-package="animation\/motion"/);
  assert.match(shell, /function LaunchMotionDashboardWorkflow/);
  assert.match(shell, /data-dx-component="launch-motion-dashboard-workflow"/);
  assert.match(shell, /data-dx-dashboard-workflow="motion-panel-orchestration"/);
  assert.match(shell, /data-dx-product-surface="launch-dashboard"/);
  assert.match(shell, /data-dx-edit-kind="dashboard-workflow"/);
  assert.doesNotMatch(shell, /data-dx-component="motion-package-proof"/);
  assert.match(shell, /primaryRoles = new Set\(\[[\s\S]*"animation"/);

  assert.match(proof, /export function LaunchMotionInteractionProof/);
  assert.match(proof, /data-dx-package="animation\/motion"/);
  assert.match(proof, /data-dx-component="motion-interaction-proof"/);
  assert.match(proof, /data-dx-motion-interaction="advance-stage"/);
  assert.match(proof, /data-dx-motion-interaction="reverse-order"/);
  assert.match(proof, /data-dx-motion-interaction="reset-proof"/);
  assert.match(proof, /data-dx-motion-interaction="toggle-reduced-motion"/);
  assert.match(proof, /data-dx-motion-policy="app-owned-reduced-motion-preview"/);
  assert.match(proof, /data-dx-motion-reduced=\{reducedMotionPreview \? "preview" : "system"\}/);
  assert.match(proof, /data-dx-motion-state=\{activeStage\.id\}/);
  assert.match(proof, /data-dx-motion-progress=\{progress\}/);
  assert.match(proof, /data-dx-motion-order=\{orderedStageIds\.join\(","\)\}/);
  assert.match(proof, /MotionControlledStatus/);
  assert.match(proof, /MotionLazyBox/);
  assert.match(proof, /MotionValueMeter/);
  assert.match(proof, /DxMotionLayoutGroup/);
  assert.match(proof, /MotionLayoutItem/);
  assert.match(proof, /useDxInstantLayoutTransition/);
  assert.match(proof, /useDxMotionPressFeedback/);
  assert.match(proof, /setStageIndex/);
  assert.match(proof, /setOrderedStageIds/);
  assert.doesNotMatch(proof, /fake|dummy|lorem/i);
  assert.doesNotMatch(
    proof,
    /\b(?:bg|text|border)-(?:neutral|slate|zinc|stone|gray|emerald|cyan|sky|amber)-/,
  );

  assert.match(routeContract, /"components\/template-app\/motion-interaction-proof\.tsx"/);
  assert.match(
    routeContract,
    /motionProofSourceGuard: "dx run --test \.\\\\benchmarks\\\\motion-launch-proof\.test\.ts"/,
  );
  assert.match(
    routeContract,
    /motionMaterializedLaunchGuard:\s*"dx run --test \.\\\\benchmarks\\\\motion-launch-materialized\.test\.ts"/,
  );
  assert.match(
    routeContract,
    /motionRuntimeInteractionGuard:\s*"dx run --test \.\\\\benchmarks\\\\motion-runtime-interaction\.test\.ts"/,
  );

  assert.match(runtimePage, /data-dx-package="animation\/motion"/);
  assert.match(runtimePage, /data-dx-component="launch-motion-dashboard-summary"/);
  assert.match(runtimePage, /data-dx-dashboard-card="animation"/);
  assert.match(runtimePage, /id="mission-motion-status"/);
  assert.match(runtimePage, /id="mission-motion-detail"/);
  assert.match(runtimePage, /id="mission-motion-policy"/);
  assert.match(runtimePage, /data-dx-component="motion-animation-card"/);
  assert.match(runtimePage, /data-dx-dashboard-workflow="motion-panel-orchestration"/);
  assert.match(runtimePage, /data-dx-product-surface="launch-dashboard"/);
  assert.match(runtimePage, /data-dx-motion-policy="app-owned-reduced-motion-preview"/);
  assert.match(runtimePage, /data-dx-motion-reduced="system"/);
  assert.match(runtimePage, /data-dx-motion-interaction="advance-stage"/);
  assert.match(runtimePage, /data-dx-motion-interaction="reverse-order"/);
  assert.match(runtimePage, /data-dx-motion-interaction="reset-proof"/);
  assert.match(runtimePage, /data-dx-motion-interaction="toggle-reduced-motion"/);
  assert.match(runtimePage, /data-dx-motion-state="source-owned"/);
  assert.match(runtimePage, /data-dx-motion-progress="34"/);
  assert.match(runtimePage, /data-dx-motion-order="source-owned,interactive,preview-ready"/);
  assert.match(runtimePage, /data-dx-motion-progress-bar/);
  assert.match(runtimePage, /data-dx-motion-stage="source-owned"/);
  assert.match(runtimePage, /data-dx-motion-stage="interactive"/);
  assert.match(runtimePage, /data-dx-motion-stage="preview-ready"/);
  assert.match(runtimeScript, /const motionStages = \[/);
  assert.match(runtimeScript, /function renderMotionProof\(\)/);
  assert.match(runtimeScript, /dashboard\.dataset\.dxDashboardMotion/);
  assert.match(runtimeScript, /setText\(\s*"#mission-motion-status"/);
  assert.match(runtimeScript, /setText\(\s*"#mission-motion-detail"/);
  assert.match(runtimeScript, /dataset\.dxMotionState/);
  assert.match(runtimeScript, /dataset\.dxMotionProgress/);
  assert.match(runtimeScript, /dataset\.dxMotionOrder/);
  assert.match(runtimeScript, /dataset\.dxMotionReduced/);
  assert.match(runtimeScript, /"#mission-motion-policy"/);
  assert.match(runtimeScript, /data-dx-motion-interaction='advance-stage'/);
  assert.match(runtimeScript, /data-dx-motion-interaction='reverse-order'/);
  assert.match(runtimeScript, /data-dx-motion-interaction='reset-proof'/);
  assert.match(runtimeScript, /data-dx-motion-interaction='toggle-reduced-motion'/);
  assert.match(runtimeStyle, /\.motion-progress-track/);
  assert.match(runtimeStyle, /\.motion-stage-list/);
  assert.match(runtimeStyle, /\.motion-stage-card/);
  assert.match(runtimeStyle, /\.motion-proof\[data-dx-motion-reduced="preview"\]/);
  const motionRuntimeStyles =
    runtimeStyle.match(/\.motion-proof[\s\S]*?\.markdown/)?.[0] ?? "";
  assert.match(runtimeStyle, /--dx-motion-active-border:/);
  assert.match(runtimeStyle, /--dx-motion-active-bg:/);
  assert.match(runtimeStyle, /--dx-motion-progress-track:/);
  assert.match(motionRuntimeStyles, /var\(--dx-motion-active-border\)/);
  assert.match(motionRuntimeStyles, /var\(--dx-motion-active-bg\)/);
  assert.match(motionRuntimeStyles, /var\(--dx-motion-progress-track\)/);
  assert.doesNotMatch(motionRuntimeStyles, /#[0-9a-f]{3,8}|rgba?\(/i);

  assert.match(cli, /NEXT_FAMILIAR_MOTION_INTERACTION_PROOF_TSX/);
  assert.match(cli, /examples\/template\/motion-interaction-proof\.tsx/);
  assert.match(cli, /"components\/template-app\/motion-interaction-proof\.tsx"/);
  assert.match(cli, /"kind": "motion_launch_proof_guard"/);
  assert.match(cli, /"dx run --test \.\\\\benchmarks\\\\motion-launch-proof\.test\.ts"/);
});
