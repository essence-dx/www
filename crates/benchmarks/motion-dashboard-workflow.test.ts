const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "motion");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("animation/motion exposes a real dashboard workflow", () => {
  const upstreamMotion = read(path.join(mirror, "packages", "motion", "package.json"));
  const upstreamReact = read(
    path.join(mirror, "packages", "framer-motion", "src", "index.ts"),
  );
  const forge = read(path.join(root, "core", "src", "ecosystem", "forge_motion.rs"));
  const dashboard = read(
    path.join(root, "examples", "dashboard", "src", "pages", "Dashboard.tsx"),
  );
  const launchShell = read(path.join(root, "examples", "template", "template-shell.tsx"));
  const motionProof = read(
    path.join(root, "examples", "template", "motion-interaction-proof.tsx"),
  );
  const workflow = read(
    path.join(
      root,
      "examples",
      "dashboard",
      "src",
      "components",
      "MotionDashboardWorkflow.tsx",
    ),
  );
  const model = read(
    path.join(root, "examples", "dashboard", "src", "lib", "motionDashboardWorkflow.ts"),
  );
  const readme = read(path.join(root, "examples", "dashboard", "README.md"));
  const packageDocs = read(path.join(root, "docs", "packages", "animation-motion.md"));
  const routeContract = read(
    path.join(root, "examples", "template", "template-route-contract.ts"),
  );
  const editContract = read(
    path.join(root, "examples", "template", "dx-studio-edit-contract.ts"),
  );
  const studioManifest = read(path.join(root, "dx-www", "src", "cli", "studio_manifest.rs"));
  const receiptPath = path.join(
    root,
    "examples",
    "www-template",
    ".dx",
    "forge",
    "receipts",
    "2026-05-22-animation-motion-dashboard-workflow.json",
  );
  const receipt = JSON.parse(read(receiptPath));
  const dx = read(path.join(root, "DX.md"));
  const todo = read(path.join(root, "TODO.md"));
  const changelog = read(path.join(root, "CHANGELOG.md"));

  assert.match(upstreamMotion, /"name": "motion"/);
  assert.match(upstreamMotion, /"\.\/react"/);
  assert.match(upstreamReact, /AnimatePresence/);
  assert.match(upstreamReact, /LazyMotion/);
  assert.match(upstreamReact, /MotionConfig/);
  assert.match(upstreamReact, /Reorder/);

  assert.match(forge, /dashboardUsage: \{/);
  assert.match(forge, /launchComponent: "examples\/template\/template-shell\.tsx"/);
  assert.match(forge, /proofComponent: "examples\/template\/motion-interaction-proof\.tsx"/);
  assert.match(forge, /launchWorkflow: "motion-panel-orchestration"/);
  assert.match(forge, /dashboardSummaryMarker: "data-dx-component=\\"launch-motion-dashboard-summary\\""/);
  assert.match(forge, /workflowMarker: "data-dx-dashboard-workflow=\\"motion-panel-orchestration\\""/);
  assert.match(forge, /reducedMotionMarker: "data-dx-motion-reduced"/);
  assert.match(forge, /2026-05-22-animation-motion-dashboard-workflow\.json/);
  assert.match(forge, /studioSurface: "motion-interaction-proof"/);
  assert.match(
    forge,
    /studioSurfaces: \["motion-dashboard-workflow", "motion-interaction-proof"\]/,
  );
  assert.match(forge, /"data-dx-motion-interaction"/);
  assert.match(forge, /"js\/motion\/dashboard-workflow\.ts"/);
  assert.match(forge, /const MOTION_DASHBOARD_WORKFLOW_TS/);
  assert.match(forge, /motionDashboardStages/);
  assert.match(forge, /getMotionDashboardStage/);
  assert.match(forge, /moveMotionDashboardStage/);
  assert.match(forge, /motionDashboardPreferenceStorageKey/);
  assert.match(forge, /readMotionDashboardPreference/);
  assert.match(forge, /writeMotionDashboardPreference/);
  assert.match(forge, /createMotionDashboardReceipt/);
  assert.match(forge, /MotionDashboardWorkflow\.tsx/);
  assert.match(forge, /motionDashboardWorkflow\.ts/);
  assert.match(forge, /dxIcon: "pack:motion"/);
  assert.match(forge, /motion-dashboard-workflow\.test\.ts/);

  assert.match(model, /packageId: 'animation\/motion'/);
  assert.match(model, /cliCommand: 'dx add motion-animation --write'/);
  assert.doesNotMatch(model, /cliCommand: 'dx add motion\/react --write'/);
  assert.match(model, /aliases: \[/);
  assert.match(model, /sourceMirror: 'G:\/WWW\/inspirations\/motion'/);
  assert.match(model, /provenance: \{/);
  assert.match(model, /exportedFiles: \[/);
  assert.match(model, /requiredEnv: \[\]/);
  assert.match(model, /appOwnedBoundaries: \[/);
  assert.match(model, /receiptPaths: \[/);
  assert.match(model, /MotionConfig/);
  assert.match(model, /LazyMotion/);
  assert.match(model, /AnimatePresence/);
  assert.match(model, /LayoutGroup/);
  assert.match(model, /Reorder/);
  assert.match(model, /useMotionValue/);
  assert.match(model, /useScroll/);
  assert.match(model, /moveMotionDashboardStage/);
  assert.match(model, /motionDashboardPreferenceStorageKey/);
  assert.match(model, /readMotionDashboardPreference/);
  assert.match(model, /writeMotionDashboardPreference/);
  assert.match(model, /createMotionDashboardReceipt/);
  assert.match(model, /officialPackageName: 'Motion & Animation'/);
  assert.match(model, /motionDashboardInspectedSourceFiles = \[/);
  assert.match(model, /inspectedSourceFiles: motionDashboardInspectedSourceFiles/);
  assert.match(model, /packages\/motion\/src\/react\.ts/);
  assert.match(model, /packages\/framer-motion\/src\/index\.ts/);
  assert.match(model, /motionDashboardSelectedSurfaces/);
  assert.match(model, /motionDashboardDxCheckVisibility/);
  assert.match(model, /schema: 'dx\.forge\.package\.dx_check_visibility'/);
  assert.match(model, /currentStatus: 'present'/);
  for (const status of [
    "present",
    "stale",
    "missing-receipt",
    "blocked",
    "unsupported-surface",
  ]) {
    assert.match(
      model,
      new RegExp(`status: '${status}'`),
      `starter dashboard metadata should document dx-check status ${status}`,
    );
  }

  assert.match(dashboard, /import \{ MotionDashboardWorkflow \}/);
  assert.match(dashboard, /<MotionDashboardWorkflow \/>/);
  assert.match(
    launchShell,
    /id: "motion",\s*label: "Motion & Animation workflow",\s*selector: '\[data-dx-component="launch-motion-dashboard-workflow"\]',\s*sourceFile: "examples\/template\/template-shell\.tsx"/,
  );
  assert.doesNotMatch(
    launchShell,
    /id: "motion",\s*label: "Motion & Animation workflow",\s*selector: '\[data-dx-component="launch-motion-dashboard-workflow"\]',\s*sourceFile: "examples\/template\/motion-interaction-proof\.tsx"/,
  );
  assert.match(launchShell, /function LaunchMotionDashboardWorkflow\(\)/);
  assert.match(launchShell, /<LaunchMotionInteractionProof \/>/);
  assert.match(workflow, /data-dx-package="animation\/motion"/);
  assert.match(workflow, /data-dx-component="dashboard-motion-workflow"/);
  assert.match(workflow, /data-dx-motion-dashboard-workflow="animated-readiness"/);
  assert.match(workflow, /data-dx-motion-action="select-stage"/);
  assert.match(workflow, /data-dx-motion-action="reverse-order"/);
  assert.match(workflow, /data-dx-motion-action="move-stage-previous"/);
  assert.match(workflow, /data-dx-motion-action="move-stage-next"/);
  assert.match(workflow, /data-dx-motion-order-direction="previous"/);
  assert.match(workflow, /data-dx-motion-order-direction="next"/);
  assert.match(workflow, /data-dx-motion-order-available/);
  assert.match(workflow, /data-dx-motion-preference-storage="local-storage"/);
  assert.match(workflow, /data-dx-motion-storage-key=\{motionDashboardPreferenceStorageKey\}/);
  assert.match(workflow, /data-dx-motion-keyboard-reorder="arrow-home-end"/);
  assert.match(workflow, /data-dx-motion-action="prepare-motion-receipt"/);
  assert.match(motionProof, /data-dx-motion-interaction="move-stage-previous"/);
  assert.match(motionProof, /data-dx-motion-interaction="move-stage-next"/);
  assert.match(motionProof, /data-dx-motion-order-available/);
  assert.match(motionProof, /data-dx-motion-preference-storage="local-storage"/);
  assert.match(motionProof, /data-dx-motion-storage-key=\{motionPreferenceStorageKey\}/);
  assert.match(motionProof, /data-dx-motion-keyboard-reorder="arrow-home-end"/);
  assert.match(motionProof, /onKeyDown=\{handleKeyboardReorder\}/);
  assert.match(workflow, /data-dx-motion-action="toggle-reduced-motion"/);
  assert.match(workflow, /data-dx-motion-policy="app-owned-reduced-motion-preview"/);
  assert.match(workflow, /data-dx-motion-progress-bar="dashboard"/);
  assert.match(workflow, /data-dx-style-surface="theme-token-card"/);
  assert.match(workflow, /data-dx-node-modules="forbidden"/);
  assert.match(workflow, /<dx-icon name="pack:motion"/);
  assert.match(workflow, /<dx-icon name="pack:reorder"/);
  assert.match(workflow, /<dx-icon name="pack:receipt"/);
  assert.match(workflow, /useState/);
  assert.doesNotMatch(workflow, /#[0-9a-fA-F]{3,8}|rgb\(|hsl\(/);
  assert.doesNotMatch(workflow, /\b(?:bg|text|border)-(?:neutral|slate|zinc|stone|gray|emerald)-/);
  assert.doesNotMatch(workflow, /lucide|heroicons|phosphor/i);

  assert.match(readme, /Motion & Animation workflow/);
  assert.match(readme, /js\/motion\/dashboard-workflow\.ts/);
  assert.match(packageDocs, /# Motion & Animation/);
  assert.match(packageDocs, /Package id: `animation\/motion`/);
  assert.match(packageDocs, /G:\\WWW\\inspirations\\motion/);
  assert.match(packageDocs, /motion\/dashboard-workflow\.ts/);
  assert.match(packageDocs, /MotionDashboardWorkflow/);
  assert.match(packageDocs, /launch-motion-dashboard-workflow/);
  assert.match(packageDocs, /motion-panel-orchestration/);
  assert.match(packageDocs, /launch-motion-dashboard-summary/);
  assert.match(packageDocs, /toggle-reduced-motion/);
  assert.match(packageDocs, /move-stage-previous/);
  assert.match(packageDocs, /move-stage-next/);
  assert.match(packageDocs, /data-dx-motion-order-available/);
  assert.match(packageDocs, /2026-05-22-animation-motion-dashboard-workflow\.json/);
  assert.doesNotMatch(packageDocs, /Coding score:\s*100\/100/);
  assert.match(packageDocs, /Reality audit: REAL/);
  assert.match(packageDocs, /Official DX package name: Motion & Animation/);
  assert.match(packageDocs, /Official CLI: `dx add motion-animation --write`/);
  assert.match(packageDocs, /dx-check visibility/);
  assert.match(packageDocs, /present/);
  assert.match(packageDocs, /missing-receipt/);
  assert.match(packageDocs, /unsupported-surface/);
  assert.match(packageDocs, /<dx-icon name="pack:motion" \/>/);
  assert.match(packageDocs, /no `node_modules` workflow/);
  assert.match(packageDocs, /App-owned boundaries/);

  assert.equal(receipt.schema, "dx.forge.package_dashboard_workflow_receipt");
  assert.equal(receipt.package_id, "animation/motion");
  assert.equal(receipt.official_package_name, "Motion & Animation");
  assert.equal(receipt.cli_command, "dx add motion-animation --write");
  assert.equal(receipt.route, "/");
  assert.equal(receipt.workflow, "motion-panel-orchestration");
  assert.equal(receipt.product_surface, "launch-dashboard");
  assert.equal(receipt.node_modules_required, false);
  assert.equal(receipt.no_runtime_execution, true);
  assert.equal(receipt.coding_score, 99);
  assert.equal(receipt.reality_audit.verdict, "REAL");
  assert.deepEqual(receipt.reality_audit.upstream_evidence, [
    "G:/WWW/inspirations/motion/packages/motion/package.json",
    "G:/WWW/inspirations/motion/packages/framer-motion/src/index.ts",
  ]);
  assert.deepEqual(receipt.inspected_source_files, [
    "G:/WWW/inspirations/motion/packages/motion/src/react.ts",
    "G:/WWW/inspirations/motion/packages/framer-motion/src/index.ts",
    "G:/WWW/inspirations/motion/packages/framer-motion/src/components/AnimatePresence/index.tsx",
    "G:/WWW/inspirations/motion/packages/framer-motion/src/components/Reorder/Group.tsx",
    "G:/WWW/inspirations/motion/packages/framer-motion/src/value/use-scroll.ts",
  ]);
  assert.deepEqual(
    receipt.selected_surfaces.map((surface) => surface.id),
    ["provider-policy", "layout-reorder", "dashboard-workflow", "source-guard-runbook"],
  );
  assert.equal(receipt.dx_check_visibility.schema, "dx.forge.package.dx_check_visibility");
  assert.equal(receipt.dx_check_visibility.current_status, "present");
  assert.deepEqual(
    receipt.dx_check_visibility.status_legend.map((entry) => entry.status),
    ["present", "stale", "missing-receipt", "blocked", "unsupported-surface"],
  );
  assert.ok(
    receipt.dx_check_visibility.monitored_surfaces.some(
      (surface) =>
        surface.id === "motion-dashboard-workflow" &&
        surface.status === "present" &&
        surface.receipt_path ===
          "examples/template/.dx/forge/receipts/2026-05-22-animation-motion-dashboard-workflow.json",
    ),
  );
  assert.ok(
    receipt.reality_audit.forge_package_files.includes(
      "core/src/ecosystem/forge_motion.rs",
    ),
  );
  assert.ok(
    receipt.reality_audit.dashboard_consumers.includes(
      "examples/template/template-shell.tsx",
    ),
  );
  assert.ok(
    receipt.reality_audit.guard_files.includes(
      "benchmarks/motion-dashboard-workflow.test.ts",
    ),
  );
  assert.ok(
    receipt.reality_audit.partial_boundaries.includes("governed browser QA"),
  );
  for (const apiName of [
    "useAnimationControls",
    "useAnimationFrame",
    "useWillChange",
    "useSpring",
    "domAnimation",
    "useTransform",
    "useMotionTemplate",
    "useMotionValueEvent",
    "useVelocity",
    "useTime",
    "useCycle",
    "usePageInView",
    "usePresence",
    "useIsPresent",
    "useDragControls",
    "motion",
    "m",
    "useAnimation",
    "animationControls",
    "WillChangeMotionValue",
    "useInstantLayoutTransition",
    "useInView",
    "AnimationPlaybackControlsWithThen",
    "MotionValue",
    "Transition",
    "Variants",
  ]) {
    assert.match(
      model,
      new RegExp(`'${apiName}'`),
      `starter dashboard metadata should include upstream Motion API ${apiName}`,
    );
    assert.ok(
      receipt.reality_audit.upstream_public_apis.includes(apiName),
      `receipt reality audit should include upstream Motion API ${apiName}`,
    );
    assert.ok(
      receipt.upstream_public_apis.includes(apiName),
      `receipt top-level API list should include upstream Motion API ${apiName}`,
    );
  }
  assert.ok(receipt.interactions.includes("advance-stage"));
  assert.ok(receipt.interactions.includes("reverse-order"));
  assert.ok(receipt.interactions.includes("move-stage-previous"));
  assert.ok(receipt.interactions.includes("move-stage-next"));
  assert.ok(receipt.interactions.includes("keyboard-reorder"));
  assert.ok(receipt.interactions.includes("persist-preference"));
  assert.ok(receipt.interactions.includes("reset-proof"));
  assert.ok(receipt.interactions.includes("toggle-reduced-motion"));
  assert.ok(receipt.stable_markers.includes('data-dx-motion-interaction="move-stage-previous"'));
  assert.ok(receipt.stable_markers.includes('data-dx-motion-interaction="move-stage-next"'));
  assert.ok(receipt.stable_markers.includes("data-dx-motion-order-available"));
  assert.ok(receipt.stable_markers.includes("data-dx-motion-preference-storage"));
  assert.ok(receipt.stable_markers.includes("data-dx-motion-storage-key"));
  assert.ok(receipt.stable_markers.includes("data-dx-motion-keyboard-reorder"));
  assert.ok(receipt.stable_markers.includes("data-dx-motion-keyboard-state"));
  assert.ok(receipt.stable_markers.includes("data-dx-motion-reduced"));
  assert.deepEqual(receipt.studio_surfaces, [
    {
      id: "motion-dashboard-workflow",
      selector: '[data-dx-component="launch-motion-dashboard-workflow"]',
      source_file: "examples/template/template-shell.tsx",
      materialized_file: "components/template-app/template-shell.tsx",
    },
    {
      id: "motion-interaction-proof",
      selector: '[data-dx-component="motion-interaction-proof"]',
      source_file: "examples/template/motion-interaction-proof.tsx",
      materialized_file: "components/template-app/motion-interaction-proof.tsx",
    },
  ]);
  assert.ok(
    receipt.guards.includes("node --test .\\benchmarks\\motion-runtime-interaction.test.ts"),
  );
  assert.match(routeContract, /2026-05-22-animation-motion-dashboard-workflow\.json/);
  assert.match(routeContract, /motionDashboardWorkflow/);
  assert.match(routeContract, /data-dx-motion-reduced/);
  assert.match(routeContract, /data-dx-motion-order-available/);
  assert.match(routeContract, /data-dx-motion-preference-storage/);
  assert.match(routeContract, /data-dx-motion-storage-key/);
  assert.match(routeContract, /data-dx-motion-keyboard-reorder/);
  assert.match(routeContract, /data-dx-motion-keyboard-state/);
  assert.match(editContract, /const motionDashboardInteractionSelectors = \[/);
  assert.match(editContract, /const motionDashboardStateMarkers = \[/);
  assert.match(editContract, /const motionDashboardReceiptPath =/);
  assert.match(editContract, /id: "motion-interaction-proof"/);
  assert.match(editContract, /selector: '\[data-dx-component="motion-interaction-proof"\]'/);
  assert.match(editContract, /interactionSelectors: motionDashboardInteractionSelectors/);
  assert.match(editContract, /stateMarkers: motionDashboardStateMarkers/);
  assert.match(
    editContract,
    /stateMarkers: \[\s*\.\.\.motionDashboardStateMarkers,\s*"data-dx-motion-policy-status",\s*\]/s,
  );
  assert.match(editContract, /"data-dx-motion-order-available"/);
  assert.match(editContract, /"data-dx-motion-preference-storage"/);
  assert.match(editContract, /"data-dx-motion-storage-key"/);
  assert.match(editContract, /"data-dx-motion-keyboard-reorder"/);
  assert.match(editContract, /"data-dx-motion-keyboard-state"/);
  assert.match(editContract, /data-dx-motion-interaction="toggle-reduced-motion"/);
  assert.match(editContract, /data-dx-motion-interaction="move-stage-previous"/);
  assert.match(editContract, /data-dx-motion-interaction="move-stage-next"/);
  assert.match(editContract, /receiptPath: motionDashboardReceiptPath/);
  assert.match(studioManifest, /"motion_interaction_marker": "data-dx-motion-interaction"/);
  assert.match(studioManifest, /studio_motion_edit_surface\(\s*"motion-interaction-proof"/);
  assert.match(studioManifest, /include_policy_status_marker: bool/);
  assert.match(
    studioManifest,
    /studio_motion_edit_surface\(\s*"motion-dashboard-workflow"[\s\S]*"motion-panel-orchestration",\s*false,\s*\)/,
  );
  assert.match(
    studioManifest,
    /studio_motion_edit_surface\(\s*"motion-interaction-proof"[\s\S]*"motion-panel-orchestration",\s*true,\s*\)/,
  );
  assert.match(studioManifest, /data-dx-motion-interaction=\\"move-stage-previous\\"/);
  assert.match(studioManifest, /data-dx-motion-interaction=\\"move-stage-next\\"/);
  assert.match(studioManifest, /data-dx-motion-order-available/);
  assert.match(studioManifest, /data-dx-motion-preference-storage/);
  assert.match(studioManifest, /data-dx-motion-storage-key/);
  assert.match(studioManifest, /data-dx-motion-keyboard-reorder/);
  assert.match(studioManifest, /data-dx-motion-keyboard-state/);
  assert.match(dx, /Motion dashboard usage/);
  assert.match(todo, /Motion & Animation workflow/);
  assert.match(changelog, /visible `animation\/motion` starter-dashboard workflow/);
  assert.ok(
    !fs.existsSync(path.join(root, "examples", "dashboard", "node_modules")),
    "dashboard starter must not add a local node_modules workflow",
  );
});
