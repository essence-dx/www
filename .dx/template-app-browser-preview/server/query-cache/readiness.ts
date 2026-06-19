import {
  createDashboardQueryCache,
  createDashboardQueryCacheStatus,
  invalidateDashboardQueryCache,
  refreshDashboardQueryCache,
  summarizeDashboardQueryCache,
  type DashboardQueryCacheSummary,
  type DashboardQueryCacheStatus,
} from "../../components/template-app/dashboard-query-cache.ts";

export const allowedDataFetchingCacheActions = ["invalidate", "refresh"] as const;
export type DataFetchingCacheAction =
  (typeof allowedDataFetchingCacheActions)[number];

export type DataFetchingCacheReadinessInput = {
  readonly filter?: string;
  readonly optimisticState?: string;
  readonly visibleProjectCount?: number;
};

export type DataFetchingCacheReadiness = {
  readonly schema: "dx.www.template.data_fetching_cache_readiness";
  readonly laneNumber: 3;
  readonly laneName: "State + Data Fetching";
  readonly route: "/api/query-cache/readiness";
  readonly packageId: "tanstack/query";
  readonly officialPackageName: "Data Fetching & Cache";
  readonly upstreamPackage: "@tanstack/react-query";
  readonly status: "source-owned-cache-readiness";
  readonly runtimeProof: false;
  readonly networkCalls: false;
  readonly nodeModulesRequired: false;
  readonly adapterBoundary: "queryclient-adapter-required";
  readonly cache: DashboardQueryCacheStatus;
  readonly appRouterRoutes: readonly string[];
  readonly serverFiles: readonly string[];
  readonly frontFacingFiles: readonly string[];
  readonly appOwnedBoundary: readonly string[];
};

export type DataFetchingCacheActionInput = DataFetchingCacheReadinessInput & {
  readonly action?: string;
  readonly queryKey?: string;
};

export type DataFetchingCacheActionReceipt = {
  readonly schema: "dx.www.template.data_fetching_cache_action_receipt";
  readonly route: "/api/query-cache/readiness";
  readonly packageId: "tanstack/query";
  readonly officialPackageName: "Data Fetching & Cache";
  readonly upstreamPackage: "@tanstack/react-query";
  readonly status: "source-owned-cache-action-dry-run";
  readonly action: DataFetchingCacheAction;
  readonly queryKey: string;
  readonly runtimeProof: false;
  readonly networkCalls: false;
  readonly nodeModulesRequired: false;
  readonly queryClientExecution: false;
  readonly adapterBoundary: "queryclient-adapter-required";
  readonly cache: DashboardQueryCacheSummary;
  readonly secretValues: readonly [];
  readonly appOwnedBoundary: readonly string[];
};

export type DataFetchingCacheActionErrorReceipt = {
  readonly schema: "dx.www.template.data_fetching_cache_action_error";
  readonly route: "/api/query-cache/readiness";
  readonly packageId: "tanstack/query";
  readonly officialPackageName: "Data Fetching & Cache";
  readonly upstreamPackage: "@tanstack/react-query";
  readonly status: "unsupported-cache-action";
  readonly action: string;
  readonly allowedActions: readonly DataFetchingCacheAction[];
  readonly queryKey: string;
  readonly runtimeProof: false;
  readonly networkCalls: false;
  readonly nodeModulesRequired: false;
  readonly queryClientExecution: false;
  readonly adapterBoundary: "queryclient-adapter-required";
  readonly secretValues: readonly [];
  readonly appOwnedBoundary: readonly string[];
};

function isDataFetchingCacheAction(
  action: string | undefined,
): action is DataFetchingCacheAction {
  return action === "invalidate" || action === "refresh";
}

function inputFromRequest(request?: Request): DataFetchingCacheReadinessInput {
  if (!request) return {};

  const url = new URL(request.url);
  const visible = Number(url.searchParams.get("visible") ?? "");

  return {
    filter: url.searchParams.get("filter") ?? undefined,
    optimisticState: url.searchParams.get("optimistic") ?? undefined,
    visibleProjectCount: Number.isFinite(visible) && visible > 0 ? visible : undefined,
  };
}

export function readDataFetchingCacheReadiness(
  input: DataFetchingCacheReadinessInput = {},
): DataFetchingCacheReadiness {
  const cache = createDashboardQueryCacheStatus({
    filter: input.filter ?? "all",
    optimisticState: input.optimisticState ?? "idle",
    lastReceiptState: "App Router readiness route",
    visibleProjectCount: input.visibleProjectCount ?? 4,
  });

  return {
    schema: "dx.www.template.data_fetching_cache_readiness",
    laneNumber: 3,
    laneName: "State + Data Fetching",
    route: "/api/query-cache/readiness",
    packageId: "tanstack/query",
    officialPackageName: "Data Fetching & Cache",
    upstreamPackage: "@tanstack/react-query",
    status: cache.status,
    runtimeProof: false,
    networkCalls: false,
    nodeModulesRequired: false,
    adapterBoundary: cache.upstreamAdapterBoundary,
    cache,
    appRouterRoutes: ["app/api/query-cache/readiness/route.ts"],
    serverFiles: ["server/query-cache/readiness.ts"],
    frontFacingFiles: [
      "components/template-app/dashboard-query-cache.ts",
      "server/query-cache/readiness.ts",
      "app/api/query-cache/readiness/route.ts",
    ],
    appOwnedBoundary: [
      "live QueryClient provider",
      "production dashboard fetchers",
      "cache persistence storage",
      "broadcast channel naming",
      "browser runtime proof",
      "stateful QueryClient mutation execution",
    ],
  };
}

export function createDataFetchingCacheReadinessResponse(request?: Request): Response {
  return Response.json(readDataFetchingCacheReadiness(inputFromRequest(request)), {
    status: 200,
    headers: {
      "cache-control": "no-store",
    },
  });
}

export function readDataFetchingCacheActionReceipt(
  input: DataFetchingCacheActionInput = {},
): DataFetchingCacheActionReceipt {
  const baseCache = createDashboardQueryCache({
    filter: input.filter ?? "all",
    optimisticState: input.optimisticState ?? "idle",
    lastReceiptState: "App Router dry-run action",
    visibleProjectCount: input.visibleProjectCount ?? 4,
  });
  const queryKey =
    input.queryKey ?? baseCache.entries[0]?.queryKey ?? "dx:dashboard:projects:all";
  const action = isDataFetchingCacheAction(input.action)
    ? input.action
    : "invalidate";
  const nextCache =
    action === "refresh"
      ? refreshDashboardQueryCache(baseCache, queryKey)
      : invalidateDashboardQueryCache(baseCache, queryKey);

  return {
    schema: "dx.www.template.data_fetching_cache_action_receipt",
    route: "/api/query-cache/readiness",
    packageId: "tanstack/query",
    officialPackageName: "Data Fetching & Cache",
    upstreamPackage: "@tanstack/react-query",
    status: "source-owned-cache-action-dry-run",
    action,
    queryKey,
    runtimeProof: false,
    networkCalls: false,
    nodeModulesRequired: false,
    queryClientExecution: false,
    adapterBoundary: baseCache.upstreamAdapterBoundary,
    cache: summarizeDashboardQueryCache(nextCache),
    secretValues: [],
    appOwnedBoundary: [
      "live QueryClient provider",
      "production dashboard fetchers",
      "cache persistence storage",
      "broadcast channel naming",
      "browser runtime proof",
      "stateful QueryClient mutation execution",
    ],
  };
}

export function readDataFetchingCacheActionErrorReceipt(
  input: DataFetchingCacheActionInput = {},
): DataFetchingCacheActionErrorReceipt {
  const baseCache = createDashboardQueryCache({
    filter: input.filter ?? "all",
    optimisticState: input.optimisticState ?? "idle",
    lastReceiptState: "Unsupported App Router dry-run action",
    visibleProjectCount: input.visibleProjectCount ?? 4,
  });
  const queryKey =
    input.queryKey ?? baseCache.entries[0]?.queryKey ?? "dx:dashboard:projects:all";

  return {
    schema: "dx.www.template.data_fetching_cache_action_error",
    route: "/api/query-cache/readiness",
    packageId: "tanstack/query",
    officialPackageName: "Data Fetching & Cache",
    upstreamPackage: "@tanstack/react-query",
    status: "unsupported-cache-action",
    action: input.action ?? "missing",
    allowedActions: allowedDataFetchingCacheActions,
    queryKey,
    runtimeProof: false,
    networkCalls: false,
    nodeModulesRequired: false,
    queryClientExecution: false,
    adapterBoundary: baseCache.upstreamAdapterBoundary,
    secretValues: [],
    appOwnedBoundary: [
      "live QueryClient provider",
      "production dashboard fetchers",
      "cache persistence storage",
      "broadcast channel naming",
      "browser runtime proof",
      "stateful QueryClient mutation execution",
    ],
  };
}

export function createDataFetchingCacheActionErrorResponse(
  input: DataFetchingCacheActionInput = {},
): Response {
  return Response.json(readDataFetchingCacheActionErrorReceipt(input), {
    status: 400,
    headers: {
      "cache-control": "no-store",
    },
  });
}

export async function createDataFetchingCacheActionResponse(
  request?: Request,
): Promise<Response> {
  const body = request ? await readRequestBody(request) : {};
  const urlInput = inputFromRequest(request);
  const input = {
    ...urlInput,
    ...body,
  };
  if (typeof input.action === "string" && !isDataFetchingCacheAction(input.action)) {
    return createDataFetchingCacheActionErrorResponse(input);
  }

  return Response.json(readDataFetchingCacheActionReceipt(input), {
    status: 200,
    headers: {
      "cache-control": "no-store",
    },
  });
}

async function readRequestBody(request: Request): Promise<DataFetchingCacheActionInput> {
  try {
    const value = await request.json();
    if (!value || typeof value !== "object" || Array.isArray(value)) {
      return {};
    }

    return value as DataFetchingCacheActionInput;
  } catch {
    return {};
  }
}
