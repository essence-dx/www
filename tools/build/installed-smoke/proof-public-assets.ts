const {
  isPublicAssetOutputPath,
  isPublicAssetSourcePath,
  manifestPathSafety,
  publicAssetOutputPathMatchesSourceOwnedAssetPath,
} = require("./manifest-output-paths.ts");

function summarizePublicAssetOutputProof(output) {
  const sizeMatchesOutput = booleanOrNull(output.sizeMatchesOutput);
  const hashMatchesOutput = booleanOrNull(output.hashMatchesOutput);
  const sourcePath = stringOrNull(output.sourcePath);
  const sourceOutputPath = stringOrNull(output.sourceOutputPath);
  const outputPath = stringOrNull(output.outputPath || output.path);
  const hash = stringOrNull(output.hash);
  const outputPathSafety = pathSafetyDetails(
    outputPath,
    output.outputPathSafe,
    output.outputPathUnsafeReason,
  );
  const outputPathSafe = outputPathSafety.safe;
  const sourcePathIsPublicAsset = proofBoolean(
    output.sourcePathIsPublicAsset,
    isPublicAssetSourcePath(sourcePath),
  );
  const outputPathIsPublicAsset = proofBoolean(
    output.outputPathIsPublicAsset,
    isPublicAssetOutputPath(outputPath),
  );
  const outputPathMatchesSourceOwnedAssetPath = proofBoolean(
    output.outputPathMatchesSourceOwnedAssetPath,
    publicAssetOutputPathMatchesSourceOwnedAssetPath(sourcePath, outputPath, hash),
  );

  return {
    eligible: publicAssetOutputEligible(output, {
      hashMatchesOutput,
      outputPathSafe,
      outputPathIsPublicAsset,
      outputPathMatchesSourceOwnedAssetPath,
      sizeMatchesOutput,
      sourcePathIsPublicAsset,
    }),
    sourcePath,
    sourceOutputPath,
    outputPath,
    outputPathSafe,
    outputPathUnsafeReason: outputPathSafety.unsafeReason,
    present: output.present === true,
    path: stringOrNull(output.path),
    hasHash: output.hasHash === true,
    hash,
    hashAlgorithm: stringOrNull(output.hashAlgorithm),
    hashAlgorithmInferred: output.hashAlgorithmInferred === true,
    hashMatchesOutput,
    outputFileNameContainsHash: output.outputFileNameContainsHash === true,
    sourcePathIsPublicAsset,
    outputPathIsPublicAsset,
    outputPathMatchesSourceOwnedAssetPath,
    size: Number.isInteger(output.size) ? output.size : null,
    outputByteLength: Number.isInteger(output.outputByteLength) ? output.outputByteLength : null,
    declaresNoNodeModules: output.declaresNoNodeModules === true,
    nodeModulesRequired: output.nodeModulesRequired === true,
    lifecycleScriptsExecuted: output.lifecycleScriptsExecuted === true,
    sourceOwnedContract: output.sourceOwnedContract === true,
    externalRuntimeRequired: output.externalRuntimeRequired === true,
    externalRuntimeExecuted: output.externalRuntimeExecuted === true,
    sizeMatchesOutput,
  };
}

function publicAssetOutputEligible(output, proof) {
  return output.present === true &&
    nonEmptyString(output.sourcePath) &&
    nonEmptyString(output.sourceOutputPath) &&
    nonEmptyString(output.outputPath || output.path) &&
    proof.outputPathSafe !== false &&
    proof.sourcePathIsPublicAsset === true &&
    proof.outputPathIsPublicAsset === true &&
    proof.outputPathMatchesSourceOwnedAssetPath === true &&
    output.hasHash === true &&
    proof.hashMatchesOutput !== false &&
    output.outputFileNameContainsHash === true &&
    output.declaresNoNodeModules === true &&
    output.lifecycleScriptsExecuted === false &&
    output.sourceOwnedContract === true &&
    output.externalRuntimeRequired === false &&
    output.externalRuntimeExecuted === false &&
    proof.sizeMatchesOutput === true;
}

function proofBoolean(value, derivedValue) {
  return typeof value === "boolean" ? value : derivedValue === true;
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
  summarizePublicAssetOutputProof,
};
