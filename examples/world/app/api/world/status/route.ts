export function GET() {
  return Response.json({
    schema: "dx.examples.world.status",
    ok: true,
    status: "source-owned-preview",
    generatedBy: "examples/world",
    redaction: "secret-values-never-included",
    categoryCount: 20,
    providerCount: 60,
    providerCatalog: "examples/world/lib/world/registry.ts",
    connectionRunner: "examples/world/lib/world/connections/runner.ts",
    liveConnectionRoute: "/api/world/live",
    routeContractCatalog: "examples/world/lib/world/routes.ts",
    liveProviderExecution: "read-only-runner-when-env-present",
    nextAction: "Use DX Env Firewall values, run the TypeScript connection runner, then import the redacted receipt.",
  });
}
