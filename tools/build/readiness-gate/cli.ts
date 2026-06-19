const path = require("node:path");

const {
  READINESS_GATE_RECEIPT,
  READINESS_GATE_SNAPSHOT,
} = require("./constants.ts");
const { inspectQuality } = require("./quality.ts");
const { createProofBundle, runProofBundle } = require("./proof-bundle.ts");
const { writeSourceProjection } = require("./projection.ts");
const { createReport } = require("./report.ts");
const { resolveReceiptSources } = require("./receipt-sources.ts");
const { createConsumerSnapshot } = require("./snapshot.ts");
const { readJson, writeJson } = require("./io.ts");

function main(args) {
  try {
    const options = parseArgs(args);
    const projectRoot = options.project || process.cwd();
    const repoRoot = path.resolve(__dirname, "..", "..", "..");
    let sources = resolveReceiptSources(projectRoot);
    let receipts = readReceipts(projectRoot, sources);

    if (options.writeSourceProjection) {
      writeSourceProjection(projectRoot, receipts);
      sources = resolveReceiptSources(projectRoot);
      receipts = readReceipts(projectRoot, sources);
    }

    const proofOptions = { projectRoot, repoRoot };
    const proofBundle = options.proofBundle
      ? options.dryRun
        ? createProofBundle("dry-run", proofOptions)
        : runProofBundle(repoRoot, proofOptions)
      : null;
    if (options.proofBundle && !options.dryRun) {
      sources = resolveReceiptSources(projectRoot);
      receipts = readReceipts(projectRoot, sources);
    }

    const report = createReport(projectRoot, receipts, inspectQuality(repoRoot), proofBundle);

    if (options.write) {
      writeJson(path.join(projectRoot, READINESS_GATE_RECEIPT), report);
    }
    if (options.writeSnapshot) {
      writeJson(
        path.join(projectRoot, READINESS_GATE_SNAPSHOT),
        createConsumerSnapshot(projectRoot, report),
      );
    }
    if (options.json) {
      process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
    } else {
      printHuman(report);
    }

    process.exitCode = report.productReady ? 0 : 1;
  } catch (error) {
    process.stderr.write(`${error.message}\n`);
    process.exitCode = 2;
  }
}

function parseArgs(args) {
  const options = {
    project: null,
    json: false,
    write: false,
    writeSnapshot: false,
    writeSourceProjection: false,
    proofBundle: false,
    dryRun: false,
  };

  for (let index = 0; index < args.length; ) {
    const arg = args[index];
    if (arg === "--project") {
      options.project = path.resolve(requireValue(args, index, arg));
      index += 2;
      continue;
    }
    if (arg === "--json") {
      options.json = true;
      index += 1;
      continue;
    }
    if (arg === "--write") {
      options.write = true;
      index += 1;
      continue;
    }
    if (arg === "--write-snapshot") {
      options.writeSnapshot = true;
      index += 1;
      continue;
    }
    if (arg === "--write-source-projection") {
      options.writeSourceProjection = true;
      index += 1;
      continue;
    }
    if (arg === "--proof-bundle") {
      options.proofBundle = true;
      index += 1;
      continue;
    }
    if (arg === "--dry-run") {
      options.dryRun = true;
      index += 1;
      continue;
    }
    if (arg === "--help" || arg === "-h") {
      printUsage();
      process.exit(0);
    }
    throw new Error(`Unknown option: ${arg}`);
  }

  return options;
}

function readReceipts(projectRoot, sources) {
  return {
    checkLaunch: readJson(projectRoot, sources.checkLaunch),
    installedBinarySmoke: readJson(projectRoot, sources.installedBinarySmoke),
    nextRustBoundary: readJson(projectRoot, sources.nextRustBoundary),
    readiness: readJson(projectRoot, sources.readiness),
    sourceBuild: readJson(projectRoot, sources.sourceBuild),
    zedHandoff: readJson(projectRoot, sources.zedHandoff),
  };
}

function requireValue(args, index, flag) {
  const value = args[index + 1];
  if (!value) {
    throw new Error(`${flag} requires a value`);
  }
  return value;
}

function printUsage() {
  process.stdout.write(
    [
      "Usage: node tools/build/dx-build-readiness-gate.ts [--project <path>] [--json] [--write] [--write-snapshot] [--write-source-projection] [--proof-bundle] [--dry-run]",
      "",
      "Reads build readiness, Zed handoff, and installed-binary smoke receipts.",
      "Use --write-source-projection to create compact source readiness receipts from an existing source-build receipt.",
      "Use --proof-bundle --dry-run to inspect the Windows-friendly cargo, Node, dx build smoke, and HTTP proof plan.",
      "Use --proof-bundle without --dry-run to execute the validation bundle; HTTP steps only probe an already-running local server.",
      "",
    ].join("\n"),
  );
}

function printHuman(report) {
  process.stdout.write(`DX build release readiness: ${report.status}\n`);
  for (const blocker of report.blockers) {
    process.stdout.write(`- ${blocker}\n`);
  }
}

module.exports = {
  main,
  parseArgs,
};
