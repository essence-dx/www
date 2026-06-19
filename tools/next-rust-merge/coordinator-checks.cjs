const CHECK_SCHEMA = "dx.nextRustMerge.coordinatorChecks";
const {
  READ_ONLY_SIDE_EFFECTS,
  coordinatorSideEffectSummary,
  readOnlySideEffects,
  tempReceiptSideEffects,
} = require("./coordinator-side-effects.cjs");

const FORBIDDEN_HEAVY_COMMANDS = [
  /\bcargo\s+build\b/i,
  /\bcargo\s+test\b/i,
  /\bnpm\s+install\b/i,
  /\bpnpm\s+install\b/i,
  /\byarn\s+install\b/i,
  /\bbun\s+install\b/i,
  /\bnext\s+dev\b/i,
  /\bnext\s+build\b/i,
  /\bplaywright\b/i,
  /\bvercel\b/i,
];

const COORDINATOR_CHECKS = [
  check({
    id: "vendor-boundary",
    lane: 1,
    file: "benchmarks/next-rust-vendor-boundary.test.ts",
    boundary: "vendor-boundary",
    proves: ["provenance", "license", "protected DX runtime boundaries"],
    sideEffects: tempReceiptSideEffects({
      receiptPaths: [
        ".dx/receipts/next-rust/vendor-boundary.json",
        ".dx/receipts/next-rust/vendor-boundary-consumer.json",
      ],
      note: "writes only temp-fixture vendor receipts when run through the benchmark",
    }),
    blocking: true,
  }),
  check({
    id: "turbo-tasks-adapter",
    lane: 2,
    file: "benchmarks/dx-build-graph-turbo-tasks-adapter.test.ts",
    boundary: "adapter-boundary",
    proves: ["source-owned invalidation plan", "parallelism plan", "persistence plan"],
    sideEffects: tempReceiptSideEffects({
      receiptPaths: [".dx/receipts/graph/turbo-tasks-diff.json"],
      note: "writes only temp-fixture graph receipts when run through the benchmark",
    }),
    blocking: true,
  }),
  check({
    id: "turbopack-core-map",
    lane: 3,
    file: "benchmarks/dx-build-graph-core-map.test.ts",
    boundary: "adapter-boundary",
    proves: [
      "vendor-backed core concept validation",
      "no node_modules graph requirement",
      "no public Turbopack graph dependency",
    ],
    healthContract: {
      schema: "dx.build.graph.turbopackCoreConceptMapStatus",
      provider:
        "tools/build-graph/turbopack-core-status.ts#createTurbopackCoreConceptMapStatus",
      consumerSnapshotField: "coreConceptMapStatus",
      statusField: "status",
      scoreField: "score",
    },
    sideEffects: tempReceiptSideEffects({
      receiptPaths: [".dx/receipts/graph/latest.json"],
      note: "writes only temp-fixture graph receipts when run through the benchmark",
    }),
    blocking: true,
  }),
  check({
    id: "turbopack-core-graph",
    lane: 3,
    file: "benchmarks/dx-build-graph-receipt.test.ts",
    boundary: "source-owned receipt",
    proves: ["dx.build.graph contract", "consumer snapshot", "Turbopack core concept mapping"],
    sideEffects: tempReceiptSideEffects({
      receiptPaths: [".dx/receipts/graph/latest.json"],
      note: "writes only temp-fixture graph receipts when run through the benchmark",
    }),
    blocking: true,
  }),
  check({
    id: "next-compatibility-map",
    lane: 4,
    file: "benchmarks/nextjs-compatibility-map.test.ts",
    boundary: "compatibility map",
    proves: ["honest Next familiarity limits", "no full parity overclaim"],
    blocking: false,
  }),
  check({
    id: "dx-style-drift-fixture",
    lane: 5,
    file: "benchmarks/dx-style-drift-fixture-consumer.test.ts",
    boundary: "source-owned dx-style receipt",
    proves: ["typed Studio/Zed drift read model", "no raw style receipt scraping"],
    blocking: true,
  }),
  check({
    id: "mdx-docs-contract",
    lane: 7,
    file: "benchmarks/mdx-docs-source-build-contract.test.ts",
    boundary: "adapter-boundary",
    proves: ["source-owned MDX receipt", "no MDX runtime takeover"],
    blocking: false,
  }),
  check({
    id: "dev-hot-reload-protocol",
    lane: 8,
    file: "benchmarks/dx-dev-hot-reload-protocol.test.ts",
    boundary: "adapter-boundary",
    proves: ["DX-branded polling HMR contract", "no Turbopack HMR public endpoint"],
    blocking: true,
  }),
  check({
    id: "app-router-server-data",
    lane: 12,
    file: "benchmarks/app-router-server-data-build-contract.test.ts",
    boundary: "source-owned App Router build output contract",
    proves: [
      "server-data.json route contract",
      "App Router build output extracted from giant CLI",
    ],
    blocking: true,
  }),
  check({
    id: "next-custom-transforms",
    lane: 11,
    file: "benchmarks/next-custom-transforms-receipt.test.ts",
    boundary: "source-owned detection receipt",
    proves: ["RSC/server-action detection receipts", "no Next runtime takeover"],
    blocking: true,
  }),
  check({
    id: "default-www-template",
    lane: 13,
    file: "benchmarks/default-www-template-contract.test.ts",
    boundary: "DX default template contract",
    proves: ["no React/RSC/Node requirement", "Forge/dx-check/Zed/Studio evidence"],
    blocking: true,
  }),
  check({
    id: "www-template-forge-reality",
    lane: 13,
    file: "benchmarks/www-template-forge-reality.test.ts",
    boundary: "source-owned Forge reality model",
    proves: ["real lock-backed packages vs status-only lanes", "no fake runtime claims"],
    blocking: true,
  }),
  check({
    id: "build-readiness-gate",
    lane: 14,
    file: "benchmarks/dx-build-readiness-gate.test.ts",
    boundary: "coordinator source-only gate",
    proves: ["source/product score split", "installed-binary smoke pointer"],
    sideEffects: tempReceiptSideEffects({
      receiptPaths: [
        ".dx/receipts/build/readiness-gate-latest.json",
        ".dx/receipts/build/readiness-gate-consumer-snapshot.json",
      ],
      note: "writes only temp-fixture readiness receipts when run through the benchmark",
    }),
    blocking: true,
  }),
  check({
    id: "schema-status-noise",
    lane: 14,
    file: "benchmarks/next-rust-schema-status-noise.test.ts",
    boundary: "coordinator source-only gate",
    proves: ["no public .v1 schema suffixes", "no Next parity overclaim"],
    blocking: true,
  }),
  check({
    id: "giant-cli-mod",
    lane: 14,
    file: "benchmarks/next-rust-giant-cli-mod.test.ts",
    boundary: "coordinator source-only gate",
    proves: ["cli/mod.rs split-risk metrics", "no feature-lane refactor from coordinator"],
    blocking: true,
  }),
  check({
    id: "merge-conflict-markers",
    lane: 14,
    file: "benchmarks/next-rust-conflict-markers.test.ts",
    boundary: "coordinator source-only gate",
    proves: [
      "no unresolved merge conflict markers in merge-sensitive source surfaces",
      "coordinator conflict scan is executable and read-only",
    ],
    sideEffects: readOnlySideEffects("no workspace writes"),
    blocking: true,
  }),
];

function check({
  id,
  lane,
  file,
  boundary,
  proves,
  blocking,
  healthContract = null,
  sideEffects = READ_ONLY_SIDE_EFFECTS,
}) {
  return {
    id,
    lane,
    boundary,
    proves,
    blocking,
    heavy: false,
    publicTurbopackDependency: false,
    requiresReactCore: false,
    requiresNodeModules: false,
    healthContract,
    sideEffects,
    command: ["node", "--test", file],
  };
}

function commandText(checkEntry) {
  return checkEntry.command.join(" ");
}

function heavyCommandViolations(checkEntry) {
  const text = commandText(checkEntry);
  return FORBIDDEN_HEAVY_COMMANDS.filter((pattern) => pattern.test(text)).map(String);
}

function coordinatorCheckSummary() {
  return {
    schema: CHECK_SCHEMA,
    lane: 14,
    laneName: "Final Coordinator",
    featureImplementation: false,
    checks: COORDINATOR_CHECKS,
    blockingCheckCount: COORDINATOR_CHECKS.filter((entry) => entry.blocking).length,
    sideEffectSummary: coordinatorSideEffectSummary(COORDINATOR_CHECKS),
  };
}

if (require.main === module) {
  const summary = coordinatorCheckSummary();
  if (process.argv.includes("--json")) {
    process.stdout.write(`${JSON.stringify(summary, null, 2)}\n`);
  } else {
    for (const entry of summary.checks) {
      process.stdout.write(`${entry.id}: ${commandText(entry)}\n`);
    }
  }
}

module.exports = {
  CHECK_SCHEMA,
  COORDINATOR_CHECKS,
  FORBIDDEN_HEAVY_COMMANDS,
  commandText,
  coordinatorCheckSummary,
  coordinatorSideEffectSummary,
  heavyCommandViolations,
};
