const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");
const projectRoot = path.join(repoRoot, "examples", "charts");

function read(relativePath) {
  return fs.readFileSync(path.join(projectRoot, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function exists(relativePath) {
  return fs.existsSync(path.join(projectRoot, relativePath));
}

function listFiles(relativeRoot) {
  const root = path.join(projectRoot, relativeRoot);
  const files = [];

  function walk(current) {
    for (const entry of fs.readdirSync(current, { withFileTypes: true })) {
      const fullPath = path.join(current, entry.name);
      if (entry.isDirectory()) {
        if (entry.name === ".dx") continue;
        walk(fullPath);
      } else {
        files.push(path.relative(projectRoot, fullPath).replace(/\\/g, "/"));
      }
    }
  }

  walk(root);
  return files;
}

function sourceAppRoutes() {
  return listFiles("app")
    .filter((file) => file === "app/page.tsx" || file.endsWith("/page.tsx"))
    .map((file) => {
      const route = file
        .slice("app/".length)
        .replace(/\/page\.tsx$/, "")
        .replace(/^page\.tsx$/, "");
      return route ? `/${route}` : "/";
    })
    .sort();
}

test("charts example is a DX WWW project with style icons forge and check", () => {
  const dx = read("dx");

  assert.match(dx, /project\(name="charts"/);
  assert.match(dx, /www\(/);
  assert.match(dx, /style\(/);
  assert.match(dx, /icons\(component=Icon source_tag=icon runtime_tag=dx-icon generated_dir=components\/icons\)/);
  assert.match(dx, /forge\(policy=forge-first-no-node-modules\)/);
  assert.match(dx, /check\(score_scale=500 lighthouse=true\)/);
  assert.ok(!fs.existsSync(path.join(projectRoot, "node_modules")));
  assert.ok(!fs.existsSync(path.join(projectRoot, "package.json")));
});

test("charts routes distinguish product routes from source proof routes", (t) => {
  const productRoutes = ["/", "/charts", "/examples", "/docs", "/theme", "/ecosystem", "/playground"];
  const proofRoutes = ["/islands", "/state-runtime"];
  const expectedBuildRoutes = sourceAppRoutes();
  const routesSource = read("lib/charts/routes.ts");

  for (const route of productRoutes) {
    assert.match(routesSource, new RegExp(`href: "${route === "/" ? "/" : route}"`));
  }

  for (const route of proofRoutes) {
    assert.doesNotMatch(routesSource, new RegExp(`href: "${route}"`));
  }

  assert.deepEqual(expectedBuildRoutes, [...productRoutes, ...proofRoutes].sort());
  assert.match(read("app/islands/page.tsx"), /data-dx-proof-route/);
  assert.match(read("app/state-runtime/page.tsx"), /data-dx-proof-route/);

  const discoveryPath = ".dx/www/output/app-route-discovery.json";
  const deployPath = ".dx/www/output/deploy-adapter.json";
  const hasDiscovery = exists(discoveryPath);
  const hasDeploy = exists(deployPath);
  const requiresGeneratedProof = process.env.DX_CHARTS_REQUIRE_GENERATED_PROOF === "1";
  if (!hasDiscovery || !hasDeploy) {
    const missing = [
      hasDiscovery ? undefined : discoveryPath,
      hasDeploy ? undefined : deployPath,
    ].filter(Boolean).join(", ");
    if (requiresGeneratedProof) {
      assert.fail(`generated charts build proof is incomplete: missing ${missing}`);
    }
    t.diagnostic(`generated charts build proof is not complete; source route contract was checked; missing ${missing}`);
    return;
  }

  const discovery = readJson(discoveryPath);
  const deploy = readJson(deployPath);
  assert.deepEqual(discovery.routes.map((route) => route.route_path).sort(), expectedBuildRoutes);
  assert.deepEqual(deploy.routes.map((route) => route.path).sort(), expectedBuildRoutes);
});

test("charts framework completeness owns route boundaries groups cache and stream contracts", () => {
  for (const boundaryFile of ["app/loading.tsx", "app/error.tsx", "app/not-found.tsx"]) {
    assert.ok(exists(boundaryFile), `${boundaryFile} should exist`);
    assert.match(read(boundaryFile), /ChartRouteBoundary/);
    assert.match(read(boundaryFile), /ChartSiteShell/);
  }

  const boundaryComponent = read("components/charts/route-boundary.tsx");
  for (const marker of ["charts:activity", "charts:terminal", "data-dx-route-boundary", "aria-live"]) {
    assert.match(boundaryComponent, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  const routeGroups = read("lib/charts/route-groups.ts");
  for (const marker of ["CHART_ROUTE_GROUPS", "product-catalog", "source-proof", "routeGroupForRoute", "assertChartRouteGroupCoverage"]) {
    assert.match(routeGroups, new RegExp(marker));
  }

  const serverDelivery = read("server/charts/delivery.ts");
  for (const marker of ["chartCatalogCachePolicy", "chartManifestStreamFrames", "revalidateSeconds", "manual-refresh", "streamBoundary"]) {
    assert.match(serverDelivery, new RegExp(marker));
  }

  const manifest = read("server/charts/manifest.ts");
  for (const marker of ["cachePolicy", "streamFrameCount", "streamBoundary"]) {
    assert.match(manifest, new RegExp(marker));
  }

  const completeness = read("components/launch/framework-completeness.ts");
  assert.doesNotMatch(completeness, /"partial"/);
  assert.match(completeness, /partialCount: 0/);
  for (const id of ["loading-error-not-found-boundaries", "route-groups", "streaming-response-boundary", "cache-revalidate-story"]) {
    const pattern = new RegExp(`"${id}"[\\s\\S]*?"source-owned"`);
    assert.match(completeness, pattern, `${id} should be source-owned`);
  }
});

test("charts source avoids npm visualization runtime imports", () => {
  const forbidden = [
    "from \"@antv/",
    "from '@antv/",
    "import \"@antv/",
    "import '@antv/",
    "require(\"@antv/",
    "require('@antv/",
    "from \"react\"",
    "from 'react'",
    "recharts",
    "d3-",
    "lodash",
    "antd",
    "styled-components",
    "node_modules",
  ];

  for (const file of listFiles(".")) {
    if (!/\.(ts|tsx|js)$/.test(file)) continue;
    const source = read(file);
    for (const marker of forbidden) {
      assert.doesNotMatch(source, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")), `${file} contains ${marker}`);
    }
  }
});

test("charts source package registry tracks AntV reference clone boundaries", () => {
  const sourcePackages = read("lib/charts/source-packages.ts");
  const index = read("lib/charts/index.ts");

  for (const marker of ["ANT_V_SOURCE_PACKAGES", "ANT_V_SERIES_LABELS", "findAntVSourcePackage", "antvPackagesBySeries", "noRuntimeImport: true"]) {
    assert.match(sourcePackages, new RegExp(marker));
  }

  for (const packageName of [
    "@antv/g2",
    "@antv/g2plot",
    "@antv/f2",
    "@antv/s2",
    "@antv/g6",
    "@antv/x6",
    "@antv/l7",
    "@antv/l7plot",
    "@antv/graphin",
    "@antv/g",
    "@antv/data-set",
    "@antv/ava",
    "@antv/gpt-vis",
    "@antv/mcp-server-chart",
    "@antv/chart-visualization-skills",
    "@ant-design/charts",
    "@ant-design/plots",
    "@ant-design/graphs",
  ]) {
    assert.match(sourcePackages, new RegExp(packageName.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const marker of ["series-2", "series-6", "series-7", "src/plots", "packages/s2-core", "packages/l7", "packages/graphs/src"]) {
    assert.match(sourcePackages, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(index, /ANT_V_SOURCE_PACKAGES/);
  assert.match(index, /AntVSourcePackage/);
});

test("charts catalog covers the AntV visualization families as DX boundaries", () => {
  const gallery = read("lib/charts/gallery.ts");
  for (const family of ["G2", "G2Plot", "F2", "S2", "G6", "X6", "L7", "L7Plot", "Graphin", "G", "DataSet", "AVA", "GPTVis", "MCP", "ChartSkills", "AntDesignCharts"]) {
    assert.match(gallery, new RegExp(`family: "${family}"|family: '${family}'|\"${family}\"`), `${family} family is represented`);
  }
});

test("charts ecosystem page exposes package parity status and source proof", () => {
  const gallery = read("lib/charts/gallery.ts");
  const ecosystem = read("components/charts/ecosystem-overview.tsx");
  const docs = read("components/charts/docs-overview.tsx");
  const readme = read("README.md");

  const ecosystemBlock = gallery.slice(gallery.indexOf("export const ecosystemFamilies"));
  const packageCount = Array.from(ecosystemBlock.matchAll(/packageName:/g)).length;
  assert.ok(packageCount >= 18, "Expected the AntV package map to remain broad.");

  for (const field of ["coverageStatus:", "sourceProof:", "interactionProof:"]) {
    const count = Array.from(ecosystemBlock.matchAll(new RegExp(field, "g"))).length;
    assert.equal(count, packageCount, `Expected every mapped package to expose ${field}`);
  }

  for (const marker of [
    "data-dx-chart-package-status",
    "data-dx-chart-source-proof",
    "data-dx-chart-parity-status",
    "aria-describedby",
    "chart-package-",
  ]) {
    assert.match(ecosystem, new RegExp(marker), `Missing ecosystem accessibility/proof marker ${marker}`);
  }

  assert.match(docs, /Package parity/i);
  assert.match(readme, /Package parity/i);
});

test("charts compiler owns the first production mark and scene surface", () => {
  const compile = read("lib/charts/compile.ts");
  for (const marker of ["compileCartesian", "compilePie", "compileRadar", "compileGauge", "compileFunnel", "compileSankey", "compileGraph", "compileMap", "compilePivot", "compileSunburst"]) {
    assert.match(compile, new RegExp(marker));
  }

  const transforms = read("lib/charts/transforms.ts");
  for (const marker of ["applyMarkTransforms", "filterRows", "sortRows", "groupRows", "binRows", "stackRows", "dodgeRows", "STACK_START_FIELD", "DODGE_INDEX_FIELD"]) {
    assert.match(transforms, new RegExp(marker));
  }

  const composition = read("lib/charts/compiler/composition.ts");
  for (const marker of ["compileFacet", "compileView", "composition.type !== \"facet\"", "composition.type !== \"view\"", "normalizeRegion", "viewRegion", "coordinateType", "translate(", "compile(facetSpec)", "compile(childSpec)"]) {
    assert.match(composition, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  const frame = read("components/charts/chart-frame.tsx");
  assert.match(frame, /role="img"/);
  assert.match(frame, /data-dx-chart-hit/);
  assert.match(frame, /data-dx-chart-mark-id/);
  assert.match(frame, /aria-pressed/);
  assert.match(frame, /transform: element\.transform/);
  assert.match(frame, /data-dx-g2-coordinate/);
  assert.match(frame, /data-dx-g2-view-id/);
  assert.match(frame, /compileChart\(item\)/);
});

test("charts spec supports source-owned composition and advanced transforms", () => {
  const compile = read("lib/charts/compile.ts");
  const spec = read("lib/charts/spec.ts");
  const cartesianMarks = read("lib/charts/compiler/cartesian-marks.ts");
  const transforms = read("lib/charts/transforms.ts");
  const gallery = read("lib/charts/gallery.ts");
  const frame = read("components/charts/chart-frame.tsx");

  assert.match(compile, /compileFacet/);
  assert.match(compile, /compileView/);
  assert.match(spec, /type: "facet"/);
  assert.match(spec, /type: "view"/);
  assert.match(spec, /CoordinateSpec/);
  assert.match(spec, /coordinate\?: CoordinateSpec/);
  assert.match(spec, /ViewCompositionChildSpec/);
  assert.match(spec, /type: "stackY"/);
  assert.match(spec, /type: "normalizeY"/);
  assert.match(spec, /type: "dodgeX"/);
  assert.match(spec, /\| "funnel"/);
  assert.match(spec, /\| "waterfall"/);
  assert.match(gallery, /g2-view-grammar/);
  assert.match(gallery, /coordinate: \{ type: "transpose" \}/);
  assert.match(frame, /data-dx-g2-view-region/);
  assert.match(cartesianMarks, /STACK_START_FIELD/);
  assert.match(cartesianMarks, /DODGE_INDEX_FIELD/);
  assert.match(cartesianMarks, /barLayout/);
  assert.match(transforms, /STACK_END_FIELD/);
});

test("charts transformed bars keep stable scene semantics", () => {
  const cartesianMarks = read("lib/charts/compiler/cartesian-marks.ts");
  const transforms = read("lib/charts/transforms.ts");

  assert.match(transforms, /switch \(transform\.type\)/);
  assert.match(transforms, /return assertNever\(transform\)/);
  assert.match(transforms, /negativeTotal/);
  assert.match(transforms, /raw < 0 \? negativeTotal : positiveTotal/);
  assert.match(transforms, /const seriesDomain = Array\.from\(new Set\(data\.map/);
  assert.doesNotMatch(transforms, /const raw = Math\.max\(0, toNumber\(readField\(row, y\)\)\)/);
  assert.match(cartesianMarks, /const labelValue = stackStart === null \|\| stackEnd === null \? value : toNumber\(stackEnd\) - toNumber\(stackStart\)/);
  assert.match(cartesianMarks, /formatValue\(labelValue, y\.format\)/);
  assert.doesNotMatch(cartesianMarks, /label: `\$\{toLabel\(readField\(datum, x\.field\)\)\}: \$\{formatValue\(value, y\.format\)\}`/);
});

test("charts DataSet slice owns transform pipeline views", () => {
  const spec = read("lib/charts/spec.ts");
  const scene = read("lib/charts/scene.ts");
  const frame = read("components/charts/chart-frame.tsx");
  const transforms = read("lib/charts/transforms.ts");
  const datasetModel = read("lib/charts/dataset-model.ts");
  const datasetPreset = read("lib/charts/presets/dataset.ts");
  const network = read("lib/charts/compiler/network.ts");
  const data = read("lib/charts/data.ts");
  const gallery = read("lib/charts/gallery.ts");
  const index = read("lib/charts/index.ts");

  for (const marker of ["DataSetTransformStepSpec", "DataSetViewSpec", "DataSetStageSpec"]) {
    assert.match(spec, new RegExp(marker));
  }

  for (const marker of ["dataSetStageId", "dataSetStageName", "dataSetTransform", "dataSetRowCount"]) {
    assert.match(scene, new RegExp(marker));
  }

  for (const attr of ["data-dx-dataset-stage-id", "data-dx-dataset-stage-name", "data-dx-dataset-transform", "data-dx-dataset-row-count"]) {
    assert.match(frame, new RegExp(attr));
  }

  for (const marker of ["createDataSetView", "runDataSetPipeline", "materializeDataSetFlowRows", "summarizeDataSetPipeline", "DATASET_STAGE_SEPARATOR"]) {
    assert.match(datasetModel, new RegExp(marker));
  }

  assert.match(transforms, /export function applyTransforms/);
  assert.match(network, /dataSetMetadata/);
  for (const marker of ["DataSetFlowChartPreset", "dataSetFlowChart", "materializeDataSetFlowRows", "dataset: config.dataset"]) {
    assert.match(datasetPreset, new RegExp(marker));
  }

  assert.match(data, /datasetPipeline/);
  assert.match(data, /materializeDataSetFlowRows\(createDataSetView\(datasetPipeline\)\)/);
  assert.match(gallery, /dataSetFlowChart\(\{/);
  assert.match(index, /createDataSetView/);
  assert.match(index, /dataSetFlowChart/);
});

test("charts F2 slice owns mobile viewport and gesture metadata", () => {
  const spec = read("lib/charts/spec.ts");
  const scene = read("lib/charts/scene.ts");
  const frame = read("components/charts/chart-frame.tsx");
  const cartesian = read("lib/charts/compiler/cartesian.ts");
  const mobileModel = read("lib/charts/mobile-model.ts");
  const wordCloud = read("lib/charts/compiler/wordcloud.ts");
  const f2WordCloud = read("lib/charts/presets/f2-wordcloud.ts");
  const data = read("lib/charts/data.ts");
  const gallery = read("lib/charts/gallery.ts");
  const index = read("lib/charts/index.ts");

  for (const marker of ["MobileChartSpec", "MobileViewportSpec", "MobileGestureSpec"]) {
    assert.match(spec, new RegExp(marker));
  }

  assert.match(spec, /"wordcloud"/);
  for (const marker of ["mobileViewport", "mobileGestures", "mobilePixelRatio", "mobileSafeArea"]) {
    assert.match(scene, new RegExp(marker));
  }

  for (const attr of ["data-dx-f2-viewport", "data-dx-f2-gestures", "data-dx-f2-pixel-ratio", "data-dx-f2-safe-area", "data-dx-f2-word", "data-dx-f2-word-weight"]) {
    assert.match(frame, new RegExp(attr));
  }

  for (const marker of ["f2CompactChart", "normalizeMobileViewport", "mobileSceneMetadata", "DEFAULT_F2_GESTURES"]) {
    assert.match(mobileModel, new RegExp(marker));
  }

  for (const marker of ["compileWordCloud", "placeWord", "wordCloudTerm", "wordCloudWeight"]) {
    assert.match(wordCloud, new RegExp(marker));
  }

  for (const marker of ["F2WordCloudPreset", "f2WordCloud", "type: \"wordcloud\""]) {
    assert.match(f2WordCloud, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(cartesian, /mobileSceneMetadata/);
  assert.match(cartesian, /coordinateSceneMetadata/);
  assert.match(cartesian, /elements\.map\(\(element\) => \(\{ \.\.\.element, \.\.\.coordinateSceneMetadata\(spec\), \.\.\.mobileMetadata \}\)\)/);
  assert.match(data, /mobileChartTerms/);
  assert.match(gallery, /f2CompactChart\(\{/);
  assert.match(gallery, /f2WordCloud\(\{/);
  assert.match(gallery, /gestures: \[\{ type: "tap-tooltip" \}, \{ type: "pan-x" \}, \{ type: "pinch-zoom" \}\]/);
  assert.match(index, /f2CompactChart/);
  assert.match(index, /f2WordCloud/);
});

test("charts graph slice owns G6 and Graphin model metadata", () => {
  const spec = read("lib/charts/spec.ts");
  const scene = read("lib/charts/scene.ts");
  const frame = read("components/charts/chart-frame.tsx");
  const network = read("lib/charts/compiler/network.ts");
  const graphModel = read("lib/charts/compiler/graph-model.ts");
  const graphLayout = read("lib/charts/compiler/graph-layout.ts");
  const agentModel = read("lib/charts/agent-model.ts");
  const data = read("lib/charts/data.ts");
  const gallery = read("lib/charts/gallery.ts");

  for (const marker of ["GraphModelSpec", "GraphNodeSpec", "GraphEdgeSpec", "GraphComboSpec", "GraphLayoutSpec", "GraphBehaviorSpec", "GraphFocusSpec", "GraphRelationActivationSpec", "GraphPluginSpec"]) {
    assert.match(spec, new RegExp(marker));
  }

  for (const marker of ["graphNodeId", "graphEdgeId", "graphComboId", "graphLayout", "graphBehaviors", "graphPlugins", "graphFocusNodeId", "graphRelationState"]) {
    assert.match(scene, new RegExp(marker));
    assert.match(frame, new RegExp(`data-dx-${marker.replace(/[A-Z]/g, (value) => `-${value.toLowerCase()}`)}`));
  }

  assert.match(graphModel, /graphModelFromSpec/);
  assert.match(graphModel, /normalizeGraphModel/);
  assert.match(graphModel, /nodesFromEdges/);
  for (const marker of ["graphInteractionState", "focusedGraphNodeIds", "activatedGraphEdgeIds", "graphRelationState"]) {
    assert.match(graphModel, new RegExp(marker));
  }
  for (const marker of ["describeGraphBehavior", "describeGraphPlugin", "graphBehaviorState", "edgeKey"]) {
    assert.match(graphModel, new RegExp(marker));
  }
  for (const layout of ["circular", "grid", "radial", "dagre-lite", "combo-cluster"]) {
    assert.match(graphLayout, new RegExp(layout));
  }
  for (const marker of ["GraphLayoutEdge", "routeGraphEdge", "routeGraphEdges", "graphEdgeRoute"]) {
    assert.match(graphLayout, new RegExp(marker));
  }
  assert.match(network, /layoutGraph\(model/);
  assert.match(network, /graphModelFromSpec/);
  assert.match(network, /graphInteractionState\(model\)/);
  assert.match(network, /describeGraphBehavior/);
  assert.match(network, /routeGraphEdge\(edge, layout\)/);
  assert.match(network, /const graphEdgeRoute/);
  assert.match(network, /kind: "path"/);
  assert.match(network, /chart-graph-edge-active/);
  assert.match(network, /chart-graph-node-focused/);
  assert.match(network, /graphComboId/);
  assert.match(network, /graphEdgeId/);
  assert.match(network, /graphNodeId/);
  assert.match(data, /graphNodes/);
  assert.match(data, /graphCombos/);
  assert.match(data, /graphModelEdges/);
  assert.match(gallery, /layout: \{ type: "combo-cluster" \}/);
  assert.match(gallery, /layout: \{ type: "dagre-lite", rankDirection: "LR" \}/);
  assert.match(gallery, /focus: \{ nodeId: "G2"/);
  assert.match(gallery, /activation: \{ nodeId: "G2"/);
  assert.match(agentModel, /layout: \{ type: "radial" \}/);
  assert.match(gallery, /plugins: \[\{ type: "minimap" \}/);
});

test("charts X6 slice owns diagram ports terminals and routing metadata", () => {
  const spec = read("lib/charts/spec.ts");
  const scene = read("lib/charts/scene.ts");
  const frame = read("components/charts/chart-frame.tsx");
  const diagramLayout = read("lib/charts/compiler/diagram-layout.ts");
  const diagramModel = read("lib/charts/compiler/diagram-model.ts");
  const diagramScene = read("lib/charts/compiler/diagram-scene.ts");
  const network = read("lib/charts/compiler/network.ts");
  const data = read("lib/charts/data.ts");
  const gallery = read("lib/charts/gallery.ts");
  const index = read("lib/charts/index.ts");

  for (const marker of ["DiagramModelSpec", "DiagramNodeSpec", "DiagramPortSpec", "DiagramTerminalSpec", "DiagramEdgeSpec", "DiagramInteractionSpec"]) {
    assert.match(spec, new RegExp(marker));
  }
  assert.match(spec, /DiagramPortStateKind/);

  for (const marker of ["diagramNodeId", "diagramPortId", "diagramEdgeId", "diagramRouter", "diagramConnector", "diagramInteractions"]) {
    assert.match(scene, new RegExp(marker));
  }

  for (const attr of ["data-dx-x6-node-id", "data-dx-x6-port-id", "data-dx-x6-edge-id", "data-dx-x6-router", "data-dx-x6-connector", "data-dx-x6-interactions"]) {
    assert.match(frame, new RegExp(attr));
  }

  for (const marker of ["buildDiagramModel", "diagramModelToGraph", "diagramSceneMetadata"]) {
    assert.match(diagramModel, new RegExp(marker));
  }
  for (const marker of ["diagramPortStateMetadata", "connectedPortKeys", "state: diagramPortState"]) {
    assert.match(diagramModel, new RegExp(marker));
  }

  for (const marker of ["layoutDiagram", "routeDiagramEdge"]) {
    assert.match(diagramLayout, new RegExp(marker));
  }
  for (const marker of ["routeManhattanDiagramEdge", "connectRoundedPath", "connectJumpoverPath"]) {
    assert.match(diagramLayout, new RegExp(marker));
  }

  assert.match(diagramScene, /routeDiagramEdge/);
  assert.match(diagramScene, /diagramSceneMetadata/);
  assert.match(diagramScene, /diagramNodeId/);
  assert.match(diagramScene, /diagram-port-state/);
  assert.match(diagramScene, /diagram-port-group/);
  assert.match(diagramScene, /diagram-port-position/);
  assert.match(network, /compileDiagram\(spec, mark\)/);
  assert.match(data, /diagramWorkflow/);
  assert.match(data, /router: \{ name: "orth"/);
  assert.match(data, /connector: \{ name: "rounded"/);
  assert.match(gallery, /diagram: diagramWorkflow/);
  assert.match(index, /diagramModelToGraph/);
});

test("charts S2 slice owns semantic pivot table metadata", () => {
  const spec = read("lib/charts/spec.ts");
  const scene = read("lib/charts/scene.ts");
  const frame = read("components/charts/chart-frame.tsx");
  const spatial = read("lib/charts/compiler/spatial.ts");
  const tableModel = read("lib/charts/compiler/table-model.ts");
  const s2 = read("lib/charts/presets/s2.ts");
  const data = read("lib/charts/data.ts");
  const gallery = read("lib/charts/gallery.ts");
  const index = read("lib/charts/index.ts");

  for (const marker of ["TableSheetSpec", "TableFieldSpec", "TableMeasureSpec", "TableInteractionSpec", "TableTotalsSpec", "TableSortSpec", "TableDrillSpec", "TableFilterSpec"]) {
    assert.match(spec, new RegExp(marker));
  }

  for (const marker of ["tableCellId", "tableRowKey", "tableColumnKey", "tableValueField", "tableSection", "tableHierarchy", "tableInteractions", "tableTotals", "tableSortState", "tableDrillPath"]) {
    assert.match(scene, new RegExp(marker));
  }

  for (const attr of ["data-dx-s2-cell-id", "data-dx-s2-row-key", "data-dx-s2-column-key", "data-dx-s2-value-field", "data-dx-s2-section", "data-dx-s2-hierarchy", "data-dx-s2-interactions", "data-dx-s2-totals", "data-dx-s2-sort-state", "data-dx-s2-drill-path"]) {
    assert.match(frame, new RegExp(attr));
  }

  for (const marker of ["buildSemanticTable", "aggregateTableCells", "applyTableSort", "applyTableFilters", "compareHeaders", "filteredRows", "filterRows", "rowTotals", "columnTotals", "grandTotal", "hierarchyType", "interactions", "tableSortState", "tableDrillPath", "tableFilterState"]) {
    assert.match(tableModel, new RegExp(marker));
  }

  assert.match(spatial, /buildSemanticTable/);
  assert.match(spatial, /tableCellId/);
  assert.match(spatial, /chart-s2-value-label/);
  for (const marker of ["S2PivotSheetPreset", "s2PivotSheet", "hierarchyType", "totals", "interactions", "sort", "drillDown", "filterRows"]) {
    assert.match(s2, new RegExp(marker));
  }
  for (const marker of ['task: "table"', 'family: "S2"', 'type: "pivot"', 'type: "brush-selection"', 'type: "sort-header"']) {
    assert.match(s2, new RegExp(marker));
  }

  assert.match(data, /pivotReadinessRows/);
  assert.match(data, /pivotReadinessFilters/);
  assert.match(gallery, /chart\("pivot-readiness"/);
  assert.match(gallery, /s2PivotSheet\(\{/);
  assert.match(gallery, /sort: \[\{ target: "row"/);
  assert.match(gallery, /drillDown: \[\{ field: "family"/);
  assert.match(gallery, /packageName: "@antv\/s2"/);
  assert.match(index, /s2PivotSheet/);
});

test("charts L7 slice owns geo layer and viewport metadata", () => {
  const spec = read("lib/charts/spec.ts");
  const scene = read("lib/charts/scene.ts");
  const frame = read("components/charts/chart-frame.tsx");
  const spatial = read("lib/charts/compiler/spatial.ts");
  const geoModel = read("lib/charts/compiler/geo-model.ts");
  const l7 = read("lib/charts/presets/l7.ts");
  const data = read("lib/charts/data.ts");
  const gallery = read("lib/charts/gallery.ts");
  const index = read("lib/charts/index.ts");

  for (const marker of ["GeoMapSpec", "GeoLayerSpec", "GeoRegionSpec", "GeoViewportSpec", "GeoBasemapSpec", "GeoInteractionSpec", "GeoLayerRuntimeSpec", "GeoLegendSpec"]) {
    assert.match(spec, new RegExp(marker));
  }

  for (const marker of ["mapLayerId", "mapLayerType", "mapFeatureId", "mapProjection", "mapViewport", "mapInteractions", "mapLayerZoom", "mapLayerBlend", "mapLegend"]) {
    assert.match(scene, new RegExp(marker));
  }

  for (const attr of ["data-dx-l7-layer-id", "data-dx-l7-layer-type", "data-dx-l7-feature-id", "data-dx-l7-projection", "data-dx-l7-viewport", "data-dx-l7-interactions", "data-dx-l7-layer-zoom", "data-dx-l7-blend", "data-dx-l7-legend"]) {
    assert.match(frame, new RegExp(attr));
  }

  for (const marker of ["buildGeoModel", "normalizeGeoLayers", "filterGeoLayersByViewport", "fitViewport", "projectGeoPoint", "mercator-lite"]) {
    assert.match(geoModel, new RegExp(marker));
  }

  assert.match(spatial, /buildGeoModel/);
  assert.match(spatial, /mapLayerId/);
  assert.match(spatial, /mapLegend\(model, valueDomain\)/);
  assert.match(spatial, /chart-map-grid/);
  assert.match(spatial, /chart-map-region/);
  for (const marker of ["L7CompositeLayerPreset", "l7CompositeLayers", "type: \"heatmap\"", "type: \"bubble\"", "zIndex"]) {
    assert.match(l7, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(data, /geoUsagePoints/);
  assert.match(data, /geoUsageRegions/);
  assert.match(data, /geoUsageRegionShapes/);
  assert.match(gallery, /l7CompositeLayers\(\{/);
  assert.match(gallery, /l7plotChoropleth\(\{/);
  assert.match(gallery, /regions: geoUsageRegionShapes/);
  assert.match(index, /l7CompositeLayers/);
  assert.match(index, /l7plotChoropleth/);
});

test("charts AVA and GPTVis slice owns chart advice scoring", () => {
  const spec = read("lib/charts/spec.ts");
  const scene = read("lib/charts/scene.ts");
  const frame = read("components/charts/chart-frame.tsx");
  const cartesianMarks = read("lib/charts/compiler/cartesian-marks.ts");
  const adviceModel = read("lib/charts/advice-model.ts");
  const ava = read("lib/charts/presets/ava.ts");
  const gptvis = read("lib/charts/presets/gptvis.ts");
  const data = read("lib/charts/data.ts");
  const gallery = read("lib/charts/gallery.ts");
  const index = read("lib/charts/index.ts");

  for (const marker of ["ChartAdviceIntentSpec", "ChartAdviceCandidateSpec", "ChartAdviceRecommendationSpec", "ChartAdviceModelSpec"]) {
    assert.match(spec, new RegExp(marker));
  }

  for (const marker of ["adviceRuleId", "adviceReason", "adviceConfidence"]) {
    assert.match(scene, new RegExp(marker));
  }

  for (const attr of ["data-dx-ava-rule-id", "data-dx-ava-reason", "data-dx-ava-confidence"]) {
    assert.match(frame, new RegExp(attr));
  }

  for (const marker of ["recommendCharts", "scoreChartCandidates", "explainRecommendation", "materializeAdviceRows", "DEFAULT_CHART_CANDIDATES"]) {
    assert.match(adviceModel, new RegExp(marker));
  }

  assert.match(cartesianMarks, /adviceMetadata/);
  assert.match(data, /chartAdviceIntent/);
  assert.match(data, /materializeAdviceRows\(recommendCharts\(chartAdviceIntent\)/);
  assert.match(ava, /advice: \{/);
  assert.match(gptvis, /advice: \{/);
  assert.match(gallery, /maxRecommendations: 5/);
  assert.match(index, /recommendCharts/);
});

test("charts package adapters own L7Plot Ant Design Charts MCP and ChartSkills boundaries", () => {
  const spec = read("lib/charts/spec.ts");
  const l7plot = read("lib/charts/presets/l7plot.ts");
  const antDesignCharts = read("lib/charts/presets/ant-design-charts.ts");
  const agentModel = read("lib/charts/agent-model.ts");
  const data = read("lib/charts/data.ts");
  const gallery = read("lib/charts/gallery.ts");
  const index = read("lib/charts/index.ts");

  for (const marker of ["AgentChartRequestSpec", "AgentChartWorkflowSpec", "ChartSkillSpec"]) {
    assert.match(spec, new RegExp(marker));
  }

  for (const marker of ["L7PlotMapPreset", "l7plotMap", "L7PlotChoroplethPreset", "l7plotChoropleth", "projection: config.projection ?? \"mercator-lite\"", "layers: [", "regions: config.regions", "type: \"region\"", "regionField"]) {
    assert.match(l7plot, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const marker of ["AntDesignColumnPreset", "antDesignColumn", "AntDesignProgressPreset", "antDesignProgress", "family: \"AntDesignCharts\"", "type: \"bar\"", "type: \"gauge\""]) {
    assert.match(antDesignCharts, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const marker of ["createMcpRequestFlow", "chartSkillsGraph", "materializeAgentFlowRows", "buildSkillGraph"]) {
    assert.match(agentModel, new RegExp(marker));
  }

  assert.match(data, /agentChartRequest/);
  assert.match(data, /chartSkillCatalog/);
  assert.match(gallery, /l7plotMap\(\{/);
  assert.match(gallery, /l7plotChoropleth\(\{/);
  assert.match(gallery, /antDesignColumn\(\{/);
  assert.match(gallery, /antDesignProgress\(\{/);
  assert.match(gallery, /createMcpRequestFlow\(\{/);
  assert.match(gallery, /chartSkillsGraph\(\{/);
  assert.match(index, /l7plotMap/);
  assert.match(index, /l7plotChoropleth/);
  assert.match(index, /antDesignColumn/);
  assert.match(index, /createMcpRequestFlow/);
  assert.match(index, /chartSkillsGraph/);
});

test("charts AVA and GPTVis adapters materialize prompt recommendations", () => {
  const spec = read("lib/charts/spec.ts");
  const promptModel = read("lib/charts/prompt-model.ts");
  const ava = read("lib/charts/presets/ava.ts");
  const gptvis = read("lib/charts/presets/gptvis.ts");
  const chartRouter = [
    read("lib/charts/chart-router.ts"),
    read("lib/charts/router/catalog-item.ts"),
    read("lib/charts/router/chart-tools.ts"),
    read("lib/charts/router/scoring.ts"),
    read("lib/charts/router/spec-builder.ts"),
    read("lib/charts/router/spec-fields.ts"),
    read("lib/charts/router/spec-summaries.ts"),
    read("lib/charts/router/route-rows.ts"),
    read("lib/charts/router/syntax.ts"),
  ].join("\n");
  const data = read("lib/charts/data.ts");
  const gallery = read("lib/charts/gallery.ts");
  const index = read("lib/charts/index.ts");

  for (const marker of ["ChartPromptFieldSpec", "ChartPromptRequestSpec", "PromptChartMaterializationSpec", "GPTVisChartType", "GptVisChartType", "ChartToolSpec", "ChartToolRouteSpec", "family: \"GPTVis\""]) {
    assert.match(spec, new RegExp(marker));
  }

  for (const marker of ["inferPromptAdviceIntent", "materializePromptRecommendations", "promptRecommendationRows"]) {
    assert.match(promptModel, new RegExp(marker));
  }

  for (const marker of ["GPT_VIS_CHART_TOOLS", "rankChartTools", "routeChartRequest", "chartCatalogItemFromRoute", "chartSpecFromToolRecommendation", "gptVisSyntaxFromRoute", "materializeToolRouteRows"]) {
    assert.match(chartRouter, new RegExp(marker));
  }

  for (const marker of ['chartType: "line"', 'chartType: "boxplot"', 'chartType: "waterfall"', 'chartType: "network-graph"', 'chartType: "chord"', "supported: true"]) {
    assert.match(chartRouter, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  const gptVisToolIds = [...chartRouter.matchAll(/id: "(gptvis-[^"]+)"/g)].map((match) => match[1]).sort();
  assert.deepEqual(gptVisToolIds, [
    "gptvis-area",
    "gptvis-bar",
    "gptvis-boxplot",
    "gptvis-chord",
    "gptvis-column",
    "gptvis-funnel",
    "gptvis-histogram",
    "gptvis-line",
    "gptvis-network-graph",
    "gptvis-pie",
    "gptvis-radar",
    "gptvis-sankey",
    "gptvis-scatter",
    "gptvis-table",
    "gptvis-treemap",
    "gptvis-waterfall",
    "gptvis-word-cloud",
  ].sort());

  for (const marker of [
    "rankChartTools(request).slice(0, request.maxRecommendations ?? 5)",
    "const selected = recommendations[0] ?? fallbackRecommendation(request)",
    "const syntax = gptVisSyntaxFromRoute(routeWithoutSyntax, chart)",
    "catalogItem: chartCatalogItemFromRoute(routedChart)",
    "spec: chart",
    "case \"boxplot\"",
    "case \"waterfall\"",
    "case \"chord\"",
    "case \"network-graph\"",
    "if (request.sampleRows && request.sampleRows.length > 0) return request.sampleRows",
    "fields.unshift({ name: \"category\", role: \"dimension\", label: \"Category\" })",
    "if (fieldSpec.role === \"time\") return index + 1",
    "route.selected.chartType === \"network-graph\" ? [\"source\", \"target\", \"value\"] : chartFieldNames(chart)",
    "selected: recommendation.toolId === route.selected.toolId",
  ]) {
    assert.match(chartRouter, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const marker of ["AvaAdviceChartPreset", "avaAdviceChart", "family: \"AVA\"", "recommendCharts"]) {
    assert.match(ava, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const marker of ["GptVisPromptChartPreset", "gptVisPromptChart", "family: \"GPTVis\"", "routeChartRequest", "materializeToolRouteRows", "router: route"]) {
    assert.match(gptvis, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(data, /gptVisPromptRequest/);
  assert.match(data, /sampleRows: packageReadiness/);
  assert.match(gallery, /gptVisPromptChart\(\{/);
  assert.match(gallery, /avaAdviceChart\(\{/);
  assert.match(index, /inferPromptAdviceIntent/);
  assert.match(index, /routeChartRequest/);
  assert.match(index, /GPT_VIS_CHART_TOOLS/);
  assert.match(index, /gptVisPromptChart/);
  assert.match(index, /avaAdviceChart/);
});

test("charts Ant Design Charts adapters cover plots and graphs subpackages", () => {
  const spec = read("lib/charts/spec.ts");
  const plots = read("lib/charts/presets/ant-design-plots.ts");
  const plotHierarchy = read("lib/charts/presets/ant-design-plots-hierarchy.ts");
  const graphs = read("lib/charts/presets/ant-design-graphs.ts");
  const gallery = read("lib/charts/gallery.ts");
  const index = read("lib/charts/index.ts");

  for (const family of ["AntDesignPlots", "AntDesignGraphs"]) {
    assert.match(spec, new RegExp(family));
    assert.match(gallery, new RegExp(family));
  }

  for (const marker of [
    "AntDesignDualAxesPreset",
    "antDesignDualAxes",
    "AntDesignHeatmapPreset",
    "antDesignHeatmap",
    "AntDesignBulletPreset",
    "antDesignBullet",
    "AntDesignGaugePreset",
    "antDesignGauge",
    "type: \"bar\"",
    "type: \"line\"",
    "type: \"heatmap\"",
    "type: \"bullet\"",
    "type: \"gauge\"",
  ]) {
    assert.match(plots, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const marker of [
    "AntDesignTreemapPreset",
    "antDesignTreemap",
    "AntDesignSunburstPreset",
    "antDesignSunburst",
    "AntDesignSankeyPreset",
    "antDesignSankey",
    "g2plotTreemap",
    "g2plotSunburst",
    "g2plotSankey",
    "family: \"AntDesignPlots\"",
  ]) {
    assert.match(plotHierarchy, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const marker of ["AntDesignGraphPreset", "antDesignGraph", "GraphModelSpec", "GraphBehaviorSpec", "GraphPluginSpec", "layout: config.layout", "defaultGraphBehaviors", "defaultGraphPlugins"]) {
    assert.match(graphs, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(gallery, /antDesignDualAxes\(\{/);
  assert.match(gallery, /antDesignHeatmap\(\{/);
  assert.match(gallery, /antDesignBullet\(\{/);
  assert.match(gallery, /antDesignGauge\(\{/);
  assert.match(gallery, /antDesignTreemap\(\{/);
  assert.match(gallery, /antDesignSunburst\(\{/);
  assert.match(gallery, /antDesignSankey\(\{/);
  assert.match(gallery, /antDesignGraph\(\{/);
  assert.match(gallery, /packageName: "@ant-design\/plots"/);
  assert.match(gallery, /packageName: "@ant-design\/graphs"/);
  assert.match(index, /antDesignDualAxes/);
  assert.match(index, /antDesignTreemap/);
  assert.match(index, /antDesignGraph/);
});

test("charts catalog has unique source-owned slugs and transform-backed presets", () => {
  const gallery = read("lib/charts/gallery.ts");
  const g2plot = read("lib/charts/presets/g2plot.ts");
  const g2plotBox = read("lib/charts/presets/g2plot-box.ts");
  const g2plotChord = read("lib/charts/presets/g2plot-chord.ts");
  const g2plotHierarchy = read("lib/charts/presets/g2plot-hierarchy.ts");
  const g2plotFunnel = read("lib/charts/presets/g2plot-funnel.ts");
  const g2plotMini = read("lib/charts/presets/g2plot-mini.ts");
  const g2plotRadial = read("lib/charts/presets/g2plot-radial.ts");
  const g2plotWaterfall = read("lib/charts/presets/g2plot-waterfall.ts");
  const cartesian = read("lib/charts/compiler/cartesian.ts");
  const chord = read("lib/charts/compiler/chord.ts");
  const funnel = read("lib/charts/compiler/funnel.ts");
  const waterfall = read("lib/charts/compiler/waterfall.ts");
  const shared = read("lib/charts/compiler/shared.ts");
  const data = read("lib/charts/data.ts");
  const index = read("lib/charts/index.ts");
  const slugMatches = [...gallery.matchAll(/chart\("([^"]+)"/g)].map((match) => match[1]);
  const specIdMatches = [...gallery.matchAll(/id: "([^"]+)"/g)].map((match) => match[1]);

  assert.equal(new Set(slugMatches).size, slugMatches.length);
  assert.ok(slugMatches.includes("target-builds"));
  assert.ok(slugMatches.includes("histogram-quality"));
  assert.ok(slugMatches.includes("boxplot-package-spread"));
  assert.ok(slugMatches.includes("bullet-readiness"));
  assert.ok(slugMatches.includes("gauge-source-readiness"));
  assert.ok(slugMatches.includes("progress-source-readiness"));
  assert.ok(slugMatches.includes("ring-progress-source-coverage"));
  assert.ok(slugMatches.includes("rose-product-mix"));
  assert.ok(slugMatches.includes("ant-design-plots-heatmap"));
  assert.ok(slugMatches.includes("ant-design-plots-bullet"));
  assert.ok(slugMatches.includes("ant-design-plots-gauge"));
  assert.ok(slugMatches.includes("ant-design-chart-progress"));
  assert.ok(slugMatches.includes("sunburst-ecosystem"));
  assert.ok(slugMatches.includes("sankey-package-relationships"));
  assert.ok(slugMatches.includes("chord-package-flows"));
  assert.ok(slugMatches.includes("pivot-readiness"));
  assert.ok(slugMatches.includes("stacked-release-work"));
  assert.ok(slugMatches.includes("normalized-release-share"));
  assert.ok(slugMatches.includes("dodged-package-modes"));
  assert.ok(slugMatches.includes("faceted-runtime-proof"));
  assert.ok(slugMatches.includes("tiny-receipt-trend"));
  assert.ok(slugMatches.includes("funnel-chart-readiness"));
  assert.ok(slugMatches.includes("waterfall-release-delta"));
  assert.match(gallery, /type: "rule"/);
  assert.match(gallery, /groupBy: \[\]/);
  assert.match(gallery, /g2plotCartesian\(\{/);
  assert.match(gallery, /g2plotHistogram\(\{/);
  assert.match(gallery, /g2plotFacet\(\{/);
  assert.match(gallery, /g2plotChord\(\{/);
  assert.match(gallery, /g2plotTreemap\(\{/);
  assert.match(gallery, /g2plotSunburst\(\{/);
  assert.match(gallery, /g2plotSankey\(\{/);
  assert.match(gallery, /g2plotBox\(\{/);
  assert.match(gallery, /g2plotFunnel\(\{/);
  assert.match(gallery, /g2plotWaterfall\(\{/);
  assert.match(gallery, /g2plotHeatmap\(\{/);
  assert.match(gallery, /g2plotBullet\(\{/);
  assert.match(gallery, /g2plotGauge\(\{/);
  assert.match(gallery, /g2plotProgress\(\{/);
  assert.match(gallery, /g2plotTiny\(\{/);
  assert.match(gallery, /g2plotRingProgress\(\{/);
  assert.match(gallery, /g2plotPie\(\{/);
  assert.match(gallery, /g2plotRadar\(\{/);
  assert.match(gallery, /g2plotRose\(\{/);
  assert.match(gallery, /g2-view-grammar/);
  assert.match(gallery, /Source-owned preset adapters and config lowering/);
  assert.match(g2plot, /export type G2PlotCartesianPreset/);
  assert.match(g2plot, /export function g2plotCartesian/);
  assert.match(g2plot, /export function g2plotHistogram/);
  assert.match(g2plot, /export function g2plotFacet/);
  assert.match(g2plot, /export type G2PlotHeatmapPreset/);
  assert.match(g2plot, /export type G2PlotBulletPreset/);
  assert.match(g2plot, /export function g2plotHeatmap/);
  assert.match(g2plot, /export function g2plotBullet/);
  assert.match(g2plot, /type: "heatmap"/);
  assert.match(g2plot, /type: "bullet"/);
  assert.match(g2plot, /\{ type: "filter" as const, field: config\.binField/);
  assert.match(g2plot, /\{ type: "bin", field: config\.binField/);
  assert.match(g2plot, /\{ type: "stackY", x: config\.xField, y: config\.yField, series: config\.seriesField \}/);
  assert.match(g2plot, /\{ type: "normalizeY", x: config\.xField, y: config\.yField, series: config\.seriesField \}/);
  assert.match(g2plot, /\{ type: "dodgeX", x: config\.xField, series: config\.seriesField \}/);
  assert.match(g2plot, /coordinate: config\.preset === "bar" \? \{ type: "transpose" \} : undefined/);
  assert.match(g2plot, /composition: \{ type: "facet", field: config\.field/);
  assert.match(g2plotChord, /export type G2PlotChordPreset/);
  assert.match(g2plotChord, /export function g2plotChord/);
  assert.match(g2plotChord, /type: "chord"/);
  assert.match(g2plotChord, /source: field\(config\.sourceField, "nominal"/);
  assert.match(g2plotChord, /target: field\(config\.targetField, "nominal"/);
  assert.match(g2plotChord, /size: field\(config\.weightField, "quantitative"/);
  assert.match(g2plotHierarchy, /export type G2PlotTreemapPreset/);
  assert.match(g2plotHierarchy, /export function g2plotTreemap/);
  assert.match(g2plotHierarchy, /export type G2PlotSunburstPreset/);
  assert.match(g2plotHierarchy, /export function g2plotSunburst/);
  assert.match(g2plotHierarchy, /export type G2PlotSankeyPreset/);
  assert.match(g2plotHierarchy, /export function g2plotSankey/);
  assert.match(g2plotHierarchy, /type: "treemap"/);
  assert.match(g2plotHierarchy, /type: "sunburst"/);
  assert.match(g2plotHierarchy, /type: "sankey"/);
  assert.match(g2plotHierarchy, /source: field\(config\.sourceField, "nominal"/);
  assert.match(g2plotHierarchy, /parent: field\(config\.parentField, "nominal"/);
  assert.match(g2plotBox, /export type G2PlotBoxPreset/);
  assert.match(g2plotBox, /export function g2plotBox/);
  assert.match(g2plotBox, /type: "boxplot"/);
  assert.match(g2plotBox, /low: field\(config\.lowField/);
  assert.match(g2plotBox, /high: field\(config\.highField/);
  assert.match(g2plotFunnel, /export type G2PlotFunnelPreset/);
  assert.match(g2plotFunnel, /export function g2plotFunnel/);
  assert.match(g2plotFunnel, /type: "funnel"/);
  assert.match(funnel, /export function compileFunnel/);
  assert.match(funnel, /trapezoidPoints/);
  assert.match(funnel, /chart-mark-funnel/);
  assert.match(g2plotWaterfall, /export type G2PlotWaterfallPreset/);
  assert.match(g2plotWaterfall, /export function g2plotWaterfall/);
  assert.match(g2plotWaterfall, /type: "waterfall"/);
  assert.match(waterfall, /export function compileWaterfall/);
  assert.match(waterfall, /cumulative/);
  assert.match(waterfall, /chart-waterfall-connector/);
  assert.match(waterfall, /chart-mark-waterfall-negative/);
  assert.match(chord, /export function compileChord/);
  assert.match(chord, /aggregateChordLinks/);
  assert.match(chord, /annularArcPath/);
  assert.match(chord, /ribbonPath/);
  assert.match(chord, /chart-mark-chord-ribbon/);
  assert.match(chord, /chordSource/);
  assert.match(chord, /chordTarget/);
  assert.match(chord, /chordNodeId/);
  assert.match(g2plotMini, /export type G2PlotTinyPreset/);
  assert.match(g2plotMini, /export type G2PlotRingProgressPreset/);
  assert.match(g2plotMini, /export type G2PlotGaugePreset/);
  assert.match(g2plotMini, /export type G2PlotProgressPreset/);
  assert.match(g2plotMini, /export function g2plotTiny/);
  assert.match(g2plotMini, /export function g2plotRingProgress/);
  assert.match(g2plotMini, /export function g2plotGauge/);
  assert.match(g2plotMini, /export function g2plotProgress/);
  assert.match(g2plotMini, /axes: \[\]/);
  assert.match(g2plotMini, /coordinate: \{ type: "theta", innerRadius: 0\.64 \}/);
  assert.match(g2plotMini, /type: "gauge"/);
  assert.match(g2plotRadial, /export type G2PlotPiePreset/);
  assert.match(g2plotRadial, /export type G2PlotRadarPreset/);
  assert.match(g2plotRadial, /export type G2PlotRosePreset/);
  assert.match(g2plotRadial, /export function g2plotPie/);
  assert.match(g2plotRadial, /export function g2plotRadar/);
  assert.match(g2plotRadial, /export function g2plotRose/);
  assert.match(g2plotRadial, /type: "pie"/);
  assert.match(g2plotRadial, /type: "radar"/);
  assert.match(g2plotRadial, /coordinate: \{ type: "polar"/);
  assert.match(g2plotRadial, /color: field\(config\.colorField, "nominal"\)/);
  assert.match(cartesian, /spec\.axes\?\.length !== 0/);
  assert.match(shared, /export function coordinateSceneMetadata/);
  assert.match(shared, /elements: elements\.map\(\(element\) => \(\{ \.\.\.metadata, \.\.\.element \}\)\)/);
  assert.match(data, /funnelConversionStages/);
  assert.match(data, /waterfallReleaseDeltas/);
  assert.match(data, /packageRelationFlows/);
  assert.match(index, /g2plotBox/);
  assert.match(index, /g2plotChord/);
  assert.match(index, /g2plotTreemap/);
  assert.match(index, /g2plotSunburst/);
  assert.match(index, /g2plotSankey/);
  assert.match(index, /g2plotFunnel/);
  assert.match(index, /g2plotPie/);
  assert.match(index, /g2plotRadar/);
  assert.match(index, /g2plotWaterfall/);
  assert.match(index, /CoordinateSpec/);
  assert.match(index, /ViewCompositionSpec/);
  assert.match(index, /g2plotCartesian/);
  assert.match(index, /g2plotTiny/);
  assert.match(index, /g2plotRingProgress/);
  assert.ok(new Set(specIdMatches).size > 20);
});

test("charts geometry preserves full annular rings for sunburst scenes", () => {
  const geometry = read("lib/charts/geometry.ts");
  const hierarchy = read("lib/charts/compiler/hierarchy.ts");
  const spec = read("lib/charts/spec.ts");
  const data = read("lib/charts/data.ts");
  const gallery = read("lib/charts/gallery.ts");
  const g2plotHierarchy = read("lib/charts/presets/g2plot-hierarchy.ts");

  assert.match(geometry, /FULL_CIRCLE_EPSILON/);
  assert.match(geometry, /annularOuterArc/);
  assert.match(geometry, /annularInnerArc/);
  assert.match(geometry, /sweep >= TAU - FULL_CIRCLE_EPSILON/);
  assert.match(spec, /\| "parent"/);
  assert.match(data, /parent: "AntV ecosystem"/);
  assert.match(gallery, /g2plotSunburst\(\{/);
  assert.match(g2plotHierarchy, /parent: field\(config\.parentField, "nominal"\)/);
  assert.match(hierarchy, /sliceDiceTreemapTiles/);
  assert.match(hierarchy, /childrenByParent/);
  assert.match(hierarchy, /addSunburstPartition/);
  assert.doesNotMatch(hierarchy, /\* 1\.9/);
  assert.doesNotMatch(hierarchy, /filter\(\(datum\) => toLabel\(readField\(datum, level\.field\)\) === levelName\)/);
});

test("charts UI uses stable slug helpers instead of catalog index coupling", () => {
  for (const file of listFiles("components/charts")) {
    if (!file.endsWith(".tsx")) continue;
    assert.doesNotMatch(read(file), /chartCatalog\[[0-9]+\]|chartCatalog\.slice\(/, `${file} should use named chart slugs`);
  }
});

test("charts runtime and security headers support keyboard-safe interaction", () => {
  const runtime = read("public/chart-runtime.js");
  assert.match(runtime, /keydown/);
  assert.match(runtime, /aria-pressed/);
  assert.match(runtime, /data-selected-count/);
  assert.match(runtime, /clampPosition/);

  const vercel = read("vercel.json");
  assert.match(vercel, /script-src 'self'/);
  assert.doesNotMatch(vercel, /script-src 'none'/);
});
