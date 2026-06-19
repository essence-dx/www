export async function POST(request: Request) {
  const body = await request.json();
  return {
    ok: true,
    status: 202,
    provider: "vercel-ai-compatible",
    message: body.message,
    appOwnedBoundary: "Set AI provider credentials in the app environment to stream model output.",
  };
}
