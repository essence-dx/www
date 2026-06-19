const { summarizeNodeModulesProofSummary } = require("./proof-output-node-modules.ts");
const { summarizePublicAssetOutputSummary } = require("./proof-output-public-assets.ts");
const { summarizeStyleOutputSummary } = require("./proof-output-styles.ts");

function summarizeOutputProofSummary(proof) {
  const cssAssetOutputProof = proof?.cssAssetOutputProof || {};
  const artifactTrust = summarizeArtifactTrust(cssAssetOutputProof.artifactTrust);
  const styleOutput = summarizeStyleOutputSummary(cssAssetOutputProof.styleOutput || {});
  const publicAssetOutput = summarizePublicAssetOutputSummary(
    cssAssetOutputProof.publicAssetOutput || {},
  );
  const nodeModulesProof = hasNodeModulesProof(proof)
    ? summarizeNodeModulesProofSummary(proof.nodeModulesProof)
    : null;
  const trustMissingChecks =
    artifactTrust && artifactTrust.trusted === false ? ["build-artifacts-trusted"] : [];
  const missingChecks = [
    ...trustMissingChecks,
    ...styleOutput.missingChecks,
    ...publicAssetOutput.missingChecks,
    ...(nodeModulesProof ? nodeModulesProof.missingChecks : []),
  ];

  return {
    required: cssAssetOutputProof.required === true,
    eligible:
      styleOutput.eligible &&
      publicAssetOutput.eligible &&
      (nodeModulesProof ? nodeModulesProof.eligible : true),
    artifactTrust,
    styleOutput,
    publicAssetOutput,
    ...(nodeModulesProof ? { nodeModulesProof } : {}),
    missingChecks,
  };
}

function hasNodeModulesProof(proof) {
  return Boolean(
    proof &&
      Object.prototype.hasOwnProperty.call(proof, "nodeModulesProof") &&
      proof.nodeModulesProof &&
      typeof proof.nodeModulesProof === "object",
  );
}

function summarizeArtifactTrust(artifactTrust) {
  if (!artifactTrust || typeof artifactTrust !== "object") {
    return null;
  }
  return {
    trusted: artifactTrust.trusted === true,
    reason: stringOrNull(artifactTrust.reason),
    staleArtifactRisk: artifactTrust.staleArtifactRisk === true,
  };
}

function stringOrNull(value) {
  return typeof value === "string" ? value : null;
}

module.exports = {
  summarizeOutputProofSummary,
};
