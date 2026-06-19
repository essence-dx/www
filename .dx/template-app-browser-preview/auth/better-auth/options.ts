import type { BetterAuthOptions } from "better-auth";

import {
  dxBetterAuthForgePackage,
  type DxBetterAuthRequiredEnv,
} from "./metadata";

export type DxBetterAuthEnv = Partial<
  Record<DxBetterAuthRequiredEnv | "NEXT_PUBLIC_BETTER_AUTH_URL", string>
>;

export type DxBetterAuthConfig = {
  baseURL: string;
  missingEnv: DxBetterAuthRequiredEnv[];
  configured: boolean;
  googleConfigured: boolean;
};

export type DxBetterAuthOptionsInput = {
  env?: DxBetterAuthEnv;
  database?: BetterAuthOptions["database"];
  plugins?: NonNullable<BetterAuthOptions["plugins"]>;
  trustedOrigins?: string[];
  overrides?: Partial<BetterAuthOptions>;
};

function readProcessEnv(): DxBetterAuthEnv {
  if (typeof process === "undefined") {
    return {};
  }

  return process.env as DxBetterAuthEnv;
}

export function readDxBetterAuthConfig(
  env: DxBetterAuthEnv = readProcessEnv(),
): DxBetterAuthConfig {
  const missingEnv = dxBetterAuthForgePackage.requiredEnv.filter(
    (key) => !env[key],
  ) as DxBetterAuthRequiredEnv[];
  const baseURL =
    env.BETTER_AUTH_URL ?? env.NEXT_PUBLIC_BETTER_AUTH_URL ?? "/api/auth";

  return {
    baseURL,
    missingEnv,
    configured: missingEnv.length === 0,
    googleConfigured: Boolean(env.GOOGLE_CLIENT_ID && env.GOOGLE_CLIENT_SECRET),
  };
}

export function createDxBetterAuthOptions(
  input: DxBetterAuthOptionsInput = {},
): BetterAuthOptions {
  const env = input.env ?? readProcessEnv();
  const config = readDxBetterAuthConfig(env);
  const socialProviders = config.googleConfigured
    ? {
        google: {
          clientId: env.GOOGLE_CLIENT_ID,
          clientSecret: env.GOOGLE_CLIENT_SECRET,
        },
      }
    : undefined;

  return {
    secret: env.BETTER_AUTH_SECRET ?? "dx-missing-better-auth-secret",
    baseURL: config.baseURL,
    database: input.database,
    emailAndPassword: {
      enabled: true,
    },
    socialProviders,
    trustedOrigins: input.trustedOrigins,
    plugins: input.plugins,
    ...input.overrides,
  };
}
