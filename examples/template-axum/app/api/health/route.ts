export function GET() {
  return Response.json({
    ok: true,
    runtime: "axum",
    sourceOwned: true,
  });
}
