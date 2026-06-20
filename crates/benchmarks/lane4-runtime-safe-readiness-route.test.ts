import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

import { databaseApiLaneRuntimeReadiness } from "../examples/template/components/template-app/package-lock-reality.ts";
import { databaseApiSourceContract } from "../examples/template/lib/database-api/source-contract.ts";
import {
  readDatabaseApiRouteReadiness,
} from "../examples/template/server/database-api/readiness.ts";
import { createDatabaseOrmReadinessResponse } from "../examples/template/server/database-orm/readiness.ts";
import { createInstantReadinessResponse } from "../examples/template/server/instant/readiness.ts";
import { createSupabaseReadinessResponse } from "../examples/template/server/supabase/readiness.ts";
import { GET } from "../examples/template/app/api/database-api/readiness/route.ts";
import {
  GET as trpcHealthGET,
  POST as trpcHealthPOST,
} from "../examples/template/app/api/trpc/health/route.ts";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

function read(relativePath: string) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("Database + API lane exposes a runtime-safe App Router readiness route", async () => {
  const readiness = readDatabaseApiRouteReadiness();
  const response = await GET(new Request("http://dx.local/api/database-api/readiness"));
  const payload = await response.json();

  assert.equal(response.status, 200);
  assert.equal(response.headers.get("cache-control"), "no-store");
  assert.equal(payload.schema, "dx.www.template.database_api_readiness");
  assert.equal(payload.laneNumber, 4);
  assert.equal(payload.laneName, "Database + API");
  assert.equal(payload.runtimeProof, false);
  assert.equal(payload.networkCalls, false);
  assert.equal(payload.hostedCredentials, false);
  assert.equal(
    payload.templateReadinessReceipt,
    ".dx/forge/template-readiness/database-api.json",
  );
  assert.equal(payload.cacheEvidence.sourceOfTruth, ".dx/forge/package-status.json");
  assert.equal(payload.cacheEvidence.currentManifestSet, "cache.manifests");
  assert.equal(
    payload.cacheEvidence.physicalManifestCaveatId,
    "physical-cache-matches-current-manifests",
  );
  assert.equal(
    payload.cacheEvidence.currentManifestSource,
    "package-status-current-manifests",
  );
  assert.deepEqual(payload, readiness);
  assert.equal(databaseApiLaneRuntimeReadiness.route, "/api/database-api/readiness");
  assert.equal(
    databaseApiLaneRuntimeReadiness.routeHandler,
    "app/api/database-api/readiness/route.ts",
  );
  assert.equal(
    databaseApiLaneRuntimeReadiness.serverFile,
    "server/database-api/readiness.ts",
  );

  const packageIds = payload.packages.map(
    (entry: { packageId: string }) => entry.packageId,
  );
  assert.deepEqual(packageIds, [
    "db/drizzle-sqlite",
    "instantdb/react",
    "supabase/client",
    "api/trpc",
  ]);
  for (const packageEntry of payload.packages) {
    assert.equal(packageEntry.runtimeProof, false);
    assert.equal(packageEntry.networkCalls, false);
    assert.equal(packageEntry.status, "source-owned-adapter-boundary");
    assert.ok(packageEntry.appOwnedBoundary.length > 0);
    assert.ok(packageEntry.frontFacingFiles.length > 0);
  }

  const dashboardRouteStatusByPackage = new Map(
    databaseApiLaneRuntimeReadiness.packages.map((packageEntry) => [
      packageEntry.packageId,
      packageEntry,
    ]),
  );
  assert.deepEqual(
    [
      dashboardRouteStatusByPackage.get("db/drizzle-sqlite")?.httpStatus,
      dashboardRouteStatusByPackage.get("instantdb/react")?.httpStatus,
      dashboardRouteStatusByPackage.get("supabase/client")?.httpStatus,
      dashboardRouteStatusByPackage.get("api/trpc")?.httpStatus,
    ],
    [501, 501, 501, 200],
  );
  assert.equal(
    dashboardRouteStatusByPackage.get("db/drizzle-sqlite")?.readinessKind,
    "runtime-gated",
  );
  assert.equal(
    dashboardRouteStatusByPackage.get("instantdb/react")?.readinessKind,
    "provider-gated",
  );
  assert.equal(
    dashboardRouteStatusByPackage.get("supabase/client")?.readinessKind,
    "provider-gated",
  );
  assert.equal(
    dashboardRouteStatusByPackage.get("api/trpc")?.readinessKind,
    "local-readiness",
  );

  assert.ok(
    payload.appRouterRoutes.includes("app/api/instant/route.ts"),
    "InstantDB route handler should be listed as a Lane 4 route surface",
  );
  assert.ok(
    payload.appRouterRoutes.includes("app/api/instant/readiness/route.ts"),
    "InstantDB readiness route should be listed as a Lane 4 route surface",
  );
  assert.ok(
    payload.appRouterRoutes.includes("app/api/trpc/[trpc]/route.ts"),
    "tRPC route handler should be listed as a Lane 4 route surface",
  );
  assert.ok(
    payload.appRouterRoutes.includes("app/api/trpc/health/route.ts"),
    "tRPC health route should be listed as a Lane 4 route surface",
  );
  assert.ok(
    payload.appRouterRoutes.includes("app/api/database-api/readiness/route.ts"),
    "Lane 4 readiness route should list itself",
  );
  assert.ok(
    payload.appRouterRoutes.includes("app/api/database-orm/readiness/route.ts"),
    "Database ORM readiness route should be listed as a Lane 4 route surface",
  );
  assert.ok(
    payload.appRouterRoutes.includes("app/api/supabase/readiness/route.ts"),
    "Supabase readiness route should be listed as a Lane 4 route surface",
  );
});

test("Database + API readiness route exposes a source-owned schema and API contract", async () => {
  const response = await GET(new Request("http://dx.local/api/database-api/readiness"));
  const payload = await response.json();

  assert.deepEqual(payload.sourceContract, databaseApiSourceContract);
  assert.equal(payload.sourceContract.schema, "dx.www.template.database_api_source_contract");
  assert.equal(payload.sourceContract.runtimeProof, false);
  assert.equal(payload.sourceContract.networkCalls, false);
  assert.equal(payload.sourceContract.hostedCredentials, false);

  assert.deepEqual(
    payload.sourceContract.schemaSurfaces.map((surface: { packageId: string }) => surface.packageId),
    ["db/drizzle-sqlite", "instantdb/react", "supabase/client"],
  );
  assert.deepEqual(payload.sourceContract.schemaSurfaces[0].tables, ["users", "posts"]);
  assert.equal(payload.sourceContract.schemaSurfaces[0].sourceFile, "db/drizzle/schema.ts");
  assert.deepEqual(payload.sourceContract.schemaSurfaces[1].entities, ["todos", "labels"]);
  assert.deepEqual(payload.sourceContract.schemaSurfaces[1].rooms, ["launch"]);
  assert.deepEqual(payload.sourceContract.schemaSurfaces[2].tables, ["profiles"]);
  assert.deepEqual(payload.sourceContract.schemaSurfaces[2].env, [
    "NEXT_PUBLIC_SUPABASE_URL",
    "NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY",
  ]);

  assert.deepEqual(
    payload.sourceContract.routeSurfaces.map((surface: { route: string }) => surface.route),
    [
      "/api/instant",
      "/api/instant/readiness",
      "/api/trpc/health",
      "/api/trpc/[trpc]",
      "/api/database-api/readiness",
      "/api/database-orm/readiness",
      "/api/supabase/readiness",
    ],
  );
  const routeMethodsByRoute = new Map(
    payload.sourceContract.routeSurfaces.map(
      (surface: { route: string; methods: readonly string[] }) => [
        surface.route,
        surface.methods,
      ],
    ),
  );
  assert.deepEqual(routeMethodsByRoute.get("/api/instant/readiness"), ["GET"]);
  assert.deepEqual(routeMethodsByRoute.get("/api/trpc/health"), ["GET", "POST"]);
  assert.deepEqual(routeMethodsByRoute.get("/api/trpc/[trpc]"), ["GET", "POST"]);
  assert.deepEqual(routeMethodsByRoute.get("/api/database-orm/readiness"), ["GET"]);

  const trpcProcedureKinds = new Map(
    payload.sourceContract.trpcProcedures.map(
      (procedure: { path: string; kind: string; runtimeProof: boolean }) => [
        procedure.path,
        {
          kind: procedure.kind,
          runtimeProof: procedure.runtimeProof,
        },
      ],
    ),
  );
  assert.deepEqual(trpcProcedureKinds.get("health"), {
    kind: "query",
    runtimeProof: false,
  });
  assert.deepEqual(trpcProcedureKinds.get("launchEvent"), {
    kind: "mutation",
    runtimeProof: false,
  });
  assert.deepEqual(trpcProcedureKinds.get("launchFeed"), {
    kind: "subscription",
    runtimeProof: false,
  });
});

test("default dashboard links the Lane 4 readiness route without live-provider overclaim", () => {
  const panel = read(
    "examples/template/components/template-app/package-reality-panel.tsx",
  );
  const reality = read(
    "examples/template/components/template-app/package-lock-reality.ts",
  );

  assert.match(panel, /data-dx-component="lane4-database-api-readiness"/);
  assert.match(panel, /data-dx-api-readiness-route=\{databaseApiLaneRuntimeReadiness\.route\}/);
  assert.match(panel, /data-dx-api-template-readiness-receipt=/);
  assert.match(panel, /data-dx-api-cache-manifest-source=/);
  assert.match(panel, /data-dx-api-physical-cache-manifest-caveat=/);
  assert.match(panel, /data-dx-runtime-proof="false"/);
  assert.match(panel, /data-dx-network-calls="false"/);
  assert.match(panel, /data-dx-api-source-contract=/);
  assert.match(panel, /data-dx-api-schema-surface-count=/);
  assert.match(panel, /data-dx-api-http-status=\{packageEntry\.httpStatus\}/);
  assert.match(panel, /data-dx-api-readiness-kind=\{packageEntry\.readinessKind\}/);
  assert.match(panel, /packageEntry\.statusLabel/);
  assert.match(panel, /packageEntry\.userMessage/);
  assert.match(panel, /databaseApiLaneRuntimeReadiness\.sourceContract\.schemaSurfaces\.map/);
  assert.match(panel, /databaseApiLaneRuntimeReadiness\.packages\.map/);
  assert.match(reality, /databaseApiLaneRuntimeReadiness/);
  assert.match(reality, /\/api\/database-orm\/readiness/);
  assert.match(reality, /\/api\/supabase\/readiness/);
  assert.doesNotMatch(panel, /Connected to Supabase|InstantDB connected|SQLite live|tRPC live/);
});

test("Type-Safe API health route returns App Router Responses without live transport claims", async () => {
  const trpcHealthRoute = read("examples/template/app/api/trpc/health/route.ts");
  const getResponse = await trpcHealthGET();
  const getPayload = await getResponse.json();
  const postResponse = await trpcHealthPOST(
    new Request("http://dx.local/api/trpc/health", {
      method: "POST",
      body: JSON.stringify({ event: "validated", route: "/" }),
    }),
  );
  const postPayload = await postResponse.json();

  assert.match(trpcHealthRoute, /Response\.json/);
  assert.match(trpcHealthRoute, /schema: "dx\.www\.template\.trpc_health"/);
  assert.match(trpcHealthRoute, /runtimeProof: false/);
  assert.match(trpcHealthRoute, /networkCalls: false/);
  assert.match(trpcHealthRoute, /hostedCredentials: false/);
  assert.doesNotMatch(
    trpcHealthRoute,
    /fetchRequestHandler|createTRPCClient|httpBatchLink|createTRPCReact|new WebSocket/,
    "health route must stay a local readiness/dry-run surface",
  );

  assert.equal(getResponse.status, 200);
  assert.equal(getResponse.headers.get("cache-control"), "no-store");
  assert.equal(getPayload.schema, "dx.www.template.trpc_health");
  assert.equal(getPayload.route, "/api/trpc/health");
  assert.equal(getPayload.packageId, "api/trpc");
  assert.equal(getPayload.status, "ready");
  assert.equal(getPayload.procedure, "health");
  assert.equal(getPayload.runtimeProof, false);
  assert.equal(getPayload.networkCalls, false);
  assert.equal(getPayload.hostedCredentials, false);

  assert.equal(postResponse.status, 202);
  assert.equal(postResponse.headers.get("cache-control"), "no-store");
  assert.equal(postPayload.schema, "dx.www.template.trpc_health");
  assert.equal(postPayload.status, "accepted");
  assert.equal(postPayload.procedure, "launchEvent");
  assert.equal(postPayload.runtimeProof, false);
  assert.equal(postPayload.networkCalls, false);
  assert.equal(postPayload.hostedCredentials, false);
});

test("Type-Safe API health route is lock-backed and discoverable as package source", () => {
  const healthRoutePath = "app/api/trpc/health/route.ts";
  const sourceManifest = JSON.parse(
    read("examples/template/.dx/forge/source-.dx/build-cache/manifest.json"),
  );
  const cacheManifest = JSON.parse(
    read(
      "examples/template/.dx/forge/cache/api-trpc/11.17.0-dx.10/.dx/build-cache/manifest.json",
    ),
  );
  const packageReceipt = JSON.parse(
    read("examples/template/.dx/forge/receipts/packages/api-trpc.json"),
  );
  const metadata = read("examples/template/lib/trpc/metadata.ts");
  const healthRouteSource = read(`examples/template/${healthRoutePath}`);
  const cachedHealthRouteSource = read(
    `examples/template/.dx/forge/cache/api-trpc/11.17.0-dx.10/${healthRoutePath}`,
  );

  const sourcePackage = sourceManifest.packages.find(
    (entry: { package_id: string }) => entry.package_id === "api/trpc",
  );
  const receiptFiles = packageReceipt.package?.files ?? packageReceipt.files ?? [];
  const receiptCacheFiles = packageReceipt.cache?.cached_files ?? [];

  assert.ok(sourcePackage, "api/trpc must exist in source-manifest");
  assert.ok(
    sourcePackage.files.some(
      (file: { path: string }) => file.path === healthRoutePath,
    ),
    "api/trpc source-manifest must lock the health route",
  );
  assert.ok(
    cacheManifest.cached_files.some(
      (file: { path: string }) => file.path === healthRoutePath,
    ),
    "api/trpc active cache manifest must include the health route",
  );
  assert.ok(
    receiptFiles.some(
      (file: { path: string }) => file.path === healthRoutePath,
    ),
    "api/trpc package receipt must include the health route",
  );
  assert.ok(
    receiptCacheFiles.some(
      (file: { path: string }) => file.path === healthRoutePath,
    ),
    "api/trpc package receipt cache must include the health route",
  );
  assert.equal(cachedHealthRouteSource, healthRouteSource);
  assert.match(metadata, /"app\/api\/trpc\/health\/route\.ts"/);
  assert.match(metadata, /healthRoute: "app\/api\/trpc\/health\/route\.ts"/);
});

test("InstantDB route returns a provider-gated response instead of throwing on missing config", () => {
  const instantRoute = read("examples/template/app/api/instant/route.ts");
  const instantHandler = read("examples/template/lib/instant/route.ts");

  assert.match(
    instantRoute,
    /export const \{ GET, POST \} = createDxInstantRouteHandlers\(\);/,
  );
  assert.match(instantHandler, /function createInstantMissingConfigResponse/);
  assert.match(instantHandler, /status: 501/);
  assert.match(instantHandler, /"cache-control": "no-store"/);
  assert.match(instantHandler, /runtimeProof: false/);
  assert.match(instantHandler, /networkCalls: false/);
  assert.match(instantHandler, /NEXT_PUBLIC_INSTANT_APP_ID/);
  assert.match(instantHandler, /catch \(error\)/);
  assert.doesNotMatch(
    instantHandler,
    /throw new Error\(`Missing required InstantDB env var/,
    "route handler should convert missing provider config into an honest 501 response",
  );
});

test("InstantDB readiness route returns provider-gated status without hosted runtime proof", async () => {
  const instantReadinessRoute = read("examples/template/app/api/instant/readiness/route.ts");
  const instantReadinessHelper = read("examples/template/server/instant/readiness.ts");
  const response = await createInstantReadinessResponse({});
  const payload = await response.json();

  assert.match(instantReadinessRoute, /createInstantReadinessResponse/);
  assert.match(instantReadinessRoute, /dynamic = "force-dynamic"/);
  assert.match(instantReadinessHelper, /schema: "dx\.www\.template\.instant_readiness"/);
  assert.match(instantReadinessHelper, /packageId: "instantdb\/react"/);
  assert.match(instantReadinessHelper, /status: readiness\.httpStatus/);
  assert.match(instantReadinessHelper, /"cache-control": "no-store"/);
  assert.match(instantReadinessHelper, /runtimeProof: false/);
  assert.match(instantReadinessHelper, /networkCalls: false/);
  assert.match(instantReadinessHelper, /hostedCredentials: false/);
  assert.match(instantReadinessHelper, /NEXT_PUBLIC_INSTANT_APP_ID/);
  assert.match(
    instantReadinessHelper,
    /This route performs local configuration validation only/,
  );
  assert.doesNotMatch(
    instantReadinessHelper,
    /createInstantRouteHandler|db\.\w+|transact|query|subscribe/,
    "readiness route must not create the hosted Instant route handler or perform hosted operations",
  );
  assert.equal(response.status, 501);
  assert.equal(response.headers.get("cache-control"), "no-store");
  assert.equal(payload.schema, "dx.www.template.instant_readiness");
  assert.equal(payload.route, "/api/instant/readiness");
  assert.equal(payload.status, "provider-gated");
  assert.equal(payload.runtimeProof, false);
  assert.equal(payload.networkCalls, false);
  assert.equal(payload.hostedCredentials, false);
  assert.deepEqual(payload.missingEnv, ["NEXT_PUBLIC_INSTANT_APP_ID"]);
});

test("InstantDB package handoff documents the provider-gated readiness route", () => {
  const packageDoc = read("docs/packages/instantdb-react.md");
  const runbook = JSON.parse(
    read("docs/packages/instantdb-react.source-guard-runbook.json"),
  );

  assert.match(packageDoc, /\/api\/instant\/readiness/);
  assert.match(packageDoc, /server\/instant\/readiness\.ts/);
  assert.match(packageDoc, /provider-gated readiness route/);
  assert.match(packageDoc, /does not create a hosted route handler/);
  assert.match(packageDoc, /hosted auth, realtime transport, storage, or streams/);

  assert.ok(
    runbook.selected_surfaces.includes("provider-gated-readiness-route"),
    "runbook should expose the InstantDB readiness route as a selected surface",
  );
  assert.equal(runbook.template_readiness.route, "/api/instant/readiness");
  assert.equal(
    runbook.template_readiness.server_file,
    "examples/template/server/instant/readiness.ts",
  );
  assert.equal(runbook.template_readiness.runtime_proof, false);
  assert.equal(runbook.template_readiness.network_calls, false);
  assert.equal(runbook.template_readiness.hosted_credentials, false);
  assert.match(
    runbook.template_readiness.boundary,
    /does not create a hosted route handler/,
  );
});

test("Supabase readiness route returns a provider-gated response instead of implying a live backend", async () => {
  const supabaseRoute = read("examples/template/app/api/supabase/readiness/route.ts");
  const supabaseHelper = read("examples/template/server/supabase/readiness.ts");
  const response = await createSupabaseReadinessResponse({});
  const payload = await response.json();

  assert.match(supabaseRoute, /createSupabaseReadinessResponse/);
  assert.match(supabaseRoute, /dynamic = "force-dynamic"/);
  assert.match(supabaseHelper, /schema: "dx\.www\.template\.supabase_readiness"/);
  assert.match(supabaseHelper, /packageId: "supabase\/client"/);
  assert.match(supabaseHelper, /status: readiness\.httpStatus/);
  assert.match(supabaseHelper, /"cache-control": "no-store"/);
  assert.match(supabaseHelper, /runtimeProof: false/);
  assert.match(supabaseHelper, /networkCalls: false/);
  assert.match(supabaseHelper, /hostedCredentials: false/);
  assert.match(supabaseHelper, /NEXT_PUBLIC_SUPABASE_URL/);
  assert.match(supabaseHelper, /NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY/);
  assert.match(
    supabaseHelper,
    /This route performs local configuration validation only/,
  );
  assert.doesNotMatch(
    supabaseHelper,
    /createClient|from\("profiles"\)|supabase\.auth|getUser/,
    "readiness route must not perform hosted Supabase operations",
  );
  assert.equal(response.status, 501);
  assert.equal(response.headers.get("cache-control"), "no-store");
  assert.equal(payload.schema, "dx.www.template.supabase_readiness");
  assert.equal(payload.route, "/api/supabase/readiness");
  assert.equal(payload.status, "provider-gated");
  assert.equal(payload.runtimeProof, false);
  assert.equal(payload.networkCalls, false);
  assert.equal(payload.hostedCredentials, false);
  assert.deepEqual(payload.missingEnv, [
    "NEXT_PUBLIC_SUPABASE_URL",
    "NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY",
  ]);
});

test("Database ORM readiness route returns runtime-gated status without opening SQLite", async () => {
  const databaseOrmRoute = read("examples/template/app/api/database-orm/readiness/route.ts");
  const databaseOrmHelper = read("examples/template/server/database-orm/readiness.ts");
  const response = await createDatabaseOrmReadinessResponse({});
  const payload = await response.json();

  assert.match(databaseOrmRoute, /createDatabaseOrmReadinessResponse/);
  assert.match(databaseOrmRoute, /dynamic = "force-dynamic"/);
  assert.match(databaseOrmHelper, /schema: "dx\.www\.template\.database_orm_readiness"/);
  assert.match(databaseOrmHelper, /packageId: "db\/drizzle-sqlite"/);
  assert.match(databaseOrmHelper, /status: readiness\.httpStatus/);
  assert.match(databaseOrmHelper, /"cache-control": "no-store"/);
  assert.match(databaseOrmHelper, /runtimeProof: false/);
  assert.match(databaseOrmHelper, /networkCalls: false/);
  assert.match(databaseOrmHelper, /hostedCredentials: false/);
  assert.match(databaseOrmHelper, /DX_DATABASE_URL/);
  assert.match(databaseOrmHelper, /DX_SQLITE_DATABASE_PATH/);
  assert.match(
    databaseOrmHelper,
    /This route validates local database runtime readiness only/,
  );
  assert.doesNotMatch(
    databaseOrmHelper,
    /new Database|better-sqlite3|drizzle\(|migrate\(|\.select\(/,
    "readiness route must not open SQLite, instantiate Drizzle, run migrations, or query data",
  );
  assert.equal(response.status, 501);
  assert.equal(response.headers.get("cache-control"), "no-store");
  assert.equal(payload.schema, "dx.www.template.database_orm_readiness");
  assert.equal(payload.route, "/api/database-orm/readiness");
  assert.equal(payload.status, "runtime-gated");
  assert.equal(payload.runtimeProof, false);
  assert.equal(payload.networkCalls, false);
  assert.equal(payload.hostedCredentials, false);
  assert.deepEqual(payload.missingConfig, [
    "DX_DATABASE_URL or DX_SQLITE_DATABASE_PATH",
    "DX_DATABASE_MIGRATIONS_REVIEWED",
    "DX_DATABASE_AUTHORIZATION_REVIEWED",
  ]);
});
