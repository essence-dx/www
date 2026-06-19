# Forge Adoption Browser Smoke

Generated: 2026-05-17T07:27:17.685Z
Project: `G:\WWW\dx-www-binary-web\.dx\adoption-browser-smoke`
Score: `100` / `100`
Passed: `true`
No node_modules: `true`

## Routes

| Route | Passed | Static/no-runtime | H1 | Links | Scripts | Artifacts | HTTP median | Browser load |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| /forge | true | true | 1 | 0 | 0 | 4/4 | 11.146 ms | 24.7 ms |
| /forge/scorecard | true | true | 1 | 0 | 0 | 2/2 | 5.143 ms | 13.2 ms |
| /forge/ci | true | true | 1 | 0 | 0 | 3/3 | 3.693 ms | 18 ms |
| /forge/evidence | true | true | 1 | 9 | 0 | 3/3 | 7.265 ms | 14 ms |
| /forge/releases | true | true | 1 | 0 | 0 | 3/3 | 4.681 ms | 33.7 ms |
| /forge/changelog | true | true | 1 | 0 | 0 | 3/3 | 2.839 ms | 20.3 ms |

## Findings

- none

## Honest Scope

- This is local browser smoke evidence for the generated Forge adoption app routes.
- It does not claim live customer traffic, CDN performance, or broad framework replacement.
- The harness never runs package installs and treats node_modules as a release-risk finding.
- Chrome timing is collected only when a local Chrome or Edge executable is available.
