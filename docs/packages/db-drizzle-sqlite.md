# Database ORM

`db/drizzle-sqlite` is the Forge-owned SQLite slice for the official DX Database ORM package lane. It is based on the local upstream mirror at `G:\WWW\inspirations\drizzle-orm`, including `drizzle-orm/src/better-sqlite3/driver.ts`, `drizzle-orm/src/better-sqlite3/migrator.ts`, `drizzle-orm/src/sqlite-core/db.ts`, SQLite table/view/query builders, relation helpers, mutation builders, and aggregate helpers.

The slice is source-owned by DX-WWW, but it does not vendor Drizzle internals. It materializes typed application code that imports upstream package public APIs such as `sqliteTable`, `sqliteView`, `relations`, `drizzle`, `migrate`, `eq`, `sql`, `count`, `countDistinct`, `avg`, `placeholder`, `union`, `intersect`, `except`, `onConflictDoUpdate`, `returning`, `withReplicas`, `selectDistinct`, and `$count`.

Upstream package: `drizzle-orm`.

## Public API

- `createDxDrizzleConnection` opens a `better-sqlite3` connection and creates a typed Drizzle database.
- `users`, `posts`, `usersRelations`, and `postsRelations` define the starter schema and relation graph.
- `listPublishedPostPreviews`, `listAuthorsWithPostCounts`, `listLaunchAudience`, `readLaunchDatabaseStats`, and related helpers cover dashboard-ready reads.
- `readDrizzleDashboardOverview` composes stats, post previews, author counts, and recent users for the dashboard starter.
- `readDrizzleDashboardQueryPlan` uses real Drizzle SQLite builders and `.toSQL()` to expose safe SQL/params previews for overview, published-post, and author-count dashboard reads before the app executes database work.
- `readDrizzleDashboardQueryPlanById` and `getDrizzleDashboardQueryPlan` select a single typed query-plan preview by dashboard surface id.
- `createDxDrizzleReplicaSet` wraps upstream `withReplicas` so read APIs route to app-owned read replicas while `insert`, `update`, `delete`, `transaction`, and `run` stay on the primary.
- `readDxDrizzleReplicaReadiness` exposes a source-only readiness contract for read replicas, including `selectDistinct`, `$count`, and app-owned topology boundaries.
- `LaunchDrizzleDashboardData` is the `/launch` dashboard workflow surface that lets operators select a local SQLite read model, preview the SQL plan, and prepare a receipt without connecting a production database.
- `applyDxDrizzleMigrations` wraps the real `drizzle-orm/better-sqlite3/migrator` API.

## Dashboard Usage

The starter dashboard consumes the package through `DrizzleDashboardWorkflow` in `examples/dashboard/src/components/DrizzleDashboardWorkflow.tsx`.

That visible workflow exposes:

- `data-dx-package="db/drizzle-sqlite"`
- `data-dx-component="dashboard-drizzle-workflow"`
- `data-dx-drizzle-action="select-dashboard-query"`
- `data-dx-drizzle-sql-preview`
- `data-dx-drizzle-action="prepare-dashboard-query"`
- `data-dx-drizzle-receipt-path`
- `data-dx-drizzle-runtime-dependencies`
- `<dx-icon name="pack:database" />`

The local interactions switch between dashboard read surfaces, show the query-plan contract from `readDrizzleDashboardQueryPlanById`, and prepare a safe SQLite runtime receipt. The overview preview is backed by a source-owned aggregate `.toSQL()` plan using `count`, `countDistinct`, `sql`, and `leftJoin` instead of a UI-only summary string. The dashboard runtime state is explicitly `missing-runtime` until the app installs `drizzle-orm` and `better-sqlite3`, configures the database path, reviews migrations, and accepts authorization policy.

## Launch Dashboard Usage

The generated `/launch` template consumes the package through `LaunchDrizzleDashboardData` in `examples/template/drizzle-query-proof.tsx`, and the default App Router template exposes `/api/database-orm/readiness` as a runtime-gated local readiness route for Database ORM.

That visible product workflow exposes:

- `data-dx-component="launch-drizzle-data-workflow"`
- `data-dx-dashboard-workflow="sqlite-read-model"`
- `data-dx-product-surface="launch-data-dashboard"`
- `data-dx-dashboard-target="mission-control-database"`
- `data-dx-source="examples/template/drizzle-query-proof.tsx"`
- `data-dx-drizzle-action="select-read-model"`
- `data-dx-drizzle-action="preview-query-plan"`
- `data-dx-drizzle-action="apply-read-model"`
- `data-dx-drizzle-query-plan-id`
- `data-dx-backend-status`
- `data-dx-backend-detail`
- `data-dx-drizzle-receipt-path`
- `data-dx-drizzle-receipt-state`
- `data-dx-drizzle-runtime-dependencies="drizzle-orm,better-sqlite3"`

It powers the database section as a local read-model dashboard: operators can switch between launch pipeline, published content, and author workload fixtures, inspect the SQL preview, and prepare a safe app-owned runtime receipt. Applying or previewing the read model also updates the mission-control database card through `data-dx-backend-status` and `data-dx-backend-detail`, so the package behavior cannot silently disappear behind catalog metadata.

The DX Studio/Web Preview edit contract maps `launch-drizzle-data-workflow` to `examples/template/drizzle-query-proof.tsx`, carries the read-model action selectors plus `data-dx-drizzle-receipt-path` and `data-dx-drizzle-runtime-dependencies`, and materializes the same runtime surface into `public/preview-manifest.json` so Zed can select the visible workflow rather than a passive catalog row. The source-owned template surface registry also includes the Drizzle receipt and runtime dependency markers in the shared `data-backend` slot contract.

`dx www preview-manifest --json` also exposes the `db/drizzle-sqlite` package surface as `Database ORM` with `LaunchDrizzleDashboardData`, `readDrizzleDashboardOverview`, `readDrizzleDashboardQueryPlan`, `readDrizzleDashboardQueryPlanById`, the same interaction selectors, and the source-owned dashboard receipt.

Generated `public/preview-manifest.json` snapshots now link `docs/packages/database-orm.source-guard-runbook.json` through root `sourceGuardRunbookFixtures` plus the `/launch` `routes[].sourceGuardRunbookFixtures` list. This gives Zed/DX Studio a direct generated-starter pointer to the package-owned Database ORM runbook fixture without executing SQLite, installing `better-sqlite3`, or parsing Rust manifest text.

## Forge Metadata

- Package id: `db/drizzle-sqlite`
- Official DX package name: `Database ORM`
- Aliases: `database/drizzle`, `db/drizzle`, `drizzle`, `drizzle-orm/sqlite`, `drizzle/sqlite`
- Upstream package: `drizzle-orm`
- Source mirror: `G:\WWW\inspirations\drizzle-orm`
- Required env: none
- Runtime dependencies owned by the app: `drizzle-orm`, `better-sqlite3`
- Dev dependencies owned by the app: `drizzle-kit`, `@types/better-sqlite3`
- Receipt paths: `.dx/forge/docs/db-drizzle-sqlite.md`, `.dx/forge/receipts/*-db-drizzle-sqlite.json`, `examples/template/.dx/forge/receipts/2026-05-22-db-drizzle-sqlite-dashboard-workflow.json`, `docs/packages/database-orm.mirror-drift.fixture.json`, `.dx/forge/source-manifest.json`
- DX icon: `<dx-icon name="pack:database" />`
- Honesty label: `SOURCE-ONLY`

## dx-check visibility

The launch dashboard receipt exposes `dx.forge.package.dx_check_visibility` for `db/drizzle-sqlite`. Consumers should render the current state as `present` for the selected Database ORM source surfaces and understand the full status legend as present, stale, missing-receipt, blocked, and unsupported-surface.

The monitored Database ORM surfaces are `db/drizzle/replicas.ts` for upstream `withReplicas` routing, `components/launch/drizzle-query-proof.tsx` for the Zed/DX Studio visible dashboard workflow, `docs/packages/database-orm.source-guard-runbook.json` for the source-guard runbook fixture, and the lock-backed source surface covering `db/drizzle/schema.ts`, `db/drizzle/metadata.ts`, `db/drizzle/README.md`, `server/database-orm/readiness.ts`, and `app/api/database-orm/readiness/route.ts`. Live SQLite reads, production migration execution, and read-replica health checks remain app-owned runtime proof, not dx-check source proof.

The shared dx-check/Zed package-status read model consumes this receipt through `examples/template/.dx/forge/package-status.json`, `examples/template/forge-package-status-read-model.ts`, and `examples/template/forge-package-status.ts`. It exposes `database_orm_receipt_present`, `database_orm_receipt_stale`, `database_orm_missing_receipt`, `database_orm_blocked_surface`, and `database_orm_unsupported_surface` metrics plus Zed receipt surfaces for `database-orm:drizzle-replica-routing` and `database-orm:drizzle-launch-dashboard-workflow`.

The DX Studio/check-panel Database ORM package row now consumes the same package-status row through `core/src/ecosystem/dx_check_receipt.rs`, preserving `Database ORM` as the user-facing package name while keeping `drizzle-orm` `0.45.3` and `G:/WWW/inspirations/drizzle-orm` as provenance. The row exposes selected replica-routing and launch-dashboard source markers, receipt status, runtime limitations, `database_orm_hash_manifest_present`, `database_orm_hash_mismatch`, `database_orm_receipt_hash_refresh_current`, `database_orm_receipt_hash_refresh_stale`, `database_orm_receipt_hash_refresh_missing`, `database_orm_dx_style_compatibility_present`, and `database_orm_dx_style_compatibility_missing` without claiming live SQLite read proof.

The static `/launch` runtime also carries a receipt-less, source-selection marker for the same package row: `data-dx-check-package-lane-template="db/drizzle-sqlite"`, `data-dx-check-package-lane-row="db/drizzle-sqlite"`, `data-dx-check-package-lane-name="Database ORM"`, upstream provenance, receipt path, and `data-dx-check-package-lane-dx-style-status="present"`. The DX Studio edit contract, launch runtime materializer, and Studio manifest attach `db/drizzle-sqlite` to the dx-check health panel package filter so Zed/DX Studio can discover the Database ORM package lane before a fresh dx-check receipt exists.

The Database ORM dashboard receipt now carries `hash_algorithm: sha256`, `files`, and `file_hashes` for the selected Drizzle Forge source and launch dashboard files. The package-status row mirrors those hashes per selected surface, adds `database_orm_hash_manifest_present` and `database_orm_hash_mismatch`, and lets dx-check mark the package stale when a hash-backed selected file changes or disappears. Runtime database execution, migration rollout, and replica health still stay outside this source-only proof.

The Database ORM package owns `examples/template/database-orm-receipt-hashes.ts` for source-hash freshness. `node tools/launch/run-template-receipt-helper.js examples/template/database-orm-receipt-hashes.ts --check` reports `dx.forge.package.receipt_hash_refresh` JSON with `database-orm:receipt-hash-refresh`, `source_guard_runbook_fixture`, `preview_manifest_materializer`, 12 tracked SHA-256 files, and path-level `current_files`, `stale_files`, `missing_files`, `stale_mirror_files`, `missing_mirror_files`, and `mirror_problem_count` attribution. `--write` refreshes the dashboard workflow receipt, `.dx/forge/package-status.json`, and `forge-package-status-read-model.ts` with the same current attribution arrays, including `database-orm-lock-backed-source` hashes for the App Router readiness route, server readiness contract, schema, metadata, and package README. `--write-mirror-drift-fixture` writes `docs/packages/database-orm.mirror-drift.fixture.json`, a SOURCE-ONLY mirror-only stale fixture where selected Drizzle source hashes remain current, `stale_files` stays empty, `stale_mirror_files` points at `tools/launch/materialize-www-template.ts`, and `mirror_problem_count` is `3` for the receipt, package-status, and read-model mirror entries that a Studio/check-panel row should surface. The selected `database-orm-source-guard-runbook`, `database-orm-preview-manifest-materializer`, and `database-orm-lock-backed-source` surfaces make stale Studio/Zed runbook fixture drift, generated preview-manifest fixture emission drift, and lock-backed source drift visible through package-status/read-model freshness, while the materializer-only drift guard proves selected Drizzle source and dashboard files stay current when only `tools/launch/materialize-www-template.ts` changes. The DX Studio/check-panel Database ORM package row now promotes receipt_hash_refresh helper freshness into first-class current/stale/missing metrics, stale-row next actions, the exact helper path arrays, and the `mirror_problem_count` aggregate through `DxCheckPanelPackageLaneHashRefreshRow`. The helper does not run live SQLite reads, install better-sqlite3, or read database secrets; it only compares local SHA-256 hashes and keeps app-owned runtime boundaries explicit.

Core `dx check` consumes the same package-status row through the Forge section. It emits `database_orm_package_present` plus the `database_orm_*` receipt/status/hash metrics, including `database_orm_receipt_hash_refresh_current`, `database_orm_receipt_hash_refresh_stale`, and `database_orm_receipt_hash_refresh_missing`, and it raises `database-orm-missing-receipt`, `database-orm-stale-receipt`, `database-orm-hash-mismatch`, `database-orm-blocked-surface`, or `database-orm-unsupported-surface` findings when the package-status row, selected surfaces, helper freshness metadata, or hash-backed selected files report those states. It also emits `database_orm_dx_style_compatibility_present` / `database_orm_dx_style_compatibility_missing` and raises `database-orm-missing-dx-style-compatibility` when the package-status row loses the source-owned style evidence. This remains source/receipt proof only; SQLite execution, migrations, replica health, and live visual QA are still app-owned.

`docs/packages/database-orm.source-guard-runbook.json` publishes `database-orm-lower-dx-check-helper-freshness` as the package-owned Studio/Zed runbook fixture for that lower-level helper freshness proof. The fixture records the exact source-only Cargo command, upstream Drizzle provenance, `database_orm_receipt_hash_refresh_*` metrics, `database_orm_hash_mismatch` byte-boundary behavior, Zed package-lane markers, receipt hash helper handoff, generated preview-manifest fields, lock-backed source file coverage, and app-owned SQLite runtime boundaries without claiming live SQLite read proof. The receipt helper now tracks this fixture as the selected `database-orm-source-guard-runbook` surface, `tools/launch/materialize-www-template.ts` as the selected `database-orm-preview-manifest-materializer` surface, and the App Router readiness/source files as `database-orm-lock-backed-source`, so fixture edits, generated preview-manifest emission drift, and lock-backed source drift become stale-detectable through `source_guard_runbook_fixture`, `preview_manifest_materializer`, and the selected surface file hashes.

## DX-Style Compatibility

The visible `LaunchDrizzleDashboardData` source declares `data-dx-style-surface="database-orm"` and uses DX token classes such as `bg-background`, `text-muted-foreground`, `border`, and `bg-muted` instead of inline style objects or hardcoded colors.

The Database ORM dashboard receipt and shared package-status row expose `dx.forge.package.dx_style_compatibility` for `launch-drizzle-data-workflow`, with `tools/launch/runtime-template/assets/launch-runtime.css` recorded as both the token source and generated CSS surface. The row adds `database_orm_dx_style_compatibility_present` and `database_orm_dx_style_compatibility_missing` so dx-check/Zed consumers can distinguish source-owned style compatibility from governed browser visual proof.

Runtime visual proof is still deferred: live SQLite data rendering, theme-token review, and browser QA remain app-owned and are not claimed by this source-only receipt.

## App-owned boundaries

- SQLite database path or `DATABASE_URL`.
- Migration SQL generation, review, and rollout order.
- Backups, retention, permissions, and hosted data policy.
- Dashboard authorization, tenant boundaries, and audit behavior.
- View definitions, CTE aliases, aggregation semantics, and business KPIs.
- Query-plan display policy, explain-plan review, index decisions, and SQL logging/redaction policy.
- Read-replica topology, replica health, routing policy, and write-after-read consistency.
- Conflict targets, merge policy, mutation authorization, and soft-delete policy.
- Any driver beyond the current SQLite `better-sqlite3` path.

## No Node Modules Path

The DX/Forge dashboard proof keeps a no `node_modules` workflow for generated starter review. Runtime execution remains app-owned: once the app chooses to run real SQLite reads, it must install and govern the declared runtime packages explicitly.

## Source Guard

`dx run --test .\benchmarks\drizzle-dashboard-workflow.test.ts` verifies that the dashboard starter imports the Drizzle workflow, the workflow preserves `data-dx-*` package and interaction markers, the package doc names the source mirror and app-owned boundaries, the component uses DX Icons metadata, no `.sr` output is introduced, and official starter UI avoids hardcoded color classes.

`dx run --test .\benchmarks\drizzle-sqlite-replica-routing-slice.test.ts` verifies that the Database ORM slice exposes upstream SQLite `withReplicas`, `selectDistinct`, and `$count` as a source-owned, app-owned read-replica boundary.

`dx run --test .\benchmarks\drizzle-dx-check-visibility-receipt.test.ts` verifies that the Database ORM dashboard receipt, Forge metadata, launch catalog, visible workflow source, and docs expose official naming plus dx-check and dx-style visibility states.

`dx run --test .\benchmarks\drizzle-package-status-read-model.test.ts` verifies that the shared package-status JSON, typed read model, exported launch status surface, package docs, Rust dx-check module, and Database ORM dashboard receipt expose receipt-backed Database ORM dx-check visibility, dx-style compatibility, dx-style missing findings, source-guard runbook fixture freshness, and SHA-256 hash manifest metadata for dx-check and Zed consumers.

`dx run --test .\benchmarks\drizzle-dx-check-package-lane-panel.test.ts` verifies that the Database ORM DX Studio/check-panel package row exposes official naming, upstream provenance, selected source markers, SHA-256 freshness metrics, `receipt_hash_refresh` helper freshness metrics, helper path arrays, `mirror_problem_count`, generated preview-manifest runbook fixture metadata, static `/launch` package-lane markers, Studio package filters, and dx-style present/missing metrics without claiming live SQLite read proof. It also covers the stale-helper-only Database ORM check-panel fixture: `receipt_hash_refresh.stale_file_count` flips the helper freshness metrics and stale row state while `database_orm_hash_mismatch` remains byte-derived from selected source files.

`dx run --test .\benchmarks\drizzle-receipt-hash-refresh.test.ts` verifies that the Database ORM receipt-hash helper checks and refreshes SHA-256 file hashes across the dashboard workflow receipt, package-status row, typed read model, source-guard runbook fixture, preview-manifest materializer, mirror-only drift fixture, path-level current/stale/missing attribution arrays, and Zed `database-orm:receipt-hash-refresh` surface without claiming live SQLite runtime proof.

`cargo test -q -p dx-www-compiler dx_check_reports_database_orm_package_status_visibility` verifies that Rust `dx check` maps the Database ORM package-status row into Forge section metrics and reports a missing-receipt finding when the dashboard workflow receipt disappears.

`cargo test -q -p dx-www-compiler database_orm_dx_style_compatibility_missing_is_reported --lib` verifies that a Database ORM package-status row without `dx_style_compatibility` flips `database_orm_dx_style_compatibility_missing` and raises `database-orm-missing-dx-style-compatibility` while the dashboard workflow receipt remains present.

`cargo test -q -p dx-www-compiler database_orm_hash_refresh_stale_helper_keeps_source_hash_clean --lib` verifies that Rust `dx check` maps Database ORM `receipt_hash_refresh` into current/stale/missing helper metrics and treats stale helper metadata as stale receipt evidence while keeping `database_orm_hash_mismatch` tied only to selected source bytes.

`docs/packages/database-orm.source-guard-runbook.json` is the package-owned source-guard runbook fixture for `database-orm-lower-dx-check-helper-freshness`, and the Zed/DX Studio manifest exposes it through `source_guard_index`, `/launch` `source_guard_runbook_index`, and structured `fixture_path` metadata.

`dx run --test .\benchmarks\drizzle-launch-proof.test.ts` verifies that the generated `/launch` route carries the Drizzle read-model workflow, mission-control database markers, DX Studio interaction selectors, runtime-safe interactions, and source-owned dashboard receipt without a template-local `node_modules` folder. `node --test .\benchmarks\lane4-runtime-safe-readiness-route.test.ts` verifies the Database ORM readiness route returns an honest runtime-gated response without opening SQLite, running migrations, or querying data.

## Intentionally Deferred

- Live SQLite reads before the app has installed `drizzle-orm` and `better-sqlite3`.
- Automatic Drizzle Kit migration generation.
- Production migration execution during template generation.
- Production read replicas, health checks, failover, and consistency policy.
- Hosted database adapters beyond the current SQLite `better-sqlite3` path.
- Tenant-specific authorization, audit logging, and backup/restore orchestration.
- Live runtime proof from generated app SQLite files; the current helper refreshes source hashes from this DX-WWW checkout and path-compatible generated-starter files only.
