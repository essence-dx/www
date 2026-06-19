import type { WorldConnectionProbe } from "../contracts";
import { inspectTursoCli } from "../local-cli";
import { configuredReadinessProbe, httpProbe } from "./shared";

function tursoHttpEndpoint(value: string): string {
  if (value.startsWith("libsql://")) {
    return `https://${value.slice("libsql://".length)}/v2/pipeline`;
  }

  if (value.startsWith("http://") || value.startsWith("https://")) {
    const url = new URL(value);
    return `${url.origin}/v2/pipeline`;
  }

  return value;
}

export const tursoConnectionProbes: readonly WorldConnectionProbe[] = [
  configuredReadinessProbe(
    {
      id: "postgresql-env-readiness",
      providerId: "postgresql",
      packageId: "database/postgresql",
      name: "PostgreSQL",
      category: "Database",
      kind: "env",
      endpoint: "env:DATABASE_URL",
      requiredEnv: ["DATABASE_URL"],
      optionalEnv: [],
    },
    "DATABASE_URL is present; exact query proof should run through the selected DX ORM or Postgres driver adapter.",
  ),
  configuredReadinessProbe(
    {
      id: "neon-env-readiness",
      providerId: "neon",
      packageId: "database/neon",
      name: "Neon",
      category: "Database",
      kind: "env",
      endpoint: "env:NEON_DATABASE_URL",
      documentationUrl: "https://neon.tech/docs/connect/connect-from-any-app",
      requiredEnv: ["NEON_DATABASE_URL"],
      optionalEnv: ["NEON_API_KEY"],
    },
    "NEON_DATABASE_URL is present; branch-specific API validation should be promoted into a Forge provider adapter.",
  ),
  httpProbe(
    {
      id: "turso-sql-over-http-select-one",
      providerId: "turso-libsql",
      packageId: "database/turso-libsql",
      name: "Turso/libSQL SQL-over-HTTP",
      category: "Database",
      kind: "http",
      endpoint: "env:TURSO_DATABASE_URL/v2/pipeline",
      documentationUrl: "https://docs.turso.tech/sdk/http/reference",
      requiredEnv: ["TURSO_DATABASE_URL", "TURSO_AUTH_TOKEN"],
      optionalEnv: [],
    },
    (env) => ({
      endpoint: tursoHttpEndpoint(env.TURSO_DATABASE_URL ?? ""),
      method: "POST",
      headers: {
        Authorization: `Bearer ${env.TURSO_AUTH_TOKEN ?? ""}`,
        "Content-Type": "application/json",
      },
      body: {
        requests: [
          { type: "execute", stmt: { sql: "SELECT 1" } },
          { type: "close" },
        ],
      },
      expectedStatuses: [200],
      evidence: "select-one-sql-over-http",
    }),
  ),
  {
    id: "turso-local-cli-shape",
    providerId: "turso-libsql",
    packageId: "database/turso-libsql",
    name: "Turso local CLI",
    category: "Database",
    kind: "cli",
    endpoint: "local-cli:tursodb --help",
    documentationUrl: "https://docs.turso.tech/cli",
    requiredEnv: [],
    optionalEnv: [],
    run: inspectTursoCli,
  },
];
