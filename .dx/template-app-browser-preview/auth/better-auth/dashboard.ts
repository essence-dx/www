import { dxBetterAuthForgePackage } from "./metadata";
import { readDxBetterAuthConfig, type DxBetterAuthEnv } from "./options";
import { dxBetterAuthSocialProviders } from "./social";

export type DxBetterAuthDashboardStatus = {
  packageId: "auth/better-auth";
  officialName: "Authentication";
  upstreamName: "better-auth";
  status: "configured" | "missing-config";
  sessionSource: "better-auth" | "missing-config";
  profileGate: "ready" | "missing-config";
  missingConfig: readonly string[];
  providers: typeof dxBetterAuthSocialProviders;
};

export function createDxBetterAuthDashboardStatus(
  env?: DxBetterAuthEnv,
): DxBetterAuthDashboardStatus {
  const config = readDxBetterAuthConfig(env);
  const status = config.configured ? "configured" : "missing-config";

  return {
    packageId: "auth/better-auth",
    officialName: dxBetterAuthForgePackage.officialName,
    upstreamName: dxBetterAuthForgePackage.upstreamName,
    status,
    sessionSource: config.configured ? "better-auth" : "missing-config",
    profileGate: config.configured ? "ready" : "missing-config",
    missingConfig: config.missingEnv,
    providers: dxBetterAuthSocialProviders,
  };
}

export const dxBetterAuthDashboardReadiness =
  createDxBetterAuthDashboardStatus();
