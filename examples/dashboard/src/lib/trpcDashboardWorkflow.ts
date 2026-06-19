export type TrpcDashboardProcedureId =
    | 'health.query'
    | 'launchEvent.mutation'
    | 'launchEvents.infiniteQuery'
    | 'launchFeed.subscription';

export type TrpcDashboardProcedure = {
    id: TrpcDashboardProcedureId;
    label: string;
    sourceApi: string;
    dashboardUse: string;
    appOwnedBoundary: string;
};

export type TrpcDashboardReceipt = {
    packageId: 'api/trpc';
    procedure: TrpcDashboardProcedureId;
    requestId: string;
    status: 'local-receipt';
    cacheAction: string;
    nextAction: string;
};

export const trpcDashboardPackage = {
    packageId: 'api/trpc',
    officialDxPackageName: 'Type-Safe API',
    packageDisplayName: 'Type-Safe API',
    upstreamPackageName: '@trpc/server',
    aliases: [
        'trpc',
        'trpc/next',
        '@trpc/server',
        '@trpc/client',
        '@trpc/tanstack-react-query',
    ],
    sourceMirror: 'G:/WWW/inspirations/trpc',
    provenance: {
        upstreamRepo: 'trpc/trpc',
        upstreamVersion: '11.17.0',
        inspectedSource: [
            'packages/server/src/unstable-core-do-not-import/initTRPC.ts',
            'packages/server/src/adapters/fetch/fetchRequestHandler.ts',
            'packages/server/src/unstable-core-do-not-import/http/resolveResponse.ts',
            'packages/client/src/createTRPCClient.ts',
            'packages/client/src/links/httpBatchLink.ts',
            'packages/client/src/links/httpBatchStreamLink.ts',
            'packages/client/src/links/httpSubscriptionLink.ts',
            'packages/client/src/links/splitLink.ts',
            'packages/tanstack-react-query/src/internals/createOptionsProxy.ts',
        ],
    },
    requiredEnv: [],
    dashboardEntryPoint: 'TrpcDashboardWorkflow',
    publicApis: [
        'initTRPC.context().create()',
        'fetchRequestHandler',
        'createTRPCClient',
        'httpBatchLink',
        'queryOptions',
        'mutationOptions',
        'infiniteQueryOptions',
        'subscriptionOptions',
    ],
    exportedFiles: [
        'lib/trpc/router.ts',
        'lib/trpc/route-handler.ts',
        'lib/trpc/provider.tsx',
        'lib/trpc/http.ts',
        'lib/trpc/client.ts',
        'lib/trpc/server-caller.ts',
        'lib/trpc/dashboard-workflow.ts',
        'components/dashboard/trpc-dashboard-workflow.tsx',
        'lib/trpc/metadata.ts',
    ],
    receiptPaths: [
        '.dx/forge/receipts/2026-05-22-api-trpc-dashboard-workflow.json',
        '.dx/forge/receipts/api-trpc.json',
        '.dx/forge/template-readiness/launch-route.json',
        '.dx/forge/template-readiness/launch-runtime-checklist.json',
    ],
    dxCheckVisibility: {
        schema: 'dx.forge.package.dx_check_visibility',
        currentStatus: 'present',
        statuses: [
            'present',
            'stale',
            'missing-receipt',
            'blocked',
            'unsupported-surface',
        ],
        receiptPath:
            'examples/template/.dx/forge/receipts/2026-05-22-api-trpc-dashboard-workflow.json',
        monitoredSurfaces: [
            'trpc-launch-dashboard-workflow',
            'trpc-starter-dashboard-workflow',
            'trpc-route-handler',
        ],
        dxCheckMetrics: [
            'type_safe_api_receipt_present',
            'type_safe_api_receipt_stale',
            'type_safe_api_missing_receipt',
            'type_safe_api_blocked_surface',
            'type_safe_api_unsupported_surface',
        ],
    },
    appOwnedBoundaries: [
        'domain router implementation',
        'authorization and session context',
        'runtime dependency installation',
        'transport, cache, and stream policy',
    ],
} as const;

export const trpcDashboardProcedures: readonly TrpcDashboardProcedure[] = [
    {
        id: 'health.query',
        label: 'Health query',
        sourceApi: 'fetchRequestHandler + queryOptions',
        dashboardUse: 'Read the typed health contract through /api/trpc/health.',
        appOwnedBoundary: 'route mounting, request context, and production cache policy',
    },
    {
        id: 'launchEvent.mutation',
        label: 'Launch event',
        sourceApi: 'createTRPCClient + mutationOptions',
        dashboardUse: 'Prepare a typed event mutation receipt without writing external state.',
        appOwnedBoundary: 'persistence, audit logging, rate limiting, and authorization',
    },
    {
        id: 'launchEvents.infiniteQuery',
        label: 'Launch feed',
        sourceApi: 'infiniteQueryOptions',
        dashboardUse: 'Describe the cursor-paginated feed contract for operator activity.',
        appOwnedBoundary: 'cursor semantics, retention, and database-backed event storage',
    },
    {
        id: 'launchFeed.subscription',
        label: 'Live feed',
        sourceApi: 'subscriptionOptions',
        dashboardUse: 'Expose the subscription-ready surface without opening a stream locally.',
        appOwnedBoundary: 'fan-out, stream pacing, runtime transport, and auth policy',
    },
];

export function getTrpcDashboardProcedure(
    procedureId: TrpcDashboardProcedureId,
): TrpcDashboardProcedure {
    return (
        trpcDashboardProcedures.find((procedure) => procedure.id === procedureId) ||
        trpcDashboardProcedures[0]
    );
}

export function createTrpcDashboardReceipt(
    procedure: TrpcDashboardProcedureId,
    sequence = 1,
): TrpcDashboardReceipt {
    return {
        packageId: trpcDashboardPackage.packageId,
        procedure,
        requestId: `dx-trpc-dashboard-${String(sequence).padStart(2, '0')}`,
        status: 'local-receipt',
        cacheAction:
            procedure === 'health.query'
                ? 'queryOptions can hydrate a health cache once the route is mounted'
                : 'query client invalidation remains app-owned until runtime wiring',
        nextAction:
            'Install the tRPC runtime dependencies, mount the app router, and connect auth/session context before executing network calls.',
    };
}
