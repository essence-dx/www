const { SCHEMAS } = require("./constants.js");

const REMOVED_TARGETS = [
  "Next DevTools clone",
  "DX-WWW DevTools",
  "/_dx/devtools",
  "external DevTools runtime",
];

const EXCLUDED_RUNTIME_TARGETS = [
  "Turbopack runtime/build adoption",
  "external bundler execution proof",
  "Turbopack powers dx build/dev",
];

const PUBLIC_CLAIM_CHECKS = [
  "next-devtools-clone-target",
  "dx-devtools-removed-target",
  "turbopack-runtime-build-adoption",
  "external-bundler-execution-proof-target",
];

const REMOVED_TARGET_CLAIMS = [
  "next-devtools-clone-target",
  "dx-devtools-removed-target",
];

const EXCLUDED_RUNTIME_TARGET_CLAIMS = [
  "turbopack-runtime-build-adoption",
  "external-bundler-execution-proof-target",
];

function containsClaim(publicClaimText, claims) {
  const claimSet = new Set(claims);
  return (publicClaimText?.forbiddenClaims ?? []).some((finding) => claimSet.has(finding.claim));
}

function buildActiveScopeBoundary(publicClaimText) {
  return {
    removedTargetsBlocked: !containsClaim(publicClaimText, REMOVED_TARGET_CLAIMS),
    excludedRuntimeTargetsBlocked: !containsClaim(publicClaimText, EXCLUDED_RUNTIME_TARGET_CLAIMS),
  };
}

function buildActiveScopeReportFromBoundary(status, boundary) {
  return {
    schema: SCHEMAS.activeScope,
    status,
    referenceOnlyNextRust: true,
    runtimeBuildAdoption: false,
    turbopackPublicArchitecture: boundary.turbopackPublicArchitecture,
    devFeedbackEndpoint: "/_dx/feedback",
    removedTargetsBlocked: boundary.removedTargetsBlocked !== false,
    removedTargets: REMOVED_TARGETS,
    excludedRuntimeTargetsBlocked: boundary.excludedRuntimeTargetsBlocked !== false,
    excludedRuntimeTargets: EXCLUDED_RUNTIME_TARGETS,
    publicClaimChecks: PUBLIC_CLAIM_CHECKS,
  };
}

function buildActiveScopeReport(statusSurface) {
  return buildActiveScopeReportFromBoundary(statusSurface.status, statusSurface.evidence.boundary);
}

function buildActiveScopeSummary(activeScope) {
  const mismatches = [];

  if (activeScope?.removedTargetsBlocked !== true) {
    mismatches.push({
      id: "removed-devtools-targets",
      label: "DevTools targets removed",
      publicClaimChecks: REMOVED_TARGET_CLAIMS,
    });
  }

  if (
    activeScope?.excludedRuntimeTargetsBlocked !== true ||
    activeScope?.runtimeBuildAdoption !== false ||
    activeScope?.turbopackPublicArchitecture !== false
  ) {
    mismatches.push({
      id: "turbopack-runtime-adoption",
      label: "Turbopack adoption blocked",
      publicClaimChecks: EXCLUDED_RUNTIME_TARGET_CLAIMS,
    });
  }

  return {
    schema: SCHEMAS.activeScopeSummary,
    status: mismatches.length === 0 ? "ok" : "fail",
    scopeStatus: activeScope?.status ?? "missing",
    mismatchCount: mismatches.length,
    mismatches,
  };
}

function activeScopeCheckEvidence(activeScope) {
  return {
    schema: activeScope.schema,
    status: activeScope.status,
    referenceOnlyNextRust: activeScope.referenceOnlyNextRust,
    runtimeBuildAdoption: activeScope.runtimeBuildAdoption,
    turbopackPublicArchitecture: activeScope.turbopackPublicArchitecture,
    devFeedbackEndpoint: activeScope.devFeedbackEndpoint,
    removedTargetsBlocked: activeScope.removedTargetsBlocked,
    excludedRuntimeTargetsBlocked: activeScope.excludedRuntimeTargetsBlocked,
    publicClaimChecks: activeScope.publicClaimChecks,
    activeScopeSummary: buildActiveScopeSummary(activeScope),
  };
}

function buildActiveScopeCheck(activeScope) {
  const evidence = activeScopeCheckEvidence(activeScope);
  const messages = [];
  const expected = {
    status: "ok",
    referenceOnlyNextRust: true,
    runtimeBuildAdoption: false,
    turbopackPublicArchitecture: false,
    devFeedbackEndpoint: "/_dx/feedback",
    removedTargetsBlocked: true,
    excludedRuntimeTargetsBlocked: true,
  };

  for (const [key, expectedValue] of Object.entries(expected)) {
    if (evidence[key] !== expectedValue) {
      messages.push(`activeScope.${key}:${evidence[key]}`);
    }
  }

  return {
    id: "next-rust.vendor-boundary.active-scope",
    title: "DX-WWW active scope excludes DevTools clone and Turbopack adoption targets",
    status: messages.length === 0 ? "ok" : "fail",
    severity: "blocking",
    messages,
    evidence,
  };
}

module.exports = {
  buildActiveScopeBoundary,
  buildActiveScopeCheck,
  buildActiveScopeReport,
  buildActiveScopeReportFromBoundary,
  buildActiveScopeSummary,
};
