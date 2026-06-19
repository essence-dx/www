import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const crypto = require("node:crypto");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const {
  summarizeAssetOutput,
  summarizeStyleOutput,
} = require("../tools/build/installed-smoke/manifest-output.ts");
const {
  summarizeOutputProofSummary,
  summarizeProof,
} = require("../tools/build/installed-smoke/proof.ts");

const repoRoot = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

test("installed smoke proves emitted CSS source-map and asset hash metadata", () => {
  const projectRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-installed-output-proof-"));
  write(
    projectRoot,
    ".dx/build/styles/app.css",
    ".hero{display:grid}\n/*# sourceMappingURL=app.css.map */",
  );
  write(projectRoot, ".dx/build/styles/app.css.map", '{"version":3,"sources":["styles/app.css"]}');
  write(projectRoot, ".dx/build/public/icons/mark-abc123.svg", "<svg />");

  const styleOutput = summarizeStyleOutput(
    projectRoot,
    [
      {
        path: "styles/app.css",
        output: ".dx/build/styles/app.css",
        hash: "css123",
        source_map_output: ".dx/build/styles/app.css.map",
        source_map_hash: "map123",
        source_map_linked: true,
        source_map_source_count: 2,
        node_modules_required: false,
        lifecycle_scripts_executed: false,
        source_owned_contract: true,
        external_runtime_required: false,
        external_runtime_executed: false,
      },
    ],
    "styles/app.css",
  );
  const assetOutput = summarizeAssetOutput(
    projectRoot,
    [
      {
        path: "public/icons/mark.svg",
        output: ".dx/build/public/icons/mark-abc123.svg",
        hash: "abc123",
        size: 7,
        node_modules_required: false,
        lifecycle_scripts_executed: false,
        source_owned_contract: true,
        external_runtime_required: false,
        external_runtime_executed: false,
      },
    ],
    "public/icons/mark.svg",
  );

  assert.equal(styleOutput.present, true);
  assert.equal(styleOutput.sourcePath, "styles/app.css");
  assert.equal(styleOutput.sourceOutputPath, ".dx/build/styles/app.css");
  assert.equal(styleOutput.outputPath, ".dx/build/styles/app.css");
  assert.equal(styleOutput.sourceOutputPathMatchesOutput, true);
  assert.equal(styleOutput.hasHash, true);
  assert.equal(styleOutput.sourceMapPresent, true);
  assert.equal(styleOutput.sourceMapLinked, true);
  assert.equal(styleOutput.sourceMapLinkedInCss, true);
  assert.equal(styleOutput.hasSourceMapHash, true);
  assert.equal(styleOutput.sourceMapSourceCount, 2);
  assert.equal(styleOutput.sourceMapJsonValid, true);
  assert.equal(styleOutput.sourceMapArtifactSourceCount, 1);
  assert.equal(styleOutput.sourceMapHasSources, true);
  assert.equal(styleOutput.declaresNoNodeModules, true);
  assert.equal(styleOutput.lifecycleScriptsExecuted, false);
  assert.equal(styleOutput.sourceOwnedContract, true);
  assert.equal(styleOutput.externalRuntimeRequired, false);
  assert.equal(styleOutput.externalRuntimeExecuted, false);

  assert.equal(assetOutput.present, true);
  assert.equal(assetOutput.sourcePath, "public/icons/mark.svg");
  assert.equal(assetOutput.sourceOutputPath, ".dx/build/public/icons/mark.svg");
  assert.equal(assetOutput.outputPath, ".dx/build/public/icons/mark-abc123.svg");
  assert.equal(assetOutput.hasHash, true);
  assert.equal(assetOutput.outputFileNameContainsHash, true);
  assert.equal(assetOutput.sourcePathIsPublicAsset, true);
  assert.equal(assetOutput.outputPathIsPublicAsset, true);
  assert.equal(assetOutput.outputPathMatchesSourceOwnedAssetPath, true);
  assert.equal(assetOutput.declaresNoNodeModules, true);
  assert.equal(assetOutput.lifecycleScriptsExecuted, false);
  assert.equal(assetOutput.sourceOwnedContract, true);
  assert.equal(assetOutput.externalRuntimeRequired, false);
  assert.equal(assetOutput.externalRuntimeExecuted, false);
  assert.equal(assetOutput.sizeMatchesOutput, true);
});

test("installed smoke rejects declared output hashes that do not match emitted bytes", () => {
  const projectRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-installed-output-proof-stale-hash-"));
  const css = ".hero{display:grid}\n/*# sourceMappingURL=app.css.map */";
  const sourceMap = '{"version":3,"sources":["styles/app.css"]}';
  const asset = "<svg />";
  const assetHash = sha256(asset);
  const assetBlake3Hash = "e261696593b3dbad";

  write(projectRoot, ".dx/build/styles/app.css", css);
  write(projectRoot, ".dx/build/styles/app.css.map", sourceMap);
  write(projectRoot, `.dx/build/public/icons/mark-${assetHash.slice(0, 16)}.svg`, asset);
  write(projectRoot, `.dx/build/public/icons/mark-${assetBlake3Hash}.svg`, asset);
  write(projectRoot, ".dx/build/public/icons/mark-deadbeefdeadbeef.svg", asset);

  const validStyleOutput = summarizeStyleOutput(
    projectRoot,
    [
      {
        path: "styles/app.css",
        output: ".dx/build/styles/app.css",
        hash: sha256(css).slice(0, 16),
        hash_algorithm: "sha256",
        source_map_output: ".dx/build/styles/app.css.map",
        source_map_hash: sha256(sourceMap).slice(0, 16),
        source_map_hash_algorithm: "sha256",
        source_map_linked: true,
        source_map_source_count: 1,
        node_modules_required: false,
        lifecycle_scripts_executed: false,
        source_owned_contract: true,
        external_runtime_required: false,
        external_runtime_executed: false,
      },
    ],
    "styles/app.css",
  );
  const validAssetOutput = summarizeAssetOutput(
    projectRoot,
    [
      {
        path: "public/icons/mark.svg",
        output: `.dx/build/public/icons/mark-${assetHash.slice(0, 16)}.svg`,
        hash: assetHash.slice(0, 16),
        hash_algorithm: "sha256",
        size: Buffer.byteLength(asset),
        node_modules_required: false,
        lifecycle_scripts_executed: false,
        source_owned_contract: true,
        external_runtime_required: false,
        external_runtime_executed: false,
      },
    ],
    "public/icons/mark.svg",
  );

  assert.equal(validStyleOutput.hashMatchesOutput, true);
  assert.equal(validStyleOutput.sourceMapHashMatchesArtifact, true);
  assert.equal(validAssetOutput.hashMatchesOutput, true);

  const validBlake3AssetOutput = summarizeAssetOutput(
    projectRoot,
    [
      {
        path: "public/icons/mark.svg",
        output: `.dx/build/public/icons/mark-${assetBlake3Hash}.svg`,
        hash: assetBlake3Hash,
        hash_algorithm: "blake3-16",
        size: Buffer.byteLength(asset),
        node_modules_required: false,
        lifecycle_scripts_executed: false,
        source_owned_contract: true,
        external_runtime_required: false,
        external_runtime_executed: false,
      },
    ],
    "public/icons/mark.svg",
  );

  assert.equal(validBlake3AssetOutput.hashMatchesOutput, true);

  const unsupportedAlgorithmAssetOutput = summarizeAssetOutput(
    projectRoot,
    [
      {
        path: "public/icons/mark.svg",
        output: `.dx/build/public/icons/mark-${assetHash.slice(0, 16)}.svg`,
        hash: assetHash.slice(0, 16),
        hash_algorithm: "md5",
        size: Buffer.byteLength(asset),
        node_modules_required: false,
        lifecycle_scripts_executed: false,
        source_owned_contract: true,
        external_runtime_required: false,
        external_runtime_executed: false,
      },
    ],
    "public/icons/mark.svg",
  );

  assert.equal(unsupportedAlgorithmAssetOutput.hashMatchesOutput, false);

  const staleStyleOutput = summarizeStyleOutput(
    projectRoot,
    [
      {
        path: "styles/app.css",
        output: ".dx/build/styles/app.css",
        hash: "deadbeefdeadbeef",
        hash_algorithm: "sha256",
        source_map_output: ".dx/build/styles/app.css.map",
        source_map_hash: "feedfacefeedface",
        source_map_hash_algorithm: "sha256",
        source_map_linked: true,
        source_map_source_count: 1,
        node_modules_required: false,
        lifecycle_scripts_executed: false,
        source_owned_contract: true,
        external_runtime_required: false,
        external_runtime_executed: false,
      },
    ],
    "styles/app.css",
  );
  const staleAssetOutput = summarizeAssetOutput(
    projectRoot,
    [
      {
        path: "public/icons/mark.svg",
        output: ".dx/build/public/icons/mark-deadbeefdeadbeef.svg",
        hash: "deadbeefdeadbeef",
        hash_algorithm: "sha256",
        size: Buffer.byteLength(asset),
        node_modules_required: false,
        lifecycle_scripts_executed: false,
        source_owned_contract: true,
        external_runtime_required: false,
        external_runtime_executed: false,
      },
    ],
    "public/icons/mark.svg",
  );

  assert.equal(staleStyleOutput.hashMatchesOutput, false);
  assert.equal(staleStyleOutput.sourceMapHashMatchesArtifact, false);
  assert.equal(staleAssetOutput.hashMatchesOutput, false);

  const proof = summarizeProof({
    binaryRole: "installed-default",
    passed: true,
    build: {
      sourceBuild: {
        manifest: {
          styleOutput: staleStyleOutput,
          publicAssetOutput: staleAssetOutput,
        },
      },
    },
  });
  const outputSummary = summarizeOutputProofSummary(proof);

  assert.equal(proof.productEligible, false);
  assert.deepEqual(
    outputSummary.missingChecks.filter((check) => check.includes("hash-matches-artifact")),
    [
      "style-output-hash-matches-artifact",
      "style-source-map-hash-matches-artifact",
      "public-asset-hash-matches-artifact",
    ],
  );
});

test("installed smoke rejects hashed public assets emitted outside the DX public output tree", () => {
  const projectRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-installed-output-proof-asset-root-"));
  const css = ".hero{display:grid}\n/*# sourceMappingURL=app.css.map */";
  const sourceMap = '{"version":3,"sources":["styles/app.css"]}';
  const asset = "<svg />";
  const assetHash = sha256(asset).slice(0, 16);

  write(projectRoot, ".dx/build/styles/app.css", css);
  write(projectRoot, ".dx/build/styles/app.css.map", sourceMap);
  write(projectRoot, `.next/static/media/mark-${assetHash}.svg`, asset);

  const styleOutput = summarizeStyleOutput(
    projectRoot,
    [
      {
        path: "styles/app.css",
        output: ".dx/build/styles/app.css",
        hash: sha256(css).slice(0, 16),
        hash_algorithm: "sha256",
        source_map_output: ".dx/build/styles/app.css.map",
        source_map_hash: sha256(sourceMap).slice(0, 16),
        source_map_hash_algorithm: "sha256",
        source_map_linked: true,
        source_map_source_count: 1,
        node_modules_required: false,
        lifecycle_scripts_executed: false,
        source_owned_contract: true,
        external_runtime_required: false,
        external_runtime_executed: false,
      },
    ],
    "styles/app.css",
  );
  const assetOutput = summarizeAssetOutput(
    projectRoot,
    [
      {
        path: "public/icons/mark.svg",
        output: `.next/static/media/mark-${assetHash}.svg`,
        hash: assetHash,
        hash_algorithm: "sha256",
        size: Buffer.byteLength(asset),
        node_modules_required: false,
        lifecycle_scripts_executed: false,
        source_owned_contract: true,
        external_runtime_required: false,
        external_runtime_executed: false,
      },
    ],
    "public/icons/mark.svg",
  );

  assert.equal(assetOutput.hashMatchesOutput, true);
  assert.equal(assetOutput.outputFileNameContainsHash, true);
  assert.equal(assetOutput.sizeMatchesOutput, true);

  const proof = summarizeProof({
    binaryRole: "installed-default",
    passed: true,
    build: {
      command: { exitCode: 0 },
      exitCode: 0,
      sourceBuild: {
        manifest: {
          styleOutput,
          publicAssetOutput: assetOutput,
        },
      },
    },
  });
  const outputSummary = summarizeOutputProofSummary(proof);

  assert.equal(proof.productEligible, false);
  assert.equal(proof.cssAssetOutputProof.publicAssetOutput.eligible, false);
  assert.deepEqual(
    outputSummary.publicAssetOutput.missingChecks.filter((check) =>
      check.startsWith("public-asset-output"),
    ),
    [
      "public-asset-output-public-root",
      "public-asset-output-source-derivative",
    ],
  );
});

test("installed smoke reports unsafe CSS and public asset manifest output paths", () => {
  const tempRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-installed-output-proof-unsafe-path-"));
  const projectRoot = path.join(tempRoot, "project");
  const outsideRoot = path.join(tempRoot, "outside");
  const css = ".hero{display:grid}\n/*# sourceMappingURL=app.css.map */";
  const sourceMap = '{"version":3,"sources":["styles/app.css"]}';
  const asset = "<svg />";
  const assetHash = sha256(asset).slice(0, 16);

  write(projectRoot, ".dx/build/styles/app.css", css);
  write(projectRoot, ".dx/build/styles/app.css.map", sourceMap);
  write(outsideRoot, `mark-${assetHash}.svg`, asset);

  const styleOutput = summarizeStyleOutput(
    projectRoot,
    [
      {
        path: "styles/app.css",
        output: "../outside/app.css",
        hash: sha256(css).slice(0, 16),
        hash_algorithm: "sha256",
        source_map_output: "../outside/app.css.map",
        source_map_hash: sha256(sourceMap).slice(0, 16),
        source_map_hash_algorithm: "sha256",
        source_map_linked: true,
        source_map_source_count: 1,
        node_modules_required: false,
        lifecycle_scripts_executed: false,
        source_owned_contract: true,
        external_runtime_required: false,
        external_runtime_executed: false,
      },
    ],
    "styles/app.css",
  );
  const assetOutput = summarizeAssetOutput(
    projectRoot,
    [
      {
        path: "public/icons/mark.svg",
        output: `../outside/mark-${assetHash}.svg`,
        hash: assetHash,
        hash_algorithm: "sha256",
        size: Buffer.byteLength(asset),
        node_modules_required: false,
        lifecycle_scripts_executed: false,
        source_owned_contract: true,
        external_runtime_required: false,
        external_runtime_executed: false,
      },
    ],
    "public/icons/mark.svg",
  );

  assert.equal(styleOutput.outputPathSafe, false);
  assert.equal(styleOutput.sourceMapOutputPathSafe, false);
  assert.equal(assetOutput.outputPathSafe, false);
  assert.equal(assetOutput.present, false);
  assert.equal(assetOutput.hashMatchesOutput, false);

  const proof = summarizeProof({
    binaryRole: "installed-default",
    passed: true,
    build: {
      command: { exitCode: 0 },
      exitCode: 0,
      sourceBuild: {
        manifest: {
          styleOutput,
          publicAssetOutput: assetOutput,
        },
      },
    },
  });
  const outputSummary = summarizeOutputProofSummary(proof);

  assert.equal(proof.productEligible, false);
  assert.ok(outputSummary.styleOutput.missingChecks.includes("style-output-path-safe"));
  assert.ok(outputSummary.styleOutput.missingChecks.includes("style-source-map-path-safe"));
  assert.ok(
    outputSummary.publicAssetOutput.missingChecks.includes("public-asset-output-path-safe"),
  );
});

test("installed smoke rejects stylesheet source maps without artifact source evidence", () => {
  const projectRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-installed-output-proof-empty-map-"));
  write(
    projectRoot,
    ".dx/build/styles/app.css",
    ".hero{display:grid}\n/*# sourceMappingURL=app.css.map */",
  );
  write(projectRoot, ".dx/build/styles/app.css.map", '{"version":3,"sources":[]}');

  const styleOutput = summarizeStyleOutput(
    projectRoot,
    [
      {
        path: "styles/app.css",
        output: ".dx/build/styles/app.css",
        hash: "css123",
        source_map_output: ".dx/build/styles/app.css.map",
        source_map_hash: "map123",
        source_map_linked: true,
        source_map_source_count: 2,
        node_modules_required: false,
        lifecycle_scripts_executed: false,
        source_owned_contract: true,
        external_runtime_required: false,
        external_runtime_executed: false,
      },
    ],
    "styles/app.css",
  );

  assert.equal(styleOutput.sourceMapJsonValid, true);
  assert.equal(styleOutput.sourceMapArtifactSourceCount, 0);
  assert.equal(styleOutput.sourceMapHasSources, false);

  const proof = summarizeProof({
    binaryRole: "installed-default",
    passed: true,
    build: {
      sourceBuild: {
        manifest: {
          styleOutput,
          publicAssetOutput: {
            sourcePath: "public/icons/mark.svg",
            sourceOutputPath: ".dx/build/public/icons/mark.svg",
            present: true,
            path: ".dx/build/public/icons/mark-abc123.svg",
            outputPath: ".dx/build/public/icons/mark-abc123.svg",
            hasHash: true,
            hash: "abc123",
            outputFileNameContainsHash: true,
            declaresNoNodeModules: true,
            lifecycleScriptsExecuted: false,
            sourceOwnedContract: true,
            externalRuntimeRequired: false,
            externalRuntimeExecuted: false,
            sizeMatchesOutput: true,
          },
        },
      },
    },
  });

  assert.equal(proof.productEligible, false);
  assert.equal(proof.cssAssetOutputProof.styleOutput.eligible, false);
  assert.equal(
    proof.cssAssetOutputProof.styleOutput.sourceMapArtifactSourceCount,
    0,
  );
});

test("installed smoke wires CSS and asset proof into source-build failures", () => {
  const sourceBuild = read("tools/build/installed-smoke/source-build.ts");
  const outputProofFailures = read("tools/build/installed-smoke/output-proof-failures.ts");
  const sourceBuildFailures = read("tools/build/installed-smoke/source-build-failures.ts");

  assert.match(sourceBuild, /summarizeStyleOutput/);
  assert.match(sourceBuild, /summarizeAssetOutput/);
  assert.match(sourceBuild, /styleOutput,/);
  assert.match(sourceBuild, /publicAssetOutput,/);
  assert.match(sourceBuild, /publicAssetSourceOutputPath: publicAssetOutput\.sourceOutputPath/);
  assert.match(sourceBuild, /publicAssetHashedOutputPath: publicAssetOutput\.outputPath/);
  assert.match(sourceBuildFailures, /outputProofFailures\(report\)/);
  assert.match(outputProofFailures, /summarizeOutputProofSummary/);
  assert.match(outputProofFailures, /source-build output proof failed: \$\{check\}/);
  assert.match(outputProofFailures, /style-source-map-present/);
  assert.match(outputProofFailures, /style-source-map-artifact-sources/);
  assert.match(outputProofFailures, /style-no-node-modules/);
  assert.match(outputProofFailures, /style-source-owned/);
  assert.match(outputProofFailures, /public-asset-source-public-path/);
  assert.match(outputProofFailures, /public-asset-output-public-root/);
  assert.match(outputProofFailures, /public-asset-output-source-derivative/);
  assert.match(outputProofFailures, /public-asset-hashed-filename/);
  assert.doesNotMatch(outputProofFailures, /source-build stylesheet source map output was not emitted/);
  assert.doesNotMatch(outputProofFailures, /Next\s+DevTools|Turbopack\s+powers/i);
});

test("installed smoke product evidence requires CSS source-map and hashed asset evidence", () => {
  const proof = summarizeProof({
    binaryRole: "installed-default",
    passed: true,
    build: {
      command: { exitCode: 0 },
      exitCode: 0,
      sourceBuild: {
        manifest: {
          styleOutput: {
            sourcePath: "styles/app.css",
            sourceOutputPath: ".dx/build/styles/app.css",
            present: true,
            path: ".dx/build/styles/app.css",
            outputPath: ".dx/build/styles/app.css",
            sourceOutputPathMatchesOutput: true,
            hasHash: true,
            sourceMapPath: ".dx/build/styles/app.css.map",
            sourceMapPresent: true,
            sourceMapLinked: true,
            sourceMapLinkedInCss: true,
            hasSourceMapHash: true,
            sourceMapSourceCount: 2,
            sourceMapJsonValid: true,
            sourceMapArtifactSourceCount: 1,
            sourceMapHasSources: true,
            declaresNoNodeModules: true,
            lifecycleScriptsExecuted: false,
            sourceOwnedContract: true,
            externalRuntimeRequired: false,
            externalRuntimeExecuted: false,
          },
          publicAssetOutput: {
            sourcePath: "public/icons/mark.svg",
            sourceOutputPath: ".dx/build/public/icons/mark.svg",
            present: true,
            path: ".dx/build/public/icons/mark-abc123.svg",
            outputPath: ".dx/build/public/icons/mark-abc123.svg",
            hasHash: true,
            hash: "abc123",
            outputFileNameContainsHash: true,
            declaresNoNodeModules: true,
            lifecycleScriptsExecuted: false,
            sourceOwnedContract: true,
            externalRuntimeRequired: false,
            externalRuntimeExecuted: false,
            sizeMatchesOutput: true,
          },
        },
      },
    },
  });

  assert.equal(proof.productEligible, true);
  assert.equal(proof.cssAssetOutputProof.required, true);
  assert.equal(proof.cssAssetOutputProof.styleOutput.eligible, true);
  assert.equal(proof.cssAssetOutputProof.styleOutput.sourcePath, "styles/app.css");
  assert.equal(proof.cssAssetOutputProof.styleOutput.sourceOutputPath, ".dx/build/styles/app.css");
  assert.equal(proof.cssAssetOutputProof.styleOutput.outputPath, ".dx/build/styles/app.css");
  assert.equal(proof.cssAssetOutputProof.styleOutput.sourceOutputPathMatchesOutput, true);
  assert.equal(proof.cssAssetOutputProof.styleOutput.sourceMapSourceCount, 2);
  assert.equal(proof.cssAssetOutputProof.styleOutput.declaresNoNodeModules, true);
  assert.equal(proof.cssAssetOutputProof.styleOutput.lifecycleScriptsExecuted, false);
  assert.equal(proof.cssAssetOutputProof.styleOutput.sourceOwnedContract, true);
  assert.equal(proof.cssAssetOutputProof.styleOutput.externalRuntimeRequired, false);
  assert.equal(proof.cssAssetOutputProof.styleOutput.externalRuntimeExecuted, false);
  assert.equal(proof.cssAssetOutputProof.publicAssetOutput.eligible, true);
  assert.equal(proof.cssAssetOutputProof.publicAssetOutput.sourcePath, "public/icons/mark.svg");
  assert.equal(proof.cssAssetOutputProof.publicAssetOutput.sourceOutputPath, ".dx/build/public/icons/mark.svg");
  assert.equal(proof.cssAssetOutputProof.publicAssetOutput.sourcePathIsPublicAsset, true);
  assert.equal(proof.cssAssetOutputProof.publicAssetOutput.outputPathIsPublicAsset, true);
  assert.equal(
    proof.cssAssetOutputProof.publicAssetOutput.outputPathMatchesSourceOwnedAssetPath,
    true,
  );
  assert.equal(
    proof.cssAssetOutputProof.publicAssetOutput.outputPath,
    ".dx/build/public/icons/mark-abc123.svg",
  );

  const missingStyleSourceMapProof = summarizeProof({
    binaryRole: "installed-default",
    passed: true,
    build: {
      sourceBuild: {
        manifest: {
          styleOutput: {
            sourcePath: "styles/app.css",
            sourceOutputPath: ".dx/build/styles/app.css",
            present: true,
            path: ".dx/build/styles/app.css",
            outputPath: ".dx/build/styles/app.css",
            sourceOutputPathMatchesOutput: true,
            hasHash: true,
            sourceMapPath: ".dx/build/styles/app.css.map",
            sourceMapPresent: false,
            sourceMapLinked: true,
            sourceMapLinkedInCss: true,
            hasSourceMapHash: true,
            sourceMapSourceCount: 2,
            sourceMapJsonValid: true,
            sourceMapArtifactSourceCount: 1,
            sourceMapHasSources: true,
            declaresNoNodeModules: true,
            lifecycleScriptsExecuted: false,
            sourceOwnedContract: true,
            externalRuntimeRequired: false,
            externalRuntimeExecuted: false,
          },
          publicAssetOutput: {
            present: true,
            path: ".dx/build/public/icons/mark-abc123.svg",
            hasHash: true,
            outputFileNameContainsHash: true,
            declaresNoNodeModules: true,
            lifecycleScriptsExecuted: false,
            sourceOwnedContract: true,
            externalRuntimeRequired: false,
            externalRuntimeExecuted: false,
            sizeMatchesOutput: true,
          },
        },
      },
    },
  });

  assert.equal(missingStyleSourceMapProof.productEligible, false);
  assert.equal(missingStyleSourceMapProof.cssAssetOutputProof.styleOutput.eligible, false);
});

test("installed smoke product evidence requires public asset source path evidence", () => {
  const proof = summarizeProof({
    binaryRole: "installed-default",
    passed: true,
    build: {
      sourceBuild: {
        manifest: {
          styleOutput: {
            sourcePath: "styles/app.css",
            sourceOutputPath: ".dx/build/styles/app.css",
            present: true,
            path: ".dx/build/styles/app.css",
            outputPath: ".dx/build/styles/app.css",
            sourceOutputPathMatchesOutput: true,
            hasHash: true,
            sourceMapPath: ".dx/build/styles/app.css.map",
            sourceMapPresent: true,
            sourceMapLinked: true,
            sourceMapLinkedInCss: true,
            hasSourceMapHash: true,
            sourceMapSourceCount: 1,
            sourceMapJsonValid: true,
            sourceMapArtifactSourceCount: 1,
            sourceMapHasSources: true,
            declaresNoNodeModules: true,
            lifecycleScriptsExecuted: false,
            sourceOwnedContract: true,
            externalRuntimeRequired: false,
            externalRuntimeExecuted: false,
          },
          publicAssetOutput: {
            present: true,
            path: ".dx/build/public/icons/mark-abc123.svg",
            outputPath: ".dx/build/public/icons/mark-abc123.svg",
            hasHash: true,
            outputFileNameContainsHash: true,
            declaresNoNodeModules: true,
            lifecycleScriptsExecuted: false,
            sourceOwnedContract: true,
            externalRuntimeRequired: false,
            externalRuntimeExecuted: false,
            sizeMatchesOutput: true,
          },
        },
      },
    },
  });
  const outputSummary = summarizeOutputProofSummary(proof);

  assert.equal(proof.productEligible, false);
  assert.equal(proof.cssAssetOutputProof.publicAssetOutput.eligible, false);
  assert.deepEqual(
    outputSummary.publicAssetOutput.missingChecks.filter((check) => check.startsWith("public-asset-source")),
    [
      "public-asset-source-path",
      "public-asset-source-output-path",
      "public-asset-source-public-path",
    ],
  );
});

function write(root, relativePath, content) {
  const target = path.join(root, relativePath);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, content);
}

function sha256(content) {
  return crypto.createHash("sha256").update(content).digest("hex");
}
