const { spawnSync } = require("node:child_process");
const path = require("node:path");

const routeSmoke = require("../launch-route-smoke.js");
const DEFAULT_ROUTE_COUNT = routeSmoke.DEFAULT_ROUTES.length;

function runCompileGate(project, options) {
  const args = buildCompileGateArgs(options);
  const result = spawnSync(process.execPath, args, {
    cwd: project,
    encoding: "utf8",
    timeout: options.compileTimeoutMs,
    windowsHide: true,
  });
  const report = parseJsonReport(result.stdout);
  const status = report ? report.status : null;
  return {
    command: [process.execPath, ...args].join(" "),
    exitCode: normalizeStatus(result.status),
    passed: result.status === 0 && status === "passed",
    report,
    status,
    stderrTail: tail(result.stderr),
    stdoutTail: tail(result.stdout),
  };
}

function buildCompileGateArgs(options) {
  const args = [
    path.join("tools", "launch", "launch-compile-gate.js"),
    "--json",
    "--timeout-ms",
    String(options.compileTimeoutMs),
  ];
  if (options.rustIdleWaitMs > 0) {
    args.push("--wait-for-rust-idle-ms", String(options.rustIdleWaitMs));
  }
  return args;
}

function runRouteSmoke(project, baseUrl, options) {
  const args = buildRouteSmokeArgs(baseUrl, options);
  const result = spawnSync(process.execPath, args, {
    cwd: project,
    encoding: "utf8",
    timeout: routeSmokeProcessTimeoutMs(options),
    windowsHide: true,
  });
  const report = parseJsonReport(result.stdout);
  return {
    command: [process.execPath, ...args].join(" "),
    exitCode: normalizeStatus(result.status),
    failureKind: report ? report.failureKind : "route-smoke-report-missing",
    passed: result.status === 0 && report && report.status === "passed",
    report,
    status: report ? report.status : null,
    stderrTail: tail(result.stderr),
    stdoutTail: tail(result.stdout),
  };
}

function buildRouteSmokeArgs(baseUrl, options) {
  return [
    path.join("tools", "launch", "launch-route-smoke.js"),
    "--json",
    "--base-url",
    baseUrl,
    "--timeout-ms",
    String(options.routeTimeoutMs),
    "--retry-count",
    String(options.routeRetryCount),
    "--retry-delay-ms",
    String(options.routeRetryDelayMs),
  ];
}

function routeSmokeProcessTimeoutMs(options) {
  const attempts = options.routeRetryCount + 1;
  return DEFAULT_ROUTE_COUNT * attempts * options.routeTimeoutMs + 30000;
}

function parseJsonReport(value) {
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

function tail(value) {
  return String(value || "")
    .trim()
    .split(/\r?\n/)
    .filter(Boolean)
    .slice(-20);
}

function normalizeStatus(status) {
  return Number.isInteger(status) && status > 0x7fffffff
    ? status - 0x100000000
    : status;
}

module.exports = {
  DEFAULT_ROUTE_COUNT,
  buildCompileGateArgs,
  buildRouteSmokeArgs,
  parseJsonReport,
  routeSmokeProcessTimeoutMs,
  runCompileGate,
  runRouteSmoke,
  tail,
};
