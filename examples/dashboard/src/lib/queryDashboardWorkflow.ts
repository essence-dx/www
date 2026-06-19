export type QueryDashboardProfileId = 'balanced' | 'live' | 'durable';
export type QueryDashboardPublicApi =
    | 'setQueryDefaults'
    | 'getQueryDefaults'
    | 'invalidateQueries';
export type QueryDashboardRetry = number | false;
export type QueryDashboardDxCheckStatus =
    | 'present'
    | 'stale'
    | 'missing-receipt'
    | 'blocked'
    | 'unsupported-surface';

export type QueryDashboardProfile = {
    id: QueryDashboardProfileId;
    label: string;
    publicApi: QueryDashboardPublicApi;
    dataFreshness: 'balanced' | 'live' | 'durable';
    gcTimeMs: number;
    retry: QueryDashboardRetry;
    staleTimeLabel: string;
    staleTimeMs: number;
    appOwnedBoundary: string;
};

export const queryDashboardReceiptPath =
    'examples/template/.dx/forge/receipts/2026-05-22-tanstack-query-dashboard-data.json' as const;

export type QueryDashboardDxCheckVisibility = {
    schema: 'dx.forge.package.dx_check_visibility';
    packageId: 'tanstack/query';
    officialPackageName: 'Data Fetching & Cache';
    currentStatus: QueryDashboardDxCheckStatus;
    statuses: readonly QueryDashboardDxCheckStatus[];
    receiptPath: typeof queryDashboardReceiptPath;
    monitoredSurfaces: readonly string[];
};

export const queryDashboardDxCheckVisibility = {
    schema: 'dx.forge.package.dx_check_visibility',
    packageId: 'tanstack/query',
    officialPackageName: 'Data Fetching & Cache',
    currentStatus: 'present',
    statuses: [
        'present',
        'stale',
        'missing-receipt',
        'blocked',
        'unsupported-surface',
    ],
    receiptPath: queryDashboardReceiptPath,
    monitoredSurfaces: [
        'query/dashboard-workflow.ts',
        'examples/template/query-cache-status.tsx',
        'examples/template/query-dashboard-read-model.ts',
        'examples/dashboard/src/components/QueryDashboardWorkflow.tsx',
    ],
} as const satisfies QueryDashboardDxCheckVisibility;

export type QueryDashboardReceipt = {
    dashboardWorkflow: 'query-cache-refresh';
    dxCheckVisibility: QueryDashboardDxCheckVisibility;
    nodeModulesRequired: false;
    officialName: 'Data Fetching & Cache';
    packageId: 'tanstack/query';
    publicApi: QueryDashboardPublicApi;
    profileId: QueryDashboardProfileId;
    receiptPath: typeof queryDashboardReceiptPath;
    runtimeExecution: false;
    status: 'local-receipt';
    upstreamPackage: '@tanstack/react-query';
    queryKey: readonly ['dx', 'dashboard', 'overview'];
    cacheDefaults: {
        gcTimeMs: number;
        retry: QueryDashboardRetry;
        staleTimeMs: number;
    };
    cacheAction: string;
    nextAction: string;
};

export const queryDashboardPackage = {
    packageId: 'tanstack/query',
    officialName: 'Data Fetching & Cache',
    upstreamPackage: '@tanstack/react-query',
    aliases: [
        'data-fetching-cache',
        'data-fetching/cache',
        'tanstack-query',
        'react-query',
        'query/tanstack',
        '@tanstack/react-query',
    ],
    sourceMirror: 'G:/WWW/inspirations/tanstack-query',
    requiredEnv: [],
    dashboardEntryPoint: 'QueryDashboardWorkflow',
    publicApis: [
        'QueryClient',
        'useQuery',
        'queryOptions',
        'setQueryDefaults',
        'getQueryDefaults',
        'invalidateQueries',
        'isFetching',
    ],
    exportedFiles: [
        'query/client.ts',
        'query/defaults.ts',
        'query/mutation.ts',
        'query/cache.ts',
        'query/dashboard-workflow.ts',
        'query/metadata.ts',
    ],
    receiptPaths: [
        queryDashboardReceiptPath,
        'docs/packages/tanstack-query.md',
        'examples/dashboard/src/components/QueryDashboardWorkflow.tsx',
        'examples/template/query-dashboard-read-model.ts',
    ],
    dxCheckVisibility: queryDashboardDxCheckVisibility,
    appOwnedBoundaries: [
        'dashboard query keys',
        'fetcher implementation',
        'cache invalidation policy',
        'runtime dependency installation',
    ],
} as const;

export const queryDashboardProfiles: readonly QueryDashboardProfile[] = [
    {
        id: 'balanced',
        label: 'Balanced cache',
        publicApi: 'setQueryDefaults',
        dataFreshness: 'balanced',
        gcTimeMs: 5 * 60_000,
        retry: 2,
        staleTimeLabel: '60s stale time',
        staleTimeMs: 60_000,
        appOwnedBoundary: 'normal dashboard refresh cadence and retry copy',
    },
    {
        id: 'live',
        label: 'Live refresh',
        publicApi: 'invalidateQueries',
        dataFreshness: 'live',
        gcTimeMs: 60_000,
        retry: 1,
        staleTimeLabel: '5s stale time',
        staleTimeMs: 5_000,
        appOwnedBoundary: 'operator polling policy and backend request budget',
    },
    {
        id: 'durable',
        label: 'Durable cache',
        publicApi: 'getQueryDefaults',
        dataFreshness: 'durable',
        gcTimeMs: 30 * 60_000,
        retry: 3,
        staleTimeLabel: '5m stale time',
        staleTimeMs: 5 * 60_000,
        appOwnedBoundary: 'offline expectations and cache-retention review',
    },
];

export function getQueryDashboardProfile(
    profileId: QueryDashboardProfileId,
): QueryDashboardProfile {
    return (
        queryDashboardProfiles.find((profile) => profile.id === profileId) ||
        queryDashboardProfiles[0]
    );
}

export function createQueryDashboardReceipt(
    profileId: QueryDashboardProfileId,
): QueryDashboardReceipt {
    const profile = getQueryDashboardProfile(profileId);

    return {
        dashboardWorkflow: 'query-cache-refresh',
        dxCheckVisibility: queryDashboardDxCheckVisibility,
        nodeModulesRequired: false,
        officialName: queryDashboardPackage.officialName,
        packageId: queryDashboardPackage.packageId,
        publicApi: profile.publicApi,
        profileId: profile.id,
        receiptPath: queryDashboardReceiptPath,
        runtimeExecution: false,
        status: 'local-receipt',
        upstreamPackage: queryDashboardPackage.upstreamPackage,
        queryKey: ['dx', 'dashboard', 'overview'],
        cacheDefaults: {
            gcTimeMs: profile.gcTimeMs,
            retry: profile.retry,
            staleTimeMs: profile.staleTimeMs,
        },
        cacheAction: `${profile.publicApi} is ready for the dashboard overview cache profile.`,
        nextAction:
            'Mount QueryClientProvider, install upstream @tanstack/react-query in the app runtime, and connect the real dashboard fetcher before network execution.',
    };
}
