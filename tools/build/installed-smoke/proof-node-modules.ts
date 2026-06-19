function summarizeNodeModulesProof(report) {
  const build = report?.build || {};
  const artifactTrust = summarizeArtifactTrust(build.artifactTrust);
  const beforePaths = pathList(build.nodeModulesBeforePaths);
  const createdPaths = pathList(build.nodeModulesCreatedPaths);
  const afterPaths = pathList(build.nodeModulesPaths);

  if (artifactTrust && artifactTrust.trusted !== true) {
    return {
      required: true,
      eligible: false,
      ignored: true,
      ignoreReason: artifactTrust.reason || "build-artifacts-untrusted",
      artifactTrust,
      commandExecuted: false,
      buildExitCode: null,
      nodeModulesPresent: build.nodeModulesPresent === true,
      nodeModulesCreated: build.nodeModulesCreated === true,
      nodeModulesBeforePaths: beforePaths,
      nodeModulesCreatedPaths: createdPaths,
      nodeModulesPaths: afterPaths,
    };
  }

  const commandExecuted = build.command?.skipped !== true &&
    Number.isInteger(build.command?.exitCode);
  const buildExitCode = Number.isInteger(build.exitCode) ? build.exitCode : null;

  return {
    required: true,
    eligible: commandExecuted &&
      buildExitCode === 0 &&
      build.nodeModulesPresent !== true &&
      build.nodeModulesCreated !== true &&
      beforePaths.length === 0 &&
      createdPaths.length === 0 &&
      afterPaths.length === 0,
    ignored: false,
    ignoreReason: null,
    artifactTrust,
    commandExecuted,
    buildExitCode,
    nodeModulesPresent: build.nodeModulesPresent === true,
    nodeModulesCreated: build.nodeModulesCreated === true,
    nodeModulesBeforePaths: beforePaths,
    nodeModulesCreatedPaths: createdPaths,
    nodeModulesPaths: afterPaths,
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

function pathList(paths) {
  return Array.isArray(paths)
    ? paths.filter((item) => typeof item === "string" && item.length > 0)
    : [];
}

function stringOrNull(value) {
  return typeof value === "string" ? value : null;
}

module.exports = {
  summarizeNodeModulesProof,
};
