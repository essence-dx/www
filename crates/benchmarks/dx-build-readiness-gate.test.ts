const assert = require("node:assert/strict");
const { spawnSync } = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");
const { createReport } = require("../tools/build/readiness-gate/report.ts");
const { runProofSteps } = require("../tools/build/readiness-gate/proof-bundle.ts");

const repoRoot = path.join(__dirname, "..");
const gatePath = path.join(repoRoot, "tools", "build", "dx-build-readiness-gate.ts");
const installedSmokeSourceFiles = [
  "tools/build/dx-build-installed-smoke.ts",
  "tools/build/installed-smoke/args.ts",
  "tools/build/installed-smoke/binary-provenance.ts",
  "tools/build/installed-smoke/build-receipt-failures.ts",
  "tools/build/installed-smoke/cli.ts",
  "tools/build/installed-smoke/constants.ts",
  "tools/build/installed-smoke/fixture.ts",
  "tools/build/installed-smoke/fixture-paths.ts",
  "tools/build/installed-smoke/io.ts",
  "tools/build/installed-smoke/manifest-asset-output.ts",
  "tools/build/installed-smoke/manifest-output.ts",
  "tools/build/installed-smoke/manifest-output-paths.ts",
  "tools/build/installed-smoke/manifest-output-source-map.ts",
  "tools/build/installed-smoke/manifest-style-output.ts",
  "tools/build/installed-smoke/proof.ts",
  "tools/build/installed-smoke/report.ts",
  "tools/build/installed-smoke/route-handler-graph.ts",
  "tools/build/installed-smoke/route-handler-manifest.ts",
  "tools/build/installed-smoke/route-handler-receipt-summary.ts",
  "tools/build/installed-smoke/route-handler-receipts.ts",
  "tools/build/installed-smoke/route-handler-requirements.ts",
  "tools/build/installed-smoke/route-output.ts",
  "tools/build/installed-smoke/runner.ts",
  "tools/build/installed-smoke/server-artifacts.ts",
  "tools/build/installed-smoke/source-build.ts",
  "tools/build/installed-smoke/source-build-failures.ts",
  "tools/build/installed-smoke/source-freshness.ts",
];

test("release readiness writer consumes workspace build receipts and hub installed smoke receipts", () => {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), "dx-build-readiness-gate-"));
  writeReceipt(root, "www/.dx/receipts/build/readiness.json", {
    schema: "dx.build.readiness",
    source_ready: true,
    source_score: 100,
    product_ready: false,
    product_score: 82,
    installed_binary_smoke: {
      receipt: ".dx/receipts/build/installed-binary-smoke-latest.json",
      status: "pending-governed-refresh",
    },
    receipts: {
      installed_binary_smoke: ".dx/receipts/build/installed-binary-smoke-latest.json",
    },
  });
  writeReceipt(root, "www/.dx/receipts/build/zed-handoff.json", {
    schema: "dx.build.zedHandoff",
    build_readiness: ".dx/receipts/build/readiness.json",
    installed_binary_smoke_receipt: ".dx/receipts/build/installed-binary-smoke-latest.json",
    style_optimization: { style_node_count: 1 },
  });
  writeReceipt(root, ".dx/receipts/build/installed-binary-smoke-latest.json", {
    schema: "dx.build.installedBinarySmoke",
    passed: false,
    failures: ["installed binary is stale", "runtime validation missing"],
  });

  const result = runGate(root, "--json", "--write", "--write-snapshot");

  assert.equal(result.status, 1, result.stdout + result.stderr);
  const report = JSON.parse(result.stdout);
  assert.equal(report.schema, "dx.build.readinessGate");
  assert.equal(report.sourceReady, true);
  assert.equal(report.productReady, false);
  assert.deepEqual(report.score, { product: 82, source: 100 });
  assert.deepEqual(report.blockers, [
    "build readiness product_ready is not true",
    "installed-binary smoke did not pass",
  ]);
  assert.deepEqual(report.proofs.blockers, [
    "release readiness product evidence is not confirmed",
    "installed smoke not product-ready",
    "runtime validation evidence not approved",
  ]);
  assert.deepEqual(
    report.proofs.requiredActionIds,
    report.requiredActions.map((action) => action.id),
  );
  assert.deepEqual(report.proofs.blockerActions, [
    {
      blocker: "release readiness product evidence is not confirmed",
      actionIds: [],
    },
    {
      blocker: "installed smoke not product-ready",
      actionIds: ["refresh-installed-binary-smoke"],
    },
    {
      blocker: "runtime validation evidence not approved",
      actionIds: ["run-governed-runtime-proof"],
    },
  ]);
  assert.equal(report.receipts.readiness.source.workspace, "www");
  assert.equal(report.receipts.zedHandoff.source.workspace, "www");
  assert.equal(report.receipts.installedBinarySmoke.source.kind, "hub");
  assert.equal(report.receipts.installedBinarySmoke.failureCount, 2);
  assert.deepEqual(
    report.requiredActions.map((action) => action.id),
    ["refresh-installed-binary-smoke", "run-governed-runtime-proof"],
  );
  assert.equal(report.requiredActions[0].consumers.zedPreview, true);
  assert.equal(report.requiredActions[0].consumers.friday, true);
  assert.equal(report.requiredActions[0].receipt, ".dx/receipts/build/installed-binary-smoke-latest.json");
  assert.equal(report.requiredActions[0].writesReceipts, true);
  assert.equal(
    report.requiredActions[0].command,
    "node www/tools/build/dx-build-installed-smoke.ts --json --require-product --receipt .dx/receipts/build/installed-binary-smoke-latest.json",
  );
  assert.equal(report.requiredActions[0].riskLevel, "review");
  assert.equal(report.requiredActions[0].requiresApproval, true);
  assert.equal(report.requiredActions[1].command, "dx check launch --json");
  assert.equal(report.requiredActions[1].receipt, ".dx/receipts/check/check-latest.json");
  assert.equal(report.requiredActions[1].riskLevel, "review");
  assert.equal(report.requiredActions[1].requiresApproval, true);
  assert.equal(report.requiredActions[1].writesReceipts, true);
  assert.equal(report.quality.smallModuleBoundary, true);
  assert.equal(report.quality.entrypointUsesSplitModules, true);
  const qualityFilePaths = report.quality.files.map((file) => file.path);
  for (const file of installedSmokeSourceFiles) {
    assert.ok(qualityFilePaths.includes(file), `release readiness quality should include ${file}`);
  }
  assert.equal(report.consumers.friday.primaryReceipt, ".dx/receipts/build/readiness-gate-latest.json");
  assert.equal(report.consumers.friday.primaryField, "requiredActions");
  assert.equal(report.receiptPath, path.join(root, ".dx", "receipts", "build", "readiness-gate-latest.json"));

  const persisted = readReceipt(root, ".dx/receipts/build/readiness-gate-latest.json");
  assert.equal(persisted.schema, "dx.build.readinessGate");
  const snapshot = readReceipt(root, ".dx/receipts/build/readiness-gate-consumer-snapshot.json");
  assert.equal(snapshot.schema, "dx.build.readinessGate.consumerSnapshot");
  assert.equal(snapshot.status.blockerCount, 2);
  assert.deepEqual(
    snapshot.requiredActions.map((action) => action.id),
    report.requiredActions.map((action) => action.id),
  );
  assert.equal(
    snapshot.requiredActions[0].command,
    "node www/tools/build/dx-build-installed-smoke.ts --json --require-product --receipt .dx/receipts/build/installed-binary-smoke-latest.json",
  );
  assert.equal(snapshot.requiredActions[0].riskLevel, "review");
  assert.equal(snapshot.requiredActions[0].writesReceipts, true);
  assert.equal(snapshot.requiredActions[0].consumers.friday, true);
  assert.equal(snapshot.requiredActions[1].receipt, ".dx/receipts/check/check-latest.json");
  assert.deepEqual(snapshot.proofs.requiredActionIds, [
    "refresh-installed-binary-smoke",
    "run-governed-runtime-proof",
  ]);
  assert.equal(snapshot.consumers.dxCli.command, "dx status");
  assert.equal(snapshot.consumers.friday.primaryReceipt, ".dx/receipts/build/readiness-gate-latest.json");
  assert.equal(snapshot.consumers.friday.primaryField, "requiredActions");
});

test("human output uses release readiness wording without changing receipt contracts", () => {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), "dx-build-readiness-gate-human-"));
  writeReceipt(root, "www/.dx/receipts/build/readiness.json", {
    schema: "dx.build.readiness",
    source_ready: true,
    source_score: 100,
    product_ready: false,
    product_score: 82,
  });

  const result = runGate(root);

  assert.equal(result.status, 1, result.stdout + result.stderr);
  assert.match(result.stdout, /^DX build release readiness: blocked/m);
  assert.doesNotMatch(result.stdout, /readiness gate/i);
});

test("release readiness writer passes only when readiness and installed smoke are product ready", () => {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), "dx-build-readiness-gate-ready-"));
  writeReceipt(root, "www/.dx/receipts/build/readiness.json", {
    schema: "dx.build.readiness",
    source_ready: true,
    source_score: 100,
    product_ready: true,
    product_score: 100,
    installed_binary_smoke: {
      receipt: ".dx/receipts/build/installed-binary-smoke-latest.json",
      status: "passed",
    },
    receipts: {
      installed_binary_smoke: ".dx/receipts/build/installed-binary-smoke-latest.json",
    },
  });
  writeReceipt(root, "www/.dx/receipts/build/zed-handoff.json", {
    schema: "dx.build.zedHandoff",
    build_readiness: ".dx/receipts/build/readiness.json",
    installed_binary_smoke_receipt: ".dx/receipts/build/installed-binary-smoke-latest.json",
    style_optimization: { style_node_count: 1 },
  });
  writeReceipt(root, ".dx/receipts/build/installed-binary-smoke-latest.json", {
    schema: "dx.build.installedBinarySmoke",
    binaryRole: "installed-default",
    passed: true,
    failures: [],
  });
  writeReceipt(root, ".dx/receipts/check/check-latest.json", {
    launch_approved: {
      approved: true,
      status: "ready",
    },
    score: 500,
    max_score: 500,
  });

  const result = runGate(root, "--json");

  assert.equal(result.status, 0, result.stdout + result.stderr);
  const report = JSON.parse(result.stdout);
  assert.equal(report.status, "ready");
  assert.equal(report.sourceReady, true);
  assert.equal(report.productReady, true);
  assert.deepEqual(report.blockers, []);
  assert.deepEqual(report.requiredActions, []);
});

test("release readiness writer dry-runs a practical Windows proof bundle", () => {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), "dx-build-readiness-gate-proof-bundle-"));

  const result = runGate(root, "--json", "--proof-bundle", "--dry-run");

  assert.equal(result.status, 1, result.stdout + result.stderr);
  const report = JSON.parse(result.stdout);
  assert.equal(report.proofBundle.mode, "dry-run");
  assert.ok(report.proofBundle.command.includes("--project"));
  assert.ok(report.proofBundle.command.includes(root));
  assert.deepEqual(
    report.proofBundle.steps.map((step) => step.id),
    [
      "cargo-check-dx-www-cli",
      "focused-readiness-node-test",
      "dx-build-installed-smoke",
      "safe-http-root-probe",
      "safe-http-dashboard-probe",
      "safe-http-login-probe",
      "safe-http-favicon-probe",
      "safe-http-hot-reload-probe",
    ],
  );
  assert.equal(
    report.proofBundle.steps[0].command,
    "cargo check -p dx-www --no-default-features --features cli --bin dx-www -j 1",
  );
  assert.equal(
    report.proofBundle.steps[1].command,
    "node --test benchmarks/dx-build-readiness-gate.test.ts",
  );
  assert.ok(report.proofBundle.steps[2].command.includes("--project"));
  assert.ok(report.proofBundle.steps[2].command.includes(root));
  assert.equal(report.proofBundle.steps[3].url, "http://127.0.0.1:3000/");
  assert.equal(report.proofBundle.steps[4].url, "http://127.0.0.1:3000/dashboard");
  assert.equal(report.proofBundle.steps[5].url, "http://127.0.0.1:3000/login");
  assert.equal(report.proofBundle.steps[6].url, "http://127.0.0.1:3000/favicon.svg");
  assert.equal(report.proofBundle.steps[6].expectContentTypeIncludes, "image/svg+xml");
  assert.equal(report.proofBundle.steps[3].safeWhen, "only probes an already-running local server");
  assert.equal(report.proofBundle.summary.total, 8);
  assert.equal(report.proofBundle.summary.executed, 0);
});

test("release readiness writer proof bundle marks HTTP proof as probe-only", () => {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), "dx-build-readiness-gate-http-safety-"));

  const result = runGate(root, "--json", "--proof-bundle", "--dry-run");

  assert.equal(result.status, 1, result.stdout + result.stderr);
  const report = JSON.parse(result.stdout);
  const httpSteps = report.proofBundle.steps.filter((step) => step.kind === "http-probe");
  assert.equal(httpSteps.length, 5);
  for (const step of httpSteps) {
    assert.equal(step.startsServer, false);
    assert.equal(step.optionalWhenUnavailable, true);
    assert.equal(step.safeWhen, "only probes an already-running local server");
  }
  assert.equal(report.proofBundle.steps[0].maxConcurrency, 1);
});

test("release readiness writer proof bundle records the selected project in replay commands", () => {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), "dx-build-readiness-gate-project-proof-"));

  const result = runGate(root, "--json", "--proof-bundle", "--dry-run");

  assert.equal(result.status, 1, result.stdout + result.stderr);
  const report = JSON.parse(result.stdout);
  const installedSmoke = report.proofBundle.steps.find((step) => step.id === "dx-build-installed-smoke");
  assert.ok(report.proofBundle.command.includes("--project"));
  assert.ok(report.proofBundle.command.includes(root));
  assert.ok(installedSmoke.command.includes("--project"));
  assert.ok(installedSmoke.command.includes(root));
});

test("release readiness writer proof bundle labels timed-out command steps", () => {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), "dx-build-readiness-gate-timeout-"));

  const bundle = runProofSteps(root, [
    {
      id: "blocked-cargo-proof",
      kind: "command",
      writesReceipts: false,
      label: "Blocked cargo proof",
      command: "node -e block past timeout",
      executable: process.execPath,
      args: ["-e", "Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, 250)"],
      timeoutMs: 50,
    },
  ]);

  assert.equal(bundle.steps[0].status, "failed");
  assert.equal(bundle.steps[0].timedOut, true);
  assert.equal(bundle.steps[0].failureReason, "timeout");
  assert.equal(bundle.steps[0].timeoutMs, 50);
  assert.equal(bundle.summary.failed, 1);
});

test("release readiness writer proof bundle skips receipt-writing steps after a failed proof", () => {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), "dx-build-readiness-gate-skip-write-"));
  const marker = "should-not-write-receipt.txt";

  const bundle = runProofSteps(root, [
    {
      id: "failing-source-proof",
      kind: "command",
      writesReceipts: false,
      label: "Failing source proof",
      command: "node -e process.exit(9)",
      executable: process.execPath,
      args: ["-e", "process.exit(9)"],
      timeoutMs: 10000,
    },
    {
      id: "receipt-writing-smoke",
      kind: "command",
      writesReceipts: true,
      label: "Receipt-writing smoke",
      command: "node -e write receipt marker",
      executable: process.execPath,
      args: [
        "-e",
        `require('node:fs').writeFileSync(${JSON.stringify(marker)}, 'stale')`,
      ],
      timeoutMs: 10000,
    },
  ]);

  assert.equal(bundle.mode, "run");
  assert.equal(bundle.steps[0].status, "failed");
  assert.equal(bundle.steps[1].status, "skipped");
  assert.equal(bundle.steps[1].skipReason, "blocked-by-prior-failure");
  assert.equal(fs.existsSync(path.join(root, marker)), false);
  assert.deepEqual(bundle.summary, {
    total: 2,
    executed: 1,
    passed: 0,
    failed: 1,
    skipped: 1,
  });
});

test("release readiness writer blocks product readiness when an executed proof bundle step fails", () => {
  const receipts = readyReceipts();
  const report = createReport("G:/Dx/www", receipts, { smallModuleBoundary: true }, {
    mode: "run",
    summary: { failed: 1 },
    steps: [
      {
        id: "cargo-check-dx-www-cli",
        status: "failed",
      },
    ],
  });

  assert.equal(report.sourceReady, true);
  assert.equal(report.productReady, false);
  assert.ok(report.blockers.includes("validation bundle step failed: cargo-check-dx-www-cli"));
});

test("release readiness writer rejects candidate override smoke as installed-binary product evidence", () => {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), "dx-build-readiness-gate-candidate-"));
  writeReceipt(root, "www/.dx/receipts/build/readiness.json", {
    schema: "dx.build.readiness",
    source_ready: true,
    source_score: 100,
    product_ready: true,
    product_score: 100,
    installed_binary_smoke: {
      receipt: ".dx/receipts/build/installed-binary-smoke-latest.json",
      status: "passed",
    },
    receipts: {
      installed_binary_smoke: ".dx/receipts/build/installed-binary-smoke-latest.json",
    },
  });
  writeReceipt(root, "www/.dx/receipts/build/zed-handoff.json", {
    schema: "dx.build.zedHandoff",
    build_readiness: ".dx/receipts/build/readiness.json",
    installed_binary_smoke_receipt: ".dx/receipts/build/installed-binary-smoke-latest.json",
    style_optimization: { style_node_count: 1 },
  });
  writeReceipt(root, ".dx/receipts/build/installed-binary-smoke-latest.json", {
    schema: "dx.build.installedBinarySmoke",
    binaryRole: "candidate-override",
    passed: true,
    failures: [],
  });
  writeReceipt(root, ".dx/receipts/check/check-latest.json", {
    launch_approved: {
      approved: true,
      status: "ready",
    },
    score: 500,
    max_score: 500,
  });

  const result = runGate(root, "--json");

  assert.equal(result.status, 1, result.stdout + result.stderr);
  const report = JSON.parse(result.stdout);
  assert.equal(report.sourceReady, true);
  assert.equal(report.productReady, false);
  assert.ok(
    report.blockers.includes("installed-binary smoke was not run against the installed default binary"),
    JSON.stringify(report.blockers, null, 2),
  );
  assert.deepEqual(
    report.requiredActions.map((action) => action.id),
    ["refresh-installed-binary-smoke"],
  );
});

test("release readiness writer rejects installed smoke when product evidence is explicitly ineligible", () => {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), "dx-build-readiness-gate-ineligible-"));
  writeReceipt(root, "www/.dx/receipts/build/readiness.json", {
    schema: "dx.build.readiness",
    source_ready: true,
    source_score: 100,
    product_ready: true,
    product_score: 100,
    installed_binary_smoke: {
      receipt: ".dx/receipts/build/installed-binary-smoke-latest.json",
      status: "passed",
    },
    receipts: {
      installed_binary_smoke: ".dx/receipts/build/installed-binary-smoke-latest.json",
    },
  });
  writeReceipt(root, "www/.dx/receipts/build/zed-handoff.json", {
    schema: "dx.build.zedHandoff",
    build_readiness: ".dx/receipts/build/readiness.json",
    installed_binary_smoke_receipt: ".dx/receipts/build/installed-binary-smoke-latest.json",
    style_optimization: { style_node_count: 1 },
  });
  writeReceipt(root, ".dx/receipts/build/installed-binary-smoke-latest.json", {
    schema: "dx.build.installedBinarySmoke",
    binaryRole: "installed-default",
    passed: true,
    failures: [],
    proof: {
      scope: "installed-default",
      productEligible: false,
      installedDefaultRequired: true,
      nextAction: "Refresh installed-binary smoke from the governed installed binary.",
    },
  });
  writeReceipt(root, ".dx/receipts/check/check-latest.json", {
    launch_approved: {
      approved: true,
      status: "ready",
    },
    score: 500,
    max_score: 500,
  });

  const result = runGate(root, "--json");

  assert.equal(result.status, 1, result.stdout + result.stderr);
  const report = JSON.parse(result.stdout);
  assert.equal(report.sourceReady, true);
  assert.equal(report.productReady, false);
  assert.ok(
    report.blockers.includes("installed-binary smoke is not product-eligible"),
    JSON.stringify(report.blockers, null, 2),
  );
  assert.deepEqual(report.proofs.blockers, ["installed-default smoke not product-ready"]);
  assert.deepEqual(
    report.requiredActions.map((action) => action.id),
    ["refresh-installed-binary-smoke"],
  );
});

test("release readiness writer snapshots expose product evidence breakdown for consumers", () => {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), "dx-build-readiness-gate-proofs-"));
  writeReceipt(root, "www/.dx/receipts/build/readiness.json", {
    schema: "dx.build.readiness",
    source_ready: true,
    source_score: 100,
    product_ready: true,
    product_score: 100,
    installed_binary_smoke: {
      receipt: ".dx/receipts/build/installed-binary-smoke-latest.json",
      status: "passed",
    },
    receipts: {
      installed_binary_smoke: ".dx/receipts/build/installed-binary-smoke-latest.json",
    },
  });
  writeReceipt(root, "www/.dx/receipts/build/zed-handoff.json", {
    schema: "dx.build.zedHandoff",
    build_readiness: ".dx/receipts/build/readiness.json",
    installed_binary_smoke_receipt: ".dx/receipts/build/installed-binary-smoke-latest.json",
  });
  writeReceipt(root, ".dx/receipts/build/installed-binary-smoke-latest.json", {
    schema: "dx.build.installedBinarySmoke",
    binaryRole: "candidate-override",
    passed: true,
    failures: [],
  });
  writeReceipt(root, ".dx/receipts/check/check-latest.json", {
    launch_approved: {
      approved: true,
      status: "ready",
    },
    score: 500,
    max_score: 500,
  });

  const result = runGate(root, "--json", "--write-snapshot");

  assert.equal(result.status, 1, result.stdout + result.stderr);
  const report = JSON.parse(result.stdout);
  assert.deepEqual(report.proofs, {
    blockerActions: [
      {
        blocker: "candidate-override smoke not product-ready",
        actionIds: ["refresh-installed-binary-smoke"],
      },
    ],
    blockers: ["candidate-override smoke not product-ready"],
    installedSmokeBinaryRole: "candidate-override",
    installedSmokePassed: true,
    installedSmokeReadyForProduct: false,
    productReady: false,
    readinessProductReady: true,
    requiredActionIds: ["refresh-installed-binary-smoke"],
    runtimeProofApproved: true,
    sourceReady: true,
  });

  const snapshot = readReceipt(root, ".dx/receipts/build/readiness-gate-consumer-snapshot.json");
  assert.deepEqual(snapshot.proofs, report.proofs);
});

test("release readiness writer requires governed launch approval before product readiness", () => {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), "dx-build-readiness-gate-runtime-"));
  writeReceipt(root, "www/.dx/receipts/build/readiness.json", {
    schema: "dx.build.readiness",
    source_ready: true,
    source_score: 100,
    product_ready: true,
    product_score: 100,
    installed_binary_smoke: {
      receipt: ".dx/receipts/build/installed-binary-smoke-latest.json",
      status: "passed",
    },
    receipts: {
      installed_binary_smoke: ".dx/receipts/build/installed-binary-smoke-latest.json",
    },
  });
  writeReceipt(root, "www/.dx/receipts/build/zed-handoff.json", {
    schema: "dx.build.zedHandoff",
    build_readiness: ".dx/receipts/build/readiness.json",
    installed_binary_smoke_receipt: ".dx/receipts/build/installed-binary-smoke-latest.json",
    style_optimization: { style_node_count: 1 },
  });
  writeReceipt(root, ".dx/receipts/build/installed-binary-smoke-latest.json", {
    schema: "dx.build.installedBinarySmoke",
    binaryRole: "installed-default",
    passed: true,
    failures: [],
  });

  const result = runGate(root, "--json");

  assert.equal(result.status, 1, result.stdout + result.stderr);
  const report = JSON.parse(result.stdout);
  assert.equal(report.sourceReady, true);
  assert.equal(report.productReady, false);
  assert.ok(
    report.blockers.includes("governed runtime validation receipt is missing"),
    JSON.stringify(report.blockers, null, 2),
  );
  assert.deepEqual(
    report.requiredActions.map((action) => action.id),
    ["run-governed-runtime-proof"],
  );
});

test("release readiness writer asks to refresh build readiness when product confirmation is the last blocker", () => {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), "dx-build-readiness-gate-product-seal-"));
  writeReceipt(root, "www/.dx/receipts/build/readiness.json", {
    schema: "dx.build.readiness",
    source_ready: true,
    source_score: 100,
    product_ready: false,
    product_score: 82,
    installed_binary_smoke: {
      receipt: ".dx/receipts/build/installed-binary-smoke-latest.json",
      status: "passed",
    },
    receipts: {
      installed_binary_smoke: ".dx/receipts/build/installed-binary-smoke-latest.json",
    },
  });
  writeReceipt(root, "www/.dx/receipts/build/zed-handoff.json", {
    schema: "dx.build.zedHandoff",
    build_readiness: ".dx/receipts/build/readiness.json",
    installed_binary_smoke_receipt: ".dx/receipts/build/installed-binary-smoke-latest.json",
    style_optimization: { style_node_count: 1 },
  });
  writeReceipt(root, ".dx/receipts/build/installed-binary-smoke-latest.json", {
    schema: "dx.build.installedBinarySmoke",
    binaryRole: "installed-default",
    passed: true,
    failures: [],
  });
  writeReceipt(root, ".dx/receipts/check/check-latest.json", {
    launch_approved: {
      approved: true,
      status: "ready",
    },
    score: 500,
    max_score: 500,
  });

  const result = runGate(root, "--json");

  assert.equal(result.status, 1, result.stdout + result.stderr);
  const report = JSON.parse(result.stdout);
  assert.equal(report.sourceReady, true);
  assert.equal(report.productReady, false);
  assert.deepEqual(
    report.requiredActions.map((action) => action.id),
    ["refresh-source-build-receipts"],
  );
  assert.deepEqual(report.proofs.blockerActions, [
    {
      blocker: "release readiness product evidence is not confirmed",
      actionIds: ["refresh-source-build-receipts"],
    },
  ]);
  assert.equal(report.requiredActions[0].command, "dx build");
  assert.equal(report.requiredActions[0].riskLevel, "safe");
  assert.equal(report.requiredActions[0].requiresApproval, false);
  assert.match(report.nextAction, /Refresh DX Build readiness/);
});

test("release readiness writer surfaces source-build receipt provenance when readiness projection is missing", () => {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), "dx-build-readiness-gate-source-build-"));
  writeReceipt(root, "www/.dx/receipts/build/latest.json", {
    schema: "dx.www.sourceBuildReceipt",
    summary: {
      routes: 1,
      route_handlers: 1,
      content_documents: 2,
      mdx_documents: 1,
      styles: 1,
      css_original_rules: 7,
      css_retained_rules: 6,
      css_pruned_rules: 1,
      css_minified_styles: 1,
      css_source_maps: 1,
      css_source_map_sources: 3,
      css_flattened_imports: 1,
      css_retained_imports: 1,
      css_asset_references: 2,
      assets: 1,
      image_assets: 1,
      image_metadata_assets: 1,
      optimized_image_variants: 0,
      image_placeholders: 1,
      image_route_references: 1,
      manifest_schema: "dx.www.sourceBuildManifest",
    },
    node_modules_required: false,
  });
  writeReceipt(root, "www/.dx/receipts/build/zed-handoff.json", {
    schema: "dx.build.zedHandoff",
    graph_receipt: ".dx/receipts/graph/latest.json",
  });
  writeReceipt(root, ".dx/receipts/build/installed-binary-smoke-latest.json", {
    schema: "dx.build.installedBinarySmoke",
    passed: false,
    failures: ["installed binary is stale"],
  });

  const result = runGate(root, "--json", "--write-snapshot");

  assert.equal(result.status, 1, result.stdout + result.stderr);
  const report = JSON.parse(result.stdout);
  assert.equal(report.receipts.sourceBuild.present, true);
  assert.equal(report.receipts.sourceBuild.schema, "dx.www.sourceBuildReceipt");
  assert.equal(report.receipts.sourceBuild.source.workspace, "www");
  assert.equal(report.receipts.sourceBuild.summary.routes, 1);
  assert.equal(report.receipts.sourceBuild.summary.routeHandlers, 1);
  assert.equal(report.receipts.sourceBuild.summary.contentDocuments, 2);
  assert.equal(report.receipts.sourceBuild.summary.mdxDocuments, 1);
  assert.equal(report.receipts.sourceBuild.summary.cssSourceMaps, 1);
  assert.equal(report.receipts.sourceBuild.summary.cssFlattenedImports, 1);
  assert.equal(report.receipts.sourceBuild.summary.cssAssetReferences, 2);
  assert.equal(report.receipts.sourceBuild.summary.imageAssets, 1);
  assert.equal(report.receipts.sourceBuild.summary.imagePlaceholders, 1);
  assert.equal(report.receipts.sourceBuild.summary.nodeModulesRequired, false);
  assert.ok(
    report.blockers.includes("build readiness projection is missing while source build receipt exists"),
    JSON.stringify(report.blockers, null, 2),
  );

  const snapshot = readReceipt(root, ".dx/receipts/build/readiness-gate-consumer-snapshot.json");
  assert.equal(snapshot.receipts.sourceBuild.present, true);
  assert.equal(snapshot.receipts.sourceBuild.summary.manifestSchema, "dx.www.sourceBuildManifest");
  assert.equal(snapshot.receipts.sourceBuild.summary.routeHandlers, 1);
  assert.equal(snapshot.receipts.sourceBuild.summary.contentDocuments, 2);
  assert.equal(snapshot.receipts.sourceBuild.summary.cssSourceMaps, 1);
  assert.equal(snapshot.receipts.sourceBuild.summary.imageAssets, 1);
});

test("release readiness writer rejects source-build receipts with node_modules paths", () => {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), "dx-build-readiness-gate-node-modules-"));
  writeReceipt(root, "www/.dx/receipts/build/latest.json", {
    schema: "dx.www.sourceBuildReceipt",
    summary: {
      routes: 1,
      route_outputs: 1,
      source_module_chunks: 1,
      styles: 1,
      assets: 1,
      manifest_schema: "dx.www.sourceBuildManifest",
    },
    node_modules_required: false,
    source_modules: [
      {
        source_path: "node_modules/react/index.js",
        chunk_path: ".dx/build/chunks/react.js",
      },
    ],
  });
  writeReceipt(root, "www/.dx/receipts/build/zed-handoff.json", {
    schema: "dx.build.zedHandoff",
    graph_receipt: ".dx/receipts/graph/latest.json",
  });
  writeReceipt(root, ".dx/receipts/build/installed-binary-smoke-latest.json", {
    schema: "dx.build.installedBinarySmoke",
    passed: false,
    failures: ["installed binary is stale"],
  });

  const result = runGate(root, "--json", "--write-source-projection");

  assert.equal(result.status, 1, result.stdout + result.stderr);
  const report = JSON.parse(result.stdout);
  assert.equal(report.receipts.sourceBuild.summary.nodeModulesRequired, false);
  assert.equal(report.receipts.sourceBuild.summary.nodeModulesPathCount, 1);
  assert.deepEqual(report.receipts.sourceBuild.summary.nodeModulesPaths, [
    "node_modules/react/index.js",
  ]);
  assert.ok(
    report.blockers.includes("source build receipt contains node_modules paths"),
    JSON.stringify(report.blockers, null, 2),
  );
  assert.equal(report.sourceReady, false);

  const readiness = readReceipt(root, ".dx/receipts/build/readiness.json");
  assert.equal(readiness.source_ready, false);
  assert.equal(readiness.graph.node_modules_required, true);
  assert.equal(readiness.graph.node_modules_path_count, 1);
  assert.deepEqual(readiness.graph.node_modules_paths, ["node_modules/react/index.js"]);
});

test("release readiness writer rejects source-build receipts that require node_modules", () => {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), "dx-build-readiness-gate-node-required-"));
  writeReceipt(root, "www/.dx/receipts/build/latest.json", {
    schema: "dx.www.sourceBuildReceipt",
    summary: {
      routes: 1,
      route_outputs: 1,
      source_module_chunks: 1,
      styles: 1,
      assets: 1,
      manifest_schema: "dx.www.sourceBuildManifest",
    },
    node_modules_required: true,
  });
  writeReceipt(root, "www/.dx/receipts/build/zed-handoff.json", {
    schema: "dx.build.zedHandoff",
    graph_receipt: ".dx/receipts/graph/latest.json",
  });
  writeReceipt(root, ".dx/receipts/build/installed-binary-smoke-latest.json", {
    schema: "dx.build.installedBinarySmoke",
    passed: false,
    failures: ["installed binary is stale"],
  });

  const result = runGate(root, "--json", "--write-source-projection");

  assert.equal(result.status, 1, result.stdout + result.stderr);
  const report = JSON.parse(result.stdout);
  assert.equal(report.receipts.sourceBuild.summary.nodeModulesRequired, true);
  assert.ok(
    report.blockers.includes("source build receipt requires node_modules"),
    JSON.stringify(report.blockers, null, 2),
  );
  assert.equal(report.sourceReady, false);

  const readiness = readReceipt(root, ".dx/receipts/build/readiness.json");
  assert.equal(readiness.source_ready, false);
  assert.equal(readiness.graph.node_modules_required, true);
  assert.equal(readiness.graph.node_modules_path_count, 0);
  assert.deepEqual(readiness.graph.node_modules_paths, []);
});

test("release readiness writer rejects route-handler receipt collections that require node_modules", () => {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), "dx-build-readiness-gate-route-receipts-"));
  writeReceipt(root, "www/.dx/receipts/build/latest.json", {
    schema: "dx.www.sourceBuildReceipt",
    summary: {
      routes: 1,
      route_handlers: 1,
      route_handler_receipt_output: ".dx/build/.dx/build-cache/route-handler-receipts.json",
      route_handler_receipts_executed: 1,
      route_handler_receipts_skipped: 0,
      route_handler_receipts_node_modules_required: true,
      route_handler_receipts_lifecycle_scripts_executed: false,
      route_outputs: 1,
      source_module_chunks: 1,
      styles: 1,
      assets: 1,
      manifest_schema: "dx.www.sourceBuildManifest",
    },
    node_modules_required: false,
  });
  writeReceipt(root, "www/.dx/receipts/build/zed-handoff.json", {
    schema: "dx.build.zedHandoff",
    graph_receipt: ".dx/receipts/graph/latest.json",
  });
  writeReceipt(root, ".dx/receipts/build/installed-binary-smoke-latest.json", {
    schema: "dx.build.installedBinarySmoke",
    passed: false,
    failures: ["installed binary is stale"],
  });

  const result = runGate(root, "--json", "--write-source-projection");

  assert.equal(result.status, 1, result.stdout + result.stderr);
  const report = JSON.parse(result.stdout);
  assert.equal(report.receipts.sourceBuild.summary.routeHandlerReceiptOutput, ".dx/build/.dx/build-cache/route-handler-receipts.json");
  assert.equal(report.receipts.sourceBuild.summary.routeHandlerReceiptsExecuted, 1);
  assert.equal(report.receipts.sourceBuild.summary.routeHandlerReceiptsNodeModulesRequired, true);
  assert.ok(
    report.blockers.includes("route-handler receipt collection requires node_modules"),
    JSON.stringify(report.blockers, null, 2),
  );
  assert.equal(report.sourceReady, false);

  const readiness = readReceipt(root, ".dx/receipts/build/readiness.json");
  assert.equal(readiness.source_ready, false);
  assert.equal(readiness.graph.route_handler_receipt_output, ".dx/build/.dx/build-cache/route-handler-receipts.json");
  assert.equal(readiness.graph.route_handler_receipts_executed, 1);
  assert.equal(readiness.graph.route_handler_receipts_node_modules_required, true);
  assert.equal(readiness.graph.node_modules_required, true);
});

test("release readiness writer writes source projection receipts without claiming runtime readiness", () => {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), "dx-build-readiness-gate-projection-"));
  writeReceipt(root, "www/.dx/receipts/build/latest.json", {
    schema: "dx.www.sourceBuildReceipt",
    summary: {
      routes: 2,
      route_handlers: 1,
      route_handler_receipt_output: ".dx/build/.dx/build-cache/route-handler-receipts.json",
      route_handler_receipts_executed: 1,
      route_handler_receipts_skipped: 1,
      route_handler_receipts_node_modules_required: false,
      route_handler_receipts_lifecycle_scripts_executed: false,
      route_outputs: 2,
      source_module_chunks: 4,
      styles: 3,
      content_documents: 2,
      mdx_documents: 1,
      css_original_rules: 9,
      css_retained_rules: 7,
      css_pruned_rules: 2,
      css_minified_styles: 2,
      css_source_maps: 2,
      css_source_map_sources: 5,
      css_flattened_imports: 2,
      css_retained_imports: 1,
      css_asset_references: 3,
      assets: 1,
      image_assets: 1,
      image_metadata_assets: 1,
      optimized_image_variants: 0,
      image_placeholders: 1,
      image_route_references: 1,
      manifest_schema: "dx.www.sourceBuildManifest",
    },
    node_modules_required: false,
    integration_boundary: "source-owned build graph, governed runtime validation pending",
  });
  writeReceipt(root, "www/.dx/receipts/next-rust/vendor-boundary-consumer.json", {
    schema: "dx.nextRust.vendorBoundary.consumerReceipt",
    status: "ok",
    snapshot: {
      schema: "dx.nextRust.vendorBoundary.consumerSnapshot",
      status: "ok",
      upstream: {
        repository: "vercel/next.js",
        commit: "f3f56ecec2f3f8cefa0f0a1323ea406740251d5c",
      },
      claimPolicy: {
        fullNextParityClaimed: false,
        nextRuntimeTakeoverClaimed: false,
        nodeModulesDefaultClaimed: false,
      },
      boundary: {
        workspaceQuarantined: true,
        runtimeTakeoverBlocked: true,
        nodeModulesDefault: false,
        turbopackPublicArchitecture: false,
      },
    },
  });
  writeReceipt(root, ".dx/receipts/build/installed-binary-smoke-latest.json", {
    schema: "dx.build.installedBinarySmoke",
    passed: false,
    failures: ["installed binary is stale"],
  });

  const result = runGate(root, "--json", "--write-source-projection");

  assert.equal(result.status, 1, result.stdout + result.stderr);
  const report = JSON.parse(result.stdout);
  assert.equal(report.sourceReady, true);
  assert.equal(report.productReady, false);
  assert.deepEqual(report.score, { product: 82, source: 100 });
  assert.deepEqual(report.blockers, [
    "build readiness product_ready is not true",
    "installed-binary smoke did not pass",
  ]);
  assert.match(report.nextAction, /Refresh installed-binary smoke/);
  assert.doesNotMatch(report.nextAction, /Refresh build readiness/);
  assert.deepEqual(
    report.requiredActions.map((action) => action.id),
    ["refresh-installed-binary-smoke", "run-governed-runtime-proof"],
  );
  assert.equal(report.receipts.nextRustBoundary.present, true);
  assert.equal(report.receipts.nextRustBoundary.status, "ok");
  assert.equal(report.receipts.nextRustBoundary.runtimeTakeoverBlocked, true);

  const readiness = readReceipt(root, ".dx/receipts/build/readiness.json");
  assert.equal(readiness.schema, "dx.build.readiness");
  assert.equal(readiness.source_ready, true);
  assert.equal(readiness.product_ready, false);
  assert.equal(readiness.product_score, 82);
  assert.equal(readiness.product_score_ceiling, 82);
  assert.ok(readiness.product_score_basis.includes("installed-binary-smoke-pending"));
  assert.ok(readiness.product_score_basis.includes("runtime-proof-pending"));
  assert.equal(readiness.receipts.source_build_receipt, "www/.dx/receipts/build/latest.json");
  assert.equal(readiness.receipts.installed_binary_smoke, ".dx/receipts/build/installed-binary-smoke-latest.json");
  assert.equal(readiness.graph.routes, 2);
  assert.equal(readiness.graph.route_handlers, 1);
  assert.equal(readiness.graph.route_handler_receipt_output, ".dx/build/.dx/build-cache/route-handler-receipts.json");
  assert.equal(readiness.graph.route_handler_receipts_executed, 1);
  assert.equal(readiness.graph.route_handler_receipts_skipped, 1);
  assert.equal(readiness.graph.route_handler_receipts_node_modules_required, false);
  assert.equal(readiness.graph.route_handler_receipts_lifecycle_scripts_executed, false);
  assert.equal(readiness.graph.content_documents, 2);
  assert.equal(readiness.graph.mdx_documents, 1);
  assert.equal(readiness.graph.css_source_maps, 2);
  assert.equal(readiness.graph.image_assets, 1);
  assert.equal(readiness.graph.image_placeholders, 1);
  assert.equal(readiness.graph.node_modules_required, false);
  assert.equal(readiness.next_rust_merge.status, "ok");
  assert.equal(readiness.next_rust_merge.runtime_takeover_blocked, true);
  assert.equal(readiness.next_rust_merge.full_nextjs_parity_claimed, false);

  const handoff = readReceipt(root, ".dx/receipts/build/zed-handoff.json");
  assert.equal(handoff.schema, "dx.build.zedHandoff");
  assert.equal(handoff.build_readiness, ".dx/receipts/build/readiness.json");
  assert.equal(handoff.source_build_receipt, "www/.dx/receipts/build/latest.json");
  assert.equal(handoff.installed_binary_smoke_receipt, ".dx/receipts/build/installed-binary-smoke-latest.json");
  assert.equal(handoff.route_handlers, 1);
  assert.equal(handoff.content_pipeline.document_count, 2);
  assert.equal(handoff.content_pipeline.mdx_document_count, 1);
  assert.equal(handoff.style_optimization.source_map_count, 2);
  assert.equal(handoff.style_optimization.flattened_import_count, 2);
  assert.equal(handoff.image_pipeline.image_asset_count, 1);
  assert.equal(handoff.image_pipeline.placeholder_count, 1);
  assert.equal(
    handoff.next_rust_merge.consumer_receipt,
    "www/.dx/receipts/next-rust/vendor-boundary-consumer.json",
  );
});

test("release readiness writer source stays split into small professional modules", () => {
  const sourceFiles = [
    "tools/build/dx-build-readiness-gate.ts",
    "tools/build/readiness-gate/cli.ts",
    "tools/build/readiness-gate/consumers.ts",
    "tools/build/readiness-gate/actions.ts",
    "tools/build/readiness-gate/constants.ts",
    "tools/build/readiness-gate/io.ts",
    "tools/build/readiness-gate/proofs.ts",
    "tools/build/readiness-gate/proof-bundle.ts",
    "tools/build/readiness-gate/proof-http-probe.ts",
    "tools/build/readiness-gate/proof-runner.ts",
    "tools/build/readiness-gate/quality.ts",
    "tools/build/readiness-gate/projection.ts",
    "tools/build/readiness-gate/projection-sections.ts",
    "tools/build/readiness-gate/receipt-checks.ts",
    "tools/build/readiness-gate/receipt-sources.ts",
    "tools/build/readiness-gate/report.ts",
    "tools/build/readiness-gate/snapshot.ts",
    "tools/build/readiness-gate/source-build.ts",
    ...installedSmokeSourceFiles,
  ];

  for (const relative of sourceFiles) {
    const source = fs.readFileSync(path.join(repoRoot, relative), "utf8");
    assert.ok(!source.includes(".v1"), `${relative} should not use .v1 schema names`);
    assert.ok(source.split(/\r?\n/).length <= 180, `${relative} is too large`);
  }
});

function runGate(root, ...args) {
  return spawnSync(process.execPath, [gatePath, "--project", root, ...args], {
    cwd: repoRoot,
    encoding: "utf8",
  });
}

function writeReceipt(root, relative, value) {
  const target = path.join(root, relative);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, `${JSON.stringify(value, null, 2)}\n`);
}

function readReceipt(root, relative) {
  return JSON.parse(fs.readFileSync(path.join(root, relative), "utf8"));
}

function readyReceipts() {
  return {
    checkLaunch: receipt({
      launch_approved: {
        approved: true,
        status: "ready",
      },
      score: 500,
      max_score: 500,
    }),
    installedBinarySmoke: receipt({
      schema: "dx.build.installedBinarySmoke",
      binaryRole: "installed-default",
      passed: true,
      failures: [],
    }),
    nextRustBoundary: missingReceipt(".dx/receipts/next-rust/vendor-boundary-consumer.json"),
    readiness: receipt({
      schema: "dx.build.readiness",
      source_ready: true,
      source_score: 100,
      product_ready: true,
      product_score: 100,
      installed_binary_smoke: {
        receipt: ".dx/receipts/build/installed-binary-smoke-latest.json",
        status: "passed",
      },
      receipts: {
        installed_binary_smoke: ".dx/receipts/build/installed-binary-smoke-latest.json",
      },
    }),
    sourceBuild: missingReceipt(".dx/receipts/build/latest.json"),
    zedHandoff: receipt({
      schema: "dx.build.zedHandoff",
      build_readiness: ".dx/receipts/build/readiness.json",
      installed_binary_smoke_receipt: ".dx/receipts/build/installed-binary-smoke-latest.json",
    }),
  };
}

function receipt(value) {
  return {
    malformed: false,
    path: ".dx/receipts/test.json",
    present: true,
    source: { kind: "hub", workspace: null, path: ".dx/receipts/test.json" },
    value,
  };
}

function missingReceipt(relativePath) {
  return {
    malformed: false,
    path: relativePath,
    present: false,
    source: { kind: "hub", workspace: null, path: relativePath },
    value: null,
  };
}
