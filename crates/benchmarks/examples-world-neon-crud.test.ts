import { readFileSync } from "node:fs";
import { join } from "node:path";
import { describe, it } from "node:test";
import assert from "node:assert/strict";

import {
  neonConnectionProbes,
  runNeonDatabaseCrudSmoke,
  worldConnectionProbes,
} from "../examples/world/lib/world/connections/providers/index.ts";
import { hasLeakedEnvValue } from "../examples/world/lib/world/connections/redaction.ts";

const repoRoot = process.cwd();
const connectionString =
  "postgresql://user:password-never-print@ep-flat-bar-a539j7gm-pooler.us-east-2.aws.neon.tech/app?sslmode=require";

describe("examples/world Neon CRUD integration", () => {
  it("keeps Neon database CRUD disabled when database env is missing", async () => {
    let fetchCalls = 0;
    const receipt = await runNeonDatabaseCrudSmoke({
      env: {},
      fetch: async () => {
        fetchCalls += 1;
        return new Response(null, { status: 500 });
      },
      now: () => new Date("2026-06-02T00:00:00.000Z"),
    });

    assert.equal(fetchCalls, 0);
    assert.equal(receipt.status, "missing-config");
    assert.deepEqual(receipt.missingEnv, ["NEON_DATABASE_URL or DATABASE_URL"]);
    assert.equal(receipt.liveProviderExecution, false);
    assert.deepEqual(receipt.secretValues, []);
  });

  it("runs create, read, update, and delete through Neon HTTP SQL without leaking secrets", async () => {
    const requests: Array<{ url: string; headers: Headers; body: string }> = [];

    const receipt = await runNeonDatabaseCrudSmoke({
      env: {
        DATABASE_URL: connectionString,
      },
      tableName: "dx_www_world_crud_test",
      rowId: "dx-www-test-row",
      now: () => new Date("2026-06-02T00:00:00.000Z"),
      fetch: async (input, init) => {
        const request = new Request(input, init);
        requests.push({
          url: request.url,
          headers: request.headers,
          body: init?.body?.toString() ?? "",
        });
        return Response.json({ fields: [], rows: [], rowCount: 1 }, { status: 200 });
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
    assert.equal(requests.length, 7);
    assert.equal(requests[0]?.url, "https://api.us-east-2.aws.neon.tech/sql");
    assert.equal(requests[0]?.headers.get("Neon-Raw-Text-Output"), "true");
    assert.equal(requests[0]?.headers.get("Neon-Array-Mode"), "true");
    assert.equal(requests[0]?.headers.get("Neon-Connection-String"), connectionString);
    assert.match(requests[0]?.body ?? "", /CREATE TABLE IF NOT EXISTS/);
    assert.match(requests[1]?.body ?? "", /INSERT INTO/);
    assert.match(requests[3]?.body ?? "", /UPDATE/);
    assert.match(requests[5]?.body ?? "", /DELETE FROM/);
    assert.match(requests[6]?.body ?? "", /DROP TABLE IF EXISTS/);
    assert.equal(hasLeakedEnvValue(receipt, { DATABASE_URL: connectionString }), false);
    assert.doesNotMatch(JSON.stringify(receipt), /password-never-print/);
  });

  it("registers Neon HTTP SQL as a safe read-only world probe", () => {
    const probeIds = worldConnectionProbes.map((probe) => probe.id);

    assert.ok(probeIds.includes("neon-http-sql-select-1"));
    assert.equal(neonConnectionProbes[0]?.optionalEnv.includes("DATABASE_URL"), true);
    assert.equal(neonConnectionProbes[0]?.endpoint, "env:NEON_DATABASE_URL/sql");
  });

  it("keeps the route contract guarded behind explicit POST confirmation", () => {
    const route = readFileSync(
      join(repoRoot, "examples/world/app/api/world/neon-crud/route.ts"),
      "utf8",
    );

    assert.match(route, /export async function POST/);
    assert.match(route, /DX_WORLD_ALLOW_NEON_CRUD/);
    assert.match(route, /x-dx-world-confirm/);
    assert.doesNotMatch(route, /npg_[A-Za-z0-9]+|postgresql:\/\/[^'"]+/);
  });

  it(
    "can execute the live Neon database CRUD smoke when explicitly enabled",
    { skip: process.env.DX_WORLD_LIVE_NEON_CRUD !== "1" },
    async () => {
      const receipt = await runNeonDatabaseCrudSmoke({
        env: process.env,
        now: () => new Date(),
      });

      assert.equal(receipt.status, "live-validated");
      assert.equal(receipt.steps.every((step) => step.ok), true);
      assert.equal(hasLeakedEnvValue(receipt, process.env), false);
    },
  );
});
