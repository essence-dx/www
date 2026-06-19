const {
  summarizeCssAssetOutputProof,
  summarizeNodeModulesProof,
  summarizeOutputProofSummary,
} = require("./proof.ts");

const OUTPUT_PROOF_FAILURE_MESSAGES = {
  "build-artifacts-trusted": "build artifacts are not trusted for output proof",
  "style-output-ignored": "stylesheet output proof was ignored because artifacts are untrusted",
  "public-asset-output-ignored": "public asset output proof was ignored because artifacts are untrusted",
  "node-modules-proof-ignored": "node_modules proof was ignored because artifacts are untrusted",
  "style-output-present": "manifest-declared stylesheet was not emitted",
  "style-source-path": "stylesheet output is missing its source path",
  "style-source-output-path": "stylesheet output is missing its source-owned output path",
  "style-output-path": "stylesheet output is missing its emitted output path",
  "style-output-path-safe": "stylesheet output path is not a safe project-relative manifest path",
  "style-output-path-no-node-modules": "stylesheet output path contains a node_modules segment",
  "style-source-output-match": "stylesheet output path does not match the source-owned output path",
  "style-output-hash": "stylesheet output is missing its content hash",
  "style-output-hash-matches-artifact": "stylesheet output hash does not match emitted bytes",
  "style-source-map-present": "stylesheet source map was not emitted",
  "style-source-map-path-safe":
    "stylesheet source map path is not a safe project-relative manifest path",
  "style-source-map-path-no-node-modules": "stylesheet source map path contains a node_modules segment",
  "style-source-map-linked": "stylesheet manifest does not declare source-map linkage",
  "style-source-map-linked-in-css": "stylesheet output does not link its source map",
  "style-source-map-hash": "stylesheet source map is missing its content hash",
  "style-source-map-hash-matches-artifact": "stylesheet source map hash does not match emitted bytes",
  "style-source-map-sources": "stylesheet source map is missing source evidence",
  "style-source-map-json-valid": "stylesheet source map is not valid JSON",
  "style-source-map-artifact-sources": "stylesheet source map artifact is missing source entries",
  "style-source-map-source-path": "stylesheet source map does not reference its manifest source path",
  "style-source-map-safe-sources": "stylesheet source map references unsafe source paths",
  "style-no-node-modules": "stylesheet output does not declare node_modules_required=false",
  "style-node-modules-not-required": "stylesheet output declares node_modules_required=true",
  "style-no-lifecycle-scripts": "stylesheet output executed lifecycle scripts",
  "style-source-owned": "stylesheet output does not declare source_owned_contract=true",
  "style-no-external-runtime": "stylesheet output requires or executed an external runtime",
  "public-asset-present": "manifest-declared public asset was not emitted",
  "public-asset-source-path": "public asset output is missing its source path",
  "public-asset-source-output-path": "public asset output is missing its source-owned output path",
  "public-asset-output-path": "public asset output is missing its emitted output path",
  "public-asset-output-path-safe":
    "public asset output path is not a safe project-relative manifest path",
  "public-asset-output-path-no-node-modules": "public asset output path contains a node_modules segment",
  "public-asset-source-public-path": "public asset source path is not under public/",
  "public-asset-output-public-root": "public asset output is not under .dx/build/public/",
  "public-asset-output-source-derivative":
    "public asset output path is not a hashed source-owned derivative of its source path",
  "public-asset-hash": "public asset output is missing its content hash",
  "public-asset-hash-matches-artifact": "public asset hash does not match emitted bytes",
  "public-asset-hashed-filename": "public asset output filename is missing its content hash",
  "public-asset-no-node-modules": "public asset output does not declare node_modules_required=false",
  "public-asset-node-modules-not-required": "public asset output declares node_modules_required=true",
  "public-asset-no-lifecycle-scripts": "public asset output executed lifecycle scripts",
  "public-asset-source-owned": "public asset output does not declare source_owned_contract=true",
  "public-asset-no-external-runtime": "public asset output requires or executed an external runtime",
  "public-asset-size-match": "public asset output size does not match the manifest",
  "node-modules-build-command-executed": "dx build command was not executed",
  "node-modules-build-succeeded": "dx build command did not exit successfully",
  "node-modules-not-present": "node_modules is present after dx build",
  "node-modules-not-created": "dx build created node_modules",
  "node-modules-no-before-paths": "node_modules existed before dx build",
  "node-modules-no-created-paths": "node_modules paths were created during dx build",
  "node-modules-no-after-paths": "node_modules paths are present after dx build",
};

function outputProofFailures(report) {
  const summary =
    report.outputProofSummary ||
    summarizeOutputProofSummary({
      cssAssetOutputProof: summarizeCssAssetOutputProof(report),
      nodeModulesProof: summarizeNodeModulesProof(report),
    });

  return Array.isArray(summary.missingChecks)
    ? summary.missingChecks.map(outputProofFailureMessage)
    : [];
}

function outputProofFailureMessage(check) {
  const detail = OUTPUT_PROOF_FAILURE_MESSAGES[check] || "unknown output proof check failed";
  return `source-build output proof failed: ${check} (${detail})`;
}

module.exports = { outputProofFailures };
