# DX Forge Installability Snapshot

Generated: 2026-05-18T18:45:24.641Z
Score: `100` / `100`
Passed: `true`

This is not a live npm/shadcn install benchmark; npm and shadcn rows are static references and no package installs are run.

## Scope

- Package installs run: `false`
- Live npm/shadcn benchmark: `false`
- Competitor builds run: `false`
- Created node_modules: `false`
- Safe public claim: Forge beta install and upgrade rows use local evidence artifacts; npm and shadcn rows are static references, not live install measurements.

## Checks

| Check | Passed | Present | Expected |
| --- | --- | ---: | ---: |
| no package installs | `true` | 1 | 1 |
| Forge installability evidence | `true` | 2 | 2 |
| artifact size evidence | `true` | 2 | 2 |
| static baseline scope | `true` | 2 | 2 |
| local reports readable | `true` | 2 | 2 |
| release bundle present | `true` | 1 | 1 |

## Rows

| ID | Operation | Kind | Time | Artifact Size | Package Install Ran | node_modules Required | Evidence |
| --- | --- | --- | ---: | ---: | --- | --- | --- |
| `dx-forge-beta-install` | beta-install | local-evidence | 2470 ms | 184496 B | `false` | `false` | benchmarks\reports\forge-package-update-rehearsal.json prepare_result.duration_ms |
| `dx-forge-beta-upgrade` | beta-upgrade | local-evidence | 95 ms | 36419 B | `false` | `false` | benchmarks\reports\forge-package-update-rehearsal.json green/yellow update.duration_ms |
| `npm-install-baseline` | npm install | static-reference | 15000 ms | 25000000 B | `false` | `true` | Install is intentionally skipped; this row is only a conservative reference point for reviewing Forge installability. |
| `shadcn-add-baseline` | npx shadcn add button | static-reference | 8000 ms | 4000000 B | `false` | `true` | Command is intentionally skipped; this row is not a live shadcn benchmark and creates no node_modules. |

## Comparisons

| Baseline | Install Time Delta | Install Artifact Delta | Upgrade Time Delta | Upgrade Artifact Delta |
| --- | ---: | ---: | ---: | ---: |
| `npm-install-baseline` | 12530 ms | 24815504 B | 14905 ms | 24963581 B |
| `shadcn-add-baseline` | 5530 ms | 3815504 B | 7905 ms | 3963581 B |

## Findings

- none

## Honest Scope

- This is an installability snapshot, not a live npm/shadcn install benchmark.
- The script never runs npm, npx, shadcn, cargo, or package manager install commands.
- Forge timing comes from local beta/adoption evidence reports when present; static baseline rows are review references only.
- Artifact size is based on local release-bundle and update-review evidence files, not downloaded remote packages.
- Broader framework or ecosystem replacement claims still require separate live benchmark suites.
