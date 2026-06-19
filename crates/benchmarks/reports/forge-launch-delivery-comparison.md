# Forge Launch Delivery Comparison

Generated: 2026-05-16T16:18:39.177Z
History source: `vertical-proof-history/index.json`

| Mode | Fixture | Generated | Delivery | Runtime asset | Resources | Scripts | Packages | Files | Decoded | Brotli | HTTP median | Chrome load | DXPK | Interaction | Evidence |
| --- | --- | --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- | --- | --- |
| static-no-runtime | forge-site | 2026-05-16T16:16:41.432Z | static | no | 1 | 0 | 2 | 7 | 5140 B | 1240 B | 1.866 ms | 38.3 ms | no | no | [md](vertical-proof-history/20260516T161641Z-forge-site.md) |
| dxpk-runtime | forge-site | 2026-05-16T13:44:12.675Z | dxpk-runtime | n/a | 3 | 2 | 2 | 7 | 27722 B | 4786 B | 1.671 ms | 38.4 ms | yes | no | [md](vertical-proof-history/20260516T134412Z-forge-site.md) |
| package-vertical | forge-combo | 2026-05-16T09:31:29.807Z | dxpk-runtime | n/a | 3 | 2 | 2 | 7 | 10867 B | 3105 B | 2.139 ms | 17 ms | yes | yes | [md](vertical-proof-history/20260516T093129Z-forge-combo.md) |

## Deltas To Static / No Runtime

| Comparison | Decoded | Decoded % | Brotli | Brotli % | HTTP median | Chrome load |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| dxpk-runtime to static-no-runtime | -22582 B | -81.5% | -3546 B | -74.1% | +0.195 ms | -0.1 ms |
| package-vertical to static-no-runtime | -5727 B | -52.7% | -1865 B | -60.1% | -0.273 ms | +21.3 ms |

## Notes

- static-no-runtime: Current public /forge route. Ships crawlable HTML and keeps DXPK as a proof artifact without a browser runtime asset.
- dxpk-runtime: Historical /forge route with DXPK browser runtime applied. Kept as a regression baseline for the static/no-runtime planner.
- package-vertical: Interactive source-owned package route. This is not the public /forge page, but it proves the runtime path still works when interaction exists.

The public /forge route should stay static/no-runtime while it has no interaction. DXPK runtime remains justified for interactive package verticals, and the report keeps the historical runtime baseline visible so payload regressions are obvious.
