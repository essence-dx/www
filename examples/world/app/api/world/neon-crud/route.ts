import {
  neonCrudAcceptedEnv,
  neonCrudOptionalEnv,
  neonCrudRequiredEnv,
  runNeonDatabaseCrudSmoke,
} from "../../../../lib/world/connections/providers";

declare const process:
  | {
      env?: Record<string, string | undefined>;
    }
  | undefined;

const confirmationValue = "neon-database-crud";

function canRunCrud(request: Request): boolean {
  const requestUrl = new URL(request.url);

  return (
    process?.env?.DX_WORLD_ALLOW_NEON_CRUD === "1" &&
    (request.headers.get("x-dx-world-confirm") === confirmationValue ||
      requestUrl.searchParams.get("confirm") === confirmationValue)
  );
}

export function GET() {
  return Response.json({
    schema: "dx.examples.world.neon-database-crud-route",
    ok: true,
    status: "ready",
    providerId: "neon",
    packageId: "database/neon",
    method: "POST",
    redaction: "secret-values-never-included",
    requiredEnv: neonCrudRequiredEnv,
    acceptedEnv: neonCrudAcceptedEnv,
    optionalEnv: neonCrudOptionalEnv,
    liveProviderExecution: false,
    routeBoundary:
      "POST runs a temporary Neon database CRUD smoke only when DX_WORLD_ALLOW_NEON_CRUD=1 and the confirmation header or query is present.",
    nextAction:
      "Set server-only Neon env values, then POST with x-dx-world-confirm: neon-database-crud.",
  });
}

export async function POST(request: Request) {
  if (!canRunCrud(request)) {
    return Response.json(
      {
        schema: "dx.examples.world.neon-database-crud-route",
        ok: false,
        status: "blocked",
        providerId: "neon",
        redaction: "secret-values-never-included",
        liveProviderExecution: false,
        nextAction:
          "Set DX_WORLD_ALLOW_NEON_CRUD=1 and pass x-dx-world-confirm: neon-database-crud to run the mutating smoke.",
      },
      { status: 428 },
    );
  }

  const receipt = await runNeonDatabaseCrudSmoke({
    env: process?.env ?? {},
    fetch,
  });

  return Response.json(
    {
      ...receipt,
      ok: receipt.status === "live-validated",
    },
    { status: receipt.status === "live-validated" ? 200 : receipt.status === "missing-config" ? 428 : 502 },
  );
}
