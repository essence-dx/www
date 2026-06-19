# 3D Scene System

official_package_name: 3D Scene System
package_id: 3d/launch-scene
upstream_package: three + @react-three/fiber + @react-three/drei
source_mirror: G:/WWW/inspirations/three.js; G:/WWW/inspirations/react-three-fiber; G:/WWW/inspirations/drei
upstream_version: three 0.184.0; @react-three/fiber 9.6.1; @react-three/drei local mirror
honesty_label: LOCK-BACKED SOURCE-OWNED
hash_algorithm: sha256

3D Scene System is the official DX Forge package lane for source-owned 3D scene surfaces. The current slice uses real public API shapes from Three, React Three Fiber, and Drei as provenance while materializing a Web Preview-safe scene runtime, dashboard workflow, renderer handoff, and optional injected R3F/Drei adapter into front-facing DX-WWW apps.

Launch status: `3d/launch-scene` is now recorded in the Forge source manifest,
package lock, current package-status cache manifests, local cache, package-add
receipt, and safety archive. That is lock-backed source ownership, not runtime
proof: browser WebGL execution, screenshots, installed Three/R3F/Drei renderer
dependencies, shader budgets, and production asset policy remain app-owned.

Add it with `dx add 3d-scene-system --write`. `three-scene`, `three/r3f/drei`, `@react-three/fiber`, `@react-three/drei`, and `spline-like-scene` remain aliases and provenance metadata, not the primary official package name.

Package ID aliases choose the Forge package at the CLI and registry layer. Source
import aliases are physical generated source files inside the project. The
package materializes `three/index.ts`, so apps can import the source-owned scene
slice through `@/three` without changing the WWW resolver. It does not make
`node_modules/three` resolvable by default.

## Inspected Upstream Source

- `three.js/package.json` for package metadata, version, export map, and MIT license context.
- `three.js/src/renderers/WebGLRenderer.js` for `THREE.WebGLRenderer`, `getContextAttributes`, `setPixelRatio`, context capability shape, and readback boundaries.
- `three.js/src/core/Raycaster.js` for `THREE.Raycaster`, `setFromCamera`, `intersectObject`, and `intersectObjects`.
- `three.js/src/math/Box3.js` for `THREE.Box3`, `setFromObject`, `expandByObject`, `getCenter`, and bounding-sphere fit inputs.
- `react-three-fiber/packages/fiber/src/core/renderer.tsx` for `@react-three/fiber createRoot(canvas)`, root configuration, `frameloop`, raycaster setup, and `setDpr`.
- `react-three-fiber/packages/fiber/src/core/store.ts` for `RootState`, `raycaster`, `viewport.dpr`, and render-loop state.
- `react-three-fiber/packages/fiber/src/core/events.ts` and `react-three-fiber/packages/fiber/src/web/Canvas.tsx` for R3F event/raycaster and Canvas setup shape.
- `drei/src/web/KeyboardControls.tsx` for the named keyboard control map pattern.
- `drei/src/core/Bounds.tsx` for `@react-three/drei Bounds.fit` and scene framing API shape.
- `drei/src/core/PerformanceMonitor.tsx`, `drei/src/core/AdaptiveDpr.tsx`, and `drei/src/core/meshBounds.tsx` for performance and raycast helper boundaries.

## Public API Slice

- `createDxLaunchScenePreset`
- `dxSceneQualityProfiles`
- `dxSceneMaterialPalettes`
- `dxSceneDashboardCameraRigs`
- `createDxSceneDashboardWorkflow`
- `createDxSceneDashboardReceipt`
- `cycleDxSceneQualityProfile`
- `cycleDxSceneMaterialPalette`
- `cycleDxSceneCameraRig`
- `captureDxSceneFrameSample`
- `createDxSceneCapabilityReport`
- `createDxSceneViewportReport`
- `createDxSceneBoundsReport`
- `createDxSceneRaycastReport`
- `createDxScenePreviewReadiness`
- `mountDxSceneWithRenderer`
- `createDxSceneR3FDreiRendererAdapter`

## Materialized Surfaces

- `components/scene/launch-scene.tsx`: front-facing React scene boundary used by `/launch`.
- `three/index.ts`: source-owned `@/three` alias barrel that re-exports the materialized scene slice.
- `lib/scene/index.ts`: dependency-free public barrel.
- `lib/scene/preset.ts`: scene graph, camera, quality profiles, and material palettes.
- `lib/scene/interaction.ts`: pointer, raycast-style hit map, and keyboard navigation policy.
- `lib/scene/dashboard-workflow.ts`: dashboard workflow and receipt model.
- `lib/scene/dashboard-controls.ts`: quality, palette, camera, and receipt control helpers.
- `lib/scene/frame-sample.ts`: WebGL/2D frame readback policy.
- `lib/scene/capability-report.ts`: WebGL capability inspection.
- `lib/scene/viewport-report.ts`: viewport and DPR budget reporting.
- `lib/scene/bounds-report.ts`: Three `Box3` / Drei `Bounds.fit`-style framing report.
- `lib/scene/raycast-report.ts`: Three `Raycaster` / R3F `state.raycaster` / Drei `meshBounds`-style hit report.
- `lib/scene/preview-readiness.ts`: Web Preview readiness contract.
- `lib/scene/performance-monitor.ts`: adaptive render-budget monitor.
- `lib/scene/renderer-handoff.ts`: source-owned renderer adapter contract.
- `lib/scene/r3f-renderer-adapter.ts`: injected Three/R3F/Drei adapter boundary.
- `lib/scene/webgl-runtime.ts`: no-install WebGL runtime.
- `lib/scene/metadata.ts`: package metadata, source mirrors, selected surfaces, and app-owned boundaries.

## Dashboard Usage

- Route: `/launch`
- Package marker: `data-dx-package="3d/launch-scene"`
- Component marker: `data-dx-component="launch-scene-webgl-proof"`
- Dashboard marker: `data-dx-component="launch-scene-dashboard-workflow"`
- Workflow marker: `data-dx-dashboard-workflow="scene-visual-ops"`
- DX style marker: `data-dx-style-surface="launch-scene"`
- Token scope marker: `data-dx-token-scope="3d/launch-scene"`
- Zed/DX Studio edit marker: `data-dx-edit-id="launch.scene"`

The generated workflow focuses scene nodes, switches preview/cinematic quality, cycles source-owned material palettes and camera rigs, captures a local canvas frame sample, inspects renderer capabilities, measures viewport DPR, computes a fit target, inspects the focused raycast hit map, resets/regresses render budget, and prepares a local render-budget receipt. It is source/materialization proof only; governed browser runtime proof is still deferred.

## Receipts

- `examples/template/.dx/forge/receipts/3d-launch-scene-dashboard-workflow.json`
- `.dx/forge/receipts/3d-launch-scene.json`
- `.dx/forge/receipts/3d-launch-scene-dashboard-workflow.json`
- `.dx/forge/docs/3d-scene-system.md`

The dashboard workflow receipt carries `files`, `source_files`, and `file_hashes`
for the selected front-facing source surface. Consumers should compare the
recorded SHA-256 values against the current files before rendering `present`;
hash mismatches should render `stale`, absent receipts should render
`missing receipt`, and runtime-only blockers should remain `blocked`.

`examples/template/3d-scene-system-receipt-hashes.ts` owns
`receipt_hash_refresh` for the selected 3D Scene System source files. Run
`node tools/launch/run-template-receipt-helper.js examples/template/3d-scene-system-receipt-hashes.ts --check --json`
to emit `dx.forge.package.receipt_hash_refresh`, and run
`node tools/launch/run-template-receipt-helper.js examples/template/3d-scene-system-receipt-hashes.ts --write` to
refresh the dashboard workflow receipt, `.dx/forge/package-status.json`, and
`examples/template/forge-package-status-read-model.ts`. The helper
publishes `3d-scene-system:receipt-hash-refresh` for Zed/DX Studio and records
`source_guard_runbook_fixture:
docs/packages/3d-scene-system.source-guard-runbook.json` plus the selected
`three-scene-system-source-guard-runbook` surface, so runbook fixture drift is
stale-detectable through the same package-status/read-model freshness path as
the front-facing scene files. It also records
`preview_manifest_materializer:
tools/launch/materialize-www-template.ts` plus the selected
`three-scene-system-preview-manifest-materializer` surface, so generated
`public/preview-manifest.json` fixture-emission drift is stale-detectable
through `3d-scene-system:receipt-hash-refresh` without claiming browser/WebGL
runtime proof. It also records
`runtime_execution: false`, `secret_access: false`, and
`runs_package_install: false`; it does not claim live browser/WebGL proof.

The launch package-status read model also promotes this receipt into the
`package_lane_visibility` collection for **3D Scene System**. That row keeps the
surface hash manifest visible to dx-check/Zed consumers through
`three_scene_system_hash_manifest_present` and
`three_scene_system_hash_mismatch`, without claiming a governed browser/WebGL
runtime pass.

The same receipt and package-status row now expose
`dx.forge.package.dx_style_compatibility` for the visible
`launch-scene-webgl-proof` and `launch-scene-dashboard-workflow` surfaces. The
evidence is the source marker pair `data-dx-style-surface="launch-scene"` and
`data-dx-token-scope="3d/launch-scene"` in
`examples/template/launch-scene.tsx`, plus the generated launch CSS
boundary in `tools/launch/runtime-template/assets/launch-runtime.css`. This
is source-marker and generated-CSS evidence only; browser rendering, WebGL
output, screenshot QA, and scene theme review remain app-owned.

## dx-check Visibility

- `present`: 3D Scene System source files, metadata, docs, receipt, package catalog entry, dx-style markers, and launch dashboard markers are present.
- `stale`: receipt hashes, inspected-source metadata, or materialized scene files no longer match current source.
- `missing receipt`: generated apps lack the 3D Scene System receipt path.
- `blocked`: runtime/browser proof, WebGL access, dependency installation, asset review, or WebXR permission approval has not been granted.
- `unsupported surface`: a requested 3D surface is outside the selected launch-scene, dashboard workflow, WebGL runtime, renderer handoff, and injected R3F/Drei adapter surfaces.

## Rust dx-check output

`core/src/ecosystem/project_check/three_scene_system_dx_check.rs` consumes the launch package-status row for **3D Scene System** and emits `three_scene_system_*` metrics from the receipt-backed visibility state. It uses the shared `core/src/ecosystem/project_check/file_hashes.rs` byte-derived SHA-256 comparator for each selected surface's `file_hashes`, then reports `three-scene-system-stale-receipt`, `three-scene-system-missing-receipt`, `three-scene-system-blocked-surface`, `three-scene-system-unsupported-surface`, and `three-scene-system-hash-mismatch` findings without claiming browser, WebGL, or screenshot proof. Rust dx-check also emits `three_scene_system_receipt_hash_refresh_current`, `three_scene_system_receipt_hash_refresh_stale`, and `three_scene_system_receipt_hash_refresh_missing` from the package-owned helper payload, while keeping `three_scene_system_hash_mismatch` byte-derived from source files. It emits `three_scene_system_dx_style_compatibility_present` and `three_scene_system_dx_style_compatibility_missing`, and raises `three-scene-system-missing-dx-style-compatibility` if the package-status row loses the `dx.forge.package.dx_style_compatibility` source-marker evidence. The package-owned fixtures `three_scene_system_hash_mismatch_metric_and_finding_are_byte_derived`, `three_scene_system_hash_refresh_stale_helper_keeps_source_hash_clean`, and `three_scene_system_dx_style_compatibility_missing_is_reported` cover the hash-drift, stale-helper-only, and missing-style-evidence paths.

## DX Studio/check-panel Row

`core/src/ecosystem/dx_check_receipt.rs` now renders a DX Studio/check-panel 3D Scene System package row from `.dx/forge/package-status.json`. The row keeps **3D Scene System** as the official package name, preserves Three/R3F/Drei provenance, surfaces selected launch-scene markers, and exposes `three_scene_system_hash_manifest_present`, `three_scene_system_hash_mismatch`, `three_scene_system_receipt_hash_refresh_current`, `three_scene_system_receipt_hash_refresh_stale`, `three_scene_system_receipt_hash_refresh_missing`, `three_scene_system_dx_style_compatibility_present`, and `three_scene_system_dx_style_compatibility_missing` beside the receipt status. Stale helper freshness now makes the row stale and points at `node tools/launch/run-template-receipt-helper.js examples/template/3d-scene-system-receipt-hashes.ts --write`; missing dx-style evidence still produces a row-level next action without claiming live browser/WebGL proof.

The static `/launch` runtime fixture also carries a static `/launch` package-lane marker with `data-dx-check-package-lane-row="3d/launch-scene"`, `data-dx-style-surface="launch-scene"`, and `data-dx-token-scope="3d/launch-scene"`. This lets DX Studio and Zed discover the official package lane, provenance, receipt path, and source-style boundary before a fresh dx-check receipt is loaded.

`examples/template/template-shell.tsx`, `examples/template/dx-studio-edit-contract.ts`, `tools/launch/materialize-www-template.ts`, and `dx-www/src/cli/studio_manifest.rs` now include `3d/launch-scene` in the DX Studio package-scoped filter for `dx-check-health-panel`. The Studio package surface uses **3D Scene System** as the front-facing name and indexes the dx-check package-lane row/status/style markers so package-scoped Studio and Zed views can jump from the 3D package lane to the visible check-panel row without treating Three, R3F, or Drei as separate official packages.

The generated-starter materialization guard for 3D Scene System runs the launch runtime materializer into a temporary starter, checks generated static launch HTML for the static `/launch` package-lane marker, and checks generated `public/preview-manifest.json` for the `launch-runtime-dx-check-panel` package scope plus `data-dx-style-surface` and `data-dx-token-scope` state markers. This is lock-backed source-owned materialization proof; it does not claim live browser/WebGL proof.

## Studio Source Guard Runbook

`docs/packages/3d-scene-system.source-guard-runbook.json` is the package-owned
Zed/DX Studio runbook fixture for **3D Scene System**. It publishes
`three-scene-system-lower-dx-check-helper-freshness` with the exact targeted
command
`cargo test -q -p dx-www-compiler three_scene_system_hash_refresh_stale_helper_keeps_source_hash_clean --lib`.
The fixture records the official package name, package id, Three/R3F/Drei
provenance, inspected upstream files, selected surfaces, lock-backed source-owned execution
policy, `three_scene_system_receipt_hash_refresh_stale`,
`three_scene_system_hash_mismatch stays byte-derived`, and
`3d-scene-system:receipt-hash-refresh` without claiming live browser/WebGL
proof. The Studio manifest exposes the same fixture through
`source_guard_index`, `/launch` `source_guard_runbook_index.fixture_paths`,
source-guard contracts, runbook commands, and `/launch` source-guard ids.
The receipt helper tracks this fixture directly and mirrors it as
`three-scene-system-source-guard-runbook`, keeping Zed/DX Studio runbook
metadata editable and hash-backed without widening the generated scene surface.
Generated `public/preview-manifest.json` snapshots now expose the same fixture
through root `sourceGuardRunbookFixtures` plus `/launch`
`routes[].sourceGuardRunbookFixtures`, preserving
`three-scene-system-lower-dx-check-helper-freshness`, `LOCK-BACKED SOURCE-OWNED`,
`runtimeProof: false`, and `3d-scene-system:receipt-hash-refresh` for starter
metadata without claiming live browser/WebGL proof. The helper now hashes
`tools/launch/materialize-www-template.ts` as the selected
`three-scene-system-preview-manifest-materializer` surface, making drift in the
shared generator that emits that metadata visible before generated starters
silently diverge.

## App-Owned Boundaries

- Production Three/R3F/Drei dependency installation.
- 3D asset licensing, mesh composition, texture policy, shader review, and postprocessing.
- WebGL/WebGPU/WebXR permission and device support review.
- Browser performance budgets, accessibility semantics, reduced-motion policy, and screenshot/runtime QA.
- Renderer swaps, external scene authoring tools, and production camera/control behavior.

## Verification

Run the narrow source guard with:

```powershell
dx run --test .\benchmarks\three-scene-package-doc.test.ts
dx run --test .\benchmarks\three-scene-dx-check-output.test.ts
dx run --test .\benchmarks\three-scene-dx-check-package-lane-panel.test.ts
dx run --test .\benchmarks\three-scene-receipt-hash-refresh.test.ts
node tools/launch/run-template-receipt-helper.js examples/template/3d-scene-system-receipt-hashes.ts --check --json
cargo test -q -p dx-www-compiler three_scene_system_hash_mismatch_metric_and_finding_are_byte_derived --lib
cargo test -q -p dx-www-compiler three_scene_system_hash_refresh_stale_helper_keeps_source_hash_clean --lib
cargo test -q -p dx-www-compiler three_scene_system_dx_style_compatibility_missing_is_reported --lib
cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_three_scene_system_package_lane_style_row --lib
dx run --test .\benchmarks\launch-scene-dashboard-workflow.test.ts
dx run --test .\benchmarks\launch-scene-readiness.test.ts
```

These guards are lock-backed source-owned checks only. Do not treat them as live WebGL or browser runtime proof.
