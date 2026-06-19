# DX CSS Data Baseline

Generated on 2026-05-28 from the latest checked MDN data in this workspace.

## Sources

- CSS source: `https://github.com/mdn/data`
- Local checkout: `target/mdn-data`
- MDN data commit: `e4fb99ca67e94d3b63dd7b8c7e2bef00e43e73c0`
- Status checked: local `HEAD` matches `origin/main`

DX Devtools also has a generated catalog at `dx-www/src/cli/devtools/css_data.generated.json` from the same MDN data commit.

## MDN CSS Counts

| Data Set | Count |
| --- | ---: |
| CSS properties | 665 |
| CSS selectors | 144 |
| CSS at-rules | 19 |
| CSS property value syntaxes | 665 |
| CSS syntax definitions | 378 |
| CSS types | 39 |
| CSS units | 30 |
| CSS functions | 105 |
| CSS tracked literal values/functions extracted from MDN syntax tables | 1,530 |
| CSS property-to-literal-value pairs extracted from property syntax | 1,198 |

## Current DX Generated Catalog

| Data Set | Count |
| --- | ---: |
| Properties | 665 |
| Selectors | 144 |
| Property value syntaxes | 665 |
| Derived property value hints | 4,331 |
| Unique derived value hints | 723 |

## Implementation Notes

- `mdn/data` is the authoritative machine-readable source for CSS properties, selectors, at-rules, syntax definitions, types, units, and functions.
- DX should use property syntax as the source of truth, then derive editor-friendly value hints from that syntax.
- For DX Devtools and `dx-style`, the important baseline is not only property count. We need to preserve the full syntax string, the MDN status metadata, and generated value hints for practical controls.
- Custom properties are tracked by MDN as `--*`.
- Vendor and nonstandard properties are included because Devtools should inspect real browser CSS, not only the clean standards subset.

## Related JavaScript Event Baseline

- JS/Web API event source: `https://github.com/mdn/browser-compat-data`
- Local checkout: `target/mdn-browser-compat-data`
- Browser compat data commit: `8f1bd6944113ef4f3c031f12af66c87d61c6c8ef`
- Status checked: local `HEAD` matches `origin/main`

| Data Set | Count |
| --- | ---: |
| Parsed browser-compat JSON files | 2,827 |
| JavaScript/Web API event compatibility entries | 516 |
| Unique JavaScript event type values | 343 |

React DOM latest stable at this check was `react-dom@19.2.6`.

| React DOM Event Surface | Count |
| --- | ---: |
| Unique native DOM event names React listens to | 86 |
| Base React event handler props | 90 |
| Two-phase base props | 86 |
| Direct base props | 4 |
| Public handler props including capture variants | 176 |

For Triple W TSX, the target should be broader than React: support the MDN browser-compat unique event type values directly, then add React-compatible aliases where useful.
