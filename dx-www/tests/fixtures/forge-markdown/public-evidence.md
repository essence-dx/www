# DX Forge Public Evidence

- Route: `/forge/evidence`
- Generated: `<generated-at>`
- Score: `100` / `100`
- Packages: `29`
- Verified packages: `29`
- Source-owned packages: `29`
- `node_modules` packages: `0`
- Public artifacts: `9`

| Section | Artifact | Model | Description |
| --- | --- | --- | --- |
| Routes | [`forge.html`](forge.html) | `DxForgeReleaseEvidenceReport` | The compact public /forge route with launch claims, package cards, and budget evidence. |
| Routes | [`forge/scorecard.html`](forge/scorecard.html) | `DxForgePackageScorecardReport` | The public /forge/scorecard route rendered from the package scorecard model. |
| Routes | [`forge/ci.html`](forge/ci.html) | `DxForgeSmokeReport + DxForgeReadinessBadge` | The public /forge/ci route rendered from secret-free CI smoke and readiness evidence. |
| Badges | [`forge-readiness-badge.json`](forge-readiness-badge.json) | `DxForgeReadinessBadge` | The compact release-readiness badge consumed by CI summaries and public status pages. |
| Claims | [`forge.claims.json`](forge.claims.json) | `DxForgeLaunchClaimsManifest` | The machine-readable claim map for the public /forge route. |
| Claims | [`forge/evidence.claims.json`](forge/evidence.claims.json) | `DxForgeLaunchClaimsManifest` | The machine-readable claim map for this /forge/evidence index. |
| Evidence | [`forge.evidence.json`](forge.evidence.json) | `DxForgeLaunchEvidenceManifest` | The package, provenance, advisory, license, and benchmark evidence backing /forge. |
| Benchmarks | [`forge-public-route-comparison.md`](forge-public-route-comparison.md) | `benchmarks/reports/forge-public-route-comparison.md` | The current compact-route comparison for /forge, /forge/scorecard, /forge/ci, /forge/evidence, and /forge/releases. |
| Benchmarks | [`forge-launch-delivery-comparison.md`](forge-launch-delivery-comparison.md) | `benchmarks/reports/forge-launch-delivery-comparison.md` | The static /forge delivery comparison against the earlier DXPK runtime delivery. |
