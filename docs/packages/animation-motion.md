# Motion & Animation

Source-owned Forge slice for the local Motion `12.38.0` mirror at `G:\WWW\inspirations\motion`.

## Real API Surface

- `motion/provider.tsx`: `DxMotionProvider` over Motion's real `MotionConfig`.
- `motion/controls.tsx`: `useDxAnimationControls`, raw control aliases, and `MotionControlledStatus` over `useAnimationControls`, `useAnimation`, `animationControls`, and `LegacyAnimationControls`.
- `motion/lazy.tsx`: `DxLazyMotionProvider`, `MotionLazyBox`, and `dxLazyMotion` over `LazyMotion`, `domAnimation`, `domMax`, `domMin`, and `m`.
- `motion/layout.tsx`, `motion/presence.tsx`, and `motion/reorder.tsx`: source-owned dashboard-safe wrappers over `LayoutGroup`, `AnimatePresence`, `Reorder`, `usePresence`, `useIsPresent`, `useInstantLayoutTransition`, and `useDragControls`.
- `motion/motion-values.tsx` and `motion/scroll-progress.tsx`: MotionValue meters and scroll progress over `useMotionValue`, `useSpring`, `useTransform`, `useMotionTemplate`, `useMotionValueEvent`, `useVelocity`, and `useScroll`.
- `motion/dashboard-workflow.ts`: package-owned dashboard workflow metadata, stage definitions, local preference read/write helpers, and `createMotionDashboardReceipt`.

The dashboard receipt records the same upstream API evidence, including `motion`, `m`, `useAnimationControls`, `useAnimation`, `animationControls`, `useAnimationFrame`, `useTime`, `useCycle`, `useWillChange`, `WillChangeMotionValue`, `usePageInView`, `useInstantLayoutTransition`, `useInView`, `useTransform`, `useMotionTemplate`, `useMotionValueEvent`, `useVelocity`, `usePresence`, `useIsPresent`, `useDragControls`, `useSpring`, `AnimationPlaybackControlsWithThen`, `MotionValue`, `Transition`, `Variants`, and the `domAnimation` / `domMax` / `domMin` feature bundles, so package discovery does not under-report the real Motion slice.

## Dashboard Usage

`examples/template/template-shell.tsx` mounts Motion as a real `/launch` dashboard workflow, not a package card:

- `data-dx-package="animation/motion"`
- `data-dx-component="launch-motion-dashboard-workflow"`
- `data-dx-dashboard-workflow="motion-panel-orchestration"`
- `data-dx-product-surface="launch-dashboard"`
- `data-dx-motion-reduced` plus `toggle-reduced-motion`
- `data-dx-motion-preference-storage="local-storage"` and `data-dx-motion-storage-key="dx.launch.motion.dashboard"`
- local stage advance, progress, press feedback, layout reorder controls, button-based move controls, Arrow/Home/End keyboard reorder, local preference persistence, and an app-owned reduced-motion policy preview

The materialized runtime route mirrors the workflow in `the static launch runtime template`, where the Motion controls update the runtime dashboard summary marker `data-dx-component="launch-motion-dashboard-summary"`, persist the local order/reduced-motion preference under `dx.launch.motion.dashboard`, and update the mission-control policy text `#mission-motion-policy`.

DX Studio/Web Preview can map the visible Motion controls back to source through `motion-dashboard-workflow` and `motion-interaction-proof` editable surfaces. The source-owned edit contract records the `advance-stage`, `reverse-order`, `move-stage-previous`, `move-stage-next`, `reset-proof`, and `toggle-reduced-motion` selectors, the shared `data-dx-motion-*` state markers, `data-dx-motion-order-available` actionability state, `data-dx-motion-keyboard-reorder`, `data-dx-motion-keyboard-state`, `data-dx-motion-preference-storage`, `data-dx-motion-storage-key`, and the Motion dashboard receipt path without requiring a template-local `node_modules` workflow. The proof-only `data-dx-motion-policy-status` marker stays scoped to `motion-interaction-proof`.

Source ownership is split intentionally: the outer `launch-motion-dashboard-workflow` selector is owned by `examples/template/template-shell.tsx`, while the nested `motion-interaction-proof` selector is owned by `examples/template/motion-interaction-proof.tsx`.

Forge metadata and the dashboard workflow receipt both advertise these editable surfaces through `dashboardWorkflow.studioSurfaces` / `studio_surfaces` while keeping the older singular `studioSurface` field for compatibility.

`examples/dashboard/src/components/MotionDashboardWorkflow.tsx` consumes the package through a visible workflow with:

- `data-dx-package="animation/motion"`
- `data-dx-component="dashboard-motion-workflow"`
- `data-dx-motion-dashboard-workflow="animated-readiness"`
- `<dx-icon name="pack:motion" />`
- local stage selection, reorder preview, `move-stage-previous`, `move-stage-next`, Arrow/Home/End keyboard reorder markers, local preference markers, `toggle-reduced-motion`, and motion policy receipt creation

The dashboard path has no `node_modules` workflow and does not claim live route choreography. It gives the app a source-owned boundary for deciding which Motion APIs should become real dashboard animation behavior.

## Metadata

- Official DX package name: Motion & Animation
- Package id: `animation/motion`
- Official CLI: `dx add motion-animation --write`
- Aliases: `motion`, `framer-motion`, `motion/react`, `animation/motion`
- Upstream package: `motion`
- Source mirror: `G:\WWW\inspirations\motion`
- Inspected source files: `packages/motion/src/react.ts`, `packages/framer-motion/src/index.ts`, `packages/framer-motion/src/components/AnimatePresence/index.tsx`, `packages/framer-motion/src/components/Reorder/Group.tsx`, and `packages/framer-motion/src/value/use-scroll.ts`
- Selected surfaces: `provider-policy`, `layout-reorder`, and `dashboard-workflow`
- Required env: none
- Receipt paths: `.dx/forge/receipts/packages/animation-motion.json`, `.dx/forge/receipts/20260523T061500000000000Z-animation-motion.json`, `.dx/forge/receipts/safety/animation-motion-archive.json`, `examples/template/.dx/forge/receipts/2026-05-22-animation-motion-dashboard-workflow.json`, `.dx/forge/docs/animation-motion.md`
- Cache manifest: `.dx/forge/cache/animation-motion/12.38.0-dx.12/manifest.json`
- Reality score: 72/100 in the default-template package reality panel; lock/cache proof is present, while governed browser/runtime QA remains app-owned.
- Reality audit: REAL. The dashboard workflow receipt records upstream evidence, Forge files, dashboard consumers, Studio manifests, guard files, and explicit partial runtime boundaries.

## dx-check visibility

The Motion & Animation receipt now carries `dx.forge.package.dx_check_visibility` so DX-WWW, dx-check, and Zed can display package health without guessing. The current source state is `present`; consumers should also understand `stale`, `missing-receipt`, `blocked`, and `unsupported-surface` for materialized-file drift, absent receipts, deferred runtime approval, or unselected Motion surfaces.

The package-status read model now exposes `motionAnimationPackageVisibility` from `.dx/forge/package-status.json`. It records the official package name, package id `animation/motion`, upstream provenance, selected editable surfaces, Zed receipt surface `motion-animation:dashboard-workflow`, and hash-backed dx-check metrics: `motion_animation_receipt_present`, `motion_animation_receipt_stale`, `motion_animation_missing_receipt`, `motion_animation_blocked_surface`, `motion_animation_unsupported_surface`, `motion_animation_hash_manifest_present`, and `motion_animation_hash_mismatch`.

The DX Studio/check-panel Motion package row now renders from that same package-status source. It carries the package-owned `receipt_hash_refresh` helper payload and exposes `motion_animation_receipt_hash_refresh_current`, `motion_animation_receipt_hash_refresh_stale`, and `motion_animation_receipt_hash_refresh_missing` beside `motion_animation_hash_mismatch`, so stale helper state is visible without opening raw JSON and without claiming live Motion browser animation proof.

The stale-helper-only Motion check-panel fixture flips `receipt_hash_refresh.stale_file_count` while keeping the selected source hashes current. That proves the panel marks Motion & Animation stale through `motion_animation_receipt_hash_refresh_stale` and leaves `motion_animation_hash_mismatch` at zero when the helper payload, not the editable Motion source files, needs review.

The static `/launch` Motion & Animation package-lane fixture now exposes the same helper freshness contract before a fresh dx-check receipt exists. `examples/template/template-shell.tsx` and `the static launch runtime template` carry `data-dx-check-package-lane-row="animation/motion"`, official package naming, upstream provenance, the dashboard workflow receipt path, `motion-animation:receipt-hash-refresh`, helper tracked/stale/missing counts, and the `motion_animation_receipt_hash_refresh_current` / stale / missing metric names. This is SOURCE-ONLY Studio/Zed discoverability and still does not claim live Motion browser animation proof.

The generated-starter materialization guard for Motion & Animation runs `tools/launch/materialize-www-template.ts` into a temporary project and proves the same package-lane row survives in generated static launch HTML. It also verifies `public/preview-manifest.json` scopes `/`, `/launch`, and the `launch-runtime-dx-check-panel` editable surface to `animation/motion`, including helper freshness markers, `data-dx-style-surface="motion-animation"`, and `data-dx-token-scope="animation/motion"` without claiming live Motion browser animation proof.

The Motion & Animation Studio source-guard/runbook entry publishes `motion-animation-generated-starter-materialization` through the Zed/DX Studio manifest so operators can rerun `dx run --test .\benchmarks\motion-dx-check-package-lane-panel.test.ts` from the `/launch` source guard index. `docs/packages/motion-animation.source-guard-runbook.json` is the package-owned JSON fixture for that contract, and the Studio manifest now exposes it through structured `fixture_path` metadata on the `source_guard_index`, `/launch` `source_guard_runbook_index.fixture_paths`, runbook contract, and runbook command instead of requiring tools to parse proof strings. The fixture records the `/launch` runbook entry, Studio marker names, receipt helper metadata, app-owned boundaries, and SOURCE-ONLY runtime limitations. This is generated-starter proof for package-lane markers, helper freshness markers, and package-scoped dx-check visibility without claiming live Motion browser animation proof.

The package-owned runbook fixture also self-describes that same structured handoff. It records top-level `fixture_path`, `guard.fixture_path`, `runbook.fixture_paths[]`, `runbook.contract.fixture_path`, and `runbook.command.fixture_path`, so Motion & Animation consumers can load the source-owned fixture path directly from JSON rather than scraping the Studio manifest or prose.

The generated `public/preview-manifest.json` now mirrors that fixture through `sourceGuardRunbookFixtures` at the manifest root and on the `/launch` route, so Zed/DX Studio can discover `docs/packages/motion-animation.source-guard-runbook.json` from generated starter output without executing the Rust Studio manifest or claiming live Motion browser animation proof.

The Motion & Animation dashboard workflow receipt now hashes that runbook fixture as the `motion-source-guard-runbook` surface. `motion-receipt-hashes.ts --check --json` reports `source_guard_runbook_fixture`, consumes `runbook.fixture_paths[]` from the fixture JSON, and mirrors it into package-status/read-model metadata as `source_guard_runbook_fixture_paths` / `sourceGuardRunbookFixturePaths`, making stale Studio runbook metadata visible through `motion-animation:receipt-hash-refresh` without claiming live Motion browser animation proof.

## Receipt Hash Refresh

Motion & Animation has a package-owned helper for reviewed source edits:

```powershell
node tools/launch/run-template-receipt-helper.js examples/template/motion-receipt-hashes.ts --check
node tools/launch/run-template-receipt-helper.js examples/template/motion-receipt-hashes.ts --check --json
node tools/launch/run-template-receipt-helper.js examples/template/motion-receipt-hashes.ts --write
```

The helper validates `package_id: animation/motion`, official package naming, upstream `motion` provenance, `hash_algorithm: sha256`, selected `file_hashes`, package-status mirrors, typed read-model hashes, `source_guard_runbook_fixture`, `source_guard_runbook_fixture_paths`, and generated-starter `examples/template/` path fallback. Its typed read-model writes are scoped to `motionAnimationPackageVisibility`, so another package row that references the same shared launch file cannot hide stale Motion & Animation evidence. It does not run browser animation runtime proof or route choreography, and it does not approve reduced-motion policy, accessibility QA, or animation budgets.

Monitored surfaces:

- `motion-dashboard-workflow`: `present`, mapped from `examples/template/template-shell.tsx` to `components/template-app/template-shell.tsx`.
- `motion-interaction-proof`: `present`, mapped from `examples/template/motion-interaction-proof.tsx` to `components/launch/motion-interaction-proof.tsx`.
- `motion-source-guard-runbook`: `present`, mapped to `docs/packages/motion-animation.source-guard-runbook.json` so fixture drift is stale-detectable.

## Rust dx-check output

`core/src/ecosystem/project_check/motion_animation_dx_check.rs` consumes the Motion & Animation package-status row from `.dx/forge/package-status.json`, resolves `examples/template/.dx/forge/receipts/2026-05-22-animation-motion-dashboard-workflow.json`, and publishes `motion_animation_*` metrics in the Forge section. The Rust output includes `motion_animation_package_present`, receipt present/stale/missing visibility, blocked and unsupported surface counts, and the hash-manifest metrics `motion_animation_hash_manifest_present` / `motion_animation_hash_mismatch`.

Motion & Animation uses the shared `project_check/file_hashes.rs` helper for SHA-256 presence and mismatch checks, so package-status path fallback and `sha256:` digest normalization match the neighboring Forge package lanes instead of carrying a lane-local hasher.

Missing package-status rows, explicit `missing-receipt` visibility states, stale visibility, blocked app-owned runtime proof, unsupported requested surfaces, and missing or stale hash-backed files emit package-specific findings such as `motion-animation-stale-receipt`, `motion-animation-missing-receipt`, and `motion-animation-hash-mismatch` without claiming live browser animation runtime proof. The `motion_animation_hash_mismatch` metric is now byte-derived SHA-256 evidence from selected front-facing Motion files, not just a package-status string. The package metrics path is covered by a temporary package-status fixture that writes a selected Motion file, records its SHA-256, mutates the file, and verifies `motion_animation_receipt_stale`, `motion_animation_hash_mismatch`, and `motion-animation-hash-mismatch` flip together. Runtime proof remains SOURCE-ONLY / ADAPTER-BOUNDARY until governed browser QA, route choreography, accessibility review, and animation budgets are approved.

## App-owned boundaries

- App-wide motion policy and reduced-motion review
- Route transition choreography
- Production preference sync beyond the local dashboard storage key
- Governed browser/accessibility QA for keyboard reorder behavior
- Performance budget for dashboard animation density

## Guard

Run the narrow source guard:

```powershell
dx run --test .\benchmarks\motion-dx-check-output.test.ts
dx run --test .\benchmarks\motion-dx-check-package-lane-panel.test.ts
dx run --test .\benchmarks\motion-receipt-hash-refresh.test.ts
dx run --test .\benchmarks\motion-package-status-read-model.test.ts
node tools/launch/run-template-receipt-helper.js examples/template/motion-receipt-hashes.ts --check --json
dx run --test .\benchmarks\motion-dashboard-workflow.test.ts
dx run --test .\benchmarks\motion-launch-proof.test.ts
dx run --test .\benchmarks\motion-runtime-interaction.test.ts
cargo test -q -p dx-www-compiler motion_animation_package_metrics_report_byte_derived_hash_mismatch --lib
cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_motion_animation_package_lane_hash_refresh_row --lib
```
