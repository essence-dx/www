# Authentication

official DX package name: `Authentication`

upstream_package: `better-auth`
source_mirror: `G:/WWW/inspirations/better-auth`
upstream_version: `1.6.11`
package_id: `auth/better-auth`
honesty: `LOCK-BACKED / ADAPTER-BOUNDARY`

## Source Surfaces

Authentication materializes selected front-facing source files for a React/Next-style app while preserving the upstream better-auth package as provenance. The source-owned files expose real upstream APIs such as `betterAuth`, `createAuthClient`, `toNextJsHandler`, `nextCookies`, `auth.api.getSession`, `auth.api.listSessions`, `auth.api.revokeSession`, `authClient.signIn.email`, `authClient.signUp.email`, `authClient.signIn.social`, `authClient.linkSocial`, `authClient.updateUser`, `authClient.changeEmail`, and `authClient.revokeOtherSessions`.

Generated source remains editable under `auth/better-auth/*`, `components/launch/auth-session-status.tsx`, and the launch receipt paths `.dx/forge/receipts/packages/auth-better-auth.json` and `.dx/forge/receipts/auth-better-auth.json`. The upstream package name stays in `upstream_package`, `source_mirror`, and provenance fields only.

Use `dx add authentication --write` for front-facing install copy. The aliases `better-auth`, `auth/betterauth`, and `auth/better-auth-next` remain supported for compatibility and provenance, but dashboards, docs, receipts, and CLI help should present the official package name `Authentication`.

## Surfaces

- Server options and route helpers for `betterAuth` plus `toNextJsHandler`.
- Next App Router catch-all route source at `app/api/auth/[...all]/route.ts`, re-exporting the app-owned `server/auth/better-auth.ts` GET/POST handlers that delegate to the source-owned package route after readiness passes.
- Next App Router readiness route source at `app/api/auth/readiness/route.ts`, returning missing env, database adapter, and session-storage boundary state without executing OAuth.
- App-owned server boundary source at `server/auth/better-auth.ts`, where a template can pass a real `BetterAuthOptions["database"]` adapter into the Authentication route handlers without Forge pretending one exists.
- Template readiness receipt at `.dx/forge/template-readiness/authentication.json`, making the default template's Authentication caveats concrete and hash-backed instead of inferred from a missing readiness folder.
- Client creation and session helpers around `createAuthClient` and `useSession`.
- Email/password, Google provider, account linking, profile, account deletion, account security, and session-management helpers.
- Dashboard workflow markers for login/session/account readiness and missing-config receipts.
- Zed/DX Studio markers through `data-dx-component`, `data-dx-auth-*`, receipt paths, and package metadata.

## dx-check visibility

Schema: `dx.forge.package.dx_check_visibility`

- `present`: selected Authentication surfaces, source markers, and the dashboard workflow receipt are present.
- `stale`: materialized Authentication files or source markers no longer match the receipt-backed surface contract.
- `missing-receipt`: selected Authentication surfaces exist without `.dx/forge/receipts/packages/auth-better-auth.json` or the dashboard workflow receipt `.dx/forge/receipts/auth-better-auth.json`.
- `blocked`: credentials, database/session policy, OAuth provider setup, email delivery, or governed runtime proof is required before claiming more.
- `unsupported-surface`: a requested Authentication surface is outside the selected upstream-backed set.

Monitored surfaces include `authentication-account-workflow` in `examples/template/template-shell.tsx`, `authentication-session-status` in `examples/template/auth-session-status.tsx`, `authentication-app-route-handler` in `examples/template/app/api/auth/[...all]/route.ts`, `authentication-readiness-route-handler` in `examples/template/app/api/auth/readiness/route.ts`, `authentication-app-server-boundary` in `examples/template/server/auth/better-auth.ts`, and `authentication-template-readiness-receipt` in `examples/template/.dx/forge/template-readiness/authentication.json`. They remain `ADAPTER-BOUNDARY`: source and receipt visibility are checkable, while deployed credentials, cookies, database adapters, and live OAuth are app-owned.

## Receipt Hash Refresh

The package-owned helper `examples/template/authentication-receipt-hashes.ts` checks and refreshes the selected Authentication SHA-256 mirrors across `examples/template/.dx/forge/receipts/auth-better-auth.json`, `examples/template/.dx/forge/package-status.json`, and `examples/template/forge-package-status-read-model.ts`. It also tracks `docs/packages/authentication.source-guard-runbook.json` as the selected `authentication-source-guard-runbook` surface, `examples/template/.dx/forge/template-readiness/authentication.json` as the selected `authentication-template-readiness-receipt` surface, `dx-www/src/cli/studio_manifest.rs` as the selected `authentication-studio-manifest-source` surface, and `tools/launch/materialize-www-template.ts` as the selected `authentication-preview-manifest-materializer` surface so Studio/Zed runbook fixture, template-readiness caveats, Studio manifest declaration, and preview-manifest materializer drift are stale-detectable instead of living outside receipt freshness.

Use `node tools/launch/run-template-receipt-helper.js examples/template/authentication-receipt-hashes.ts --check` to verify current source bytes, `node tools/launch/run-template-receipt-helper.js examples/template/authentication-receipt-hashes.ts --check --json` for the `dx.forge.package.receipt_hash_refresh` report, and `node tools/launch/run-template-receipt-helper.js examples/template/authentication-receipt-hashes.ts --write` after intentional selected-surface edits. The helper reports `source_guard_runbook_fixture: docs/packages/authentication.source-guard-runbook.json`, `studio_manifest_source: dx-www/src/cli/studio_manifest.rs`, `preview_manifest_materializer: tools/launch/materialize-www-template.ts`, eleven tracked files, exact `tracked_files`, `current_files`, `stale_files`, `missing_files`, `stale_mirror_files`, `missing_mirror_files`, `mirror_problem_count` attribution, and `authentication:receipt-hash-refresh` for Zed/DX Studio visibility. A Studio-manifest-only, materializer-only, template-readiness-only, readiness-route-only, or app-server-boundary-only drift names the specific helper path without marking the selected account workflow, session status, dashboard workflow, or source-guard runbook fixture files stale. It does not run Better Auth, read secrets, open a browser, or prove live OAuth.

## DX-Style Compatibility

The visible Authentication launch account workflow declares `data-dx-style-surface="authentication-account-workflow"`, and the session panel declares `data-dx-style-surface="authentication-session-status"`. Both selected surfaces use dx-style token classes such as `bg-card`, `text-card-foreground`, `border-border`, `text-muted-foreground`, and `bg-muted` instead of package-specific hardcoded color contracts.

The Authentication receipt and shared package-status row expose `dx.forge.package.dx_style_compatibility` with `styles/theme.css` as the token source and `styles/generated.css` as the generated CSS target. Rust dx-check reports `authentication_dx_style_compatibility_present` and `authentication_dx_style_compatibility_missing`, and raises `authentication-missing-dx-style-compatibility` if the package-status row loses that evidence.

This is SOURCE-ONLY style evidence. Live OAuth/session visual proof, browser QA, deployed cookies, and theme review remain app-owned.

## Rust dx-check output

`core/src/ecosystem/project_check/authentication_dx_check.rs` consumes `.dx/forge/package-status.json` and the `.dx/forge/receipts/auth-better-auth.json` receipt to publish `authentication_*` Forge metrics. The receipt now carries `hash_algorithm: sha256` and per-surface `file_hashes` for the selected launch shell, session status, and dashboard workflow files. Rust dx-check routes hash comparison through the shared `project_check/file_hashes.rs` helper so generated-root fallback and `sha256:` normalization match neighboring package lanes. It publishes `authentication_hash_manifest_present`, `authentication_hash_mismatch`, `authentication_receipt_hash_refresh_current`, `authentication_receipt_hash_refresh_stale`, `authentication_receipt_hash_refresh_missing`, `authentication_dx_style_compatibility_present`, and `authentication_dx_style_compatibility_missing`; source hash mismatches still emit `authentication-hash-mismatch`, and any hash mismatch or helper-only drift derives `authentication_receipt_stale` plus `authentication-stale-receipt` before claiming source freshness. Helper drift is counted from both the numeric helper counters and path arrays such as `receipt_hash_refresh.stale_files`, `receipt_hash_refresh.stale_mirror_files`, `receipt_hash_refresh.missing_files`, and `receipt_hash_refresh.missing_mirror_files`, so materializer-only stale paths remain visible without pretending selected Authentication source hashes changed. The Rust section also reports `authentication-missing-receipt`, `authentication-blocked-surface`, `authentication-unsupported-surface`, and `authentication-missing-dx-style-compatibility` findings from the receipt-backed package-status row without claiming live OAuth or session runtime proof.

The package-owned fixture `authentication_package_metrics_reports_missing_dx_style_compatibility` writes a temporary Authentication receipt and package-status row, proves a row without `dx_style_compatibility` flips `authentication_dx_style_compatibility_missing` plus `authentication-missing-dx-style-compatibility`, and proves the present metric returns when `dx.forge.package.dx_style_compatibility` evidence is restored. This keeps the proof at the package metrics boundary while live OAuth, browser QA, cookies, and deployed session storage stay app-owned.

The helper-freshness fixture `authentication_package_metrics_reports_helper_freshness_from_path_arrays` writes a temporary Authentication package-status row with current helper arrays, then changes only `receipt_hash_refresh.stale_files` and `receipt_hash_refresh.stale_mirror_files` to prove `authentication_receipt_hash_refresh_stale` flips while `authentication_hash_mismatch` stays zero. It also proves `receipt_hash_refresh.missing_files` flips `authentication_receipt_hash_refresh_missing`. This is SOURCE-ONLY dx-check evidence, not live Better Auth runtime proof.

## DX Studio / Check-Panel Row

`core/src/ecosystem/dx_check_receipt.rs` now reads the Authentication package-status row into `view_model.package_lane_rows` as official package `Authentication` with package id `auth/better-auth`. The row preserves `upstream_package: better-auth`, `upstream_version: 1.6.11`, and `source_mirror: G:/WWW/inspirations/better-auth` as provenance metadata only.

The row surfaces selected `authentication-account-workflow` and `authentication-session-status` source markers, receipt presence, runtime limitations, `receipt_hash_refresh`, `authentication_hash_manifest_present`, `authentication_hash_mismatch`, `authentication_receipt_hash_refresh_current`, `authentication_receipt_hash_refresh_stale`, `authentication_receipt_hash_refresh_missing`, `authentication_dx_style_compatibility_present`, and `authentication_dx_style_compatibility_missing`. Helper-stale state makes the row stale even when selected source file bytes still match, and helper-missing state gives Studio/Zed a direct next action to restore the package-owned helper metadata. It also reports blocked app-owned runtime surfaces when provider credentials, callback URLs, cookies, email delivery, database adapters, or hosted sessions are required before claiming runtime proof.

The targeted shared-reader fixture is `cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_authentication_package_lane_hash_refresh_row --lib`. It writes temporary Authentication account/session source files, proves the current helper row reports `authentication_receipt_hash_refresh_current`, then flips only `receipt_hash_refresh.stale_file_count` so `authentication_receipt_hash_refresh_stale` becomes visible while `authentication_hash_mismatch` stays zero.

The launch shell and static `/launch` runtime fixture also carry a receipt-less Authentication package-lane template with `data-dx-check-package-lane-template="auth/better-auth"`, `data-dx-check-package-lane-row="auth/better-auth"`, `data-dx-check-package-lane-dx-style-status="present"`, `data-dx-check-package-lane-hash-refresh-current-file-list`, `data-dx-check-package-lane-hash-refresh-stale-file-list`, `data-dx-check-package-lane-hash-refresh-missing-file-list`, `data-dx-check-package-lane-hash-refresh-stale-mirror-file-list`, `data-dx-check-package-lane-hash-refresh-missing-mirror-file-list`, `data-dx-style-surface="authentication-account-workflow"`, and `data-dx-token-scope="auth/better-auth"`. This lets DX Studio and Zed discover the official Authentication lane and helper path attribution before a fresh dx-check receipt has been loaded.

This is SOURCE-ONLY/ADAPTER-BOUNDARY panel evidence. It does not claim live OAuth, deployed cookies, or hosted session runtime proof.

## DX Studio Source-Guard Runbook

`docs/packages/authentication.source-guard-runbook.json` is the package-owned source-guard runbook fixture for the `authentication-package-lane-panel` guard. It mirrors the `/launch` `source_guard_runbook_index` contract, selected Authentication source markers, upstream Better Auth provenance, the package-owned helper `authentication:receipt-hash-refresh`, and the exact lightweight command `dx run --test .\benchmarks\authentication-dx-check-package-lane-panel.test.ts`.

DX Studio and Zed can read the fixture path from `dx-www/src/cli/studio_manifest.rs` without parsing raw Rust strings, and the receipt helper now hashes the fixture itself through `authentication-source-guard-runbook`, the template-readiness receipt through `authentication-template-readiness-receipt`, and the Studio manifest source through `authentication-studio-manifest-source`. The fixture is ADAPTER-BOUNDARY evidence: it proves source-owned Authentication package-lane visibility, but live OAuth, deployed cookies, database adapters, email delivery, and hosted session runtime proof stay app-owned.

The Studio manifest source-guard proof list now publishes the exact helper path-list marker names (`data-dx-check-package-lane-hash-refresh-current-file-list`, `data-dx-check-package-lane-hash-refresh-stale-file-list`, `data-dx-check-package-lane-hash-refresh-missing-file-list`, `data-dx-check-package-lane-hash-refresh-stale-mirror-file-list`, and `data-dx-check-package-lane-hash-refresh-missing-mirror-file-list`) plus `authentication_receipt_hash_refresh_current`, `authentication_receipt_hash_refresh_stale`, and `authentication_receipt_hash_refresh_missing`. That keeps Studio/Zed discovery aligned with the static `/launch` markers without requiring a live OAuth/session run.

Generated starter snapshots also expose this fixture through `public/preview-manifest.json`: the root `sourceGuardRunbookFixtures` array carries the Authentication fixture object, and the `/launch` route lists `docs/packages/authentication.source-guard-runbook.json` in `routes[].sourceGuardRunbookFixtures`. That preview-manifest link is still ADAPTER-BOUNDARY metadata only; it does not run Better Auth or prove OAuth/session runtime behavior.

## Required Env

- `BETTER_AUTH_SECRET`
- `BETTER_AUTH_URL`
- `GOOGLE_CLIENT_ID`
- `GOOGLE_CLIENT_SECRET`

Optional app env:

- `BETTER_AUTH_TRUSTED_ORIGINS`
- `NEXT_PUBLIC_BETTER_AUTH_URL`

## Boundaries

Authentication does not claim live OAuth, deployed session storage, email delivery, database migration, account deletion governance, or production cookie policy. The app owns credentials, trusted origins, the database adapter, session lifetime, Google callback URLs, email verification, account linking policy, audit logging, and browser/runtime proof. The app-owned server boundary shows exactly where to pass a real Better Auth database adapter; the readiness route only reports whether those app-owned boundaries are configured.
