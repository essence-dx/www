import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

import {
  createDxDashboardIntlReceipt,
  getDashboardLocaleAlternateLinks,
  getDashboardLocaleRoutePreview,
  normalizeLocale,
} from "../examples/template/next-intl-dashboard-locale-contract.ts";
import {
  createDataFetchingCacheActionResponse,
  createDataFetchingCacheReadinessResponse,
} from "../examples/template/server/query-cache/readiness.ts";
import { dxBetterAuthForgePackage } from "../examples/template/auth/better-auth/metadata.ts";
import { createDatabaseApiReadinessResponse } from "../examples/template/server/database-api/readiness.ts";
import { createDatabaseOrmReadinessResponse } from "../examples/template/server/database-orm/readiness.ts";
import { createInstantReadinessResponse } from "../examples/template/server/instant/readiness.ts";
import { createSupabaseReadinessResponse } from "../examples/template/server/supabase/readiness.ts";
import { createDxAiMissingProviderResponse } from "../examples/template/lib/ai/provider-boundary.ts";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

function read(relativePath: string) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

async function readJson(response: Response) {
  return response.json() as Promise<Record<string, unknown>>;
}

function assertNoLiveProviderProof(payload: Record<string, unknown>) {
  if ("runtimeProof" in payload) {
    assert.equal(payload.runtimeProof, false);
  }
  if ("runtimeExecution" in payload) {
    assert.equal(payload.runtimeExecution, false);
  }
  if ("networkCalls" in payload) {
    assert.equal(payload.networkCalls, false);
  }
  if ("hostedCredentials" in payload) {
    assert.equal(payload.hostedCredentials, false);
  }
  if ("secretValues" in payload) {
    assert.deepEqual(payload.secretValues, []);
  }
}

test("source-owned template readiness helpers execute without live-provider claims", async () => {
  const queryReadiness = createDataFetchingCacheReadinessResponse(
    new Request("https://dx.local/api/query-cache/readiness?filter=ready&optimistic=queued&visible=2"),
  );
  const queryPayload = await readJson(queryReadiness);
  assert.equal(queryReadiness.status, 200);
  assert.equal(queryPayload.schema, "dx.www.template.data_fetching_cache_readiness");
  assert.equal(queryPayload.adapterBoundary, "queryclient-adapter-required");
  assertNoLiveProviderProof(queryPayload);

  const queryAction = await createDataFetchingCacheActionResponse(
    new Request("https://dx.local/api/query-cache/readiness", {
      method: "POST",
      body: JSON.stringify({
        action: "refresh",
        filter: "ready",
        optimisticState: "queued",
        visibleProjectCount: 2,
      }),
    }),
  );
  const queryActionPayload = await readJson(queryAction);
  assert.equal(queryAction.status, 200);
  assert.equal(queryActionPayload.schema, "dx.www.template.data_fetching_cache_action_receipt");
  assert.equal(queryActionPayload.queryClientExecution, false);
  assertNoLiveProviderProof(queryActionPayload);

  const databaseApi = createDatabaseApiReadinessResponse();
  const databaseApiPayload = await readJson(databaseApi);
  assert.equal(databaseApi.status, 200);
  assert.equal(databaseApiPayload.schema, "dx.www.template.database_api_readiness");
  assertNoLiveProviderProof(databaseApiPayload);

  const databaseOrm = createDatabaseOrmReadinessResponse({});
  const databaseOrmPayload = await readJson(databaseOrm);
  assert.equal(databaseOrm.status, 501);
  assert.equal(databaseOrmPayload.schema, "dx.www.template.database_orm_readiness");
  assert.equal(databaseOrmPayload.status, "runtime-gated");
  assertNoLiveProviderProof(databaseOrmPayload);

  const instant = createInstantReadinessResponse({});
  const instantPayload = await readJson(instant);
  assert.equal(instant.status, 501);
  assert.equal(instantPayload.schema, "dx.www.template.instant_readiness");
  assert.equal(instantPayload.status, "provider-gated");
  assertNoLiveProviderProof(instantPayload);

  const supabase = createSupabaseReadinessResponse({});
  const supabasePayload = await readJson(supabase);
  assert.equal(supabase.status, 501);
  assert.equal(supabasePayload.schema, "dx.www.template.supabase_readiness");
  assert.equal(supabasePayload.status, "provider-gated");
  assertNoLiveProviderProof(supabasePayload);

  const ai = createDxAiMissingProviderResponse({
    provider: "openai-compatible",
    capability: "chat-stream",
    requiredEnv: "AI_PROVIDER_API_KEY",
    appOwnedBoundary: "Provider credentials, model selection, billing, and safety policy stay app-owned.",
  });
  const aiPayload = await readJson(ai);
  assert.equal(ai.status, 501);
  assert.equal(aiPayload.status, "missing-config");
  assert.equal(aiPayload.modelStreaming, false);
  assert.equal(aiPayload.providerRuntime, false);
  assertNoLiveProviderProof(aiPayload);

  const authMissingEnv = dxBetterAuthForgePackage.requiredEnv.filter(
    (name) => !({} as Record<string, string | undefined>)[name],
  );
  assert.deepEqual(authMissingEnv, [
    "BETTER_AUTH_SECRET",
    "BETTER_AUTH_URL",
    "GOOGLE_CLIENT_ID",
    "GOOGLE_CLIENT_SECRET",
  ]);
  assertNoLiveProviderProof({
    runtimeExecution: false,
    liveSessionExecution: false,
    secretValues: [],
  });

  const intlLocale = normalizeLocale("bn");
  const intlRoute = getDashboardLocaleRoutePreview(intlLocale);
  const intlReceipt = createDxDashboardIntlReceipt({
    locale: intlLocale,
    planPricePreview: "$49",
    previewWindow: "May 23, 2026, 10:00 AM",
    providerLocale: "en",
  });
  assert.equal(intlLocale, "bn");
  assert.equal(intlRoute.href, "/bn");
  assert.equal(intlRoute.hrefLang, "bn-BD");
  assert.equal(intlReceipt.packageId, "i18n/next-intl");
  assert.equal(intlReceipt.component, "next-intl-dashboard-locale-workflow");
  assert.equal(intlReceipt.locale, "bn");
  assert.equal(intlReceipt.routePreviewDetails.seoBoundary, "app-owned-alternate-link-review");
  assert.deepEqual(
    getDashboardLocaleAlternateLinks().map((link) => ({
      href: link.href,
      hrefLang: link.hrefLang,
      locale: link.locale,
      rel: link.rel,
    })),
    [
      { href: "/", hrefLang: "en", locale: "en", rel: "alternate" },
      { href: "/bn", hrefLang: "bn-BD", locale: "bn", rel: "alternate" },
    ],
  );
  assert.ok(intlReceipt.appOwnedBoundaries.includes("Message quality"));
  assertNoLiveProviderProof({
    runtimeExecution: false,
    liveProviderExecution: false,
    networkCalls: false,
    secretValues: [],
  });

  const paymentsRoute = read(
    "examples/template/app/api/payments/stripe-js/readiness/route.ts",
  );
  assert.match(paymentsRoute, /runtimeExecution: false/);
  assert.match(paymentsRoute, /stripeLiveExecution: false/);
  assert.match(paymentsRoute, /secretValues: \[\]/);

  const n8nRoute = read(
    "examples/template/app/api/automations/n8n/dry-run/route.ts",
  );
  assert.match(n8nRoute, /runtimeExecution: false/);
  assert.match(n8nRoute, /secretValues: \[\]/);
  assert.match(n8nRoute, /does not run n8n/);
});

test("Forge reality model exposes route-helper execution proof without lifting live-proof cap", async () => {
  const reality = await import("../examples/template/components/template-app/package-reality.ts");
  const proofRows = reality.templateReadinessExecutionProofRows;

  assert.ok(Array.isArray(proofRows), "package reality should export execution proof rows");
  assert.equal(reality.forgeRealitySummary.score, 89);
  assert.equal(reality.forgeRealitySummary.scoreCeilingWithoutLiveProof, 89);
  assert.equal(reality.forgeRealitySummary.readinessExecutionProofCount, proofRows.length);
  assert.equal(reality.forgeRealitySummary.readinessExecutionProofPackageCount, 10);
  assert.deepEqual(reality.forgeRealitySummary.readinessExecutionProofPackageIds, [
    "ai/vercel-ai",
    "api/trpc",
    "auth/better-auth",
    "automations/n8n",
    "db/drizzle-sqlite",
    "i18n/next-intl",
    "instantdb/react",
    "payments/stripe-js",
    "supabase/client",
    "tanstack/query",
  ]);

  for (const row of proofRows) {
    assert.equal(row.runtimeProof, false);
    assert.equal(row.liveProviderExecution, false);
    assert.deepEqual(row.secretValues, []);
    assert.match(row.exercisedBy, /template-readiness-execution-proof\.test\.ts/);
  }
});

test("dashboard keeps route-helper proof visible but secondary", () => {
  const panel = read("examples/template/components/template-app/package-reality-panel.tsx");
  const generatedDashboard = read("tools/launch/runtime-template/pages/index.html");

  assert.match(panel, /templateReadinessExecutionProofRows/);
  assert.match(panel, /data-dx-forge-readiness-execution-proof-count/);
  assert.match(panel, /data-dx-forge-readiness-execution-proof-package-count/);
  assert.match(panel, /data-dx-component="template-readiness-execution-proof"/);
  assert.match(panel, /Route helper proof/);
  assert.doesNotMatch(panel, /live provider proof achieved|browser proof achieved/i);

  assert.match(generatedDashboard, /data-dx-component="template-readiness-execution-proof"/);
  assert.match(generatedDashboard, /data-dx-readiness-execution-route="\/"/);
  assert.match(generatedDashboard, /data-dx-readiness-execution-packages="i18n\/next-intl"/);
  assert.match(generatedDashboard, /data-dx-readiness-execution-route="\/api\/query-cache\/readiness"/);
  assert.match(generatedDashboard, /data-dx-live-provider-execution="false"/);
  assert.doesNotMatch(generatedDashboard, /live provider proof achieved|browser proof achieved/i);
});
