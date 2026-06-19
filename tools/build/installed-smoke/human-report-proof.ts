function printCssAssetOutputProof(proof) {
  if (!proof || proof.required !== true) {
    return;
  }

  const styleOutput = proof.styleOutput || {};
  const publicAssetOutput = proof.publicAssetOutput || {};
  if (proof.artifactTrust && proof.artifactTrust.trusted !== true) {
    process.stdout.write(
      `Build artifact proof: ignored (${proof.artifactTrust.reason || "untrusted build artifacts"})\n`,
    );
  }

  process.stdout.write(
    `CSS output proof: ${formatEligibility(styleOutput.eligible)} (${formatPath(styleOutput.path)}; ` +
      `hash: ${formatYesNo(styleOutput.hasHash)}; ` +
      `source map: ${styleOutput.sourceMapPresent === true ? "present" : "missing"}; ` +
      `linked: ${formatYesNo(styleOutput.sourceMapLinked)}; ` +
      `linked in CSS: ${formatYesNo(styleOutput.sourceMapLinkedInCss)}; ` +
      `source-map hash: ${formatHashProof(styleOutput.hasSourceMapHash, styleOutput.sourceMapHash)}; ` +
      `source-map JSON: ${formatYesNo(styleOutput.sourceMapJsonValid)}; ` +
      `sources: ${formatSourceCount(styleOutput.sourceMapSourceCount)}; ` +
      `artifact sources: ${formatSourceCount(styleOutput.sourceMapArtifactSourceCount)}; ` +
      `no node_modules: ${formatYesNo(styleOutput.declaresNoNodeModules)}; ` +
      `lifecycle scripts executed: ${formatYesNo(styleOutput.lifecycleScriptsExecuted)}; ` +
      `source-owned: ${formatYesNo(styleOutput.sourceOwnedContract)}; ` +
      `external runtime required: ${formatYesNo(styleOutput.externalRuntimeRequired)}; ` +
      `external runtime executed: ${formatYesNo(styleOutput.externalRuntimeExecuted)})\n`,
  );

  process.stdout.write(
    `Public asset proof: ${formatEligibility(publicAssetOutput.eligible)} (${formatPath(publicAssetOutput.path)}; ` +
      `hash: ${formatYesNo(publicAssetOutput.hasHash)}; ` +
      `hashed filename: ${formatYesNo(publicAssetOutput.outputFileNameContainsHash)}; ` +
      `public source: ${formatYesNo(publicAssetOutput.sourcePathIsPublicAsset)}; ` +
      `public output: ${formatYesNo(publicAssetOutput.outputPathIsPublicAsset)}; ` +
      `source-derived: ${formatYesNo(publicAssetOutput.outputPathMatchesSourceOwnedAssetPath)}; ` +
      `no node_modules: ${formatYesNo(publicAssetOutput.declaresNoNodeModules)}; ` +
      `lifecycle scripts executed: ${formatYesNo(publicAssetOutput.lifecycleScriptsExecuted)}; ` +
      `source-owned: ${formatYesNo(publicAssetOutput.sourceOwnedContract)}; ` +
      `external runtime required: ${formatYesNo(publicAssetOutput.externalRuntimeRequired)}; ` +
      `external runtime executed: ${formatYesNo(publicAssetOutput.externalRuntimeExecuted)}; ` +
      `size: ${formatAssetSize(publicAssetOutput)}; ` +
      `size match: ${formatYesNo(publicAssetOutput.sizeMatchesOutput)})\n`,
  );
}

function printNodeModulesProof(proof) {
  if (!proof || proof.required !== true) {
    return;
  }

  if (proof.ignored === true) {
    process.stdout.write(
      `No-node_modules proof: ignored (${proof.ignoreReason || "untrusted build artifacts"})\n`,
    );
    return;
  }

  process.stdout.write(
    `No-node_modules proof: ${formatEligibility(proof.eligible)} (` +
      `command executed: ${formatYesNo(proof.commandExecuted)}; ` +
      `build exit: ${formatExitCode(proof.buildExitCode)}; ` +
      `present before: ${formatPathCount(proof.nodeModulesBeforePaths)}; ` +
      `present after: ${formatPathCount(proof.nodeModulesPaths)}; ` +
      `created: ${formatYesNo(proof.nodeModulesCreated)}; ` +
      `created paths: ${formatPathCount(proof.nodeModulesCreatedPaths)})\n`,
  );
}

function formatEligibility(value) {
  return value === true ? "eligible" : "not eligible";
}

function formatPath(value) {
  return typeof value === "string" && value.length > 0 ? value : "missing output";
}

function formatYesNo(value) {
  return value === true ? "yes" : "no";
}

function formatSourceCount(value) {
  return Number.isInteger(value) ? String(value) : "0";
}

function formatHashProof(hasHash, hash) {
  return typeof hash === "string" && hash.length > 0 ? hash : formatYesNo(hasHash);
}

function formatAssetSize(publicAssetOutput) {
  const expectedSize = Number.isInteger(publicAssetOutput.size) ? publicAssetOutput.size : null;
  const outputSize = Number.isInteger(publicAssetOutput.outputByteLength)
    ? publicAssetOutput.outputByteLength
    : null;

  if (expectedSize !== null && outputSize !== null) {
    return `${expectedSize}/${outputSize} bytes`;
  }
  if (expectedSize !== null) {
    return `expected ${expectedSize} bytes`;
  }
  if (outputSize !== null) {
    return `output ${outputSize} bytes`;
  }
  return "unknown";
}

function formatExitCode(value) {
  return Number.isInteger(value) ? String(value) : "not run";
}

function formatPathCount(paths) {
  return Array.isArray(paths) ? String(paths.length) : "0";
}

module.exports = {
  printCssAssetOutputProof,
  printNodeModulesProof,
};
