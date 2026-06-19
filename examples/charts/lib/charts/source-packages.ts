import type { ChartFamily } from "./spec";

export type AntVSourceSeries =
  | "series-2"
  | "series-6"
  | "series-7"
  | "ai"
  | "renderer"
  | "adapter";

export type AntVSourcePackage = {
  id: string;
  packageName: string;
  family: ChartFamily;
  series: AntVSourceSeries;
  sourcePath: string;
  dxBoundary: string;
  sourceOwnedAdapter: string;
  upstreamSignals: readonly string[];
  subpackages: readonly string[];
  noRuntimeImport: true;
};

export const ANT_V_SOURCE_PACKAGES: readonly AntVSourcePackage[] = [
  {
    id: "g2",
    packageName: "@antv/g2",
    family: "G2",
    series: "series-2",
    sourcePath: "inspirations/antv/G2",
    dxBoundary: "Grammar, marks, scales, transforms, coordinates, interactions, and theme tokens.",
    sourceOwnedAdapter: "DX ChartSpec plus compiler-owned SVG scene generation.",
    upstreamSignals: ["src/mark", "src/transform", "src/scale", "src/coordinate", "src/interaction", "src/theme"],
    subpackages: ["@antv/g2"],
    noRuntimeImport: true,
  },
  {
    id: "g2plot",
    packageName: "@antv/g2plot",
    family: "G2Plot",
    series: "series-2",
    sourcePath: "inspirations/antv/G2Plot",
    dxBoundary: "Preset plot configs lowered into local marks, transforms, coordinates, and scenes.",
    sourceOwnedAdapter: "G2Plot preset factories and typed transform metadata.",
    upstreamSignals: ["src/plots", "src/adaptor", "src/interactions", "src/core"],
    subpackages: [
      "area",
      "bar",
      "bidirectional-bar",
      "box",
      "bullet",
      "chord",
      "circle-packing",
      "column",
      "dual-axes",
      "facet",
      "funnel",
      "gauge",
      "heatmap",
      "histogram",
      "line",
      "liquid",
      "mix",
      "pie",
      "progress",
      "radar",
      "radial-bar",
      "ring-progress",
      "rose",
      "sankey",
      "scatter",
      "stock",
      "sunburst",
      "tiny-area",
      "tiny-column",
      "tiny-line",
      "treemap",
      "venn",
      "violin",
      "waterfall",
      "word-cloud",
    ],
    noRuntimeImport: true,
  },
  {
    id: "f2",
    packageName: "@antv/f2",
    family: "F2",
    series: "series-2",
    sourcePath: "inspirations/antv/F2",
    dxBoundary: "Mobile-first viewport, compact gestures, and extension marks.",
    sourceOwnedAdapter: "F2 mobile metadata plus deterministic compact chart scenes.",
    upstreamSignals: ["packages/f2", "packages/f2-wordcloud", "packages/f2-react", "packages/f2-vue"],
    subpackages: ["@antv/f2", "@antv/f2-alipay", "@antv/f2-my", "@antv/f2-node", "@antv/f2-react", "@antv/f2-vue", "@antv/f2-wordcloud", "@antv/f2-wx"],
    noRuntimeImport: true,
  },
  {
    id: "s2",
    packageName: "@antv/s2",
    family: "S2",
    series: "series-2",
    sourcePath: "inspirations/antv/S2",
    dxBoundary: "Spreadsheet-style pivot tables, tree tables, totals, drills, filters, and sorted cells.",
    sourceOwnedAdapter: "S2 table metadata plus local pivot scene model.",
    upstreamSignals: ["packages/s2-core", "packages/s2-react", "packages/s2-react-components", "packages/s2-ssr", "packages/s2-vue"],
    subpackages: ["@antv/s2", "@antv/s2-react", "@antv/s2-react-components", "@antv/s2-ssr", "@antv/s2-vue"],
    noRuntimeImport: true,
  },
  {
    id: "g6",
    packageName: "@antv/g6",
    family: "G6",
    series: "series-6",
    sourcePath: "inspirations/antv/G6",
    dxBoundary: "Graph layouts, nodes, edges, combos, behaviors, plugins, and interaction state.",
    sourceOwnedAdapter: "DX graph model and graph scene compiler.",
    upstreamSignals: ["packages/g6", "packages/g6-extension-react", "packages/g6-extension-3d", "packages/g6-ssr"],
    subpackages: ["@antv/g6", "@antv/g6-extension-react", "@antv/g6-extension-3d", "@antv/g6-ssr"],
    noRuntimeImport: true,
  },
  {
    id: "x6",
    packageName: "@antv/x6",
    family: "X6",
    series: "series-6",
    sourcePath: "inspirations/antv/X6",
    dxBoundary: "Diagram editor nodes, ports, terminals, connectors, routing, and selection metadata.",
    sourceOwnedAdapter: "DX diagram model lowered into graph-compatible scene metadata.",
    upstreamSignals: ["package.json", "site", "examples"],
    subpackages: ["@antv/x6"],
    noRuntimeImport: true,
  },
  {
    id: "l7",
    packageName: "@antv/l7",
    family: "L7",
    series: "series-7",
    sourcePath: "inspirations/antv/L7",
    dxBoundary: "Map scene, sources, layers, viewport, basemap, renderer, and geospatial interactions.",
    sourceOwnedAdapter: "L7 layer metadata and local spatial scene compiler.",
    upstreamSignals: ["packages/l7", "packages/core", "packages/layers", "packages/map", "packages/maps", "packages/scene", "packages/source"],
    subpackages: ["@antv/l7", "@antv/l7-core", "@antv/l7-component", "@antv/l7-layers", "@antv/l7-map", "@antv/l7-maps", "@antv/l7-renderer", "@antv/l7-scene", "@antv/l7-source", "@antv/l7-three", "@antv/l7-utils"],
    noRuntimeImport: true,
  },
  {
    id: "l7plot",
    packageName: "@antv/l7plot",
    family: "L7Plot",
    series: "series-7",
    sourcePath: "inspirations/antv/L7Plot",
    dxBoundary: "Geospatial preset charts and composite layer configs.",
    sourceOwnedAdapter: "L7Plot preset factories for dot maps, choropleths, and composite layers.",
    upstreamSignals: ["packages/l7plot", "packages/component", "packages/composite-layers"],
    subpackages: ["@antv/l7plot", "@antv/l7plot-component", "@antv/l7-composite-layers"],
    noRuntimeImport: true,
  },
  {
    id: "graphin",
    packageName: "@antv/graphin",
    family: "Graphin",
    series: "series-6",
    sourcePath: "inspirations/antv/Graphin",
    dxBoundary: "Graph application shell, node detail workflows, core assets, and SDK data contracts.",
    sourceOwnedAdapter: "Graph app card metadata over the local graph model.",
    upstreamSignals: ["packages/graphin", "packages/gi-sdk", "packages/gi-core-assets"],
    subpackages: ["@antv/graphin", "@antv/gi-sdk", "@antv/gi-core-assets"],
    noRuntimeImport: true,
  },
  {
    id: "g",
    packageName: "@antv/g",
    family: "G",
    series: "renderer",
    sourcePath: "inspirations/antv/G",
    dxBoundary: "Renderer capability contracts across SVG, Canvas, WebGL, WebGPU, plugins, and math utilities.",
    sourceOwnedAdapter: "Renderer capability matrix and scene metadata.",
    upstreamSignals: ["packages/g", "packages/g-lite", "packages/g-canvas", "packages/g-svg", "packages/g-webgl", "packages/g-math"],
    subpackages: ["@antv/g", "@antv/g-lite", "@antv/g-canvas", "@antv/g-svg", "@antv/g-webgl", "@antv/g-math"],
    noRuntimeImport: true,
  },
  {
    id: "data-set",
    packageName: "@antv/data-set",
    family: "DataSet",
    series: "adapter",
    sourcePath: "inspirations/antv/data-set",
    dxBoundary: "Data views, transforms, joins, folds, bins, filters, and derived table pipelines.",
    sourceOwnedAdapter: "Local DataSet pipeline runner and transform summaries.",
    upstreamSignals: ["src", "package.json"],
    subpackages: ["@antv/data-set"],
    noRuntimeImport: true,
  },
  {
    id: "ava",
    packageName: "@antv/ava",
    family: "AVA",
    series: "ai",
    sourcePath: "inspirations/antv/AVA",
    dxBoundary: "Chart advice, data analysis, recommendation scoring, and explanation rows.",
    sourceOwnedAdapter: "Local advice model and recommendation materialization.",
    upstreamSignals: ["src/ava.ts", "src/visualization/advisor.ts", "src/visualization/generator.ts"],
    subpackages: ["@antv/ava"],
    noRuntimeImport: true,
  },
  {
    id: "gpt-vis",
    packageName: "@antv/gpt-vis",
    family: "GPTVis",
    series: "ai",
    sourcePath: "inspirations/antv/GPT-Vis",
    dxBoundary: "Prompt chart syntax, tool routing, chart registry, and generated chart specs.",
    sourceOwnedAdapter: "DX GPTVis router and syntax materialization.",
    upstreamSignals: ["src/gpt-vis/index.ts", "bindings", "vis syntax"],
    subpackages: ["@antv/gpt-vis", "@antv/gpt-vis-ssr", "streamlit-gpt-vis"],
    noRuntimeImport: true,
  },
  {
    id: "mcp-server-chart",
    packageName: "@antv/mcp-server-chart",
    family: "MCP",
    series: "ai",
    sourcePath: "inspirations/antv/mcp-server-chart",
    dxBoundary: "Agent-facing chart request catalog and server tool contracts.",
    sourceOwnedAdapter: "Local MCP request flow graph and chart skill catalog.",
    upstreamSignals: ["package.json", "src"],
    subpackages: ["@antv/mcp-server-chart"],
    noRuntimeImport: true,
  },
  {
    id: "chart-visualization-skills",
    packageName: "@antv/chart-visualization-skills",
    family: "ChartSkills",
    series: "ai",
    sourcePath: "inspirations/antv/chart-visualization-skills",
    dxBoundary: "Reusable visualization skill prompts, generated chart examples, and harness workflows.",
    sourceOwnedAdapter: "Local chart skills graph and workflow materialization.",
    upstreamSignals: ["package.json", "harness"],
    subpackages: ["@antv/chart-visualization-skills"],
    noRuntimeImport: true,
  },
  {
    id: "ant-design-charts",
    packageName: "@ant-design/charts",
    family: "AntDesignCharts",
    series: "adapter",
    sourcePath: "inspirations/antv/ant-design-charts/packages/charts",
    dxBoundary: "React-style chart component wrappers represented as source-owned configs.",
    sourceOwnedAdapter: "Ant Design Charts config adapter without React or package imports.",
    upstreamSignals: ["packages/charts", "packages/plots", "packages/graphs", "packages/util"],
    subpackages: ["@ant-design/charts", "@ant-design/plots", "@ant-design/graphs", "@ant-design/charts-util"],
    noRuntimeImport: true,
  },
  {
    id: "ant-design-plots",
    packageName: "@ant-design/plots",
    family: "AntDesignPlots",
    series: "adapter",
    sourcePath: "inspirations/antv/ant-design-charts/packages/plots",
    dxBoundary: "Ant Design plot component configs lowered to DX chart specs.",
    sourceOwnedAdapter: "Dual-axes and plot wrappers mapped to local G2Plot-style marks.",
    upstreamSignals: ["packages/plots/src/components", "packages/plots/src/core", "packages/plots/src/hooks"],
    subpackages: ["@ant-design/plots"],
    noRuntimeImport: true,
  },
  {
    id: "ant-design-graphs",
    packageName: "@ant-design/graphs",
    family: "AntDesignGraphs",
    series: "adapter",
    sourcePath: "inspirations/antv/ant-design-charts/packages/graphs",
    dxBoundary: "Ant Design graph component configs lowered to DX graph models.",
    sourceOwnedAdapter: "Graph component adapter over local G6/Graphin semantics.",
    upstreamSignals: ["packages/graphs/src"],
    subpackages: ["@ant-design/graphs"],
    noRuntimeImport: true,
  },
];

export const ANT_V_SERIES_LABELS: Record<AntVSourceSeries, string> = {
  "series-2": "Series 2 statistical and mobile charts",
  "series-6": "Series 6 graph and diagram systems",
  "series-7": "Series 7 geospatial systems",
  ai: "AI and agent visualization tools",
  renderer: "Renderer foundations",
  adapter: "Framework adapter packages",
};

export function findAntVSourcePackage(id: string): AntVSourcePackage | undefined {
  return ANT_V_SOURCE_PACKAGES.find((entry) => entry.id === id || entry.packageName === id);
}

export function antvPackagesBySeries(series: AntVSourceSeries): AntVSourcePackage[] {
  return ANT_V_SOURCE_PACKAGES.filter((entry) => entry.series === series);
}

export function antvSourcePackageCount(): number {
  return ANT_V_SOURCE_PACKAGES.length;
}
