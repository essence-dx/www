# Internationalization

`i18n/next-intl` is the source-owned DX Forge Internationalization package.
Upstream provenance: `upstream_package: next-intl`, `upstream_version: 4.12.0`,
`source_mirror: G:/WWW/inspirations/next-intl`.

## Real Surface

- App Router routing through `defineRouting`.
- Navigation helpers through `createNavigation`.
- Request config through `getRequestConfig`.
- Client provider wiring through `NextIntlClientProvider`.
- Client copy and formatting through `useTranslations`, `useLocale`, and `useFormatter`.

## Dashboard Usage

The Internationalization dashboard locale workflow is wired into `/launch` as
`data-dx-component="next-intl-dashboard-locale-workflow"`. It switches product
dashboard copy between English and Bangla, updates mission-control copy,
records route preview, hreflang, locale-prefix, date/time formatter preview,
localized plan-price preview, alternate-link hints, and support SLA text, and
prepares a safe local `createDxDashboardIntlReceipt` receipt.

`dx templates --json` and launch companion receipts now expose this dashboard
workflow, exported files, the typed
`next-intl-dashboard-locale-contract.ts` copy/receipt boundary, real Bengali
dashboard copy, typed `DashboardLocaleRoutePreview` hreflang/prefix route
preview data, typed `DashboardIntlFormatPreview` `useFormatter.dateTime`
formatter data, typed `DashboardIntlNumberPreview` `useFormatter.number`
currency data, typed `DashboardLocaleAlternateLink` alternate-link data, source
mirror, receipt paths, and `pack:i18n` icon metadata so DX CLI and Zed can open
the real workflow source instead of only the older status proof component.

DX Studio also indexes the workflow as an editable surface through
`data-dx-component="next-intl-dashboard-locale-workflow"`, mapping the
responsive locale/copy card back to
`examples/template/next-intl-dashboard-locale.tsx` without requiring
template-local `node_modules`. A second Studio surface,
`next-intl-dashboard-message-contract`, maps the same visible
`LaunchDashboard` namespace back to
`examples/template/next-intl-dashboard-locale-contract.ts`, so text
edits land in the typed copy/receipt boundary instead of the rendered
component. The Studio marker index also names the package-specific selectors
`data-dx-intl-dashboard-workflow`,
`data-dx-intl-action`, `data-dx-dashboard-copy-locale`,
`data-dx-intl-copy-target="readiness"`, `data-dx-intl-readiness-copy`,
`data-dx-intl-receipt-state`, `data-dx-intl-hreflang`,
`data-dx-intl-locale-prefix`, `data-dx-intl-alternate-links`,
`data-dx-intl-format-preview`, `data-dx-intl-format-source-api`,
`data-dx-intl-format-time-zone`, `data-dx-intl-number-preview`,
`data-dx-intl-number-source-api`, `data-dx-intl-number-currency`,
`data-dx-intl-alternate-locale`, `data-dx-intl-alternate-href`, and
`data-dx-intl-message-namespace` so Zed can resolve locale buttons, readiness
copy, route hreflang/prefix data, alternate-link review hints, receipt
state, selected copy locale, and message namespace usage back to the same
source-owned workflow. The package surface now also advertises interaction
selectors for `data-dx-intl-locale-option`, `data-dx-intl-copy-target`,
route/plan/SLA copy fields, provider/preview locale state, and receipt
locale/route/hreflang outputs.

## Receipts

- `.dx/forge/receipts/*-i18n-next-intl.json`
- `.dx/forge/docs/i18n-next-intl.md`
- `examples/template/.dx/forge/receipts/2026-05-22-i18n-next-intl-dashboard-locale.json`
- `the static launch runtime template#mission-control`

## dx-check visibility

The dashboard workflow receipt exposes `dx.forge.package.dx_check_visibility`
for `i18n/next-intl`. Consumers should render the current selected
Internationalization surfaces as `present` and understand the full status
legend as present, stale, missing-receipt, blocked, and unsupported-surface.

The monitored Internationalization surfaces are
`next-intl-dashboard-locale-workflow` for the visible `/launch` locale switcher
and `next-intl-dashboard-message-contract` for the typed `LaunchDashboard`
message and receipt boundary. Production middleware placement, dependency
installation, governed browser routing proof, and SEO alternate-link review
remain app-owned runtime proof, not dx-check source proof.

The shared dx-check/Zed package-status read model also exposes the
Internationalization row with `internationalization_receipt_present`,
`internationalization_receipt_stale`, `internationalization_missing_receipt`,
`internationalization_blocked_surface`, and
`internationalization_unsupported_surface` metrics,
`internationalization_receipt_hash_refresh_current`,
`internationalization_receipt_hash_refresh_stale`, and
`internationalization_receipt_hash_refresh_missing`, plus Zed surfaces
`internationalization:next-intl-dashboard-locale-workflow` and
`internationalization:next-intl-dashboard-message-contract`.

## Receipt hash evidence

The dashboard workflow receipt now records `hash_algorithm: sha256` and
`file_hashes` for the selected Internationalization source files:
`examples/template/next-intl-dashboard-locale-contract.ts`,
`examples/template/next-intl-dashboard-locale.tsx`, and
`core/src/ecosystem/forge_next_intl.rs`, plus the package-owned
`docs/packages/next-intl.source-guard-runbook.json` fixture and shared
`tools/launch/materialize-www-template.ts` preview-manifest
materializer. The shared package-status row mirrors the source-guard fixture as
`internationalization-source-guard-runbook` and the materializer as
`internationalization-preview-manifest-materializer` so `dx check` can report
`internationalization_hash_manifest_present` and
`internationalization_hash_mismatch` without claiming live locale routing proof.
The Rust checker uses the shared `project_check/file_hashes.rs` SHA-256
comparator so generated-template root fallback and `sha256:` hash normalization
match neighboring package lanes.

## Receipt Hash Refresh

Internationalization has a package-owned helper for selected source hash
freshness:

```bash
node tools/launch/run-template-receipt-helper.js examples/template/internationalization-receipt-hashes.ts --check
node tools/launch/run-template-receipt-helper.js examples/template/internationalization-receipt-hashes.ts --write
node tools/launch/run-template-receipt-helper.js examples/template/internationalization-receipt-hashes.ts --check --json
```

The helper emits `dx.forge.package.receipt_hash_refresh`, refreshes the
dashboard workflow receipt, `.dx/forge/package-status.json`, and the typed
`forge-package-status-read-model.ts` mirror, reports
`source_guard_runbook_fixture: docs/packages/next-intl.source-guard-runbook.json`,
`preview_manifest_materializer: tools/launch/materialize-www-template.ts`,
and five `tracked_files`,
and publishes
`internationalization:receipt-hash-refresh` for Zed/DX Studio. It does not run
browser locale routing, install runtime dependencies, read secrets, or claim SEO
alternate-link/browser proof.

The DX Studio/check-panel row now consumes that same `receipt_hash_refresh`
payload as helper freshness evidence beside `internationalization_hash_mismatch`.
Current helpers emit `internationalization_receipt_hash_refresh_current`; stale
or missing helper evidence flips
`internationalization_receipt_hash_refresh_stale` or
`internationalization_receipt_hash_refresh_missing` without changing selected
source hashes. The stale helper only guard keeps this SOURCE-ONLY and does not
claim live locale routing proof.

## DX-Style Compatibility

The visible Internationalization dashboard surface declares
`data-dx-style-surface="internationalization"` and stays on dx-style token
classes (`bg-card`, `text-card-foreground`, `bg-muted`, `text-muted-foreground`,
`bg-primary`, and `text-primary-foreground`) instead of inline color objects.
The dashboard workflow receipt and package-status row mirror
`dx.forge.package.dx_style_compatibility` with token source
`styles/theme.css`, generated CSS `styles/generated.css`, selected
surface `next-intl-dashboard-locale-workflow`, source files, marker evidence,
and `runtime_proof: false`.

`dx check` reports `internationalization_dx_style_compatibility_present` or
`internationalization_dx_style_compatibility_missing`; missing evidence raises
`internationalization-missing-dx-style-compatibility` without claiming live
locale routing proof, browser visual proof, translation QA, SEO alternate-link
review, or runtime dependency installation.

## DX Studio/check-panel Internationalization row

The DX Studio/check-panel Internationalization row is rendered from the shared
`.dx/forge/package-status.json` row, not from a hidden `node_modules` package.
It preserves official **Internationalization** naming while keeping `next-intl`
as provenance metadata, exposes the selected
`next-intl-dashboard-locale-workflow` source markers, reports
`internationalization_hash_manifest_present` and
`internationalization_hash_mismatch`, consumes `receiptHashRefresh` /
`receipt_hash_refresh` helper freshness through
`internationalization_receipt_hash_refresh_current`,
`internationalization_receipt_hash_refresh_stale`, and
`internationalization_receipt_hash_refresh_missing`, and mirrors
`internationalization_dx_style_compatibility_present` /
`internationalization_dx_style_compatibility_missing` so Studio and Zed can show
missing style evidence without opening raw dx-check JSON or claiming live locale
routing proof. A stale-helper-only fixture flips
`receipt_hash_refresh.stale_file_count` while keeping
`internationalization_hash_mismatch` at zero, proving helper drift is visible as
its own Internationalization freshness state.

## Internationalization www-template package-lane template

The static `/launch` Studio preview now carries an Internationalization
package-lane template with
`data-dx-check-package-lane-template="i18n/next-intl"` and
`data-dx-style-surface="internationalization"`. The template preserves official
**Internationalization** naming, keeps `next-intl` and
`G:/WWW/inspirations/next-intl` as provenance, points to
`examples/template/.dx/forge/receipts/2026-05-22-i18n-next-intl-dashboard-locale.json`,
and marks the row `missing-receipt` until a fresh dx-check receipt replaces it
with live package-lane metrics without claiming live locale routing proof.

Internationalization static `/launch` helper freshness markers are now present
on the same source-owned row. The authored launch shell and mirrored runtime
page expose `examples/template/internationalization-receipt-hashes.ts`,
`node tools/launch/run-template-receipt-helper.js examples/template/internationalization-receipt-hashes.ts --check --json`,
`internationalization:receipt-hash-refresh`, the selected source counts, and
the `internationalization_receipt_hash_refresh_current`,
`internationalization_receipt_hash_refresh_stale`, and
`internationalization_receipt_hash_refresh_missing` metric ids before a fresh
dx-check receipt is loaded. Generated-starter materialization preserves those
markers in generated static launch HTML, keeping the helper SOURCE-ONLY and without
claiming live locale routing proof.

The Rust DX Studio manifest now mirrors that source-only contract through the
`internationalization-launch-package-lane-template` guard, the `/launch`
source-guard runbook, and the `i18n/next-intl` package-surface marker index. It
lists the helper path, JSON check command,
`internationalization:receipt-hash-refresh`, selected source counts, metric ids,
and `data-dx-check-package-lane-hash-refresh-*` marker names, so Zed/DX Studio
can rerun the helper from the source-guard runbook without scraping the runtime
page first or claiming live locale routing proof.

`docs/packages/next-intl.source-guard-runbook.json` is the package-owned
SOURCE-ONLY runbook fixture for that guard. It mirrors the official
Internationalization package metadata, upstream `next-intl` provenance, selected
surfaces, inspected upstream files, helper command, dx-check metrics,
Zed/DX Studio marker names, app-owned boundaries, and runtime limitations so
tools can read the contract without parsing `studio_manifest.rs`.

Generated-starter proof is now attached to the same guard. The focused
benchmark runs `tools/launch/materialize-www-template.ts` into a
temporary project and verifies generated static launch HTML plus
`public/preview-manifest.json` keep `i18n/next-intl` route package metadata,
the package-lane markers, dx-style markers, `data-dx-intl-dashboard-workflow`,
and the `launch-runtime-dx-check-panel` package scope. This remains
SOURCE-ONLY and does not claim live locale routing proof, browser visual proof,
SEO alternate-link review, or runtime dependency installation.

Generated `public/preview-manifest.json` now also links the package-owned
runbook fixture from root `sourceGuardRunbookFixtures` and the `/launch`
`routes[].sourceGuardRunbookFixtures` list. The fixture records that preview
manifest contract so Zed/DX Studio can discover the Internationalization
runbook from generated starter output without parsing Rust manifest code or
claiming live locale routing proof. The materializer that emits this generated
metadata is now hash-backed in the Internationalization receipt helper, receipt,
package-status row, and typed read model so future preview-manifest fixture
drift is stale-detectable through `internationalization:receipt-hash-refresh`.

## Rust dx-check output

`core/src/ecosystem/project_check/internationalization_dx_check.rs` consumes the
same package-status row and emits `internationalization_package_present`,
`internationalization_receipt_present`, `internationalization_receipt_stale`,
`internationalization_missing_receipt`,
`internationalization_blocked_surface`, and
`internationalization_unsupported_surface`,
`internationalization_hash_manifest_present`,
`internationalization_hash_mismatch`,
`internationalization_dx_style_compatibility_present`, and
`internationalization_dx_style_compatibility_missing` in the Forge `dx check`
section.
It raises `internationalization-missing-package-status`,
`internationalization-missing-receipt`, `internationalization-stale-receipt`,
`internationalization-blocked-surface`, and
`internationalization-unsupported-surface` findings without claiming live
locale routing proof. Hash drift raises `internationalization-hash-mismatch`
from selected source file bytes through `project_check/file_hashes.rs`; missing
style evidence raises `internationalization-missing-dx-style-compatibility`.

## Four-Pass Verdict

Verdict: REAL for coding readiness. The audited receipt records the upstream
source files, exported next-intl public APIs, Forge package files, `/launch`
dashboard consumer, runtime bridge, DX Studio markers, partial runtime
boundaries, the typed `DashboardLocaleRoutePreview` route preview contract, the
typed `DashboardLocaleAlternateLink` alternate-link contract, the typed
`DashboardIntlFormatPreview` formatter contract, the typed
`DashboardIntlNumberPreview` number-formatting contract, and the
package-specific source guard. The remaining partial work is governed
browser proof, production locale routing verification, SEO alternate-link
review, and runtime dependency verification.

## App-Owned Boundary

The app still owns translation quality, locale routing policy, middleware
placement, SEO metadata, production route alternates, and runtime dependency
installation. This slice does not create local `node_modules`.

Status: Internationalization dashboard locale workflow source receipt, dashboard-first
template discovery, DX Studio/Web Preview source mapping, package-specific
interaction marker indexing, dx-check visibility receipt metadata, and UTF-8
copy-integrity guard coverage are wired; runtime browser verification remains
pending under Friday/user governance.
