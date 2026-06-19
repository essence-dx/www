const fs = require("node:fs");
const path = require("node:path");

const SCHEMA = "dx.www.launch.browserRenderProof";
const FORMAT = 1;
const EXPECTED_BROWSER_ROUTES = Object.freeze([
  "/",
  "/dashboard",
  "/login",
]);

function readBrowserProof(project, proofPath) {
  if (!proofPath) {
    return missingBrowserProof();
  }

  const resolvedPath = path.resolve(project, proofPath);
  let receipt = null;
  try {
    receipt = JSON.parse(fs.readFileSync(resolvedPath, "utf8"));
  } catch (error) {
    return invalidBrowserProof(resolvedPath, `failed to read browser proof: ${error.message}`);
  }

  return validateBrowserProof(project, resolvedPath, receipt);
}

function validateBrowserProof(project, proofPath, receipt) {
  const routes = Array.isArray(receipt.routes) ? receipt.routes : [];
  const routeSummaries = routes.map((route) =>
    summarizeBrowserRoute(project, proofPath, route),
  );
  const screenshotEvidence = summarizeScreenshotEvidence(routeSummaries);
  const missingRoutes = EXPECTED_BROWSER_ROUTES.filter(
    (route) => !routeSummaries.some((summary) => summary.route === route),
  );
  const failedRoutes = routeSummaries
    .filter((route) => route.status !== "passed")
    .map((route) => route.route || "<unknown>");
  const schemaValid = receipt.schema === SCHEMA;
  const formatValid = receipt.format === FORMAT;
  const statusPassed = receipt.status === "passed";
  const valid =
    schemaValid &&
    formatValid &&
    statusPassed &&
    missingRoutes.length === 0 &&
    failedRoutes.length === 0 &&
    screenshotEvidence.reusedScreenshots.length === 0;

  return {
    path: displayPath(project, proofPath),
    rawPath: proofPath,
    valid,
    error: valid
      ? null
      : browserProofError({
          failedRoutes,
          formatValid,
          missingRoutes,
          reusedScreenshots: screenshotEvidence.reusedScreenshots,
          schemaValid,
          statusPassed,
        }),
    summary: {
      schema: SCHEMA,
      format: FORMAT,
      status: valid ? "passed" : "failed",
      path: displayPath(project, proofPath),
      requiredRoutes: [...EXPECTED_BROWSER_ROUTES],
      routeCount: routeSummaries.length,
      passedRouteCount: routeSummaries.length - failedRoutes.length,
      screenshotCount: screenshotEvidence.uniqueScreenshotCount,
      reusedScreenshotCount: screenshotEvidence.reusedScreenshots.length,
      reusedScreenshots: screenshotEvidence.reusedScreenshots,
      missingRoutes,
      failedRoutes,
      routes: routeSummaries,
    },
  };
}

function summarizeScreenshotEvidence(routeSummaries) {
  const screenshotCounts = new Map();
  for (const route of routeSummaries) {
    if (!route.screenshotExists || !route.screenshot) {
      continue;
    }
    screenshotCounts.set(route.screenshot, (screenshotCounts.get(route.screenshot) || 0) + 1);
  }

  const reusedScreenshots = [...screenshotCounts.entries()]
    .filter(([_screenshot, count]) => count > 1)
    .map(([screenshot]) => normalizeSlashes(path.basename(screenshot)));

  return {
    uniqueScreenshotCount: screenshotCounts.size,
    reusedScreenshots,
  };
}

function summarizeBrowserRoute(project, proofPath, route) {
  const screenshot = typeof route.screenshot === "string" ? route.screenshot : null;
  const screenshotPath = screenshot
    ? resolveArtifactPath(project, proofPath, screenshot)
    : null;
  const screenshotExists = Boolean(screenshotPath && fs.existsSync(screenshotPath));
  const visibleTextLength = Number.isFinite(route.visibleTextLength)
    ? route.visibleTextLength
    : 0;
  const mainPresent = route.mainPresent === true;
  const statusPassed = route.status === "passed" || route.passed === true;
  const httpStatusPassed =
    route.httpStatus === undefined || route.httpStatus === null || route.httpStatus === 200;
  const routeName = typeof route.route === "string" ? route.route : null;
  const routeUrl = typeof route.url === "string" ? route.url : null;
  const routeUrlMatches = routeUrlMatchesRoute(routeName, routeUrl);
  const navigationError =
    typeof route.navigationError === "string" && route.navigationError.trim().length > 0
      ? route.navigationError
      : null;
  const blankPage = route.blankPage === true || visibleTextLength <= 0;
  const passed =
    statusPassed &&
    httpStatusPassed &&
    routeUrlMatches &&
    navigationError === null &&
    mainPresent &&
    !blankPage &&
    screenshotExists;

  return {
    route: routeName,
    url: routeUrl,
    status: passed ? "passed" : "failed",
    httpStatus: Number.isFinite(route.httpStatus) ? route.httpStatus : null,
    routeUrlMatches,
    navigationError,
    visibleTextLength,
    mainPresent,
    blankPage,
    screenshot: screenshot ? displayPath(project, screenshotPath) : null,
    screenshotExists,
  };
}

function routeUrlMatchesRoute(routeName, routeUrl) {
  if (!routeName || !routeUrl) {
    return false;
  }

  let pathname = null;
  try {
    pathname = new URL(routeUrl, "http://127.0.0.1").pathname;
  } catch (_error) {
    return false;
  }

  return normalizeRoutePath(pathname) === normalizeRoutePath(routeName);
}

function normalizeRoutePath(value) {
  const normalized = `/${String(value).replace(/^\/+/, "")}`;
  if (normalized.length > 1 && normalized.endsWith("/")) {
    return normalized.slice(0, -1);
  }
  return normalized;
}

function resolveArtifactPath(project, proofPath, artifactPath) {
  if (path.isAbsolute(artifactPath)) {
    return path.normalize(artifactPath);
  }

  const fromProject = path.resolve(project, artifactPath);
  if (fs.existsSync(fromProject)) {
    return fromProject;
  }

  return path.resolve(path.dirname(proofPath), artifactPath);
}

function displayPath(project, value) {
  if (!value) {
    return null;
  }

  const resolved = path.resolve(value);
  const relative = path.relative(project, resolved);
  if (!relative.startsWith("..") && !path.isAbsolute(relative)) {
    return normalizeSlashes(relative);
  }
  return normalizeSlashes(resolved);
}

function missingBrowserProof() {
  return {
    path: null,
    rawPath: null,
    valid: false,
    error: "browser proof not attached",
    summary: {
      schema: SCHEMA,
      format: FORMAT,
      status: "missing",
      path: null,
      requiredRoutes: [...EXPECTED_BROWSER_ROUTES],
      routeCount: 0,
      passedRouteCount: 0,
      screenshotCount: 0,
      missingRoutes: [...EXPECTED_BROWSER_ROUTES],
      failedRoutes: [],
      routes: [],
    },
  };
}

function invalidBrowserProof(proofPath, error) {
  return {
    path: proofPath,
    rawPath: proofPath,
    valid: false,
    error,
    summary: {
      schema: SCHEMA,
      format: FORMAT,
      status: "failed",
      path: normalizeSlashes(proofPath),
      requiredRoutes: [...EXPECTED_BROWSER_ROUTES],
      routeCount: 0,
      passedRouteCount: 0,
      screenshotCount: 0,
      missingRoutes: [...EXPECTED_BROWSER_ROUTES],
      failedRoutes: [],
      routes: [],
    },
  };
}

function browserProofBlocker(browserProof) {
  const summary = browserProof.summary || missingBrowserProof().summary;
  return {
    kind: "browser-render-proof-invalid",
    path: summary.path,
    error: browserProof.error,
    missingRoutes: summary.missingRoutes,
    failedRoutes: summary.failedRoutes,
    screenshotCount: summary.screenshotCount,
  };
}

function browserProofError({
  failedRoutes,
  formatValid,
  missingRoutes,
  reusedScreenshots = [],
  schemaValid,
  statusPassed,
}) {
  const errors = [];
  if (!schemaValid) {
    errors.push(`schema must be ${SCHEMA}`);
  }
  if (!formatValid) {
    errors.push(`format must be ${FORMAT}`);
  }
  if (!statusPassed) {
    errors.push("status must be passed");
  }
  if (missingRoutes.length > 0) {
    errors.push(`missing routes: ${missingRoutes.join(", ")}`);
  }
  if (failedRoutes.length > 0) {
    errors.push(`failed routes: ${failedRoutes.join(", ")}`);
  }
  if (reusedScreenshots.length > 0) {
    errors.push(`reused screenshots: ${reusedScreenshots.join(", ")}`);
  }
  return errors.join("; ");
}

function normalizeSlashes(value) {
  return String(value).replace(/\\/g, "/");
}

module.exports = {
  EXPECTED_BROWSER_ROUTES,
  FORMAT,
  SCHEMA,
  browserProofBlocker,
  missingBrowserProof,
  readBrowserProof,
  validateBrowserProof,
};
