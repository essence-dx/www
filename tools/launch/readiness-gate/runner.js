const fs = require("node:fs");
const path = require("node:path");

const { runCompileGate, runRouteSmoke } = require("./commands.js");
const {
  checkExistingPreview,
  startPreview,
  stopPreview,
} = require("./preview.js");
const {
  browserProofBlocker,
  readBrowserProof,
} = require("./browser-proof.js");
const { readinessReport } = require("./report.js");

async function runReadinessGate(options) {
  const project = path.resolve(options.project || process.cwd());
  const template = path.resolve(
    project,
    options.template || path.join("examples", "template"),
  );
  const binary = path.resolve(
    project,
    options.binary || path.join("target", "debug", binaryName()),
  );
  const baseUrl = `http://${options.host}:${options.port}`;
  const blockers = [];
  const browserProof = readBrowserProof(project, options.browserProof);
  if (options.browserProof && !browserProof.valid) {
    blockers.push(browserProofBlocker(browserProof));
  }
  let preview = null;

  if (options.existingPreview) {
    const compile = {
      passed: true,
      reason: "existing-preview",
      status: "skipped",
    };
    const existingPreview = await checkExistingPreview({
      baseUrl,
      timeoutMs: options.routeTimeoutMs,
    });

    if (!existingPreview.ready) {
      blockers.push({ kind: "existing-preview-unreachable", baseUrl });
      return buildReport({
        baseUrl,
        binary,
        browserProof,
        blockers,
        compile,
        preview: existingPreview,
        template,
      });
    }

    const routeSmoke = runRouteSmoke(project, baseUrl, options);
    if (!routeSmoke.passed) {
      blockers.push(routeSmokeBlocker(routeSmoke));
    }

    return buildReport({
      baseUrl,
      binary,
      browserProof,
      blockers,
      compile,
      preview: existingPreview,
      routeSmoke,
      template,
    });
  }

  const compile = runCompileGate(project, options);
  if (!compile.passed) {
    blockers.push({ kind: "compile-gate-failed", status: compile.status });
    return buildReport({ baseUrl, binary, browserProof, blockers, compile, template });
  }

  if (!fs.existsSync(binary)) {
    blockers.push({ kind: "dev-binary-missing", path: binary });
    return buildReport({ baseUrl, binary, browserProof, blockers, compile, template });
  }

  if (!fs.existsSync(template)) {
    blockers.push({ kind: "template-missing", path: template });
    return buildReport({ baseUrl, binary, browserProof, blockers, compile, template });
  }

  try {
    preview = await startPreview({ baseUrl, binary, options, template });
    if (!preview.ready) {
      blockers.push({
        kind: "preview-start-failed",
        exitCode: preview.exitCode,
        stderrTail: preview.stderrTail,
      });
      return buildReport({
        baseUrl,
        binary,
        browserProof,
        blockers,
        compile,
        preview: preview.summary,
        template,
      });
    }

    const routeSmoke = runRouteSmoke(project, baseUrl, options);
    if (!routeSmoke.passed) {
      blockers.push(routeSmokeBlocker(routeSmoke));
    }

    return buildReport({
      baseUrl,
      binary,
      browserProof,
      blockers,
      compile,
      preview: preview.summary,
      routeSmoke,
      template,
    });
  } finally {
    if (preview) {
      await stopPreview(preview.child);
    }
  }
}

function buildReport({
  baseUrl,
  binary,
  browserProof,
  blockers,
  compile,
  preview = null,
  routeSmoke = null,
  template,
}) {
  return readinessReport({
    baseUrl,
    binary,
    browserProof,
    blockers,
    compile,
    preview,
    routeSmoke,
    template,
  });
}

function binaryName() {
  return process.platform === "win32" ? "dx-www.exe" : "dx-www";
}

function routeSmokeBlocker(routeSmoke) {
  const report = routeSmoke && routeSmoke.report;
  if (!report) {
    return {
      kind: routeSmoke?.failureKind || "route-smoke-failed",
      status: routeSmoke?.status || null,
    };
  }

  const failedRoutes = Array.isArray(report.routes)
    ? report.routes
        .filter((route) => route && !route.passed)
        .map((route) => route.route)
    : [];
  const contentContractFailedRoutes = Array.isArray(report.routes)
    ? report.routes
        .filter(
          (route) =>
            route &&
            !route.passed &&
            route.contentContract &&
            route.contentContract.inspected,
        )
        .map((route) => route.route)
    : [];

  return {
    baseUrl: report.baseUrl || null,
    kind:
      routeSmoke.failureKind ||
      report.failureKind ||
      "route-smoke-failed",
    status: routeSmoke.status || report.status || null,
    passedRouteCount: report.passedRouteCount || 0,
    failedRouteCount: report.failedRouteCount || 0,
    failedRoutes,
    routeContentProof: report.routeContentProof === true,
    browserRuntimeProof: report.browserRuntimeProof === true,
    contentContractRouteCount: report.contentContractRouteCount || 0,
    contentContractPassedRouteCount:
      report.contentContractPassedRouteCount || 0,
    contentContractFailedRouteCount:
      report.contentContractFailedRouteCount || 0,
    contentContractFailedRoutes,
  };
}

module.exports = {
  runReadinessGate,
  routeSmokeBlocker,
};
