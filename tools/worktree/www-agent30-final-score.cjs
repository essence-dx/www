"use strict";

const { spawnSync } = require("node:child_process");
const fs = require("node:fs");
const path = require("node:path");

const HARD_CAP_RULES = [
  {
    id: "dirty-worktree-chaos",
    cap: 84,
    applies: (evidence) => evidence.dirtyEntryCount >= 500,
    message: "dirty worktree has 500 or more entries",
    nextAction: "Curate or quarantine dirty entries before raising the final score.",
  },
  {
    id: "worktree-ownership-blockers",
    cap: 83,
    applies: (evidence) => evidence.worktreeBlockingRiskCount > 0,
    message: "worktree ownership map still has blocking risks",
    nextAction: "Resolve unclassified or blocking ownership risks before raising the final score.",
  },
  {
    id: "dirty-worktree-present",
    cap: 92,
    applies: (evidence) => evidence.dirtyEntryCount > 0 && evidence.dirtyEntryCount < 500,
    message: "dirty worktree still has uncommitted entries",
    nextAction: "Reduce the dirty worktree to reviewed release-control slices.",
  },
  {
    id: "generated-artifacts-unreviewed",
    cap: 88,
    applies: (evidence) => evidence.generatedArtifactsReviewed !== true,
    message: "generated artifacts have not been reviewed or quarantined",
    nextAction: "Review generated .dx, receipt, report, and temp artifacts.",
  },
  {
    id: "rust-warnings-present",
    cap: 89,
    applies: (evidence) => evidence.rustWarningCount > 50,
    message: "Rust build still emits more than 50 warnings",
    nextAction: "Reduce Rust warning count before claiming a 90+ integration score.",
  },
  {
    id: "rust-warnings-nonzero",
    cap: 92,
    applies: (evidence) => evidence.rustWarningCount > 0 && evidence.rustWarningCount <= 50,
    message: "Rust build still emits warnings",
    nextAction: "Drive the warning count to zero or keep the score capped.",
  },
  {
    id: "diff-check-not-clean",
    cap: 80,
    applies: (evidence) => evidence.diffCheckPassed !== true,
    message: "git diff --check has not passed",
    nextAction: "Run and fix git diff --check before trusting release scoring.",
  },
  {
    id: "conflict-scan-not-clean",
    cap: 80,
    applies: (evidence) => evidence.conflictMarkerScanPassed !== true,
    message: "conflict-marker scan has not passed",
    nextAction: "Run and fix the conflict-marker scan before trusting release scoring.",
  },
  {
    id: "cargo-check-missing",
    cap: 86,
    applies: (evidence) => evidence.cargoCheckPassed !== true,
    message: "focused cargo check proof is missing or failed",
    nextAction: "Run the focused dx-www cargo check with low concurrency.",
  },
  {
    id: "cargo-build-missing",
    cap: 88,
    applies: (evidence) => evidence.cargoBuildPassed !== true,
    message: "focused cargo build proof is missing or failed",
    nextAction: "Run the focused dx-www cargo build before release scoring.",
  },
  {
    id: "dx-build-missing",
    cap: 90,
    applies: (evidence) => evidence.dxBuildPassed !== true,
    message: "real dx build proof is missing or failed",
    nextAction: "Run real dx build in the launch template and inspect .dx/build output.",
  },
  {
    id: "focused-suite-missing",
    cap: 90,
    applies: (evidence) => evidence.focusedSuitePassed !== true,
    message: "focused suite summary is missing or failed",
    nextAction: "Run the focused suites that own the current integration diff.",
  },
  {
    id: "http-proof-missing",
    cap: 91,
    applies: (evidence) => evidence.httpProofPassed !== true,
    message: "HTTP proof is missing or failed",
    nextAction: "Probe the running dev server before raising the score.",
  },
  {
    id: "browser-proof-missing",
    cap: 93,
    applies: (evidence) => evidence.browserProofPassed !== true,
    message: "browser screenshot proof is missing or failed",
    nextAction: "Capture browser proof for the key routes.",
  },
  {
    id: "overlay-recovery-proof-missing",
    cap: 94,
    applies: (evidence) => evidence.overlayRecoveryProofPassed !== true,
    message: "diagnostics overlay recovery proof is missing or failed",
    nextAction: "Prove error overlay recovery before claiming final readiness.",
  },
  {
    id: "full-cargo-test-missing",
    cap: 94,
    applies: (evidence) => evidence.fullCargoTestPassed !== true,
    message: "full or accepted cargo test proof is missing",
    nextAction: "Run the strongest practical cargo test or record why it remains blocked.",
  },
  {
    id: "clippy-missing",
    cap: 94,
    applies: (evidence) => evidence.clippyPassed !== true,
    message: "cargo clippy proof is missing or failed",
    nextAction: "Run focused clippy after warning cleanup makes it practical.",
  },
];

function scoreFinalIntegration(input = {}) {
  const evidence = normalizeEvidence(input);
  const hardCaps = HARD_CAP_RULES.map((rule) => ({
    id: rule.id,
    applied: rule.applies(evidence),
    cap: rule.cap,
    message: rule.message,
    nextAction: rule.nextAction,
  }));
  const appliedCaps = hardCaps.filter((cap) => cap.applied);
  const score = appliedCaps.reduce((current, cap) => Math.min(current, cap.cap), 100);
  const scoreTable = hardCaps.map((cap) => ({
    id: cap.id,
    status: cap.applied ? "applied" : "clear",
    cap: cap.cap,
    scoreImpact: cap.applied ? `caps at ${cap.cap}` : "none",
    issue: cap.message,
    nextAction: cap.nextAction,
  }));

  return {
    schema: "dx.www.finalIntegrationScore",
    format: 1,
    status: score === 100 ? "ready" : score >= 95 ? "candidate" : "blocked",
    score,
    maxScore: 100,
    evidence,
    hardCaps,
    scoreTable,
    remainingBlockerIds: appliedCaps.map((cap) => cap.id),
    nextAction: appliedCaps[0]?.nextAction || "Keep final integration evidence current.",
  };
}

function evidenceFromReadinessGateReport(report = {}) {
  const steps = Array.isArray(report.proofBundle?.steps) ? report.proofBundle.steps : [];
  const cargoCheckStep = findProofStep(steps, "cargo-check-dx-www-cli");
  const dxBuildStep = findProofStep(steps, "dx-build-installed-smoke");

  return {
    cargoCheckPassed: proofStepPassed(cargoCheckStep),
    dxBuildPassed: proofStepPassed(dxBuildStep),
    focusedSuitePassed: proofStepPassed(findProofStep(steps, "focused-readiness-node-test")),
    httpProofPassed: httpProofPassed(steps),
    rustWarningCount: rustWarningCountFromStep(cargoCheckStep),
  };
}

function normalizeEvidence(input) {
  return {
    cargoBuildPassed: input.cargoBuildPassed === true,
    cargoCheckPassed: input.cargoCheckPassed === true,
    clippyPassed: input.clippyPassed === true,
    conflictMarkerScanPassed: input.conflictMarkerScanPassed === true,
    diffCheckPassed: input.diffCheckPassed === true,
    dirtyEntryCount: numberOrZero(input.dirtyEntryCount),
    dxBuildPassed: input.dxBuildPassed === true,
    focusedSuitePassed: input.focusedSuitePassed === true,
    fullCargoTestPassed: input.fullCargoTestPassed === true,
    generatedArtifactsReviewed: input.generatedArtifactsReviewed === true,
    httpProofPassed: input.httpProofPassed === true,
    rustWarningCount: numberOrZero(input.rustWarningCount),
    worktreeBlockingRiskCount: numberOrZero(input.worktreeBlockingRiskCount),
    browserProofPassed: input.browserProofPassed === true,
    overlayRecoveryProofPassed: input.overlayRecoveryProofPassed === true,
  };
}

function numberOrZero(value) {
  const number = Number(value);
  return Number.isFinite(number) && number > 0 ? number : 0;
}

function findProofStep(steps, id) {
  return steps.find((step) => step && step.id === id) || null;
}

function proofStepPassed(step) {
  return step?.passed === true && step?.status === "passed";
}

function httpProofPassed(steps) {
  const httpSteps = steps.filter((step) => step?.kind === "http-probe");
  return httpSteps.length > 0 && httpSteps.every(proofStepPassed);
}

function rustWarningCountFromStep(step) {
  if (!step) {
    return 0;
  }
  return rustWarningCountFromText(`${step.stderrTail || ""}\n${step.stdoutTail || ""}`);
}

function rustWarningCountFromText(text) {
  let warningCount = 0;
  for (const match of text.matchAll(/\bgenerated\s+(\d+)\s+warnings?\b/g)) {
    warningCount = Math.max(warningCount, Number(match[1]));
  }
  const explicitWarnings = text
    .split(/\r?\n/)
    .filter((line) => /^warning(?:\[[^\]]+\])?:/.test(line))
    .filter((line) => !/\bgenerated\s+\d+\s+warnings?\b/.test(line));
  return Math.max(warningCount, explicitWarnings.length);
}

function collectWorktreeEvidence(repoRoot = process.cwd(), runner = spawnSync) {
  const status = runner("git", ["status", "--short"], {
    cwd: repoRoot,
    encoding: "utf8",
    windowsHide: true,
  });
  const diffCheck = runner("git", ["diff", "--check"], {
    cwd: repoRoot,
    encoding: "utf8",
    windowsHide: true,
  });
  const conflictScan = runner(
    "rg",
    [
      "-n",
      "^(<<<<<<<|=======|>>>>>>>)",
      "--glob",
      "!vendor/**",
      "--glob",
      "!target/**",
      "--glob",
      "!target-*/**",
      "--glob",
      "!node_modules/**",
      "--glob",
      "!.git/**",
      ".",
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
      windowsHide: true,
    },
  );

  return worktreeEvidenceFromCommandResults({
    conflictScanStatus: conflictScan.status,
    diffCheckStatus: diffCheck.status,
    statusStatus: status.status,
    statusStdout: status.stdout,
  });
}

function worktreeEvidenceFromCommandResults(results = {}) {
  return {
    conflictMarkerScanPassed: results.conflictScanStatus === 1,
    diffCheckPassed: results.diffCheckStatus === 0,
    dirtyEntryCount:
      results.statusStatus === 0
        ? String(results.statusStdout || "")
            .split(/\r?\n/)
            .filter((line) => line.trim().length > 0).length
        : 0,
  };
}

function main(argv = process.argv.slice(2)) {
  try {
    const options = parseArgs(argv);
    const reportEvidence = options.readinessReportPath
      ? evidenceFromReadinessGateReport(readJsonFile(options.readinessReportPath))
      : {};
    const worktreeEvidence = options.worktree ? collectWorktreeEvidence(options.repoRoot) : {};
    const fileEvidence = options.evidencePath ? readJsonFile(options.evidencePath) : {};
    const evidence = {
      ...reportEvidence,
      ...worktreeEvidence,
      ...fileEvidence,
    };
    const report = scoreFinalIntegration(evidence);

    if (options.json) {
      process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
    } else {
      printHuman(report);
    }

    process.exitCode = report.status === "ready" ? 0 : 1;
  } catch (error) {
    process.stderr.write(`${error.message}\n`);
    process.exitCode = 2;
  }
}

function parseArgs(argv) {
  const options = {
    evidencePath: null,
    readinessReportPath: null,
    repoRoot: process.cwd(),
    worktree: false,
    json: false,
  };

  for (let index = 0; index < argv.length; ) {
    const arg = argv[index];
    if (arg === "--evidence") {
      options.evidencePath = path.resolve(requireValue(argv, index, arg));
      index += 2;
      continue;
    }
    if (arg === "--readiness-report") {
      options.readinessReportPath = path.resolve(requireValue(argv, index, arg));
      index += 2;
      continue;
    }
    if (arg === "--repo") {
      options.repoRoot = path.resolve(requireValue(argv, index, arg));
      index += 2;
      continue;
    }
    if (arg === "--worktree") {
      options.worktree = true;
      index += 1;
      continue;
    }
    if (arg === "--json") {
      options.json = true;
      index += 1;
      continue;
    }
    if (arg === "--help" || arg === "-h") {
      printUsage();
      process.exit(0);
    }
    throw new Error(`Unknown option: ${arg}`);
  }

  if (!options.evidencePath && !options.readinessReportPath && !options.worktree) {
    throw new Error("--evidence, --readiness-report, or --worktree is required");
  }

  return options;
}

function requireValue(argv, index, flag) {
  const value = argv[index + 1];
  if (!value) {
    throw new Error(`${flag} requires a value`);
  }
  return value;
}

function readJsonFile(filePath) {
  return JSON.parse(fs.readFileSync(filePath, "utf8"));
}

function printHuman(report) {
  process.stdout.write(`DX-WWW final integration score: ${report.score} / 100 (${report.status})\n`);
  process.stdout.write("Score cap table:\n");
  for (const row of report.scoreTable || []) {
    process.stdout.write(`- ${row.status.toUpperCase()} ${row.id}: ${row.scoreImpact}; ${row.issue}\n`);
  }
  for (const id of report.remainingBlockerIds) {
    process.stdout.write(`- ${id}\n`);
  }
  process.stdout.write(`Next action: ${report.nextAction}\n`);
}

function printUsage() {
  process.stdout.write(
    [
      "Usage: node tools/worktree/www-agent30-final-score.cjs [--evidence <file>] [--readiness-report <file>] [--worktree] [--repo <path>] [--json]",
      "",
      "Scores final integration evidence with hard caps.",
      "Use --readiness-report to derive proof-bundle fields, and --evidence to provide or override worktree fields.",
      "Use --worktree to collect dirty-entry, git diff --check, and conflict-marker evidence from --repo.",
      "",
    ].join("\n"),
  );
}

if (require.main === module) {
  main();
}

module.exports = {
  HARD_CAP_RULES,
  collectWorktreeEvidence,
  evidenceFromReadinessGateReport,
  main,
  normalizeEvidence,
  parseArgs,
  rustWarningCountFromText,
  scoreFinalIntegration,
  worktreeEvidenceFromCommandResults,
};
