const fs = require("node:fs");

const { hashMatchesArtifact } = require("./artifact-hash.ts");
const { summarizeManifestHashAlgorithm } = require("./manifest-hash-algorithm.ts");
const {
  declaredOutput,
  isPublicAssetOutputPath,
  isPublicAssetSourcePath,
  manifestPathBaseName,
  publicAssetOutputPathMatchesSourceOwnedAssetPath,
  sourceOwnedOutputPath,
} = require("./manifest-output-paths.ts");

function summarizeAssetOutput(root, entries, sourcePath) {
  const entry = manifestEntry(entries, sourcePath);
  const output = declaredOutput(root, entry?.output);
  const source = stringOrNull(entry?.path);
  const sourceOutputPath = sourceOwnedOutputPath(source);
  const hash = stringOrNull(entry?.hash);
  const hashAlgorithm = summarizeManifestHashAlgorithm(
    entry,
    entry?.hash_algorithm || entry?.hashAlgorithm,
    entry?.hash,
  );
  const expectedSize = Number.isInteger(entry?.size) ? entry.size : null;
  const outputByteLength = output.present ? fs.statSync(output.absolutePath).size : null;

  return {
    sourcePath: source,
    sourceOutputPath,
    path: output.path,
    outputPath: output.path,
    present: output.present,
    hash,
    hasHash: nonEmptyString(hash),
    hashAlgorithm: hashAlgorithm.algorithm,
    hashAlgorithmInferred: hashAlgorithm.inferred,
    hashMatchesOutput: hashMatchesArtifact(
      hash,
      hashAlgorithm.algorithm,
      output,
    ),
    outputFileNameContainsHash: Boolean(hash) && manifestPathBaseName(output.path).includes(hash),
    outputPathSafe: output.pathSafe,
    outputPathUnsafeReason: output.pathUnsafeReason,
    sourcePathIsPublicAsset: isPublicAssetSourcePath(source),
    outputPathIsPublicAsset: isPublicAssetOutputPath(output.path),
    outputPathMatchesSourceOwnedAssetPath: publicAssetOutputPathMatchesSourceOwnedAssetPath(
      source,
      output.path,
      hash,
    ),
    size: expectedSize,
    outputByteLength,
    sizeMatchesOutput:
      expectedSize === null || outputByteLength === null ? null : expectedSize === outputByteLength,
    declaresNoNodeModules: entry?.node_modules_required === false,
    nodeModulesRequired: entry?.node_modules_required === true,
    lifecycleScriptsExecuted: entry?.lifecycle_scripts_executed === true,
    sourceOwnedContract: entry?.source_owned_contract === true,
    externalRuntimeRequired: entry?.external_runtime_required === true,
    externalRuntimeExecuted: entry?.external_runtime_executed === true,
  };
}

function manifestEntry(entries, sourcePath) {
  return Array.isArray(entries) ? entries.find((item) => item.path === sourcePath) || null : null;
}

function nonEmptyString(value) {
  return typeof value === "string" && value.length > 0;
}

function stringOrNull(value) {
  return typeof value === "string" ? value : null;
}

module.exports = { summarizeAssetOutput };
