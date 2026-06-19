# Backend Platform Client

Official DX package: Backend Platform Client
packageId: supabase/client
upstream_package: @supabase/ssr + @supabase/supabase-js
upstream_version: @supabase/ssr latest; @supabase/supabase-js ^2

`supabase/client` is the package id for the Forge-owned Backend Platform Client slice for DX-WWW account and backend readiness flows. It is based on the local upstream mirror at `G:\WWW\inspirations\supabase`, especially the user-management account/profile example and Supabase Studio API docs.

The slice is source-owned by DX-WWW, but it does not vendor Supabase internals. It materializes typed app code that imports the public Supabase APIs through `@supabase/ssr` and `@supabase/supabase-js` once the application chooses its runtime dependencies.

## Public API

- `readSupabasePublicConfig` validates the browser-safe public URL and publishable key boundary.
- `createDxSupabaseBrowserClient` and `createDxSupabaseServerClient` wrap the SSR client factories.
- `getDxSupabaseCurrentProfile` reads the current user and profile row through the app-owned Supabase client.
- `upsertDxSupabaseProfile` prepares the profile update path used by the account dashboard.
- `dxSupabaseProfileFields`, `readDxSupabaseProfileConfigStatus`, `readDxSupabaseProfilesReadModel`, `createDxSupabaseProfilePreview`, `updateDxSupabaseProfileDraft`, and `createDxSupabaseProfileUpsertReceipt` power the `/launch` account-data dashboard without placing workflow logic inside the component.
- Realtime, auth, storage, RPC, and Edge Function helpers expose source-owned API slices while policy remains app-owned.

## Dashboard Usage

The generated `/launch` shell mounts `LaunchSupabaseProfileWorkflow` inside `data-dx-section="account-data-dashboard"` rather than hiding it in the backend proof card. The data dashboard also exposes `data-dx-dashboard-workflow="supabase-schema-query"` for a safe local `profiles` read-model check while hosted credentials stay app-owned.

Stable markers include:

- `data-dx-package="supabase/client"`
- `data-dx-component="supabase-profile-workflow"`
- `data-dx-component="supabase-schema-query-workflow"`
- `data-dx-dashboard-workflow="account-profile-settings"`
- `data-dx-dashboard-workflow="supabase-schema-query"`
- `data-dx-dashboard-card="database-supabase-client"`
- `data-dx-supabase-action="load-profile-fixture"`
- `data-dx-supabase-action="prepare-profile-upsert"`
- `data-dx-supabase-profile-field="fullName"`
- `data-dx-supabase-profile-field="username"`
- `data-dx-supabase-profile-field="website"`
- `data-dx-supabase-profile-label`
- `data-dx-supabase-query-operation`
- `data-dx-supabase-receipt-path="examples/template/.dx/forge/receipts/2026-05-22-supabase-client-dashboard-workflow.json"`
- `<dx-icon name="database:supabase" />`

DX Studio can source-select `supabase-account-data-dashboard`, `supabase-profile-workflow`, and `supabase-schema-query-workflow` through `examples/template/dx-studio-edit-contract.ts`, using responsive design-system edit operations instead of absolute positioning.

The local interaction lets the dashboard edit a profile draft, load a safe local fixture, run the source-owned `readDxSupabaseProfilesReadModel` preview for `supabase.from('profiles').select('id, full_name, username, website')`, and prepare an upsert receipt. If public Supabase env is missing, the component reports that state honestly and does not pretend live writes succeeded.

The editable field list is owned by `dxSupabaseProfileFields`, so field labels, input types, autocomplete hints, and `data-dx-supabase-profile-field` markers stay together instead of being scattered through the TSX surface.

The Zed preview package surface advertises the account profile workflow, the local schema-query workflow, `lib/supabase/metadata.ts`, interaction selectors, receipt paths, and app-owned Supabase boundaries. Generated starters also materialize `.dx/forge/receipts/2026-05-22-supabase-client-dashboard-workflow.json`, so the visible receipt-path marker stays tied to a source-owned receipt instead of silently regressing to a metadata-only card.

## Reality audit

The dashboard workflow receipt classifies the slice as `REAL` for source-owned Forge package code, generated `/launch` materialization, visible account/profile and schema-query interactions, and DX Studio/Web Preview discovery. Hosted Supabase reads, writes, realtime subscriptions, project credentials, RLS rollout, and governed browser QA remain explicit partial runtime boundaries.

## dx-check visibility

The dashboard workflow receipt carries `dx.forge.package.dx_check_visibility` for Backend Platform Client. The current source state is `present` for the selected profile workflow and local schema-query workflow; consumers should also render `stale`, `missing-receipt`, `blocked`, and `unsupported-surface` for receipt drift, absent receipts, app-owned Supabase credentials/runtime proof, or unselected privileged Supabase surfaces.

Supported status labels: present, stale, missing-receipt, blocked, and unsupported-surface.

## dx-style compatibility

The visible Backend Platform Client profile workflow and local schema-query workflow expose `dx.forge.package.dx_style_compatibility` as source-only style evidence. The source markers are:

- `data-dx-style-surface="backend-platform-client"`
- `data-dx-token-scope="supabase/client"`

The compatibility record points at `tools/launch/runtime-template/assets/launch-runtime.css` as the generated CSS boundary and covers `examples/template/supabase-profile-workflow.tsx` plus `examples/template/data-status.tsx`. It is `SOURCE-ONLY` / `ADAPTER-BOUNDARY`: dx-style markers and generated CSS references are checkable, while hosted Supabase credentials, Auth/database runtime behavior, accessibility review, and browser visual QA remain app-owned.

dx-check and package-status consumers should expose `backend_platform_client_dx_style_compatibility_present` and `backend_platform_client_dx_style_compatibility_missing` beside the existing receipt, status, and hash metrics. Zed/DX Studio should use `backend-platform-client:dx-style-compatibility` to jump from the package row to the two visible source surfaces without treating hosted runtime proof as completed.

## Receipt Integrity

The dashboard workflow receipt carries `hash_algorithm: sha256` and `file_hashes` for selected Backend Platform Client dashboard files and docs:

- `docs/packages/supabase-client.md`
- `docs/packages/backend-platform-client.source-guard-runbook.json`
- `core/src/ecosystem/forge_supabase.rs`
- `examples/template/supabase-profile-workflow-state.ts`
- `examples/template/supabase-profile-workflow.tsx`
- `examples/template/data-status.tsx`
- `tools/launch/materialize-www-template.ts`
- `examples/template/lib/supabase/metadata.ts`
- `examples/template/lib/supabase/README.md`
- `examples/template/lib/supabase/schema.sql`
- `examples/template/server/supabase/readiness.ts`
- `examples/template/app/api/supabase/readiness/route.ts`

The package-owned source-guard runbook fixture, generated preview-manifest materializer, and lock-backed Supabase readiness source are hash-backed here, so source-guard runbook fixture drift, generated `sourceGuardRunbookFixtures` emission drift, and provider-gated readiness route drift become stale-detectable through the same Backend Platform Client receipt, package-status row, and typed read model.

Shared `dx-www/src/cli/mod.rs` and `examples/template/package-catalog.ts` remain provenance/visibility surfaces, not stale-gating hash inputs, because other package lanes legitimately edit those shared files.

Refresh selected hash evidence with:

```powershell
node tools/launch/run-template-receipt-helper.js examples/template/backend-platform-client-receipt-hashes.ts --check
node tools/launch/run-template-receipt-helper.js examples/template/backend-platform-client-receipt-hashes.ts --check --json
node tools/launch/run-template-receipt-helper.js examples/template/backend-platform-client-receipt-hashes.ts --write
```

The helper checks or refreshes only Backend Platform Client receipt, package-status, and read-model SHA-256 values. It does not contact hosted Supabase, read project credentials, run migrations, or claim live Auth, database, Storage, Realtime, or Edge Function proof.

## package-status read model

The shared launch package-status read model consumes this receipt as the Backend Platform Client row. It should expose `backend_platform_client_package_present`, `backend_platform_client_receipt_present`, `backend_platform_client_receipt_stale`, `backend_platform_client_missing_receipt`, `backend_platform_client_blocked_surface`, `backend_platform_client_unsupported_surface`, `backend_platform_client_hash_manifest_present`, `backend_platform_client_hash_mismatch`, `backend_platform_client_receipt_hash_refresh_current`, `backend_platform_client_receipt_hash_refresh_stale`, and `backend_platform_client_receipt_hash_refresh_missing` for dx-check and Zed/DX Studio consumers.

The same row now carries `receipt_hash_refresh` / `receiptHashRefresh` with helper path, check/write/json commands, tracked/stale/missing counts, the hash-backed `docs/packages/backend-platform-client.source-guard-runbook.json` fixture path, the hash-backed `tools/launch/materialize-www-template.ts` preview-manifest materializer path, `backend-platform-client:receipt-hash-refresh`, and explicit no-runtime/no-secret flags so DX Studio and Zed can show receipt freshness without opening raw JSON.

The package-status row is `ADAPTER-BOUNDARY`: it proves the selected profile and schema-query surfaces, source markers, receipt path, and file hashes. Hosted Supabase reads, writes, Auth provider behavior, Realtime subscriptions, RLS rollout, and governed browser QA remain app-owned runtime proof.

## DX Studio/check-panel package row

The DX Studio/check-panel Backend Platform Client package row is rendered from `.dx/forge/package-status.json` as `supabase/client`, with official package name `Backend Platform Client` and upstream provenance kept in metadata. It surfaces `backend_platform_client_hash_manifest_present`, `backend_platform_client_hash_mismatch`, `backend_platform_client_receipt_hash_refresh_current`, `backend_platform_client_receipt_hash_refresh_stale`, `backend_platform_client_receipt_hash_refresh_missing`, and `receipt_hash_refresh` so Studio, Zed, and dx-check consumers can see selected receipt freshness without opening the raw receipt file.

The row publishes `backend-platform-client:receipt-hash-refresh` plus helper path, check/write/json commands, tracked/stale/missing file counts, no-secret/no-runtime flags, and runtime limitations. The launch shell and Studio marker registry expose `data-dx-check-package-lane-hash-refresh-status`, `data-dx-check-package-lane-hash-refresh-helper`, `data-dx-check-package-lane-hash-refresh-json-command`, `data-dx-check-package-lane-hash-refresh-zed`, `data-dx-check-package-lane-hash-refresh-tracked-files`, `data-dx-check-package-lane-hash-refresh-stale-files`, and `data-dx-check-package-lane-hash-refresh-missing-files` for visible check-panel discovery.

This is a `SOURCE-ONLY` / `ADAPTER-BOUNDARY` panel proof for selected package files and helper freshness without claiming hosted Supabase runtime proof.

The static /launch Backend Platform Client package-lane row is available before `.dx/receipts/check/check-latest.json` exists. `examples/template/template-shell.tsx` owns the template, and `the static launch runtime template` mirrors `data-dx-check-package-lane-template="supabase/client"`, `data-dx-check-package-lane-row="supabase/client"`, official package naming, upstream provenance, the dashboard workflow receipt path, helper freshness counts, and `backend-platform-client:receipt-hash-refresh` for Zed/DX Studio discovery without claiming hosted Supabase runtime proof.

The generated-starter materialization guard for Backend Platform Client runs `tools/launch/materialize-www-template.ts` into a temporary starter and verifies generated static launch HTML preserves the static `supabase/client` package-lane row, receipt path, helper JSON command, Zed refresh marker, hash-refresh counts, and `data-dx-package="supabase/client"`. It also verifies generated `public/preview-manifest.json` scopes `/`, `/launch`, and `launch-runtime-dx-check-panel` to `supabase/client`, and now links `docs/packages/backend-platform-client.source-guard-runbook.json` through root `sourceGuardRunbookFixtures` plus `/launch` `routes[].sourceGuardRunbookFixtures` with `backend-platform-client-lower-dx-check-helper-freshness`, `SOURCE-ONLY`, `runtimeProof: false`, and `backend-platform-client:receipt-hash-refresh` without claiming hosted Supabase runtime proof.

## Rust dx-check output

`core/src/ecosystem/project_check/backend_platform_client_dx_check.rs` consumes the Backend Platform Client package-status row and its selected receipt evidence from the Rust Forge section. It emits `backend_platform_client_package_present`, `backend_platform_client_receipt_present`, `backend_platform_client_receipt_stale`, `backend_platform_client_missing_receipt`, `backend_platform_client_blocked_surface`, `backend_platform_client_unsupported_surface`, `backend_platform_client_hash_manifest_present`, `backend_platform_client_hash_mismatch`, `backend_platform_client_receipt_hash_refresh_current`, `backend_platform_client_receipt_hash_refresh_stale`, `backend_platform_client_receipt_hash_refresh_missing`, `backend_platform_client_dx_style_compatibility_present`, and `backend_platform_client_dx_style_compatibility_missing`.

The findings are `backend-platform-client-missing-package-status`, `backend-platform-client-missing-receipt`, `backend-platform-client-stale-receipt`, `backend-platform-client-blocked-surface`, `backend-platform-client-unsupported-surface`, `backend-platform-client-hash-mismatch`, and `backend-platform-client-missing-dx-style-compatibility`. These report source-owned visibility, receipt presence, selected-surface boundaries, byte-derived SHA-256 file drift, and missing style-token metadata through the shared project-check hash helper without claiming hosted Supabase runtime proof.

The package-owned Rust fixture `backend_platform_client_hash_mismatch_metric_and_finding_are_byte_derived` writes a temporary Backend Platform Client package-status row and receipt, mutates one hash-backed generated profile workflow file, and proves `backend_platform_client_receipt_stale`, `backend_platform_client_hash_mismatch`, and `backend-platform-client-hash-mismatch` flip together from current bytes.

The package-owned Rust fixture `backend_platform_client_hash_refresh_stale_helper_keeps_source_hash_clean` writes a temporary Backend Platform Client package-status row with a current source hash, flips only `receipt_hash_refresh.status` and `stale_file_count`, and proves `backend_platform_client_receipt_hash_refresh_stale` plus `backend_platform_client_receipt_stale` turn on while `backend_platform_client_hash_mismatch` remains `0`.

The package-owned Rust fixture `backend_platform_client_dx_style_missing_metric_and_finding_flip` writes the same temporary package-status row with `dx.forge.package.dx_style_compatibility`, proves `backend_platform_client_dx_style_compatibility_present = 1`, removes only the dx-style block, and proves `backend_platform_client_dx_style_compatibility_missing = 1` with `backend-platform-client-missing-dx-style-compatibility`.

Run the targetable Rust fixtures when Cargo is allowed and the shared workspace is calm:

```powershell
cargo test -q -p dx-www-compiler backend_platform_client_hash_refresh_stale_helper_keeps_source_hash_clean --lib
cargo test -q -p dx-www-compiler backend_platform_client_dx_style_missing_metric_and_finding_flip --lib
```

The source guard for the Rust output contract remains:

```powershell
dx run --test .\benchmarks\supabase-dx-check-output.test.ts
```

## Backend Platform Client source-guard/runbook fixture

`dx-www/src/cli/studio_manifest.rs` now publishes `backend-platform-client-dx-style-rust-check-output` and `backend-platform-client-lower-dx-check-helper-freshness` in `source_guard_index` and the `/launch` `source_guard_runbook_index`. Zed and DX Studio can list the exact targeted commands `cargo test -q -p dx-www-compiler backend_platform_client_dx_style_missing_metric_and_finding_flip --lib` and `cargo test -q -p dx-www-compiler backend_platform_client_hash_refresh_stale_helper_keeps_source_hash_clean --lib`, the source-only contracts, the dx-style present/missing metrics, and the helper-freshness stale metric while keeping `backend_platform_client_hash_mismatch` byte-derived without claiming hosted Supabase runtime proof.

`docs/packages/backend-platform-client.source-guard-runbook.json` is the package-owned JSON fixture for the same guards. It records the official package name, upstream Supabase provenance, inspected source files, exact Cargo commands, lightweight Node guard command, `/launch` `source_guard_runbook_index` contract, `source_guard_fixture_paths` for `backend-platform-client-lower-dx-check-helper-freshness`, the generated `public/preview-manifest.json` `sourceGuardRunbookFixtures` contract, Zed/DX Studio markers, receipt hash helper, app-owned boundaries, and `SOURCE-ONLY` runtime limitations so tooling can read the Backend Platform Client runbook contract without parsing raw Rust source.

This remains `SOURCE-ONLY` / `ADAPTER-BOUNDARY`: the runbook proves source-owned dx-check output and Studio discovery for selected Backend Platform Client surfaces. Hosted Supabase Auth/database reads and writes, Storage, Realtime, RPC, Edge Functions, accessibility review, and browser visual QA remain app-owned.

## Forge Metadata

- Package id: `supabase/client`
- Aliases: `db/supabase`, `supabase/ssr`, `backend/supabase`
- Source mirror: `G:\WWW\inspirations\supabase`
- Required env: `NEXT_PUBLIC_SUPABASE_URL`, `NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY`
- Runtime dependencies owned by the app: `@supabase/ssr`, `@supabase/supabase-js`
- Exported dashboard APIs: `lib/supabase/profile-workflow.ts`, `lib/supabase/metadata.ts`
- Receipt paths: `.dx/forge/docs/supabase-client.md`, `.dx/forge/receipts/*-supabase-client.json`, `examples/template/.dx/forge/receipts/2026-05-22-supabase-client-dashboard-workflow.json`, `.dx/forge/receipts/2026-05-22-supabase-client-dashboard-workflow.json`, `docs/packages/supabase-client.md`
- DX icon: `<dx-icon name="database:supabase" />`

## App-Owned Boundaries

- Supabase project provisioning and dependency versions.
- Auth redirect allow-list, provider credentials, consent screens, and callback policy.
- Profile table, Storage bucket, RLS policy, and migration rollout.
- Realtime channel authorization, broadcast/presence policy, and token refresh.
- Service-role secrets, admin jobs, and any privileged server-only operations.
- Production form submission, audit trail, rate limits, and profile-field policy.

## No Node Modules Path

The DX/Forge `/launch` proof remains source-owned and does not create a template-local `node_modules` folder. Live Supabase writes become available only after the application installs its selected Supabase dependencies and configures the required public env.

## Source Guard

Run the narrow guard with:

```powershell
dx run --test .\benchmarks\supabase-dashboard-workflow.test.ts
```

## Intentionally Deferred

- Creating a Supabase project or importing credentials.
- Running SQL migrations against a hosted project.
- Claiming hosted reads, writes, auth, or realtime behavior without governed runtime evidence.
- Selecting final profile fields, authorization rules, and audit/logging policy.
