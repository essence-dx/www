import { createTemplateBetterAuthReadiness } from "@/server/auth/better-auth";

export const runtime = "nodejs";

export function GET() {
  const readiness = createTemplateBetterAuthReadiness();

  return Response.json(
    {
      ok: true,
      ...readiness,
      packageReadinessStatus: readiness.status,
      status: readiness.canRunRouteHandlers
        ? "ready"
        : "adapter-boundary",
      liveRouteHandlersHttpStatus: readiness.canRunRouteHandlers ? 200 : 501,
      runtimeExecution: false,
      liveSessionExecution: false,
      adapter: "better-auth",
      officialPackageName: "Authentication",
      upstreamPackage: "better-auth",
      databaseAdapterConfigured: readiness.databaseAdapterConfigured,
      sessionStorage: readiness.sessionStorage,
      adapterBoundaries: readiness.adapterBoundaries,
      databaseBoundary: readiness.databaseBoundary,
      migrationsRequired: readiness.migrationsRequired,
    },
    { status: 200 },
  );
}
