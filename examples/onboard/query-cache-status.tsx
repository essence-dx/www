"use client";

import * as React from "react";
import { useMutation, useQuery } from "@tanstack/react-query";

import {
  DxQueryActivityStatus,
  getDxLatestMutationState,
  useDxPendingMutationState,
  useDxQueryActivity,
} from "@/lib/query/activity";
import {
  countDxFetchingQueries,
  countDxMutatingRequests,
  readDxQueriesData,
  readDxQueryState,
} from "@/lib/query/cache";
import { summarizeDxQueryCache } from "@/lib/query/cache-events";
import {
  DX_QUERY_BROADCAST_CHANNEL,
  mountDxQueryBroadcastClient,
  readDxQueryBroadcastStatus,
  type DxQueryBroadcastHandle,
} from "@/lib/query/broadcast";
import { dxQueryOptions } from "@/lib/query/client";
import {
  resumeDxPausedMutations,
  summarizeDxQueryClientCaches,
} from "@/lib/query/client-lifecycle";
import {
  readDxQueryDefaults,
  setDxQueryDefaults,
  summarizeDxDefaultPolicies,
} from "@/lib/query/defaults";
import {
  DxQueryDevtools,
  readDxQueryDevtoolsStatus,
} from "@/lib/query/devtools";
import {
  dxConditionalQueryOptions,
  dxMaybeQueryFn,
} from "@/lib/query/disabled";
import {
  getDxQueryErrorKind,
  isDxCancelledQueryError,
} from "@/lib/query/errors";
import { fetchDxQueryData } from "@/lib/query/fetch";
import { summarizeDxHydrationState } from "@/lib/query/hydration";
import {
  DxQueryLifecycleStatus,
  setDxQueryFocused,
  setDxQueryOnline,
  useDxQueryLifecycleStatus,
} from "@/lib/query/lifecycle";
import {
  dxMutationOptions,
  invalidateDxQueries,
} from "@/lib/query/mutation";
import {
  formatDxMutationResultError,
  summarizeDxMutationResult,
} from "@/lib/query/mutation-result";
import {
  formatDxQueryResultError,
  summarizeDxQueryResult,
} from "@/lib/query/query-result";
import { summarizeDxCacheMatches } from "@/lib/query/matches";
import {
  DxReactQueryStreamedHydration,
  readDxQueryNextStreamingStatus,
} from "@/lib/query/next-streaming";
import {
  createDxQueryObserver,
  readDxQueryObserverSnapshot,
} from "@/lib/query/observers";
import {
  dxKeepPreviousData,
  readDxPlaceholderState,
  shareDxQueryData,
} from "@/lib/query/placeholder";
import {
  DX_QUERY_PERSIST_KEY,
  createDxBrowserQueryPersister,
  readDxPersistedQueryStatus,
} from "@/lib/query/persist";
import {
  readDxQueryClientContextStatus,
  useDxRequiredQueryClient,
} from "@/lib/query/react-context";
import {
  createDxBrowserSyncStoragePersister,
  readDxSyncStoragePersisterStatus,
} from "@/lib/query/sync-persist";
import { hashDxQueryKey } from "@/lib/query/keys";
import { DxQueryRestoreStatus } from "@/lib/query/restoring";
import { readDxQueryRuntimeStatus } from "@/lib/query/runtime";
import {
  findDxCachedMutation,
  findDxCachedQuery,
  summarizeDxMutationCoreState,
  summarizeDxQueryCoreState,
} from "@/lib/query/state";

import {
  readLaunchQueryDashboardData,
  type LaunchQueryDashboardData,
} from "./query-dashboard-read-model";

declare global {
  namespace JSX {
    interface IntrinsicElements {
      "dx-icon": React.HTMLAttributes<HTMLElement> & { name: string };
    }
  }
}

const launchCacheStatusQuery = dxQueryOptions({
  queryKey: ["dx", "launch", "dashboard", "read-model"] as const,
  queryFn: async () => readLaunchQueryDashboardData(),
  staleTime: 60_000,
});

type LaunchQueryPolicyId = "balanced" | "fast" | "durable";

const launchQueryPolicies = {
  balanced: {
    label: "Balanced",
    staleTime: 60_000,
    gcTime: 5 * 60_000,
    retry: 2,
  },
  fast: {
    label: "Fast refresh",
    staleTime: 5_000,
    gcTime: 60_000,
    retry: 1,
  },
  durable: {
    label: "Durable cache",
    staleTime: 5 * 60_000,
    gcTime: 30 * 60_000,
    retry: 3,
  },
} as const;

export function LaunchQueryCacheStatus() {
  const queryClient = useDxRequiredQueryClient();
  const [pausedMutationResumeState, setPausedMutationResumeState] =
    React.useState<"idle" | "resuming" | "resumed" | "error">("idle");
  const [launchCacheFetchState, setLaunchCacheFetchState] = React.useState<
    "idle" | "fetching" | "fetched" | "error"
  >("idle");
  const [showQueryDevtools, setShowQueryDevtools] = React.useState(false);
  const [launchQueryPolicy, setLaunchQueryPolicy] =
    React.useState<LaunchQueryPolicyId>("balanced");
  const [launchQueryBroadcastHandle, setLaunchQueryBroadcastHandle] =
    React.useState<DxQueryBroadcastHandle | null>(null);
  const status = useQuery(launchCacheStatusQuery);
  const launchQueryActivityFilters = React.useMemo(
    () => ({ queryKey: ["dx", "launch"] as const }),
    [],
  );
  const launchMutationActivityFilters = React.useMemo(
    () => ({ mutationKey: ["dx", "launch"] as const }),
    [],
  );
  const launchQueryActivity = useDxQueryActivity({
    queryFilters: launchQueryActivityFilters,
    mutationFilters: launchMutationActivityFilters,
    client: queryClient,
  });
  const launchPendingMutationStates = useDxPendingMutationState({
    filters: launchMutationActivityFilters,
    select: (mutation) => ({
      failureCount: mutation.state.failureCount,
      paused: mutation.state.isPaused,
      status: mutation.state.status,
      submittedAt: mutation.state.submittedAt,
    }),
    client: queryClient,
  });
  const launchLatestPendingMutationState = getDxLatestMutationState(
    launchPendingMutationStates,
  );
  const launchCachedQuery = findDxCachedQuery(queryClient, {
    queryKey: launchCacheStatusQuery.queryKey,
  });
  const launchCachedMutation = findDxCachedMutation(
    queryClient,
    launchMutationActivityFilters,
  );
  const launchQueryCoreStateSummary =
    summarizeDxQueryCoreState(launchCachedQuery);
  const launchMutationCoreStateSummary =
    summarizeDxMutationCoreState(launchCachedMutation);
  const launchQueryObserver = React.useMemo(
    () => createDxQueryObserver(queryClient, launchCacheStatusQuery),
    [queryClient],
  );
  const launchQueryPersistPersister = React.useMemo(
    () =>
      createDxBrowserQueryPersister({
        key: DX_QUERY_PERSIST_KEY,
        throttleTimeMs: 1_000,
      }),
    [],
  );
  const launchQuerySyncPersister = React.useMemo(
    () =>
      createDxBrowserSyncStoragePersister({
        key: `${DX_QUERY_PERSIST_KEY}:sync`,
        throttleTimeMs: 1_000,
      }),
    [],
  );
  const launchCacheKeyHash = hashDxQueryKey(launchCacheStatusQuery.queryKey);
  const launchCacheDiagnosticsQuery = dxConditionalQueryOptions({
    queryKey: [
      "dx",
      "launch",
      "query-cache-diagnostics",
      status.data?.status ?? "waiting",
    ] as const,
    queryFn: dxMaybeQueryFn(status.isSuccess, async () => ({
      source: "dependent-query",
      status: status.data?.status ?? "ok",
    })),
    placeholderData: dxKeepPreviousData,
    staleTime: 60_000,
    structuralSharing: shareDxQueryData,
  });
  const diagnostics = useQuery(launchCacheDiagnosticsQuery);
  const refreshStatus = useMutation(
    dxMutationOptions({
      mutationKey: ["dx", "launch", "query-cache-refresh"] as const,
      mutationFn: async () => readLaunchQueryDashboardData(),
      onSuccess: async () => {
        await invalidateDxQueries(queryClient, [
          launchCacheStatusQuery.queryKey,
        ]);
      },
    }),
  );
  const launchStatusQuerySummary = summarizeDxQueryResult(status);
  const launchStatusQueryErrorMessage = formatDxQueryResultError(status);
  const launchRefreshMutationSummary =
    summarizeDxMutationResult(refreshStatus);
  const launchRefreshMutationErrorMessage =
    formatDxMutationResultError(refreshStatus);
  const launchCacheEventSummary = summarizeDxQueryCache(queryClient);
  const launchQueryClientCaches = summarizeDxQueryClientCaches(queryClient);
  const launchQueryClientContextStatus =
    readDxQueryClientContextStatus(queryClient);
  const launchHydrationSummary = summarizeDxHydrationState(queryClient);
  const launchQueryAppliedDefaults = readDxQueryDefaults(
    queryClient,
    launchCacheStatusQuery.queryKey,
  );
  const launchQueryDefaultPolicies = summarizeDxDefaultPolicies(
    queryClient,
    ["dx", "launch"] as const,
    ["dx", "launch"] as const,
  );
  const launchQueryObserverSnapshot = readDxQueryObserverSnapshot(
    launchQueryObserver,
  );
  const launchCacheMatchSummary = summarizeDxCacheMatches(
    queryClient,
    { queryKey: ["dx", "launch"] as const },
    { mutationKey: ["dx", "launch"] as const },
  );
  const launchCacheSnapshotCount = readDxQueriesData(queryClient, {
    queryKey: ["dx", "launch"] as const,
  }).length;
  const launchCacheQueryState = readDxQueryState<LaunchQueryDashboardData>(
    queryClient,
    launchCacheStatusQuery.queryKey,
  );
  const launchCacheFetchingCount = countDxFetchingQueries(queryClient, {
    queryKey: ["dx", "launch"] as const,
  });
  const launchCacheMutatingCount = countDxMutatingRequests(queryClient, {
    mutationKey: ["dx", "launch"] as const,
  });
  const launchQueryRuntimeStatus = readDxQueryRuntimeStatus();
  const launchQueryLifecycleStatus = useDxQueryLifecycleStatus();
  const launchQueryPersistenceStatus = readDxPersistedQueryStatus({
    persister: launchQueryPersistPersister,
    key: DX_QUERY_PERSIST_KEY,
  });
  const launchQuerySyncPersistenceStatus = readDxSyncStoragePersisterStatus({
    persister: launchQuerySyncPersister,
    key: `${DX_QUERY_PERSIST_KEY}:sync`,
  });
  const launchQueryBroadcastStatus = readDxQueryBroadcastStatus(
    launchQueryBroadcastHandle,
    DX_QUERY_BROADCAST_CHANNEL,
  );
  const launchQueryDevtoolsStatus = readDxQueryDevtoolsStatus({
    enabled: showQueryDevtools,
    environment: "www-template",
    panel: "floating",
  });
  const launchQueryNextStreamingStatus = readDxQueryNextStreamingStatus({
    enabled: true,
    runtime: "next-app-router",
  });
  const launchQueryPlaceholderState = readDxPlaceholderState(
    diagnostics.data,
    diagnostics.isPlaceholderData,
  );
  const handleResumePausedMutations = React.useCallback(async () => {
    setPausedMutationResumeState("resuming");

    try {
      await resumeDxPausedMutations(queryClient);
      setPausedMutationResumeState("resumed");
    } catch {
      setPausedMutationResumeState("error");
    }
  }, [queryClient]);
  const handleFetchQueryNow = React.useCallback(async () => {
    setLaunchCacheFetchState("fetching");

    try {
      await fetchDxQueryData(queryClient, launchCacheStatusQuery);
      setLaunchCacheFetchState("fetched");
    } catch {
      setLaunchCacheFetchState("error");
    }
  }, [queryClient]);
  const handleSetQueryLifecycle = React.useCallback(
    (next: { focused?: boolean; online?: boolean }) => {
      if (typeof next.online === "boolean") {
        setDxQueryOnline(next.online);
      }

      if ("focused" in next) {
        setDxQueryFocused(next.focused);
      }
    },
    [],
  );
  const handleApplyQueryPolicy = React.useCallback(
    (policyId: LaunchQueryPolicyId) => {
      const policy = launchQueryPolicies[policyId];

      setDxQueryDefaults(queryClient, launchCacheStatusQuery.queryKey, {
        gcTime: policy.gcTime,
        retry: policy.retry,
        staleTime: policy.staleTime,
      });
      setLaunchQueryPolicy(policyId);
      void queryClient.invalidateQueries({
        queryKey: launchCacheStatusQuery.queryKey,
      });
    },
    [queryClient],
  );
  React.useEffect(() => {
    const broadcastHandle = mountDxQueryBroadcastClient({
      queryClient,
      channel: DX_QUERY_BROADCAST_CHANNEL,
    });

    setLaunchQueryBroadcastHandle(broadcastHandle);

    return () => broadcastHandle.close();
  }, [queryClient]);

  if (status.isPending) {
    return (
      <span
        data-query-state="loading"
        data-query-result-status={launchStatusQuerySummary.status}
        data-query-result-fetch-status={launchStatusQuerySummary.fetchStatus}
        data-query-result-can-refetch={String(
          launchStatusQuerySummary.canRefetch,
        )}
      >
        Dashboard data warming
      </span>
    );
  }

  if (status.isError) {
    const launchQueryErrorKind = getDxQueryErrorKind(status.error);
    const launchQueryErrorMessage = isDxCancelledQueryError(status.error)
      ? "Query request cancelled"
      : launchStatusQueryErrorMessage;

    return (
      <span
        data-query-state="error"
        data-query-error-kind={launchQueryErrorKind}
        data-query-result-status={launchStatusQuerySummary.status}
        data-query-result-fetch-status={launchStatusQuerySummary.fetchStatus}
        data-query-result-can-refetch={String(
          launchStatusQuerySummary.canRefetch,
        )}
        role="alert"
      >
        {launchQueryErrorMessage}
      </span>
    );
  }

  return (
    <div className="grid gap-3 text-sm">
      <span data-query-state="ready">
        Query cache ready: {status.data.status}
      </span>
      <section
        className="grid gap-3 rounded-md border border-border bg-card p-3 text-card-foreground"
        data-dx-component="tanstack-query-dashboard-data-workflow"
        data-dx-dashboard-workflow="query-backed-dashboard-data"
        data-dx-package="tanstack/query"
        data-dx-product-surface="launch-dashboard"
        data-dx-style-surface="data-fetching-cache"
        data-dx-query-dashboard-package-count={status.data.packageCount}
        data-dx-query-dashboard-required-env-count={
          status.data.requiredEnvCount
        }
        data-dx-query-dashboard-role-count={status.data.roleCount}
        data-dx-query-dashboard-source={status.data.source}
        data-dx-query-dashboard-updated-at={status.data.readAt}
      >
        <span className="flex items-center gap-2 font-medium">
          <dx-icon name="pack:tanstack-query" aria-hidden="true" className="size-4" />
          Query-backed dashboard data
        </span>
        <span className="text-xs text-muted-foreground">
          Launch package catalog, roles, environment gates, and app-owned
          boundaries are read through a cached Data Fetching &amp; Cache model.
        </span>
        <span className="grid gap-2 sm:grid-cols-4">
          <span className="rounded-md border border-border bg-muted/40 p-2">
            <span className="block text-xs text-muted-foreground">Packages</span>
            <span className="text-lg font-semibold text-foreground">
              {status.data.packageCount}
            </span>
          </span>
          <span className="rounded-md border border-border bg-muted/40 p-2">
            <span className="block text-xs text-muted-foreground">Roles</span>
            <span className="text-lg font-semibold text-foreground">
              {status.data.roleCount}
            </span>
          </span>
          <span className="rounded-md border border-border bg-muted/40 p-2">
            <span className="block text-xs text-muted-foreground">Env gates</span>
            <span className="text-lg font-semibold text-foreground">
              {status.data.requiredEnvCount}
            </span>
          </span>
          <span className="rounded-md border border-border bg-muted/40 p-2">
            <span className="block text-xs text-muted-foreground">
              App boundaries
            </span>
            <span className="text-lg font-semibold text-foreground">
              {status.data.appOwnedBoundaryCount}
            </span>
          </span>
        </span>
        <ul
          className="grid gap-2"
          data-dx-query-dashboard-queue="package-readiness"
        >
          {status.data.dashboardPackages.map((item) => (
            <li
              key={item.packageId}
              className="grid gap-1 rounded-md border border-border bg-muted/40 p-2 sm:grid-cols-[minmax(0,1fr)_auto_auto]"
              data-dx-query-dashboard-row={item.packageId}
              data-dx-query-package-boundary-count={item.appOwnedBoundaryCount}
              data-dx-query-package-command={item.command}
              data-dx-query-package-id={item.packageId}
              data-dx-query-package-name={item.displayName}
              data-dx-query-package-receipt-count={item.receiptCount}
              data-dx-query-package-role={item.role}
              data-dx-query-package-status={item.status}
            >
              <span className="grid gap-0.5">
                <span className="font-medium text-foreground">
                  {item.displayName}
                </span>
                <span className="text-xs text-muted-foreground">
                  {item.packageId} - {item.role}
                </span>
              </span>
              <span className="text-xs text-muted-foreground">
                {item.receiptCount} receipts
              </span>
              <span className="text-xs font-medium text-foreground">
                {item.status === "ready"
                  ? "Ready"
                  : `${item.requiredEnvCount} env gates`}
              </span>
            </li>
          ))}
        </ul>
      </section>
      <span
        className="grid gap-2 rounded-md border border-border bg-card p-3 text-card-foreground"
        data-dx-component="tanstack-query-dashboard-settings"
        data-dx-package="tanstack/query"
        data-query-dashboard-focus-state={
          launchQueryLifecycleStatus.focused ? "focused" : "background"
        }
        data-query-dashboard-online-state={
          launchQueryLifecycleStatus.online ? "online" : "offline"
        }
        data-query-dashboard-state={
          launchQueryLifecycleStatus.paused ? "paused" : "ready"
        }
        data-query-dashboard-workflow="settings-state-refresh"
      >
        <span className="flex items-center gap-2 font-medium">
          <dx-icon name="pack:tanstack-query" aria-hidden="true" className="size-4" />
          Dashboard query controls
        </span>
        <DxQueryLifecycleStatus
          className="text-xs text-muted-foreground"
          data-query-dashboard-lifecycle="focus-online"
        />
        <span
          aria-label="Data Fetching & Cache lifecycle controls"
          className="inline-flex flex-wrap gap-2"
        >
          <button
            type="button"
            className="rounded-md border border-input bg-background px-2 py-1 text-xs text-foreground transition-colors hover:bg-accent hover:text-accent-foreground data-[active=true]:bg-primary data-[active=true]:text-primary-foreground"
            data-active={launchQueryLifecycleStatus.online}
            data-query-dashboard-action="set-online"
            onClick={() => handleSetQueryLifecycle({ online: true })}
          >
            Online
          </button>
          <button
            type="button"
            className="rounded-md border border-input bg-background px-2 py-1 text-xs text-foreground transition-colors hover:bg-accent hover:text-accent-foreground data-[active=true]:bg-primary data-[active=true]:text-primary-foreground"
            data-active={!launchQueryLifecycleStatus.online}
            data-query-dashboard-action="set-offline"
            onClick={() => handleSetQueryLifecycle({ online: false })}
          >
            Offline mode
          </button>
          <button
            type="button"
            className="rounded-md border border-input bg-background px-2 py-1 text-xs text-foreground transition-colors hover:bg-accent hover:text-accent-foreground data-[active=true]:bg-primary data-[active=true]:text-primary-foreground"
            data-active={launchQueryLifecycleStatus.focused}
            data-query-dashboard-action="set-focused"
            onClick={() => handleSetQueryLifecycle({ focused: true })}
          >
            Focused
          </button>
          <button
            type="button"
            className="rounded-md border border-input bg-background px-2 py-1 text-xs text-foreground transition-colors hover:bg-accent hover:text-accent-foreground data-[active=true]:bg-primary data-[active=true]:text-primary-foreground"
            data-active={!launchQueryLifecycleStatus.focused}
            data-query-dashboard-action="set-background"
            onClick={() => handleSetQueryLifecycle({ focused: false })}
          >
            Background
          </button>
        </span>
        <span
          className="grid gap-2 rounded-md border border-border bg-muted/40 p-2"
          data-query-dashboard-policy={launchQueryPolicy}
          data-query-dashboard-policy-stale-time={String(
            launchQueryPolicies[launchQueryPolicy].staleTime,
          )}
          data-query-dashboard-policy-gc-time={String(
            launchQueryPolicies[launchQueryPolicy].gcTime,
          )}
          data-query-dashboard-policy-retry={String(
            launchQueryPolicies[launchQueryPolicy].retry,
          )}
          data-query-dashboard-policy-applied-stale-time={String(
            launchQueryAppliedDefaults.staleTime ?? "unset",
          )}
          data-query-dashboard-policy-applied-retry={String(
            launchQueryAppliedDefaults.retry ?? "unset",
          )}
        >
          <span className="text-xs font-medium text-foreground">
            Cache policy
          </span>
          <span className="text-xs text-muted-foreground">
            Applies Data Fetching &amp; Cache defaults for the launch dashboard read key.
          </span>
          <span className="inline-flex flex-wrap gap-2">
            <button
              type="button"
              className="rounded-md border border-input bg-background px-2 py-1 text-xs text-foreground transition-colors hover:bg-accent hover:text-accent-foreground data-[active=true]:bg-primary data-[active=true]:text-primary-foreground"
              data-active={launchQueryPolicy === "balanced"}
              data-query-dashboard-action="set-balanced-cache"
              onClick={() => handleApplyQueryPolicy("balanced")}
            >
              Balanced
            </button>
            <button
              type="button"
              className="rounded-md border border-input bg-background px-2 py-1 text-xs text-foreground transition-colors hover:bg-accent hover:text-accent-foreground data-[active=true]:bg-primary data-[active=true]:text-primary-foreground"
              data-active={launchQueryPolicy === "fast"}
              data-query-dashboard-action="set-fast-cache"
              onClick={() => handleApplyQueryPolicy("fast")}
            >
              Fast refresh
            </button>
            <button
              type="button"
              className="rounded-md border border-input bg-background px-2 py-1 text-xs text-foreground transition-colors hover:bg-accent hover:text-accent-foreground data-[active=true]:bg-primary data-[active=true]:text-primary-foreground"
              data-active={launchQueryPolicy === "durable"}
              data-query-dashboard-action="set-durable-cache"
              onClick={() => handleApplyQueryPolicy("durable")}
            >
              Durable cache
            </button>
          </span>
        </span>
      </span>
      <span
        className="sr-only"
        data-query-result-status={launchStatusQuerySummary.status}
        data-query-result-fetch-status={launchStatusQuerySummary.fetchStatus}
        data-query-result-can-refetch={String(
          launchStatusQuerySummary.canRefetch,
        )}
      >
        Query result: {launchStatusQuerySummary.status}
      </span>
      <span className="sr-only" data-query-key-hash={launchCacheKeyHash}>
        Query key hash: {launchCacheKeyHash}
      </span>
      <span
        className="sr-only"
        data-query-cache-matches={launchCacheSnapshotCount}
      >
        Launch cache matches: {launchCacheSnapshotCount}
      </span>
      <span
        className="sr-only"
        data-query-cache-size={launchCacheEventSummary.queries}
      >
        Query cache size: {launchCacheEventSummary.queries}
      </span>
      <span
        className="sr-only"
        data-query-client-cache-count={launchQueryClientCaches.queries}
        data-mutation-client-cache-count={launchQueryClientCaches.mutations}
      >
        QueryClient caches: {launchQueryClientCaches.state}
      </span>
      <span
        className="sr-only"
        data-query-client-context-state={
          launchQueryClientContextStatus.state
        }
        data-query-client-context-provider={
          launchQueryClientContextStatus.provider
        }
      >
        QueryClient context: {launchQueryClientContextStatus.provider}
      </span>
      <span
        className="sr-only"
        data-query-core-state={launchQueryCoreStateSummary.status}
        data-query-core-observers={launchQueryCoreStateSummary.observers}
        data-query-core-invalidated={String(
          launchQueryCoreStateSummary.invalidated,
        )}
      >
        Query core state: {launchQueryCoreStateSummary.status}
      </span>
      <span
        className="sr-only"
        data-mutation-core-state={launchMutationCoreStateSummary.status}
        data-mutation-core-failures={
          launchMutationCoreStateSummary.failureCount
        }
        data-mutation-core-paused={String(
          launchMutationCoreStateSummary.paused,
        )}
      >
        Mutation core state: {launchMutationCoreStateSummary.status}
      </span>
      <span
        className="sr-only"
        data-query-match-count={launchCacheMatchSummary.queries}
        data-mutation-match-count={launchCacheMatchSummary.mutations}
      >
        Query matches: {launchCacheMatchSummary.queries}
      </span>
      <span
        className="sr-only"
        data-query-hydration-queries={launchHydrationSummary.queries}
        data-query-hydration-state={launchHydrationSummary.state}
      >
        Hydration queries: {launchHydrationSummary.queries}
      </span>
      <span
        className="sr-only"
        data-query-default-retry={String(
          launchQueryDefaultPolicies.retry ?? "unset",
        )}
        data-query-default-stale-time={String(
          launchQueryDefaultPolicies.staleTime ?? "unset",
        )}
      >
        Query defaults: {String(launchQueryDefaultPolicies.retry ?? "unset")}
      </span>
      <span
        className="sr-only"
        data-query-observer-status={launchQueryObserverSnapshot.status}
        data-query-observer-fetch-status={
          launchQueryObserverSnapshot.fetchStatus
        }
      >
        Observer status: {launchQueryObserverSnapshot.status}
      </span>
      <DxQueryActivityStatus
        className="sr-only"
        queryFilters={launchQueryActivityFilters}
        mutationFilters={launchMutationActivityFilters}
        client={queryClient}
        data-query-activity-label={launchQueryActivity.label}
      />
      <span
        className="sr-only"
        data-query-activity-state={launchQueryActivity.busy ? "busy" : "idle"}
        data-query-activity-fetching={launchQueryActivity.fetching}
        data-query-activity-mutating={launchQueryActivity.mutating}
      >
        Query activity: {launchQueryActivity.label}
      </span>
      <span
        className="sr-only"
        data-query-pending-mutation-count={launchPendingMutationStates.length}
        data-query-latest-pending-mutation-status={
          launchLatestPendingMutationState?.status ?? "none"
        }
        data-query-latest-pending-mutation-paused={String(
          launchLatestPendingMutationState?.paused ?? false,
        )}
      >
        Pending mutations: {launchPendingMutationStates.length}
      </span>
      <span
        className="sr-only"
        data-query-fetching-count={launchCacheFetchingCount}
      >
        Fetching queries: {launchCacheFetchingCount}
      </span>
      <span
        className="sr-only"
        data-query-mutating-count={launchCacheMutatingCount}
      >
        Mutating requests: {launchCacheMutatingCount}
      </span>
      <span
        className="sr-only"
        data-query-data-updated-at={launchCacheQueryState?.dataUpdatedAt ?? 0}
      >
        Query data updated at: {launchCacheQueryState?.dataUpdatedAt ?? 0}
      </span>
      <span
        className="sr-only"
        data-query-runtime-env={launchQueryRuntimeStatus.environment}
        data-query-server-env={
          launchQueryRuntimeStatus.server ? "true" : "false"
        }
      >
        Query runtime: {launchQueryRuntimeStatus.environment}
      </span>
      <span
        className="sr-only"
        data-query-persist-storage={launchQueryPersistenceStatus.storage}
        data-query-persist-key={launchQueryPersistenceStatus.key}
        data-query-persist-restoring={String(
          launchQueryPersistenceStatus.restoring,
        )}
      >
        Query persistence: {launchQueryPersistenceStatus.storage}
      </span>
      <span
        className="sr-only"
        data-query-sync-persist-storage={
          launchQuerySyncPersistenceStatus.storage
        }
        data-query-sync-persist-key={launchQuerySyncPersistenceStatus.key}
        data-query-sync-persist-deprecated={String(
          launchQuerySyncPersistenceStatus.deprecated,
        )}
      >
        Query sync persistence: {launchQuerySyncPersistenceStatus.storage}
      </span>
      <span
        className="sr-only"
        data-query-broadcast-channel={launchQueryBroadcastStatus.channel}
        data-query-broadcast-connected={String(
          launchQueryBroadcastStatus.connected,
        )}
        data-query-broadcast-transport={launchQueryBroadcastStatus.transport}
      >
        Query broadcast: {launchQueryBroadcastStatus.transport}
      </span>
      <span
        className="sr-only"
        data-query-devtools-enabled={String(
          launchQueryDevtoolsStatus.enabled,
        )}
        data-query-devtools-env={launchQueryDevtoolsStatus.environment}
        data-query-devtools-panel={launchQueryDevtoolsStatus.panel}
      >
        Query devtools: {launchQueryDevtoolsStatus.panel}
      </span>
      <DxReactQueryStreamedHydration>
        <span
          className="sr-only"
          data-query-next-streaming-mode={launchQueryNextStreamingStatus.mode}
          data-query-next-streaming-runtime={
            launchQueryNextStreamingStatus.runtime
          }
          data-query-next-streaming-transformer={String(
            launchQueryNextStreamingStatus.transformer,
          )}
        >
          Query streamed hydration: {launchQueryNextStreamingStatus.mode}
        </span>
      </DxReactQueryStreamedHydration>
      <span
        className="sr-only"
        data-query-placeholder-state={launchQueryPlaceholderState.state}
        data-query-placeholder-source={launchQueryPlaceholderState.source}
      >
        Placeholder data: {launchQueryPlaceholderState.state}
      </span>
      <span
        className="sr-only"
        data-mutation-result-status={launchRefreshMutationSummary.status}
        data-mutation-result-can-reset={String(
          launchRefreshMutationSummary.canReset,
        )}
        data-mutation-result-failure-count={
          launchRefreshMutationSummary.failureCount
        }
      >
        Refresh mutation: {launchRefreshMutationSummary.status}
      </span>
      <button
        type="button"
        className="rounded-md border px-2 py-1 text-xs"
        data-query-refresh-state={
          refreshStatus.isPending
            ? "refreshing"
            : refreshStatus.isError
              ? "error"
              : "idle"
        }
        disabled={refreshStatus.isPending}
        data-dx-query-action="refresh-dashboard-data"
        onClick={() => refreshStatus.mutate()}
      >
        {refreshStatus.isPending ? "Refreshing data" : "Refresh dashboard data"}
      </button>
      <button
        type="button"
        className="rounded-md border px-2 py-1 text-xs"
        data-paused-mutation-resume-state={pausedMutationResumeState}
        disabled={pausedMutationResumeState === "resuming"}
        onClick={() => void handleResumePausedMutations()}
      >
        {pausedMutationResumeState === "resuming"
          ? "Resuming"
          : "Resume paused"}
      </button>
      <button
        type="button"
        className="rounded-md border px-2 py-1 text-xs"
        data-query-fetch-action-state={launchCacheFetchState}
        data-dx-query-action="fetch-dashboard-data-now"
        disabled={launchCacheFetchState === "fetching"}
        onClick={() => void handleFetchQueryNow()}
      >
        {launchCacheFetchState === "fetching"
          ? "Fetching data"
          : "Fetch dashboard data"}
      </button>
      <button
        type="button"
        className="rounded-md border px-2 py-1 text-xs"
        data-query-devtools-toggle-state={
          showQueryDevtools ? "visible" : "hidden"
        }
        onClick={() => setShowQueryDevtools((visible) => !visible)}
      >
        {showQueryDevtools ? "Hide devtools" : "Show devtools"}
      </button>
      {refreshStatus.isError ? (
        <span role="alert" className="text-xs text-destructive">
          {launchRefreshMutationErrorMessage}
        </span>
      ) : null}
      <DxQueryRestoreStatus
        className="text-xs"
        data-query-restore-slot="launch-cache"
      />
      <span
        className="text-xs"
        data-query-dependent-state={
          diagnostics.isSuccess
            ? "ready"
            : diagnostics.isError
              ? "error"
              : "waiting"
        }
      >
        Dashboard dependent query:{" "}
        {diagnostics.isSuccess ? diagnostics.data.status : "waiting"}
      </span>
      <DxQueryDevtools
        enabled={showQueryDevtools}
        environment="www-template"
        buttonPosition="bottom-right"
        position="bottom"
      />
    </div>
  );
}
