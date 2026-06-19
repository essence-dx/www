export type AiRealtimeWorkflowProviderId =
  | "openai"
  | "anthropic"
  | "google-gemini"
  | "ably"
  | "pusher"
  | "supabase-realtime"
  | "upstash-qstash"
  | "temporal"
  | "cloudflare-queues";

export type AiRealtimeWorkflowProviderCategory = "ai" | "realtime" | "workflows";

export type AiRealtimeWorkflowProviderStatus =
  | "missing-config"
  | "configured-readiness"
  | "live-validated"
  | "provider-error"
  | "blocked";

export type AiRealtimeWorkflowFetch = (
  input: string,
  init?: RequestInit,
) => Promise<Response>;

export type AiRealtimeWorkflowProbeContext = {
  env?: Record<string, string | undefined>;
  fetch?: AiRealtimeWorkflowFetch;
  now?: () => Date;
};

export type AiRealtimeWorkflowProviderDefinition = {
  providerId: AiRealtimeWorkflowProviderId;
  providerName: string;
  packageId: string;
  category: AiRealtimeWorkflowProviderCategory;
  requiredEnv: readonly string[];
  optionalEnv: readonly string[];
  endpointTemplate: string;
  safeReadOnlyEndpoint: boolean;
  method: "GET";
  receiptSchema: string;
  documentationUrl: string;
  appOwnedBoundary: string;
  unavailableReason?: string;
};

export type AiRealtimeWorkflowProviderProbeResult = {
  schema: "dx.examples.world.ai_realtime_workflow_provider_probe";
  providerId: AiRealtimeWorkflowProviderId;
  providerName: string;
  packageId: string;
  category: AiRealtimeWorkflowProviderCategory;
  status: AiRealtimeWorkflowProviderStatus;
  operation: "read-only" | "not-probed";
  method: "GET" | null;
  endpoint: string;
  redactedEndpoint: string;
  safeReadOnlyEndpoint: boolean;
  liveReadOnlyRequest: boolean;
  liveProviderExecution: boolean;
  httpStatus?: number;
  requiredEnv: readonly string[];
  optionalEnv: readonly string[];
  presentEnv: readonly string[];
  missingEnv: readonly string[];
  headerNames: readonly string[];
  receiptSchema: string;
  redaction: "secret-values-never-included";
  secretValues: [];
  blockers: readonly string[];
  nextAction: string;
  checkedAt: string;
  documentationUrl: string;
};

type ReadOnlyRequest = {
  url: string;
  redactedUrl: string;
  init: RequestInit;
  headerNames: readonly string[];
};

type RequestBuilder = (
  definition: AiRealtimeWorkflowProviderDefinition,
  env: Record<string, string | undefined>,
  now: Date,
) => Promise<ReadOnlyRequest>;

const providerRequestBuilders: Record<AiRealtimeWorkflowProviderId, RequestBuilder | undefined> = {
  openai: async (_definition, env) => ({
    url: "https://api.openai.com/v1/models",
    redactedUrl: "https://api.openai.com/v1/models",
    init: {
      method: "GET",
      headers: {
        Authorization: `Bearer ${requiredEnvValue("OPENAI_API_KEY", env)}`,
        "Cache-Control": "no-store",
      },
    },
    headerNames: ["Authorization", "Cache-Control"],
  }),
  anthropic: async (_definition, env) => ({
    url: "https://api.anthropic.com/v1/models",
    redactedUrl: "https://api.anthropic.com/v1/models",
    init: {
      method: "GET",
      headers: {
        "x-api-key": requiredEnvValue("ANTHROPIC_API_KEY", env),
        "anthropic-version": "2023-06-01",
        "Cache-Control": "no-store",
      },
    },
    headerNames: ["x-api-key", "anthropic-version", "Cache-Control"],
  }),
  "google-gemini": async (_definition, env) => {
    const key = requiredEnvValue("GOOGLE_GENERATIVE_AI_API_KEY", env);

    return {
      url: `https://generativelanguage.googleapis.com/v1beta/models?key=${encodeURIComponent(key)}`,
      redactedUrl: "https://generativelanguage.googleapis.com/v1beta/models?key=REDACTED",
      init: {
        method: "GET",
        headers: {
          "Cache-Control": "no-store",
        },
      },
      headerNames: ["Cache-Control"],
    };
  },
  ably: async (_definition, env) => ({
    url: "https://rest.ably.io/channels",
    redactedUrl: "https://rest.ably.io/channels",
    init: {
      method: "GET",
      headers: {
        Authorization: `Basic ${encodeBase64Ascii(requiredEnvValue("ABLY_API_KEY", env))}`,
        "Cache-Control": "no-store",
      },
    },
    headerNames: ["Authorization", "Cache-Control"],
  }),
  pusher: async (_definition, env, now) => {
    const appId = requiredEnvValue("PUSHER_APP_ID", env);
    const key = requiredEnvValue("PUSHER_KEY", env);
    const secret = requiredEnvValue("PUSHER_SECRET", env);
    const cluster = requiredEnvValue("PUSHER_CLUSTER", env);
    const path = `/apps/${encodeURIComponent(appId)}/channels`;
    const timestamp = Math.floor(now.getTime() / 1000).toString();
    const signedQuery = `auth_key=${encodeURIComponent(key)}&auth_timestamp=${timestamp}&auth_version=1.0`;
    const stringToSign = `GET\n${path}\n${signedQuery}`;
    const signature = await hmacSha256Hex(secret, stringToSign);

    return {
      url: `https://api-${encodeURIComponent(cluster)}.pusher.com${path}?${signedQuery}&auth_signature=${signature}`,
      redactedUrl: `https://api-${encodeURIComponent(cluster)}.pusher.com${path}?auth_key=REDACTED&auth_timestamp=${timestamp}&auth_version=1.0&auth_signature=REDACTED`,
      init: {
        method: "GET",
        headers: {
          "Cache-Control": "no-store",
        },
      },
      headerNames: ["Cache-Control"],
    };
  },
  "supabase-realtime": undefined,
  "upstash-qstash": async (_definition, env) => ({
    url: "https://qstash.upstash.io/v2/schedules",
    redactedUrl: "https://qstash.upstash.io/v2/schedules",
    init: {
      method: "GET",
      headers: {
        Authorization: `Bearer ${requiredEnvValue("QSTASH_TOKEN", env)}`,
        "Cache-Control": "no-store",
      },
    },
    headerNames: ["Authorization", "Cache-Control"],
  }),
  temporal: undefined,
  "cloudflare-queues": async (_definition, env) => {
    const accountId = requiredEnvValue("CLOUDFLARE_ACCOUNT_ID", env);

    return {
      url: `https://api.cloudflare.com/client/v4/accounts/${encodeURIComponent(accountId)}/queues`,
      redactedUrl: "https://api.cloudflare.com/client/v4/accounts/REDACTED/queues",
      init: {
        method: "GET",
        headers: {
          Authorization: `Bearer ${requiredEnvValue("CLOUDFLARE_API_TOKEN", env)}`,
          "Cache-Control": "no-store",
        },
      },
      headerNames: ["Authorization", "Cache-Control"],
    };
  },
};

export const aiRealtimeWorkflowProviderDefinitions = [
  {
    providerId: "openai",
    providerName: "OpenAI",
    packageId: "ai/openai",
    category: "ai",
    requiredEnv: ["OPENAI_API_KEY"],
    optionalEnv: ["OPENAI_ORG_ID", "OPENAI_PROJECT_ID"],
    endpointTemplate: "https://api.openai.com/v1/models",
    safeReadOnlyEndpoint: true,
    method: "GET",
    receiptSchema: "dx.forge.world.ai",
    documentationUrl: "https://platform.openai.com/docs/api-reference/models/list",
    appOwnedBoundary:
      "Model generation, tool execution, token spend, and prompt storage stay app-owned; this probe only lists model access.",
  },
  {
    providerId: "anthropic",
    providerName: "Anthropic",
    packageId: "ai/anthropic",
    category: "ai",
    requiredEnv: ["ANTHROPIC_API_KEY"],
    optionalEnv: ["ANTHROPIC_VERSION"],
    endpointTemplate: "https://api.anthropic.com/v1/models",
    safeReadOnlyEndpoint: true,
    method: "GET",
    receiptSchema: "dx.forge.world.ai",
    documentationUrl: "https://docs.anthropic.com/en/api/models-list",
    appOwnedBoundary:
      "Message creation, tool use, prompt caching, and model spend stay app-owned; this probe only lists model access.",
  },
  {
    providerId: "google-gemini",
    providerName: "Google Gemini",
    packageId: "ai/google-gemini",
    category: "ai",
    requiredEnv: ["GOOGLE_GENERATIVE_AI_API_KEY"],
    optionalEnv: ["GOOGLE_GENERATIVE_AI_API_VERSION"],
    endpointTemplate: "https://generativelanguage.googleapis.com/v1beta/models",
    safeReadOnlyEndpoint: true,
    method: "GET",
    receiptSchema: "dx.forge.world.ai",
    documentationUrl: "https://ai.google.dev/api/models",
    appOwnedBoundary:
      "Content generation, multimodal uploads, and safety policy stay app-owned; this probe only lists model access.",
  },
  {
    providerId: "ably",
    providerName: "Ably",
    packageId: "realtime/ably",
    category: "realtime",
    requiredEnv: ["ABLY_API_KEY"],
    optionalEnv: ["ABLY_CLIENT_ID"],
    endpointTemplate: "https://rest.ably.io/channels",
    safeReadOnlyEndpoint: true,
    method: "GET",
    receiptSchema: "dx.forge.world.realtime",
    documentationUrl: "https://ably.com/docs/api/rest-api",
    appOwnedBoundary:
      "Token issuance, channel authorization, presence membership, and message publishing stay app-owned; this probe lists channels only.",
  },
  {
    providerId: "pusher",
    providerName: "Pusher",
    packageId: "realtime/pusher",
    category: "realtime",
    requiredEnv: ["PUSHER_APP_ID", "PUSHER_KEY", "PUSHER_SECRET", "PUSHER_CLUSTER"],
    optionalEnv: ["PUSHER_HOST"],
    endpointTemplate: "https://api-{cluster}.pusher.com/apps/{app_id}/channels",
    safeReadOnlyEndpoint: true,
    method: "GET",
    receiptSchema: "dx.forge.world.realtime",
    documentationUrl: "https://pusher.com/docs/channels/library_auth_reference/rest-api/",
    appOwnedBoundary:
      "Private-channel auth, presence user data, and event publishing stay app-owned; this probe signs a read-only channel list request.",
  },
  {
    providerId: "supabase-realtime",
    providerName: "Supabase Realtime",
    packageId: "realtime/supabase-realtime",
    category: "realtime",
    requiredEnv: ["SUPABASE_URL", "SUPABASE_SERVICE_ROLE_KEY"],
    optionalEnv: ["NEXT_PUBLIC_SUPABASE_ANON_KEY", "SUPABASE_REALTIME_CHANNEL"],
    endpointTemplate: "wss://{project-ref}.supabase.co/realtime/v1/websocket",
    safeReadOnlyEndpoint: false,
    method: "GET",
    receiptSchema: "dx.forge.world.realtime",
    documentationUrl: "https://supabase.com/docs/guides/realtime",
    appOwnedBoundary:
      "Broadcast, presence, database change subscriptions, and private-channel policy stay app-owned; WWW only records the realtime boundary until a WebSocket receipt is imported.",
    unavailableReason:
      "no-safe-read-only-http-endpoint: Supabase Realtime validation needs WebSocket subscription proof or an imported provider receipt.",
  },
  {
    providerId: "upstash-qstash",
    providerName: "Upstash QStash",
    packageId: "workflows/upstash-qstash",
    category: "workflows",
    requiredEnv: ["QSTASH_TOKEN"],
    optionalEnv: ["QSTASH_CURRENT_SIGNING_KEY", "QSTASH_NEXT_SIGNING_KEY"],
    endpointTemplate: "https://qstash.upstash.io/v2/schedules",
    safeReadOnlyEndpoint: true,
    method: "GET",
    receiptSchema: "dx.forge.world.queue",
    documentationUrl: "https://upstash.com/docs/qstash/api-refence/schedules/list-schedules",
    appOwnedBoundary:
      "Publishing, enqueueing, signature verification, and webhook delivery stay app-owned; this probe lists schedules only.",
  },
  {
    providerId: "temporal",
    providerName: "Temporal",
    packageId: "workflows/temporal",
    category: "workflows",
    requiredEnv: ["TEMPORAL_ADDRESS", "TEMPORAL_NAMESPACE"],
    optionalEnv: ["TEMPORAL_API_KEY", "TEMPORAL_TASK_QUEUE"],
    endpointTemplate: "https://{namespace-id}.tmprl.cloud:7233",
    safeReadOnlyEndpoint: false,
    method: "GET",
    receiptSchema: "dx.forge.world.workflow",
    documentationUrl: "https://docs.temporal.io/",
    appOwnedBoundary:
      "Worker startup, workflow start, activity execution, cancellation, and replay stay app-owned; WWW records readiness until a Temporal SDK or Cloud Ops receipt is imported.",
    unavailableReason:
      "no-safe-read-only-http-endpoint: Temporal validation needs SDK/gRPC DescribeNamespace/ListNamespaces proof or an imported provider receipt.",
  },
  {
    providerId: "cloudflare-queues",
    providerName: "Cloudflare Queues",
    packageId: "workflows/cloudflare-queues",
    category: "workflows",
    requiredEnv: ["CLOUDFLARE_ACCOUNT_ID", "CLOUDFLARE_API_TOKEN"],
    optionalEnv: ["CLOUDFLARE_QUEUE_ID", "CLOUDFLARE_QUEUE_NAME"],
    endpointTemplate: "https://api.cloudflare.com/client/v4/accounts/{account_id}/queues",
    safeReadOnlyEndpoint: true,
    method: "GET",
    receiptSchema: "dx.forge.world.queue",
    documentationUrl: "https://developers.cloudflare.com/api/resources/queues/",
    appOwnedBoundary:
      "Queue creation, message push, pull, ack, retry, and consumers stay app-owned; this probe lists queues only.",
  },
] as const satisfies readonly AiRealtimeWorkflowProviderDefinition[];

export function findAiRealtimeWorkflowProvider(
  providerId: string | null | undefined,
): AiRealtimeWorkflowProviderDefinition | undefined {
  return aiRealtimeWorkflowProviderDefinitions.find(
    (provider) => provider.providerId === providerId,
  );
}

export async function probeAiRealtimeWorkflowProvider(
  providerId: AiRealtimeWorkflowProviderId,
  context: AiRealtimeWorkflowProbeContext = {},
): Promise<AiRealtimeWorkflowProviderProbeResult> {
  const definition = findAiRealtimeWorkflowProvider(providerId);

  if (!definition) {
    throw new Error(`Unknown AI realtime workflow provider: ${providerId}`);
  }

  const env = context.env ?? {};
  const checkedAt = (context.now ?? (() => new Date()))().toISOString();
  const presentEnv = presentEnvNames(definition, env);
  const missingEnv = missingEnvNames(definition, env);

  if (missingEnv.length > 0) {
    return buildResult(definition, {
      checkedAt,
      status: "missing-config",
      operation: "not-probed",
      endpoint: definition.endpointTemplate,
      redactedEndpoint: definition.endpointTemplate,
      liveReadOnlyRequest: false,
      liveProviderExecution: false,
      presentEnv,
      missingEnv,
      headerNames: [],
      blockers: [`missing-env:${missingEnv.join(",")}`],
      nextAction: `Provide missing Env Firewall keys before probing ${definition.providerName}: ${missingEnv.join(", ")}.`,
    });
  }

  if (!definition.safeReadOnlyEndpoint) {
    return buildResult(definition, {
      checkedAt,
      status: "configured-readiness",
      operation: "not-probed",
      endpoint: definition.endpointTemplate,
      redactedEndpoint: definition.endpointTemplate,
      liveReadOnlyRequest: false,
      liveProviderExecution: false,
      presentEnv,
      missingEnv,
      headerNames: [],
      blockers: [definition.unavailableReason ?? "no-safe-read-only-http-endpoint"],
      nextAction:
        "Import a redacted provider receipt from the app-owned runtime before marking this provider live-validated.",
    });
  }

  if (!context.fetch) {
    return buildResult(definition, {
      checkedAt,
      status: "configured-readiness",
      operation: "not-probed",
      endpoint: definition.endpointTemplate,
      redactedEndpoint: definition.endpointTemplate,
      liveReadOnlyRequest: false,
      liveProviderExecution: false,
      presentEnv,
      missingEnv,
      headerNames: [],
      blockers: ["fetch-not-provided"],
      nextAction:
        "Run this adapter from a server route with an explicit read-only fetch capability, then import the redacted receipt.",
    });
  }

  const builder = providerRequestBuilders[definition.providerId];

  if (!builder) {
    return buildResult(definition, {
      checkedAt,
      status: "blocked",
      operation: "not-probed",
      endpoint: definition.endpointTemplate,
      redactedEndpoint: definition.endpointTemplate,
      liveReadOnlyRequest: false,
      liveProviderExecution: false,
      presentEnv,
      missingEnv,
      headerNames: [],
      blockers: ["request-builder-unavailable"],
      nextAction:
        "Add a provider-specific read-only request builder before attempting live validation.",
    });
  }

  try {
    const request = await builder(definition, env, new Date(checkedAt));
    const response = await context.fetch(request.url, request.init);
    const status = response.ok ? "live-validated" : "provider-error";

    return buildResult(definition, {
      checkedAt,
      status,
      operation: "read-only",
      endpoint: request.redactedUrl,
      redactedEndpoint: request.redactedUrl,
      liveReadOnlyRequest: true,
      liveProviderExecution: true,
      httpStatus: response.status,
      presentEnv,
      missingEnv,
      headerNames: request.headerNames,
      blockers: response.ok ? [] : [`provider-http-status:${response.status}`],
      nextAction: response.ok
        ? "Import this redacted read-only provider receipt before promoting framework live proof."
        : "Keep the provider gated and review the redacted provider error before retrying.",
    });
  } catch (error) {
    return buildResult(definition, {
      checkedAt,
      status: "blocked",
      operation: "not-probed",
      endpoint: definition.endpointTemplate,
      redactedEndpoint: definition.endpointTemplate,
      liveReadOnlyRequest: false,
      liveProviderExecution: false,
      presentEnv,
      missingEnv,
      headerNames: [],
      blockers: [error instanceof Error ? error.message : "provider-request-build-failed"],
      nextAction:
        "Keep the provider gated until the read-only request can be constructed without exposing secrets.",
    });
  }
}

function buildResult(
  definition: AiRealtimeWorkflowProviderDefinition,
  input: Omit<
    AiRealtimeWorkflowProviderProbeResult,
    | "schema"
    | "providerId"
    | "providerName"
    | "packageId"
    | "category"
    | "method"
    | "safeReadOnlyEndpoint"
    | "requiredEnv"
    | "optionalEnv"
    | "receiptSchema"
    | "redaction"
    | "secretValues"
    | "documentationUrl"
  >,
): AiRealtimeWorkflowProviderProbeResult {
  return {
    schema: "dx.examples.world.ai_realtime_workflow_provider_probe",
    providerId: definition.providerId,
    providerName: definition.providerName,
    packageId: definition.packageId,
    category: definition.category,
    method: input.operation === "read-only" ? definition.method : null,
    safeReadOnlyEndpoint: definition.safeReadOnlyEndpoint,
    requiredEnv: definition.requiredEnv,
    optionalEnv: definition.optionalEnv,
    receiptSchema: definition.receiptSchema,
    redaction: "secret-values-never-included",
    secretValues: [],
    documentationUrl: definition.documentationUrl,
    ...input,
  };
}

function requiredEnvValue(name: string, env: Record<string, string | undefined>): string {
  const value = env[name];

  if (!value) {
    throw new Error(`missing-env:${name}`);
  }

  return value;
}

function presentEnvNames(
  definition: AiRealtimeWorkflowProviderDefinition,
  env: Record<string, string | undefined>,
): readonly string[] {
  return [...definition.requiredEnv, ...definition.optionalEnv].filter((name) => Boolean(env[name]));
}

function missingEnvNames(
  definition: AiRealtimeWorkflowProviderDefinition,
  env: Record<string, string | undefined>,
): readonly string[] {
  return definition.requiredEnv.filter((name) => !env[name]);
}

function encodeBase64Ascii(value: string): string {
  const alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
  let output = "";

  for (let index = 0; index < value.length; index += 3) {
    const first = value.charCodeAt(index);
    const second = index + 1 < value.length ? value.charCodeAt(index + 1) : 0;
    const third = index + 2 < value.length ? value.charCodeAt(index + 2) : 0;
    const bits = (first << 16) | (second << 8) | third;

    output += alphabet[(bits >> 18) & 63];
    output += alphabet[(bits >> 12) & 63];
    output += index + 1 < value.length ? alphabet[(bits >> 6) & 63] : "=";
    output += index + 2 < value.length ? alphabet[bits & 63] : "=";
  }

  return output;
}

async function hmacSha256Hex(secret: string, message: string): Promise<string> {
  if (!globalThis.crypto?.subtle) {
    throw new Error("web-crypto-unavailable-for-pusher-signature");
  }

  const encoder = new TextEncoder();
  const key = await globalThis.crypto.subtle.importKey(
    "raw",
    encoder.encode(secret),
    { name: "HMAC", hash: "SHA-256" },
    false,
    ["sign"],
  );
  const signature = await globalThis.crypto.subtle.sign("HMAC", key, encoder.encode(message));

  return Array.from(new Uint8Array(signature))
    .map((byte) => byte.toString(16).padStart(2, "0"))
    .join("");
}
