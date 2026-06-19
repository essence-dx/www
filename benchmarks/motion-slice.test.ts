const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function forgeRegistrySource() {
  return [
    "core/src/ecosystem/forge_registry.rs",
    "core/src/ecosystem/forge_motion.rs",
    "core/src/ecosystem/forge_registry_parts/registry_operations.rs",
    "core/src/ecosystem/forge_registry_parts/package_lanes.rs",
  ]
    .map(read)
    .join("\n");
}

function cliSource() {
  return [
    "dx-www/src/cli/mod.rs",
    "dx-www/src/cli/help_text.rs",
    "dx-www/src/cli/mod_parts/next_familiar_template.rs",
    "dx-www/src/cli/tests/part_04.rs",
  ]
    .map(read)
    .join("\n");
}

test("Motion Forge slice exposes provider, controls, page visibility, frame timing, will-change hints, lazy features, motion values, scoped feedback, presence, layout, reorder, and scroll progress for the launch shell", () => {
  const motionSlice = read("core/src/ecosystem/forge_motion.rs");
  const registry = forgeRegistrySource();
  const cli = cliSource();
  const shell = read("examples/onboard/template-shell.tsx");
  const catalog = read("examples/onboard/package-catalog.ts");
  const motionTemplateList =
    motionSlice.match(/pub\(super\) fn motion_templates\(\)[\s\S]*?\n\}/)?.[0] ??
    "";

  assert.match(motionSlice, /MOTION_VERSION: &str = "12\.38\.0-dx\.12"/);
  assert.match(motionSlice, /"js\/motion\/controls\.tsx"/);
  assert.match(motionSlice, /"js\/motion\/frame\.tsx"/);
  assert.match(motionSlice, /"js\/motion\/layout\.tsx"/);
  assert.match(motionSlice, /"js\/motion\/lazy\.tsx"/);
  assert.match(motionSlice, /"js\/motion\/motion-values\.tsx"/);
  assert.match(motionSlice, /"js\/motion\/page-visibility\.tsx"/);
  assert.match(motionSlice, /"js\/motion\/provider\.tsx"/);
  assert.match(motionSlice, /"js\/motion\/presence\.tsx"/);
  assert.match(motionSlice, /"js\/motion\/reorder\.tsx"/);
  assert.match(motionSlice, /"js\/motion\/scoped-animate\.tsx"/);
  assert.match(motionSlice, /"js\/motion\/scroll-progress\.tsx"/);
  assert.match(motionSlice, /"js\/motion\/will-change\.tsx"/);
  assert.match(motionSlice, /"js\/motion\/dashboard-workflow\.ts"/);
  assert.match(motionSlice, /MotionConfig/);
  assert.match(motionSlice, /MotionConfigProps/);
  assert.match(motionSlice, /dxMotionConfigDefaults/);
  assert.match(motionSlice, /DxMotionProvider/);
  assert.match(motionSlice, /reducedMotion: "user"/);
  assert.match(motionSlice, /skipAnimations = false/);
  assert.match(motionSlice, /useAnimationControls/);
  assert.match(motionSlice, /useAnimation/);
  assert.match(motionSlice, /animationControls/);
  assert.match(motionSlice, /LegacyAnimationControls/);
  assert.match(motionSlice, /AnimationDefinition/);
  assert.match(motionSlice, /dxAnimationControlTargets/);
  assert.match(motionSlice, /useDxAnimationControls/);
  assert.match(motionSlice, /useDxRawAnimationControls/);
  assert.match(motionSlice, /useDxLegacyAnimation/);
  assert.match(motionSlice, /MotionControlledStatus/);
  assert.match(motionSlice, /createDxAnimationControls/);
  assert.match(motionSlice, /data-dx-motion="controlled-status"/);
  assert.match(motionSlice, /useAnimationFrame/);
  assert.match(motionSlice, /useTime/);
  assert.match(motionSlice, /useCycle/);
  assert.match(motionSlice, /CycleState/);
  assert.match(motionSlice, /dxMotionFramePhases/);
  assert.match(motionSlice, /useDxFrameClock/);
  assert.match(motionSlice, /MotionFrameTicker/);
  assert.match(motionSlice, /data-dx-motion="frame-ticker"/);
  assert.match(motionSlice, /useWillChange/);
  assert.match(motionSlice, /WillChangeMotionValue/);
  assert.match(motionSlice, /dxWillChangeProperties/);
  assert.match(motionSlice, /createDxWillChangeValue/);
  assert.match(motionSlice, /useDxWillChange/);
  assert.match(motionSlice, /MotionWillChangeBox/);
  assert.match(motionSlice, /data-dx-motion="will-change-box"/);
  assert.match(motionSlice, /AnimatePresence/);
  assert.match(motionSlice, /AnimatePresenceProps/);
  assert.match(motionSlice, /LayoutGroup/);
  assert.match(motionSlice, /usePresence/);
  assert.match(motionSlice, /useIsPresent/);
  assert.match(motionSlice, /dxMotionPresenceDefaults/);
  assert.match(motionSlice, /DxMotionPresence/);
  assert.match(motionSlice, /MotionPresenceItem/);
  assert.match(motionSlice, /mode: "popLayout"/);
  assert.match(motionSlice, /useInstantLayoutTransition/);
  assert.match(motionSlice, /DxMotionLayoutGroup/);
  assert.match(motionSlice, /MotionLayoutItem/);
  assert.match(motionSlice, /useDxInstantLayoutTransition/);
  assert.match(motionSlice, /dxMotionLayoutId/);
  assert.match(motionSlice, /layoutDependency/);
  assert.match(motionSlice, /layoutRoot/);
  assert.match(motionSlice, /LazyMotion/);
  assert.match(motionSlice, /domAnimation/);
  assert.match(motionSlice, /domMax/);
  assert.match(motionSlice, /domMin/);
  assert.match(motionSlice, /DxLazyMotionProvider/);
  assert.match(motionSlice, /MotionLazyBox/);
  assert.match(motionSlice, /dxLazyMotionFeatures/);
  assert.match(motionSlice, /dxLazyMotion = m/);
  assert.match(motionSlice, /data-dx-motion="lazy-motion-box"/);
  assert.match(motionSlice, /useMotionValue/);
  assert.match(motionSlice, /useTransform/);
  assert.match(motionSlice, /useMotionTemplate/);
  assert.match(motionSlice, /useMotionValueEvent/);
  assert.match(motionSlice, /useVelocity/);
  assert.match(motionSlice, /useDxMotionValueMeter/);
  assert.match(motionSlice, /MotionValueMeter/);
  assert.match(motionSlice, /data-dx-motion="motion-value-meter"/);
  assert.match(motionSlice, /data-dx-motion="motion-value-meter-indicator"/);
  assert.match(motionSlice, /usePageInView/);
  assert.match(motionSlice, /useDxPageVisibility/);
  assert.match(motionSlice, /MotionPageVisibilityBadge/);
  assert.match(motionSlice, /data-dx-motion="page-visibility-badge"/);
  assert.match(motionSlice, /Reorder/);
  assert.match(motionSlice, /useDragControls/);
  assert.match(motionSlice, /DragControls/);
  assert.match(motionSlice, /DxReorderGroup/);
  assert.match(motionSlice, /DxReorderItem/);
  assert.match(motionSlice, /useDxReorderControls/);
  assert.match(motionSlice, /dxMotionReorderDefaults/);
  assert.match(motionSlice, /dragListener: false/);
  assert.match(motionSlice, /useAnimate/);
  assert.match(motionSlice, /AnimationPlaybackControlsWithThen/);
  assert.match(motionSlice, /useDxScopedAnimate/);
  assert.match(motionSlice, /useDxMotionPressFeedback/);
  assert.match(motionSlice, /useScroll/);
  assert.match(motionSlice, /useSpring/);
  assert.match(motionSlice, /MotionScrollProgress/);
  assert.match(motionSlice, /useDxScrollProgress/);
  assert.match(motionSlice, /data-dx-motion="scroll-progress"/);
  assert.match(motionSlice, /data-dx-motion="pressable"/);
  assert.match(motionSlice, /useAnimationControls, useAnimation, animationControls, and LegacyAnimationControls/);
  assert.match(motionSlice, /useAnimationFrame, useTime, and useCycle frame timing/);
  assert.match(motionSlice, /useWillChange and WillChangeMotionValue performance hints/);
  assert.match(motionSlice, /useAnimate, scoped selector animation/);
  assert.match(motionSlice, /LayoutGroup, layoutId, layoutDependency, and useInstantLayoutTransition/);
  assert.match(motionSlice, /LazyMotion, domAnimation, domMax, domMin, and m feature bundles/);
  assert.match(motionSlice, /useMotionValue, useTransform, useMotionTemplate, useMotionValueEvent, and useVelocity/);
  assert.match(motionSlice, /usePageInView document visibility hook/);
  assert.match(motionSlice, /AnimatePresence, LayoutGroup, and presence hooks/);
  assert.match(motionSlice, /Reorder and useDragControls drag sorting/);
  assert.match(motionSlice, /useScroll, useSpring, and MotionValue scroll progress/);
  assert.match(motionSlice, /MotionConfig provider defaults/);
  assert.match(motionSlice, /dashboardWorkflow/);
  assert.match(motionSlice, /dxAdd: "dx add motion-animation --write"/);
  assert.doesNotMatch(motionSlice, /dxAdd: "dx add motion\/react --write"/);
  assert.match(motionSlice, /launch-motion-dashboard-summary/);
  assert.match(motionSlice, /motion-panel-orchestration/);
  assert.match(motionSlice, /launch-dashboard/);
  assert.match(motionSlice, /motion-animation-card/);
  assert.match(motionSlice, /data-dx-motion-reduced/);
  assert.match(motionSlice, /toggle-reduced-motion/);
  assert.match(motionSlice, /advance-stage/);
  assert.match(motionSlice, /reverse-order/);
  assert.match(motionSlice, /reset-proof/);
  assert.match(motionSlice, /--dx-motion-active-border/);
  assert.match(motionSlice, /--dx-motion-active-bg/);
  assert.match(motionSlice, /--dx-motion-progress-track/);
  assert.match(motionSlice, /motion-launch-materialized\.test\.ts/);
  assert.match(motionSlice, /motion-runtime-interaction\.test\.ts/);
  assert.match(motionSlice, /dx-motion-scroll-progress/);
  assert.match(motionSlice, /dx-motion-meter-indicator/);
  assert.match(motionSlice, /dx-motion-meter-track/);
  assert.doesNotMatch(motionSlice, /bg-emerald-400/);
  assert.doesNotMatch(motionSlice, /bg-neutral-800/);
  assert.doesNotMatch(motionSlice, /rgba\(52, 211, 153/);

  assert.match(registry, /"animation\/motion" => build_registry_package/);
  assert.match(registry, /"motion-animation"[\s\S]*"motion-and-animation"/);
  assert.match(registry, /MOTION_VERSION/);
  assert.match(registry, /useAnimationControls\/useAnimation imperative controls/);
  assert.match(registry, /motion\/controls\.tsx/);
  assert.match(registry, /useAnimationFrame\/useTime frame timing/);
  assert.match(registry, /motion\/frame\.tsx/);
  assert.match(registry, /useWillChange\/WillChangeMotionValue performance hints/);
  assert.match(registry, /motion\/will-change\.tsx/);
  assert.match(registry, /usePageInView document visibility/);
  assert.match(registry, /motion\/page-visibility\.tsx/);
  assert.match(registry, /MOTION_VERSION: &str = "12\.38\.0-dx\.12"/);
  assert.match(registry, /motion\/dashboard-workflow\.ts/);
  assert.equal((motionTemplateList.match(/"js\/motion\//g) ?? []).length, 17);

  assert.match(shell, /DxMotionProvider/);
  assert.match(shell, /MotionControlledStatus/);
  assert.match(shell, /MotionFrameTicker/);
  assert.match(shell, /MotionWillChangeBox/);
  assert.match(shell, /DxLazyMotionProvider/);
  assert.match(shell, /MotionLazyBox/);
  assert.match(shell, /DxMotionPresence/);
  assert.match(shell, /MotionPresenceItem/);
  assert.match(shell, /DxMotionLayoutGroup/);
  assert.match(shell, /MotionLayoutItem/);
  assert.match(shell, /useDxInstantLayoutTransition/);
  assert.match(shell, /dxMotionLayoutId/);
  assert.match(shell, /MotionValueMeter/);
  assert.match(shell, /MotionPageVisibilityBadge/);
  assert.match(shell, /DxReorderGroup/);
  assert.match(shell, /DxReorderItem/);
  assert.match(shell, /useDxReorderControls/);
  assert.match(shell, /orderedPackageIds/);
  assert.match(shell, /data-dx-motion="launch-package-reorder"/);
  assert.match(shell, /data-dx-motion="launch-package-layout"/);
  assert.match(shell, /data-dx-motion="launch-package-drag-handle"/);
  assert.match(shell, /layoutGroupId="launch-package-catalog"/);
  assert.match(shell, /reducedMotion="user"/);
  assert.match(shell, /MotionScrollProgress/);
  assert.match(shell, /useDxMotionPressFeedback/);
  assert.match(shell, /motionPress\.scope/);
  assert.match(shell, /motionPress\.press\(event\.currentTarget\)/);
  assert.match(shell, /data-dx-motion="launch-scroll-progress"/);
  assert.match(shell, /data-dx-motion="launch-controlled-status"/);
  assert.match(shell, /data-dx-motion="launch-frame-ticker"/);
  assert.match(shell, /data-dx-motion="launch-page-visibility"/);
  assert.match(shell, /data-dx-motion="launch-will-change-metric"/);
  assert.match(shell, /data-dx-motion="launch-lazy-capability-row"/);
  assert.match(shell, /data-dx-motion="launch-package-meter"/);
  assert.match(shell, /data-dx-motion="launch-role-meter"/);
  assert.match(shell, /data-dx-motion="pressable"/);
  assert.match(shell, /function LaunchMotionDashboardWorkflow/);
  assert.match(shell, /data-dx-component="launch-motion-dashboard-workflow"/);
  assert.match(shell, /data-dx-dashboard-workflow="motion-panel-orchestration"/);
  assert.match(shell, /data-dx-product-surface="launch-dashboard"/);
  assert.doesNotMatch(shell, /data-dx-component="motion-package-proof"/);

  assert.match(catalog, /packageId: "animation\/motion"/);
  assert.match(catalog, /officialPackageName: "Motion & Animation"/);
  assert.match(catalog, /command: "dx add motion-animation --write"/);
  assert.doesNotMatch(catalog, /command: "dx add motion\/react --write"/);
  assert.match(catalog, /upstreamPackage: "motion"/);
  assert.match(catalog, /inspectedSourceFiles: \[/);
  assert.match(catalog, /selectedSurfaces: \[/);
  assert.match(catalog, /dxCheckVisibility: \{/);
  assert.match(catalog, /currentStatus: "present"/);
  assert.match(catalog, /imperative animation sequencing/);
  assert.match(catalog, /page visibility policy/);
  assert.match(catalog, /frame sampling policy/);
  assert.match(catalog, /will-change performance hints/);

  assert.match(cli, /"command": "dx add motion-animation --write"/);
  assert.match(cli, /package\["cli_add"\] == "dx add motion-animation --write"/);
  assert.match(cli, /dx add motion-animation\s+Add editable Motion & Animation files/);
  assert.match(cli, /dx forge add motion-animation --project \. --dry-run/);
  assert.doesNotMatch(cli, /dx add motion\/react --write/);
  assert.doesNotMatch(cli, /forge add motion\/react/);
});
