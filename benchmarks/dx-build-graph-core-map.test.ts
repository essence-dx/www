import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const { execFileSync, spawnSync } = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const {
  TURBOPACK_CORE_DX_GRAPH_CONCEPTS,
  createTurbopackCoreConceptMapStatus,
  createDxBuildGraphConsumerSnapshot,
  findNextRustVendorWorkspaceRoot,
  validateTurbopackCoreConceptMap,
  writeDxBuildGraphReceipt,
} = require("../tools/build-graph");

const repoRoot = path.join(__dirname, "..");
const fixtureRoot = path.join(__dirname, "fixtures", "build-graph", "minimal-app");

test("turbopack core concept map is reusable and backed by vendored source files", () => {
  assert.ok(Array.isArray(TURBOPACK_CORE_DX_GRAPH_CONCEPTS));

  const validation = validateTurbopackCoreConceptMap(
    repoRoot,
    TURBOPACK_CORE_DX_GRAPH_CONCEPTS,
  );

  assert.equal(validation.schema, "dx.build.graph.turbopackCoreConceptMapValidation");
  assert.equal(validation.turbopackPublicDependency, false);
  assert.equal(validation.publicArchitecture, false);
  assert.equal(validation.adapterBoundary, true);
  assert.equal(validation.nodeModulesRequired, false);
  assert.deepEqual(validation.missingVendorPaths, []);
  assert.deepEqual(validation.conceptsWithNodeModules, []);
  assert.deepEqual(validation.conceptsWithoutBoundary, []);
  assert.deepEqual(validation.conceptsWithPublicOverclaim, []);
  assert.deepEqual(validation.upstreamConcepts, [
    "Asset",
    "ChunkingContext",
    "ForgeSourceSurface",
    "Module",
    "ModuleGraph",
    "ModuleReference",
    "OutputAsset",
    "SourceOwnedInvalidation",
  ]);
  for (const kind of [
    "dx-style-css",
    "forge-surface",
    "public-asset",
    "route-shell-chunk",
    "source-module",
    "source-module-chunk",
    "tsx-route",
  ]) {
    assert.ok(validation.coveredNodeKinds.includes(kind), `missing node kind ${kind}`);
  }
  for (const kind of [
    "imports",
    "imports-source-module",
    "links-entry-module",
    "owns-source",
  ]) {
    assert.ok(validation.coveredEdgeKinds.includes(kind), `missing edge kind ${kind}`);
  }
});

test("turbopack core concept map rejects broad runtime and parity overclaims", () => {
  const validVendorPath =
    "vendor/next-rust/turbopack/crates/turbopack-core/src/module_graph/mod.rs";
  const overclaimConcepts = [
    {
      upstreamConcept: "ReactCoreOverclaim",
      vendorPaths: [validVendorPath],
      dxContracts: ["dx.build.graph"],
      dxNodeKinds: ["tsx-route"],
      dxEdgeKinds: ["imports"],
      dxReceiptFields: ["graph.nodes"],
      boundary: "React and RSC are required as the core app model",
      nodeModulesRequired: false,
    },
    {
      upstreamConcept: "NodeFoundationOverclaim",
      vendorPaths: [validVendorPath],
      dxContracts: ["dx.build.graph"],
      dxNodeKinds: ["source-module-chunk"],
      dxEdgeKinds: ["imports-source-module"],
      dxReceiptFields: ["graph.edges"],
      boundary: "Node and NAPI become the default foundation for graph work",
      nodeModulesRequired: false,
    },
    {
      upstreamConcept: "TurbopackPublicModelOverclaim",
      vendorPaths: [validVendorPath],
      dxContracts: ["dx.build.graph"],
      dxNodeKinds: ["route-shell-chunk"],
      dxEdgeKinds: ["links-entry-module"],
      dxReceiptFields: ["graph.nodes"],
      boundary: "Turbopack is the public graph model for DX",
      nodeModulesRequired: false,
    },
    {
      upstreamConcept: "FullNextParityOverclaim",
      vendorPaths: [validVendorPath],
      dxContracts: ["dx.build.graph"],
      dxNodeKinds: ["deploy-output"],
      dxEdgeKinds: ["emitted-from"],
      dxReceiptFields: ["route_outputs"],
      boundary: "full Next.js parity is proven by this map",
      nodeModulesRequired: false,
    },
  ];

  const validation = validateTurbopackCoreConceptMap(repoRoot, overclaimConcepts);
  assert.deepEqual(validation.missingVendorPaths, []);
  assert.deepEqual(validation.conceptsWithPublicOverclaim, [
    "FullNextParityOverclaim",
    "NodeFoundationOverclaim",
    "ReactCoreOverclaim",
    "TurbopackPublicModelOverclaim",
  ]);
  assert.equal(validation.adapterBoundary, false);

  const status = createTurbopackCoreConceptMapStatus(repoRoot, overclaimConcepts);
  assert.equal(status.status, "blocked");
  assert.ok(status.score < 100);
  assert.ok(status.blockingReasons.includes("public-architecture-overclaim"));
  assert.equal(status.evidence.conceptsWithPublicOverclaimCount, 4);
});

test("consumer snapshot exposes the Turbopack core concept map validation", () => {
  const outputDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-build-graph-core-map-"));
  const receiptPath = path.join(outputDir, "latest.json");

  writeDxBuildGraphReceipt(fixtureRoot, receiptPath, {
    changedPaths: ["styles/generated.css"],
  });

  const snapshot = createDxBuildGraphConsumerSnapshot(receiptPath);
  assert.equal(
    snapshot.coreConceptMapValidation.schema,
    "dx.build.graph.turbopackCoreConceptMapValidation",
  );
  assert.equal(snapshot.coreConceptMapValidation.adapterBoundary, true);
  assert.equal(snapshot.coreConceptMapValidation.nodeModulesRequired, false);
  assert.equal(snapshot.coreConceptMapValidation.publicArchitecture, false);
  assert.equal(snapshot.coreConceptMapValidation.turbopackPublicDependency, false);
  assert.deepEqual(snapshot.coreConceptMapValidation.missingVendorPaths, []);
  assert.ok(
    snapshot.coreConceptMapValidation.upstreamConcepts.includes("ModuleGraph"),
  );
});

test("Turbopack core concept map status is a shared Lane 3 health contract", () => {
  const status = createTurbopackCoreConceptMapStatus(
    repoRoot,
    TURBOPACK_CORE_DX_GRAPH_CONCEPTS,
  );

  assert.equal(status.schema, "dx.build.graph.turbopackCoreConceptMapStatus");
  assert.equal(status.lane, 3);
  assert.equal(status.laneName, "Turbopack Core Module Graph");
  assert.equal(status.status, "passing");
  assert.equal(status.score, 100);
  assert.deepEqual(status.blockingReasons, []);
  assert.equal(status.architecture.dxRuntimeAuthoritative, true);
  assert.equal(status.architecture.publicTurbopackDependency, false);
  assert.equal(status.architecture.reactRequiredCore, false);
  assert.equal(status.architecture.nodeModulesRequired, false);
  assert.equal(status.architecture.nodeNapiFoundation, false);
  assert.deepEqual(status.boundary, {
    sourceOnly: true,
    adapterBoundary: true,
    publicArchitecture: false,
    fullParityProven: false,
    nextRuntimeAdopted: false,
  });
  assert.equal(status.evidence.missingVendorPathCount, 0);
  assert.ok(status.evidence.upstreamConcepts.includes("ModuleGraph"));
  assert.equal(
    status.recommendedAction,
    "read dx.build.graph.consumerSnapshot.coreConceptMapStatus",
  );

  const outputDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-build-graph-core-status-"));
  const receiptPath = path.join(outputDir, "latest.json");
  writeDxBuildGraphReceipt(fixtureRoot, receiptPath, {
    changedPaths: ["styles/generated.css"],
  });

  const snapshot = createDxBuildGraphConsumerSnapshot(receiptPath);
  assert.equal(
    snapshot.coreConceptMapStatus.schema,
    "dx.build.graph.turbopackCoreConceptMapStatus",
  );
  assert.equal(snapshot.coreConceptMapStatus.status, "passing");
  assert.equal(snapshot.coreConceptMapStatus.architecture.nodeModulesRequired, false);
  assert.deepEqual(snapshot.coreConceptMapStatus.boundary, status.boundary);
});

test("Turbopack core status uses a shared Next Rust vendor root resolver", () => {
  const workspaceRoot = findNextRustVendorWorkspaceRoot(__dirname);
  assert.equal(workspaceRoot, repoRoot);
  assert.ok(
    fs.existsSync(
      path.join(
        workspaceRoot,
        "vendor",
        "next-rust",
        "turbopack",
        "crates",
        "turbopack-core",
      ),
    ),
  );

  const status = createTurbopackCoreConceptMapStatus(workspaceRoot);
  assert.equal(status.status, "passing");
  assert.equal(status.evidence.missingVendorPathCount, 0);
});

test("build graph CLI emits Turbopack core concept map status directly", () => {
  const cliPath = path.join(repoRoot, "tools", "build-graph", "dx-graph.ts");
  const output = execFileSync(
    process.execPath,
    [cliPath, "--core-map-status", "--json"],
    { encoding: "utf8" },
  );
  const status = JSON.parse(output);

  assert.equal(status.schema, "dx.build.graph.turbopackCoreConceptMapStatus");
  assert.equal(status.lane, 3);
  assert.equal(status.status, "passing");
  assert.equal(status.score, 100);
  assert.equal(status.architecture.publicTurbopackDependency, false);
  assert.equal(status.architecture.reactRequiredCore, false);
  assert.equal(status.architecture.nodeModulesRequired, false);
  assert.deepEqual(status.boundary, {
    sourceOnly: true,
    adapterBoundary: true,
    publicArchitecture: false,
    fullParityProven: false,
    nextRuntimeAdopted: false,
  });
  assert.equal(status.evidence.missingVendorPathCount, 0);
  assert.equal(status.validation.turbopackPublicDependency, false);
});

test("build graph CLI rejects Turbopack core status with graph scan flags", () => {
  const cliPath = path.join(repoRoot, "tools", "build-graph", "dx-graph.ts");
  const result = spawnSync(
    process.execPath,
    [
      cliPath,
      "--core-map-status",
      "--changed",
      "styles/generated.css",
      "--json",
    ],
    { encoding: "utf8", windowsHide: true },
  );

  assert.notEqual(result.status, 0);
  assert.equal(result.stdout, "");
  assert.match(
    result.stderr,
    /--core-map-status cannot be combined with graph receipt, diff, snapshot, or changed-path flags/,
  );
});
