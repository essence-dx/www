# Forge Public Release History

Updated: 2026-05-16T18:56:27.813540700+00:00

## Latest

- Dashboard score: 93 / 100
- Dashboard passed: true
- Required score: 90 / 100
- Public routes: 5
- Total decoded bytes: 24192 B
- Total Brotli estimate: 5103 B
- Smallest Brotli route: `/forge/releases`

| Route | Fixture | Delivery | Decoded | Brotli | HTTP median | Chrome load | Budget |
| --- | --- | --- | ---: | ---: | ---: | ---: | --- |
| /forge | forge-site | static | 5143 B | 1240 B | 1.914 ms | 10.2 ms | yes |
| /forge/scorecard | forge-scorecard | static | 4189 B | 1040 B | 2.195 ms | 8.4 ms | yes |
| /forge/ci | forge-ci | static | 3309 B | 862 B | 1.767 ms | 8.9 ms | yes |
| /forge/evidence | forge-evidence | static | 6988 B | 1106 B | 0.939 ms | 8.3 ms | yes |
| /forge/releases | forge-releases | static | 4563 B | 855 B | 1.763 ms | 11.3 ms | yes |

## Latest Regression Checks

- Total decoded public payload grew from 19629 B to 24192 B.
- Total Brotli public payload grew from 4248 B to 5103 B.

## Records

| Recorded | Dashboard | Routes | Decoded | Brotli | Findings | Regressions |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 2026-05-16T18:56:27.813540700+00:00 | 93 | 5 | 24192 B | 5103 B | 0 | 2 |
| 2026-05-16T18:28:57.583123600+00:00 | 93 | 4 | 19629 B | 4248 B | 0 | 0 |

