export type InstantDashboardSurfaceId =
    | 'realtime-todos'
    | 'presence-room'
    | 'auth-storage-streams'
    | 'sync-table-events';

export type InstantDashboardSurface = {
    id: InstantDashboardSurfaceId;
    label: string;
    publicApi: string;
    dashboardUse: string;
    appBoundary: string;
};

export type InstantDashboardReceipt = {
    packageId: 'instantdb/react';
    surfaceId: InstantDashboardSurfaceId;
    status: 'missing-config';
    receiptId: string;
    nextAction: string;
};

export type InstantDashboardDxCheckStatus =
    | 'present'
    | 'stale'
    | 'missing-receipt'
    | 'blocked'
    | 'unsupported-surface';

export type InstantDashboardDxCheckLegendEntry = {
    status: InstantDashboardDxCheckStatus;
    meaning: string;
};

export type InstantDashboardDxCheckSurface = {
    surfaceId: string;
    status: InstantDashboardDxCheckStatus;
    receiptPath: string;
    files: readonly string[];
    sourceMarkers: readonly string[];
};

export type InstantDashboardDxCheckVisibility = {
    schema: 'dx.forge.package.dx_check_visibility';
    officialPackageName: 'Realtime App Database';
    packageId: 'instantdb/react';
    currentStatus: InstantDashboardDxCheckStatus;
    receiptStatus: InstantDashboardDxCheckStatus;
    receiptPath: string;
    statusLegend: readonly InstantDashboardDxCheckLegendEntry[];
    monitoredSurfaces: readonly InstantDashboardDxCheckSurface[];
};

export type InstantDashboardDxStyleCompatibility = {
    schema: 'dx.forge.package.dx_style_compatibility';
    status: 'present';
    tokenSource: string;
    generatedCss: string;
    visibleSurfaces: readonly string[];
    sourceFiles: readonly string[];
    dataDxMarkers: readonly string[];
    runtimeProof: false;
    runtimeLimitations: readonly string[];
};

export const instantDashboardInspectedSourceFiles = [
    'client/packages/react/package.json',
    'client/packages/react/src/index.ts',
    'client/packages/core/src/index.ts',
    'client/packages/react-common/src/InstantReactAbstractDatabase.tsx',
    'client/sandbox/react-nextjs/pages/play/sync-table.tsx',
] as const;

export const instantDashboardSelectedSurfaces = [
    'realtime-todos',
    'presence-room',
    'auth-storage-streams',
    'sync-table-events',
    'dashboard-workflow',
] as const;

export const instantDashboardDxCheckVisibility = {
    schema: 'dx.forge.package.dx_check_visibility',
    officialPackageName: 'Realtime App Database',
    packageId: 'instantdb/react',
    currentStatus: 'present',
    receiptStatus: 'present',
    receiptPath:
        'examples/template/.dx/forge/receipts/2026-05-22-instantdb-realtime-dashboard.json',
    statusLegend: [
        {
            status: 'present',
            meaning:
                'selected Realtime App Database surfaces, source markers, and receipt are present',
        },
        {
            status: 'stale',
            meaning:
                'materialized Realtime App Database files or hashes no longer match the receipt',
        },
        {
            status: 'missing-receipt',
            meaning:
                'selected Realtime App Database surfaces exist without the dashboard workflow receipt',
        },
        {
            status: 'blocked',
            meaning:
                'app-owned Instant configuration or hosted runtime proof is required before claiming more',
        },
        {
            status: 'unsupported-surface',
            meaning:
                'a requested Realtime App Database surface is outside the selected upstream-backed set',
        },
    ],
    monitoredSurfaces: [
        {
            surfaceId: 'instantdb-runtime-dashboard-workflow',
            status: 'present',
            receiptPath:
                'examples/template/.dx/forge/receipts/2026-05-22-instantdb-realtime-dashboard.json',
            files: [
                'examples/template/runtime-pages/index.html',
                'examples/template/runtime-assets/launch-runtime.ts',
            ],
            sourceMarkers: [
                'data-dx-package="instantdb/react"',
                'data-dx-component="instantdb-runtime-dashboard-workflow"',
                'data-dx-instant-action="prepare-local-schema-receipt"',
                'data-dx-style-surface="realtime-app-database"',
            ],
        },
        {
            surfaceId: 'dashboard-instantdb-workflow',
            status: 'present',
            receiptPath:
                'examples/template/.dx/forge/receipts/2026-05-22-instantdb-realtime-dashboard.json',
            files: [
                'examples/dashboard/src/lib/instantdbDashboard.ts',
                'examples/dashboard/src/components/InstantDbDashboardWorkflow.tsx',
                'components/dashboard/instantdb-dashboard-workflow.tsx',
            ],
            sourceMarkers: [
                'data-dx-package="instantdb/react"',
                'data-dx-component="dashboard-instantdb-workflow"',
                'data-dx-instant-dashboard-workflow="realtime-boundary"',
                'data-dx-style-surface="realtime-app-database"',
            ],
        },
        {
            surfaceId: 'sync-table-events',
            status: 'present',
            receiptPath:
                'examples/template/.dx/forge/receipts/2026-05-22-instantdb-realtime-dashboard.json',
            files: ['lib/instant/sync-table.ts'],
            sourceMarkers: [
                'SyncTableCallbackEventType',
                'StoreInterfaceStoreName',
                'db.core._syncTableExperimental',
            ],
        },
    ],
} as const satisfies InstantDashboardDxCheckVisibility;

export const instantDashboardDxStyleCompatibility = {
    schema: 'dx.forge.package.dx_style_compatibility',
    status: 'present',
    tokenSource: 'examples/template/runtime-assets/launch-runtime.css',
    generatedCss: 'examples/template/runtime-assets/launch-runtime.css',
    visibleSurfaces: [
        'instantdb-runtime-dashboard-workflow',
        'dashboard-instantdb-workflow',
    ],
    sourceFiles: [
        'examples/template/instantdb-status.tsx',
        'examples/template/runtime-pages/index.html',
        'examples/dashboard/src/components/InstantDbDashboardWorkflow.tsx',
    ],
    dataDxMarkers: ['data-dx-style-surface="realtime-app-database"'],
    runtimeProof: false,
    runtimeLimitations: [
        'SOURCE-ONLY: Realtime App Database style evidence is source-visible and token-compatible. No live browser style proof was run.',
        'ADAPTER-BOUNDARY: hosted Instant runtime configuration remains app-owned.',
    ],
} as const satisfies InstantDashboardDxStyleCompatibility;

export const instantDashboardPackage = {
    packageId: 'instantdb/react',
    officialPackageName: 'Realtime App Database',
    aliases: ['@instantdb/react', 'instantdb', 'db/instantdb'],
    upstreamPackage: '@instantdb/react',
    upstreamVersion: '0.0.0',
    sourceMirror: 'G:/WWW/inspirations/instantdb',
    requiredEnv: ['NEXT_PUBLIC_INSTANT_APP_ID'],
    inspectedSourceFiles: instantDashboardInspectedSourceFiles,
    selectedSurfaces: instantDashboardSelectedSurfaces,
    dxCheckVisibility: instantDashboardDxCheckVisibility,
    dxStyleCompatibility: instantDashboardDxStyleCompatibility,
    honestyLabel: 'ADAPTER-BOUNDARY',
    exportedFiles: [
        'lib/instant/env.ts',
        'lib/instant/schema.ts',
        'lib/instant/client.ts',
        'lib/instant/next-client.tsx',
        'lib/instant/next-server.ts',
        'lib/instant/queries.ts',
        'lib/instant/status.ts',
        'lib/instant/subscriptions.ts',
        'lib/instant/pagination.ts',
        'lib/instant/diagnostics.ts',
        'lib/instant/mutations.ts',
        'lib/instant/rules.ts',
        'lib/instant/perms.ts',
        'lib/instant/auth.ts',
        'lib/instant/oauth.ts',
        'lib/instant/storage.ts',
        'lib/instant/streams.ts',
        'lib/instant/sync-table.ts',
        'lib/instant/route.ts',
        'lib/instant/metadata.ts',
        'lib/instant/dashboard-workflow.ts',
        'components/dashboard/instantdb-dashboard-workflow.tsx',
        'components/instant/instant-todos.tsx',
        'components/instant/instant-cursors.tsx',
        'components/instant/instant-auth-boundary.tsx',
        'components/launch/instantdb-status.tsx',
        'app/api/instant/route.ts',
        'app/instant-launch/page.tsx',
        'examples/dashboard/src/lib/instantdbDashboard.ts',
        'examples/dashboard/src/components/InstantDbDashboardWorkflow.tsx',
    ],
    receiptPaths: [
        '.dx/forge/docs/instantdb-react.md',
        '.dx/forge/receipts/*-instantdb-react.json',
        'docs/packages/instantdb-react.md',
        'examples/template/.dx/forge/receipts/2026-05-22-instantdb-realtime-dashboard.json',
        'examples/dashboard/README.md#instantdb-dashboard-workflow',
    ],
    provenance:
        'Inspected the local InstantDB React export map, init(), i.schema, room hooks, auth, storage, streams, SyncTableCallbackEventType, db.core._syncTableExperimental, and Next SSR helpers before exposing this dashboard workflow.',
    appOwnedBoundaries: [
        'Instant dashboard app id',
        'rules and auth policy',
        'production schema and unique indexes',
        'file access rules',
        'stream lifecycle and topic payload policy',
        'experimental Sync Table subscriptions and local store retention',
    ],
} as const;

export const instantDashboardSurfaces: readonly InstantDashboardSurface[] = [
    {
        id: 'realtime-todos',
        label: 'Realtime todos',
        publicApi: 'init + i.schema + db.useQuery + db.transact + db.tx',
        dashboardUse:
            'Read and mutate launch todo state after the app owns the Instant app id and rules.',
        appBoundary: 'NEXT_PUBLIC_INSTANT_APP_ID and deployed schema',
    },
    {
        id: 'presence-room',
        label: 'Presence room',
        publicApi:
            'db.room + db.rooms.usePresence + db.rooms.useSyncPresence + db.rooms.useTypingIndicator',
        dashboardUse:
            'Show dashboard reviewers, typing readiness, and collaborative cursor state.',
        appBoundary: 'room naming, payload policy, and authenticated visibility',
    },
    {
        id: 'auth-storage-streams',
        label: 'Auth, storage, streams',
        publicApi:
            'db.auth + db.storage.uploadFile + db.streams.createWriteStream + createInstantRouteHandler',
        dashboardUse:
            'Expose account, file, stream, and first-party route readiness without browser secrets.',
        appBoundary: 'auth providers, file rules, stream retention, and route mounting',
    },
    {
        id: 'sync-table-events',
        label: 'Sync Table events',
        publicApi:
            'SyncTableCallbackEventType + db.core._syncTableExperimental + StoreInterfaceStoreName',
        dashboardUse:
            'Surface source-owned event summaries for local table sync without claiming hosted execution.',
        appBoundary:
            'local persistence store, subscription lifetime, and runtime validation',
    },
];

export function getInstantDashboardSurface(
    surfaceId: InstantDashboardSurfaceId,
): InstantDashboardSurface {
    return (
        instantDashboardSurfaces.find((surface) => surface.id === surfaceId) ||
        instantDashboardSurfaces[0]
    );
}

export function createInstantDashboardReceipt(
    surfaceId: InstantDashboardSurfaceId,
): InstantDashboardReceipt {
    return {
        packageId: instantDashboardPackage.packageId,
        surfaceId,
        status: 'missing-config',
        receiptId: `instantdb-dashboard-${surfaceId}`,
        nextAction:
            'Create the Instant app, set NEXT_PUBLIC_INSTANT_APP_ID, review rules, then pass the real db client into the dashboard workflow.',
    };
}
