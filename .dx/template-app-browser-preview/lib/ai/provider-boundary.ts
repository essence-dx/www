export type DxAiProviderBoundaryOptions = {
  provider: "openai-compatible" | "gateway";
  capability: string;
  requiredEnv: string;
  appOwnedBoundary: string;
};

export type DxAiExtendedRouteBoundaryOptions = DxAiProviderBoundaryOptions & {
  route: string;
  enableEnv?: string;
  credentialsConfigured?: boolean;
};

export const DX_AI_EXTENDED_ROUTES_ENABLE_ENV = "DX_ENABLE_EXTENDED_AI_ROUTES";

export function isDxAiExtendedRouteEnabled(
  env: Record<string, string | undefined> = process.env,
): boolean {
  return env[DX_AI_EXTENDED_ROUTES_ENABLE_ENV] === "true";
}

export function createDxAiMissingProviderResponse({
  provider,
  capability,
  requiredEnv,
  appOwnedBoundary,
}: DxAiProviderBoundaryOptions): Response {
  return Response.json(
    {
      ok: false,
      status: "missing-config",
      httpStatus: 501,
      provider,
      capability,
      requiredEnv: [requiredEnv],
      credentialsConfigured: false,
      adapterBoundary: "provider-credential-boundary",
      runtimeExecution: false,
      modelStreaming: false,
      providerRuntime: false,
      secretValues: [],
      appOwnedBoundary,
    },
    { status: 501 },
  );
}

export function createDxAiExtendedRouteDisabledResponse({
  provider,
  capability,
  requiredEnv,
  appOwnedBoundary,
  route,
  enableEnv = DX_AI_EXTENDED_ROUTES_ENABLE_ENV,
  credentialsConfigured = false,
}: DxAiExtendedRouteBoundaryOptions): Response {
  return Response.json(
    {
      ok: false,
      status: "extended-route-disabled",
      httpStatus: 501,
      route,
      provider,
      capability,
      requiredEnv: [enableEnv, requiredEnv],
      credentialsConfigured,
      adapterBoundary: "extended-provider-route-boundary",
      proofSurface: "outside-default-ai-surface",
      defaultAiSurface: false,
      runtimeExecution: false,
      modelStreaming: false,
      providerRuntime: false,
      secretValues: [],
      appOwnedBoundary,
    },
    { status: 501 },
  );
}
