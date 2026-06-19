const { outputTail } = require("./command-summary.ts");

function summarizeArtifactTrust(input) {
  if (binaryIdentityFailures(input.binaryIdentity).length > 0) {
    return untrustedArtifacts("binary-unavailable");
  }
  if (input.binarySourceFreshness && input.binarySourceFreshness.fresh === false) {
    return untrustedArtifacts("stale-binary");
  }
  if (input.build && input.build.skipped === true) {
    return untrustedArtifacts(input.build.skipReason || "dx-build-skipped");
  }
  if (!input.build || input.build.status !== 0) {
    return untrustedArtifacts("dx-build-failed");
  }
  return {
    trusted: true,
    reason: null,
    staleArtifactRisk: false,
    summariesReadFromDisk: true,
  };
}

function binaryIdentityFailures(binaryIdentity) {
  if (!binaryIdentity || binaryIdentity.present !== true || binaryIdentity.kind !== "file") {
    return ["dx build binary is missing or not executable"];
  }
  return [];
}

function sourceFreshnessFailures(binarySourceFreshness) {
  return binarySourceFreshness && binarySourceFreshness.fresh === false
    ? ["dx build binary is stale relative to source changes"]
    : [];
}

function summarizeEnvironmentBlocker(input) {
  const buildOutput = `${input.build?.stderr || ""}\n${input.build?.stdout || ""}`;
  if (/not enough space|os error 112|ENOSPC/i.test(buildOutput)) {
    return {
      kind: "disk-space",
      command: "dx build",
      failure: "build environment has insufficient disk space",
      message: "The host reported insufficient disk space while writing build output.",
      evidence: outputTail(buildOutput),
    };
  }
  return null;
}

function untrustedArtifacts(reason) {
  return {
    trusted: false,
    reason,
    staleArtifactRisk: true,
    summariesReadFromDisk: true,
  };
}

module.exports = {
  binaryIdentityFailures,
  sourceFreshnessFailures,
  summarizeArtifactTrust,
  summarizeEnvironmentBlocker,
};
