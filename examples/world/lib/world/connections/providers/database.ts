import {
  buildProviderResult,
  checkedAt,
  configuredProbe,
  isSafeReadinessUrl,
  missingConfigResult,
  missingEnvNames,
  readEnvValue,
  readProviderEnv,
} from "./env";
import type {
  DataIdentityConnectionResult,
  DataIdentityProviderDefinition,
  DataIdentityProbeContext,
} from "./data-identity-types";
import type { WorldConnectionProbe } from "../contracts";
import { connectionResult, runReadOnlyHttpProbe } from "../http";
import { configuredReadinessProbe } from "./shared";

export type WorldDatabaseProviderId =
  | "postgresql"
  | "neon"
  | "supabase"
  | "turso-libsql"
  | "dx-orm-forge-database"
  | "drizzle"
  | "prisma";

export const databaseProviderDefinitions = [
  databaseProvider("postgresql", "PostgreSQL", ["DATABASE_URL"], ["POSTGRES_READINESS_URL"]),
  databaseProvider("neon", "Neon", ["NEON_DATABASE_URL"], ["NEON_READINESS_URL", "NEON_API_KEY"]),
  databaseProvider("supabase", "Supabase", [
    "NEXT_PUBLIC_SUPABASE_URL",
    "NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY",
  ], ["SUPABASE_READINESS_URL"]),
  databaseProvider("turso-libsql", "Turso/libSQL", [
    "TURSO_DATABASE_URL",
    "TURSO_AUTH_TOKEN",
  ], []),
  databaseProvider("dx-orm-forge-database", "DX ORM / Forge database", ["DATABASE_URL"], []),
  databaseProvider("drizzle", "Drizzle", ["DATABASE_URL"], ["DRIZZLE_CONFIG_PATH"]),
  databaseProvider("prisma", "Prisma", ["DATABASE_URL"], ["PRISMA_SCHEMA_PATH"]),
] as const satisfies readonly DataIdentityProviderDefinition<WorldDatabaseProviderId>[];

export const databaseConnectionProbes: readonly WorldConnectionProbe[] = [
  optionalSafeDatabaseHttpProbe({
    id: "postgresql-safe-http-readiness",
    providerId: "postgresql",
    packageId: "database/postgresql",
    name: "PostgreSQL readiness endpoint",
    endpointEnv: "POSTGRES_READINESS_URL",
    requiredEnv: ["DATABASE_URL"],
    optionalEnv: ["POSTGRES_READINESS_URL"],
    documentationUrl: "https://www.postgresql.org/docs/current/libpq-connect.html",
  }),
  optionalSafeDatabaseHttpProbe({
    id: "neon-safe-http-readiness",
    providerId: "neon",
    packageId: "database/neon",
    name: "Neon readiness endpoint",
    endpointEnv: "NEON_READINESS_URL",
    requiredEnv: ["NEON_DATABASE_URL"],
    optionalEnv: ["NEON_READINESS_URL", "NEON_API_KEY"],
    documentationUrl: "https://neon.com/docs/get-started/connect-neon",
  }),
  optionalSafeDatabaseHttpProbe({
    id: "supabase-config-readiness",
    providerId: "supabase",
    packageId: "database/supabase",
    name: "Supabase readiness endpoint",
    endpointEnv: "SUPABASE_READINESS_URL",
    requiredEnv: ["NEXT_PUBLIC_SUPABASE_URL", "NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY"],
    optionalEnv: ["SUPABASE_READINESS_URL"],
    documentationUrl: "https://supabase.com/docs/guides/api",
  }),
  configuredReadinessProbe(
    {
      id: "dx-orm-forge-database-config-readiness",
      providerId: "dx-orm-forge-database",
      packageId: "database/dx-orm-forge-database",
      name: "DX ORM / Forge database",
      category: "ORM",
      kind: "env",
      endpoint: "env:DATABASE_URL",
      requiredEnv: ["DATABASE_URL"],
      optionalEnv: [],
    },
    "DATABASE_URL is present; schema, migration, and write execution remain app-owned.",
  ),
  configuredReadinessProbe(
    {
      id: "drizzle-config-readiness",
      providerId: "drizzle",
      packageId: "orm/drizzle",
      name: "Drizzle",
      category: "ORM",
      kind: "env",
      endpoint: "env:DATABASE_URL",
      requiredEnv: ["DATABASE_URL"],
      optionalEnv: ["DRIZZLE_CONFIG_PATH"],
    },
    "DATABASE_URL is present; Drizzle schema and migrations remain app-owned until a driver adapter runs.",
  ),
  configuredReadinessProbe(
    {
      id: "prisma-config-readiness",
      providerId: "prisma",
      packageId: "orm/prisma",
      name: "Prisma",
      category: "ORM",
      kind: "env",
      endpoint: "env:DATABASE_URL",
      requiredEnv: ["DATABASE_URL"],
      optionalEnv: ["PRISMA_SCHEMA_PATH"],
    },
    "DATABASE_URL is present; Prisma client generation and migration execution remain app-owned.",
  ),
];

export async function probeDatabaseProvider(
  providerId: WorldDatabaseProviderId,
  context: DataIdentityProbeContext = {},
): Promise<DataIdentityConnectionResult<WorldDatabaseProviderId>> {
  const definition = readDatabaseProvider(providerId);
  const env = readProviderEnv(context);

  if (missingEnvNames(env, definition.requiredEnv).length > 0) {
    return missingConfigResult(definition, context);
  }

  if (providerId === "turso-libsql") {
    return probeTurso(definition, context);
  }

  return probeConfiguredDatabase(definition, context);
}

export function readDatabaseProvider(
  providerId: WorldDatabaseProviderId,
): DataIdentityProviderDefinition<WorldDatabaseProviderId> {
  const definition = databaseProviderDefinitions.find((item) => item.id === providerId);
  if (!definition) {
    throw new Error(`Unsupported world database provider: ${providerId}`);
  }
  return definition;
}

function databaseProvider(
  id: WorldDatabaseProviderId,
  name: string,
  requiredEnv: readonly string[],
  optionalEnv: readonly string[],
): DataIdentityProviderDefinition<WorldDatabaseProviderId> {
  return {
    id,
    name,
    kind: "database",
    category: id === "drizzle" || id === "prisma" || id === "dx-orm-forge-database"
      ? "ORM, query builder, and schema tooling"
      : "Database and data platform",
    requiredEnv,
    optionalEnv,
    receiptSchemas: [
      "dx.forge.world.connection",
      "dx.forge.world.schema",
      "dx.forge.world.provider-live-proof",
      "dx.forge.world.preview-only",
    ],
    appOwnedBoundary:
      "Schema design, migrations, data writes, credentials, authorization, backup policy, and dependency installation stay app-owned.",
    statusEndpointEnv: optionalEnv.find((name) => name.endsWith("_READINESS_URL")),
  };
}

async function probeConfiguredDatabase(
  definition: DataIdentityProviderDefinition<WorldDatabaseProviderId>,
  context: DataIdentityProbeContext,
): Promise<DataIdentityConnectionResult<WorldDatabaseProviderId>> {
  const endpointName = definition.statusEndpointEnv;
  const endpoint = endpointName ? readEnvValue(readProviderEnv(context), endpointName) : null;

  if (!endpoint || !context.fetch) {
    return buildProviderResult({
      context,
      definition,
      nextAction:
        "Run a read-only provider check from an app-owned route or add a safe HTTPS readiness endpoint.",
      probe: configuredProbe(
        context,
        endpoint
          ? "Provider env is configured; fetch is unavailable so the safe HTTP readiness endpoint was not called."
          : "Provider env is configured; no safe HTTP readiness endpoint is declared.",
      ),
      status: "configured-readiness",
    });
  }

  if (!isSafeReadinessUrl(endpoint)) {
    return buildProviderResult({
      blockers: [`${endpointName} must be an HTTPS URL without credentials, query, or fragment.`],
      context,
      definition,
      nextAction: `Replace ${endpointName} with a safe read-only readiness endpoint.`,
      probe: configuredProbe(context, "Provider env is configured; unsafe readiness URL was not called."),
      status: "configured-readiness",
    });
  }

  const response = await context.fetch(endpoint, {
    method: "GET",
    headers: { "Cache-Control": "no-store" },
  });

  return buildProviderResult({
    blockers: response.ok ? [] : [`${endpointName} returned HTTP ${response.status}.`],
    context,
    definition,
    nextAction: response.ok
      ? "Import the redacted provider-live-proof receipt."
      : "Fix the provider readiness endpoint before claiming live validation.",
    probe: {
      kind: "safe-http-readiness",
      checkedAt: checkedAt(context),
      live: response.ok,
      endpointEnv: endpointName,
      endpointKind: "safe-http",
      message: response.ok
        ? "Safe HTTPS readiness endpoint responded successfully."
        : "Safe HTTPS readiness endpoint did not pass.",
    },
    status: response.ok ? "live-validated" : "blocked",
  });
}

async function probeTurso(
  definition: DataIdentityProviderDefinition<WorldDatabaseProviderId>,
  context: DataIdentityProbeContext,
): Promise<DataIdentityConnectionResult<WorldDatabaseProviderId>> {
  const env = readProviderEnv(context);
  const databaseUrl = readEnvValue(env, "TURSO_DATABASE_URL");
  const authToken = readEnvValue(env, "TURSO_AUTH_TOKEN");
  const endpoint = databaseUrl ? tursoPipelineEndpoint(databaseUrl) : null;

  if (!endpoint || !authToken || !context.fetch) {
    return buildProviderResult({
      blockers: endpoint ? [] : ["TURSO_DATABASE_URL must be http(s):// or libsql://."],
      context,
      definition,
      nextAction:
        "Provide fetch-capable server runtime and run the Turso SELECT 1 receipt.",
      probe: configuredProbe(
        context,
        endpoint
          ? "Turso env is configured; fetch is unavailable so SELECT 1 was not called."
          : "Turso env is configured, but the database URL cannot be converted to an HTTP pipeline endpoint.",
      ),
      status: "configured-readiness",
    });
  }

  const response = await context.fetch(endpoint, {
    method: "POST",
    headers: {
      Authorization: `Bearer ${authToken}`,
      "Cache-Control": "no-store",
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      requests: [
        {
          type: "execute",
          stmt: { sql: "SELECT 1" },
        },
        { type: "close" },
      ],
    }),
  });

  return buildProviderResult({
    blockers: response.ok ? [] : [`Turso SELECT 1 returned HTTP ${response.status}.`],
    context,
    definition,
    nextAction: response.ok
      ? "Import the Turso provider-live-proof receipt."
      : "Fix Turso credentials, endpoint, or database availability before claiming live validation.",
    probe: {
      kind: "turso-libsql-http-select-1",
      checkedAt: checkedAt(context),
      live: response.ok,
      endpointEnv: "TURSO_DATABASE_URL",
      endpointKind: "safe-http",
      message: response.ok
        ? "Turso HTTP pipeline accepted a read-only SELECT 1 probe."
        : "Turso HTTP pipeline did not pass the read-only SELECT 1 probe.",
    },
    status: response.ok ? "live-validated" : "blocked",
  });
}

function tursoPipelineEndpoint(databaseUrl: string): string | null {
  try {
    const url = new URL(databaseUrl);
    if (url.username || url.password || url.search || url.hash) {
      return null;
    }

    if (url.protocol === "libsql:") {
      return `https://${url.host}/v2/pipeline`;
    }

    if (url.protocol === "https:" || url.protocol === "http:") {
      return `${url.origin}/v2/pipeline`;
    }

    return null;
  } catch {
    return null;
  }
}

function optionalSafeDatabaseHttpProbe(input: {
  id: string;
  providerId: string;
  packageId: string;
  name: string;
  endpointEnv: string;
  requiredEnv: readonly string[];
  optionalEnv: readonly string[];
  documentationUrl?: string;
}): WorldConnectionProbe {
  const base = {
    id: input.id,
    providerId: input.providerId,
    packageId: input.packageId,
    name: input.name,
    category: "Database",
    kind: "http" as const,
    endpoint: `env:${input.endpointEnv}`,
    documentationUrl: input.documentationUrl,
    requiredEnv: input.requiredEnv,
    optionalEnv: input.optionalEnv,
  };

  return {
    ...base,
    run: async (context, envStatus) => {
      const endpoint = context.env[input.endpointEnv]?.trim();

      if (!endpoint) {
        return connectionResult({
          probe: base,
          context,
          envStatus,
          state: "configured-readiness",
          ok: true,
          durationMs: 0,
          evidence: "env-contract-satisfied",
          message:
            "Provider env is present; no safe HTTP readiness endpoint is configured.",
        });
      }

      if (!isSafeReadinessUrl(endpoint)) {
        return connectionResult({
          probe: base,
          context,
          envStatus,
          state: "configured-readiness",
          ok: true,
          durationMs: 0,
          evidence: "unsafe-readiness-endpoint-skipped",
          message:
            "Configured readiness endpoint was skipped because it is not HTTPS or contains credentials, query, or fragment.",
        });
      }

      return runReadOnlyHttpProbe(context, envStatus, {
        ...base,
        endpoint,
        method: "GET",
        headers: { "Cache-Control": "no-store" },
        expectedStatuses: [200, 204],
        evidence: "safe-http-readiness",
      });
    },
  };
}
