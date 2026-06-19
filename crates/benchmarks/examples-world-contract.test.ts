import { readFileSync, readdirSync, statSync } from "node:fs";
import { join, relative } from "node:path";
import { describe, it } from "node:test";
import assert from "node:assert/strict";

const repoRoot = process.cwd();
const worldRoot = join(repoRoot, "examples", "world");

function read(path: string): string {
  return readFileSync(join(repoRoot, path), "utf8");
}

function listFiles(root: string): string[] {
  const output: string[] = [];

  for (const entry of readdirSync(root)) {
    const path = join(root, entry);
    const stats = statSync(path);

    if (stats.isDirectory()) {
      if (entry === ".dx") {
        continue;
      }

      output.push(...listFiles(path));
    } else {
      output.push(path);
    }
  }

  return output;
}

describe("examples/world provider lab", () => {
  it("covers the first three targets from WORLD.md", () => {
    const source = read("WORLD.md");
    const laneSource = listFiles(join(worldRoot, "lib", "world", "lanes"))
      .map((file) => readFileSync(file, "utf8"))
      .join("\n");

    const expected = [
      "PostgreSQL",
      "Neon",
      "Turso/libSQL",
      "DX ORM / Forge database",
      "Drizzle",
      "Prisma",
      "DX Auth / Forge auth",
      "Better Auth",
      "Clerk",
      "PostgreSQL RLS",
      "OpenFGA",
      "Casbin",
      "Stripe",
      "Lemon Squeezy",
      "Paddle",
      "AWS S3",
      "Cloudflare R2",
      "Vercel Blob",
      "Meilisearch",
      "Typesense",
      "Algolia",
      "pgvector",
      "Pinecone",
      "MongoDB Atlas Vector Search",
      "OpenAI",
      "Anthropic",
      "Google Gemini",
      "WebSocket/SSE",
      "Supabase Realtime",
      "Ably",
      "Cloudflare Queues",
      "Upstash QStash",
      "Temporal",
      "Redis/Valkey",
      "Upstash Redis",
      "Cloudflare KV",
      "PostHog",
      "Plausible",
      "Vercel Analytics",
      "OpenTelemetry",
      "Sentry",
      "Datadog",
      "Content collections/MDX",
      "Sanity",
      "Strapi",
      "Resend",
      "Twilio",
      "Firebase Cloud Messaging",
      "Vercel",
      "Cloudflare",
      "Fly.io",
      "DX Env Firewall",
      "GitHub Advanced Security",
      "Sigstore",
      "GitHub",
      "Linear",
      "Notion",
      "FormatJS/Intl",
      "Google Maps",
      "React Hook Form shape",
    ];

    for (const provider of expected) {
      assert.match(source, new RegExp(provider.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
      assert.match(laneSource, new RegExp(provider.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
    }
  });

  it("keeps the example TypeScript-only", () => {
    const forbidden = listFiles(worldRoot)
      .map((file) => relative(worldRoot, file))
      .filter((file) => /\.(cjs|mjs|js)$/.test(file));

    assert.deepEqual(forbidden, []);
  });

  it("keeps framework upgrades as PLAN.md suggestions", () => {
    const plan = read("PLAN.md");
    assert.match(plan, /World Integration Adapter Suggestions/);
    assert.match(plan, /provider adapter contract/);
    assert.match(plan, /Env Firewall/);
  });

  it("ships a redacted status API route", () => {
    const route = read("examples/world/app/api/world/status/route.ts");
    const providerRoute = read("examples/world/app/api/world/provider/route.ts");
    const liveRoute = read("examples/world/app/api/world/live/route.ts");
    const status = read("examples/world/lib/world/status.ts");

    assert.match(route, /Response\.json\(\{/);
    assert.match(providerRoute, /Response\.json\(\{/);
    assert.match(liveRoute, /connectionRunner/);
    assert.match(route, /read-only-runner-when-env-present/);
    assert.match(providerRoute, /credentialState/);
    assert.match(status, /secret-values-never-included/);
    assert.doesNotMatch(status, /process\.env\.[A-Z0-9_]+/);
  });

  it("includes a real TypeScript live connection runner", () => {
    const runner = read("examples/world/lib/world/connections/runner.ts");
    const providers = read("examples/world/lib/world/connections/providers/index.ts");
    const redaction = read("examples/world/lib/world/connections/redaction.ts");

    assert.match(runner, /runWorldConnectionProbes/);
    assert.match(runner, /live-connections\.json/);
    assert.match(providers, /vercelConnectionProbes/);
    assert.match(providers, /tursoConnectionProbes/);
    assert.match(redaction, /hasLeakedEnvValue/);
  });

  it("defines package ids and route contracts for provider cards", () => {
    const contracts = read("examples/world/lib/world/contracts.ts");
    const routes = read("examples/world/lib/world/routes.ts");
    const lanes = listFiles(join(worldRoot, "lib", "world", "lanes"))
      .map((file) => readFileSync(file, "utf8"))
      .join("\n");

    assert.match(contracts, /packageId/);
    assert.match(contracts, /routeHandlers/);
    assert.match(routes, /WorldRouteContract/);
    assert.match(lanes, /NEXT_PUBLIC_CLERK_PUBLISHABLE_KEY/);
  });

  it("keeps operations provider probes credential-gated and read-only", () => {
    const providersRoot = join(worldRoot, "lib", "world", "connections", "providers");
    const providerFiles = listFiles(providersRoot)
      .map((file) => relative(worldRoot, file).replace(/\\/g, "/"))
      .filter((file) => file.includes("operations") || file.endsWith("probe-contract.ts"));

    assert.ok(providerFiles.includes("lib/world/connections/providers/operations.ts"));

    const providerSource = read("examples/world/lib/world/connections/providers/operations.ts");
    const providerRoute = read("examples/world/app/api/world/provider/route.ts");

    for (const providerId of [
      "redis-valkey",
      "upstash-redis",
      "cloudflare-kv",
      "posthog",
      "plausible",
      "vercel-analytics",
      "sentry",
      "datadog",
      "opentelemetry",
    ]) {
      assert.match(providerSource, new RegExp(`providerId: "${providerId}"`));
    }

    for (const envName of [
      "REDIS_URL",
      "REDIS_READINESS_URL",
      "UPSTASH_REDIS_REST_URL",
      "UPSTASH_REDIS_REST_TOKEN",
      "CLOUDFLARE_ACCOUNT_ID",
      "CLOUDFLARE_API_TOKEN",
      "CLOUDFLARE_KV_NAMESPACE_ID",
      "NEXT_PUBLIC_POSTHOG_KEY",
      "POSTHOG_HOST",
      "POSTHOG_PERSONAL_API_KEY",
      "POSTHOG_PROJECT_ID",
      "NEXT_PUBLIC_PLAUSIBLE_DOMAIN",
      "PLAUSIBLE_API_KEY",
      "PLAUSIBLE_API_HOST",
      "VERCEL_PROJECT_ID",
      "VERCEL_TOKEN",
      "SENTRY_DSN",
      "SENTRY_AUTH_TOKEN",
      "SENTRY_ORG",
      "SENTRY_PROJECT",
      "DD_API_KEY",
      "DD_SITE",
      "DATADOG_SITE",
      "OTEL_EXPORTER_OTLP_ENDPOINT",
      "OTEL_HEALTHCHECK_URL",
    ]) {
      assert.match(providerSource, new RegExp(envName));
    }

    assert.match(providerSource, /"missing-config"/);
    assert.match(providerSource, /method: "GET"/);
    assert.match(providerSource, /operation: "read-only"/);
    assert.doesNotMatch(providerSource, /\bPOST\b|\bPUT\b|\bPATCH\b|\bDELETE\b|set-get|read-write/);
    assert.match(providerRoute, /runOperationsProviderProbe/);
    assert.match(providerRoute, /credentialState: probe.state/);
    assert.match(providerRoute, /liveProviderExecution: probe.state === "live-validated"/);
  });

  it("keeps live provider probes receipt-gated until route handler runtime gaps are closed", () => {
    const liveProbes = read("examples/world/lib/world/live-probes.ts");
    const plan = read("PLAN.md");

    for (const gap of [
      "imported-helper-execution",
      "server-only-env-injection",
      "route-handler-fetch",
      "provider-receipt-import",
    ]) {
      assert.match(liveProbes, new RegExp(gap));
      assert.match(plan, new RegExp(gap));
    }

    assert.match(liveProbes, /providerReceiptImported: boolean/);
    assert.match(liveProbes, /liveProviderExecution: false/);
    assert.match(liveProbes, /status: "configured-readiness"/);
    assert.match(liveProbes, /status: "live-validated"/);
    assert.match(liveProbes, /generatedDxReceiptPolicy: "never-commit-generated-receipts"/);
  });
});
