export type DataIdentityConnectionStatus =
  | "missing-config"
  | "configured-readiness"
  | "live-validated"
  | "blocked";

export type DataIdentityProviderKind = "database" | "auth";

export type DataIdentityProbeKind =
  | "configured-readiness"
  | "missing-env"
  | "safe-http-readiness"
  | "turso-libsql-http-select-1"
  | "local-status-endpoint";

export type DataIdentityFetchInit = {
  method?: "GET" | "POST";
  headers?: Record<string, string>;
  body?: string;
};

export type DataIdentityFetchResponse = {
  ok: boolean;
  status: number;
  json?: () => Promise<unknown>;
  text?: () => Promise<string>;
};

export type DataIdentityFetch = (
  url: string,
  init: DataIdentityFetchInit,
) => Promise<DataIdentityFetchResponse>;

export type DataIdentityProbeContext = {
  env?: Record<string, string | undefined>;
  fetch?: DataIdentityFetch;
  now?: () => Date;
};

export type DataIdentityProviderDefinition<ProviderId extends string = string> = {
  id: ProviderId;
  name: string;
  kind: DataIdentityProviderKind;
  category: string;
  requiredEnv: readonly string[];
  optionalEnv: readonly string[];
  receiptSchemas: readonly string[];
  appOwnedBoundary: string;
  statusEndpointEnv?: string;
};

export type DataIdentityProbe = {
  kind: DataIdentityProbeKind;
  checkedAt: string;
  live: boolean;
  endpointEnv?: string;
  endpointKind?: "local" | "safe-http" | "not-probed";
  message: string;
};

export type DataIdentityConnectionResult<ProviderId extends string = string> = {
  schema: "dx.examples.world.provider-connection";
  providerId: ProviderId;
  providerName: string;
  kind: DataIdentityProviderKind;
  category: string;
  status: DataIdentityConnectionStatus;
  requiredEnv: readonly string[];
  optionalEnv: readonly string[];
  presentEnv: readonly string[];
  missingEnv: readonly string[];
  receiptSchemas: readonly string[];
  appOwnedBoundary: string;
  redaction: "secret-values-never-included";
  probe: DataIdentityProbe;
  blockers: readonly string[];
  nextAction: string;
};
