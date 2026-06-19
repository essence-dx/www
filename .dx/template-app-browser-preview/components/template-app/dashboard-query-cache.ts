import { defaultTemplateLockReality } from "./package-lock-reality.ts";

export type DashboardQueryCacheRuntimeBoundary = "source-owned-template-cache";
export type DashboardQueryAdapterBoundary = "queryclient-adapter-required";
export type DashboardQueryCacheEntryState = "fresh" | "optimistic" | "stale";

export type DashboardQueryCacheEntry = {
  readonly queryKey: string;
  readonly state: DashboardQueryCacheEntryState;
  readonly itemCount: number;
  readonly invalidated: boolean;
  readonly source: "launch-dashboard-template";
  readonly updatedAtLabel: string;
};

export type DashboardQueryCache = {
  readonly packageId: "tanstack/query";
  readonly runtimeBoundary: DashboardQueryCacheRuntimeBoundary;
  readonly upstreamAdapterBoundary: DashboardQueryAdapterBoundary;
  readonly entries: readonly DashboardQueryCacheEntry[];
};

export type DashboardQueryCacheSummary = {
  readonly cacheEntryCount: number;
  readonly readyEntryCount: number;
  readonly staleEntryCount: number;
  readonly invalidatedEntryCount: number;
  readonly optimisticEntryCount: number;
};

export type DashboardQueryCacheStatus = {
  readonly status: "source-owned-cache-readiness";
  readonly runtimeBoundary: DashboardQueryCacheRuntimeBoundary;
  readonly upstreamAdapterBoundary: DashboardQueryAdapterBoundary;
  readonly packageId: "tanstack/query";
  readonly queryKey: string;
  readonly cacheEntryCount: number;
  readonly readyEntryCount: number;
  readonly staleEntryCount: number;
  readonly invalidatedEntryCount: number;
  readonly optimisticEntryCount: number;
  readonly optimisticState: string;
  readonly lastReceiptState: string;
  readonly readinessLabel: string;
};

type DashboardQueryCacheInput = {
  readonly filter?: string;
  readonly lockBackedPackageCount?: number;
  readonly optimisticState: string;
  readonly lastReceiptState: string;
  readonly visibleProjectCount: number;
};

const DEFAULT_LOCK_BACKED_PACKAGE_COUNT = defaultTemplateLockReality.packageIds.length;

export function createDashboardQueryCacheKey(filter = "all") {
  return `dx:dashboard:projects:${filter}`;
}

export function createDashboardQueryCache(input: DashboardQueryCacheInput): DashboardQueryCache {
  const projectCount = Math.max(1, input.visibleProjectCount);
  const projectState =
    input.optimisticState === "queued" ? "optimistic" : "fresh";

  return {
    packageId: "tanstack/query",
    runtimeBoundary: "source-owned-template-cache",
    upstreamAdapterBoundary: "queryclient-adapter-required",
    entries: [
      {
        queryKey: createDashboardQueryCacheKey(input.filter),
        state: projectState,
        itemCount: projectCount,
        invalidated: false,
        source: "launch-dashboard-template",
        updatedAtLabel: input.lastReceiptState,
      },
      {
        queryKey: "dx:forge:package-reality",
        state: input.optimisticState === "applied" ? "fresh" : "optimistic",
        itemCount: input.lockBackedPackageCount ?? DEFAULT_LOCK_BACKED_PACKAGE_COUNT,
        invalidated: false,
        source: "launch-dashboard-template",
        updatedAtLabel: "Package status ready",
      },
    ],
  };
}

export function invalidateDashboardQueryCache(
  cache: DashboardQueryCache,
  queryKey: string,
): DashboardQueryCache {
  return {
    ...cache,
    entries: cache.entries.map((entry) =>
      entry.queryKey === queryKey
        ? { ...entry, state: "stale", invalidated: true }
        : entry,
    ),
  };
}

export function refreshDashboardQueryCache(
  cache: DashboardQueryCache,
  queryKey: string,
): DashboardQueryCache {
  return {
    ...cache,
    entries: cache.entries.map((entry) =>
      entry.queryKey === queryKey
        ? { ...entry, state: "fresh", invalidated: false }
        : entry,
    ),
  };
}

export function summarizeDashboardQueryCache(
  cache: DashboardQueryCache,
): DashboardQueryCacheSummary {
  const invalidatedEntryCount = cache.entries.filter((entry) => entry.invalidated).length;
  const staleEntryCount = cache.entries.filter((entry) => entry.state === "stale").length;
  const optimisticEntryCount = cache.entries.filter(
    (entry) => !entry.invalidated && entry.state === "optimistic",
  ).length;
  const readyEntryCount = cache.entries.filter(
    (entry) => !entry.invalidated && entry.state !== "stale",
  ).length;

  return {
    cacheEntryCount: cache.entries.length,
    readyEntryCount,
    staleEntryCount,
    invalidatedEntryCount,
    optimisticEntryCount,
  };
}

export function createDashboardQueryCacheStatus(
  input: DashboardQueryCacheInput,
): DashboardQueryCacheStatus {
  const cache = createDashboardQueryCache(input);
  const summary = summarizeDashboardQueryCache(cache);

  return {
    status: "source-owned-cache-readiness",
    runtimeBoundary: cache.runtimeBoundary,
    upstreamAdapterBoundary: cache.upstreamAdapterBoundary,
    packageId: cache.packageId,
    queryKey: cache.entries[0]?.queryKey ?? createDashboardQueryCacheKey(input.filter),
    cacheEntryCount: summary.cacheEntryCount,
    readyEntryCount: summary.readyEntryCount,
    staleEntryCount: summary.staleEntryCount,
    invalidatedEntryCount: summary.invalidatedEntryCount,
    optimisticEntryCount: summary.optimisticEntryCount,
    optimisticState: input.optimisticState,
    lastReceiptState: input.lastReceiptState,
    readinessLabel:
      input.optimisticState === "applied"
        ? "Local cache update applied"
        : "Local cache ready",
  };
}
