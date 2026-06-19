pub(super) const TANSTACK_QUERY_VERSION: &str = "5.100.10-dx.0";

pub(super) fn tanstack_query_templates() -> Vec<(&'static str, &'static str)> {
    vec![
        ("js/query/client.ts", TANSTACK_QUERY_CLIENT_TS),
        (
            "js/query/client-lifecycle.ts",
            TANSTACK_QUERY_CLIENT_LIFECYCLE_TS,
        ),
        ("js/query/defaults.ts", TANSTACK_QUERY_DEFAULTS_TS),
        (
            "js/query/dashboard-workflow.ts",
            TANSTACK_QUERY_DASHBOARD_WORKFLOW_TS,
        ),
        ("js/query/provider.tsx", TANSTACK_QUERY_PROVIDER_TSX),
        (
            "js/query/react-context.tsx",
            TANSTACK_QUERY_REACT_CONTEXT_TSX,
        ),
        ("js/query/restoring.tsx", TANSTACK_QUERY_RESTORING_TSX),
        ("js/query/persist.tsx", TANSTACK_QUERY_PERSIST_TSX),
        ("js/query/sync-persist.ts", TANSTACK_QUERY_SYNC_PERSIST_TS),
        ("js/query/devtools.tsx", TANSTACK_QUERY_DEVTOOLS_TSX),
        ("js/query/broadcast.ts", TANSTACK_QUERY_BROADCAST_TS),
        (
            "js/query/next-streaming.tsx",
            TANSTACK_QUERY_NEXT_STREAMING_TSX,
        ),
        ("js/query/fetch.ts", TANSTACK_QUERY_FETCH_TS),
        ("js/query/prefetch.tsx", TANSTACK_QUERY_PREFETCH_TSX),
        ("js/query/hydration.ts", TANSTACK_QUERY_HYDRATION_TS),
        ("js/query/mutation.ts", TANSTACK_QUERY_MUTATION_TS),
        (
            "js/query/mutation-result.ts",
            TANSTACK_QUERY_MUTATION_RESULT_TS,
        ),
        ("js/query/query-result.ts", TANSTACK_QUERY_RESULT_TS),
        ("js/query/disabled.ts", TANSTACK_QUERY_DISABLED_TS),
        ("js/query/placeholder.ts", TANSTACK_QUERY_PLACEHOLDER_TS),
        ("js/query/keys.ts", TANSTACK_QUERY_KEYS_TS),
        ("js/query/state.ts", TANSTACK_QUERY_STATE_TS),
        ("js/query/infinite.ts", TANSTACK_QUERY_INFINITE_TS),
        ("js/query/activity.tsx", TANSTACK_QUERY_ACTIVITY_TSX),
        ("js/query/observers.ts", TANSTACK_QUERY_OBSERVERS_TS),
        ("js/query/cache.ts", TANSTACK_QUERY_CACHE_TS),
        ("js/query/cache-events.ts", TANSTACK_QUERY_CACHE_EVENTS_TS),
        ("js/query/matches.ts", TANSTACK_QUERY_MATCHES_TS),
        ("js/query/queries.tsx", TANSTACK_QUERY_QUERIES_TSX),
        ("js/query/lifecycle.tsx", TANSTACK_QUERY_LIFECYCLE_TSX),
        ("js/query/runtime.ts", TANSTACK_QUERY_RUNTIME_TS),
        ("js/query/stream.ts", TANSTACK_QUERY_STREAM_TS),
        ("js/query/suspense.tsx", TANSTACK_QUERY_SUSPENSE_TSX),
        (
            "js/query/prefetch-hooks.tsx",
            TANSTACK_QUERY_PREFETCH_HOOKS_TSX,
        ),
        ("js/query/errors.ts", TANSTACK_QUERY_ERRORS_TS),
        (
            "js/query/error-boundary.tsx",
            TANSTACK_QUERY_ERROR_BOUNDARY_TSX,
        ),
        ("js/query/metadata.ts", TANSTACK_QUERY_METADATA_TS),
        ("js/query/README.md", TANSTACK_QUERY_README_MD),
    ]
}

const TANSTACK_QUERY_CLIENT_TS: &str = r#"import {
  QueryClient,
  queryOptions,
  type DefaultOptions,
  type QueryFunction,
  type QueryKey,
} from "@tanstack/react-query";

export const DX_QUERY_DEFAULT_STALE_TIME_MS = 60_000;
export const DX_QUERY_DEFAULT_GC_TIME_MS = 5 * 60_000;

export type DxQueryClientConfig = {
  defaultOptions?: DefaultOptions;
  staleTimeMs?: number;
  gcTimeMs?: number;
  retry?: number | false;
};

export function createDxQueryClient(config: DxQueryClientConfig = {}) {
  const queryDefaults = config.defaultOptions?.queries ?? {};

  return new QueryClient({
    defaultOptions: {
      ...config.defaultOptions,
      queries: {
        staleTime: config.staleTimeMs ?? DX_QUERY_DEFAULT_STALE_TIME_MS,
        gcTime: config.gcTimeMs ?? DX_QUERY_DEFAULT_GC_TIME_MS,
        retry: config.retry ?? 2,
        refetchOnWindowFocus: false,
        ...queryDefaults,
      },
    },
  });
}

let browserQueryClient: QueryClient | undefined;

export function getDxBrowserQueryClient(config?: DxQueryClientConfig) {
  if (typeof window === "undefined") {
    return createDxQueryClient(config);
  }

  browserQueryClient ??= createDxQueryClient(config);
  return browserQueryClient;
}

export type DxQueryOptionsInput<
  TQueryFnData,
  TQueryKey extends QueryKey,
> = {
  queryKey: TQueryKey;
  queryFn: QueryFunction<TQueryFnData, TQueryKey>;
  staleTime?: number;
  gcTime?: number;
  enabled?: boolean;
};

export function dxQueryOptions<
  TQueryFnData,
  TQueryKey extends QueryKey,
>(input: DxQueryOptionsInput<TQueryFnData, TQueryKey>) {
  return queryOptions(input);
}
"#;

const TANSTACK_QUERY_CLIENT_LIFECYCLE_TS: &str = r#"import {
  type MutationCache,
  type QueryCache,
  type QueryClient,
} from "@tanstack/react-query";

export type DxQueryClientCaches = {
  queryCache: QueryCache;
  mutationCache: MutationCache;
};

export type DxQueryClientCacheSummary = {
  queries: number;
  mutations: number;
  staleQueries: number;
  pausedMutations: number;
  state: "empty" | "ready" | "paused";
};

export function mountDxQueryClient(client: QueryClient) {
  client.mount();

  let mounted = true;

  return () => {
    if (!mounted) return;
    mounted = false;
    client.unmount();
  };
}

export function unmountDxQueryClient(client: QueryClient) {
  client.unmount();
  return client;
}

export function clearDxQueryClient(client: QueryClient) {
  client.clear();
  return client;
}

export function resumeDxPausedMutations(client: QueryClient) {
  return client.resumePausedMutations();
}

export function readDxQueryClientCaches(
  client: QueryClient,
): DxQueryClientCaches {
  return {
    queryCache: client.getQueryCache(),
    mutationCache: client.getMutationCache(),
  };
}

export function summarizeDxQueryClientCaches(
  client: QueryClient,
): DxQueryClientCacheSummary {
  const { queryCache, mutationCache } = readDxQueryClientCaches(client);
  const queries = queryCache.getAll();
  const mutations = mutationCache.getAll();
  const staleQueries = queries.filter((query) => query.isStale()).length;
  const pausedMutations = mutations.filter(
    (mutation) => mutation.state.isPaused,
  ).length;

  return {
    queries: queries.length,
    mutations: mutations.length,
    staleQueries,
    pausedMutations,
    state:
      pausedMutations > 0
        ? "paused"
        : queries.length + mutations.length > 0
          ? "ready"
          : "empty",
  };
}
"#;

const TANSTACK_QUERY_DEFAULTS_TS: &str = r#"import {
  type DefaultError,
  type DefaultOptions,
  type MutationKey,
  type MutationObserverOptions,
  type QueryClient,
  type QueryKey,
  type QueryObserverOptions,
} from "@tanstack/react-query";

export type DxDefaultOptions<TError = DefaultError> =
  DefaultOptions<TError>;

export type DxQueryDefaultOptions<
  TQueryFnData = unknown,
  TError = DefaultError,
  TData = TQueryFnData,
  TQueryData = TQueryFnData,
  TQueryKey extends QueryKey = QueryKey,
> = Omit<
  QueryObserverOptions<
    TQueryFnData,
    TError,
    TData,
    TQueryData,
    TQueryKey
  >,
  "queryKey"
>;

export type DxMutationDefaultOptions<
  TData = unknown,
  TError = DefaultError,
  TVariables = void,
  TOnMutateResult = unknown,
> = Omit<
  MutationObserverOptions<TData, TError, TVariables, TOnMutateResult>,
  "mutationKey"
>;

export type DxDefaultPolicySummary = {
  hasQueryDefaults: boolean;
  hasMutationDefaults: boolean;
  retry: unknown;
  staleTime: unknown;
  gcTime: unknown;
  mutationRetry: unknown;
};

export function readDxDefaultOptions<TError = DefaultError>(
  client: QueryClient,
): DxDefaultOptions<TError> {
  return client.getDefaultOptions() as DxDefaultOptions<TError>;
}

export function applyDxDefaultOptions<TError = DefaultError>(
  client: QueryClient,
  options: DxDefaultOptions<TError>,
) {
  client.setDefaultOptions(options);
  return client;
}

export function setDxQueryDefaults<
  TQueryFnData = unknown,
  TError = DefaultError,
  TData = TQueryFnData,
  TQueryData = TQueryFnData,
  TQueryKey extends QueryKey = QueryKey,
>(
  client: QueryClient,
  queryKey: TQueryKey,
  options: DxQueryDefaultOptions<
    TQueryFnData,
    TError,
    TData,
    TQueryData,
    TQueryKey
  >,
) {
  client.setQueryDefaults(queryKey, options);
  return client;
}

export function readDxQueryDefaults<TQueryKey extends QueryKey = QueryKey>(
  client: QueryClient,
  queryKey: TQueryKey,
) {
  return client.getQueryDefaults(queryKey);
}

export function setDxMutationDefaults<
  TData = unknown,
  TError = DefaultError,
  TVariables = void,
  TOnMutateResult = unknown,
>(
  client: QueryClient,
  mutationKey: MutationKey,
  options: DxMutationDefaultOptions<
    TData,
    TError,
    TVariables,
    TOnMutateResult
  >,
) {
  client.setMutationDefaults(mutationKey, options);
  return client;
}

export function readDxMutationDefaults(
  client: QueryClient,
  mutationKey: MutationKey,
) {
  return client.getMutationDefaults(mutationKey);
}

export function summarizeDxDefaultPolicies(
  client: QueryClient,
  queryKey: QueryKey,
  mutationKey: MutationKey,
): DxDefaultPolicySummary {
  const defaults = readDxDefaultOptions(client);
  const queryDefaults = readDxQueryDefaults(client, queryKey);
  const mutationDefaults = readDxMutationDefaults(client, mutationKey);

  return {
    hasQueryDefaults: Object.keys(queryDefaults).length > 0,
    hasMutationDefaults: Object.keys(mutationDefaults).length > 0,
    retry: queryDefaults.retry ?? defaults.queries?.retry,
    staleTime: queryDefaults.staleTime ?? defaults.queries?.staleTime,
    gcTime: queryDefaults.gcTime ?? defaults.queries?.gcTime,
    mutationRetry: mutationDefaults.retry ?? defaults.mutations?.retry,
  };
}
"#;

const TANSTACK_QUERY_DASHBOARD_WORKFLOW_TS: &str = r#"import {
  type QueryClient,
  type QueryKey,
} from "@tanstack/react-query";

import {
  createDxQueryClient,
  type DxQueryClientConfig,
} from "./client";
import { summarizeDxQueryClientCaches } from "./client-lifecycle";
import {
  readDxQueryDefaults,
  setDxQueryDefaults,
  type DxQueryDefaultOptions,
} from "./defaults";
import { invalidateDxQueries } from "./mutation";

export type DxDashboardQueryProfileId = "balanced" | "live" | "durable";
export type DxDashboardQueryPublicApi =
  | "setQueryDefaults"
  | "getQueryDefaults"
  | "invalidateQueries";
export type DxDashboardQueryDxCheckStatus =
  | "present"
  | "stale"
  | "missing-receipt"
  | "blocked"
  | "unsupported-surface";

export type DxDashboardQueryDxCheckVisibility = {
  schema: "dx.forge.package.dx_check_visibility";
  packageId: "tanstack/query";
  officialPackageName: "Data Fetching & Cache";
  currentStatus: DxDashboardQueryDxCheckStatus;
  statuses: readonly DxDashboardQueryDxCheckStatus[];
  receiptPath: typeof DX_DASHBOARD_QUERY_RECEIPT_PATH;
  monitoredSurfaces: readonly string[];
  blockedSurfaces: readonly string[];
  unsupportedSurfaces: readonly string[];
};

export type DxDashboardQueryProfile = {
  id: DxDashboardQueryProfileId;
  label: string;
  publicApi: DxDashboardQueryPublicApi;
  staleTime: number;
  gcTime: number;
  retry: number | false;
  dataFreshness: "balanced" | "live" | "durable";
};

export type DxDashboardQueryReadiness = {
  packageId: "tanstack/query";
  queryKey: QueryKey;
  profileId: DxDashboardQueryProfileId;
  staleTime: unknown;
  gcTime: unknown;
  retry: unknown;
  cacheState: "empty" | "ready" | "paused";
  cachedQueries: number;
  staleQueries: number;
};

export type DxDashboardQueryReceipt = {
  cacheDefaults: {
    gcTime: number;
    retry: number | false;
    staleTime: number;
  };
  dashboardWorkflow: "query-backed-dashboard-data";
  dxCheckVisibility: DxDashboardQueryDxCheckVisibility;
  nodeModulesRequired: false;
  officialName: "Data Fetching & Cache";
  packageId: "tanstack/query";
  publicApi: DxDashboardQueryPublicApi;
  queryKey: QueryKey;
  profileId: DxDashboardQueryProfileId;
  receiptPath: typeof DX_DASHBOARD_QUERY_RECEIPT_PATH;
  runtimeExecution: false;
  status: "local-receipt";
  upstreamPackage: "@tanstack/react-query";
  cacheAction: string;
  nextAction: string;
};

export const DX_DASHBOARD_QUERY_RECEIPT_PATH =
  "examples/template/.dx/forge/receipts/2026-05-22-tanstack-query-dashboard-data.json" as const;

export const dxDashboardQueryDxCheckVisibility = {
  schema: "dx.forge.package.dx_check_visibility",
  packageId: "tanstack/query",
  officialPackageName: "Data Fetching & Cache",
  currentStatus: "present",
  statuses: [
    "present",
    "stale",
    "missing-receipt",
    "blocked",
    "unsupported-surface",
  ],
  receiptPath: DX_DASHBOARD_QUERY_RECEIPT_PATH,
  monitoredSurfaces: [
    "query/dashboard-workflow.ts",
    "examples/template/query-cache-status.tsx",
    "examples/template/query-dashboard-read-model.ts",
    "examples/dashboard/src/components/QueryDashboardWorkflow.tsx",
  ],
  blockedSurfaces: [
    "live browser query runtime proof",
    "production dashboard network fetcher",
  ],
  unsupportedSurfaces: [
    "unselected devtools runtime panel",
    "unselected persisted-storage runtime policy",
    "unselected cross-tab broadcast runtime proof",
  ],
} as const satisfies DxDashboardQueryDxCheckVisibility;

export const DX_DASHBOARD_QUERY_KEY = [
  "dx",
  "dashboard",
  "overview",
] as const;

export const dxDashboardQueryProfiles: readonly DxDashboardQueryProfile[] = [
  {
    id: "balanced",
    label: "Balanced cache",
    publicApi: "setQueryDefaults",
    staleTime: 60_000,
    gcTime: 5 * 60_000,
    retry: 2,
    dataFreshness: "balanced",
  },
  {
    id: "live",
    label: "Live refresh",
    publicApi: "invalidateQueries",
    staleTime: 5_000,
    gcTime: 60_000,
    retry: 1,
    dataFreshness: "live",
  },
  {
    id: "durable",
    label: "Durable cache",
    publicApi: "getQueryDefaults",
    staleTime: 5 * 60_000,
    gcTime: 30 * 60_000,
    retry: 3,
    dataFreshness: "durable",
  },
] as const;

export function getDxDashboardQueryProfile(
  profileId: DxDashboardQueryProfileId,
): DxDashboardQueryProfile {
  return (
    dxDashboardQueryProfiles.find((profile) => profile.id === profileId) ??
    dxDashboardQueryProfiles[0]
  );
}

function toDxQueryDefaults(
  profile: DxDashboardQueryProfile,
): DxQueryDefaultOptions {
  return {
    gcTime: profile.gcTime,
    retry: profile.retry,
    staleTime: profile.staleTime,
  };
}

export function createDxDashboardQueryClient(
  config: DxQueryClientConfig = {},
  profileId: DxDashboardQueryProfileId = "balanced",
): QueryClient {
  const client = createDxQueryClient(config);
  applyDxDashboardQueryProfile(client, profileId);
  return client;
}

export function applyDxDashboardQueryProfile<TQueryKey extends QueryKey>(
  client: QueryClient,
  profileId: DxDashboardQueryProfileId,
  queryKey: TQueryKey = DX_DASHBOARD_QUERY_KEY as unknown as TQueryKey,
): DxDashboardQueryReadiness {
  const profile = getDxDashboardQueryProfile(profileId);

  setDxQueryDefaults(client, queryKey, toDxQueryDefaults(profile));
  return readDxDashboardQueryReadiness(client, profile.id, queryKey);
}

export function readDxDashboardQueryReadiness<TQueryKey extends QueryKey>(
  client: QueryClient,
  profileId: DxDashboardQueryProfileId = "balanced",
  queryKey: TQueryKey = DX_DASHBOARD_QUERY_KEY as unknown as TQueryKey,
): DxDashboardQueryReadiness {
  const defaults = readDxQueryDefaults(client, queryKey);
  const caches = summarizeDxQueryClientCaches(client);

  return {
    packageId: "tanstack/query",
    queryKey,
    profileId,
    staleTime: defaults.staleTime,
    gcTime: defaults.gcTime,
    retry: defaults.retry,
    cacheState: caches.state,
    cachedQueries: caches.queries,
    staleQueries: caches.staleQueries,
  };
}

export async function refreshDxDashboardQuery<TQueryKey extends QueryKey>(
  client: QueryClient,
  queryKey: TQueryKey = DX_DASHBOARD_QUERY_KEY as unknown as TQueryKey,
): Promise<DxDashboardQueryReadiness> {
  await invalidateDxQueries(client, [queryKey]);
  return readDxDashboardQueryReadiness(client, "balanced", queryKey);
}

export function createDxDashboardQueryReceipt(
  readiness: DxDashboardQueryReadiness,
): DxDashboardQueryReceipt {
  return {
    cacheDefaults: {
      gcTime:
        typeof readiness.gcTime === "number"
          ? readiness.gcTime
          : getDxDashboardQueryProfile(readiness.profileId).gcTime,
      retry:
        typeof readiness.retry === "number" || readiness.retry === false
          ? readiness.retry
          : getDxDashboardQueryProfile(readiness.profileId).retry,
      staleTime:
        typeof readiness.staleTime === "number"
          ? readiness.staleTime
          : getDxDashboardQueryProfile(readiness.profileId).staleTime,
    },
    dashboardWorkflow: "query-backed-dashboard-data",
    dxCheckVisibility: dxDashboardQueryDxCheckVisibility,
    nodeModulesRequired: false,
    officialName: "Data Fetching & Cache",
    packageId: readiness.packageId,
    publicApi: getDxDashboardQueryProfile(readiness.profileId).publicApi,
    queryKey: readiness.queryKey,
    profileId: readiness.profileId,
    receiptPath: DX_DASHBOARD_QUERY_RECEIPT_PATH,
    runtimeExecution: false,
    status: "local-receipt",
    upstreamPackage: "@tanstack/react-query",
    cacheAction:
      "QueryClient.invalidateQueries can refresh this dashboard key when the app mounts its real fetcher.",
    nextAction:
      "Install upstream @tanstack/react-query in the app runtime, provide the dashboard fetcher, and keep query-key ownership in application code.",
  };
}
"#;

const TANSTACK_QUERY_PROVIDER_TSX: &str = r#""use client";

import * as React from "react";
import {
  HydrationBoundary,
  QueryClientProvider,
  type DehydratedState,
  type QueryClient,
} from "@tanstack/react-query";

import {
  getDxBrowserQueryClient,
  type DxQueryClientConfig,
} from "./client";

export type DxQueryProviderProps = {
  children: React.ReactNode;
  client?: QueryClient;
  config?: DxQueryClientConfig;
  state?: DehydratedState;
};

export function DxQueryProvider({
  children,
  client,
  config,
  state,
}: DxQueryProviderProps) {
  const [queryClient] = React.useState(
    () => client ?? getDxBrowserQueryClient(config),
  );
  const content = state ? (
    <HydrationBoundary state={state}>{children}</HydrationBoundary>
  ) : (
    children
  );

  return (
    <QueryClientProvider client={queryClient}>
      {content}
    </QueryClientProvider>
  );
}
"#;

const TANSTACK_QUERY_REACT_CONTEXT_TSX: &str = r#""use client";

import * as React from "react";
import {
  HydrationBoundary,
  QueryClientContext,
  useQueryClient,
  type DefaultError,
  type DefinedInitialDataInfiniteOptions,
  type DefinedInitialDataOptions,
  type HydrationBoundaryProps,
  type InfiniteData,
  type QueryClient,
  type QueryClientProviderProps,
  type QueryKey,
  type UndefinedInitialDataInfiniteOptions,
  type UndefinedInitialDataOptions,
  type UnusedSkipTokenInfiniteOptions,
  type UnusedSkipTokenOptions,
} from "@tanstack/react-query";

export const DxQueryClientContext = QueryClientContext;

export type DxQueryClientProviderProps = QueryClientProviderProps;
export type DxHydrationBoundaryProps = HydrationBoundaryProps;

export type DxDefinedQueryOptions<
  TQueryFnData = unknown,
  TError = DefaultError,
  TData = TQueryFnData,
  TQueryKey extends QueryKey = QueryKey,
> = DefinedInitialDataOptions<TQueryFnData, TError, TData, TQueryKey>;

export type DxUndefinedQueryOptions<
  TQueryFnData = unknown,
  TError = DefaultError,
  TData = TQueryFnData,
  TQueryKey extends QueryKey = QueryKey,
> = UndefinedInitialDataOptions<TQueryFnData, TError, TData, TQueryKey>;

export type DxUnusedSkipTokenQueryOptions<
  TQueryFnData = unknown,
  TError = DefaultError,
  TData = TQueryFnData,
  TQueryKey extends QueryKey = QueryKey,
> = UnusedSkipTokenOptions<TQueryFnData, TError, TData, TQueryKey>;

export type DxDefinedInfiniteQueryOptions<
  TQueryFnData = unknown,
  TError = DefaultError,
  TData = InfiniteData<TQueryFnData>,
  TQueryKey extends QueryKey = QueryKey,
  TPageParam = unknown,
> = DefinedInitialDataInfiniteOptions<
  TQueryFnData,
  TError,
  TData,
  TQueryKey,
  TPageParam
>;

export type DxUndefinedInfiniteQueryOptions<
  TQueryFnData = unknown,
  TError = DefaultError,
  TData = InfiniteData<TQueryFnData>,
  TQueryKey extends QueryKey = QueryKey,
  TPageParam = unknown,
> = UndefinedInitialDataInfiniteOptions<
  TQueryFnData,
  TError,
  TData,
  TQueryKey,
  TPageParam
>;

export type DxUnusedSkipTokenInfiniteQueryOptions<
  TQueryFnData = unknown,
  TError = DefaultError,
  TData = InfiniteData<TQueryFnData>,
  TQueryKey extends QueryKey = QueryKey,
  TPageParam = unknown,
> = UnusedSkipTokenInfiniteOptions<
  TQueryFnData,
  TError,
  TData,
  TQueryKey,
  TPageParam
>;

export type DxQueryClientContextProvider =
  | "query-client-provider"
  | "explicit-client"
  | "missing";

export type DxQueryClientContextStatus = {
  state: "ready" | "missing";
  provider: DxQueryClientContextProvider;
  queries: number;
  mutations: number;
};

export type DxQueryClientContextBridgeProps = Pick<
  QueryClientProviderProps,
  "client" | "children"
>;

export function useDxRequiredQueryClient(client?: QueryClient): QueryClient {
  return useQueryClient(client);
}

export function readDxQueryClientContextStatus(
  client?: QueryClient | null,
  provider: Exclude<DxQueryClientContextProvider, "missing"> =
    "query-client-provider",
): DxQueryClientContextStatus {
  if (!client) {
    return {
      state: "missing",
      provider: "missing",
      queries: 0,
      mutations: 0,
    };
  }

  return {
    state: "ready",
    provider,
    queries: client.getQueryCache().getAll().length,
    mutations: client.getMutationCache().getAll().length,
  };
}

export function useDxQueryClientContextStatus(
  client?: QueryClient,
): DxQueryClientContextStatus {
  const queryClient = useDxRequiredQueryClient(client);

  return readDxQueryClientContextStatus(
    queryClient,
    client ? "explicit-client" : "query-client-provider",
  );
}

export function DxQueryClientContextBridge({
  children,
  client,
}: DxQueryClientContextBridgeProps): React.JSX.Element {
  return (
    <QueryClientContext.Provider value={client}>
      {children}
    </QueryClientContext.Provider>
  );
}

export function DxHydrationBoundary(
  props: DxHydrationBoundaryProps,
): React.JSX.Element {
  return <HydrationBoundary {...props} />;
}
"#;

const TANSTACK_QUERY_RESTORING_TSX: &str = r#""use client";

import * as React from "react";
import {
  IsRestoringProvider,
  useIsRestoring,
} from "@tanstack/react-query";

export type DxQueryRestoringProviderProps = {
  children: React.ReactNode;
  restoring: boolean;
};

export function DxQueryRestoringProvider({
  children,
  restoring,
}: DxQueryRestoringProviderProps) {
  return (
    <IsRestoringProvider value={restoring}>
      {children}
    </IsRestoringProvider>
  );
}

export type DxQueryRestoreState = "restoring" | "ready";

export type DxQueryRestoreStatusState = {
  restoring: boolean;
  state: DxQueryRestoreState;
  label: string;
};

export function useDxQueryRestoreStatus(): DxQueryRestoreStatusState {
  const restoring = useIsRestoring();

  return {
    restoring,
    state: restoring ? "restoring" : "ready",
    label: restoring ? "Restoring query cache" : "Query cache ready",
  };
}

export type DxQueryRestoreStatusProps =
  React.HTMLAttributes<HTMLSpanElement> & {
    label?: (status: DxQueryRestoreStatusState) => React.ReactNode;
  };

export function DxQueryRestoreStatus({
  children,
  label,
  ...props
}: DxQueryRestoreStatusProps) {
  const status = useDxQueryRestoreStatus();

  return (
    <span
      {...props}
      data-query-restore-state={status.state}
      data-query-restoring={status.restoring ? "true" : "false"}
    >
      {children ?? label?.(status) ?? status.label}
    </span>
  );
}
"#;

const TANSTACK_QUERY_PERSIST_TSX: &str = r#""use client";

import * as React from "react";
import {
  experimental_createQueryPersister,
  persistQueryClient,
  persistQueryClientRestore,
  persistQueryClientSave,
  persistQueryClientSubscribe,
  removeOldestQuery,
  type AsyncStorage,
  type PersistedClient,
  type PersistedQueryClientRestoreOptions,
  type PersistedQueryClientSaveOptions,
  type PersistQueryClientOptions,
  type PersistRetryer,
  type Persister,
  type StoragePersisterOptions,
} from "@tanstack/query-persist-client-core";
import { createAsyncStoragePersister } from "@tanstack/query-async-storage-persister";
import {
  PersistQueryClientProvider,
  type PersistQueryClientProviderProps,
} from "@tanstack/react-query-persist-client";

export const DX_QUERY_PERSIST_KEY = "dx-query-offline-cache";
export const DX_QUERY_PERSIST_BUSTER = "dx-query-v1";
export const DX_QUERY_PERSISTED_QUERY_PREFIX = "dx-query";
export const DX_QUERY_PERSIST_MAX_AGE_MS = 1000 * 60 * 60 * 24;

export type DxQueryPersistStorage = {
  getItem: (key: string) => string | null;
  setItem: (key: string, value: string) => void;
  removeItem: (key: string) => void;
};

export type DxQueryPersistStatus = {
  key: string;
  buster: string;
  maxAgeMs: number;
  storage: "browser" | "custom" | "missing";
  persister: boolean;
  restoring: boolean;
};

export type DxBrowserQueryPersisterConfig = {
  storage?: DxQueryPersistStorage | null;
  key?: string;
  throttleTimeMs?: number;
  retry?: PersistRetryer;
};

export type DxPersistedQueryClientOptions = PersistQueryClientOptions;
export type DxPersistedClient = PersistedClient;
export type DxQueryPersister = Persister;
export type DxFineGrainedQueryPersisterOptions<TStorageValue = string> =
  StoragePersisterOptions<TStorageValue>;

function readDxBrowserStorage(): DxQueryPersistStorage | undefined {
  if (typeof window === "undefined") return undefined;

  try {
    return window.localStorage;
  } catch {
    return undefined;
  }
}

export function createDxQueryAsyncStorage(
  storage: DxQueryPersistStorage | null | undefined,
): AsyncStorage<string> | undefined {
  if (!storage) return undefined;

  return {
    getItem: (key) => storage.getItem(key),
    setItem: (key, value) => storage.setItem(key, value),
    removeItem: (key) => storage.removeItem(key),
  };
}

export function createDxBrowserQueryPersister(
  config: DxBrowserQueryPersisterConfig = {},
): Persister {
  const storage =
    config.storage === undefined ? readDxBrowserStorage() : config.storage;

  return createAsyncStoragePersister({
    storage: createDxQueryAsyncStorage(storage),
    key: config.key ?? DX_QUERY_PERSIST_KEY,
    throttleTime: config.throttleTimeMs ?? 1000,
    retry: config.retry ?? removeOldestQuery,
  });
}

export function mountDxPersistedQueryClient(
  options: DxPersistedQueryClientOptions,
) {
  return persistQueryClient({
    maxAge: DX_QUERY_PERSIST_MAX_AGE_MS,
    buster: DX_QUERY_PERSIST_BUSTER,
    ...options,
  });
}

export function restoreDxPersistedQueryClient(
  options: PersistedQueryClientRestoreOptions,
) {
  return persistQueryClientRestore({
    maxAge: DX_QUERY_PERSIST_MAX_AGE_MS,
    buster: DX_QUERY_PERSIST_BUSTER,
    ...options,
  });
}

export function saveDxPersistedQueryClient(
  options: PersistedQueryClientSaveOptions,
) {
  return persistQueryClientSave({
    buster: DX_QUERY_PERSIST_BUSTER,
    ...options,
  });
}

export function subscribeDxPersistedQueryClient(
  options: PersistedQueryClientSaveOptions,
) {
  return persistQueryClientSubscribe({
    buster: DX_QUERY_PERSIST_BUSTER,
    ...options,
  });
}

export function createDxFineGrainedQueryPersister<
  TStorageValue = string,
>(options: DxFineGrainedQueryPersisterOptions<TStorageValue>) {
  return experimental_createQueryPersister({
    prefix: DX_QUERY_PERSISTED_QUERY_PREFIX,
    buster: DX_QUERY_PERSIST_BUSTER,
    maxAge: DX_QUERY_PERSIST_MAX_AGE_MS,
    ...options,
  });
}

export function readDxPersistedQueryStatus(
  input: {
    persister?: Persister | null;
    storage?: DxQueryPersistStorage | null;
    key?: string;
    buster?: string;
    maxAgeMs?: number;
    restoring?: boolean;
  } = {},
): DxQueryPersistStatus {
  const storage =
    input.storage !== undefined
      ? input.storage
        ? "custom"
        : "missing"
      : readDxBrowserStorage()
        ? "browser"
        : "missing";

  return {
    key: input.key ?? DX_QUERY_PERSIST_KEY,
    buster: input.buster ?? DX_QUERY_PERSIST_BUSTER,
    maxAgeMs: input.maxAgeMs ?? DX_QUERY_PERSIST_MAX_AGE_MS,
    storage,
    persister: Boolean(input.persister),
    restoring: input.restoring ?? false,
  };
}

export type DxPersistQueryClientProviderProps = Omit<
  PersistQueryClientProviderProps,
  "persistOptions"
> & {
  persistOptions: Omit<PersistQueryClientOptions, "queryClient">;
  maxAgeMs?: number;
  buster?: string;
};

export function DxPersistQueryClientProvider({
  persistOptions,
  maxAgeMs = DX_QUERY_PERSIST_MAX_AGE_MS,
  buster = DX_QUERY_PERSIST_BUSTER,
  ...props
}: DxPersistQueryClientProviderProps): React.JSX.Element {
  return (
    <PersistQueryClientProvider
      {...props}
      persistOptions={{
        maxAge: maxAgeMs,
        buster,
        ...persistOptions,
      }}
    />
  );
}
"#;

const TANSTACK_QUERY_SYNC_PERSIST_TS: &str = r#"import { createSyncStoragePersister } from "@tanstack/query-sync-storage-persister";
import {
  removeOldestQuery,
  type PersistedClient,
  type PersistRetryer,
  type Persister,
} from "@tanstack/query-persist-client-core";

export const DX_QUERY_SYNC_PERSIST_KEY = "dx-query-sync-offline-cache";
export const DX_QUERY_SYNC_PERSIST_THROTTLE_MS = 1000;

export type DxSyncStorage = {
  getItem: (key: string) => string | null;
  setItem: (key: string, value: string) => void;
  removeItem: (key: string) => void;
};

export type DxSyncStorageKind = "browser" | "custom" | "memory" | "missing";

export type DxSyncStoragePersisterStatus = {
  key: string;
  storage: DxSyncStorageKind;
  persister: boolean;
  throttleTimeMs: number;
  retry: "remove-oldest-query" | "custom" | "none";
  deprecated: true;
};

export type DxSyncStoragePersisterConfig = {
  storage?: DxSyncStorage | null;
  key?: string;
  throttleTimeMs?: number;
  retry?: PersistRetryer | null;
  serialize?: (client: PersistedClient) => string;
  deserialize?: (cachedString: string) => PersistedClient;
};

function readDxBrowserSyncStorage(): DxSyncStorage | undefined {
  if (typeof window === "undefined") return undefined;

  try {
    return window.localStorage;
  } catch {
    return undefined;
  }
}

export function createDxMemorySyncStorage(
  seed: Record<string, string> = {},
): DxSyncStorage {
  const store = new Map(Object.entries(seed));

  return {
    getItem: (key) => store.get(key) ?? null,
    setItem: (key, value) => {
      store.set(key, value);
    },
    removeItem: (key) => {
      store.delete(key);
    },
  };
}

export function createDxBrowserSyncStoragePersister(
  config: DxSyncStoragePersisterConfig = {},
): Persister {
  const storage =
    config.storage === undefined ? readDxBrowserSyncStorage() : config.storage;

  return createSyncStoragePersister({
    storage,
    key: config.key ?? DX_QUERY_SYNC_PERSIST_KEY,
    throttleTime: config.throttleTimeMs ?? DX_QUERY_SYNC_PERSIST_THROTTLE_MS,
    serialize: config.serialize,
    deserialize: config.deserialize,
    retry: config.retry === null ? undefined : (config.retry ?? removeOldestQuery),
  });
}

export function readDxSyncStoragePersisterStatus(
  input: {
    persister?: Persister | null;
    storage?: DxSyncStorage | null;
    storageKind?: DxSyncStorageKind;
    key?: string;
    throttleTimeMs?: number;
    retry?: PersistRetryer | null;
  } = {},
): DxSyncStoragePersisterStatus {
  const storage =
    input.storageKind ??
    (input.storage !== undefined
      ? input.storage
        ? "custom"
        : "missing"
      : readDxBrowserSyncStorage()
        ? "browser"
        : "missing");

  return {
    key: input.key ?? DX_QUERY_SYNC_PERSIST_KEY,
    storage,
    persister: Boolean(input.persister),
    throttleTimeMs:
      input.throttleTimeMs ?? DX_QUERY_SYNC_PERSIST_THROTTLE_MS,
    retry:
      input.retry === undefined
        ? "remove-oldest-query"
        : input.retry
          ? "custom"
          : "none",
    deprecated: true,
  };
}
"#;

const TANSTACK_QUERY_DEVTOOLS_TSX: &str = r#""use client";

import * as React from "react";
import {
  ReactQueryDevtools,
  ReactQueryDevtoolsPanel,
  type DevtoolsPanelOptions,
} from "@tanstack/react-query-devtools";

export type DxQueryDevtoolsPanelMode = "floating" | "embedded" | "off";

export type DxQueryDevtoolsStatus = {
  enabled: boolean;
  environment: string;
  panel: DxQueryDevtoolsPanelMode;
  productionSafe: boolean;
};

export type DxQueryDevtoolsProps =
  React.ComponentProps<typeof ReactQueryDevtools> & {
    enabled?: boolean;
    environment?: string;
  };

export type DxQueryDevtoolsPanelProps = DevtoolsPanelOptions & {
  enabled?: boolean;
  height?: React.CSSProperties["height"];
};

export function readDxQueryDevtoolsStatus(
  input: {
    enabled?: boolean;
    environment?: string;
    panel?: DxQueryDevtoolsPanelMode;
  } = {},
): DxQueryDevtoolsStatus {
  const enabled = input.enabled ?? false;
  const environment = input.environment ?? "unknown";

  return {
    enabled,
    environment,
    panel: enabled ? (input.panel ?? "floating") : "off",
    productionSafe: !enabled || environment !== "production",
  };
}

export function DxQueryDevtools({
  enabled = false,
  environment: _environment,
  initialIsOpen = false,
  buttonPosition = "bottom-right",
  position = "bottom",
  hideDisabledQueries = true,
  theme = "system",
  ...props
}: DxQueryDevtoolsProps): React.JSX.Element | null {
  if (!enabled) return null;

  return (
    <ReactQueryDevtools
      {...props}
      initialIsOpen={initialIsOpen}
      buttonPosition={buttonPosition}
      position={position}
      hideDisabledQueries={hideDisabledQueries}
      theme={theme}
    />
  );
}

export function DxQueryDevtoolsPanel({
  enabled = false,
  height = 360,
  style,
  ...props
}: DxQueryDevtoolsPanelProps): React.JSX.Element | null {
  if (!enabled) return null;

  return (
    <ReactQueryDevtoolsPanel
      {...props}
      style={{
        height,
        ...style,
      }}
    />
  );
}
"#;

const TANSTACK_QUERY_BROADCAST_TS: &str = r#"import { broadcastQueryClient } from "@tanstack/query-broadcast-client-experimental";
import type { QueryClient } from "@tanstack/react-query";
import type { BroadcastChannelOptions } from "broadcast-channel";

export const DX_QUERY_BROADCAST_CHANNEL = "dx-query-cache";

export type DxQueryBroadcastTransport =
  | "broadcast-channel"
  | "disabled"
  | "server";

export type DxQueryBroadcastStatus = {
  enabled: boolean;
  connected: boolean;
  channel: string;
  transport: DxQueryBroadcastTransport;
  cleanupRegistered: boolean;
};

export type DxQueryBroadcastHandle = {
  status: DxQueryBroadcastStatus;
  close: () => void;
  unsubscribe: () => void;
};

export type DxQueryBroadcastOptions = {
  queryClient: QueryClient;
  channel?: string;
  enabled?: boolean;
  options?: BroadcastChannelOptions;
};

export function createDxQueryBroadcastDisabledHandle(
  channel = DX_QUERY_BROADCAST_CHANNEL,
  transport: Exclude<
    DxQueryBroadcastTransport,
    "broadcast-channel"
  > = "disabled",
): DxQueryBroadcastHandle {
  return {
    status: {
      enabled: false,
      connected: false,
      channel,
      transport,
      cleanupRegistered: false,
    },
    close: () => {},
    unsubscribe: () => {},
  };
}

export function mountDxQueryBroadcastClient({
  queryClient,
  channel = DX_QUERY_BROADCAST_CHANNEL,
  enabled = true,
  options,
}: DxQueryBroadcastOptions): DxQueryBroadcastHandle {
  if (!enabled) {
    return createDxQueryBroadcastDisabledHandle(channel);
  }

  if (typeof window === "undefined") {
    return createDxQueryBroadcastDisabledHandle(channel, "server");
  }

  const status: DxQueryBroadcastStatus = {
    enabled: true,
    connected: true,
    channel,
    transport: "broadcast-channel",
    cleanupRegistered: true,
  };
  const cleanup = broadcastQueryClient({
    queryClient,
    broadcastChannel: channel,
    options,
  });

  const close = () => {
    if (!status.connected) return;

    cleanup();
    status.connected = false;
    status.cleanupRegistered = false;
  };

  return {
    status,
    close,
    unsubscribe: close,
  };
}

export function readDxQueryBroadcastStatus(
  handle?: DxQueryBroadcastHandle | null,
  channel = DX_QUERY_BROADCAST_CHANNEL,
): DxQueryBroadcastStatus {
  return handle?.status ?? createDxQueryBroadcastDisabledHandle(channel).status;
}
"#;

const TANSTACK_QUERY_NEXT_STREAMING_TSX: &str = r#""use client";

import * as React from "react";
import { ReactQueryStreamedHydration } from "@tanstack/react-query-next-experimental";
import type {
  DehydrateOptions,
  DehydratedState,
  HydrateOptions,
  QueryClient,
} from "@tanstack/react-query";

export type DxQueryNextStreamingMode = "streamed-hydration" | "disabled";

export type DxQueryNextStreamingStatus = {
  enabled: boolean;
  mode: DxQueryNextStreamingMode;
  runtime: "next-app-router" | "unknown";
  transformer: boolean;
  nonce: boolean;
};

export type DxQueryNextStreamingTransformer =
  NonNullable<
    React.ComponentProps<typeof ReactQueryStreamedHydration>["transformer"]
  >;

export type DxQueryNextStreamingOptions = {
  hydrate?: HydrateOptions;
  dehydrate?: DehydrateOptions;
};

export type DxQueryStreamedHydrationSnapshot = DehydratedState;

export type DxReactQueryStreamedHydrationProps = {
  children: React.ReactNode;
  enabled?: boolean;
  queryClient?: QueryClient;
  nonce?: string;
  options?: DxQueryNextStreamingOptions;
  transformer?: DxQueryNextStreamingTransformer;
};

export function readDxQueryNextStreamingStatus(
  input: {
    enabled?: boolean;
    runtime?: DxQueryNextStreamingStatus["runtime"];
    transformer?: DxQueryNextStreamingTransformer | null;
    nonce?: string | null;
  } = {},
): DxQueryNextStreamingStatus {
  const enabled = input.enabled ?? true;

  return {
    enabled,
    mode: enabled ? "streamed-hydration" : "disabled",
    runtime: input.runtime ?? "next-app-router",
    transformer: Boolean(input.transformer),
    nonce: Boolean(input.nonce),
  };
}

export function createDxQueryNextStreamingOptions(
  options: DxQueryNextStreamingOptions = {},
): DxQueryNextStreamingOptions {
  return options;
}

export function DxReactQueryStreamedHydration({
  children,
  enabled = true,
  queryClient,
  nonce,
  options,
  transformer,
}: DxReactQueryStreamedHydrationProps): React.JSX.Element {
  if (!enabled) {
    return <>{children}</>;
  }

  return (
    <ReactQueryStreamedHydration
      queryClient={queryClient}
      nonce={nonce}
      options={options}
      transformer={transformer}
    >
      {children}
    </ReactQueryStreamedHydration>
  );
}
"#;

const TANSTACK_QUERY_FETCH_TS: &str = r#"import {
  type DefaultError,
  type EnsureInfiniteQueryDataOptions,
  type EnsureQueryDataOptions,
  type FetchInfiniteQueryOptions,
  type FetchQueryOptions,
  type InfiniteData,
  type QueryClient,
  type QueryKey,
} from "@tanstack/react-query";

export type DxFetchQueryOptions<
  TQueryFnData = unknown,
  TError = DefaultError,
  TData = TQueryFnData,
  TQueryKey extends QueryKey = QueryKey,
  TPageParam = never,
> = FetchQueryOptions<TQueryFnData, TError, TData, TQueryKey, TPageParam>;

export type DxEnsureQueryDataOptions<
  TQueryFnData = unknown,
  TError = DefaultError,
  TData = TQueryFnData,
  TQueryKey extends QueryKey = QueryKey,
> = EnsureQueryDataOptions<TQueryFnData, TError, TData, TQueryKey>;

export type DxFetchInfiniteQueryOptions<
  TQueryFnData = unknown,
  TError = DefaultError,
  TData = TQueryFnData,
  TQueryKey extends QueryKey = QueryKey,
  TPageParam = unknown,
> = FetchInfiniteQueryOptions<
  TQueryFnData,
  TError,
  TData,
  TQueryKey,
  TPageParam
>;

export type DxEnsureInfiniteQueryDataOptions<
  TQueryFnData = unknown,
  TError = DefaultError,
  TData = TQueryFnData,
  TQueryKey extends QueryKey = QueryKey,
  TPageParam = unknown,
> = EnsureInfiniteQueryDataOptions<
  TQueryFnData,
  TError,
  TData,
  TQueryKey,
  TPageParam
>;

export type DxFetchSummary = {
  queries: number;
  fetching: number;
  mutations: number;
  staleQueries: number;
};

export function fetchDxQueryData<
  TQueryFnData,
  TError = DefaultError,
  TData = TQueryFnData,
  TQueryKey extends QueryKey = QueryKey,
  TPageParam = never,
>(
  client: QueryClient,
  options: DxFetchQueryOptions<
    TQueryFnData,
    TError,
    TData,
    TQueryKey,
    TPageParam
  >,
): Promise<TData> {
  return client.fetchQuery(options);
}

export function prefetchDxQueryData<
  TQueryFnData = unknown,
  TError = DefaultError,
  TData = TQueryFnData,
  TQueryKey extends QueryKey = QueryKey,
>(
  client: QueryClient,
  options: FetchQueryOptions<TQueryFnData, TError, TData, TQueryKey>,
) {
  return client.prefetchQuery(options);
}

export function ensureDxQueryDataFresh<
  TQueryFnData,
  TError = DefaultError,
  TData = TQueryFnData,
  TQueryKey extends QueryKey = QueryKey,
>(
  client: QueryClient,
  options: DxEnsureQueryDataOptions<TQueryFnData, TError, TData, TQueryKey>,
): Promise<TData> {
  return client.ensureQueryData({
    revalidateIfStale: true,
    ...options,
  });
}

export function fetchDxInfiniteQueryData<
  TQueryFnData,
  TError = DefaultError,
  TData = TQueryFnData,
  TQueryKey extends QueryKey = QueryKey,
  TPageParam = unknown,
>(
  client: QueryClient,
  options: DxFetchInfiniteQueryOptions<
    TQueryFnData,
    TError,
    TData,
    TQueryKey,
    TPageParam
  >,
): Promise<InfiniteData<TData, TPageParam>> {
  return client.fetchInfiniteQuery(options);
}

export function prefetchDxInfiniteQueryData<
  TQueryFnData,
  TError = DefaultError,
  TData = TQueryFnData,
  TQueryKey extends QueryKey = QueryKey,
  TPageParam = unknown,
>(
  client: QueryClient,
  options: DxFetchInfiniteQueryOptions<
    TQueryFnData,
    TError,
    TData,
    TQueryKey,
    TPageParam
  >,
) {
  return client.prefetchInfiniteQuery(options);
}

export function ensureDxInfiniteQueryData<
  TQueryFnData,
  TError = DefaultError,
  TData = TQueryFnData,
  TQueryKey extends QueryKey = QueryKey,
  TPageParam = unknown,
>(
  client: QueryClient,
  options: DxEnsureInfiniteQueryDataOptions<
    TQueryFnData,
    TError,
    TData,
    TQueryKey,
    TPageParam
  >,
): Promise<InfiniteData<TData, TPageParam>> {
  return client.ensureInfiniteQueryData(options);
}

export function createDxFetchSummary(client: QueryClient): DxFetchSummary {
  const queries = client.getQueryCache().getAll();

  return {
    queries: queries.length,
    fetching: client.isFetching(),
    mutations: client.getMutationCache().getAll().length,
    staleQueries: queries.filter((query) => query.isStale()).length,
  };
}
"#;

const TANSTACK_QUERY_PREFETCH_TSX: &str = r#"import type * as React from "react";
import {
  HydrationBoundary,
  dehydrate,
  type DehydratedState,
  type QueryClient,
  type QueryFunction,
  type QueryKey,
} from "@tanstack/react-query";

import {
  createDxQueryClient,
  type DxQueryClientConfig,
} from "./client";

export type DxPrefetchQueryInput<
  TQueryFnData,
  TQueryKey extends QueryKey,
> = {
  queryKey: TQueryKey;
  queryFn: QueryFunction<TQueryFnData, TQueryKey>;
  client?: QueryClient;
  config?: DxQueryClientConfig;
  staleTime?: number;
};

export type DxPrefetchQueryResult = {
  client: QueryClient;
  state: DehydratedState;
};

export async function prefetchDxQuery<
  TQueryFnData,
  TQueryKey extends QueryKey,
>({
  queryKey,
  queryFn,
  client,
  config,
  staleTime,
}: DxPrefetchQueryInput<TQueryFnData, TQueryKey>): Promise<DxPrefetchQueryResult> {
  const queryClient = client ?? createDxQueryClient(config);

  await queryClient.prefetchQuery({
    queryKey,
    queryFn,
    staleTime,
  });

  return {
    client: queryClient,
    state: dehydrate(queryClient),
  };
}

export function DxHydrationBoundary({
  children,
  state,
}: {
  children: React.ReactNode;
  state: DehydratedState;
}) {
  return <HydrationBoundary state={state}>{children}</HydrationBoundary>;
}
"#;

const TANSTACK_QUERY_HYDRATION_TS: &str = r#"import {
  defaultShouldDehydrateMutation,
  defaultShouldDehydrateQuery,
  dehydrate,
  hydrate,
  type DehydratedState,
  type DehydrateOptions,
  type HydrateOptions,
  type QueryClient,
} from "@tanstack/react-query";

export type DxDehydrateOptions = DehydrateOptions;
export type DxHydrateOptions = HydrateOptions;

export type DxHydrationSummaryState =
  | "empty"
  | "queries"
  | "mutations"
  | "mixed";

export type DxHydrationSummary = {
  state: DxHydrationSummaryState;
  queries: number;
  mutations: number;
  latestDehydratedAt: number;
};

export const dxShouldDehydrateQuery = defaultShouldDehydrateQuery;
export const dxShouldDehydrateMutation = defaultShouldDehydrateMutation;

export function createDxDehydrateOptions(
  options: DxDehydrateOptions = {},
): DxDehydrateOptions {
  return {
    serializeData: options.serializeData,
    shouldDehydrateMutation:
      options.shouldDehydrateMutation ?? defaultShouldDehydrateMutation,
    shouldDehydrateQuery:
      options.shouldDehydrateQuery ?? defaultShouldDehydrateQuery,
    shouldRedactErrors: options.shouldRedactErrors ?? (() => true),
  };
}

export function createDxHydrateOptions(
  options: DxHydrateOptions = {},
): DxHydrateOptions {
  return options;
}

export function dehydrateDxQueryClient(
  client: QueryClient,
  options?: DxDehydrateOptions,
) {
  return dehydrate(client, createDxDehydrateOptions(options));
}

export function hydrateDxQueryClient(
  client: QueryClient,
  state: unknown,
  options?: DxHydrateOptions,
) {
  hydrate(client, state, createDxHydrateOptions(options));
  return client;
}

export function summarizeDxDehydratedState(
  state: DehydratedState,
): DxHydrationSummary {
  const queries = state.queries.length;
  const mutations = state.mutations.length;
  const latestDehydratedAt = state.queries.reduce(
    (latest, query) => Math.max(latest, query.dehydratedAt ?? 0),
    0,
  );

  return {
    queries,
    mutations,
    latestDehydratedAt,
    state: formatDxHydrationSummaryState(queries, mutations),
  };
}

export function summarizeDxHydrationState(
  client: QueryClient,
  options?: DxDehydrateOptions,
) {
  return summarizeDxDehydratedState(
    dehydrateDxQueryClient(client, options),
  );
}

function formatDxHydrationSummaryState(
  queries: number,
  mutations: number,
): DxHydrationSummaryState {
  if (queries > 0 && mutations > 0) return "mixed";
  if (queries > 0) return "queries";
  if (mutations > 0) return "mutations";
  return "empty";
}
"#;

const TANSTACK_QUERY_MUTATION_TS: &str = r#"import {
  mutationOptions,
  type DefaultError,
  type InvalidateQueryFilters,
  type MutationKey,
  type QueryClient,
  type QueryKey,
  type UseMutationOptions,
} from "@tanstack/react-query";

export type DxInvalidateQueryTarget<TQueryKey extends QueryKey = QueryKey> =
  | TQueryKey
  | InvalidateQueryFilters<TQueryKey>;

export type DxMutationOptionsInput<
  TData = unknown,
  TError = DefaultError,
  TVariables = void,
  TOnMutateResult = unknown,
> = UseMutationOptions<TData, TError, TVariables, TOnMutateResult>;

export function dxMutationOptions<
  TData = unknown,
  TError = DefaultError,
  TVariables = void,
  TOnMutateResult = unknown,
>(
  input: DxMutationOptionsInput<TData, TError, TVariables, TOnMutateResult>,
) {
  return mutationOptions(input);
}

export async function invalidateDxQueries<
  TQueryKey extends QueryKey = QueryKey,
>(
  client: QueryClient,
  targets: readonly DxInvalidateQueryTarget<TQueryKey>[],
) {
  await Promise.all(
    targets.map((target) =>
      client.invalidateQueries(normalizeInvalidateTarget(target)),
    ),
  );
}

export function dxMutationKey<TMutationKey extends MutationKey>(
  mutationKey: TMutationKey,
) {
  return mutationKey;
}

function normalizeInvalidateTarget<TQueryKey extends QueryKey>(
  target: DxInvalidateQueryTarget<TQueryKey>,
): InvalidateQueryFilters<TQueryKey> {
  return Array.isArray(target) ? { queryKey: target } : target;
}
"#;

const TANSTACK_QUERY_MUTATION_RESULT_TS: &str = r#"import {
  type DefaultError,
  type MutateFunction,
  type MutateOptions,
  type MutationFunction,
  type MutationFunctionContext,
  type MutationObserverBaseResult,
  type MutationStatus,
  type UseMutateAsyncFunction,
  type UseMutateFunction,
  type UseMutationResult,
} from "@tanstack/react-query";

export type DxMutationFunction<
  TData = unknown,
  TVariables = unknown,
> = MutationFunction<TData, TVariables>;

export type DxMutationFunctionContext = MutationFunctionContext;

export type DxMutateOptions<
  TData = unknown,
  TError = DefaultError,
  TVariables = void,
  TOnMutateResult = unknown,
> = MutateOptions<TData, TError, TVariables, TOnMutateResult>;

export type DxMutateFunction<
  TData = unknown,
  TError = DefaultError,
  TVariables = void,
  TOnMutateResult = unknown,
> = MutateFunction<TData, TError, TVariables, TOnMutateResult>;

export type DxUseMutateFunction<
  TData = unknown,
  TError = DefaultError,
  TVariables = void,
  TOnMutateResult = unknown,
> = UseMutateFunction<TData, TError, TVariables, TOnMutateResult>;

export type DxUseMutateAsyncFunction<
  TData = unknown,
  TError = DefaultError,
  TVariables = void,
  TOnMutateResult = unknown,
> = UseMutateAsyncFunction<TData, TError, TVariables, TOnMutateResult>;

export type DxMutationObserverBaseResult<
  TData = unknown,
  TError = DefaultError,
  TVariables = void,
  TOnMutateResult = unknown,
> = MutationObserverBaseResult<TData, TError, TVariables, TOnMutateResult>;

export type DxMutationResult<
  TData = unknown,
  TError = DefaultError,
  TVariables = unknown,
  TOnMutateResult = unknown,
> = UseMutationResult<TData, TError, TVariables, TOnMutateResult>;

export type DxMutationResultSummary = {
  status: MutationStatus;
  idle: boolean;
  pending: boolean;
  success: boolean;
  error: boolean;
  paused: boolean;
  failureCount: number;
  submittedAt: number;
  canMutate: boolean;
  canReset: boolean;
  hasData: boolean;
  hasError: boolean;
  hasVariables: boolean;
};

export function summarizeDxMutationResult<
  TData = unknown,
  TError = DefaultError,
  TVariables = unknown,
  TOnMutateResult = unknown,
>(
  result: UseMutationResult<TData, TError, TVariables, TOnMutateResult>,
): DxMutationResultSummary {
  const hasData = typeof result.data !== "undefined";
  const hasError = result.error !== null;
  const hasVariables = typeof result.variables !== "undefined";

  return {
    status: result.status,
    idle: result.isIdle,
    pending: result.isPending,
    success: result.isSuccess,
    error: result.isError,
    paused: result.isPaused,
    failureCount: result.failureCount,
    submittedAt: result.submittedAt,
    canMutate: !result.isPending,
    canReset:
      result.status !== "idle" ||
      result.failureCount > 0 ||
      hasData ||
      hasError ||
      hasVariables,
    hasData,
    hasError,
    hasVariables,
  };
}

export function formatDxMutationResultError<
  TData = unknown,
  TError = unknown,
  TVariables = unknown,
  TOnMutateResult = unknown,
>(
  result: Pick<
    UseMutationResult<TData, TError, TVariables, TOnMutateResult>,
    "error" | "isError"
  >,
  fallback = "Mutation failed",
) {
  if (!result.isError || result.error === null) return "";

  if (result.error instanceof Error && result.error.message.trim().length > 0) {
    return result.error.message;
  }

  if (typeof result.error === "string" && result.error.trim().length > 0) {
    return result.error;
  }

  return fallback;
}

export async function runDxMutation<
  TData = unknown,
  TError = DefaultError,
  TVariables = void,
  TOnMutateResult = unknown,
>(
  mutateAsync: UseMutateAsyncFunction<
    TData,
    TError,
    TVariables,
    TOnMutateResult
  >,
  variables: TVariables,
  options?: MutateOptions<TData, TError, TVariables, TOnMutateResult>,
) {
  return mutateAsync(variables, options);
}
"#;

const TANSTACK_QUERY_RESULT_TS: &str = r#"import {
  type DefaultError,
  type FetchStatus,
  type InfiniteQueryObserverBaseResult,
  type InfiniteQueryObserverResult,
  type QueryObserverBaseResult,
  type QueryObserverResult,
  type QueryStatus,
  type DefinedUseInfiniteQueryResult,
  type DefinedUseQueryResult,
  type UseBaseQueryResult,
  type UseInfiniteQueryResult,
  type UseQueryResult,
  type UseSuspenseInfiniteQueryResult,
  type UseSuspenseQueryResult,
} from "@tanstack/react-query";

export type DxQueryObserverBaseResult<
  TData = unknown,
  TError = DefaultError,
> = QueryObserverBaseResult<TData, TError>;

export type DxQueryObserverResult<
  TData = unknown,
  TError = DefaultError,
> = QueryObserverResult<TData, TError>;

export type DxUseBaseQueryResult<
  TData = unknown,
  TError = DefaultError,
> = UseBaseQueryResult<TData, TError>;

export type DxUseQueryResult<
  TData = unknown,
  TError = DefaultError,
> = UseQueryResult<TData, TError>;

export type DxDefinedUseQueryResult<
  TData = unknown,
  TError = DefaultError,
> = DefinedUseQueryResult<TData, TError>;

export type DxUseSuspenseQueryResult<
  TData = unknown,
  TError = DefaultError,
> = UseSuspenseQueryResult<TData, TError>;

export type DxInfiniteQueryObserverBaseResult<
  TData = unknown,
  TError = DefaultError,
> = InfiniteQueryObserverBaseResult<TData, TError>;

export type DxInfiniteQueryObserverResult<
  TData = unknown,
  TError = DefaultError,
> = InfiniteQueryObserverResult<TData, TError>;

export type DxUseInfiniteQueryResult<
  TData = unknown,
  TError = DefaultError,
> = UseInfiniteQueryResult<TData, TError>;

export type DxDefinedUseInfiniteQueryResult<
  TData = unknown,
  TError = DefaultError,
> = DefinedUseInfiniteQueryResult<TData, TError>;

export type DxUseSuspenseInfiniteQueryResult<
  TData = unknown,
  TError = DefaultError,
> = UseSuspenseInfiniteQueryResult<TData, TError>;

export type DxQueryResultSummary = {
  status: QueryStatus;
  fetchStatus: FetchStatus;
  pending: boolean;
  loading: boolean;
  fetching: boolean;
  success: boolean;
  error: boolean;
  stale: boolean;
  placeholder: boolean;
  refetching: boolean;
  paused: boolean;
  enabled: boolean;
  dataUpdatedAt: number;
  errorUpdatedAt: number;
  failureCount: number;
  canRefetch: boolean;
  hasData: boolean;
  hasError: boolean;
};

export type DxInfiniteQueryResultSummary = DxQueryResultSummary & {
  fetchingNextPage: boolean;
  fetchingPreviousPage: boolean;
  hasNextPage: boolean;
  hasPreviousPage: boolean;
};

export function summarizeDxQueryResult<
  TData = unknown,
  TError = DefaultError,
>(
  result: QueryObserverBaseResult<TData, TError>,
): DxQueryResultSummary {
  const hasData = typeof result.data !== "undefined";
  const hasError = result.error !== null;

  return {
    status: result.status,
    fetchStatus: result.fetchStatus,
    pending: result.isPending,
    loading: result.isLoading,
    fetching: result.isFetching,
    success: result.isSuccess,
    error: result.isError,
    stale: result.isStale,
    placeholder: result.isPlaceholderData,
    refetching: result.isRefetching,
    paused: result.isPaused,
    enabled: result.isEnabled,
    dataUpdatedAt: result.dataUpdatedAt,
    errorUpdatedAt: result.errorUpdatedAt,
    failureCount: result.failureCount,
    canRefetch: result.isEnabled && !result.isFetching,
    hasData,
    hasError,
  };
}

export function summarizeDxInfiniteQueryResult<
  TData = unknown,
  TError = DefaultError,
>(
  result: InfiniteQueryObserverBaseResult<TData, TError>,
): DxInfiniteQueryResultSummary {
  return {
    ...summarizeDxQueryResult(result),
    fetchingNextPage: result.isFetchingNextPage,
    fetchingPreviousPage: result.isFetchingPreviousPage,
    hasNextPage: Boolean(result.hasNextPage),
    hasPreviousPage: Boolean(result.hasPreviousPage),
  };
}

export function formatDxQueryResultError<
  TData = unknown,
  TError = unknown,
>(
  result: Pick<QueryObserverBaseResult<TData, TError>, "error" | "isError">,
  fallback = "Query failed",
) {
  if (!result.isError || result.error === null) return "";

  if (result.error instanceof Error && result.error.message.trim().length > 0) {
    return result.error.message;
  }

  if (typeof result.error === "string" && result.error.trim().length > 0) {
    return result.error;
  }

  return fallback;
}
"#;

const TANSTACK_QUERY_DISABLED_TS: &str = r#"import {
  queryOptions,
  skipToken,
  type DefaultError,
  type QueryFunction,
  type QueryKey,
  type SkipToken,
  type UndefinedInitialDataOptions,
} from "@tanstack/react-query";

export const dxSkipToken = skipToken;

export type DxConditionalQueryFn<
  TQueryFnData,
  TQueryKey extends QueryKey,
> = QueryFunction<TQueryFnData, TQueryKey> | SkipToken;

export type DxConditionalQueryOptionsInput<
  TQueryFnData = unknown,
  TError = DefaultError,
  TData = TQueryFnData,
  TQueryKey extends QueryKey = QueryKey,
> = Omit<
  UndefinedInitialDataOptions<TQueryFnData, TError, TData, TQueryKey>,
  "queryFn"
> & {
  queryFn: DxConditionalQueryFn<TQueryFnData, TQueryKey>;
};

export function dxConditionalQueryOptions<
  TQueryFnData = unknown,
  TError = DefaultError,
  TData = TQueryFnData,
  TQueryKey extends QueryKey = QueryKey,
>(
  input: DxConditionalQueryOptionsInput<
    TQueryFnData,
    TError,
    TData,
    TQueryKey
  >,
) {
  return queryOptions(input);
}

export function dxMaybeQueryFn<
  TQueryFnData,
  TQueryKey extends QueryKey,
>(
  enabled: boolean,
  queryFn: QueryFunction<TQueryFnData, TQueryKey>,
): DxConditionalQueryFn<TQueryFnData, TQueryKey> {
  return enabled ? queryFn : dxSkipToken;
}

export function isDxQuerySkipped(queryFn: unknown): queryFn is SkipToken {
  return queryFn === dxSkipToken;
}
"#;

const TANSTACK_QUERY_PLACEHOLDER_TS: &str = r#"import {
  keepPreviousData,
  replaceEqualDeep,
  type DefaultError,
  type PlaceholderDataFunction,
  type QueryKey,
} from "@tanstack/react-query";

export const dxKeepPreviousData = keepPreviousData;

export type DxPlaceholderDataFunction<
  TQueryFnData = unknown,
  TError = DefaultError,
  TQueryData = TQueryFnData,
  TQueryKey extends QueryKey = QueryKey,
> = PlaceholderDataFunction<TQueryFnData, TError, TQueryData, TQueryKey>;

export type DxPlaceholderState = {
  state: "placeholder" | "resolved" | "empty";
  source: "previous-data" | "current-data" | "empty";
  hasData: boolean;
};

export function createDxPlaceholderData<
  TQueryFnData = unknown,
  TError = DefaultError,
  TQueryData = TQueryFnData,
  TQueryKey extends QueryKey = QueryKey,
>(
  fallback?: TQueryData,
): DxPlaceholderDataFunction<
  TQueryFnData,
  TError,
  TQueryData,
  TQueryKey
> {
  return (previousData) => previousData ?? fallback;
}

export function shareDxQueryData<TData>(
  previousData: unknown,
  nextData: TData,
): TData {
  return replaceEqualDeep(previousData, nextData);
}

export function readDxPlaceholderState<TData>(
  data: TData | undefined,
  isPlaceholderData: boolean,
): DxPlaceholderState {
  const hasData = typeof data !== "undefined";

  return {
    hasData,
    state: isPlaceholderData ? "placeholder" : hasData ? "resolved" : "empty",
    source: isPlaceholderData
      ? "previous-data"
      : hasData
        ? "current-data"
        : "empty",
  };
}
"#;

const TANSTACK_QUERY_KEYS_TS: &str = r#"import {
  hashKey,
  partialMatchKey,
  type QueryKey,
} from "@tanstack/react-query";

export type DxQueryKey = QueryKey;

export function dxQueryKey<TQueryKey extends QueryKey>(
  queryKey: TQueryKey,
): TQueryKey {
  return queryKey;
}

export function dxScopedQueryKey<
  TScope extends string,
  TParts extends readonly unknown[],
>(
  scope: TScope,
  ...parts: TParts
): readonly [TScope, ...TParts] {
  return [scope, ...parts] as readonly [TScope, ...TParts];
}

export function hashDxQueryKey(queryKey: QueryKey) {
  return hashKey(queryKey);
}

export function isDxPartialQueryKeyMatch(
  queryKey: QueryKey,
  partialQueryKey: QueryKey,
) {
  return partialMatchKey(queryKey, partialQueryKey);
}
"#;

const TANSTACK_QUERY_STATE_TS: &str = r#"import {
  type FetchStatus,
  type Mutation,
  type MutationFilters,
  type MutationState,
  type MutationStatus,
  type Query,
  type QueryClient,
  type QueryFilters,
  type QueryKey,
  type QueryState,
  type QueryStatus,
  type Updater,
} from "@tanstack/react-query";

export type DxQueryCoreState<TData = unknown, TError = unknown> = QueryState<
  TData,
  TError
>;

export type DxMutationCoreState<
  TData = unknown,
  TError = unknown,
  TVariables = unknown,
  TOnMutateResult = unknown,
> = MutationState<TData, TError, TVariables, TOnMutateResult>;

export type DxQueryCoreStateSummary = {
  state: "ready" | "missing";
  queryHash: string | null;
  queryKey: QueryKey | null;
  status: QueryStatus | "missing";
  fetchStatus: FetchStatus | "missing";
  observers: number;
  active: boolean;
  disabled: boolean;
  stale: boolean;
  invalidated: boolean;
  dataUpdatedAt: number;
  errorUpdatedAt: number;
};

export type DxMutationCoreStateSummary = {
  state: "ready" | "missing";
  mutationId: number | null;
  status: MutationStatus | "missing";
  paused: boolean;
  failureCount: number;
  submittedAt: number;
  hasVariables: boolean;
};

export type DxFindCachedQueryFilters = QueryFilters & {
  queryKey: QueryKey;
};

export function findDxCachedQuery<TData = unknown, TError = unknown>(
  client: QueryClient,
  filters: DxFindCachedQueryFilters,
): Query<unknown, TError, TData> | undefined {
  return client.getQueryCache().find(filters) as
    | Query<unknown, TError, TData>
    | undefined;
}

export function findDxCachedMutation<
  TData = unknown,
  TError = unknown,
  TVariables = unknown,
  TOnMutateResult = unknown,
>(
  client: QueryClient,
  filters: MutationFilters,
): Mutation<TData, TError, TVariables, TOnMutateResult> | undefined {
  return client.getMutationCache().find(filters);
}

export function readDxQueryCoreState<TData = unknown, TError = unknown>(
  query?: Query<unknown, TError, TData> | null,
): QueryState<TData, TError> | undefined {
  return query?.state;
}

export function readDxMutationCoreState<
  TData = unknown,
  TError = unknown,
  TVariables = unknown,
  TOnMutateResult = unknown,
>(
  mutation?: Mutation<TData, TError, TVariables, TOnMutateResult> | null,
): MutationState<TData, TError, TVariables, TOnMutateResult> | undefined {
  return mutation?.state;
}

export function summarizeDxQueryCoreState(
  query?: Query | null,
): DxQueryCoreStateSummary {
  if (!query) {
    return {
      state: "missing",
      queryHash: null,
      queryKey: null,
      status: "missing",
      fetchStatus: "missing",
      observers: 0,
      active: false,
      disabled: false,
      stale: false,
      invalidated: false,
      dataUpdatedAt: 0,
      errorUpdatedAt: 0,
    };
  }

  const state = query.state;

  return {
    state: "ready",
    queryHash: query.queryHash,
    queryKey: query.queryKey,
    status: state.status,
    fetchStatus: state.fetchStatus,
    observers: query.getObserversCount(),
    active: query.isActive(),
    disabled: query.isDisabled(),
    stale: query.isStale(),
    invalidated: state.isInvalidated,
    dataUpdatedAt: state.dataUpdatedAt,
    errorUpdatedAt: state.errorUpdatedAt,
  };
}

export function summarizeDxMutationCoreState(
  mutation?: Mutation | null,
): DxMutationCoreStateSummary {
  if (!mutation) {
    return {
      state: "missing",
      mutationId: null,
      status: "missing",
      paused: false,
      failureCount: 0,
      submittedAt: 0,
      hasVariables: false,
    };
  }

  const state = mutation.state;

  return {
    state: "ready",
    mutationId: mutation.mutationId,
    status: state.status,
    paused: state.isPaused,
    failureCount: state.failureCount,
    submittedAt: state.submittedAt,
    hasVariables: typeof state.variables !== "undefined",
  };
}

export function resolveDxStateUpdater<TInput, TOutput>(
  input: TInput,
  updater: Updater<TInput, TOutput>,
): TOutput {
  return typeof updater === "function"
    ? (updater as (value: TInput) => TOutput)(input)
    : updater;
}
"#;

const TANSTACK_QUERY_INFINITE_TS: &str = r#"import {
  infiniteQueryOptions,
  type DefaultError,
  type FetchInfiniteQueryOptions,
  type InfiniteData,
  type QueryClient,
  type QueryKey,
  type UndefinedInitialDataInfiniteOptions,
} from "@tanstack/react-query";

export const DX_INFINITE_QUERY_DEFAULT_PAGE_SIZE = 20;

export type DxInfiniteItemsPage<TItem, TCursor = unknown> = {
  items: readonly TItem[];
  nextCursor?: TCursor | null;
};

export type DxInfiniteQueryOptionsInput<
  TQueryFnData,
  TError = DefaultError,
  TData = InfiniteData<TQueryFnData>,
  TQueryKey extends QueryKey = QueryKey,
  TPageParam = unknown,
> = UndefinedInitialDataInfiniteOptions<
  TQueryFnData,
  TError,
  TData,
  TQueryKey,
  TPageParam
>;

export type DxPrefetchInfiniteQueryInput<
  TQueryFnData = unknown,
  TError = DefaultError,
  TData = TQueryFnData,
  TQueryKey extends QueryKey = QueryKey,
  TPageParam = unknown,
> = FetchInfiniteQueryOptions<
  TQueryFnData,
  TError,
  TData,
  TQueryKey,
  TPageParam
>;

export function dxInfiniteQueryOptions<
  TQueryFnData,
  TError = DefaultError,
  TData = InfiniteData<TQueryFnData>,
  TQueryKey extends QueryKey = QueryKey,
  TPageParam = unknown,
>(
  input: DxInfiniteQueryOptionsInput<
    TQueryFnData,
    TError,
    TData,
    TQueryKey,
    TPageParam
  >,
) {
  return infiniteQueryOptions(input);
}

export async function prefetchDxInfiniteQuery<
  TQueryFnData = unknown,
  TError = DefaultError,
  TData = TQueryFnData,
  TQueryKey extends QueryKey = QueryKey,
  TPageParam = unknown,
>(
  client: QueryClient,
  input: DxPrefetchInfiniteQueryInput<
    TQueryFnData,
    TError,
    TData,
    TQueryKey,
    TPageParam
  >,
) {
  await client.prefetchInfiniteQuery(input);
  return client;
}

export function flattenDxInfiniteItems<TItem, TCursor = unknown>(
  data:
    | InfiniteData<DxInfiniteItemsPage<TItem, TCursor>, TCursor>
    | undefined,
): TItem[] {
  return data?.pages.flatMap((page) => [...page.items]) ?? [];
}

export function getDxNextPageParam<TCursor>(
  lastPage: { nextCursor?: TCursor | null },
) {
  return lastPage.nextCursor ?? undefined;
}
"#;

const TANSTACK_QUERY_ACTIVITY_TSX: &str = r#""use client";

import type * as React from "react";
import {
  useIsFetching,
  useIsMutating,
  useMutationState,
  type Mutation,
  type MutationFilters,
  type MutationState,
  type QueryClient,
  type QueryFilters,
} from "@tanstack/react-query";

export type DxQueryActivityInput = {
  queryFilters?: QueryFilters;
  mutationFilters?: MutationFilters;
  client?: QueryClient;
};

export type DxQueryActivity = {
  fetching: number;
  mutating: number;
  busy: boolean;
  label: string;
};

export function useDxQueryActivity({
  queryFilters,
  mutationFilters,
  client,
}: DxQueryActivityInput = {}): DxQueryActivity {
  const fetching = useIsFetching(queryFilters, client);
  const mutating = useIsMutating(mutationFilters, client);

  return {
    fetching,
    mutating,
    busy: fetching + mutating > 0,
    label: formatDxQueryActivityLabel({ fetching, mutating }),
  };
}

export type DxMutationStateOptions<TResult = MutationState> = {
  filters?: MutationFilters;
  select?: (mutation: Mutation) => TResult;
  client?: QueryClient;
};

export function useDxPendingMutationState<TResult = MutationState>({
  filters,
  select,
  client,
}: DxMutationStateOptions<TResult> = {}) {
  return useMutationState<TResult>(
    {
      filters: { ...filters, status: filters?.status ?? "pending" },
      select,
    },
    client,
  );
}

export function getDxLatestMutationState<TResult>(
  states: readonly TResult[],
): TResult | undefined {
  return states.length > 0 ? states[states.length - 1] : undefined;
}

export function formatDxQueryActivityLabel({
  fetching,
  mutating,
}: Pick<DxQueryActivity, "fetching" | "mutating">) {
  if (fetching > 0 && mutating > 0) {
    return `${fetching} fetching, ${mutating} mutating`;
  }

  if (fetching > 0) {
    return `${fetching} fetching`;
  }

  if (mutating > 0) {
    return `${mutating} mutating`;
  }

  return "Cache idle";
}

export type DxQueryActivityStatusProps =
  React.HTMLAttributes<HTMLSpanElement> & DxQueryActivityInput;

export function DxQueryActivityStatus({
  children,
  queryFilters,
  mutationFilters,
  client,
  ...props
}: DxQueryActivityStatusProps) {
  const activity = useDxQueryActivity({
    queryFilters,
    mutationFilters,
    client,
  });

  return (
    <span
      {...props}
      data-query-activity-state={activity.busy ? "busy" : "idle"}
      data-query-fetching-count={activity.fetching}
      data-query-mutating-count={activity.mutating}
    >
      {children ?? activity.label}
    </span>
  );
}
"#;

const TANSTACK_QUERY_OBSERVERS_TS: &str = r#"import {
  InfiniteQueryObserver,
  MutationObserver,
  QueriesObserver,
  QueryObserver,
  type DefaultError,
  type InfiniteData,
  type InfiniteQueryObserverOptions,
  type InfiniteQueryObserverResult,
  type MutationObserverOptions,
  type MutationObserverResult,
  type QueriesObserverOptions,
  type QueryClient,
  type QueryKey,
  type QueryObserverOptions,
  type QueryObserverResult,
} from "@tanstack/react-query";

export type DxObserverUnsubscribe = () => void;

export function createDxQueryObserver<
  TQueryFnData = unknown,
  TError = DefaultError,
  TData = TQueryFnData,
  TQueryData = TQueryFnData,
  TQueryKey extends QueryKey = QueryKey,
>(
  client: QueryClient,
  options: QueryObserverOptions<
    TQueryFnData,
    TError,
    TData,
    TQueryData,
    TQueryKey
  >,
) {
  return new QueryObserver<
    TQueryFnData,
    TError,
    TData,
    TQueryData,
    TQueryKey
  >(client, options);
}

export function createDxInfiniteQueryObserver<
  TQueryFnData = unknown,
  TError = DefaultError,
  TData = InfiniteData<TQueryFnData>,
  TQueryKey extends QueryKey = QueryKey,
  TPageParam = unknown,
>(
  client: QueryClient,
  options: InfiniteQueryObserverOptions<
    TQueryFnData,
    TError,
    TData,
    TQueryKey,
    TPageParam
  >,
) {
  return new InfiniteQueryObserver<
    TQueryFnData,
    TError,
    TData,
    TQueryKey,
    TPageParam
  >(client, options);
}

export function createDxMutationObserver<
  TData = unknown,
  TError = DefaultError,
  TVariables = void,
  TOnMutateResult = unknown,
>(
  client: QueryClient,
  options: MutationObserverOptions<
    TData,
    TError,
    TVariables,
    TOnMutateResult
  >,
) {
  return new MutationObserver<
    TData,
    TError,
    TVariables,
    TOnMutateResult
  >(client, options);
}

export function createDxQueriesObserver<
  TCombinedResult = Array<QueryObserverResult>,
>(
  client: QueryClient,
  queries: Array<QueryObserverOptions>,
  options?: QueriesObserverOptions<TCombinedResult>,
) {
  return new QueriesObserver<TCombinedResult>(client, queries, options);
}

export function subscribeDxQueryObserver<
  TQueryFnData = unknown,
  TError = DefaultError,
  TData = TQueryFnData,
  TQueryData = TQueryFnData,
  TQueryKey extends QueryKey = QueryKey,
>(
  observer: QueryObserver<
    TQueryFnData,
    TError,
    TData,
    TQueryData,
    TQueryKey
  >,
  listener: (result: QueryObserverResult<TData, TError>) => void,
): DxObserverUnsubscribe {
  return observer.subscribe(listener);
}

export function readDxQueryObserverSnapshot<
  TQueryFnData = unknown,
  TError = DefaultError,
  TData = TQueryFnData,
  TQueryData = TQueryFnData,
  TQueryKey extends QueryKey = QueryKey,
>(
  observer: QueryObserver<
    TQueryFnData,
    TError,
    TData,
    TQueryData,
    TQueryKey
  >,
): QueryObserverResult<TData, TError> {
  return observer.getCurrentResult();
}

export function readDxInfiniteQueryObserverSnapshot<
  TQueryFnData = unknown,
  TError = DefaultError,
  TData = InfiniteData<TQueryFnData>,
  TQueryKey extends QueryKey = QueryKey,
  TPageParam = unknown,
>(
  observer: InfiniteQueryObserver<
    TQueryFnData,
    TError,
    TData,
    TQueryKey,
    TPageParam
  >,
): InfiniteQueryObserverResult<TData, TError> {
  return observer.getCurrentResult();
}

export function readDxMutationObserverSnapshot<
  TData = unknown,
  TError = DefaultError,
  TVariables = void,
  TOnMutateResult = unknown,
>(
  observer: MutationObserver<
    TData,
    TError,
    TVariables,
    TOnMutateResult
  >,
): MutationObserverResult<TData, TError, TVariables, TOnMutateResult> {
  return observer.getCurrentResult();
}

export function readDxQueriesObserverSnapshot(
  observer: QueriesObserver,
): Array<QueryObserverResult> {
  return observer.getCurrentResult();
}
"#;

const TANSTACK_QUERY_CACHE_TS: &str = r#"import {
  type CancelOptions,
  type DefaultError,
  type EnsureQueryDataOptions,
  type MutationFilters,
  type QueryClient,
  type QueryFilters,
  type QueryKey,
  type QueryState,
  type RefetchOptions,
  type RefetchQueryFilters,
  type ResetOptions,
  type SetDataOptions,
  type Updater,
} from "@tanstack/react-query";

export type DxQueryDataUpdater<TData> = Updater<
  TData | undefined,
  TData | undefined
>;

export type DxQueryState<TData = unknown, TError = DefaultError> = QueryState<
  TData,
  TError
>;

export function readDxQueryData<
  TData = unknown,
  TQueryKey extends QueryKey = QueryKey,
>(client: QueryClient, queryKey: TQueryKey) {
  return client.getQueryData<TData, TQueryKey>(queryKey);
}

export function readDxQueryState<
  TData = unknown,
  TError = DefaultError,
  TQueryKey extends QueryKey = QueryKey,
>(
  client: QueryClient,
  queryKey: TQueryKey,
): DxQueryState<TData, TError> | undefined {
  return client.getQueryState<TData, TError, TQueryKey, TData, TError>(
    queryKey,
  );
}

export function countDxFetchingQueries<
  TQueryKey extends QueryKey = QueryKey,
>(client: QueryClient, filters?: QueryFilters<TQueryKey>) {
  return client.isFetching(filters);
}

export function countDxMutatingRequests<
  TData = unknown,
  TError = DefaultError,
  TVariables = unknown,
  TOnMutateResult = unknown,
>(
  client: QueryClient,
  filters?: MutationFilters<TData, TError, TVariables, TOnMutateResult>,
) {
  return client.isMutating(filters);
}

export function writeDxQueryData<
  TData = unknown,
  TQueryKey extends QueryKey = QueryKey,
>(
  client: QueryClient,
  queryKey: TQueryKey,
  updater: DxQueryDataUpdater<TData>,
  options?: SetDataOptions,
) {
  return client.setQueryData<TData, TQueryKey>(queryKey, updater, options);
}

export function readDxQueriesData<
  TData = unknown,
  TQueryKey extends QueryKey = QueryKey,
>(
  client: QueryClient,
  filters: QueryFilters<TQueryKey>,
) {
  return client.getQueriesData<TData, QueryFilters<TQueryKey>>(filters);
}

export function writeDxQueriesData<
  TData = unknown,
  TQueryKey extends QueryKey = QueryKey,
>(
  client: QueryClient,
  filters: QueryFilters<TQueryKey>,
  updater: DxQueryDataUpdater<TData>,
  options?: SetDataOptions,
) {
  return client.setQueriesData<TData, QueryFilters<TQueryKey>>(
    filters,
    updater,
    options,
  );
}

export async function ensureDxQueryData<
  TQueryFnData,
  TError = DefaultError,
  TData = TQueryFnData,
  TQueryKey extends QueryKey = QueryKey,
>(
  client: QueryClient,
  input: EnsureQueryDataOptions<TQueryFnData, TError, TData, TQueryKey>,
) {
  return client.ensureQueryData(input);
}

export async function refetchDxQueries<
  TQueryKey extends QueryKey = QueryKey,
>(
  client: QueryClient,
  filters?: RefetchQueryFilters<TQueryKey>,
  options?: RefetchOptions,
) {
  await client.refetchQueries(filters, options);
  return client;
}

export async function resetDxQueries<
  TQueryKey extends QueryKey = QueryKey,
>(
  client: QueryClient,
  filters?: QueryFilters<TQueryKey>,
  options?: ResetOptions,
) {
  await client.resetQueries(filters, options);
  return client;
}

export async function cancelDxQueries<
  TQueryKey extends QueryKey = QueryKey,
>(
  client: QueryClient,
  filters?: QueryFilters<TQueryKey>,
  options?: CancelOptions,
) {
  await client.cancelQueries(filters, options);
  return client;
}

export function removeDxQueries<TQueryKey extends QueryKey = QueryKey>(
  client: QueryClient,
  filters?: QueryFilters<TQueryKey>,
) {
  client.removeQueries(filters);
  return client;
}

export function dxExactQueryFilter<TQueryKey extends QueryKey>(
  queryKey: TQueryKey,
): QueryFilters<TQueryKey> {
  return { queryKey, exact: true };
}
"#;

const TANSTACK_QUERY_CACHE_EVENTS_TS: &str = r#"import {
  MutationCache,
  QueryCache,
  notifyManager,
  type MutationCacheNotifyEvent,
  type QueryCacheNotifyEvent,
  type QueryClient,
} from "@tanstack/react-query";

export type DxQueryCacheEvent = QueryCacheNotifyEvent;
export type DxMutationCacheEvent = MutationCacheNotifyEvent;

export type DxQueryCacheEventListener = (event: DxQueryCacheEvent) => void;
export type DxMutationCacheEventListener = (
  event: DxMutationCacheEvent,
) => void;

export type DxQueryCacheSources = {
  queryCache: QueryCache;
  mutationCache?: MutationCache;
};

export type DxInstrumentedQueryCaches = DxQueryCacheSources & {
  dispose: () => void;
};

export type DxQueryCacheSummary = {
  queries: number;
  mutations: number;
  fetching: number;
  pausedMutations: number;
  state: "idle" | "active" | "paused";
  latestQueryHash?: string;
  latestMutationKey?: unknown;
};

export function createDxQueryCache(
  config?: ConstructorParameters<typeof QueryCache>[0],
) {
  return new QueryCache(config);
}

export function createDxMutationCache(
  config?: ConstructorParameters<typeof MutationCache>[0],
) {
  return new MutationCache(config);
}

export function subscribeDxQueryCacheEvents(
  cache: QueryCache,
  listener: DxQueryCacheEventListener,
) {
  return cache.subscribe(listener);
}

export function subscribeDxMutationCacheEvents(
  cache: MutationCache,
  listener: DxMutationCacheEventListener,
) {
  return cache.subscribe(listener);
}

export function createDxInstrumentedQueryCaches({
  onQueryEvent,
  onMutationEvent,
}: {
  onQueryEvent?: DxQueryCacheEventListener;
  onMutationEvent?: DxMutationCacheEventListener;
} = {}): DxInstrumentedQueryCaches {
  const queryCache = createDxQueryCache();
  const mutationCache = createDxMutationCache();
  const unsubscribeQuery = onQueryEvent
    ? subscribeDxQueryCacheEvents(queryCache, onQueryEvent)
    : undefined;
  const unsubscribeMutation = onMutationEvent
    ? subscribeDxMutationCacheEvents(mutationCache, onMutationEvent)
    : undefined;

  return {
    queryCache,
    mutationCache,
    dispose: () => {
      unsubscribeQuery?.();
      unsubscribeMutation?.();
    },
  };
}

export function dxBatchQueryCacheNotifications<T>(callback: () => T): T {
  return notifyManager.batch(callback);
}

export function summarizeDxQueryCache(
  input: QueryClient | DxQueryCacheSources,
): DxQueryCacheSummary {
  const { queryCache, mutationCache } = resolveDxQueryCacheSources(input);
  const queries = queryCache.getAll();
  const mutations = mutationCache?.getAll() ?? [];
  const latestQuery = queries[queries.length - 1];
  const latestMutation = mutations[mutations.length - 1];
  const fetching = queries.filter(
    (query) => query.state.fetchStatus === "fetching",
  ).length;
  const pausedMutations = mutations.filter(
    (mutation) => mutation.state.isPaused,
  ).length;

  return {
    queries: queries.length,
    mutations: mutations.length,
    fetching,
    pausedMutations,
    state:
      pausedMutations > 0 ? "paused" : fetching > 0 ? "active" : "idle",
    latestQueryHash: latestQuery?.queryHash,
    latestMutationKey: latestMutation?.options.mutationKey,
  };
}

function resolveDxQueryCacheSources(
  input: QueryClient | DxQueryCacheSources,
): DxQueryCacheSources {
  if ("getQueryCache" in input && typeof input.getQueryCache === "function") {
    return {
      queryCache: input.getQueryCache(),
      mutationCache: input.getMutationCache(),
    };
  }

  return input;
}
"#;

const TANSTACK_QUERY_MATCHES_TS: &str = r#"import {
  matchMutation,
  matchQuery,
  type Mutation,
  type MutationFilters,
  type Query,
  type QueryClient,
  type QueryFilters,
} from "@tanstack/react-query";

export type DxCacheMatchSummary = {
  queries: number;
  mutations: number;
  hasQueryMatch: boolean;
  hasMutationMatch: boolean;
};

export type DxMatchedQuery = Query<any, any, any, any>;
export type DxMatchedMutation = Mutation<any, any, any, any>;

export function doesDxQueryMatch(
  filters: QueryFilters,
  query: DxMatchedQuery,
): boolean {
  return matchQuery(filters, query);
}

export function doesDxMutationMatch(
  filters: MutationFilters,
  mutation: DxMatchedMutation,
): boolean {
  return matchMutation(filters, mutation);
}

export function getDxMatchingQueries(
  client: QueryClient,
  filters: QueryFilters = {},
): Array<DxMatchedQuery> {
  return client
    .getQueryCache()
    .getAll()
    .filter((query) => doesDxQueryMatch(filters, query));
}

export function getDxMatchingMutations(
  client: QueryClient,
  filters: MutationFilters = {},
): Array<DxMatchedMutation> {
  return client
    .getMutationCache()
    .getAll()
    .filter((mutation) => doesDxMutationMatch(filters, mutation));
}

export function countDxMatchingQueries(
  client: QueryClient,
  filters: QueryFilters = {},
): number {
  return getDxMatchingQueries(client, filters).length;
}

export function countDxMatchingMutations(
  client: QueryClient,
  filters: MutationFilters = {},
): number {
  return getDxMatchingMutations(client, filters).length;
}

export function summarizeDxCacheMatches(
  client: QueryClient,
  queryFilters: QueryFilters = {},
  mutationFilters: MutationFilters = {},
): DxCacheMatchSummary {
  const queries = countDxMatchingQueries(client, queryFilters);
  const mutations = countDxMatchingMutations(client, mutationFilters);

  return {
    queries,
    mutations,
    hasQueryMatch: queries > 0,
    hasMutationMatch: mutations > 0,
  };
}
"#;

const TANSTACK_QUERY_QUERIES_TSX: &str = r#""use client";

import type * as React from "react";
import {
  useQueries,
  type QueriesOptions,
  type QueriesResults,
  type QueryClient,
  type UseQueryResult,
} from "@tanstack/react-query";

export type DxQueriesInput<
  TQueries extends Array<any>,
  TCombinedResult = QueriesResults<TQueries>,
> = {
  queries: readonly [...QueriesOptions<TQueries>];
  combine?: (result: QueriesResults<TQueries>) => TCombinedResult;
  subscribed?: boolean;
  client?: QueryClient;
};

export function useDxQueries<
  TQueries extends Array<any>,
  TCombinedResult = QueriesResults<TQueries>,
>({
  queries,
  combine,
  subscribed,
  client,
}: DxQueriesInput<TQueries, TCombinedResult>): TCombinedResult {
  return useQueries({ queries, combine, subscribed }, client);
}

export type DxQueryResultState = Pick<
  UseQueryResult,
  "isPending" | "isFetching" | "isError" | "isSuccess"
>;

export type DxQueriesSummaryState =
  | "idle"
  | "pending"
  | "error"
  | "success"
  | "mixed";

export type DxQueriesSummary = {
  total: number;
  active: number;
  pending: number;
  fetching: number;
  errors: number;
  success: number;
  state: DxQueriesSummaryState;
};

export function getDxQueriesSummary(
  results: readonly DxQueryResultState[],
): DxQueriesSummary {
  const pending = results.filter((result) => result.isPending).length;
  const fetching = results.filter((result) => result.isFetching).length;
  const errors = results.filter((result) => result.isError).length;
  const success = results.filter((result) => result.isSuccess).length;
  const active = results.filter(
    (result) => result.isPending || result.isFetching,
  ).length;
  const total = results.length;

  return {
    total,
    active,
    pending,
    fetching,
    errors,
    success,
    state: getDxQueriesSummaryState({ total, active, errors, success }),
  };
}

export function formatDxQueriesSummary(summary: DxQueriesSummary) {
  if (summary.state === "idle") return "No queries";
  if (summary.state === "error") {
    return formatQueryCount(summary.errors, "query failed", "queries failed");
  }
  if (summary.state === "pending") {
    return formatQueryCount(summary.active, "query active", "queries active");
  }
  if (summary.state === "success") return "All queries ready";

  return `${summary.success}/${summary.total} queries ready`;
}

export type DxQueriesStatusProps =
  React.HTMLAttributes<HTMLSpanElement> & {
    results: readonly DxQueryResultState[];
    label?: (summary: DxQueriesSummary) => React.ReactNode;
  };

export function DxQueriesStatus({
  children,
  results,
  label,
  ...props
}: DxQueriesStatusProps) {
  const summary = getDxQueriesSummary(results);

  return (
    <span
      {...props}
      data-query-batch-state={summary.state}
      data-query-batch-total={summary.total}
      data-query-batch-active={summary.active}
      data-query-batch-errors={summary.errors}
    >
      {children ?? label?.(summary) ?? formatDxQueriesSummary(summary)}
    </span>
  );
}

function getDxQueriesSummaryState({
  total,
  active,
  errors,
  success,
}: Pick<DxQueriesSummary, "total" | "active" | "errors" | "success">): DxQueriesSummaryState {
  if (total === 0) return "idle";
  if (errors > 0) return "error";
  if (active > 0) return "pending";
  if (success === total) return "success";
  return "mixed";
}

function formatQueryCount(count: number, singular: string, plural: string) {
  return `${count} ${count === 1 ? singular : plural}`;
}
"#;

const TANSTACK_QUERY_LIFECYCLE_TSX: &str = r#""use client";

import * as React from "react";
import { focusManager, onlineManager } from "@tanstack/react-query";

export type DxQueryFocusEventSetup = (
  setFocused: (focused?: boolean) => void,
) => (() => void) | undefined;

export type DxQueryOnlineEventSetup = (
  setOnline: (online: boolean) => void,
) => (() => void) | undefined;

export type DxQueryLifecycleBridge = {
  focus?: DxQueryFocusEventSetup;
  online?: DxQueryOnlineEventSetup;
};

export type DxQueryLifecycleStatus = {
  focused: boolean;
  online: boolean;
  paused: boolean;
  label: string;
};

export function installDxQueryLifecycleBridge({
  focus,
  online,
}: DxQueryLifecycleBridge = {}) {
  if (focus) {
    focusManager.setEventListener(focus);
  }

  if (online) {
    onlineManager.setEventListener(online);
  }

  return readDxQueryLifecycleStatus();
}

export function setDxQueryFocused(focused?: boolean) {
  focusManager.setFocused(focused);
  return readDxQueryLifecycleStatus();
}

export function setDxQueryOnline(online: boolean) {
  onlineManager.setOnline(online);
  return readDxQueryLifecycleStatus();
}

export function readDxQueryLifecycleStatus(): DxQueryLifecycleStatus {
  const focused = focusManager.isFocused();
  const online = onlineManager.isOnline();

  return {
    focused,
    online,
    paused: !online,
    label: formatDxQueryLifecycleLabel({ focused, online }),
  };
}

export function subscribeDxQueryLifecycle(
  listener: (status: DxQueryLifecycleStatus) => void,
) {
  const notify = () => listener(readDxQueryLifecycleStatus());
  const unsubscribeFocus = focusManager.subscribe(notify);
  const unsubscribeOnline = onlineManager.subscribe(notify);

  notify();

  return () => {
    unsubscribeFocus();
    unsubscribeOnline();
  };
}

export function useDxQueryLifecycleStatus() {
  const [status, setStatus] = React.useState(readDxQueryLifecycleStatus);

  React.useEffect(() => subscribeDxQueryLifecycle(setStatus), []);

  return status;
}

export type DxQueryLifecycleStatusProps =
  React.HTMLAttributes<HTMLSpanElement> & {
    label?: (status: DxQueryLifecycleStatus) => React.ReactNode;
  };

export function DxQueryLifecycleStatus({
  children,
  label,
  ...props
}: DxQueryLifecycleStatusProps) {
  const status = useDxQueryLifecycleStatus();

  return (
    <span
      {...props}
      data-query-online-state={status.online ? "online" : "offline"}
      data-query-focus-state={status.focused ? "focused" : "background"}
      data-query-paused-state={status.paused ? "paused" : "ready"}
    >
      {children ?? label?.(status) ?? status.label}
    </span>
  );
}

export function formatDxQueryLifecycleLabel({
  focused,
  online,
}: Pick<DxQueryLifecycleStatus, "focused" | "online">) {
  if (!online) return "Offline";
  return focused ? "Online and focused" : "Online in background";
}
"#;

const TANSTACK_QUERY_RUNTIME_TS: &str = r#"import {
  defaultScheduler,
  environmentManager,
  isServer,
  timeoutManager,
  type ManagedTimerId,
  type TimeoutCallback,
  type TimeoutProvider,
} from "@tanstack/react-query";

export type DxQueryRuntimeEnvironment = "server" | "client";

export type DxQueryServerDetector = () => boolean;

export type DxQueryScheduledCallback = () => void;

export type DxQueryTimeoutProvider<TTimerId extends ManagedTimerId = ManagedTimerId> =
  TimeoutProvider<TTimerId>;

export type DxQueryRuntimeStatus = {
  environment: DxQueryRuntimeEnvironment;
  server: boolean;
  defaultIsServer: boolean;
};

export function readDxQueryRuntimeStatus(): DxQueryRuntimeStatus {
  const server = environmentManager.isServer();

  return {
    environment: server ? "server" : "client",
    server,
    defaultIsServer: isServer,
  };
}

export function setDxQueryServerEnvironment(
  isServerValue: DxQueryServerDetector,
): DxQueryRuntimeStatus {
  environmentManager.setIsServer(isServerValue);
  return readDxQueryRuntimeStatus();
}

export function installDxQueryTimeoutProvider<
  TTimerId extends ManagedTimerId,
>(
  provider: DxQueryTimeoutProvider<TTimerId>,
): DxQueryRuntimeStatus {
  timeoutManager.setTimeoutProvider(provider);
  return readDxQueryRuntimeStatus();
}

export function scheduleDxQueryTick(
  callback: DxQueryScheduledCallback,
): void {
  defaultScheduler(callback);
}

export function setDxQueryTimeout(
  callback: TimeoutCallback,
  delay: number,
): ManagedTimerId {
  return timeoutManager.setTimeout(callback, delay);
}

export function clearDxQueryTimeout(
  timeoutId: ManagedTimerId | undefined,
): void {
  timeoutManager.clearTimeout(timeoutId);
}

export function setDxQueryInterval(
  callback: TimeoutCallback,
  delay: number,
): ManagedTimerId {
  return timeoutManager.setInterval(callback, delay);
}

export function clearDxQueryInterval(
  intervalId: ManagedTimerId | undefined,
): void {
  timeoutManager.clearInterval(intervalId);
}
"#;

const TANSTACK_QUERY_STREAM_TS: &str = r#"import {
  experimental_streamedQuery,
  type QueryFunction,
  type QueryFunctionContext,
  type QueryKey,
} from "@tanstack/react-query";

export type DxStreamRefetchMode = "append" | "reset" | "replace";

export type DxStreamFunction<
  TChunk,
  TQueryKey extends QueryKey = QueryKey,
> = (
  context: QueryFunctionContext<TQueryKey>,
) => AsyncIterable<TChunk> | Promise<AsyncIterable<TChunk>>;

export type DxStreamedQueryInput<
  TChunk,
  TData,
  TQueryKey extends QueryKey = QueryKey,
> = {
  streamFn: DxStreamFunction<TChunk, TQueryKey>;
  reducer: (current: TData, chunk: TChunk) => TData;
  initialValue: TData;
  refetchMode?: DxStreamRefetchMode;
};

export type DxStreamedArrayQueryInput<
  TChunk,
  TQueryKey extends QueryKey = QueryKey,
> = {
  streamFn: DxStreamFunction<TChunk, TQueryKey>;
  refetchMode?: DxStreamRefetchMode;
};

export function dxStreamedQuery<
  TChunk,
  TData,
  TQueryKey extends QueryKey = QueryKey,
>(
  input: DxStreamedQueryInput<TChunk, TData, TQueryKey>,
): QueryFunction<TData, TQueryKey> {
  return experimental_streamedQuery(input);
}

export function dxStreamedArrayQuery<
  TChunk,
  TQueryKey extends QueryKey = QueryKey,
>(
  input: DxStreamedArrayQueryInput<TChunk, TQueryKey>,
): QueryFunction<TChunk[], TQueryKey> {
  return experimental_streamedQuery(input);
}

export function dxStreamedTextQuery<TQueryKey extends QueryKey = QueryKey>(
  input: DxStreamedArrayQueryInput<string, TQueryKey>,
): QueryFunction<string, TQueryKey> {
  return dxStreamedQuery({
    ...input,
    initialValue: "",
    reducer: (text, chunk) => text + chunk,
  });
}

export async function* decodeDxUtf8Stream(
  stream: ReadableStream<Uint8Array> | null | undefined,
): AsyncIterable<string> {
  if (!stream) return;

  const reader = stream.getReader();
  const decoder = new TextDecoder();

  try {
    while (true) {
      const { done, value } = await reader.read();

      if (done) break;
      if (!value) continue;

      const text = decoder.decode(value, { stream: true });
      if (text.length > 0) yield text;
    }

    const remaining = decoder.decode();
    if (remaining.length > 0) yield remaining;
  } finally {
    reader.releaseLock();
  }
}
"#;

const TANSTACK_QUERY_SUSPENSE_TSX: &str = r#""use client";

import {
  useSuspenseInfiniteQuery,
  useSuspenseQueries,
  useSuspenseQuery,
  type DefaultError,
  type InfiniteData,
  type QueryClient,
  type QueryKey,
  type SuspenseQueriesOptions,
  type SuspenseQueriesResults,
  type UseSuspenseInfiniteQueryOptions,
  type UseSuspenseInfiniteQueryResult,
  type UseSuspenseQueryOptions,
  type UseSuspenseQueryResult,
} from "@tanstack/react-query";

export type DxSuspenseQueryOptionsInput<
  TQueryFnData = unknown,
  TError = DefaultError,
  TData = TQueryFnData,
  TQueryKey extends QueryKey = QueryKey,
> = UseSuspenseQueryOptions<TQueryFnData, TError, TData, TQueryKey>;

export type DxSuspenseQueryResult<
  TData = unknown,
  TError = DefaultError,
> = UseSuspenseQueryResult<TData, TError>;

export function useDxSuspenseQuery<
  TQueryFnData = unknown,
  TError = DefaultError,
  TData = TQueryFnData,
  TQueryKey extends QueryKey = QueryKey,
>(
  input: DxSuspenseQueryOptionsInput<
    TQueryFnData,
    TError,
    TData,
    TQueryKey
  >,
  client?: QueryClient,
) {
  return useSuspenseQuery(input, client);
}

export type DxSuspenseInfiniteQueryOptionsInput<
  TQueryFnData = unknown,
  TError = DefaultError,
  TData = InfiniteData<TQueryFnData>,
  TQueryKey extends QueryKey = QueryKey,
  TPageParam = unknown,
> = UseSuspenseInfiniteQueryOptions<
  TQueryFnData,
  TError,
  TData,
  TQueryKey,
  TPageParam
>;

export type DxSuspenseInfiniteQueryResult<
  TData = unknown,
  TError = DefaultError,
> = UseSuspenseInfiniteQueryResult<TData, TError>;

export function useDxSuspenseInfiniteQuery<
  TQueryFnData = unknown,
  TError = DefaultError,
  TData = InfiniteData<TQueryFnData>,
  TQueryKey extends QueryKey = QueryKey,
  TPageParam = unknown,
>(
  input: DxSuspenseInfiniteQueryOptionsInput<
    TQueryFnData,
    TError,
    TData,
    TQueryKey,
    TPageParam
  >,
  client?: QueryClient,
) {
  return useSuspenseInfiniteQuery(input, client);
}

export type DxSuspenseQueriesInput<
  TQueries extends Array<any>,
  TCombinedResult = SuspenseQueriesResults<TQueries>,
> = {
  queries: readonly [...SuspenseQueriesOptions<TQueries>];
  combine?: (result: SuspenseQueriesResults<TQueries>) => TCombinedResult;
};

export function useDxSuspenseQueries<
  TQueries extends Array<any>,
  TCombinedResult = SuspenseQueriesResults<TQueries>,
>(
  input: DxSuspenseQueriesInput<TQueries, TCombinedResult>,
  client?: QueryClient,
): TCombinedResult {
  return useSuspenseQueries(input, client);
}
"#;

const TANSTACK_QUERY_PREFETCH_HOOKS_TSX: &str = r#""use client";

import {
  usePrefetchInfiniteQuery,
  usePrefetchQuery,
  type DefaultError,
  type FetchInfiniteQueryOptions,
  type QueryClient,
  type QueryKey,
  type UsePrefetchQueryOptions,
} from "@tanstack/react-query";

export type DxPrefetchQueryHookInput<
  TQueryFnData = unknown,
  TError = DefaultError,
  TData = TQueryFnData,
  TQueryKey extends QueryKey = QueryKey,
> = UsePrefetchQueryOptions<TQueryFnData, TError, TData, TQueryKey>;

export type DxPrefetchInfiniteQueryHookInput<
  TQueryFnData = unknown,
  TError = DefaultError,
  TData = TQueryFnData,
  TQueryKey extends QueryKey = QueryKey,
  TPageParam = unknown,
> = FetchInfiniteQueryOptions<
  TQueryFnData,
  TError,
  TData,
  TQueryKey,
  TPageParam
>;

export function useDxPrefetchQuery<
  TQueryFnData = unknown,
  TError = DefaultError,
  TData = TQueryFnData,
  TQueryKey extends QueryKey = QueryKey,
>(
  input: DxPrefetchQueryHookInput<TQueryFnData, TError, TData, TQueryKey>,
  client?: QueryClient,
) {
  usePrefetchQuery(input, client);
}

export function useDxPrefetchInfiniteQuery<
  TQueryFnData = unknown,
  TError = DefaultError,
  TData = TQueryFnData,
  TQueryKey extends QueryKey = QueryKey,
  TPageParam = unknown,
>(
  input: DxPrefetchInfiniteQueryHookInput<
    TQueryFnData,
    TError,
    TData,
    TQueryKey,
    TPageParam
  >,
  client?: QueryClient,
) {
  usePrefetchInfiniteQuery(input, client);
}

export type DxPrefetchOnRenderProps =
  | {
      kind?: "query";
      query: DxPrefetchQueryHookInput;
      client?: QueryClient;
    }
  | {
      kind: "infinite";
      query: DxPrefetchInfiniteQueryHookInput;
      client?: QueryClient;
    };

export function DxPrefetchOnRender(props: DxPrefetchOnRenderProps) {
  if (props.kind === "infinite") {
    useDxPrefetchInfiniteQuery(props.query, props.client);
  } else {
    useDxPrefetchQuery(props.query, props.client);
  }

  return null;
}
"#;

const TANSTACK_QUERY_ERRORS_TS: &str = r#"import {
  CancelledError,
  isCancelledError,
  shouldThrowError,
  type CancelOptions,
  type DefaultError,
  type QueryKey,
  type ThrowOnError,
} from "@tanstack/react-query";

export type DxQueryCancelOptions = CancelOptions;

export type DxQueryErrorKind = "cancelled" | "error" | "empty";

export type DxQueryThrowOnError<
  TQueryFnData = unknown,
  TError = DefaultError,
  TQueryData = TQueryFnData,
  TQueryKey extends QueryKey = QueryKey,
> = ThrowOnError<TQueryFnData, TError, TQueryData, TQueryKey>;

export function createDxCancelledQueryError(
  options?: DxQueryCancelOptions,
): CancelledError {
  return new CancelledError(options);
}

export function isDxCancelledQueryError(
  error: unknown,
): error is CancelledError {
  return isCancelledError(error);
}

export function getDxQueryErrorKind(error: unknown): DxQueryErrorKind {
  if (!error) {
    return "empty";
  }

  return isCancelledError(error) ? "cancelled" : "error";
}

export function shouldThrowDxQueryError<
  TThrowOnError extends (...args: Array<any>) => boolean,
>(
  throwOnError: boolean | TThrowOnError | undefined,
  params: Parameters<TThrowOnError>,
): boolean {
  return shouldThrowError(throwOnError, params);
}

export function formatDxQueryErrorMessage(error: unknown): string {
  if (isCancelledError(error)) {
    if (error.silent) {
      return "The query was cancelled silently.";
    }

    if (error.revert) {
      return "The query was cancelled and reverted.";
    }

    return "The query was cancelled.";
  }

  if (error instanceof Error && error.message.trim().length > 0) {
    return error.message;
  }

  if (typeof error === "string" && error.trim().length > 0) {
    return error;
  }

  return "The query request failed. Review the request and retry.";
}

export function describeDxQueryError(error: unknown) {
  return {
    kind: getDxQueryErrorKind(error),
    message: formatDxQueryErrorMessage(error),
    cancelled: isCancelledError(error),
  };
}
"#;

const TANSTACK_QUERY_ERROR_BOUNDARY_TSX: &str = r#""use client";

import * as React from "react";
import {
  QueryErrorResetBoundary,
  useQueryErrorResetBoundary,
  type QueryErrorClearResetFunction,
  type QueryErrorIsResetFunction,
  type QueryErrorResetFunction,
} from "@tanstack/react-query";

import { formatDxQueryErrorMessage } from "./errors";

export type DxQueryErrorResetBoundaryApi = {
  clearReset: QueryErrorClearResetFunction;
  isReset: QueryErrorIsResetFunction;
  reset: QueryErrorResetFunction;
};

export type DxQueryErrorResetBoundaryProps = {
  children: (boundary: DxQueryErrorResetBoundaryApi) => React.ReactNode;
};

export function DxQueryErrorResetBoundary({
  children,
}: DxQueryErrorResetBoundaryProps) {
  return <QueryErrorResetBoundary>{children}</QueryErrorResetBoundary>;
}

export function useDxQueryErrorResetBoundary() {
  return useQueryErrorResetBoundary();
}

export type DxQueryResetButtonProps =
  React.ButtonHTMLAttributes<HTMLButtonElement> & {
    onReset?: QueryErrorResetFunction;
  };

export function DxQueryResetButton({
  children = "Retry",
  onClick,
  onReset,
  type = "button",
  ...props
}: DxQueryResetButtonProps) {
  const boundary = useQueryErrorResetBoundary();

  return (
    <button
      {...props}
      type={type}
      onClick={(event) => {
        boundary.reset();
        onReset?.();
        onClick?.(event);
      }}
    >
      {children}
    </button>
  );
}

export type DxQueryErrorFallbackProps = {
  error: unknown;
  title?: React.ReactNode;
  retryLabel?: React.ReactNode;
  className?: string;
  onRetry?: QueryErrorResetFunction;
};

export function DxQueryErrorFallback({
  error,
  title = "Query failed",
  retryLabel = "Retry query",
  className,
  onRetry,
}: DxQueryErrorFallbackProps) {
  return (
    <div className={className} role="alert">
      <p>{title}</p>
      <p>{formatDxQueryErrorMessage(error)}</p>
      <DxQueryResetButton onReset={onRetry}>{retryLabel}</DxQueryResetButton>
    </div>
  );
}
"#;

const TANSTACK_QUERY_METADATA_TS: &str = r#"export const dxTanstackQueryPackage = {
  packageId: "tanstack/query",
  officialName: "Data Fetching & Cache",
  upstreamPackage: "@tanstack/react-query",
  upstreamCorePackage: "@tanstack/query-core",
  version: "5.100.10",
  forgeVersion: "5.100.10-dx.0",
  aliases: [
    "data-fetching-cache",
    "data-fetching/cache",
    "tanstack-query",
    "react-query",
    "query/tanstack",
    "@tanstack/react-query",
  ],
  sourceMirror: "G:\\WWW\\inspirations\\tanstack-query",
  provenance: {
    upstreamRepository: "https://github.com/TanStack/query",
    localMirror: "G:\\WWW\\inspirations\\tanstack-query",
    inspectedPaths: [
      "packages/query-core/src/queryClient.ts",
      "packages/query-core/src/focusManager.ts",
      "packages/query-core/src/onlineManager.ts",
      "packages/react-query/src/useQuery.ts",
      "packages/react-query/src/useMutation.ts",
      "docs/framework/react/overview.md",
    ],
    sourceOwned: true,
  },
  publicApi: [
    "QueryClient",
    "mount",
    "unmount",
    "clear",
    "resumePausedMutations",
    "getQueryCache",
    "getMutationCache",
    "DefaultOptions",
    "getDefaultOptions",
    "setDefaultOptions",
    "getQueryDefaults",
    "setQueryDefaults",
    "createDxDashboardQueryClient",
    "applyDxDashboardQueryProfile",
    "refreshDxDashboardQuery",
    "getMutationDefaults",
    "setMutationDefaults",
    "QueryClientProvider",
    "QueryClientContext",
    "useQueryClient",
    "QueryClientProviderProps",
    "HydrationBoundary",
    "HydrationBoundaryProps",
    "dehydrate",
    "hydrate",
    "defaultShouldDehydrateQuery",
    "defaultShouldDehydrateMutation",
    "DefinedInitialDataOptions",
    "UndefinedInitialDataOptions",
    "UnusedSkipTokenOptions",
    "DefinedInitialDataInfiniteOptions",
    "UndefinedInitialDataInfiniteOptions",
    "UnusedSkipTokenInfiniteOptions",
    "useQuery",
    "useMutation",
    "useInfiniteQuery",
    "useQueries",
    "useSuspenseQuery",
    "useSuspenseInfiniteQuery",
    "useSuspenseQueries",
    "usePrefetchQuery",
    "usePrefetchInfiniteQuery",
    "useIsRestoring",
    "IsRestoringProvider",
    "PersistQueryClientProvider",
    "persistQueryClient",
    "persistQueryClientRestore",
    "persistQueryClientSave",
    "persistQueryClientSubscribe",
    "createAsyncStoragePersister",
    "createSyncStoragePersister",
    "experimental_createQueryPersister",
    "removeOldestQuery",
    "Persister",
    "PersistedClient",
    "PersistQueryClientOptions",
    "ReactQueryDevtools",
    "ReactQueryDevtoolsPanel",
    "DevtoolsPanelOptions",
    "broadcastQueryClient",
    "BroadcastChannelOptions",
    "ReactQueryStreamedHydration",
    "FetchQueryOptions",
    "FetchInfiniteQueryOptions",
    "EnsureQueryDataOptions",
    "EnsureInfiniteQueryDataOptions",
    "InfiniteData",
    "fetchQuery",
    "fetchInfiniteQuery",
    "ensureQueryData",
    "ensureInfiniteQueryData",
    "experimental_streamedQuery",
    "skipToken",
    "keepPreviousData",
    "replaceEqualDeep",
    "PlaceholderDataFunction",
    "hashKey",
    "partialMatchKey",
    "matchQuery",
    "matchMutation",
    "QueryFilters",
    "MutationFilters",
    "QueryState",
    "MutationState",
    "QueryStatus",
    "FetchStatus",
    "MutationStatus",
    "Updater",
    "getQueryData",
    "getQueriesData",
    "setQueryData",
    "setQueriesData",
    "ensureQueryData",
    "refetchQueries",
    "resetQueries",
    "getQueryState",
    "isFetching",
    "isMutating",
    "QueryCache",
    "MutationCache",
    "Query",
    "Mutation",
    "notifyManager",
    "QueryCacheNotifyEvent",
    "MutationCacheNotifyEvent",
    "focusManager",
    "onlineManager",
    "environmentManager",
    "timeoutManager",
    "defaultScheduler",
    "isServer",
    "ManagedTimerId",
    "TimeoutProvider",
    "CancelledError",
    "isCancelledError",
    "shouldThrowError",
    "CancelOptions",
    "ThrowOnError",
    "cancelQueries",
    "removeQueries",
    "useIsFetching",
    "useIsMutating",
    "useMutationState",
    "queryOptions",
    "infiniteQueryOptions",
    "mutationOptions",
    "QueryObserver",
    "InfiniteQueryObserver",
    "MutationObserver",
    "QueriesObserver",
    "UseMutationResult",
    "UseMutateFunction",
    "UseMutateAsyncFunction",
    "MutateFunction",
    "MutateOptions",
    "MutationFunction",
    "MutationFunctionContext",
    "MutationObserverBaseResult",
    "UseBaseQueryResult",
    "UseQueryResult",
    "DefinedUseQueryResult",
    "UseSuspenseQueryResult",
    "UseInfiniteQueryResult",
    "DefinedUseInfiniteQueryResult",
    "UseSuspenseInfiniteQueryResult",
    "QueryObserverOptions",
    "InfiniteQueryObserverOptions",
    "MutationObserverOptions",
    "QueryObserverBaseResult",
    "QueryObserverResult",
    "InfiniteQueryObserverBaseResult",
    "InfiniteQueryObserverResult",
    "MutationObserverResult",
    "QueryErrorResetBoundary",
    "useQueryErrorResetBoundary",
    "prefetchQuery",
    "prefetchInfiniteQuery",
    "invalidateQueries",
  ],
  materializedFiles: [
    "query/client.ts",
    "query/client-lifecycle.ts",
    "query/defaults.ts",
    "query/dashboard-workflow.ts",
    "query/provider.tsx",
    "query/react-context.tsx",
    "query/restoring.tsx",
    "query/persist.tsx",
    "query/sync-persist.ts",
    "query/devtools.tsx",
    "query/broadcast.ts",
    "query/next-streaming.tsx",
    "query/fetch.ts",
    "query/prefetch.tsx",
    "query/hydration.ts",
    "query/mutation.ts",
    "query/mutation-result.ts",
    "query/query-result.ts",
    "query/disabled.ts",
    "query/placeholder.ts",
    "query/keys.ts",
    "query/state.ts",
    "query/infinite.ts",
    "query/activity.tsx",
    "query/observers.ts",
    "query/cache.ts",
    "query/cache-events.ts",
    "query/matches.ts",
    "query/queries.tsx",
    "query/lifecycle.tsx",
    "query/runtime.ts",
    "query/stream.ts",
    "query/suspense.tsx",
    "query/prefetch-hooks.tsx",
    "query/errors.ts",
    "query/error-boundary.tsx",
    "query/metadata.ts",
    "query/README.md",
  ],
  exportedFiles: [
    "query/client.ts",
    "query/defaults.ts",
    "query/dashboard-workflow.ts",
    "query/provider.tsx",
    "query/react-context.tsx",
    "query/fetch.ts",
    "query/mutation.ts",
    "query/cache.ts",
    "query/lifecycle.tsx",
    "query/metadata.ts",
    "query/README.md",
  ],
  requiredEnv: [],
  appOwnedBoundaries: [
    "Query keys, fetchers, cache invalidation, cache defaults registration order, cache-state inspection policy, loading UI, and error UI",
    "Focus and online manager policy, offline simulation, retry timing, and production network-status copy",
    "Cache persistence storage, retention, privacy review, and broadcast channel naming",
    "Runtime dependency installation and version review for optional devtools, persistence, streaming, and broadcast packages",
  ],
  dxCheckVisibility: {
    schema: "dx.forge.package.dx_check_visibility",
    currentStatus: "present",
    statuses: [
      "present",
      "stale",
      "missing-receipt",
      "blocked",
      "unsupported-surface",
    ],
    receiptPath: "examples/template/.dx/forge/receipts/2026-05-22-tanstack-query-dashboard-data.json",
    monitoredSurfaces: [
      "query/dashboard-workflow.ts",
      "examples/template/query-cache-status.tsx",
      "examples/template/query-dashboard-read-model.ts",
      "examples/dashboard/src/components/QueryDashboardWorkflow.tsx",
    ],
  },
  receiptIntegrity: {
    hashAlgorithm: "sha256",
    trackedFiles: [
      "core/src/ecosystem/forge_tanstack_query.rs",
      "examples/template/query-cache-status.tsx",
      "examples/template/query-dashboard-read-model.ts",
      "examples/template/template-shell.tsx",
      "examples/template/dx-studio-edit-contract.ts",
      "examples/template/template-route-contract.ts",
      "examples/template/package-catalog.ts",
      "tools/launch/runtime-template/pages/index.html",
      "tools/launch/runtime-template/assets/launch-runtime.ts",
      "examples/dashboard/src/lib/queryDashboardWorkflow.ts",
      "examples/dashboard/src/components/QueryDashboardWorkflow.tsx",
    ],
    staleReceiptPolicy:
      "dx-check should mark the Data Fetching & Cache receipt stale when any tracked query dashboard source-surface hash changes without a refreshed receipt.",
  },
  receiptPaths: [
    "examples/template/.dx/forge/receipts/2026-05-22-tanstack-query-dashboard-data.json",
    "docs/packages/tanstack-query.md",
    "examples/dashboard/src/components/QueryDashboardWorkflow.tsx",
    "examples/template/query-cache-status.tsx",
    "examples/template/query-dashboard-read-model.ts",
    "examples/template/dx-studio-edit-contract.ts#tanstack-query-dashboard-data",
    "examples/template/template-route-contract.ts#tanstackQueryDashboardData",
    "tools/launch/runtime-template/pages/index.html#tanstack-query-dashboard-data-workflow",
    "tools/launch/runtime-template/assets/launch-runtime.ts#readLaunchQueryDashboardData",
  ],
  dashboardUsage: {
    route: "/launch",
    component: "tanstack-query-dashboard-data-workflow",
    sourceFile: "examples/template/query-cache-status.tsx",
    readModelSourceFile: "examples/template/query-dashboard-read-model.ts",
    starterDashboardFile: "examples/dashboard/src/components/QueryDashboardWorkflow.tsx",
    runtimePage: "tools/launch/runtime-template/pages/index.html",
    runtimeAsset: "tools/launch/runtime-template/assets/launch-runtime.ts",
    workflow: "query-backed-dashboard-data",
    dxIcon: "pack:tanstack-query",
    packageApi: "query/dashboard-workflow.ts",
    packageQueueMarker: "data-dx-query-dashboard-queue=\"package-readiness\"",
    packageRowMarkers: [
      "data-dx-query-package-id",
      "data-dx-query-package-role",
      "data-dx-query-package-status",
    ],
  },
  requiredDependencies: [
    {
      name: "@tanstack/react-query",
      version: "^5.100.10",
      reason: "Provides the real React hooks, QueryClient, hydration, mutation, and observer APIs.",
    },
    {
      name: "@tanstack/react-query-persist-client",
      version: "^5.100.10",
      reason: "Provides PersistQueryClientProvider for restored React query caches.",
    },
    {
      name: "@tanstack/query-persist-client-core",
      version: "^5.100.10",
      reason: "Provides persistQueryClient, restore/save/subscribe helpers, persister types, and fine-grained query persistence.",
    },
    {
      name: "@tanstack/query-async-storage-persister",
      version: "^5.100.10",
      reason: "Provides createAsyncStoragePersister for browser or custom async storage adapters.",
    },
    {
      name: "@tanstack/query-sync-storage-persister",
      version: "^5.100.10",
      reason: "Provides the deprecated but still real createSyncStoragePersister for legacy synchronous browser storage adapters.",
    },
    {
      name: "@tanstack/react-query-devtools",
      version: "^5.100.10",
      reason: "Provides the real React Query Devtools panel and floating toggle for opt-in launch diagnostics.",
    },
    {
      name: "@tanstack/query-broadcast-client-experimental",
      version: "^5.100.10",
      reason: "Provides broadcastQueryClient for experimental cross-tab query cache sync.",
    },
    {
      name: "broadcast-channel",
      version: "^7.0.0",
      reason: "Provides BroadcastChannelOptions for configuring the upstream query broadcast transport.",
    },
    {
      name: "@tanstack/react-query-next-experimental",
      version: "^5.100.10",
      reason: "Provides ReactQueryStreamedHydration for Next App Router streamed query hydration.",
    },
    {
      name: "next",
      version: "^13 || ^14 || ^15 || ^16",
      reason: "Peer dependency required by @tanstack/react-query-next-experimental for App Router streaming.",
    },
    {
      name: "react",
      version: "^18 || ^19",
      reason: "Peer dependency required by @tanstack/react-query.",
    },
  ],
  cli: {
    dxAdd: "dx add data-fetching-cache --write",
    dxDryRun: "dx add data-fetching-cache --dry-run --format json",
  },
} as const;

export type DxTanstackQueryPackageMetadata = typeof dxTanstackQueryPackage;
"#;

const TANSTACK_QUERY_README_MD: &str = r#"# Data Fetching & Cache Forge Slice

This package materializes a small source-owned adapter around the real upstream `@tanstack/react-query` v5 API. It does not reimplement the upstream cache engine, hide a fake cache, or run package lifecycle scripts.

## Forge Metadata

- Package id: `tanstack/query`.
- Official package name: Data Fetching & Cache.
- Upstream package: `@tanstack/react-query`.
- Aliases: `data-fetching-cache`, `data-fetching/cache`, `tanstack-query`, `react-query`, `query/tanstack`, and `@tanstack/react-query`.
- Source mirror: `G:\WWW\inspirations\tanstack-query`.
- Required env: none.
- App-owned boundaries: query keys, fetchers, invalidation policy, focus/online policy, persistence storage, broadcast channel naming, loading UI, and error UI.
- Receipt paths: `examples/template/.dx/forge/receipts/2026-05-22-tanstack-query-dashboard-data.json`, `docs/packages/tanstack-query.md`, and `examples/template/query-cache-status.tsx`.
- dx-check visibility: `dx.forge.package.dx_check_visibility` reports `present`, `stale`, `missing-receipt`, `blocked`, and `unsupported-surface` states for the selected dashboard workflow.
- Receipt integrity: the dashboard workflow receipt uses `hash_algorithm: sha256` and selected `file_hashes`; dx-check reports `data_fetching_cache_hash_manifest_present` and `data_fetching_cache_hash_mismatch` without claiming live QueryClient runtime proof.

## Owned Files

- `query/client.ts` creates a launch-ready `QueryClient` with conservative defaults.
- `query/client-lifecycle.ts` adds QueryClient lifecycle helpers around `mount`, `unmount`, `clear`, `resumePausedMutations`, `getQueryCache`, and `getMutationCache`.
- `query/defaults.ts` adds default policy helpers around `getDefaultOptions`, `setDefaultOptions`, `getQueryDefaults`, `setQueryDefaults`, `getMutationDefaults`, and `setMutationDefaults`.
- `query/dashboard-workflow.ts` adds dashboard cache profile helpers around `QueryClient`, `setQueryDefaults`, `getQueryDefaults`, `invalidateQueries`, and cache summaries.
- `query/provider.tsx` provides a client component wrapper for `QueryClientProvider` and optional hydration state.
- `query/react-context.tsx` exposes React context helpers around `QueryClientContext`, `useQueryClient`, `QueryClientProviderProps`, `HydrationBoundaryProps`, and the typed query option overload contracts.
- `query/restoring.tsx` adds cache restore-state helpers around `useIsRestoring` and `IsRestoringProvider`.
- `query/persist.tsx` adds persisted cache helpers around `PersistQueryClientProvider`, `persistQueryClient`, `persistQueryClientRestore`, `persistQueryClientSave`, `persistQueryClientSubscribe`, `createAsyncStoragePersister`, `experimental_createQueryPersister`, and `removeOldestQuery`.
- `query/sync-persist.ts` adds legacy sync storage persister helpers around `createSyncStoragePersister`, `PersistRetryer`, `PersistedClient`, `Persister`, and `removeOldestQuery`.
- `query/devtools.tsx` adds opt-in React Query Devtools helpers around `ReactQueryDevtools`, `ReactQueryDevtoolsPanel`, and `DevtoolsPanelOptions`.
- `query/broadcast.ts` adds cross-tab cache sync helpers around `broadcastQueryClient` and `BroadcastChannelOptions`.
- `query/next-streaming.tsx` adds Next App Router streamed hydration helpers around `ReactQueryStreamedHydration`.
- `query/fetch.ts` adds imperative fetch helpers around `fetchQuery`, `prefetchQuery`, `fetchInfiniteQuery`, `prefetchInfiniteQuery`, `ensureQueryData`, and `ensureInfiniteQueryData`.
- `query/prefetch.tsx` exposes a server prefetch helper around `prefetchQuery`, `dehydrate`, and `HydrationBoundary`.
- `query/hydration.ts` adds hydration policy helpers around `hydrate`, `dehydrate`, `defaultShouldDehydrateQuery`, and `defaultShouldDehydrateMutation`.
- `query/mutation.ts` adds typed mutation options and cache invalidation helpers around `mutationOptions` and `QueryClient.invalidateQueries`.
- `query/mutation-result.ts` adds mutation result helpers around `UseMutationResult`, `UseMutateFunction`, `UseMutateAsyncFunction`, `MutateOptions`, `MutationFunctionContext`, and `MutationObserverBaseResult`.
- `query/query-result.ts` adds query result helpers around `UseQueryResult`, `UseBaseQueryResult`, `DefinedUseQueryResult`, `UseInfiniteQueryResult`, `UseSuspenseQueryResult`, `QueryObserverBaseResult`, and `InfiniteQueryObserverBaseResult`.
- `query/disabled.ts` adds typed dependent-query helpers around `skipToken`; use `enabled: false` instead when manual `refetch()` must work.
- `query/placeholder.ts` adds placeholder and structural-sharing helpers around `keepPreviousData` and `replaceEqualDeep`.
- `query/keys.ts` adds deterministic key helpers around `hashKey` and `partialMatchKey` for receipts, cache targeting, and diagnostics.
- `query/state.ts` adds core query and mutation state helpers around `Query`, `Mutation`, `QueryState`, `MutationState`, and `Updater`.
- `query/infinite.ts` adds typed infinite-query helpers around `infiniteQueryOptions`, `prefetchInfiniteQuery`, cursor extraction, and item flattening.
- `query/activity.tsx` adds launch-ready cache activity helpers around `useIsFetching`, `useIsMutating`, and `useMutationState`.
- `query/observers.ts` adds non-React observer bridge helpers around `QueryObserver`, `InfiniteQueryObserver`, `MutationObserver`, and `QueriesObserver`.
- `query/cache.ts` adds typed cache control helpers around `getQueryData`, `getQueryState`, `getQueriesData`, `setQueryData`, `setQueriesData`, `ensureQueryData`, `isFetching`, `isMutating`, `refetchQueries`, `resetQueries`, `cancelQueries`, and `removeQueries`.
- `query/cache-events.ts` adds cache event instrumentation helpers around `QueryCache`, `MutationCache`, and `notifyManager`.
- `query/matches.ts` adds cache matching helpers around `matchQuery`, `matchMutation`, `QueryFilters`, and `MutationFilters`.
- `query/queries.tsx` adds typed parallel query helpers around `useQueries`, `QueriesOptions`, and `QueriesResults`.
- `query/lifecycle.tsx` adds focus and online manager helpers around `focusManager` and `onlineManager`.
- `query/runtime.ts` adds runtime manager helpers around `environmentManager`, `timeoutManager`, `defaultScheduler`, and `isServer`.
- `query/stream.ts` adds streamed query helpers around `experimental_streamedQuery` and AsyncIterable UTF-8 decoding.
- `query/suspense.tsx` adds typed Suspense query helpers around `useSuspenseQuery`, `useSuspenseInfiniteQuery`, and `useSuspenseQueries`.
- `query/prefetch-hooks.tsx` adds render-time prefetch hooks around `usePrefetchQuery` and `usePrefetchInfiniteQuery`.
- `query/errors.ts` adds cancellation and throw-policy helpers around `CancelledError`, `isCancelledError`, and `shouldThrowError`.
- `query/error-boundary.tsx` exposes a tiny reset boundary around `QueryErrorResetBoundary` and `useQueryErrorResetBoundary`.
- `query/metadata.ts` lets DX CLI, Zed, and launch templates discover the upstream package and supported public API surface.

## dx-check Visibility

The dashboard workflow receipt carries `dx.forge.package.dx_check_visibility` with the current source state and the status legend `present`, `stale`, `missing-receipt`, `blocked`, and `unsupported-surface`. It also carries `hash_algorithm: sha256` and selected `file_hashes`, so DX-WWW, dx-check, and Zed can report whether Data Fetching & Cache surfaces are present, stale, missing receipt, blocked, unsupported, or hash-stale without claiming live QueryClient runtime proof.

## Required App Dependency

Install or provide `@tanstack/react-query`, `@tanstack/react-query-persist-client`, `@tanstack/query-persist-client-core`, `@tanstack/query-async-storage-persister`, `@tanstack/query-sync-storage-persister`, `@tanstack/react-query-devtools`, `@tanstack/query-broadcast-client-experimental`, `@tanstack/react-query-next-experimental`, `broadcast-channel`, Next.js, and React in the app runtime when using persistence, devtools, cross-tab sync, or streamed hydration. Forge owns these adapter files and receipts; it does not vendor upstream internals.

Cross-tab sync uses the upstream experimental broadcast client. Applications still own channel naming, privacy review, rollout gates, cache retention policy, and production observability.

Sync storage persistence uses the upstream deprecated sync persister only for legacy localStorage/sessionStorage compatibility. New apps should prefer the async persister; applications still own quota handling, private-browsing behavior, storage migration, and persisted payload review.

Next App Router streamed hydration uses the upstream experimental Next package. Applications still own provider placement, Content Security Policy nonce wiring, transformer choice, serialized payload review, and runtime route verification.

## Template Usage

The launch dashboard uses this slice as a real server-state workflow: it calls `useQuery()` to read the launch package catalog as cached dashboard data, refreshes through a `useMutation()` invalidation path, exposes focus/online controls through `DxQueryLifecycleStatus`, `setDxQueryOnline()`, and `setDxQueryFocused()` from the real upstream `onlineManager` and `focusManager` APIs, and applies cache policy controls through `setDxQueryDefaults()` plus `readDxQueryDefaults()`.

```tsx
import { useQuery } from "@tanstack/react-query";
import { dxQueryOptions } from "@/lib/query/client";

const launchStatusQuery = dxQueryOptions({
  queryKey: ["dx", "launch", "status"] as const,
  queryFn: async ({ signal }) => {
    const response = await fetch("/api/dx/launch/status", { signal });
    if (!response.ok) throw new Error("Unable to load launch status");
    return response.json() as Promise<{ score: number; label: string }>;
  },
});

export function LaunchStatus() {
  const query = useQuery(launchStatusQuery);

  if (query.isPending) return <p>Loading status...</p>;
  if (query.isError) return <p role="alert">{query.error.message}</p>;

  return <p>{query.data.label}: {query.data.score}/100</p>;
}
```

```tsx
import { useMutation, useQueryClient } from "@tanstack/react-query";
import {
  dxMutationOptions,
  invalidateDxQueries,
} from "@/lib/query/mutation";

export function LaunchRefreshButton() {
  const queryClient = useQueryClient();
  const mutation = useMutation(
    dxMutationOptions({
      mutationKey: ["dx", "launch", "refresh"],
      mutationFn: async () =>
        fetch("/api/dx/launch/refresh", { method: "POST" }),
      onSuccess: async () => {
        await invalidateDxQueries(queryClient, [
          ["dx", "launch", "status"] as const,
        ]);
      },
    }),
  );

  return (
    <button
      type="button"
      disabled={mutation.isPending}
      onClick={() => mutation.mutate()}
    >
      Refresh launch status
    </button>
  );
}
```

```tsx
import {
  DxQueryErrorResetBoundary,
  DxQueryResetButton,
} from "@/lib/query/error-boundary";

export function LaunchQueryBoundary({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <DxQueryErrorResetBoundary>
      {({ reset }) => (
        <>
          {children}
          <DxQueryResetButton onReset={reset}>Retry queries</DxQueryResetButton>
        </>
      )}
    </DxQueryErrorResetBoundary>
  );
}
```
"#;
