import { betterAuth } from "better-auth";
import { nextCookies } from "better-auth/next-js";

import {
  createDxBetterAuthOptions,
  readDxBetterAuthConfig,
  type DxBetterAuthOptionsInput,
} from "./options";

export type DxBetterAuthServer = ReturnType<typeof betterAuth>;

const dxBetterAuthAdapterBoundaries = [
  "database adapter, session storage, migrations",
  "cookie lifetime, trusted origins, and secure deployment policy",
  "Google OAuth callback URL and provider-console credentials",
] as const;

export function createDxBetterAuth(
  input: DxBetterAuthOptionsInput = {},
): DxBetterAuthServer {
  const plugins = [nextCookies(), ...(input.plugins ?? [])];

  return betterAuth(
    createDxBetterAuthOptions({
      ...input,
      plugins,
    }),
  );
}

export function createDxBetterAuthReadiness(
  input: DxBetterAuthOptionsInput = {},
) {
  const config = readDxBetterAuthConfig(input.env);
  const databaseAdapterConfigured = Boolean(input.database);

  return {
    status: config.configured ? "configured" : "missing-config",
    credentialsConfigured: config.configured,
    databaseAdapterConfigured,
    sessionStorage: databaseAdapterConfigured ? "database-adapter" : "app-owned",
    canRunRouteHandlers: config.configured && databaseAdapterConfigured,
    missingConfig: config.missingEnv,
    baseURL: config.baseURL,
    adapterBoundaries: dxBetterAuthAdapterBoundaries,
  };
}
