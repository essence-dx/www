const assert = require("node:assert/strict");
const { execFileSync, spawn } = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.join(__dirname, "..");
const readinessGatePath = path.join(
  repoRoot,
  "tools",
  "launch",
  "launch-readiness-gate.js",
);
const readinessGate = require(readinessGatePath);
const browserProof = require(path.join(
  repoRoot,
  "tools",
  "launch",
  "readiness-gate",
  "browser-proof.js",
));

test("launch readiness gate plans compile before live route smoke", () => {
  const options = readinessGate.parseArgs([
    "--port",
    "3101",
    "--route-timeout-ms",
    "30000",
    "--route-retry-count",
    "2",
    "--route-retry-delay-ms",
    "500",
  ]);

  const compileArgs = readinessGate.buildCompileGateArgs(options);
  const routeArgs = readinessGate.buildRouteSmokeArgs(
    "http://127.0.0.1:3101",
    options,
  );

  assert.deepEqual(compileArgs, [
    path.join("tools", "launch", "launch-compile-gate.js"),
    "--json",
    "--timeout-ms",
    "600000",
    "--wait-for-rust-idle-ms",
    "180000",
  ]);
  assert.equal(
    compileArgs.includes("--include-route-smoke"),
    false,
    "compile phase must not exercise the live preview server",
  );
  assert.deepEqual(routeArgs, [
    path.join("tools", "launch", "launch-route-smoke.js"),
    "--json",
    "--base-url",
    "http://127.0.0.1:3101",
    "--timeout-ms",
    "30000",
    "--retry-count",
    "2",
    "--retry-delay-ms",
    "500",
  ]);
});

test("launch readiness gate can validate an already-running preview without starting one", async () => {
  const server = await startFixturePreview();

  try {
    const report = await readinessGate.runReadinessGate(
      readinessGate.parseArgs([
        "--existing-preview",
        "--port",
        String(server.port),
        "--route-timeout-ms",
        "1000",
        "--route-retry-count",
        "0",
      ]),
    );

    assert.equal(report.status, "passed");
    assert.deepEqual(report.blockers, []);
    assert.deepEqual(report.compile, {
      passed: true,
      reason: "existing-preview",
      status: "skipped",
    });
    assert.equal(report.preview.mode, "existing-preview");
    assert.equal(report.preview.ready, true);
    assert.equal(report.preview.started, false);
    assert.equal(report.routeSmoke.report.status, "passed");
    assert.deepEqual(report.evidence, {
      schema: "dx.www.launch.readinessEvidence",
      format: 1,
      routeSmokeProof: true,
      routeContentProof: true,
      routeContentContractRoutes: 3,
      routeSmokeRoutes: [
        "/",
        "/dashboard",
        "/login",
        "/_dx/hot-reload/version",
        "/api/trpc/health",
      ],
      previewMode: "existing-preview",
      browserRuntimeProof: false,
      browserRenderProof: {
        schema: "dx.www.launch.browserRenderProof",
        format: 1,
        status: "missing",
        path: null,
        requiredRoutes: ["/", "/dashboard", "/login"],
        routeCount: 0,
        passedRouteCount: 0,
        screenshotCount: 0,
        missingRoutes: ["/", "/dashboard", "/login"],
        failedRoutes: [],
        routes: [],
      },
      liveProviderProof: false,
      scoreGateEligible: false,
      boundary:
        "This gate proves compile readiness, preview reachability, HTTP route status, and bounded route content markers only; it does not prove browser canvas rendering, native browser scrolling, WebGL interaction, or live provider execution.",
      requiredFor90Plus: [
        "Attach browser route proof for /, /dashboard, and /login.",
        "Attach live-provider evidence only after real app-owned credentials execute without exposing secrets.",
      ],
    });
    assert.equal(report.routeSmoke.report.routeContentProof, true);
    assert.equal(report.routeSmoke.report.browserRuntimeProof, false);
    assert.equal(report.routeSmoke.report.contentContractRouteCount, 3);
    assert.equal(report.routeSmoke.report.contentContractPassedRouteCount, 3);
    assert.equal(report.routeSmoke.report.contentContractFailedRouteCount, 0);
    assert.deepEqual(
      report.routeSmoke.report.routes.map((route) => route.route),
      [
        "/",
        "/dashboard",
        "/login",
        "/_dx/hot-reload/version",
        "/api/trpc/health",
      ],
    );
  } finally {
    await server.stop();
  }
});

test("launch readiness gate can persist an honest report artifact", async () => {
  const server = await startFixturePreview();
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-launch-readiness-gate-"));
  const out = path.join(dir, "readiness-report.json");

  try {
    execFileSync(
      process.execPath,
      [
        readinessGatePath,
        "--existing-preview",
        "--port",
        String(server.port),
        "--route-timeout-ms",
        "1000",
        "--route-retry-count",
        "0",
        "--out",
        out,
      ],
      { cwd: repoRoot, encoding: "utf8" },
    );

    const report = JSON.parse(fs.readFileSync(out, "utf8"));
    assert.equal(report.schema, "dx.www.launch.readinessGate");
    assert.equal(report.status, "passed");
    assert.equal(report.evidence.routeSmokeProof, true);
    assert.equal(report.evidence.browserRuntimeProof, false);
    assert.equal(report.evidence.browserRenderProof.status, "missing");
    assert.equal(report.evidence.liveProviderProof, false);
    assert.equal(report.evidence.scoreGateEligible, false);
    assert.deepEqual(report.blockers, []);
  } finally {
    await server.stop();
  }
});

test("launch readiness gate accepts attached browser route screenshot proof", async () => {
  const server = await startFixturePreview();
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-launch-browser-proof-"));
  const proofPath = writeBrowserProofFixture(dir);

  try {
    const report = await readinessGate.runReadinessGate(
      readinessGate.parseArgs([
        "--existing-preview",
        "--port",
        String(server.port),
        "--route-timeout-ms",
        "1000",
        "--route-retry-count",
        "0",
        "--browser-proof",
        proofPath,
      ]),
    );

    assert.equal(report.status, "passed");
    assert.deepEqual(report.blockers, []);
    assert.equal(report.evidence.routeSmokeProof, true);
    assert.equal(report.evidence.routeContentProof, true);
    assert.equal(report.evidence.browserRuntimeProof, true);
    assert.equal(report.evidence.browserRenderProof.status, "passed");
    assert.equal(report.evidence.browserRenderProof.passedRouteCount, 3);
    assert.equal(report.evidence.browserRenderProof.screenshotCount, 3);
    assert.deepEqual(report.evidence.browserRenderProof.missingRoutes, []);
    assert.deepEqual(report.evidence.browserRenderProof.failedRoutes, []);
    assert.deepEqual(report.evidence.requiredFor90Plus, [
      "Attach live-provider evidence only after real app-owned credentials execute without exposing secrets.",
    ]);
    assert.equal(
      report.evidence.scoreGateEligible,
      false,
      "browser proof alone must not claim provider-backed score eligibility",
    );
  } finally {
    await server.stop();
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test("launch browser proof rejects missing screenshot routes", () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-launch-browser-proof-"));
  const proofPath = path.join(dir, "browser-proof.json");
  fs.writeFileSync(
    proofPath,
    JSON.stringify(
      {
        schema: browserProof.SCHEMA,
        format: browserProof.FORMAT,
        status: "passed",
        routes: [
          {
            route: "/",
            status: "passed",
            httpStatus: 200,
            visibleTextLength: 12,
            mainPresent: true,
            blankPage: false,
            screenshot: "missing.png",
          },
        ],
      },
      null,
      2,
    ),
  );

  try {
    const proof = browserProof.readBrowserProof(repoRoot, proofPath);

    assert.equal(proof.valid, false);
    assert.match(proof.error, /missing routes: \/dashboard, \/login/);
    assert.match(proof.error, /failed routes: \//);
    assert.deepEqual(browserProof.browserProofBlocker(proof), {
      kind: "browser-render-proof-invalid",
      path: proof.summary.path,
      error: proof.error,
      missingRoutes: ["/dashboard", "/login"],
      failedRoutes: ["/"],
      screenshotCount: 0,
    });
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test("launch browser proof rejects stale route screenshots", () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-launch-browser-proof-"));
  const screenshotDir = path.join(dir, "screenshots");
  fs.mkdirSync(screenshotDir, { recursive: true });
  const screenshotFor = (name: string) => {
    const screenshot = path.join(screenshotDir, `${name}.png`);
    fs.writeFileSync(screenshot, "png");
    return screenshot;
  };
  const proofPath = path.join(dir, "browser-proof.json");
  fs.writeFileSync(
    proofPath,
    JSON.stringify(
      {
        schema: browserProof.SCHEMA,
        format: browserProof.FORMAT,
        status: "passed",
        routes: [
          {
            route: "/",
            url: "http://127.0.0.1:3000/",
            status: "passed",
            httpStatus: 200,
            visibleTextLength: 128,
            mainPresent: true,
            blankPage: false,
            screenshot: screenshotFor("root"),
          },
          {
            route: "/dashboard",
            url: "http://127.0.0.1:3000/",
            status: "passed",
            httpStatus: 200,
            visibleTextLength: 128,
            mainPresent: true,
            blankPage: false,
            screenshot: screenshotFor("dashboard"),
          },
          {
            route: "/login",
            url: "http://127.0.0.1:3000/login",
            status: "passed",
            httpStatus: 200,
            visibleTextLength: 128,
            mainPresent: true,
            blankPage: false,
            navigationError: "TypeError: route navigation failed",
            screenshot: screenshotFor("login"),
          },
        ],
      },
      null,
      2,
    ),
  );

  try {
    const proof = browserProof.readBrowserProof(repoRoot, proofPath);
    const dashboard = proof.summary.routes.find((route) => route.route === "/dashboard");
    const login = proof.summary.routes.find((route) => route.route === "/login");

    assert.equal(proof.valid, false);
    assert.match(proof.error, /failed routes: \/dashboard, \/login/);
    assert.equal(dashboard?.routeUrlMatches, false);
    assert.equal(dashboard?.status, "failed");
    assert.match(login?.navigationError || "", /navigation failed/);
    assert.equal(login?.status, "failed");
    assert.equal(proof.summary.screenshotCount, 3);
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test("launch browser proof rejects reused screenshot artifacts", () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-launch-browser-proof-"));
  const screenshot = path.join(dir, "shared.png");
  fs.writeFileSync(screenshot, "png");
  const proofPath = path.join(dir, "browser-proof.json");
  fs.writeFileSync(
    proofPath,
    JSON.stringify(
      {
        schema: browserProof.SCHEMA,
        format: browserProof.FORMAT,
        status: "passed",
        routes: ["/", "/dashboard", "/login"].map((route) => ({
          route,
          url: `http://127.0.0.1:3000${route}`,
          status: "passed",
          httpStatus: 200,
          visibleTextLength: 128,
          mainPresent: true,
          blankPage: false,
          screenshot,
        })),
      },
      null,
      2,
    ),
  );

  try {
    const proof = browserProof.readBrowserProof(repoRoot, proofPath);

    assert.equal(proof.valid, false);
    assert.match(proof.error, /reused screenshots: shared\.png/);
    assert.equal(proof.summary.reusedScreenshotCount, 1);
    assert.equal(proof.summary.screenshotCount, 1);
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test("launch readiness gate reports route smoke blockers with route detail", () => {
  assert.deepEqual(
    readinessGate.routeSmokeBlocker({
      failureKind: "content-contract",
      status: "failed",
      report: {
        baseUrl: "http://127.0.0.1:3103",
        browserRuntimeProof: false,
        contentContractFailedRouteCount: 2,
        contentContractPassedRouteCount: 1,
        contentContractRouteCount: 3,
        failedRouteCount: 2,
        failureKind: "content-contract",
        passedRouteCount: 1,
        routeContentProof: false,
        status: "failed",
        routes: [
          {
            route: "/",
            passed: false,
            contentContract: { inspected: true, passed: false },
          },
          {
            route: "/dashboard",
            passed: false,
            contentContract: { inspected: true, passed: false },
          },
          {
            route: "/api/trpc/health",
            passed: true,
            contentContract: { inspected: false, passed: true },
          },
        ],
      },
    }),
    {
      baseUrl: "http://127.0.0.1:3103",
      browserRuntimeProof: false,
      contentContractFailedRouteCount: 2,
      contentContractFailedRoutes: ["/", "/dashboard"],
      contentContractPassedRouteCount: 1,
      contentContractRouteCount: 3,
      failedRouteCount: 2,
      failedRoutes: ["/", "/dashboard"],
      kind: "content-contract",
      passedRouteCount: 1,
      routeContentProof: false,
      status: "failed",
    },
  );
});

test("launch readiness gate keeps public contract honest", () => {
  const source = fs.readFileSync(readinessGatePath, "utf8");

  assert.equal(readinessGate.SCHEMA, "dx.www.launch.readinessGate");
  assert.equal(readinessGate.FORMAT, 1);
  assert.ok(!source.includes(".v1"), "gate must not introduce public .v1 schema names");
  assert.ok(!source.includes("node_modules"), "gate must not rely on node_modules");
  assert.ok(!source.includes("100/100"), "gate must not claim fake perfection");
  assert.ok(!source.includes("Turbopack HMR"), "gate must not overclaim HMR");
  assert.ok(!source.includes("scoreGateEligible: true"), "route smoke must not lift the live-proof score cap");
});

test("launch readiness gate budgets route smoke for all launch endpoints", () => {
  const options = readinessGate.parseArgs([
    "--route-timeout-ms",
    "30000",
    "--route-retry-count",
    "2",
  ]);

  assert.equal(
    readinessGate.routeSmokeProcessTimeoutMs(options),
    readinessGate.DEFAULT_ROUTE_COUNT * 3 * 30000 + 30000,
  );
});

function writeBrowserProofFixture(dir: string) {
  const screenshotDir = path.join(dir, "screenshots");
  fs.mkdirSync(screenshotDir, { recursive: true });
  const routes = ["/", "/dashboard", "/login"].map((route) => {
    const name = route === "/" ? "root" : route.slice(1);
    const screenshot = path.join(screenshotDir, `${name}.png`);
    fs.writeFileSync(screenshot, "png");
    return {
      route,
      url: `http://127.0.0.1:3000${route}`,
      status: "passed",
      httpStatus: 200,
      visibleTextLength: 128,
      mainPresent: true,
      blankPage: false,
      screenshot,
    };
  });
  const proofPath = path.join(dir, "browser-proof.json");
  fs.writeFileSync(
    proofPath,
    JSON.stringify(
      {
        schema: browserProof.SCHEMA,
        format: browserProof.FORMAT,
        status: "passed",
        routes,
      },
      null,
      2,
    ),
  );
  return proofPath;
}

function startFixturePreview() {
  const script = `
const http = require("node:http");
  const okRoutes = new Set([
  "/",
  "/dashboard",
  "/login",
  "/_dx/hot-reload/version",
  "/api/trpc/health",
]);
const routeBodies = {
  "/": "<main data-dx-route=\\"/\\" data-dx-scroll-proof=\\"document-flow-no-lock\\" data-dx-wheel-scroll=\\"native\\" data-dx-scroll-content=\\"viewport-plus\\" data-dx-component=\\"template-landing-scene\\"></main>",
  "/dashboard": "<main data-dx-route=\\"/dashboard\\" data-dx-scroll-proof=\\"document-flow-no-lock\\" data-dx-wheel-scroll=\\"native\\" data-dx-scroll-content=\\"viewport-plus\\" data-dx-component=\\"template-dashboard-page\\"></main>",
  "/login": "<main data-dx-route=\\"/login\\" data-dx-scroll-proof=\\"document-flow-no-lock\\" data-dx-wheel-scroll=\\"native\\" data-dx-scroll-content=\\"viewport-plus\\" data-dx-component=\\"template-login-page\\"></main>",
  "/_dx/hot-reload/version": "ok",
  "/api/trpc/health": "{\\"ok\\":true}",
};
const server = http.createServer((request, response) => {
  response.statusCode = okRoutes.has(request.url) ? 200 : 404;
  response.end(routeBodies[request.url] || "not found");
});
server.listen(0, "127.0.0.1", () => {
  process.stdout.write(String(server.address().port) + "\\n");
});
process.on("SIGTERM", () => server.close(() => process.exit(0)));
`;
  const child = spawn(process.execPath, ["-e", script], {
    stdio: ["ignore", "pipe", "pipe"],
    windowsHide: true,
  });

  return new Promise((resolve, reject) => {
    let stdout = "";
    let stderr = "";
    const timeout = setTimeout(() => {
      child.kill();
      reject(new Error("fixture preview did not start"));
    }, 5000);

    child.stderr.on("data", (chunk) => {
      stderr += String(chunk);
    });
    child.stdout.on("data", (chunk) => {
      stdout += String(chunk);
      const line = stdout.split(/\r?\n/).find(Boolean);
      if (!line) {
        return;
      }
      const port = Number.parseInt(line, 10);
      if (!Number.isFinite(port)) {
        clearTimeout(timeout);
        child.kill();
        reject(new Error(`fixture preview emitted invalid port: ${line}`));
        return;
      }
      clearTimeout(timeout);
      resolve({
        port,
        stop: () =>
          new Promise((stopResolve) => {
            child.once("exit", () => stopResolve());
            child.kill();
            setTimeout(() => stopResolve(), 2000);
          }),
      });
    });
    child.once("exit", (code) => {
      if (!stdout.trim()) {
        clearTimeout(timeout);
        reject(
          new Error(
            `fixture preview exited before listening: ${code}; ${stderr.trim()}`,
          ),
        );
      }
    });
  });
}
