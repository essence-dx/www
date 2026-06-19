export function GET() {
  return {
    ok: true,
    status: "ready",
    router: "trpc",
    procedure: "health",
    runtime: "dx-www-source-owned-route",
  };
}

export async function POST(request: Request) {
  const body = await request.json();
  return {
    ok: true,
    status: 202,
    router: "trpc",
    procedure: "launchEvent",
    payload: body,
  };
}
