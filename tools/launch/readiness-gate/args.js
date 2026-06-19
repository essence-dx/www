const DEFAULT_COMPILE_TIMEOUT_MS = 600000;
const DEFAULT_DEV_START_TIMEOUT_MS = 120000;
const DEFAULT_HOST = "127.0.0.1";
const DEFAULT_PORT = 3000;
const DEFAULT_ROUTE_RETRY_COUNT = 2;
const DEFAULT_ROUTE_RETRY_DELAY_MS = 500;
const DEFAULT_ROUTE_TIMEOUT_MS = 30000;
const DEFAULT_RUST_IDLE_WAIT_MS = 180000;

function parseArgs(argv) {
  const options = {
    binary: null,
    browserProof: null,
    compileTimeoutMs: DEFAULT_COMPILE_TIMEOUT_MS,
    devStartTimeoutMs: DEFAULT_DEV_START_TIMEOUT_MS,
    existingPreview: false,
    host: DEFAULT_HOST,
    json: false,
    out: null,
    port: DEFAULT_PORT,
    project: process.cwd(),
    routeRetryCount: DEFAULT_ROUTE_RETRY_COUNT,
    routeRetryDelayMs: DEFAULT_ROUTE_RETRY_DELAY_MS,
    routeTimeoutMs: DEFAULT_ROUTE_TIMEOUT_MS,
    rustIdleWaitMs: DEFAULT_RUST_IDLE_WAIT_MS,
    template: null,
  };

  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index];
    if (arg === "--json") {
      options.json = true;
    } else if (arg === "--existing-preview") {
      options.existingPreview = true;
    } else if (arg === "--binary") {
      index += 1;
      options.binary = requiredValue(argv[index], "--binary");
    } else if (arg === "--browser-proof") {
      index += 1;
      options.browserProof = requiredValue(argv[index], "--browser-proof");
    } else if (arg === "--compile-timeout-ms") {
      index += 1;
      options.compileTimeoutMs = parsePositiveInteger(
        requiredValue(argv[index], "--compile-timeout-ms"),
        "--compile-timeout-ms",
      );
    } else if (arg === "--dev-start-timeout-ms") {
      index += 1;
      options.devStartTimeoutMs = parsePositiveInteger(
        requiredValue(argv[index], "--dev-start-timeout-ms"),
        "--dev-start-timeout-ms",
      );
    } else if (arg === "--host") {
      index += 1;
      options.host = requiredValue(argv[index], "--host");
    } else if (arg === "--out" || arg === "--output") {
      index += 1;
      options.out = requiredValue(argv[index], arg);
    } else if (arg === "--port") {
      index += 1;
      options.port = parsePort(requiredValue(argv[index], "--port"));
    } else if (arg === "--project") {
      index += 1;
      options.project = requiredValue(argv[index], "--project");
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
    } else if (arg === "--route-timeout-ms") {
      index += 1;
      options.routeTimeoutMs = parsePositiveInteger(
        requiredValue(argv[index], "--route-timeout-ms"),
        "--route-timeout-ms",
      );
    } else if (arg === "--rust-idle-wait-ms") {
      index += 1;
      options.rustIdleWaitMs = parseNonNegativeInteger(
        requiredValue(argv[index], "--rust-idle-wait-ms"),
        "--rust-idle-wait-ms",
      );
    } else if (arg === "--template") {
      index += 1;
      options.template = requiredValue(argv[index], "--template");
    } else if (arg === "--help" || arg === "-h") {
      printHelp();
      process.exit(0);
    } else {
      throw new Error(`Unknown launch readiness gate option: ${arg}`);
    }
  }

  return options;
}

function parsePort(value) {
  const parsed = parsePositiveInteger(value, "--port");
  if (parsed > 65535) {
    throw new Error("--port must be <= 65535");
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

function parseNonNegativeInteger(value, flag) {
  const parsed = Number.parseInt(value, 10);
  if (!Number.isFinite(parsed) || parsed < 0) {
    throw new Error(`${flag} must be an integer >= 0`);
  }
  return parsed;
}

function requiredValue(value, flag) {
  if (!value) {
    throw new Error(`${flag} requires a value`);
  }
  return value;
}

function printHelp() {
  process.stdout.write(
    [
      "Usage: node tools/launch/launch-readiness-gate.js [--json]",
      "       [--existing-preview]",
      "       [--out <path>]",
      "       [--project <path>] [--template <path>] [--binary <path>]",
      "       [--browser-proof <path>]",
      "       [--host <host>] [--port <port>]",
      "       [--compile-timeout-ms <ms>] [--rust-idle-wait-ms <ms>]",
      "       [--dev-start-timeout-ms <ms>]",
      "       [--route-timeout-ms <ms>]",
      "       [--route-retry-count <n>] [--route-retry-delay-ms <ms>]",
    ].join("\n") + "\n",
  );
}

module.exports = {
  DEFAULT_COMPILE_TIMEOUT_MS,
  DEFAULT_DEV_START_TIMEOUT_MS,
  DEFAULT_HOST,
  DEFAULT_PORT,
  DEFAULT_ROUTE_RETRY_COUNT,
  DEFAULT_ROUTE_RETRY_DELAY_MS,
  DEFAULT_ROUTE_TIMEOUT_MS,
  DEFAULT_RUST_IDLE_WAIT_MS,
  parseArgs,
};
