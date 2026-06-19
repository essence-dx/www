const { spawnSync } = require("node:child_process");

const { runHttpProbe } = require("./proof-http-probe.ts");

function runProofSteps(repoRoot, steps, createBundle) {
  const started = Date.now();
  const results = [];
  let failedStep = null;

  for (const step of steps) {
    const result =
      failedStep && step.writesReceipts
        ? skipBlockedReceiptWrite(step, failedStep)
        : runStep(repoRoot, step);
    results.push(result);
    if (!failedStep && result.status === "failed") {
      failedStep = result;
    }
  }

  return {
    ...createBundle(),
    durationMs: Date.now() - started,
    steps: results,
    summary: summarizeSteps(results, "run"),
  };
}

function runStep(repoRoot, step) {
  if (step.kind === "http-probe") {
    return runHttpProbe(repoRoot, step);
  }
  return runCommand(repoRoot, step);
}

function runCommand(repoRoot, step) {
  const started = Date.now();
  const result = spawnSync(step.executable, step.args, {
    cwd: repoRoot,
    encoding: "utf8",
    timeout: step.timeoutMs,
  });
  const timedOut = result.error?.code === "ETIMEDOUT";
  const passed = result.status === 0;

  return {
    ...publicStep(step),
    durationMs: Date.now() - started,
    exitCode: result.status,
    timeoutMs: step.timeoutMs,
    timedOut,
    failureReason: commandFailureReason({ passed, timedOut, result }),
    passed,
    status: passed ? "passed" : "failed",
    stderrTail: tail(result.stderr),
    stdoutTail: tail(result.stdout),
  };
}

function commandFailureReason({ passed, timedOut, result }) {
  if (passed) {
    return null;
  }
  if (timedOut) {
    return "timeout";
  }
  if (result.error) {
    return "spawn-error";
  }
  return "exit-code";
}

function skipBlockedReceiptWrite(step, failedStep) {
  return {
    ...publicStep(step),
    durationMs: 0,
    exitCode: null,
    passed: false,
    status: "skipped",
    skipReason: "blocked-by-prior-failure",
    blockedByStep: failedStep.id,
    stderrTail: "",
    stdoutTail: "",
  };
}

function publicStep(step) {
  const { args, executable, timeoutMs, ...publicFields } = step;
  return publicFields;
}

function summarizeSteps(steps, mode) {
  const executed = mode === "run" ? steps.filter((step) => step.status !== "skipped").length : 0;
  return {
    total: steps.length,
    executed,
    passed: steps.filter((step) => step.status === "passed").length,
    failed: steps.filter((step) => step.status === "failed").length,
    skipped: steps.filter((step) => step.status === "skipped").length,
  };
}

function tail(value) {
  return value ? value.slice(-4000) : "";
}

module.exports = {
  publicStep,
  runProofSteps,
  summarizeSteps,
};
