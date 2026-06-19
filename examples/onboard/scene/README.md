# DX WWW Launch Scene

This is the Web Preview-safe scene adapter for the launch template. It proves
the 3D boundary without installing a local `node_modules` folder or forcing
React Three Fiber into every starter app.

## Owned files

- `index.ts` is the consumer entrypoint for Forge package users who want the
  preset, runtime, readiness helpers, metadata, and types without importing
  individual internal files.
- `preset.ts` owns the scene graph, colors, camera hints, and motion policy.
- `interaction.ts` owns procedural node hit testing and interaction events.
- `dashboard-controls.ts` owns dashboard control cycling and local receipt
  summary formatting for the visible `/launch` scene workflow.
- `frame-sample.ts` owns canvas/WebGL frame readback for the visible dashboard
  sample action.
- `capability-report.ts` owns WebGL context capability inspection for the
  visible dashboard renderer-readiness action.
- `viewport-report.ts` owns viewport and DPR budget reporting for the visible
  dashboard render-size action.
- `bounds-report.ts` owns source-owned scene bounds and fit-target reporting
  for the visible dashboard framing action.
- `raycast-report.ts` owns source-owned scene hit testing in the shape of the
  Three `Raycaster` / R3F `state.raycaster` / Drei `meshBounds` workflow.
- `preview-readiness.ts` owns the no-install readiness report for Web Preview.
- `performance-monitor.ts` owns adaptive DPR and shader-detail budget sampling.
- `renderer-handoff.ts` owns the renderer adapter contract used by the launch
  template.
- `r3f-renderer-adapter.ts` owns the injected Three/R3F/Drei adapter boundary
  for apps that provide those runtime packages.
- `webgl-runtime.ts` owns the no-install renderer and render-budget policy used
  by Web Preview.
- `metadata.ts` records the future Forge package handoff and app-owned runtime
  boundaries.
- `launch-scene.tsx` is the React boundary consumed by the template shell.

## Consumer entrypoint

```ts
import {
  createDxLaunchScenePreset,
  createDxSceneRendererHandoff,
  mountDxSceneWithRenderer,
} from "@/lib/scene";
import type { DxLaunchScenePreset } from "@/lib/scene";
```

The barrel stays dependency-free. It re-exports the local preset, direct WebGL
runtime, renderer handoff, render-budget helper, Web Preview readiness helpers,
package metadata, and typed scene contract without importing Three, React Three
Fiber, or Drei.

## Dashboard workflow

`dashboard-workflow.ts` turns the scene graph into a launch dashboard workflow,
while `dashboard-controls.ts` owns the small control helpers consumed by the
React surface. They export `createDxSceneDashboardWorkflow`,
`createDxSceneDashboardReceipt`, `dxSceneDashboardWorkflowProfiles`,
`cycleDxSceneQualityProfile`, `cycleDxSceneMaterialPalette`,
`cycleDxSceneCameraRig`, `captureDxSceneFrameSample`, and
`createDxSceneCapabilityReport`, `createDxSceneViewportReport`,
`createDxSceneBoundsReport`, `createDxSceneRaycastReport`, and
`formatDxSceneDashboardReceiptSummary` for
runtime health, render budget, frame readback, renderer capabilities, viewport
DPR budgets, bounds fitting, Raycaster-style hit reporting, and Web Preview readiness. The generated
`/launch` route uses those APIs through the React `LaunchScene` boundary and
the dependency-free runtime page. Stable selectors include
`data-dx-package="3d/launch-scene"`,
`data-dx-component="launch-scene-webgl-proof"`,
`data-dx-component="launch-scene-dashboard-workflow"`,
`data-dx-dashboard-workflow="scene-visual-ops"`,
`data-dx-scene-action="focus-dashboard-node"`, and
`data-dx-scene-action="apply-render-budget"`. The same workflow now exposes
`data-dx-scene-quality-profile` and
`data-dx-scene-action="toggle-quality-profile"` so `/launch` can switch between
the package-owned `preview` and `cinematic` render budgets before preparing a
local `scene-render-budget-*` receipt state. It exposes
`data-dx-scene-action="cycle-material-palette"` and
`data-dx-scene-material-palette` so `/launch` can switch between the
source-owned Aurora and Graphite material palettes and update the WebGL/2D
scene colors without importing external assets. It exposes
`dxSceneDashboardCameraRigs`,
`resolveDxSceneDashboardCameraRig`,
`data-dx-scene-action="cycle-camera-rig"`, and
`data-dx-scene-camera-rig` so the generated route can cycle source-owned
Orbit/Isometric/Inspection camera presets that affect the rendered WebGL view
without importing Drei at runtime. The workflow also exposes
`data-dx-scene-action="capture-frame-sample"` and
`data-dx-scene-frame-sample` so the generated route can capture a safe local
canvas sample through `captureDxSceneFrameSample`, using WebGL `readPixels`
when available and a 2D `getImageData` fallback where the browser exposes one.
It exposes `data-dx-scene-action="inspect-renderer-capabilities"`,
`data-dx-scene-capability-report`, and
`data-dx-scene-capability-status` so `/launch` can read the browser WebGL
context attributes, supported extensions, texture limits, and renderbuffer
limits through `createDxSceneCapabilityReport` without importing Three at
runtime. It also exposes `data-dx-scene-action="measure-viewport-dpr"`,
`data-dx-scene-viewport-report`, and `data-dx-scene-viewport-status` so the
dashboard can compare CSS size, drawing-buffer size, device pixel ratio, and
the effective quality/performance pixel-ratio budget through
`createDxSceneViewportReport`. It also exposes
`data-dx-scene-action="fit-scene-bounds"`, `data-dx-scene-bounds-report`, and
`data-dx-scene-bounds-status` so the dashboard can compute a Three `Box3` /
Drei `Bounds.fit`-style fit target, scene size, and camera distance from the
source-owned procedural scene through `createDxSceneBoundsReport`. It also
exposes `data-dx-scene-action="inspect-raycast-hit"`,
`data-dx-scene-raycast-report`, and `data-dx-scene-raycast-status` so the
dashboard can inspect the focused node hit map through
`createDxSceneRaycastReport` without importing Three/R3F/Drei at runtime.
Production asset licensing, shader budgets, full
Three/R3F/Drei package installation, and XR permissions remain app-owned.

## Provenance and ownership

The package metadata records the local source mirrors used for the public API
shape: `G:/WWW/inspirations/three.js`, `G:/WWW/inspirations/react-three-fiber`,
and `G:/WWW/inspirations/drei`. The inspected upstream surface is
`WebGLRenderer`, `Box3`, `Raycaster`, `Scene`, `PerspectiveCamera`, and `ShaderMaterial` from Three,
`Canvas/createRoot` plus frameloop/root-state shape from React Three Fiber, and
`KeyboardControls`, `PerformanceMonitor`, `AdaptiveDpr`, `Bounds`, and
`meshBounds` from Drei. DX WWW
owns the code in this slice, including the Web Preview renderer, scene preset,
interaction bridge, performance budget, renderer handoff, metadata, and React
surface. Applications still own dependency installation, asset licensing, full
mesh composition, postprocessing, XR permissions, and production renderer swaps.

## Renderer handoff

The `renderer-handoff.ts` module exposes
`createDxSceneWebGLRendererAdapter`, `createDxSceneRendererHandoff`, and
`mountDxSceneWithRenderer`. The default adapter mounts the source-owned WebGL
runtime, but the handoff object records the package id, scene preset, renderer
capabilities, and app-owned boundaries that a future Three/R3F/Drei renderer
adapter must satisfy. This keeps Web Preview useful today while giving Forge a
real public swap point instead of a wrapper that only renames the current
runtime. Full Three renderer ownership, R3F root configuration, Drei controls,
asset loaders, XR, and postprocessing remain app-owned until the Forge package
ships those implementations directly.

## Injected Three/R3F/Drei adapter

`r3f-renderer-adapter.ts` provides `createDxSceneR3FDreiRendererAdapter` for
apps that already own `three`, `@react-three/fiber`, and `@react-three/drei`.
It does not import those packages. Instead, the app injects the upstream-shaped
`createRoot(canvas)` function and a scene-element factory. The adapter configures
the root with `frameloop`, `events`, and `onCreated`, renders the injected scene
element, unmounts through the R3F root, and exposes the same DX controller shape
used by Web Preview. The helper also exports `createDxSceneDreiKeyboardMap`,
which maps the DX keyboard actions to Drei `KeyboardControls` entries. Real
mesh composition, React element creation, Suspense, loaders, controls,
postprocessing, and package installation remain app-owned.

## Frame callback bridge

The runtime exposes an `onFrame` callback bridge inspired by React Three
Fiber's `useFrame`, but implemented as source-owned WebGL adapter state. The
callback receives elapsed time, delta time, motion mode, pointer parallax,
viewport size, and the active preset. Apps can use it for lightweight launch
telemetry or renderer handoff checks without installing Three/R3F/Drei. Scene
composition, assets, controls, loaders, and production renderer swaps remain
app-owned.

## Node interaction bridge

The `interaction.ts` module exposes `createDxSceneInteractionMap`,
`pickDxSceneNode`, and `createDxSceneNodeInteraction` for R3F-style pointer
events on the source-owned procedural scene. The WebGL runtime uses those
helpers to publish `onNodeHover` and `onNodeSelect` callbacks with node ids,
hit anchors, normalized pointer coordinates, and miss states. This gives Web
Preview a real object interaction contract without copying Three's raycaster or
R3F's event manager. Full mesh raycasting, pointer capture, event bubbling,
selection boxes, and Spline-authored object graphs remain app-owned.

## Keyboard node navigation

The interaction module also exports `dxSceneKeyboardBindings`,
`resolveDxSceneKeyboardAction`, and `createDxSceneKeyboardInteraction`, a
source-owned counterpart to the named key map pattern used by Drei
KeyboardControls. The WebGL runtime listens for focused canvas `keydown` events
and maps arrow keys to previous/next node focus, Enter/Space to node selection,
and Escape to a miss state. The launch template enables this with a focusable
canvas and routes keyboard navigation through the same `onNodeHover` and
`onNodeSelect` callbacks as pointer interaction. Application-level shortcuts,
focus-ring styling, command palette integration, and full R3F event stores
remain app-owned.

## Node geometry uniforms

Preset node `radius`, `orbit`, and `opacity` values are uploaded as shader
uniforms and drive the source-owned core, halo, ring, grid, and spark passes.
The public `createDxSceneNodeUniforms` helper exposes that runtime contract for
tests, Web Preview checks, and future Forge package handoff work. Mesh loading,
material replacement, scene authoring tools, and a full R3F/Drei renderer swap
remain intentionally app-owned.

## Camera uniforms

Preset camera `position`, `target`, and `focalLength` values are normalized by
`createDxSceneCameraUniforms` and uploaded to the shader as camera uniforms.
The direct WebGL renderer uses those values for lens scale, depth scale, and
camera drift so the exported camera contract affects Web Preview rendering.
Projection matrices, orbit controls, XR camera state, and full Three/R3F/Drei
camera replacement remain app-owned.

## Material palettes

The `preset.ts` module exports `dxSceneMaterialPalettes` with source-owned
`aurora` and `graphite` palettes for background, lighting, node colors, and
shader material uniforms. `createDxLaunchScenePreset({ palette })` resolves
those palettes and feeds `contrast`, `bloom`, and `sheen` into the WebGL shader
through `createDxSceneMaterialUniforms`. This is the Forge-owned look contract
for Web Preview; imported texture assets, physically based material stacks,
tone-mapping policy, and brand-specific palette authoring remain app-owned.

## Quality profiles

The `preset.ts` module exports `dxSceneQualityProfiles` with `preview` and
`cinematic` render budgets. `createDxLaunchScenePreset({ quality })` resolves
the active profile and `createDxSceneRenderBudget` applies its pixel-ratio cap,
frame intervals, antialiasing flag, and WebGL power preference. The runtime also
uploads `createDxSceneQualityUniforms(scene)` as shader detail, so quality
changes affect both browser cost and visual density. Full performance monitors,
adaptive DPR regressions, custom renderer loops, WebXR, and app-specific
profiling remain intentionally app-owned.

## Performance monitor

The `performance-monitor.ts` module provides a source-owned adaptive budget
inspired by React Three Fiber performance regression and Drei performance
monitoring patterns. `createDxScenePerformanceMonitor(scene)` samples frame
delta values from the WebGL runtime, tracks a performance factor, and emits a
budget that lowers device-pixel-ratio and shader detail when frame timing drops.
`regress()` is available for future controls or route transitions that need an
immediate low-cost frame path. The monitor does not copy R3F or Drei runtime
code; renderer stores, Suspense integration, control regress hooks, WebXR, and
full adaptive postprocessing remain app-owned.

## Performance regression controls

The WebGL controller exposes `regressPerformance(reason)` and
`resetPerformance()` so apps can intentionally drop to a low-cost render budget
during pointer-heavy interaction or route transitions. The launch scene enables
`regressOnPointerMove`, which mirrors the way Drei controls call performance
regression during control changes while keeping the implementation DX-owned and
dependency-free. The public `reason` values are `interaction`,
`route-transition`, and `manual`; app routing policy, transition timing, and
renderer-store integration remain app-owned.

## License boundary

The adapter is source-owned DX WWW code. The metadata file records the package
mirrors that inspired the future Forge package path: Three, React Three Fiber,
Drei, and XR. The package does not copy upstream implementation code or bundled
assets. Apps can replace this runtime with a Forge-owned 3D package later
without changing the `LaunchScene` component boundary.

## Web Preview behavior

The runtime uses browser WebGL directly, respects reduced-motion settings, and
falls back to the visible launch shell if WebGL is unavailable. It does not run
package installs, local servers, or heavy builds.
