# Documentation System

Official DX package name: `Documentation System`

Package id: `content/fumadocs-next`

Upstream package: `fumadocs`

`content/fumadocs-next` is the Forge-owned Documentation System package slice in DX-WWW. It is based on the local upstream mirror at `G:\WWW\inspirations\fumadocs`, especially `packages/core`, `packages/mdx`, `packages/openapi`, and the Next examples.

The slice materializes source files that import Fumadocs public APIs. It does not vendor Fumadocs internals, run `create-fumadocs-app`, install packages, or create a template-local `node_modules` workflow. Framework config stays in `framework.www.*` and `framework.fumadocs.*` inside the extensionless `dx` file instead of a generated `next.config.*` or `source.config.ts` file.

## Real Upstream Surface

- `dx` as the app-owned MDX adapter/config boundary.
- `defineDocs` and `defineConfig` from `fumadocs-mdx/config`.
- `loader` from `fumadocs-core/source`.
- `iconPlugin`, `statusBadgesPlugin`, and `slugsFromData` from Fumadocs source plugins, with page-tree icons resolved through the DX Icon system.
- `getBreadcrumbItems`, `flattenTree`, `findNeighbour`, and `getPageTreePeers` for page-tree navigation.
- `getTableOfContents`, `TOCItemType`, and generated `page.data.toc` for outlines.
- `llms(source)` and processed Markdown exports for AI-readable docs.
- `DocsLayout`, `RootProvider`, `DocsPage`, `DocsBody`, `DocsTitle`, `DocsDescription`, `createRelativeLink`, and default MDX components from `fumadocs-ui`.
- `createOpenAPI`, `staticSource`, `loaderPlugin`, `createProxy`, `proxyUrl`, `createAPIPage`, `createCodeUsageGeneratorRegistry`, `registerDefault`, `defineClientConfig`, and request code usage generators from `fumadocs-openapi`.
- `createFromSource`, `staticGET`, and `useDocsSearch` for dynamic and static search paths.

## Exported Files

- `lib/fumadocs/source.ts`
- `lib/fumadocs/source-plugins.tsx`
- `lib/fumadocs/layout.tsx`
- `lib/fumadocs/navigation.ts`
- `lib/fumadocs/toc.ts`
- `lib/fumadocs/llms.ts`
- `lib/fumadocs/openapi.ts`
- `lib/fumadocs/openapi-code-usage.ts`
- `lib/fumadocs/search.ts`
- `lib/fumadocs/search-client.ts`
- `lib/fumadocs/readiness.ts`
- `lib/fumadocs/dashboard-workflow.ts`
- `lib/fumadocs/metadata.ts`
- `lib/fumadocs/route-contract.ts`
- `components/dashboard/fumadocs-docs-workflow.tsx`
- `components/launch/docs-status.tsx`
- `components/mdx.tsx`
- `components/api-page.tsx`
- `components/api-page.client.tsx`
- `app/docs/layout.tsx`
- `app/docs/[[...slug]]/page.tsx`
- `app/docs/readiness/route.ts`
- `app/llms.txt/route.ts`
- `app/llms-full.txt/route.ts`
- `app/llms.mdx/docs/[[...slug]]/route.ts`
- `app/api/search/route.ts`
- `app/api/search-static/route.ts`
- `app/api/openapi/proxy/route.ts`
- `content/docs/meta.json`
- `content/docs/index.mdx`
- `openapi/dx-www.yaml`
- `lib/fumadocs/README.md`

## Dashboard Workflow

The starter dashboard consumes the package through the Documentation System workflow component in `examples/dashboard/src/components/FumadocsDocsWorkflow.tsx`.

The generated `/launch` dashboard consumes the same package boundary through `LaunchDocsStatus` in `examples/template/docs-status.tsx`. It is mounted as a help/docs/changelog workflow, not a package proof card.

The visible workflows expose:

- `data-dx-package="content/fumadocs-next"`
- `data-dx-component="dashboard-fumadocs-docs-workflow"`
- `data-dx-component="launch-fumadocs-docs-workflow"`
- `data-dx-fumadocs-dashboard-workflow="docs-ops"`
- `data-dx-dashboard-workflow="docs-help-changelog"`
- `data-dx-dashboard-card="docs-help"`
- `data-dx-product-surface="dashboard-help-content"`
- `data-dx-fumadocs-dashboard-target="mission-control-docs"`
- `data-dx-fumadocs-interaction="page-tree-selector"`
- `data-dx-fumadocs-action="safe-local-route-preview"`
- `data-dx-fumadocs-page-option`
- `data-dx-fumadocs-rendered-markdown`
- `data-dx-fumadocs-changelog`
- `data-dx-fumadocs-rendered-route`
- `data-dx-fumadocs-selected-page`
- `data-dx-fumadocs-toc-count`
- `data-dx-fumadocs-local-response`
- `data-dx-fumadocs-receipt-route`
- `data-dx-fumadocs-missing-config`
- `data-dx-docs-readiness`
- `data-dx-docs-openapi-code-usage`
- `data-dx-docs-openapi-proxy`
- `public/preview-manifest.json` selector `[data-dx-component="launch-fumadocs-docs-workflow"]`
- `<dx-icon name="pack:fumadocs" />`

The local interaction uses `createFumadocsNavigationReceipt` to preview the selected docs route, breadcrumb, peer count, TOC count, and next required app action. The template also exposes `GET /docs/readiness` from `app/docs/readiness/route.ts`, backed by `lib/fumadocs/readiness.ts`, so auditors can inspect local route/file completeness, DX Icon source plugin wiring, and explicit live-renderer/search/OpenAPI boundaries. In the generated `/launch` runtime, route selection also updates the mission-control docs card so the dashboard summarizes the active Documentation System route and receipt state. It does not claim live docs rendering, search indexing, OpenAPI proxying, or hosted content governance before the application owns those runtime decisions.

The page-tree controls expose `aria-pressed` alongside `data-dx-fumadocs-page-selected`, and the local receipt output uses `role="status"` plus `aria-live="polite"` so the visible dashboard workflow is keyboard/screen-reader reachable during the final governed browser pass.

## DX-Style Compatibility

The visible Documentation System surfaces declare `data-dx-style-surface="documentation-system"` and rely on DX token classes such as `bg-background`, `text-muted-foreground`, `border-border`, `bg-card`, and `text-card-foreground` rather than hardcoded colors or inline style objects.

The package catalog, generated Forge metadata, and dashboard workflow receipt expose `dx.forge.package.dx_style_compatibility` with `styles/theme.css` as the token source and `styles/generated.css` as the generated CSS target. Runtime visual proof remains governed separately with browser QA and live Fumadocs renderer checks.

## Forge Metadata

- Package id: `content/fumadocs-next`
- Official DX package name: `Documentation System`
- Aliases: `fumadocs`, `fumadocs-next`, `docs`
- Upstream package: `fumadocs`
- Source mirror: `G:\WWW\inspirations\fumadocs`
- Required env: `DX_FUMADOCS_OPENAPI_ALLOWED_ORIGINS`
- Receipt paths: `.dx/forge/docs/content-fumadocs-next.md`, `.dx/forge/receipts/*-content-fumadocs-next.json`, `docs/packages/content-fumadocs-next.md`, `docs/packages/content-fumadocs-next.source-guard-runbook.json`, `examples/template/.dx/forge/receipts/2026-05-22-content-fumadocs-dashboard-workflow.json`, materialized as `.dx/forge/receipts/2026-05-22-content-fumadocs-dashboard-workflow.json` by `dx new`
- DX icon: `<dx-icon name="pack:fumadocs" />`
- Dashboard usage: `/launch` uses `launch-fumadocs-docs-workflow` from `examples/template/docs-status.tsx` for route selection, mission-control docs summary updates, OpenAPI code-usage metadata, OpenAPI missing-config readiness, the `/docs/readiness` source-owned readiness route, changelog notes, and a safe local route receipt. The package catalog, runtime materializer, conversion-proof preview manifest, and Studio manifest expose the same package-specific selectors for Zed Web Preview edits.
- Template surface registry: `examples/template/template-surface-registry.ts` owns the `content-docs` surface with selector `[data-dx-component="launch-fumadocs-docs-workflow"]`, the Fumadocs receipt path, and the package guard.
- DX Studio edit contract: `examples/template/dx-studio-edit-contract.ts` maps `docs-help-changelog-workflow` to the package-specific page selector, route preview action, rendered markdown, changelog, missing-config, local receipt, TOC, route, OpenAPI proxy, and dashboard workflow receipt markers, and scopes the dx-check health panel to `content/fumadocs-next` so Studio can package-filter the Documentation System package-lane row.
- Generated preview manifest: `tools/launch/materialize-www-template.ts` writes the same route preview, page selector, rendered markdown, changelog, missing-config, local receipt, OpenAPI code-usage, and OpenAPI proxy markers into generated `public/preview-manifest.json` for `launch-runtime-docs`, keeps `launch-runtime-dx-check-panel` scoped to `content/fumadocs-next` with helper freshness marker names, and exposes `docs/packages/content-fumadocs-next.source-guard-runbook.json` through root `sourceGuardRunbookFixtures` plus the `/launch` route `routes[].sourceGuardRunbookFixtures` entry.
- Zed manifest handoff: `dx-www/src/cli/studio_manifest.rs` and `examples/template/template-route-contract.ts` expose the same package-specific docs workflow selectors, dx-check package-lane marker scope, and Documentation System Studio source-guard/runbook entry so CLI/Zed discovery points to the real route-preview workflow and helper-backed package row instead of a generic package listing.
- Upstream API receipt handoff: the dashboard workflow receipt records the same MDX config, source plugin, page-tree, TOC, UI layout/page, LLM processed Markdown, OpenAPI proxy/page/code-usage, and search APIs that the Forge slice imports.
- Reality audit: the dashboard workflow receipt classifies the slice as `REAL` for source-owned Forge package code, generated `/launch` materialization, and Studio discovery, with live Fumadocs renderer/search/OpenAPI runtime proof still governed separately.
- Launch dashboard workflow: `docs-help-changelog`
- Launch product surface: `dashboard-help-content`

## Receipt Integrity

The dashboard workflow receipt records selected surfaces for `docs-app-router`, `dashboard-help-workflow`, `llm-export`, `openapi-reference`, `docs-runtime-readiness`, and `search-index`.

It also records SHA-256 source hashes for the Forge slice, package catalog, launch dashboard workflow, starter dashboard workflow API/component, this package document, the package-owned source-guard runbook fixture, and the preview-manifest materializer `tools/launch/materialize-www-template.ts`. `dx-check` and Zed/DX Studio should treat the receipt as present but stale when any tracked source-surface hash changes without refreshing the stale receipt.

Documentation System hash freshness now uses the shared Rust comparator in `core/src/ecosystem/project_check/file_hashes.rs` for both selected-surface `file_hashes` and receipt-level `source_hashes`, so the stale check is byte-derived and shares the same generated-root path fallback as neighboring Forge package lanes.

The shared package-status read model exposes the same Documentation System row through `examples/template/forge-package-status-read-model.ts`, including selected surfaces, source hashes, dx-style compatibility, and Zed receipt surface metadata for `documentation-system:docs-help-changelog`.

The package-owned hash refresh helper is:

```powershell
node tools/launch/run-template-receipt-helper.js examples/template/documentation-system-receipt-hashes.ts --check
node tools/launch/run-template-receipt-helper.js examples/template/documentation-system-receipt-hashes.ts --write
node tools/launch/run-template-receipt-helper.js examples/template/documentation-system-receipt-hashes.ts --check --json
```

It checks and refreshes only local Documentation System SHA-256 receipt, package-status, and read-model hashes. The source-guard runbook fixture and preview-manifest materializer are hash-tracked as `docs/packages/content-fumadocs-next.source-guard-runbook.json` and `tools/launch/materialize-www-template.ts`, mirrored as `source_guard_runbook_fixture` and `preview_manifest_materializer` in package-status, and mirrored as `sourceGuardRunbookFixture` and `previewManifestMaterializer` in the typed read model. Helper JSON, package-status, and the typed read model now expose `current_files`, `stale_files`, `missing_files`, `stale_mirror_files`, `missing_mirror_files`, and `mirror_problem_count`, so materializer-only generated preview-manifest drift is attributed to `tools/launch/materialize-www-template.ts` without marking selected docs or dashboard source files stale. The materializer hash mirror is exposed as selected surface `documentation-system-preview-manifest-materializer` so the generated `public/preview-manifest.json` source path can be inspected without claiming runtime proof. It does not run live Fumadocs rendering, start a server, install packages, read secrets, execute the OpenAPI proxy, or build hosted search indexes.

The helper scopes read-model mirror checks to `documentationSystemPackageVisibility`, so shared files such as `examples/template/package-catalog.ts` can also appear in neighboring package rows without making Documentation System falsely stale.

## Rust dx-check output

The core Forge checker now maps this package-status row through `core/src/ecosystem/project_check/documentation_system_dx_check.rs` and `core/src/ecosystem/project_check.rs`. It emits `documentation_system_package_present`, `documentation_system_receipt_present`, `documentation_system_receipt_stale`, `documentation_system_missing_receipt`, `documentation_system_blocked_surface`, `documentation_system_unsupported_surface`, `documentation_system_hash_manifest_present`, `documentation_system_hash_mismatch`, `documentation_system_receipt_hash_refresh_current`, `documentation_system_receipt_hash_refresh_stale`, `documentation_system_receipt_hash_refresh_missing`, `documentation_system_dx_style_compatibility_present`, and `documentation_system_dx_style_compatibility_missing`.

The checker raises `documentation-system-missing-package-status`, `documentation-system-missing-receipt`, `documentation-system-stale-receipt`, `documentation-system-blocked-surface`, `documentation-system-unsupported-surface`, `documentation-system-hash-mismatch`, and `documentation-system-missing-dx-style-compatibility` findings from package-status, receipt, selected-surface, SHA-256 source-hash, and dx-style compatibility evidence without claiming live Fumadocs renderer runtime proof.

The SHA-256 comparison path is shared with `project_check/file_hashes.rs`: selected surfaces use `count_sha256_file_hash_mismatches`, and Documentation System receipt source hashes use `count_sha256_path_hash_mismatches`.

The package-owned Rust fixture `documentation_system_hash_mismatch_metric_and_finding_are_byte_derived` creates a temporary Documentation System package-status row and dashboard workflow receipt, records a fresh `content/docs/index.mdx` hash, flips only `receipt_hash_refresh.stale_file_count` to prove `documentation_system_receipt_hash_refresh_stale` marks the lower-level receipt stale while `documentation_system_hash_mismatch` stays `0`, then mutates the docs file and verifies `documentation_system_receipt_stale`, `documentation_system_hash_mismatch`, and `documentation-system-hash-mismatch` flip together.

## DX Studio/check-panel Row

The DX Studio/check-panel Documentation System package row is produced by `core/src/ecosystem/dx_check_receipt.rs` from `.dx/forge/package-status.json`. It keeps the official package label `Documentation System` in user-facing UI while preserving `fumadocs`, `16.8.12`, and `G:/WWW/inspirations/fumadocs` as provenance fields.

The row mirrors selected surfaces, receipt status, runtime limitations, `documentation_system_hash_manifest_present`, `documentation_system_hash_mismatch`, `documentation_system_receipt_hash_refresh_current`, `documentation_system_receipt_hash_refresh_stale`, `documentation_system_receipt_hash_refresh_missing`, `documentation_system_dx_style_compatibility_present`, and `documentation_system_dx_style_compatibility_missing` into `check_panel.view_model.package_lane_rows`. The focused fixture `dx_check_latest_panel_exposes_documentation_system_package_lane_hash_row` proves the row starts fresh from a temporary hash-backed docs file and then flips stale when that docs file changes, without claiming live Fumadocs renderer runtime proof.

The static `/launch` package-lane template now mirrors the same Documentation System row before a fresh dx-check receipt is loaded. `examples/template/template-shell.tsx`, `the static launch runtime template`, the runtime materializer, the DX Studio edit contract, and the Rust Studio manifest carry `data-dx-check-package-lane-template="content/fumadocs-next"`, `documentation-system:receipt-hash-refresh`, `documentation_system_receipt_hash_refresh_current`, `documentation_system_receipt_hash_refresh_stale`, `documentation_system_receipt_hash_refresh_missing`, `data-dx-style-surface="documentation-system"`, `data-dx-token-scope="content/fumadocs-next"`, and `data-dx-package="content/fumadocs-next"`. The generated-starter materialization guard for Documentation System proves these markers survive into generated static launch HTML and `public/preview-manifest.json` without claiming live Fumadocs renderer runtime proof.

## Studio Source Guard Runbook

The Documentation System Studio source-guard/runbook entry is `documentation-system-generated-starter-materialization`. It is published through `dx-www/src/cli/studio_manifest.rs` in `source_guard_index`, `/launch` source guard ids, `/launch` source-only contracts, and `/launch` runbook commands.

The package-owned Documentation System runbook fixture is `docs/packages/content-fumadocs-next.source-guard-runbook.json`. It mirrors the same source guard, helper command, generated preview-manifest fixture contract, upstream provenance, selected surfaces, dx-check metric names, Zed/DX Studio marker names, app-owned boundaries, and `SOURCE-ONLY` runtime limitations without claiming live Fumadocs renderer runtime proof.

```powershell
dx run --test .\benchmarks\fumadocs-dx-check-package-lane-panel.test.ts
node tools/launch/run-template-receipt-helper.js examples/template/documentation-system-receipt-hashes.ts --check --json
```

The runbook exposes `data-dx-check-package-lane-row="content/fumadocs-next"`, `data-dx-token-scope="content/fumadocs-next"`, root `sourceGuardRunbookFixtures`, `/launch` `routes[].sourceGuardRunbookFixtures`, `documentation-system:receipt-hash-refresh`, `examples/template/documentation-system-receipt-hashes.ts`, and `content/fumadocs-next source-only Studio discovery` without claiming live Fumadocs renderer runtime proof.

## App-owned Boundaries

- Dependency installation and version policy for `fumadocs-core`, `fumadocs-ui`, `fumadocs-mdx`, `fumadocs-openapi`, `next`, `react`, and `react-dom`.
- Existing Next config merge review.
- Content governance, private-doc exclusion, docs publication policy, and AI crawler exposure.
- Source plugin taxonomy, icon naming, status lifecycle, slug/canonical URL policy, and redirects.
- OpenAPI schema governance, proxy allowed origins, auth/cookie forwarding, request playground policy, and request code sample policy.
- Search UI, static-index payload budget, multilingual search, vector search, analytics, hosting, and runtime verification.

## No Node Modules Path

The DX/Forge starter path remains source-owned and no-install. It writes source files, package metadata, route contracts, dashboard workflow proof, and the Fumadocs dashboard workflow receipt, but the consuming application decides when to install runtime dependencies and run live WWW/Fumadocs verification.

## Source Guard

```powershell
dx run --test .\benchmarks\fumadocs-dashboard-workflow.test.ts
dx run --test .\benchmarks\fumadocs-dx-check-package-lane-panel.test.ts
dx run --test .\benchmarks\fumadocs-receipt-hash-refresh.test.ts
node tools/launch/run-template-receipt-helper.js examples/template/documentation-system-receipt-hashes.ts --check --json
```

The guard fails if upstream API evidence, receipt-level upstream API handoff, front-facing package API discovery, dashboard usage, DX icon markers, the `/docs/readiness` source-owned route contract, Forge metadata, package catalog metadata, generated-route receipt materialization, template surface registry discovery, `docs-workflow` runtime identity, the Fumadocs runtime materializer preview-manifest selector, the static `/launch` Documentation System package-lane template, the generated-starter materialization guard for Documentation System, the Documentation System Studio source-guard/runbook entry, the dashboard workflow receipt, the receipt-level `REAL` reality audit, hash-refresh helper visibility, accessibility semantics, or this package document disappears.

## Intentionally Deferred

- Live Fumadocs rendering before app-owned dependency installation.
- Automatic merge of external Next config files; DX-WWW keeps generated framework config in `dx`.
- Production OpenAPI proxy execution before `DX_FUMADOCS_OPENAPI_ALLOWED_ORIGINS` is configured.
- Private-content redaction and AI indexing policy.
- Hosted search indexing, multilingual/vector search, and analytics.
- Runtime browser/server evidence, which stays Friday/user-governed under the no-heavy-build/no-server rule.
