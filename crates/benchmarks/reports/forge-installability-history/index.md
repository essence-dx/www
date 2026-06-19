# DX Forge Installability Trend History

Generated: 2026-05-18T18:45:24.641Z
Snapshots: `1`
Latest trend: `baseline`

Negative deltas mean the newer snapshot is faster or smaller than the previous snapshot.

## Checks

| Check | Passed | Present | Expected |
| --- | --- | ---: | ---: |
| installability history | `true` | 1 | 1 |
| latest snapshot passed | `true` | 1 | 1 |
| no package installs across history | `true` | 1 | 1 |

## Snapshots

| Generated | Passed | Score | Install | Delta | Upgrade | Delta | Trend | Snapshot |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- |
| 2026-05-18T18:45:24.641Z | `true` | 100 | 2470 ms | n/a | 95 ms | n/a | `baseline` | `snapshots/2026-05-18T18-45-24-641Z.json` |

## Honest Scope

- Trend history compares local Forge installability snapshots only.
- Negative time and byte deltas mean the newer snapshot is smaller or faster than the previous snapshot.
- npm and shadcn rows remain static references; this history does not run package installs.
- Each history snapshot is copied into the history folder so release-to-release movement can be reviewed later.
