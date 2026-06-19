import type { BetterAuthOptions } from "better-auth";

import type { DxBetterAuthOptionsInput } from "@/auth/better-auth/options";
import { createDxBetterAuthRouteHandlers } from "@/auth/better-auth/route";
import { createDxBetterAuthReadiness } from "@/auth/better-auth/server";

export type DxTemplateBetterAuthDatabase = BetterAuthOptions["database"];

export const dxTemplateBetterAuthDatabaseBoundary = {
  schema: "dx.template.authentication.database_boundary",
  packageId: "auth/better-auth",
  officialPackageName: "Authentication",
  upstreamPackage: "better-auth",
  appOwned: true,
  runtimeProof: false,
  requiredInput: 'BetterAuthOptions["database"]',
  acceptedUpstreamShapes: [
    "Better Auth DBAdapterInstance",
    "Kysely database plus type",
    "SQLite, PostgreSQL, MySQL, D1, or Bun database object accepted by Better Auth",
  ],
  migrationBoundary: {
    owner: "app",
    packageManagerLifecycleRequired: false,
    dxExecutesMigrationCli: false,
    reviewedAdapterPlan: "dx forge import npm better-auth --plan",
    note:
      "Generate and run Better Auth migrations only inside the consuming app's reviewed package-manager workflow; DX/Forge does not run live npm/npx migration commands for the starter.",
  },
  note:
    "Forge materializes the Authentication server boundary, but the app must choose and pass a real Better Auth database adapter before live sessions are enabled.",
} as const;

export function createTemplateBetterAuthReadiness(
  input: DxBetterAuthOptionsInput = {},
) {
  const readiness = createDxBetterAuthReadiness(input);

  return {
    ...readiness,
    databaseBoundary: dxTemplateBetterAuthDatabaseBoundary,
    appOwnedDatabaseAdapter: readiness.databaseAdapterConfigured,
    migrationsRequired: !readiness.databaseAdapterConfigured,
  };
}

export function createTemplateBetterAuthSessionReceipt(
  input: DxBetterAuthOptionsInput = {},
) {
  const readiness = createTemplateBetterAuthReadiness(input);

  return {
    ok: true,
    schema: "dx.template.authentication.session_receipt",
    status: readiness.credentialsConfigured
      ? "configured-anonymous-session"
      : "anonymous-session",
    httpStatus: 200,
    route: "/api/auth/session",
    adapter: "better-auth",
    officialPackageName: "Authentication",
    upstreamPackage: "better-auth",
    authenticated: false,
    session: null,
    runtimeExecution: false,
    liveSessionExecution: false,
    credentialsConfigured: readiness.credentialsConfigured,
    databaseAdapterConfigured: readiness.databaseAdapterConfigured,
    sessionStorage: readiness.sessionStorage,
    missingConfig: readiness.missingConfig,
    baseURL: readiness.baseURL,
    adapterBoundaries: readiness.adapterBoundaries,
    databaseBoundary: readiness.databaseBoundary,
    appOwnedBoundary:
      "Live Better Auth session lookup requires app-owned cookies, database adapter, migrations, and deployment policy.",
  };
}

function createMissingAuthResponse(
  readiness: ReturnType<typeof createTemplateBetterAuthReadiness>,
  method: "GET" | "POST",
) {
  return Response.json(
    {
      ok: false,
      status: "adapter-boundary",
      httpStatus: 501,
      method,
      adapter: "better-auth",
      officialPackageName: "Authentication",
      upstreamPackage: "better-auth",
      runtimeExecution: false,
      liveSessionExecution: false,
      credentialsConfigured: readiness.credentialsConfigured,
      databaseAdapterConfigured: readiness.databaseAdapterConfigured,
      sessionStorage: readiness.sessionStorage,
      missingConfig: readiness.missingConfig,
      adapterBoundaries: readiness.adapterBoundaries,
      databaseBoundary: readiness.databaseBoundary,
      migrationsRequired: readiness.migrationsRequired,
      message:
        "Configure Authentication credentials and pass an app-owned Better Auth database adapter before enabling live sessions.",
    },
    { status: 501 },
  );
}

export function createTemplateBetterAuthRouteHandlers(
  input: DxBetterAuthOptionsInput = {},
) {
  const readiness = createTemplateBetterAuthReadiness(input);

  if (!readiness.canRunRouteHandlers) {
    return {
      GET() {
        return createMissingAuthResponse(readiness, "GET");
      },
      POST() {
        return createMissingAuthResponse(readiness, "POST");
      },
    };
  }

  return createDxBetterAuthRouteHandlers(input);
}

const handlers = createTemplateBetterAuthRouteHandlers();

export const GET = handlers.GET;
export const POST = handlers.POST;
