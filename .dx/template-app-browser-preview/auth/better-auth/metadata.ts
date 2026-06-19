export const dxBetterAuthForgePackage = {
  packageId: "auth/better-auth",
  officialName: "Authentication",
  upstreamName: "better-auth",
  upstreamVersion: "1.6.11",
  version: "1.6.11-dx.9",
  sourceKind: "curated-registry",
  license: "MIT",
  providerSurfaces: ["email-password", "google-oauth"],
  requiredEnv: [
    "BETTER_AUTH_SECRET",
    "BETTER_AUTH_URL",
    "GOOGLE_CLIENT_ID",
    "GOOGLE_CLIENT_SECRET",
  ],
  publicEnv: ["NEXT_PUBLIC_BETTER_AUTH_URL"],
  sourceMirror: "G:/WWW/inspirations/better-auth",
  receiptPaths: {
    package: ".dx/forge/receipts/packages/auth-better-auth.json",
    dashboard: ".dx/forge/receipts/auth-better-auth.json",
    rollback: ".dx/forge/receipts/20260523T052300000000000Z-auth-better-auth.json",
    safetyArchive: ".dx/forge/receipts/safety/auth-better-auth-archive.json",
  },
  provenance: {
    source: "dx-forge-curated-registry",
    upstreamReference: "npm:better-auth@1.6.11",
    sourceMirror: "G:/WWW/inspirations/better-auth",
    verified: false,
    note: "DX inspected the local Better Auth source mirror, React client, Next.js handler integration, session APIs, email-password actions, account management APIs, and Google OAuth provider shape before curating this source-owned Authentication slice.",
  },
} as const;

export type DxBetterAuthRequiredEnv =
  (typeof dxBetterAuthForgePackage.requiredEnv)[number];

export type DxBetterAuthProviderSurface =
  (typeof dxBetterAuthForgePackage.providerSurfaces)[number];
