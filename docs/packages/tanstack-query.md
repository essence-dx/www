# Data Fetching & Cache Forge Slice

Package id: `tanstack/query`

Official package name: Data Fetching & Cache

Upstream package: `@tanstack/react-query`

Source mirror: `G:\WWW\inspirations\tanstack-query`

This slice is source-owned DX glue around upstream `@tanstack/react-query` v5. It exposes small typed files for `QueryClient`, query options, mutations, cache helpers, hydration, persistence, runtime state, lifecycle managers, and default policies. It does not vendor upstream internals and does not create a template-local `node_modules` workflow.

## Dashboard Workflow

The launch dashboard consumes `LaunchQueryCacheStatus` as a visible server-state surface:

- `useQuery` reads the launch package catalog, package roles, required env gates, and app-owned boundaries as a cached dashboard read model.
- `useMutation` refreshes and invalidates the dashboard read query.
- The read model now includes a package-readiness queue derived from the real launch package catalog, including package id, role, command, receipt count, required-env count, and ready/missing-env status.
- `examples/template/query-dashboard-read-model.ts` owns the typed catalog read model, while `query-cache-status.tsx` stays focused on the visible Data Fetching & Cache client workflow.
- `DxQueryLifecycleStatus`, `setDxQueryOnline`, and `setDxQueryFocused` drive focus/online readiness controls.
- `setDxQueryDefaults` and `readDxQueryDefaults` apply and report balanced, fast-refresh, and durable cache profiles.
- `QueryDashboardWorkflow` is consumed by the starter dashboard as a visible cache-profile and refresh-receipt surface.
- `query/dashboard-workflow.ts` exports `createDxDashboardQueryClient`, `applyDxDashboardQueryProfile`, `refreshDxDashboardQuery`, and `createDxDashboardQueryReceipt` over the real QueryClient defaults and invalidation APIs.

Stable `/launch` markers include `data-dx-package="tanstack/query"`, `data-dx-component="tanstack-query-dashboard-data-workflow"`, `data-dx-dashboard-workflow="query-backed-dashboard-data"`, `data-dx-product-surface="launch-dashboard"`, `data-dx-query-dashboard-source`, `data-dx-query-dashboard-queue="package-readiness"`, `data-dx-query-package-id`, `data-dx-query-package-role`, `data-dx-query-package-status`, `data-dx-query-action="refresh-dashboard-data"`, `data-dx-query-action="fetch-dashboard-data-now"`, `data-dx-component="tanstack-query-dashboard-settings"`, `data-query-dashboard-action`, and `data-query-dashboard-policy`.

The no-node-modules runtime bridge also materializes the same dashboard workflow in `the static launch runtime template` and `tools/launch/runtime-template/assets/launch-runtime.ts`. That bridge reads a safe local launch catalog summary and updates mission-control counts; the TSX source remains the real upstream query integration.

DX Studio maps the workflow through `examples/template/dx-studio-edit-contract.ts` as `tanstack-query-dashboard-data`, with responsive grid edits only. The route contract records the source-owned receipt at `examples/template/.dx/forge/receipts/2026-05-22-tanstack-query-dashboard-data.json` so DX CLI/Zed discovery can find the visible `/launch` interaction without relying on package-card metadata.

The runtime materializer copies that receipt into generated projects at `.dx/forge/receipts/2026-05-22-tanstack-query-dashboard-data.json` and adds `launch-runtime-query-dashboard-data` to `public/preview-manifest.json` with query dashboard markers, interaction selectors, and responsive edit operations.

Friday's governed live guard now checks the same browser markers: package id, query-backed workflow, runtime catalog source, dashboard counts, package-readiness queue rows, refresh action, safe local read action, result state, mission-control query status, and the generated `public/preview-manifest.json` surface plus copied receipt path. The guard is documented in the receipt but is not run during source-only package work.

The starter dashboard markers include `data-dx-component="dashboard-tanstack-query-workflow"`, `data-dx-dashboard-workflow="query-cache-refresh"`, `data-dx-query-profile`, `data-dx-query-action`, `data-dx-query-dashboard-receipt-path`, `data-dx-query-receipt-path`, and `data-dx-query-receipt-state`. The visible starter workflow points at the same source-owned dashboard receipt used by `/launch`: `examples/template/.dx/forge/receipts/2026-05-22-tanstack-query-dashboard-data.json`.

The starter receipt object carries `receiptPath`, `runtimeExecution: false`, `nodeModulesRequired: false`, `dashboardWorkflow`, a typed `publicApi` value (`setQueryDefaults`, `getQueryDefaults`, or `invalidateQueries`), and the selected cache defaults (`staleTimeMs`, `gcTimeMs`, and `retry`), so the visible action is an honest local handoff and not a fake runtime success.

## dx-check Visibility

The Data Fetching & Cache dashboard receipt now carries `dx.forge.package.dx_check_visibility` plus `hash_algorithm: sha256` and selected `file_hashes`, so dx-check, DX-WWW, and Zed can report `present`, `stale`, `missing-receipt`, `blocked`, `unsupported-surface`, and hash-derived stale states without inferring status from UI copy. User-facing docs spell these as present, stale, missing receipt, blocked, and unsupported surface while the receipt keeps the hyphenated machine labels.

Current status is `present` for the selected dashboard workflow, read model, starter workflow, and materialized receipt. Runtime proof, production network fetchers, persisted cache policy, cross-tab broadcast verification, and devtools panel exposure remain app-owned or governed surfaces; dx-check should show them as blocked or unsupported surface instead of treating the package as missing. If any tracked dashboard source hash differs from the receipt, dx-check should report `data_fetching_cache_hash_mismatch` and mark the receipt stale without claiming live QueryClient runtime proof.

## Package-Status Read Model

The shared launch package-status read model now includes a Data Fetching & Cache row for `tanstack/query`. It keeps upstream `@tanstack/react-query` and version `5.100.10` as provenance metadata, registers `data-fetching-cache-query-dashboard-workflow` and `data-fetching-cache-starter-dashboard-workflow` as selected surfaces, mirrors the receipt `hash_algorithm: sha256` and `file_hashes` onto those selected surfaces, publishes `data_fetching_cache_*` dx-check metrics, and adds the query dashboard Zed receipt surfaces without claiming live browser QueryClient proof.

## Receipt Hash Refresh

`examples/template/data-fetching-cache-receipt-hashes.ts` owns the Data Fetching & Cache hash-refresh workflow for the dashboard receipt, `.dx/forge/package-status.json`, and `examples/template/forge-package-status-read-model.ts`.

Run `node tools/launch/run-template-receipt-helper.js examples/template/data-fetching-cache-receipt-hashes.ts --check` to report stale or missing selected hashes, or `node tools/launch/run-template-receipt-helper.js examples/template/data-fetching-cache-receipt-hashes.ts --write` after reviewing source changes to refresh the receipt, package-status row, and typed read model. `--check --json` emits `dx.forge.package.receipt_hash_refresh` with `data-fetching-cache:receipt-hash-refresh`, `source_guard_runbook_fixture`, `preview_manifest_materializer`, `tracked_files`, tracked/stale/missing counts, per-file freshness, no-runtime/no-secret flags, and runtime limitations.

The package-owned source-guard runbook fixture is hash-tracked as `docs/packages/data-fetching-cache.source-guard-runbook.json`, and the shared preview-manifest materializer is hash-tracked as `tools/launch/materialize-www-template.ts`, so Studio/Zed runbook metadata or generated `sourceGuardRunbookFixtures` emission drift becomes stale receipt evidence without claiming live QueryClient execution.

The helper does not run live QueryClient execution, production fetchers, persistence, broadcast sync, dependency installation, or browser proof.

## DX-Style Compatibility

The visible Data Fetching & Cache dashboard surfaces now publish `dx.forge.package.dx_style_compatibility` as SOURCE-ONLY evidence. `examples/template/query-cache-status.tsx` carries `data-dx-style-surface="data-fetching-cache"` and uses DX token classes such as `border-border`, `bg-card`, and `text-card-foreground`; `examples/dashboard/src/components/QueryDashboardWorkflow.tsx` continues to expose `data-dx-style-surface="theme-token"` for the starter dashboard workflow.

The receipt and package-status row record `token_source: styles/theme.css`, `generated_css: styles/generated.css`, the selected style surfaces, source files, source markers, and `runtime_proof: false`. `dx-check` reports `data_fetching_cache_dx_style_compatibility_present` and `data_fetching_cache_dx_style_compatibility_missing`, and emits `data-fetching-cache-missing-dx-style-compatibility` if the package-status row loses the style evidence. This does not claim browser visual proof, governed theme review, dependency installation, or live QueryClient execution.

## Rust dx-check output

`core/src/ecosystem/project_check/data_fetching_cache_dx_check.rs` consumes the Data Fetching & Cache package-status row from `.dx/forge/package-status.json`, resolves `.dx/forge/receipts/2026-05-22-tanstack-query-dashboard-data.json`, compares selected SHA-256 `file_hashes` through the shared `core/src/ecosystem/project_check/file_hashes.rs` helper, and publishes `data_fetching_cache_package_present`, `data_fetching_cache_receipt_present`, `data_fetching_cache_receipt_stale`, `data_fetching_cache_missing_receipt`, `data_fetching_cache_blocked_surface`, `data_fetching_cache_unsupported_surface`, `data_fetching_cache_hash_manifest_present`, `data_fetching_cache_hash_mismatch`, `data_fetching_cache_dx_style_compatibility_present`, and `data_fetching_cache_dx_style_compatibility_missing` in the Forge section. Missing package-status, missing receipt, stale, blocked, unsupported selected surfaces, hash mismatches, and missing style evidence emit `data-fetching-cache-*` findings such as `data-fetching-cache-missing-receipt`, `data-fetching-cache-hash-mismatch`, and `data-fetching-cache-missing-dx-style-compatibility` without claiming live QueryClient runtime proof. The package-owned Rust fixture `data_fetching_cache_hash_mismatch_flips_when_selected_file_changes` mutates a temporary hash-backed query dashboard file and proves the metric plus finding flip together; `data_fetching_cache_dx_style_compatibility_metric_tracks_package_status_row` removes style evidence from a temporary package-status row and proves the missing metric plus finding flip together.

## DX Studio Check-Panel Row

`core/src/ecosystem/dx_check_receipt.rs` now renders `tanstack/query` in `check_panel.view_model.package_lane_rows` from `.dx/forge/package-status.json`. The DX Studio/check-panel Data Fetching & Cache package row keeps `Data Fetching & Cache` as the front-facing package name, keeps `@tanstack/react-query` `5.100.10` and `G:/WWW/inspirations/tanstack-query` as provenance, and surfaces selected dashboard/starter source markers, SHA-256 freshness, dx-style evidence, `receipt_hash_refresh`, `data-fetching-cache:receipt-hash-refresh`, and the metrics `data_fetching_cache_receipt_hash_refresh_current`, `data_fetching_cache_receipt_hash_refresh_stale`, and `data_fetching_cache_receipt_hash_refresh_missing` without claiming live QueryClient execution.

The package-lane fixture covers a stale-helper-only branch: if `receipt_hash_refresh.stale_file_count` changes while selected dashboard source hashes still match, the row becomes `stale`, `data_fetching_cache_receipt_hash_refresh_stale` becomes `1`, and `data_fetching_cache_hash_mismatch` stays `0`.

## Static Launch Package-Lane Template

The Data Fetching & Cache static / package-lane template now exposes `data-dx-check-package-lane-template="tanstack/query"` and `data-dx-check-package-lane-row="tanstack/query"` before a fresh dx-check receipt is loaded. The source launch shell and static runtime page publish official package naming, upstream `@tanstack/react-query` `5.100.10` provenance, the dashboard workflow receipt path, `data-fetching-cache:receipt-hash-refresh`, helper counts, `data_fetching_cache_receipt_hash_refresh_current`, `data_fetching_cache_receipt_hash_refresh_stale`, `data_fetching_cache_receipt_hash_refresh_missing`, `data-dx-style-surface="data-fetching-cache"`, `data-dx-token-scope="tanstack/query"`, and `data-dx-package="tanstack/query"` without claiming live QueryClient execution.

The launch materializer carries those markers into generated static launch HTML, keeps `tanstack/query` in the generated `launch-runtime-dx-check-panel` package scope, and links `docs/packages/data-fetching-cache.source-guard-runbook.json` from generated `public/preview-manifest.json` through root `sourceGuardRunbookFixtures` plus the `/launch` `routes[].sourceGuardRunbookFixtures` entry, so DX Studio and Zed can discover helper freshness from the generated starter even when `.dx/receipts/check/check-latest.json` has not been created yet.

## Source Guard

The Data Fetching & Cache Studio source-guard/runbook entry is `data-fetching-cache-generated-starter-materialization`. DX Studio and Zed can list `dx run --test .\benchmarks\tanstack-query-dx-check-package-lane-panel.test.ts` from `source_guard_index` and `/launch` `source_guard_runbook_index` to rerun the generated-starter materialization proof for `data-dx-check-package-lane-row="tanstack/query"`, `data-dx-token-scope="tanstack/query"`, and `data-fetching-cache:receipt-hash-refresh` without claiming live QueryClient execution.

`docs/packages/data-fetching-cache.source-guard-runbook.json` is the package-owned JSON fixture for that runbook contract. It mirrors the official Data Fetching & Cache package metadata, upstream `@tanstack/react-query` `5.100.10` provenance, inspected QueryClient/React hook source files, the selected dashboard/starter/helper surfaces, generated preview-manifest `sourceGuardRunbookFixtures` exposure, the hash-backed preview-manifest materializer, the exact source-only command, Zed/DX Studio marker names, app-owned boundaries, and `SOURCE-ONLY` runtime limitations without claiming live QueryClient execution.

Run the narrow guard with:

```powershell
dx run --test .\benchmarks\tanstack-query-hash-receipt.test.ts
dx run --test .\benchmarks\tanstack-query-dx-check-output.test.ts
dx run --test .\benchmarks\tanstack-query-dx-style-compatibility.test.ts
dx run --test .\benchmarks\tanstack-query-receipt-hash-refresh.test.ts
dx run --test .\benchmarks\tanstack-query-dx-check-package-lane-panel.test.ts
node tools/launch/run-template-receipt-helper.js examples/template/data-fetching-cache-receipt-hashes.ts --check --json
cargo test -q -p dx-www-compiler data_fetching_cache_hash_mismatch_flips_when_selected_file_changes --lib
cargo test -q -p dx-www-compiler data_fetching_cache_dx_style_compatibility_metric_tracks_package_status_row --lib
dx run --test .\benchmarks\tanstack-query-dashboard-workflow.test.ts
dx run --test .\benchmarks\tanstack-query-slice.test.ts -- --test-name-pattern "tanstack query launch companion powers dashboard data reads"
```

## App-Owned Boundaries

Applications still own query keys, fetchers, retry policy, cache retention, default-registration order, persistence storage, broadcast channel naming, loading UI, error UI, telemetry retention, and dependency installation.

## Deferred Runtime Proof

This coding pass verifies source wiring only. Browser runtime capture, production dashboard fetcher wiring, optional devtools exposure, and cross-tab broadcast verification remain launch-governed checks.
