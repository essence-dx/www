# DX Forge Public Launch Handoff

This is the final human handoff before publishing the public Forge evidence bundle. It is intentionally narrower than the full launch checklist: use it after the command gates are green, when a reviewer needs to decide whether the public claims, changelog, bundle hashes, route budgets, and Pages artifacts are safe to publish.

Run commands from `G:\WWW`.

## Handoff Inputs

Prepare these folders and reports before starting review:

- `.dx\ci`
- `.dx\forge-pages`
- `.dx\forge-release-bundle`
- `benchmarks\reports\forge-public-route-comparison.json`
- `benchmarks\reports\forge-public-route-comparison.md`
- `benchmarks\reports\forge-public-release-history.json`
- `benchmarks\reports\forge-public-release-history.md`
- `benchmarks\reports\forge-public-launch-changelog.json`
- `benchmarks\reports\forge-public-launch-changelog.md`

The handoff is blocked if any input was generated from stale route comparison data, a failed release-dashboard gate, or an unreviewed `.dx\ci` folder.

## Command Gate

Run the bundle and review gates against the exact folders being handed off:

```powershell
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- forge release-bundle --project . --out .\.dx\forge-release-bundle --format markdown --fail-under 90
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- forge release-bundle --verify .\.dx\forge-release-bundle --format markdown --fail-under 90
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- forge release-review --project . --bundle .\.dx\forge-release-bundle --dashboard .\.dx\ci\forge-release-dashboard.json --history .\benchmarks\reports\forge-public-release-history.json --route-comparison .\benchmarks\reports\forge-public-route-comparison.json --format markdown --output .\.dx\ci\forge-release-review.md --fail-under 90
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- forge launch-copy-review --project . --route-comparison .\benchmarks\reports\forge-public-route-comparison.json --source-review .\benchmarks\reports\forge-source-owned-package-review.json --static-evidence .\benchmarks\reports\forge-static-competitor-evidence.json --format markdown --output .\.dx\ci\forge-launch-copy-review.md --fail-under 90
```

All four commands must pass. The first two prove the bundle can be assembled and re-verified. The third joins the release dashboard, bundle manifest, launch changelog, release history, route comparison, and human signoff checks into one review report. The fourth scans final public beta copy against the same evidence and blocks universal npm/framework replacement claims.

For an adoption-inclusive release candidate, refresh route comparison and release history with `/forge/adoption` first, then use the opt-in bundle flag:

```powershell
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- forge release-bundle --project . --out .\.dx\forge-release-bundle-adoption --include-adoption --format markdown --fail-under 90
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- forge release-bundle --verify .\.dx\forge-release-bundle-adoption --include-adoption --format markdown --fail-under 90
```

Do not replace the stable default six-route bundle with the adoption-inclusive bundle until the release reviewer explicitly promotes the adoption evidence lane.

## Review Matrix

| Area | Evidence | Reviewer decision |
| --- | --- | --- |
| Public claims | `docs\forge-launch-limitations.md`, `public\forge.claims.json`, `public\forge.scorecard.claims.json`, `public\forge.ci.claims.json`, `public\forge.evidence.claims.json`, `public\forge.releases.claims.json`, `public\forge.changelog.claims.json`, `public\forge.adoption.claims.json` | Approve only if every claim is backed by local Forge evidence and avoids universal npm replacement, live traffic, customer adoption, or impossible security guarantees. |
| Launch copy review | `.dx\ci\forge-launch-copy-review.md`, `docs\forge-public-beta-quickstart.md`, `docs\forge-public-launch-handoff.md`, `benchmarks\reports\forge-static-competitor-evidence.md` | Approve only if the automated copy gate passes, source-owned security and static/no-runtime performance highlights remain backed by evidence, and blocked replacement/security claims are absent. |
| Launch changelog | `benchmarks\reports\forge-public-launch-changelog.json`, `benchmarks\reports\forge-public-launch-changelog.md`, `.dx\forge-release-bundle\forge-public-launch-changelog.json`, `.dx\forge-release-bundle\forge-public-launch-changelog.md` | Approve only if the changelog was generated from the same promoted release-history JSON, reports the current promoted public route set, keeps honest scope limits, and has no unresolved regression findings. |
| Bundle manifest hashes | `.dx\forge-release-bundle\forge-release-manifest.json`, `.dx\forge-release-bundle\forge-release-manifest.md` | Approve only if the manifest uses `hash_algorithm = blake3`, `integrity.scheme = dx-forge-release-manifest-v1`, lists every public artifact, and `dx forge release-bundle --verify` recomputes every file hash without findings. Adoption-inclusive bundles must also list `forge-adoption-report.json` and `/forge/adoption` route artifacts. |
| Route budgets | `benchmarks\reports\forge-public-route-comparison.json`, `benchmarks\reports\forge-public-route-comparison.md` | Approve only if `/forge`, `/forge/scorecard`, `/forge/ci`, `/forge/evidence`, `/forge/releases`, `/forge/changelog`, and `/forge/adoption` are measured, static, budget-passing routes. |
| Competitor evidence | `benchmarks\reports\forge-static-competitor-evidence.json`, `benchmarks\reports\forge-static-competitor-evidence.md` | Approve only if the report says it is a static-floor fixture, not a full framework benchmark, includes Astro, Svelte, HTMX, and Next.js static floors, and does not claim broad framework or npm replacement. |
| Pages artifacts | `.dx\forge-pages\forge-readiness-badge.json`, `.dx\forge-pages\forge\ci\index.html`, `.dx\forge-pages\forge\releases\index.html`, `.dx\forge-pages\forge\changelog\index.html`, `.dx\forge-pages\forge\adoption\index.html`, route `.dxp`, `.claims.json`, and `.proof.json` files | Approve only if `dx forge ci --verify-pages .\.dx\forge-pages --fail-under 90` passes and the clean-route indexes match the route HTML files. |
| Dependency boundary | `.dx\ci`, `.dx\forge-pages`, `.dx\forge-release-bundle`, temp projects used by smoke tests | Approve only if the launch path creates no `node_modules` folder and public artifacts contain no `CLOUDFLARE_R2_`, `DX_FORGE_R2_LIVE`, `R2_SECRET`, or `SECRET_ACCESS_KEY` markers. |

## Claim Review Rules

Allowed public claims:

- DX Forge materializes curated source-owned packages into editable files.
- DX Forge records receipts, source manifests, rollback evidence, package docs, and scorecards.
- DX Forge blocks install-time package scripts on the Forge add/update path.
- DX Forge verifies public release bundles with BLAKE3 artifact hashes.
- DX Forge currently proves a seven-route static public evidence surface with no runtime dependency and no `node_modules`.

Blocked public claims:

- Forge replaces npm, cargo, pip, or all package managers today.
- Forge prevents every software supply-chain attack.
- Forge proves real customer traffic, production adoption, or enterprise migration success.
- Forge beats every frontend framework in every scenario.
- Forge makes third-party code safe after arbitrary local edits.

If marketing copy needs a stronger claim than the allowed list, add evidence first and update `docs\forge-launch-limitations.md` in the same review.

## Competitor Evidence Review

Refresh the static-floor competitor evidence before public launch copy is approved:

```powershell
node .\benchmarks\compare-forge-static-competitors.ts
```

Review `benchmarks\reports\forge-static-competitor-evidence.md` and confirm it states:

- The report is not a full framework benchmark.
- Competitor builds are not run.
- Package installs are not run.
- No `node_modules` is created.
- It does not prove broad framework replacement.
- Static Astro, Svelte, HTMX, or Next.js floors may be smaller when they omit Forge evidence, claims, receipts, proof metadata, and review copy.

Use this report to keep public claims sober. Use `benchmarks\reports\real-route-comparison.md` only for broader real-route comparisons, and do not mix the two scopes in launch copy.

## Bundle Manifest Hash Review

Open `.dx\forge-release-bundle\forge-release-manifest.json` and verify these fields:

- `version` is `1`.
- `hash_algorithm` is `blake3`.
- `integrity.scheme` is `dx-forge-release-manifest-v1`.
- `integrity.signed` is `false` for the default local bundle, or `true` only when a signed manifest also carries `publisher_identity.status = signed`.
- `integrity.digest` is present.
- Signed manifests include `publisher_identity.algorithm = ed25519`, an `ed25519:` public key, an `ed25519-blake3:` key id derived from that public key, `signed_at`, and matching publisher/integrity signatures.
- Every artifact row has `path`, `artifact_type`, `bytes`, and `blake3`.

The default manifest is review integrity, not publisher identity. It proves local bundle contents did not change between assembly and verification. A manifest may be presented as cryptographic publisher identity only after `dx forge release-bundle --verify` accepts the Ed25519 signed publisher identity for the exact bundle being promoted.

## Changelog Review

Open `benchmarks\reports\forge-public-launch-changelog.md` and the matching JSON. Confirm:

- The latest dashboard score is at least the release threshold.
- `route_count` covers the seven-route public comparison surface.
- Added, removed, and changed routes are explained by the release history.
- `honest_scope` keeps at least the current limitations from `docs\forge-launch-limitations.md`.
- There are no unresolved findings or payload regressions.

If the changelog JSON changes, regenerate the Markdown before handoff so humans and automation review the same claims.

## Route Budget Review

Open `benchmarks\reports\forge-public-route-comparison.md` and confirm all required routes are present:

- `/forge`
- `/forge/scorecard`
- `/forge/ci`
- `/forge/evidence`
- `/forge/releases`
- `/forge/changelog`

Each route must be static, budget-passing, and backed by matching JSON. If a route was intentionally added or payload size changed, record the allowance in `dx.config.toml` under `[forge.release_history]`; do not silently accept unexplained growth.

## Pages Artifact Review

Verify the Pages preview before publishing:

```powershell
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- forge ci --verify-pages .\.dx\forge-pages --fail-under 90
```

Review these files as the minimum public preview surface:

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

The handoff fails if any clean-route `index.html` is missing, any DXPK packet is missing the `DXPK` header, any claims manifest points at the wrong route, or any proof file is invalid JSON.

## Final Signoff

Record the review outcome in the release notes or PR description:

| Check | Result | Evidence |
| --- | --- | --- |
| Release bundle verified | pass/fail | `.dx\forge-release-bundle\forge-release-manifest.json` |
| Release review passed | pass/fail | `.dx\ci\forge-release-review.md` |
| Launch copy review passed | pass/fail | `.dx\ci\forge-launch-copy-review.md` |
| Launch changelog reviewed | pass/fail | `benchmarks\reports\forge-public-launch-changelog.md` |
| Six route budgets passed | pass/fail | `benchmarks\reports\forge-public-route-comparison.md` |
| Pages preview verified | pass/fail | `.dx\forge-pages` |
| No secrets or `node_modules` | pass/fail | `dx forge release-bundle --verify` and `dx forge ci --verify-pages` |

Publish only when every row is `pass`.
