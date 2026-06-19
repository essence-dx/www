export type FumadocsDashboardPageId = "overview" | "api-reference" | "llm-export";

export type FumadocsDashboardPage = {
    id: FumadocsDashboardPageId;
    title: string;
    route: string;
    description: string;
    breadcrumb: string[];
    peers: string[];
    toc: string[];
};

export type FumadocsNavigationReceipt = {
    packageId: "content/fumadocs-next";
    status: "local-preview";
    selectedPageId: FumadocsDashboardPageId;
    route: string;
    breadcrumb: string;
    tocCount: number;
    peerCount: number;
    nextAction: string;
};

export const fumadocsDashboardContract = {
    packageId: "content/fumadocs-next",
    officialName: "Documentation System",
    aliases: ["fumadocs", "fumadocs-next", "docs"],
    upstreamPackage: "fumadocs",
    sourceMirror: "G:/WWW/inspirations/fumadocs",
    basedOn: "G:/WWW/inspirations/fumadocs",
    requiredEnv: ["DX_FUMADOCS_OPENAPI_ALLOWED_ORIGINS"],
    exportedFiles: [
        "lib/fumadocs/source.ts",
        "lib/fumadocs/navigation.ts",
        "lib/fumadocs/toc.ts",
        "lib/fumadocs/llms.ts",
        "lib/fumadocs/openapi.ts",
        "lib/fumadocs/search.ts",
        "components/dashboard/fumadocs-docs-workflow.tsx",
    ],
    receiptPaths: [
        ".dx/forge/docs/content-fumadocs-next.md",
        ".dx/forge/receipts/*-content-fumadocs-next.json",
    ],
    upstreamPublicApis: [
        "getBreadcrumbItems from fumadocs-core/breadcrumb",
        "flattenTree from fumadocs-core/page-tree",
        "findNeighbour from fumadocs-core/page-tree",
        "getPageTreePeers from fumadocs-core/page-tree",
        "llms from fumadocs-core/source",
        "createOpenAPI from fumadocs-openapi/server",
        "createFromSource from fumadocs-core/search/server",
        "useDocsSearch from fumadocs-core/search/client",
    ],
    appOwnedBoundaries: [
        "Content governance, private-doc exclusion, route publishing, search budgets, and OpenAPI proxy allowed origins.",
        "Dependency installation stays app-owned; the DX/Forge starter path does not create template-local node_modules.",
    ],
} as const;

export const fumadocsDashboardPages: FumadocsDashboardPage[] = [
    {
        id: "overview",
        title: "Launch docs",
        route: "/docs",
        description: "Editable Fumadocs App Router docs generated from source-owned files.",
        breadcrumb: ["Docs", "Launch", "Overview"],
        peers: ["API reference", "LLM export"],
        toc: ["Route contract", "Source content", "Search boundary"],
    },
    {
        id: "api-reference",
        title: "API reference",
        route: "/docs/api",
        description: "Fumadocs OpenAPI virtual docs with an app-owned proxy boundary.",
        breadcrumb: ["Docs", "API", "Reference"],
        peers: ["Launch docs", "LLM export"],
        toc: ["OpenAPI schema", "Allowed origins", "Request snippets"],
    },
    {
        id: "llm-export",
        title: "LLM export",
        route: "/llms.txt",
        description: "AI-readable docs index powered by the real Fumadocs llms(source) contract.",
        breadcrumb: ["Docs", "AI", "LLMs"],
        peers: ["Launch docs", "API reference"],
        toc: ["Index route", "Full export", "Private content boundary"],
    },
];

export function getFumadocsDashboardPage(pageId: FumadocsDashboardPageId) {
    return (
        fumadocsDashboardPages.find((page) => page.id === pageId) ||
        fumadocsDashboardPages[0]
    );
}

export function createFumadocsNavigationReceipt({
    pageId,
}: {
    pageId: FumadocsDashboardPageId;
}): FumadocsNavigationReceipt {
    const page = getFumadocsDashboardPage(pageId);

    return {
        packageId: "content/fumadocs-next",
        status: "local-preview",
        selectedPageId: page.id,
        route: page.route,
        breadcrumb: page.breadcrumb.join(" / "),
        tocCount: page.toc.length,
        peerCount: page.peers.length,
        nextAction:
            "Materialize the Documentation System package, review content policy, and configure DX_FUMADOCS_OPENAPI_ALLOWED_ORIGINS before proxying external OpenAPI requests.",
    };
}
