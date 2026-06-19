const { spawnSync } = require("node:child_process");
const fs = require("node:fs");
const path = require("node:path");

const {
  COORDINATOR_CHECKS,
  commandText,
  coordinatorCheckSummary,
  heavyCommandViolations,
} = require("./coordinator-checks.cjs");
const {
  PREFLIGHT_SCHEMA,
  buildCoordinatorPreflightReport,
} = require("./coordinator-preflight.cjs");
const {
  coordinatorArchitecture,
  coordinatorProofContract,
  coordinatorReceiptWritePolicy,
} = require("./coordinator-report-contract.cjs");
const {
  READINESS_AUDIT_SCHEMA,
  buildReadinessAuditReport,
  readDefaultReadinessAuditInput,
  readReadinessAuditInput,
} = require("./coordinator-readiness-audit.cjs");
const {
  AUDIT_COMPARISON_SCHEMA,
  AUDIT_COMPARISON_OPEN_GAPS_SCHEMA,
  compactAuditComparisonOpenGaps,
  compareAuditBaselineToCoordinatorRun,
} = require("./coordinator-audit-comparison.cjs");

const RUN_SCHEMA = "dx.nextRustMerge.coordinatorRun";
const FAILURES_SCHEMA = "dx.nextRustMerge.coordinatorFailures";
const ARGUMENT_ERROR_SCHEMA = "dx.nextRustMerge.coordinatorArgumentError";
const DEFAULT_COORDINATOR_RECEIPT_PATH =
  ".dx/receipts/next-rust/coordinator-scorecard.json";

function runCoordinatorChecks({
  checks = COORDINATOR_CHECKS,
  cwd = process.cwd(),
  runCommand = spawnCheck,
  clock = () => Date.now(),
  startedAt = new Date().toISOString(),
} = {}) {
  const results = [];

  for (const entry of checks) {
    const began = clock();
    const violations = heavyCommandViolations(entry);
    const output =
      violations.length > 0
        ? {
            status: 1,
            stdout: "",
            stderr: `blocked heavy command: ${violations.join(", ")}`,
            signal: null,
          }
        : runCommand(entry.command, { cwd });
    const durationMs = Math.max(0, clock() - began);
    const exitCode = typeof output.status === "number" ? output.status : 1;

    results.push({
      id: entry.id,
      lane: entry.lane,
      boundary: entry.boundary,
      blocking: entry.blocking,
      command: commandText(entry),
      status: exitCode === 0 ? "passed" : "failed",
      exitCode,
      signal: output.signal || null,
      durationMs,
      failureSummary:
        exitCode === 0 ? null : summarizeFailure(output.stdout, output.stderr, exitCode),
      stdoutPreview: preview(output.stdout),
      stderrPreview: preview(output.stderr),
      proves: entry.proves,
      healthContract: entry.healthContract || null,
      sideEffects: entry.sideEffects,
    });
  }

  return buildCoordinatorRunReport(results, {
    startedAt,
    finishedAt: new Date().toISOString(),
  });
}

function buildCoordinatorRunReport(results, timing) {
  const total = results.length;
  const failed = results.filter((entry) => entry.status !== "passed");
  const blockingFailed = failed.filter((entry) => entry.blocking);
  const nonBlockingFailed = failed.filter((entry) => !entry.blocking);
  const score = scoreCoordinatorRun({
    total,
    blockingFailed: blockingFailed.length,
    nonBlockingFailed: nonBlockingFailed.length,
  });
  const proofContract = coordinatorProofContract("run");

  return {
    schema: RUN_SCHEMA,
    lane: 14,
    laneName: "Final Coordinator",
    featureImplementation: false,
    ...proofContract,
    status:
      blockingFailed.length > 0
        ? "blocked"
        : nonBlockingFailed.length > 0
          ? "warning"
          : "passing",
    score,
    totals: {
      total,
      passed: total - failed.length,
      failed: failed.length,
      blockingFailed: blockingFailed.length,
      nonBlockingFailed: nonBlockingFailed.length,
    },
    architecture: coordinatorArchitecture(),
    receiptWritePolicy: coordinatorReceiptWritePolicy({
      checks: results,
      executionMode: "run",
      writesDefaultScorecard: false,
    }),
    ...timing,
    checks: results,
  };
}

function scoreCoordinatorRun({ total, blockingFailed, nonBlockingFailed }) {
  if (total === 0) return 0;
  const penalty = blockingFailed * 12 + nonBlockingFailed * 4;
  return Math.max(0, Math.min(100, 100 - penalty));
}

function compactCoordinatorFailures(report) {
  const checks = Array.isArray(report.checks) ? report.checks : [];
  const failures = checks
    .filter((entry) => entry.status && entry.status !== "passed")
    .map((entry) => ({
      id: entry.id,
      lane: entry.lane,
      boundary: entry.boundary,
      blocking: entry.blocking,
      command: entry.command,
      status: entry.status,
      exitCode: entry.exitCode,
      signal: entry.signal,
      durationMs: entry.durationMs,
      failureSummary: entry.failureSummary,
      proves: entry.proves,
      sideEffects: entry.sideEffects,
    }));

  return {
    schema: FAILURES_SCHEMA,
    generatedFromSchema: report.schema,
    lane: report.lane,
    laneName: report.laneName,
    featureImplementation: false,
    executionMode: report.executionMode || "run",
    proofLevel: report.proofLevel || "executed-lightweight-source-checks",
    status: report.status || "not-run",
    score: report.score ?? null,
    totals:
      report.totals ||
      {
        total: checks.length,
        passed: checks.length - failures.length,
        failed: failures.length,
        blockingFailed: failures.filter((entry) => entry.blocking).length,
        nonBlockingFailed: failures.filter((entry) => !entry.blocking).length,
      },
    failureCount: failures.length,
    blockingFailureCount: failures.filter((entry) => entry.blocking).length,
    architecture: report.architecture || coordinatorArchitecture(),
    receiptWritePolicy:
      report.receiptWritePolicy ||
      coordinatorReceiptWritePolicy({
        checks,
        executionMode: "run",
        writesDefaultScorecard: false,
      }),
    startedAt: report.startedAt,
    finishedAt: report.finishedAt,
    failures,
  };
}

function resolveCoordinatorChecks({ checks = COORDINATOR_CHECKS, only = null } = {}) {
  if (!only) return checks;

  const selected = checks.filter((entry) => entry.id === only);
  if (selected.length > 0) return selected;

  const error = new Error(`Unknown coordinator check id "${only}"`);
  error.code = "DX_NEXT_RUST_UNKNOWN_CHECK";
  error.unknownCheckId = only;
  error.availableCheckIds = checks.map((entry) => entry.id);
  throw error;
}

function buildCoordinatorArgumentError(error) {
  return {
    schema: ARGUMENT_ERROR_SCHEMA,
    lane: 14,
    laneName: "Final Coordinator",
    featureImplementation: false,
    status: "argument-error",
    exitCode: 2,
    message: error.message,
    code: error.code || "DX_NEXT_RUST_ARGUMENT_ERROR",
    requestedCheckId: error.unknownCheckId || null,
    availableCheckIds: error.availableCheckIds || [],
    architecture: coordinatorArchitecture(),
  };
}

function spawnCheck(command, { cwd }) {
  return spawnSync(command[0], command.slice(1), {
    cwd,
    encoding: "utf8",
    windowsHide: true,
  });
}

function summarizeFailure(stdout, stderr, exitCode) {
  const lines = `${stdout || ""}\n${stderr || ""}`
    .split(/\r?\n/)
    .map((line) => line.trim())
    .filter(Boolean);
  return (
    lines.find((line) => line.startsWith("not ok ")) ||
    lines.find((line) => /^error:/i.test(line)) ||
    lines.find((line) => /ERR_|AssertionError|TypeError/.test(line)) ||
    lines[0] ||
    `command exited ${exitCode}`
  );
}

function preview(value, maxLength = 700) {
  const text = String(value || "").trim();
  if (text.length <= maxLength) return text;
  return `${text.slice(0, maxLength)}...`;
}

function writeCoordinatorReport(report, targetPath, { cwd = process.cwd() } = {}) {
  if (!targetPath || typeof targetPath !== "string") {
    throw new Error("--write requires an explicit file path");
  }
  const resolvedPath = path.resolve(cwd, targetPath);
  const reportWithPath = {
    ...report,
    receiptPath: resolvedPath,
  };

  fs.mkdirSync(path.dirname(resolvedPath), { recursive: true });
  fs.writeFileSync(resolvedPath, `${JSON.stringify(reportWithPath, null, 2)}\n`);
  return reportWithPath;
}

function resolveCoordinatorReceiptPath({ writePath = null, writeDefault = false } = {}) {
  if (writePath) return writePath;
  return writeDefault ? DEFAULT_COORDINATOR_RECEIPT_PATH : null;
}

function parseArgs(argv) {
  const audit = argv.includes("--audit");
  const run = argv.includes("--run");
  const json = argv.includes("--json");
  const preflight = argv.includes("--preflight") || argv.includes("--dry-run");
  const writeDefault = argv.includes("--write-default");
  const failuresOnly = argv.includes("--failures-only");
  const confirmedOpenOnly =
    argv.includes("--confirmed-open-only") || argv.includes("--open-gaps-only");
  const auditBaseline = argv.includes("--audit-baseline");
  const compareAuditBaseline = argv.includes("--compare-audit-baseline");
  const auditFileIndex = argv.indexOf("--audit-file");
  const auditFile = auditFileIndex >= 0 ? argv[auditFileIndex + 1] : null;
  const onlyIndex = argv.indexOf("--only");
  const only = onlyIndex >= 0 ? argv[onlyIndex + 1] : null;
  const writeIndex = argv.indexOf("--write");
  const writePath = writeIndex >= 0 ? argv[writeIndex + 1] : null;
  return {
    audit,
    auditBaseline,
    auditFile,
    compareAuditBaseline,
    confirmedOpenOnly,
    run,
    json,
    preflight,
    failuresOnly,
    only,
    writePath: resolveCoordinatorReceiptPath({ writePath, writeDefault }),
    writeDefault,
  };
}

function main(argv = process.argv.slice(2)) {
  const args = parseArgs(argv);
  const summary = coordinatorCheckSummary();
  let checks;
  try {
    checks = resolveCoordinatorChecks({ checks: summary.checks, only: args.only });
  } catch (error) {
    const output = buildCoordinatorArgumentError(error);
    if (args.json) {
      process.stdout.write(`${JSON.stringify(output, null, 2)}\n`);
    } else {
      process.stderr.write(`${output.message}\n`);
      process.stderr.write(`available checks: ${output.availableCheckIds.join(", ")}\n`);
    }
    process.exitCode = output.exitCode;
    return;
  }

  let auditInput = null;
  if (
    (args.audit || args.compareAuditBaseline) &&
    (args.auditFile || args.auditBaseline || args.compareAuditBaseline)
  ) {
    try {
      auditInput = args.auditFile
        ? readReadinessAuditInput(args.auditFile)
        : readDefaultReadinessAuditInput();
    } catch (error) {
      const output = buildCoordinatorArgumentError(error);
      if (args.json) {
        process.stdout.write(`${JSON.stringify(output, null, 2)}\n`);
      } else {
        process.stderr.write(`${output.message}\n`);
      }
      process.exitCode = output.exitCode;
      return;
    }
  }

  let output = args.compareAuditBaseline
    ? compareAuditBaselineToCoordinatorRun({
        auditInput,
        coordinatorReport: runCoordinatorChecks({ checks }),
      })
    : args.audit
    ? buildReadinessAuditReport({
        coordinatorReport: args.run
          ? runCoordinatorChecks({ checks })
          : buildCoordinatorPreflightReport({ checks }),
        auditInput: auditInput || undefined,
      })
    : args.preflight
      ? buildCoordinatorPreflightReport({ checks })
      : args.run
        ? runCoordinatorChecks({ checks })
        : { ...summary, checks };
  if (args.failuresOnly && !args.preflight && !args.audit) {
    output = compactCoordinatorFailures(output);
  }
  if (args.confirmedOpenOnly && output.schema === AUDIT_COMPARISON_SCHEMA) {
    output = compactAuditComparisonOpenGaps(output);
  }
  if (args.writePath) {
    output = writeCoordinatorReport(output, args.writePath);
  }
  if (args.json) {
    process.stdout.write(`${JSON.stringify(output, null, 2)}\n`);
    process.exitCode =
      output.status === "blocked" || output.status === "needs-review" ? 1 : 0;
    return;
  }

  const printableEntries = output.failures || output.checks;
  if (args.failuresOnly && printableEntries.length === 0) {
    process.stdout.write("failures: none\n");
  }
  for (const entry of printableEntries) {
    const prefix = entry.status ? `${entry.status}:` : "check:";
    process.stdout.write(`${prefix} ${entry.id} ${entry.command}\n`);
  }
  if (output.score !== undefined) {
    process.stdout.write(`score: ${output.score}/100\n`);
  }
  process.exitCode = output.status === "blocked" ? 1 : 0;
}

if (require.main === module) {
  main();
}

module.exports = {
  ARGUMENT_ERROR_SCHEMA,
  AUDIT_COMPARISON_SCHEMA,
  AUDIT_COMPARISON_OPEN_GAPS_SCHEMA,
  READINESS_AUDIT_SCHEMA,
  DEFAULT_COORDINATOR_RECEIPT_PATH,
  FAILURES_SCHEMA,
  PREFLIGHT_SCHEMA,
  RUN_SCHEMA,
  buildCoordinatorArgumentError,
  buildReadinessAuditReport,
  compactAuditComparisonOpenGaps,
  compareAuditBaselineToCoordinatorRun,
  buildCoordinatorPreflightReport,
  buildCoordinatorRunReport,
  readDefaultReadinessAuditInput,
  readReadinessAuditInput,
  compactCoordinatorFailures,
  main,
  parseArgs,
  resolveCoordinatorChecks,
  resolveCoordinatorReceiptPath,
  runCoordinatorChecks,
  scoreCoordinatorRun,
  summarizeFailure,
  writeCoordinatorReport,
};
