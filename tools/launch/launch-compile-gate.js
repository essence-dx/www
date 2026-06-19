#!/usr/bin/env node

const { spawnSync } = require("node:child_process");
const path = require("node:path");

const SCHEMA = "dx.www.launch.compileGate";
const FORMAT = 1;
const DEFAULT_TIMEOUT_MS = 300000;
const DEFAULT_ROUTE_BASE_URL = "http://127.0.0.1:3000";
const DEFAULT_ROUTE_RETRY_COUNT = 0;
const DEFAULT_ROUTE_RETRY_DELAY_MS = 250;
const DEFAULT_ROUTE_TIMEOUT_MS = 30000;
const DEFAULT_RUST_IDLE_POLL_MS = 250;
const GIT_LINE_ENDING_WARNING_PATTERN =
  /^warning: in the working copy of '.+', (?:LF will be replaced by CRLF|CRLF will be replaced by LF) the next time Git touches it$/;

const COMMANDS = Object.freeze([
  {
    id: "dx-www-cli-dev-server-check",
    kind: "cargo-check",
    expectedExitCodes: [0],
    args: [
      "cargo",
      "check",
      "-q",
      "-p",
      "dx-www",
      "--no-default-features",
      "--features",
      "cli,dev-server",
    ],
  },
  {
    id: "dx-www-cli-dev-server-binary-build",
    kind: "cargo-build",
    expectedExitCodes: [0],
    args: [
      "cargo",
      "build",
      "-q",
      "-p",
      "dx-www",
      "--no-default-features",
      "--features",
      "cli,dev-server",
    ],
  },
  {
    id: "launch-readiness-bundle-unit",
    kind: "cargo-test",
    expectedExitCodes: [0],
    args: [
      "cargo",
      "test",
      "-q",
      "-p",
      "dx-www",
      "launch_readiness_reports_green_without_runtime_artifacts",
      "--lib",
      "--no-default-features",
      "--features",
      "cli,dev-server",
    ],
  },
  {
    id: "launch-gate-node-unit",
    kind: "node-test",
    expectedExitCodes: [0],
    args: [
      "node",
      "--test",
      "benchmarks/launch-compile-gate.test.ts",
      "benchmarks/launch-readiness-gate.test.ts",
    ],
  },
  {
    id: "diff-whitespace-check",
    kind: "git-check",
    expectedExitCodes: [0],
    args: ["git", "diff", "--check"],
  },
  {
    id: "launch-gate-source-whitespace-scan",
    kind: "source-scan",
    expectedExitCodes: [1],
    args: [
      "rg",
      "-n",
      "[ \\t]+$",
      "tools/launch/launch-compile-gate.js",
      "tools/launch/launch-route-smoke.js",
      "benchmarks/launch-compile-gate.test.ts",
    ],
  },
  {
    id: "conflict-marker-scan",
    kind: "source-scan",
    expectedExitCodes: [1],
    args: [
      "rg",
      "-n",
      "^(<<<<<<<|=======|>>>>>>>)",
      "dx-www/src",
      "dx-www/tests",
      "benchmarks",
      "tools/launch",
    ],
  },
]);

function main(argv) {
  const options = parseArgs(argv);
  const cwd = path.resolve(options.project || process.cwd());
  const commands = commandSet(options);
  const selected = commands.filter(
    (command) => options.only.size === 0 || options.only.has(command.id),
  );

  if (selected.length === 0) {
    process.stderr.write("No launch compile gate command matched --only.\n");
    process.exitCode = 2;
    return;
  }

  const listed = selected.map(commandSummary);
  if (options.list) {
    writeReport(options, {
      schema: SCHEMA,
      format: FORMAT,
      executionMode: "list",
      project: cwd,
      timeoutMs: options.timeoutMs,
      failOnRustWarnings: options.failOnRustWarnings,
      rustIdlePollMs: options.rustIdlePollMs,
      commands: listed,
      waitForRustIdleMs: options.waitForRustIdleMs,
    });
    return;
  }

  const results = selected.map((command) =>
    runCommand(command, cwd, options.timeoutMs, options),
  );
  const failed = results.filter((result) => !result.passed);
  const blockers = readinessBlockers(results);
  writeReport(options, {
    schema: SCHEMA,
    format: FORMAT,
    executionMode: "execute",
    blockers,
    project: cwd,
    timeoutMs: options.timeoutMs,
    failOnRustWarnings: options.failOnRustWarnings,
    rustIdlePollMs: options.rustIdlePollMs,
    status: failed.length === 0 ? "passed" : "failed",
    commands: listed,
    results,
    waitForRustIdleMs: options.waitForRustIdleMs,
  });
  process.exitCode = failed.length === 0 ? 0 : 1;
}

function parseArgs(argv) {
  const options = {
    json: false,
    list: false,
    project: null,
    failOnRustWarnings: false,
    includeRouteSmoke: false,
    only: new Set(),
    routeBaseUrl: DEFAULT_ROUTE_BASE_URL,
    routeRetryCount: DEFAULT_ROUTE_RETRY_COUNT,
    routeRetryDelayMs: DEFAULT_ROUTE_RETRY_DELAY_MS,
    routeTimeoutMs: DEFAULT_ROUTE_TIMEOUT_MS,
    rustIdlePollMs: DEFAULT_RUST_IDLE_POLL_MS,
    timeoutMs: DEFAULT_TIMEOUT_MS,
    waitForRustIdleMs: 0,
  };
  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index];
    if (arg === "--json") {
      options.json = true;
    } else if (arg === "--list") {
      options.list = true;
    } else if (arg === "--fail-on-rust-warnings") {
      options.failOnRustWarnings = true;
    } else if (arg === "--include-route-smoke") {
      options.includeRouteSmoke = true;
    } else if (arg === "--project") {
      index += 1;
      options.project = requiredValue(argv[index], "--project");
    } else if (arg === "--route-base-url") {
      index += 1;
      options.routeBaseUrl = requiredValue(argv[index], "--route-base-url");
    } else if (arg === "--route-timeout-ms") {
      index += 1;
      options.routeTimeoutMs = parseTimeout(
        requiredValue(argv[index], "--route-timeout-ms"),
      );
    } else if (arg === "--route-retry-count") {
      index += 1;
      options.routeRetryCount = parseNonNegativeInteger(
        requiredValue(argv[index], "--route-retry-count"),
        "--route-retry-count",
      );
    } else if (arg === "--route-retry-delay-ms") {
      index += 1;
      options.routeRetryDelayMs = parseNonNegativeInteger(
        requiredValue(argv[index], "--route-retry-delay-ms"),
        "--route-retry-delay-ms",
      );
    } else if (arg === "--only") {
      index += 1;
      options.only.add(requiredValue(argv[index], "--only"));
    } else if (arg === "--timeout-ms") {
      index += 1;
      options.timeoutMs = parseTimeout(
        requiredValue(argv[index], "--timeout-ms"),
      );
    } else if (arg === "--wait-for-rust-idle-ms") {
      index += 1;
      options.waitForRustIdleMs = parseNonNegativeInteger(
        requiredValue(argv[index], "--wait-for-rust-idle-ms"),
        "--wait-for-rust-idle-ms",
      );
    } else if (arg === "--rust-idle-poll-ms") {
      index += 1;
      options.rustIdlePollMs = parsePositiveInteger(
        requiredValue(argv[index], "--rust-idle-poll-ms"),
        "--rust-idle-poll-ms",
      );
    } else if (arg === "--help" || arg === "-h") {
      printHelp();
      process.exit(0);
    } else {
      throw new Error(`Unknown launch compile gate option: ${arg}`);
    }
  }
  return options;
}

function commandSet(options) {
  const commands = [...COMMANDS];
  if (options.includeRouteSmoke) {
    commands.push(routeSmokeCommand(options));
  }
  return commands;
}

function routeSmokeCommand(options) {
  const args = [
    "node",
    "tools/launch/launch-route-smoke.js",
    "--json",
    "--base-url",
    options.routeBaseUrl,
    "--timeout-ms",
    String(options.routeTimeoutMs),
  ];
  if (options.routeRetryCount > 0) {
    args.push("--retry-count", String(options.routeRetryCount));
  }
  if (options.routeRetryDelayMs !== DEFAULT_ROUTE_RETRY_DELAY_MS) {
    args.push("--retry-delay-ms", String(options.routeRetryDelayMs));
  }
  return {
    id: "localhost-preview-route-smoke",
    kind: "http-smoke",
    expectedExitCodes: [0],
    args,
  };
}

function parseTimeout(value) {
  const parsed = Number.parseInt(value, 10);
  if (!Number.isFinite(parsed) || parsed < 1000) {
    throw new Error("--timeout-ms must be an integer >= 1000");
  }
  return parsed;
}

function parseNonNegativeInteger(value, flag) {
  const parsed = Number.parseInt(value, 10);
  if (!Number.isFinite(parsed) || parsed < 0) {
    throw new Error(`${flag} must be an integer >= 0`);
  }
  return parsed;
}

function parsePositiveInteger(value, flag) {
  const parsed = Number.parseInt(value, 10);
  if (!Number.isFinite(parsed) || parsed < 1) {
    throw new Error(`${flag} must be an integer >= 1`);
  }
  return parsed;
}

function requiredValue(value, flag) {
  if (!value) {
    throw new Error(`${flag} requires a value`);
  }
  return value;
}

function commandSummary(command) {
  return {
    id: command.id,
    kind: command.kind,
    command: command.args.join(" "),
    expectedExitCodes: expectedExitCodes(command),
  };
}

function runCommand(command, cwd, timeoutMs, options = {}) {
  const started = Date.now();
  const [program, ...args] = command.args;
  const rustIdleWait = isCargoCommand(command)
    ? waitForRustIdle({
        pollMs: options.rustIdlePollMs || DEFAULT_RUST_IDLE_POLL_MS,
        timeoutMs: options.waitForRustIdleMs || 0,
      })
    : emptyRustIdleWait();

  if (rustIdleWait.timedOut) {
    return rustProcessContentionResult(command, started, rustIdleWait);
  }

  const rustProcessSnapshotBefore = isCargoCommand(command)
    ? readRustProcessSnapshot()
    : null;
  const result = spawnSync(program, args, {
    cwd,
    encoding: "utf8",
    timeout: timeoutMs,
    windowsHide: true,
  });
  const rustProcessSnapshotAfter = isCargoCommand(command)
    ? readRustProcessSnapshot()
    : null;
  const exitCode = normalizeExitCode(result.status);
  const expectedCodes = expectedExitCodes(command);
  const warningStats = warningStatsForOutput(result.stdout, result.stderr);
  const warningCount = warningStats.warningCount;
  const diagnosticLines = extractDiagnosticLines(result.stdout, result.stderr);
  const childReport = parseChildJsonReport(result.stdout);
  const warningPolicyFailure = shouldFailForRustWarnings(
    command,
    warningCount,
    options,
  );
  const passed = expectedCodes.includes(exitCode) && !warningPolicyFailure;
  const rustProcessCountBefore = rustProcessSnapshotBefore
    ? rustProcessSnapshotBefore.processes.length
    : 0;
  const rustProcessCountAfter = rustProcessSnapshotAfter
    ? rustProcessSnapshotAfter.processes.length
    : 0;
  return {
    ...commandSummary(command),
    passed,
    exitCode,
    rawExitCode: result.status,
    terminated: Number.isInteger(exitCode) && exitCode < 0,
    warningCount,
    lineEndingWarningCount: warningStats.lineEndingWarningCount,
    nonLineEndingWarningCount: warningStats.nonLineEndingWarningCount,
    warningPolicyFailure,
    diagnosticLines,
    childReport,
    rustProcessCountBefore,
    rustProcessCountAfter,
    rustProcessesBefore: rustProcessSnapshotBefore
      ? rustProcessSnapshotBefore.processes
      : [],
    rustProcessesAfter: rustProcessSnapshotAfter
      ? rustProcessSnapshotAfter.processes
      : [],
    rustProcessSnapshotError: rustProcessSnapshotError(
      rustProcessSnapshotBefore,
      rustProcessSnapshotAfter,
    ),
    rustIdleWaitedMs: rustIdleWait.waitedMs,
    rustIdleWaitTimedOut: false,
    signal: result.signal,
    error: result.error ? result.error.message : null,
    failureKind: commandFailureKind({
      error: result.error ? result.error.message : null,
      exitCode,
      passed,
      rustProcessCountAfter,
      rustProcessCountBefore,
      rustIdleWaitTimedOut: false,
      terminated: Number.isInteger(exitCode) && exitCode < 0,
      warningPolicyFailure,
    }),
    durationMs: Date.now() - started,
    stdoutTail: tail(result.stdout),
    stderrTail: tail(result.stderr),
  };
}

function emptyRustIdleWait() {
  return {
    error: null,
    processCountAfterWait: 0,
    processCountBeforeWait: 0,
    processesAfterWait: [],
    timedOut: false,
    waitedMs: 0,
  };
}

function rustProcessContentionResult(command, started, rustIdleWait) {
  const rustProcessCountBefore = rustIdleWait.processCountBeforeWait;
  const rustProcessCountAfter = rustIdleWait.processCountAfterWait;
  return {
    ...commandSummary(command),
    passed: false,
    exitCode: null,
    rawExitCode: null,
    terminated: false,
    warningCount: 0,
    lineEndingWarningCount: 0,
    nonLineEndingWarningCount: 0,
    warningPolicyFailure: false,
    diagnosticLines: [],
    childReport: null,
    rustProcessCountBefore,
    rustProcessCountAfter,
    rustProcessesBefore: rustIdleWait.processesBeforeWait,
    rustProcessesAfter: rustIdleWait.processesAfterWait,
    rustProcessSnapshotError: rustIdleWait.error,
    rustIdleWaitedMs: rustIdleWait.waitedMs,
    rustIdleWaitTimedOut: true,
    signal: null,
    error: "Timed out waiting for active Rust processes to finish",
    failureKind: commandFailureKind({
      passed: false,
      rustIdleWaitTimedOut: true,
    }),
    durationMs: Date.now() - started,
    stdoutTail: [],
    stderrTail: [],
  };
}

function waitForRustIdle({
  pollMs = DEFAULT_RUST_IDLE_POLL_MS,
  readSnapshot = readRustProcessSnapshot,
  sleep = sleepSync,
  timeoutMs = 0,
} = {}) {
  let firstSnapshot = normalizeRustSnapshot(readSnapshot());
  let currentSnapshot = firstSnapshot;
  let waitedMs = 0;

  if (timeoutMs <= 0) {
    return {
      error: rustProcessSnapshotError(firstSnapshot, currentSnapshot),
      processCountAfterWait: currentSnapshot.processes.length,
      processCountBeforeWait: firstSnapshot.processes.length,
      processesAfterWait: currentSnapshot.processes,
      processesBeforeWait: firstSnapshot.processes,
      timedOut: false,
      waitedMs,
    };
  }

  while (currentSnapshot.processes.length > 0 && waitedMs < timeoutMs) {
    sleep(pollMs);
    waitedMs += pollMs;
    currentSnapshot = normalizeRustSnapshot(readSnapshot());
  }

  return {
    error: rustProcessSnapshotError(firstSnapshot, currentSnapshot),
    processCountAfterWait: currentSnapshot.processes.length,
    processCountBeforeWait: firstSnapshot.processes.length,
    processesAfterWait: currentSnapshot.processes,
    processesBeforeWait: firstSnapshot.processes,
    timedOut: currentSnapshot.processes.length > 0,
    waitedMs,
  };
}

function normalizeRustSnapshot(snapshot) {
  if (!snapshot) {
    return { error: "missing rust process snapshot", processes: [] };
  }
  return {
    error: snapshot.error || null,
    processes: Array.isArray(snapshot.processes) ? snapshot.processes : [],
  };
}

function sleepSync(ms) {
  if (ms <= 0) {
    return;
  }
  const state = new Int32Array(new SharedArrayBuffer(4));
  Atomics.wait(state, 0, 0, ms);
}

function readRustProcessSnapshot() {
  if (process.platform !== "win32") {
    return { error: null, processes: [] };
  }

  const result = spawnSync(
    "powershell.exe",
    [
      "-NoProfile",
      "-Command",
      [
        "$ErrorActionPreference='SilentlyContinue';",
        "Get-Process |",
          "Where-Object { $_.ProcessName -match '^(cargo|rustc|rustdoc)$' } |",
          "Select-Object Id,ProcessName |",
          "ConvertTo-Json -Compress",
      ].join(" "),
    ],
    {
      encoding: "utf8",
      timeout: 5000,
      windowsHide: true,
    },
  );

  if (result.error) {
    return { error: result.error.message, processes: [] };
  }
  if (result.status !== 0) {
    return {
      error: `rust process snapshot exited ${normalizeExitCode(result.status)}`,
      processes: [],
    };
  }

  const text = String(result.stdout || "").trim();
  if (!text) {
    return { error: null, processes: [] };
  }

  try {
    const parsed = JSON.parse(text);
    const processes = (Array.isArray(parsed) ? parsed : [parsed])
      .map((processInfo) => ({
        id: processInfo.Id,
        name: processInfo.ProcessName,
      }))
      .filter((processInfo) => processInfo.id && processInfo.name)
      .slice(0, 20);
    return { error: null, processes };
  } catch (error) {
    return {
      error: `failed to parse rust process snapshot: ${error.message}`,
      processes: [],
    };
  }
}

function rustProcessSnapshotError(before, after) {
  const errors = [before, after]
    .map((snapshot) => (snapshot ? snapshot.error : null))
    .filter(Boolean);
  return errors.length > 0 ? errors.join("; ") : null;
}

function normalizeExitCode(status) {
  if (!Number.isInteger(status)) {
    return status;
  }
  return status > 0x7fffffff ? status - 0x100000000 : status;
}

function countWarnings(value) {
  return String(value || "")
    .split(/\r?\n/)
    .filter((line) => line.startsWith("warning:")).length;
}

function countLineEndingWarnings(value) {
  return String(value || "")
    .split(/\r?\n/)
    .filter(isGitLineEndingWarning).length;
}

function isGitLineEndingWarning(line) {
  return GIT_LINE_ENDING_WARNING_PATTERN.test(String(line || ""));
}

function warningStatsForOutput(...values) {
  const warningCount = values.reduce(
    (total, value) => total + countWarnings(value),
    0,
  );
  const lineEndingWarningCount = values.reduce(
    (total, value) => total + countLineEndingWarnings(value),
    0,
  );
  return {
    warningCount,
    lineEndingWarningCount,
    nonLineEndingWarningCount: Math.max(warningCount - lineEndingWarningCount, 0),
  };
}

function shouldFailForRustWarnings(command, warningCount, options = {}) {
  return (
    options.failOnRustWarnings === true &&
    isCargoCommand(command) &&
    warningCount > 0
  );
}

function commandFailureKind(result) {
  if (result.passed) {
    return null;
  }
  if (result.rustIdleWaitTimedOut) {
    return "rust-process-contention";
  }
  if (result.warningPolicyFailure) {
    return "warning-policy";
  }
  if (result.error) {
    return "spawn-error";
  }
  if (result.terminated && hasActiveRustProcesses(result)) {
    return "terminated-with-active-rust-processes";
  }
  if (result.terminated) {
    return "terminated";
  }
  return "unexpected-exit";
}

function readinessBlockers(results = []) {
  return results
    .filter((result) => result && !result.passed)
    .map((result) => routeSmokeBlocker(result) || commandBlocker(result));
}

function routeSmokeBlocker(result) {
  const report = result.childReport;
  if (!report || report.schema !== "dx.www.launch.routeSmoke") {
    return null;
  }

  const kindByFailure = {
    "content-contract": "preview-route-content-contract",
    "route-failure": "preview-route-failure",
    "route-timeout": "preview-route-timeout",
    "server-timeout": "preview-server-timeout",
    "server-unreachable": "preview-server-unreachable",
  };

  const routes = Array.isArray(report.routes) ? report.routes : [];
  const failedRoutes = routes
    .filter((route) => route && route.passed === false && route.route)
    .map((route) => route.route);
  const timedOutRoutes = routes
    .filter(
      (route) =>
        route &&
        route.passed === false &&
        route.route &&
        route.errorCode === "ETIMEDOUT",
    )
    .map((route) => route.route);
  const contentContractFailures = contentContractFailuresForRoutes(routes);

  const blocker = {
    baseUrl: report.baseUrl || null,
    commandId: result.id,
    failedRouteCount: report.failedRouteCount || 0,
    kind: kindByFailure[report.failureKind] || "preview-route-smoke-failed",
    passedRouteCount: report.passedRouteCount || 0,
  };

  if (failedRoutes.length > 0) {
    blocker.failedRoutes = failedRoutes;
  }
  if (timedOutRoutes.length > 0) {
    blocker.timedOutRoutes = timedOutRoutes;
  }
  if (contentContractFailures.length > 0) {
    blocker.contentContractFailures = contentContractFailures;
  }

  return blocker;
}

function contentContractFailuresForRoutes(routes = []) {
  return routes
    .filter(
      (route) =>
        route &&
        route.route &&
        route.contentContract &&
        route.contentContract.inspected === true &&
        route.contentContract.passed === false &&
        (route.errorCode === "CONTENT_CONTRACT" || route.statusCode === 200),
    )
    .map((route) => ({
      missingMarkers: Array.isArray(route.contentContract.missingMarkers)
        ? route.contentContract.missingMarkers
        : [],
      route: route.route,
    }));
}

function commandBlocker(result) {
  if (result.failureKind === "rust-process-contention") {
    const blocker = {
      commandId: result.id,
      kind: "rust-process-contention",
      rustProcessCountAfter: result.rustProcessCountAfter || 0,
      rustProcessCountBefore: result.rustProcessCountBefore || 0,
    };
    addRustProcessIds(blocker, result);
    return blocker;
  }

  if (result.warningPolicyFailure) {
    return {
      commandId: result.id,
      kind: "rust-warning-policy",
      warningCount: result.warningCount || 0,
    };
  }

  if (result.failureKind === "terminated-with-active-rust-processes") {
    return {
      commandId: result.id,
      exitCode: result.exitCode,
      kind: "cargo-terminated-with-active-rust-processes",
      rustProcessCountAfter: result.rustProcessCountAfter || 0,
      rustProcessCountBefore: result.rustProcessCountBefore || 0,
    };
  }

  return {
    commandId: result.id,
    exitCode: result.exitCode ?? null,
    kind: result.failureKind || "command-failed",
  };
}

function addRustProcessIds(blocker, result) {
  const before = rustProcessIds(result.rustProcessesBefore);
  const after = rustProcessIds(result.rustProcessesAfter);
  if (before.length > 0) {
    blocker.rustProcessIdsBefore = before;
  }
  if (after.length > 0) {
    blocker.rustProcessIdsAfter = after;
  }
}

function rustProcessIds(processes) {
  if (!Array.isArray(processes)) {
    return [];
  }
  return processes
    .map((process) => process && process.id)
    .filter((id) => Number.isInteger(id));
}

function hasActiveRustProcesses(result) {
  return (
    (result.rustProcessCountBefore || 0) > 0 ||
    (result.rustProcessCountAfter || 0) > 0
  );
}

function isCargoCommand(command) {
  return (
    command.kind === "cargo-build" ||
    command.kind === "cargo-check" ||
    command.kind === "cargo-test"
  );
}

function extractDiagnosticLines(...values) {
  return values
    .flatMap((value) => String(value || "").split(/\r?\n/))
    .map((line) => line.trimEnd())
    .filter((line) => /^(error(\[[^\]]+\])?:|fatal:)/.test(line))
    .slice(-20);
}

function parseChildJsonReport(value) {
  const text = String(value || "").trim();
  if (!text.startsWith("{")) {
    return null;
  }
  try {
    return JSON.parse(text);
  } catch (_error) {
    return null;
  }
}

function expectedExitCodes(command) {
  return command.expectedExitCodes || [0];
}

function tail(value) {
  const lines = String(value || "")
    .trim()
    .split(/\r?\n/)
    .filter(Boolean);
  return lines.slice(-20);
}

function writeReport(options, report) {
  if (options.json) {
    process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
    return;
  }
  process.stdout.write(renderText(report));
}

function renderText(report) {
  const lines = [`DX WWW launch compile gate (${report.executionMode})`];
  for (const command of report.commands) {
    lines.push(`- ${command.id}: ${command.command}`);
  }
  if (report.results) {
    for (const result of report.results) {
      lines.push(
        `  ${result.id}: ${result.passed ? "passed" : "failed"}${resultSummarySuffix(result)}`,
      );
    }
  }
  return `${lines.join("\n")}\n`;
}

function resultSummarySuffix(result) {
  const parts = [];
  const lineEndingWarningCount = result.lineEndingWarningCount || 0;
  const nonLineEndingWarningCount =
    typeof result.nonLineEndingWarningCount === "number"
      ? result.nonLineEndingWarningCount
      : Math.max((result.warningCount || 0) - lineEndingWarningCount, 0);
  if (nonLineEndingWarningCount > 0) {
    parts.push(countLabel(nonLineEndingWarningCount, "warning"));
  }
  if (lineEndingWarningCount > 0) {
    parts.push(countLabel(lineEndingWarningCount, "line-ending notice"));
  }
  if (result.warningPolicyFailure) {
    parts.push("warning policy failed");
  }
  if (result.rustIdleWaitTimedOut) {
    parts.push(`rust idle wait timed out after ${result.rustIdleWaitedMs || 0}ms`);
  }
  if (result.terminated) {
    parts.push(`terminated exit ${result.exitCode}`);
  }
  if (!result.passed && hasActiveRustProcesses(result)) {
    parts.push(
      `active rust processes: before ${countWithRustProcessIds(
        result.rustProcessCountBefore || 0,
        result.rustProcessesBefore,
      )}, after ${countWithRustProcessIds(
        result.rustProcessCountAfter || 0,
        result.rustProcessesAfter,
      )}`,
    );
  }
  if (result.error) {
    parts.push(`spawn error: ${result.error}`);
  }
  if (result.childReport && result.childReport.schema === "dx.www.launch.routeSmoke") {
    const totalRouteCount =
      result.childReport.passedRouteCount + result.childReport.failedRouteCount;
    const routeStatus = `${result.childReport.passedRouteCount}/${totalRouteCount} routes`;
    if (result.childReport.failureKind) {
      parts.push(`route-smoke: ${result.childReport.failureKind}, ${routeStatus}`);
    } else {
      parts.push(`route-smoke: ${routeStatus}`);
    }
    const failedRoutes = failedRouteNames(result.childReport);
    if (failedRoutes.length > 0) {
      parts.push(`failed routes: ${failedRoutes.join(", ")}`);
    }
  }
  return parts.length > 0 ? ` (${parts.join("; ")})` : "";
}

function countWithRustProcessIds(count, processes) {
  const ids = rustProcessIds(processes);
  if (ids.length === 0) {
    return String(count);
  }
  return `${count} [${ids.join(", ")}]`;
}

function failedRouteNames(report) {
  if (!report || !Array.isArray(report.routes)) {
    return [];
  }
  return report.routes
    .filter((route) => route && route.passed === false && route.route)
    .map((route) => route.route);
}

function countLabel(count, singular) {
  return `${count} ${singular}${count === 1 ? "" : "s"}`;
}

function printHelp() {
  process.stdout.write(
    [
      "Usage: node tools/launch/launch-compile-gate.js [--json] [--list]",
      "       [--project <path>] [--only <command-id>] [--timeout-ms <ms>]",
      "       [--fail-on-rust-warnings]",
      "       [--wait-for-rust-idle-ms <ms>] [--rust-idle-poll-ms <ms>]",
      "       [--include-route-smoke] [--route-base-url <url>]",
      "       [--route-timeout-ms <ms>]",
      "       [--route-retry-count <n>] [--route-retry-delay-ms <ms>]",
    ].join("\n") + "\n",
  );
}

if (require.main === module) {
  try {
    main(process.argv.slice(2));
  } catch (error) {
    process.stderr.write(`${error.message}\n`);
    process.exitCode = 2;
  }
}

module.exports = {
  COMMANDS,
  DEFAULT_ROUTE_BASE_URL,
  DEFAULT_ROUTE_RETRY_COUNT,
  DEFAULT_ROUTE_RETRY_DELAY_MS,
  DEFAULT_ROUTE_TIMEOUT_MS,
  DEFAULT_RUST_IDLE_POLL_MS,
  DEFAULT_TIMEOUT_MS,
  FORMAT,
  SCHEMA,
  commandSet,
  commandFailureKind,
  countLineEndingWarnings,
  countWarnings,
  extractDiagnosticLines,
  hasActiveRustProcesses,
  isCargoCommand,
  main,
  normalizeExitCode,
  parseChildJsonReport,
  readinessBlockers,
  resultSummarySuffix,
  shouldFailForRustWarnings,
  waitForRustIdle,
  warningStatsForOutput,
};
