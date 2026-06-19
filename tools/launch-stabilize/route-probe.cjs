const http = require("node:http");
const https = require("node:https");

const LAUNCH_ROUTE_PROBE_SCHEMA = "dx.www.launchStabilize.routeProbe";
const LAUNCH_ROUTE_CANDIDATES_SCHEMA = "dx.www.launchStabilize.routeProbe.candidates";
const DEFAULT_TIMEOUT_MS = 2000;

const DEFAULT_LAUNCH_ROUTE_PROBES = Object.freeze([
  routeProbe("/", "home page"),
  routeProbe("/", "root page"),
  routeProbe("/dashboard", "dashboard page"),
  routeProbe("/_dx/hot-reload/version", "DX hot reload version"),
  routeProbe("/api/trpc/health", "DX source-safe tRPC health"),
]);
const DATABASE_API_READINESS_ROUTE_PROBE = routeProbe(
  "/api/database-api/readiness",
  "database API readiness",
);

function routeProbe(path, label, expectedStatuses = [200]) {
  return Object.freeze({
    path,
    label,
    expectedStatuses: Object.freeze([...expectedStatuses]),
    adapterBoundary: false,
  });
}

async function probeLaunchRoutes({
  baseUrl = "http://127.0.0.1:3000",
  routes = DEFAULT_LAUNCH_ROUTE_PROBES,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  generatedAt = new Date().toISOString(),
} = {}) {
  const normalizedBaseUrl = normalizeBaseUrl(baseUrl);
  const routeResults = [];

  for (const route of routes) {
    routeResults.push(await probeRoute(normalizedBaseUrl, route, timeoutMs));
  }

  const passedCount = routeResults.filter((route) => route.status === "passed").length;
  const adapterBoundaryCount = routeResults.filter(
    (route) => route.status === "adapter-boundary",
  ).length;
  const failedCount = routeResults.filter((route) => route.status === "failed").length;
  const connectivity = summarizeConnectivity(routeResults);

  return {
    schema: LAUNCH_ROUTE_PROBE_SCHEMA,
    format: 1,
    generatedAt,
    baseUrl: normalizedBaseUrl,
    status:
      failedCount > 0 ? "failed" : adapterBoundaryCount > 0 ? "adapter-boundary" : "passed",
    passedCount,
    adapterBoundaryCount,
    failedCount,
    connectivity,
    routes: routeResults,
  };
}

async function probeLaunchRouteCandidates({
  baseUrls = ["http://127.0.0.1:3000"],
  routes = DEFAULT_LAUNCH_ROUTE_PROBES,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  generatedAt = new Date().toISOString(),
} = {}) {
  const candidates = normalizeBaseUrlCandidates(baseUrls);
  const attempts = [];

  for (const baseUrl of candidates) {
    attempts.push(
      await probeLaunchRoutes({
        baseUrl,
        routes,
        timeoutMs,
        generatedAt,
      }),
    );
  }

  const selectedReport = selectBestCandidateReport(attempts);

  return {
    schema: LAUNCH_ROUTE_CANDIDATES_SCHEMA,
    format: 1,
    generatedAt,
    status: selectedReport.status,
    selectedBaseUrl: selectedReport.baseUrl,
    selectedReport,
    attempts,
  };
}

async function probeRoute(baseUrl, route, timeoutMs) {
  const started = Date.now();
  const expectedStatuses = normalizeExpectedStatuses(route.expectedStatuses);
  const url = new URL(route.path, baseUrl);
  let response;

  try {
    response = await requestStatus(url, timeoutMs);
  } catch (error) {
    return {
      path: route.path,
      label: route.label || route.path,
      url: url.toString(),
      expectedStatuses,
      actualStatus: null,
      adapterBoundary: Boolean(route.adapterBoundary),
      status: "failed",
      elapsedMs: Date.now() - started,
      evidence: `request failed: ${requestErrorMessage(error)}`,
    };
  }

  const matched = expectedStatuses.includes(response.statusCode);
  const status = !matched
    ? "failed"
    : route.adapterBoundary
      ? "adapter-boundary"
      : "passed";

  return {
    path: route.path,
    label: route.label || route.path,
    url: url.toString(),
    expectedStatuses,
    actualStatus: response.statusCode,
    adapterBoundary: Boolean(route.adapterBoundary),
    status,
    elapsedMs: Date.now() - started,
    evidence: routeEvidence(status, expectedStatuses, response.statusCode, route),
    contentType: response.contentType,
  };
}

function requestStatus(url, timeoutMs) {
  const transport = url.protocol === "https:" ? https : http;

  return new Promise((resolve, reject) => {
    const request = transport.request(
      url,
      {
        method: "GET",
        timeout: timeoutMs,
        headers: {
          accept: "application/json,text/html,text/plain,*/*",
          "user-agent": "dx-www-launch-stabilize-route-probe",
        },
      },
      (response) => {
        response.resume();
        response.once("end", () => {
          resolve({
            statusCode: response.statusCode || 0,
            contentType: response.headers["content-type"] || "",
          });
        });
      },
    );

    request.once("timeout", () => {
      request.destroy(new Error(`timed out after ${timeoutMs}ms`));
    });
    request.once("error", reject);
    request.end();
  });
}

function summarizeConnectivity(routeResults) {
  const requestFailures = routeResults.filter((route) => route.actualStatus === null);
  const responseCount = routeResults.length - requestFailures.length;

  if (requestFailures.length === 0) {
    return {
      status: "reachable",
      failureKind: null,
      failedConnectionCount: 0,
      responseCount,
      evidence: `${responseCount} route probes received HTTP responses`,
    };
  }

  const failureKind = connectionFailureKind(requestFailures.map((route) => route.evidence));
  const status = responseCount === 0 ? "unreachable" : "mixed";

  return {
    status,
    failureKind,
    failedConnectionCount: requestFailures.length,
    responseCount,
    evidence:
      status === "unreachable"
        ? `server did not accept TCP connections for ${requestFailures.length} route probes`
        : `${responseCount} route probes received HTTP responses and ${requestFailures.length} connection attempts failed`,
  };
}

function connectionFailureKind(evidenceMessages) {
  const evidence = evidenceMessages.join("\n");
  if (/ECONNREFUSED/i.test(evidence)) return "connection-refused";
  if (/timed out|ETIMEDOUT/i.test(evidence)) return "timeout";
  if (/ENOTFOUND|EAI_AGAIN/i.test(evidence)) return "dns";
  return "request-error";
}

function requestErrorMessage(error) {
  if (error && typeof error.message === "string" && error.message.length > 0) {
    return error.message;
  }

  if (Array.isArray(error?.errors) && error.errors.length > 0) {
    return error.errors
      .map((entry) => entry?.message || entry?.code || String(entry))
      .filter(Boolean)
      .join("; ");
  }

  return error?.code || String(error);
}

function routeEvidence(status, expectedStatuses, actualStatus, route) {
  if (status === "passed") {
    return `received expected HTTP ${actualStatus}`;
  }

  if (status === "adapter-boundary") {
    return `received expected adapter-boundary HTTP ${actualStatus}`;
  }

  return `expected ${expectedStatuses.join(" or ")} but received ${actualStatus} for ${
    route.label || route.path
  }`;
}

function normalizeExpectedStatuses(expectedStatuses) {
  const values = Array.isArray(expectedStatuses) && expectedStatuses.length > 0
    ? expectedStatuses
    : [200];

  return values.map((status) => Number(status)).filter((status) => Number.isInteger(status));
}

function normalizeBaseUrl(baseUrl) {
  const parsed = new URL(baseUrl);
  parsed.pathname = parsed.pathname.replace(/\/+$/, "");
  parsed.search = "";
  parsed.hash = "";
  return parsed.toString().replace(/\/$/, "");
}

function normalizeBaseUrlCandidates(baseUrls) {
  const values = Array.isArray(baseUrls) ? baseUrls : [baseUrls];
  const normalized = [];
  const seen = new Set();

  for (const value of values) {
    if (typeof value !== "string" || value.trim().length === 0) continue;

    const baseUrl = normalizeBaseUrl(value.trim());
    if (!seen.has(baseUrl)) {
      seen.add(baseUrl);
      normalized.push(baseUrl);
    }
  }

  if (normalized.length === 0) {
    throw new Error("at least one launch route base URL candidate is required");
  }

  return normalized;
}

function selectBestCandidateReport(attempts) {
  if (!Array.isArray(attempts) || attempts.length === 0) {
    throw new Error("at least one launch route probe attempt is required");
  }

  return (
    attempts.find((attempt) => Number(attempt.failedCount || 0) === 0) ||
    attempts.find((attempt) => attempt.connectivity?.status === "reachable") ||
    attempts.find((attempt) => attempt.connectivity?.status === "mixed") ||
    attempts[0]
  );
}

function parseCliArgs(argv) {
  const options = {
    baseUrl: "http://127.0.0.1:3000",
    baseUrlCandidates: null,
    timeoutMs: DEFAULT_TIMEOUT_MS,
    json: false,
    routes: DEFAULT_LAUNCH_ROUTE_PROBES,
  };

  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index];
    if (arg === "--base-url") {
      options.baseUrl = requireValue(argv, (index += 1), arg);
    } else if (arg === "--base-url-candidates") {
      options.baseUrlCandidates = parseBaseUrlCandidateArg(requireValue(argv, (index += 1), arg));
    } else if (arg === "--timeout-ms") {
      options.timeoutMs = Number(requireValue(argv, (index += 1), arg));
    } else if (arg === "--json") {
      options.json = true;
    } else if (
      arg === "--include-database-readiness" ||
      arg === "--include-database-readiness-boundary"
    ) {
      options.routes = withDatabaseApiReadinessProbe(options.routes);
    } else {
      throw new Error(`Unknown argument: ${arg}`);
    }
  }

  if (!Number.isFinite(options.timeoutMs) || options.timeoutMs < 1) {
    throw new Error("--timeout-ms must be a positive number");
  }

  return options;
}

function withDatabaseApiReadinessProbe(routes) {
  const values = Array.isArray(routes) ? routes : DEFAULT_LAUNCH_ROUTE_PROBES;
  if (values.some((route) => route?.path === DATABASE_API_READINESS_ROUTE_PROBE.path)) {
    return values;
  }

  return [...values, DATABASE_API_READINESS_ROUTE_PROBE];
}

function parseBaseUrlCandidateArg(value) {
  return value
    .split(",")
    .map((entry) => entry.trim())
    .filter(Boolean);
}

function requireValue(argv, index, flag) {
  const value = argv[index];
  if (!value || value.startsWith("--")) {
    throw new Error(`${flag} requires a value`);
  }
  return value;
}

function printTextReport(report) {
  if (report.schema === LAUNCH_ROUTE_CANDIDATES_SCHEMA) {
    printCandidateTextReport(report);
    return;
  }

  process.stdout.write(
    [
      `DX-WWW launch route probe: ${report.status}`,
      `Base URL: ${report.baseUrl}`,
      `Routes: ${report.passedCount} passed, ${report.adapterBoundaryCount} adapter-boundary, ${report.failedCount} failed`,
      ...report.routes.map(
        (route) =>
          `- ${route.path}: ${route.status} (${route.actualStatus ?? "no response"}) ${route.evidence}`,
      ),
      "",
    ].join("\n"),
  );
}

function printCandidateTextReport(report) {
  process.stdout.write(
    [
      `DX-WWW launch route probe candidates: ${report.status}`,
      `Selected base URL: ${report.selectedBaseUrl}`,
      `Attempts: ${report.attempts.length}`,
      ...report.attempts.map(
        (attempt) =>
          `- ${attempt.baseUrl}: ${attempt.status} (${attempt.connectivity?.status || "unknown"})`,
      ),
      "",
    ].join("\n"),
  );
}

if (require.main === module) {
  (async () => {
    try {
      const options = parseCliArgs(process.argv.slice(2));
      const report = options.baseUrlCandidates
        ? await probeLaunchRouteCandidates({
            baseUrls: options.baseUrlCandidates,
            routes: options.routes,
            timeoutMs: options.timeoutMs,
          })
        : await probeLaunchRoutes(options);

      if (options.json) {
        process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
      } else {
        printTextReport(report);
      }

      const failedCount = Number(report.selectedReport?.failedCount ?? report.failedCount ?? 0);
      process.exitCode = failedCount === 0 ? 0 : 1;
    } catch (error) {
      process.stderr.write(`${error.message}\n`);
      process.exitCode = 1;
    }
  })();
}

module.exports = {
  DEFAULT_LAUNCH_ROUTE_PROBES,
  DATABASE_API_READINESS_ROUTE_PROBE,
  LAUNCH_ROUTE_CANDIDATES_SCHEMA,
  LAUNCH_ROUTE_PROBE_SCHEMA,
  probeLaunchRouteCandidates,
  probeLaunchRoutes,
};
