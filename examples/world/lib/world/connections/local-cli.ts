import type { WorldConnectionContext, WorldConnectionEnvStatus, WorldConnectionResult } from "./contracts";
import { connectionResult } from "./http";

declare const process:
  | {
      env?: Record<string, string | undefined>;
    }
  | undefined;

type BunSpawnResult = {
  exitCode: number;
  stdout: Uint8Array;
  stderr: Uint8Array;
};

type BunRuntime = {
  spawnSync: (
    command: readonly string[],
    options?: {
      stdout?: "pipe";
      stderr?: "pipe";
      env?: Record<string, string | undefined>;
    },
  ) => BunSpawnResult;
};

const bundledNode = "C:\\Users\\Computer\\.cache\\codex-runtimes\\codex-primary-runtime\\dependencies\\node\\bin\\node.exe";
const vercelCliJs =
  "G:\\Dev\\Tools\\Scoop\\persist\\bun\\install\\global\\node_modules\\vercel\\dist\\vc.js";
const tursoShellExe =
  "G:\\Dev\\Tools\\Scoop\\persist\\bun\\install\\global\\node_modules\\@tursodatabase\\cli-win32-x64-msvc\\tursodb.exe";

function bunRuntime(): BunRuntime | undefined {
  return (globalThis as { Bun?: BunRuntime }).Bun;
}
function decode(output: Uint8Array): string {
  return new TextDecoder().decode(output).trim();
}

function runCommand(command: readonly string[]): { exitCode: number; stdout: string; stderr: string } | undefined {
  const bun = bunRuntime();

  if (!bun) {
    return undefined;
  }

  const result = bun.spawnSync(command, {
    stdout: "pipe",
    stderr: "pipe",
    env: process?.env,
  });

  return {
    exitCode: result.exitCode,
    stdout: decode(result.stdout),
    stderr: decode(result.stderr),
  };
}

export async function runVercelCliProbe(
  context: WorldConnectionContext,
  envStatus: WorldConnectionEnvStatus,
): Promise<WorldConnectionResult> {
  const probe = {
    id: "vercel-cli-whoami",
    providerId: "vercel",
    packageId: "vercel",
    name: "Vercel CLI",
    category: "Deployment",
    endpoint: "local-cli:vercel whoami",
    documentationUrl: "https://vercel.com/docs/cli",
    kind: "cli" as const,
  };

  if (!context.includeCli) {
    return connectionResult({
      probe,
      context,
      envStatus,
      state: "configured-readiness",
      ok: false,
      durationMs: 0,
      evidence: "cli-disabled",
      message: "Local CLI probing is disabled for this run.",
    });
  }

  const startedAt = Date.now();
  const result = runCommand([bundledNode, vercelCliJs, "whoami"]);

  if (!result) {
    return connectionResult({
      probe,
      context,
      envStatus,
      state: "blocked",
      ok: false,
      durationMs: Date.now() - startedAt,
      evidence: "bun-runtime-unavailable",
      message: "The TypeScript runner needs Bun to execute local CLI probes.",
    });
  }

  if (result.exitCode === 0 && result.stdout) {
    return connectionResult({
      probe,
      context,
      envStatus,
      state: "live-validated",
      ok: true,
      durationMs: Date.now() - startedAt,
      evidence: "authenticated-cli-user",
      message: `Vercel CLI authenticated as ${result.stdout.split(/\s+/)[0]}.`,
    });
  }

  return connectionResult({
    probe,
    context,
    envStatus,
    state: "blocked",
    ok: false,
    durationMs: Date.now() - startedAt,
    evidence: "cli-auth-unavailable",
    message: result.stderr || result.stdout || "Vercel CLI did not return an authenticated user.",
  });
}

export async function inspectTursoCli(
  context: WorldConnectionContext,
  envStatus: WorldConnectionEnvStatus,
): Promise<WorldConnectionResult> {
  const probe = {
    id: "turso-local-cli-shape",
    providerId: "turso-libsql",
    packageId: "turso",
    name: "Turso local CLI",
    category: "Database",
    endpoint: "local-cli:tursodb --help",
    documentationUrl: "https://docs.turso.tech/cli",
    kind: "cli" as const,
  };

  const startedAt = Date.now();
  const result = context.includeCli ? runCommand([tursoShellExe, "--help"]) : undefined;

  if (!context.includeCli) {
    return connectionResult({
      probe,
      context,
      envStatus,
      state: "configured-readiness",
      ok: false,
      durationMs: 0,
      evidence: "cli-disabled",
      message: "Local CLI probing is disabled for this run.",
    });
  }

  if (!result) {
    return connectionResult({
      probe,
      context,
      envStatus,
      state: "blocked",
      ok: false,
      durationMs: Date.now() - startedAt,
      evidence: "bun-runtime-unavailable",
      message: "The TypeScript runner needs Bun to inspect local CLI tools.",
    });
  }

  const shellOnly = /interactive SQL shell/i.test(`${result.stdout}\n${result.stderr}`);

  return connectionResult({
    probe,
    context,
    envStatus,
    state: shellOnly ? "blocked" : "configured-readiness",
    ok: !shellOnly,
    durationMs: Date.now() - startedAt,
    evidence: shellOnly ? "installed-cli-is-sql-shell" : "cli-shape-available",
    message: shellOnly
      ? "Installed Turso binary is the SQL shell, not the account-management CLI; use env-based SQL-over-HTTP for live proof."
      : "A Turso CLI shape was detected.",
  });
}
