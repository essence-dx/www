import type { BetterAuthOptions } from "better-auth";

export type DxBetterAuthEnv = Record<string, string | undefined>;

export type DxBetterAuthRuntime = {
  appName: string;
  baseURL: string;
  secret: string;
  trustedOrigins: string[];
  socialProviders: NonNullable<BetterAuthOptions["socialProviders"]>;
  emailAndPassword: {
    enabled: boolean;
  };
};

const DEFAULT_APP_NAME = "DX";
const DEFAULT_BASE_URL = "http://localhost:3000";

export function defaultBetterAuthEnv(): DxBetterAuthEnv {
  return (globalThis as unknown as { process?: { env?: DxBetterAuthEnv } }).process?.env ?? {};
}

export function readBetterAuthRuntime(
  env: DxBetterAuthEnv = defaultBetterAuthEnv(),
): DxBetterAuthRuntime {
  const baseURL = optionalEnv(env, "BETTER_AUTH_URL") ?? DEFAULT_BASE_URL;
  const trustedOrigins = readList(env, "BETTER_AUTH_TRUSTED_ORIGINS");

  return {
    appName: optionalEnv(env, "BETTER_AUTH_APP_NAME") ?? DEFAULT_APP_NAME,
    baseURL,
    secret: requiredEnv(env, "BETTER_AUTH_SECRET"),
    trustedOrigins: trustedOrigins.length > 0 ? trustedOrigins : [baseURL],
    socialProviders: readSocialProviders(env),
    emailAndPassword: {
      enabled: env.BETTER_AUTH_EMAIL_PASSWORD_ENABLED !== "false",
    },
  };
}

export function readSocialProviders(
  env: DxBetterAuthEnv = defaultBetterAuthEnv(),
): NonNullable<BetterAuthOptions["socialProviders"]> {
  return {
    ...readOAuthProvider(env, "google", "GOOGLE"),
  };
}

function readOAuthProvider(
  env: DxBetterAuthEnv,
  provider: "google",
  prefix: "GOOGLE",
): NonNullable<BetterAuthOptions["socialProviders"]> {
  const clientId = optionalEnv(env, `${prefix}_CLIENT_ID`);
  const clientSecret = optionalEnv(env, `${prefix}_CLIENT_SECRET`);

  if (!clientId && !clientSecret) {
    return {};
  }

  if (!clientId || !clientSecret) {
    throw new Error(`${prefix}_CLIENT_ID and ${prefix}_CLIENT_SECRET must be configured together`);
  }

  return {
    google: {
      clientId,
      clientSecret,
    },
  };
}

function readList(env: DxBetterAuthEnv, key: string): string[] {
  return (env[key] ?? "")
    .split(/[,\s]+/)
    .map((value) => value.trim())
    .filter(Boolean);
}

function optionalEnv(env: DxBetterAuthEnv, key: string): string | undefined {
  const value = env[key]?.trim();
  return value ? value : undefined;
}

function requiredEnv(env: DxBetterAuthEnv, key: string): string {
  const value = optionalEnv(env, key);
  if (!value) {
    throw new Error(`Missing required Authentication env var: ${key}`);
  }
  return value;
}
