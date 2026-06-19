import assert from "node:assert/strict";
import test from "node:test";

import {
  checkWorldConnectionProvider,
  worldConnectionProviders,
} from "../examples/world/lib/world/connections/providers/index.ts";
import type { WorldConnectionProvider } from "../examples/world/lib/world/connections/providers/types.ts";

const lane5ProviderIds = [
  "vercel",
  "cloudflare",
  "fly-io",
  "netlify",
  "sanity",
  "strapi",
  "contentful",
  "resend",
  "twilio",
  "firebase-cloud-messaging",
] as const;

function lane5Providers(): readonly WorldConnectionProvider[] {
  const byId = new Map(worldConnectionProviders.map((provider) => [provider.id, provider]));
  return lane5ProviderIds.map((id) => {
    const provider = byId.get(id);
    assert.ok(provider, `missing lane 5 provider probe: ${id}`);
    return provider;
  });
}

test("lane 5 declares content, deployment, and notification provider probes", () => {
  const providers = lane5Providers();

  assert.deepEqual(providers.map((provider) => provider.id), [
    "vercel",
    "cloudflare",
    "fly-io",
    "netlify",
    "sanity",
    "strapi",
    "contentful",
    "resend",
    "twilio",
    "firebase-cloud-messaging",
  ]);

  for (const provider of providers) {
    assert.ok(provider.requiredEnv.length > 0, `${provider.id} must declare credential env`);
    assert.match(provider.receiptSchema, /^dx\.forge\.world\./);
    assert.equal(provider.secretRedaction, "secret-values-never-included");
    assert.match(provider.readiness.endpointLabel, /^(GET|HEAD|local-cli:|env:)/);
  }
});

test("missing credentials return missing-config without provider reads", async () => {
  let fetchCalls = 0;

  const results = await Promise.all(
    lane5ProviderIds.map((providerId) =>
      checkWorldConnectionProvider(providerId, {
        env: {},
        fetch: async () => {
          fetchCalls += 1;
          return new Response("unexpected", { status: 500 });
        },
      }),
    ),
  );

  assert.equal(fetchCalls, 0);
  assert.deepEqual(
    results.map((result) => result.status),
    lane5ProviderIds.map(() => "missing-config"),
  );
  assert.equal(JSON.stringify(results).includes("cloudflare-token"), false);
});

test("configured providers use read-only metadata endpoints", async () => {
  const requests: Array<{ method: string; url: string; body?: string }> = [];
  const env = {
    CLOUDFLARE_API_TOKEN: "cloudflare-token",
    FLY_API_TOKEN: "fly-token",
    FLY_APP_NAME: "dx-app",
    NETLIFY_AUTH_TOKEN: "netlify-token",
    SANITY_DATASET: "production",
    SANITY_PROJECT_ID: "project",
    STRAPI_API_TOKEN: "strapi-token",
    STRAPI_URL: "https://cms.example.test",
    CONTENTFUL_DELIVERY_TOKEN: "contentful-token",
    CONTENTFUL_ENVIRONMENT: "master",
    CONTENTFUL_SPACE_ID: "space",
    RESEND_API_KEY: "resend-token",
    TWILIO_ACCOUNT_SID: "AC123",
    TWILIO_AUTH_TOKEN: "twilio-token",
    VERCEL_TOKEN: "vercel-token",
  };

  const providerIds = [
    "vercel",
    "cloudflare",
    "fly-io",
    "netlify",
    "sanity",
    "strapi",
    "contentful",
    "resend",
    "twilio",
  ] as const;
  const results = await Promise.all(
    providerIds.map((providerId) =>
      checkWorldConnectionProvider(providerId, {
        env,
        fetch: async (url, init) => {
          requests.push({
            method: init?.method ?? "GET",
            url: url.toString(),
            body: typeof init?.body === "string" ? init.body : undefined,
          });
          return new Response("{}", { status: 200, headers: { "content-type": "application/json" } });
        },
      }),
    ),
  );

  assert.deepEqual(
    results.map((result) => result.status),
    providerIds.map(() => "live-validated"),
  );
  assert.deepEqual(
    requests.map((request) => request.method),
    ["GET", "GET", "GET", "GET", "GET", "GET", "GET", "GET", "GET"],
  );
  assert.match(requests[0].url, /^https:\/\/api\.vercel\.com\/v2\/user/);
  assert.match(requests[1].url, /^https:\/\/api\.cloudflare\.com\/client\/v4\/user\/tokens\/verify/);
  assert.match(requests[2].url, /^https:\/\/api\.machines\.dev\/v1\/apps\/dx-app/);
  assert.match(requests[3].url, /^https:\/\/api\.netlify\.com\/api\/v1\/user/);
  assert.match(requests[4].url, /^https:\/\/project\.api\.sanity\.io\//);
  assert.match(requests[5].url, /^https:\/\/cms\.example\.test\/api/);
  assert.match(requests[6].url, /^https:\/\/cdn\.contentful\.com\/spaces\/space\//);
  assert.match(requests[7].url, /^https:\/\/api\.resend\.com\/domains/);
  assert.match(requests[8].url, /^https:\/\/api\.twilio\.com\/2010-04-01\/Accounts\/AC123\.json/);
});

test("FCM remains configured-readiness because safe read-only delivery proof is unavailable", async () => {
  let fetchCalls = 0;
  const result = await checkWorldConnectionProvider("firebase-cloud-messaging", {
    env: {
      FCM_PROJECT_ID: "dx-project",
      FCM_SERVICE_ACCOUNT_JSON: '{"client_email":"bot@example.com","private_key":"redacted"}',
    },
    fetch: async () => {
      fetchCalls += 1;
      return new Response("unexpected", { status: 500 });
    },
  });

  assert.equal(fetchCalls, 0);
  assert.equal(result.status, "configured-readiness");
  assert.match(result.nextAction, /operator-approved test token/);
});
