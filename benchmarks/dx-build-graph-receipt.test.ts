const assert = require("node:assert/strict");
const { execFileSync } = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const {
  createDxBuildGraphConsumerSnapshot,
  readDxBuildGraphReceipt,
  scanDxBuildGraph,
  writeDxBuildGraphReceipt,
} = require("../tools/build-graph");

const fixtureRoot = path.join(
  __dirname,
  "fixtures",
  "build-graph",
  "minimal-app",
);

test("build graph receipt models DX launch sources and invalidates dependents", () => {
  const report = scanDxBuildGraph(fixtureRoot, {
    changedPaths: ["styles/generated.css"],
  });

  assert.equal(report.schema, "dx.build.graph");
  assert.equal(report.names.buildGraph, "dx.build.graph");
  assert.equal(report.names.wwwModuleGraph, "dx.www.moduleGraph");
  assert.equal(report.names.forgeSourceGraph, "dx.forge.sourceGraph");
  assert.equal(report.positioning.turbopackPublicDependency, false);

  const kinds = new Set(report.graph.nodes.map((node) => node.kind));
  for (const kind of [
    "tsx-route",
    "tsx-component",
    "dx-style-css",
    "forge-surface",
    "dx-check-receipt",
    "public-asset",
    "deploy-output",
  ]) {
    assert.ok(kinds.has(kind), `expected graph node kind ${kind}`);
  }

  const route = nodeByPath(report, "app/page.tsx");
  const component = nodeByPath(report, "components/LaunchPanel.tsx");
  const css = nodeByPath(report, "styles/generated.css");
  const logo = nodeByPath(report, "public/logo.svg");

  assert.equal(route.kind, "tsx-route");
  assert.equal(component.kind, "tsx-component");
  assert.equal(css.kind, "dx-style-css");
  assert.equal(logo.kind, "public-asset");
  assert.ok(hasEdge(report, route.id, component.id, "imports"));
  assert.ok(hasEdge(report, component.id, css.id, "imports"));
  assert.ok(hasEdge(report, css.id, logo.id, "imports"));

  assert.deepEqual(report.invalidation.changedNodeIds, [css.id]);
  assert.deepEqual(report.invalidation.rebuildNodeIds, [
    css.id,
    component.id,
    route.id,
  ]);
  for (const affectedNodeId of [
    css.id,
    component.id,
    route.id,
    "forge:ui/launch-panel#dashboard",
    "dx-check-receipt:.dx/receipts/check/latest.json",
    "deploy-output:.dx/deploy/vercel-plan.json",
  ]) {
    assert.ok(
      report.invalidation.affectedNodeIds.includes(affectedNodeId),
      `expected invalidation to include ${affectedNodeId}`,
    );
  }

  assert.ok(
    report.provenance.some(
      (entry) =>
        entry.name === "Turbopack source study" &&
        entry.license === "MIT" &&
        entry.copiedCode === false,
    ),
  );

  const outputDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-build-graph-"));
  const receiptPath = path.join(outputDir, "graph-latest.json");
  const written = writeDxBuildGraphReceipt(fixtureRoot, receiptPath, {
    changedPaths: ["styles/generated.css"],
  });

  assert.equal(written.receiptPath, receiptPath);
  assert.equal(JSON.parse(fs.readFileSync(receiptPath, "utf8")).schema, "dx.build.graph");
});

test("build graph connects CSS url assets and invalidates dependent routes", () => {
  const report = scanDxBuildGraph(fixtureRoot, {
    changedPaths: ["public/logo.svg"],
  });

  const route = nodeByPath(report, "app/page.tsx");
  const component = nodeByPath(report, "components/LaunchPanel.tsx");
  const css = nodeByPath(report, "styles/generated.css");
  const logo = nodeByPath(report, "public/logo.svg");

  assert.ok(hasEdge(report, css.id, logo.id, "imports"));
  assert.deepEqual(report.invalidation.changedNodeIds, [logo.id]);
  assert.ok(report.invalidation.affectedNodeIds.includes(css.id));
  assert.ok(report.invalidation.affectedNodeIds.includes(component.id));
  assert.ok(report.invalidation.affectedNodeIds.includes(route.id));
  assert.deepEqual(report.invalidation.rebuildNodeIds, [
    css.id,
    component.id,
    route.id,
  ]);
});

test("build graph connects formatted CSS URL function assets", () => {
  const projectRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-build-graph-css-url-"));
  fs.mkdirSync(path.join(projectRoot, "app"), { recursive: true });
  fs.mkdirSync(path.join(projectRoot, "components"), { recursive: true });
  fs.mkdirSync(path.join(projectRoot, "styles"), { recursive: true });
  fs.mkdirSync(path.join(projectRoot, "public", "icons"), { recursive: true });
  fs.writeFileSync(
    path.join(projectRoot, "app", "page.tsx"),
    `import { LaunchPanel } from "../components/LaunchPanel";

export default function Page() {
  return <LaunchPanel />;
}
`,
  );
  fs.writeFileSync(
    path.join(projectRoot, "components", "LaunchPanel.tsx"),
    `import "../styles/generated.css";

export function LaunchPanel() {
  return <section className="hero">Launch</section>;
}
`,
  );
  fs.writeFileSync(
    path.join(projectRoot, "styles", "generated.css"),
    `.hero {
  background-image: URL ( "/icons/mark.svg?v=1#sprite" );
}
`,
  );
  fs.writeFileSync(
    path.join(projectRoot, "public", "icons", "mark.svg"),
    "<svg />",
  );

  const report = scanDxBuildGraph(projectRoot, {
    changedPaths: ["public/icons/mark.svg"],
  });

  const route = nodeByPath(report, "app/page.tsx");
  const component = nodeByPath(report, "components/LaunchPanel.tsx");
  const css = nodeByPath(report, "styles/generated.css");
  const mark = nodeByPath(report, "public/icons/mark.svg");

  assert.ok(hasEdge(report, css.id, mark.id, "imports"));
  assert.deepEqual(report.invalidation.changedNodeIds, [mark.id]);
  assert.ok(report.invalidation.affectedNodeIds.includes(css.id));
  assert.ok(report.invalidation.affectedNodeIds.includes(component.id));
  assert.ok(report.invalidation.affectedNodeIds.includes(route.id));
  assert.deepEqual(report.invalidation.rebuildNodeIds, [
    css.id,
    component.id,
    route.id,
  ]);
});

test("build graph resolves Next-familiar @ imports without node_modules", () => {
  const projectRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-build-graph-at-alias-"));
  fs.mkdirSync(path.join(projectRoot, "app"), { recursive: true });
  fs.mkdirSync(path.join(projectRoot, "components"), { recursive: true });
  fs.mkdirSync(path.join(projectRoot, "styles"), { recursive: true });
  fs.writeFileSync(
    path.join(projectRoot, "app", "page.tsx"),
    `import { LaunchPanel } from "@/components/LaunchPanel";

export default function Page() {
  return <LaunchPanel />;
}
`,
  );
  fs.writeFileSync(
    path.join(projectRoot, "components", "LaunchPanel.tsx"),
    `import "@/styles/generated.css";

export function LaunchPanel() {
  return <section className="hero">Launch</section>;
}
`,
  );
  fs.writeFileSync(
    path.join(projectRoot, "styles", "generated.css"),
    `.hero { display: grid; }
`,
  );

  const report = scanDxBuildGraph(projectRoot, {
    changedPaths: ["styles/generated.css"],
  });

  const route = nodeByPath(report, "app/page.tsx");
  const component = nodeByPath(report, "components/LaunchPanel.tsx");
  const css = nodeByPath(report, "styles/generated.css");

  assert.ok(hasEdge(report, route.id, component.id, "imports"));
  assert.ok(hasEdge(report, component.id, css.id, "imports"));
  assert.deepEqual(report.invalidation.changedNodeIds, [css.id]);
  assert.deepEqual(report.invalidation.rebuildNodeIds, [
    css.id,
    component.id,
    route.id,
  ]);
  assert.equal(fs.existsSync(path.join(projectRoot, "node_modules")), false);
});

test("build graph links source support modules under lib server and src", () => {
  const projectRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-build-graph-source-modules-"));
  fs.mkdirSync(path.join(projectRoot, "app"), { recursive: true });
  fs.mkdirSync(path.join(projectRoot, "components"), { recursive: true });
  fs.mkdirSync(path.join(projectRoot, "lib"), { recursive: true });
  fs.mkdirSync(path.join(projectRoot, "server"), { recursive: true });
  fs.mkdirSync(path.join(projectRoot, "src", "domain"), { recursive: true });
  fs.writeFileSync(
    path.join(projectRoot, "app", "page.tsx"),
    `import { LaunchPanel } from "@/components/LaunchPanel";

export default function Page() {
  return <LaunchPanel />;
}
`,
  );
  fs.writeFileSync(
    path.join(projectRoot, "components", "LaunchPanel.tsx"),
    `import { formatLaunchTitle } from "@/lib/launch-copy";
import { loadLaunchMetrics } from "@/server/loaders";

export function LaunchPanel() {
  return <section>{formatLaunchTitle(loadLaunchMetrics().status)}</section>;
}
`,
  );
  fs.writeFileSync(
    path.join(projectRoot, "lib", "launch-copy.ts"),
    `export function formatLaunchTitle(status: string) {
  return \`Launch \${status}\`;
}
`,
  );
  fs.writeFileSync(
    path.join(projectRoot, "server", "loaders.ts"),
    `import { currentLaunchStatus } from "@/src/domain/status";

export function loadLaunchMetrics() {
  return { status: currentLaunchStatus };
}
`,
  );
  fs.writeFileSync(
    path.join(projectRoot, "src", "domain", "status.ts"),
    `export const currentLaunchStatus = "ready";
`,
  );

  const report = scanDxBuildGraph(projectRoot, {
    changedPaths: ["src/domain/status.ts"],
  });

  const route = nodeByPath(report, "app/page.tsx");
  const component = nodeByPath(report, "components/LaunchPanel.tsx");
  const libCopy = nodeByPath(report, "lib/launch-copy.ts");
  const serverLoader = nodeByPath(report, "server/loaders.ts");
  const domainStatus = nodeByPath(report, "src/domain/status.ts");

  for (const sourceModule of [libCopy, serverLoader, domainStatus]) {
    assert.equal(sourceModule.kind, "source-module");
    assert.equal(sourceModule.contract, "dx.www.moduleGraph");
  }
  assert.ok(hasEdge(report, route.id, component.id, "imports"));
  assert.ok(hasEdge(report, component.id, libCopy.id, "imports"));
  assert.ok(hasEdge(report, component.id, serverLoader.id, "imports"));
  assert.ok(hasEdge(report, serverLoader.id, domainStatus.id, "imports"));
  assert.deepEqual(report.invalidation.changedNodeIds, [domainStatus.id]);
  assert.deepEqual(report.invalidation.rebuildNodeIds, [
    domainStatus.id,
    serverLoader.id,
    component.id,
    route.id,
  ]);
  const tasksByNode = new Map(
    report.turboTasksAdapter.tasks.map((task) => [task.nodeId, task]),
  );
  assert.equal(tasksByNode.get(domainStatus.id).cacheable, true);
  assert.equal(tasksByNode.get(serverLoader.id).cacheable, true);
  assert.equal(fs.existsSync(path.join(projectRoot, "node_modules")), false);
});

test("build graph receipt has a stable consumer snapshot for DX WWW and Zed", () => {
  const outputDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-build-graph-reader-"));
  const receiptPath = path.join(outputDir, "latest.json");

  writeDxBuildGraphReceipt(fixtureRoot, receiptPath, {
    changedPaths: ["styles/generated.css"],
  });

  const receipt = readDxBuildGraphReceipt(receiptPath);
  assert.equal(receipt.schema, "dx.build.graph");
  assert.equal(receipt.receiptPath, receiptPath);

  const snapshot = createDxBuildGraphConsumerSnapshot(receiptPath);
  assert.equal(snapshot.schema, "dx.build.graph.consumerSnapshot");
  assert.equal(snapshot.sourceSchema, "dx.build.graph");
  assert.equal(snapshot.receiptPath, receiptPath);
  assert.equal(snapshot.names.buildGraph, "dx.build.graph");
  assert.equal(snapshot.positioning.turbopackPublicDependency, false);
  assert.equal(snapshot.graph.nodeKindCounts["tsx-route"], 1);
  assert.equal(snapshot.graph.nodeKindCounts["dx-style-css"], 1);
  assert.equal(snapshot.graph.styleOptimization.styleNodeCount, 1);
  assert.equal(snapshot.graph.styleOptimization.originalRuleCount, 0);
  assert.equal(snapshot.graph.styleOptimization.retainedRuleCount, 0);
  assert.equal(snapshot.graph.styleOptimization.prunedRuleCount, 0);
  assert.equal(snapshot.graph.styleOptimization.minifiedStyleCount, 0);
  assert.ok(snapshot.invalidation.affectedCount >= 3);
  assert.ok(snapshot.consumers.dxCli.command.includes("dx graph"));
  assert.equal(snapshot.consumers.dxWww.receiptPath, ".dx/receipts/graph/latest.json");
  assert.equal(snapshot.consumers.zedPreview.primaryField, "invalidation.affectedNodeIds");
});

test("build graph receipt maps Turbopack core concepts onto DX-owned graph surfaces", () => {
  const report = scanDxBuildGraph(fixtureRoot, {
    changedPaths: ["styles/generated.css"],
  });

  assert.equal(report.positioning.turbopackPublicDependency, false);
  assert.equal(report.positioning.turbopackCoreAdapterBoundary, true);
  assert.ok(Array.isArray(report.coreConceptMap));

  const byConcept = new Map(
    report.coreConceptMap.map((entry) => [entry.upstreamConcept, entry]),
  );
  for (const concept of [
    "ModuleGraph",
    "Module",
    "ModuleReference",
    "Asset",
    "OutputAsset",
    "ChunkingContext",
    "SourceOwnedInvalidation",
    "ForgeSourceSurface",
  ]) {
    assert.ok(byConcept.has(concept), `expected concept mapping for ${concept}`);
  }

  assert.deepEqual(byConcept.get("ModuleGraph").dxContracts, [
    "dx.build.graph",
    "dx.www.moduleGraph",
  ]);
  assert.ok(
    byConcept
      .get("Module")
      .dxNodeKinds.includes("source-module"),
  );
  assert.ok(
    byConcept
      .get("Module")
      .dxNodeKinds.includes("source-module-chunk"),
  );
  assert.ok(
    byConcept
      .get("ModuleReference")
      .dxEdgeKinds.includes("imports-source-module"),
  );
  assert.ok(byConcept.get("Asset").dxNodeKinds.includes("dx-style-css"));
  assert.ok(byConcept.get("Asset").dxNodeKinds.includes("public-asset"));
  assert.ok(
    byConcept
      .get("OutputAsset")
      .dxNodeKinds.includes("route-shell-chunk"),
  );
  assert.ok(
    byConcept
      .get("ForgeSourceSurface")
      .dxNodeKinds.includes("forge-surface"),
  );
  assert.equal(byConcept.get("ForgeSourceSurface").nodeModulesRequired, false);

  const outputDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-build-graph-map-"));
  const receiptPath = path.join(outputDir, "latest.json");
  writeDxBuildGraphReceipt(fixtureRoot, receiptPath, {
    changedPaths: ["styles/generated.css"],
  });
  const snapshot = createDxBuildGraphConsumerSnapshot(receiptPath);
  assert.equal(snapshot.positioning.turbopackCoreAdapterBoundary, true);
  assert.deepEqual(snapshot.coreConceptMap.coveredNodeKinds, [
    "deploy-output",
    "dx-check-receipt",
    "dx-style-css",
    "forge-surface",
    "public-asset",
    "route-shell-chunk",
    "source-module",
    "source-module-chunk",
    "tsx-component",
    "tsx-route",
  ]);
  assert.equal(snapshot.coreConceptMap.nodeModulesRequired, false);
});

test("build graph CLI can emit the compact consumer snapshot", () => {
  const cliPath = path.join(__dirname, "..", "tools", "build-graph", "dx-graph.ts");
  const output = execFileSync(
    process.execPath,
    [
      cliPath,
      "--project",
      fixtureRoot,
      "--changed",
      "styles/generated.css",
      "--consumer-snapshot",
      "--json",
    ],
    { encoding: "utf8" },
  );

  const snapshot = JSON.parse(output);
  assert.equal(snapshot.schema, "dx.build.graph.consumerSnapshot");
  assert.equal(snapshot.sourceSchema, "dx.build.graph");
  assert.equal(snapshot.graph.nodeKindCounts["tsx-route"], 1);
  assert.equal(snapshot.graph.styleOptimization.styleNodeCount, 1);
  assert.equal(snapshot.graph.styleOptimization.prunedRuleCount, 0);
  assert.ok(snapshot.invalidation.affectedCount >= 3);
  assert.equal(snapshot.consumers.zedPreview.primaryField, "invalidation.affectedNodeIds");
});

function nodeByPath(report, relativePath) {
  const normalized = relativePath.replaceAll("\\", "/");
  const node = report.graph.nodes.find((candidate) => candidate.path === normalized);
  assert.ok(node, `expected graph node for ${normalized}`);
  return node;
}

function hasEdge(report, from, to, kind) {
  return report.graph.edges.some(
    (edge) => edge.from === from && edge.to === to && edge.kind === kind,
  );
}
