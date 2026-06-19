const assert = require("node:assert/strict");
const crypto = require("node:crypto");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const { blake3Hex } = require("../tools/build/installed-smoke/blake3-16.ts");
const {
  summarizeAssetOutput,
  summarizeStyleOutput,
} = require("../tools/build/installed-smoke/manifest-output.ts");
const { outputProofFailures } = require("../tools/build/installed-smoke/output-proof-failures.ts");
const {
  summarizeOutputProofSummary,
  summarizeProof,
} = require("../tools/build/installed-smoke/proof.ts");

test("installed smoke rejects CSS source maps that omit the manifest source path", () => {
  const { outputSummary, proof, styleOutput } = buildStyleOutputProof({
    sourceMap: {
      version: 3,
      sources: ["styles/old.css"],
    },
  });

  assert.equal(styleOutput.sourceMapJsonValid, true);
  assert.equal(styleOutput.sourceMapArtifactSourceCount, 1);
  assert.equal(styleOutput.sourceMapHasSources, true);
  assert.deepEqual(styleOutput.sourceMapSources, ["styles/old.css"]);
  assert.equal(styleOutput.sourceMapIncludesSourcePath, false);

  assert.equal(proof.productEligible, false);
  assert.equal(proof.cssAssetOutputProof.styleOutput.eligible, false);
  assert.ok(outputSummary.styleOutput.missingChecks.includes("style-source-map-source-path"));
  assert.ok(
    outputProofFailures({ outputProofSummary: outputSummary }).includes(
      "source-build output proof failed: style-source-map-source-path (stylesheet source map does not reference its manifest source path)",
    ),
  );
});

test("installed smoke accepts CSS source maps that compose sourceRoot with source entries", () => {
  const { outputSummary, proof, styleOutput } = buildStyleOutputProof({
    sourceMap: {
      version: 3,
      sourceRoot: "styles",
      sources: ["app.css"],
    },
  });

  assert.equal(styleOutput.sourceMapJsonValid, true);
  assert.equal(styleOutput.sourceMapArtifactSourceCount, 1);
  assert.equal(styleOutput.sourceMapHasSources, true);
  assert.deepEqual(styleOutput.sourceMapSources, ["styles/app.css"]);
  assert.equal(styleOutput.sourceMapIncludesSourcePath, true);
  assert.equal(proof.cssAssetOutputProof.styleOutput.eligible, true);
  assert.equal(proof.productEligible, true);
  assert.equal(outputSummary.styleOutput.eligible, true);
  assert.equal(outputSummary.styleOutput.missingChecks.length, 0);
});

test("installed smoke rejects CSS source maps that escape the source-owned boundary", () => {
  const { outputSummary, proof, styleOutput } = buildStyleOutputProof({
    sourceMap: {
      version: 3,
      sourceRoot: "..",
      sources: ["styles/app.css"],
    },
  });

  assert.equal(styleOutput.sourceMapJsonValid, true);
  assert.deepEqual(styleOutput.sourceMapSources, ["../styles/app.css"]);
  assert.equal(styleOutput.sourceMapIncludesSourcePath, true);
  assert.equal(styleOutput.sourceMapHasUnsafeSources, true);
  assert.equal(styleOutput.sourceMapUnsafeSourceCount, 1);
  assert.deepEqual(styleOutput.sourceMapUnsafeSources, ["../styles/app.css"]);

  assert.equal(proof.cssAssetOutputProof.styleOutput.eligible, false);
  assert.equal(proof.productEligible, false);
  assert.ok(outputSummary.styleOutput.missingChecks.includes("style-source-map-safe-sources"));
  assert.ok(
    outputProofFailures({ outputProofSummary: outputSummary }).includes(
      "source-build output proof failed: style-source-map-safe-sources (stylesheet source map references unsafe source paths)",
    ),
  );
});

test("installed smoke verifies source-owned BLAKE3 hashes when manifest labels are absent", () => {
  const { proof, publicAssetOutput, styleOutput } = buildStyleOutputProof({
    declareHashAlgorithm: false,
    hashAlgorithm: "blake3-16",
    hashFunction: blake3Prefix,
    sourceMap: {
      version: 3,
      sources: ["styles/app.css"],
    },
  });

  assert.equal(styleOutput.hashAlgorithm, "blake3-16");
  assert.equal(styleOutput.hashAlgorithmInferred, true);
  assert.equal(styleOutput.hashMatchesOutput, true);
  assert.equal(styleOutput.sourceMapHashAlgorithm, "blake3-16");
  assert.equal(styleOutput.sourceMapHashAlgorithmInferred, true);
  assert.equal(styleOutput.sourceMapHashMatchesArtifact, true);
  assert.equal(publicAssetOutput.hashAlgorithm, "blake3-16");
  assert.equal(publicAssetOutput.hashAlgorithmInferred, true);
  assert.equal(publicAssetOutput.hashMatchesOutput, true);
  assert.equal(proof.productEligible, true);
});

function buildStyleOutputProof({
  declareHashAlgorithm = true,
  hashAlgorithm = "sha256",
  hashFunction = (content) => sha256(content).slice(0, 16),
  sourceMap,
}) {
  const projectRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-installed-map-provenance-"));
  const css = ".hero{display:grid}\n/*# sourceMappingURL=app.css.map */";
  const sourceMapContent = JSON.stringify(sourceMap);
  const asset = "<svg />";
  const assetHash = hashFunction(asset);
  const cssHash = hashFunction(css);
  const sourceMapHash = hashFunction(sourceMapContent);
  const hashAlgorithmFields = declareHashAlgorithm
    ? {
        hash_algorithm: hashAlgorithm,
        source_map_hash_algorithm: hashAlgorithm,
      }
    : {};
  const assetHashAlgorithmFields = declareHashAlgorithm
    ? {
        hash_algorithm: hashAlgorithm,
      }
    : {};

  write(projectRoot, ".dx/build/styles/app.css", css);
  write(projectRoot, ".dx/build/styles/app.css.map", sourceMapContent);
  write(projectRoot, `.dx/build/public/icons/mark-${assetHash}.svg`, asset);

  const styleOutput = summarizeStyleOutput(
    projectRoot,
    [
      {
        path: "styles/app.css",
        output: ".dx/build/styles/app.css",
        hash: cssHash,
        source_map_output: ".dx/build/styles/app.css.map",
        source_map_hash: sourceMapHash,
        source_map_linked: true,
        source_map_source_count: 1,
        node_modules_required: false,
        lifecycle_scripts_executed: false,
        source_owned_contract: true,
        external_runtime_required: false,
        external_runtime_executed: false,
        ...hashAlgorithmFields,
      },
    ],
    "styles/app.css",
  );
  const publicAssetOutput = summarizeAssetOutput(
    projectRoot,
    [
      {
        path: "public/icons/mark.svg",
        output: `.dx/build/public/icons/mark-${assetHash}.svg`,
        hash: assetHash,
        size: Buffer.byteLength(asset),
        node_modules_required: false,
        lifecycle_scripts_executed: false,
        source_owned_contract: true,
        external_runtime_required: false,
        external_runtime_executed: false,
        ...assetHashAlgorithmFields,
      },
    ],
    "public/icons/mark.svg",
  );

  const proof = summarizeProof({
    binaryRole: "installed-default",
    passed: true,
    build: {
      command: { exitCode: 0 },
      exitCode: 0,
      sourceBuild: {
        manifest: {
          styleOutput,
          publicAssetOutput,
        },
      },
    },
  });

  return {
    outputSummary: summarizeOutputProofSummary(proof),
    proof,
    publicAssetOutput,
    styleOutput,
  };
}

function write(root, relativePath, content) {
  const target = path.join(root, relativePath);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, content);
}

function sha256(content) {
  return crypto.createHash("sha256").update(content).digest("hex");
}

function blake3Prefix(content) {
  return blake3Hex(Buffer.from(content)).slice(0, 16);
}
