# Forms

official_package_name: Forms
package_id: forms/react-hook-form
upstream_package: react-hook-form
source_mirror: G:/WWW/inspirations/react-hook-form
upstream_version: 7.75.0
honesty_label: SOURCE-ONLY

Forms is the official DX Forge package lane for React form state. This slice uses the real `react-hook-form` public APIs as provenance and materializes selected source-owned surfaces into DX-WWW apps instead of hiding the integration inside `node_modules`.

Add it with `dx add forms --write`. `react-hook-form`, `rhf`, and `forms/rhf` remain accepted aliases for source provenance and migration compatibility.

## Inspected Upstream Source

- `package.json` for package metadata, export map, version, peer dependency, and MIT license context.
- `LICENSE` for the upstream MIT license.
- `src/index.ts` for public exports.
- `src/useForm.ts` for `useForm`, `register`, `handleSubmit`, form control setup, and subscription flow.
- `src/useFormContext.tsx` for provider/context boundaries.
- `src/controller.tsx` and `src/useController.ts` for controlled-field behavior.
- `src/useFieldArray.ts` for array field helpers.
- `src/types/form.ts` for `Resolver`, `FieldErrors`, submit handler, subscription, and registration contracts.

## Public API Slice

- `useForm`
- `FormProvider`
- `useFormContext`
- `register`
- `handleSubmit`
- `Controller`
- `useController`
- `useFieldArray`
- `useWatch`
- `Resolver`
- `FieldErrors`

## Materialized Surfaces

- `lib/forms/react-hook-form/form.tsx`: `DxHookForm` provider shell and `useDxHookForm`.
- `lib/forms/react-hook-form/fields.tsx`: `DxInputField`, `DxSelectField`, `DxControlledField`, `useDxFieldArray`, and field-error helpers.
- `lib/forms/react-hook-form/dry-run-receipt.ts`: `createDxFormDryRunReceipt` for local submit receipts that store field shape and boundary metadata without storing submitted values.
- `lib/forms/react-hook-form/resolver.ts`: `createDxZodResolver` and issue-to-`FieldErrors` mapping.
- `lib/forms/react-hook-form/example.tsx`: launch signup example.
- `lib/forms/react-hook-form/metadata.ts`: package id, official name, upstream provenance, surfaces, and app-owned boundaries.
- `components/launch/template-lead-form.tsx`: the front-facing launch lead workflow.

## Dashboard Usage

- Route: `/launch`
- Component marker: `data-dx-component="template-lead-form"`
- Field group marker: `data-dx-component="launch-lead-fields"`
- Package marker: `data-dx-package="forms/react-hook-form"`
- Zed/DX Studio edit marker: `data-dx-edit-id="launch.lead-form"`
- Editable form field marker: `data-dx-editable="form-fields"`
- Insert target: `data-dx-insert-slot="template-lead-form"`

The generated workflow collects launch email, notes, and enum-backed selections through the Forms provider, registered input/select helpers, textarea registration, visible error state, resolver bridge, local dry-run receipt helper, and app-owned submit handler. It is source/materialization proof only; dependency installation and governed browser proof remain app-owned.

## Receipts

- `examples/template/.dx/forge/receipts/2026-05-22-forms-dashboard-workflow.json`
- Generated starters materialize the same workflow receipt at `.dx/forge/receipts/2026-05-22-forms-dashboard-workflow.json` through the launch route contract and `dx new` writer.
- `.dx/forge/receipts/*-forms-react-hook-form.json`
- `.dx/forge/docs/forms-react-hook-form.md`

The dashboard workflow receipt records `hash_algorithm: sha256` and `file_hashes` for the curated launch lead workflow, provider/field source files, source-guard runbook fixture, and Studio manifest handoff. Package-status mirrors those hashes on the selected Forms surfaces so stale detection can compare current source bytes instead of trusting status labels alone.

## dx-check Visibility

Forms is also registered in the shared dx-check/Zed package-status read model at `examples/template/forge-package-status-read-model.ts` and `examples/template/.dx/forge/package-status.json`.

- `present`: Forms source files, metadata, docs, receipt, package catalog entry, and launch markers are present.
- `stale`: receipt hashes or inspected-source metadata no longer match the current source files.
- `missing receipt`: generated apps lack the Forms receipt path.
- `blocked`: the host app has not installed `react-hook-form` or has not approved runtime verification.
- `unsupported surface`: a requested Forms surface is outside the curated provider, field, field-array, resolver, and launch-lead workflow surfaces.

## Rust dx-check output

`core/src/ecosystem/project_check/forms_dx_check.rs` consumes the Forms package-status row and receipt path, then publishes `forms_package_present`, `forms_receipt_present`, `forms_receipt_stale`, `forms_missing_receipt`, `forms_blocked_surface`, `forms_unsupported_surface`, `forms_receipt_hash_refresh_current`, `forms_receipt_hash_refresh_stale`, and `forms_receipt_hash_refresh_missing` into the Forge `dx check` section. It raises `forms-missing-package-status`, `forms-missing-receipt`, `forms-stale-receipt`, `forms-blocked-surface`, and `forms-unsupported-surface` findings from package-status and receipt evidence without claiming browser submission proof.

Hash-backed visibility adds `forms_hash_manifest_present` and `forms_hash_mismatch`. A mismatch raises `forms-hash-mismatch` and marks the Forms receipt stale until the reviewed source files and receipt hashes are regenerated. Lower `dx check` and the DX Studio/check-panel read model both report `forms_receipt_hash_refresh_current`, `forms_receipt_hash_refresh_stale`, and `forms_receipt_hash_refresh_missing` from the package-status `receipt_hash_refresh` helper payload.

The package-owned Rust fixture `forms_hash_mismatch_metric_and_finding_are_byte_derived` creates a temporary Forms package-status row, mutates a hash-backed `template-lead-form.tsx`, and proves `forms_hash_mismatch`, `forms_receipt_stale`, and `forms-hash-mismatch` flip together from current file bytes.

The package-owned Rust fixture `forms_package_metrics_reports_helper_freshness_from_path_arrays` creates a temporary Forms package-status row, marks only `receipt_hash_refresh.stale_files`, `stale_mirror_files`, or `missing_files`, and proves lower `dx check` flips `forms_receipt_hash_refresh_*` plus `forms_receipt_stale` while `forms_hash_mismatch` stays byte-clean.

## DX Studio/check-panel Forms package row

`core/src/ecosystem/dx_check_receipt.rs` now renders a DX Studio/check-panel Forms package row from `.dx/forge/package-status.json` beside the latest `dx check` receipt. The row keeps `Forms` as the official package name, keeps `react-hook-form` `7.75.0` and `G:/WWW/inspirations/react-hook-form` as provenance, exposes selected surfaces and Zed/DX Studio source markers, mirrors `forms_hash_manifest_present` plus byte-derived `forms_hash_mismatch`, and surfaces `receiptHashRefresh` helper freshness metrics without claiming browser submission proof.

The package-row guard `forms-dx-check-package-lane-panel.test.ts` now requires the Forms row to expose `receipt_hash_refresh`, `forms_receipt_hash_refresh_current`, `forms_receipt_hash_refresh_stale`, and `forms_receipt_hash_refresh_missing` beside the existing hash mismatch metrics. The shared `receipt_hash_refresh_counts` path treats non-empty `stale_files` / `stale_mirror_files` as stale and non-empty `missing_files` / `missing_mirror_files` as missing, even if numeric helper count mirrors drift. The package-owned Rust fixture `dx_check_latest_panel_exposes_forms_package_lane_hash_row` writes a temporary Forms package-status row and dashboard workflow receipt, verifies the fresh package row, covers a stale-helper-only case by naming `docs/packages/forms-react-hook-form.md` in `receipt_hash_refresh.stale_files` while `forms_hash_mismatch = 0`, then mutates the selected launch form and proves the check-panel row flips to stale with `forms_hash_mismatch = 1`.

The static `/launch` package-lane fixture also exposes Forms before a fresh `dx check` receipt exists. `examples/template/template-shell.tsx` owns the template row and `the static launch runtime template` mirrors `data-dx-check-package-lane-template="forms/react-hook-form"`, official `Forms` naming, upstream `react-hook-form` provenance, the dashboard workflow receipt path, `forms:receipt-hash-refresh`, tracked/stale/missing helper counts, stale/missing helper path-list markers, and the `forms_receipt_hash_refresh_*` metric names without claiming browser submission proof.

The Forms generated-starter materialization guard runs `tools/launch/materialize-www-template.ts` into a temporary app, then proves generated static launch HTML keeps the Forms package-lane row, helper JSON command, `forms:receipt-hash-refresh`, stale/missing helper path-list markers, helper metric markers, `data-dx-style-surface="forms"`, `data-dx-token-scope="forms/react-hook-form"`, and `data-dx-package="forms/react-hook-form"`. It also checks generated `public/preview-manifest.json` scopes `/`, `/launch`, and `launch-runtime-dx-check-panel` to `forms/react-hook-form`; the source DX Studio edit contract, runtime materializer, and Rust Studio manifest now carry the same dx-check panel package filter without claiming browser submission proof.

The Forms Studio source-guard/runbook entry in `dx-www/src/cli/studio_manifest.rs` publishes `forms-generated-starter-materialization` and `forms-package-metrics-helper-freshness-path-arrays` for `/launch` so Zed/DX Studio can list `dx run --test .\benchmarks\forms-dx-check-package-lane-panel.test.ts` and `cargo test -q -p dx-www-compiler forms_package_metrics_reports_helper_freshness_from_path_arrays --lib` directly from `source_guard_index` and `source_guard_runbook_index`. The runbook points at the generated-starter materialization proof, the lower dx-check helper freshness proof, `data-dx-check-package-lane-row="forms/react-hook-form"`, `data-dx-token-scope="forms/react-hook-form"`, and `forms:receipt-hash-refresh` without claiming browser submission proof.

`docs/packages/forms.source-guard-runbook.json` is the package-owned JSON fixture for that same guard. It records the official package name, upstream provenance, exact guard command, `/launch` `source_guard_runbook_index` contract, Zed/DX Studio markers, receipt hash helper, app-owned boundaries, structured `fixture_path` metadata, and `SOURCE-ONLY` runtime limitations so tooling can read the Forms runbook contract without parsing raw Rust source.

Generated starter `public/preview-manifest.json` now links that same fixture through the root `sourceGuardRunbookFixtures` list and the `/launch` `routes[].sourceGuardRunbookFixtures` entry. This gives Zed/DX Studio a generated-app handoff back to the Forms runbook fixture without claiming browser submission proof.

The Forms dashboard workflow receipt now hashes that runbook fixture as the selected `forms-source-guard-runbook` surface, hashes `dx-www/src/cli/studio_manifest.rs` as the selected `forms-studio-manifest` surface, and hashes `core/src/ecosystem/project_check/forms_dx_check.rs` as the selected `forms-lower-dx-check` surface. `node tools/launch/run-template-receipt-helper.js examples/template/forms-receipt-hashes.ts --check --json` reports `source_guard_runbook_fixture`, 7 tracked files, `tracked_files`, `current_files`, `stale_files`, `missing_files`, `stale_mirror_files`, and `missing_mirror_files` so stale Studio runbook metadata, structured `fixture_path` handoff drift, or lower dx-check helper metric drift names the exact affected paths through `forms:receipt-hash-refresh` without claiming browser submission proof.

## Receipt hash helper

Forms owns a scoped receipt-hash helper at `examples/template/forms-receipt-hashes.ts`. Run `node tools/launch/run-template-receipt-helper.js examples/template/forms-receipt-hashes.ts --check` to compare the dashboard workflow receipt, `.dx/forge/package-status.json`, and `forge-package-status-read-model.ts` against current local SHA-256 bytes. Run `node tools/launch/run-template-receipt-helper.js examples/template/forms-receipt-hashes.ts --write` after reviewing source/doc changes to refresh those mirrors together.

The helper emits `dx.forge.package.receipt_hash_refresh` with `forms:receipt-hash-refresh`, keeps `Forms` as the official package name, keeps `react-hook-form` `7.75.0` as provenance, reports `source_guard_runbook_fixture`, includes the lower dx-check module in its tracked path arrays, and includes structured `tracked_files`, `current_files`, `stale_files`, `missing_files`, `stale_mirror_files`, and `missing_mirror_files` arrays for package-status and DX Studio remediation. It does not run browser submission proof, install packages, read secrets, persist form submissions, or claim final accessibility review.

The Forms package-status row mirrors that helper output as `receipt_hash_refresh`, and the typed read model mirrors it as `receiptHashRefresh`. Zed/DX Studio can read helper path, check/write/json commands, tracked/stale/missing counts, current/stale/missing path arrays, stale/missing mirror path arrays, no-runtime/no-secret flags, runtime limitations, and `forms:receipt-hash-refresh` without opening raw helper output. The helper scopes read-model hash checks to `formsPackageVisibility`, so another package row cannot mask a stale Forms hash for a shared launch file.

## App-Owned Boundaries

- Dependency installation.
- Validation schema quality and copy.
- Accessibility review for final form labels, descriptions, errors, and focus behavior.
- Submit handlers, spam protection, persistence, authorization, audit trail, and email/notification policy.
- Governed browser runtime proof.

## Verification

Run the narrow guard with:

```powershell
dx run --test .\benchmarks\forms-react-hook-form-package-doc.test.ts
dx run --test .\benchmarks\forms-package-status-read-model.test.ts
dx run --test .\benchmarks\forms-dx-check-output.test.ts
dx run --test .\benchmarks\forms-dx-check-package-lane-panel.test.ts
dx run --test .\benchmarks\forms-receipt-hash-refresh.test.ts
node tools/launch/run-template-receipt-helper.js examples/template/forms-receipt-hashes.ts --check --json
cargo test -q -p dx-www-compiler forms_hash_mismatch_metric_and_finding_are_byte_derived --lib
cargo test -q -p dx-www-compiler forms_package_metrics_reports_helper_freshness_from_path_arrays --lib
cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_forms_package_lane_hash_row --lib
```

This guard is source-only. Do not treat it as runtime proof.
