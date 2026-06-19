import { describe, it } from "node:test";
import assert from "node:assert/strict";

import {
  probeAuthProvider,
  probeDatabaseProvider,
  worldConnectionProbes,
} from "../examples/world/lib/world/connections/providers/index.ts";

describe("examples/world data and identity provider probes", () => {
  it("keeps missing database config redacted", async () => {
    const result = await probeDatabaseProvider("postgresql", {
      env: {
        DATABASE_URL: "postgres://secret-user:secret-pass@example.local/db",
      },
    });

    assert.equal(result.status, "configured-readiness");
    assert.deepEqual(result.presentEnv, ["DATABASE_URL"]);
    assert.doesNotMatch(JSON.stringify(result), /secret-pass/);
  });

  it("runs Turso SELECT 1 through the safe HTTP pipeline when credentials exist", async () => {
    const calls: Array<{ url: string; init: { body?: string; headers?: Record<string, string> } }> = [];

    const result = await probeDatabaseProvider("turso-libsql", {
      env: {
        TURSO_DATABASE_URL: "libsql://example-org-example-db.turso.io",
        TURSO_AUTH_TOKEN: "turso-secret",
      },
      fetch: async (url, init) => {
        calls.push({ url, init });
        return new Response("{}", { status: 200, headers: { "content-type": "application/json" } });
      },
    });

    assert.equal(result.status, "live-validated");
    assert.equal(result.probe.kind, "turso-libsql-http-select-1");
    assert.equal(calls.length, 1);
    assert.equal(calls[0]?.url, "https://example-org-example-db.turso.io/v2/pipeline");
    assert.match(calls[0]?.init.body ?? "", /SELECT 1/);
    assert.equal(calls[0]?.init.headers?.Authorization, "Bearer turso-secret");
    assert.doesNotMatch(JSON.stringify(result), /turso-secret/);
  });

  it("uses configured readiness for hosted auth until a local status endpoint exists", async () => {
    let fetchCount = 0;

    const result = await probeAuthProvider("better-auth", {
      env: {
        BETTER_AUTH_SECRET: "better-auth-secret",
        BETTER_AUTH_URL: "https://app.example.com",
        BETTER_AUTH_STATUS_URL: "https://app.example.com/api/auth/readiness",
      },
      fetch: async () => {
        fetchCount += 1;
        return new Response(null, { status: 200 });
      },
    });

    assert.equal(result.status, "configured-readiness");
    assert.equal(fetchCount, 0);
    assert.doesNotMatch(JSON.stringify(result), /better-auth-secret/);
  });

  it("probes local auth readiness endpoints without sending secrets", async () => {
    const headers: Array<HeadersInit | undefined> = [];

    const result = await probeAuthProvider("auth-js", {
      env: {
        AUTH_SECRET: "auth-js-secret",
        AUTH_URL: "http://127.0.0.1:3000",
        AUTHJS_STATUS_URL: "http://127.0.0.1:3000/api/auth/readiness",
      },
      fetch: async (_url, init) => {
        headers.push(init?.headers);
        return new Response(null, { status: 204 });
      },
    });

    assert.equal(result.status, "live-validated");
    assert.equal(result.probe.kind, "local-status-endpoint");
    assert.deepEqual(headers[0], { "Cache-Control": "no-store" });
    assert.doesNotMatch(JSON.stringify(result), /auth-js-secret/);
  });

  it("exposes data and auth probes for the shared world connection runner", () => {
    const probeIds = worldConnectionProbes.map((probe) => probe.id);

    assert.ok(probeIds.includes("supabase-config-readiness"));
    assert.ok(probeIds.includes("better-auth-local-status"));
    assert.ok(probeIds.includes("auth-js-local-status"));
  });
});
