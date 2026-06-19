# WebAssembly Bridge Forge Package

Official DX package name: `WebAssembly Bridge`

Package id: `wasm/bindgen`

Official command: `dx add webassembly-bridge --write`

Upstream package: `wasm-bindgen`

Upstream version: `0.2.121`

Honesty label: `ADAPTER-BOUNDARY`

Source mirror: `G:\WWW\inspirations\wasm-bindgen`

dx-check visibility: `present`, `stale`, `missing-receipt`, `blocked`, `unsupported-surface`

Shared package-status read model: `examples/template/forge-package-status-read-model.ts` now exposes `webAssemblyBridgePackageVisibility` for `wasm/bindgen`, with `webassembly_bridge_*` dx-check metrics, selected surface source markers, and the Zed receipt surface `wasm-bindgen-dashboard-workflow`.

## What is real

- Source-owned loader APIs for generated wasm-bindgen `default init(input)`, `initSync(input)`, cache management, memory views, allocator helpers, externref helpers, exception storage, closure cleanup, and Response MIME diagnostics.
- Source-owned React hook APIs for loading, preloading, reloading, clearing, cancelling, resetting, and inspecting generated wasm-bindgen modules.
- A generated dashboard workflow component at `wasm/bindgen/dashboard-workflow.tsx`.
- A starter dashboard workflow at `examples/dashboard/src/components/WasmBindgenWorkflow.tsx`.
- A launch route workflow at `examples/template/wasm-interop-status.tsx`.
- Launch package catalog discovery in `examples/template/package-catalog.ts`.
- Real `/launch` operating dashboard placement through `data-dx-component="launch-wasm-compute-dashboard-workflow"` and the mission-control local compute metric.
- Template discovery metadata for `LaunchWasmInteropStatus`, `wasm-compute-dashboard-workflow`, `pack:wasm-bindgen`, and no-env local compute readiness.

## Visible dashboard workflow

The starter dashboard imports `WasmBindgenWorkflow` and exposes:

- `data-dx-package="wasm/bindgen"`
- `data-dx-component="dashboard-wasm-bindgen-workflow"`
- `data-dx-dashboard-workflow="wasm-interop"`
- `data-dx-wasm-dashboard-action="run-local-add"`
- `data-dx-wasm-add-result`
- `<dx-icon name="pack:wasm-bindgen" />`

The safe local action runs `WebAssembly.instantiate(localAddWasmBytes)` and returns an add result. The generated-module path shows an honest missing-config receipt until the app owns a Rust crate, a `.wasm` artifact, and generated JavaScript glue.

## Launch dashboard workflow

The `/launch` route mounts the same WebAssembly action inside the operating dashboard instead of leaving it as a package card:

- `data-dx-component="launch-wasm-compute-dashboard-workflow"`
- `data-dx-dashboard-card="local-compute"`
- `data-dx-dashboard-workflow="local-compute-readiness"`
- `data-dx-dashboard-metric="wasm-compute"`
- `data-dx-product-surface="launch-dashboard"`

DX Studio edit contract:

- Surface id: `wasm-compute-dashboard-workflow`
- Selector: `data-dx-component="launch-wasm-compute-dashboard-workflow"`
- Operations: move/reorder section, update text/content, insert icon/media
- Policy: responsive design-system grid, no absolute positioning, no template-local `node_modules`
- Materialized Zed preview surface: `launch-runtime-wasm-compute-dashboard`
- Discovery owners: package catalog, generated package metadata, launch route contract, dashboard receipt
- Generated starter receipt path: `.dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json`
- Starter dashboard metadata also records the same preview surface so DX Studio can trace from `WasmBindgenWorkflow` to the generated `/launch` local-compute section.

The runtime bridge updates the mission-control WASM metric after the local add action succeeds or fails. It does not claim that an app-owned Rust crate or generated wasm-bindgen module exists.

## App-owned boundaries

- Rust crate source and `#[wasm_bindgen]` export design.
- `wasm32-unknown-unknown` build profile and generated `.wasm` artifact.
- wasm-bindgen CLI install, version pin, target selection, and output directory.
- Generated JavaScript glue import path and cache key ownership.
- Browser CSP, MIME serving, memory growth, and WebAssembly performance/security review.

## Receipts and guard

- Package receipt: `.dx/forge/receipts/wasm-bindgen.json`
- Launch receipt: `.dx/launch/receipts/wasm-bindgen-launch.json`
- Dashboard workflow receipt: `examples/template/.dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json`
- Hash manifest: `hash_algorithm: sha256` with `file_hashes` for the selected launch/dashboard workflow files and the Forge slice.
- Source-guard runbook fixture: `source_guard_runbook_fixture` points to `docs/packages/wasm-bindgen.source-guard-runbook.json`, and selected surface `webassembly-bridge-source-guard-runbook` carries its own SHA-256 hash evidence.
- Receipt hash refresh helper: `examples/template/webassembly-bridge-receipt-hashes.ts` supports `--check`, `--write`, and `--check --json` for the WebAssembly Bridge dashboard workflow receipt, package-status row, and typed read model.
- The receipt hash refresh helper canonicalizes the legacy generated `tools/launch/runtime-template/assets/launch-runtime.js` receipt path back to the source-owned `tools/launch/runtime-template/assets/launch-runtime.ts` file before hashing, so WebAssembly Bridge source freshness does not stay falsely missing after the launch runtime moved to TypeScript.
- Receipt hash refresh visibility: package-status, the read model, lower dx-check, and the DX Studio/check-panel row expose `webassembly-bridge:receipt-hash-refresh`, `webassembly_bridge_receipt_hash_refresh_current`, `webassembly_bridge_receipt_hash_refresh_stale`, and `webassembly_bridge_receipt_hash_refresh_missing` with `receipt_hash_refresh.tracked_files[]`, `receipt_hash_refresh.current_files[]`, `receipt_hash_refresh.stale_files[]`, `receipt_hash_refresh.missing_files[]`, `receipt_hash_refresh.stale_mirror_files[]`, and `receipt_hash_refresh.missing_mirror_files[]` path arrays.
- Stale-only source attribution: the receipt helper now emits `stale_surface_ids`, `stale_surface_types`, `missing_surface_ids`, and `missing_surface_types`. The package-owned fixture `webassembly-bridge-check-panel-source-stale-only` mutates only `core/src/ecosystem/dx_check_receipt.rs` and expects `webassembly-bridge-check-panel-source` / `check_panel_source`, proving shared check-panel source drift independently from lower dx-check, generated starter, runbook, materializer, dashboard, and runtime helper files.
- Helper self-freshness surfaces: the helper now hash-backs `core/src/ecosystem/project_check/wasm_bindgen_dx_check.rs` as selected surface `webassembly-bridge-lower-dx-check-source` and `core/src/ecosystem/dx_check_receipt.rs` as selected surface `webassembly-bridge-check-panel-source`. `receipt_hash_refresh.lower_dx_check_source`, `receipt_hash_refresh.check_panel_source`, the dashboard workflow receipt, package-status row, and typed read model all carry these paths so helper-metric drift is stale-detectable through `webassembly-bridge:receipt-hash-refresh`.
- Source guard: `benchmarks/wasm-bindgen-slice.test.ts`
- Hash receipt guard: `benchmarks/wasm-bindgen-hash-receipt.test.ts`
- Receipt hash refresh guard: `benchmarks/wasm-bindgen-receipt-hash-refresh.test.ts`

## Rust dx-check output

`core/src/ecosystem/project_check/wasm_bindgen_dx_check.rs` consumes the shared package-status row for `wasm/bindgen` and emits `webassembly_bridge_*` metrics from the dashboard workflow receipt. It reports `webassembly-bridge-stale-receipt`, `webassembly-bridge-missing-receipt`, `webassembly-bridge-blocked-surface`, and `webassembly-bridge-unsupported-surface` findings from receipt and source-marker evidence without claiming browser execution proof.

Hash-backed visibility now adds `webassembly_bridge_hash_manifest_present` and `webassembly_bridge_hash_mismatch` so package-status and Rust dx-check can distinguish a present receipt from stale selected WebAssembly Bridge source files. Rust dx-check now uses a shared byte-level SHA-256 helper for `file_hashes`, including the www-template path fallback used by generated starter surfaces. The hash guard remains `SOURCE-ONLY`: it proves source freshness, not live generated-Wasm execution.

The WebAssembly Bridge checker also carries a package-owned Rust fixture, `webassembly_bridge_hash_mismatch_metric_and_finding_are_byte_derived`, that builds a temporary package-status row, mutates a hash-backed selected file, and asserts `webassembly_bridge_hash_mismatch` plus `webassembly-bridge-hash-mismatch` flip together. The hash guard remains source-owned and should be rerun directly when selected-file freshness logic changes; this pass verified the dx-style missing-state fixture below.

Receipt-helper visibility now adds `webassembly_bridge_receipt_hash_refresh_current`, `webassembly_bridge_receipt_hash_refresh_stale`, and `webassembly_bridge_receipt_hash_refresh_missing` to the lower Rust dx-check output. Exact `receipt_hash_refresh.stale_files[]` and `receipt_hash_refresh.stale_mirror_files[]` entries mark helper freshness stale; exact `receipt_hash_refresh.missing_files[]` and `receipt_hash_refresh.missing_mirror_files[]` entries mark it missing, while `webassembly_bridge_hash_mismatch` remains byte-derived from selected source file hashes. The focused fixture `webassembly_bridge_package_metrics_reports_helper_freshness_from_path_arrays` proves helper path arrays can flip helper freshness without falsely blaming selected WebAssembly Bridge source bytes.

The receipt helper now tracks that lower dx-check module itself plus `core/src/ecosystem/dx_check_receipt.rs`. The selected surfaces `webassembly-bridge-lower-dx-check-source` and `webassembly-bridge-check-panel-source` keep the Rust helper-freshness fixture and Studio/check-panel row source hash-backed without claiming live generated-Wasm or browser style proof.

## DX-Style Compatibility

The WebAssembly Bridge dashboard and launch workflow now publish `dx.forge.package.dx_style_compatibility` evidence in the dashboard workflow receipt, package-status row, and typed read model. The evidence covers the visible `dashboard-wasm-bindgen-workflow`, `launch-wasm-compute-dashboard-workflow`, and `wasm-bindgen-readiness-workflow` surfaces, using `data-dx-style-surface="theme-token-card"` and `data-dx-style-surface="theme-token"` markers with `styles/theme.css` and generated app CSS as the style-token boundary.

Rust dx-check reports `webassembly_bridge_dx_style_compatibility_present` and `webassembly_bridge_dx_style_compatibility_missing`, and raises `webassembly-bridge-missing-dx-style-compatibility` if the WebAssembly Bridge package-status row loses that compatibility evidence. This remains `SOURCE-ONLY` style evidence: no live governed browser style proof, generated-Wasm runtime proof, or app-owned security/performance review is claimed.

The checker now includes `webassembly_bridge_dx_style_missing_metric_and_finding_flip`, a focused Rust fixture that starts with dx-style evidence present, rewrites the temporary WebAssembly Bridge package-status row without `dx_style_compatibility`, and proves `webassembly_bridge_dx_style_compatibility_missing` plus `webassembly-bridge-missing-dx-style-compatibility` flip while the dashboard receipt remains present.

## DX Studio/check-panel row

`core/src/ecosystem/dx_check_receipt.rs` now renders the DX Studio/check-panel WebAssembly Bridge package row from `.dx/forge/package-status.json` into `check_panel.view_model.package_lane_rows`. The row keeps `WebAssembly Bridge` as the user-facing package name, stores `wasm-bindgen` `0.2.121` and `G:/WWW/inspirations/wasm-bindgen` as provenance, and surfaces selected `wasm-bindgen-readiness-workflow` source markers plus receipt, hash, receipt_hash_refresh path arrays, and dx-style evidence.

The package row exposes `webassembly_bridge_hash_manifest_present`, `webassembly_bridge_hash_mismatch`, `webassembly_bridge_receipt_hash_refresh_current`, `webassembly_bridge_receipt_hash_refresh_stale`, `webassembly_bridge_receipt_hash_refresh_missing`, `webassembly_bridge_dx_style_compatibility_present`, and `webassembly_bridge_dx_style_compatibility_missing` so Studio and Zed can distinguish a present receipt, stale selected files, stale or missing helper paths, and missing style evidence without claiming live generated-Wasm or browser style proof.

## Static launch package-lane template

The static `/` WebAssembly Bridge package-lane template is now available before `.dx/receipts/check/check-latest.json` exists. `examples/template/template-shell.tsx` owns the authored template row and `the static launch runtime template` mirrors `data-dx-check-package-lane-template="wasm/bindgen"`, `data-dx-check-package-lane-row="wasm/bindgen"`, `data-dx-check-package-lane-name="WebAssembly Bridge"`, `wasm-bindgen` `0.2.121`, `G:/WWW/inspirations/wasm-bindgen`, the dashboard workflow receipt path, `data-dx-check-package-lane-dx-style-status="present"`, `data-dx-style-surface="theme-token"`, and `data-dx-token-scope="wasm/bindgen"`.

The source DX Studio edit contract, launch runtime materializer, and Rust Studio manifest include `wasm/bindgen` in the `dx-check-health-panel` package filter, so Studio can discover the WebAssembly Bridge row without waiting for a receipt-backed package-status load. The static row repeats the receipt-backed metric vocabulary (`webassembly_bridge_hash_manifest_present`, `webassembly_bridge_hash_mismatch`, `webassembly_bridge_dx_style_compatibility_present`, and `webassembly_bridge_dx_style_compatibility_missing`) while live generated-Wasm runtime proof remains app-owned.

## Generated-Starter Source Guard

The WebAssembly Bridge generated-starter materialization guard now runs `dx run --test .\benchmarks\wasm-bindgen-dx-check-package-lane-panel.test.ts` and materializes `tools/launch/materialize-www-template.ts` into a temporary starter. It proves generated static launch HTML keeps the `wasm/bindgen` package-lane row, official `WebAssembly Bridge` naming, `wasm-bindgen` `0.2.121` provenance, `G:/WWW/inspirations/wasm-bindgen`, `data-dx-check-package-lane-dx-style-status="present"`, `data-dx-style-surface="theme-token"`, `data-dx-token-scope="wasm/bindgen"`, and the receipt-backed WebAssembly Bridge metric vocabulary.

The WebAssembly Bridge Studio source-guard/runbook entry is published as `webassembly-bridge-generated-starter-materialization` in `source_guard_index`, `/launch` `source_guard_ids`, the `/launch` source-only contracts, and the `/launch` runbook commands. This is `SOURCE-ONLY` proof: it checks source materialization, provenance, dx-style markers, and package-scoped dx-check panel metadata without claiming live generated-Wasm runtime proof.

The WebAssembly Bridge lower dx-check helper freshness guard is now published as `webassembly-bridge-lower-dx-check-helper-freshness` in `source_guard_index`, `/launch` fixture metadata, source-only contracts, and runbook commands. It points at `core/src/ecosystem/project_check/wasm_bindgen_dx_check.rs` and the focused `cargo test -q -p dx-www-compiler webassembly_bridge_package_metrics_reports_helper_freshness_from_path_arrays --lib` proof so Studio/Zed can find the helper path-array fixture directly; `webassembly_bridge_hash_mismatch stays byte-derived`, and the guard remains source-only without live generated-Wasm runtime proof.

The WebAssembly Bridge check-panel helper freshness guard is now published as `webassembly-bridge-check-panel-helper-freshness` in the same Studio/runbook surfaces. It points at `core/src/ecosystem/dx_check_receipt.rs` and the focused `cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_webassembly_bridge_package_lane_hash_refresh_row --lib` proof so Studio/Zed can discover the shared check-panel row source that reports `webassembly_bridge_receipt_hash_refresh_current`, stale, and missing path-array metrics; `webassembly_bridge_hash_mismatch stays byte-derived`, and the guard remains source-only without live generated-Wasm runtime proof.

The package-owned runbook fixture is now `docs/packages/wasm-bindgen.source-guard-runbook.json`. It mirrors the `source_guard_runbook_index` contract, exact lightweight command, Studio `fixture_path` links, generated `public/preview-manifest.json` fixture exposure, Zed/DX Studio markers, receipt hash guard, upstream `wasm-bindgen` `0.2.121` provenance, and app-owned WebAssembly runtime boundaries without claiming live generated-Wasm runtime proof. The generated starter materializer now emits this fixture in root `sourceGuardRunbookFixtures` and `/launch` `routes[].sourceGuardRunbookFixtures`, so Studio can discover the WebAssembly Bridge source guard from generated metadata instead of parsing Rust.

The fixture is now receipt-backed as selected surface `webassembly-bridge-source-guard-runbook` in the WebAssembly Bridge dashboard workflow receipt, package-status row, and typed read model. Its `source_guard_runbook_fixture` path and SHA-256 hash make source-guard fixture drift stale-detectable through the existing WebAssembly Bridge dx-check hash path while runtime proof remains app-owned.

This pass also refreshed the WebAssembly Bridge dashboard workflow receipt, package-status row, and typed read-model hash mirrors for the current shared launch template bytes. The refresh is limited to WebAssembly Bridge-selected files and keeps runtime proof app-owned.
