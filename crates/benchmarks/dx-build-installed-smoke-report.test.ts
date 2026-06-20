import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const { createReport } = require("../tools/build/installed-smoke/report.ts");

test("installed smoke probes canonical www build help command", () => {
  const cliSource = fs.readFileSync(
    path.join(__dirname, "..", "tools", "build", "installed-smoke", "cli.ts"),
    "utf8",
  );

  assert.match(
    cliSource,
    /\["www", "build", "--help"\]/,
    "installed smoke should probe the canonical dx www build help command",
  );
  assert.doesNotMatch(
    cliSource,
    /\["build", "--help"\]/,
    "installed smoke should not probe the legacy top-level dx build help command for native Android support",
  );
});

test("installed smoke classifies disk-full build failures before derived artifact failures", () => {
  const report = createReport(createReportInput({
    build: {
      status: 1,
      stdout: "",
      stderr:
        'IoError { message: "There is not enough space on the disk. (os error 112)", path: Some(".dx/build/app/index.html") }',
      command: "G:\\Dx\\bin\\dx-www.exe",
      args: ["build"],
      dxArgs: ["build"],
      cwd: "G:\\Temp\\fixture",
    },
  }));

  assert.equal(report.environmentBlocker.kind, "disk-space");
  assert.equal(report.environmentBlocker.command, "dx build");
  assert.match(report.environmentBlocker.evidence, /os error 112/);
  assert.ok(report.failures.includes("build environment has insufficient disk space"));
  assert.equal(
    report.failures.includes("dx build did not write .dx/build/.dx/build-cache/manifest.json"),
    false,
    "disk-full reports should not bury the root cause under derived artifact failures",
  );
});

test("installed smoke fails stale installed help that hides Android target support", () => {
  const report = createReport(createReportInput({
    help: {
      status: 0,
      stdout: "",
      stderr:
        "dx www build: Run the DX source-owned build engine\nUses the source-owned build engine and does not install node_modules.",
      command: "G:\\Dx\\bin\\dx-www.exe",
      args: ["www", "build", "--help"],
      dxArgs: ["www", "build", "--help"],
      cwd: "G:\\Temp\\empty",
    },
  }));

  assert.deepEqual(report.help.command.args, ["www", "build", "--help"]);
  assert.equal(report.help.sourceOwnedContractVisible, true);
  assert.equal(report.help.androidTargetVisible, false);
  assert.ok(report.failures.includes("dx www build --help did not describe the Android build target"));
  assert.equal(report.passed, false);
});

test("installed smoke report exposes compact CSS and asset output proof summary", () => {
  const projectRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-installed-report-output-proof-"));
  write(projectRoot, ".dx/build/styles/app.css", ".hero{display:grid}\n/*# sourceMappingURL=app.css.map */");
  write(projectRoot, ".dx/build/styles/app.css.map", '{"version":3,"sources":["styles/app.css"]}');
  write(projectRoot, ".dx/build/public/icons/mark-abc123.svg", "<svg />");

  const report = createReport(createReportInput({
    projectRoot,
    sourceBuildManifest: {
      ok: true,
      value: sourceBuildManifest(),
    },
  }));

  assert.equal(report.outputProofSummary.required, true);
  assert.equal(report.outputProofSummary.eligible, true);
  assert.equal(report.outputProofSummary.styleOutput.eligible, true);
  assert.equal(report.outputProofSummary.styleOutput.sourcePath, "styles/app.css");
  assert.equal(report.outputProofSummary.styleOutput.sourceOutputPath, ".dx/build/styles/app.css");
  assert.equal(report.outputProofSummary.styleOutput.outputPath, ".dx/build/styles/app.css");
  assert.equal(report.outputProofSummary.styleOutput.sourceOutputPathMatchesOutput, true);
  assert.equal(report.outputProofSummary.styleOutput.path, ".dx/build/styles/app.css");
  assert.equal(report.outputProofSummary.styleOutput.sourceMapPath, ".dx/build/styles/app.css.map");
  assert.equal(report.outputProofSummary.styleOutput.declaresNoNodeModules, true);
  assert.equal(report.outputProofSummary.styleOutput.lifecycleScriptsExecuted, false);
  assert.equal(report.outputProofSummary.styleOutput.sourceOwnedContract, true);
  assert.equal(report.outputProofSummary.styleOutput.externalRuntimeRequired, false);
  assert.equal(report.outputProofSummary.styleOutput.externalRuntimeExecuted, false);
  assert.deepEqual(report.outputProofSummary.styleOutput.missingChecks, []);
  assert.equal(report.outputProofSummary.publicAssetOutput.eligible, true);
  assert.equal(report.outputProofSummary.publicAssetOutput.sourcePath, "public/icons/mark.svg");
  assert.equal(
    report.outputProofSummary.publicAssetOutput.sourceOutputPath,
    ".dx/build/public/icons/mark.svg",
  );
  assert.equal(report.outputProofSummary.publicAssetOutput.sourcePathIsPublicAsset, true);
  assert.equal(report.outputProofSummary.publicAssetOutput.outputPathIsPublicAsset, true);
  assert.equal(
    report.outputProofSummary.publicAssetOutput.outputPathMatchesSourceOwnedAssetPath,
    true,
  );
  assert.equal(report.outputProofSummary.publicAssetOutput.path, ".dx/build/public/icons/mark-abc123.svg");
  assert.deepEqual(report.outputProofSummary.publicAssetOutput.missingChecks, []);
  assert.deepEqual(report.outputProofSummary.missingChecks, []);

  const missingMapRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-installed-report-output-proof-missing-"));
  write(missingMapRoot, ".dx/build/styles/app.css", ".hero{display:grid}\n/*# sourceMappingURL=app.css.map */");
  write(missingMapRoot, ".dx/build/public/icons/mark-abc123.svg", "<svg />");

  const missingMapReport = createReport(createReportInput({
    projectRoot: missingMapRoot,
    sourceBuildManifest: {
      ok: true,
      value: sourceBuildManifest(),
    },
  }));

  assert.equal(missingMapReport.outputProofSummary.eligible, false);
  assert.deepEqual(missingMapReport.outputProofSummary.styleOutput.missingChecks, [
    "style-source-map-present",
    "style-source-map-json-valid",
    "style-source-map-artifact-sources",
    "style-source-map-source-path",
  ]);
  assert.ok(missingMapReport.outputProofSummary.missingChecks.includes("style-source-map-present"));
  assert.ok(missingMapReport.outputProofSummary.missingChecks.includes("style-source-map-json-valid"));
  assert.ok(missingMapReport.outputProofSummary.missingChecks.includes("style-source-map-artifact-sources"));
  assert.ok(missingMapReport.outputProofSummary.missingChecks.includes("style-source-map-source-path"));
  assert.ok(
    missingMapReport.failures.includes(
      "source-build output proof failed: style-source-map-present (stylesheet source map was not emitted)",
    ),
  );
  assert.ok(
    missingMapReport.failures.includes(
      "source-build output proof failed: style-source-map-json-valid (stylesheet source map is not valid JSON)",
    ),
  );
  assert.ok(
    missingMapReport.failures.includes(
      "source-build output proof failed: style-source-map-artifact-sources (stylesheet source map artifact is missing source entries)",
    ),
  );
  assert.ok(
    missingMapReport.failures.includes(
      "source-build output proof failed: style-source-map-source-path (stylesheet source map does not reference its manifest source path)",
    ),
  );
  assert.equal(
    missingMapReport.failures.includes("source-build stylesheet source map output was not emitted"),
    false,
    "CSS/asset failures should be reported from outputProofSummary.missingChecks, not a duplicate ad hoc list",
  );
});

test("installed smoke keeps missing public assets distinct from size mismatches", () => {
  const missingAssetRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-installed-report-missing-asset-"));
  writeValidStyleOutput(missingAssetRoot);

  const missingAssetReport = createReport(createReportInput({
    projectRoot: missingAssetRoot,
    sourceBuildManifest: {
      ok: true,
      value: sourceBuildManifest(),
    },
  }));

  assert.deepEqual(missingAssetReport.outputProofSummary.publicAssetOutput.missingChecks, [
    "public-asset-present",
  ]);
  assert.equal(missingAssetReport.outputProofSummary.publicAssetOutput.sizeMatchesOutput, null);
  assert.ok(
    missingAssetReport.failures.includes(
      "source-build output proof failed: public-asset-present (manifest-declared public asset was not emitted)",
    ),
  );
  assert.equal(
    missingAssetReport.failures.some((failure) => failure.includes("public-asset-size-match")),
    false,
    "missing public asset output should not also report an unmeasurable size mismatch",
  );

  const wrongSizeRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-installed-report-asset-size-"));
  writeValidStyleOutput(wrongSizeRoot);
  write(wrongSizeRoot, ".dx/build/public/icons/mark-abc123.svg", "<svg>larger</svg>");

  const wrongSizeReport = createReport(createReportInput({
    projectRoot: wrongSizeRoot,
    sourceBuildManifest: {
      ok: true,
      value: sourceBuildManifest(),
    },
  }));

  assert.deepEqual(wrongSizeReport.outputProofSummary.publicAssetOutput.missingChecks, [
    "public-asset-size-match",
  ]);
  assert.equal(wrongSizeReport.outputProofSummary.publicAssetOutput.sizeMatchesOutput, false);
  assert.ok(
    wrongSizeReport.failures.includes(
      "source-build output proof failed: public-asset-size-match (public asset output size does not match the manifest)",
    ),
  );
});

function createReportInput(overrides = {}) {
  const projectRoot = overrides.projectRoot || "G:\\Temp\\fixture";
  const buildDir = path.join(projectRoot, ".dx", "build");
  const receiptsDir = path.join(projectRoot, ".dx", "receipts");
  const missingJson = { ok: false, value: null };

  return {
    binary: "G:\\Dx\\bin\\dx-www.exe",
    binaryProvenance: {
      defaultBinary: "G:\\Dx\\bin\\dx-www.exe",
      override: false,
      role: "installed-default",
    },
    binaryIdentity: {
      path: "G:\\Dx\\bin\\dx-www.exe",
      present: true,
      kind: "file",
      byteLength: 123456,
      sha256: "abc123",
    },
    runner: null,
    projectRoot,
    receipt: path.join(receiptsDir, "build", "installed-binary-smoke-latest.json"),
    requireProduct: true,
    help: {
      status: 0,
      stdout: "",
      stderr:
        "dx www build: Run the DX source-owned build engine\nUSAGE:\n    dx www build --target android\nOPTIONS:\n    --target <target>\nUses the source-owned build engine and does not install node_modules.",
      command: "G:\\Dx\\bin\\dx-www.exe",
      args: ["www", "build", "--help"],
      dxArgs: ["www", "build", "--help"],
      cwd: "G:\\Temp\\empty",
    },
    helpReadOnly: true,
    build: {
      status: 0,
      stdout: "",
      stderr: "",
      command: "G:\\Dx\\bin\\dx-www.exe",
      args: ["build"],
      dxArgs: ["build"],
      cwd: projectRoot,
    },
    nodeModulesCreated: false,
    manifest: missingJson,
    appExecution: missingJson,
    serverData: missingJson,
    serverContracts: missingJson,
    deployAdapter: missingJson,
    sourceBuildManifest: missingJson,
    sourceBuildReceipt: missingJson,
    routeHandlerReceipts: missingJson,
    canonicalReceipt: missingJson,
    graphReceipt: missingJson,
    graphConsumerSnapshot: missingJson,
    nextFamiliarCompatibilityEvidence: missingJson,
    zedHandoff: missingJson,
    readiness: missingJson,
    manifestPath: path.join(buildDir, ".dx/build-cache/manifest.json"),
    sourceBuildManifestPath: path.join(buildDir, "source-build-manifest.json"),
    sourceBuildReceiptPath: path.join(buildDir, ".dx/build-cache/source-build-receipt.json"),
    routeHandlerReceiptsPath: path.join(buildDir, ".dx/build-cache/route-handler-receipts.json"),
    canonicalReceiptPath: path.join(receiptsDir, "build", "latest.json"),
    graphReceiptPath: path.join(receiptsDir, "graph", "latest.json"),
    graphConsumerSnapshotPath: path.join(receiptsDir, "graph", "consumer-snapshot.json"),
    nextFamiliarCompatibilityEvidencePath: path.join(
      buildDir,
      "next-familiar-compatibility-evidence.json",
    ),
    nextRuntimeParityEvidencePath: path.join(buildDir, "next-runtime-parity-evidence.json"),
    appHtmlPath: path.join(buildDir, "app", "index.html"),
    appPacketPath: path.join(buildDir, "app", "index.dxpk"),
    appPageGraphPath: path.join(buildDir, "app", "page-graph.json"),
    appExecutionPath: path.join(buildDir, "app", "app-router-execution.json"),
    serverDataPath: path.join(buildDir, "app", "server-data.json"),
    serverContractsPath: path.join(buildDir, "server-contracts.json"),
    deployAdapterPath: path.join(buildDir, ".dx/build-cache/deploy-adapter.json"),
    zedHandoffPath: path.join(receiptsDir, "build", "zed-handoff.json"),
    readinessPath: path.join(receiptsDir, "build", "readiness.json"),
    ...overrides,
  };
}

function sourceBuildManifest() {
  return {
    schema: "dx.www.sourceBuildManifest",
    styles: [
      {
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
      },
    ],
    assets: [
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
    node_modules_required: false,
  };
}

function writeValidStyleOutput(root) {
  write(root, ".dx/build/styles/app.css", ".hero{display:grid}\n/*# sourceMappingURL=app.css.map */");
  write(root, ".dx/build/styles/app.css.map", '{"version":3,"sources":["styles/app.css"]}');
}

function write(root, relativePath, content) {
  const target = path.join(root, relativePath);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, content);
}
