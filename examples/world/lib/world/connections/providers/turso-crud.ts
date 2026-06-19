import type { WorldConnectionFetch } from "../contracts";
import { readableEndpointLabel } from "../redaction";

export const tursoCrudRequiredEnv = ["TURSO_DATABASE_URL", "TURSO_AUTH_TOKEN"] as const;
export const tursoCrudOptionalEnv = ["TURSO_ORGANIZATION", "TURSO_DATABASE_NAME"] as const;

export type TursoCrudState = "missing-config" | "live-validated" | "blocked";

export type TursoCrudStepName =
  | "create-table"
  | "insert-row"
  | "read-row"
  | "update-row"
  | "read-updated-row"
  | "delete-row"
  | "drop-table";

export type TursoCrudStep = {
  name: TursoCrudStepName;
  ok: boolean;
  endpoint: string;
  evidence: string;
  affectedRowCount?: number;
  rowsRead?: number;
  rowsWritten?: number;
};

export type TursoDatabaseCrudReceipt = {
  schema: "dx.examples.world.turso-database-crud";
  providerId: "turso-libsql";
  packageId: "database/turso-libsql";
  category: "Turso/libSQL";
  status: TursoCrudState;
  redaction: "secret-values-never-included";
  checkedAt: string;
  databaseHost: string;
  tableName: string;
  requiredEnv: readonly string[];
  optionalEnv: readonly string[];
  presentEnv: readonly string[];
  missingEnv: readonly string[];
  secretValues: [];
  liveProviderExecution: boolean;
  steps: readonly TursoCrudStep[];
  nextAction: string;
};

export type TursoDatabaseCrudOptions = {
  env?: Record<string, string | undefined>;
  fetch?: WorldConnectionFetch;
  now?: () => Date;
  tableName?: string;
  rowId?: string;
};

type TursoContext = {
  endpoint: URL;
  authToken: string;
  checkedAt: string;
  databaseHost: string;
  tableName: string;
  rowId: string;
  presentEnv: readonly string[];
  missingEnv: readonly string[];
};

type LibsqlValue =
  | { type: "text"; value: string }
  | { type: "integer"; value: string };

type LibsqlStatement = {
  sql: string;
  args?: readonly LibsqlValue[];
};

type LibsqlPipelineRequest =
  | { type: "execute"; stmt: LibsqlStatement }
  | { type: "close" };

type LibsqlPipelineResult = {
  type?: string;
  response?: {
    type?: string;
    result?: {
      affected_row_count?: number;
      rows_read?: number;
      rows_written?: number;
    };
  };
  error?: {
    message?: string;
  };
};

declare const process:
  | {
      env?: Record<string, string | undefined>;
    }
  | undefined;

const stepNames = [
  "create-table",
  "insert-row",
  "read-row",
  "update-row",
  "read-updated-row",
  "delete-row",
  "drop-table",
] as const satisfies readonly TursoCrudStepName[];

export async function runTursoDatabaseCrudSmoke(
  options: TursoDatabaseCrudOptions = {},
): Promise<TursoDatabaseCrudReceipt> {
  const env = options.env ?? process?.env ?? {};
  const fetchImpl = options.fetch ?? fetch;
  const context = buildTursoContext(env, options);

  if (context.missingEnv.length > 0) {
    return tursoReceipt({
      context,
      status: "missing-config",
      liveProviderExecution: false,
      steps: [],
      nextAction: `Set ${context.missingEnv.join(", ")} before running Turso/libSQL CRUD proof.`,
    });
  }

  const statements = crudStatements(context);
  const pipelineResult = await runTursoPipeline(fetchImpl, context, statements);
  const steps = stepNames.map((name, index) =>
    stepFromPipelineResult(name, context, pipelineResult.results[index]),
  );
  const ok = pipelineResult.ok && steps.every((step) => step.ok);

  return tursoReceipt({
    context,
    status: ok ? "live-validated" : "blocked",
    liveProviderExecution: true,
    steps,
    nextAction: ok
      ? "Import this redacted Turso/libSQL database CRUD receipt."
      : "Review the failed Turso/libSQL HTTP pipeline step before claiming live database write proof.",
  });
}

function buildTursoContext(
  env: Record<string, string | undefined>,
  options: TursoDatabaseCrudOptions,
): TursoContext {
  const checkedAt = (options.now?.() ?? new Date()).toISOString();
  const databaseUrl = readEnv(env, "TURSO_DATABASE_URL");
  const authToken = readEnv(env, "TURSO_AUTH_TOKEN");
  const endpoint = databaseUrl ? tursoPipelineEndpoint(databaseUrl) : null;
  const presentEnv = [...tursoCrudRequiredEnv, ...tursoCrudOptionalEnv].filter((name) =>
    Boolean(readEnv(env, name)),
  );
  const missingEnv = [
    ...(databaseUrl ? [] : ["TURSO_DATABASE_URL"]),
    ...(authToken ? [] : ["TURSO_AUTH_TOKEN"]),
    ...(endpoint || !databaseUrl ? [] : ["valid TURSO_DATABASE_URL"]),
  ];
  const runId = checkedAt.replace(/\D/g, "").slice(0, 14) || "00000000000000";

  return {
    endpoint: endpoint ?? new URL("https://preview-only.turso.io/v2/pipeline"),
    authToken: authToken ?? "",
    checkedAt,
    databaseHost: endpoint?.host ?? "preview-only.turso.io",
    tableName: options.tableName ?? `dx_www_crud_${runId}`,
    rowId: options.rowId ?? `dx-www-${runId}`,
    presentEnv,
    missingEnv,
  };
}

function readEnv(env: Record<string, string | undefined>, name: string): string | null {
  const value = env[name]?.trim();
  return value ? value : null;
}

function tursoPipelineEndpoint(databaseUrl: string): URL | null {
  try {
    const url = new URL(databaseUrl);

    if (url.username || url.password || url.search || url.hash) {
      return null;
    }

    if (url.protocol === "libsql:") {
      return new URL(`https://${url.host}/v2/pipeline`);
    }

    if (url.protocol === "https:" || url.protocol === "http:") {
      return new URL("/v2/pipeline", url.origin);
    }

    return null;
  } catch {
    return null;
  }
}

function crudStatements(context: TursoContext): readonly LibsqlStatement[] {
  const table = quoteIdentifier(context.tableName);

  return [
    {
      sql: `CREATE TABLE IF NOT EXISTS ${table} (
        id TEXT PRIMARY KEY,
        title TEXT NOT NULL,
        counter INTEGER NOT NULL,
        updated_at TEXT NOT NULL
      )`,
    },
    {
      sql: `INSERT INTO ${table} (id, title, counter, updated_at) VALUES (?, ?, ?, datetime('now'))`,
      args: [textArg(context.rowId), textArg("created-from-dx-www"), integerArg(1)],
    },
    {
      sql: `SELECT id, title, counter FROM ${table} WHERE id = ?`,
      args: [textArg(context.rowId)],
    },
    {
      sql: `UPDATE ${table} SET title = ?, counter = ?, updated_at = datetime('now') WHERE id = ?`,
      args: [textArg("updated-from-dx-www"), integerArg(2), textArg(context.rowId)],
    },
    {
      sql: `SELECT id, title, counter FROM ${table} WHERE id = ?`,
      args: [textArg(context.rowId)],
    },
    {
      sql: `DELETE FROM ${table} WHERE id = ?`,
      args: [textArg(context.rowId)],
    },
    {
      sql: `DROP TABLE IF EXISTS ${table}`,
    },
  ];
}

function textArg(value: string): LibsqlValue {
  return { type: "text", value };
}

function integerArg(value: number): LibsqlValue {
  return { type: "integer", value: String(value) };
}

async function runTursoPipeline(
  fetchImpl: WorldConnectionFetch,
  context: TursoContext,
  statements: readonly LibsqlStatement[],
): Promise<{ ok: boolean; results: readonly LibsqlPipelineResult[] }> {
  try {
    const requests: LibsqlPipelineRequest[] = [
      ...statements.map((stmt) => ({ type: "execute" as const, stmt })),
      { type: "close" },
    ];
    const response = await fetchImpl(context.endpoint, {
      method: "POST",
      headers: {
        Authorization: `Bearer ${context.authToken}`,
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ requests }),
    });
    const body = await response.json().catch(() => undefined);
    const results = Array.isArray(body?.results) ? body.results : [];

    return {
      ok: response.ok,
      results,
    };
  } catch (error) {
    return {
      ok: false,
      results: [
        {
          type: "error",
          error: { message: error instanceof Error ? error.name : "unknown-error" },
        },
      ],
    };
  }
}

function stepFromPipelineResult(
  name: TursoCrudStepName,
  context: TursoContext,
  result: LibsqlPipelineResult | undefined,
): TursoCrudStep {
  const executeResult = result?.response?.result;
  const ok = result?.type === "ok";

  return {
    name,
    ok,
    endpoint: readableEndpointLabel(context.endpoint.toString()),
    evidence: ok ? `${name}-ok` : result?.error?.message ?? "missing-pipeline-result",
    affectedRowCount: executeResult?.affected_row_count,
    rowsRead: executeResult?.rows_read,
    rowsWritten: executeResult?.rows_written,
  };
}

function tursoReceipt(input: {
  context: TursoContext;
  status: TursoCrudState;
  liveProviderExecution: boolean;
  steps: readonly TursoCrudStep[];
  nextAction: string;
}): TursoDatabaseCrudReceipt {
  return {
    schema: "dx.examples.world.turso-database-crud",
    providerId: "turso-libsql",
    packageId: "database/turso-libsql",
    category: "Turso/libSQL",
    status: input.status,
    redaction: "secret-values-never-included",
    checkedAt: input.context.checkedAt,
    databaseHost: input.context.databaseHost,
    tableName: input.context.tableName,
    requiredEnv: tursoCrudRequiredEnv,
    optionalEnv: tursoCrudOptionalEnv,
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
    throw new Error("Turso CRUD table name must be a simple SQL identifier.");
  }

  return `"${identifier}"`;
}
