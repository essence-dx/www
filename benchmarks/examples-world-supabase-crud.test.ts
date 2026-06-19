import { readFileSync } from "node:fs";
import { join } from "node:path";
import { describe, it } from "node:test";
import assert from "node:assert/strict";

import {
  runSupabaseStorageCrudSmoke,
  supabaseConnectionProbes,
  worldConnectionProbes,
} from "../examples/world/lib/world/connections/providers/index.ts";
import { hasLeakedEnvValue } from "../examples/world/lib/world/connections/redaction.ts";

const repoRoot = process.cwd();
const secret = "supabase-secret-never-print";
const projectUrl = "https://project-ref.supabase.co";

describe("examples/world Supabase CRUD integration", () => {
  it("keeps Supabase Storage CRUD disabled when server credentials are missing", async () => {
    let fetchCalls = 0;
    const receipt = await runSupabaseStorageCrudSmoke({
      env: {},
      fetch: async () => {
        fetchCalls += 1;
        return new Response(null, { status: 500 });
      },
      now: () => new Date("2026-06-02T00:00:00.000Z"),
    });

    assert.equal(fetchCalls, 0);
    assert.equal(receipt.status, "missing-config");
    assert.deepEqual(receipt.missingEnv, ["SUPABASE_URL", "SUPABASE_SECRET_KEY"]);
    assert.equal(receipt.liveProviderExecution, false);
    assert.deepEqual(receipt.secretValues, []);
  });

  it("runs create, read, update, and delete through Supabase Storage REST without leaking secrets", async () => {
    const requests: Array<{ url: string; method: string; headers: Headers; body: string }> = [];

    const receipt = await runSupabaseStorageCrudSmoke({
      env: {
        SUPABASE_URL: projectUrl,
        SUPABASE_SECRET_KEY: secret,
      },
      bucket: "dx-www-crud-test",
      objectPath: "checks/supabase-crud-test.json",
      now: () => new Date("2026-06-02T00:00:00.000Z"),
      fetch: async (input, init) => {
        const request = new Request(input, init);
        requests.push({
          url: request.url,
          method: request.method,
          headers: request.headers,
          body: init?.body?.toString() ?? "",
        });
        return Response.json({ ok: true }, { status: 200 });
      },
    });

    assert.equal(receipt.status, "live-validated");
    assert.equal(receipt.liveProviderExecution, true);
    assert.deepEqual(
      receipt.steps.map((step) => step.name),
      [
        "list-buckets",
        "create-bucket",
        "create-object",
        "read-object",
        "update-object",
        "delete-object",
        "delete-bucket",
      ],
    );
    assert.deepEqual(
      requests.map((request) => request.method),
      ["GET", "POST", "POST", "GET", "POST", "DELETE", "DELETE"],
    );
    assert.equal(
      requests[2]?.url,
      "https://project-ref.supabase.co/storage/v1/object/dx-www-crud-test/checks/supabase-crud-test.json",
    );
    assert.equal(requests[2]?.headers.get("x-upsert"), "true");
    assert.match(requests[5]?.body ?? "", /checks\/supabase-crud-test\.json/);
    assert.equal(hasLeakedEnvValue(receipt, { SUPABASE_SECRET_KEY: secret }), false);
    assert.doesNotMatch(JSON.stringify(receipt), /supabase-secret-never-print/);
  });

  it("registers Supabase bucket access as a safe read-only world probe", () => {
    const probeIds = worldConnectionProbes.map((probe) => probe.id);

    assert.ok(probeIds.includes("supabase-storage-buckets-readiness"));
    assert.equal(supabaseConnectionProbes[0]?.requiredEnv.includes("SUPABASE_SECRET_KEY"), true);
    assert.equal(
      supabaseConnectionProbes[0]?.endpoint,
      "env:SUPABASE_URL/storage/v1/bucket",
    );
  });

  it("keeps the route contract guarded behind explicit POST confirmation", () => {
    const route = readFileSync(
      join(repoRoot, "examples/world/app/api/world/supabase-crud/route.ts"),
      "utf8",
    );

    assert.match(route, /export async function POST/);
    assert.match(route, /DX_WORLD_ALLOW_SUPABASE_CRUD/);
    assert.match(route, /x-dx-world-confirm/);
    assert.doesNotMatch(route, /sb_secret_|sb_publishable_/);
  });

  it(
    "can execute the live Supabase CRUD smoke when explicitly enabled",
    { skip: process.env.DX_WORLD_LIVE_SUPABASE_CRUD !== "1" },
    async () => {
      const receipt = await runSupabaseStorageCrudSmoke({
        env: process.env,
        now: () => new Date(),
      });

      assert.equal(receipt.status, "live-validated");
      assert.equal(receipt.steps.every((step) => step.ok), true);
      assert.equal(hasLeakedEnvValue(receipt, process.env), false);
    },
  );
});
