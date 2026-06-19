#!/usr/bin/env node
const { execFileSync } = require("node:child_process");
const crypto = require("node:crypto");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");

const root = path.resolve(__dirname, "../..");
const laneNames = new Map([
  [1, "v4.2/v4.3 palettes and color utilities"],
  [2, "Logical utilities and block/inline property aliases"],
  [3, "Media/state/pseudo variants"],
  [4, "Container query variants"],
  [5, "Arbitrary variants and at-rule selector grammar"],
  [6, "CSS directives"],
  [7, "Fixture matrix, live comparison runner, receipts, candidate inventory"],
  [8, "Starter/no-runtime integration and unsupported diagnostics"],
]);

function parseArgs(argv) {
  const args = {
    matrix: "related-crates/style/fixtures/tailwind-v43-official-fixture-matrix.json",
    json: false,
    keepTemp: false,
    receipt: null,
    tailwindCss: null,
    validateFixtureBinary: null,
  };

  for (let index = 2; index < argv.length; index += 1) {
    const arg = argv[index];
    if (arg === "--matrix") {
      args.matrix = argv[++index];
    } else if (arg === "--json") {
      args.json = true;
    } else if (arg === "--keep-temp") {
      args.keepTemp = true;
    } else if (arg === "--receipt") {
      args.receipt = argv[++index];
    } else if (arg === "--tailwind-css") {
      args.tailwindCss = argv[++index];
    } else if (arg === "--validate-fixture-binary") {
      args.validateFixtureBinary = argv[++index];
    } else {
      throw new Error(`unknown argument: ${arg}`);
    }
  }

  return args;
}

function readJson(filePath) {
  return JSON.parse(fs.readFileSync(filePath, "utf8"));
}

function readOfficialInventory(matrix) {
  const inventoryPath = matrix.officialSource?.candidateInventory;
  if (!inventoryPath) {
    throw new Error("fixture matrix must reference an official Tailwind candidate inventory");
  }

  const absoluteInventoryPath = path.resolve(root, inventoryPath);
  const inventory = readJson(absoluteInventoryPath);
  if (inventory.schema !== "dx.style.tailwindOfficialCandidateInventory") {
    throw new Error(`invalid official candidate inventory schema: ${inventory.schema}`);
  }
  if (inventory.tailwindPackage?.version !== matrix.tailwindPackage.version) {
    throw new Error(
      `inventory Tailwind version ${inventory.tailwindPackage?.version} does not match matrix ${matrix.tailwindPackage.version}`,
    );
  }
  if (inventory.officialSource?.commit !== matrix.officialSource.commit) {
    throw new Error(
      `inventory commit ${inventory.officialSource?.commit} does not match matrix ${matrix.officialSource.commit}`,
    );
  }
  if (inventory.tailwindRuntimeDependency !== false) {
    throw new Error("official candidate inventory must not require Tailwind at runtime");
  }
  if (inventory.officialFixtureMatrix?.schema !== "dx.style.tailwindOfficialFixtureMatrix") {
    throw new Error("official candidate inventory must include the official fixture matrix index");
  }
  const truth = matrix.officialFixtureTruth;
  if (!truth) {
    throw new Error("fixture matrix must carry synchronized officialFixtureTruth metadata");
  }
  const expectedTruth = {
    inventory: inventoryPath,
    sourceFileCount: inventory.sourceFileCount,
    candidateSourceFileCount:
      inventory.candidateSourceFileCount ?? inventory.sourceFileCount,
    candidateCount: inventory.candidateCount,
    fixtureCount: inventory.officialFixtureMatrix.fixtureCount,
    fixtureSourceFileCount: inventory.officialFixtureMatrix.fixtureSourceFileCount,
    snapshotOutputPolicy: "fingerprinted-not-vendored",
    fullTailwindParity: false,
  };
  for (const [key, expected] of Object.entries(expectedTruth)) {
    if (truth[key] !== expected) {
      throw new Error(
        `fixture matrix officialFixtureTruth.${key} drifted: expected ${expected}, got ${truth[key]}`,
      );
    }
  }
  return {
    path: inventoryPath,
    inventory,
  };
}

function run(command, args, options = {}) {
  const runsThroughCmd = process.platform === "win32" && command.endsWith(".cmd");
  const finalCommand = runsThroughCmd ? process.env.ComSpec || "cmd.exe" : command;
  const finalArgs = runsThroughCmd ? ["/d", "/s", "/c", command, ...args] : args;

  return execFileSync(finalCommand, finalArgs, {
    cwd: options.cwd ?? root,
    encoding: "utf8",
    env: options.env ? { ...process.env, ...options.env } : process.env,
    maxBuffer: 20 * 1024 * 1024,
    stdio: ["ignore", "pipe", "pipe"],
    timeout: options.timeout ?? 180_000,
  });
}

function stableJson(value) {
  if (Array.isArray(value)) {
    return value.map((entry) => stableJson(entry));
  }
  if (value && typeof value === "object") {
    return Object.fromEntries(
      Object.keys(value)
        .sort()
        .map((key) => [key, stableJson(value[key])]),
    );
  }
  return value;
}

function sha256Json(value) {
  return crypto
    .createHash("sha256")
    .update(JSON.stringify(stableJson(value)))
    .digest("hex");
}

function receiptInputFingerprints(matrix, officialInventory) {
  const comparisonEntries = matrix.classes.map((entry) => ({
    className: entry.className,
    area: entry.area,
    sourceIds: entry.sourceIds ?? [],
    comparisonMode: entry.comparisonMode,
    ownerLane: entry.ownerLane ?? null,
    ownerLaneName: entry.ownerLaneName ?? null,
    fullTailwindParity: entry.fullTailwindParity === true,
    tailwindRequiredFragments: entry.tailwindRequiredFragments ?? [],
    dxStyleRequiredFragments: entry.dxStyleRequiredFragments ?? [],
    differenceNote: entry.differenceNote ?? null,
    gapNote: entry.gapNote ?? null,
  }));

  return {
    fixtureMatrixClassesSha256: sha256Json(matrix.classes.map((entry) => entry.className)),
    fixtureMatrixComparisonSha256: sha256Json(comparisonEntries),
    officialCandidateInventorySha256: sha256Json(
      [...new Set(officialInventory.inventory.candidates ?? [])].sort(),
    ),
    officialFixtureSnapshotsSha256: sha256Json(
      officialInventory.inventory.officialFixtureMatrix?.entries ?? [],
    ),
  };
}

function fixtureMatrixIntegrity(matrix) {
  const seen = new Map();
  const duplicateClassNames = [];
  const duplicateClassEntries = [];

  for (const [index, entry] of matrix.classes.entries()) {
    if (!seen.has(entry.className)) {
      seen.set(entry.className, index);
      continue;
    }

    if (!duplicateClassNames.includes(entry.className)) {
      duplicateClassNames.push(entry.className);
    }
    duplicateClassEntries.push({
      className: entry.className,
      firstIndex: seen.get(entry.className),
      duplicateIndex: index,
      area: entry.area ?? null,
      comparisonMode: entry.comparisonMode ?? null,
    });
  }

  return {
    valid: duplicateClassNames.length === 0,
    classCount: matrix.classes.length,
    uniqueClassCount: seen.size,
    duplicateClassCount: duplicateClassNames.length,
    duplicateClassNames,
    duplicateClassEntries,
    policy:
      "The governed fixture matrix must keep one comparison entry per class name so exact, known-different, and Tailwind-only gap counts cannot be inflated by duplicate canaries.",
  };
}

function assertFixtureMatrixIntegrity(matrix) {
  const integrity = fixtureMatrixIntegrity(matrix);
  if (!integrity.valid) {
    const error = new Error(
      `fixture matrix integrity failed: duplicate class entries ${integrity.duplicateClassNames.join(", ")}`,
    );
    error.matrixIntegrity = integrity;
    throw markComparisonBlocker(error, "fixture-matrix-integrity");
  }
  return integrity;
}

function normalizeCss(css) {
  return String(css)
    .replace(/\r\n/g, "\n")
    .replace(/\s+/g, " ")
    .replace(/\s*([{}:;,>~+()])\s*/g, "$1")
    .trim();
}

function includesFragments(css, fragments) {
  const normalizedCss = normalizeCss(css);
  return fragments.every((fragment) => normalizedCss.includes(normalizeCss(fragment)));
}

function ownerLaneForEntry(entry) {
  const explicitLane = Number(entry.ownerLane ?? entry.ownerLaneNumber);
  if (Number.isInteger(explicitLane) && laneNames.has(explicitLane)) {
    return {
      ownerLaneNumber: explicitLane,
      ownerLaneName: entry.ownerLaneName || laneNames.get(explicitLane),
    };
  }

  if (entry.area === "colors") {
    return lane(1);
  }
  if (
    [
      "layout",
      "flexbox-grid",
      "spacing",
      "sizing",
      "logical-sizing",
      "logical-spacing",
      "logical-inset",
      "logical-scroll",
      "logical-borders",
    ].includes(entry.area)
  ) {
    return lane(2);
  }
  if (entry.area === "variants") {
    return lane(3);
  }
  if (entry.area === "container-query") {
    return lane(4);
  }
  if (entry.area === "arbitrary-variants") {
    return lane(5);
  }
  if (entry.area === "css-directives") {
    return lane(6);
  }
  if (entry.area === "starter-runtime") {
    return lane(8);
  }

  return {
    ownerLaneNumber: null,
    ownerLaneName: "unassigned core utility lane",
  };
}

function lane(ownerLaneNumber) {
  return {
    ownerLaneNumber,
    ownerLaneName: laneNames.get(ownerLaneNumber),
  };
}

function failureReasonsForEntry(entry) {
  if (entry.status !== "failed") {
    return [];
  }

  const reasons = [];
  if (!entry.tailwindFragmentsPresent) {
    reasons.push("missing-tailwind-fragments");
  }
  if (entry.comparisonMode === "tailwind-only-gap" && entry.dxStyleSupported) {
    reasons.push("tailwind-only-gap-reclassification-needed");
  } else if (!entry.dxStyleSupported) {
    reasons.push("unsupported-by-dx-style");
  } else if (!entry.dxStyleFragmentsPresent) {
    reasons.push("missing-dx-style-fragments");
  }
  if (reasons.length === 0) {
    reasons.push("comparison-mode-invariant-failed");
  }
  return reasons;
}

function incrementCount(target, key) {
  target[key] = (target[key] ?? 0) + 1;
}

function failedClassHandoffs(failedEntries) {
  return failedEntries.map((entry) => ({
    className: entry.className,
    area: entry.area,
    comparisonMode: entry.comparisonMode,
    ownerLaneNumber: entry.ownerLaneNumber,
    ownerLaneName: entry.ownerLaneName,
    failureReasons: entry.failureReasons,
    tailwindFragmentsPresent: entry.tailwindFragmentsPresent,
    dxStyleSupported: entry.dxStyleSupported,
    dxStyleFragmentsPresent: entry.dxStyleFragmentsPresent,
    differenceNote: entry.differenceNote,
    gapNote: entry.gapNote,
  }));
}

function failureLaneBuckets(failedEntries) {
  const buckets = new Map();
  for (const entry of failedEntries) {
    const key = `${entry.ownerLaneNumber ?? "unassigned"}:${entry.ownerLaneName}`;
    if (!buckets.has(key)) {
      buckets.set(key, {
        ownerLaneNumber: entry.ownerLaneNumber,
        ownerLaneName: entry.ownerLaneName,
        failedCount: 0,
        failedClassNames: [],
        failureReasonCounts: {},
        comparisonModeCounts: {},
        areaCounts: {},
      });
    }

    const bucket = buckets.get(key);
    bucket.failedCount += 1;
    bucket.failedClassNames.push(entry.className);
    incrementCount(bucket.comparisonModeCounts, entry.comparisonMode);
    incrementCount(bucket.areaCounts, entry.area);
    for (const reason of entry.failureReasons) {
      incrementCount(bucket.failureReasonCounts, reason);
    }
  }

  return [...buckets.values()].sort((left, right) => {
    const leftLane = left.ownerLaneNumber ?? Number.MAX_SAFE_INTEGER;
    const rightLane = right.ownerLaneNumber ?? Number.MAX_SAFE_INTEGER;
    return leftLane === rightLane
      ? left.ownerLaneName.localeCompare(right.ownerLaneName)
      : leftLane - rightLane;
    });
}

function comparisonResultSummary(entries) {
  const byComparisonMode = new Map();
  const byOwnerLane = new Map();
  let passedCount = 0;
  let failedCount = 0;

  for (const entry of entries) {
    const passed = entry.status === "passed";
    if (passed) {
      passedCount += 1;
    } else {
      failedCount += 1;
    }

    if (!byComparisonMode.has(entry.comparisonMode)) {
      byComparisonMode.set(entry.comparisonMode, {
        comparisonMode: entry.comparisonMode,
        classCount: 0,
        passedCount: 0,
        failedCount: 0,
        failedClassNames: [],
      });
    }
    const modeBucket = byComparisonMode.get(entry.comparisonMode);
    modeBucket.classCount += 1;
    if (passed) {
      modeBucket.passedCount += 1;
    } else {
      modeBucket.failedCount += 1;
      modeBucket.failedClassNames.push(entry.className);
    }

    const laneKey = `${entry.ownerLaneNumber ?? "unassigned"}:${entry.ownerLaneName}`;
    if (!byOwnerLane.has(laneKey)) {
      byOwnerLane.set(laneKey, {
        ownerLaneNumber: entry.ownerLaneNumber,
        ownerLaneName: entry.ownerLaneName,
        classCount: 0,
        passedCount: 0,
        failedCount: 0,
        comparisonModeCounts: {},
        failedClassNames: [],
      });
    }
    const laneBucket = byOwnerLane.get(laneKey);
    laneBucket.classCount += 1;
    incrementCount(laneBucket.comparisonModeCounts, entry.comparisonMode);
    if (passed) {
      laneBucket.passedCount += 1;
    } else {
      laneBucket.failedCount += 1;
      laneBucket.failedClassNames.push(entry.className);
    }
  }

  const modeOrder = new Map(
    ["exact-fragment-match", "known-different", "tailwind-only-gap"].map((mode, index) => [
      mode,
      index,
    ]),
  );

  const byComparisonModeBuckets = [...byComparisonMode.values()]
    .sort((left, right) => {
      const leftOrder = modeOrder.get(left.comparisonMode) ?? Number.MAX_SAFE_INTEGER;
      const rightOrder = modeOrder.get(right.comparisonMode) ?? Number.MAX_SAFE_INTEGER;
      return leftOrder === rightOrder
        ? left.comparisonMode.localeCompare(right.comparisonMode)
        : leftOrder - rightOrder;
    })
    .map((bucket) => ({
      ...bucket,
      passPercent: percentage(bucket.passedCount, bucket.classCount),
    }));

  const byOwnerLaneBuckets = [...byOwnerLane.values()]
    .sort((left, right) => {
      const leftLane = left.ownerLaneNumber ?? Number.MAX_SAFE_INTEGER;
      const rightLane = right.ownerLaneNumber ?? Number.MAX_SAFE_INTEGER;
      return leftLane === rightLane
        ? left.ownerLaneName.localeCompare(right.ownerLaneName)
        : leftLane - rightLane;
    })
    .map((bucket) => ({
      ...bucket,
      passPercent: percentage(bucket.passedCount, bucket.classCount),
    }));

  return {
    classCount: entries.length,
    passedCount,
    failedCount,
    passPercent: percentage(passedCount, entries.length),
    byComparisonMode: byComparisonModeBuckets,
    byOwnerLane: byOwnerLaneBuckets,
    policy:
      "Summarizes actual pass/fail comparison results by comparison mode and owner lane; classification policy remains separate from observed CSS output.",
  };
}

function percentage(part, total) {
  if (total === 0) {
    return 0;
  }
  return Number(((part / total) * 100).toFixed(2));
}

function matrixClassificationSummary(matrix) {
  const byComparisonMode = {};
  const byOwnerLane = new Map();
  let exactFragmentMatchCount = 0;
  let knownDifferentCount = 0;
  let tailwindOnlyGapCount = 0;

  for (const entry of matrix.classes) {
    const mode = entry.comparisonMode;
    incrementCount(byComparisonMode, mode);
    if (mode === "exact-fragment-match") {
      exactFragmentMatchCount += 1;
    } else if (mode === "known-different") {
      knownDifferentCount += 1;
    } else if (mode === "tailwind-only-gap") {
      tailwindOnlyGapCount += 1;
    }

    const owner = ownerLaneForEntry(entry);
    const key = `${owner.ownerLaneNumber ?? "unassigned"}:${owner.ownerLaneName}`;
    if (!byOwnerLane.has(key)) {
      byOwnerLane.set(key, {
        ownerLaneNumber: owner.ownerLaneNumber,
        ownerLaneName: owner.ownerLaneName,
        classCount: 0,
        comparisonModeCounts: {},
        areaCounts: {},
      });
    }

    const bucket = byOwnerLane.get(key);
    bucket.classCount += 1;
    incrementCount(bucket.comparisonModeCounts, mode);
    incrementCount(bucket.areaCounts, entry.area);
  }

  const classCount = matrix.classes.length;
  const byOwnerLaneBuckets = [...byOwnerLane.values()].sort((left, right) => {
    const leftLane = left.ownerLaneNumber ?? Number.MAX_SAFE_INTEGER;
    const rightLane = right.ownerLaneNumber ?? Number.MAX_SAFE_INTEGER;
    return leftLane === rightLane
      ? left.ownerLaneName.localeCompare(right.ownerLaneName)
      : leftLane - rightLane;
  });

  return {
    classCount,
    exactFragmentMatchCount,
    knownDifferentCount,
    tailwindOnlyGapCount,
    byComparisonMode,
    byOwnerLane: byOwnerLaneBuckets,
    exactOutputParityPercent: percentage(exactFragmentMatchCount, classCount),
    governedCompatibilityPercent: percentage(
      exactFragmentMatchCount + knownDifferentCount,
      classCount,
    ),
    tailwindOnlyGapPercent: percentage(tailwindOnlyGapCount, classCount),
    fullTailwindParity: false,
    policy:
      "Classification summary describes the governed fixture matrix contract, not full Tailwind parity; known-different entries, when present, remain covered but are not exact output matches.",
  };
}

function receiptEvidenceQuality(tailwindBuild, dxBuild, comparisonStatus, blockerStage = null) {
  const fixtureFreshness = dxBuild?.fixtureBinaryFreshness ?? null;
  const dxStyleCssSource = dxBuild?.source ?? null;
  const tailwindCssSource = tailwindBuild?.source ?? null;
  const liveTailwindExecution = tailwindBuild?.liveTailwindExecution === true;
  const staleFixtureBinaryAllowed = fixtureFreshness?.staleAllowed === true;
  const freshDxStyleOutput =
    dxStyleCssSource === "cargo-run" ||
    fixtureFreshness === null ||
    (fixtureFreshness.fresh === true && !staleFixtureBinaryAllowed);
  const nonCanonicalReasons = [];

  if (comparisonStatus === "blocked") {
    nonCanonicalReasons.push(`comparison-blocked:${blockerStage ?? "unknown"}`);
  }
  if (tailwindCssSource === "provided-css-file") {
    nonCanonicalReasons.push("tailwind-css-fixture-used");
  } else if (!liveTailwindExecution) {
    nonCanonicalReasons.push("tailwind-cli-not-executed");
  }
  if (!freshDxStyleOutput) {
    nonCanonicalReasons.push(
      staleFixtureBinaryAllowed
        ? "stale-dx-style-fixture-binary-allowed"
        : "dx-style-output-not-fresh",
    );
  }
  if (dxStyleCssSource === null) {
    nonCanonicalReasons.push("dx-style-output-unavailable");
  }

  return {
    canonicalLiveComparison: nonCanonicalReasons.length === 0,
    comparisonStatus,
    liveTailwindExecution,
    tailwindCssSource,
    dxStyleCssSource,
    freshDxStyleOutput,
    staleFixtureBinaryAllowed,
    fixtureBinaryFresh: fixtureFreshness?.fresh ?? null,
    nonCanonicalReasons,
    policy:
      "Canonical live comparison evidence requires Tailwind CLI execution plus fresh dx-style output; provided CSS fixtures, stale allowed binaries, and blocked runs are local diagnostics only.",
  };
}

function officialCandidateInventoryCoverage(matrix, officialInventory) {
  const candidates = new Set(officialInventory.inventory.candidates ?? []);
  const seen = new Set();
  const duplicateClassNames = new Set();
  const missingClassEntries = [];
  let presentClassCount = 0;

  for (const entry of matrix.classes) {
    if (seen.has(entry.className)) {
      duplicateClassNames.add(entry.className);
    }
    seen.add(entry.className);

    if (candidates.has(entry.className)) {
      presentClassCount += 1;
      continue;
    }

    const owner = ownerLaneForEntry(entry);
    missingClassEntries.push({
      className: entry.className,
      area: entry.area,
      comparisonMode: entry.comparisonMode,
      sourceIds: entry.sourceIds ?? [],
      ownerLaneNumber: owner.ownerLaneNumber,
      ownerLaneName: owner.ownerLaneName,
      fullTailwindParity: entry.fullTailwindParity === true,
    });
  }

  const missingSourceIdCounts = {};
  for (const entry of missingClassEntries) {
    for (const sourceId of entry.sourceIds) {
      incrementCount(missingSourceIdCounts, sourceId);
    }
  }

  return {
    matrixClassCount: matrix.classes.length,
    uniqueMatrixClassCount: seen.size,
    inventoryCandidateCount: officialInventory.inventory.candidateCount,
    inventorySourceFileCount: officialInventory.inventory.sourceFileCount,
    inventoryCandidateSourceFileCount:
      officialInventory.inventory.candidateSourceFileCount ??
      officialInventory.inventory.sourceFileCount,
    presentClassCount,
    missingClassCount: missingClassEntries.length,
    duplicateClassCount: duplicateClassNames.size,
    matrixClassesAllInInventory: missingClassEntries.length === 0,
    missingClassNames: missingClassEntries.map((entry) => entry.className),
    duplicateClassNames: [...duplicateClassNames],
    missingClassEntries,
    missingSourceIdCounts,
    policy:
      "Candidate inventory coverage is provenance metadata; docs/release matrix canaries absent from Tailwind source-test candidates remain CSS comparison entries, not Tailwind-only gaps.",
  };
}

function positiveIntegerEnv(name, fallback) {
  const raw = process.env[name];
  if (!raw) {
    return fallback;
  }

  const parsed = Number(raw);
  if (!Number.isInteger(parsed) || parsed <= 0) {
    throw new Error(`${name} must be a positive integer number of milliseconds`);
  }
  return parsed;
}

function npmCommand() {
  return process.platform === "win32" ? "npm.cmd" : "npm";
}

function npxCommand() {
  return process.platform === "win32" ? "npx.cmd" : "npx";
}

function cargoCommand() {
  if (process.env.DX_STYLE_CARGO_BIN) {
    return process.env.DX_STYLE_CARGO_BIN;
  }
  return process.platform === "win32" ? "cargo.exe" : "cargo";
}

function cargoTargetDir(tmpDir) {
  return (
    process.env.DX_STYLE_CARGO_TARGET_DIR ||
    path.join(tmpDir, "dx-style-live-cargo-target")
  );
}

function cargoRunTimeoutMs() {
  return positiveIntegerEnv("DX_STYLE_CARGO_RUN_TIMEOUT_MS", 540_000);
}

function fixtureBinaryTimeoutMs() {
  return positiveIntegerEnv("DX_STYLE_FIXTURE_BIN_TIMEOUT_MS", 60_000);
}

function fixtureCssBinaryPath() {
  return process.env.DX_STYLE_FIXTURE_CSS_BIN || null;
}

function repoRelative(filePath) {
  return path.relative(root, filePath).replace(/\\/g, "/");
}

function displayPath(filePath) {
  const absolutePath = path.resolve(root, filePath);
  const relativePath = repoRelative(absolutePath);
  return relativePath.startsWith("..") ? absolutePath : relativePath;
}

function walkFiles(dir, predicate, files = []) {
  if (!fs.existsSync(dir)) {
    return files;
  }

  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    const entryPath = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      walkFiles(entryPath, predicate, files);
    } else if (predicate(entryPath)) {
      files.push(entryPath);
    }
  }

  return files;
}

function fixtureBinaryFreshnessInputs(matrixPath) {
  return [
    matrixPath,
    ...walkFiles(path.join(root, "related-crates/style/src"), (filePath) =>
      filePath.endsWith(".rs"),
    ),
  ];
}

function staleFixtureBinariesAllowed() {
  return process.env.DX_STYLE_ALLOW_STALE_FIXTURE_BIN === "1";
}

function dxStyleFixtureBinaryFreshness(binaryPath, matrixPath) {
  const absoluteBinaryPath = path.resolve(root, binaryPath);
  if (!fs.existsSync(absoluteBinaryPath)) {
    throw new Error(`dx-style fixture binary does not exist: ${binaryPath}`);
  }

  const binaryStat = fs.statSync(absoluteBinaryPath);
  const inputs = fixtureBinaryFreshnessInputs(matrixPath).filter((input) =>
    fs.existsSync(input),
  );
  const newestInput = inputs
    .map((input) => ({ path: input, mtimeMs: fs.statSync(input).mtimeMs }))
    .sort((left, right) => right.mtimeMs - left.mtimeMs)[0] ?? null;
  const fresh = newestInput ? binaryStat.mtimeMs >= newestInput.mtimeMs : true;
  const staleAllowed = staleFixtureBinariesAllowed();

  return {
    schema: "dx.style.fixtureBinaryFreshnessReceipt",
    schemaVersion: 1,
    binaryPath: repoRelative(absoluteBinaryPath) || absoluteBinaryPath,
    binaryMtimeMs: binaryStat.mtimeMs,
    newestInputPath: newestInput ? repoRelative(newestInput.path) : null,
    newestInputMtimeMs: newestInput?.mtimeMs ?? null,
    inputCount: inputs.length,
    fresh,
    staleAllowed,
    reason: fresh
      ? "dx-style fixture binary is fresh against the matrix and Rust source inputs"
      : staleAllowed
        ? "stale dx-style fixture binary allowed by DX_STYLE_ALLOW_STALE_FIXTURE_BIN=1 for local debugging"
        : "stale dx-style fixture binary rejected before live Tailwind comparison to avoid false classname gaps",
  };
}

function writeJsonOrSummary(args, receipt, summary) {
  if (args.receipt) {
    const receiptPath = path.resolve(root, args.receipt);
    fs.mkdirSync(path.dirname(receiptPath), { recursive: true });
    fs.writeFileSync(receiptPath, `${JSON.stringify(receipt, null, 2)}\n`);
  }

  if (args.json) {
    process.stdout.write(`${JSON.stringify(receipt, null, 2)}\n`);
  } else {
    process.stdout.write(`${summary}\n`);
  }
}

function markComparisonBlocker(error, stage, tailwindBuild = null, dxBuild = null) {
  error.comparisonBlockerStage = error.comparisonBlockerStage || stage;
  if (tailwindBuild) {
    error.comparisonTailwindBuild = error.comparisonTailwindBuild || tailwindBuild;
  }
  if (dxBuild) {
    error.comparisonDxBuild = error.comparisonDxBuild || dxBuild;
  }
  return error;
}

function errorText(error) {
  return error && error.message ? error.message : String(error);
}

function commandErrorReceipt(error) {
  return {
    status: Number.isInteger(error?.status) ? error.status : null,
    signal: error?.signal ?? null,
    stdout: error?.stdout ? String(error.stdout).slice(0, 4000) : "",
    stderr: error?.stderr ? String(error.stderr).slice(0, 4000) : "",
  };
}

function blockedNextAction(blockerStage, error) {
  const canonicalCommand =
    "node --test .\\benchmarks\\dx-style-live-tailwind-v43-comparison.test.ts";
  const message = errorText(error);

  if (blockerStage === "dx-style-build") {
    if (/timed out|timeout|ETIMEDOUT|SIGTERM/i.test(message)) {
      return `Resolve active Cargo/Rust build contention or let the current build finish, then rerun ${canonicalCommand}.`;
    }
    return `Fix dx-style fixture CSS generation, then rerun ${canonicalCommand}.`;
  }

  if (blockerStage === "dx-style-fixture-binary-stale") {
    return `Rebuild dx_style_fixture_css or unset DX_STYLE_FIXTURE_CSS_BIN, then rerun ${canonicalCommand}; do not use stale fixture output for canonical parity evidence.`;
  }

  if (blockerStage === "tailwind-build") {
    return `Restore the temporary Tailwind CLI build path, including npm/network availability, then rerun ${canonicalCommand}.`;
  }

  if (blockerStage === "tailwind-css-input") {
    return "Provide a readable --tailwind-css fixture path or omit --tailwind-css for live Tailwind CLI execution.";
  }

  if (blockerStage === "fixture-matrix-integrity") {
    return "Fix duplicate or invalid fixture matrix entries before rerunning the live comparison.";
  }

  return `Inspect blockerCommand, fix the blocked stage, then rerun ${canonicalCommand}.`;
}

function buildTailwindCss(matrix, tmpDir, args) {
  if (args.tailwindCss) {
    const cssPath = path.resolve(root, args.tailwindCss);
    const tailwindBuild = {
      source: "provided-css-file",
      path: displayPath(cssPath),
      liveTailwindExecution: false,
    };
    if (!fs.existsSync(cssPath)) {
      throw markComparisonBlocker(
        new Error(`Tailwind CSS fixture file is unavailable: ${displayPath(cssPath)}`),
        "tailwind-css-input",
        tailwindBuild,
      );
    }
    return {
      ...tailwindBuild,
      css: fs.readFileSync(cssPath, "utf8"),
    };
  }

  const tailwindBuild = {
    source: "tailwind-cli",
    path: null,
    liveTailwindExecution: true,
  };
  try {
    fs.writeFileSync(
      path.join(tmpDir, "package.json"),
      JSON.stringify({ private: true, type: "commonjs" }, null, 2),
    );

    run(
      npmCommand(),
      [
        "install",
        "--no-save",
        "--silent",
        `${matrix.tailwindPackage.name}@${matrix.tailwindPackage.version}`,
        `${matrix.tailwindCliPackage.name}@${matrix.tailwindCliPackage.version}`,
      ],
      { cwd: tmpDir },
    );

    const classList = matrix.classes.map((entry) => entry.className).join(" ");
    fs.writeFileSync(
      path.join(tmpDir, "input.css"),
      `@import "tailwindcss" source(none);\n@source inline("${classList}");\n`,
    );

    run(npxCommand(), ["tailwindcss", "-i", "input.css", "-o", "tailwind.css"], {
      cwd: tmpDir,
    });

    return {
      ...tailwindBuild,
      css: fs.readFileSync(path.join(tmpDir, "tailwind.css"), "utf8"),
    };
  } catch (error) {
    throw markComparisonBlocker(error, "tailwind-build", tailwindBuild);
  }
}

function buildDxStyleCss(matrixPath, tmpDir) {
  const fixtureBinary = fixtureCssBinaryPath();
  if (fixtureBinary) {
    const fixtureBinaryFreshness = dxStyleFixtureBinaryFreshness(fixtureBinary, matrixPath);
    const dxBuild = {
      source: "provided-binary",
      fixtureBinaryFreshness,
      cargoTargetDir: null,
    };
    if (!fixtureBinaryFreshness.fresh && !fixtureBinaryFreshness.staleAllowed) {
      return {
        ...dxBuild,
        receipt: null,
      };
    }

    try {
      return {
        ...dxBuild,
        receipt: readReceiptJson(
          run(fixtureBinary, ["--matrix", matrixPath], {
            cwd: root,
            timeout: fixtureBinaryTimeoutMs(),
          }),
        ),
      };
    } catch (error) {
      throw markComparisonBlocker(error, "dx-style-build", null, dxBuild);
    }
  }

  const targetDir = cargoTargetDir(tmpDir);
  const dxBuild = {
    source: "cargo-run",
    fixtureBinaryFreshness: null,
    cargoTargetDir: displayPath(targetDir),
  };
  try {
    const output = run(
      cargoCommand(),
      [
        "run",
        "-p",
        "dx-style",
        "--quiet",
        "--bin",
        "dx_style_fixture_css",
        "--",
        "--matrix",
        matrixPath,
      ],
      {
        cwd: root,
        env: { CARGO_TARGET_DIR: targetDir },
        timeout: cargoRunTimeoutMs(),
      },
    );

    return {
      ...dxBuild,
      receipt: readReceiptJson(output),
    };
  } catch (error) {
    throw markComparisonBlocker(error, "dx-style-build", null, dxBuild);
  }
}

function readReceiptJson(output) {
  const trimmed = output.trim();
  const jsonStart = trimmed.indexOf("{");
  if (jsonStart === -1) {
    throw new Error(`dx-style fixture runner did not emit JSON: ${output}`);
  }
  return JSON.parse(trimmed.slice(jsonStart));
}

function compareEntry(entry, tailwindCss, dxEntry) {
  const tailwindFragmentsPresent = includesFragments(
    tailwindCss,
    entry.tailwindRequiredFragments,
  );
  const dxCss = dxEntry?.generated_css ?? "";
  const dxFragmentsPresent = includesFragments(dxCss, entry.dxStyleRequiredFragments ?? []);
  const dxSupported = dxEntry?.supported === true;

  let passed = false;
  if (entry.comparisonMode === "exact-fragment-match") {
    passed = tailwindFragmentsPresent && dxSupported && dxFragmentsPresent;
  } else if (entry.comparisonMode === "known-different") {
    passed = tailwindFragmentsPresent && dxSupported && dxFragmentsPresent;
  } else if (entry.comparisonMode === "tailwind-only-gap") {
    passed = tailwindFragmentsPresent && !dxSupported;
  } else {
    throw new Error(`unsupported comparisonMode ${entry.comparisonMode}`);
  }

  const compared = {
    className: entry.className,
    area: entry.area,
    comparisonMode: entry.comparisonMode,
    status: passed ? "passed" : "failed",
    tailwindFragmentsPresent,
    dxStyleSupported: dxSupported,
    dxStyleFragmentsPresent: dxFragmentsPresent,
    differenceNote: entry.differenceNote ?? null,
    gapNote: entry.gapNote ?? null,
  };
  const owner = ownerLaneForEntry(entry);
  return {
    ...compared,
    ...owner,
    failureReasons: failureReasonsForEntry({ ...compared, ...owner }),
  };
}

function createReceipt(matrix, matrixPath, officialInventory, tailwindBuild, dxBuild, tmpDir, keepTemp) {
  const dxEntries = new Map(
    dxBuild.receipt.classes.map((entry) => [entry.class_name, entry]),
  );
  const entries = matrix.classes.map((entry) =>
    compareEntry(entry, tailwindBuild.css, dxEntries.get(entry.className)),
  );
  const failedEntries = entries.filter((entry) => entry.status === "failed");
  const failedCount = failedEntries.length;
  const failedClassHandoffsReceipt = failedClassHandoffs(failedEntries);
  const failureLaneBucketsReceipt = failureLaneBuckets(failedEntries);

  return {
    schema: "dx.style.liveTailwindComparisonReceipt",
    schemaVersion: 1,
    comparisonStatus: "compared",
    fixtureMatrix: path.relative(root, matrixPath).replace(/\\/g, "/"),
    fixtureMatrixIngested: true,
    officialSourceCommit: matrix.officialSource?.commit ?? null,
    officialCandidateInventory: officialInventory.path,
    officialCandidateCount: officialInventory.inventory.candidateCount,
    officialSourceFileCount: officialInventory.inventory.sourceFileCount,
    officialFixtureCount: officialInventory.inventory.officialFixtureMatrix.fixtureCount,
    officialFixtureSourceFileCount:
      officialInventory.inventory.officialFixtureMatrix.fixtureSourceFileCount,
    inputFingerprints: receiptInputFingerprints(matrix, officialInventory),
    matrixIntegrity: fixtureMatrixIntegrity(matrix),
    classificationSummary: matrixClassificationSummary(matrix),
    comparisonResultSummary: comparisonResultSummary(entries),
    evidenceQuality: receiptEvidenceQuality(tailwindBuild, dxBuild, "compared"),
    officialCandidateInventoryCoverage: officialCandidateInventoryCoverage(
      matrix,
      officialInventory,
    ),
    tailwindPackage: `${matrix.tailwindPackage.name}@${matrix.tailwindPackage.version}`,
    tailwindCliPackage: `${matrix.tailwindCliPackage.name}@${matrix.tailwindCliPackage.version}`,
    liveTailwindExecution: tailwindBuild.liveTailwindExecution,
    tailwindCssSource: tailwindBuild.source,
    tailwindCssPath: tailwindBuild.path,
    dxStyleCssSource: dxBuild.source,
    dxStyleFixtureBinaryFreshness: dxBuild.fixtureBinaryFreshness,
    dxStyleCargoTargetDir: dxBuild.cargoTargetDir,
    tailwindRuntimeDependency: false,
    packageManifestMutation: false,
    tailwindInstallScope: tailwindBuild.liveTailwindExecution
      ? "temporary-directory-only"
      : "skipped-provided-css-file",
    temporaryDirectory: keepTemp ? tmpDir : null,
    classCount: entries.length,
    exactFragmentMatchCount: entries.filter(
      (entry) => entry.comparisonMode === "exact-fragment-match",
    ).length,
    knownDifferentCount: entries.filter(
      (entry) => entry.comparisonMode === "known-different",
    ).length,
    tailwindOnlyGapCount: entries.filter(
      (entry) => entry.comparisonMode === "tailwind-only-gap",
    ).length,
    failedCount,
    failedClassNames: failedEntries.map((entry) => entry.className),
    failedUnsupportedByDxStyleClassNames: failedEntries
      .filter((entry) => !entry.dxStyleSupported)
      .map((entry) => entry.className),
    failedMissingTailwindFragmentsClassNames: failedEntries
      .filter((entry) => !entry.tailwindFragmentsPresent)
      .map((entry) => entry.className),
    failedMissingDxStyleFragmentsClassNames: failedEntries
      .filter((entry) => entry.dxStyleSupported && !entry.dxStyleFragmentsPresent)
      .map((entry) => entry.className),
    failedKnownDifferentClassNames: failedEntries
      .filter((entry) => entry.comparisonMode === "known-different")
      .map((entry) => entry.className),
    failedExactFragmentMatchClassNames: failedEntries
      .filter((entry) => entry.comparisonMode === "exact-fragment-match")
      .map((entry) => entry.className),
    failedTailwindOnlyGapClassNames: failedEntries
      .filter((entry) => entry.comparisonMode === "tailwind-only-gap")
      .map((entry) => entry.className),
    failedClassHandoffs: failedClassHandoffsReceipt,
    failureLaneBuckets: failureLaneBucketsReceipt,
    fullTailwindParity: false,
    entries,
  };
}

function plannedTailwindBuild(args) {
  if (args.tailwindCss) {
    const cssPath = path.resolve(root, args.tailwindCss);
    return {
      source: "provided-css-file",
      path: displayPath(cssPath),
      liveTailwindExecution: false,
    };
  }

  return {
    source: "tailwind-cli",
    path: null,
    liveTailwindExecution: false,
  };
}

function createBlockedReceipt(
  matrix,
  matrixPath,
  officialInventory,
  args,
  dxBuild,
  tmpDir,
  keepTemp,
  error,
) {
  const blockerStage = error?.comparisonBlockerStage ?? "dx-style-build";
  const tailwindBuild = error?.comparisonTailwindBuild ?? plannedTailwindBuild(args);
  const blockedDxBuild = dxBuild ?? error?.comparisonDxBuild ?? null;
  const classificationSummary = matrixClassificationSummary(matrix);

  return {
    schema: "dx.style.liveTailwindComparisonBlockedReceipt",
    schemaVersion: 1,
    comparisonStatus: "blocked",
    blockerStage,
    blockerMessage: errorText(error),
    blockerCommand: commandErrorReceipt(error),
    nextAction: blockedNextAction(blockerStage, error),
    blockedRunPolicy:
      "Blocked live comparisons are infrastructure evidence only: they keep fullTailwindParity false, emit the governed matrix counts, and must not be counted as canonical parity passes.",
    fixtureMatrix: path.relative(root, matrixPath).replace(/\\/g, "/"),
    fixtureMatrixIngested: true,
    officialSourceCommit: matrix.officialSource?.commit ?? null,
    officialCandidateInventory: officialInventory.path,
    officialCandidateCount: officialInventory.inventory.candidateCount,
    officialSourceFileCount: officialInventory.inventory.sourceFileCount,
    officialFixtureCount: officialInventory.inventory.officialFixtureMatrix.fixtureCount,
    officialFixtureSourceFileCount:
      officialInventory.inventory.officialFixtureMatrix.fixtureSourceFileCount,
    inputFingerprints: receiptInputFingerprints(matrix, officialInventory),
    matrixIntegrity: error?.matrixIntegrity ?? fixtureMatrixIntegrity(matrix),
    classificationSummary,
    evidenceQuality: receiptEvidenceQuality(
      tailwindBuild,
      blockedDxBuild,
      "blocked",
      blockerStage,
    ),
    officialCandidateInventoryCoverage: officialCandidateInventoryCoverage(
      matrix,
      officialInventory,
    ),
    tailwindPackage: `${matrix.tailwindPackage.name}@${matrix.tailwindPackage.version}`,
    tailwindCliPackage: `${matrix.tailwindCliPackage.name}@${matrix.tailwindCliPackage.version}`,
    liveTailwindExecution: tailwindBuild.liveTailwindExecution,
    tailwindCssSource: tailwindBuild.source,
    tailwindCssPath: tailwindBuild.path,
    dxStyleCssSource: blockedDxBuild?.source ?? null,
    dxStyleFixtureBinaryFreshness: blockedDxBuild?.fixtureBinaryFreshness ?? null,
    dxStyleCargoTargetDir: blockedDxBuild?.cargoTargetDir ?? null,
    tailwindRuntimeDependency: false,
    packageManifestMutation: false,
    tailwindInstallScope: args.tailwindCss
      ? "skipped-provided-css-file"
      : "temporary-directory-only",
    temporaryDirectory: keepTemp ? tmpDir : null,
    classCount: matrix.classes.length,
    comparedClassCount: 0,
    exactFragmentMatchCount: classificationSummary.exactFragmentMatchCount,
    knownDifferentCount: classificationSummary.knownDifferentCount,
    tailwindOnlyGapCount: classificationSummary.tailwindOnlyGapCount,
    failedCount: null,
    failedClassNames: [],
    failedUnsupportedByDxStyleClassNames: [],
    failedMissingTailwindFragmentsClassNames: [],
    failedMissingDxStyleFragmentsClassNames: [],
    failedClassHandoffs: [],
    failureLaneBuckets: [],
    fullTailwindParity: false,
    entries: [],
  };
}

function main() {
  const args = parseArgs(process.argv);
  const matrixPath = path.resolve(root, args.matrix);
  const matrix = readJson(matrixPath);

  if (matrix.tailwindRuntimeDependency !== false) {
    throw new Error("fixture matrix must not require Tailwind as a runtime dependency");
  }
  if (matrix.tailwindPackage.version !== "4.3.0") {
    throw new Error(`expected tailwindcss@4.3.0, got ${matrix.tailwindPackage.version}`);
  }
  const officialInventory = readOfficialInventory(matrix);

  if (args.validateFixtureBinary) {
    const receipt = dxStyleFixtureBinaryFreshness(args.validateFixtureBinary, matrixPath);
    writeJsonOrSummary(
      args,
      receipt,
      `dx-style fixture binary freshness: ${receipt.fresh ? "fresh" : "stale"}`,
    );
    if (!receipt.fresh && !receipt.staleAllowed) {
      process.exitCode = 1;
    }
    return;
  }

  const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-style-tailwind-v43-"));
  let dxBuild = null;
  try {
    try {
      assertFixtureMatrixIntegrity(matrix);
      dxBuild = buildDxStyleCss(matrixPath, tmpDir);
      if (!dxBuild.receipt) {
        const error = markComparisonBlocker(
          new Error(
            dxBuild.fixtureBinaryFreshness?.reason ??
              "dx-style fixture binary did not produce a comparison receipt",
          ),
          "dx-style-fixture-binary-stale",
          null,
          dxBuild,
        );
        const receipt = createBlockedReceipt(
          matrix,
          matrixPath,
          officialInventory,
          args,
          dxBuild,
          tmpDir,
          args.keepTemp,
          error,
        );
        writeJsonOrSummary(
          args,
          receipt,
          `Tailwind v4.3 live comparison blocked at ${receipt.blockerStage}: ${receipt.blockerMessage}`,
        );
        process.exitCode = 1;
        return;
      }
      const tailwindBuild = buildTailwindCss(matrix, tmpDir, args);
      const receipt = createReceipt(
        matrix,
        matrixPath,
        officialInventory,
        tailwindBuild,
        dxBuild,
        tmpDir,
        args.keepTemp,
      );

      writeJsonOrSummary(
        args,
        receipt,
        `Tailwind v4.3 live comparison: ${receipt.classCount - receipt.failedCount}/${receipt.classCount} passed`,
      );

      if (receipt.failedCount > 0) {
        process.exitCode = 1;
      }
    } catch (error) {
      const receipt = createBlockedReceipt(
        matrix,
        matrixPath,
        officialInventory,
        args,
        dxBuild,
        tmpDir,
        args.keepTemp,
        markComparisonBlocker(error, error?.comparisonBlockerStage ?? "dx-style-build"),
      );
      writeJsonOrSummary(
        args,
        receipt,
        `Tailwind v4.3 live comparison blocked at ${receipt.blockerStage}: ${receipt.blockerMessage}`,
      );
      process.exitCode = 1;
    }
  } finally {
    if (!args.keepTemp) {
      fs.rmSync(tmpDir, { recursive: true, force: true });
    }
  }
}

main();
