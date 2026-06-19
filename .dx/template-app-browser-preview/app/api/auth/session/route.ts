import { createTemplateBetterAuthSessionReceipt } from "@/server/auth/better-auth";

const defaultAuthEnv = {
  BETTER_AUTH_SECRET: "",
  BETTER_AUTH_URL: "",
  GOOGLE_CLIENT_ID: "",
  GOOGLE_CLIENT_SECRET: "",
};

export function GET() {
  const receipt = {
    ...createTemplateBetterAuthSessionReceipt({ env: defaultAuthEnv }),
    adapter: "better-auth",
    credentialsConfigured: false,
    appOwnedBoundary:
      "Live Better Auth session lookup requires app-owned cookies, database adapter, migrations, and deployment policy.",
  };

  return Response.json(receipt, {
    status: 200,
    headers: { "cache-control": "no-store" },
  });
}
