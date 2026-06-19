import { readFileSync } from "node:fs";
import { join } from "node:path";
import { describe, it } from "node:test";
import assert from "node:assert/strict";

import {
  firebaseConnectionProbes,
  runFirebaseFirestoreCrudSmoke,
  worldConnectionProbes,
} from "../examples/world/lib/world/connections/providers/index.ts";
import { hasLeakedEnvValue } from "../examples/world/lib/world/connections/redaction.ts";

const repoRoot = process.cwd();
const apiKey = "firebase-api-key-never-print";

describe("examples/world Firebase Firestore CRUD integration", () => {
  it("keeps Firebase Firestore CRUD disabled when project config is missing", async () => {
    let fetchCalls = 0;
    const receipt = await runFirebaseFirestoreCrudSmoke({
      env: {},
      fetch: async () => {
        fetchCalls += 1;
        return new Response(null, { status: 500 });
      },
      now: () => new Date("2026-06-02T00:00:00.000Z"),
    });

    assert.equal(fetchCalls, 0);
    assert.equal(receipt.status, "missing-config");
    assert.deepEqual(receipt.missingEnv, ["FIREBASE_PROJECT_ID", "FIREBASE_API_KEY"]);
    assert.equal(receipt.liveProviderExecution, false);
    assert.deepEqual(receipt.secretValues, []);
  });

  it("runs create, read, update, and delete through Firestore REST without leaking config values", async () => {
    const requests: Array<{ url: string; method: string; headers: Headers; body: string }> = [];

    const receipt = await runFirebaseFirestoreCrudSmoke({
      env: {
        FIREBASE_PROJECT_ID: "firebase-project",
        FIREBASE_API_KEY: apiKey,
      },
      collection: "dx_www_crud_test",
      documentId: "check-test",
      now: () => new Date("2026-06-02T00:00:00.000Z"),
      fetch: async (input, init) => {
        const request = new Request(input, init);
        requests.push({
          url: request.url,
          method: request.method,
          headers: request.headers,
          body: init?.body?.toString() ?? "",
        });
        return Response.json({ name: "projects/firebase-project/databases/(default)/documents/doc" });
      },
    });

    assert.equal(receipt.status, "live-validated");
    assert.equal(receipt.liveProviderExecution, true);
    assert.deepEqual(
      receipt.steps.map((step) => step.name),
      ["create-document", "read-document", "update-document", "delete-document"],
    );
    assert.deepEqual(
      requests.map((request) => request.method),
      ["POST", "GET", "PATCH", "DELETE"],
    );
    assert.match(requests[0]?.url ?? "", /firestore\.googleapis\.com\/v1\/projects\/firebase-project/);
    assert.match(requests[0]?.url ?? "", /documentId=check-test/);
    assert.match(requests[2]?.url ?? "", /updateMask\.fieldPaths=title/);
    assert.match(requests[2]?.url ?? "", /updateMask\.fieldPaths=counter/);
    assert.equal(requests[0]?.headers.get("Content-Type"), "application/json");
    assert.match(requests[0]?.body ?? "", /created-from-dx-www/);
    assert.match(requests[2]?.body ?? "", /updated-from-dx-www/);
    assert.equal(hasLeakedEnvValue(receipt, { FIREBASE_API_KEY: apiKey }), false);
    assert.doesNotMatch(JSON.stringify(receipt), /firebase-api-key-never-print/);
  });

  it("reports Firebase security-rule blocks honestly instead of promoting failed CRUD", async () => {
    const receipt = await runFirebaseFirestoreCrudSmoke({
      env: {
        FIREBASE_PROJECT_ID: "firebase-project",
        FIREBASE_API_KEY: apiKey,
      },
      collection: "dx_www_crud_test",
      documentId: "check-test",
      now: () => new Date("2026-06-02T00:00:00.000Z"),
      fetch: async () => Response.json({ error: { status: "PERMISSION_DENIED" } }, { status: 403 }),
    });

    assert.equal(receipt.status, "blocked");
    assert.equal(receipt.liveProviderExecution, true);
    assert.equal(receipt.steps.every((step) => step.ok), false);
    assert.match(receipt.nextAction, /rules\/auth|Admin\/service-account/);
  });

  it("registers Firebase Firestore as a safe read-only world probe", () => {
    const probeIds = worldConnectionProbes.map((probe) => probe.id);

    assert.ok(probeIds.includes("firebase-firestore-document-readiness"));
    assert.equal(firebaseConnectionProbes[0]?.requiredEnv.includes("FIREBASE_API_KEY"), true);
    assert.equal(
      firebaseConnectionProbes[0]?.endpoint,
      "env:FIREBASE_PROJECT_ID/firestore/documents",
    );
  });

  it("keeps the route contract guarded behind explicit POST confirmation", () => {
    const route = readFileSync(
      join(repoRoot, "examples/world/app/api/world/firebase-crud/route.ts"),
      "utf8",
    );

    assert.match(route, /export async function POST/);
    assert.match(route, /DX_WORLD_ALLOW_FIREBASE_CRUD/);
    assert.match(route, /x-dx-world-confirm/);
    assert.doesNotMatch(route, /AIza[0-9A-Za-z_-]+/);
  });

  it(
    "can execute the live Firebase Firestore CRUD smoke when explicitly enabled",
    { skip: process.env.DX_WORLD_LIVE_FIREBASE_CRUD !== "1" },
    async () => {
      const receipt = await runFirebaseFirestoreCrudSmoke({
        env: process.env,
        now: () => new Date(),
      });

      assert.equal(hasLeakedEnvValue(receipt, process.env), false);
      assert.ok(["live-validated", "blocked"].includes(receipt.status));
    },
  );
});
