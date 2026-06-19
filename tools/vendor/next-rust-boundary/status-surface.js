const { SCHEMAS } = require("./constants.js");
const {
  buildActiveScopeReportFromBoundary,
  buildActiveScopeSummary,
} = require("./active-scope.js");
const { buildDxCheckProjection } = require("./dx-check-projection.js");
const { buildEditorSurfaceProjection } = require("./editor-surface-projection.js");
const { receiptDisplayPath } = require("./paths.js");

function receiptEvidence(repoRoot, check) {
  return {
    path: receiptDisplayPath(repoRoot, check.receiptPath),
    status: check.status,
    stale: check.stale,
    mismatches: check.mismatches,
  };
}

function receiptBlockers(label, check) {
  if (check.status === "ok" && check.stale === false) {
    return [];
  }
  if (check.mismatches.length === 0) {
    return [`${label} receipt ${check.status}`];
  }
  return check.mismatches.map((mismatch) => `${label} receipt ${check.status}:${mismatch}`);
}

function buildNextRustVendorBoundaryStatusSurfaceReport(
  repoRoot,
  snapshot,
  sourceReceiptCheck,
  consumerReceiptCheck,
) {
  const blockers = [
    ...receiptBlockers("source", sourceReceiptCheck),
    ...receiptBlockers("consumer", consumerReceiptCheck),
  ];
  if (snapshot.status !== "ok") {
    blockers.push(`consumer snapshot ${snapshot.status}`);
  }

  const status = blockers.length === 0 ? "ok" : "blocked";
  const activeScope = buildActiveScopeReportFromBoundary(status, snapshot.boundary);
  const activeScopeSummary = buildActiveScopeSummary(activeScope);

  return {
    schema: SCHEMAS.statusSurface,
    status,
    surface: {
      kind: "dx-check.statusSurface",
      id: "next-rust-vendor-boundary",
      owner: "DX-WWW",
      adapterBoundary: "executable receipt adapter, not native runtime integration",
    },
    evidence: {
      sourceReceipt: receiptEvidence(repoRoot, sourceReceiptCheck),
      consumerReceipt: receiptEvidence(repoRoot, consumerReceiptCheck),
      boundary: snapshot.boundary,
    },
    dxCheck: buildDxCheckProjection(snapshot, blockers, activeScope),
    editorSurfaces: buildEditorSurfaceProjection(snapshot, blockers, activeScope),
    activeScope,
    activeScopeSummary,
    markers: [
      {
        name: "data-dx-next-rust-vendor-boundary-status",
        value: status,
      },
      {
        name: "data-dx-next-rust-vendor-boundary-upstream",
        value: snapshot.upstream.commit,
      },
      {
        name: "data-dx-next-rust-vendor-boundary-public-architecture",
        value: snapshot.vendor.publicArchitecture,
      },
      {
        name: "data-dx-next-rust-vendor-boundary-claim-scope",
        value: snapshot.claimPolicy.allowedClaimScope.join("|"),
      },
    ],
    blockers,
    unproven: snapshot.adapterBoundary.blockedUntilProven,
    checks: [
      "node tools/vendor/next-rust-boundary-check.js --active-scope",
      "node tools/vendor/next-rust-boundary-check.js --check-receipt",
      "node tools/vendor/next-rust-boundary-check.js --check-consumer-receipt",
    ],
  };
}

module.exports = {
  buildNextRustVendorBoundaryStatusSurfaceReport,
};
