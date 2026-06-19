#!/usr/bin/env node

const http = require("node:http");
const https = require("node:https");

const SCHEMA = "dx.www.launch.routeSmoke";
const FORMAT = 1;
const DEFAULT_BASE_URL = "http://127.0.0.1:3000";
const DEFAULT_RETRY_COUNT = 0;
const DEFAULT_RETRY_DELAY_MS = 250;
const DEFAULT_TIMEOUT_MS = 30000;
const DEFAULT_MAX_BODY_BYTES = 262144;
const DEFAULT_ROUTES = Object.freeze([
  "/",
  "/dashboard",
  "/login",
  "/_dx/hot-reload/version",
  "/api/trpc/health",
]);
const ROUTE_CONTENT_CONTRACTS = Object.freeze({
  "/": Object.freeze([
    'data-dx-route="/"',
    'data-dx-scroll-proof="document-flow-no-lock"',
    'data-dx-wheel-scroll="native"',
    'data-dx-scroll-content="viewport-plus"',
    'data-dx-component="template-landing-scene"',
  ]),
  "/dashboard": Object.freeze([
    'data-dx-route="/dashboard"',
    'data-dx-scroll-proof="document-flow-no-lock"',
    'data-dx-wheel-scroll="native"',
    'data-dx-scroll-content="viewport-plus"',
    'data-dx-component="template-dashboard-page"',
  ]),
  "/login": Object.freeze([
    'data-dx-route="/login"',
    'data-dx-scroll-proof="document-flow-no-lock"',
    'data-dx-wheel-scroll="native"',
    'data-dx-scroll-content="viewport-plus"',
    'data-dx-component="template-login-page"',
  ]),
});
const ROUTE_SOURCE_CONTENT_CONTRACTS = Object.freeze({
  "/": Object.freeze([
    'data-dx-app-router-runtime="source-owned-app-router"',
    'data-dx-route-source="app/page.tsx"',
    'data-dx-component="template-landing-page"',
  ]),
  "/dashboard": Object.freeze([
    'data-dx-app-router-runtime="source-owned-app-router"',
    'data-dx-route-source="app/dashboard/page.tsx"',
    'data-dx-tsx-static-dom-preview="layout-template-page-composition"',
  ]),
  "/login": Object.freeze([
    'data-dx-app-router-runtime="source-owned-app-router"',
    'data-dx-route-source="app/login/page.tsx"',
    'data-dx-auth-readiness-endpoint="/api/auth/readiness"',
  ]),
});

async function main(argv) {
  const options = parseArgs(argv);
  const report = await runRouteSmoke(options);
  writeReport(options, report);
  process.exitCode = report.status === "passed" ? 0 : 1;
}

function parseArgs(argv) {
  const options = {
    baseUrl: DEFAULT_BASE_URL,
    json: false,
    retryCount: DEFAULT_RETRY_COUNT,
    retryDelayMs: DEFAULT_RETRY_DELAY_MS,
    routes: [...DEFAULT_ROUTES],
    routeSpecified: false,
    timeoutMs: DEFAULT_TIMEOUT_MS,
    maxBodyBytes: DEFAULT_MAX_BODY_BYTES,
  };

  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index];
    if (arg === "--json") {
      options.json = true;
    } else if (arg === "--base-url") {
      index += 1;
      options.baseUrl = requiredValue(argv[index], "--base-url");
    } else if (arg === "--route") {
      index += 1;
      if (!options.routeSpecified) {
        options.routes = [];
        options.routeSpecified = true;
      }
      options.routes.push(requiredValue(argv[index], "--route"));
    } else if (arg === "--timeout-ms") {
      index += 1;
      options.timeoutMs = parseTimeout(
        requiredValue(argv[index], "--timeout-ms"),
      );
    } else if (arg === "--retry-count") {
      index += 1;
      options.retryCount = parseNonNegativeInteger(
        requiredValue(argv[index], "--retry-count"),
        "--retry-count",
      );
    } else if (arg === "--retry-delay-ms") {
      index += 1;
      options.retryDelayMs = parseNonNegativeInteger(
        requiredValue(argv[index], "--retry-delay-ms"),
        "--retry-delay-ms",
      );
    } else if (arg === "--max-body-bytes") {
      index += 1;
      options.maxBodyBytes = parsePositiveInteger(
        requiredValue(argv[index], "--max-body-bytes"),
        "--max-body-bytes",
      );
    } else if (arg === "--help" || arg === "-h") {
      printHelp();
      process.exit(0);
    } else {
      throw new Error(`Unknown launch route smoke option: ${arg}`);
    }
  }

  if (options.routes.length === 0) {
    throw new Error("At least one --route value is required");
  }

  return options;
}

function requiredValue(value, flag) {
  if (!value) {
    throw new Error(`${flag} requires a value`);
  }
  return value;
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

async function runRouteSmoke(options) {
  const baseUrl = normalizeBaseUrl(options.baseUrl);
  const retryCount = options.retryCount ?? DEFAULT_RETRY_COUNT;
  const retryDelayMs = options.retryDelayMs ?? DEFAULT_RETRY_DELAY_MS;
  const routes = options.routes || [...DEFAULT_ROUTES];
  const timeoutMs = options.timeoutMs ?? DEFAULT_TIMEOUT_MS;
  const maxBodyBytes = options.maxBodyBytes ?? DEFAULT_MAX_BODY_BYTES;
  let report = null;

  for (let attempt = 0; attempt <= retryCount; attempt += 1) {
    const probes = await probeRoutes(baseUrl, routes, timeoutMs, maxBodyBytes);
    report = routeSmokeReport({
      attempt,
      baseUrl,
      maxBodyBytes,
      probes,
      retryCount,
      retryDelayMs,
      timeoutMs,
    });

    if (report.status === "passed" || !shouldRetryRouteSmoke(report)) {
      return report;
    }

    if (attempt < retryCount) {
      await sleep(retryDelayMs);
    }
  }

  return report;
}

async function probeRoutes(baseUrl, routes, timeoutMs, maxBodyBytes) {
  const probes = [];
  for (const route of routes) {
    probes.push(await probeRoute(baseUrl, route, timeoutMs, maxBodyBytes));
  }
  return probes;
}

function routeSmokeReport({
  attempt,
  baseUrl,
  maxBodyBytes,
  probes,
  retryCount,
  retryDelayMs,
  timeoutMs,
}) {
  const summary = classifyRouteSmoke(probes);
  const failed = probes.filter((probe) => !probe.passed);

  return {
    schema: SCHEMA,
    format: FORMAT,
    attempts: attempt + 1,
    baseUrl,
    retryCount,
    retryDelayMs,
    timeoutMs,
    maxBodyBytes,
    status: failed.length === 0 ? "passed" : "failed",
    ...summary,
    routeContentProof:
      summary.contentContractRouteCount > 0 &&
      summary.contentContractFailedRouteCount === 0,
    browserRuntimeProof: false,
    routes: probes,
  };
}

function shouldRetryRouteSmoke(report) {
  return (
    report.failureKind === "route-timeout" ||
    report.failureKind === "server-timeout" ||
    report.failureKind === "server-unreachable"
  );
}

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

function classifyRouteSmoke(probes) {
  const failed = probes.filter((probe) => !probe.passed);
  const serverReachable = probes.some((probe) => probe.statusCode !== null);
  const timedOut = failed.some((probe) => probe.errorCode === "ETIMEDOUT");
  const contentContractRoutes = probes.filter(
    (probe) => probe.contentContract?.inspected,
  );
  const contentContractFailedRouteCount = contentContractRoutes.filter(
    (probe) => !probe.contentContract.passed,
  ).length;
  let failureKind = null;

  if (failed.length > 0 && !serverReachable) {
    failureKind = timedOut ? "server-timeout" : "server-unreachable";
  } else if (failed.length > 0 && timedOut) {
    failureKind = "route-timeout";
  } else if (contentContractFailedRouteCount > 0) {
    failureKind = "content-contract";
  } else if (failed.length > 0) {
    failureKind = "route-failure";
  }

  return {
    passedRouteCount: probes.length - failed.length,
    failedRouteCount: failed.length,
    serverReachable,
    failureKind,
    contentContractRouteCount: contentContractRoutes.length,
    contentContractPassedRouteCount:
      contentContractRoutes.length - contentContractFailedRouteCount,
    contentContractFailedRouteCount,
  };
}

function normalizeBaseUrl(value) {
  const parsed = new URL(value);
  if (parsed.protocol !== "http:" && parsed.protocol !== "https:") {
    throw new Error("--base-url must use http or https");
  }
  return parsed.toString().replace(/\/$/, "");
}

function probeRoute(baseUrl, route, timeoutMs, maxBodyBytes) {
  const url = new URL(route, `${baseUrl}/`);
  const transport = url.protocol === "https:" ? https : http;

  return new Promise((resolve) => {
    const request = transport.request(
      url,
      { method: "GET", timeout: timeoutMs },
      (response) => {
        const chunks = [];
        const contentTracker = createRouteContentTracker(route);
        let receivedBytes = 0;
        response.on("data", (chunk) => {
          receivedBytes += chunk.length;
          contentTracker.observe(chunk);
          if (receivedBytes <= maxBodyBytes) {
            chunks.push(chunk);
            return;
          }

          const retainedBytes = chunks.reduce(
            (total, retained) => total + retained.length,
            0,
          );
          const remainingBytes = maxBodyBytes - retainedBytes;
          if (remainingBytes > 0) {
            chunks.push(chunk.subarray(0, remainingBytes));
          }
        });
        response.on("end", () => {
          const body = Buffer.concat(chunks).toString("utf8");
          const contentContract = contentTracker.finish();
          const statusPassed = response.statusCode === 200;
          const passed = statusPassed && contentContract.passed;
          resolve({
            route,
            url: url.toString(),
            statusCode: response.statusCode,
            contentType: response.headers["content-type"] || null,
            bodyBytes: receivedBytes,
            bodyTruncated: receivedBytes > maxBodyBytes,
            contentContract,
            passed,
            ...(passed
              ? {}
              : routeFailureDetail(response.statusCode, contentContract)),
          });
        });
      },
    );

    request.on("timeout", () => {
      const error = new Error(`timed out after ${timeoutMs}ms`);
      error.code = "ETIMEDOUT";
      request.destroy(error);
    });
    request.on("error", (error) => {
      resolve({
        route,
        url: url.toString(),
        statusCode: null,
        passed: false,
        contentContract: uninspectedRouteContent(),
        errorCode: error.code || null,
        error: error.message,
      });
    });
    request.end();
  });
}

function createRouteContentTracker(route) {
  const markerGroups = routeContentContractGroups(route);
  if (markerGroups.length === 0) {
    return {
      observe() {},
      finish: uninspectedRouteContent,
    };
  }

  const foundMarkers = new Set();
  const allMarkers = [...new Set(markerGroups.flatMap((group) => group.markers))];
  const maxMarkerLength = allMarkers.reduce(
    (max, marker) => Math.max(max, marker.length),
    0,
  );
  let tail = "";

  return {
    observe(chunk) {
      const text = tail + chunk.toString("utf8");
      for (const marker of allMarkers) {
        if (!foundMarkers.has(marker) && text.includes(marker)) {
          foundMarkers.add(marker);
        }
      }
      tail = text.slice(-Math.max(maxMarkerLength - 1, 0));
    },
    finish() {
      return evaluateRouteContentContract(route, (marker) => foundMarkers.has(marker));
    },
  };
}

function inspectRouteContent(route, body) {
  return evaluateRouteContentContract(route, (marker) => body.includes(marker));
}

function routeContentContractGroups(route) {
  const groups = [];
  if (ROUTE_CONTENT_CONTRACTS[route]) {
    groups.push({
      name: "static-runtime-route-markers",
      markers: ROUTE_CONTENT_CONTRACTS[route],
    });
  }
  if (ROUTE_SOURCE_CONTENT_CONTRACTS[route]) {
    groups.push({
      name: "source-owned-app-router-route-markers",
      markers: ROUTE_SOURCE_CONTENT_CONTRACTS[route],
    });
  }
  return groups;
}

function evaluateRouteContentContract(route, hasMarker) {
  const markerGroups = routeContentContractGroups(route);
  if (markerGroups.length === 0) {
    return uninspectedRouteContent();
  }

  const results = markerGroups.map((group) => ({
    ...group,
    missingMarkers: group.markers.filter((marker) => !hasMarker(marker)),
  }));
  const passing = results.find((result) => result.missingMarkers.length === 0);
  const selected =
    passing ||
    results.reduce((best, result) =>
      result.missingMarkers.length < best.missingMarkers.length ? result : best,
    );

  return {
    inspected: true,
    passed: Boolean(passing),
    contract: selected.name,
    acceptedContract: passing ? passing.name : null,
    requiredMarkers: selected.markers,
    missingMarkers: passing ? [] : selected.missingMarkers,
  };
}

function uninspectedRouteContent() {
  return {
    inspected: false,
    passed: true,
    requiredMarkers: [],
    missingMarkers: [],
  };
}

function routeFailureDetail(statusCode, contentContract) {
  if (statusCode !== 200) {
    return {
      errorCode: "HTTP_STATUS",
      error: `expected HTTP 200, received ${statusCode}`,
    };
  }

  if (!contentContract.passed) {
    return {
      errorCode: "CONTENT_CONTRACT",
      error: `missing route content markers: ${contentContract.missingMarkers.join(", ")}`,
    };
  }

  return {};
}

function writeReport(options, report) {
  if (options.json) {
    process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
    return;
  }

  const lines = [`DX WWW launch route smoke: ${report.status}`];
  if (report.attempts > 1) {
    lines.push(`Attempts: ${report.attempts}`);
  }
  for (const route of report.routes) {
    const detail = route.statusCode === null ? route.error : route.statusCode;
    lines.push(`- ${route.route}: ${route.passed ? "passed" : "failed"} (${detail})`);
  }
  process.stdout.write(`${lines.join("\n")}\n`);
}

function printHelp() {
  process.stdout.write(
    [
      "Usage: node tools/launch/launch-route-smoke.js [--json]",
      "       [--base-url <url>] [--route <path>] [--timeout-ms <ms>]",
      "       [--retry-count <n>] [--retry-delay-ms <ms>]",
      "       [--max-body-bytes <bytes>]",
    ].join("\n") + "\n",
  );
}

if (require.main === module) {
  main(process.argv.slice(2)).catch((error) => {
    process.stderr.write(`${error.message}\n`);
    process.exitCode = 2;
  });
}

module.exports = {
  DEFAULT_BASE_URL,
  DEFAULT_MAX_BODY_BYTES,
  DEFAULT_RETRY_COUNT,
  DEFAULT_RETRY_DELAY_MS,
  DEFAULT_ROUTES,
  DEFAULT_TIMEOUT_MS,
  FORMAT,
  SCHEMA,
  normalizeBaseUrl,
  parseArgs,
  ROUTE_CONTENT_CONTRACTS,
  ROUTE_SOURCE_CONTENT_CONTRACTS,
  createRouteContentTracker,
  routeContentContractGroups,
  inspectRouteContent,
  probeRoute,
  probeRoutes,
  classifyRouteSmoke,
  runRouteSmoke,
  shouldRetryRouteSmoke,
};
