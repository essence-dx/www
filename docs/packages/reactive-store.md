# Reactive Store

Official DX package name: `Reactive Store`

Package id: `reactive/store`

Provenance metadata:

- upstream_package: `@tanstack/store`
- based_on: `@tanstack/react-store`
- source_mirror: `G:/WWW/inspirations/tanstack-store`
- upstream_version: `0.11.0`

Honesty label: `SOURCE-OWNED TEMPLATE STORE`

This lane materializes selected, editable reactive state source into a DX project. The inspected upstream public APIs are `Store`, `ReadonlyStore`, `createStore`, `createAtom`, `createAsyncAtom`, `batch`, `flush`, `shallow`, `useSelector`, `useAtom`, `createStoreContext`, and the deprecated `useStore` alias.

## Selected Surfaces

- Core store: `store.ts`, `atom.ts`, `types.ts`
- Atom graph: `atom.ts`, `types.ts`
- Comparison helper: `shallow.ts`
- React selector adapter: `react.ts`
- React context adapter: `context.tsx`
- Package metadata and receipt helper: `metadata.ts`

## Selective Surface Installs

Default `dx add reactive-store --write` materializes the full editable slice. Use the Forge selector form when an app only needs one surface:

- `dx forge add reactive/store#core-store --write`
- `dx forge add reactive/store#atom-graph --write`
- `dx forge add reactive/store#comparison-helper --write`
- `dx forge add reactive/store#react-selector --write`
- `dx forge add reactive/store#react-context --write`

The `core-store` selector includes `atom.ts` because the upstream `Store` implementation is atom-backed. The React selector surface includes the store and atom files it imports, but it does not generate unused comparison-helper code. The `react-context` surface exposes upstream `createStoreContext` as a non-DOM React context transport for app-owned atoms and stores; it does not add visible UI or force store/atom files unless the app requests them.

## Front-Facing Files

Default materialization writes editable source under `lib/forge/state/reactive-store/`. Forge does not create a hidden generated package folder and does not install `node_modules`.

## dx-check Visibility

Reactive Store metadata exposes dx-check visibility states for DX-WWW and future Zed panels: present, stale, missing receipt, blocked, unsupported surface.

The receipt helper records selected surfaces, files, optional hashes, provenance, source mirror, runtime limitations, and app-owned boundaries. Generated applications should keep the receipt at `.dx/forge/receipts/packages/reactive-store.json`.

The launch package catalog now uses the shared `dx.forge.package.dx_check_visibility` schema for Reactive Store instead of a lane-specific receipt schema. Its `receiptIntegrity` metadata points at the five files guarded by `reactive-store:receipt-hash-refresh`: `context.tsx`, `metadata.ts`, README, `docs/packages/reactive-store.source-guard-runbook.json`, and `tools/launch/materialize-www-template.ts`.

## Package Status Read Model

The launch package-status read model now exposes `reactive/store#react-context` with a hash-backed package receipt at `.dx/forge/receipts/packages/reactive-store.json`. The package-status row is `present` when the selected `context.tsx`, `metadata.ts`, README, package-owned source-guard runbook fixture, and generated preview-manifest materializer match the receipt hashes; future dx-check work can mark it `stale` if those files drift. The Reactive Store row and the top-level launch metrics both mirror `reactive_store_hash_manifest_present` and `reactive_store_hash_mismatch` so Zed/DX Studio can show hash freshness without scraping raw Rust output.

Tracked metrics: `reactive_store_package_present`, `reactive_store_receipt_present`, `reactive_store_receipt_stale`, `reactive_store_missing_receipt`, `reactive_store_blocked_surface`, `reactive_store_unsupported_surface`, `reactive_store_hash_manifest_present`, `reactive_store_hash_mismatch`, `reactive_store_receipt_hash_refresh_current`, `reactive_store_receipt_hash_refresh_stale`, and `reactive_store_receipt_hash_refresh_missing`.

## Receipt Hash Refresh Helper

`examples/template/reactive-store-receipt-hashes.ts` refreshes the selected SHA-256 hashes for the official **Reactive Store** package receipt, `.dx/forge/package-status.json`, and `forge-package-status-read-model.ts`.

```powershell
node tools/launch/run-template-receipt-helper.js examples/template/reactive-store-receipt-hashes.ts --check
node tools/launch/run-template-receipt-helper.js examples/template/reactive-store-receipt-hashes.ts --write
node tools/launch/run-template-receipt-helper.js examples/template/reactive-store-receipt-hashes.ts --check --json
```

The helper validates `package_id: reactive/store`, official package naming, upstream `@tanstack/store` provenance, `based_on: @tanstack/react-store`, `source_mirror: G:/WWW/inspirations/tanstack-store`, and `hash_algorithm: sha256`. It now tracks five selected files, including `docs/packages/reactive-store.source-guard-runbook.json` as `source_guard_runbook_fixture` and `tools/launch/materialize-www-template.ts` as `preview_manifest_materializer`, then mirrors those hashes through the receipt, package-status JSON, and typed read model. JSON output separates direct selected-file status from mirror status with `current_files`, `stale_files`, `missing_files`, `stale_mirror_files`, and `missing_mirror_files`, so a materializer-only drift can be shown without falsely blaming `react-context` source files. It reports `reactive-store:receipt-hash-refresh` for Zed/DX Studio and does not run React browser runtime proof, install packages, or read secrets.

## Rust dx-check output

`core/src/ecosystem/project_check/reactive_store_dx_check.rs` reads `.dx/forge/package-status.json`, resolves `.dx/forge/receipts/packages/reactive-store.json`, and emits the `reactive_store_*` metric family into the shared Forge `dx check` section.

The Rust helper reports `reactive-store-missing-receipt`, `reactive-store-stale-receipt`, `reactive-store-blocked-surface`, `reactive-store-unsupported-surface`, and `reactive-store-hash-mismatch`. Hash-backed selected surfaces are checked through the shared byte-level SHA-256 helper, so changed or missing `react-context` files become stale without claiming live React runtime proof.

Lower-level dx-check output also reads package-status `receipt_hash_refresh` and emits `reactive_store_receipt_hash_refresh_current` / `reactive_store_receipt_hash_refresh_stale` / `reactive_store_receipt_hash_refresh_missing`. The source-only fixture `reactive_store_hash_refresh_stale_helper_keeps_source_hash_clean` proves stale helper freshness marks the receipt stale while keeping `reactive_store_hash_mismatch = 0` when selected file bytes still match.

## dx-check Panel Row

`core/src/ecosystem/dx_check_receipt.rs` now renders Reactive Store into `check_panel.view_model.package_lane_rows` from `.dx/forge/receipts/packages/reactive-store.json`. The row preserves official package naming, upstream provenance, selected `react-context` surfaces, runtime limitations, and hash-backed `reactive_store_hash_manifest_present` / `reactive_store_hash_mismatch` metrics for Studio and Zed panels.

The panel row also promotes package-status `receipt_hash_refresh` into `receipt_hash_refresh`, `reactive_store_receipt_hash_refresh_current`, `reactive_store_receipt_hash_refresh_stale`, and `reactive_store_receipt_hash_refresh_missing`, so Studio and Zed can show receipt_hash_refresh helper freshness beside byte-level hash drift without opening raw package-status JSON. The row payload carries `current_files`, `stale_files`, `missing_files`, `stale_mirror_files`, and `missing_mirror_files`; the launch shell renders `data-dx-check-package-lane-hash-refresh-stale-file-list` from those arrays for panel consumers.

The panel row compares current selected file bytes against the receipt `file_hashes` and marks the row `stale` when a materialized Reactive Store file drifts. A targetable stale-helper-only fixture flips `receipt_hash_refresh.stale_file_count` and verifies `reactive_store_receipt_hash_refresh_stale = 1` while `reactive_store_hash_mismatch = 0`, proving helper drift is not confused with source-byte drift. This is a `SOURCE-OWNED TEMPLATE STORE`; it does not install React dependencies or claim browser runtime proof.

Static launch preview now includes a hidden `data-dx-check-package-lane-row="reactive/store"` template in `the static launch runtime template`, so Zed/DX Studio can discover the official Reactive Store package-lane marker before a real dx-check receipt is loaded. The React shell template and checked-in static HTML marker also expose `reactive-store-receipt-hashes.ts`, `reactive-store:receipt-hash-refresh`, helper file counts, stale-file-list, and `reactive_store_receipt_hash_refresh_*` metric names. The static template still reports `missing` / `missing-receipt`; loaded receipts remain the source of truth for `present` or `stale`.

Studio package-scoped discovery now lists `reactive/store` in the `/launch` manifest `forge_packages`, the `dx-check-health-panel` editable surface package filter, and `routes[].package_surfaces`. This lets Studio search for the Reactive Store package row through the manifest indexes instead of scraping raw DOM markers only.

Generated `public/preview-manifest.json` now exposes the Reactive Store source-guard runbook fixture through root `sourceGuardRunbookFixtures` metadata and `/launch` `routes[].sourceGuardRunbookFixtures`. That links `reactive-store-lower-dx-check-helper-freshness` to `docs/packages/reactive-store.source-guard-runbook.json` for Zed/DX Studio without executing the Rust manifest or claiming live React runtime proof. The receipt helper also hash-backs `tools/launch/materialize-www-template.ts`, so future changes to that generated preview-manifest emission become stale-detectable through Reactive Store package-status freshness.

## App-Owned Boundaries

Applications still own state shape, actions, mutation policy, persistence, sensitive-state review, render-performance expectations, dependency installation, and browser runtime proof.

## Source Guard

The lower-level helper freshness fixture is published to the Studio/Zed source-guard runbook as `reactive-store-lower-dx-check-helper-freshness`, so operators can rerun the Reactive Store CLI metric proof from `/launch` without scanning Rust tests or claiming live React runtime proof.

The package-owned JSON fixture is `docs/packages/reactive-store.source-guard-runbook.json`. It mirrors the `/launch` runbook command, official **Reactive Store** package metadata, upstream `@tanstack/store` / `@tanstack/react-store` provenance, source-owned template execution policy, Zed/DX Studio marker names, generated preview-manifest exposure, receipt helper handoff, app-owned state boundaries, and `SOURCE-OWNED TEMPLATE STORE` runtime limitations without parsing raw Rust. The receipt hash helper tracks this fixture plus the shared preview-manifest materializer as first-class selected surfaces, so stale Studio/Zed runbook metadata and generated-starter fixture emission drift are visible through package-status/read-model freshness without claiming live React runtime proof.

```powershell
dx run --test .\benchmarks\reactive-store-slice.test.ts
dx run --test .\benchmarks\reactive-store-receipt-hash-refresh.test.ts
dx run --test .\benchmarks\reactive-store-dx-check-package-lane-panel.test.ts
cargo test -q -p dx-www-compiler reactive_store_hash_refresh_stale_helper_keeps_source_hash_clean --lib
cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_reactive_store_package_lane_hash_refresh_row --lib
```
