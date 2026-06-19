const assert = require("node:assert/strict");
const http = require("node:http");
const { spawn, spawnSync } = require("node:child_process");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.join(__dirname, "..");
const gatePath = path.join(repoRoot, "tools", "launch", "launch-compile-gate.js");
const gate = require(gatePath);
const routeSmoke = require(path.join(
  repoRoot,
  "tools",
  "launch",
  "launch-route-smoke.js",
));

function runNode(args) {
  return new Promise((resolve) => {
    const child = spawn(process.execPath, args, {
      cwd: repoRoot,
      windowsHide: true,
    });
    let stdout = "";
    let stderr = "";

    child.stdout.on("data", (chunk) => {
      stdout += chunk;
    });
    child.stderr.on("data", (chunk) => {
      stderr += chunk;
    });
    child.on("error", (error) => {
      resolve({ status: null, stdout, stderr: stderr + error.message });
    });
    child.on("close", (status) => {
      resolve({ status, stdout, stderr });
    });
  });
}

function listen(server, port = 0) {
  return new Promise((resolve, reject) => {
    server.once("error", reject);
    server.listen(port, "127.0.0.1", resolve);
  });
}

function closeServer(server) {
  return new Promise((resolve, reject) => {
    server.close((error) => {
      if (error) {
        reject(error);
      } else {
        resolve();
      }
    });
  });
}

async function reservePort() {
  const server = http.createServer();
  await listen(server);
  const { port } = server.address();
  await closeServer(server);
  return port;
}

function routeContent(route) {
  const markers = routeSmoke.ROUTE_CONTENT_CONTRACTS[route] || [];
  return `<main>${markers.map((marker) => `<span ${marker}></span>`).join("")}</main>`;
}

function writeLaunchRoute(response, route) {
  response.writeHead(200, { "content-type": "text/html; charset=utf-8" });
  response.end(routeContent(route));
}

test("launch route smoke covers the preview launch contract endpoints", () => {
  assert.deepEqual(routeSmoke.DEFAULT_ROUTES, [
    "/",
    "/dashboard",
    "/login",
    "/_dx/hot-reload/version",
    "/api/trpc/health",
  ]);
});

test("launch route smoke validates content contracts beyond retained body sample", async () => {
  const server = http.createServer((request, response) => {
    if (request.url !== "/") {
      response.writeHead(404, { "content-type": "text/plain; charset=utf-8" });
      response.end("not found");
      return;
    }

    response.writeHead(200, { "content-type": "text/html; charset=utf-8" });
    response.write("<!doctype html><main>");
    response.write("x".repeat(128));
    response.end(routeContent("/"));
  });

  await listen(server);
  const { port } = server.address();

  try {
    const probe = await routeSmoke.probeRoute(
      `http://127.0.0.1:${port}`,
      "/",
      1000,
      32,
    );

    assert.equal(probe.statusCode, 200);
    assert.equal(probe.bodyTruncated, true);
    assert.equal(probe.contentContract.inspected, true);
    assert.equal(probe.contentContract.passed, true);
    assert.deepEqual(probe.contentContract.missingMarkers, []);
    assert.equal(probe.passed, true);
  } finally {
    await closeServer(server);
  }
});

test("launch route smoke accepts source-owned App Router route markers", () => {
  const sourceOwnedRoutes = {
    "/": [
      'data-dx-app-router-runtime="source-owned-app-router"',
      'data-dx-route-source="app/page.tsx"',
      'data-dx-component="template-landing-page"',
      'data-dx-context-runtime="source-owned-provider-value-map"',
    ],
    "/dashboard": [
      'data-dx-app-router-runtime="source-owned-app-router"',
      'data-dx-route-source="app/dashboard/page.tsx"',
      'data-dx-tsx-static-dom-preview="layout-template-page-composition"',
    ],
    "/login": [
      'data-dx-app-router-runtime="source-owned-app-router"',
      'data-dx-route-source="app/login/page.tsx"',
      'data-dx-auth-readiness-endpoint="/api/auth/readiness"',
    ],
  };

  for (const [route, markers] of Object.entries(sourceOwnedRoutes)) {
    const content = `<html><body>${markers.join("\n")}</body></html>`;
    const result = routeSmoke.inspectRouteContent(route, content);

    assert.equal(result.inspected, true, route);
    assert.equal(result.passed, true, route);
    assert.deepEqual(result.missingMarkers, [], route);
  }
});

test("launch compile gate lists the known-good preview launch checks", () => {
  const result = spawnSync(process.execPath, [gatePath, "--list", "--json"], {
    cwd: repoRoot,
    encoding: "utf8",
    windowsHide: true,
  });

  assert.equal(result.status, 0, result.stdout + result.stderr);
  const report = JSON.parse(result.stdout);

  assert.equal(report.schema, "dx.www.launch.compileGate");
  assert.equal(report.format, 1);
  assert.equal(report.executionMode, "list");
  assert.equal(report.timeoutMs, 300000);
  assert.deepEqual(
    report.commands.map((command) => command.id),
    [
      "dx-www-cli-dev-server-check",
      "dx-www-cli-dev-server-binary-build",
      "launch-readiness-bundle-unit",
      "launch-gate-node-unit",
      "diff-whitespace-check",
      "launch-gate-source-whitespace-scan",
      "conflict-marker-scan",
    ],
  );
  assert.equal(
    report.commands[0].command,
    "cargo check -q -p dx-www --no-default-features --features cli,dev-server",
  );
  assert.equal(
    report.commands[1].command,
    "cargo build -q -p dx-www --no-default-features --features cli,dev-server",
  );
  assert.equal(
    report.commands[2].command,
    "cargo test -q -p dx-www launch_readiness_reports_green_without_runtime_artifacts --lib --no-default-features --features cli,dev-server",
  );
  assert.equal(
    report.commands[3].command,
    "node --test benchmarks/launch-compile-gate.test.ts benchmarks/launch-readiness-gate.test.ts",
  );
  assert.equal(report.commands[4].command, "git diff --check");
  assert.equal(
    report.commands[5].command,
    "rg -n [ \\t]+$ tools/launch/launch-compile-gate.js tools/launch/launch-route-smoke.js benchmarks/launch-compile-gate.test.ts",
  );
  assert.deepEqual(report.commands[5].expectedExitCodes, [1]);
  assert.equal(
    report.commands[6].command,
    "rg -n ^(<<<<<<<|=======|>>>>>>>) dx-www/src dx-www/tests benchmarks tools/launch",
  );
  assert.deepEqual(report.commands[6].expectedExitCodes, [1]);

  const source = fs.readFileSync(gatePath, "utf8");
  assert.ok(!source.includes(".v1"), "gate must not introduce public .v1 schema names");
  assert.ok(!source.includes("node_modules"), "gate must not rely on node_modules");
  assert.ok(!source.includes("100/100"), "gate must not claim fake perfection");
  assert.ok(!source.includes("Turbopack HMR"), "gate must not overclaim HMR");
});

test("launch compile gate treats a clean conflict scan as passed", () => {
  const result = spawnSync(
    process.execPath,
    [gatePath, "--only", "conflict-marker-scan", "--json"],
    {
      cwd: repoRoot,
      encoding: "utf8",
      windowsHide: true,
    },
  );

  assert.equal(result.status, 0, result.stdout + result.stderr);
  const report = JSON.parse(result.stdout);

  assert.equal(report.executionMode, "execute");
  assert.equal(report.status, "passed");
  assert.equal(report.results.length, 1);
  assert.equal(report.results[0].id, "conflict-marker-scan");
  assert.equal(report.results[0].passed, true);
  assert.equal(report.results[0].exitCode, 1);
  assert.equal(report.results[0].warningCount, 0);
});

test("launch compile gate checks untracked launch source whitespace", () => {
  const result = spawnSync(
    process.execPath,
    [gatePath, "--only", "launch-gate-source-whitespace-scan", "--json"],
    {
      cwd: repoRoot,
      encoding: "utf8",
      windowsHide: true,
    },
  );

  assert.equal(result.status, 0, result.stdout + result.stderr);
  const report = JSON.parse(result.stdout);

  assert.equal(report.executionMode, "execute");
  assert.equal(report.status, "passed");
  assert.equal(report.results.length, 1);
  assert.equal(report.results[0].id, "launch-gate-source-whitespace-scan");
  assert.equal(report.results[0].passed, true);
  assert.equal(report.results[0].exitCode, 1);
  assert.equal(report.results[0].warningCount, 0);
});

test("launch compile gate reports Windows negative exits readably", () => {
  assert.equal(gate.normalizeExitCode(0), 0);
  assert.equal(gate.normalizeExitCode(1), 1);
  assert.equal(gate.normalizeExitCode(4294967295), -1);
  assert.equal(gate.normalizeExitCode(null), null);
});

test("launch compile gate counts Rust warnings in command output", () => {
  assert.equal(
    gate.countWarnings([
      "warning: unused import: `Thing`",
      "  --> dx-www\\src\\lib.rs:1:1",
      "warning: function `helper` is never used",
      "note: warnings are still allowed but visible",
    ].join("\n")),
    2,
  );
  assert.equal(gate.countWarnings(""), 0);
});

test("launch compile gate separates Git line-ending notices from actionable warnings", () => {
  const output = [
    "warning: in the working copy of 'dx-www/src/lib.rs', LF will be replaced by CRLF the next time Git touches it",
    "warning: unused import: `Thing`",
    "warning: in the working copy of 'README.md', CRLF will be replaced by LF the next time Git touches it",
  ].join("\n");

  assert.equal(gate.countWarnings(output), 3);
  assert.equal(gate.countLineEndingWarnings(output), 2);
  assert.deepEqual(gate.warningStatsForOutput(output), {
    warningCount: 3,
    lineEndingWarningCount: 2,
    nonLineEndingWarningCount: 1,
  });
  assert.equal(
    gate.resultSummarySuffix({
      warningCount: 2,
      lineEndingWarningCount: 2,
      nonLineEndingWarningCount: 0,
    }),
    " (2 line-ending notices)",
  );
  assert.equal(
    gate.resultSummarySuffix({
      warningCount: 3,
      lineEndingWarningCount: 2,
      nonLineEndingWarningCount: 1,
    }),
    " (1 warning; 2 line-ending notices)",
  );
});

test("launch compile gate extracts actionable diagnostic lines", () => {
  assert.deepEqual(
    gate.extractDiagnosticLines(
      "running 1 test\n",
      [
        "warning: unused import: `Thing`",
        "error[E0425]: cannot find value `route` in this scope",
        "  --> dx-www\\src\\lib.rs:12:5",
        "error: could not compile `dx-www` due to 1 previous error",
      ].join("\n"),
    ),
    [
      "error[E0425]: cannot find value `route` in this scope",
      "error: could not compile `dx-www` due to 1 previous error",
    ],
  );
});

test("launch compile gate parses child JSON reports", () => {
  assert.deepEqual(
    gate.parseChildJsonReport('{"schema":"dx.www.launch.routeSmoke","status":"passed"}\n'),
    {
      schema: "dx.www.launch.routeSmoke",
      status: "passed",
    },
  );
  assert.equal(gate.parseChildJsonReport("not-json"), null);
});

test("launch compile gate summarizes route smoke child reports", () => {
  assert.equal(
    gate.resultSummarySuffix({
      warningCount: 2,
      childReport: {
        schema: "dx.www.launch.routeSmoke",
        passedRouteCount: 0,
        failedRouteCount: 5,
        failureKind: "server-unreachable",
      },
    }),
    " (2 warnings; route-smoke: server-unreachable, 0/5 routes)",
  );
});

test("launch compile gate names failed route smoke routes in text summaries", () => {
  assert.equal(
    gate.resultSummarySuffix({
      childReport: {
        schema: "dx.www.launch.routeSmoke",
        passedRouteCount: 1,
        failedRouteCount: 2,
        failureKind: "route-failure",
        routes: [
          { route: "/", passed: false, statusCode: 500 },
          { route: "/dashboard", passed: false, errorCode: "ETIMEDOUT" },
          { route: "/api/trpc/health", passed: true, statusCode: 200 },
        ],
      },
    }),
    " (route-smoke: route-failure, 1/3 routes; failed routes: /, /dashboard)",
  );
});

test("launch compile gate classifies silent terminated command failures", () => {
  assert.equal(
    gate.commandFailureKind({
      passed: false,
      terminated: true,
      warningPolicyFailure: false,
      error: null,
      exitCode: -1,
    }),
    "terminated",
  );
  assert.equal(
    gate.resultSummarySuffix({
      passed: false,
      terminated: true,
      warningCount: 0,
      exitCode: -1,
    }),
    " (terminated exit -1)",
  );
});

test("launch compile gate identifies terminated cargo amid active Rust processes", () => {
  assert.equal(
    gate.commandFailureKind({
      passed: false,
      terminated: true,
      warningPolicyFailure: false,
      error: null,
      exitCode: -1,
      rustProcessCountBefore: 2,
      rustProcessCountAfter: 1,
    }),
    "terminated-with-active-rust-processes",
  );
  assert.equal(
    gate.resultSummarySuffix({
      passed: false,
      terminated: true,
      warningCount: 0,
      exitCode: -1,
      rustProcessCountBefore: 2,
      rustProcessCountAfter: 1,
      rustProcessesBefore: [
        { id: 101, name: "cargo" },
        { id: 202, name: "rustc" },
      ],
      rustProcessesAfter: [{ id: 202, name: "rustc" }],
    }),
    " (terminated exit -1; active rust processes: before 2 [101, 202], after 1 [202])",
  );
});

test("launch compile gate waits for active Rust processes to clear", () => {
  const snapshots = [
    { error: null, processes: [{ id: 1, name: "cargo" }] },
    { error: null, processes: [{ id: 1, name: "cargo" }] },
    { error: null, processes: [] },
  ];
  const slept = [];

  const result = gate.waitForRustIdle({
    pollMs: 100,
    readSnapshot: () => snapshots.shift(),
    sleep: (ms) => slept.push(ms),
    timeoutMs: 500,
  });

  assert.equal(result.timedOut, false);
  assert.equal(result.waitedMs, 200);
  assert.equal(result.processCountBeforeWait, 1);
  assert.equal(result.processCountAfterWait, 0);
  assert.deepEqual(slept, [100, 100]);
});

test("launch compile gate leaves Rust idle waiting disabled by default", () => {
  const result = gate.waitForRustIdle({
    pollMs: 100,
    readSnapshot: () => ({ error: null, processes: [{ id: 1, name: "cargo" }] }),
    sleep: () => {
      throw new Error("disabled idle wait should not sleep");
    },
    timeoutMs: 0,
  });

  assert.equal(result.timedOut, false);
  assert.equal(result.waitedMs, 0);
  assert.equal(result.processCountBeforeWait, 1);
  assert.equal(result.processCountAfterWait, 1);
});

test("launch compile gate fails clearly when Rust processes stay busy", () => {
  const result = gate.waitForRustIdle({
    pollMs: 100,
    readSnapshot: () => ({
      error: null,
      processes: [
        { id: 1, name: "cargo" },
        { id: 2, name: "rustc" },
      ],
    }),
    sleep: () => {},
    timeoutMs: 250,
  });

  assert.equal(result.timedOut, true);
  assert.equal(result.waitedMs, 300);
  assert.equal(result.processCountBeforeWait, 2);
  assert.equal(result.processCountAfterWait, 2);
  assert.deepEqual(result.processesAfterWait, [
    { id: 1, name: "cargo" },
    { id: 2, name: "rustc" },
  ]);
  assert.equal(
    gate.commandFailureKind({
      passed: false,
      rustIdleWaitTimedOut: true,
    }),
    "rust-process-contention",
  );
  assert.equal(
    gate.resultSummarySuffix({
      passed: false,
      rustIdleWaitTimedOut: true,
      rustIdleWaitedMs: 300,
      warningCount: 0,
      rustProcessCountBefore: 2,
      rustProcessCountAfter: 2,
    }),
    " (rust idle wait timed out after 300ms; active rust processes: before 2, after 2)",
  );
});

test("launch compile gate exposes optional Rust idle wait policy", () => {
  const result = spawnSync(
    process.execPath,
    [
      gatePath,
      "--list",
      "--json",
      "--wait-for-rust-idle-ms",
      "2000",
      "--rust-idle-poll-ms",
      "50",
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
      windowsHide: true,
    },
  );

  assert.equal(result.status, 0, result.stdout + result.stderr);
  const report = JSON.parse(result.stdout);

  assert.equal(report.waitForRustIdleMs, 2000);
  assert.equal(report.rustIdlePollMs, 50);
});

test("launch compile gate summarizes machine-readable readiness blockers", () => {
  assert.deepEqual(
    gate.readinessBlockers([
      {
        id: "dx-www-cli-dev-server-check",
        passed: false,
        failureKind: "rust-process-contention",
        rustProcessCountBefore: 2,
        rustProcessCountAfter: 2,
      },
      {
        id: "localhost-preview-route-smoke",
        passed: false,
        childReport: {
          schema: "dx.www.launch.routeSmoke",
          baseUrl: "http://127.0.0.1:3000",
          failureKind: "server-unreachable",
          failedRouteCount: 5,
          passedRouteCount: 0,
        },
      },
    ]),
    [
      {
        commandId: "dx-www-cli-dev-server-check",
        kind: "rust-process-contention",
        rustProcessCountAfter: 2,
        rustProcessCountBefore: 2,
      },
      {
        baseUrl: "http://127.0.0.1:3000",
        commandId: "localhost-preview-route-smoke",
        failedRouteCount: 5,
        kind: "preview-server-unreachable",
        passedRouteCount: 0,
      },
    ],
  );
  assert.deepEqual(gate.readinessBlockers([{ passed: true }]), []);
});

test("launch compile gate names failed route smoke blockers", () => {
  assert.deepEqual(
    gate.readinessBlockers([
      {
        id: "localhost-preview-route-smoke",
        passed: false,
        childReport: {
          schema: "dx.www.launch.routeSmoke",
          baseUrl: "http://127.0.0.1:3000",
          failureKind: "route-timeout",
          failedRouteCount: 2,
          passedRouteCount: 1,
          routes: [
            {
              route: "/",
              statusCode: null,
              passed: false,
              errorCode: "ETIMEDOUT",
            },
            {
              route: "/",
              statusCode: 500,
              passed: false,
            },
            {
              route: "/api/trpc/health",
              statusCode: 200,
              passed: true,
            },
          ],
        },
      },
    ]),
    [
      {
        baseUrl: "http://127.0.0.1:3000",
        commandId: "localhost-preview-route-smoke",
        failedRouteCount: 2,
        failedRoutes: ["/", "/"],
        kind: "preview-route-timeout",
        passedRouteCount: 1,
        timedOutRoutes: ["/"],
      },
    ],
  );
});

test("launch compile gate names route content contract blockers", () => {
  assert.deepEqual(
    gate.readinessBlockers([
      {
        id: "localhost-preview-route-smoke",
        passed: false,
        childReport: {
          schema: "dx.www.launch.routeSmoke",
          baseUrl: "http://127.0.0.1:3000",
          failureKind: "content-contract",
          failedRouteCount: 1,
          passedRouteCount: 5,
          routes: [
            {
              route: "/",
              statusCode: 200,
              passed: false,
              errorCode: "CONTENT_CONTRACT",
              contentContract: {
                inspected: true,
                passed: false,
                missingMarkers: [
                  'data-dx-component="launch-operating-dashboard"',
                ],
              },
            },
            {
              route: "/api/trpc/health",
              statusCode: 200,
              passed: true,
            },
          ],
        },
      },
    ]),
    [
      {
        baseUrl: "http://127.0.0.1:3000",
        commandId: "localhost-preview-route-smoke",
        contentContractFailures: [
          {
            missingMarkers: [
              'data-dx-component="launch-operating-dashboard"',
            ],
            route: "/",
          },
        ],
        failedRouteCount: 1,
        failedRoutes: ["/"],
        kind: "preview-route-content-contract",
        passedRouteCount: 5,
      },
    ],
  );
});

test("launch compile gate names rust process contention blockers", () => {
  assert.deepEqual(
    gate.readinessBlockers([
      {
        id: "dx-www-cli-dev-server-check",
        passed: false,
        failureKind: "rust-process-contention",
        rustProcessCountBefore: 3,
        rustProcessCountAfter: 2,
        rustProcessesBefore: [
          { id: 101, name: "cargo" },
          { id: 202, name: "rustc" },
          { id: null, name: "rustc" },
        ],
        rustProcessesAfter: [
          { id: 101, name: "cargo" },
          { id: 202, name: "rustc" },
        ],
      },
    ]),
    [
      {
        commandId: "dx-www-cli-dev-server-check",
        kind: "rust-process-contention",
        rustProcessCountAfter: 2,
        rustProcessCountBefore: 3,
        rustProcessIdsAfter: [101, 202],
        rustProcessIdsBefore: [101, 202],
      },
    ],
  );
});

test("launch compile gate can fail cargo commands on Rust warnings", () => {
  assert.equal(
    gate.shouldFailForRustWarnings(
      { kind: "cargo-check" },
      1,
      { failOnRustWarnings: true },
    ),
    true,
  );
  assert.equal(
    gate.shouldFailForRustWarnings(
      { kind: "cargo-build" },
      1,
      { failOnRustWarnings: true },
    ),
    true,
  );
  assert.equal(
    gate.shouldFailForRustWarnings(
      { kind: "git-check" },
      1,
      { failOnRustWarnings: true },
    ),
    false,
  );
  assert.equal(
    gate.shouldFailForRustWarnings(
      { kind: "cargo-test" },
      0,
      { failOnRustWarnings: true },
    ),
    false,
  );
  assert.equal(
    gate.shouldFailForRustWarnings(
      { kind: "cargo-check" },
      1,
      { failOnRustWarnings: false },
    ),
    false,
  );
});

test("launch compile gate can list the optional localhost route smoke", () => {
  const result = spawnSync(
    process.execPath,
    [gatePath, "--list", "--json", "--include-route-smoke"],
    {
      cwd: repoRoot,
      encoding: "utf8",
      windowsHide: true,
    },
  );

  assert.equal(result.status, 0, result.stdout + result.stderr);
  const report = JSON.parse(result.stdout);
  const routeSmoke = report.commands.find(
    (command) => command.id === "localhost-preview-route-smoke",
  );

  assert.ok(routeSmoke, "route smoke command should be opt-in and listed");
  assert.equal(routeSmoke.kind, "http-smoke");
  assert.equal(
    routeSmoke.command,
    "node tools/launch/launch-route-smoke.js --json --base-url http://127.0.0.1:3000 --timeout-ms 30000",
  );
});

test("launch route smoke parses retry options for startup waits", () => {
  const options = routeSmoke.parseArgs([
    "--retry-count",
    "2",
    "--retry-delay-ms",
    "25",
  ]);

  assert.equal(options.retryCount, 2);
  assert.equal(options.retryDelayMs, 25);
});

test("launch compile gate passes retry options to route smoke", () => {
  const result = spawnSync(
    process.execPath,
    [
      gatePath,
      "--list",
      "--json",
      "--include-route-smoke",
      "--route-retry-count",
      "2",
      "--route-retry-delay-ms",
      "25",
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
      windowsHide: true,
    },
  );

  assert.equal(result.status, 0, result.stdout + result.stderr);
  const report = JSON.parse(result.stdout);
  const routeSmokeCommand = report.commands.find(
    (command) => command.id === "localhost-preview-route-smoke",
  );

  assert.ok(routeSmokeCommand, "route smoke command should be listed");
  assert.equal(
    routeSmokeCommand.command,
    "node tools/launch/launch-route-smoke.js --json --base-url http://127.0.0.1:3000 --timeout-ms 30000 --retry-count 2 --retry-delay-ms 25",
  );
});

test("launch route smoke verifies the preview routes against an HTTP server", async (t) => {
  const server = http.createServer((request, response) => {
    if (routeSmoke.DEFAULT_ROUTES.includes(request.url)) {
      writeLaunchRoute(response, request.url);
    } else {
      response.writeHead(404);
      response.end();
    }
  });

  await new Promise((resolve, reject) => {
    server.once("error", reject);
    server.listen(0, "127.0.0.1", resolve);
  });
  t.after(() => server.close());

  const { port } = server.address();
  const report = await routeSmoke.runRouteSmoke({
    baseUrl: `http://127.0.0.1:${port}`,
    routes: [...routeSmoke.DEFAULT_ROUTES],
    timeoutMs: 1000,
  });

  assert.equal(report.schema, "dx.www.launch.routeSmoke");
  assert.equal(report.format, 1);
  assert.equal(report.status, "passed");
  assert.equal(report.passedRouteCount, routeSmoke.DEFAULT_ROUTES.length);
  assert.equal(report.failedRouteCount, 0);
  assert.equal(report.serverReachable, true);
  assert.equal(report.failureKind, null);
  assert.equal(report.contentContractRouteCount, 3);
  assert.equal(report.contentContractPassedRouteCount, 3);
  assert.equal(report.contentContractFailedRouteCount, 0);
  assert.equal(report.routeContentProof, true);
  assert.deepEqual(
    report.routes.map((route) => [route.route, route.statusCode, route.passed]),
    routeSmoke.DEFAULT_ROUTES.map((route) => [route, 200, true]),
  );
});

test("launch route smoke probes routes sequentially", async (t) => {
  let active = 0;
  let maxActive = 0;
  const server = http.createServer((request, response) => {
    active += 1;
    maxActive = Math.max(maxActive, active);
    setTimeout(() => {
      response.writeHead(request.url === "/first" || request.url === "/second" ? 200 : 404);
      response.end();
      active -= 1;
    }, 25);
  });

  await listen(server);
  t.after(() => server.close());

  const { port } = server.address();
  const report = await routeSmoke.runRouteSmoke({
    baseUrl: `http://127.0.0.1:${port}`,
    routes: ["/first", "/second"],
    timeoutMs: 1000,
  });

  assert.equal(report.status, "passed");
  assert.equal(maxActive, 1);
});

test("launch route smoke retries while the preview server starts", async (t) => {
  const port = await reservePort();
  const server = http.createServer((request, response) => {
    if (request.url === "/") {
      writeLaunchRoute(response, request.url);
      return;
    }
    response.writeHead(404);
    response.end();
  });
  let serverStarted = false;

  const timer = setTimeout(() => {
    server.listen(port, "127.0.0.1");
    serverStarted = true;
  }, 75);

  t.after(() => {
    clearTimeout(timer);
    if (serverStarted) {
      server.close();
    }
  });

  const report = await routeSmoke.runRouteSmoke({
    baseUrl: `http://127.0.0.1:${port}`,
    routes: ["/"],
    timeoutMs: 1000,
    retryCount: 5,
    retryDelayMs: 50,
  });

  assert.equal(report.status, "passed");
  assert.equal(report.passedRouteCount, 1);
  assert.equal(report.failedRouteCount, 0);
  assert.equal(report.retryCount, 5);
  assert.equal(report.retryDelayMs, 50);
  assert.ok(report.attempts > 1, `expected retries, got ${report.attempts}`);
});

test("launch route smoke retries when reachable routes time out", async (t) => {
  const server = http.createServer((request, response) => {
    if (request.url !== "/") {
      response.writeHead(404);
      response.end();
      return;
    }

    if (!server.seenSlowRoute) {
      server.seenSlowRoute = true;
      setTimeout(() => {
        if (!response.destroyed) {
          response.writeHead(200);
          response.end();
        }
      }, 75);
      return;
    }

    writeLaunchRoute(response, request.url);
  });

  await listen(server);
  t.after(() => server.close());

  const { port } = server.address();
  const report = await routeSmoke.runRouteSmoke({
    baseUrl: `http://127.0.0.1:${port}`,
    routes: ["/"],
    timeoutMs: 20,
    retryCount: 2,
    retryDelayMs: 10,
  });

  assert.equal(report.status, "passed");
  assert.equal(report.attempts, 2);
});

test("launch route smoke classifies reachable timed-out routes", () => {
  assert.deepEqual(
    routeSmoke.classifyRouteSmoke([
      {
        route: "/_dx/hot-reload/version",
        statusCode: 200,
        passed: true,
      },
      {
        route: "/",
        statusCode: null,
        passed: false,
        errorCode: "ETIMEDOUT",
      },
    ]),
    {
      passedRouteCount: 1,
      failedRouteCount: 1,
      serverReachable: true,
      failureKind: "route-timeout",
      contentContractRouteCount: 0,
      contentContractPassedRouteCount: 0,
      contentContractFailedRouteCount: 0,
    },
  );
  assert.equal(
    routeSmoke.shouldRetryRouteSmoke({ failureKind: "route-timeout" }),
    true,
  );
});

test("launch compile gate embeds optional route smoke child report", async (t) => {
  const server = http.createServer((request, response) => {
    if (routeSmoke.DEFAULT_ROUTES.includes(request.url)) {
      writeLaunchRoute(response, request.url);
    } else {
      response.writeHead(404);
      response.end();
    }
  });

  await new Promise((resolve, reject) => {
    server.once("error", reject);
    server.listen(0, "127.0.0.1", resolve);
  });
  t.after(() => server.close());

  const { port } = server.address();
  const result = await runNode(
    [
      gatePath,
      "--include-route-smoke",
      "--only",
      "localhost-preview-route-smoke",
      "--json",
      "--route-base-url",
      `http://127.0.0.1:${port}`,
      "--route-timeout-ms",
      "5000",
    ],
  );

  assert.equal(result.status, 0, result.stdout + result.stderr);
  const report = JSON.parse(result.stdout);
  const routeResult = report.results[0];

  assert.equal(routeResult.id, "localhost-preview-route-smoke");
  assert.equal(routeResult.childReport.schema, "dx.www.launch.routeSmoke");
  assert.equal(routeResult.childReport.status, "passed");
  assert.equal(routeResult.childReport.serverReachable, true);
});

test("launch compile gate includes route smoke blockers in JSON reports", async () => {
  const port = await reservePort();
  const result = await runNode([
    gatePath,
    "--include-route-smoke",
    "--only",
    "localhost-preview-route-smoke",
    "--json",
    "--route-base-url",
    `http://127.0.0.1:${port}`,
    "--route-timeout-ms",
    "1000",
  ]);

  assert.equal(result.status, 1, result.stdout + result.stderr);
  const report = JSON.parse(result.stdout);
  const routeResult = report.results[0];

  assert.equal(routeResult.childReport.serverReachable, false);
  assert.equal(routeResult.childReport.contentContractRouteCount, 0);
  assert.equal(routeResult.childReport.contentContractPassedRouteCount, 0);
  assert.equal(routeResult.childReport.contentContractFailedRouteCount, 0);

  assert.deepEqual(report.blockers, [
    {
      baseUrl: `http://127.0.0.1:${port}`,
      commandId: "localhost-preview-route-smoke",
      failedRouteCount: routeSmoke.DEFAULT_ROUTES.length,
      failedRoutes: routeSmoke.DEFAULT_ROUTES,
      kind: "preview-server-unreachable",
      passedRouteCount: 0,
    },
  ]);
});

test("launch route smoke classifies an unreachable preview server", () => {
  assert.deepEqual(
    routeSmoke.classifyRouteSmoke([
      {
        route: "/",
        statusCode: null,
        passed: false,
        errorCode: "ECONNREFUSED",
      },
      {
        route: "/",
        statusCode: null,
        passed: false,
        errorCode: "ECONNREFUSED",
      },
    ]),
    {
      passedRouteCount: 0,
      failedRouteCount: 2,
      serverReachable: false,
      failureKind: "server-unreachable",
      contentContractRouteCount: 0,
      contentContractPassedRouteCount: 0,
      contentContractFailedRouteCount: 0,
    },
  );
});
