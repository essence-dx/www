const fs = require("node:fs");
const path = require("node:path");

const GIANT_CLI_MOD_SCHEMA = "dx.nextRustMerge.giantCliModCheck";
const DEFAULT_GIANT_CLI_MOD_FILE = "dx-www/src/cli/mod.rs";

const DEFAULT_GIANT_CLI_MOD_LIMITS = Object.freeze({
  maxLineCount: 3_000,
  maxDeclarationCount: 250,
  maxByteCount: 350_000,
});

function runGiantCliModCheck({
  cwd = process.cwd(),
  file = DEFAULT_GIANT_CLI_MOD_FILE,
  limits = DEFAULT_GIANT_CLI_MOD_LIMITS,
} = {}) {
  const resolvedFile = path.resolve(cwd, file);
  const metrics = readRustFileMetrics(resolvedFile);
  const normalizedLimits = normalizeLimits(limits);
  const violations = buildViolations({ file: resolvedFile, metrics, limits: normalizedLimits });

  return {
    schema: GIANT_CLI_MOD_SCHEMA,
    lane: 14,
    laneName: "Final Coordinator",
    featureImplementation: false,
    proofLevel: "read-only-cli-size-risk-scan",
    status: violations.length === 0 ? "passing" : "risk-open",
    file: resolvedFile,
    limits: normalizedLimits,
    metrics,
    violations,
    sideEffects: [],
  };
}

function readRustFileMetrics(filePath) {
  if (!fs.existsSync(filePath)) {
    return {
      exists: false,
      lineCount: 0,
      byteCount: 0,
      declarationCount: 0,
      moduleDeclarationCount: 0,
    };
  }

  const text = fs.readFileSync(filePath, "utf8");
  const lines = text.length === 0 ? [] : text.split(/\r?\n/);

  return {
    exists: true,
    lineCount: lines.length,
    byteCount: Buffer.byteLength(text, "utf8"),
    declarationCount: countMatchingLines(
      lines,
      /^\s*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?(?:fn|struct|enum|impl|trait|mod)\b/,
    ),
    moduleDeclarationCount: countMatchingLines(
      lines,
      /^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+[A-Za-z_][A-Za-z0-9_]*\s*;/,
    ),
  };
}

function buildViolations({ file, metrics, limits }) {
  if (!metrics.exists) {
    return [
      {
        id: "missing-cli-mod",
        file,
        actual: 0,
        limit: 1,
        message: "Expected dx-www/src/cli/mod.rs to exist before assessing split risk.",
      },
    ];
  }

  return [
    metricViolation({
      id: "line-count",
      file,
      actual: metrics.lineCount,
      limit: limits.maxLineCount,
      message: "dx-www/src/cli/mod.rs is above the coordinator split-risk line threshold.",
    }),
    metricViolation({
      id: "declaration-count",
      file,
      actual: metrics.declarationCount,
      limit: limits.maxDeclarationCount,
      message: "dx-www/src/cli/mod.rs has too many declarations for safe lane coordination.",
    }),
    metricViolation({
      id: "byte-count",
      file,
      actual: metrics.byteCount,
      limit: limits.maxByteCount,
      message: "dx-www/src/cli/mod.rs is above the coordinator split-risk byte threshold.",
    }),
  ].filter(Boolean);
}

function metricViolation({ id, file, actual, limit, message }) {
  if (actual <= limit) return null;
  return { id, file, actual, limit, message };
}

function countMatchingLines(lines, pattern) {
  return lines.reduce((count, line) => count + (pattern.test(line) ? 1 : 0), 0);
}

function normalizeLimits(limits) {
  return {
    maxLineCount: positiveInteger(limits.maxLineCount, "maxLineCount"),
    maxDeclarationCount: positiveInteger(
      limits.maxDeclarationCount,
      "maxDeclarationCount",
    ),
    maxByteCount: positiveInteger(limits.maxByteCount, "maxByteCount"),
  };
}

function positiveInteger(value, fieldName) {
  if (!Number.isInteger(value) || value <= 0) {
    throw new Error(`${fieldName} must be a positive integer`);
  }
  return value;
}

if (require.main === module) {
  const report = runGiantCliModCheck();

  if (process.argv.includes("--json")) {
    process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
  } else if (report.status === "passing") {
    process.stdout.write("giant cli mod check passed\n");
  } else {
    for (const violation of report.violations) {
      process.stdout.write(
        `${violation.file} ${violation.id}: ${violation.actual} > ${violation.limit}\n`,
      );
    }
  }

  process.exitCode = report.status === "passing" ? 0 : 1;
}

module.exports = {
  DEFAULT_GIANT_CLI_MOD_FILE,
  DEFAULT_GIANT_CLI_MOD_LIMITS,
  GIANT_CLI_MOD_SCHEMA,
  runGiantCliModCheck,
};
