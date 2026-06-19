# Validation & Schemas

Official DX package: Validation & Schemas
Official CLI: `dx add validation-schemas --write`
packageId: validation/zod
upstream_package: zod
upstream_version: 4.4.3
Honesty label: SOURCE-ONLY

`validation/zod` is the package id for the source-owned DX Forge Validation & Schemas slice in the launch template and starter dashboard. It imports real `zod` APIs and keeps app policy outside the package.

## Public API Slice

- `dxDashboardSettingsSchema`
- `safeParseDxDashboardSettingsForm`
- `formatDxDashboardSettingsIssues`
- `createDxDashboardSettingsReceipt`
- `templateLoginSchema`
- `templateWorkspaceSettingsSchema`
- `templateProfileSchema`
- `templateBillingContactSchema`
- `safeParseTemplateForm`
- `validateDxInput`
- `z.flattenError`, `z.treeifyError`, `z.prettifyError`

## Dashboard Usage

- `/launch` mission control summary: `data-dx-component="launch-settings-validation-summary"`
- Package marker: `data-dx-package="validation/zod"`
- Form package marker: `data-dx-form-package="forms/react-hook-form"`
- `/launch` dashboard card: `data-dx-dashboard-card="settings"`
- Runtime workflow marker: `data-dx-dashboard-workflow="settings-validation"`
- DX Style surface marker: `data-dx-style-surface="validation-schemas"`
- Token scope marker: `data-dx-token-scope="validation/zod"`
- Product surface marker: `data-dx-product-surface="account-settings"`
- Mission-control controls marker: `data-dx-zod-dashboard-controls="mission-control"`
- Mission-control editable fieldset: `data-dx-zod-dashboard-fieldset="editable-settings"`
- Mission-control editable fields: `workspaceName`, `contactEmail`, `defaultLocale`, `theme`, `previewMode`, `launchScoreTarget`, and `packageReceiptsRequired`
- Mission-control actions: `data-dx-zod-dashboard-action="load-invalid-settings"` and `data-dx-zod-dashboard-action="load-valid-settings"`
- Mission-control receipt marker: `data-dx-zod-dashboard-receipt="idle"`
- Mission-control receipt JSON: `id="mission-settings-receipt-json"` with `data-dx-zod-dashboard-receipt-api="createDxDashboardSettingsReceipt"`
- Runtime form marker: `data-dx-zod-form="dashboard-settings"`
- Schema marker: `data-dx-zod-schema="dxDashboardSettingsSchema"`
- Public API marker: `data-dx-zod-public-api="safeParseDxDashboardSettingsForm"`
- Runtime field-error API marker: `data-dx-zod-field-errors-api="z.flattenError"`
- Starter dashboard field state marker: `data-dx-zod-dashboard-field-state`
- Starter dashboard field error marker: `data-dx-zod-dashboard-field-error`
- Settings summary marker: `data-dx-zod-settings-summary="idle"`
- RHF boundary marker: `data-dx-rhf-boundary="runtime-safe-form"`
- Source API marker: `data-dx-source-owned-api="lib/validation/zod/dashboard-settings.ts"`
- Starter dashboard component: `examples/dashboard/src/components/ZodSettingsValidator.tsx`
- Starter metadata: `examples/dashboard/src/lib/zodDashboardSettings.ts` records the exact dashboard workflow receipt, `/launch` usage, and field-state/error markers.
- Template surface registry: `examples/template/template-surface-registry.ts` maps the `settings-validation` slot to `data-dx-component="zod-dashboard-settings-form"` and the concrete dashboard receipt.
- Studio/Web Preview handoff: `examples/template/dx-studio-edit-contract.ts` and `tools/launch/materialize-www-template.ts` expose the Zod settings selectors, receipt path, and generated `launch-runtime-settings-validation` surface.

The runtime `/launch` form validates actual account settings fields: `workspaceName`, `contactEmail`, `defaultLocale`, `theme`, `previewMode`, `launchScoreTarget`, and `packageReceiptsRequired`. Mission control can edit the full dashboard settings payload, submit invalid and valid states into that same form, then the form updates the dashboard when the settings/profile payload is valid or blocked. The browser exposes structured Zod-style issues, a `z.flattenError` field-error map, a validated settings summary, parsed settings JSON, and a `createDxDashboardSettingsReceipt`-shaped JSON receipt without claiming persistence. The starter dashboard validator maps the same issue and field-error data into `aria-invalid`, field-state markers, and per-field error markers for Zed/Web Preview selection.

## Default Template Forms

- Source-owned schemas: `examples/template/lib/validation/zod/template-forms.ts`
- Template usage: `examples/template/components/template-app/forms.tsx`
- Receipt surface: `template-forms-validation`
- Schema marker: `dx.validation.template_forms`
- Covered forms: login, workspace settings, profile, and billing contact
- Field-error API: `z.flattenError`

The default Next-style launch template imports its login, settings, profile, and billing contact schemas from `@/lib/validation/zod/template-forms` instead of embedding Zod definitions inside the Forms component. This keeps the Forms package focused on React Hook Form ownership while Validation & Schemas owns the typed Zod schema surface and `safeParseTemplateForm` source API. The surface is source-only: it validates local form payloads and exposes typed errors, but submit destinations, session creation, persistence, payment sessions, and browser runtime proof remain app-owned.

## dx-check Visibility

Validation & Schemas exposes `dx.forge.package.dx_check_visibility` metadata with `present`, `stale`, `missing receipt`, `blocked`, and `unsupported surface` states. The monitored surfaces are `dashboard-settings-validation`, `template-forms-validation`, `launch-package-catalog`, `starter-dashboard-settings-validator`, `generated-starter-materialization`, and `validation-schemas-source-guard-runbook`. The current dashboard settings and template form validation surfaces are `present` because the source-owned files, markers, and receipt are materialized; `stale` means the receipt no longer matches the generated/source files, `missing receipt` means dx-check cannot find the governed receipt, `blocked` means runtime proof or app-owned authorization/persistence is unresolved, and `unsupported surface` means a requested Zod API was not selected for this curated slice.

The shared package-status read model exposes the same lane as `validationSchemasPackageVisibility` in `examples/template/forge-package-status-read-model.ts` and `.dx/forge/package-status.json`. dx-check/Zed consumers can read `validation_schemas_receipt_present`, `validation_schemas_receipt_stale`, `validation_schemas_missing_receipt`, `validation_schemas_blocked_surface`, `validation_schemas_unsupported_surface`, `validation_schemas_dx_style_compatibility_present`, and `validation_schemas_dx_style_compatibility_missing` without running a browser or claiming runtime proof.

The dashboard settings receipt also records `hash_algorithm: sha256`, tracked `files`, and `file_hashes` for selected front-facing Validation & Schemas source files, the generated-starter materializer `tools/launch/materialize-www-template.ts`, and the package-owned source-guard fixture `docs/packages/validation-schemas.source-guard-runbook.json`. The read model mirrors those hashes as `sourceHashes` and per-surface `fileHashes` so Zed/DX Studio can show whether a selected schema, generated metadata surface, or runbook fixture is receipt-backed without scraping source files directly.

Rust `dx check` consumes `validation_schemas_package_present`, `validation_schemas_receipt_present`, `validation_schemas_receipt_stale`, `validation_schemas_missing_receipt`, `validation_schemas_blocked_surface`, `validation_schemas_unsupported_surface`, `validation_schemas_hash_manifest_present`, `validation_schemas_hash_mismatch`, `validation_schemas_dx_style_compatibility_present`, and `validation_schemas_dx_style_compatibility_missing` from `.dx/forge/package-status.json` plus the dashboard settings receipt. It reports `validation-schemas-missing-package-status`, `validation-schemas-missing-receipt`, `validation-schemas-stale-receipt`, `validation-schemas-blocked-surface`, `validation-schemas-unsupported-surface`, `validation-schemas-hash-mismatch`, and `validation-schemas-missing-dx-style-compatibility` findings so stale, missing, blocked, unsupported, hash-mismatched, and style-metadata-missing Validation & Schemas states are visible without inspecting raw Forge receipts.

Hash freshness now routes through the shared `core/src/ecosystem/project_check/file_hashes.rs` SHA-256 comparator, so generated-root path fallback and `sha256:` normalization match neighboring package lanes. The package-owned Rust fixture `validation_schemas_hash_mismatch_flips_when_selected_file_changes` writes a temporary Validation & Schemas package-status row and receipt, verifies a fresh selected file reports `validation_schemas_hash_mismatch = 0`, mutates that file, and verifies `validation_schemas_receipt_stale`, `validation_schemas_hash_mismatch`, and `validation-schemas-hash-mismatch` flip together.

## DX-Style Compatibility

Validation & Schemas now publishes `dx.forge.package.dx_style_compatibility` in the package catalog, dashboard settings receipt, `.dx/forge/package-status.json`, and typed package-status read model. The visible launch and starter dashboard surfaces carry `data-dx-style-surface="validation-schemas"` and `data-dx-token-scope="validation/zod"` while continuing to use DX theme tokens from `styles/theme.css` and `styles/generated.css`; no inline color literals or browser visual proof are claimed.

The Rust checker records `validation_schemas_dx_style_compatibility_present` and `validation_schemas_dx_style_compatibility_missing`, and the package-owned fixture `validation_schemas_dx_style_missing_metric_and_finding_flip` verifies that removing the metadata raises `validation-schemas-missing-dx-style-compatibility`. This is SOURCE-ONLY proof: `dx run --test .\benchmarks\zod-dx-style-compatibility.test.ts` guards the source markers and metadata, while full browser style review remains app-owned.

Receipt hash maintenance is package-owned through `node tools/launch/run-template-receipt-helper.js examples/template/validation-schemas-receipt-hashes.ts --check`. Use `--write` after reviewing source changes to refresh the dashboard settings receipt, `.dx/forge/package-status.json`, and `forge-package-status-read-model.ts` hashes together. The helper is SOURCE-ONLY: it does not run browser runtime proof, install dependencies, persist settings, read secrets, or claim governed launch validation.

Zed/DX Studio can read the same helper state from package-status `receipt_hash_refresh` and typed read-model `receiptHashRefresh`, including helper path, check/write commands, `source_guard_runbook_fixture`, `preview_manifest_materializer`, tracked file paths, exact `current_files`, `stale_files`, `missing_files`, `stale_mirror_files`, `missing_mirror_files`, tracked/stale/missing counts, no-runtime/no-secret flags, and `validation-schemas:receipt-hash-refresh`. The helper currently tracks 14 selected files, including `lib/validation/zod/template-forms.ts`, `components/template-app/forms.tsx`, the generated-starter materializer, and the package-owned source-guard fixture, so package-scoped schema, manifest, and runbook metadata changes become stale-detectable with path attribution.

The DX Studio/check-panel Validation & Schemas package row is also sourced from `.dx/forge/package-status.json` in `core/src/ecosystem/dx_check_receipt.rs`. It renders selected dashboard settings surfaces, the full `receipt_hash_refresh` helper payload, `tracked_files`, `source_guard_runbook_fixture`, `preview_manifest_materializer`, exact `current_files`, `stale_files`, `missing_files`, mirror attribution arrays, and metrics including `validation_schemas_hash_manifest_present`, `validation_schemas_hash_mismatch`, `validation_schemas_receipt_hash_refresh_current`, `validation_schemas_receipt_hash_refresh_stale`, and `validation_schemas_receipt_hash_refresh_missing` without claiming live Validation & Schemas runtime proof.

Static launch preview: `the static launch runtime template` carries static /launch package-lane markers for Validation & Schemas with `data-dx-check-package-lane-row="validation/zod"`, the governed dashboard settings receipt path, upstream `zod` provenance, `data-dx-check-package-lane-hash-refresh-helper="examples/template/validation-schemas-receipt-hashes.ts"`, and `data-dx-check-package-lane-hash-refresh-zed="validation-schemas:receipt-hash-refresh"` so Studio can discover helper freshness before a fresh dx-check receipt is loaded.

Generated starter proof: the generated-starter materialization guard for Validation & Schemas runs `tools/launch/materialize-www-template.ts` into a temporary project, verifies the materialized static launch HTML keeps the `validation/zod` package-lane row, helper freshness markers, `data-dx-style-surface="validation-schemas"`, and `data-dx-token-scope="validation/zod"`, then verifies `public/preview-manifest.json` scopes `launch-runtime-dx-check-panel` and `launch-runtime-settings-validation` to `validation/zod` with the dx-style/token state markers. Generated preview-manifest snapshots now also expose `docs/packages/validation-schemas.source-guard-runbook.json` through root `sourceGuardRunbookFixtures` and `/launch` `routes[].sourceGuardRunbookFixtures`, preserving `validation-schemas-generated-starter-materialization`, `SOURCE-ONLY`, `runtimeProof: false`, and `validation-schemas:receipt-hash-refresh`. The materializer is a hash-backed selected surface named `generated-starter-materialization`, so changes to generated preview metadata flip Validation & Schemas receipt freshness; `benchmarks/zod-receipt-hash-refresh.test.ts` now proves a materializer-only stale hash reports `tools/launch/materialize-www-template.ts` in `stale_files` while selected settings validation sources remain in `current_files`. This is still SOURCE-ONLY; it proves generated files and Studio metadata, not browser runtime execution.

Source Studio proof: the source Studio dx-check panel package scope for Validation & Schemas is guarded in `benchmarks/zod-dx-check-package-lane-panel.test.ts`. The guard reads `examples/template/dx-studio-edit-contract.ts` and `dx-www/src/cli/studio_manifest.rs`, then verifies the authored `dx-check-health-panel` surface includes `validation/zod` alongside the generated `launch-runtime-dx-check-panel` scope. This is still SOURCE-ONLY; it proves Studio/Zed package filtering metadata, not live browser or native Zed rendering.

Studio source-guard/runbook proof: `dx-www/src/cli/studio_manifest.rs` now publishes the Validation & Schemas generated-starter materialization guard as `validation-schemas-generated-starter-materialization` in `source_guard_index`, `/launch` source guard ids, source-only contracts, and runbook commands with `dx run --test .\benchmarks\zod-dx-check-package-lane-panel.test.ts`. Zed/DX Studio can list the exact SOURCE-ONLY Validation & Schemas package-lane, helper freshness, and dx-style marker proof without scanning raw tests or claiming live Validation & Schemas runtime proof.

`docs/packages/validation-schemas.source-guard-runbook.json` is the package-owned JSON fixture for that same runbook contract. It records the official package name, upstream `zod` provenance, inspected source files, selected surfaces, exact guard command, `/launch` `source_guard_runbook_index` contract, generated preview-manifest fixture paths, Zed/DX Studio marker names, receipt hash helper metadata, app-owned boundaries, and `SOURCE-ONLY` runtime limitations so tooling can read the Validation & Schemas runbook contract without parsing raw Rust source. The Studio manifest exposes the same path through structured `fixture_path` metadata, generated preview-manifest metadata links it for starter snapshots, and the receipt helper tracks it as the `validation-schemas-source-guard-runbook` selected surface; the fixture is not live runtime proof.

## Provenance

Source mirror: `G:/WWW/inspirations/zod`

Inspected upstream package metadata and the Zod v4 classic exports for schema parsing, metadata, strict objects, and error formatting before selecting this launch slice.

## App-Owned Boundaries

- Accepted schema design
- Account/settings persistence
- Authorization before account changes
- Final validation copy and locale policy
- External JSON Schema trust policy
- Runtime dependency installation
- Downstream database writes and audit trail

## Receipts

- `.dx/forge/docs/validation-zod.md`
- `.dx/forge/receipts/*-validation-zod.json`
- `examples/template/.dx/forge/receipts/2026-05-22-validation-zod-dashboard-settings.json`
- `examples/template/validation-schemas-receipt-hashes.ts`
- `docs/packages/validation-schemas.source-guard-runbook.json`
- `the static launch runtime template#mission-control`
- `examples/dashboard/README.md#zod-settings-validation`
