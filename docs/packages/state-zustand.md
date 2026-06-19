# State Management

Package id: `state/zustand`

Official DX package name: `State Management`

Upstream package: `zustand`

Source mirror: `G:/WWW/inspirations/zustand`

This Forge slice implements the official DX State Management lane as source-owned DX glue around Zustand 5.0.13. It exposes the launch-useful public API surface from the real upstream package: vanilla stores, React selectors, equality-aware selectors, persistence, selector subscriptions, DevTools, Redux-style reducers, SSR mutation safety, and the optional Immer middleware boundary. It does not vendor the full upstream package, and the official template path stays no template-local `node_modules`.

## Real API Slice

- `createStore`, `create`, `useStore`, `Mutate`, `StoreMutators`, and `StoreMutatorIdentifier` match Zustand's vanilla and React store shape.
- `createWithEqualityFn`, `useStoreWithEqualityFn`, `shallow`, and `useShallow` support stable derived dashboard selections.
- `persist`, `createJSONStorage`, `PersistApi`, `PersistOptions`, `persist.rehydrate`, `hasHydrated`, `onHydrate`, and `onFinishHydration` support reviewed browser persistence and hydration visibility.
- `subscribeWithSelector`, `combine`, `devtools`, `redux`, and `unstable_ssrSafe` cover the middleware paths used by the launch template.
- `immer` is exposed through the source-owned optional peer boundary, but the application still owns installing and approving the external `immer` dependency.

## Dashboard Workflow

`examples/template/state-zustand-dashboard.tsx` owns the dashboard settings store. The visible `/launch` workflow persists density, queue focus, command hints, save/reset actions, and manual rehydration state through `dx-launch-dashboard-settings`.

Stable source and generated markers include:

- `data-dx-package="state/zustand"`
- `data-dx-component="launch-dashboard-state-shell"`
- `data-dx-component="launch-dashboard-state-workflow"`
- `data-dx-component="launch-dashboard-state-summary"`
- `data-dx-style-surface="state-management"`
- `data-dx-zustand-store="launch-dashboard-settings"`
- `data-dx-zustand-persist-key="dx-launch-dashboard-settings"`
- `data-dx-zustand-action="rehydrate-dashboard-settings"`
- `data-dx-zustand-hydration-event`
- `data-dx-zustand-rehydrate-state`

The shell-level state binding lives in `examples/template/template-shell.tsx`, and the shadcn operator controls consume the same Zustand store through `examples/template/shadcn-dashboard-controls.tsx`. The generated no-`node_modules` runtime bridge mirrors the interaction in `the static launch runtime template` and `tools/launch/runtime-template/assets/launch-runtime.ts`.

## Receipts And Discovery

The source-owned dashboard receipt is:

`examples/template/.dx/forge/receipts/2026-05-22-state-zustand-dashboard-workflow.json`

DX Studio/Web Preview discovery maps this package through `examples/template/dx-studio-edit-contract.ts` and `dx-www/src/cli/studio_manifest.rs`, including the shell selector, package markers, persist key, hydration event, rehydrate action state, and the dx-check package-lane markers for `data-dx-check-package-lane-row`, `data-dx-check-package-lane-status`, `data-dx-check-package-lane-receipt-status`, `data-dx-check-package-lane-name`, `data-dx-check-package-lane-upstream-package`, `data-dx-check-package-lane-source-mirror`, `data-dx-check-package-lane-receipt-path`, and `data-dx-check-package-lane-dx-style-status`.

Generated starter provenance is part of the same receipt: `examples/template/template-route-contract.ts` declares the `/launch` package workflow contract, and `dx-www/src/cli/mod.rs` materializes `components/launch/state-zustand-dashboard.tsx` plus the copied workflow receipt for `dx new`.

The dashboard workflow and package-lane receipt now expose `dx.forge.package.dx_style_compatibility` for the official State Management lane. The compatibility metadata records `styles/theme.css` as the token source, `styles/generated.css` as the generated CSS target, visible surfaces `launch-dashboard-state-workflow` and `launch-dashboard-state-shell`, and the shared `data-dx-style-surface="state-management"` marker. The status is `SOURCE-ONLY`: source markers, package receipts, and dx-check metrics prove the contract exists, while browser visual proof and final theme review remain app-owned.

The State Management receipt hash helper is `examples/template/state-management-receipt-hashes.ts`. It publishes `dx.forge.package.receipt_hash_refresh` as `receipt_hash_refresh` in `.dx/forge/package-status.json` and `receiptHashRefresh` in `examples/template/forge-package-status-read-model.ts`, with Zed/DX Studio visibility under `state-management:receipt-hash-refresh`. The helper checks or refreshes SHA-256 hashes for the selected dashboard state source, shell binding, runtime page, runtime bridge, this package doc, the package-owned runbook fixture `docs/packages/state-zustand.source-guard-runbook.json`, the shared preview-manifest materializer `tools/launch/materialize-www-template.ts` that emits the State Management fixture into generated starter metadata, and the Studio manifest source `dx-www/src/cli/studio_manifest.rs` that publishes the State Management source-guard/runbook declarations:

```powershell
node tools/launch/run-template-receipt-helper.js examples/template/state-management-receipt-hashes.ts --check
node tools/launch/run-template-receipt-helper.js examples/template/state-management-receipt-hashes.ts --write
node tools/launch/run-template-receipt-helper.js examples/template/state-management-receipt-hashes.ts --check --json
```

This is `SOURCE-ONLY`: the helper reads local files and receipt metadata only. It does not execute browser storage, read secrets, install Zustand, open DevTools, or prove persisted-state runtime behavior.

The helper JSON also reports exact `current_files`, `stale_files`, `missing_files`, `stale_mirror_files`, and `missing_mirror_files` arrays. That makes preview-manifest materializer or Studio manifest declaration drift attributable without falsely implying that the selected dashboard state source, shell binding, runtime page, or runtime bridge changed. The targeted Rust fixture `dx_check_latest_panel_exposes_state_management_stale_helper_file_attribution` proves the dx-check row preserves `tools/launch/materialize-www-template.ts` in `stale_files` while the selected State Management dashboard, shell, runtime, doc, and runbook source files remain in `current_files`.

## dx-check Visibility

The launch package-status read model now exposes a State Management lane row for dx-check and dashboard consumers:

- `present`: the State Management package receipt and selected surfaces are available.
- `stale`: a selected State Management surface hash or receipt timestamp no longer matches the status receipt.
- `missing-receipt`: the package lane is installed, but `.dx/forge/receipts/packages/state-zustand.json` is missing.
- `blocked`: a requested State Management surface needs an app-owned boundary decision, such as persisted-state sensitivity review.
- `unsupported-surface`: the app requested a Zustand upstream surface that this official DX State Management slice does not materialize.

The current www-template row is `SOURCE-ONLY`: `examples/template/forge-package-status-read-model.ts` and `.dx/forge/package-status.json` expose `state_management_receipt_present`, `state_management_receipt_stale`, `state_management_missing_receipt`, `state_management_blocked_surface`, `state_management_unsupported_surface`, `state_management_dx_style_compatibility_present`, and `state_management_dx_style_compatibility_missing` as machine-readable metric names. `core/src/ecosystem/project_check/state_management_dx_check.rs` now maps the same State Management package receipt into Rust dx-check Forge-section metrics and emits `state-management-missing-receipt`, `state-management-stale-receipt`, `state-management-blocked-surface`, `state-management-unsupported-surface`, and `state-management-missing-dx-style-compatibility` findings when the receipt state requires action.

`core/src/ecosystem/dx_check_receipt.rs` now carries State Management package-status and receipt evidence into `dx.www.check_panel_view_model` as `package_lane_rows`. The row keeps the official name `State Management`, preserves upstream provenance as `zustand` `5.0.13` from `G:/WWW/inspirations/zustand`, includes selected surfaces and runtime limitations, exposes `receipt_hash_refresh` with `state-management:receipt-hash-refresh`, and reports metric values for `state_management_package_present`, `state_management_receipt_present`, `state_management_receipt_stale`, `state_management_missing_receipt`, `state_management_blocked_surface`, `state_management_unsupported_surface`, `state_management_receipt_hash_refresh_current`, `state_management_receipt_hash_refresh_stale`, `state_management_receipt_hash_refresh_missing`, `state_management_dx_style_compatibility_present`, and `state_management_dx_style_compatibility_missing`. `examples/template/template-shell.tsx` renders those `package_lane_rows` directly in the dx-check panel, while the static `the static launch runtime template` template exposes the same helper path, JSON check command, stale/current/missing counts, `data-dx-check-package-lane-hash-refresh-stale-file-list`, and metric-name markers before a fresh dx-check receipt exists. `tools/launch/materialize-www-template.ts` now preserves those markers in generated static launch HTML and scopes the generated `launch-runtime-dx-check-panel` editable surface to `state/zustand`, so Zed/DX Studio package search can find the State Management helper row and exact stale helper file list in a starter app without scraping the raw page. `examples/template/dx-studio-edit-contract.ts` and `dx-www/src/cli/studio_manifest.rs` expose matching source markers for Zed/DX Studio. Runtime proof remains deferred until browser storage, theme rendering, and hydration checks are explicitly approved.

The generated-starter guard is now published as `state-management-generated-starter-materialization` in the Studio `source_guard_index` and `/launch` `source_guard_runbook_index`. The stale-helper Rust proof is also published as `state-management-check-panel-stale-helper-attribution`, with the exact source-only command `cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_state_management_stale_helper_file_attribution --lib` and the same package-owned fixture path. `docs/packages/state-zustand.source-guard-runbook.json` records both contracts: the official State Management name, `state/zustand` package id, upstream `zustand` provenance, inspected source files, exact guard commands, Zed/DX Studio markers, receipt hash helper, Studio manifest source, app-owned boundaries, and `SOURCE-ONLY` runtime limitations. Generated `public/preview-manifest.json` snapshots now expose the fixture through the root `sourceGuardRunbookFixtures` list and `/launch` `routes[].sourceGuardRunbookFixtures`, so starter output can point back to the source-owned runbook without parsing Rust. Zed/DX Studio can list the exact source-only State Management commands beside the package row without running a server or installing packages, and the receipt helper now marks `dx-www/src/cli/studio_manifest.rs` stale when those declarations drift. The fixture is source-only, without claiming browser storage or visual runtime proof.

## App-Owned Boundaries

Applications still own selector granularity, equality policy, persisted-state sensitivity review, durable storage policy, DevTools availability, action taxonomy, SSR data-fetching policy, optional Immer installation, and browser runtime approval. Browser/Web Preview click evidence remains governed; this package doc only records source wiring and source-guard proof.

## Source Guard

Run the narrow source guards with:

```powershell
dx run --test .\benchmarks\zustand-slice.test.ts
dx run --test .\benchmarks\zustand-launch-materialized.test.ts
dx run --test .\benchmarks\zustand-dx-check-package-lane-panel.test.ts
dx run --test .\benchmarks\zustand-dx-style-compatibility.test.ts
dx run --test .\benchmarks\state-management-receipt-hash-refresh.test.ts
cargo test -p dx-www-compiler state_management_dx_style_missing_metric_and_finding_flip --lib
cargo test -p dx-www-compiler dx_check_latest_panel_exposes_state_management_package_lane_row --lib
cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_state_management_stale_helper_file_attribution --lib
cargo test -p dx-www-compiler dx_check_reports_state_management_visibility_statuses --lib
```
