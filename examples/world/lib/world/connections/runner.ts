import type {
  WorldConnectionContext,
  WorldConnectionFetch,
  WorldConnectionProbe,
  WorldConnectionReceipt,
  WorldConnectionResult,
} from "./contracts";
import { connectionResult } from "./http";
import { connectionEnvStatus, hasLeakedEnvValue } from "./redaction";
import { worldConnectionProbes } from "./providers";

declare const process:
  | {
      env?: Record<string, string | undefined>;
    }
  | undefined;

type BunRuntime = {
  write: (path: string, value: string) => Promise<unknown>;
};

export const worldConnectionReceiptPath = "examples/world/.dx/receipts/world/live-connections.json";

function buildContext(options: {
  env?: Record<string, string | undefined>;
  allowNetwork?: boolean;
  includeCli?: boolean;
  timeoutMs?: number;
  fetch?: WorldConnectionFetch;
}): WorldConnectionContext {
  return {
    env: options.env ?? process?.env ?? {},
    allowNetwork: options.allowNetwork ?? true,
    includeCli: options.includeCli ?? true,
    timeoutMs: options.timeoutMs ?? 3500,
    checkedAt: new Date().toISOString(),
    fetch: options.fetch ?? fetch,
  };
}

function missingConfigResult(
  context: WorldConnectionContext,
  probe: WorldConnectionProbe,
  envStatus: ReturnType<typeof connectionEnvStatus>,
): WorldConnectionResult {
  return connectionResult({
    probe: { ...probe, kind: probe.kind },
    context,
    envStatus,
    state: "missing-config",
    ok: false,
    durationMs: 0,
    evidence: "required-env-missing",
    message: "Required Env Firewall keys are missing; live provider validation was not attempted.",
  });
}

function assertNoSecretLeak(receipt: WorldConnectionReceipt, env: Record<string, string | undefined>): void {
  if (hasLeakedEnvValue(receipt, env)) {
    throw new Error("World connection receipt contained a raw env value; refusing to write it.");
  }
}

function receiptTotals(results: readonly WorldConnectionResult[]): WorldConnectionReceipt["totals"] {
  return {
    probes: results.length,
    liveValidated: results.filter((result) => result.state === "live-validated").length,
    configuredReadiness: results.filter((result) => result.state === "configured-readiness").length,
    missingConfig: results.filter((result) => result.state === "missing-config").length,
    blocked: results.filter((result) => result.state === "blocked").length,
    previewOnly: results.filter((result) => result.state === "preview-only").length,
  };
}

export async function runWorldConnectionProbes(options: {
  env?: Record<string, string | undefined>;
  allowNetwork?: boolean;
  includeCli?: boolean;
  timeoutMs?: number;
  fetch?: WorldConnectionFetch;
  probes?: readonly WorldConnectionProbe[];
} = {}): Promise<WorldConnectionReceipt> {
  const context = buildContext(options);
  const probes = options.probes ?? worldConnectionProbes;
  const results: WorldConnectionResult[] = [];

  for (const probe of probes) {
    const envStatus = connectionEnvStatus(probe.requiredEnv, probe.optionalEnv ?? [], context.env);

    if (envStatus.missingEnv.length > 0) {
      results.push(missingConfigResult(context, probe, envStatus));
      continue;
    }

    results.push(await probe.run(context, envStatus));
  }

  const receipt: WorldConnectionReceipt = {
    schema: "dx.examples.world.live-connections",
    generatedBy: "examples/world",
    redaction: "secret-values-never-included",
    checkedAt: context.checkedAt,
    runner: "examples/world/lib/world/connections/runner.ts",
    receiptPath: worldConnectionReceiptPath,
    totals: receiptTotals(results),
    results,
  };

  assertNoSecretLeak(receipt, context.env);

  return receipt;
}

export async function writeWorldConnectionReceipt(receipt: WorldConnectionReceipt): Promise<void> {
  const bun = (globalThis as { Bun?: BunRuntime }).Bun;

  if (!bun) {
    throw new Error("Writing the world connection receipt requires the Bun runtime.");
  }

  await bun.write(worldConnectionReceiptPath, `${JSON.stringify(receipt, null, 2)}\n`);
}
