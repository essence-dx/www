export type DrizzleDashboardQueryId =
    | 'overview'
    | 'published-posts'
    | 'author-counts';

export type DrizzleDashboardQuery = {
    id: DrizzleDashboardQueryId;
    label: string;
    packageExport: string;
    queryPlanExport: string;
    publicApi: string;
    resultShape: string;
    sqlPreview: string;
};

export type DrizzleDashboardReceipt = {
    queryId: DrizzleDashboardQueryId;
    status: DrizzleDashboardRuntimeStatus;
    packageId: 'db/drizzle-sqlite';
    receiptPath: typeof drizzleDashboardWorkflowReceiptPath;
    nextAction: string;
};

export type DrizzleDashboardRuntimeStatus = 'missing-runtime';

export type DrizzleDashboardRuntimeReadiness = {
    status: DrizzleDashboardRuntimeStatus;
    runtimeDependencies: typeof drizzleDashboardRuntimeDependencies;
    nextAction: string;
};

export const drizzleDashboardWorkflowReceiptPath =
    'examples/template/.dx/forge/receipts/2026-05-22-db-drizzle-sqlite-dashboard-workflow.json' as const;

export const drizzleDashboardRuntimeDependencies = ['drizzle-orm', 'better-sqlite3'] as const;

export const drizzleDashboardPackage = {
    packageId: 'db/drizzle-sqlite',
    aliases: ['database/drizzle', 'db/drizzle', 'drizzle', 'drizzle/sqlite'],
    sourceMirror: 'G:/WWW/inspirations/drizzle-orm',
    requiredEnv: [],
    dashboardEntryPoint: 'readDrizzleDashboardOverview',
    dashboardQueryPlanEntryPoint: 'readDrizzleDashboardQueryPlan',
    dashboardQueryPlanByIdEntryPoint: 'readDrizzleDashboardQueryPlanById',
    exportedFiles: [
        'db/drizzle/client.ts',
        'db/drizzle/schema.ts',
        'db/drizzle/analytics.ts',
        'db/drizzle/joins.ts',
        'db/drizzle/cte-queries.ts',
        'db/drizzle/dashboard-workflow.ts',
    ],
    receiptPaths: [
        '.dx/forge/docs/db-drizzle-sqlite.md',
        '.dx/forge/receipts/*-db-drizzle-sqlite.json',
        drizzleDashboardWorkflowReceiptPath,
        '.dx/forge/source-manifest.json',
    ],
    appOwnedBoundaries: [
        'DATABASE_URL or SQLite file path',
        'better-sqlite3 runtime installation',
        'migration SQL review and rollout',
        'dashboard authorization policy',
    ],
} as const;

export const drizzleDashboardQueries: readonly DrizzleDashboardQuery[] = [
    {
        id: 'overview',
        label: 'Dashboard overview',
        packageExport: 'readDrizzleDashboardOverview',
        queryPlanExport: 'readDrizzleDashboardQueryPlanById',
        publicApi:
            'count/countDistinct/sql aggregate + leftJoin + toSQL()',
        resultShape: 'stats, previews, authorCounts, recentUsers',
        sqlPreview:
            "select count(distinct users.id), count(distinct users.role), count(posts.id), count(*) filter (where posts.status = 'published') from users left join posts",
    },
    {
        id: 'published-posts',
        label: 'Published posts',
        packageExport: 'listPublishedPostPreviews',
        queryPlanExport: 'readDrizzleDashboardQueryPlanById',
        publicApi: 'select().from().innerJoin().where().toSQL()',
        resultShape: 'title, slug, authorName, publishedAt',
        sqlPreview:
            'select posts.title, posts.slug, users.name from posts inner join users where posts.status = ?',
    },
    {
        id: 'author-counts',
        label: 'Author counts',
        packageExport: 'listAuthorsWithPostCounts',
        queryPlanExport: 'readDrizzleDashboardQueryPlanById',
        publicApi: 'select().from().leftJoin().groupBy().toSQL()',
        resultShape: 'authorId, name, postCount',
        sqlPreview:
            'select users.id, users.name, count(posts.id) from users left join posts group by users.id',
    },
];

export function getDrizzleDashboardQuery(
    queryId: DrizzleDashboardQueryId,
): DrizzleDashboardQuery {
    return (
        drizzleDashboardQueries.find((query) => query.id === queryId) ||
        drizzleDashboardQueries[0]
    );
}

export function readDrizzleDashboardRuntimeReadiness(): DrizzleDashboardRuntimeReadiness {
    return {
        status: 'missing-runtime',
        runtimeDependencies: drizzleDashboardRuntimeDependencies,
        nextAction:
            'Install drizzle-orm + better-sqlite3, configure the SQLite path, and run reviewed migrations before executing dashboard reads.',
    };
}

export function createDrizzleDashboardReceipt(
    queryId: DrizzleDashboardQueryId,
): DrizzleDashboardReceipt {
    const runtimeReadiness = readDrizzleDashboardRuntimeReadiness();

    return {
        queryId,
        status: runtimeReadiness.status,
        packageId: drizzleDashboardPackage.packageId,
        receiptPath: drizzleDashboardWorkflowReceiptPath,
        nextAction: runtimeReadiness.nextAction,
    };
}
