import {
  firebaseFirestoreCrudOptionalEnv,
  firebaseFirestoreCrudRequiredEnv,
  runFirebaseFirestoreCrudSmoke,
} from "../../../../lib/world/connections/providers";

declare const process:
  | {
      env?: Record<string, string | undefined>;
    }
  | undefined;

const confirmationValue = "firebase-firestore-crud";

function canRunCrud(request: Request): boolean {
  const requestUrl = new URL(request.url);

  return (
    process?.env?.DX_WORLD_ALLOW_FIREBASE_CRUD === "1" &&
    (request.headers.get("x-dx-world-confirm") === confirmationValue ||
      requestUrl.searchParams.get("confirm") === confirmationValue)
  );
}

export function GET() {
  return Response.json({
    schema: "dx.examples.world.firebase-firestore-crud-route",
    ok: true,
    status: "ready",
    providerId: "firebase-firestore",
    packageId: "database/firebase-firestore",
    method: "POST",
    redaction: "secret-values-never-included",
    requiredEnv: firebaseFirestoreCrudRequiredEnv,
    optionalEnv: firebaseFirestoreCrudOptionalEnv,
    liveProviderExecution: false,
    routeBoundary:
      "POST runs a temporary Firebase Firestore CRUD smoke only when DX_WORLD_ALLOW_FIREBASE_CRUD=1 and the confirmation header or query is present.",
    nextAction:
      "Set Firebase env values, then POST with x-dx-world-confirm: firebase-firestore-crud.",
  });
}

export async function POST(request: Request) {
  if (!canRunCrud(request)) {
    return Response.json(
      {
        schema: "dx.examples.world.firebase-firestore-crud-route",
        ok: false,
        status: "blocked",
        providerId: "firebase-firestore",
        redaction: "secret-values-never-included",
        liveProviderExecution: false,
        nextAction:
          "Set DX_WORLD_ALLOW_FIREBASE_CRUD=1 and pass x-dx-world-confirm: firebase-firestore-crud to run the mutating smoke.",
      },
      { status: 428 },
    );
  }

  const receipt = await runFirebaseFirestoreCrudSmoke({
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
