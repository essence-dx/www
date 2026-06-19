import { betterAuth, type BetterAuthOptions } from "better-auth";
import { nextCookies } from "better-auth/next-js";

import {
  readBetterAuthRuntime,
  type DxBetterAuthEnv,
} from "./options";

export type DxBetterAuthDatabase = NonNullable<BetterAuthOptions["database"]>;
export type DxBetterAuthPluginList = NonNullable<BetterAuthOptions["plugins"]>;

export type DxBetterAuthServerOptions = {
  database: DxBetterAuthDatabase;
  env?: DxBetterAuthEnv;
  plugins?: DxBetterAuthPluginList;
  overrides?: Partial<
    Omit<
      BetterAuthOptions,
      | "baseURL"
      | "database"
      | "emailAndPassword"
      | "plugins"
      | "secret"
      | "socialProviders"
      | "trustedOrigins"
    >
  >;
};

export function createDxBetterAuth({
  database,
  env,
  plugins = [],
  overrides = {},
}: DxBetterAuthServerOptions) {
  const runtime = readBetterAuthRuntime(env);

  return betterAuth({
    ...overrides,
    appName: runtime.appName,
    baseURL: runtime.baseURL,
    secret: runtime.secret,
    database,
    trustedOrigins: runtime.trustedOrigins,
    emailAndPassword: runtime.emailAndPassword,
    socialProviders: runtime.socialProviders,
    plugins: [...plugins, nextCookies()],
  });
}
