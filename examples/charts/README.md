# DX WWW Charts

`examples/charts` is a source-owned chart framework example inspired by AntV G2 and the AntV visualization ecosystem. It does not install or import AntV, React DOM, Recharts, D3, chart UI packages, or npm runtime dependencies.

The local AntV reference clones live under `G:\Dx\www\inspirations\antv` and are reference material only.

## What Is Built

- Package parity coverage for the AntV chart ecosystem, including per-package status, source proof, interaction proof, and DX-owned adapter boundaries.
- A DX-native chart grammar with typed specs, local data, scales, marks, axes, legends, themes, and compiled SVG scenes.
- Source-owned G2Plot-style preset adapters for column, bar, line, area, scatter, histogram, facet, heatmap, bullet, box, radial, rose, gauge, funnel, waterfall, tiny, ring-progress, and progress configs.
- Source-owned F2-style compact mobile charts and wordcloud extension marks with deterministic SVG text layout.
- Source-owned DataSet-style transform views that run typed filter, aggregate, and sort pipelines before chart materialization.
- Source-owned graph model support for nodes, edges, combos, deterministic layouts, focused relation state, behavior metadata, and plugin metadata.
- Source-owned S2-style pivot table semantics for row fields, column fields, measures, totals, sort state, drill paths, hierarchy metadata, and interaction metadata.
- Source-owned L7 and L7Plot-style geo layers with projection, viewport, basemap, composite heat/bubble layers, region layers, feature, and interaction metadata.
- Source-owned AVA and GPTVis-style chart advice scoring, prompt-to-tool routing, generated DX chart specs, and scene metadata.
- WWW-native global store proof through `lib/stores/counter.ts` and the state
  runtime route, separate from upstream Zustand provenance.
- React-style event authoring proof, including quoted event-class commands such
  as `onClick="bg-accent"` and braced action logic that lowers only when safe.
- A docs-first app with overview, chart gallery, examples, grammar docs, theme gallery, ecosystem map, and read-only playground routes.
- Source-owned chart renderers for bars, lines, areas, points, rules, heatmaps, pie arcs, radar, treemap, sankey, graph, map, pivot-table, and wordcloud scenes.
- A tiny dependency-free browser runtime for tooltip text and mark selection.
- DX Style config with grouped/atomic utility ownership, DX Icon `<Icon />`
  support, generated import map, Forge receipts, and DX Check project contract
  from `dx new`.

## Package Parity

The ecosystem route is a status surface, not a full upstream parity claim. Each package card exposes:

- `coverageStatus`: Implemented, Adapter, Model, or Reference.
- `sourceProof`: the local chart grammar, adapter, model, router, or catalog evidence backing the package slice.
- `interactionProof`: the accessible/runtime behavior that users can inspect without importing upstream packages.
- `data-dx-chart-*` attributes for benchmark receipts and future package-lane verification.

## Boundaries

- No `node_modules` folder is required for this project.
- AntV source is not imported or vendored into the app.
- Browser E2E, WebGL rendering, live code editing, and full G2 runtime parity are not claimed by this example.

## Local Commands

Run these from this project root:

```powershell
dx icons sync --json
dx imports sync --json
dx style build --json
dx icons check --json
dx imports check --json
dx style check --json
dx check --json
dx check --project-contract --strict-project-contract --json
```
