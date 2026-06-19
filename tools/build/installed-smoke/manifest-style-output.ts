const fs = require("node:fs");
const path = require("node:path");

const { hashMatchesArtifact } = require("./artifact-hash.ts");
const { summarizeManifestHashAlgorithm } = require("./manifest-hash-algorithm.ts");
const {
  declaredOutput,
  sourceOwnedOutputPath,
} = require("./manifest-output-paths.ts");
const { summarizeSourceMapArtifact } = require("./manifest-output-source-map.ts");

function summarizeStyleOutput(root, entries, sourcePath) {
  const entry = manifestEntry(entries, sourcePath);
  const output = declaredOutput(root, entry?.output);
  const sourceMap = declaredOutput(root, entry?.source_map_output);
  const source = stringOrNull(entry?.path);
  const sourceOutputPath = sourceOwnedOutputPath(source);
  const cssText = output.present ? fs.readFileSync(output.absolutePath, "utf8") : "";
  const sourceMapArtifact = summarizeSourceMapArtifact(sourceMap.absolutePath, sourceMap.present);
  const sourceMapFileName =
    typeof entry?.source_map_output === "string" ? path.posix.basename(entry.source_map_output) : null;
  const styleHashAlgorithm = summarizeManifestHashAlgorithm(
    entry,
    entry?.hash_algorithm || entry?.hashAlgorithm,
    entry?.hash,
  );
  const sourceMapHashAlgorithm = summarizeManifestHashAlgorithm(
    entry,
    entry?.source_map_hash_algorithm || entry?.sourceMapHashAlgorithm,
    entry?.source_map_hash,
  );

  return {
    sourcePath: source,
    sourceOutputPath,
    path: output.path,
    outputPath: output.path,
    present: output.present,
    outputPathSafe: output.pathSafe,
    outputPathUnsafeReason: output.pathUnsafeReason,
    sourceOutputPathMatchesOutput:
      Boolean(sourceOutputPath) && Boolean(output.path) && sourceOutputPath === output.path,
    hash: stringOrNull(entry?.hash),
    hasHash: nonEmptyString(entry?.hash),
    hashAlgorithm: styleHashAlgorithm.algorithm,
    hashAlgorithmInferred: styleHashAlgorithm.inferred,
    hashMatchesOutput: hashMatchesArtifact(
      entry?.hash,
      styleHashAlgorithm.algorithm,
      output,
    ),
    sourceMapPath: sourceMap.path,
    sourceMapPresent: sourceMap.present,
    sourceMapOutputPathSafe: sourceMap.pathSafe,
    sourceMapOutputPathUnsafeReason: sourceMap.pathUnsafeReason,
    sourceMapLinked: entry?.source_map_linked === true,
    sourceMapLinkedInCss:
      Boolean(sourceMapFileName) && cssText.includes(`sourceMappingURL=${sourceMapFileName}`),
    sourceMapHash: stringOrNull(entry?.source_map_hash),
    hasSourceMapHash: nonEmptyString(entry?.source_map_hash),
    sourceMapHashAlgorithm: sourceMapHashAlgorithm.algorithm,
    sourceMapHashAlgorithmInferred: sourceMapHashAlgorithm.inferred,
    sourceMapHashMatchesArtifact: hashMatchesArtifact(
      entry?.source_map_hash,
      sourceMapHashAlgorithm.algorithm,
      sourceMap,
    ),
    sourceMapSourceCount: Number.isInteger(entry?.source_map_source_count)
      ? entry.source_map_source_count
      : 0,
    sourceMapJsonValid: sourceMapArtifact.jsonValid,
    sourceMapArtifactSourceCount: sourceMapArtifact.sourceCount,
    sourceMapSources: sourceMapArtifact.sources,
    sourceMapHasSources: sourceMapArtifact.sourceCount > 0,
    sourceMapIncludesSourcePath: sourceMapIncludesSourcePath(sourceMapArtifact.sources, source),
    sourceMapUnsafeSourceCount: sourceMapArtifact.unsafeSourceCount,
    sourceMapUnsafeSources: sourceMapArtifact.unsafeSources,
    sourceMapHasUnsafeSources: sourceMapArtifact.unsafeSourceCount > 0,
    sourceMapError: sourceMapArtifact.error,
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

function sourceMapIncludesSourcePath(sources, sourcePath) {
  const expected = normalizeSourceMapSource(sourcePath);
  if (!expected || !Array.isArray(sources)) {
    return false;
  }
  return sources.some((source) => {
    const normalized = normalizeSourceMapSource(source);
    return normalized === expected || normalized.endsWith(`/${expected}`);
  });
}

function normalizeSourceMapSource(source) {
  if (typeof source !== "string" || source.length === 0) {
    return "";
  }
  const withoutScheme = source.replace(/^[a-z][a-z0-9+.-]*:\/+/i, "");
  const normalized = path.posix.normalize(withoutScheme.replaceAll("\\", "/"));
  return normalized === "." ? "" : normalized.replace(/^\.\//, "");
}

module.exports = { summarizeStyleOutput };
