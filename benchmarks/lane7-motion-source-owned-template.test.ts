import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

const motionSourceFiles = [
  "examples/template/motion/presets.ts",
  "examples/template/motion/provider.tsx",
  "examples/template/motion/controls.tsx",
  "examples/template/motion/frame.tsx",
  "examples/template/motion/layout.tsx",
  "examples/template/motion/lazy.tsx",
  "examples/template/motion/motion-values.tsx",
  "examples/template/motion/page-visibility.tsx",
  "examples/template/motion/presence.tsx",
  "examples/template/motion/reorder.tsx",
  "examples/template/motion/reveal.tsx",
  "examples/template/motion/scoped-animate.tsx",
  "examples/template/motion/scroll-progress.tsx",
  "examples/template/motion/will-change.tsx",
  "examples/template/motion/dashboard-workflow.ts",
  "examples/template/motion/metadata.ts",
  "examples/template/motion/README.md",
] as const;

function read(relativePath: string) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function exists(relativePath: string) {
  return fs.existsSync(path.join(root, relativePath));
}

function importedMotionModules(source: string) {
  return Array.from(source.matchAll(/from ["']@\/motion\/([^"']+)["']/g), (match) => {
    const moduleName = match[1];
    return `examples/template/motion/${moduleName}`;
  });
}

function assertMotionModuleExists(modulePathWithoutExtension: string) {
  const candidates = [
    `${modulePathWithoutExtension}.tsx`,
    `${modulePathWithoutExtension}.ts`,
    `${modulePathWithoutExtension}.md`,
  ];
  assert.ok(
    candidates.some((candidate) => exists(candidate)),
    `${modulePathWithoutExtension} should resolve to a source-owned Motion file`,
  );
}

test("Motion & Animation materializes the source-owned template modules imported by /launch", () => {
  for (const sourceFile of motionSourceFiles) {
    assert.ok(exists(sourceFile), `${sourceFile} should be materialized`);
  }

  const launchShell = read("examples/template/template-shell.tsx");
  const interactionProof = read("examples/template/motion-interaction-proof.tsx");

  for (const importedModule of [
    ...importedMotionModules(launchShell),
    ...importedMotionModules(interactionProof),
  ]) {
    assertMotionModuleExists(importedModule);
  }

  const provider = read("examples/template/motion/provider.tsx");
  const controls = read("examples/template/motion/controls.tsx");
  const lazy = read("examples/template/motion/lazy.tsx");
  const layout = read("examples/template/motion/layout.tsx");
  const motionValues = read("examples/template/motion/motion-values.tsx");
  const reorder = read("examples/template/motion/reorder.tsx");
  const reveal = read("examples/template/motion/reveal.tsx");
  const workflow = read("examples/template/motion/dashboard-workflow.ts");

  assert.match(provider, /from "motion\/react"/);
  assert.match(provider, /MotionConfig/);
  assert.match(provider, /DxMotionProvider/);
  assert.match(controls, /useAnimationControls/);
  assert.match(controls, /MotionControlledStatus/);
  assert.match(lazy, /LazyMotion/);
  assert.match(lazy, /domAnimation/);
  assert.match(lazy, /domMax/);
  assert.match(lazy, /domMin/);
  assert.match(layout, /LayoutGroup/);
  assert.match(layout, /useInstantLayoutTransition/);
  assert.match(layout, /dxMotionLayoutId/);
  assert.match(motionValues, /useMotionValue/);
  assert.match(motionValues, /useMotionValueEvent/);
  assert.match(motionValues, /MotionValueMeter/);
  assert.match(reorder, /Reorder/);
  assert.match(reorder, /useDragControls/);
  assert.match(reveal, /MotionReveal/);
  assert.match(workflow, /packageId: "animation\/motion"/);
  assert.match(workflow, /motionDashboardPreferenceStorageKey/);

  for (const source of [
    provider,
    controls,
    lazy,
    layout,
    motionValues,
    reorder,
    reveal,
    workflow,
  ]) {
    assert.doesNotMatch(source, /lucide-react/);
    assert.doesNotMatch(source, /fake|dummy|lorem/i);
  }
});

test("Motion & Animation receipt tracks the materialized template modules", () => {
  const helper = read("examples/template/motion-receipt-hashes.ts");
  const receipt = read(
    "examples/template/.dx/forge/receipts/2026-05-22-animation-motion-dashboard-workflow.json",
  );
  const packageStatus = read("examples/template/.dx/forge/package-status.json");
  const readModel = read("examples/template/forge-package-status-read-model.ts");

  assert.match(helper, /MOTION_SOURCE_FILES/);
  assert.match(helper, /motion-source-owned-template-helpers/);

  for (const sourceFile of motionSourceFiles) {
    assert.match(helper, new RegExp(sourceFile.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
    assert.match(receipt, new RegExp(sourceFile.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
    assert.match(
      packageStatus,
      new RegExp(sourceFile.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
    );
    assert.match(readModel, new RegExp(sourceFile.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }
});
