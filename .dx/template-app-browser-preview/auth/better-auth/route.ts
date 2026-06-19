import { toNextJsHandler } from "better-auth/next-js";

import type { DxBetterAuthOptionsInput } from "./options";
import { createDxBetterAuth, createDxBetterAuthReadiness } from "./server";

export function createDxBetterAuthRouteHandlers(
  input: DxBetterAuthOptionsInput = {},
) {
  const readiness = createDxBetterAuthReadiness(input);

  if (!readiness.canRunRouteHandlers) {
    return {
      GET() {
        return Response.json(
          {
            ok: false,
            status: "adapter-boundary",
            httpStatus: 501,
            adapter: "better-auth",
            runtimeExecution: false,
            liveSessionExecution: false,
            credentialsConfigured: readiness.credentialsConfigured,
            databaseAdapterConfigured: readiness.databaseAdapterConfigured,
            missingConfig: readiness.missingConfig,
            adapterBoundaries: readiness.adapterBoundaries,
            message:
              "Set Authentication server, Google provider, and database adapter configuration before enabling sessions.",
          },
          { status: 501 },
        );
      },
      POST() {
        return Response.json(
          {
            ok: false,
            status: "adapter-boundary",
            httpStatus: 501,
            adapter: "better-auth",
            runtimeExecution: false,
            liveSessionExecution: false,
            credentialsConfigured: readiness.credentialsConfigured,
            databaseAdapterConfigured: readiness.databaseAdapterConfigured,
            missingConfig: readiness.missingConfig,
            adapterBoundaries: readiness.adapterBoundaries,
          },
          { status: 501 },
        );
      },
    };
  }

  return toNextJsHandler(createDxBetterAuth(input).handler);
}

const handlers = createDxBetterAuthRouteHandlers();

export const GET = handlers.GET;
export const POST = handlers.POST;
