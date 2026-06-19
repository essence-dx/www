import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const { spawnSync } = require("node:child_process");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const runner = path.join(root, "tools/dx-style/live-tailwind-v43-compare.cjs");
const matrix = "related-crates/style/fixtures/tailwind-v43-official-fixture-matrix.json";
const sha256Pattern = /^[a-f0-9]{64}$/;

function runRunner(args, env = {}) {
  return spawnSync(process.execPath, [runner, ...args], {
    cwd: root,
    encoding: "utf8",
    env: { ...process.env, ...env },
  });
}

function makeStaleFixtureBinary() {
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-style-stale-fixture-bin-"));
  const binaryPath = path.join(tempDir, process.platform === "win32" ? "fixture.exe" : "fixture");
  fs.writeFileSync(binaryPath, "");
  fs.utimesSync(binaryPath, new Date("2000-01-01T00:00:00Z"), new Date("2000-01-01T00:00:00Z"));
  return { tempDir, binaryPath };
}

function makeFixtureBinaryWithReceipt(tempDir, receipt) {
  const receiptPath = path.join(tempDir, "fixture-receipt.json");
  const scriptPath = path.join(tempDir, "fixture.cjs");
  fs.writeFileSync(receiptPath, `${JSON.stringify(receipt, null, 2)}\n`);
  fs.writeFileSync(
    scriptPath,
    `const fs = require("node:fs");\nprocess.stdout.write(fs.readFileSync(${JSON.stringify(receiptPath)}, "utf8"));\n`,
  );

  const binaryPath = path.join(tempDir, process.platform === "win32" ? "fixture.cmd" : "fixture");
  if (process.platform === "win32") {
    fs.writeFileSync(binaryPath, `@echo off\r\nnode "%~dp0fixture.cjs"\r\n`);
  } else {
    fs.writeFileSync(binaryPath, `#!/usr/bin/env sh\nnode "$(dirname "$0")/fixture.cjs"\n`);
    fs.chmodSync(binaryPath, 0o755);
  }
  fs.utimesSync(binaryPath, new Date("2099-01-01T00:00:00Z"), new Date("2099-01-01T00:00:00Z"));
  return binaryPath;
}

function makeFailingCommand(tempDir, commandName, message) {
  const scriptPath = path.join(tempDir, `${commandName}-fail.cjs`);
  fs.writeFileSync(
    scriptPath,
    `process.stderr.write(${JSON.stringify(`${message}\n`)});\nprocess.exit(42);\n`,
  );
  const commandPath = path.join(tempDir, process.platform === "win32" ? `${commandName}.cmd` : commandName);
  if (process.platform === "win32") {
    fs.writeFileSync(commandPath, `@echo off\r\nnode "%~dp0${commandName}-fail.cjs"\r\n`);
  } else {
    fs.writeFileSync(commandPath, `#!/usr/bin/env sh\nnode "$(dirname "$0")/${commandName}-fail.cjs"\n`);
    fs.chmodSync(commandPath, 0o755);
  }
  fs.utimesSync(commandPath, new Date("2099-01-01T00:00:00Z"), new Date("2099-01-01T00:00:00Z"));
  return commandPath;
}

function makeSleepingCommand(tempDir, commandName, delayMs) {
  const scriptPath = path.join(tempDir, `${commandName}-sleep.cjs`);
  fs.writeFileSync(scriptPath, `setTimeout(() => {}, ${delayMs});\n`);
  const commandPath = path.join(tempDir, process.platform === "win32" ? `${commandName}.cmd` : commandName);
  if (process.platform === "win32") {
    fs.writeFileSync(commandPath, `@echo off\r\nnode "%~dp0${commandName}-sleep.cjs"\r\n`);
  } else {
    fs.writeFileSync(commandPath, `#!/usr/bin/env sh\nnode "$(dirname "$0")/${commandName}-sleep.cjs"\n`);
    fs.chmodSync(commandPath, 0o755);
  }
  fs.utimesSync(commandPath, new Date("2099-01-01T00:00:00Z"), new Date("2099-01-01T00:00:00Z"));
  return commandPath;
}

function makeCargoTargetGuardCommand(tempDir) {
  const scriptPath = path.join(tempDir, "cargo-target-guard.cjs");
  fs.writeFileSync(
    scriptPath,
    `
if (!process.env.CARGO_TARGET_DIR) {
  process.stderr.write("missing CARGO_TARGET_DIR\\n");
  process.exit(42);
}
if (!process.env.CARGO_TARGET_DIR.includes("dx-style-live-cargo-target")) {
  process.stderr.write("unexpected CARGO_TARGET_DIR: " + process.env.CARGO_TARGET_DIR + "\\n");
  process.exit(42);
}
process.stdout.write(JSON.stringify({
  classes: [
    { class_name: "block", supported: true, generated_css: ".block { display: block; }" }
  ]
}) + "\\n");
`,
  );
  const commandPath = path.join(tempDir, process.platform === "win32" ? "cargo-target-guard.cmd" : "cargo-target-guard");
  if (process.platform === "win32") {
    fs.writeFileSync(commandPath, `@echo off\r\nnode "%~dp0cargo-target-guard.cjs"\r\n`);
  } else {
    fs.writeFileSync(commandPath, `#!/usr/bin/env sh\nnode "$(dirname "$0")/cargo-target-guard.cjs"\n`);
    fs.chmodSync(commandPath, 0o755);
  }
  return commandPath;
}

function makeTailwindCssFile(tempDir) {
  const tailwindCssPath = path.join(tempDir, "tailwind.css");
  fs.writeFileSync(
    tailwindCssPath,
    ".block { display: block; }\n@unknown rule { .\\[\\@unknown_rule\\]\\:p-4 { padding: 1rem; } }\n",
  );
  return tailwindCssPath;
}

function makeMinimalMatrix(tempDir) {
  const officialMatrix = JSON.parse(fs.readFileSync(path.join(root, matrix), "utf8"));
  const inventory = JSON.parse(
    fs.readFileSync(
      path.join(root, "related-crates/style/fixtures/tailwind-v43-official-candidate-inventory.json"),
      "utf8",
    ),
  );
  const byClass = new Map(officialMatrix.classes.map((entry) => [entry.className, entry]));
  const classes = [
    {
      ...byClass.get("block"),
      ownerLane: 2,
      ownerLaneName: "Logical utilities and block/inline property aliases",
    },
    byClass.get("[@unknown_rule]:p-4"),
  ];
  const matrixPath = path.join(tempDir, "matrix.json");
  fs.writeFileSync(
    matrixPath,
    `${JSON.stringify(
      {
        ...officialMatrix,
        classes,
        officialFixtureTruth: {
          inventory: officialMatrix.officialSource.candidateInventory,
          sourceFileCount: inventory.sourceFileCount,
          candidateSourceFileCount: inventory.candidateSourceFileCount ?? inventory.sourceFileCount,
          candidateCount: inventory.candidateCount,
          fixtureCount: inventory.officialFixtureMatrix.fixtureCount,
          fixtureSourceFileCount: inventory.officialFixtureMatrix.fixtureSourceFileCount,
          snapshotOutputPolicy: "fingerprinted-not-vendored",
          fullTailwindParity: false,
        },
      },
      null,
      2,
    )}\n`,
  );
  return matrixPath;
}

function makeSingleClassMatrix(tempDir, className) {
  const officialMatrix = JSON.parse(fs.readFileSync(path.join(root, matrix), "utf8"));
  const inventory = JSON.parse(
    fs.readFileSync(
      path.join(root, "related-crates/style/fixtures/tailwind-v43-official-candidate-inventory.json"),
      "utf8",
    ),
  );
  const entry = officialMatrix.classes.find((candidate) => candidate.className === className);
  assert.ok(entry, `expected fixture matrix entry for ${className}`);
  const matrixPath = path.join(tempDir, "single-class-matrix.json");
  fs.writeFileSync(
    matrixPath,
    `${JSON.stringify(
      {
        ...officialMatrix,
        classes: [entry],
        officialFixtureTruth: {
          inventory: officialMatrix.officialSource.candidateInventory,
          sourceFileCount: inventory.sourceFileCount,
          candidateSourceFileCount: inventory.candidateSourceFileCount ?? inventory.sourceFileCount,
          candidateCount: inventory.candidateCount,
          fixtureCount: inventory.officialFixtureMatrix.fixtureCount,
          fixtureSourceFileCount: inventory.officialFixtureMatrix.fixtureSourceFileCount,
          snapshotOutputPolicy: "fingerprinted-not-vendored",
          fullTailwindParity: false,
        },
      },
      null,
      2,
    )}\n`,
  );
  return matrixPath;
}

function makeClassificationSummaryMatrix(tempDir) {
  const officialMatrix = JSON.parse(fs.readFileSync(path.join(root, matrix), "utf8"));
  const inventory = JSON.parse(
    fs.readFileSync(
      path.join(root, "related-crates/style/fixtures/tailwind-v43-official-candidate-inventory.json"),
      "utf8",
    ),
  );
  const byClass = new Map(officialMatrix.classes.map((entry) => [entry.className, entry]));
  const classes = [
    byClass.get("block"),
    byClass.get("text-shadow-lg/20"),
    byClass.get("[@unknown_rule]:p-4"),
  ];
  const matrixPath = path.join(tempDir, "classification-matrix.json");
  fs.writeFileSync(
    matrixPath,
    `${JSON.stringify(
      {
        ...officialMatrix,
        classes,
        officialFixtureTruth: {
          inventory: officialMatrix.officialSource.candidateInventory,
          sourceFileCount: inventory.sourceFileCount,
          candidateSourceFileCount: inventory.candidateSourceFileCount ?? inventory.sourceFileCount,
          candidateCount: inventory.candidateCount,
          fixtureCount: inventory.officialFixtureMatrix.fixtureCount,
          fixtureSourceFileCount: inventory.officialFixtureMatrix.fixtureSourceFileCount,
          snapshotOutputPolicy: "fingerprinted-not-vendored",
          fullTailwindParity: false,
        },
      },
      null,
      2,
    )}\n`,
  );
  return matrixPath;
}

function makeDuplicateClassMatrix(tempDir) {
  const officialMatrix = JSON.parse(fs.readFileSync(path.join(root, matrix), "utf8"));
  const inventory = JSON.parse(
    fs.readFileSync(
      path.join(root, "related-crates/style/fixtures/tailwind-v43-official-candidate-inventory.json"),
      "utf8",
    ),
  );
  const block = officialMatrix.classes.find((entry) => entry.className === "block");
  const matrixPath = path.join(tempDir, "duplicate-class-matrix.json");
  fs.writeFileSync(
    matrixPath,
    `${JSON.stringify(
      {
        ...officialMatrix,
        classes: [block, { ...block }],
        officialFixtureTruth: {
          inventory: officialMatrix.officialSource.candidateInventory,
          sourceFileCount: inventory.sourceFileCount,
          candidateSourceFileCount: inventory.candidateSourceFileCount ?? inventory.sourceFileCount,
          candidateCount: inventory.candidateCount,
          fixtureCount: inventory.officialFixtureMatrix.fixtureCount,
          fixtureSourceFileCount: inventory.officialFixtureMatrix.fixtureSourceFileCount,
          snapshotOutputPolicy: "fingerprinted-not-vendored",
          fullTailwindParity: false,
        },
      },
      null,
      2,
    )}\n`,
  );
  return matrixPath;
}

test("live comparison rejects stale dx-style fixture binaries before reporting CSS gaps", () => {
  const { tempDir, binaryPath } = makeStaleFixtureBinary();
  try {
    const result = runRunner([
      "--matrix",
      matrix,
      "--validate-fixture-binary",
      binaryPath,
      "--json",
    ]);

    assert.notEqual(result.status, 0);
    const receipt = JSON.parse(result.stdout);
    assert.equal(receipt.schema, "dx.style.fixtureBinaryFreshnessReceipt");
    assert.equal(receipt.fresh, false);
    assert.equal(receipt.staleAllowed, false);
    assert.match(receipt.reason, /stale/i);
    assert.match(receipt.newestInputPath, /related-crates\/style\/src|tailwind-v43-official-fixture-matrix/);
  } finally {
    fs.rmSync(tempDir, { recursive: true, force: true });
  }
});

test("live comparison can explicitly mark stale fixture binaries as allowed for local debugging", () => {
  const { tempDir, binaryPath } = makeStaleFixtureBinary();
  try {
    const result = runRunner(
      ["--matrix", matrix, "--validate-fixture-binary", binaryPath, "--json"],
      { DX_STYLE_ALLOW_STALE_FIXTURE_BIN: "1" },
    );

    assert.equal(result.status, 0, result.stderr);
    const receipt = JSON.parse(result.stdout);
    assert.equal(receipt.schema, "dx.style.fixtureBinaryFreshnessReceipt");
    assert.equal(receipt.fresh, false);
    assert.equal(receipt.staleAllowed, true);
    assert.match(receipt.reason, /allowed/i);
  } finally {
    fs.rmSync(tempDir, { recursive: true, force: true });
  }
});

test("live comparison cargo execution uses an isolated target directory", () => {
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-style-live-cargo-target-guard-"));
  try {
    const matrixPath = makeSingleClassMatrix(tempDir, "block");
    const tailwindCssPath = path.join(tempDir, "tailwind.css");
    fs.writeFileSync(tailwindCssPath, ".block { display: block; }\n");
    const cargoPath = makeCargoTargetGuardCommand(tempDir);

    const result = runRunner(
      ["--matrix", matrixPath, "--tailwind-css", tailwindCssPath, "--json"],
      { DX_STYLE_CARGO_BIN: cargoPath },
    );

    assert.equal(result.status, 0, result.stderr || result.stdout);
    const receipt = JSON.parse(result.stdout);
    assert.equal(receipt.schema, "dx.style.liveTailwindComparisonReceipt");
    assert.equal(receipt.comparisonStatus, "compared");
    assert.equal(receipt.dxStyleCssSource, "cargo-run");
    assert.match(receipt.dxStyleCargoTargetDir, /dx-style-live-cargo-target/);
    assert.equal(receipt.failedCount, 0);
  } finally {
    fs.rmSync(tempDir, { recursive: true, force: true });
  }
});

test("live comparison receipt source exposes failure buckets and dx-style generation metadata", () => {
  const source = fs.readFileSync(runner, "utf8");

  for (const marker of [
    "dxStyleCssSource",
    "dxStyleFixtureBinaryFreshness",
    "failedClassNames",
    "failedUnsupportedByDxStyleClassNames",
    "failedMissingTailwindFragmentsClassNames",
    "failedMissingDxStyleFragmentsClassNames",
    "failureLaneBuckets",
    "failedClassHandoffs",
    "tailwindCssSource",
    "--tailwind-css",
    '@import "tailwindcss" source(none);',
    '@source inline("${classList}");',
    "DX_STYLE_ALLOW_STALE_FIXTURE_BIN",
    "DX_STYLE_CARGO_BIN",
    "DX_STYLE_CARGO_TARGET_DIR",
    "dxStyleCargoTargetDir",
    "DX_STYLE_CARGO_RUN_TIMEOUT_MS",
    "DX_STYLE_FIXTURE_BIN_TIMEOUT_MS",
    "dx.style.liveTailwindComparisonBlockedReceipt",
    "comparisonStatus",
    "blockerStage",
    "nextAction",
    "blockedRunPolicy",
    "officialCandidateInventoryCoverage",
    "inputFingerprints",
    "fixtureMatrixComparisonSha256",
    "classificationSummary",
    "governedCompatibilityPercent",
    "comparisonResultSummary",
    "byComparisonMode",
    "evidenceQuality",
    "canonicalLiveComparison",
    "nonCanonicalReasons",
    "tailwind-cli-not-executed",
    "matrixIntegrity",
    "duplicateClassNames",
  ]) {
    assert.ok(source.includes(marker), `runner source should include ${marker}`);
  }
});

test("live comparison rejects duplicate matrix class entries before reporting parity counts", () => {
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-style-live-duplicate-matrix-"));
  try {
    const matrixPath = makeDuplicateClassMatrix(tempDir);
    const tailwindCssPath = makeTailwindCssFile(tempDir);
    const binaryPath = makeFixtureBinaryWithReceipt(tempDir, {
      classes: [{ class_name: "block", supported: true, generated_css: ".block{display:block}" }],
    });

    const result = runRunner(["--matrix", matrixPath, "--tailwind-css", tailwindCssPath, "--json"], {
      DX_STYLE_FIXTURE_CSS_BIN: binaryPath,
    });

    assert.notEqual(result.status, 0);
    const receipt = JSON.parse(result.stdout);
    assert.equal(receipt.schema, "dx.style.liveTailwindComparisonBlockedReceipt");
    assert.equal(receipt.blockerStage, "fixture-matrix-integrity");
    assert.match(receipt.blockerMessage, /duplicate/i);
    assert.equal(receipt.matrixIntegrity?.valid, false);
    assert.deepEqual(receipt.matrixIntegrity?.duplicateClassNames, ["block"]);
    assert.equal(receipt.comparedClassCount, 0);
    assert.deepEqual(receipt.failedClassNames, []);
    assert.equal(receipt.fullTailwindParity, false);
  } finally {
    fs.rmSync(tempDir, { recursive: true, force: true });
  }
});

test("live comparison receipt groups failures by owning worker lane for handoff", () => {
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-style-live-lane-receipt-"));
  try {
    const matrixPath = makeMinimalMatrix(tempDir);
    const tailwindCssPath = makeTailwindCssFile(tempDir);
    const binaryPath = makeFixtureBinaryWithReceipt(tempDir, {
      classes: [
        { class_name: "block", supported: false, generated_css: "" },
        { class_name: "[@unknown_rule]:p-4", supported: false, generated_css: "" },
      ],
    });

    const result = runRunner(["--matrix", matrixPath, "--tailwind-css", tailwindCssPath, "--json"], {
      DX_STYLE_FIXTURE_CSS_BIN: binaryPath,
    });

    assert.notEqual(result.status, 0);
    const receipt = JSON.parse(result.stdout);
    assert.equal(receipt.schema, "dx.style.liveTailwindComparisonReceipt");
    assert.equal(receipt.classCount, 2);
    assert.equal(receipt.failedCount, 2);
    assert.deepEqual(receipt.failedClassNames.sort(), ["[@unknown_rule]:p-4", "block"].sort());
    assert.ok(Array.isArray(receipt.failedClassHandoffs));
    assert.equal(receipt.failedClassHandoffs.length, 2);

    const handoffs = new Map(receipt.failedClassHandoffs.map((entry) => [entry.className, entry]));
    assert.equal(handoffs.get("block").ownerLaneNumber, 2);
    assert.equal(
      handoffs.get("block").ownerLaneName,
      "Logical utilities and block/inline property aliases",
    );
    assert.deepEqual(handoffs.get("block").failureReasons, ["unsupported-by-dx-style"]);
    assert.equal(handoffs.get("[@unknown_rule]:p-4").ownerLaneNumber, 5);
    assert.equal(
      handoffs.get("[@unknown_rule]:p-4").ownerLaneName,
      "Arbitrary variants and at-rule selector grammar",
    );

    const buckets = new Map(
      receipt.failureLaneBuckets.map((bucket) => [bucket.ownerLaneNumber, bucket]),
    );
    assert.deepEqual(buckets.get(2).failedClassNames, ["block"]);
    assert.deepEqual(buckets.get(5).failedClassNames, ["[@unknown_rule]:p-4"]);
    assert.equal(buckets.get(2).failureReasonCounts["unsupported-by-dx-style"], 1);
    assert.equal(buckets.get(5).failureReasonCounts["unsupported-by-dx-style"], 1);
  } finally {
    fs.rmSync(tempDir, { recursive: true, force: true });
  }
});

test("live comparison receipt reports official candidate inventory coverage separately from CSS gaps", () => {
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-style-live-inventory-coverage-"));
  try {
    const matrixPath = makeMinimalMatrix(tempDir);
    const tailwindCssPath = makeTailwindCssFile(tempDir);
    const binaryPath = makeFixtureBinaryWithReceipt(tempDir, {
      classes: [
        { class_name: "block", supported: false, generated_css: "" },
        { class_name: "[@unknown_rule]:p-4", supported: false, generated_css: "" },
      ],
    });

    const result = runRunner(["--matrix", matrixPath, "--tailwind-css", tailwindCssPath, "--json"], {
      DX_STYLE_FIXTURE_CSS_BIN: binaryPath,
    });

    assert.notEqual(result.status, 0);
    const receipt = JSON.parse(result.stdout);
    const coverage = receipt.officialCandidateInventoryCoverage;
    assert.equal(coverage?.matrixClassCount, 2);
    assert.equal(coverage?.inventoryCandidateCount, receipt.officialCandidateCount);
    assert.equal(coverage?.duplicateClassCount, 0);
    assert.ok(
      coverage?.missingClassNames?.includes("block"),
      "coverage should identify matrix classes absent from the official source-test candidate inventory",
    );
    assert.ok(
      coverage?.missingClassEntries?.some((entry) => entry.className === "block" && entry.ownerLaneNumber === 2),
      "coverage entries should keep lane ownership for downstream handoff",
    );
    assert.equal(
      receipt.failedClassNames.includes("block"),
      true,
      "CSS comparison failure remains classified separately from inventory coverage",
    );
  } finally {
    fs.rmSync(tempDir, { recursive: true, force: true });
  }
});

test("live comparison receipts include stable input fingerprints for reproducible handoff", () => {
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-style-live-input-fingerprints-"));
  try {
    const matrixPath = makeMinimalMatrix(tempDir);
    const tailwindCssPath = makeTailwindCssFile(tempDir);
    const missingTailwindCssPath = path.join(tempDir, "missing-tailwind.css");
    const binaryPath = makeFixtureBinaryWithReceipt(tempDir, {
      classes: [
        { class_name: "block", supported: true, generated_css: ".block{display:block}" },
        { class_name: "[@unknown_rule]:p-4", supported: false, generated_css: "" },
      ],
    });

    const result = runRunner(["--matrix", matrixPath, "--tailwind-css", tailwindCssPath, "--json"], {
      DX_STYLE_FIXTURE_CSS_BIN: binaryPath,
    });
    assert.notEqual(result.status, 0);
    const receipt = JSON.parse(result.stdout);
    const fingerprints = receipt.inputFingerprints;
    for (const key of [
      "fixtureMatrixClassesSha256",
      "fixtureMatrixComparisonSha256",
      "officialCandidateInventorySha256",
      "officialFixtureSnapshotsSha256",
    ]) {
      assert.match(fingerprints?.[key] ?? "", sha256Pattern, `${key} should be a stable sha256`);
    }
    assert.notEqual(
      fingerprints.fixtureMatrixClassesSha256,
      fingerprints.fixtureMatrixComparisonSha256,
      "class-list and comparison-contract fingerprints should describe different inputs",
    );

    const blocked = runRunner(["--matrix", matrixPath, "--tailwind-css", missingTailwindCssPath, "--json"], {
      DX_STYLE_FIXTURE_CSS_BIN: binaryPath,
    });
    assert.notEqual(blocked.status, 0);
    const blockedReceipt = JSON.parse(blocked.stdout);
    assert.equal(blockedReceipt.schema, "dx.style.liveTailwindComparisonBlockedReceipt");
    assert.deepEqual(
      blockedReceipt.inputFingerprints,
      fingerprints,
      "blocked receipts should preserve the same input fingerprints before comparison starts",
    );
  } finally {
    fs.rmSync(tempDir, { recursive: true, force: true });
  }
});

test("live comparison receipts summarize matrix classifications before comparison succeeds", () => {
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-style-live-classification-summary-"));
  try {
    const matrixPath = makeClassificationSummaryMatrix(tempDir);
    const missingTailwindCssPath = path.join(tempDir, "missing-tailwind.css");
    const binaryPath = makeFixtureBinaryWithReceipt(tempDir, {
      classes: [
        { class_name: "block", supported: true, generated_css: ".block{display:block}" },
        {
          class_name: "text-shadow-lg/20",
          supported: true,
          generated_css:
            ".text-shadow-lg\\/20{--tw-text-shadow-alpha:20%;text-shadow:0px 1px 2px var(--tw-text-shadow-color, oklab(from rgb(0 0 0 / 0.1) l a b / 20%))}",
        },
        { class_name: "[@unknown_rule]:p-4", supported: true, generated_css: "@unknown rule{.x{padding:1rem}}" },
      ],
    });

    const result = runRunner(["--matrix", matrixPath, "--tailwind-css", missingTailwindCssPath, "--json"], {
      DX_STYLE_FIXTURE_CSS_BIN: binaryPath,
    });

    assert.notEqual(result.status, 0);
    const receipt = JSON.parse(result.stdout);
    assert.equal(receipt.schema, "dx.style.liveTailwindComparisonBlockedReceipt");
    const summary = receipt.classificationSummary;
    assert.equal(summary?.classCount, 3);
    assert.equal(summary.exactFragmentMatchCount, 3);
    assert.equal(summary.knownDifferentCount, 0);
    assert.equal(summary.tailwindOnlyGapCount, 0);
    assert.equal(summary.exactOutputParityPercent, 100);
    assert.equal(summary.governedCompatibilityPercent, 100);
    assert.equal(summary.fullTailwindParity, false);
    assert.match(summary.policy, /classification/i);

    const laneBuckets = new Map(summary.byOwnerLane.map((bucket) => [bucket.ownerLaneNumber, bucket]));
    assert.deepEqual(laneBuckets.get(2).comparisonModeCounts, { "exact-fragment-match": 1 });
    assert.deepEqual(laneBuckets.get(null).comparisonModeCounts, { "exact-fragment-match": 1 });
    assert.deepEqual(laneBuckets.get(5).comparisonModeCounts, { "exact-fragment-match": 1 });
  } finally {
    fs.rmSync(tempDir, { recursive: true, force: true });
  }
});

test("live comparison receipts summarize comparison results by mode and owner lane", () => {
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-style-live-result-summary-"));
  try {
    const matrixPath = makeClassificationSummaryMatrix(tempDir);
    const tailwindCssPath = path.join(tempDir, "tailwind-result-summary.css");
    fs.writeFileSync(
      tailwindCssPath,
      [
        ".block { display: block; }",
        ".text-shadow-lg\\/20 { --tw-text-shadow-alpha: 20%; text-shadow: 0px 1px 2px var(--tw-text-shadow-color, oklab(from rgb(0 0 0 / 0.1) l a b / 20%)); }",
        "@unknown rule { .\\[\\@unknown_rule\\]\\:p-4 { padding: 1rem; } }",
      ].join("\n"),
    );
    const binaryPath = makeFixtureBinaryWithReceipt(tempDir, {
      classes: [
        { class_name: "block", supported: true, generated_css: ".block{display:block}" },
        {
          class_name: "text-shadow-lg/20",
          supported: true,
          generated_css:
            ".text-shadow-lg\\/20{--tw-text-shadow-alpha:20%;text-shadow:0px 1px 2px var(--tw-text-shadow-color, oklab(from rgb(0 0 0 / 0.1) l a b / 20%))}",
        },
        { class_name: "[@unknown_rule]:p-4", supported: false, generated_css: "" },
      ],
    });

    const result = runRunner(["--matrix", matrixPath, "--tailwind-css", tailwindCssPath, "--json"], {
      DX_STYLE_FIXTURE_CSS_BIN: binaryPath,
    });

    assert.notEqual(result.status, 0);
    const receipt = JSON.parse(result.stdout);
    const summary = receipt.comparisonResultSummary;
    assert.equal(summary?.classCount, 3);
    assert.equal(summary.passedCount, 2);
    assert.equal(summary.failedCount, 1);
    assert.equal(summary.passPercent, 66.67);

    const byMode = new Map(summary.byComparisonMode.map((bucket) => [bucket.comparisonMode, bucket]));
    assert.deepEqual(byMode.get("exact-fragment-match"), {
      comparisonMode: "exact-fragment-match",
      classCount: 3,
      passedCount: 2,
      failedCount: 1,
      passPercent: 66.67,
      failedClassNames: ["[@unknown_rule]:p-4"],
    });
    assert.equal(byMode.has("known-different"), false);

    const byLane = new Map(summary.byOwnerLane.map((bucket) => [bucket.ownerLaneNumber, bucket]));
    assert.deepEqual(byLane.get(2).failedClassNames, []);
    assert.deepEqual(byLane.get(null).failedClassNames, []);
    assert.deepEqual(byLane.get(5).failedClassNames, ["[@unknown_rule]:p-4"]);
    assert.equal(byLane.get(5).failedCount, 1);
  } finally {
    fs.rmSync(tempDir, { recursive: true, force: true });
  }
});

test("live comparison receipts flag non-canonical local fixture evidence explicitly", () => {
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-style-live-evidence-quality-"));
  try {
    const matrixPath = makeMinimalMatrix(tempDir);
    const tailwindCssPath = makeTailwindCssFile(tempDir);
    const missingTailwindCssPath = path.join(tempDir, "missing-tailwind.css");
    const binaryPath = makeFixtureBinaryWithReceipt(tempDir, {
      classes: [
        { class_name: "block", supported: true, generated_css: ".block{display:block}" },
        { class_name: "[@unknown_rule]:p-4", supported: false, generated_css: "" },
      ],
    });

    const result = runRunner(["--matrix", matrixPath, "--tailwind-css", tailwindCssPath, "--json"], {
      DX_STYLE_FIXTURE_CSS_BIN: binaryPath,
    });

    assert.notEqual(result.status, 0);
    const receipt = JSON.parse(result.stdout);
    assert.equal(receipt.schema, "dx.style.liveTailwindComparisonReceipt");
    assert.equal(receipt.evidenceQuality?.canonicalLiveComparison, false);
    assert.equal(receipt.evidenceQuality?.comparisonStatus, "compared");
    assert.equal(receipt.evidenceQuality?.liveTailwindExecution, false);
    assert.equal(receipt.evidenceQuality?.tailwindCssSource, "provided-css-file");
    assert.equal(receipt.evidenceQuality?.dxStyleCssSource, "provided-binary");
    assert.equal(receipt.evidenceQuality?.freshDxStyleOutput, true);
    assert.deepEqual(receipt.evidenceQuality?.nonCanonicalReasons, [
      "tailwind-css-fixture-used",
    ]);
    assert.match(receipt.evidenceQuality?.policy ?? "", /canonical/i);

    const stale = makeStaleFixtureBinary();
    try {
      const staleReceipt = runRunner(
        ["--matrix", matrixPath, "--tailwind-css", tailwindCssPath, "--json"],
        {
          DX_STYLE_FIXTURE_CSS_BIN: stale.binaryPath,
          DX_STYLE_ALLOW_STALE_FIXTURE_BIN: "1",
        },
      );

      assert.notEqual(staleReceipt.status, 0);
      const staleReceiptJson = JSON.parse(staleReceipt.stdout);
      assert.equal(staleReceiptJson.evidenceQuality?.canonicalLiveComparison, false);
      assert.equal(staleReceiptJson.evidenceQuality?.staleFixtureBinaryAllowed, true);
      assert.ok(
        staleReceiptJson.evidenceQuality?.nonCanonicalReasons?.includes(
          "stale-dx-style-fixture-binary-allowed",
        ),
      );
    } finally {
      fs.rmSync(stale.tempDir, { recursive: true, force: true });
    }

    const blocked = runRunner(["--matrix", matrixPath, "--tailwind-css", missingTailwindCssPath, "--json"], {
      DX_STYLE_FIXTURE_CSS_BIN: binaryPath,
    });
    assert.notEqual(blocked.status, 0);
    const blockedReceipt = JSON.parse(blocked.stdout);
    assert.equal(blockedReceipt.schema, "dx.style.liveTailwindComparisonBlockedReceipt");
    assert.equal(blockedReceipt.evidenceQuality?.canonicalLiveComparison, false);
    assert.equal(blockedReceipt.evidenceQuality?.comparisonStatus, "blocked");
    assert.ok(blockedReceipt.nextAction);
    assert.ok(
      blockedReceipt.evidenceQuality?.nonCanonicalReasons?.includes(
        "comparison-blocked:tailwind-css-input",
      ),
    );
  } finally {
    fs.rmSync(tempDir, { recursive: true, force: true });
  }
});

test("live comparison writes a receipt artifact before exiting nonzero on failures", () => {
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-style-live-receipt-file-"));
  try {
    const matrixPath = makeMinimalMatrix(tempDir);
    const tailwindCssPath = makeTailwindCssFile(tempDir);
    const receiptPath = path.join(tempDir, "artifacts", "live-receipt.json");
    const binaryPath = makeFixtureBinaryWithReceipt(tempDir, {
      classes: [
        { class_name: "block", supported: true, generated_css: ".block{display:block}" },
        { class_name: "[@unknown_rule]:p-4", supported: false, generated_css: "" },
      ],
    });

    const result = runRunner(
      [
        "--matrix",
        matrixPath,
        "--tailwind-css",
        tailwindCssPath,
        "--receipt",
        receiptPath,
        "--json",
      ],
      { DX_STYLE_FIXTURE_CSS_BIN: binaryPath },
    );

    assert.notEqual(result.status, 0);
    assert.ok(fs.existsSync(receiptPath), "failing live comparison should still write receipt artifact");
    const stdoutReceipt = JSON.parse(result.stdout);
    const fileReceipt = JSON.parse(fs.readFileSync(receiptPath, "utf8"));
    assert.deepEqual(fileReceipt, stdoutReceipt);
    assert.equal(fileReceipt.failedCount, 1);
    assert.deepEqual(fileReceipt.failedClassNames, ["[@unknown_rule]:p-4"]);
    assert.equal(fileReceipt.failureLaneBuckets[0].ownerLaneNumber, 5);
    assert.equal(fileReceipt.failedClassHandoffs[0].ownerLaneNumber, 5);
  } finally {
    fs.rmSync(tempDir, { recursive: true, force: true });
  }
});

test("live comparison writes a blocked receipt artifact when Tailwind input is unavailable", () => {
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-style-live-blocked-receipt-"));
  try {
    const matrixPath = makeMinimalMatrix(tempDir);
    const missingTailwindCssPath = path.join(tempDir, "missing-tailwind.css");
    const receiptPath = path.join(tempDir, "artifacts", "blocked-receipt.json");
    const binaryPath = makeFixtureBinaryWithReceipt(tempDir, {
      classes: [
        { class_name: "block", supported: true, generated_css: ".block{display:block}" },
        { class_name: "[@unknown_rule]:p-4", supported: false, generated_css: "" },
      ],
    });

    const result = runRunner(
      [
        "--matrix",
        matrixPath,
        "--tailwind-css",
        missingTailwindCssPath,
        "--receipt",
        receiptPath,
        "--json",
      ],
      { DX_STYLE_FIXTURE_CSS_BIN: binaryPath },
    );

    assert.notEqual(result.status, 0);
    assert.ok(fs.existsSync(receiptPath), "blocked comparison should still write a receipt artifact");
    const stdoutReceipt = JSON.parse(result.stdout);
    const fileReceipt = JSON.parse(fs.readFileSync(receiptPath, "utf8"));
    assert.deepEqual(fileReceipt, stdoutReceipt);
    assert.equal(fileReceipt.schema, "dx.style.liveTailwindComparisonBlockedReceipt");
    assert.equal(fileReceipt.comparisonStatus, "blocked");
    assert.equal(fileReceipt.blockerStage, "tailwind-css-input");
    assert.match(fileReceipt.blockerMessage, /missing-tailwind\.css/);
    assert.equal(fileReceipt.tailwindCssSource, "provided-css-file");
    assert.equal(fileReceipt.tailwindCssPath, missingTailwindCssPath);
    assert.equal(fileReceipt.liveTailwindExecution, false);
    assert.equal(fileReceipt.tailwindRuntimeDependency, false);
    assert.equal(fileReceipt.packageManifestMutation, false);
    assert.equal(fileReceipt.fullTailwindParity, false);
  } finally {
    fs.rmSync(tempDir, { recursive: true, force: true });
  }
});

test("live comparison writes a blocked receipt artifact when dx-style generation fails", () => {
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-style-live-dx-blocked-receipt-"));
  try {
    const matrixPath = makeMinimalMatrix(tempDir);
    const receiptPath = path.join(tempDir, "artifacts", "dx-blocked-receipt.json");
    const tailwindCssPath = makeTailwindCssFile(tempDir);
    const binaryPath = makeFailingCommand(tempDir, "fixture", "dx-style fixture failed");

    const result = runRunner(
      [
        "--matrix",
        matrixPath,
        "--tailwind-css",
        tailwindCssPath,
        "--receipt",
        receiptPath,
        "--json",
      ],
      { DX_STYLE_FIXTURE_CSS_BIN: binaryPath },
    );

    assert.notEqual(result.status, 0);
    assert.ok(fs.existsSync(receiptPath), "dx-style generation failures should still write a receipt");
    const receipt = JSON.parse(fs.readFileSync(receiptPath, "utf8"));
    assert.equal(receipt.schema, "dx.style.liveTailwindComparisonBlockedReceipt");
    assert.equal(receipt.comparisonStatus, "blocked");
    assert.equal(receipt.blockerStage, "dx-style-build");
    assert.match(receipt.blockerMessage, /fixture/);
    assert.equal(receipt.dxStyleCssSource, "provided-binary");
    assert.equal(receipt.dxStyleFixtureBinaryFreshness?.fresh, true);
    assert.equal(receipt.tailwindCssSource, "provided-css-file");
    assert.equal(receipt.liveTailwindExecution, false);
    assert.equal(receipt.fullTailwindParity, false);
  } finally {
    fs.rmSync(tempDir, { recursive: true, force: true });
  }
});

test("live comparison writes a blocked receipt artifact when cargo fixture generation times out", () => {
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-style-live-cargo-timeout-"));
  try {
    const matrixPath = makeMinimalMatrix(tempDir);
    const receiptPath = path.join(tempDir, "artifacts", "cargo-timeout-receipt.json");
    const tailwindCssPath = makeTailwindCssFile(tempDir);
    const cargoPath = makeSleepingCommand(tempDir, "cargo-slow", 5000);

    const result = runRunner(
      [
        "--matrix",
        matrixPath,
        "--tailwind-css",
        tailwindCssPath,
        "--receipt",
        receiptPath,
        "--json",
      ],
      {
        DX_STYLE_CARGO_BIN: cargoPath,
        DX_STYLE_CARGO_RUN_TIMEOUT_MS: "50",
      },
    );

    assert.notEqual(result.status, 0);
    assert.ok(fs.existsSync(receiptPath), "cargo timeouts should still write a blocked receipt");
    const stdoutReceipt = JSON.parse(result.stdout);
    const receipt = JSON.parse(fs.readFileSync(receiptPath, "utf8"));
    assert.deepEqual(receipt, stdoutReceipt);
    assert.equal(receipt.schema, "dx.style.liveTailwindComparisonBlockedReceipt");
    assert.equal(receipt.comparisonStatus, "blocked");
    assert.equal(receipt.blockerStage, "dx-style-build");
    assert.match(receipt.blockerMessage, /timed out|timeout|ETIMEDOUT/i);
    assert.match(receipt.nextAction, /Cargo\/Rust build contention/);
    assert.equal(receipt.dxStyleCssSource, "cargo-run");
    assert.equal(receipt.tailwindCssSource, "provided-css-file");
    assert.equal(receipt.liveTailwindExecution, false);
    assert.equal(receipt.comparedClassCount, 0);
    assert.deepEqual(receipt.failedClassNames, []);
    assert.equal(receipt.fullTailwindParity, false);
  } finally {
    fs.rmSync(tempDir, { recursive: true, force: true });
  }
});

test("live comparison writes a blocked receipt artifact when Tailwind CLI generation fails", () => {
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-style-live-tailwind-blocked-receipt-"));
  try {
    const matrixPath = makeMinimalMatrix(tempDir);
    const receiptPath = path.join(tempDir, "artifacts", "tailwind-blocked-receipt.json");
    const binaryPath = makeFixtureBinaryWithReceipt(tempDir, {
      classes: [
        { class_name: "block", supported: true, generated_css: ".block{display:block}" },
        { class_name: "[@unknown_rule]:p-4", supported: false, generated_css: "" },
      ],
    });
    makeFailingCommand(tempDir, process.platform === "win32" ? "npm" : "npm", "tailwind npm install failed");
    const pathKey = process.platform === "win32" ? "Path" : "PATH";
    const existingPath = process.env[pathKey] || process.env.PATH || "";

    const result = runRunner(
      ["--matrix", matrixPath, "--receipt", receiptPath, "--json"],
      {
        DX_STYLE_FIXTURE_CSS_BIN: binaryPath,
        [pathKey]: `${tempDir}${path.delimiter}${existingPath}`,
      },
    );

    assert.notEqual(result.status, 0);
    assert.ok(fs.existsSync(receiptPath), "Tailwind CLI failures should still write a receipt");
    const receipt = JSON.parse(fs.readFileSync(receiptPath, "utf8"));
    assert.equal(receipt.schema, "dx.style.liveTailwindComparisonBlockedReceipt");
    assert.equal(receipt.comparisonStatus, "blocked");
    assert.equal(receipt.blockerStage, "tailwind-build");
    assert.match(receipt.blockerMessage, /npm/);
    assert.equal(receipt.tailwindCssSource, "tailwind-cli");
    assert.equal(receipt.liveTailwindExecution, true);
    assert.equal(receipt.dxStyleCssSource, "provided-binary");
    assert.equal(receipt.fullTailwindParity, false);
  } finally {
    fs.rmSync(tempDir, { recursive: true, force: true });
  }
});
