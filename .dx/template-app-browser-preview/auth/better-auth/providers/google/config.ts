import type { DxBetterAuthEnv } from "../../options";

export type DxBetterAuthGoogleProviderConfig = {
  provider: "google";
  configured: boolean;
  missingEnv: readonly string[];
};

export function createDxBetterAuthGoogleProviderConfig(
  env: DxBetterAuthEnv = typeof process === "undefined"
    ? {}
    : (process.env as DxBetterAuthEnv),
): DxBetterAuthGoogleProviderConfig {
  const missingEnv = [
    !env.GOOGLE_CLIENT_ID ? "GOOGLE_CLIENT_ID" : null,
    !env.GOOGLE_CLIENT_SECRET ? "GOOGLE_CLIENT_SECRET" : null,
  ].filter(Boolean) as string[];

  return {
    provider: "google",
    configured: missingEnv.length === 0,
    missingEnv,
  };
}
