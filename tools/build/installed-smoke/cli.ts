const fs = require("node:fs");
const path = require("node:path");

const { parseArgs, printUsage } = require("./args.ts");
const {
  classifyBinary,
  DEFAULT_INSTALLED_BINARY,
  inspectBinary,
  inspectBinarySourceFreshness,
} = require("./binary-provenance.ts");
const { INSTALLED_BINARY_SMOKE_RECEIPT } = require("./constants.ts");
const {
  createEmptyProject,
  createFixtureProject,
} = require("./fixture.ts");
const {
  printHumanReport,
  smokeCommandPassed,
} = require("./human-report.ts");
const { readJson } = require("./io.ts");
const { diffNodeModulesDirs, findNodeModulesDirs } = require("./no-node-modules.ts");
const { writeReportReceipt } = require("./receipt-writer.ts");
const { createReport } = require("./report.ts");
const { runDx, skippedDx } = require("./runner.ts");

const HELP_COMMAND_ARGS = ["www", "build", "--help"];
const BUILD_COMMAND_ARGS = ["build"];

function main(args) {
  let options;
  try {
    options = parseArgs(args);
  } catch (error) {
    process.stderr.write(`dx build installed smoke: ${error.message}\n`);
    printUsage(process.stderr);
    process.exit(2);
  }
  if (options.help) {
    printUsage();
    process.exit(0);
  }

  const projectRoot = options.project || createFixtureProject();
  const binary = options.binary || process.env.DX_WWW_BINARY || DEFAULT_INSTALLED_BINARY;
  const receiptPath = options.receipt || path.join(projectRoot, INSTALLED_BINARY_SMOKE_RECEIPT);
  const binaryIdentity = inspectBinary(binary);
  const binarySourceFreshness = inspectBinarySourceFreshness(binaryIdentity);
  const skipForStaleBinary = binarySourceFreshness.fresh === false;
  const nodeModulesBeforePaths = findNodeModulesDirs(projectRoot);
  const helpProject = createEmptyProject();
  const help = skipForStaleBinary
    ? skippedDx(binary, options.runner, HELP_COMMAND_ARGS, helpProject, "stale-binary")
    : runDx(binary, options.runner, HELP_COMMAND_ARGS, helpProject);
  const build = skipForStaleBinary
    ? skippedDx(binary, options.runner, BUILD_COMMAND_ARGS, projectRoot, "stale-binary")
    : runDx(binary, options.runner, BUILD_COMMAND_ARGS, projectRoot);
  const nodeModulesPaths = findNodeModulesDirs(projectRoot);
  const nodeModulesCreatedPaths = diffNodeModulesDirs(nodeModulesBeforePaths, nodeModulesPaths);
  const paths = buildReceiptPaths(projectRoot);
  const report = createReport({
    binary,
    binaryProvenance: classifyBinary(binary),
    binaryIdentity,
    binarySourceFreshness,
    runner: options.runner,
    projectRoot,
    receipt: receiptPath,
    requireProduct: options.requireProduct,
    help,
    helpReadOnly: !fs.existsSync(path.join(helpProject, ".dx")),
    build,
    nodeModulesPresent: nodeModulesPaths.length > 0,
    nodeModulesCreated: nodeModulesCreatedPaths.length > 0,
    nodeModulesBeforePaths,
    nodeModulesCreatedPaths,
    nodeModulesPaths,
    manifest: readJson(paths.manifestPath),
    appExecution: readJson(paths.appExecutionPath),
    serverData: readJson(paths.serverDataPath),
    serverContracts: readJson(paths.serverContractsPath),
    deployAdapter: readJson(paths.deployAdapterPath),
    sourceBuildManifest: readJson(paths.sourceBuildManifestPath),
    sourceBuildReceipt: readJson(paths.sourceBuildReceiptPath),
    routeHandlerReceipts: readJson(paths.routeHandlerReceiptsPath),
    canonicalReceipt: readJson(paths.canonicalReceiptPath),
    graphReceipt: readJson(paths.graphReceiptPath),
    graphConsumerSnapshot: readJson(paths.graphConsumerSnapshotPath),
    nextFamiliarCompatibilityEvidence: readJson(paths.nextFamiliarCompatibilityEvidencePath),
    zedHandoff: readJson(paths.zedHandoffPath),
    readiness: readJson(paths.readinessPath),
    ...paths,
  });

  writeReportReceipt(report, receiptPath);

  if (options.json) {
    process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
  } else {
    printHumanReport(report);
  }

  process.exit(smokeCommandPassed(report) ? 0 : 1);
}

function buildReceiptPaths(projectRoot) {
  return {
    manifestPath: path.join(projectRoot, ".dx", "build", ".dx/build-cache/manifest.json"),
    sourceBuildManifestPath: path.join(projectRoot, ".dx", "build", "source-build-.dx/build-cache/manifest.json"),
    sourceBuildReceiptPath: path.join(projectRoot, ".dx", "build", ".dx/build-cache/source-build-receipt.json"),
    routeHandlerReceiptsPath: path.join(projectRoot, ".dx", "build", ".dx/build-cache/route-handler-receipts.json"),
    canonicalReceiptPath: path.join(projectRoot, ".dx", "receipts", "build", "latest.json"),
    graphReceiptPath: path.join(projectRoot, ".dx", "receipts", "graph", "latest.json"),
    graphConsumerSnapshotPath: path.join(projectRoot, ".dx", "receipts", "graph", "consumer-snapshot.json"),
    nextFamiliarCompatibilityEvidencePath: path.join(projectRoot, ".dx", "build", "next-familiar-compatibility-evidence.json"),
    nextRuntimeParityEvidencePath: path.join(projectRoot, ".dx", "build", "next-runtime-parity-evidence.json"),
    appHtmlPath: path.join(projectRoot, ".dx", "build", "app", "index.html"),
    appPacketPath: path.join(projectRoot, ".dx", "build", "app", "index.dxpk"),
    appPageGraphPath: path.join(projectRoot, ".dx", "build", "app", "page-graph.json"),
    appExecutionPath: path.join(projectRoot, ".dx", "build", "app", "app-router-execution.json"),
    serverDataPath: path.join(projectRoot, ".dx", "build", "app", "server-data.json"),
    serverContractsPath: path.join(projectRoot, ".dx", "build", "server-contracts.json"),
    deployAdapterPath: path.join(projectRoot, ".dx", "build", ".dx/build-cache/deploy-adapter.json"),
    zedHandoffPath: path.join(projectRoot, ".dx", "receipts", "build", "zed-handoff.json"),
    readinessPath: path.join(projectRoot, ".dx", "receipts", "build", "readiness.json"),
  };
}

module.exports = {
  main,
};
