import { readDxMobileAuthReadiness } from "../../../../../server/mobile-auth/readiness";

export async function POST() {
  const readiness = readDxMobileAuthReadiness();

  return Response.json(
    {
      schema: "dx.mobile.auth.runtime_response",
      status: "auth-runtime-boundary",
      authProviderPackage: readiness.authProviderPackage,
      nativeBridge: readiness.nativeBridge,
      appOwnedRuntimeBoundaries: readiness.appOwnedRuntimeBoundaries,
    },
    { status: 503 },
  );
}
