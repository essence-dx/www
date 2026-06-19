import { materializeAdviceRows, recommendCharts } from "./advice-model";
import { createDataSetView, materializeDataSetFlowRows } from "./dataset-model";
import type { AgentChartRequestSpec, ChartAdviceIntentSpec, ChartPromptRequestSpec, ChartSkillSpec, DataSetViewSpec, Datum, DiagramModelSpec, GeoRegionSpec, GraphComboSpec, GraphEdgeSpec, GraphNodeSpec, TableFilterSpec } from "./spec";

export const weeklyActivity: Datum[] = [
  { day: "Mon", builds: 18, failures: 2, lane: "Core", target: 28 },
  { day: "Tue", builds: 24, failures: 3, lane: "Style", target: 28 },
  { day: "Wed", builds: 31, failures: 4, lane: "Charts", target: 28 },
  { day: "Thu", builds: 27, failures: 2, lane: "Icons", target: 28 },
  { day: "Fri", builds: 36, failures: 5, lane: "Check", target: 28 },
  { day: "Sat", builds: 22, failures: 1, lane: "Docs", target: 28 },
  { day: "Sun", builds: 15, failures: 1, lane: "Runtime", target: 28 },
];

export const mobileChartTerms: Datum[] = [
  { term: "tap", weight: 38 },
  { term: "tooltip", weight: 34 },
  { term: "gesture", weight: 31 },
  { term: "sparkline", weight: 29 },
  { term: "viewport", weight: 25 },
  { term: "pinch", weight: 22 },
  { term: "pan", weight: 20 },
  { term: "safe area", weight: 18 },
  { term: "pixel ratio", weight: 16 },
  { term: "compact", weight: 14 },
];

export const receiptTrend: Datum[] = [
  { week: "W1", score: 72, series: "Style" },
  { week: "W2", score: 81, series: "Style" },
  { week: "W3", score: 86, series: "Style" },
  { week: "W4", score: 91, series: "Style" },
  { week: "W5", score: 96, series: "Style" },
  { week: "W1", score: 65, series: "Check" },
  { week: "W2", score: 74, series: "Check" },
  { week: "W3", score: 82, series: "Check" },
  { week: "W4", score: 88, series: "Check" },
  { week: "W5", score: 94, series: "Check" },
];

export const releaseWorkMix: Datum[] = [
  { week: "W1", lane: "Grammar", hours: 18 },
  { week: "W1", lane: "Runtime", hours: 12 },
  { week: "W1", lane: "Proof", hours: 10 },
  { week: "W2", lane: "Grammar", hours: 22 },
  { week: "W2", lane: "Runtime", hours: 18 },
  { week: "W2", lane: "Proof", hours: 14 },
  { week: "W3", lane: "Grammar", hours: 28 },
  { week: "W3", lane: "Runtime", hours: 20 },
  { week: "W3", lane: "Proof", hours: 18 },
  { week: "W4", lane: "Grammar", hours: 30 },
  { week: "W4", lane: "Runtime", hours: 24 },
  { week: "W4", lane: "Proof", hours: 22 },
];

export const packageModeScores: Datum[] = [
  { package: "G2", mode: "Source", score: 92 },
  { package: "G2", mode: "Runtime", score: 78 },
  { package: "G2", mode: "Proof", score: 86 },
  { package: "G2Plot", mode: "Source", score: 88 },
  { package: "G2Plot", mode: "Runtime", score: 76 },
  { package: "G2Plot", mode: "Proof", score: 82 },
  { package: "G6", mode: "Source", score: 80 },
  { package: "G6", mode: "Runtime", score: 62 },
  { package: "G6", mode: "Proof", score: 70 },
  { package: "L7", mode: "Source", score: 76 },
  { package: "L7", mode: "Runtime", score: 58 },
  { package: "L7", mode: "Proof", score: 66 },
];

export const packageReadiness: Datum[] = [
  { package: "G2", score: 92, family: "Statistical" },
  { package: "G2Plot", score: 84, family: "Statistical" },
  { package: "F2", score: 79, family: "Mobile" },
  { package: "S2", score: 88, family: "Table" },
  { package: "G6", score: 83, family: "Graph" },
  { package: "X6", score: 81, family: "Editor" },
  { package: "L7", score: 78, family: "Map" },
  { package: "AVA", score: 74, family: "AI" },
];

export const packageAdoptionSignals: Datum[] = [
  { package: "G2", readiness: 92, adoption: 88, family: "Statistical" },
  { package: "G2Plot", readiness: 84, adoption: 81, family: "Statistical" },
  { package: "S2", readiness: 88, adoption: 76, family: "Table" },
  { package: "G6", readiness: 83, adoption: 78, family: "Graph" },
  { package: "L7", readiness: 78, adoption: 69, family: "Map" },
  { package: "GPTVis", readiness: 72, adoption: 64, family: "AI" },
];

export const runtimeProofs: Datum[] = [
  { surface: "SVG", complexity: 16, proof: 96, family: "Renderer" },
  { surface: "Canvas", complexity: 46, proof: 74, family: "Renderer" },
  { surface: "WebGL", complexity: 82, proof: 42, family: "Renderer" },
  { surface: "Tooltip", complexity: 24, proof: 88, family: "Interaction" },
  { surface: "Brush", complexity: 55, proof: 58, family: "Interaction" },
  { surface: "Theme", complexity: 30, proof: 91, family: "Style" },
  { surface: "A11y", complexity: 34, proof: 86, family: "Check" },
];

export const qualityScores: Datum[] = [
  { score: 64 },
  { score: 68 },
  { score: 71 },
  { score: 73 },
  { score: 75 },
  { score: 78 },
  { score: 81 },
  { score: 83 },
  { score: 84 },
  { score: 86 },
  { score: 88 },
  { score: 90 },
  { score: 91 },
  { score: 93 },
  { score: 95 },
  { score: 97 },
  { score: 98 },
  { score: 99 },
];

export const packageSpread: Datum[] = [
  { package: "G2", low: 72, q1: 86, median: 92, q3: 96, high: 99 },
  { package: "G2Plot", low: 68, q1: 80, median: 88, q3: 93, high: 97 },
  { package: "G6", low: 58, q1: 72, median: 83, q3: 89, high: 94 },
  { package: "S2", low: 62, q1: 76, median: 86, q3: 91, high: 96 },
  { package: "L7", low: 54, q1: 68, median: 79, q3: 86, high: 92 },
];

export const bulletReadiness: Datum[] = [
  { metric: "Grammar", value: 92, target: 95 },
  { metric: "Transforms", value: 78, target: 90 },
  { metric: "Runtime", value: 84, target: 92 },
  { metric: "A11y", value: 88, target: 94 },
  { metric: "Receipts", value: 96, target: 98 },
];

export const gaugeSignals: Datum[] = [
  { metric: "Source-owned coverage", value: 0.86, target: 0.92 },
];

export const waterfallReleaseDeltas: Datum[] = [
  { stage: "Planned scope", delta: 34, total: false },
  { stage: "Renderer work", delta: 18, total: false },
  { stage: "Risk removed", delta: -10, total: false },
  { stage: "DX checks", delta: 12, total: false },
  { stage: "Final proof", delta: 54, total: true },
];

export const funnelConversionStages: Datum[] = [
  { stage: "Spec loaded", count: 120 },
  { stage: "Fields mapped", count: 96 },
  { stage: "Preset selected", count: 72 },
  { stage: "DX style checked", count: 58 },
  { stage: "Receipt shipped", count: 42 },
];

export const proportionMix: Datum[] = [
  { label: "Dashboards", value: 34 },
  { label: "Exploration", value: 24 },
  { label: "Storytelling", value: 19 },
  { label: "Operations", value: 14 },
  { label: "Research", value: 9 },
];

export const heatmapCells: Datum[] = [
  { lane: "Core", task: "Spec", value: 92 },
  { lane: "Core", task: "Scale", value: 86 },
  { lane: "Core", task: "Mark", value: 88 },
  { lane: "Style", task: "Spec", value: 76 },
  { lane: "Style", task: "Scale", value: 94 },
  { lane: "Style", task: "Mark", value: 80 },
  { lane: "Icon", task: "Spec", value: 74 },
  { lane: "Icon", task: "Scale", value: 70 },
  { lane: "Icon", task: "Mark", value: 96 },
  { lane: "Check", task: "Spec", value: 84 },
  { lane: "Check", task: "Scale", value: 82 },
  { lane: "Check", task: "Mark", value: 90 },
];

export const radarCapabilities: Datum[] = [
  { axis: "Grammar", score: 95, series: "DX" },
  { axis: "Themes", score: 88, series: "DX" },
  { axis: "A11y", score: 90, series: "DX" },
  { axis: "Runtime", score: 76, series: "DX" },
  { axis: "Receipts", score: 98, series: "DX" },
  { axis: "Grammar", score: 91, series: "AntV" },
  { axis: "Themes", score: 93, series: "AntV" },
  { axis: "A11y", score: 70, series: "AntV" },
  { axis: "Runtime", score: 96, series: "AntV" },
  { axis: "Receipts", score: 58, series: "AntV" },
];

export const treemapPackages: Datum[] = [
  { label: "G2", value: 34 },
  { label: "G6", value: 22 },
  { label: "S2", value: 18 },
  { label: "L7", value: 17 },
  { label: "X6", value: 15 },
  { label: "AVA", value: 9 },
];

export const sunburstPackages: Datum[] = [
  { level: 0, label: "AntV ecosystem", parent: "", value: 140 },
  { level: 1, label: "G2 grammar", parent: "AntV ecosystem", value: 42 },
  { level: 1, label: "G2Plot presets", parent: "AntV ecosystem", value: 30 },
  { level: 1, label: "Graph systems", parent: "AntV ecosystem", value: 26 },
  { level: 1, label: "Geo layers", parent: "AntV ecosystem", value: 20 },
  { level: 1, label: "AI helpers", parent: "AntV ecosystem", value: 14 },
  { level: 1, label: "Support surfaces", parent: "AntV ecosystem", value: 8 },
  { level: 2, label: "Marks", parent: "G2 grammar", value: 18 },
  { level: 2, label: "Transforms", parent: "G2 grammar", value: 16 },
  { level: 2, label: "Coordinates", parent: "G2 grammar", value: 8 },
  { level: 2, label: "Preset gallery", parent: "G2Plot presets", value: 18 },
  { level: 2, label: "Preset docs", parent: "G2Plot presets", value: 12 },
  { level: 2, label: "Graphs", parent: "Graph systems", value: 18 },
  { level: 2, label: "Diagrams", parent: "Graph systems", value: 8 },
  { level: 2, label: "Maps", parent: "Geo layers", value: 15 },
  { level: 2, label: "Basemaps", parent: "Geo layers", value: 5 },
  { level: 2, label: "Recommendations", parent: "AI helpers", value: 9 },
  { level: 2, label: "Assistants", parent: "AI helpers", value: 5 },
  { level: 2, label: "Themes", parent: "Support surfaces", value: 5 },
  { level: 2, label: "Receipts", parent: "Support surfaces", value: 3 },
];

export const datasetPipeline: DataSetViewSpec = {
  id: "dataset-transform-pipeline",
  label: "DataSet transform pipeline",
  sourceLabel: "Release rows",
  sourceRows: releaseWorkMix,
  steps: [
    { id: "filter-release-work", label: "Filter >= 14h", transform: { type: "filter", field: "hours", min: 14 } },
    { id: "aggregate-lane-hours", label: "Aggregate lane hours", transform: { type: "group", groupBy: ["lane"], field: "hours", as: "hours", reducer: "sum" } },
    { id: "sort-lane-hours", label: "Sort lane totals", transform: { type: "sort", field: "hours", order: "desc" } },
  ],
};

export const flowEdges: Datum[] = materializeDataSetFlowRows(createDataSetView(datasetPipeline));

export const graphNodes: GraphNodeSpec[] = [
  { id: "G", label: "G Renderer", combo: "runtime", value: 9 },
  { id: "G2", label: "G2 Grammar", combo: "statistical", value: 10 },
  { id: "G2Plot", label: "G2Plot Presets", combo: "statistical", value: 8 },
  { id: "F2", label: "F2 Mobile", combo: "runtime", value: 6 },
  { id: "S2", label: "S2 Tables", combo: "analysis", value: 7 },
  { id: "G6", label: "G6 Graphs", combo: "graph", value: 8 },
  { id: "Graphin", label: "Graphin App", combo: "graph", value: 6 },
  { id: "X6", label: "X6 Diagrams", combo: "graph", value: 6 },
  { id: "L7", label: "L7 Maps", combo: "geo", value: 7 },
  { id: "AVA", label: "AVA Advice", combo: "analysis", value: 5 },
];

export const graphCombos: GraphComboSpec[] = [
  { id: "statistical", label: "Statistical charts" },
  { id: "graph", label: "Graph systems" },
  { id: "geo", label: "Geo layers" },
  { id: "runtime", label: "Rendering runtime" },
  { id: "analysis", label: "Analysis helpers" },
];

export const graphModelEdges: GraphEdgeSpec[] = [
  { id: "edge-g2-g", source: "G2", target: "G", relation: "renders through", weight: 4 },
  { id: "edge-g2plot-g2", source: "G2Plot", target: "G2", relation: "preset layer", weight: 5 },
  { id: "edge-f2-g", source: "F2", target: "G", relation: "mobile renderer", weight: 3 },
  { id: "edge-s2-g", source: "S2", target: "G", relation: "table canvas", weight: 3 },
  { id: "edge-g6-g", source: "G6", target: "G", relation: "graph renderer", weight: 4 },
  { id: "edge-graphin-g6", source: "Graphin", target: "G6", relation: "app shell", weight: 4 },
  { id: "edge-x6-g", source: "X6", target: "G", relation: "diagram renderer", weight: 3 },
  { id: "edge-l7-g", source: "L7", target: "G", relation: "map renderer", weight: 3 },
  { id: "edge-ava-g2", source: "AVA", target: "G2", relation: "chart advice", weight: 2 },
];

export const graphEdges: Datum[] = graphModelEdges.map((edge) => ({
  id: edge.id ?? "",
  source: edge.source,
  target: edge.target,
  relation: edge.relation ?? "",
  weight: edge.weight ?? 1,
}));

export const packageRelationFlows: Datum[] = [
  { source: "G2Plot", target: "G2", weight: 14, relation: "preset layer" },
  { source: "G2", target: "G", weight: 12, relation: "renderer grammar" },
  { source: "F2", target: "G", weight: 8, relation: "mobile renderer" },
  { source: "S2", target: "G", weight: 7, relation: "table canvas" },
  { source: "G6", target: "G", weight: 10, relation: "graph renderer" },
  { source: "Graphin", target: "G6", weight: 7, relation: "graph app shell" },
  { source: "X6", target: "G", weight: 6, relation: "diagram renderer" },
  { source: "L7", target: "G", weight: 8, relation: "map renderer" },
  { source: "AVA", target: "G2", weight: 5, relation: "chart advice" },
  { source: "GPTVis", target: "G2Plot", weight: 6, relation: "prompt route" },
];

export const diagramWorkflow: DiagramModelSpec = {
  nodes: [
    { id: "source", label: "Source rows", ports: [{ id: "out", group: "output", label: "Rows", position: "right" }] },
    { id: "transform", label: "Transform", ports: [{ id: "in", group: "input", label: "Input", position: "left" }, { id: "out", group: "output", label: "View", position: "right" }] },
    { id: "scene", label: "Scene graph", ports: [{ id: "in", group: "input", label: "Spec", position: "left" }, { id: "out", group: "output", label: "SVG", position: "right" }] },
    { id: "receipt", label: "Check receipt", ports: [{ id: "in", group: "input", label: "Proof", position: "left" }] },
  ],
  edges: [
    { id: "edge-source-transform", source: { cell: "source", port: "out" }, target: { cell: "transform", port: "in" }, label: "filter and group", router: { name: "orth", padding: 12 }, connector: { name: "rounded", radius: 8 } },
    { id: "edge-transform-scene", source: { cell: "transform", port: "out" }, target: { cell: "scene", port: "in" }, label: "compile spec", router: { name: "orth", padding: 12 }, connector: { name: "rounded", radius: 8 } },
    { id: "edge-scene-receipt", source: { cell: "scene", port: "out" }, target: { cell: "receipt", port: "in" }, label: "verify", router: { name: "orth", padding: 12 }, connector: { name: "rounded", radius: 8 } },
  ],
  interactions: [{ type: "select-node" }, { type: "connect-port" }, { type: "drag-node" }, { type: "zoom-canvas" }],
};

export const geoUsagePoints: Datum[] = [
  { city: "Dhaka", x: 90.4, y: 23.8, value: 220 },
  { city: "Singapore", x: 103.8, y: 1.3, value: 180 },
  { city: "Tokyo", x: 139.6, y: 35.7, value: 240 },
  { city: "Paris", x: 2.3, y: 48.8, value: 140 },
  { city: "San Francisco", x: -122.4, y: 37.7, value: 160 },
  { city: "Sao Paulo", x: -46.6, y: -23.5, value: 120 },
];

export const geoUsageRegions: Datum[] = [
  { region: "South Asia", x: 90.4, y: 23.8, value: 220 },
  { region: "Southeast Asia", x: 103.8, y: 1.3, value: 180 },
  { region: "East Asia", x: 139.6, y: 35.7, value: 240 },
  { region: "Western Europe", x: 2.3, y: 48.8, value: 140 },
  { region: "North America West", x: -122.4, y: 37.7, value: 160 },
  { region: "South America East", x: -46.6, y: -23.5, value: 120 },
];

export const geoUsageRegionShapes: GeoRegionSpec[] = [
  { id: "South Asia", label: "South Asia launch cluster", points: [[68, 7], [91, 5], [100, 18], [96, 32], [72, 34], [65, 20]] },
  { id: "Southeast Asia", label: "Southeast Asia runtime cluster", points: [[94, -10], [112, -8], [118, 4], [110, 17], [96, 12], [90, 0]] },
  { id: "East Asia", label: "East Asia ecosystem cluster", points: [[118, 22], [145, 20], [154, 34], [144, 48], [122, 43], [112, 31]] },
  { id: "Western Europe", label: "Western Europe proof cluster", points: [[-10, 38], [14, 37], [24, 48], [12, 57], [-6, 55], [-16, 46]] },
  { id: "North America West", label: "North America West editor cluster", points: [[-132, 26], [-108, 24], [-96, 40], [-108, 53], [-132, 50], [-144, 36]] },
  { id: "South America East", label: "South America East adoption cluster", points: [[-60, -35], [-35, -33], [-30, -18], [-42, -5], [-62, -12], [-70, -26]] },
];

export const mapPoints = geoUsagePoints;

export const pivotReadinessRows: Datum[] = [
  { quarter: "Q1", tier: "Core grammar", family: "G2", visible: true, value: 92 },
  { quarter: "Q2", tier: "Core grammar", family: "G2", visible: true, value: 95 },
  { quarter: "Q3", tier: "Core grammar", family: "G2", visible: true, value: 97 },
  { quarter: "Q1", tier: "Graph grammar", family: "G6", visible: true, value: 78 },
  { quarter: "Q2", tier: "Graph grammar", family: "G6", visible: true, value: 84 },
  { quarter: "Q3", tier: "Graph grammar", family: "G6", visible: true, value: 86 },
  { quarter: "Q1", tier: "Geo grammar", family: "L7", visible: false, value: 66 },
  { quarter: "Q2", tier: "Geo grammar", family: "L7", visible: true, value: 75 },
  { quarter: "Q3", tier: "Geo grammar", family: "L7", visible: true, value: 82 },
  { quarter: "Q1", tier: "AI workflow", family: "AVA", visible: false, value: 58 },
  { quarter: "Q2", tier: "AI workflow", family: "AVA", visible: false, value: 67 },
  { quarter: "Q3", tier: "AI workflow", family: "AVA", visible: true, value: 74 },
];

export const pivotReadinessFilters: TableFilterSpec[] = [
  { field: "visible", equals: true },
  { field: "value", min: 70 },
];

export const pivotCells = pivotReadinessRows;

export const chartAdviceIntent: ChartAdviceIntentSpec = {
  task: "comparison",
  recordCount: 18,
  dimensions: ["choice", "reason"],
  measures: ["confidence"],
};

export const aiChartAdvice: Datum[] = materializeAdviceRows(recommendCharts(chartAdviceIntent).slice(0, 5));

export const gptVisPromptRequest: ChartPromptRequestSpec = {
  id: "prompt-package-readiness",
  prompt: "Compare package readiness scores by family and explain the strongest chart choice.",
  task: "comparison",
  recordCount: packageReadiness.length,
  sampleRows: packageReadiness,
  outputFamily: "GPTVis",
  maxRecommendations: 5,
  fields: [
    { name: "package", role: "dimension", label: "Package" },
    { name: "family", role: "dimension", label: "Family" },
    { name: "score", role: "measure", label: "Readiness score" },
  ],
};

export const agentChartRequest: AgentChartRequestSpec = {
  id: "request-chart-readiness",
  prompt: "Create a chart that explains package readiness by family.",
  task: "comparison",
  inputFields: ["package", "score", "family"],
  outputFamily: "G2Plot",
};

export const chartSkillCatalog: ChartSkillSpec[] = [
  { id: "skill-data-shape", label: "Infer data shape", task: "ai", family: "AVA", produces: ["bar", "point"] },
  { id: "skill-preset-select", label: "Select preset", task: "comparison", family: "GPTVis", produces: ["bar"], dependsOn: ["skill-data-shape"] },
  { id: "skill-style-check", label: "Apply DX style", task: "composition", family: "G2Plot", produces: ["bar", "line"], dependsOn: ["skill-preset-select"] },
  { id: "skill-receipt", label: "Emit receipt", task: "flow", family: "MCP", produces: ["sankey"], dependsOn: ["skill-style-check"] },
];
