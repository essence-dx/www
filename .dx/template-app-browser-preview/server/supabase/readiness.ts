import {
  defaultSupabaseEnv,
  readSupabasePublicConfig,
  type DxSupabaseEnv,
} from "../../lib/supabase/env.ts";

const requiredSupabaseEnv = [
  "NEXT_PUBLIC_SUPABASE_URL",
  "NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY",
] as const;

export type DxSupabaseReadiness = {
  readonly schema: "dx.www.template.supabase_readiness";
  readonly packageId: "supabase/client";
  readonly officialName: "Backend Platform Client";
  readonly route: "/api/supabase/readiness";
  readonly status: "provider-gated" | "configured-source-owned-adapter-boundary";
  readonly httpStatus: 200 | 501;
  readonly runtimeProof: false;
  readonly networkCalls: false;
  readonly hostedCredentials: false;
  readonly requiredEnv: typeof requiredSupabaseEnv;
  readonly missingEnv: readonly string[];
  readonly validationError: string | null;
  readonly localProject: boolean | null;
  readonly sourceOwnedSurfaces: readonly string[];
  readonly appOwnedBoundary: readonly string[];
  readonly message: string;
};

export function readSupabaseReadiness(
  env: DxSupabaseEnv = defaultSupabaseEnv(),
): DxSupabaseReadiness {
  const missingEnv = requiredSupabaseEnv.filter((key) => !env[key]?.trim());
  const validation = validateSupabaseConfig(env, missingEnv.length === 0);
  const httpStatus = validation.error || missingEnv.length > 0 ? 501 : 200;

  return {
    schema: "dx.www.template.supabase_readiness",
    packageId: "supabase/client",
    officialName: "Backend Platform Client",
    route: "/api/supabase/readiness",
    status:
      httpStatus === 200
        ? "configured-source-owned-adapter-boundary"
        : "provider-gated",
    httpStatus,
    runtimeProof: false,
    networkCalls: false,
    hostedCredentials: false,
    requiredEnv: requiredSupabaseEnv,
    missingEnv,
    validationError: validation.error,
    localProject: validation.localProject,
    sourceOwnedSurfaces: [
      "lib/supabase/env.ts",
      "lib/supabase/profiles.ts",
      "lib/supabase/profile-workflow.ts",
      "server/supabase/readiness.ts",
      "app/api/supabase/readiness/route.ts",
    ],
    appOwnedBoundary: [
      "Supabase project URL and publishable key",
      "RLS policy migration",
      "Auth redirect allow-list",
      "hosted read/write/realtime proof",
    ],
    message:
      httpStatus === 200
        ? "Supabase public configuration validates locally; hosted reads, writes, auth, RLS, and realtime remain app-owned proof."
        : "This route performs local configuration validation only; configure Supabase public env before enabling hosted reads, writes, auth, RLS, or realtime.",
  };
}

export function createSupabaseReadinessResponse(env?: DxSupabaseEnv): Response {
  const readiness = readSupabaseReadiness(env);

  return Response.json(readiness, {
    status: readiness.httpStatus,
    headers: {
      "cache-control": "no-store",
    },
  });
}

function validateSupabaseConfig(
  env: DxSupabaseEnv,
  shouldValidate: boolean,
): { readonly error: string | null; readonly localProject: boolean | null } {
  if (!shouldValidate) {
    return { error: null, localProject: null };
  }

  try {
    const config = readSupabasePublicConfig(env);
    return { error: null, localProject: config.isLocal };
  } catch (error) {
    return {
      error: error instanceof Error ? error.message : "Invalid Supabase public configuration.",
      localProject: null,
    };
  }
}
