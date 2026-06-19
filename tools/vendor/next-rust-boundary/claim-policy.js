const { SCHEMAS } = require("./constants.js");

function buildClaimPolicy() {
  return {
    schema: SCHEMAS.claimPolicy,
    allowedClaimScope: [
      "quarantined provenance",
      "license evidence",
      "selected Rust build infrastructure reference",
      "DX-owned receipt handoff",
    ],
    unprovenClaims: [
      "full Next.js parity",
      "Next.js runtime takeover",
      "React/RSC core app model",
      "Node/NAPI default foundation",
      "node_modules default resolver",
      "Turbopack public architecture",
    ],
    fullNextParityClaimed: false,
    nextRuntimeTakeoverClaimed: false,
    reactRscCoreDependencyClaimed: false,
    nodeNapiFoundationClaimed: false,
    nodeModulesDefaultClaimed: false,
    turbopackPublicArchitectureClaimed: false,
  };
}

module.exports = {
  buildClaimPolicy,
};
