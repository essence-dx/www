const { SCHEMAS } = require("./constants.js");
const { buildActiveScopeCheck } = require("./active-scope.js");

const RECEIPT_EVIDENCE_PATHS = [
  ".dx/receipts/next-rust/vendor-boundary.json",
  ".dx/receipts/next-rust/vendor-boundary-consumer.json",
];

function checkStatus(ok) {
  return ok ? "ok" : "fail";
}

function buildReceiptCheck(blockers) {
  return {
    id: "next-rust.vendor-boundary.receipts",
    title: "Next/Turbopack Rust vendor boundary receipts are fresh",
    status: checkStatus(blockers.length === 0),
    severity: "blocking",
    messages: blockers,
    evidencePaths: RECEIPT_EVIDENCE_PATHS,
  };
}

function buildRuntimeTakeoverCheck(boundary) {
  const evidence = {
    runtimeTakeoverBlocked: boundary.runtimeTakeoverBlocked,
    nextRuntimeRequired: boundary.nextRuntimeRequired,
    reactRscRequired: boundary.reactRscRequired,
    nodeNapiRequired: boundary.nodeNapiRequired,
    nodeModulesDefault: boundary.nodeModulesDefault,
    turbopackPublicArchitecture: boundary.turbopackPublicArchitecture,
  };
  const messages = Object.entries(evidence)
    .filter(([key, value]) => {
      return key === "runtimeTakeoverBlocked" ? value !== true : value !== false;
    })
    .map(([key, value]) => `${key}:${value}`);

  return {
    id: "next-rust.vendor-boundary.runtime-takeover",
    title: "Vendored Next/Turbopack Rust does not replace DX runtime foundations",
    status: checkStatus(messages.length === 0),
    severity: "blocking",
    messages,
    evidence,
  };
}

function buildPublicClaimsCheck(boundary) {
  const evidence = {
    publicDependencyClaimsBlocked: boundary.publicDependencyClaimsBlocked,
    vendoredCargoDependencyClaimsBlocked: boundary.vendoredCargoDependencyClaimsBlocked,
    vendorSourceInclusionBlocked: boundary.vendorSourceInclusionBlocked,
    publicSourceExposureBlocked: boundary.publicSourceExposureBlocked,
  };
  const messages = Object.entries(evidence)
    .filter(([, value]) => value !== true)
    .map(([key, value]) => `${key}:${value}`);

  return {
    id: "next-rust.vendor-boundary.public-claims",
    title: "Public dependency and source claims stay DX-owned",
    status: checkStatus(messages.length === 0),
    severity: "blocking",
    messages,
    evidence,
  };
}

function buildDxCheckProjection(snapshot, blockers, activeScope) {
  const checks = [
    buildReceiptCheck(blockers),
    buildRuntimeTakeoverCheck(snapshot.boundary),
    buildPublicClaimsCheck(snapshot.boundary),
    buildActiveScopeCheck(activeScope),
  ];

  return {
    schema: SCHEMAS.dxCheckProjection,
    status: checkStatus(checks.every((check) => check.status === "ok")),
    adapterBoundary: "source-owned JSON projection for dx-check receipt consumers; native renderer unimplemented",
    checks,
  };
}

module.exports = {
  buildDxCheckProjection,
};
