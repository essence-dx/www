# DX Forge Public Launch Checklist

This is the release operator checklist for publishing DX Forge evidence without changing the default local/offline safety model. It ties the secret-free CI lane, public Pages bundle, release notes, route benchmarks, and release dashboard into one reviewable flow.

Run commands from `G:\WWW`.

Use `docs\forge-public-launch-handoff.md` as the final human review packet after these command gates pass. The handoff doc covers claim review, launch changelog review, release-bundle BLAKE3 manifest hashes, route budgets, Pages artifacts, secret-marker checks, and the no-`node_modules` boundary.

Use `docs\forge-real-project-adoption.md` when a developer needs the shorter clean-project adoption path with exact local commands, expected artifacts, strict Forge checks, release-bundle verification, release-readiness trend evidence, and honest non-replacement claims.

Use `docs\forge-adoption-launch-checklist.md` when the release reviewer needs the adoption-first launch flow that joins the clean app, `dx forge ci`, adoption report, Pages preview, and benchmark evidence.

Use `docs\forge-public-beta-quickstart.md` when a beta developer needs the shortest public onboarding route from `dx forge init-app` to strict Forge checks, source-owned review evidence, and no-`node_modules` proof.

## Release Goal

The public launch is ready only when:

- Forge CI artifacts pass.
- The GitHub Pages publish bundle passes before deploy.
- Release notes summarize the current scorecard, route measurements, and honest limitations.
- Public evidence links resolve to expected artifacts.
- Public route comparison covers `/forge`, `/forge/scorecard`, `/forge/ci`, `/forge/evidence`, `/forge/releases`, `/forge/changelog`, and `/forge/adoption`.
- `dx forge release-dashboard` passes at the configured threshold.
- `dx forge release-candidate` passes after joining dashboard-era CI artifacts, Pages verification, route comparison, source-owned review, static competitor evidence, secret-marker scans, and no-`node_modules` proof.
- `dx forge release-operations` passes against the signed release manifest, trust-regression report, release-candidate report, CI artifacts, and public-evidence export.
- `dx forge publish-plan` passes against the release bundle, Pages preview, dry-run registry smoke, release-operations report, cache policy, rollback inputs, no-secret checks, and no-`node_modules` proof.
- `dx forge release-triage` is available for any failed release-operations or publish-plan report and groups action into missing artifacts, secret risk, cache policy, rollback readiness, and dependency boundary.
- `dx forge ci-snippets` can generate portable GitHub Actions, PowerShell, and generic runner snippets that replay the same beta promotion evidence path.
- `dx forge beta-artifact-verify` passes against the already downloaded release bundle, Pages preview, and R2 registry smoke evidence without rebuilding locally.
- `benchmarks\reports\forge-installability-snapshot.md` compares Forge beta install/upgrade evidence with skipped npm and shadcn static-reference rows without running package installs.
- No `node_modules` folder is created by the Forge launch path.
- No `CLOUDFLARE_R2_` or other secret environment markers appear in public artifacts.

## 1. Preflight

Start clean and make sure you are launching the checked-in state:

```powershell
git status --short
cargo fmt --manifest-path .\www\Cargo.toml -p dx-www -- --check
cargo check --manifest-path .\www\Cargo.toml -p dx-www --lib
```

Do not continue if there are unrelated dirty files or compile failures.

## 2. Generate CI And Pages Evidence

Use the secret-free Forge CI runner. This produces the review bundle and a Pages publish preview:

```powershell
.\scripts\ci\forge-ci.ps1 -ArtifactDir .\.dx\ci -PagesDir .\.dx\forge-pages -FailUnder 90
```

Expected primary artifacts:

- `.dx\ci\forge-smoke.json`
- `.dx\ci\forge-smoke.md`
- `.dx\ci\forge-triage.md`
- `.dx\ci\forge-readiness-badge.json`
- `.dx\ci\forge-evidence.json`
- `.dx\ci\forge-scorecard.json`
- `.dx\ci\forge-benchmark-history.json`
- `.dx\forge-pages\forge-readiness-badge.json`
- `.dx\forge-pages\forge\ci.html`
- `.dx\forge-pages\forge\ci\index.html`
- `.dx\forge-pages\forge\ci.claims.json`
- `.dx\forge-pages\forge\ci.dxp`
- `.dx\forge-pages\forge\ci.proof.json`
- `.dx\forge-pages\forge\releases.html`
- `.dx\forge-pages\forge\releases\index.html`
- `.dx\forge-pages\forge\releases.claims.json`
- `.dx\forge-pages\forge\releases.dxp`
- `.dx\forge-pages\forge\releases.proof.json`
- `.dx\forge-pages\forge\changelog.html`
- `.dx\forge-pages\forge\changelog\index.html`
- `.dx\forge-pages\forge\changelog.claims.json`
- `.dx\forge-pages\forge\changelog.dxp`
- `.dx\forge-pages\forge\changelog.proof.json`
- `.dx\ci\forge-release-bundle-adoption\forge-release-manifest.json`
- `.dx\ci\forge-manifest-signing.json`
- `.dx\ci\forge-registry-smoke.json`
- `.dx\ci\forge-release-operations.json`
- `.dx\ci\forge-release-operations.md`
- `.dx\ci\forge-publish-plan.json`
- `.dx\ci\forge-publish-plan.md`

When assembling a full `dx forge release-bundle`, the default bundle must include `forge-public-launch-changelog.json`, `forge-public-launch-changelog.md`, and the same static `/forge/changelog` route artifacts generated from the promoted release history. Keep this default path stable for existing operators. Use `--include-adoption` only after the promoted route comparison and release history include `/forge/adoption`; that opt-in bundle also carries the adoption smoke/report/page artifacts plus `/forge/adoption` HTML, claims, DXPK, and proof files.

## 3. Verify Existing Artifacts

Re-run the verification commands against the generated folders. These commands must reuse existing evidence and avoid regenerating or publishing anything:

```powershell
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- forge ci --verify-artifacts .\.dx\ci --fail-under 90
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- forge ci --verify-pages .\.dx\forge-pages --fail-under 90
```

The command strings above intentionally include:

- `dx-www -- forge ci --verify-artifacts .\.dx\ci --fail-under 90`
- `dx-www -- forge ci --verify-pages .\.dx\forge-pages --fail-under 90`

## 4. Generate Public Release Notes

Write release notes from the same benchmark history that CI produced:

```powershell
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- forge release-notes --project . --history .\.dx\ci\forge-benchmark-history.json --format markdown --output .\.dx\ci\forge-release-notes.md --fail-under 90
```

The command string above intentionally includes `dx-www -- forge release-notes --project .`.

Review `.dx\ci\forge-release-notes.md` for:

- readiness score
- package scorecard status
- latest `/forge` route measurement
- honest launch limitations
- absence of secret markers

## 5. Export Public Evidence Map

Export the same evidence map used by `/forge/evidence`:

```powershell
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- forge public-evidence --project . --format markdown --output .\.dx\ci\forge-public-evidence.md
```

The command string above intentionally includes `dx-www -- forge public-evidence --project .`.

Review `.dx\ci\forge-public-evidence.md` and confirm it links the launch page, scorecard, CI evidence, `forge-readiness-badge.json`, claims manifests, evidence models, and benchmark comparisons.

When the public evidence directory already exists, verify it without regenerating routes:

```powershell
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- forge public-evidence --verify public --format markdown --fail-under 90
```

This verifier checks the exported `/forge/evidence` route, every linked public artifact, the readiness badge, benchmark comparison files, secret-marker hygiene, and the no-`node_modules` boundary.

## 6. Refresh Public Route Benchmarks

Run this lane on a machine with Node.js and Chrome available. It updates the public benchmark reports that the dashboard checks:

```powershell
$env:DX_VERTICAL_BUDGET_GATE = "1"
node .\benchmarks\measure-vertical-proof.ts
node .\benchmarks\compare-forge-launch-delivery.ts
Remove-Item Env:\DX_VERTICAL_BUDGET_GATE
```

The command strings above intentionally include:

- `benchmarks\measure-vertical-proof.ts`
- `benchmarks\compare-forge-launch-delivery.ts`

Review:

- `benchmarks\reports\forge-public-route-comparison.json`
- `benchmarks\reports\forge-public-route-comparison.md`
- `benchmarks\reports\forge-launch-delivery-comparison.json`
- `benchmarks\reports\forge-launch-delivery-comparison.md`

The route comparison must include all seven public routes and keep them static/no-runtime unless a later TODO explicitly changes the route strategy. The CI script fails before release-dashboard recording if the comparison loses `/forge/changelog` or `/forge/adoption`, drops below the required route count, marks a required route non-static, or records a failed route budget.

For framework comparison claims, keep deterministic static-floor fixtures and live framework builds separated:

```powershell
node .\benchmarks\compare-forge-static-competitors.ts
node .\benchmarks\compare-forge-live-frameworks.ts
```

Set `DX_FORGE_LIVE_FRAMEWORKS=1` only on a machine with already-installed Astro, Svelte, HTMX, or Next.js baseline projects. The live harness must skip missing `node_modules` instead of installing packages, and public copy must label live-build rows separately from static-floor rows.

## 7. Run The Release Dashboard Gate

Run the one-command release gate after the artifacts, Pages preview, notes, and route comparison exist:

```powershell
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- forge release-dashboard --project . --ci-artifacts .\.dx\ci --pages .\.dx\forge-pages --history .\.dx\ci\forge-benchmark-history.json --route-comparison .\benchmarks\reports\forge-public-route-comparison.json --format markdown --output .\.dx\ci\forge-release-dashboard.md --fail-under 90
```

The command string above intentionally includes `dx-www -- forge release-dashboard --project .`.

Review `forge-release-dashboard.md` before publishing. It should show passing checks for:

- CI artifacts
- Pages bundle
- release notes
- public evidence
- route comparison

Also write the machine-readable dashboard and append the dashboard plus route-comparison result to benchmark history:

```powershell
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- forge release-dashboard --project . --ci-artifacts .\.dx\ci --pages .\.dx\forge-pages --history .\.dx\ci\forge-benchmark-history.json --route-comparison .\benchmarks\reports\forge-public-route-comparison.json --format json --output .\.dx\ci\forge-release-dashboard.json --fail-under 90 --quiet
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- forge release-history --dashboard .\.dx\ci\forge-release-dashboard.json --route-comparison .\benchmarks\reports\forge-public-route-comparison.json --output .\benchmarks\reports\forge-public-release-history.json --format markdown --quiet
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- forge launch-changelog --history .\benchmarks\reports\forge-public-release-history.json --format markdown --output .\benchmarks\reports\forge-public-launch-changelog.md --fail-under 90 --quiet
```

The command strings above intentionally include `dx-www -- forge release-history --dashboard` and `dx-www -- forge launch-changelog --history`.

Review `benchmarks\reports\forge-public-release-history.md` before publishing. It should show the dashboard score, the current public route measurements, total payload, and no release-dashboard findings.

Release-history regression checks are strict unless `dx.config.toml` opts into reviewed allowances under `[forge.release_history]`. Use `expected_route_additions` plus `max_added_route_*_bytes` for intentional new public routes; use `max_total_*_growth_bytes` and `max_route_*_growth_bytes` only for explicitly approved payload drift. Missing routes, failed route budgets, and dashboard failures still remain release findings.

Review `benchmarks\reports\forge-public-launch-changelog.md` before publishing public copy. It is generated from release-history records only and should not claim live traffic, customer adoption, or universal npm replacement coverage.

Run the release-candidate gate when the source-owned package review and static competitor evidence reports are present:

```powershell
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- forge release-candidate --project . --ci-artifacts .\.dx\ci --pages .\.dx\forge-pages --route-comparison .\benchmarks\reports\forge-public-route-comparison.json --source-review .\benchmarks\reports\forge-source-owned-package-review.json --static-evidence .\benchmarks\reports\forge-static-competitor-evidence.json --format markdown --output .\.dx\ci\forge-release-candidate.md --fail-under 90
```

The command string above intentionally includes `dx-www -- forge release-candidate --project .`.

Review `forge-release-candidate.md` before publishing. It should show passing checks for CI artifacts, Pages bundle, route comparison, source-owned review, static competitor evidence, secret markers, and no `node_modules`.

The CI runner now writes the final hosted-beta shipping reports in the same artifact bundle. Review these before publishing:

```powershell
dx forge release-bundle-inspect --bundle .\.dx\ci\forge-release-bundle-adoption --format markdown --output .\.dx\ci\forge-release-bundle-inspect.md --fail-under 90
dx forge release-triage --release-operations .\.dx\ci\forge-release-operations.json --publish-plan .\.dx\ci\forge-publish-plan.json --format markdown --output .\.dx\ci\forge-release-triage.md --fail-under 90
dx forge beta-artifact-verify --release-bundle .\.dx\ci\forge-release-bundle-adoption --pages .\.dx\forge-pages --registry-smoke .\.dx\ci\forge-registry-smoke.json --format markdown --output .\.dx\ci\forge-beta-artifact-verify.md --fail-under 90
node .\benchmarks\measure-forge-installability-snapshot.ts
Get-Content .\.dx\ci\forge-release-operations.md
Get-Content .\.dx\ci\forge-publish-plan.md
Get-Content .\.dx\ci\forge-release-triage.md
Get-Content .\.dx\ci\forge-beta-artifact-verify.md
Get-Content .\benchmarks\reports\forge-installability-snapshot.md
```

`forge-release-bundle-inspect.md` should report `ready-for-beta-review`; `forge-release-operations.md` should report `ready-to-ship`; `forge-publish-plan.md` should report `ready-to-publish-plan`; `forge-release-triage.md` should either show no immediate actions or group failures by operator action; `forge-beta-artifact-verify.md` should report `ready-for-downloaded-beta-install` with `requires_rebuild=false`; `forge-installability-snapshot.md` should report score `100`, local Forge install/upgrade evidence, static npm/shadcn reference rows, and no package installs. The corresponding JSON files are the automation contract for deployment gates, rollback review, and beta installability review.

Before publishing, complete the final handoff in `docs\forge-public-launch-handoff.md` against the exact `.dx\ci`, `.dx\forge-pages`, `.dx\ci\forge-release-bundle-adoption`, and `benchmarks\reports` artifacts being promoted.

## 8. Preserve And Publish Release History

Release history is the durable launch-review trail. Treat it as a promoted evidence artifact, not as a temporary CI log.

Preserve these files for every public launch review:

- `.dx\ci\forge-public-release-history.json`
- `.dx\ci\forge-public-release-history.md`
- `.dx\ci\forge-release-dashboard.json`
- `.dx\ci\forge-release-dashboard.md`
- `.dx\ci\forge-release-candidate.md`
- `.dx\ci\forge-public-route-comparison.json`
- `benchmarks\reports\forge-public-release-history.json`
- `benchmarks\reports\forge-public-release-history.md`
- `benchmarks\reports\forge-public-launch-changelog.md`
- `benchmarks\reports\forge-public-route-comparison.json`
- `benchmarks\reports\forge-public-route-comparison.md`

The `.dx\ci` copies are short-lived workflow artifacts. The `benchmarks\reports` copies are the reviewable source-controlled launch record after the operator intentionally promotes them.

Before announcing a release, generate or refresh the public release route from the promoted history:

```powershell
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- prove vertical --fixture forge-releases --out public --write
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- prove vertical --fixture forge-changelog --out public --write
```

The command string above intentionally includes `dx-www -- prove vertical --fixture forge-releases`.
It also intentionally includes `dx-www -- prove vertical --fixture forge-changelog`.

Publish or preview `/forge/releases/` only after `forge-public-release-history.json` and `.md` are promoted together. Do not publish a release-history page from stale route-comparison data, a failed dashboard gate, or an ad hoc `.dx\ci` folder that has not been reviewed.
Publish or preview `/forge/changelog/` only after the matching `forge-public-launch-changelog.json` and `.md` are generated from that same promoted release-history JSON.

When the public launch review is approved, keep the human Markdown and the machine JSON together. If the JSON changes, regenerate the Markdown in the same review so public claims, route totals, and regression findings stay paired.

## 9. Publish Decision

Publish only when all of these are true:

- `forge-release-dashboard.md` passes.
- `forge-release-candidate.md` passes.
- `forge-release-dashboard.json` is recorded in `benchmarks\reports\forge-public-release-history.json`.
- `forge-readiness-badge.json` reports a passing status.
- `forge-public-route-comparison.json` includes seven passing public routes, including `/forge/changelog` and `/forge/adoption`.
- `.dx\forge-pages` verifies with `dx forge ci --verify-pages`.
- The public artifact scan shows no `CLOUDFLARE_R2_`, `DX_FORGE_R2_LIVE`, `R2_SECRET`, or `SECRET_ACCESS_KEY`.
- `forge-installability-snapshot.md` passed without running npm, npx, shadcn, or package installs.
- The release path did not create `node_modules`.

If a gate fails, keep these files together for triage:

- `.dx\ci\forge-triage.md`
- `.dx\ci\forge-smoke.json`
- `.dx\ci\forge-readiness-badge.json`
- `.dx\ci\forge-release-dashboard.md`
- `.dx\ci\forge-release-dashboard.json`
- `.dx\ci\forge-release-candidate.md`
- `.dx\ci\forge-public-release-history.md`
- `.dx\ci\forge-public-release-history.json`

## 10. Cleanup

Do not commit `.dx\ci` or `.dx\forge-pages`. After review:

```powershell
Remove-Item -LiteralPath .\.dx\ci -Recurse -Force
Remove-Item -LiteralPath .\.dx\forge-pages -Recurse -Force
```

Keep promoted release summaries in `docs/` or `benchmarks/reports/` only when they are intentionally part of the public launch record.
