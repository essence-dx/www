import type {
  WorldConnectionContext,
  WorldConnectionEnvStatus,
  WorldConnectionKind,
  WorldConnectionResult,
} from "./contracts";
import { readableEndpointLabel } from "./redaction";

type JsonBody = Record<string, unknown> | readonly unknown[];

type ReadOnlyHttpProbe = {
  id: string;
  providerId: string;
  packageId: string;
  name: string;
  category: string;
  endpoint: string;
  documentationUrl?: string;
  method?: "GET" | "POST";
  headers?: Record<string, string>;
  body?: JsonBody;
  expectedStatuses?: readonly number[];
  evidence: string;
};

export function connectionResult(input: {
  probe: Pick<
    ReadOnlyHttpProbe,
    "id" | "providerId" | "packageId" | "name" | "category" | "endpoint" | "documentationUrl"
  > & { kind?: WorldConnectionKind };
  context: WorldConnectionContext;
  envStatus: WorldConnectionEnvStatus;
  state: WorldConnectionResult["state"];
  ok: boolean;
  durationMs: number;
  evidence: string;
  message: string;
  httpStatus?: number;
}): WorldConnectionResult {
  return {
    schema: "dx.examples.world.connection-result",
    id: input.probe.id,
    providerId: input.probe.providerId,
    packageId: input.probe.packageId,
    name: input.probe.name,
    category: input.probe.category,
    kind: input.probe.kind ?? "http",
    state: input.state,
    ok: input.ok,
    readOnly: true,
    checkedAt: input.context.checkedAt,
    durationMs: input.durationMs,
    endpoint: readableEndpointLabel(input.probe.endpoint),
    httpStatus: input.httpStatus,
    requiredEnv: input.envStatus.requiredEnv,
    optionalEnv: input.envStatus.optionalEnv,
    presentEnv: input.envStatus.presentEnv,
    missingEnv: input.envStatus.missingEnv,
    documentationUrl: input.probe.documentationUrl,
    evidence: input.evidence,
    message: input.message,
  };
}

export async function runReadOnlyHttpProbe(
  context: WorldConnectionContext,
  envStatus: WorldConnectionEnvStatus,
  probe: ReadOnlyHttpProbe,
): Promise<WorldConnectionResult> {
  if (!context.allowNetwork) {
    return connectionResult({
      probe,
      context,
      envStatus,
      state: "configured-readiness",
      ok: false,
      durationMs: 0,
      evidence: "network-disabled",
      message: "Required env is present, but live network probing is disabled for this run.",
    });
  }

  const startedAt = Date.now();
  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), context.timeoutMs);
  const method = probe.method ?? "GET";

  try {
    const response = await context.fetch(probe.endpoint, {
      method,
      headers: probe.headers,
      body: probe.body ? JSON.stringify(probe.body) : undefined,
      signal: controller.signal,
    });
    const expectedStatuses = probe.expectedStatuses ?? [200];
    const ok = expectedStatuses.includes(response.status);

    return connectionResult({
      probe,
      context,
      envStatus,
      state: ok ? "live-validated" : "blocked",
      ok,
      durationMs: Date.now() - startedAt,
      httpStatus: response.status,
      evidence: ok ? probe.evidence : `http-status-${response.status}`,
      message: ok
        ? "Read-only provider probe completed with expected status."
        : "Provider responded, but not with the expected read-only validation status.",
    });
  } catch (error) {
    const reason = error instanceof Error ? error.name : "unknown-error";

    return connectionResult({
      probe,
      context,
      envStatus,
      state: "blocked",
      ok: false,
      durationMs: Date.now() - startedAt,
      evidence: reason,
      message: "Read-only provider probe could not complete.",
    });
  } finally {
    clearTimeout(timeout);
  }
}
