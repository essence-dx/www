import { createDxBetterAuthGoogleProviderConfig } from "./config";

export function readDxBetterAuthGoogleCallbackState() {
  const config = createDxBetterAuthGoogleProviderConfig();

  return {
    provider: "google" as const,
    callbackReady: config.configured,
    missingConfig: config.missingEnv,
    callbackURL: "/api/auth/callback/google",
  };
}
