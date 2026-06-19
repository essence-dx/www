export type WorldConnectionProviderId =
  | "stripe"
  | "lemon-squeezy"
  | "paddle"
  | "aws-s3"
  | "cloudflare-r2"
  | "vercel-blob"
  | "algolia"
  | "meilisearch"
  | "typesense"
  | "pinecone"
  | "vercel"
  | "cloudflare"
  | "fly-io"
  | "netlify"
  | "sanity"
  | "strapi"
  | "contentful"
  | "resend"
  | "twilio"
  | "firebase-cloud-messaging";

export type WorldConnectionCategory =
  | "payments"
  | "storage"
  | "search"
  | "vector-search"
  | "deployment"
  | "content"
  | "notifications";

export type WorldConnectionMethod = "GET" | "HEAD";

export type WorldConnectionStatus =
  | "missing-config"
  | "configured-readiness"
  | "live-validated"
  | "provider-error"
  | "unknown-provider";

export type WorldConnectionEnv = Record<string, string | undefined>;

export type WorldFetch = (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>;

export type WorldReadinessRequest = {
  input: RequestInfo | URL;
  init: RequestInit;
  endpoint: string;
};

export type WorldProviderReadiness = {
  method: WorldConnectionMethod;
  endpointLabel: string;
  buildRequest?: (env: WorldConnectionEnv) => Promise<WorldReadinessRequest>;
  sdkOperation?: string;
};

export type WorldConnectionProvider = {
  id: WorldConnectionProviderId;
  categoryId: WorldConnectionCategory;
  name: string;
  packageId: string;
  requiredEnv: readonly string[];
  optionalEnv: readonly string[];
  receiptSchema: string;
  readiness: WorldProviderReadiness;
  secretRedaction: "secret-values-never-included";
  nextAction: string;
};

export type WorldConnectionCheckOptions = {
  env?: WorldConnectionEnv;
  fetch?: WorldFetch;
};

export type WorldConnectionCheckResult = {
  schema: "dx.examples.world.connection_provider_readiness";
  providerId: string;
  packageId?: string;
  categoryId?: WorldConnectionCategory;
  status: WorldConnectionStatus;
  method?: WorldConnectionMethod;
  endpoint?: string;
  requiredEnv: readonly string[];
  presentEnv: readonly string[];
  missingEnv: readonly string[];
  secretValues: readonly string[];
  liveProviderExecution: boolean;
  httpStatus?: number;
  receiptSchema?: string;
  redaction: "secret-values-never-included";
  nextAction: string;
};
