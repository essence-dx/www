import { createDxBetterAuthGoogleProviderConfig } from "./config";

export function GET() {
  const config = createDxBetterAuthGoogleProviderConfig();

  return Response.json(
    {
      provider: "google",
      status: config.configured ? "ready" : "missing-config",
      configured: config.configured,
      missingConfig: config.missingEnv,
      oauthExecution: false,
      liveOAuthHttpStatus: config.configured ? 200 : 501,
      boundary:
        "Google OAuth starts only after the app provides provider credentials.",
    },
    { status: 200 },
  );
}
