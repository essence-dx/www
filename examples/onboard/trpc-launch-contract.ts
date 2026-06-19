export const trpcLaunchContract = {
  packageId: "api/trpc",
  officialName: "Type-Safe API",
  upstreamPackage: "@trpc/server",
  route: "/api/trpc/[trpc]",
  endpoint: "/api/trpc",
  procedures: {
    health: "health.query",
    launchEvent: "launchEvent.mutation",
  },
  sourceApis: [
    "initTRPC.create",
    "fetchRequestHandler",
    "trpc.health.queryOptions()",
    "trpc.launchEvent.mutationOptions()",
    "trpc.health.queryFilter()",
  ],
} as const;

export type LaunchEventResult = {
  cacheAction: "invalidate health.query";
  event: "validated";
  procedure: typeof trpcLaunchContract.procedures.launchEvent;
  requestId: string;
  route: "/";
  status: "accepted";
};

export type HealthCheckResult = {
  cacheAction: "hydrate health.query";
  procedure: typeof trpcLaunchContract.procedures.health;
  requestId: string;
  route: "/api/trpc/health";
  status: "ready";
};

export type TrpcLaunchWorkflowResult = HealthCheckResult | LaunchEventResult;

export function createLocalHealthCheck(sequence: number): HealthCheckResult {
  return {
    cacheAction: "hydrate health.query",
    procedure: trpcLaunchContract.procedures.health,
    requestId: `dx-trpc-health-${String(sequence).padStart(2, "0")}`,
    route: "/api/trpc/health",
    status: "ready",
  };
}

export function createLocalLaunchEvent(sequence: number): LaunchEventResult {
  return {
    cacheAction: "invalidate health.query",
    event: "validated",
    procedure: trpcLaunchContract.procedures.launchEvent,
    requestId: `dx-trpc-local-${String(sequence).padStart(2, "0")}`,
    route: "/",
    status: "accepted",
  };
}
