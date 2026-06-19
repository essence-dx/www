import type { WorldConnectionProbe } from "../contracts";
import { connectionResult, runReadOnlyHttpProbe } from "../http";

type ProviderProbeBase = Omit<WorldConnectionProbe, "run">;

export function configuredReadinessProbe(base: ProviderProbeBase, message: string): WorldConnectionProbe {
  return {
    ...base,
    run: async (context, envStatus) =>
      connectionResult({
        probe: { ...base, kind: base.kind },
        context,
        envStatus,
        state: "configured-readiness",
        ok: true,
        durationMs: 0,
        evidence: "env-contract-satisfied",
        message,
      }),
  };
}
export function httpProbe(
  base: ProviderProbeBase,
  request: (env: Record<string, string | undefined>) => {
    endpoint: string;
    method?: "GET" | "POST";
    headers?: Record<string, string>;
    body?: Record<string, unknown> | readonly unknown[];
    expectedStatuses?: readonly number[];
    evidence: string;
  },
): WorldConnectionProbe {
  return {
    ...base,
    run: async (context, envStatus) => {
      const nextRequest = request(context.env);

      return runReadOnlyHttpProbe(context, envStatus, {
        ...base,
        ...nextRequest,
      });
    },
  };
}
