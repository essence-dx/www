const {
  declaredOutput,
  isPublicAssetOutputPath,
  isPublicAssetSourcePath,
  isSafeManifestPath,
  publicAssetOutputPathMatchesSourceOwnedAssetPath,
} = require("./manifest-output-paths.ts");
const { summarizeAssetOutput } = require("./manifest-asset-output.ts");
const { summarizeStyleOutput } = require("./manifest-style-output.ts");

function manifestDeclaredOutput(root, entries, sourcePath) {
  const entry = manifestEntry(entries, sourcePath);
  const output = declaredOutput(root, entry?.output);
  return {
    path: output.path,
    present: output.present,
  };
}

function manifestEntry(entries, sourcePath) {
  return Array.isArray(entries) ? entries.find((item) => item.path === sourcePath) || null : null;
}

module.exports = {
  isPublicAssetOutputPath,
  isPublicAssetSourcePath,
  isSafeManifestPath,
  manifestDeclaredOutput,
  publicAssetOutputPathMatchesSourceOwnedAssetPath,
  summarizeAssetOutput,
  summarizeStyleOutput,
};
