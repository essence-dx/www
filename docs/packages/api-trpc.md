# Type-Safe API

Official DX package: `Type-Safe API`

Package id: `api/trpc`

Upstream provenance: `@trpc/server`, `@trpc/client`, and
`@trpc/tanstack-react-query` from the local source mirror at
`G:\WWW\inspirations\trpc`, including `packages/server`, `packages/client`, and
`packages/tanstack-react-query`.

The Type-Safe API slice is source-owned by DX-WWW. It does not vendor tRPC
internals or create template-local `node_modules`; it materializes application
code that imports real public APIs such as `initTRPC.context().create()`,
`fetchRequestHandler`, `createTRPCClient`, `httpBatchLink`, `queryOptions`,
`mutationOptions`, `infiniteQueryOptions`, and `subscriptionOptions`.

## Public API

- `createDxTrpcContext` creates request context with headers, request id, and an
  app-provided session.
- `appRouter`, `AppRouter`, `AppRouterInputs`, and `AppRouterOutputs` provide a
  typed starter router with health, launch event, paginated feed, and
  subscription procedures.
- `createDxTrpcRouteHandler` adapts the router through tRPC's fetch adapter.
- `createDxTrpcClient`, `createDxTrpcStreamingClient`, and
  `createDxTrpcSubscriptionClient` cover batch, stream, and subscription
  transport boundaries.
- `DxTrpcProvider` wires TanStack Query and the tRPC options proxy.
- `TrpcDashboardWorkflow` is the public dashboard workflow component for the
  starter workflow.
- `trpc-launch-contract.ts` owns the launch route/procedure contract plus local
  health and `launchEvent` receipt helpers consumed by the `/launch` workflow.

## Dashboard Usage

The starter dashboard consumes the package through `TrpcDashboardWorkflow` in
`examples/dashboard/src/components/TrpcDashboardWorkflow.tsx`. The Forge package
also materializes the same workflow at
`components/dashboard/trpc-dashboard-workflow.tsx` with its metadata helper at
`lib/trpc/dashboard-workflow.ts`.

The generated `/launch` route now consumes the package as a product dashboard
workflow instead of only a catalog-readiness card. `LaunchAccountDataDashboard`
mounts the tRPC surface with
`data-dx-component="launch-trpc-api-dashboard-workflow"`,
`data-dx-dashboard-card="typed-api"`, and
`data-dx-dashboard-workflow="typed-api-readiness"`. The runtime-safe page also
wires mission-control buttons for `data-dx-trpc-action="check-health"` and
`data-dx-trpc-action="prepare-launch-event"`.

That visible workflow exposes:

- `data-dx-package="api/trpc"`
- `data-dx-component="dashboard-trpc-workflow"`
- `data-dx-dashboard-workflow="typed-api-boundary"`
- `data-dx-trpc-action="select-procedure"`
- `data-dx-trpc-action="prepare-local-receipt"`
- `data-dx-trpc-action="check-health"` on `/launch`
- `data-dx-trpc-action="prepare-launch-event"` on `/launch`
- `<dx-icon name="api:trpc" />`

The local interactions prepare safe typed health and launch-event receipts. They
do not claim a network call until the app has installed runtime dependencies,
mounted the router, connected auth/session context, and accepted
transport/cache policy.

The runtime materializer also copies the dashboard receipt into generated apps
and exposes `launch-runtime-trpc-api-dashboard` in `public/preview-manifest.json`
so Zed Web Preview can select the typed API workflow directly from `/launch`.
The source-owned Studio manifest now also indexes
`launch-trpc-api-dashboard-workflow`, `trpc-launch-health-workflow`, the tRPC
action markers, and the same receipt path, so Zed can map the live dashboard
controls back to the source instead of falling through to older catalog-only
surfaces.
The generated App Router page and runtime `.html` page now rely on that dashboard
surface instead of materializing a separate legacy route/runtime-only surface.
The shared database/backend card now links to `#mission-trpc` with
`data-dx-backend-action="open-typed-api-dashboard"` instead of carrying a second
legacy health button, so the visible tRPC behavior has one canonical dashboard
owner.
The runtime-safe `/backend` route now follows the same rule: its tRPC surface is
`data-dx-component="trpc-backend-workflow"` and links back to
`/launch#mission-trpc` with `data-dx-trpc-action="open-launch-workflow"` instead
of keeping a separate backend health button.

## Forge Metadata

- Official DX package name: `Type-Safe API`
- Package id: `api/trpc`
- Upstream package: `@trpc/server`
- Aliases: `trpc`, `trpc/next`, `@trpc/server`, `@trpc/client`,
  `@trpc/tanstack-react-query`
- Source mirror: `G:\WWW\inspirations\trpc`
- Required env: none
- Runtime dependencies owned by the app: `@trpc/server`, `@trpc/client`,
  `@trpc/tanstack-react-query`, `@tanstack/react-query`, `zod`, React, Next.js
- Exported dashboard files: `lib/trpc/dashboard-workflow.ts`,
  `components/dashboard/trpc-dashboard-workflow.tsx`,
  `components/launch/trpc-launch-contract.ts`,
  `components/launch/trpc-launch-health.tsx`
- Receipt paths: `.dx/forge/receipts/2026-05-22-api-trpc-dashboard-workflow.json`,
  `.dx/forge/receipts/api-trpc.json`,
  `.dx/forge/template-readiness/launch-route.json`,
  `.dx/forge/template-readiness/launch-runtime-checklist.json`
- DX icon: `<dx-icon name="api:trpc" />`
- Local contract exports recorded in the dashboard receipt:
  `trpcLaunchContract`, `HealthCheckResult`, `LaunchEventResult`,
  `TrpcLaunchWorkflowResult`, `createLocalHealthCheck`, and
  `createLocalLaunchEvent`

## dx-check Visibility

The dashboard workflow receipt exposes `dx.forge.package.dx_check_visibility`
for the selected Type-Safe API surfaces. dx-check can report present, stale, missing-receipt, blocked, and unsupported-surface states without running a server or installing dependencies.

The receipt and package-status row now record `hash_algorithm: sha256` plus
`file_hashes` for eleven selected source-owned files:
`core/src/ecosystem/forge_trpc.rs`,
`examples/template/trpc-launch-health.tsx`,
`examples/template/trpc-launch-contract.ts`,
`examples/dashboard/src/components/TrpcDashboardWorkflow.tsx`,
`examples/dashboard/src/lib/trpcDashboardWorkflow.ts`,
`docs/packages/api-trpc.source-guard-runbook.json`,
`tools/launch/materialize-www-template.ts`,
`examples/template/.dx/forge/template-readiness/database-api.json`,
`examples/template/app/api/trpc/health/route.ts`,
`examples/template/lib/trpc/metadata.ts`, and
`examples/template/lib/trpc/README.md`. Rust dx-check uses the shared
byte-level SHA-256 helper, so stale Type-Safe API visibility is now byte-derived
when one of those eleven selected source-owned files is missing or changed.

Current monitored surfaces:

- `trpc-launch-dashboard-workflow` materialized as
  `components/launch/trpc-launch-health.tsx`
- `trpc-starter-dashboard-workflow` materialized as
  `components/dashboard/trpc-dashboard-workflow.tsx`
- `trpc-route-handler` materialized as `app/api/trpc/[trpc]/route.ts`
- `type-safe-api-source-guard-runbook` tracking
  `docs/packages/api-trpc.source-guard-runbook.json`
- `type-safe-api-preview-manifest-materializer` tracking
  `tools/launch/materialize-www-template.ts`
- `type-safe-api-template-readiness-receipt` tracking
  `examples/template/.dx/forge/template-readiness/database-api.json`
- `type-safe-api-lock-backed-source` tracking
  `examples/template/app/api/trpc/health/route.ts`,
  `examples/template/lib/trpc/metadata.ts`, and
  `examples/template/lib/trpc/README.md`

The metrics are `type_safe_api_receipt_present`,
`type_safe_api_receipt_stale`, `type_safe_api_missing_receipt`,
`type_safe_api_blocked_surface`, and
`type_safe_api_unsupported_surface`, plus
`type_safe_api_hash_manifest_present` and `type_safe_api_hash_mismatch`. They
prove source/receipt/hash visibility only; live tRPC execution remains
app-owned and governed separately.

The shared launch package-status read model now consumes the same receipt as a
**Type-Safe API** row in
`examples/template/.dx/forge/package-status.json`,
`examples/template/forge-package-status-read-model.ts`, and
`examples/template/forge-package-status.ts`. That row exposes the
selected launch dashboard, starter dashboard, and route-handler surfaces to
dx-check and Zed/DX Studio with `type-safe-api:*` receipt surface ids while
keeping upstream package names as provenance metadata.

Rust `dx check` now consumes this shared row through the package-lane checker in
`core/src/ecosystem/project_check/type_safe_api_dx_check.rs`. It emits
`type_safe_api_package_present` plus the receipt, stale, missing-receipt,
blocked, unsupported-surface, hash-manifest, and hash-mismatch metrics, and raises
`type-safe-api-missing-package-status`, `type-safe-api-missing-receipt`,
`type-safe-api-stale-receipt`, `type-safe-api-blocked-surface`, or
`type-safe-api-unsupported-surface`, or `type-safe-api-hash-mismatch` findings
from package-status, receipt state, and selected-surface hashes. This is
SOURCE-ONLY / ADAPTER-BOUNDARY evidence; it does not claim a live route runtime.
The package-owned fixture
`type_safe_api_hash_mismatch_flips_when_selected_file_changes` proves the clean
hash path by writing a temporary Type-Safe API package-status row, recording the
current SHA-256 for `trpc-launch-health.tsx`, mutating that selected file, and
verifying `type_safe_api_receipt_stale`, `type_safe_api_hash_mismatch`, and
`type-safe-api-hash-mismatch` flip together.

The DX-WWW check-panel reader now mirrors the same package-status row into
`check_panel.view_model.package_lane_rows`. The **Type-Safe API** row carries
the `receipt_hash_refresh` helper handoff for
`type-safe-api:receipt-hash-refresh` and emits
`type_safe_api_receipt_hash_refresh_current`,
`type_safe_api_receipt_hash_refresh_stale`, and
`type_safe_api_receipt_hash_refresh_missing` beside the package, receipt,
surface, and SHA-256 metrics. This makes stale helper state visible in DX
Studio/Zed without treating source-only evidence as live tRPC runtime proof.
The row also preserves the helper's `current_files`, `stale_files`,
`missing_files`, `stale_mirror_files`, and `missing_mirror_files` path arrays,
so Studio can name the exact stale helper or mirror path without opening the raw
helper JSON.

The check-panel row now keeps unsupported requested Type-Safe API surfaces
visible instead of collapsing them into a counter. If a generated app asks for
an unsupported surface such as `trpc-websocket-subscriptions`, the row adds that
request to `selected_surfaces` with `status: "unsupported-surface"`, a reason,
and the app-owned boundary that must be solved before Forge can claim support.
The guard keeps this SOURCE-ONLY / ADAPTER-BOUNDARY path honest for subscription
transport, connection authorization, stream fan-out, retry policy, and hosted
runtime limits.

`docs/packages/api-trpc.source-guard-runbook.json` is the package-owned JSON
fixture for the `type-safe-api-unsupported-surface-context` source guard. It
records the official **Type-Safe API** package metadata, upstream tRPC
provenance, inspected source files, exact Cargo proof command, `/launch`
`source_guard_runbook_index` contract, dx-check marker names, receipt hash
helper handoff, app-owned boundaries, and `SOURCE-ONLY` runtime limitations
without claiming live tRPC subscriptions.

The Type-Safe API static launch package-lane template is also present in the
receipt-less `/launch` dx-check panel with
`data-dx-check-package-lane-template="api/trpc"`,
`data-dx-check-package-lane-row="api/trpc"`, and
`type-safe-api:receipt-hash-refresh` helper markers. It points to
`examples/template/type-safe-api-receipt-hashes.ts`, tracks eleven
selected source-owned files, and keeps the status at `missing` /
`missing-receipt` until a real dx-check receipt is loaded, without claiming
live tRPC route execution.

The Type-Safe API static helper metric markers now mirror the shared
check-panel metric names:
`type_safe_api_receipt_hash_refresh_current`,
`type_safe_api_receipt_hash_refresh_stale`, and
`type_safe_api_receipt_hash_refresh_missing`. Zed/DX Studio can discover those
metric ids from the static source and generated starter before a fresh
`check_panel.view_model.package_lane_rows` receipt exists.
The same static helper path array markers expose
`data-dx-check-package-lane-hash-refresh-current-file-list`,
`data-dx-check-package-lane-hash-refresh-stale-file-list`,
`data-dx-check-package-lane-hash-refresh-missing-file-list`,
`data-dx-check-package-lane-hash-refresh-stale-mirror-file-list`, and
`data-dx-check-package-lane-hash-refresh-missing-mirror-file-list`, so a
receipt-less static `/launch` page can still show which Type-Safe API helper or
mirror paths are current, stale, or missing without claiming live tRPC route
execution.

The generated-starter materialization guard for Type-Safe API now runs
`tools/launch/materialize-www-template.ts` into a temporary starter and
checks the generated static launch HTML plus `public/preview-manifest.json`.
That guard proves `api/trpc` keeps its static dx-check package-lane markers,
dashboard workflow receipt path, `type-safe-api:receipt-hash-refresh`, and
`launch-runtime-dx-check-panel` package scope after materialization, without
claiming live Type-Safe API runtime proof.
Generated `public/preview-manifest.json` now also exposes
`docs/packages/api-trpc.source-guard-runbook.json` through the root
`sourceGuardRunbookFixtures` array and the `/launch`
`routes[].sourceGuardRunbookFixtures` entry. That gives Zed/DX Studio a
machine-readable path from generated starter output back to the package-owned
`type-safe-api-unsupported-surface-context` runbook without claiming live
Type-Safe API runtime proof or live tRPC subscriptions.
The same `docs/packages/api-trpc.source-guard-runbook.json` fixture and the
`tools/launch/materialize-www-template.ts` preview-manifest materializer
are now hash-backed selected surfaces in the Type-Safe API receipt,
package-status row, and typed read model. The helper reports
`source_guard_runbook_fixture`, `preview_manifest_materializer`, and eleven
tracked files so runbook or generated preview-manifest metadata drift becomes
stale-detectable through `type-safe-api:receipt-hash-refresh`.

## Receipt Hash Refresh

`node tools/launch/run-template-receipt-helper.js examples/template/type-safe-api-receipt-hashes.ts --check`
verifies the dashboard workflow receipt, `.dx/forge/package-status.json`, and
`forge-package-status-read-model.ts` all agree with current SHA-256 bytes for
the selected Type-Safe API source files. Use
`node tools/launch/run-template-receipt-helper.js examples/template/type-safe-api-receipt-hashes.ts --write` after
reviewing intentional Type-Safe API source edits to refresh those mirrors
together. The helper emits `dx.forge.package.receipt_hash_refresh` with
`type-safe-api:receipt-hash-refresh` for Zed/DX Studio handoff, including
`source_guard_runbook_fixture` for
`docs/packages/api-trpc.source-guard-runbook.json` and
`preview_manifest_materializer` for
`tools/launch/materialize-www-template.ts`. The shared
package-status/read-model row mirrors the same helper freshness so stale helper
state is visible without opening the raw receipt JSON. It does not run live tRPC
route execution, install dependencies, contact a server, read secrets, or claim
production router/auth/persistence proof.

The helper JSON also reports `tracked_files`, `current_files`, `stale_files`,
`missing_files`, `stale_mirror_files`, `missing_mirror_files`, and
`mirror_problem_count`. The focused fixture in
`dx run --test .\benchmarks\trpc-receipt-hash-refresh.test.ts` seeds only
`docs/packages/api-trpc.source-guard-runbook.json` and
`tools/launch/materialize-www-template.ts` with stale hashes, then proves
those two helper surfaces are attributed without marking the route handler,
launch health contract, starter dashboard component, dashboard helper, or
Forge slice stale.

## App-owned boundaries

- Domain routers, procedures, authorization, and session policy.
- Runtime dependency installation and package-manager governance.
- Request limits, request id propagation, auth token source, and cross-origin
  headers.
- Persistence, audit logging, rate limiting, error redaction, and production
  observability.
- Pagination cursor semantics, event retention, subscription fan-out, stream
  pacing, JSONL/proxy compatibility, and cache/CDN policy.
- Serializer dependency selection, custom type registration, and payload
  compatibility review.

## No Node Modules Path

The DX/Forge dashboard workflow keeps a no `node_modules` path for generated
starter review. Runtime execution remains app-owned: once the app chooses to run
real tRPC procedures, it must install and govern the declared runtime packages
explicitly.

## Source Guard

`dx run --test .\benchmarks\trpc-dashboard-workflow.test.ts` now reads the
local upstream mirror at `G:\WWW\inspirations\trpc` and fails if the claimed
`initTRPC`, fetch adapter, client links, or TanStack options-proxy APIs are no
longer present in source. The dashboard receipt also records the exact upstream
source files and asserted APIs under `upstream_source_guard` so the package
cannot drift into metadata-only listing. It also verifies that the dashboard
starter imports the tRPC workflow, the Forge slice exports the dashboard
workflow helper and `.tsx` component, the package doc names the source mirror
and app-owned boundaries, the component uses DX Icons metadata, and official
starter UI avoids hardcoded color classes.

`dx run --test .\benchmarks\trpc-dx-check-visibility-receipt.test.ts` verifies
the Type-Safe API dx-check visibility receipt, selected-surface states,
hash-backed freshness manifest, launch catalog metadata, CLI discovery JSON,
and docs contract. `dx run --test
.\benchmarks\trpc-dx-check-package-lane-panel.test.ts` verifies the
check-panel package row, the static `/launch` package-lane template, and the
generated-starter materialization guard for Type-Safe API without claiming live
Type-Safe API runtime proof. `dx run --test
.\benchmarks\trpc-receipt-hash-refresh.test.ts` verifies receipt-helper
attribution for current, stale, and write-refresh states, including the
runbook/materializer-only stale fixture. `dx run --test
.\benchmarks\trpc-rust-dx-check-metrics.test.ts` guards the Rust package
metrics, the clean-to-stale hash fixture, and the targetable check-panel helper
freshness fixture. The single-fixture Cargo handoff is
`cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_type_safe_api_package_lane_hash_refresh_row --lib`;
it writes a temporary Type-Safe API package-status row, verifies
`type_safe_api_receipt_hash_refresh_current`, flips only
`receipt_hash_refresh.stale_file_count`, `current_files`, `stale_files`, and
`stale_mirror_files`, and then verifies
`type_safe_api_receipt_hash_refresh_stale` plus the exact stale path arrays
while keeping `type_safe_api_hash_mismatch` at zero. `dx run --test
.\benchmarks\trpc-forge-slice.test.ts` verifies that the package
metadata, launch shell, source-owned receipt, and runtime-safe `/launch` page all
name the real tRPC dashboard workflow. `dx run --test
.\benchmarks\trpc-launch-runtime-proof.test.ts` verifies generated receipt
copying, no-`node_modules` materialization, Web Preview marker discovery, and
the safe local health/launchEvent interactions.

## Intentionally Deferred

- Live route execution before the app installs and approves tRPC runtime
  dependencies.
- Production domain routers, tenant authorization, and rate limits.
- Durable event storage for the launch mutation and infinite feed.
- Production subscription fan-out, reconnect policy, and stream backpressure.
- Hosted observability, incident response policy, and deployed cache-header
  verification.

## Four-Pass Verdict

REAL for coding readiness. The slice is based on upstream tRPC server, client,
and TanStack Query public APIs; it materializes source-owned Forge files, powers
the starter dashboard and generated `/launch` mission workflow, records receipt
and Studio/Web Preview metadata, and leaves runtime dependency installation,
production routers, auth, persistence, and observability app-owned.
