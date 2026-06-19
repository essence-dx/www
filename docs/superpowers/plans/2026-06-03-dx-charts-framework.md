# DX Charts Framework Example Plan

**Date:** 2026-06-03
**Workspace:** `G:\Dx\www`
**Target:** `examples/charts`

## Goal

Build a production-ready DX WWW charts example inspired by AntV G2 and the AntV visualization ecosystem, without npm packages or runtime imports from AntV. The project must use DX WWW TSX, DX Style, DX Icon, and DX Check, while keeping source files small, typed, and maintainable.

## Reference Sources

- `inspirations/antv/G2`
- `inspirations/antv/G2Plot`
- `inspirations/antv/F2`
- `inspirations/antv/S2`
- `inspirations/antv/G6`
- `inspirations/antv/X6`
- `inspirations/antv/L7`
- `inspirations/antv/L7Plot`
- `inspirations/antv/Graphin`
- `inspirations/antv/G`
- `inspirations/antv/data-set`
- `inspirations/antv/AVA`
- `inspirations/antv/mcp-server-chart`
- `inspirations/antv/GPT-Vis`
- `inspirations/antv/chart-visualization-skills`
- `inspirations/antv/ant-design-charts`

These clones are reference-only and ignored by git.

## Product Shape

- Route `/`: chart framework dashboard with quick start, preview chart, package family map, and task finder.
- Route `/charts`: chart gallery by analytical task.
- Route `/docs`: concise grammar documentation for spec, data, transforms, scales, marks, axes, legends, themes, and interactions.
- Route `/theme`: theme gallery and palette/spec tokens.
- Route `/ecosystem`: AntV-inspired package family map translated into DX-native boundaries.

## Architecture

- `lib/charts/spec.ts`: public typed spec model.
- `lib/charts/data.ts`: source-owned datasets and metadata.
- `lib/charts/scales.ts`: linear, band, point, and ordinal scale helpers.
- `lib/charts/format.ts`: deterministic label and tick formatting.
- `lib/charts/geometry.ts`: path and shape helpers.
- `lib/charts/compile.ts`: spec-to-scene compiler.
- `lib/charts/scene.ts`: compiled SVG scene model.
- `lib/charts/gallery.ts`: chart catalog and route metadata.
- `components/charts/chart-frame.tsx`: accessible SVG renderer.
- `components/charts/*`: focused UI panels for gallery, docs, ecosystem, and theme.
- `public/chart-runtime.js`: tiny dependency-free tooltip/highlight runtime when needed.

## Implementation Steps

- [x] Create `examples/charts` DX project scaffold from existing WWW example conventions.
- [x] Add typed chart kernel modules with no package imports.
- [x] Implement cartesian marks: bar, line, area, point, rule.
- [x] Implement non-cartesian/example scene renderers for pie, radar, heatmap, treemap, sankey, graph, map, and pivot-table cards using the same catalog model.
- [x] Add catalog-driven TSX routes and components.
- [x] Add DX Icon wrapper and use `<Icon />` in commands/navigation.
- [x] Add DX Style tokens, generated CSS target, and chart CSS.
- [x] Run source-owned style/icon/check receipt generation where supported.
- [x] Add focused source guard tests without package installs.
- [ ] Stage chart-source slices, commit, and sync after focused verification.

## Verification Notes

Current source coverage includes `benchmarks/charts-www-project.test.cjs` and
`benchmarks/charts-waterfall-compiler.test.cjs`. These guard the DX-native chart
project shape, source-owned catalog, radial G2Plot adapters, waterfall adapter
and typed total semantics, L7/L7Plot map boundaries, DataSet/F2/S2/G6/X6/GPTVis
source slices, and route/runtime source contracts.

The chart source slice still needs a clean staging/commit/sync pass after the
dirty S2/table work is grouped intentionally. Do not present the chart lane as
synced until the current chart diff is staged, committed, and pushed.

Do not claim browser E2E, WebGL, live editor execution, or full G2 runtime
parity unless those paths are actually implemented and verified. The completed
project scope is a DX-native chart grammar/gallery with real source-owned chart
rendering and documentation.
