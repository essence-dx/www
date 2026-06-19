# DX Forge CI Smoke And Benchmark Sequence

This document defines the launch-safe Forge verification path for CI. It is intentionally local and offline by default: no Cloudflare R2 secrets, no browser plugin, and no full workspace build are required for the normal smoke lane.

Run commands from `G:\WWW`.

For the end-to-end public release sequence, use `docs/forge-public-launch-checklist.md` after this CI smoke lane is green.
For the final human publication review, use `docs/forge-public-launch-handoff.md` after the public launch checklist and release-review gate pass.
For the adoption-first release path, use `docs/forge-adoption-launch-checklist.md` after the local clean-project adoption smoke is green.

## Fast Smoke Lane

Use this lane for pull requests and frequent automation loops. It verifies that the active `dx-www` crate compiles, the public Forge launch path can run in a temp project, and no `node_modules` folder is created.

```powershell
.\scripts\ci\forge-ci.ps1 -SkipArtifactLane
```

The smoke test exercises the same release path as the CLI:

- `dx add ui/button --write`
- `dx add icon search --write`
- `dx add auth/better-auth --write`
- `dx check --strict-forge`
- `dx forge doctor`
- `dx forge verify-package --all`
- `dx forge scorecard`
- `dx forge launch-page`
- `dx forge evidence`
- launch-page quality checks for headings, SEO/static shape, links, and claims manifest consistency

## CLI Smoke Artifact Lane

Use this lane when CI should persist machine-readable evidence as artifacts.

```powershell
.\scripts\ci\forge-ci.ps1 -ArtifactDir .\.dx\ci -FailUnder 90
```

To also prepare a static publish bundle for GitHub Pages, pass `-PagesDir`:

```powershell
.\scripts\ci\forge-ci.ps1 -ArtifactDir .\.dx\ci -PagesDir .\.dx\forge-pages -FailUnder 90
```

Expected artifacts:

- `.dx/ci/forge-smoke.json`
- `.dx/ci/forge-smoke.md`
- `.dx/ci/forge-triage.md`
- `.dx/ci/forge-readiness-badge.json`
- `.dx/ci/forge-evidence.json`
- `.dx/ci/forge-scorecard.json`
- `.dx/ci/forge-benchmark-history.json`
- `.dx/ci/forge.html`
- `.dx/ci/forge.claims.json`
- `.dx/ci/forge.evidence.json`
- `.dx/ci/forge.dxp`
- `.dx/ci/forge-proof.json`
- `.dx/ci/forge-public-route-comparison.json` when `-PagesDir` is provided
- `.dx/ci/forge-release-dashboard.md` when `-PagesDir` is provided
- `.dx/ci/forge-release-dashboard.json` when `-PagesDir` is provided
- `.dx/ci/forge-public-release-history.json` when `-PagesDir` is provided
- `.dx/ci/forge-public-release-history.md` when `-PagesDir` is provided
- `.dx/ci/forge-public-launch-changelog.json` when `-PagesDir` is provided
- `.dx/ci/forge-public-launch-changelog.md` when `-PagesDir` is provided
- `.dx/ci/forge-public-evidence-verify.json` when `-PagesDir` is provided
- `.dx/ci/forge-trust-regression.json` when `-PagesDir` is provided
- `.dx/ci/forge-trust-regression.md` when `-PagesDir` is provided
- `.dx/ci/forge-release-candidate.json` when `-PagesDir` is provided
- `.dx/ci/forge-release-candidate.md` when `-PagesDir` is provided
- `.dx/ci/forge-release-bundle-adoption/forge-release-manifest.json` when `-PagesDir` is provided
- `.dx/ci/forge-manifest-signing.json` when `-PagesDir` is provided
- `.dx/ci/forge-registry-smoke.json` when `-PagesDir` is provided
- `.dx/ci/forge-registry-smoke.md` when `-PagesDir` is provided
- `.dx/ci/forge-release-operations.json` when `-PagesDir` is provided
- `.dx/ci/forge-release-operations.md` when `-PagesDir` is provided
- `.dx/ci/forge-publish-plan.json` when `-PagesDir` is provided
- `.dx/ci/forge-publish-plan.md` when `-PagesDir` is provided
- `forge-public-launch-changelog.json` and `.md` inside any `dx forge release-bundle --out <dir>` publish bundle
- `forge/changelog.html`, `forge/changelog/index.html`, `forge/changelog.claims.json`, `forge/changelog.dxp`, and `forge/changelog.proof.json` inside any full `dx forge release-bundle --out <dir>` publish bundle

These commands must not create `node_modules`.

`forge-readiness-badge.json` is the compact status artifact for README badges, release dashboards, and CI summaries. It is derived from the Forge smoke report, release proof, package scorecard, launch-page quality checks, no-`node_modules` status, and the latest `/forge` benchmark snapshot.

`forge-triage.md` is the human-readable failure report for CI review. It combines smoke findings, launch-page quality checks, strict launch-gate findings, package/evidence gate status, the readiness score, and the latest `/forge` budget signal.

## Secret-Free Badge And Public Forge Route Publishing

The GitHub Actions workflow prepares a second artifact named `dx-forge-pages` from `.dx/forge-pages`. It is safe to create on pull requests because it is built from local Forge evidence only and does not read R2 credentials.

The publish bundle contains:

- `.dx/forge-pages/forge-readiness-badge.json`
- `.dx/forge-pages/forge/ci.html`
- `.dx/forge-pages/forge/ci/index.html`
- `.dx/forge-pages/forge/ci.claims.json`
- `.dx/forge-pages/forge/ci.dxp`
- `.dx/forge-pages/forge/ci.proof.json`
- `.dx/forge-pages/forge/releases.html`
- `.dx/forge-pages/forge/releases/index.html`
- `.dx/forge-pages/forge/releases.claims.json`
- `.dx/forge-pages/forge/releases.dxp`
- `.dx/forge-pages/forge/releases.proof.json`
- `.dx/forge-pages/forge/changelog.html`
- `.dx/forge-pages/forge/changelog/index.html`
- `.dx/forge-pages/forge/changelog.claims.json`
- `.dx/forge-pages/forge/changelog.dxp`
- `.dx/forge-pages/forge/changelog.proof.json`
- `.dx/forge-pages/forge/adoption.html`
- `.dx/forge-pages/forge/adoption/index.html`
- `.dx/forge-pages/forge/adoption.claims.json`
- `.dx/forge-pages/forge/adoption.dxp`
- `.dx/forge-pages/forge/adoption.proof.json`
- `.dx/forge-pages/proof.json`

The CI script validates that publish shape before upload with:

```powershell
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- forge ci --verify-pages .\.dx\forge-pages --fail-under 90
```

That verifier fails on missing readiness badge JSON, missing clean-route `/forge/ci/index.html`, `/forge/releases/index.html`, `/forge/changelog/index.html`, or `/forge/adoption/index.html`, invalid claims or proof JSON, a missing `DXPK` proof artifact, `node_modules`, or leaked secret-environment markers.

After the Pages verifier passes, the script runs the release dashboard gate:

```powershell
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- forge release-dashboard --project <temp> --ci-artifacts .\.dx\ci --pages .\.dx\forge-pages --history .\.dx\ci\forge-benchmark-history.json --route-comparison .\.dx\ci\forge-public-route-comparison.json --format markdown --output .\.dx\ci\forge-release-dashboard.md --fail-under 90 --quiet
```

This uses only local artifacts, the checked-in public route comparison JSON, and a temp project; it must not read secret environment variables or create `node_modules`.

The CI script then records the release-dashboard JSON and route comparison into local release-history artifacts:

```powershell
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- forge release-history --dashboard .\.dx\ci\forge-release-dashboard.json --route-comparison .\.dx\ci\forge-public-route-comparison.json --output .\.dx\ci\forge-public-release-history.json --format markdown --quiet
```

The same artifact lane then generates launch-changelog JSON and Markdown from the reviewed release-history evidence:

```powershell
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- forge launch-changelog --history .\.dx\ci\forge-public-release-history.json --output .\.dx\ci\forge-public-launch-changelog.json --format json --fail-under 90 --quiet
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- forge launch-changelog --history .\.dx\ci\forge-public-release-history.json --output .\.dx\ci\forge-public-launch-changelog.md --format markdown --fail-under 90 --quiet
```

GitHub Actions checks that `forge-public-release-history.json`, `forge-public-release-history.md`, `forge-public-launch-changelog.json`, and `forge-public-launch-changelog.md` exist before uploading `dx-forge-ci`.

The same `-PagesDir` lane now continues into the public beta shipping gate. It creates a temp publisher key outside `.dx/ci`, signs the release-bundle manifest, runs the dry-run R2 registry smoke, joins trust-regression and release-candidate reports, and writes both machine and human reports for release-operations and publish-plan:

```powershell
dx forge release-bundle --project <temp> --out .\.dx\ci\forge-release-bundle-adoption --include-adoption --format json --fail-under 90 --quiet
dx forge publisher-key generate --out <temp-publisher-key-dir> --signer dx-forge-ci --force --format json --quiet
dx forge publisher-key sign --key <temp-publisher-key-dir>\publisher-key.private.json --manifest .\.dx\ci\forge-release-bundle-adoption\forge-release-manifest.json --manifest-output .\.dx\ci\forge-release-bundle-adoption\forge-release-manifest.json --format json --output .\.dx\ci\forge-manifest-signing.json --quiet
dx forge registry smoke --remote r2 --local .\.dx\ci\forge-registry-smoke --format json --output .\.dx\ci\forge-registry-smoke.json --fail-under 90 --quiet
dx forge release-operations --project <temp> --release-manifest .\.dx\ci\forge-release-bundle-adoption\forge-release-manifest.json --trust-regression .\.dx\ci\forge-trust-regression.json --release-candidate .\.dx\ci\forge-release-candidate.json --ci-artifacts .\.dx\ci --public-evidence .\.dx\ci\forge-public-evidence --format json --output .\.dx\ci\forge-release-operations.json --fail-under 90 --quiet
dx forge publish-plan --project <temp> --release-bundle .\.dx\ci\forge-release-bundle-adoption --pages .\.dx\forge-pages --registry-smoke .\.dx\ci\forge-registry-smoke.json --release-operations .\.dx\ci\forge-release-operations.json --format json --output .\.dx\ci\forge-publish-plan.json --fail-under 90 --quiet
```

The private publisher key is generated under the runner temp directory and is not copied into `.dx/ci` or the Pages preview. The uploaded evidence is the signed manifest plus the non-secret signing, registry-smoke, release-operations, and publish-plan reports.

For tracked release proof after the full public route measurement suite has been rerun, write the machine-readable dashboard and append it to the benchmark-side release history:

```powershell
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- forge release-dashboard --project <temp> --ci-artifacts .\.dx\ci --pages .\.dx\forge-pages --history .\.dx\ci\forge-benchmark-history.json --route-comparison .\benchmarks\reports\forge-public-route-comparison.json --format json --output .\.dx\ci\forge-release-dashboard.json --fail-under 90 --quiet
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- forge release-history --dashboard .\.dx\ci\forge-release-dashboard.json --route-comparison .\benchmarks\reports\forge-public-route-comparison.json --output .\benchmarks\reports\forge-public-release-history.json --format markdown --quiet
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- forge launch-changelog --history .\benchmarks\reports\forge-public-release-history.json --format markdown --output .\benchmarks\reports\forge-public-launch-changelog.md --fail-under 90 --quiet
```

This updates `benchmarks/reports/forge-public-release-history.json`, `.md`, and the human launch changelog from local evidence only.

To keep public framework comparison copy honest, refresh the static-floor competitor evidence after the public route comparison is current:

```powershell
node .\benchmarks\compare-forge-static-competitors.ts
```

This writes `benchmarks/reports/forge-static-competitor-evidence.json` and `.md`. It intentionally does not run Astro, Svelte, HTMX, or Next.js builds; the report is a generous static-floor fixture for public Forge routes, not a full framework benchmark. The report also includes the focused `/forge/adoption` browser-evidence row from `forge-public-route-comparison.json` beside deterministic Astro, Svelte, HTMX, and Next.js static floors.

When a machine already has local baseline framework projects installed, generate the separated live-harness report:

```powershell
node .\benchmarks\compare-forge-live-frameworks.ts
```

By default this writes `benchmarks/reports/forge-live-framework-harness.json` and `.md` without running framework builds. To run installed local baselines, set `DX_FORGE_LIVE_FRAMEWORKS=1` first. The harness never runs package installs; missing `node_modules` or build scripts become skipped rows, while deterministic static-floor evidence remains labeled separately.

To verify the real adoption app public routes with local browser evidence, run:

```powershell
node .\benchmarks\measure-forge-adoption-browser-smoke.ts
```

This writes `benchmarks/reports/forge-adoption-browser-smoke.json` and `.md`, checks all six generated public Forge routes, and records local Chrome load timing when a Chrome or Edge executable is available. It still creates no `node_modules`.

To rehearse source-owned package updates in the real adoption app, run:

```powershell
node .\benchmarks\measure-forge-package-update-rehearsal.ts
```

This writes `benchmarks/reports/forge-package-update-rehearsal.json` and `.md`, covering green update writes, yellow local-edit review, red quarantine, rollback restoration, strict-check evidence, and no `node_modules`.

To review the source-owned package fixture as one launch artifact, run:

```powershell
node .\benchmarks\measure-forge-source-owned-package-review.ts
```

This writes `benchmarks/reports/forge-source-owned-package-review.json` and `.md`, joining package docs, receipts, curated advisory placeholders, `dx forge verify-package --all`, local-edit yellow review, rollback rehearsal, and no-`node_modules` proof for the adoption example app. Curated packages must carry advisory placeholders; local compiler-owned source artifacts remain visible without being treated as curated registry packages.

To record the beta installability snapshot after the beta artifacts and package-review reports exist, run:

```powershell
node .\benchmarks\measure-forge-installability-snapshot.ts
```

This writes `benchmarks/reports/forge-installability-snapshot.json` and `.md`, comparing local Forge beta install/upgrade timing and artifact-size evidence with static npm and shadcn reference rows. It intentionally does not run npm, npx, shadcn, framework builds, or package installs, and it must not create `node_modules`.

To generate the public adoption route from the real adoption report, run inside a prepared adoption project:

```powershell
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- prove vertical --fixture forge-adoption --out public --write --format markdown
```

This writes `/forge/adoption` as static/no-runtime DX-WWW output with a route claims manifest and local adoption-report evidence. The normal `dx forge ci` artifact lane now writes and verifies the adoption report plus `/forge/adoption` route artifacts without R2 secrets, `node_modules`, or a full workspace build.

The PowerShell CI wrapper builds the `dx-www` binary once before the artifact lane, then calls that executable directly for Forge artifact, Pages, dashboard, history, and changelog commands. Keep it that way so low-resource machines do not pay repeated `cargo run` startup cost or print repeated Cargo warning/output blocks for every artifact step.

## Release History Preservation

Release-history artifacts are the public launch review trail for dashboard score, route totals, payload drift, budget failures, missing routes, and no-`node_modules` behavior. Preserve JSON and Markdown together so humans and automation review the same state.

Short-lived CI artifacts:

- `.dx/ci/forge-public-release-history.json`
- `.dx/ci/forge-public-release-history.md`
- `.dx/ci/forge-release-dashboard.json`
- `.dx/ci/forge-release-dashboard.md`
- `.dx/ci/forge-public-route-comparison.json`
- `.dx/ci/forge-public-launch-changelog.json`
- `.dx/ci/forge-public-launch-changelog.md`
- `.dx/ci/forge-adoption-smoke.json`
- `.dx/ci/forge-adoption-report.json`
- `.dx/ci/forge-adoption-report.md`
- `.dx/ci/forge/adoption.html`
- `.dx/ci/forge/adoption.claims.json`
- `.dx/ci/forge/adoption.proof.json`
- `.dx/ci/forge-public-evidence-verify.json`
- `.dx/ci/forge-release-candidate.json`
- `.dx/ci/forge-release-candidate.md`
- `.dx/ci/forge-registry-smoke.json`
- `.dx/ci/forge-registry-smoke.md`
- `.dx/ci/forge-release-operations.json`
- `.dx/ci/forge-release-operations.md`
- `.dx/ci/forge-publish-plan.json`
- `.dx/ci/forge-publish-plan.md`

Promoted launch-review artifacts:

- `benchmarks/reports/forge-public-release-history.json`
- `benchmarks/reports/forge-public-release-history.md`
- `benchmarks/reports/forge-public-launch-changelog.json`
- `benchmarks/reports/forge-public-launch-changelog.md`
- `benchmarks/reports/forge-public-route-comparison.json`
- `benchmarks/reports/forge-public-route-comparison.md`

Only the promoted `benchmarks/reports` files should be treated as the durable launch record. The `.dx/ci` copies are workflow evidence and can expire with normal CI retention after review.

By default, release-history regression checks are strict: dashboard score drops, failed route budgets, missing routes, total payload growth, and per-route payload growth become review findings. When a reviewed launch intentionally adds public routes, record that policy in `dx.config.toml` instead of accepting unexplained payload growth:

```toml
[forge.release_history]
expected_route_additions = 2
max_total_decoded_growth_bytes = 0
max_total_brotli_growth_bytes = 0
max_route_decoded_growth_bytes = 0
max_route_brotli_growth_bytes = 0
max_added_route_decoded_bytes = 10000
max_added_route_brotli_bytes = 2000
```

The CI script also has an explicit public-route guard before it records the release-dashboard artifacts. The route comparison must include `/forge`, `/forge/scorecard`, `/forge/ci`, `/forge/evidence`, `/forge/releases`, `/forge/changelog`, and `/forge/adoption`; each required route must be measured, static, and budget-passing.

Expected-route payload is excluded from total-growth checks only when the added route count is within `expected_route_additions`; existing route growth, missing routes, failed route budgets, and dashboard failures still stay reviewable.

When the promoted history is ready, publish the release history through the static Forge releases route:

```powershell
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- prove vertical --fixture forge-releases --out public --write
```

Publish `/forge/releases/` only after `forge-public-release-history.json` and `.md` are promoted in the same review. If the JSON changes, regenerate the Markdown before publishing.

Publish the launch changelog through the static Forge changelog route from the same promoted release history:

```powershell
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- prove vertical --fixture forge-changelog --out public --write
```

Publish `/forge/changelog/` only when the matching `forge-public-launch-changelog.json` and `.md` are generated from that same reviewed release-history JSON.

For a repository using GitHub Pages, the public badge JSON will be available at:

```text
https://<owner>.github.io/<repo>/forge-readiness-badge.json
```

The public CI evidence route will be available at:

```text
https://<owner>.github.io/<repo>/forge/ci/
```

The public release-history route will be available at:

```text
https://<owner>.github.io/<repo>/forge/releases/
```

The public launch-changelog route will be available at:

```text
https://<owner>.github.io/<repo>/forge/changelog/
```

The public adoption-evidence route will be available at:

```text
https://<owner>.github.io/<repo>/forge/adoption/
```

Use the badge URL in a README through any Shields-compatible JSON endpoint renderer. Keep it opt-in until the repository URL and launch audience are decided.

## Artifact History Retention And Cleanup

Forge CI artifacts are release proof, but they are not source code. Keep the workflow outputs long enough for review and incident triage, then let GitHub Actions expire them.

Default workflow retention:

- `dx-forge-ci`: 30 days. This includes smoke JSON/Markdown, triage, evidence, scorecard, benchmark history, launch HTML, claims, evidence model, proof summary, and DXPK packet.
- `dx-forge-pages`: 14 days. This is only the publish preview for `forge-readiness-badge.json`, `/forge/ci/`, `/forge/releases/`, `/forge/changelog/`, and `/forge/adoption/`; a GitHub Pages deployment is the durable public copy when publishing is enabled.

For long-lived release notes, promote the small human-readable summaries into docs or benchmark reports intentionally. Do not commit `.dx/ci`, `.dx/forge-pages`, or ad hoc local smoke folders.

Local cleanup after a repeated run:

```powershell
Remove-Item -LiteralPath .\.dx\ci -Recurse -Force
Remove-Item -LiteralPath .\.dx\forge-pages -Recurse -Force
```

When investigating a failed run, keep `forge-triage.md`, `forge-readiness-badge.json`, and `forge-smoke.json` together. They form the minimal reproducible evidence packet for the release gate.

## Workflow Templates

Committed templates:

- `.github/workflows/forge-ci.yml` runs the secret-free Forge smoke, uploads `.dx/ci` evidence artifacts, uploads a `.dx/forge-pages` preview artifact, and can publish the badge plus `/forge/ci`, `/forge/releases`, `/forge/changelog`, and `/forge/adoption` routes to GitHub Pages from a manual `workflow_dispatch` run with `publish_pages=true`.
- `scripts/ci/forge-ci.ps1` is the generic PowerShell runner for local CI, self-hosted runners, and other providers.

To generate portable snippets for another repository or CI provider, run:

```powershell
dx forge ci-snippets --out .\.dx\forge-ci-snippets --artifact-dir .dx/ci --pages-dir .dx/forge-pages --format markdown --output .\.dx\forge-ci-snippets\README.md --fail-under 90
```

This writes:

- `.dx/forge-ci-snippets/github-actions/forge-ci.yml`
- `.dx/forge-ci-snippets/powershell/forge-ci.ps1`
- `.dx/forge-ci-snippets/generic/forge-ci.sh`
- `.dx/forge-ci-snippets/README.md`

Every snippet runs `scripts/ci/forge-ci.ps1`, then replays `dx forge release-triage` and `dx forge beta-artifact-verify` against the same `forge-release-operations.json`, `forge-publish-plan.json`, release bundle, Pages preview, and registry-smoke evidence.

The workflow intentionally avoids R2 credentials, the Codex browser plugin, and `cargo build --workspace`. GitHub Pages publishing uses the built-in GitHub Actions token and OIDC permissions, not private Cloudflare or Forge registry secrets.

## Launch Budget Lane

Use this lane on a machine with Node.js and Google Chrome available on the normal Windows install paths. It uses Chrome directly through the DevTools protocol; it does not require the Codex browser plugin or any in-app browser integration.

```powershell
$env:DX_VERTICAL_PROOF_MODE = "forge-site"
$env:DX_VERTICAL_BUDGET_GATE = "1"
node .\benchmarks\measure-vertical-proof.ts
node .\benchmarks\compare-forge-launch-delivery.ts
Remove-Item Env:\DX_VERTICAL_PROOF_MODE
Remove-Item Env:\DX_VERTICAL_BUDGET_GATE
```

The budget gate currently checks:

- decoded route bytes
- Brotli-estimated route bytes
- HTTP route median
- Chrome load-event median

Project-owned thresholds can live in `dx.config.toml`:

```toml
[forge.launch_budget]
profile = "compact-forge-launch"
max_decoded_bytes = 30000
max_brotli_bytes = 5500
max_http_route_median_ms = 5
max_chrome_load_event_ms = 75

[forge.launch_budget.scorecard]
profile = "compact-forge-scorecard"
max_decoded_bytes = 20000
max_brotli_bytes = 3500
max_http_route_median_ms = 5
max_chrome_load_event_ms = 75

[forge.launch_budget.ci]
profile = "compact-forge-ci"
max_decoded_bytes = 16000
max_brotli_bytes = 3000
max_http_route_median_ms = 5
max_chrome_load_event_ms = 75

[forge.launch_budget.evidence]
profile = "compact-forge-evidence"
max_decoded_bytes = 24000
max_brotli_bytes = 4000
max_http_route_median_ms = 5
max_chrome_load_event_ms = 75

[forge.launch_budget.releases]
profile = "compact-forge-releases"
max_decoded_bytes = 18000
max_brotli_bytes = 3000
max_http_route_median_ms = 5
max_chrome_load_event_ms = 75
```

The benchmark script reads `dx.config.toml` from the repository root by default. Set `DX_VERTICAL_BUDGET_CONFIG` to point at another config file. Existing environment overrides still win over config values:

- `DX_FORGE_SITE_MAX_DECODED_BYTES`
- `DX_FORGE_SITE_MAX_BROTLI_BYTES`
- `DX_FORGE_SITE_MAX_HTTP_MEDIAN_MS`
- `DX_FORGE_SITE_MAX_CHROME_LOAD_MS`
- `DX_FORGE_SCORECARD_MAX_DECODED_BYTES`
- `DX_FORGE_SCORECARD_MAX_BROTLI_BYTES`
- `DX_FORGE_SCORECARD_MAX_HTTP_MEDIAN_MS`
- `DX_FORGE_SCORECARD_MAX_CHROME_LOAD_MS`
- `DX_FORGE_CI_MAX_DECODED_BYTES`
- `DX_FORGE_CI_MAX_BROTLI_BYTES`
- `DX_FORGE_CI_MAX_HTTP_MEDIAN_MS`
- `DX_FORGE_CI_MAX_CHROME_LOAD_MS`
- `DX_FORGE_EVIDENCE_MAX_DECODED_BYTES`
- `DX_FORGE_EVIDENCE_MAX_BROTLI_BYTES`
- `DX_FORGE_EVIDENCE_MAX_HTTP_MEDIAN_MS`
- `DX_FORGE_EVIDENCE_MAX_CHROME_LOAD_MS`
- `DX_FORGE_RELEASES_MAX_DECODED_BYTES`
- `DX_FORGE_RELEASES_MAX_BROTLI_BYTES`
- `DX_FORGE_RELEASES_MAX_HTTP_MEDIAN_MS`
- `DX_FORGE_RELEASES_MAX_CHROME_LOAD_MS`

Chrome is optional for exploratory benchmark runs, but the enforced launch budget expects Chrome timing evidence.

## R2 And Secrets Policy

Do not set R2 credentials in the default CI smoke lane. Forge registry R2 commands are opt-in and should only run in a separate live-publish job.

For the default hosted-registry boundary check, use the dry-run smoke command. It joins local registry init, package integrity, R2 publish planning, R2 pull planning, and no-`node_modules` evidence without requiring private credentials:

```powershell
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- forge registry smoke --remote r2 --local .\.dx\forge-registry-smoke --format markdown --output .\.dx\ci\forge-registry-smoke.md --quiet
```

Default CI should not require:

- `CLOUDFLARE_R2_ACCOUNT_ID`
- `CLOUDFLARE_R2_BUCKET`
- `CLOUDFLARE_R2_ACCESS_KEY_ID`
- `CLOUDFLARE_R2_SECRET_ACCESS_KEY`
- `CLOUDFLARE_R2_PUBLIC_BASE_URL`
- `DX_FORGE_R2_LIVE`

Run live R2 smoke only in a protected job with explicit credentials and a reviewed publish target.

## Failure Triage

- If `cargo check -p dx-www --lib` fails, fix compile errors before running benchmark scripts.
- If Forge smoke fails on launch-page quality, inspect `.dx/forge/launch-smoke/` and `public/forge.claims.json`.
- If the budget gate fails on bytes, inspect `benchmarks/reports/vertical-proof-measurement.md` and `benchmarks/reports/vertical-proof-triage.md`.
- If the budget gate fails only on Chrome timing, re-run once on an idle runner before changing thresholds.
- If `node_modules` appears, treat it as a release blocker for the Forge smoke path.
