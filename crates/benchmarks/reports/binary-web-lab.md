# Binary Web Lab

Local experiment comparing HTML, JSON, rkyv, dictionary packets, and template/data packets for DX-WWW-like page graphs.

| Scenario | Encoding | Raw | gzip | Brotli | Access median |
| --- | --- | ---: | ---: | ---: | ---: |
| tiny-counter | html-string | 427 | 243 | 146 | 175 ns |
| tiny-counter | json-serde | 966 | 293 | 235 | 7490 ns |
| tiny-counter | rkyv-zero-copy | 608 | 310 | 246 | 0 ns |
| tiny-counter | dx-dict-packet | 290 | 236 | 176 | 150 ns |
| tiny-counter | dx-template-data | 43 | 57 | 47 | 15 ns |
| docs-160-sections | html-string | 36349 | 786 | 463 | 17420 ns |
| docs-160-sections | json-serde | 62677 | 1735 | 999 | 440310 ns |
| docs-160-sections | rkyv-zero-copy | 47056 | 5034 | 2708 | 0 ns |
| docs-160-sections | dx-dict-packet | 8213 | 1588 | 1014 | 5190 ns |
| docs-160-sections | dx-template-data | 21991 | 620 | 391 | 930 ns |
| marketing-180-cards | html-string | 40617 | 876 | 527 | 19335 ns |
| marketing-180-cards | json-serde | 80790 | 2295 | 1401 | 864595 ns |
| marketing-180-cards | rkyv-zero-copy | 59284 | 7867 | 3452 | 0 ns |
| marketing-180-cards | dx-dict-packet | 7173 | 2174 | 1262 | 7960 ns |
| marketing-180-cards | dx-template-data | 18271 | 597 | 433 | 1505 ns |
| dashboard-1200-rows | html-string | 203444 | 7876 | 3912 | 93285 ns |
| dashboard-1200-rows | json-serde | 547068 | 20033 | 10870 | 7102790 ns |
| dashboard-1200-rows | rkyv-zero-copy | 339048 | 62021 | 35062 | 0 ns |
| dashboard-1200-rows | dx-dict-packet | 68827 | 22937 | 6384 | 77985 ns |
| dashboard-1200-rows | dx-template-data | 43628 | 6256 | 2675 | 13280 ns |

## Mmap

- Scenario: dashboard-1200-rows
- Packet bytes: 43628
- read_to_vec median: 103500 ns
- mmap median: 94700 ns

## Breakthrough Experiments

| Experiment | Strategy | Raw | gzip | Brotli | Access median |
| --- | --- | ---: | ---: | ---: | ---: |
| dashboard-full-data | dx-template-slots | 43628 | 6256 | 2675 | 13415 ns |
| dashboard-full-data | dx-columnar-slots | 21655 | 6175 | 3544 | 7210 ns |
| dashboard-full-data | dx-semantic-codec | 22 | 42 | 26 | 10 ns |
| initial-viewport | dx-viewport-40 | 638 | 267 | 199 | 182 ns |
| 12-row-live-update | html-row-fragments | 1187 | 187 | 147 | 457 ns |
| 12-row-live-update | json-cell-patch | 1152 | 209 | 163 | 9805 ns |
| 12-row-live-update | dx-cell-patch | 145 | 153 | 134 | 85 ns |
| 600-row-bulk-update | html-row-fragments | 102020 | 5540 | 2512 | 41452 ns |
| 600-row-bulk-update | json-range-op | 74 | 90 | 66 | 560 ns |
| 600-row-bulk-update | dx-range-op | 10 | 30 | 14 | 5 ns |

## Findings

- Template/data separation is the strongest size lever for repeated UI.
- Columnar slot data and semantic codecs are the next jump after template/data packets.
- Patch streams are the biggest live-app advantage: changed cells can be shipped as operations instead of HTML or JSON trees.
- rkyv is excellent for server/build zero-copy access, but generic archived object graphs are not automatically the smallest browser payload.
- memmap2 helps build/server/edge cache startup for large packets; browsers still need fetched ArrayBuffers or streams.
- Tiny pages need static or micro-JS mode; WASM should only ship when its fixed cost is amortized.
