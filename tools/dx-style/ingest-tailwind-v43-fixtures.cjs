#!/usr/bin/env node
const { execFileSync } = require("node:child_process");
const crypto = require("node:crypto");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");

const root = path.resolve(__dirname, "../..");
const OFFICIAL_REPOSITORY = "tailwindlabs/tailwindcss";
const OFFICIAL_REPOSITORY_URL = "https://github.com/tailwindlabs/tailwindcss.git";
const OFFICIAL_TAG = "v4.3.0";
const OFFICIAL_COMMIT = "588bd7371f4cae96426e1387819b7fd1d99765f9";
const TAILWIND_PACKAGE = {
  name: "tailwindcss",
  version: "4.3.0",
  tarball: "https://registry.npmjs.org/tailwindcss/-/tailwindcss-4.3.0.tgz",
  shasum: "0a874e044a859cf6de413f3a59e76a9bedf05264",
  integrity:
    "sha512-y6nxMGB1nMW9R6k96e5gdIFzcfL/gTJRNaqGes1YvkLnPVXzWgbqFF2yLC0T8G774n24cx3Pe8XrKoniCOAH+Q==",
};
const TAILWIND_CLI_PACKAGE = {
  name: "@tailwindcss/cli",
  version: "4.3.0",
};
const SOURCE_ROOTS = [
  "packages/tailwindcss/src",
  "packages/@tailwindcss-cli/src",
  "packages/@tailwindcss-postcss/src",
  "integrations",
];

function parseArgs(argv) {
  const args = {
    sourceDir: null,
    output:
      "related-crates/style/fixtures/tailwind-v43-official-candidate-inventory.json",
    matrix: "related-crates/style/fixtures/tailwind-v43-official-fixture-matrix.json",
    syncMatrix: true,
    json: false,
    keepTemp: false,
  };

  for (let index = 2; index < argv.length; index += 1) {
    const arg = argv[index];
    if (arg === "--source-dir") {
      args.sourceDir = argv[++index];
    } else if (arg === "--output") {
      args.output = argv[++index];
    } else if (arg === "--matrix") {
      args.matrix = argv[++index];
    } else if (arg === "--no-sync-matrix") {
      args.syncMatrix = false;
    } else if (arg === "--json") {
      args.json = true;
    } else if (arg === "--keep-temp") {
      args.keepTemp = true;
    } else {
      throw new Error(`unknown argument: ${arg}`);
    }
  }

  return args;
}

function relativePathFromRoot(filePath) {
  return path.relative(root, path.resolve(root, filePath)).replace(/\\/g, "/");
}

function gitCommand() {
  return process.platform === "win32" ? "git.exe" : "git";
}

function run(command, args, options = {}) {
  return execFileSync(command, args, {
    cwd: options.cwd ?? root,
    encoding: "utf8",
    maxBuffer: 50 * 1024 * 1024,
    stdio: ["ignore", "pipe", "pipe"],
    timeout: options.timeout ?? 180_000,
  });
}

function cloneOfficialSource() {
  const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-style-tailwind-v43-source-"));
  run(
    gitCommand(),
    [
      "clone",
      "--depth",
      "1",
      "--branch",
      OFFICIAL_TAG,
      OFFICIAL_REPOSITORY_URL,
      tmpDir,
      "--quiet",
    ],
    { timeout: 180_000 },
  );
  return tmpDir;
}

function walkFiles(dir, out = []) {
  if (!fs.existsSync(dir)) {
    return out;
  }

  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    const fullPath = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      walkFiles(fullPath, out);
    } else if (
      /\.(test|spec)\.(ts|tsx|js|jsx)$/.test(entry.name) ||
      entry.name.endsWith(".snap")
    ) {
      out.push(fullPath);
    }
  }
  return out;
}

function isCandidateLike(value) {
  if (value.length < 2 || value.length > 140) {
    return false;
  }
  if (/\s/.test(value)) {
    return false;
  }
  if (/[;{}<>]/.test(value)) {
    return false;
  }
  if (!/[-:@[\]()/!*.]/.test(value)) {
    return false;
  }
  if (
    /^(css|html|js|ts|tsx|div|span|test|expect|theme|utilities|components|base|true|false|null|undefined|let|const|return|await|async|from|import|export)$/.test(
      value,
    )
  ) {
    return false;
  }
  return /^[!?-]?(?:\[[^\s]+]|[a-zA-Z0-9_@*.][^\s`'"<>]*)(?::[!?-]?(?:\[[^\s]+]|[a-zA-Z0-9_@*.][^\s`'"<>]*))*$/.test(
    value,
  );
}

function collectSourceFiles(sourceDir) {
  return SOURCE_ROOTS.flatMap((sourceRoot) =>
    walkFiles(path.join(sourceDir, sourceRoot)),
  ).sort((a, b) => a.localeCompare(b));
}

function lineNumberAt(text, offset) {
  return text.slice(0, offset).split(/\r?\n/).length;
}

function quotedCandidateStrings(input) {
  const quotedString = /['"`]([^'"`\n]{2,140})['"`]/g;
  const candidates = [];
  let match;

  while ((match = quotedString.exec(input))) {
    const candidate = match[1].trim();
    if (isCandidateLike(candidate)) {
      candidates.push(candidate);
    }
  }

  return [...new Set(candidates)];
}

function classifyFixture(sourceFile, context) {
  const normalized = sourceFile.replace(/\\/g, "/");
  if (normalized.endsWith("/candidate.test.ts")) {
    return "candidate-parser";
  }
  if (/compileCss|compiler\.build|build\(/.test(context)) {
    return "compiler-output";
  }
  if (/@source|@theme|@utility|@custom-variant|@variant|@apply|@reference/.test(context)) {
    return "css-directive";
  }
  if (/variant|variants/.test(context)) {
    return "variant-parser";
  }
  return "inline-snapshot";
}

function extractOfficialFixtures(sourceDir, sourceFiles) {
  const fixtureEntries = [];
  const snapshotCall = /toMatchInlineSnapshot\(\s*`([\s\S]*?)`\s*\)/g;

  for (const sourceFile of sourceFiles) {
    const text = fs.readFileSync(sourceFile, "utf8");
    let match;

    while ((match = snapshotCall.exec(text))) {
      const expectStart = text.lastIndexOf("expect(", match.index);
      const contextStart = Math.max(0, (expectStart === -1 ? match.index : expectStart) - 1600);
      const context = text.slice(contextStart, match.index);
      const candidates = quotedCandidateStrings(context).slice(-40);

      if (candidates.length === 0) {
        continue;
      }

      const snapshot = match[1].trim();
      fixtureEntries.push({
        sourceFile: path.relative(sourceDir, sourceFile).replace(/\\/g, "/"),
        line: lineNumberAt(text, match.index),
        kind: classifyFixture(sourceFile, context),
        candidates,
        snapshotSha256: crypto.createHash("sha256").update(snapshot).digest("hex"),
        snapshotLineCount: snapshot === "" ? 0 : snapshot.split(/\r?\n/).length,
      });
    }
  }

  return fixtureEntries.sort((a, b) => {
    const fileOrder = a.sourceFile.localeCompare(b.sourceFile);
    return fileOrder === 0 ? a.line - b.line : fileOrder;
  });
}

function extractCandidates(sourceDir, sourceFiles) {
  const candidates = new Set();
  const sourceFileHits = new Map();
  const quotedString = /['"`]([^'"`\n]{2,140})['"`]/g;

  for (const sourceFile of sourceFiles) {
    const text = fs.readFileSync(sourceFile, "utf8");
    let match;
    let hitCount = 0;
    while ((match = quotedString.exec(text))) {
      const candidate = match[1].trim();
      if (!isCandidateLike(candidate)) {
        continue;
      }
      candidates.add(candidate);
      hitCount += 1;
    }
    if (hitCount > 0) {
      sourceFileHits.set(path.relative(sourceDir, sourceFile).replace(/\\/g, "/"), hitCount);
    }
  }

  return {
    sourceFileCount: sourceFiles.length,
    candidateSourceFileCount: sourceFileHits.size,
    sourceFileHits: Object.fromEntries([...sourceFileHits.entries()].sort()),
    candidates: [...candidates].sort((a, b) => a.localeCompare(b)),
  };
}

function buildInventory(sourceDir) {
  const commit = run(gitCommand(), ["rev-parse", "HEAD"], { cwd: sourceDir }).trim();
  if (commit !== OFFICIAL_COMMIT) {
    throw new Error(`expected ${OFFICIAL_REPOSITORY}@${OFFICIAL_TAG} ${OFFICIAL_COMMIT}, got ${commit}`);
  }

  const sourceFiles = collectSourceFiles(sourceDir);
  const extracted = extractCandidates(sourceDir, sourceFiles);
  const officialFixtures = extractOfficialFixtures(sourceDir, sourceFiles);
  return {
    schema: "dx.style.tailwindOfficialCandidateInventory",
    schemaVersion: 1,
    generatedBy: "tools/dx-style/ingest-tailwind-v43-fixtures.cjs",
    generatedFrom: "official Tailwind v4.3 source tests, inline snapshot fixtures, and npm package metadata",
    officialSource: {
      repository: OFFICIAL_REPOSITORY,
      tag: OFFICIAL_TAG,
      commit: OFFICIAL_COMMIT,
      sourceRoots: SOURCE_ROOTS,
    },
    tailwindPackage: TAILWIND_PACKAGE,
    tailwindCliPackage: TAILWIND_CLI_PACKAGE,
    tailwindRuntimeDependency: false,
    fullTailwindParity: false,
    sourceFileCount: extracted.sourceFileCount,
    candidateSourceFileCount: extracted.candidateSourceFileCount,
    candidateCount: extracted.candidates.length,
    officialFixtureMatrix: {
      schema: "dx.style.tailwindOfficialFixtureMatrix",
      schemaVersion: 1,
      generatedFrom:
        "official Tailwind v4.3 inline snapshot tests; expected output is fingerprinted, not vendored",
      fixtureCount: officialFixtures.length,
      fixtureSourceFileCount: new Set(officialFixtures.map((entry) => entry.sourceFile)).size,
      entries: officialFixtures,
    },
    sourceFileHits: extracted.sourceFileHits,
    candidates: extracted.candidates,
  };
}

function syncMatrixOfficialFixtureTruth(inventory, inventoryPath, matrixPath) {
  const absoluteMatrixPath = path.resolve(root, matrixPath);
  if (!fs.existsSync(absoluteMatrixPath)) {
    throw new Error(`expected fixture matrix to exist: ${relativePathFromRoot(matrixPath)}`);
  }

  const matrix = JSON.parse(fs.readFileSync(absoluteMatrixPath, "utf8"));
  if (matrix.schema !== "dx.style.tailwindOfficialFixtureMatrix") {
    throw new Error(`invalid fixture matrix schema: ${matrix.schema}`);
  }
  if (matrix.tailwindPackage?.version !== inventory.tailwindPackage.version) {
    throw new Error(
      `matrix Tailwind version ${matrix.tailwindPackage?.version} does not match inventory ${inventory.tailwindPackage.version}`,
    );
  }
  if (matrix.officialSource?.commit !== inventory.officialSource.commit) {
    throw new Error(
      `matrix commit ${matrix.officialSource?.commit} does not match inventory ${inventory.officialSource.commit}`,
    );
  }

  const inventoryRelativePath = relativePathFromRoot(inventoryPath);
  matrix.officialFixtureTruth = {
    inventory: inventoryRelativePath,
    sourceFileCount: inventory.sourceFileCount,
    candidateSourceFileCount: inventory.candidateSourceFileCount,
    candidateCount: inventory.candidateCount,
    fixtureCount: inventory.officialFixtureMatrix.fixtureCount,
    fixtureSourceFileCount: inventory.officialFixtureMatrix.fixtureSourceFileCount,
    snapshotOutputPolicy: "fingerprinted-not-vendored",
    fullTailwindParity: false,
  };

  fs.writeFileSync(absoluteMatrixPath, `${JSON.stringify(matrix, null, 2)}\n`);
  return relativePathFromRoot(matrixPath);
}

function main() {
  const args = parseArgs(process.argv);
  let sourceDir = args.sourceDir ? path.resolve(args.sourceDir) : null;
  let tempSourceDir = null;

  try {
    if (!sourceDir) {
      tempSourceDir = cloneOfficialSource();
      sourceDir = tempSourceDir;
    }

    const inventory = buildInventory(sourceDir);
    const outputPath = path.resolve(root, args.output);
    fs.mkdirSync(path.dirname(outputPath), { recursive: true });
    fs.writeFileSync(outputPath, `${JSON.stringify(inventory, null, 2)}\n`);
    const syncedMatrix = args.syncMatrix
      ? syncMatrixOfficialFixtureTruth(inventory, args.output, args.matrix)
      : null;

    const summary = {
      output: path.relative(root, outputPath).replace(/\\/g, "/"),
      syncedMatrix,
      sourceFileCount: inventory.sourceFileCount,
      candidateSourceFileCount: inventory.candidateSourceFileCount,
      candidateCount: inventory.candidateCount,
      officialFixtureCount: inventory.officialFixtureMatrix.fixtureCount,
      officialFixtureSourceFileCount: inventory.officialFixtureMatrix.fixtureSourceFileCount,
      officialCommit: inventory.officialSource.commit,
      tailwindRuntimeDependency: inventory.tailwindRuntimeDependency,
    };

    if (args.json) {
      process.stdout.write(`${JSON.stringify(summary, null, 2)}\n`);
    } else {
      process.stdout.write(
        `Ingested ${summary.candidateCount} Tailwind v4.3 source-test candidates from ${summary.candidateSourceFileCount}/${summary.sourceFileCount} files\n`,
      );
      process.stdout.write(
        `Indexed ${summary.officialFixtureCount} official Tailwind v4.3 inline-snapshot fixtures from ${summary.officialFixtureSourceFileCount} files\n`,
      );
    }
  } finally {
    if (tempSourceDir && !args.keepTemp) {
      fs.rmSync(tempSourceDir, { recursive: true, force: true });
    }
  }
}

main();
