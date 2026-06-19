const { summarizePublicAssetOutputProof } = require("./proof-public-assets.ts");
const { summarizeStyleOutputProof } = require("./proof-css-style-output.ts");

function summarizeCssAssetOutputProof(report) {
  const artifactTrust = report?.build?.artifactTrust || null;
  if (artifactTrust && artifactTrust.trusted !== true) {
    return summarizeIgnoredCssAssetOutputProof(artifactTrust);
  }

  const manifest = report?.build?.sourceBuild?.manifest || {};
  return {
    required: true,
    artifactTrust: summarizeArtifactTrust(artifactTrust),
    styleOutput: summarizeStyleOutputProof(manifest.styleOutput || {}),
    publicAssetOutput: summarizePublicAssetOutputProof(manifest.publicAssetOutput || {}),
  };
}

function summarizeIgnoredCssAssetOutputProof(artifactTrust) {
  const trust = summarizeArtifactTrust(artifactTrust);
  return {
    required: true,
    artifactTrust: trust,
    styleOutput: ignoredOutput(trust.reason),
    publicAssetOutput: ignoredOutput(trust.reason),
  };
}

function ignoredOutput(reason) {
  return {
    eligible: false,
    ignored: true,
    ignoreReason: reason || "build-artifacts-untrusted",
  };
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
  summarizeCssAssetOutputProof,
};
