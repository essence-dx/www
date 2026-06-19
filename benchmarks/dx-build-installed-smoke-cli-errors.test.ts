import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const { spawnSync } = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.join(__dirname, "..");
const smokePath = path.join(repoRoot, "tools", "build", "dx-build-installed-smoke.ts");

test("installed smoke CLI reports argument errors without a Node stack trace", () => {
  const result = spawnSync(
    process.execPath,
    [smokePath, "--definitely-not-a-real-flag"],
    {
      cwd: repoRoot,
      encoding: "utf8",
    },
  );

  assert.equal(result.status, 2, result.stdout + result.stderr);
  assert.equal(result.stdout, "");
  assert.match(result.stderr, /dx build installed smoke: Unknown option: --definitely-not-a-real-flag/);
  assert.match(result.stderr, /Usage: node tools\/build\/dx-build-installed-smoke\.ts/);
  assert.doesNotMatch(result.stderr, /at parseArgs/);
  assert.doesNotMatch(result.stderr, /Node\.js v/);
});

test("installed smoke CLI receipt keeps compact CSS and asset output proof", () => {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), "dx-installed-cli-output-proof-"));
  const projectRoot = path.join(root, "project");
  const receiptPath = path.join(root, "installed-binary-smoke.json");
  const fakeDx = path.join(root, "fake-dx.ts");
  fs.mkdirSync(projectRoot, { recursive: true });
  fs.writeFileSync(fakeDx, fakeDxBuildScript());

  const result = spawnSync(
    process.execPath,
    [
      smokePath,
      "--binary",
      fakeDx,
      "--runner",
      process.execPath,
      "--project",
      projectRoot,
      "--receipt",
      receiptPath,
      "--json",
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
    },
  );

  assert.equal(result.status, 1, result.stdout + result.stderr);
  assert.equal(result.stderr, "");
  assert.equal(fs.existsSync(receiptPath), true);

  const stdoutReport = JSON.parse(result.stdout);
  const receipt = JSON.parse(fs.readFileSync(receiptPath, "utf8"));
  assert.deepEqual(receipt.outputProofSummary, stdoutReport.outputProofSummary);
  assert.equal(receipt.outputProofSummary.required, true);
  assert.equal(receipt.outputProofSummary.eligible, true);
  assert.equal(receipt.outputProofSummary.styleOutput.sourcePath, "styles/app.css");
  assert.equal(receipt.outputProofSummary.styleOutput.sourceOutputPath, ".dx/build/styles/app.css");
  assert.equal(receipt.outputProofSummary.styleOutput.outputPath, ".dx/build/styles/app.css");
  assert.equal(receipt.outputProofSummary.styleOutput.sourceOutputPathMatchesOutput, true);
  assert.equal(receipt.outputProofSummary.styleOutput.path, ".dx/build/styles/app.css");
  assert.equal(receipt.outputProofSummary.styleOutput.present, true);
  assert.equal(receipt.outputProofSummary.styleOutput.hasHash, true);
  assert.equal(receipt.outputProofSummary.styleOutput.hash, "css123");
  assert.equal(receipt.outputProofSummary.styleOutput.sourceMapPath, ".dx/build/styles/app.css.map");
  assert.equal(receipt.outputProofSummary.styleOutput.sourceMapPresent, true);
  assert.equal(receipt.outputProofSummary.styleOutput.sourceMapLinked, true);
  assert.equal(receipt.outputProofSummary.styleOutput.sourceMapLinkedInCss, true);
  assert.equal(receipt.outputProofSummary.styleOutput.hasSourceMapHash, true);
  assert.equal(receipt.outputProofSummary.styleOutput.sourceMapHash, "map123");
  assert.equal(receipt.outputProofSummary.styleOutput.sourceMapJsonValid, true);
  assert.equal(receipt.outputProofSummary.styleOutput.sourceMapArtifactSourceCount, 1);
  assert.equal(receipt.outputProofSummary.styleOutput.declaresNoNodeModules, true);
  assert.equal(receipt.outputProofSummary.styleOutput.lifecycleScriptsExecuted, false);
  assert.equal(receipt.outputProofSummary.styleOutput.sourceOwnedContract, true);
  assert.equal(receipt.outputProofSummary.styleOutput.externalRuntimeRequired, false);
  assert.equal(receipt.outputProofSummary.styleOutput.externalRuntimeExecuted, false);
  assert.deepEqual(receipt.outputProofSummary.styleOutput.missingChecks, []);
  assert.equal(receipt.outputProofSummary.publicAssetOutput.sourcePath, "public/icons/mark.svg");
  assert.equal(
    receipt.outputProofSummary.publicAssetOutput.sourceOutputPath,
    ".dx/build/public/icons/mark.svg",
  );
  assert.equal(receipt.outputProofSummary.publicAssetOutput.path, ".dx/build/public/icons/mark-abc123.svg");
  assert.equal(receipt.outputProofSummary.publicAssetOutput.present, true);
  assert.equal(receipt.outputProofSummary.publicAssetOutput.hasHash, true);
  assert.equal(receipt.outputProofSummary.publicAssetOutput.hash, "abc123");
  assert.equal(receipt.outputProofSummary.publicAssetOutput.outputFileNameContainsHash, true);
  assert.equal(receipt.outputProofSummary.publicAssetOutput.sourcePathIsPublicAsset, true);
  assert.equal(receipt.outputProofSummary.publicAssetOutput.outputPathIsPublicAsset, true);
  assert.equal(
    receipt.outputProofSummary.publicAssetOutput.outputPathMatchesSourceOwnedAssetPath,
    true,
  );
  assert.equal(receipt.outputProofSummary.publicAssetOutput.declaresNoNodeModules, true);
  assert.equal(receipt.outputProofSummary.publicAssetOutput.lifecycleScriptsExecuted, false);
  assert.equal(receipt.outputProofSummary.publicAssetOutput.sourceOwnedContract, true);
  assert.equal(receipt.outputProofSummary.publicAssetOutput.externalRuntimeRequired, false);
  assert.equal(receipt.outputProofSummary.publicAssetOutput.externalRuntimeExecuted, false);
  assert.equal(receipt.outputProofSummary.publicAssetOutput.sizeMatchesOutput, true);
  assert.equal(receipt.outputProofSummary.publicAssetOutput.outputByteLength, 7);
  assert.deepEqual(receipt.outputProofSummary.publicAssetOutput.missingChecks, []);
});

function fakeDxBuildScript() {
  return String.raw`
const fs = require("node:fs");
const path = require("node:path");

if (process.argv[2] === "build" && process.argv[3] === "--help") {
  process.stdout.write("dx build: source-owned build engine\nThis source-owned build engine does not install node_modules.\n");
  process.exit(0);
}

if (process.argv[2] !== "build") {
  process.stderr.write("unsupported fake dx command\n");
  process.exit(2);
}

const root = process.cwd();
write(".dx/build/styles/app.css", ".hero{display:grid}\n/*# sourceMappingURL=app.css.map */");
write(".dx/build/styles/app.css.map", JSON.stringify({
  version: 3,
  sources: ["styles/app.css"],
}));
write(".dx/build/public/icons/mark-abc123.svg", "<svg />");
write(".dx/build/source-build-manifest.json", JSON.stringify({
  schema: "dx.www.sourceBuildManifest",
  styles: [{
    path: "styles/app.css",
    output: ".dx/build/styles/app.css",
    hash: "css123",
    source_map_output: ".dx/build/styles/app.css.map",
    source_map_hash: "map123",
    source_map_linked: true,
    source_map_source_count: 1,
    node_modules_required: false,
    lifecycle_scripts_executed: false,
    source_owned_contract: true,
    external_runtime_required: false,
    external_runtime_executed: false,
  }],
  assets: [{
    path: "public/icons/mark.svg",
    output: ".dx/build/public/icons/mark-abc123.svg",
    hash: "abc123",
    size: 7,
    node_modules_required: false,
    lifecycle_scripts_executed: false,
    source_owned_contract: true,
    external_runtime_required: false,
    external_runtime_executed: false,
  }],
  node_modules_required: false,
}));

function write(relativePath, content) {
  const target = path.join(root, relativePath);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, content);
}
`;
}
