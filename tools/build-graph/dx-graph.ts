#!/usr/bin/env node
const fs = require("node:fs");
const path = require("node:path");

const {
  createDxBuildGraphConsumerSnapshotFromReceipt,
  readDxBuildGraphReceipt,
} = require("./reader.ts");
const { scanDxBuildGraph, writeDxBuildGraphReceipt } = require("./receipt.ts");
const { diffTurboTasksAdapterPlans } = require("./turbo-tasks-adapter.ts");
const {
  createTurboTasksAdapterDiffConsumerSummary,
} = require("./turbo-tasks-diff-reader.ts");
const {
  createTurboTasksAdapterStatusFromReceipt,
} = require("./turbo-tasks-status.ts");
const {
  createTurbopackCoreConceptMapStatus,
} = require("./turbopack-core-status.ts");
const {
  findNextRustVendorWorkspaceRoot,
} = require("./vendor-root.ts");

const args = process.argv.slice(2);
const project = valueAfter("--project") || process.cwd();
const changedPaths = valuesAfter("--changed");
const receiptPath =
  valueAfter("--write") || valueAfter("--receipt") || valueAfter("--out");
const diffAgainstPath = valueAfter("--diff-against") || valueAfter("--diff");
const diffReceiptPath = valueAfter("--write-diff") || valueAfter("--diff-out");
const diffSummaryPath =
  valueAfter("--diff-summary") || valueAfter("--summary-diff");
const consumerSnapshot =
  args.includes("--consumer-snapshot") || args.includes("--snapshot");
const coreMapStatus =
  args.includes("--core-map-status") || args.includes("--turbopack-core-status");
const turboTasksStatus =
  args.includes("--turbo-tasks-status") ||
  args.includes("--turbo-tasks-adapter-status");
const json =
  args.includes("--json") ||
  Boolean(receiptPath) ||
  Boolean(diffAgainstPath) ||
  Boolean(diffReceiptPath) ||
  Boolean(diffSummaryPath) ||
  consumerSnapshot ||
  coreMapStatus ||
  turboTasksStatus;

const outOfScopeExecutionFlag = requestedOutOfScopeExecutionFlag();
if (outOfScopeExecutionFlag) {
  throw new Error(
    `Turbo Tasks execution is out of scope for DX-WWW (${outOfScopeExecutionFlag}); use --turbo-tasks-status, --diff-against, or --write-diff for reference-only build graph evidence.`,
  );
}

if (diffSummaryPath && diffSummaryHasConflictingGraphFlags()) {
  throw new Error(
    "--diff-summary cannot be combined with graph scan, receipt, diff, or snapshot flags",
  );
}

if (diffReceiptPath && !diffAgainstPath) {
  throw new Error("--write-diff requires --diff-against <receipt>");
}

if (turboTasksStatus && turboTasksStatusHasConflictingGraphFlags()) {
  throw new Error(
    "--turbo-tasks-status cannot be combined with receipt, diff, or snapshot flags",
  );
}

if (coreMapStatus && coreMapStatusHasConflictingGraphFlags()) {
  throw new Error(
    "--core-map-status cannot be combined with graph receipt, diff, snapshot, or changed-path flags",
  );
}

let report = null;
let output = null;

if (coreMapStatus) {
  output = createTurbopackCoreConceptMapStatus(
    findNextRustVendorWorkspaceRoot(__dirname),
  );
} else if (diffSummaryPath) {
  output = createTurboTasksAdapterDiffConsumerSummary(diffSummaryPath);
} else {
  report = receiptPath
    ? writeDxBuildGraphReceipt(project, receiptPath, { changedPaths })
    : scanDxBuildGraph(project, { changedPaths });

  output = report;
  if (turboTasksStatus) {
    output = createTurboTasksAdapterStatusFromReceipt(
      report,
      report.receiptPath || receiptPath || null,
    );
  } else if (diffAgainstPath) {
    const previousReceipt = readDxBuildGraphReceipt(diffAgainstPath);
    output = diffTurboTasksAdapterPlans(
      previousReceipt.turboTasksAdapter,
      report.turboTasksAdapter,
    );
  } else if (consumerSnapshot) {
    output = createDxBuildGraphConsumerSnapshotFromReceipt(
      report,
      report.receiptPath || receiptPath || null,
    );
  }

  if (diffReceiptPath) {
    writeJsonFile(diffReceiptPath, output);
  }
}

if (json) {
  process.stdout.write(`${JSON.stringify(output, null, 2)}\n`);
} else {
  if (!report) {
    throw new Error("text output requires a build graph scan");
  }

  process.stdout.write(
    [
      "DX build graph",
      `project: ${path.resolve(project)}`,
      `nodes: ${report.graph.nodes.length}`,
      `edges: ${report.graph.edges.length}`,
      `affected: ${report.invalidation.affectedNodeIds.length}`,
    ].join("\n") + "\n",
  );
}

function valueAfter(flag) {
  const index = args.indexOf(flag);
  if (index === -1) return null;
  return args[index + 1] || null;
}

function valuesAfter(flag) {
  const values = [];
  for (let index = 0; index < args.length; index += 1) {
    if (args[index] === flag && args[index + 1]) {
      values.push(args[index + 1]);
      index += 1;
    }
  }
  return values;
}

function requestedOutOfScopeExecutionFlag() {
  const outOfScopeFlags = new Set([
    "--execute-turbo-tasks",
    "--run-turbo-tasks-adapter",
    "--write-execution",
    "--execution-out",
    "--write-task-run",
    "--execution-summary",
    "--task-run-summary",
    "--write-execution-handoff",
    "--execution-handoff",
    "--task-run-handoff",
    "--execution-handoff-summary",
    "--task-run-handoff-summary",
    "--execution-handoff-read-model",
    "--task-run-handoff-read-model",
    "--zed-handoff",
    "--turbo-tasks-zed-handoff",
    "--zed-handoff-panel",
    "--turbo-tasks-zed-handoff-panel",
    "--write-zed-handoff",
    "--zed-handoff-out",
    "--turbo-tasks-zed-handoff-out",
    "--task-cache-root",
    "--cache-root",
  ]);
  return args.find((arg) => outOfScopeFlags.has(arg)) || null;
}

function diffSummaryHasConflictingGraphFlags() {
  return (
    changedPaths.length > 0 ||
    Boolean(receiptPath) ||
    Boolean(diffAgainstPath) ||
    Boolean(diffReceiptPath) ||
    consumerSnapshot ||
    turboTasksStatus ||
    coreMapStatus
  );
}

function coreMapStatusHasConflictingGraphFlags() {
  return (
    Boolean(receiptPath) ||
    Boolean(diffAgainstPath) ||
    Boolean(diffReceiptPath) ||
    Boolean(diffSummaryPath) ||
    consumerSnapshot ||
    changedPaths.length > 0 ||
    turboTasksStatus
  );
}

function turboTasksStatusHasConflictingGraphFlags() {
  return (
    Boolean(receiptPath) ||
    Boolean(diffAgainstPath) ||
    Boolean(diffReceiptPath) ||
    Boolean(diffSummaryPath) ||
    consumerSnapshot ||
    coreMapStatus
  );
}

function writeJsonFile(targetPath, value) {
  const absoluteTargetPath = path.resolve(targetPath);
  fs.mkdirSync(path.dirname(absoluteTargetPath), { recursive: true });
  fs.writeFileSync(absoluteTargetPath, `${JSON.stringify(value, null, 2)}\n`);
}
