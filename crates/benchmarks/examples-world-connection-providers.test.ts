import { describe, it } from "node:test";
import assert from "node:assert/strict";

import {
  checkWorldConnectionProvider,
  worldConnectionProviders,
} from "../examples/world/lib/world/connections/providers/index.ts";

describe("examples/world commerce, media, search, and vector connection providers", () => {
  it("declares read-only provider checks for the lane 2 provider set", () => {
    const providerIds = worldConnectionProviders.map((provider) => provider.id);
    const lane2ProviderIds = [
      "stripe",
      "lemon-squeezy",
      "paddle",
      "aws-s3",
      "cloudflare-r2",
      "vercel-blob",
      "algolia",
      "meilisearch",
      "typesense",
      "pinecone",
    ];

    assert.deepEqual(providerIds.slice(0, lane2ProviderIds.length), lane2ProviderIds);

    for (const providerId of lane2ProviderIds) {
      assert.ok(providerIds.includes(providerId), `${providerId} must remain registered`);
    }

    for (const provider of worldConnectionProviders) {
      assert.ok(provider.requiredEnv.length > 0, `${provider.id} must declare required env names`);
      assert.match(provider.receiptSchema, /^dx\.forge\.world\./);
      assert.doesNotMatch(provider.readiness.method, /POST|PUT|PATCH|DELETE/);
      assert.equal(provider.secretRedaction, "secret-values-never-included");
    }
  });

  it("returns missing-config without touching the network when credentials are absent", async () => {
    let fetchCalls = 0;
    const result = await checkWorldConnectionProvider("stripe", {
      env: {},
      fetch: async () => {
        fetchCalls += 1;
        return new Response("should not run", { status: 500 });
      },
    });

    assert.equal(fetchCalls, 0);
    assert.equal(result.status, "missing-config");
    assert.equal(result.providerId, "stripe");
    assert.deepEqual(result.missingEnv, ["STRIPE_SECRET_KEY"]);
    assert.deepEqual(result.secretValues, []);
    assert.equal(result.liveProviderExecution, false);
  });

  it("performs only a read-only request when required credentials are present", async () => {
    const requests: Request[] = [];
    const result = await checkWorldConnectionProvider("meilisearch", {
      env: {
        MEILISEARCH_HOST: "https://search.example.test",
        MEILISEARCH_API_KEY: "redacted-test-key",
      },
      fetch: async (input, init) => {
        requests.push(new Request(input, init));
        return Response.json({ results: [] }, { status: 200 });
      },
    });

    assert.equal(result.status, "live-validated");
    assert.equal(result.liveProviderExecution, true);
    assert.equal(result.httpStatus, 200);
    assert.deepEqual(result.secretValues, []);
    assert.equal(requests.length, 1);
    assert.equal(requests[0].method, "GET");
    assert.equal(requests[0].url, "https://search.example.test/indexes?limit=1");
  });

  it("keeps provider failures honest instead of promoting failed checks", async () => {
    const result = await checkWorldConnectionProvider("pinecone", {
      env: {
        PINECONE_API_KEY: "redacted-test-key",
        PINECONE_INDEX: "docs",
      },
      fetch: async () => Response.json({ message: "forbidden" }, { status: 403 }),
    });

    assert.equal(result.status, "provider-error");
    assert.equal(result.httpStatus, 403);
    assert.equal(result.liveProviderExecution, true);
    assert.deepEqual(result.secretValues, []);
  });
});
