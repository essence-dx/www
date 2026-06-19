import {
  databaseApiSourceContract,
  type DatabaseApiSourceContract,
} from "../../lib/database-api/source-contract.ts";

export type DatabaseApiPackageReadinessStatus = "source-owned-adapter-boundary";

export type DatabaseApiPackageReadiness = {
  readonly packageId: string;
  readonly officialName: string;
  readonly status: DatabaseApiPackageReadinessStatus;
  readonly runtimeProof: false;
  readonly networkCalls: false;
  readonly frontFacingFiles: readonly string[];
  readonly localProof: string;
  readonly appOwnedBoundary: readonly string[];
};

export type DatabaseApiRouteReadiness = {
  readonly schema: "dx.www.template.database_api_readiness";
  readonly laneNumber: 4;
  readonly laneName: "Database + API";
  readonly route: "/api/database-api/readiness";
  readonly templateReadinessReceipt: ".dx/forge/template-readiness/database-api.json";
  readonly runtimeProof: false;
  readonly networkCalls: false;
  readonly hostedCredentials: false;
  readonly cacheEvidence: DatabaseApiCacheEvidence;
  readonly appRouterRoutes: readonly string[];
  readonly serverFiles: readonly string[];
  readonly sourceContract: DatabaseApiSourceContract;
  readonly packages: readonly DatabaseApiPackageReadiness[];
  readonly boundary: string;
};

export type DatabaseApiCacheEvidence = {
  readonly sourceOfTruth: ".dx/forge/package-status.json";
  readonly currentManifestSet: "cache.manifests";
  readonly currentManifestCountField: "cache.current_manifest_count";
  readonly physicalManifestCountField: "cache.physical_manifest_count";
  readonly stalePhysicalManifestCountField: "cache.stale_physical_manifest_count";
  readonly currentManifestSource: "package-status-current-manifests";
  readonly physicalManifestCaveatId: "physical-cache-matches-current-manifests";
  readonly laneCacheManifests: Record<
    "db/drizzle-sqlite" | "instantdb/react" | "supabase/client" | "api/trpc",
    string
  >;
};

export const databaseApiRouteReadinessPackages = [
  {
    packageId: "db/drizzle-sqlite",
    officialName: "Database ORM",
    status: "source-owned-adapter-boundary",
    runtimeProof: false,
    networkCalls: false,
    frontFacingFiles: [
      "db/drizzle/schema.ts",
      "db/drizzle/dashboard-workflow.ts",
      "server/database-orm/readiness.ts",
      "app/api/database-orm/readiness/route.ts",
    ],
    localProof:
      "Schema, query-plan, dashboard workflow source, and the runtime-gated readiness route are materialized and lock-backed.",
    appOwnedBoundary: [
      "SQLite database path",
      "better-sqlite3 runtime install",
      "migration rollout",
      "tenant authorization",
    ],
  },
  {
    packageId: "instantdb/react",
    officialName: "Realtime App Database",
    status: "source-owned-adapter-boundary",
    runtimeProof: false,
    networkCalls: false,
    frontFacingFiles: [
      "lib/instant/schema.ts",
      "lib/instant/status.ts",
      "app/api/instant/route.ts",
      "server/instant/readiness.ts",
      "app/api/instant/readiness/route.ts",
    ],
    localProof:
      "Schema, missing-config checks, the Instant App Router route surface, and a provider-gated readiness route are materialized.",
    appOwnedBoundary: [
      "NEXT_PUBLIC_INSTANT_APP_ID",
      "hosted rules and auth policy",
      "realtime transport",
      "storage and stream runtime proof",
    ],
  },
  {
    packageId: "supabase/client",
    officialName: "Backend Platform Client",
    status: "source-owned-adapter-boundary",
    runtimeProof: false,
    networkCalls: false,
    frontFacingFiles: [
      "lib/supabase/env.ts",
      "lib/supabase/profiles.ts",
      "lib/supabase/profile-workflow.ts",
      "lib/supabase/.env.example",
    ],
    localProof:
      "Profile read model, config gate, and public-env validation are materialized without hosted credentials.",
    appOwnedBoundary: [
      "Supabase project credentials",
      "RLS migration",
      "Auth redirect allow-list",
      "hosted read/write/realtime proof",
    ],
  },
  {
    packageId: "api/trpc",
    officialName: "Type-Safe API",
    status: "source-owned-adapter-boundary",
    runtimeProof: false,
    networkCalls: false,
    frontFacingFiles: [
      "lib/trpc/router.ts",
      "lib/trpc/route-handler.ts",
      "app/api/trpc/health/route.ts",
      "app/api/trpc/[trpc]/route.ts",
      "lib/database-api/source-contract.ts",
      "server/database-api/readiness.ts",
      "app/api/database-api/readiness/route.ts",
    ],
    localProof:
      "Router, App Router route handlers, and the Database + API readiness contract are materialized and receipt-backed.",
    appOwnedBoundary: [
      "production auth context",
      "transport and subscription policy",
      "request limits",
      "observability",
    ],
  },
] as const satisfies readonly DatabaseApiPackageReadiness[];

export const databaseApiCacheEvidence = {
  sourceOfTruth: ".dx/forge/package-status.json",
  currentManifestSet: "cache.manifests",
  currentManifestCountField: "cache.current_manifest_count",
  physicalManifestCountField: "cache.physical_manifest_count",
  stalePhysicalManifestCountField: "cache.stale_physical_manifest_count",
  currentManifestSource: "package-status-current-manifests",
  physicalManifestCaveatId: "physical-cache-matches-current-manifests",
  laneCacheManifests: {
    "db/drizzle-sqlite": ".dx/forge/cache/db-drizzle-sqlite/0.1.0/manifest.json",
    "instantdb/react": ".dx/forge/cache/instantdb-react/0.0.0-dx.0/manifest.json",
    "supabase/client": ".dx/forge/cache/supabase-client/0.1.0/manifest.json",
    "api/trpc": ".dx/forge/cache/api-trpc/11.17.0-dx.10/manifest.json",
  },
} as const satisfies DatabaseApiCacheEvidence;

export function readDatabaseApiRouteReadiness(): DatabaseApiRouteReadiness {
  return {
    schema: "dx.www.template.database_api_readiness",
    laneNumber: 4,
    laneName: "Database + API",
    route: "/api/database-api/readiness",
    templateReadinessReceipt: ".dx/forge/template-readiness/database-api.json",
    runtimeProof: false,
    networkCalls: false,
    hostedCredentials: false,
    cacheEvidence: databaseApiCacheEvidence,
    appRouterRoutes: [
      "app/api/instant/route.ts",
      "app/api/instant/readiness/route.ts",
      "app/api/trpc/health/route.ts",
      "app/api/trpc/[trpc]/route.ts",
      "app/api/database-api/readiness/route.ts",
      "app/api/database-orm/readiness/route.ts",
      "app/api/supabase/readiness/route.ts",
    ],
    serverFiles: [
      "server/database-api/readiness.ts",
      "server/instant/readiness.ts",
      "server/database-orm/readiness.ts",
      "server/supabase/readiness.ts",
    ],
    sourceContract: databaseApiSourceContract,
    packages: databaseApiRouteReadinessPackages,
    boundary:
      "This route executes locally and reports package readiness only; it does not open hosted database connections, install dependencies, or claim provider runtime proof.",
  };
}

export function createDatabaseApiReadinessResponse(): Response {
  return Response.json(readDatabaseApiRouteReadiness(), {
    status: 200,
    headers: {
      "cache-control": "no-store",
    },
  });
}
