export type DatabaseApiSchemaSurface = {
  readonly packageId: string;
  readonly officialName: string;
  readonly sourceFile: string;
  readonly kind: "drizzle-sqlite" | "instantdb-schema" | "supabase-profile";
  readonly tables?: readonly string[];
  readonly entities?: readonly string[];
  readonly rooms?: readonly string[];
  readonly env?: readonly string[];
  readonly localProof: string;
  readonly runtimeProof: false;
  readonly networkCalls: false;
  readonly hostedCredentials: false;
  readonly appOwnedBoundary: readonly string[];
};

export type DatabaseApiRouteSurface = {
  readonly packageId: string;
  readonly route: string;
  readonly methods: readonly string[];
  readonly handlerFile: string;
  readonly runtimeProof: false;
  readonly networkCalls: false;
  readonly hostedCredentials: false;
  readonly appOwnedBoundary: readonly string[];
};

export type DatabaseApiTrpcProcedureSurface = {
  readonly path: string;
  readonly kind: "query" | "mutation" | "subscription";
  readonly sourceFile: "lib/trpc/router.ts";
  readonly localProof: string;
  readonly runtimeProof: false;
  readonly appOwnedBoundary: readonly string[];
};

export type DatabaseApiSourceContract = {
  readonly schema: "dx.www.template.database_api_source_contract";
  readonly route: "/api/database-api/readiness";
  readonly runtimeProof: false;
  readonly networkCalls: false;
  readonly hostedCredentials: false;
  readonly schemaSurfaces: readonly DatabaseApiSchemaSurface[];
  readonly routeSurfaces: readonly DatabaseApiRouteSurface[];
  readonly trpcProcedures: readonly DatabaseApiTrpcProcedureSurface[];
};

export const databaseApiSourceContract = {
  schema: "dx.www.template.database_api_source_contract",
  route: "/api/database-api/readiness",
  runtimeProof: false,
  networkCalls: false,
  hostedCredentials: false,
  schemaSurfaces: [
    {
      packageId: "db/drizzle-sqlite",
      officialName: "Database ORM",
      sourceFile: "db/drizzle/schema.ts",
      kind: "drizzle-sqlite",
      tables: ["users", "posts"],
      localProof:
        "Drizzle schema source defines users/posts tables, indexes, relations, and select/insert model types.",
      runtimeProof: false,
      networkCalls: false,
      hostedCredentials: false,
      appOwnedBoundary: [
        "SQLite database path",
        "better-sqlite3 installation",
        "migration execution",
        "tenant access policy",
      ],
    },
    {
      packageId: "instantdb/react",
      officialName: "Realtime App Database",
      sourceFile: "lib/instant/schema.ts",
      kind: "instantdb-schema",
      entities: ["todos", "labels"],
      rooms: ["launch"],
      env: ["NEXT_PUBLIC_INSTANT_APP_ID"],
      localProof:
        "Instant schema source defines todos, labels, todoLabels, launch presence, and launchPing topic surfaces.",
      runtimeProof: false,
      networkCalls: false,
      hostedCredentials: false,
      appOwnedBoundary: [
        "hosted Instant app id",
        "rules and auth policy",
        "realtime connection",
        "storage and stream retention",
      ],
    },
    {
      packageId: "supabase/client",
      officialName: "Backend Platform Client",
      sourceFile: "lib/supabase/profiles.ts",
      kind: "supabase-profile",
      tables: ["profiles"],
      env: [
        "NEXT_PUBLIC_SUPABASE_URL",
        "NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY",
      ],
      localProof:
        "Supabase profile source defines typed profile rows, select/upsert helpers, and a missing-config public env gate.",
      runtimeProof: false,
      networkCalls: false,
      hostedCredentials: false,
      appOwnedBoundary: [
        "hosted Supabase project",
        "RLS policy",
        "auth redirect allow-list",
        "read/write/realtime proof",
      ],
    },
  ],
  routeSurfaces: [
    {
      packageId: "instantdb/react",
      route: "/api/instant",
      methods: ["POST"],
      handlerFile: "app/api/instant/route.ts",
      runtimeProof: false,
      networkCalls: false,
      hostedCredentials: false,
      appOwnedBoundary: ["Instant hosted app id and route auth policy"],
    },
    {
      packageId: "instantdb/react",
      route: "/api/instant/readiness",
      methods: ["GET"],
      handlerFile: "app/api/instant/readiness/route.ts",
      runtimeProof: false,
      networkCalls: false,
      hostedCredentials: false,
      appOwnedBoundary: [
        "Instant hosted app id, rules, auth policy, realtime transport, storage, and stream proof",
      ],
    },
    {
      packageId: "api/trpc",
      route: "/api/trpc/health",
      methods: ["GET", "POST"],
      handlerFile: "app/api/trpc/health/route.ts",
      runtimeProof: false,
      networkCalls: false,
      hostedCredentials: false,
      appOwnedBoundary: [
        "production auth context, transport policy, persistence, subscriptions, and observability",
      ],
    },
    {
      packageId: "api/trpc",
      route: "/api/trpc/[trpc]",
      methods: ["GET", "POST"],
      handlerFile: "app/api/trpc/[trpc]/route.ts",
      runtimeProof: false,
      networkCalls: false,
      hostedCredentials: false,
      appOwnedBoundary: ["production auth context and transport limits"],
    },
    {
      packageId: "api/trpc",
      route: "/api/database-api/readiness",
      methods: ["GET"],
      handlerFile: "app/api/database-api/readiness/route.ts",
      runtimeProof: false,
      networkCalls: false,
      hostedCredentials: false,
      appOwnedBoundary: ["provider runtime proof stays outside this local readiness route"],
    },
    {
      packageId: "db/drizzle-sqlite",
      route: "/api/database-orm/readiness",
      methods: ["GET"],
      handlerFile: "app/api/database-orm/readiness/route.ts",
      runtimeProof: false,
      networkCalls: false,
      hostedCredentials: false,
      appOwnedBoundary: [
        "database location, driver install, migrations, and tenant authorization review",
      ],
    },
    {
      packageId: "supabase/client",
      route: "/api/supabase/readiness",
      methods: ["GET"],
      handlerFile: "app/api/supabase/readiness/route.ts",
      runtimeProof: false,
      networkCalls: false,
      hostedCredentials: false,
      appOwnedBoundary: [
        "Supabase public project configuration and hosted runtime proof",
      ],
    },
  ],
  trpcProcedures: [
    {
      path: "health",
      kind: "query",
      sourceFile: "lib/trpc/router.ts",
      localProof: "Returns local request id and server timestamp through the typed router.",
      runtimeProof: false,
      appOwnedBoundary: ["deployment health policy and observability"],
    },
    {
      path: "launchReadiness",
      kind: "query",
      sourceFile: "lib/trpc/router.ts",
      localProof: "Returns the typed launch readiness result for a template input.",
      runtimeProof: false,
      appOwnedBoundary: ["production caller auth and rate limits"],
    },
    {
      path: "launchEvents",
      kind: "query",
      sourceFile: "lib/trpc/router.ts",
      localProof: "Returns typed paginated launch event fixture rows.",
      runtimeProof: false,
      appOwnedBoundary: ["durable event store and retention"],
    },
    {
      path: "launchEvent",
      kind: "mutation",
      sourceFile: "lib/trpc/router.ts",
      localProof: "Accepts a typed launch event mutation input and returns a local receipt shape.",
      runtimeProof: false,
      appOwnedBoundary: ["write authorization and persistence"],
    },
    {
      path: "launchFeed",
      kind: "subscription",
      sourceFile: "lib/trpc/router.ts",
      localProof: "The subscription procedure is source-visible but not claimed as live transport proof.",
      runtimeProof: false,
      appOwnedBoundary: ["WebSocket/SSE transport, fan-out, retry, and stream lifecycle"],
    },
  ],
} as const satisfies DatabaseApiSourceContract;
