import {
  buildProviderResult,
  checkedAt,
  configuredProbe,
  isLocalReadinessUrl,
  missingConfigResult,
  missingEnvNames,
  readEnvValue,
  readProviderEnv,
} from "./env";
import type {
  DataIdentityConnectionResult,
  DataIdentityProviderDefinition,
  DataIdentityProbeContext,
} from "./data-identity-types";
import type { WorldConnectionProbe } from "../contracts";
import { connectionResult, runReadOnlyHttpProbe } from "../http";

export type WorldAuthProviderId =
  | "dx-auth-forge-auth"
  | "better-auth"
  | "clerk"
  | "auth-js";

export const authProviderDefinitions = [
  authProvider("dx-auth-forge-auth", "DX Auth / Forge auth", [
    "DX_AUTH_SECRET",
    "DX_AUTH_URL",
  ], ["DX_AUTH_STATUS_URL"]),
  authProvider("better-auth", "Better Auth", [
    "BETTER_AUTH_SECRET",
    "BETTER_AUTH_URL",
  ], ["BETTER_AUTH_TRUSTED_ORIGINS", "BETTER_AUTH_STATUS_URL"]),
  authProvider("clerk", "Clerk", [
    "CLERK_SECRET_KEY",
    "NEXT_PUBLIC_CLERK_PUBLISHABLE_KEY",
  ], ["CLERK_WEBHOOK_SECRET", "CLERK_STATUS_URL"]),
  authProvider("auth-js", "Auth.js", ["AUTH_SECRET", "AUTH_URL"], ["AUTHJS_STATUS_URL"]),
] as const satisfies readonly DataIdentityProviderDefinition<WorldAuthProviderId>[];

export const authConnectionProbes: readonly WorldConnectionProbe[] = [
  localAuthStatusProbe({
    id: "dx-auth-local-status",
    providerId: "dx-auth-forge-auth",
    packageId: "auth/dx-auth-forge-auth",
    name: "DX Auth / Forge auth",
    endpointEnv: "DX_AUTH_STATUS_URL",
    requiredEnv: ["DX_AUTH_SECRET", "DX_AUTH_URL"],
    optionalEnv: ["DX_AUTH_STATUS_URL"],
  }),
  localAuthStatusProbe({
    id: "better-auth-local-status",
    providerId: "better-auth",
    packageId: "auth/better-auth",
    name: "Better Auth",
    endpointEnv: "BETTER_AUTH_STATUS_URL",
    requiredEnv: ["BETTER_AUTH_SECRET", "BETTER_AUTH_URL"],
    optionalEnv: ["BETTER_AUTH_TRUSTED_ORIGINS", "BETTER_AUTH_STATUS_URL"],
    documentationUrl: "https://better-auth.com/docs/concepts/database",
  }),
  localAuthStatusProbe({
    id: "clerk-local-status",
    providerId: "clerk",
    packageId: "auth/clerk",
    name: "Clerk",
    endpointEnv: "CLERK_STATUS_URL",
    requiredEnv: ["CLERK_SECRET_KEY", "NEXT_PUBLIC_CLERK_PUBLISHABLE_KEY"],
    optionalEnv: ["CLERK_WEBHOOK_SECRET", "CLERK_STATUS_URL"],
    documentationUrl: "https://clerk.com/docs",
  }),
  localAuthStatusProbe({
    id: "auth-js-local-status",
    providerId: "auth-js",
    packageId: "auth/auth-js",
    name: "Auth.js",
    endpointEnv: "AUTHJS_STATUS_URL",
    requiredEnv: ["AUTH_SECRET", "AUTH_URL"],
    optionalEnv: ["AUTHJS_STATUS_URL"],
    documentationUrl: "https://authjs.dev",
  }),
];

export async function probeAuthProvider(
  providerId: WorldAuthProviderId,
  context: DataIdentityProbeContext = {},
): Promise<DataIdentityConnectionResult<WorldAuthProviderId>> {
  const definition = readAuthProvider(providerId);
  const env = readProviderEnv(context);

  if (missingEnvNames(env, definition.requiredEnv).length > 0) {
    return missingConfigResult(definition, context);
  }

  return probeConfiguredAuth(definition, context);
}

export function readAuthProvider(
  providerId: WorldAuthProviderId,
): DataIdentityProviderDefinition<WorldAuthProviderId> {
  const definition = authProviderDefinitions.find((item) => item.id === providerId);
  if (!definition) {
    throw new Error(`Unsupported world auth provider: ${providerId}`);
  }
  return definition;
}

function authProvider(
  id: WorldAuthProviderId,
  name: string,
  requiredEnv: readonly string[],
  optionalEnv: readonly string[],
): DataIdentityProviderDefinition<WorldAuthProviderId> {
  return {
    id,
    name,
    kind: "auth",
    category: "Authentication and identity",
    requiredEnv,
    optionalEnv,
    receiptSchemas: [
      "dx.forge.world.auth",
      "dx.forge.world.security",
      "dx.forge.world.provider-live-proof",
      "dx.forge.world.preview-only",
    ],
    appOwnedBoundary:
      "Session storage, cookies, callback URLs, provider credentials, user data, audit logging, and account lifecycle policy stay app-owned.",
    statusEndpointEnv: optionalEnv.find((item) => item.endsWith("_STATUS_URL")),
  };
}

async function probeConfiguredAuth(
  definition: DataIdentityProviderDefinition<WorldAuthProviderId>,
  context: DataIdentityProbeContext,
): Promise<DataIdentityConnectionResult<WorldAuthProviderId>> {
  const endpointName = definition.statusEndpointEnv;
  const endpoint = endpointName ? readEnvValue(readProviderEnv(context), endpointName) : null;

  if (!endpoint || !context.fetch) {
    return buildProviderResult({
      context,
      definition,
      nextAction:
        "Mount a local status endpoint or import an app-owned auth provider receipt.",
      probe: configuredProbe(
        context,
        endpoint
          ? "Auth env is configured; fetch is unavailable so the local status endpoint was not called."
          : "Auth env is configured; no local status endpoint is declared.",
      ),
      status: "configured-readiness",
    });
  }

  if (!isLocalReadinessUrl(endpoint)) {
    return buildProviderResult({
      blockers: [`${endpointName} is not local; hosted auth endpoints are not probed by this adapter.`],
      context,
      definition,
      nextAction:
        "Use an app-owned provider receipt for hosted auth, or expose a local redacted readiness endpoint.",
      probe: configuredProbe(
        context,
        "Auth env is configured; non-local status endpoint was intentionally skipped.",
      ),
      status: "configured-readiness",
    });
  }

  const response = await context.fetch(endpoint, {
    method: "GET",
    headers: { "Cache-Control": "no-store" },
  });

  return buildProviderResult({
    blockers: response.ok ? [] : [`${endpointName} returned HTTP ${response.status}.`],
    context,
    definition,
    nextAction: response.ok
      ? "Import the redacted auth provider-live-proof receipt."
      : "Fix the local auth readiness endpoint before claiming live validation.",
    probe: {
      kind: "local-status-endpoint",
      checkedAt: checkedAt(context),
      live: response.ok,
      endpointEnv: endpointName,
      endpointKind: "local",
      message: response.ok
        ? "Local auth readiness endpoint responded successfully."
        : "Local auth readiness endpoint did not pass.",
    },
    status: response.ok ? "live-validated" : "blocked",
  });
}

function localAuthStatusProbe(input: {
  id: string;
  providerId: string;
  packageId: string;
  name: string;
  endpointEnv: string;
  requiredEnv: readonly string[];
  optionalEnv: readonly string[];
  documentationUrl?: string;
}): WorldConnectionProbe {
  const base = {
    id: input.id,
    providerId: input.providerId,
    packageId: input.packageId,
    name: input.name,
    category: "Authentication",
    kind: "http" as const,
    endpoint: `env:${input.endpointEnv}`,
    documentationUrl: input.documentationUrl,
    requiredEnv: input.requiredEnv,
    optionalEnv: input.optionalEnv,
  };

  return {
    ...base,
    run: async (context, envStatus) => {
      const endpoint = context.env[input.endpointEnv]?.trim();

      if (!endpoint) {
        return connectionResult({
          probe: base,
          context,
          envStatus,
          state: "configured-readiness",
          ok: true,
          durationMs: 0,
          evidence: "env-contract-satisfied",
          message:
            "Auth env is present; no local status endpoint is configured.",
        });
      }

      if (!isLocalReadinessUrl(endpoint)) {
        return connectionResult({
          probe: base,
          context,
          envStatus,
          state: "configured-readiness",
          ok: true,
          durationMs: 0,
          evidence: "hosted-status-endpoint-skipped",
          message:
            "Hosted auth endpoints are not probed by this adapter; use an app-owned provider receipt.",
        });
      }

      return runReadOnlyHttpProbe(context, envStatus, {
        ...base,
        endpoint,
        method: "GET",
        headers: { "Cache-Control": "no-store" },
        expectedStatuses: [200, 204],
        evidence: "local-auth-status-readable",
      });
    },
  };
}
