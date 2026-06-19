function summarizeNodeModulesProofSummary(proof) {
  const ignored = proof.ignored === true;

  return {
    required: proof.required === true,
    eligible: proof.eligible === true,
    ignored,
    ignoreReason: stringOrNull(proof.ignoreReason),
    artifactTrust: summarizeArtifactTrust(proof.artifactTrust),
    commandExecuted: proof.commandExecuted === true,
    buildExitCode: Number.isInteger(proof.buildExitCode) ? proof.buildExitCode : null,
    nodeModulesPresent: proof.nodeModulesPresent === true,
    nodeModulesCreated: proof.nodeModulesCreated === true,
    nodeModulesBeforePaths: stringList(proof.nodeModulesBeforePaths),
    nodeModulesCreatedPaths: stringList(proof.nodeModulesCreatedPaths),
    nodeModulesPaths: stringList(proof.nodeModulesPaths),
    missingChecks: ignored ? ["node-modules-proof-ignored"] : missingChecks([
      ["node-modules-build-command-executed", proof.commandExecuted === true],
      ["node-modules-build-succeeded", proof.buildExitCode === 0],
      ["node-modules-not-present", proof.nodeModulesPresent !== true],
      ["node-modules-not-created", proof.nodeModulesCreated !== true],
      ["node-modules-no-before-paths", stringList(proof.nodeModulesBeforePaths).length === 0],
      ["node-modules-no-created-paths", stringList(proof.nodeModulesCreatedPaths).length === 0],
      ["node-modules-no-after-paths", stringList(proof.nodeModulesPaths).length === 0],
    ]),
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

function missingChecks(checks) {
  return checks.filter(([, passed]) => passed !== true).map(([name]) => name);
}

function stringOrNull(value) {
  return typeof value === "string" ? value : null;
}

function stringList(value) {
  return Array.isArray(value)
    ? value.filter((item) => typeof item === "string" && item.length > 0)
    : [];
}

module.exports = {
  summarizeNodeModulesProofSummary,
};
