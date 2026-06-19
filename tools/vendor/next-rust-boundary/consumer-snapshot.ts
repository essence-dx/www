const { buildActiveScopeBoundary } = require("./active-scope.js");
const { buildAdapterBoundary } = require("./adapter-boundary.js");
const { buildClaimPolicy } = require("./claim-policy.js");
const { SCHEMAS } = require("./constants.js");
const { receiptDisplayPath } = require("./paths.js");

function buildNextRustVendorBoundaryConsumerSnapshotReport(
  absoluteRepoRoot,
  absoluteReceiptPath,
  report,
  receiptCheck,
) {
  const receiptFresh = receiptCheck.status === "ok" && receiptCheck.stale === false;
  const nextRuntimeRequired = report.workspaceQuarantine.forbiddenWorkspaceMentions.some(
    (foundation) => ["next-core", "next-napi-bindings", "turbopack-nodejs"].includes(foundation),
  );
  const workspaceQuarantined =
    report.workspaceQuarantine.vendorInWorkspace === false &&
    report.workspaceQuarantine.forbiddenWorkspaceMentions.length === 0;
  const runtimeTakeoverBlocked =
    !nextRuntimeRequired &&
    !report.runtimeTakeover.reactRscRequired &&
    !report.runtimeTakeover.nodeNapiRequired &&
    !report.runtimeTakeover.nodeModulesDefault;
  const vendoredCargoDependencyClaimsBlocked =
    report.vendoredCargoDependencyClaims.forbiddenDependencies.length === 0;
  const vendorSourceInclusionBlocked =
    report.vendorSourceInclusion.forbiddenInclusions.length === 0;
  const publicSourceExposureBlocked =
    report.publicSourceExposure.forbiddenExposures.length === 0;
  const boundaryClean =
    workspaceQuarantined &&
    runtimeTakeoverBlocked &&
    vendoredCargoDependencyClaimsBlocked &&
    vendorSourceInclusionBlocked &&
    publicSourceExposureBlocked;
  const status = report.status === "ok" && receiptFresh && boundaryClean ? "ok" : "fail";

  return {
    schema: SCHEMAS.consumerSnapshot,
    status,
    sourceReceipt: {
      schema: report.schema,
      path: receiptDisplayPath(absoluteRepoRoot, absoluteReceiptPath),
      fresh: receiptFresh,
      checkStatus: receiptCheck.status,
      mismatchCount: receiptCheck.mismatches.length,
      mismatches: receiptCheck.mismatches,
    },
    vendor: {
      root: report.vendorRoot,
      role: "quarantined build infrastructure reference",
      publicArchitecture: "DX-WWW runtime/security/source model",
    },
    claimPolicy: buildClaimPolicy(),
    adapterBoundary: buildAdapterBoundary(),
    upstream: report.upstream,
    license: {
      file: report.license.file,
      sha256: report.license.sha256,
      mitNoticePresent: report.license.mitNoticePresent,
    },
    counts: {
      importedGroups: report.importedGroups.expected.length,
      protectedBoundaries: report.protectedBoundaries.expected.length,
      excludedCoreFoundations: report.excludedCoreFoundations.expected.length,
    },
    boundary: {
      workspaceQuarantined,
      runtimeTakeoverBlocked,
      publicDependencyClaimsBlocked:
        report.publicDependencyClaims.forbiddenDependencies.length === 0,
      vendoredCargoDependencyClaimsBlocked,
      vendorSourceInclusionBlocked,
      publicSourceExposureBlocked,
      nextRuntimeRequired,
      reactRscRequired: report.runtimeTakeover.reactRscRequired,
      nodeNapiRequired: report.runtimeTakeover.nodeNapiRequired,
      nodeModulesDefault: report.runtimeTakeover.nodeModulesDefault,
      turbopackPublicArchitecture: false,
      ...buildActiveScopeBoundary(report.publicClaimText),
    },
  };
}

module.exports = {
  buildNextRustVendorBoundaryConsumerSnapshotReport,
};
