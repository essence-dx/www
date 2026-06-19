import { readFileSync } from "node:fs";
import { join } from "node:path";
import { describe, it } from "node:test";
import assert from "node:assert/strict";

import {
  runTursoDatabaseCrudSmoke,
  tursoConnectionProbes,
} from "../examples/world/lib/world/connections/providers/index.ts";
import { hasLeakedEnvValue } from "../examples/world/lib/world/connections/redaction.ts";

const repoRoot = process.cwd();
const databaseUrl = "libsql://database.example.turso.io";
const token = "turso-token-never-print";

describe("examples/world Turso CRUD integration", () => {
  it("keeps Turso database CRUD disabled when credentials are missing", async () => {
    let fetchCalls = 0;
    const receipt = await runTursoDatabaseCrudSmoke({
      env: {},
      fetch: async () => {
        fetchCalls += 1;
        return new Response(null, { status: 500 });
      },
      now: () => new Date("2026-06-02T00:00:00.000Z"),
    });

    assert.equal(fetchCalls, 0);
    assert.equal(receipt.status, "missing-config");
    assert.deepEqual(receipt.missingEnv, ["TURSO_DATABASE_URL", "TURSO_AUTH_TOKEN"]);
    assert.equal(receipt.liveProviderExecution, false);
    assert.deepEqual(receipt.secretValues, []);
  });

  it("runs create, read, update, and delete through the libSQL HTTP pipeline without leaking tokens", async () => {
    const requests: Array<{ url: string; headers: Headers; body: string }> = [];

    const receipt = await runTursoDatabaseCrudSmoke({
      env: {
        TURSO_DATABASE_URL: databaseUrl,
        TURSO_AUTH_TOKEN: token,
      },
      tableName: "dx_www_crud_test",
      rowId: "dx-www-test-row",
      now: () => new Date("2026-06-02T00:00:00.000Z"),
      fetch: async (input, init) => {
        const request = new Request(input, init);
        requests.push({
          url: request.url,
          headers: request.headers,
          body: init?.body?.toString() ?? "",
        });
        return Response.json({
          results: [
            okResult({ rows_written: 3 }),
            okResult({ affected_row_count: 1, rows_written: 1 }),
            okResult({ rows_read: 1 }),
            okResult({ affected_row_count: 1, rows_written: 1 }),
            okResult({ rows_read: 1 }),
            okResult({ affected_row_count: 1, rows_written: 1 }),
            okResult({ rows_written: 3 }),
            { type: "ok", response: { type: "close" } },
          ],
        });
      },
    });

    assert.equal(receipt.status, "live-validated");
    assert.equal(receipt.liveProviderExecution, true);
    assert.deepEqual(
      receipt.steps.map((step) => step.name),
      [
        "create-table",
        "insert-row",
        "read-row",
        "update-row",
        "read-updated-row",
        "delete-row",
        "drop-table",
      ],
    );
    assert.equal(requests.length, 1);
    assert.equal(requests[0]?.url, "https://database.example.turso.io/v2/pipeline");
    assert.equal(requests[0]?.headers.get("Authorization"), `Bearer ${token}`);
    assert.match(requests[0]?.body ?? "", /CREATE TABLE IF NOT EXISTS/);
    assert.match(requests[0]?.body ?? "", /INSERT INTO/);
    assert.match(requests[0]?.body ?? "", /SELECT id, title, counter/);
    assert.match(requests[0]?.body ?? "", /UPDATE/);
    assert.match(requests[0]?.body ?? "", /DELETE FROM/);
    assert.match(requests[0]?.body ?? "", /DROP TABLE IF EXISTS/);
    assert.match(requests[0]?.body ?? "", /"args":/);
    assert.equal(hasLeakedEnvValue(receipt, { TURSO_AUTH_TOKEN: token }), false);
    assert.doesNotMatch(JSON.stringify(receipt), /turso-token-never-print/);
  });

  it("reports pipeline errors honestly instead of promoting failed CRUD", async () => {
    const receipt = await runTursoDatabaseCrudSmoke({
      env: {
        TURSO_DATABASE_URL: databaseUrl,
        TURSO_AUTH_TOKEN: token,
      },
      tableName: "dx_www_crud_test",
      rowId: "dx-www-test-row",
      now: () => new Date("2026-06-02T00:00:00.000Z"),
      fetch: async () =>
        Response.json({
          results: [
            okResult({ rows_written: 3 }),
            { type: "error", error: { message: "permission denied" } },
          ],
        }),
    });

    assert.equal(receipt.status, "blocked");
    assert.equal(receipt.liveProviderExecution, true);
    assert.equal(receipt.steps[0]?.ok, true);
    assert.equal(receipt.steps[1]?.ok, false);
    assert.match(receipt.nextAction, /failed Turso\/libSQL HTTP pipeline step/);
  });

  it("keeps the existing Turso SELECT 1 probe registered", () => {
    const probeIds = tursoConnectionProbes.map((probe) => probe.id);

    assert.ok(probeIds.includes("turso-sql-over-http-select-one"));
  });

  it("keeps the route contract guarded behind explicit POST confirmation", () => {
    const route = readFileSync(
      join(repoRoot, "examples/world/app/api/world/turso-crud/route.ts"),
      "utf8",
    );

    assert.match(route, /export async function POST/);
    assert.match(route, /DX_WORLD_ALLOW_TURSO_CRUD/);
    assert.match(route, /x-dx-world-confirm/);
    assert.doesNotMatch(route, /Bearer /);
  });

  it(
    "can execute the live Turso database CRUD smoke when explicitly enabled",
    { skip: process.env.DX_WORLD_LIVE_TURSO_CRUD !== "1" },
    async () => {
      const receipt = await runTursoDatabaseCrudSmoke({
        env: process.env,
        now: () => new Date(),
      });

      assert.equal(receipt.status, "live-validated");
      assert.equal(receipt.steps.every((step) => step.ok), true);
      assert.equal(hasLeakedEnvValue(receipt, process.env), false);
    },
  );
});

function okResult(result: {
  affected_row_count?: number;
  rows_read?: number;
  rows_written?: number;
}) {
  return {
    type: "ok",
    response: {
      type: "execute",
      result: {
        cols: [],
        rows: [],
        affected_row_count: result.affected_row_count ?? 0,
        last_insert_rowid: null,
        rows_read: result.rows_read ?? 0,
        rows_written: result.rows_written ?? 0,
      },
    },
  };
}
