export function GET() {
  return {
    ok: true,
    status: "ready",
    adapter: "better-auth",
    credentialsConfigured: false,
    appOwnedBoundary: "Set auth provider secrets in the app environment.",
  };
}
