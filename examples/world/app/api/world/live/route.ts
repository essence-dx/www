export function GET() {
  return Response.json({
    schema: "dx.examples.world.live-connections",
    ok: true,
    status: "runner-ready",
    generatedBy: "examples/world",
    redaction: "secret-values-never-included",
    connectionRunner: "examples/world/lib/world/connections/runner.ts",
    runner: "examples/world/lib/world/connections/runner.ts",
    providerCatalog: "examples/world/lib/world/connections/providers/index.ts",
    receiptPath: "examples/world/.dx/receipts/world/live-connections.json",
    liveProviderExecution: "read-only-runner-when-env-present",
    routeHandlerBoundary:
      "WWW route handlers currently expose this runner contract; imported helper execution with fetch/env injection belongs in the root PLAN.md framework work.",
    nextAction: "Run the TypeScript live connection benchmark to write a redacted receipt from local credentials.",
  });
}
