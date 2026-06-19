import {
  defaultInstantEnv,
  readInstantConfig,
  type DxInstantEnv,
} from "../../lib/instant/env.ts";

const requiredInstantEnv = ["NEXT_PUBLIC_INSTANT_APP_ID"] as const;

export type DxInstantReadiness = {
  readonly schema: "dx.www.template.instant_readiness";
  readonly packageId: "instantdb/react";
  readonly officialName: "Realtime App Database";
  readonly route: "/api/instant/readiness";
  readonly status: "provider-gated" | "configured-source-owned-adapter-boundary";
  readonly httpStatus: 200 | 501;
  readonly runtimeProof: false;
  readonly networkCalls: false;
  readonly hostedCredentials: false;
  readonly requiredEnv: typeof requiredInstantEnv;
  readonly missingEnv: readonly string[];
  readonly validationError: string | null;
  readonly sourceOwnedSurfaces: readonly string[];
  readonly appOwnedBoundary: readonly string[];
  readonly message: string;
};

export function readInstantReadiness(
  env: DxInstantEnv = defaultInstantEnv(),
): DxInstantReadiness {
  const missingEnv = requiredInstantEnv.filter((key) => !env[key]?.trim());
  const validationError = validateInstantConfig(env, missingEnv.length === 0);
  const httpStatus = validationError || missingEnv.length > 0 ? 501 : 200;

  return {
    schema: "dx.www.template.instant_readiness",
    packageId: "instantdb/react",
    officialName: "Realtime App Database",
    route: "/api/instant/readiness",
    status:
      httpStatus === 200
        ? "configured-source-owned-adapter-boundary"
        : "provider-gated",
    httpStatus,
    runtimeProof: false,
    networkCalls: false,
    hostedCredentials: false,
    requiredEnv: requiredInstantEnv,
    missingEnv,
    validationError,
    sourceOwnedSurfaces: [
      "lib/instant/env.ts",
      "lib/instant/schema.ts",
      "lib/instant/status.ts",
      "lib/instant/route.ts",
      "server/instant/readiness.ts",
      "app/api/instant/readiness/route.ts",
    ],
    appOwnedBoundary: [
      "Instant hosted app id",
      "rules and auth policy",
      "realtime transport",
      "storage and stream runtime proof",
    ],
    message:
      httpStatus === 200
        ? "InstantDB public configuration validates locally; hosted rules, auth, realtime transport, storage, and stream proof remain app-owned."
        : "This route performs local configuration validation only; configure the Instant app id before enabling hosted auth, realtime transport, storage, or streams.",
  };
}

export function createInstantReadinessResponse(env?: DxInstantEnv): Response {
  const readiness = readInstantReadiness(env);

  return Response.json(readiness, {
    status: readiness.httpStatus,
    headers: {
      "cache-control": "no-store",
    },
  });
}

function validateInstantConfig(
  env: DxInstantEnv,
  shouldValidate: boolean,
): string | null {
  if (!shouldValidate) {
    return null;
  }

  try {
    readInstantConfig(env);
    return null;
  } catch (error) {
    return error instanceof Error
      ? error.message
      : "Invalid InstantDB public configuration.";
  }
}
