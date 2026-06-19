# UI Components

official DX package name: **UI Components**. Front-facing package ids such as `ui/button`, `ui/card`, `ui/alert`, `ui/avatar`, and `ui/skeleton` are selectable Forge surface ids. Compatibility ids such as `shadcn/ui/button` stay provenance metadata for upstream parity and receipt traceability.

- package_id: `ui/button`
- compatibility_id: `shadcn/ui/button`
- aliases: `ui/button`, `ui/alert`, `ui/avatar`, `ui/badge`, `ui/card`, `ui/field`, `ui/input`, `ui/item`, `ui/label`, `ui/separator`, `ui/skeleton`, `ui/textarea`
- upstream_package: `shadcn-ui`
- upstream_version: `0.0.1`
- based_on: `shadcn-ui v4 registry plus Radix Primitives`
- source_mirror: `G:/WWW/inspirations/shadcn-ui`
- source_mirror: `G:/WWW/inspirations/radix-primitives`
- required_env: none

selected surfaces: `button`, `alert`, `avatar`, `badge`, `card`, `label`, `separator`, `field`, `item`, `input`, `skeleton`, `textarea`

Inspected source files:

- `G:/WWW/inspirations/shadcn-ui/package.json`
- `G:/WWW/inspirations/shadcn-ui/apps/v4/registry/new-york-v4/ui/alert.tsx`
- `G:/WWW/inspirations/shadcn-ui/apps/v4/registry/new-york-v4/ui/avatar.tsx`
- `G:/WWW/inspirations/shadcn-ui/apps/v4/registry/new-york-v4/ui/button.tsx`
- `G:/WWW/inspirations/shadcn-ui/apps/v4/registry/new-york-v4/ui/badge.tsx`
- `G:/WWW/inspirations/shadcn-ui/apps/v4/registry/new-york-v4/ui/card.tsx`
- `G:/WWW/inspirations/shadcn-ui/apps/v4/registry/new-york-v4/ui/field.tsx`
- `G:/WWW/inspirations/shadcn-ui/apps/v4/registry/new-york-v4/ui/input.tsx`
- `G:/WWW/inspirations/shadcn-ui/apps/v4/registry/new-york-v4/ui/item.tsx`
- `G:/WWW/inspirations/shadcn-ui/apps/v4/registry/new-york-v4/ui/label.tsx`
- `G:/WWW/inspirations/shadcn-ui/apps/v4/registry/new-york-v4/ui/separator.tsx`
- `G:/WWW/inspirations/shadcn-ui/apps/v4/registry/new-york-v4/ui/skeleton.tsx`
- `G:/WWW/inspirations/shadcn-ui/apps/v4/registry/new-york-v4/ui/textarea.tsx`
- `G:/WWW/inspirations/radix-primitives/packages/react/slot/src/slot.tsx`
- `G:/WWW/inspirations/radix-primitives/packages/react/label/src/label.tsx`
- `G:/WWW/inspirations/radix-primitives/packages/react/separator/src/separator.tsx`

Upstream APIs kept visible:

- upstream registry: `Alert`, `AlertTitle`, `AlertDescription`, `Avatar`, `AvatarImage`, `AvatarFallback`, `Button`, `buttonVariants`, `Badge`, `badgeVariants`, `Card`, `CardHeader`, `CardTitle`, `CardDescription`, `CardContent`, `Field`, `FieldGroup`, `FieldLabel`, `FieldDescription`, `Input`, `Label`, `Textarea`, `Item`, `ItemGroup`, `ItemContent`, `ItemTitle`, `ItemDescription`, `ItemActions`, `Separator`, `Skeleton`
- Radix provenance: `Slot`, `createSlot`, `Label`, `Separator`, and `Root` exports from the inspected primitive packages

Front-facing files:

- `components/ui/button.tsx`
- `components/ui/alert.tsx`
- `components/ui/avatar.tsx`
- `components/ui/badge.tsx`
- `components/ui/card.tsx`
- `components/ui/field.tsx`
- `components/ui/input.tsx`
- `components/ui/item.tsx`
- `components/ui/label.tsx`
- `components/ui/separator.tsx`
- `components/ui/skeleton.tsx`
- `components/ui/textarea.tsx`
- `components/launch/shadcn-dashboard-controls-contract.tsx`
- `components/launch/shadcn-dashboard-controls.tsx`
- `examples/dashboard/src/lib/shadcnDashboardControls.ts`
- `examples/dashboard/src/components/ShadcnDashboardControls.tsx`

Receipt coverage:

- `examples/template/.dx/forge/receipts/2026-05-22-shadcn-dashboard-controls.json`
- `.dx/forge/receipts/*-shadcn-ui-button.json`
- `.dx/forge/receipts/*-shadcn-ui-card.json`
- `.dx/forge/docs/shadcn-ui-button.md`
- `.dx/forge/template-readiness/launch-route.json`

dx-check visibility: `present`, `stale`, `missing receipt`, `blocked`, `unsupported surface`.

## Package-Status Read Model

The shared launch package-status read model now carries a receipt-backed UI Components row for `shadcn/ui/button` with the selected surfaces `ui-components-source-primitives`, `ui-components-dashboard-controls`, and `ui-components-runtime-controls`. The row publishes `ui_components_receipt_present`, `ui_components_receipt_stale`, `ui_components_missing_receipt`, `ui_components_blocked_surface`, `ui_components_unsupported_surface`, `ui_components_hash_manifest_present`, and `ui_components_hash_mismatch` so dx-check, Zed, and DX Studio can render present, stale, missing-receipt, blocked, unsupported-surface, and hash-drift states without scanning all unselected shadcn-ui registry components.

Zed receipt surfaces exposed by the read model are `ui-components:source-primitives`, `ui-components:dashboard-controls`, `ui-components:runtime-controls`, and `ui-components:preview-manifest-materializer`.

Hash refresh visibility: package-status and `forge-package-status-read-model.ts` expose `receipt_hash_refresh` / `receiptHashRefresh` with helper commands, tracked file count, stale/missing counts, no-runtime/no-secret flags, and the Zed surface `ui-components:receipt-hash-refresh`.

## Rust dx-check output

`core/src/ecosystem/project_check/ui_components_dx_check.rs` consumes `.dx/forge/package-status.json` and the UI Components dashboard controls receipt to publish `ui_components_*` metrics in the Forge `dx check` section. It emits `ui_components_package_present`, `ui_components_receipt_present`, `ui_components_receipt_stale`, `ui_components_missing_receipt`, `ui_components_blocked_surface`, `ui_components_unsupported_surface`, `ui_components_hash_manifest_present`, `ui_components_hash_mismatch`, `ui_components_receipt_hash_refresh_current`, `ui_components_receipt_hash_refresh_stale`, and `ui_components_receipt_hash_refresh_missing`, plus `ui-components-stale-receipt`, `ui-components-missing-receipt`, `ui-components-blocked-surface`, `ui-components-unsupported-surface`, and `ui-components-hash-mismatch` findings when package-status, receipt, selected-file hash evidence, or receipt-hash helper freshness regresses.

This remains `SOURCE-ONLY`: dx-check verifies package-status and receipt visibility for source-owned shadcn-ui/Radix surfaces without claiming browser UI runtime proof.

## Hash-backed stale detection

The UI Components receipt and package-status row now carry SHA-256 hashes for the selected shadcn-ui/Radix-derived UI source files, the package-owned source-guard runbook fixture, and the preview-manifest materializer integration that publishes the fixture to generated starters: `button.tsx`, `slot.tsx`, the dashboard controls contract/component, `docs/packages/ui-components.source-guard-runbook.json`, and `tools/launch/materialize-www-template.ts`. This hash-backed stale detection contract lets dx-check compare the UI-owned hashes with local project files and raise `ui-components-hash-mismatch` before any worker can claim source freshness over stale UI surfaces, stale Studio runbook metadata, or stale preview-manifest fixture emission.

## Receipt hash refresh

Use `node tools/launch/run-template-receipt-helper.js examples/template/ui-components-receipt-hashes.ts --check` after editing selected UI Components source files, `docs/packages/ui-components.source-guard-runbook.json`, or the UI Components preview-manifest fixture emission in `tools/launch/materialize-www-template.ts`. Use `node tools/launch/run-template-receipt-helper.js examples/template/ui-components-receipt-hashes.ts --write` only after reviewing the source change; it refreshes the dashboard controls receipt hash array, dx-check visibility hash mirror, package-status row, and typed read-model hashes when those mirrors exist. It also reports `source_guard_runbook_fixture` so Zed/DX Studio can see when the package-owned runbook fixture is part of the freshness contract. `--check --json` now reports six tracked files plus `stale_files` and `missing_files`, which lets editor surfaces flag materializer-only drift without blaming selected shadcn-ui/Radix dashboard source files. The helper does not run browser UI runtime proof, install packages, read secrets, or claim accessibility/browser verification.

## DX Studio/check-panel UI Components package row

`core/src/ecosystem/dx_check_receipt.rs` now promotes the UI Components package-status row into `check_panel.view_model.package_lane_rows` with the nested `receipt_hash_refresh` helper payload intact. DX Studio and Zed can read `data-dx-check-package-lane-hash-refresh-status`, helper path, JSON check command, and `ui-components:receipt-hash-refresh` beside `ui_components_hash_manifest_present`, `ui_components_hash_mismatch`, `ui_components_receipt_hash_refresh_current`, `ui_components_receipt_hash_refresh_stale`, and `ui_components_receipt_hash_refresh_missing` without opening raw receipt JSON or claiming browser UI runtime proof. The focused stale-helper fixture flips `receipt_hash_refresh.stale_file_count` while selected source hashes remain current, so helper drift alone makes the package row stale.

## Static /launch UI Components package-lane fixture

`examples/template/template-shell.tsx` and `the static launch runtime template` now expose a receipt-less `shadcn/ui/button` package-lane template for official **UI Components**. The static row carries the UI Components helper path, JSON check command, `ui-components:receipt-hash-refresh`, the six-file tracked count from the receipt helper, stale/missing counts, and metric-name markers for `ui_components_receipt_hash_refresh_current`, `ui_components_receipt_hash_refresh_stale`, and `ui_components_receipt_hash_refresh_missing`.

This gives DX Studio and Zed a source marker for helper freshness before `.dx/receipts/check/check-latest.json` exists. It remains `SOURCE-ONLY`: the static fixture is discovery metadata for selected shadcn-ui/Radix surfaces and does not claim browser UI runtime proof.

## UI Components generated-starter materialization guard

`benchmarks/ui-components-dx-check-package-lane-panel.test.ts` now materializes a temporary starter with `tools/launch/materialize-www-template.ts` and verifies generated static launch HTML plus `public/preview-manifest.json`. The guard proves the generated `/` and `/launch` route metadata include `shadcn/ui/button`, the generated `launch-runtime-dx-check-panel` edit surface is package-scoped to UI Components, and the helper freshness markers for `ui-components:receipt-hash-refresh` survive template output.

The source DX Studio edit contract, runtime materializer, and Rust Studio manifest all include `shadcn/ui/button` in the dx-check health panel package filter. This remains `SOURCE-ONLY`: the guard proves source/materialization visibility for selected shadcn-ui/Radix UI surfaces without claiming browser UI runtime proof.

Generated `public/preview-manifest.json` now also exposes the package-owned runbook fixture through root `sourceGuardRunbookFixtures` metadata and the `/launch` route `routes[].sourceGuardRunbookFixtures` list. The generated metadata points at `docs/packages/ui-components.source-guard-runbook.json`, `ui-components-generated-starter-materialization`, `SOURCE-ONLY`, `runtimeProof: false`, and `ui-components:receipt-hash-refresh`, so Zed/DX Studio can connect generated starter output back to the UI Components source guard without parsing raw Rust or claiming browser UI runtime proof.

## UI Components Studio source-guard/runbook entry

`dx-www/src/cli/studio_manifest.rs` now publishes `ui-components-generated-starter-materialization` in `source_guard_index` and the `/launch` `source_guard_runbook_index`. Zed and DX Studio can list the exact command `dx run --test .\benchmarks\ui-components-dx-check-package-lane-panel.test.ts`, the source-only contract, and the `shadcn/ui/button` package-lane markers without scanning raw tests or running a browser.

`docs/DX_WWW_FRAMEWORK_STRUCTURE.md` now records that same Lane 20 guard beside the older package-doc guard, so framework-level worker guidance and the Studio runbook point at one source-owned UI Components generated-starter proof without claiming browser UI runtime proof.

`docs/packages/ui-components.source-guard-runbook.json` is the package-owned JSON fixture for the same guard. It records the official package name, upstream provenance, exact guard command, `/launch` `source_guard_runbook_index` contract, preview-manifest exposure fields, Zed/DX Studio markers, receipt hash helper, app-owned boundaries, and `SOURCE-ONLY` runtime limitations so tooling can read the UI Components runbook contract without parsing raw Rust source. The Rust Studio manifest now publishes the same fixture through structured `fixture_path` metadata on the source guard, `/launch` runbook contract, `/launch` command, and `source_guard_runbook_index.fixture_paths`.

This remains `SOURCE-ONLY`: the runbook proves generated starter materialization, helper freshness markers, and package-scoped dx-check discovery for selected UI Components surfaces without claiming browser UI runtime proof.

## Forge registry metadata

Registry descriptions and generated package metadata present `UI Components` as the official package lane. `shadcn-ui`, `@radix-ui/react-slot`, `@radix-ui/react-label`, and `@radix-ui/react-separator` stay provenance fields only, including `upstream_package`, `upstream_name`, `based_on`, inspected source files, and source mirror paths.

dx-style compatibility:

- UI Components surfaces use `data-slot` contracts and theme-token classes.
- Visible dashboard files avoid hardcoded hex, RGB, HSL, and single-palette Tailwind color branding.
- Unsupported utility classes remain dx-style check findings instead of being hidden by component metadata.

Zed/DX Studio markers:

- `data-dx-component="shadcn-dashboard-controls"`
- `data-dx-component="shadcn-dashboard-controls-runtime"`
- `data-dx-dashboard-workflow="operator-controls"`
- `data-dx-package="shadcn/ui/button"`
- `data-dx-shadcn-dashboard-action="set-density"`
- `data-dx-shadcn-dashboard-action="select-queue"`
- `data-dx-shadcn-dashboard-keyboard="arrow-roving-focus"`
- `data-dx-shadcn-dashboard-action="focus-target-card"`
- `data-dx-shadcn-dashboard-action="preview-dashboard-receipt"`

App-owned boundaries:

- dashboard persistence target
- final operator copy
- final accessibility review
- full upstream registry synchronization
- governed browser proof

Honesty label: `SOURCE-ONLY`.

Runtime proof is deferred. The source-owned dashboard workflow, metadata, receipt, dx-style compatibility notes, and Zed/DX Studio markers are present, but no local server, browser automation, package install, broad suite, deploy, or runtime proof was run in this lane pass.
