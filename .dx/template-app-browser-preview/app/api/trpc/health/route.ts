export const dynamic = "force-dynamic";

const noStoreHeaders = {
  "cache-control": "no-store",
} as const;

export function GET() {
  return Response.json(
    {
      schema: "dx.www.template.trpc_health",
      ok: true,
      status: "ready",
      route: "/api/trpc/health",
      packageId: "api/trpc",
      router: "trpc",
      procedure: "health",
      runtime: "dx-www-source-owned-route",
      runtimeProof: false,
      networkCalls: false,
      hostedCredentials: false,
      boundary:
        "Local Type-Safe API readiness only; production auth context, transport, subscriptions, persistence, and observability stay app-owned.",
    },
    { status: 200, headers: noStoreHeaders },
  );
}

export async function POST(request: Request) {
  const body = await readJsonBody(request);

  return Response.json(
    {
      schema: "dx.www.template.trpc_health",
      ok: true,
      status: "accepted",
      route: "/api/trpc/health",
      packageId: "api/trpc",
      router: "trpc",
      procedure: "launchEvent",
      payload: body,
      runtime: "dx-www-source-owned-route",
      runtimeProof: false,
      networkCalls: false,
      hostedCredentials: false,
      boundary:
        "Local launchEvent dry-run only; no hosted tRPC transport, subscription stream, or persistence is executed.",
    },
    { status: 202, headers: noStoreHeaders },
  );
}

async function readJsonBody(request: Request) {
  try {
    const value = await request.json();
    return value && typeof value === "object" && !Array.isArray(value)
      ? (value as Record<string, unknown>)
      : {};
  } catch {
    return {};
  }
}
