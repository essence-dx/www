"use strict";

const { spawnSync } = require("node:child_process");
const fs = require("node:fs");
const {
  buildLane12CommitPlanReport,
  buildLane12OwnershipReport,
  buildLane12OwnerSummaryReport,
  buildLane12StagedOwnershipReport,
  buildLane12StageablePathsReport,
  summarizeOwnership,
} = require("./lane12-ownership-report.cjs");
const {
  classifyStatusLines,
} = require("./lane12-ownership-rules.cjs");

const CURRENT_GIT_STATUS_ARGS = Object.freeze([
  "--no-optional-locks",
  "status",
  "--short",
]);

const STAGED_GIT_STATUS_ARGS = Object.freeze([
  "--no-optional-locks",
  "diff",
  "--cached",
  "--name-status",
  "--find-renames",
]);

function runCli({
  argv = process.argv.slice(2),
  cwd = process.cwd(),
  stdin = process.stdin,
  stdout = process.stdout,
  stderr = process.stderr,
  readCurrentStatus = readCurrentGitStatus,
  readStagedStatus = readStagedGitStatus,
  readStdinStatus = readStatusTextFromStdin,
} = {}) {
  const strict = argv.includes("--strict");
  const current = argv.includes("--current");
  const staged = argv.includes("--staged");
  const compact = argv.includes("--compact");
  const stageableOnly = argv.includes("--stageable-only");
  const owners = argv.includes("--owners");
  const commitPlan = argv.includes("--commit-plan");
  const statusText = staged
    ? readStagedStatus(cwd)
    : current
      ? readCurrentStatus(cwd)
      : readDefaultStatusText({ cwd, stdin, readCurrentStatus, readStdinStatus });
  const reportOptions = {
    compact: compact && !stageableOnly && !owners,
    generatedAt: new Date().toISOString(),
  };
  const report = staged
    ? buildLane12StagedOwnershipReport(statusText, reportOptions)
    : buildLane12OwnershipReport(statusText, reportOptions);
  const output = stageableOnly
    ? buildLane12StageablePathsReport(report)
    : owners
      ? buildLane12OwnerSummaryReport(report)
      : commitPlan
        ? buildLane12CommitPlanReport(report)
        : report;
  stdout.write(`${JSON.stringify(output, null, 2)}\n`);

  if (strict && report.blockedLane12Commit) {
    stderr.write("Lane 12 strict check blocked mixed worktree ownership\n");
    process.exitCode = 2;
  }
}

function readDefaultStatusText({ cwd, stdin, readCurrentStatus, readStdinStatus }) {
  const stdinText = readStdinStatus(stdin);
  return stdinText.trim().length === 0 ? readCurrentStatus(cwd) : stdinText;
}

function readStatusTextFromStdin(stdin) {
  if (!stdin || typeof stdin.fd !== "number" || stdin.fd < 0 || stdin.isTTY) {
    return "";
  }
  return fs.readFileSync(stdin.fd, "utf8");
}

function readCurrentGitStatus(cwd, { spawn = spawnSync } = {}) {
  const result = spawn("git", CURRENT_GIT_STATUS_ARGS, {
    cwd,
    encoding: "utf8",
  });
  if (result.status !== 0) {
    throw new Error(
      `git ${CURRENT_GIT_STATUS_ARGS.join(" ")} failed with exit code ${result.status}: ${String(
        result.stderr || "",
      ).trim()}`,
    );
  }
  return result.stdout;
}

function readStagedGitStatus(cwd, { spawn = spawnSync } = {}) {
  const result = spawn("git", STAGED_GIT_STATUS_ARGS, {
    cwd,
    encoding: "utf8",
  });
  if (result.status !== 0) {
    throw new Error(
      `git ${STAGED_GIT_STATUS_ARGS.join(" ")} failed with exit code ${result.status}: ${String(
        result.stderr || "",
      ).trim()}`,
    );
  }
  return result.stdout;
}

if (require.main === module) {
  runCli();
}

module.exports = {
  buildLane12CommitPlanReport,
  buildLane12StagedOwnershipReport,
  buildLane12OwnershipReport,
  classifyStatusLines,
  readCurrentGitStatus,
  readStagedGitStatus,
  runCli,
  summarizeOwnership,
};
