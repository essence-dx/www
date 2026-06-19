# Forge Pages Bundle Shape

- Artifacts: `22`
- Checks: `16`
- Score: `100`
- Passed: `true`

## Artifacts

| Artifact | JSON | Passed |
| --- | --- | --- |
| `forge-readiness-badge.json` | `true` | `true` |
| `forge/ci.html` | `n/a` | `true` |
| `forge/ci/index.html` | `n/a` | `true` |
| `forge/ci.claims.json` | `true` | `true` |
| `forge/ci.dxp` | `n/a` | `true` |
| `forge/ci.proof.json` | `true` | `true` |
| `forge/releases.html` | `n/a` | `true` |
| `forge/releases/index.html` | `n/a` | `true` |
| `forge/releases.claims.json` | `true` | `true` |
| `forge/releases.dxp` | `n/a` | `true` |
| `forge/releases.proof.json` | `true` | `true` |
| `forge/changelog.html` | `n/a` | `true` |
| `forge/changelog/index.html` | `n/a` | `true` |
| `forge/changelog.claims.json` | `true` | `true` |
| `forge/changelog.dxp` | `n/a` | `true` |
| `forge/changelog.proof.json` | `true` | `true` |
| `forge/adoption.html` | `n/a` | `true` |
| `forge/adoption/index.html` | `n/a` | `true` |
| `forge/adoption.claims.json` | `true` | `true` |
| `forge/adoption.dxp` | `n/a` | `true` |
| `forge/adoption.proof.json` | `true` | `true` |
| `proof.json` | `true` | `true` |

## Checks

| Check | Artifacts | Passed | Message |
| --- | --- | --- | --- |
| `readiness badge` | `forge-readiness-badge.json` | `true` | badge reports passed and no node_modules |
| `/forge/ci claims` | `forge/ci.claims.json` | `true` | claims manifest targets /forge/ci and has reviewable statuses |
| `/forge/ci proof` | `forge/ci.proof.json` | `true` | proof summary targets /forge/ci and references HTML plus DXPK |
| `/forge/ci legacy proof` | `proof.json` | `true` | legacy proof.json still targets /forge/ci |
| `/forge/ci clean route` | `forge/ci.html, forge/ci/index.html` | `true` | clean-route index matches forge/ci.html |
| `/forge/releases claims` | `forge/releases.claims.json` | `true` | claims manifest targets /forge/releases and has reviewable statuses |
| `/forge/releases proof` | `forge/releases.proof.json` | `true` | proof summary targets /forge/releases and references HTML plus DXPK |
| `/forge/releases clean route` | `forge/releases.html, forge/releases/index.html` | `true` | clean-route index matches forge/releases.html |
| `/forge/changelog claims` | `forge/changelog.claims.json` | `true` | claims manifest targets /forge/changelog and has reviewable statuses |
| `/forge/changelog proof` | `forge/changelog.proof.json` | `true` | proof summary targets /forge/changelog and references HTML plus DXPK |
| `/forge/changelog clean route` | `forge/changelog.html, forge/changelog/index.html` | `true` | clean-route index matches forge/changelog.html |
| `/forge/adoption claims` | `forge/adoption.claims.json` | `true` | claims manifest targets /forge/adoption and has reviewable statuses |
| `/forge/adoption proof` | `forge/adoption.proof.json` | `true` | proof summary targets /forge/adoption and references HTML plus DXPK |
| `/forge/adoption clean route` | `forge/adoption.html, forge/adoption/index.html` | `true` | clean-route index matches forge/adoption.html |
| `publish bundle dependency boundary` | `node_modules` | `true` | no node_modules directory in publish bundle |
| `secret-free public bundle` | `forge-readiness-badge.json, forge/ci.html, forge/ci/index.html, forge/ci.claims.json, forge/ci.dxp, forge/ci.proof.json, forge/releases.html, forge/releases/index.html, forge/releases.claims.json, forge/releases.dxp, forge/releases.proof.json, forge/changelog.html, forge/changelog/index.html, forge/changelog.claims.json, forge/changelog.dxp, forge/changelog.proof.json, forge/adoption.html, forge/adoption/index.html, forge/adoption.claims.json, forge/adoption.dxp, forge/adoption.proof.json, proof.json` | `true` | no secret markers found |
