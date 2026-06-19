import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const http = require("node:http");
const test = require("node:test");

const {
  DEFAULT_LAUNCH_ROUTE_PROBES,
  LAUNCH_ROUTE_PROBE_SCHEMA,
  probeLaunchRoutes,
  probeLaunchRouteCandidates,
} = require("../tools/launch-stabilize/route-probe.cjs");

test("launch route probe verifies required runtime routes", async () => {
  const { baseUrl, close } = await startFixtureServer((request, response) => {
    const statusByPath = new Map([
      ["/", 200],
      ["/", 200],
      ["/dashboard", 200],
      ["/_dx/hot-reload/version", 200],
      ["/api/trpc/health", 200],
    ]);
    response.writeHead(statusByPath.get(new URL(request.url, baseUrl).pathname) || 404, {
      "content-type": "application/json",
    });
    response.end(JSON.stringify({ ok: true }));
  });

  try {
    const report = await probeLaunchRoutes({
      baseUrl,
      generatedAt: "2026-05-23T00:00:00.000Z",
    });

    assert.equal(report.schema, LAUNCH_ROUTE_PROBE_SCHEMA);
    assert.equal(report.format, 1);
    assert.equal(report.baseUrl, baseUrl);
    assert.equal(report.status, "passed");
    assert.equal(report.generatedAt, "2026-05-23T00:00:00.000Z");
    assert.equal(report.passedCount, DEFAULT_LAUNCH_ROUTE_PROBES.length);
    assert.equal(report.failedCount, 0);
    assert.deepEqual(
      report.routes.map((route) => route.path),
      ["/", "/", "/dashboard", "/_dx/hot-reload/version", "/api/trpc/health"],
    );
    assert.equal(report.routes.every((route) => route.status === "passed"), true);
    assert.doesNotMatch(JSON.stringify(report), /\.v1\b/);
  } finally {
    await close();
  }
});

test("launch route probe reports adapter-boundary routes without hiding them", async () => {
  const { baseUrl, close } = await startFixtureServer((request, response) => {
    const requestPath = new URL(request.url, baseUrl).pathname;
    response.writeHead(requestPath === "/api/provider-boundary" ? 501 : 404, {
      "content-type": "application/json",
    });
    response.end(JSON.stringify({ ok: false, boundary: "adapter-boundary" }));
  });

  try {
    const report = await probeLaunchRoutes({
      baseUrl,
      routes: [
        {
          path: "/api/provider-boundary",
          label: "provider boundary",
          expectedStatuses: [501],
          adapterBoundary: true,
        },
      ],
      generatedAt: "2026-05-23T00:00:00.000Z",
    });

    assert.equal(report.status, "adapter-boundary");
    assert.equal(report.adapterBoundaryCount, 1);
    assert.equal(report.failedCount, 0);
    assert.equal(report.routes[0].status, "adapter-boundary");
    assert.equal(report.routes[0].actualStatus, 501);
    assert.equal(report.routes[0].adapterBoundary, true);
  } finally {
    await close();
  }
});

test("launch route probe treats database API readiness as executable source-owned route", async () => {
  const { baseUrl, close } = await startFixtureServer((request, response) => {
    const requestPath = new URL(request.url, baseUrl).pathname;
    response.writeHead(requestPath === "/api/database-api/readiness" ? 200 : 404, {
      "content-type": "application/json",
    });
    response.end(JSON.stringify({ ok: true, schema: "dx.www.template.database_api_readiness" }));
  });

  try {
    const report = await probeLaunchRoutes({
      baseUrl,
      routes: [
        {
          path: "/api/database-api/readiness",
          label: "database API readiness",
          expectedStatuses: [200],
        },
      ],
      generatedAt: "2026-05-23T00:00:00.000Z",
    });

    assert.equal(report.status, "passed");
    assert.equal(report.passedCount, 1);
    assert.equal(report.adapterBoundaryCount, 0);
    assert.equal(report.failedCount, 0);
    assert.equal(report.routes[0].status, "passed");
    assert.equal(report.routes[0].actualStatus, 200);
    assert.equal(report.routes[0].adapterBoundary, false);
  } finally {
    await close();
  }
});

test("launch route probe fails loudly on unexpected route status", async () => {
  const { baseUrl, close } = await startFixtureServer((_request, response) => {
    response.writeHead(404, { "content-type": "text/plain" });
    response.end("missing");
  });

  try {
    const report = await probeLaunchRoutes({
      baseUrl,
      routes: [{ path: "/", label: "launch page", expectedStatuses: [200] }],
      generatedAt: "2026-05-23T00:00:00.000Z",
    });

    assert.equal(report.status, "failed");
    assert.equal(report.passedCount, 0);
    assert.equal(report.failedCount, 1);
    assert.equal(report.routes[0].status, "failed");
    assert.equal(report.routes[0].actualStatus, 404);
    assert.deepEqual(report.routes[0].expectedStatuses, [200]);
    assert.match(report.routes[0].evidence, /expected 200 but received 404/);
  } finally {
    await close();
  }
});

test("launch route probe distinguishes an unreachable server from route failures", async () => {
  const { baseUrl, close } = await startFixtureServer((_request, response) => {
    response.writeHead(200, { "content-type": "text/plain" });
    response.end("closing");
  });
  await close();

  const report = await probeLaunchRoutes({
    baseUrl,
    routes: [{ path: "/", label: "launch page", expectedStatuses: [200] }],
    generatedAt: "2026-05-23T00:00:00.000Z",
  });

  assert.equal(report.status, "failed");
  assert.equal(report.connectivity.status, "unreachable");
  assert.equal(report.connectivity.failureKind, "connection-refused");
  assert.equal(report.connectivity.failedConnectionCount, 1);
  assert.match(report.connectivity.evidence, /server did not accept TCP connections/);
});

test("launch route probe candidates select the first reachable launch server", async () => {
  const closed = await startFixtureServer((_request, response) => {
    response.writeHead(200, { "content-type": "text/plain" });
    response.end("closing");
  });
  await closed.close();

  const { baseUrl, close } = await startFixtureServer((_request, response) => {
    response.writeHead(200, { "content-type": "text/plain" });
    response.end("launch");
  });

  try {
    const report = await probeLaunchRouteCandidates({
      baseUrls: [closed.baseUrl, baseUrl],
      routes: [{ path: "/", label: "launch page", expectedStatuses: [200] }],
      generatedAt: "2026-05-23T00:00:00.000Z",
    });

    assert.equal(report.schema, "dx.www.launchStabilize.routeProbe.candidates");
    assert.equal(report.format, 1);
    assert.equal(report.status, "passed");
    assert.equal(report.selectedBaseUrl, baseUrl);
    assert.equal(report.selectedReport.baseUrl, baseUrl);
    assert.equal(report.attempts.length, 2);
    assert.equal(report.attempts[0].baseUrl, closed.baseUrl);
    assert.equal(report.attempts[0].connectivity.status, "unreachable");
    assert.equal(report.attempts[1].status, "passed");
    assert.doesNotMatch(JSON.stringify(report), /\.v1\b/);
  } finally {
    await close();
  }
});

function startFixtureServer(handler) {
  return new Promise((resolve, reject) => {
    const server = http.createServer(handler);
    server.once("error", reject);
    server.listen(0, "127.0.0.1", () => {
      const address = server.address();
      resolve({
        baseUrl: `http://127.0.0.1:${address.port}`,
        close: () =>
          new Promise((closeResolve, closeReject) => {
            server.close((error) => (error ? closeReject(error) : closeResolve()));
          }),
      });
    });
  });
}
