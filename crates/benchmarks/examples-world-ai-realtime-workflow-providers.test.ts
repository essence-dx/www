import { describe, it } from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";

import {
  aiRealtimeWorkflowProviderDefinitions,
  probeAiRealtimeWorkflowProvider,
} from "../examples/world/lib/world/connections/providers/ai-realtime-workflows.ts";
import { GET as getWorldProviderStatus } from "../examples/world/app/api/world/provider/route.ts";

describe("examples/world lane 3 AI realtime workflow provider probes", () => {
  it("exposes Pusher in the AI realtime workflow lane card registry", () => {
    const laneSource = readFileSync(
      "examples/world/lib/world/lanes/ai-realtime-workflows.ts",
      "utf8",
    );

    assert.match(laneSource, /id: "pusher"/);
    assert.match(laneSource, /name: "Pusher"/);
    assert.match(laneSource, /PUSHER_APP_ID/);
    assert.match(laneSource, /PUSHER_SECRET/);
  });

  it("declares the requested AI, realtime, and workflow provider adapters", () => {
    assert.deepEqual(
      aiRealtimeWorkflowProviderDefinitions.map((provider) => provider.providerId),
      [
        "openai",
        "anthropic",
        "google-gemini",
        "ably",
        "pusher",
        "supabase-realtime",
        "upstash-qstash",
        "temporal",
        "cloudflare-queues",
      ],
    );

    const source = JSON.stringify(aiRealtimeWorkflowProviderDefinitions);

    for (const envName of [
      "OPENAI_API_KEY",
      "ANTHROPIC_API_KEY",
      "GOOGLE_GENERATIVE_AI_API_KEY",
      "ABLY_API_KEY",
      "PUSHER_APP_ID",
      "PUSHER_KEY",
      "PUSHER_SECRET",
      "PUSHER_CLUSTER",
      "SUPABASE_URL",
      "SUPABASE_SERVICE_ROLE_KEY",
      "QSTASH_TOKEN",
      "TEMPORAL_ADDRESS",
      "TEMPORAL_NAMESPACE",
      "CLOUDFLARE_ACCOUNT_ID",
      "CLOUDFLARE_API_TOKEN",
    ]) {
      assert.match(source, new RegExp(envName));
    }

    for (const endpoint of [
      "https://api.openai.com/v1/models",
      "https://api.anthropic.com/v1/models",
      "https://generativelanguage.googleapis.com/v1beta/models",
      "https://rest.ably.io/channels",
      "https://api-{cluster}.pusher.com/apps/{app_id}/channels",
      "wss://{project-ref}.supabase.co/realtime/v1/websocket",
      "https://qstash.upstash.io/v2/schedules",
      "https://{namespace-id}.tmprl.cloud:7233",
      "https://api.cloudflare.com/client/v4/accounts/{account_id}/queues",
    ]) {
      assert.match(source, new RegExp(endpoint.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
    }
  });

  it("returns missing-config without touching the network or exposing secrets", async () => {
    let fetchCalls = 0;

    for (const provider of aiRealtimeWorkflowProviderDefinitions) {
      const result = await probeAiRealtimeWorkflowProvider(provider.providerId, {
        env: {},
        fetch: async () => {
          fetchCalls += 1;
          return Response.json({ shouldNotRun: true }, { status: 500 });
        },
      });

      assert.equal(result.status, "missing-config");
      assert.equal(result.providerId, provider.providerId);
      assert.equal(result.liveProviderExecution, false);
      assert.equal(result.liveReadOnlyRequest, false);
      assert.deepEqual(result.secretValues, []);
      assert.ok(result.missingEnv.length > 0);
    }

    assert.equal(fetchCalls, 0);
  });

  it("performs a read-only OpenAI model-list probe when credentials and fetch are present", async () => {
    const requests: Request[] = [];
    const result = await probeAiRealtimeWorkflowProvider("openai", {
      env: {
        OPENAI_API_KEY: "openai-secret",
      },
      fetch: async (input, init) => {
        requests.push(new Request(input, init));
        return Response.json({ data: [] }, { status: 200 });
      },
    });

    assert.equal(result.status, "live-validated");
    assert.equal(result.liveProviderExecution, true);
    assert.equal(result.liveReadOnlyRequest, true);
    assert.equal(result.operation, "read-only");
    assert.equal(result.httpStatus, 200);
    assert.equal(requests.length, 1);
    assert.equal(requests[0].method, "GET");
    assert.equal(requests[0].url, "https://api.openai.com/v1/models");
    assert.equal(requests[0].headers.get("authorization"), "Bearer openai-secret");
    assert.deepEqual(result.secretValues, []);
    assert.doesNotMatch(JSON.stringify(result), /openai-secret/);
  });

  it("keeps providers without universal read-only HTTP probes in configured readiness", async () => {
    let fetchCalls = 0;
    const result = await probeAiRealtimeWorkflowProvider("supabase-realtime", {
      env: {
        SUPABASE_URL: "https://project-ref.supabase.co",
        SUPABASE_SERVICE_ROLE_KEY: "supabase-secret",
      },
      fetch: async () => {
        fetchCalls += 1;
        return Response.json({ shouldNotRun: true }, { status: 500 });
      },
    });

    assert.equal(result.status, "configured-readiness");
    assert.equal(result.liveProviderExecution, false);
    assert.equal(result.liveReadOnlyRequest, false);
    assert.equal(fetchCalls, 0);
    assert.deepEqual(result.secretValues, []);
    assert.match(result.blockers.join("\n"), /no-safe-read-only-http-endpoint/);
    assert.doesNotMatch(JSON.stringify(result), /supabase-secret/);
  });

  it("exposes missing-config through the world provider route for lane 3 providers", async () => {
    const response = await getWorldProviderStatus(
      new Request("https://dx.local/api/world/provider?provider=openai"),
    );
    const payload = await response.json();

    assert.equal(response.status, 428);
    assert.equal(payload.providerId, "openai");
    assert.equal(payload.status, "missing-config");
    assert.deepEqual(payload.missingEnv, ["OPENAI_API_KEY"]);
    assert.deepEqual(payload.secretValues, []);
    assert.equal(payload.liveProviderExecution, false);
  });
});
