function summarizePublicAssetOutputSummary(output) {
  const sizeMatchesOutput = booleanOrNull(output.sizeMatchesOutput);
  const ignored = output.ignored === true;

  return {
    eligible: output.eligible === true,
    ignored,
    ignoreReason: stringOrNull(output.ignoreReason),
    present: output.present === true,
    sourcePath: stringOrNull(output.sourcePath),
    sourceOutputPath: stringOrNull(output.sourceOutputPath),
    path: stringOrNull(output.path),
    outputPath: stringOrNull(output.outputPath),
    outputPathSafe: booleanOrNull(output.outputPathSafe),
    outputPathUnsafeReason: stringOrNull(output.outputPathUnsafeReason),
    hash: stringOrNull(output.hash),
    hashAlgorithm: stringOrNull(output.hashAlgorithm),
    hashAlgorithmInferred: output.hashAlgorithmInferred === true,
    hashMatchesOutput: booleanOrNull(output.hashMatchesOutput),
    hasHash: output.hasHash === true,
    outputFileNameContainsHash: output.outputFileNameContainsHash === true,
    sourcePathIsPublicAsset: output.sourcePathIsPublicAsset === true,
    outputPathIsPublicAsset: output.outputPathIsPublicAsset === true,
    outputPathMatchesSourceOwnedAssetPath: output.outputPathMatchesSourceOwnedAssetPath === true,
    declaresNoNodeModules: output.declaresNoNodeModules === true,
    nodeModulesRequired: output.nodeModulesRequired === true,
    lifecycleScriptsExecuted: output.lifecycleScriptsExecuted === true,
    sourceOwnedContract: output.sourceOwnedContract === true,
    externalRuntimeRequired: output.externalRuntimeRequired === true,
    externalRuntimeExecuted: output.externalRuntimeExecuted === true,
    sizeMatchesOutput,
    size: Number.isInteger(output.size) ? output.size : null,
    outputByteLength: Number.isInteger(output.outputByteLength) ? output.outputByteLength : null,
    missingChecks: ignored ? ["public-asset-output-ignored"] : missingChecks([
      ["public-asset-present", output.present === true],
      ["public-asset-source-path", nonEmptyString(output.sourcePath)],
      ["public-asset-source-output-path", nonEmptyString(output.sourceOutputPath)],
      ["public-asset-output-path", nonEmptyString(output.outputPath || output.path)],
      ["public-asset-output-path-safe", output.outputPathSafe !== false],
      [
        "public-asset-output-path-no-node-modules",
        output.outputPathUnsafeReason !== "node-modules-segment",
      ],
      ["public-asset-source-public-path", output.sourcePathIsPublicAsset === true],
      ["public-asset-output-public-root", output.outputPathIsPublicAsset === true],
      [
        "public-asset-output-source-derivative",
        output.outputPathMatchesSourceOwnedAssetPath === true,
      ],
      ["public-asset-hash", output.hasHash === true],
      ["public-asset-hash-matches-artifact", output.hashMatchesOutput !== false],
      ["public-asset-hashed-filename", output.outputFileNameContainsHash === true],
      ["public-asset-no-node-modules", output.declaresNoNodeModules === true],
      ["public-asset-node-modules-not-required", output.nodeModulesRequired !== true],
      ["public-asset-no-lifecycle-scripts", output.lifecycleScriptsExecuted === false],
      ["public-asset-source-owned", output.sourceOwnedContract === true],
      [
        "public-asset-no-external-runtime",
        output.externalRuntimeRequired !== true && output.externalRuntimeExecuted !== true,
      ],
      ["public-asset-size-match", publicAssetSizeCheckPassed(output)],
    ]),
  };
}

function publicAssetSizeCheckPassed(output) {
  return output.present !== true || output.sizeMatchesOutput === true;
}

function missingChecks(checks) {
  return checks.filter(([, passed]) => passed !== true).map(([name]) => name);
}

function stringOrNull(value) {
  return typeof value === "string" ? value : null;
}

function booleanOrNull(value) {
  return typeof value === "boolean" ? value : null;
}

function nonEmptyString(value) {
  return typeof value === "string" && value.length > 0;
}

module.exports = {
  summarizePublicAssetOutputSummary,
};
