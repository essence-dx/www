# Forge Package Vertical Comparison

Generated: 2026-05-16T08:24:08.940Z

| Fixture mode | Forge packages | Tracked files | Decoded bytes | Brotli bytes | HTTP median | Chrome load median | DXPK applied | Interaction |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | --- | --- |
| forge-package | 1 | 4 | 10669 B | 3026 B | 1.997 ms | 27.8 ms | true | true |
| forge-icon | 1 | 3 | 11008 B | 3125 B | 2.987 ms | 44.0 ms | true | true |
| forge-combo | 2 | 7 | 10867 B | 3105 B | 3.314 ms | 59.1 ms | true | true |

## Deltas

| Comparison | Packages | Tracked files | Decoded bytes | Brotli bytes | HTTP median | Chrome load median |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| forge-icon vs forge-package | 0 | -1 | +339 B | +99 B | +0.990 ms | +16.2 ms |
| forge-combo vs forge-icon | +1 | +4 | -141 B | -20 B | +0.327 ms | +15.1 ms |

The selected-icon route proves the new package vertical with `dx/icon/search`: one source-owned Forge package, three tracked files, no `node_modules`, DXPK DOM apply success, and a working counter interaction. The combined route starts the next proof set by materializing both `shadcn/ui/button` and `dx/icon/search` in one page.
