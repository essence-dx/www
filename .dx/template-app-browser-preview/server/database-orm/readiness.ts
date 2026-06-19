const requiredDatabaseRuntimeConfig = [
  "DX_DATABASE_URL or DX_SQLITE_DATABASE_PATH",
  "DX_DATABASE_MIGRATIONS_REVIEWED",
  "DX_DATABASE_AUTHORIZATION_REVIEWED",
] as const;

export type DxDatabaseOrmReadinessEnv = Partial<
  Record<
    | "DX_DATABASE_URL"
    | "DX_SQLITE_DATABASE_PATH"
    | "DX_DATABASE_MIGRATIONS_REVIEWED"
    | "DX_DATABASE_AUTHORIZATION_REVIEWED",
    string | undefined
  >
>;

export type DxDatabaseOrmReadiness = {
  readonly schema: "dx.www.template.database_orm_readiness";
  readonly packageId: "db/drizzle-sqlite";
  readonly officialName: "Database ORM";
  readonly route: "/api/database-orm/readiness";
  readonly status: "runtime-gated" | "configured-source-owned-adapter-boundary";
  readonly httpStatus: 200 | 501;
  readonly runtimeProof: false;
  readonly networkCalls: false;
  readonly hostedCredentials: false;
  readonly requiredConfig: typeof requiredDatabaseRuntimeConfig;
  readonly missingConfig: readonly string[];
  readonly sourceOwnedSurfaces: readonly string[];
  readonly schemaTables: readonly string[];
  readonly appOwnedBoundary: readonly string[];
  readonly message: string;
};

export function defaultDatabaseOrmReadinessEnv(): DxDatabaseOrmReadinessEnv {
  return {
    DX_DATABASE_URL: process.env.DX_DATABASE_URL,
    DX_SQLITE_DATABASE_PATH: process.env.DX_SQLITE_DATABASE_PATH,
    DX_DATABASE_MIGRATIONS_REVIEWED:
      process.env.DX_DATABASE_MIGRATIONS_REVIEWED,
    DX_DATABASE_AUTHORIZATION_REVIEWED:
      process.env.DX_DATABASE_AUTHORIZATION_REVIEWED,
  };
}

export function readDatabaseOrmReadiness(
  env: DxDatabaseOrmReadinessEnv = defaultDatabaseOrmReadinessEnv(),
): DxDatabaseOrmReadiness {
  const missingConfig = readMissingDatabaseOrmConfig(env);
  const httpStatus = missingConfig.length === 0 ? 200 : 501;

  return {
    schema: "dx.www.template.database_orm_readiness",
    packageId: "db/drizzle-sqlite",
    officialName: "Database ORM",
    route: "/api/database-orm/readiness",
    status:
      httpStatus === 200
        ? "configured-source-owned-adapter-boundary"
        : "runtime-gated",
    httpStatus,
    runtimeProof: false,
    networkCalls: false,
    hostedCredentials: false,
    requiredConfig: requiredDatabaseRuntimeConfig,
    missingConfig,
    sourceOwnedSurfaces: [
      "db/drizzle/schema.ts",
      "db/drizzle/dashboard-workflow.ts",
      "server/database-orm/readiness.ts",
      "app/api/database-orm/readiness/route.ts",
    ],
    schemaTables: ["users", "posts"],
    appOwnedBoundary: [
      "database file or connection URL",
      "SQLite driver package installation",
      "migration review and rollout",
      "tenant authorization policy",
      "backup, retention, and audit policy",
    ],
    message:
      httpStatus === 200
        ? "Database ORM runtime inputs are locally acknowledged; this still does not open a database, run migrations, or prove deployed data access."
        : "This route validates local database runtime readiness only; configure the database location and app-owned reviews before enabling live reads, writes, migrations, or tenant access.",
  };
}

export function createDatabaseOrmReadinessResponse(
  env?: DxDatabaseOrmReadinessEnv,
): Response {
  const readiness = readDatabaseOrmReadiness(env);

  return Response.json(readiness, {
    status: readiness.httpStatus,
    headers: {
      "cache-control": "no-store",
    },
  });
}

function readMissingDatabaseOrmConfig(
  env: DxDatabaseOrmReadinessEnv,
): readonly string[] {
  const hasDatabaseLocation =
    Boolean(env.DX_DATABASE_URL?.trim()) ||
    Boolean(env.DX_SQLITE_DATABASE_PATH?.trim());
  const migrationsReviewed =
    env.DX_DATABASE_MIGRATIONS_REVIEWED?.trim().toLowerCase() === "true";
  const authorizationReviewed =
    env.DX_DATABASE_AUTHORIZATION_REVIEWED?.trim().toLowerCase() === "true";

  return requiredDatabaseRuntimeConfig.filter((item) => {
    if (item === "DX_DATABASE_URL or DX_SQLITE_DATABASE_PATH") {
      return !hasDatabaseLocation;
    }
    if (item === "DX_DATABASE_MIGRATIONS_REVIEWED") {
      return !migrationsReviewed;
    }
    return !authorizationReviewed;
  });
}
