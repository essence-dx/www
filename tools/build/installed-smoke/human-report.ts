const {
  printCssAssetOutputProof,
  printNodeModulesProof,
} = require("./human-report-proof.ts");
const { printBinarySummary } = require("./human-binary-summary.ts");
const { printCommandDiagnostics } = require("./human-command-diagnostics.ts");
const { printNodeModulesBoundary } = require("./human-node-modules-boundary.ts");
const {
  formatRouteHandlerStaleEvidenceEntry,
} = require("./route-handler-stale-evidence.ts");

function smokeCommandPassed(report) {
  return report.passed === true &&
    (report.productProofRequired !== true || report.productProofPassed === true);
}

function printHumanReport(report) {
  const commandPassed = smokeCommandPassed(report);
  process.stdout.write(`Command result: ${formatCommandResult(report, commandPassed)}\n`);
  process.stdout.write(`DX build installed-binary smoke: ${report.passed ? "passed" : "failed"}\n`);
  process.stdout.write(
    `Product proof: ${report.productProofPassed ? "eligible" : "not eligible"} (${report.proof.scope})\n`,
  );
  printCssAssetOutputProof(report.proof?.cssAssetOutputProof);
  printNodeModulesProof(report.proof?.nodeModulesProof);
  printRouteHandlerReadiness(
    report.build?.sourceBuild?.routeHandlerReceipt?.routeHandlerReadiness,
  );
  printRouteHandlerStaleEvidence(
    report.build?.sourceBuild?.routeHandlerStaleEvidence,
  );
  printNodeModulesBoundary(report.build);
  if (report.proof?.nextAction) {
    process.stdout.write(`Product next action: ${report.proof.nextAction}\n`);
  }
  printBinarySummary(report);
  if (report.receiptPath) {
    process.stdout.write(`Receipt: ${report.receiptPath}\n`);
  }
  for (const failure of report.failures) {
    process.stdout.write(`- ${failure}\n`);
  }
  if (!report.passed) {
    printCommandDiagnostics("Build", report.build?.command);
    if (helpCommandNeedsDiagnostics(report)) {
      printCommandDiagnostics("Help", report.help.command);
    }
  }
  process.stdout.write(`Fixture: ${report.projectRoot}\n`);
}

function printRouteHandlerReadiness(readiness) {
  if (!Array.isArray(readiness) || readiness.length === 0) {
    return;
  }
  process.stdout.write("Route handlers:\n");
  for (const item of readiness) {
    process.stdout.write(`- ${formatRouteHandlerReadiness(item)}\n`);
  }
}

function printRouteHandlerStaleEvidence(staleEvidence) {
  if (!staleEvidence || staleEvidence.count <= 0) {
    return;
  }
  const entries = Array.isArray(staleEvidence.entries) ? staleEvidence.entries : [];
  const omittedCount = staleEvidence.omittedCount ?? Math.max(0, staleEvidence.count - entries.length);
  const suffix = omittedCount > 0 || entries.length !== staleEvidence.count
    ? ` (showing ${entries.length}, omitted ${omittedCount})`
    : "";
  process.stdout.write(`Route-handler stale evidence: ${staleEvidence.count} found${suffix}\n`);
  for (const entry of entries) {
    process.stdout.write(`- ${formatRouteHandlerStaleEvidenceEntry(entry)}\n`);
  }
}

function formatRouteHandlerReadiness(item) {
  const label = `${item.method || "METHOD"} ${item.route || "(unknown route)"} (${item.sourcePath || "unknown source"})`;
  if (item.buildStatus === "skipped") {
    return `${label}: skipped (${item.skipReason || "unspecified"}); expected status ${item.expectedStatus ?? "unknown"}`;
  }
  if (item.buildStatus === "executed") {
    return [
      `${label}: executed`,
      `status ${formatStatusPair(item.responseStatus, item.expectedStatus)}`,
      `source-owned: ${formatYesNo(item.sourceOwnedRuntimeBoundary)}`,
      `no node_modules: ${formatYesNo(item.declaresNoNodeModules)}`,
    ].join("; ");
  }
  return `${label}: ${item.buildStatus || "missing"}; expected status ${item.expectedStatus ?? "unknown"}`;
}

function formatStatusPair(actual, expected) {
  return `${actual ?? "unknown"}/${expected ?? "unknown"}`;
}

function helpCommandNeedsDiagnostics(report) {
  const help = report.help;
  if (!help || !help.command) {
    return false;
  }
  return help.command.exitCode !== 0 ||
    help.readOnly === false ||
    help.sourceOwnedContractVisible === false ||
    help.androidTargetVisible === false;
}

function formatCommandResult(report, commandPassed) {
  if (commandPassed) {
    return "passed";
  }
  if (report.passed === true && report.productProofRequired === true) {
    return "failed (product evidence required)";
  }
  return "failed";
}

function formatYesNo(value) {
  if (value === true) {
    return "yes";
  }
  if (value === false) {
    return "no";
  }
  return "unknown";
}

module.exports = { printHumanReport, smokeCommandPassed };
