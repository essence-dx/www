const { manifestPathSafety } = require("./manifest-output-paths.ts");

function summarizeStyleOutputProof(output) {
  const sourceMapSourceCount = Number.isInteger(output.sourceMapSourceCount)
    ? output.sourceMapSourceCount
    : 0;
  const hashMatchesOutput = booleanOrNull(output.hashMatchesOutput);
  const sourceMapHashMatchesArtifact = booleanOrNull(output.sourceMapHashMatchesArtifact);
  const outputPath = stringOrNull(output.outputPath || output.path);
  const sourceMapPath = stringOrNull(output.sourceMapPath);
  const outputPathSafety = pathSafetyDetails(
    outputPath,
    output.outputPathSafe,
    output.outputPathUnsafeReason,
  );
  const sourceMapOutputPathSafety = pathSafetyDetails(
    sourceMapPath,
    output.sourceMapOutputPathSafe,
    output.sourceMapOutputPathUnsafeReason,
  );
  const outputPathSafe = outputPathSafety.safe;
  const sourceMapOutputPathSafe = sourceMapOutputPathSafety.safe;

  return {
    eligible: styleOutputEligible(output, {
      hashMatchesOutput,
      outputPathSafe,
      sourceMapHashMatchesArtifact,
      sourceMapOutputPathSafe,
      sourceMapSourceCount,
    }),
    sourcePath: stringOrNull(output.sourcePath),
    sourceOutputPath: stringOrNull(output.sourceOutputPath),
    outputPath,
    outputPathSafe,
    outputPathUnsafeReason: outputPathSafety.unsafeReason,
    sourceOutputPathMatchesOutput: output.sourceOutputPathMatchesOutput === true,
    present: output.present === true,
    path: stringOrNull(output.path),
    hash: stringOrNull(output.hash),
    hashAlgorithm: stringOrNull(output.hashAlgorithm),
    hashAlgorithmInferred: output.hashAlgorithmInferred === true,
    hashMatchesOutput,
    sourceMapPath,
    sourceMapOutputPathSafe,
    sourceMapOutputPathUnsafeReason: sourceMapOutputPathSafety.unsafeReason,
    hasHash: output.hasHash === true,
    sourceMapPresent: output.sourceMapPresent === true,
    sourceMapLinked: output.sourceMapLinked === true,
    sourceMapLinkedInCss: output.sourceMapLinkedInCss === true,
    hasSourceMapHash: output.hasSourceMapHash === true,
    sourceMapHash: stringOrNull(output.sourceMapHash),
    sourceMapHashAlgorithm: stringOrNull(output.sourceMapHashAlgorithm),
    sourceMapHashAlgorithmInferred: output.sourceMapHashAlgorithmInferred === true,
    sourceMapHashMatchesArtifact,
    sourceMapSourceCount,
    sourceMapJsonValid: output.sourceMapJsonValid === true,
    sourceMapArtifactSourceCount: Number.isInteger(output.sourceMapArtifactSourceCount)
      ? output.sourceMapArtifactSourceCount
      : 0,
    sourceMapHasSources: output.sourceMapHasSources === true,
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
  };
}

function styleOutputEligible(output, proof) {
  return output.present === true &&
    nonEmptyString(output.sourcePath) &&
    nonEmptyString(output.sourceOutputPath) &&
    nonEmptyString(output.outputPath || output.path) &&
    proof.outputPathSafe !== false &&
    output.sourceOutputPathMatchesOutput === true &&
    output.hasHash === true &&
    proof.hashMatchesOutput !== false &&
    output.sourceMapPresent === true &&
    proof.sourceMapOutputPathSafe !== false &&
    output.sourceMapLinked === true &&
    output.sourceMapLinkedInCss === true &&
    output.hasSourceMapHash === true &&
    proof.sourceMapHashMatchesArtifact !== false &&
    proof.sourceMapSourceCount > 0 &&
    output.sourceMapJsonValid === true &&
    output.sourceMapHasSources === true &&
    output.sourceMapIncludesSourcePath !== false &&
    output.sourceMapHasUnsafeSources !== true &&
    output.declaresNoNodeModules === true &&
    output.lifecycleScriptsExecuted === false &&
    output.sourceOwnedContract === true &&
    output.externalRuntimeRequired === false &&
    output.externalRuntimeExecuted === false;
}

function stringOrNull(value) {
  return typeof value === "string" ? value : null;
}

function booleanOrNull(value) {
  return typeof value === "boolean" ? value : null;
}

function pathSafetyDetails(value, explicit, explicitUnsafeReason) {
  const explicitReason = stringOrNull(explicitUnsafeReason);
  if (typeof explicit === "boolean") {
    return {
      safe: explicit,
      unsafeReason: explicit ? null : explicitReason || derivedUnsafeReason(value),
    };
  }
  if (!nonEmptyString(value)) {
    return {
      safe: null,
      unsafeReason: null,
    };
  }

  const safety = manifestPathSafety(value);
  return {
    safe: safety.safe,
    unsafeReason: safety.unsafeReason,
  };
}

function derivedUnsafeReason(value) {
  return nonEmptyString(value) ? manifestPathSafety(value).unsafeReason : null;
}

function nonEmptyString(value) {
  return typeof value === "string" && value.length > 0;
}

module.exports = { summarizeStyleOutputProof };
