import type {
  WorldConnectionContext,
  WorldConnectionEnvStatus,
  WorldConnectionProbe,
  WorldConnectionResult,
} from "../contracts";
import { connectionResult, runReadOnlyHttpProbe } from "../http";
import { connectionEnvStatus } from "../redaction";

declare const process:
  | {
      env?: Record<string, string | undefined>;
    }
  | undefined;

type OperationsProviderId =
  | "redis-valkey"
  | "upstash-redis"
  | "cloudflare-kv"
  | "posthog"
  | "plausible"
  | "vercel-analytics"
  | "sentry"
  | "datadog"
  | "opentelemetry";

type OperationsHttpRequest = {
  endpoint: string;
  headers?: Record<string, string>;
  expectedStatuses?: readonly number[];
  evidence: string;
};

type OperationsProbeBase = Omit<WorldConnectionProbe, "run"> & {
  operation: "read-only";
  method: "GET";
};

type OperationsProbeDefinition = OperationsProbeBase & {
  request?: (env: Record<string, string | undefined>) => OperationsHttpRequest | undefined;
  configuredMessage: string;
};

const defaultPostHogHost = "https://app.posthog.com";
const defaultPlausibleHost = "https://plausible.io";
const defaultDatadogSite = "datadoghq.com";

const operationsMissingConfigPolicy = {
  status: "missing-config",
  method: "GET",
  operation: "read-only",
} as const;

function joinUrl(base: string, path: string): string {
  return `${base.replace(/\/+$/, "")}/${path.replace(/^\/+/, "")}`;
}

function requiredUrlValue(env: Record<string, string | undefined>, name: string): string | undefined {
  const value = env[name];

  if (!value) {
    return undefined;
  }

  return value;
}

function bearer(value: string): Record<string, string> {
  return { Authorization: `Bearer ${value}` };
}

function configuredReadinessResult(
  probe: OperationsProbeDefinition,
  context: WorldConnectionContext,
  envStatus: WorldConnectionEnvStatus,
): WorldConnectionResult {
  return connectionResult({
    probe: { ...probe, kind: probe.kind },
    context,
    envStatus,
    state: "configured-readiness",
    ok: true,
    durationMs: 0,
    evidence: "safe-read-only-endpoint-not-configured",
    message: probe.configuredMessage,
  });
}

function operationsProbe(definition: OperationsProbeDefinition): WorldConnectionProbe {
  return {
    ...definition,
    run: async (context, envStatus) => {
      const request = definition.request?.(context.env);

      if (!request) {
        return configuredReadinessResult(definition, context, envStatus);
      }

      return runReadOnlyHttpProbe(context, envStatus, {
        ...definition,
        ...request,
        method: "GET",
      });
    },
  };
}

export const operationConnectionProbes: readonly WorldConnectionProbe[] = [
  operationsProbe({
    id: "redis-valkey-readiness",
    providerId: "redis-valkey",
    packageId: "cache-config/redis-valkey",
    name: "Redis/Valkey",
    category: "Operations",
    kind: "env",
    endpoint: "env:REDIS_URL",
    documentationUrl: "https://redis.io/docs/latest/commands/ping/",
    requiredEnv: ["REDIS_URL"],
    optionalEnv: ["REDIS_READINESS_URL"],
    method: operationsMissingConfigPolicy.method,
    operation: operationsMissingConfigPolicy.operation,
    configuredMessage:
      "Redis/Valkey credentials are present. Live validation requires an app-owned read-only readiness URL because this template does not open raw Redis sockets.",
    request: (env) => {
      const endpoint = requiredUrlValue(env, "REDIS_READINESS_URL");

      if (!endpoint) {
        return undefined;
      }

      return {
        endpoint,
        expectedStatuses: [200, 204],
        evidence: "app-owned-redis-readiness-get",
      };
    },
  }),
  operationsProbe({
    id: "upstash-redis-ping",
    providerId: "upstash-redis",
    packageId: "cache-config/upstash-redis",
    name: "Upstash Redis",
    category: "Operations",
    kind: "http",
    endpoint: "env:UPSTASH_REDIS_REST_URL/ping",
    documentationUrl: "https://upstash.com/docs/redis/features/restapi",
    requiredEnv: ["UPSTASH_REDIS_REST_URL", "UPSTASH_REDIS_REST_TOKEN"],
    optionalEnv: [],
    method: operationsMissingConfigPolicy.method,
    operation: operationsMissingConfigPolicy.operation,
    configuredMessage: "Upstash Redis credentials are present, but the read-only REST ping was not executed.",
    request: (env) => {
      const base = requiredUrlValue(env, "UPSTASH_REDIS_REST_URL");
      const token = requiredUrlValue(env, "UPSTASH_REDIS_REST_TOKEN");

      if (!base || !token) {
        return undefined;
      }

      return {
        endpoint: joinUrl(base, "ping"),
        headers: bearer(token),
        evidence: "upstash-redis-rest-ping",
      };
    },
  }),
  operationsProbe({
    id: "cloudflare-kv-namespace-keys",
    providerId: "cloudflare-kv",
    packageId: "cache-config/cloudflare-kv",
    name: "Cloudflare KV",
    category: "Operations",
    kind: "http",
    endpoint: "env:CLOUDFLARE_KV_NAMESPACE_ID/keys",
    documentationUrl: "https://developers.cloudflare.com/api/resources/kv/",
    requiredEnv: ["CLOUDFLARE_ACCOUNT_ID", "CLOUDFLARE_API_TOKEN", "CLOUDFLARE_KV_NAMESPACE_ID"],
    optionalEnv: [],
    method: operationsMissingConfigPolicy.method,
    operation: operationsMissingConfigPolicy.operation,
    configuredMessage: "Cloudflare KV credentials are present, but the read-only namespace key-list probe was not executed.",
    request: (env) => {
      const accountId = requiredUrlValue(env, "CLOUDFLARE_ACCOUNT_ID");
      const token = requiredUrlValue(env, "CLOUDFLARE_API_TOKEN");
      const namespaceId = requiredUrlValue(env, "CLOUDFLARE_KV_NAMESPACE_ID");

      if (!accountId || !token || !namespaceId) {
        return undefined;
      }

      return {
        endpoint: `https://api.cloudflare.com/client/v4/accounts/${accountId}/storage/kv/namespaces/${namespaceId}/keys?limit=1`,
        headers: bearer(token),
        evidence: "cloudflare-kv-namespace-keys-list",
      };
    },
  }),
  operationsProbe({
    id: "posthog-project-readiness",
    providerId: "posthog",
    packageId: "analytics/posthog",
    name: "PostHog",
    category: "Operations",
    kind: "http",
    endpoint: "env:POSTHOG_PROJECT_ID",
    documentationUrl: "https://posthog.com/docs/api",
    requiredEnv: ["NEXT_PUBLIC_POSTHOG_KEY", "POSTHOG_PERSONAL_API_KEY", "POSTHOG_PROJECT_ID"],
    optionalEnv: ["POSTHOG_HOST"],
    method: operationsMissingConfigPolicy.method,
    operation: operationsMissingConfigPolicy.operation,
    configuredMessage: "PostHog analytics config is present, but no read-only project metadata probe was executed.",
    request: (env) => {
      const host = env.POSTHOG_HOST ?? defaultPostHogHost;
      const token = requiredUrlValue(env, "POSTHOG_PERSONAL_API_KEY");
      const projectId = requiredUrlValue(env, "POSTHOG_PROJECT_ID");

      if (!token || !projectId) {
        return undefined;
      }

      return {
        endpoint: joinUrl(host, `api/projects/${projectId}/`),
        headers: bearer(token),
        evidence: "posthog-project-read-metadata",
      };
    },
  }),
  operationsProbe({
    id: "plausible-stats-aggregate",
    providerId: "plausible",
    packageId: "analytics/plausible",
    name: "Plausible",
    category: "Operations",
    kind: "http",
    endpoint: "env:NEXT_PUBLIC_PLAUSIBLE_DOMAIN",
    documentationUrl: "https://plausible.io/docs/stats-api",
    requiredEnv: ["NEXT_PUBLIC_PLAUSIBLE_DOMAIN", "PLAUSIBLE_API_KEY"],
    optionalEnv: ["PLAUSIBLE_API_HOST"],
    method: operationsMissingConfigPolicy.method,
    operation: operationsMissingConfigPolicy.operation,
    configuredMessage: "Plausible analytics config is present, but no read-only stats aggregate probe was executed.",
    request: (env) => {
      const host = env.PLAUSIBLE_API_HOST ?? defaultPlausibleHost;
      const siteId = requiredUrlValue(env, "NEXT_PUBLIC_PLAUSIBLE_DOMAIN");
      const token = requiredUrlValue(env, "PLAUSIBLE_API_KEY");

      if (!siteId || !token) {
        return undefined;
      }

      const query = new URLSearchParams({
        site_id: siteId,
        period: "day",
        metrics: "visitors",
      });

      return {
        endpoint: `${joinUrl(host, "api/v1/stats/aggregate")}?${query.toString()}`,
        headers: bearer(token),
        evidence: "plausible-stats-aggregate-read",
      };
    },
  }),
  operationsProbe({
    id: "vercel-analytics-project-readiness",
    providerId: "vercel-analytics",
    packageId: "analytics/vercel-analytics",
    name: "Vercel Analytics",
    category: "Operations",
    kind: "http",
    endpoint: "env:VERCEL_PROJECT_ID",
    documentationUrl: "https://vercel.com/docs/analytics/using-web-analytics",
    requiredEnv: ["VERCEL_PROJECT_ID"],
    optionalEnv: ["VERCEL_TOKEN"],
    method: operationsMissingConfigPolicy.method,
    operation: operationsMissingConfigPolicy.operation,
    configuredMessage:
      "Vercel Analytics project identity is present. Live analytics data remains provider-gated unless an app supplies a safe read-only Vercel API token.",
    request: (env) => {
      const projectId = requiredUrlValue(env, "VERCEL_PROJECT_ID");
      const token = requiredUrlValue(env, "VERCEL_TOKEN");

      if (!projectId || !token) {
        return undefined;
      }

      return {
        endpoint: `https://api.vercel.com/v9/projects/${projectId}`,
        headers: bearer(token),
        evidence: "vercel-project-readiness-read",
      };
    },
  }),
  operationsProbe({
    id: "sentry-project-readiness",
    providerId: "sentry",
    packageId: "observability/sentry",
    name: "Sentry",
    category: "Operations",
    kind: "http",
    endpoint: "env:SENTRY_PROJECT",
    documentationUrl: "https://docs.sentry.io/api/projects/retrieve-a-project/",
    requiredEnv: ["SENTRY_DSN", "SENTRY_AUTH_TOKEN", "SENTRY_ORG", "SENTRY_PROJECT"],
    optionalEnv: [],
    method: operationsMissingConfigPolicy.method,
    operation: operationsMissingConfigPolicy.operation,
    configuredMessage: "Sentry credentials are present, but no read-only project metadata probe was executed.",
    request: (env) => {
      const token = requiredUrlValue(env, "SENTRY_AUTH_TOKEN");
      const org = requiredUrlValue(env, "SENTRY_ORG");
      const project = requiredUrlValue(env, "SENTRY_PROJECT");

      if (!token || !org || !project) {
        return undefined;
      }

      return {
        endpoint: `https://sentry.io/api/0/projects/${org}/${project}/`,
        headers: bearer(token),
        evidence: "sentry-project-read-metadata",
      };
    },
  }),
  operationsProbe({
    id: "datadog-api-key-validate",
    providerId: "datadog",
    packageId: "observability/datadog",
    name: "Datadog",
    category: "Operations",
    kind: "http",
    endpoint: "env:DD_API_KEY",
    documentationUrl: "https://docs.datadoghq.com/api/latest/authentication/",
    requiredEnv: ["DD_API_KEY"],
    optionalEnv: ["DD_SITE", "DATADOG_SITE"],
    method: operationsMissingConfigPolicy.method,
    operation: operationsMissingConfigPolicy.operation,
    configuredMessage: "Datadog API key is present, but no read-only key validation probe was executed.",
    request: (env) => {
      const apiKey = requiredUrlValue(env, "DD_API_KEY");
      const site = env.DD_SITE ?? env.DATADOG_SITE ?? defaultDatadogSite;

      if (!apiKey) {
        return undefined;
      }

      return {
        endpoint: `https://api.${site}/api/v1/validate`,
        headers: { "DD-API-KEY": apiKey },
        evidence: "datadog-api-key-validate",
      };
    },
  }),
  operationsProbe({
    id: "opentelemetry-healthcheck",
    providerId: "opentelemetry",
    packageId: "observability/opentelemetry",
    name: "OpenTelemetry",
    category: "Operations",
    kind: "env",
    endpoint: "env:OTEL_EXPORTER_OTLP_ENDPOINT",
    documentationUrl: "https://opentelemetry.io/docs/specs/otlp/",
    requiredEnv: ["OTEL_EXPORTER_OTLP_ENDPOINT"],
    optionalEnv: ["OTEL_EXPORTER_OTLP_HEADERS", "OTEL_HEALTHCHECK_URL"],
    method: operationsMissingConfigPolicy.method,
    operation: operationsMissingConfigPolicy.operation,
    configuredMessage:
      "OpenTelemetry OTLP export config is present. Live validation requires an app-owned collector health-check URL because OTLP export endpoints receive telemetry payloads.",
    request: (env) => {
      const endpoint = requiredUrlValue(env, "OTEL_HEALTHCHECK_URL");

      if (!endpoint) {
        return undefined;
      }

      return {
        endpoint,
        expectedStatuses: [200, 204],
        evidence: "opentelemetry-collector-healthcheck-get",
      };
    },
  }),
];

const operationProbeByProviderId = new Map<string, WorldConnectionProbe>(
  operationConnectionProbes.map((probe) => [probe.providerId, probe]),
);

function buildContext(options: {
  env?: Record<string, string | undefined>;
  allowNetwork?: boolean;
  timeoutMs?: number;
}): WorldConnectionContext {
  return {
    env: options.env ?? process?.env ?? {},
    allowNetwork: options.allowNetwork ?? true,
    includeCli: false,
    timeoutMs: options.timeoutMs ?? 3500,
    checkedAt: new Date().toISOString(),
    fetch,
  };
}

function missingConfigResult(
  probe: WorldConnectionProbe,
  context: WorldConnectionContext,
  envStatus: WorldConnectionEnvStatus,
): WorldConnectionResult {
  return connectionResult({
    probe: { ...probe, kind: probe.kind },
    context,
    envStatus,
    state: operationsMissingConfigPolicy.status,
    ok: false,
    durationMs: 0,
    evidence: "required-env-missing",
    message: "Required Env Firewall keys are missing; live provider validation was not attempted.",
  });
}

export async function runOperationsProviderProbe(
  providerId: string,
  options: {
    env?: Record<string, string | undefined>;
    allowNetwork?: boolean;
    timeoutMs?: number;
  } = {},
): Promise<WorldConnectionResult | undefined> {
  const probe = operationProbeByProviderId.get(providerId);

  if (!probe) {
    return undefined;
  }

  const context = buildContext(options);
  const envStatus = connectionEnvStatus(probe.requiredEnv, probe.optionalEnv ?? [], context.env);

  if (envStatus.missingEnv.length > 0) {
    return missingConfigResult(probe, context, envStatus);
  }

  return probe.run(context, envStatus);
}
