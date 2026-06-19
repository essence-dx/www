const { INSTALLED_BINARY_SMOKE_RECEIPT } = require("./constants.ts");
const { summarizeCommand } = require("./command-summary.ts");
const { helpSummaryFailures, summarizeHelp } = require("./help-summary.ts");
const {
  binaryIdentityFailures,
  sourceFreshnessFailures,
  summarizeArtifactTrust,
  summarizeEnvironmentBlocker,
} = require("./report-diagnostics.ts");
const { relativePath } = require("./io.ts");
const { buildReceiptFailures } = require("./build-receipt-failures.ts");
const { summarizeManifest } = require("./manifest-summary.ts");
const { summarizeNextFamiliarCompatibility } = require("./next-familiar-compatibility.ts");
const { summarizeReadiness } = require("./readiness.ts");
const {
  sourceBuildFailures,
  summarizeAppRouterOutputs,
  summarizeFixture,
  summarizeSourceBuild,
} = require("./source-build.ts");
const {
  serverArtifactFailures,
  summarizeDeployAdapter,
  summarizeServerContracts,
} = require("./server-artifacts.ts");
const { summarizeOutputProofSummary, summarizeProof } = require("./proof.ts");

const SOURCE_FRESHNESS_PATH_SAMPLE_LIMIT = 24;
function createReport(input) {
  const helpText = `${input.help.stdout}\n${input.help.stderr}`;
  const report = {
    schema: "dx.build.installedBinarySmoke",
    schemaRevision: 1,
    binary: input.binary,
    binaryDefault: input.binaryProvenance.defaultBinary,
    binaryOverride: input.binaryProvenance.override,
    binaryRole: input.binaryProvenance.role,
    binaryIdentity: input.binaryIdentity,
    binarySourceFreshness: summarizeBinarySourceFreshness(input.binarySourceFreshness),
    runner: input.runner || null,
    projectRoot: input.projectRoot,
    receiptPath: input.receipt || null,
    passed: false,
    productProofRequired: input.requireProduct === true,
    fixture: summarizeFixture(input.projectRoot),
    help: summarizeHelp(input.help, helpText, input.helpReadOnly),
    build: summarizeBuild(input),
    environmentBlocker: summarizeEnvironmentBlocker(input),
    failures: [],
  };

  addFailures(report, input);
  report.passed = report.failures.length === 0;
  report.proof = summarizeProof(report);
  report.outputProofSummary = summarizeOutputProofSummary(report.proof);
  report.productProofPassed = report.passed && report.proof.productEligible === true;
  return report;
}

function summarizeBinarySourceFreshness(binarySourceFreshness) {
  if (!binarySourceFreshness || typeof binarySourceFreshness !== "object") {
    return binarySourceFreshness;
  }
  const summary = { ...binarySourceFreshness };
  if (Array.isArray(binarySourceFreshness.trackedSourcePaths)) {
    summary.trackedSourcePathSample = binarySourceFreshness.trackedSourcePaths.slice(
      0,
      SOURCE_FRESHNESS_PATH_SAMPLE_LIMIT,
    );
    summary.trackedSourcePathsTruncated =
      binarySourceFreshness.trackedSourcePaths.length >
      summary.trackedSourcePathSample.length;
    delete summary.trackedSourcePaths;
  }
  return summary;
}

function summarizeBuild(input) {
  return {
    exitCode: input.build.status,
    command: summarizeCommand(input.build),
    artifactTrust: summarizeArtifactTrust(input),
    manifestPath: relativePath(input.projectRoot, input.manifestPath),
    zedHandoffPath: relativePath(input.projectRoot, input.zedHandoffPath),
    readinessPath: relativePath(input.projectRoot, input.readinessPath),
    nodeModulesPresent: input.nodeModulesPresent === true,
    nodeModulesCreated: input.nodeModulesCreated,
    nodeModulesBeforePaths: Array.isArray(input.nodeModulesBeforePaths) ? input.nodeModulesBeforePaths : [],
    nodeModulesCreatedPaths: Array.isArray(input.nodeModulesCreatedPaths) ? input.nodeModulesCreatedPaths : [],
    nodeModulesPaths: Array.isArray(input.nodeModulesPaths) ? input.nodeModulesPaths : [],
    appRouter: summarizeAppRouterOutputs(input),
    manifest: summarizeManifest(input),
    serverContracts: summarizeServerContracts(input),
    deployAdapter: summarizeDeployAdapter(input),
    nextFamiliarCompatibilityEvidence: summarizeNextFamiliarCompatibility(input),
    sourceBuild: summarizeSourceBuild(input),
    zedHandoff: summarizeZedHandoff(input.zedHandoff),
    readiness: summarizeReadiness(input),
  };
}

function summarizeZedHandoff(zedHandoff) {
  return {
    present: zedHandoff.ok,
    schema: zedHandoff.ok ? zedHandoff.value.schema : null,
    hasStyleOptimization:
      zedHandoff.ok &&
      zedHandoff.value &&
      typeof zedHandoff.value.style_optimization === "object",
    hasBuildReadinessPointer:
      zedHandoff.ok &&
      zedHandoff.value &&
      zedHandoff.value.build_readiness === ".dx/receipts/build/readiness.json",
    hasInstalledBinarySmokeReceiptPointer:
      zedHandoff.ok &&
      zedHandoff.value &&
      zedHandoff.value.installed_binary_smoke_receipt === INSTALLED_BINARY_SMOKE_RECEIPT,
  };
}

function addFailures(report, input) {
  const binaryFailures = binaryIdentityFailures(input.binaryIdentity);
  for (const failure of binaryFailures) {
    report.failures.push(failure);
  }
  if (binaryFailures.length > 0) {
    pushIf(report, input.help.status !== 0, "dx www build --help exited non-zero");
    pushIf(report, input.build.status !== 0, "dx build fixture exited non-zero");
    return;
  }
  const binarySourceFreshnessFailures = sourceFreshnessFailures(input.binarySourceFreshness);
  for (const failure of binarySourceFreshnessFailures) {
    report.failures.push(failure);
  }
  if (binarySourceFreshnessFailures.length > 0) {
    return;
  }
  for (const failure of helpSummaryFailures(report.help)) {
    report.failures.push(failure);
  }
  if (report.environmentBlocker) {
    pushIf(report, input.build.status !== 0, "dx build fixture exited non-zero");
    report.failures.push(report.environmentBlocker.failure);
    return;
  }
  for (const failure of buildReceiptFailures(report, input)) {
    report.failures.push(failure);
  }
  for (const failure of sourceBuildFailures(report)) {
    report.failures.push(failure);
  }
  for (const failure of serverArtifactFailures(report)) {
    report.failures.push(failure);
  }
}

function pushIf(report, condition, message) {
  if (condition) {
    report.failures.push(message);
  }
}

module.exports = {
  createReport,
};
