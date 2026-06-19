function summarizeStyleOutputSummary(output) {
  const ignored = output.ignored === true;
  return {
    eligible: output.eligible === true,
    ignored,
    ignoreReason: stringOrNull(output.ignoreReason),
    present: output.present === true,
    sourcePath: stringOrNull(output.sourcePath),
    sourceOutputPath: stringOrNull(output.sourceOutputPath),
    outputPath: stringOrNull(output.outputPath),
    outputPathSafe: booleanOrNull(output.outputPathSafe),
    outputPathUnsafeReason: stringOrNull(output.outputPathUnsafeReason),
    sourceOutputPathMatchesOutput: output.sourceOutputPathMatchesOutput === true,
    path: stringOrNull(output.path),
    hash: stringOrNull(output.hash),
    hashAlgorithm: stringOrNull(output.hashAlgorithm),
    hashAlgorithmInferred: output.hashAlgorithmInferred === true,
    hashMatchesOutput: booleanOrNull(output.hashMatchesOutput),
    hasHash: output.hasHash === true,
    sourceMapPath: stringOrNull(output.sourceMapPath),
    sourceMapOutputPathSafe: booleanOrNull(output.sourceMapOutputPathSafe),
    sourceMapOutputPathUnsafeReason: stringOrNull(output.sourceMapOutputPathUnsafeReason),
    sourceMapPresent: output.sourceMapPresent === true,
    sourceMapLinked: output.sourceMapLinked === true,
    sourceMapLinkedInCss: output.sourceMapLinkedInCss === true,
    hasSourceMapHash: output.hasSourceMapHash === true,
    sourceMapHash: stringOrNull(output.sourceMapHash),
    sourceMapHashAlgorithm: stringOrNull(output.sourceMapHashAlgorithm),
    sourceMapHashAlgorithmInferred: output.sourceMapHashAlgorithmInferred === true,
    sourceMapHashMatchesArtifact: booleanOrNull(output.sourceMapHashMatchesArtifact),
    sourceMapJsonValid: output.sourceMapJsonValid === true,
    sourceMapSourceCount: Number.isInteger(output.sourceMapSourceCount)
      ? output.sourceMapSourceCount
      : 0,
    sourceMapArtifactSourceCount: Number.isInteger(output.sourceMapArtifactSourceCount)
      ? output.sourceMapArtifactSourceCount
      : 0,
    sourceMapIncludesSourcePath: booleanOrNull(output.sourceMapIncludesSourcePath),
    sourceMapHasUnsafeSources: output.sourceMapHasUnsafeSources === true,
    sourceMapUnsafeSourceCount: Number.isInteger(output.sourceMapUnsafeSourceCount)
      ? output.sourceMapUnsafeSourceCount
      : 0,
    declaresNoNodeModules: output.declaresNoNodeModules === true,
    nodeModulesRequired: output.nodeModulesRequired === true,
    lifecycleScriptsExecuted: output.lifecycleScriptsExecuted === true,
    sourceOwnedContract: output.sourceOwnedContract === true,
    externalRuntimeRequired: output.externalRuntimeRequired === true,
    externalRuntimeExecuted: output.externalRuntimeExecuted === true,
    missingChecks: ignored ? ["style-output-ignored"] : missingChecks([
      ["style-output-present", output.present === true],
      ["style-source-path", nonEmptyString(output.sourcePath)],
      ["style-source-output-path", nonEmptyString(output.sourceOutputPath)],
      ["style-output-path", nonEmptyString(output.outputPath || output.path)],
      ["style-output-path-safe", output.outputPathSafe !== false],
      ["style-output-path-no-node-modules", output.outputPathUnsafeReason !== "node-modules-segment"],
      ["style-source-output-match", output.sourceOutputPathMatchesOutput === true],
      ["style-output-hash", output.hasHash === true],
      ["style-output-hash-matches-artifact", output.hashMatchesOutput !== false],
      ["style-source-map-present", output.sourceMapPresent === true],
      ["style-source-map-path-safe", output.sourceMapOutputPathSafe !== false],
      [
        "style-source-map-path-no-node-modules",
        output.sourceMapOutputPathUnsafeReason !== "node-modules-segment",
      ],
      ["style-source-map-linked", output.sourceMapLinked === true],
      ["style-source-map-linked-in-css", output.sourceMapLinkedInCss === true],
      ["style-source-map-hash", output.hasSourceMapHash === true],
      ["style-source-map-hash-matches-artifact", output.sourceMapHashMatchesArtifact !== false],
      ["style-source-map-sources", Number.isInteger(output.sourceMapSourceCount) && output.sourceMapSourceCount > 0],
      ["style-source-map-json-valid", output.sourceMapJsonValid === true],
      ["style-source-map-artifact-sources", output.sourceMapHasSources === true],
      ["style-source-map-source-path", output.sourceMapIncludesSourcePath !== false],
      ["style-source-map-safe-sources", output.sourceMapHasUnsafeSources !== true],
      ["style-no-node-modules", output.declaresNoNodeModules === true],
      ["style-node-modules-not-required", output.nodeModulesRequired !== true],
      ["style-no-lifecycle-scripts", output.lifecycleScriptsExecuted === false],
      ["style-source-owned", output.sourceOwnedContract === true],
      [
        "style-no-external-runtime",
        output.externalRuntimeRequired !== true && output.externalRuntimeExecuted !== true,
      ],
    ]),
  };
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
  summarizeStyleOutputSummary,
};
