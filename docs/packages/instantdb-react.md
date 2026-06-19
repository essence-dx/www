# Realtime App Database

Package id: `instantdb/react`. Upstream package: `@instantdb/react`.

Realtime App Database is the Forge-owned InstantDB React slice for realtime
DX-WWW dashboard data. It is based on the local upstream mirror at
`G:\WWW\inspirations\instantdb`, especially `client/packages/react`,
`client/packages/react-common`, and the Next app-dir helpers.

The slice is source-owned by DX-WWW, but it does not vendor InstantDB internals
or claim hosted realtime behavior without app configuration. It materializes
typed app code that imports the real public APIs from `@instantdb/react` and
`@instantdb/react/nextjs`.

## Public API

- `init`, `i.schema`, `id`, `lookup`, `db.useQuery`, `db.transact`, and `db.tx`
  cover schema, local ids, queries, and transactions.
- `db.room`, `db.rooms.usePresence`, `db.rooms.useSyncPresence`,
  `db.rooms.useTopicEffect`, `db.rooms.usePublishTopic`, and
  `db.rooms.useTypingIndicator` cover realtime collaboration rooms.
- `db.useAuth`, auth boundary helpers, OAuth readiness helpers, and Next SSR
  helpers expose the session boundary without importing credentials.
- Storage helpers expose `db.storage.uploadFile`, `$files` reads through
  `db.queryOnce`, and file deletion through
  `db.tx.$files[lookup("path", path)].delete()` while file policy remains
  app-owned.
- Stream helpers expose `db.streams.createWriteStream` and readable stream
  readiness while topic retention and payload policy stay app-owned.
- Sync Table helpers expose `SyncTableCallbackEventType`,
  `SyncTableCallbackEvent`, `StoreInterfaceStoreName`, and
  `subscribeInstantLaunchSyncTable`, which delegates to the upstream
  `db.core._syncTableExperimental` surface while local persistence policy and
  runtime proof remain app-owned.
- `createInstantRouteHandler` is surfaced for the app-owned route boundary.
- `/api/instant/readiness` is a provider-gated readiness route backed by
  `server/instant/readiness.ts`. It validates local configuration only and
  does not create a hosted route handler, connect to realtime transport, or
  claim hosted auth, realtime transport, storage, or streams.

## Dashboard Usage

The starter dashboard consumes the package through
`InstantDbDashboardWorkflow` in
`examples/dashboard/src/components/InstantDbDashboardWorkflow.tsx`.

The launch dashboard uses two visible surfaces:

- Source TSX shell: `data-dx-component="launch-instantdb-dashboard-workflow"`
  in `examples/template/template-shell.tsx`.
- Runtime bridge: `data-dx-component="instantdb-runtime-dashboard-workflow"`
  in `the static launch runtime template`.

Stable markers include:

- `data-dx-package="instantdb/react"`
- `data-dx-dashboard-workflow="realtime-data-readiness"`
- `data-dx-instant-readiness="runtime-dashboard-readiness"`
- `data-dx-instant-action="prepare-local-schema-receipt"`
- `data-dx-instant-required-env="NEXT_PUBLIC_INSTANT_APP_ID"`
- `<dx-icon name="pack:database" />`

The local interaction prepares a safe realtime schema receipt for todos,
presence, typing, files, and streams. If `NEXT_PUBLIC_INSTANT_APP_ID` is
missing, the UI reports that state and does not pretend a hosted write or
subscription succeeded.

The default App Router template also exposes `/api/instant/readiness` for
Realtime App Database. That endpoint returns a provider-gated 501 when the
Instant app id is missing, and returns local configuration readiness only when
the app provides `NEXT_PUBLIC_INSTANT_APP_ID`; it is not hosted InstantDB
runtime proof.

## Forge Metadata

- Package id: `instantdb/react`
- Aliases: `@instantdb/react`, `instantdb`, `db/instantdb`
- Source mirror: `G:\WWW\inspirations\instantdb`
- Required env: `NEXT_PUBLIC_INSTANT_APP_ID`
- Runtime dependencies owned by the app: `@instantdb/react`
- Exported dashboard files: `examples/dashboard/src/lib/instantdbDashboard.ts`,
  `lib/instant/sync-table.ts`,
  `lib/instant/dashboard-workflow.ts`,
  `components/dashboard/instantdb-dashboard-workflow.tsx`,
  `server/instant/readiness.ts`,
  `app/api/instant/readiness/route.ts`,
  `examples/dashboard/src/components/InstantDbDashboardWorkflow.tsx`,
  `the static launch runtime template#instantdb-runtime-dashboard-workflow`,
  and `tools/launch/runtime-template/assets/launch-runtime.ts#bindInstantDbRuntimeProof`
- The launch workflow receipt records the full materialized package slice:
  env, schema, client, Next SSR helpers, queries, status, subscriptions,
  pagination, diagnostics, mutations, rules, perms, auth, OAuth, storage,
  streams, Sync Table event helpers, route handlers, provider-gated readiness
  route, metadata, dashboard workflow, todos, cursors, auth boundary, API route,
  and mount page.
- Receipt paths: `.dx/forge/docs/instantdb-react.md`,
  `.dx/forge/receipts/*-instantdb-react.json`,
  `docs/packages/instantdb-react.md`, and
  `examples/template/.dx/forge/receipts/2026-05-22-instantdb-realtime-dashboard.json`
- DX icon: `<dx-icon name="pack:database" />`

## dx-check Visibility

Realtime App Database now publishes `dx.forge.package.dx_check_visibility`
metadata in the Forge template, launch catalog, starter dashboard read model,
dashboard workflow receipt, CLI package JSON, and package-status read model.

- Honesty label: `ADAPTER-BOUNDARY` for the package because credentials,
  rules, app provisioning, and hosted runtime proof stay app-owned.
- Sync Table helper status: `SOURCE-ONLY` until governed hosted Sync Table
  runtime evidence is approved and run.
- Status vocabulary: `present`, `stale`, `missing-receipt`, `blocked`, and
  `unsupported-surface`.
- Monitored surfaces: `instantdb-runtime-dashboard-workflow`,
  `dashboard-instantdb-workflow`, `sync-table-events`,
  `provider-gated-readiness-route`, and
  `realtime-app-database-source-guard-runbook`.
- dx-check metrics: `realtime_app_database_receipt_present`,
  `realtime_app_database_receipt_stale`,
  `realtime_app_database_missing_receipt`,
  `realtime_app_database_blocked_surface`, and
  `realtime_app_database_unsupported_surface`,
  `realtime_app_database_hash_manifest_present`, and
  `realtime_app_database_hash_mismatch`,
  `realtime_app_database_receipt_hash_refresh_current`,
  `realtime_app_database_receipt_hash_refresh_stale`, and
  `realtime_app_database_receipt_hash_refresh_missing`,
  `realtime_app_database_dx_style_compatibility_present`, and
  `realtime_app_database_dx_style_compatibility_missing`.
- Receipt integrity: the dashboard workflow receipt records `hash_algorithm:
  sha256` and selected `file_hashes` for `core/src/ecosystem/forge_instantdb.rs`,
  the launch realtime status/runtime files, and the exported dashboard workflow
  source files. It also hash-backs
  `docs/packages/instantdb-react.source-guard-runbook.json` as the selected
  `realtime-app-database-source-guard-runbook` surface, so stale Studio/Zed
  runbook metadata is visible through receipt freshness. Shared launch
  shell/catalog/package handoff docs/benchmark files remain provenance
  surfaces, not package-freshness inputs.
- Rust dx-check emitter: `core/src/ecosystem/project_check/realtime_app_database_dx_check.rs`
  reads `.dx/forge/package-status.json`, resolves the dashboard workflow
  receipt, publishes `realtime_app_database_package_present`, and raises
  `realtime-app-database-missing-receipt`,
  `realtime-app-database-stale-receipt`,
  `realtime-app-database-blocked-surface`, or
  `realtime-app-database-unsupported-surface` findings from package-status
  evidence without claiming hosted realtime execution. It also reports
  `realtime-app-database-hash-mismatch` when a selected hash-backed source file
  is missing, has a stale SHA-256 digest, or package-status marks that surface
  stale. It also raises
  `realtime-app-database-missing-dx-style-compatibility` when the
  package-status row loses source-owned style evidence. Hosted realtime runtime
  proof remains deferred.
- Package-owned fixture:
  `realtime_app_database_hash_mismatch_flips_when_selected_file_changes` writes
  a temporary generated-root Realtime App Database surface, records its SHA-256
  digest in package-status, mutates the file, and proves the stale metric plus
  `realtime-app-database-hash-mismatch` finding flip together.
- DX Studio/check-panel Realtime App Database package row:
  `core/src/ecosystem/dx_check_receipt.rs` now reads the `instantdb/react`
  package-status row into `check_panel.view_model.package_lane_rows`, preserves
  `@instantdb/react` `0.0.0` and `G:/WWW/inspirations/instantdb` as provenance,
  surfaces selected realtime dashboard source markers, receipt status,
  runtime limitations, `realtime_app_database_hash_manifest_present`,
  `realtime_app_database_hash_mismatch`,
  `realtime_app_database_receipt_hash_refresh_current`,
  `realtime_app_database_receipt_hash_refresh_stale`,
  `realtime_app_database_receipt_hash_refresh_missing`,
  `realtime_app_database_dx_style_compatibility_present`, and
  `realtime_app_database_dx_style_compatibility_missing` without claiming
  hosted Instant runtime proof.
  This row is SOURCE-ONLY without claiming hosted Instant runtime proof.
- Targetable check-panel helper-freshness fixture:
  `dx_check_latest_panel_exposes_realtime_app_database_package_lane_hash_refresh_row`
  writes a temporary Realtime App Database package-status row, verifies
  `realtime_app_database_receipt_hash_refresh_current`, flips only
  `receipt_hash_refresh.status` and `receipt_hash_refresh.stale_file_count`,
  then proves `realtime_app_database_receipt_hash_refresh_stale` rises while
  `realtime_app_database_hash_mismatch` stays zero. This is SOURCE-ONLY and
  does not claim hosted Instant runtime proof.
- Studio/Zed source-guard runbook fixture:
  `docs/packages/instantdb-react.source-guard-runbook.json` publishes
  `realtime-app-database-check-panel-helper-freshness` for `/launch` with the
  exact command
  `cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_realtime_app_database_package_lane_hash_refresh_row --lib`.
  `dx-www/src/cli/studio_manifest.rs` exposes the same fixture path in
  `source_guard_index`, `source_guard_runbook_index.fixture_paths[]`, the
  runbook contract, and the runbook command so Studio/Zed can open the
  SOURCE-ONLY stale-helper proof without parsing prose or claiming hosted
  Instant runtime proof. The fixture is also SHA-256 tracked by the
  Realtime App Database dashboard workflow receipt and package-status row as
  `realtime-app-database-source-guard-runbook`.
- Static `/launch` Realtime App Database package-lane template:
  `examples/template/template-shell.tsx` owns the typed template row and
  `the static launch runtime template` mirrors
  `data-dx-check-package-lane-template="instantdb/react"`, the official package
  name, upstream provenance, dashboard receipt path,
  `data-dx-check-package-lane-hash-refresh-status="current"`,
  `data-dx-check-package-lane-hash-refresh-helper` for
  `examples/template/realtime-app-database-receipt-hashes.ts`,
  `data-dx-check-package-lane-hash-refresh-zed` as
  `realtime-app-database:receipt-hash-refresh`,
  tracked/stale/missing helper counts, the
  `realtime_app_database_receipt_hash_refresh_current`,
  `realtime_app_database_receipt_hash_refresh_stale`, and
  `realtime_app_database_receipt_hash_refresh_missing` metric ids,
  `data-dx-check-package-lane-dx-style-status="present"`,
  `data-dx-style-surface="realtime-app-database"`, and
  `data-dx-token-scope="instantdb/react"` so Studio/Zed can discover the row
  before a fresh dx-check receipt exists. Hosted Instant runtime proof remains
  app-owned.
- Realtime App Database generated-starter materialization guard:
  `benchmarks/instantdb-dx-check-package-lane-panel.test.ts` runs
  `tools/launch/materialize-www-template.ts` into a temporary starter,
  verifies generated static launch HTML keeps the Realtime App Database
  package-lane markers, verifies `public/preview-manifest.json` scopes `/`,
  `/launch`, and `launch-runtime-dx-check-panel` to `instantdb/react`, and
  checks the source edit contract, materializer, and Rust Studio manifest all
  include `instantdb/react` in the dx-check panel package scope without claiming
  hosted Instant runtime proof.
- Realtime App Database receipt-hash refresh helper:
  `examples/template/realtime-app-database-receipt-hashes.ts` checks
  and refreshes the selected SHA-256 receipt manifest, package-status
  `receipt_hash_refresh` row, typed read-model `receiptHashRefresh` mirror, and
  `realtime-app-database:receipt-hash-refresh` Zed/DX Studio visibility. Run
  `node tools/launch/run-template-receipt-helper.js examples/template/realtime-app-database-receipt-hashes.ts --check`
  or
  `node tools/launch/run-template-receipt-helper.js examples/template/realtime-app-database-receipt-hashes.ts --write`.
  The helper reports `source_guard_runbook_fixture` as
  `docs/packages/instantdb-react.source-guard-runbook.json` and tracks seven
  files. It does not run hosted Instant runtime proof, read secrets, install
  dependencies, or claim live subscriptions.

## DX-Style Compatibility

Realtime App Database publishes `dx.forge.package.dx_style_compatibility`
metadata for the visible launch and starter dashboard surfaces without claiming
browser proof.

- Status: `SOURCE-ONLY` for source markers and token compatibility;
  `ADAPTER-BOUNDARY` for hosted Instant runtime configuration.
- Token source: `tools/launch/runtime-template/assets/launch-runtime.css`.
- Generated CSS evidence:
  `tools/launch/runtime-template/assets/launch-runtime.css`.
- Visible surfaces: `instantdb-runtime-dashboard-workflow` and
  `dashboard-instantdb-workflow`.
- Source files:
  `examples/template/instantdb-status.tsx`,
  `the static launch runtime template`, and
  `examples/dashboard/src/components/InstantDbDashboardWorkflow.tsx`.
- Required source marker:
  `data-dx-style-surface="realtime-app-database"`.
- dx-check metrics:
  `realtime_app_database_dx_style_compatibility_present` and
  `realtime_app_database_dx_style_compatibility_missing`.
- Missing evidence finding:
  `realtime-app-database-missing-dx-style-compatibility`.

## App-Owned Boundaries

- Instant app provisioning, endpoint selection, and `NEXT_PUBLIC_INSTANT_APP_ID`.
- Production schema, unique indexes, rules, and auth policy.
- OAuth providers, redirects, token issuance, and authenticated route design.
- Room naming, presence privacy, topic payload policy, and event retention.
- Storage bucket policy, file access rules, upload limits, and retention.
- Stream lifecycle, payload shape, backpressure, and production observability.
- Experimental Sync Table subscriptions, local store retention, unsubscribe
  behavior, and runtime validation.

## No Node Modules Path

The DX/Forge `/launch` proof remains source-owned and does not create a
template-local `node_modules` folder. Hosted InstantDB reads, writes, presence,
storage, and streams become available only after the application installs and
governs its selected InstantDB dependency and configures the required app id.

## Source Guard

Run the narrow guard with:

```powershell
dx run --test .\benchmarks\instantdb-rust-dx-check-visibility.test.ts
dx run --test .\benchmarks\instantdb-receipt-hash-visibility.test.ts
dx run --test .\benchmarks\instantdb-receipt-hash-refresh.test.ts
dx run --test .\benchmarks\instantdb-dx-style-compatibility.test.ts
dx run --test .\benchmarks\instantdb-dx-check-package-lane-panel.test.ts
dx run --test .\benchmarks\instantdb-dx-check-visibility.test.ts
dx run --test .\benchmarks\instantdb-dashboard-workflow.test.ts
node -e "JSON.parse(require('fs').readFileSync('docs/packages/instantdb-react.source-guard-runbook.json','utf8'))"
node .\examples\template\realtime-app-database-receipt-hashes.ts --check --json
cargo test -q -p dx-www-compiler realtime_app_database_hash_mismatch_flips_when_selected_file_changes --lib
cargo test -q -p dx-www-compiler dx_check_reports_realtime_app_database_package_status_visibility --lib
cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_realtime_app_database_package_lane_style_row --lib
cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_realtime_app_database_package_lane_hash_refresh_row --lib
```

## Intentionally Deferred

- Creating or importing an Instant app.
- Claiming hosted realtime subscriptions without governed runtime evidence.
- Deploying production rules, auth policy, storage policy, or stream retention.
- Selecting final product schema, tenant boundaries, and audit behavior.
