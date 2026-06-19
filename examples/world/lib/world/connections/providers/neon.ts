import type {
  WorldConnectionContext,
  WorldConnectionEnvStatus,
  WorldConnectionFetch,
  WorldConnectionProbe,
  WorldConnectionResult,
} from "../contracts";
import { connectionResult } from "../http";
import { readableEndpointLabel } from "../redaction";

export const neonCrudRequiredEnv = ["NEON_DATABASE_URL or DATABASE_URL"] as const;
export const neonCrudAcceptedEnv = ["NEON_DATABASE_URL", "DATABASE_URL"] as const;
export const neonCrudOptionalEnv = ["NEON_API_KEY", "NEON_PROJECT_ID"] as const;

export type NeonCrudState = "missing-config" | "live-validated" | "blocked";

export type NeonCrudStepName =
  | "create-table"
  | "insert-row"
  | "read-row"
  | "update-row"
  | "read-updated-row"
  | "delete-row"
  | "drop-table";

export type NeonCrudStep = {
  name: NeonCrudStepName;
  ok: boolean;
  httpStatus?: number;
  endpoint: string;
  evidence: string;
  rowCount?: number;
};

export type NeonDatabaseCrudReceipt = {
  schema: "dx.examples.world.neon-database-crud";
  providerId: "neon";
  packageId: "database/neon";
  category: "Serverless Postgres";
  status: NeonCrudState;
  redaction: "secret-values-never-included";
  checkedAt: string;
  projectHost: string;
  databaseName: string;
  tableName: string;
  requiredEnv: readonly string[];
  acceptedEnv: readonly string[];
  optionalEnv: readonly string[];
  presentEnv: readonly string[];
  missingEnv: readonly string[];
  secretValues: [];
  liveProviderExecution: boolean;
  steps: readonly NeonCrudStep[];
  nextAction: string;
};

export type NeonDatabaseCrudOptions = {
  env?: Record<string, string | undefined>;
  fetch?: WorldConnectionFetch;
  now?: () => Date;
  tableName?: string;
  rowId?: string;
};

type NeonContext = {
  connectionString: string;
  endpoint: URL;
  checkedAt: string;
  projectHost: string;
  databaseName: string;
  tableName: string;
  rowId: string;
  presentEnv: readonly string[];
  missingEnv: readonly string[];
};

type NeonSqlResult = {
  ok: boolean;
  httpStatus?: number;
  endpoint: string;
  rowCount?: number;
  evidence: string;
};

type NeonSqlQuery = {
  query: string;
  params?: readonly unknown[];
};

declare const process:
  | {
      env?: Record<string, string | undefined>;
    }
  | undefined;

export const neonConnectionProbes: readonly WorldConnectionProbe[] = [
  {
    id: "neon-http-sql-select-1",
    providerId: "neon",
    packageId: "database/neon",
    name: "Neon HTTP SQL SELECT 1",
    category: "Serverless Postgres",
    kind: "http",
    endpoint: "env:NEON_DATABASE_URL/sql",
    documentationUrl: "https://neon.com/docs/serverless/serverless-driver",
    requiredEnv: [],
    optionalEnv: [...neonCrudAcceptedEnv, ...neonCrudOptionalEnv],
    run: runNeonReadinessProbe,
  },
];

export async function runNeonDatabaseCrudSmoke(
  options: NeonDatabaseCrudOptions = {},
): Promise<NeonDatabaseCrudReceipt> {
  const env = options.env ?? process?.env ?? {};
  const fetchImpl = options.fetch ?? fetch;
  const context = buildNeonContext(env, options);

  if (context.missingEnv.length > 0) {
    return neonReceipt({
      context,
      status: "missing-config",
      liveProviderExecution: false,
      steps: [],
      nextAction: "Set NEON_DATABASE_URL or DATABASE_URL before running Neon database CRUD proof.",
    });
  }

  const table = quoteIdentifier(context.tableName);
  const steps: NeonCrudStep[] = [];

  steps.push(
    stepResult(
      "create-table",
      await runNeonSql(fetchImpl, context, {
        query: `CREATE TABLE IF NOT EXISTS ${table} (
          id text PRIMARY KEY,
          title text NOT NULL,
          counter integer NOT NULL,
          updated_at timestamptz NOT NULL DEFAULT now()
        )`,
      }),
    ),
  );

  steps.push(
    stepResult(
      "insert-row",
      await runNeonSql(fetchImpl, context, {
        query: `INSERT INTO ${table} (id, title, counter) VALUES ($1, $2, $3)`,
        params: [context.rowId, "created-from-dx-www", 1],
      }),
    ),
  );

  steps.push(
    stepResult(
      "read-row",
      await runNeonSql(fetchImpl, context, {
        query: `SELECT id, title, counter FROM ${table} WHERE id = $1`,
        params: [context.rowId],
      }),
    ),
  );

  steps.push(
    stepResult(
      "update-row",
      await runNeonSql(fetchImpl, context, {
        query: `UPDATE ${table} SET title = $2, counter = $3, updated_at = now() WHERE id = $1`,
        params: [context.rowId, "updated-from-dx-www", 2],
      }),
    ),
  );

  steps.push(
    stepResult(
      "read-updated-row",
      await runNeonSql(fetchImpl, context, {
        query: `SELECT id, title, counter FROM ${table} WHERE id = $1`,
        params: [context.rowId],
      }),
    ),
  );

  steps.push(
    stepResult(
      "delete-row",
      await runNeonSql(fetchImpl, context, {
        query: `DELETE FROM ${table} WHERE id = $1`,
        params: [context.rowId],
      }),
    ),
  );

  steps.push(
    stepResult(
      "drop-table",
      await runNeonSql(fetchImpl, context, {
        query: `DROP TABLE IF EXISTS ${table}`,
      }),
    ),
  );

  const ok = steps.every((step) => step.ok);

  return neonReceipt({
    context,
    status: ok ? "live-validated" : "blocked",
    liveProviderExecution: true,
    steps,
    nextAction: ok
      ? "Import this redacted Neon database CRUD receipt."
      : "Review the failed Neon SQL step before claiming live database write proof.",
  });
}

async function runNeonReadinessProbe(
  context: WorldConnectionContext,
  envStatus: WorldConnectionEnvStatus,
): Promise<WorldConnectionResult> {
  const neon = buildNeonContext(context.env, { now: () => new Date(context.checkedAt) });

  if (neon.missingEnv.length > 0) {
    return connectionResult({
      probe: neonConnectionProbes[0],
      context,
      envStatus: {
        ...envStatus,
        requiredEnv: neonCrudRequiredEnv,
        missingEnv: neon.missingEnv,
        presentEnv: neon.presentEnv,
      },
      state: "missing-config",
      ok: false,
      durationMs: 0,
      evidence: "required-env-missing",
      message: "Neon database env is missing; SQL SELECT 1 was not attempted.",
    });
  }

  if (!context.allowNetwork) {
    return connectionResult({
      probe: neonConnectionProbes[0],
      context,
      envStatus: {
        ...envStatus,
        requiredEnv: neonCrudRequiredEnv,
        missingEnv: [],
        presentEnv: neon.presentEnv,
      },
      state: "configured-readiness",
      ok: false,
      durationMs: 0,
      evidence: "network-disabled",
      message: "Neon database env is present, but live network probing is disabled.",
    });
  }

  const startedAt = Date.now();
  const result = await runNeonSql(context.fetch, neon, {
    query: "SELECT 1 AS dx_www_ok",
  });

  return connectionResult({
    probe: {
      ...neonConnectionProbes[0],
      endpoint: result.endpoint,
    },
    context,
    envStatus: {
      ...envStatus,
      requiredEnv: neonCrudRequiredEnv,
      missingEnv: [],
      presentEnv: neon.presentEnv,
    },
    state: result.ok ? "live-validated" : "blocked",
    ok: result.ok,
    durationMs: Date.now() - startedAt,
    evidence: result.evidence,
    message: result.ok
      ? "Neon HTTP SQL SELECT 1 completed successfully."
      : "Neon HTTP SQL SELECT 1 did not pass.",
    httpStatus: result.httpStatus,
  });
}

function buildNeonContext(
  env: Record<string, string | undefined>,
  options: NeonDatabaseCrudOptions,
): NeonContext {
  const checkedAt = (options.now?.() ?? new Date()).toISOString();
  const connectionString = readEnv(env, "NEON_DATABASE_URL") ?? readEnv(env, "DATABASE_URL");
  const presentEnv = [...neonCrudAcceptedEnv, ...neonCrudOptionalEnv].filter((name) =>
    Boolean(readEnv(env, name)),
  );
  const missingEnv = connectionString ? [] : ["NEON_DATABASE_URL or DATABASE_URL"];
  const parsed = connectionString
    ? parsePostgresConnectionString(connectionString)
    : {
        endpoint: new URL("https://api.preview-only.neon.tech/sql"),
        projectHost: "preview-only.neon.tech",
        databaseName: "preview-only",
      };
  const runId = checkedAt.replace(/\D/g, "").slice(0, 14) || "00000000000000";

  return {
    connectionString: connectionString ?? "",
    endpoint: parsed.endpoint,
    checkedAt,
    projectHost: parsed.projectHost,
    databaseName: parsed.databaseName,
    tableName: options.tableName ?? `dx_www_world_crud_${runId}`,
    rowId: options.rowId ?? `dx-www-${runId}`,
    presentEnv,
    missingEnv,
  };
}

function parsePostgresConnectionString(connectionString: string): {
  endpoint: URL;
  projectHost: string;
  databaseName: string;
} {
  const url = new URL(connectionString);

  if (url.protocol !== "postgresql:" && url.protocol !== "postgres:") {
    throw new Error("Neon database URL must use postgresql:// or postgres://.");
  }

  const hostParts = url.hostname.split(".");
  hostParts[0] = "api";

  return {
    endpoint: new URL(`https://${hostParts.join(".")}/sql`),
    projectHost: url.hostname,
    databaseName: url.pathname.replace(/^\/+/, "") || "postgres",
  };
}

function readEnv(env: Record<string, string | undefined>, name: string): string | null {
  const value = env[name]?.trim();
  return value ? value : null;
}

async function runNeonSql(
  fetchImpl: WorldConnectionFetch,
  context: NeonContext,
  query: NeonSqlQuery,
): Promise<NeonSqlResult> {
  try {
    const response = await fetchImpl(context.endpoint, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        "Neon-Connection-String": context.connectionString,
        "Neon-Raw-Text-Output": "true",
        "Neon-Array-Mode": "true",
      },
      body: JSON.stringify({
        query: query.query,
        params: query.params ?? [],
      }),
    });
    const rowCount = await readRowCount(response);

    return {
      ok: response.ok,
      httpStatus: response.status,
      endpoint: readableEndpointLabel(context.endpoint.toString()),
      rowCount,
      evidence: response.ok ? "neon-http-sql-ok" : `http-status-${response.status}`,
    };
  } catch (error) {
    return {
      ok: false,
      endpoint: readableEndpointLabel(context.endpoint.toString()),
      evidence: error instanceof Error ? error.name : "unknown-error",
    };
  }
}

async function readRowCount(response: Response): Promise<number | undefined> {
  try {
    const body = await response.json();

    if (typeof body?.rowCount === "number") {
      return body.rowCount;
    }

    if (Array.isArray(body?.rows)) {
      return body.rows.length;
    }
  } catch {
    return undefined;
  }

  return undefined;
}

function stepResult(name: NeonCrudStepName, result: NeonSqlResult): NeonCrudStep {
  return {
    name,
    ok: result.ok,
    httpStatus: result.httpStatus,
    endpoint: result.endpoint,
    evidence: result.evidence,
    rowCount: result.rowCount,
  };
}

function neonReceipt(input: {
  context: NeonContext;
  status: NeonCrudState;
  liveProviderExecution: boolean;
  steps: readonly NeonCrudStep[];
  nextAction: string;
}): NeonDatabaseCrudReceipt {
  return {
    schema: "dx.examples.world.neon-database-crud",
    providerId: "neon",
    packageId: "database/neon",
    category: "Serverless Postgres",
    status: input.status,
    redaction: "secret-values-never-included",
    checkedAt: input.context.checkedAt,
    projectHost: input.context.projectHost,
    databaseName: input.context.databaseName,
    tableName: input.context.tableName,
    requiredEnv: neonCrudRequiredEnv,
    acceptedEnv: neonCrudAcceptedEnv,
    optionalEnv: neonCrudOptionalEnv,
    presentEnv: input.context.presentEnv,
    missingEnv: input.context.missingEnv,
    secretValues: [],
    liveProviderExecution: input.liveProviderExecution,
    steps: input.steps,
    nextAction: input.nextAction,
  };
}

function quoteIdentifier(identifier: string): string {
  if (!/^[_a-zA-Z][_a-zA-Z0-9]*$/.test(identifier)) {
    throw new Error("Neon CRUD table name must be a simple SQL identifier.");
  }

  return `"${identifier}"`;
}
