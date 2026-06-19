export type WorldConnectionState =
  | "preview-only"
  | "missing-config"
  | "configured-readiness"
  | "live-validated"
  | "blocked";

export type WorldConnectionKind = "http" | "cli" | "env";

export type WorldConnectionFetch = (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>;

export type WorldConnectionContext = {
  env: Record<string, string | undefined>;
  allowNetwork: boolean;
  includeCli: boolean;
  timeoutMs: number;
  checkedAt: string;
  fetch: WorldConnectionFetch;
};

export type WorldConnectionEnvStatus = {
  requiredEnv: readonly string[];
  optionalEnv: readonly string[];
  presentEnv: readonly string[];
  missingEnv: readonly string[];
};

export type WorldConnectionResult = {
  schema: "dx.examples.world.connection-result";
  id: string;
  providerId: string;
  packageId: string;
  name: string;
  category: string;
  kind: WorldConnectionKind;
  state: WorldConnectionState;
  ok: boolean;
  readOnly: true;
  checkedAt: string;
  durationMs: number;
  endpoint: string;
  httpStatus?: number;
  requiredEnv: readonly string[];
  optionalEnv: readonly string[];
  presentEnv: readonly string[];
  missingEnv: readonly string[];
  documentationUrl?: string;
  evidence: string;
  message: string;
};

export type WorldConnectionProbe = {
  id: string;
  providerId: string;
  packageId: string;
  name: string;
  category: string;
  kind: WorldConnectionKind;
  endpoint: string;
  documentationUrl?: string;
  requiredEnv: readonly string[];
  optionalEnv?: readonly string[];
  run: (
    context: WorldConnectionContext,
    envStatus: WorldConnectionEnvStatus,
  ) => Promise<WorldConnectionResult>;
};

export type WorldConnectionReceipt = {
  schema: "dx.examples.world.live-connections";
  generatedBy: "examples/world";
  redaction: "secret-values-never-included";
  checkedAt: string;
  runner: string;
  receiptPath: string;
  totals: {
    probes: number;
    liveValidated: number;
    configuredReadiness: number;
    missingConfig: number;
    blocked: number;
    previewOnly: number;
  };
  results: readonly WorldConnectionResult[];
};
