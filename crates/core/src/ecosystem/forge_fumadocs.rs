pub(super) const FUMADOCS_NEXT_VERSION: &str = "16.8.12-dx.11";

pub(super) fn fumadocs_next_templates() -> Vec<(&'static str, &'static str)> {
    vec![
        ("js/lib/fumadocs/source.ts", FUMADOCS_SOURCE_TS),
        (
            "js/lib/fumadocs/source-plugins.tsx",
            FUMADOCS_SOURCE_PLUGINS_TSX,
        ),
        ("js/lib/fumadocs/navigation.ts", FUMADOCS_NAVIGATION_TS),
        ("js/lib/fumadocs/toc.ts", FUMADOCS_TOC_TS),
        ("js/lib/fumadocs/layout.tsx", FUMADOCS_LAYOUT_TSX),
        ("js/lib/fumadocs/llms.ts", FUMADOCS_LLMS_TS),
        ("js/lib/fumadocs/openapi.ts", FUMADOCS_OPENAPI_TS),
        (
            "js/lib/fumadocs/openapi-code-usage.ts",
            FUMADOCS_OPENAPI_CODE_USAGE_TS,
        ),
        ("js/lib/fumadocs/search.ts", FUMADOCS_SEARCH_TS),
        (
            "js/lib/fumadocs/search-client.ts",
            FUMADOCS_SEARCH_CLIENT_TS,
        ),
        (
            "js/lib/fumadocs/dashboard-workflow.ts",
            FUMADOCS_DASHBOARD_WORKFLOW_TS,
        ),
        ("js/lib/fumadocs/metadata.ts", FUMADOCS_METADATA_TS),
        (
            "js/lib/fumadocs/route-contract.ts",
            FUMADOCS_ROUTE_CONTRACT_TS,
        ),
        ("js/lib/fumadocs/readiness.ts", FUMADOCS_READINESS_TS),
        ("js/components/mdx.tsx", FUMADOCS_MDX_COMPONENTS_TSX),
        ("js/components/api-page.tsx", FUMADOCS_API_PAGE_TSX),
        (
            "js/components/api-page.client.tsx",
            FUMADOCS_API_PAGE_CLIENT_TSX,
        ),
        (
            "js/components/dashboard/fumadocs-docs-workflow.tsx",
            FUMADOCS_DASHBOARD_WORKFLOW_TSX,
        ),
        ("js/app/docs/layout.tsx", FUMADOCS_DOCS_LAYOUT_TSX),
        ("js/app/docs/[[...slug]]/page.tsx", FUMADOCS_DOCS_PAGE_TSX),
        ("js/app/llms.txt/route.ts", FUMADOCS_LLMS_TXT_ROUTE_TS),
        (
            "js/app/llms-full.txt/route.ts",
            FUMADOCS_LLMS_FULL_TXT_ROUTE_TS,
        ),
        (
            "js/app/llms.mdx/docs/[[...slug]]/route.ts",
            FUMADOCS_LLMS_MDX_ROUTE_TS,
        ),
        ("js/app/api/search/route.ts", FUMADOCS_SEARCH_ROUTE_TS),
        (
            "js/app/api/search-static/route.ts",
            FUMADOCS_STATIC_SEARCH_ROUTE_TS,
        ),
        (
            "js/app/api/openapi/proxy/route.ts",
            FUMADOCS_OPENAPI_PROXY_ROUTE_TS,
        ),
        (
            "js/app/docs/readiness/route.ts",
            FUMADOCS_DOCS_READINESS_ROUTE_TS,
        ),
        ("js/content/docs/meta.json", FUMADOCS_META_JSON),
        ("js/content/docs/index.mdx", FUMADOCS_INDEX_MDX),
        ("js/openapi/dx-launch.yaml", FUMADOCS_OPENAPI_YAML),
        ("js/lib/fumadocs/README.md", FUMADOCS_README_MD),
    ]
}

const FUMADOCS_SOURCE_TS: &str = r#"import { docs } from "collections/server";
import { loader } from "fumadocs-core/source";
import {
  dxFumadocsOpenAPI,
  dxFumadocsOpenAPIContract,
} from "@/lib/fumadocs/openapi";
import {
  dxFumadocsSourcePlugins,
  dxFumadocsSlugFn,
} from "@/lib/fumadocs/source-plugins";

export const dxFumadocsOpenAPILoaderPluginConfig = {
  plugins: [dxFumadocsOpenAPI.loaderPlugin()],
} as const;

export const source = loader(
  {
    docs: docs.toFumadocsSource(),
    openapi: await dxFumadocsOpenAPI.staticSource({
      baseDir: dxFumadocsOpenAPIContract.docsBaseDir,
      groupBy: "tag",
      meta: {
        folderStyle: "folder",
      },
    }),
  },
  {
    baseUrl: "/docs",
    plugins: [...dxFumadocsSourcePlugins, ...dxFumadocsOpenAPILoaderPluginConfig.plugins],
    slugs: dxFumadocsSlugFn,
  },
);
"#;

const FUMADOCS_SOURCE_PLUGINS_TSX: &str = r#"import { Icon } from "@/components/icons/icon";
import { iconPlugin } from "fumadocs-core/source/icon";
import { slugsFromData } from "fumadocs-core/source/slugs";
import { statusBadgesPlugin } from "fumadocs-core/source/status-badges";

export const dxFumadocsSourcePluginContract = {
  upstreamApis: [
    "iconPlugin from fumadocs-core/source/icon",
    "statusBadgesPlugin from fumadocs-core/source/status-badges",
    "slugsFromData from fumadocs-core/source/slugs",
  ],
  appOwnedApis: ["DX Icon component from @/components/icons/icon"],
  defaultIcon: "action:rocket",
  slugField: "slug",
  frontmatterFields: ["icon", "status", "slug"],
  statusTaxonomy: ["new", "beta", "deprecated", "experimental"],
  appOwnedBoundaries: [
    "Source plugin taxonomy, DX icon naming, status lifecycle, and sidebar badge copy.",
    "Slug redirects, canonical URL policy, and collision review before publishing.",
    "Runtime verification that page-tree icons and badges render with the app theme.",
  ],
} as const;

export const dxFumadocsSlugFn = slugsFromData(dxFumadocsSourcePluginContract.slugField);

const dxFumadocsIconAliases: Record<string, string> = {
  Rocket: "action:rocket",
  rocket: "action:rocket",
};

export function resolveDxFumadocsIconName(icon = dxFumadocsSourcePluginContract.defaultIcon) {
  return dxFumadocsIconAliases[icon] ?? icon;
}

export const dxFumadocsIconPlugin = iconPlugin((icon) => {
  const iconName = resolveDxFumadocsIconName(icon);

  return (
    <Icon
      aria-label="Documentation page icon"
      className="docs-page-tree-icon"
      data-dx-docs-icon-source="dx-icons"
      name={iconName}
    />
  );
});

export const dxFumadocsSourcePlugins = [
  dxFumadocsIconPlugin,
  statusBadgesPlugin({
    renderBadge(status) {
      return (
        <span
          className="ms-2 rounded border px-1.5 py-0.5 text-[10px] font-medium"
          data-dx-docs-status={status}
        >
          {status}
        </span>
      );
    },
  }),
] as const;
"#;

const FUMADOCS_NAVIGATION_TS: &str = r#"import { getBreadcrumbItems } from "fumadocs-core/breadcrumb";
import { findNeighbour, flattenTree, getPageTreePeers } from "fumadocs-core/page-tree";
import { source } from "@/lib/fumadocs/source";

type DxFumadocsNavigationItem = {
  name: string;
  url?: string;
};

function toNavigationItem(item?: { name: unknown; url?: string }): DxFumadocsNavigationItem | null {
  if (!item) return null;

  return {
    name: typeof item.name === "string" ? item.name : String(item.name ?? ""),
    url: item.url,
  };
}

export const dxFumadocsNavigationContract = {
  upstreamApis: [
    "getBreadcrumbItems from fumadocs-core/breadcrumb",
    "flattenTree from fumadocs-core/page-tree",
    "findNeighbour from fumadocs-core/page-tree",
    "getPageTreePeers from fumadocs-core/page-tree",
    "source.getPageTree() from fumadocs-core/source loader",
  ],
  defaultUrl: "/docs",
  treeSource: "source.getPageTree()",
  surfaces: ["breadcrumb", "previous-next", "sidebar-count", "peers"],
  appOwnedBoundaries: [
    "Navigation policy, page ordering, root separation, and sidebar grouping remain app-owned.",
    "Canonical URL review is required before publishing custom slug/frontmatter changes.",
    "Runtime verification should confirm breadcrumbs and previous-next links match the deployed docs tree.",
  ],
} as const;

export function getDxFumadocsPageBreadcrumbs(
  url = dxFumadocsNavigationContract.defaultUrl,
  locale?: string,
) {
  const tree = source.getPageTree(locale);

  return getBreadcrumbItems(url, tree, {
    includePage: true,
    includeRoot: { url: dxFumadocsNavigationContract.defaultUrl },
  }).map((item) => ({
    name: typeof item.name === "string" ? item.name : String(item.name ?? ""),
    url: item.url,
  }));
}

export function getDxFumadocsNavigationSnapshot(
  url = dxFumadocsNavigationContract.defaultUrl,
  locale?: string,
) {
  const tree = source.getPageTree(locale);
  const pages = flattenTree(tree.children);
  const neighbour = findNeighbour(tree, url);

  return {
    url,
    locale: locale ?? null,
    totalPages: pages.length,
    breadcrumbs: getDxFumadocsPageBreadcrumbs(url, locale),
    previous: toNavigationItem(neighbour.previous),
    next: toNavigationItem(neighbour.next),
    peers: getPageTreePeers(tree, url).map((item) => ({
      name: typeof item.name === "string" ? item.name : String(item.name ?? ""),
      url: item.url,
    })),
  };
}
"#;

const FUMADOCS_TOC_TS: &str = r#"import { getTableOfContents } from "fumadocs-core/content/toc";
import type { TOCItemType } from "fumadocs-core/toc";
import { source } from "@/lib/fumadocs/source";

type DxFumadocsPage = (typeof source)["$inferPage"];

export type DxFumadocsTocItem = {
  title: string;
  url: string;
  depth: number;
  step?: number;
};

function normalizeDxFumadocsTocItem(item: TOCItemType): DxFumadocsTocItem {
  return {
    title: typeof item.title === "string" ? item.title : String(item.title ?? ""),
    url: item.url,
    depth: item.depth,
    step: item._step,
  };
}

export const dxFumadocsTocContract = {
  upstreamApis: [
    "getTableOfContents from fumadocs-core/content/toc",
    "TOCItemType from fumadocs-core/toc",
    "page.data.toc from fumadocs-mdx runtime",
    "DocsPage toc prop from fumadocs-ui/layouts/docs/page",
  ],
  tocFile: "lib/fumadocs/toc.ts",
  docsPageProp: "DocsPage toc",
  surfaces: ["docs-page-outline", "markdown-outline", "toc-summary"],
  appOwnedBoundaries: [
    "TOC depth policy, heading copy, hidden headings, and scroll behavior remain app-owned.",
    "CMS or remote content should use its own trusted TOC source when available.",
    "Runtime verification should confirm page outlines match rendered headings.",
  ],
} as const;

export function getDxFumadocsMarkdownToc(content: string): TOCItemType[] {
  return getTableOfContents(content);
}

export function getDxFumadocsPageToc(page: DxFumadocsPage): TOCItemType[] {
  return page.data.toc as TOCItemType[];
}

export function getDxFumadocsPageTocSummary(page: DxFumadocsPage) {
  const items = getDxFumadocsPageToc(page).map(normalizeDxFumadocsTocItem);
  const depths = items.map((item) => item.depth);

  return {
    url: page.url,
    title: page.data.title,
    headingCount: items.length,
    minDepth: depths.length > 0 ? Math.min(...depths) : null,
    maxDepth: depths.length > 0 ? Math.max(...depths) : null,
    items,
  };
}
"#;

const FUMADOCS_LLMS_TS: &str = r#"import { source } from "@/lib/fumadocs/source";
import { llms } from "fumadocs-core/source";

type DxFumadocsPage = (typeof source)["$inferPage"];

export const dxFumadocsLLMsContract = {
  indexRoute: "/llms.txt",
  fullRoute: "/llms-full.txt",
  pageMarkdownBaseRoute: "/llms.mdx/docs",
  pageMarkdownRoute: "/llms.mdx/docs/[[...slug]]",
  upstreamApi: "llms from fumadocs-core/source",
  processedMarkdown: "page.data.getText(\"processed\")",
  appOwnedBoundaries: [
    "AI indexing policy, robots policy, and public content exposure review.",
    "Content redaction, private docs exclusion, and generated text retention policy.",
    "Runtime verification that generated LLM routes match the deployed docs tree.",
  ],
} as const;

export function createDxFumadocsLLMsIndex() {
  return llms(source);
}

export function getDxFumadocsPageMarkdownUrl(page: DxFumadocsPage) {
  const segments = [...page.slugs, "content.md"];

  return {
    segments,
    url: `${dxFumadocsLLMsContract.pageMarkdownBaseRoute}/${segments.join("/")}`,
  };
}

export async function getDxFumadocsLLMText(page: DxFumadocsPage) {
  if (page.type === "openapi") {
    return JSON.stringify(page.data.getSchema().bundled, null, 2);
  }

  const processed = await page.data.getText("processed");

  return `# ${page.data.title} (${page.url})

${processed}`;
}
"#;

const FUMADOCS_OPENAPI_TS: &str = r#"import { createOpenAPI } from "fumadocs-openapi/server";

export const dxFumadocsOpenAPIContract = {
  upstreamApis: [
    "createOpenAPI from fumadocs-openapi/server",
    "openapi.staticSource from fumadocs-openapi/server",
    "openapi.loaderPlugin() from fumadocs-openapi/server",
    "openapi.createProxy from fumadocs-openapi/server",
    "createAPIPage from fumadocs-openapi/ui",
  ],
  schemaPath: "./openapi/dx-launch.yaml",
  schemaFile: "openapi/dx-launch.yaml",
  docsBaseDir: "api",
  docsRoute: "/docs/api",
  proxyRoute: "/api/openapi/proxy",
  allowedOriginsEnv: "DX_FUMADOCS_OPENAPI_ALLOWED_ORIGINS",
  proxyMode: "deny-unconfigured-origins",
  component: "APIPage",
  appOwnedBoundaries: [
    "OpenAPI schema governance, public endpoint review, and release/version ownership.",
    "OpenAPI proxy allowed origins, auth header forwarding policy, request playground safety, and production request limits.",
    "Schema generation cadence and runtime verification against deployed API routes.",
  ],
} as const;

export function readDxFumadocsOpenAPIAllowedOrigins(
  env: Record<string, string | undefined> = process.env,
) {
  return (env[dxFumadocsOpenAPIContract.allowedOriginsEnv] ?? "")
    .split(",")
    .map((origin) => origin.trim())
    .filter(Boolean);
}

export const dxFumadocsOpenAPI = createOpenAPI({
  input: [dxFumadocsOpenAPIContract.schemaPath],
  proxyUrl: dxFumadocsOpenAPIContract.proxyRoute,
});

export const openapi = dxFumadocsOpenAPI;
"#;

const FUMADOCS_OPENAPI_CODE_USAGE_TS: &str = r#"import {
  createCodeUsageGeneratorRegistry,
  type CodeUsageGenerator,
} from "fumadocs-openapi/requests/generators";
import { registerDefault } from "fumadocs-openapi/requests/generators/all";

export const dxFumadocsOpenAPICodeUsageContract = {
  upstreamApis: [
    "createCodeUsageGeneratorRegistry from fumadocs-openapi/requests/generators",
    "registerDefault from fumadocs-openapi/requests/generators/all",
    "defineClientConfig from fumadocs-openapi/ui/client",
  ],
  defaultGenerators: ["curl", "js", "go", "python", "java", "csharp"],
  customGenerators: ["dx-launch-fetch"],
  appOwnedBoundaries: [
    "Request code sample policy, SDK naming, auth header redaction, and sensitive example data.",
    "Language list, media adapters, generated sample QA, and playground request safety.",
    "Public API drift review before publishing generated examples.",
  ],
} as const;

const dxLaunchFetchGenerator: CodeUsageGenerator = {
  label: "DX Launch Fetch",
  lang: "ts",
  generate(url, data) {
    const headers = Object.fromEntries(
      Object.entries(data.header).map(([key, value]) => [key, value.value]),
    );
    const requestInit = [`method: ${JSON.stringify(data.method.toUpperCase())}`];

    if (Object.keys(headers).length > 0) {
      requestInit.push(`headers: ${JSON.stringify(headers, null, 2)}`);
    }

    if (data.bodyMediaType) {
      requestInit.push("// Add an app-owned encoded body for this media type.");
    }

    return [
      `const response = await fetch(${JSON.stringify(url)}, {`,
      ...requestInit.map((line) => `  ${line},`),
      "});",
      "",
      "if (!response.ok) {",
      "  throw new Error(`DX docs request failed: ${response.status}`);",
      "}",
      "",
      "const payload = await response.json();",
    ].join("\n");
  },
};

export const dxFumadocsOpenAPICodeUsages = createCodeUsageGeneratorRegistry();

registerDefault(dxFumadocsOpenAPICodeUsages);
dxFumadocsOpenAPICodeUsages.add("dx-launch-fetch", dxLaunchFetchGenerator);
"#;

const FUMADOCS_SEARCH_TS: &str = r#"import { source } from "@/lib/fumadocs/source";
import { createFromSource } from "fumadocs-core/search/server";

export const dxFumadocsSearchContract = {
  route: "/api/search",
  staticRoute: "/api/search-static",
  queryParam: "query",
  optionalParams: ["locale", "tag", "limit", "mode"],
  language: "english",
  upstreamApis: [
    "createFromSource from fumadocs-core/search/server",
    "staticGET from Fumadocs SearchAPI",
  ],
  source: "dxFumadocsSearchSource",
  appOwnedBoundaries: [
    "Search UI placement and empty/error/loading states.",
    "Search analytics, abuse limits, deployment policy, and runtime verification.",
    "Multilingual/vector policy beyond the default English Orama index.",
  ],
} as const;

export const dxFumadocsSearchConfig = {
  language: dxFumadocsSearchContract.language,
} as const;

export function createDxFumadocsSearchApi() {
  return createFromSource(source, dxFumadocsSearchConfig);
}
"#;

const FUMADOCS_SEARCH_CLIENT_TS: &str = r#"import type { ClientPreset } from "fumadocs-core/search/client";

export const dxFumadocsSearchClientContract = {
  upstreamApi: "useDocsSearch from fumadocs-core/search/client",
  fetchPreset: "dxFumadocsFetchSearchClient",
  staticPreset: "dxFumadocsStaticSearchClient",
  dynamicSearchRoute: "/api/search",
  staticSearchRoute: "/api/search-static",
  appOwnedBoundaries: [
    "Search dialog UI, keyboard behavior, and result empty/error/loading states.",
    "Client-side static index payload budget and cache policy.",
    "Production analytics, abuse limits, and multilingual/vector search policy.",
  ],
} as const;

export const dxFumadocsFetchSearchClient = {
  type: "fetch",
  api: dxFumadocsSearchClientContract.dynamicSearchRoute,
} satisfies ClientPreset;

export const dxFumadocsStaticSearchClient = {
  type: "static",
  from: dxFumadocsSearchClientContract.staticSearchRoute,
} satisfies ClientPreset;
"#;

const FUMADOCS_LAYOUT_TSX: &str = r#"import type { BaseLayoutProps } from "fumadocs-ui/layouts/shared";

export function baseOptions(): BaseLayoutProps {
  return {
    nav: {
      title: "DX Launch Docs",
    },
  };
}
"#;

const FUMADOCS_DASHBOARD_WORKFLOW_TS: &str = r#"import { dxFumadocsLLMsContract } from "@/lib/fumadocs/llms";
import { dxFumadocsOpenAPIContract } from "@/lib/fumadocs/openapi";
import { dxFumadocsNavigationContract } from "@/lib/fumadocs/navigation";
import { dxFumadocsRouteContract } from "@/lib/fumadocs/route-contract";
import { dxFumadocsSearchClientContract } from "@/lib/fumadocs/search-client";

export type DxFumadocsDashboardPageId =
  | "overview"
  | "api-reference"
  | "llm-export";

export type DxFumadocsDashboardPage = {
  id: DxFumadocsDashboardPageId;
  title: string;
  route: string;
  description: string;
  breadcrumb: string[];
  peers: string[];
  toc: string[];
};

export type DxFumadocsDashboardReceipt = {
  packageId: typeof dxFumadocsRouteContract.packageId;
  status: "local-preview";
  selectedPageId: DxFumadocsDashboardPageId;
  route: string;
  breadcrumb: string;
  tocCount: number;
  peerCount: number;
  nextAction: string;
};

export const dxFumadocsDashboardWorkflowContract = {
  packageId: dxFumadocsRouteContract.packageId,
  officialName: "Documentation System",
  upstreamPackage: "fumadocs",
  componentFile: "components/dashboard/fumadocs-docs-workflow.tsx",
  apiFile: "lib/fumadocs/dashboard-workflow.ts",
  launchWorkflowFile: "components/launch/docs-status.tsx",
  dashboardWorkflow: "docs-help-changelog",
  productSurface: "dashboard-help-content",
  sourceMirror: "G:/WWW/inspirations/fumadocs",
  basedOn: "G:/WWW/inspirations/fumadocs",
  selectedSurfaces: [
    "docs-app-router",
    "docs-runtime-readiness",
    "dashboard-help-workflow",
    "llm-export",
    "openapi-reference",
    "search-index",
  ],
  dxCheckVisibility: {
    schema: "dx.forge.package.dx_check_visibility",
    currentStatus: "present",
    statuses: ["present", "stale", "missing-receipt", "blocked", "unsupported-surface"],
    receiptPath:
      "examples/template/.dx/forge/receipts/2026-05-22-content-fumadocs-dashboard-workflow.json",
    monitoredSurfaces: [
      "docs-app-router",
      "dashboard-help-workflow",
      "llm-export",
      "openapi-reference",
      "search-index",
    ],
  },
  receiptIntegrity: {
    hashAlgorithm: "sha256",
    trackedFiles: [
      "core/src/ecosystem/forge_fumadocs.rs",
      "examples/template/package-catalog.ts",
      "examples/template/docs-status.tsx",
      "examples/dashboard/src/lib/fumadocsDocsWorkflow.ts",
      "examples/dashboard/src/components/FumadocsDocsWorkflow.tsx",
      "docs/packages/content-fumadocs-next.md",
    ],
    staleReceiptPolicy:
      "dx-check should mark the Documentation System receipt stale when any tracked source-surface hash changes without a refreshed receipt.",
  },
  dxStyleCompatibility: {
    schema: "dx.forge.package.dx_style_compatibility",
    tokenSource: "styles/theme.css",
    generatedCss: "styles/generated.css",
    visibleSurfaces: [
      "dashboard-help-workflow",
      "docs-app-router",
      "openapi-reference",
    ],
    sourceFiles: [
      "examples/template/docs-status.tsx",
      "examples/dashboard/src/components/FumadocsDocsWorkflow.tsx",
      "core/src/ecosystem/forge_fumadocs.rs",
    ],
    runtimeProof: false,
    runtimeLimitations: [
      "governed browser QA",
      "live Fumadocs renderer proof",
      "existing app theme token review",
    ],
  },
  requiredEnv: [dxFumadocsOpenAPIContract.allowedOriginsEnv],
  receiptPaths: [
    ".dx/forge/docs/content-fumadocs-next.md",
    ".dx/forge/receipts/*-content-fumadocs-next.json",
    "docs/packages/content-fumadocs-next.md",
    "examples/template/.dx/forge/receipts/2026-05-22-content-fumadocs-dashboard-workflow.json",
  ],
  upstreamPublicApis: [
    dxFumadocsNavigationContract.upstreamApis[0],
    dxFumadocsNavigationContract.upstreamApis[1],
    dxFumadocsNavigationContract.upstreamApis[2],
    dxFumadocsLLMsContract.upstreamApi,
    dxFumadocsOpenAPIContract.upstreamApis[0],
    "createFromSource from fumadocs-core/search/server",
    `useDocsSearch preset ${dxFumadocsSearchClientContract.fetchPreset}`,
  ],
  domMarkers: [
    "data-dx-component=\"launch-fumadocs-docs-workflow\"",
    "data-dx-dashboard-workflow=\"docs-help-changelog\"",
    "data-dx-product-surface=\"dashboard-help-content\"",
    "data-dx-fumadocs-interaction=\"page-tree-selector\"",
    "data-dx-fumadocs-action=\"safe-local-route-preview\"",
    "data-dx-fumadocs-page-option",
    "data-dx-fumadocs-rendered-markdown",
    "data-dx-fumadocs-changelog",
    "data-dx-fumadocs-rendered-route",
    "data-dx-fumadocs-selected-page",
    "data-dx-fumadocs-toc-count",
    "data-dx-fumadocs-local-response",
    "data-dx-fumadocs-receipt-route",
    "data-dx-fumadocs-missing-config",
    "data-dx-docs-openapi-code-usage",
    "data-dx-docs-openapi-proxy",
  ],
  appOwnedBoundaries: [
    "Content governance, private-doc exclusion, route publishing, search budgets, and OpenAPI proxy allowed origins.",
    "Dependency installation stays app-owned; the DX/Forge starter path does not create template-local node_modules.",
  ],
} as const;

export const dxFumadocsDashboardPages: DxFumadocsDashboardPage[] = [
  {
    id: "overview",
    title: "Launch docs",
    route: dxFumadocsRouteContract.docsRoute,
    description:
      "Editable Documentation System App Router docs generated from source-owned files.",
    breadcrumb: ["Docs", "Launch", "Overview"],
    peers: ["API reference", "LLM export"],
    toc: ["Route contract", "Source content", "Search boundary"],
  },
  {
    id: "api-reference",
    title: "API reference",
    route: dxFumadocsRouteContract.openApiDocsRoute,
    description:
      "Documentation System OpenAPI virtual docs with an app-owned proxy boundary.",
    breadcrumb: ["Docs", "API", "Reference"],
    peers: ["Launch docs", "LLM export"],
    toc: ["OpenAPI schema", "Allowed origins", "Request snippets"],
  },
  {
    id: "llm-export",
    title: "LLM export",
    route: dxFumadocsRouteContract.llmsIndexRoute,
    description:
      "AI-readable docs index powered by the upstream llms(source) contract.",
    breadcrumb: ["Docs", "AI", "LLMs"],
    peers: ["Launch docs", "API reference"],
    toc: ["Index route", "Full export", "Private content boundary"],
  },
];

export function getDxFumadocsDashboardPage(
  pageId: DxFumadocsDashboardPageId,
) {
  return (
    dxFumadocsDashboardPages.find((page) => page.id === pageId) ??
    dxFumadocsDashboardPages[0]
  );
}

export function createFumadocsNavigationReceipt({
  pageId,
}: {
  pageId: DxFumadocsDashboardPageId;
}): DxFumadocsDashboardReceipt {
  const page = getDxFumadocsDashboardPage(pageId);

  return {
    packageId: dxFumadocsDashboardWorkflowContract.packageId,
    status: "local-preview",
    selectedPageId: page.id,
    route: page.route,
    breadcrumb: page.breadcrumb.join(" / "),
    tocCount: page.toc.length,
    peerCount: page.peers.length,
    nextAction: `Materialize the ${dxFumadocsDashboardWorkflowContract.officialName} package, review content policy, and configure ${dxFumadocsOpenAPIContract.allowedOriginsEnv} before proxying external OpenAPI requests.`,
  };
}
"#;

const FUMADOCS_METADATA_TS: &str = r#"import { dxFumadocsRouteContract } from "./route-contract";

export const dxFumadocsNextPackage = {
  packageId: "content/fumadocs-next",
  officialName: "Documentation System",
  aliases: ["fumadocs", "fumadocs-next", "docs"],
  upstreamPackage: "fumadocs",
  basedOn: "G:/WWW/inspirations/fumadocs",
  sourceMirror: "G:/WWW/inspirations/fumadocs",
  provenance:
    "Inspected G:/WWW/inspirations/fumadocs packages/core public exports, page-tree helpers, source loader, llms route helpers, search APIs, packages/mdx config, packages/openapi server/UI exports, and next-min example before curating this source-owned DX package slice.",
  upstreamPackages: [
    { name: "fumadocs-core", version: "16.8.12" },
    { name: "fumadocs-ui", version: "16.8.12" },
    { name: "fumadocs-mdx", version: "15.0.7" },
    { name: "fumadocs-openapi", version: "10.8.6" },
  ],
  forgeVersion: "16.8.12-dx.11",
  framework: "next-app-router",
  requiredEnv: ["DX_FUMADOCS_OPENAPI_ALLOWED_ORIGINS"],
  receiptPaths: [
    ".dx/forge/docs/content-fumadocs-next.md",
    ".dx/forge/receipts/*-content-fumadocs-next.json",
    "docs/packages/content-fumadocs-next.md",
    "examples/template/.dx/forge/receipts/2026-05-22-content-fumadocs-dashboard-workflow.json",
  ],
  selectedSurfaces: [
    "docs-app-router",
    "docs-runtime-readiness",
    "dashboard-help-workflow",
    "llm-export",
    "openapi-reference",
    "search-index",
  ],
  receiptIntegrity: {
    hashAlgorithm: "sha256",
    trackedFiles: [
      "core/src/ecosystem/forge_fumadocs.rs",
      "examples/template/package-catalog.ts",
      "examples/template/docs-status.tsx",
      "examples/dashboard/src/lib/fumadocsDocsWorkflow.ts",
      "examples/dashboard/src/components/FumadocsDocsWorkflow.tsx",
      "docs/packages/content-fumadocs-next.md",
    ],
    staleReceiptPolicy:
      "dx-check should mark the Documentation System receipt stale when any tracked source-surface hash changes without a refreshed receipt.",
  },
  dxStyleCompatibility: {
    schema: "dx.forge.package.dx_style_compatibility",
    tokenSource: "styles/theme.css",
    generatedCss: "styles/generated.css",
    visibleSurfaces: [
      "dashboard-help-workflow",
      "docs-app-router",
      "openapi-reference",
    ],
    sourceFiles: [
      "examples/template/docs-status.tsx",
      "examples/dashboard/src/components/FumadocsDocsWorkflow.tsx",
      "core/src/ecosystem/forge_fumadocs.rs",
    ],
    runtimeProof: false,
    runtimeLimitations: [
      "governed browser QA",
      "live Fumadocs renderer proof",
      "existing app theme token review",
    ],
  },
  publicApi: [
    "extensionless dx config owns WWW/Fumadocs adapter settings; typed DxConfig extracts framework.www.* and framework.fumadocs.* values",
    "loader from fumadocs-core/source",
    "iconPlugin from fumadocs-core/source/icon",
    "DX Icon component from @/components/icons/icon",
    "statusBadgesPlugin from fumadocs-core/source/status-badges",
    "slugsFromData from fumadocs-core/source/slugs",
    "getBreadcrumbItems from fumadocs-core/breadcrumb",
    "flattenTree, findNeighbour, and getPageTreePeers from fumadocs-core/page-tree",
    "getTableOfContents from fumadocs-core/content/toc",
    "TOCItemType from fumadocs-core/toc",
    "page.data.toc from fumadocs-mdx runtime",
    "llms from fumadocs-core/source",
    "page.data.getText(\"processed\") from fumadocs-mdx runtime",
    "createOpenAPI from fumadocs-openapi/server",
    "openapi.staticSource from fumadocs-openapi/server",
    "openapi.loaderPlugin() from fumadocs-openapi/server",
    "openapi.createProxy from fumadocs-openapi/server",
    "createAPIPage from fumadocs-openapi/ui",
    "createCodeUsageGeneratorRegistry from fumadocs-openapi/requests/generators",
    "registerDefault from fumadocs-openapi/requests/generators/all",
    "defineClientConfig from fumadocs-openapi/ui/client",
    "page.data.getAPIPageProps() from fumadocs-openapi/server",
    "page.data.getSchema() from fumadocs-openapi/server",
    "createFromSource from fumadocs-core/search/server",
    "staticGET from Fumadocs SearchAPI",
    "useDocsSearch from fumadocs-core/search/client",
    "DocsLayout from fumadocs-ui/layouts/docs",
    "RootProvider from fumadocs-ui/provider/next",
    "DocsPage, DocsBody, DocsTitle, DocsDescription from fumadocs-ui/layouts/docs/page",
    "createRelativeLink and default MDX components from fumadocs-ui/mdx",
    "FumadocsDocsWorkflow dashboard workflow component",
    "LaunchDocsStatus docs/help/changelog workflow component",
  ],
  materializedFiles: [
    "lib/fumadocs/source.ts",
    "lib/fumadocs/source-plugins.tsx",
    "lib/fumadocs/navigation.ts",
    "lib/fumadocs/toc.ts",
    "lib/fumadocs/layout.tsx",
    "lib/fumadocs/llms.ts",
    "lib/fumadocs/openapi.ts",
    "lib/fumadocs/openapi-code-usage.ts",
    "lib/fumadocs/search.ts",
    "lib/fumadocs/search-client.ts",
    "lib/fumadocs/readiness.ts",
    "lib/fumadocs/dashboard-workflow.ts",
    "lib/fumadocs/metadata.ts",
    "lib/fumadocs/route-contract.ts",
    "components/mdx.tsx",
    "components/api-page.tsx",
    "components/api-page.client.tsx",
    "components/dashboard/fumadocs-docs-workflow.tsx",
    "app/docs/layout.tsx",
    "app/docs/[[...slug]]/page.tsx",
    "app/docs/readiness/route.ts",
    "app/llms.txt/route.ts",
    "app/llms-full.txt/route.ts",
    "app/llms.mdx/docs/[[...slug]]/route.ts",
    "app/api/search/route.ts",
    "app/api/search-static/route.ts",
    "app/api/openapi/proxy/route.ts",
    "content/docs/meta.json",
    "content/docs/index.mdx",
    "openapi/dx-launch.yaml",
    "lib/fumadocs/README.md",
  ],
  requiredDependencies: [
    {
      name: "fumadocs-core",
      version: "^16.8.12",
      reason: "Source loader, page tree, schemas, and content utilities.",
    },
    {
      name: "fumadocs-ui",
      version: "^16.8.12",
      reason: "Docs layouts, page primitives, MDX components, provider, and CSS preset.",
    },
    {
      name: "fumadocs-mdx",
      version: "^15.0.7",
      reason: "MDX collection config and generated source data.",
    },
    {
      name: "fumadocs-openapi",
      version: "^10.8.6",
      reason: "OpenAPI schema loader, virtual docs pages, APIPage UI, and source loader plugin.",
    },
    {
      name: "zod",
      version: "^4.4.3",
      reason: "Typed Fumadocs page frontmatter schema extensions for status and slug.",
    },
    {
      name: "next",
      version: "^15.3.0 || ^16.0.0",
      reason: "App Router routes and metadata APIs used by the template.",
    },
    {
      name: "react",
      version: "^19.2.0",
      reason: "Fumadocs UI and provider peer dependency.",
    },
    {
      name: "react-dom",
      version: "^19.2.0",
      reason: "Fumadocs UI and provider peer dependency.",
    },
  ],
  discovery: {
    dxAdd: "dx add content/fumadocs-next --write",
    dxDryRun: "dx add content/fumadocs-next --dry-run --format json",
    zedSurface: "docs_template",
    routeContractFile: "lib/fumadocs/route-contract.ts",
    sourcePluginFile: "lib/fumadocs/source-plugins.tsx",
    sourcePluginIconSurface: "dx-icons",
    dxIconComponentFile: "components/icons/icon.tsx",
    sourcePluginFrontmatterFields: ["icon", "status", "slug"],
    navigationFile: "lib/fumadocs/navigation.ts",
    navigationSurfaces: ["breadcrumb", "previous-next", "sidebar-count", "peers"],
    tocFile: "lib/fumadocs/toc.ts",
    tocSurfaces: ["docs-page-outline", "markdown-outline", "toc-summary"],
    llmsIndexRoute: "/llms.txt",
    llmsFullRoute: "/llms-full.txt",
    llmsPageMarkdownRoute: "/llms.mdx/docs/[[...slug]]",
    llmsConfigFile: "lib/fumadocs/llms.ts",
    readinessRoute: "/docs/readiness",
    readinessRouteFile: "app/docs/readiness/route.ts",
    readinessSourceFile: "lib/fumadocs/readiness.ts",
    openApiDocsRoute: "/docs/api",
    openApiSchemaFile: "openapi/dx-launch.yaml",
    openApiConfigFile: "lib/fumadocs/openapi.ts",
    openApiCodeUsageFile: "lib/fumadocs/openapi-code-usage.ts",
    openApiProxyRoute: "/api/openapi/proxy",
    openApiProxyRouteFile: "app/api/openapi/proxy/route.ts",
    openApiProxyAllowedOriginsEnv: "DX_FUMADOCS_OPENAPI_ALLOWED_ORIGINS",
    openApiPageComponentFile: "components/api-page.tsx",
    openApiClientConfigFile: "components/api-page.client.tsx",
    searchRoute: "/api/search",
    staticSearchRoute: "/api/search-static",
    searchConfigFile: "lib/fumadocs/search.ts",
    searchClientFile: "lib/fumadocs/search-client.ts",
    dashboardWorkflowApiFile: "lib/fumadocs/dashboard-workflow.ts",
    dashboardWorkflowFile: "components/dashboard/fumadocs-docs-workflow.tsx",
    launchDashboardWorkflowFile: "components/launch/docs-status.tsx",
    launchDashboardWorkflow: "docs-help-changelog",
    launchDashboardMarkers: [
      "data-dx-component=\"launch-fumadocs-docs-workflow\"",
      "data-dx-dashboard-workflow=\"docs-help-changelog\"",
      "data-dx-dashboard-card=\"docs-help\"",
      "data-dx-fumadocs-dashboard-target=\"mission-control-docs\"",
      "data-dx-fumadocs-interaction=\"page-tree-selector\"",
      "data-dx-fumadocs-action=\"safe-local-route-preview\"",
      "data-dx-fumadocs-page-option",
      "data-dx-fumadocs-rendered-markdown",
      "data-dx-fumadocs-changelog",
      "data-dx-fumadocs-rendered-route",
      "data-dx-fumadocs-selected-page",
      "data-dx-fumadocs-toc-count",
      "data-dx-fumadocs-local-response",
      "data-dx-fumadocs-receipt-route",
      "data-dx-fumadocs-missing-config",
      "data-dx-docs-icon-source=\"dx-icons\"",
      "data-dx-docs-readiness",
      "data-dx-docs-openapi-code-usage",
      "data-dx-docs-openapi-proxy",
    ],
  },
  routeContract: dxFumadocsRouteContract,
  boundaries: [
    "Forge materializes editable docs app source; it does not run create-fumadocs-app or package lifecycle scripts.",
    "The application owns dependency installation, framework.www.* and framework.fumadocs.* values in dx, source plugin taxonomy, navigation policy, toc policy, slug/canonical URL policy, OpenAPI schema governance, OpenAPI proxy allowed origins, auth forwarding policy, request code sample policy, search UI, AI indexing policy, private content exclusion, multilingual/vector policy, hosting, and content governance.",
  ],
} as const;

export type DxFumadocsNextPackage = typeof dxFumadocsNextPackage;
"#;

const FUMADOCS_ROUTE_CONTRACT_TS: &str = r#"export const dxFumadocsRouteContract = {
  packageId: "content/fumadocs-next",
  route: "/docs",
  docsRoute: "/docs",
  readinessRoute: "/docs/readiness",
  llmsIndexRoute: "/llms.txt",
  llmsFullRoute: "/llms-full.txt",
  llmsPageMarkdownRoute: "/llms.mdx/docs/[[...slug]]",
  openApiDocsRoute: "/docs/api",
  openApiProxyRoute: "/api/openapi/proxy",
  openApiProxyAllowedOriginsEnv: "DX_FUMADOCS_OPENAPI_ALLOWED_ORIGINS",
  searchRoute: "/api/search",
  staticSearchRoute: "/api/search-static",
  contentDir: "content/docs",
  sourcePluginFile: "lib/fumadocs/source-plugins.tsx",
  iconComponentFile: "components/icons/icon.tsx",
  sourcePluginIconSurface: "dx-icons",
  sourcePluginFrontmatterFields: ["icon", "status", "slug"],
  navigationFile: "lib/fumadocs/navigation.ts",
  navigationSurfaces: ["breadcrumb", "previous-next", "sidebar-count", "peers"],
  tocFile: "lib/fumadocs/toc.ts",
  tocSurfaces: ["docs-page-outline", "markdown-outline", "toc-summary"],
  openApiSchemaFile: "openapi/dx-launch.yaml",
  configOwnerFile: "dx",
  dxConfigKeys: {
    wwwConfigOwner: "framework.www.config_owner_file",
    wwwGeneratedConfigFiles: "framework.www.config_files",
    docsRoute: "framework.fumadocs.docs_route",
    readinessRoute: "framework.fumadocs.readiness_route",
    openApiProxyRoute: "framework.fumadocs.openapi_proxy_route",
    openApiAllowedOriginsEnv: "framework.fumadocs.openapi_allowed_origins_env",
    searchRoute: "framework.fumadocs.search_route",
    staticSearchRoute: "framework.fumadocs.static_search_route",
    generatedRoutes: "framework.fumadocs.generated_routes",
    requiredRuntimePackages: "framework.fumadocs.required_runtime_packages",
  },
  sourceFile: "lib/fumadocs/source.ts",
  layoutOptionsFile: "lib/fumadocs/layout.tsx",
  llmsConfigFile: "lib/fumadocs/llms.ts",
  openApiConfigFile: "lib/fumadocs/openapi.ts",
  openApiCodeUsageFile: "lib/fumadocs/openapi-code-usage.ts",
  openApiProxyRouteFile: "app/api/openapi/proxy/route.ts",
  searchConfigFile: "lib/fumadocs/search.ts",
  searchClientFile: "lib/fumadocs/search-client.ts",
  metadataFile: "lib/fumadocs/metadata.ts",
  routeContractFile: "lib/fumadocs/route-contract.ts",
  dashboardWorkflowApiFile: "lib/fumadocs/dashboard-workflow.ts",
  dashboardWorkflowFile: "components/dashboard/fumadocs-docs-workflow.tsx",
  mdxComponentsFile: "components/mdx.tsx",
  openApiPageComponentFile: "components/api-page.tsx",
  openApiClientConfigFile: "components/api-page.client.tsx",
  docsLayoutFile: "app/docs/layout.tsx",
  docsPageFile: "app/docs/[[...slug]]/page.tsx",
  readinessRouteFile: "app/docs/readiness/route.ts",
  llmsIndexRouteFile: "app/llms.txt/route.ts",
  llmsFullRouteFile: "app/llms-full.txt/route.ts",
  llmsPageMarkdownRouteFile: "app/llms.mdx/docs/[[...slug]]/route.ts",
  searchRouteFile: "app/api/search/route.ts",
  staticSearchRouteFile: "app/api/search-static/route.ts",
  searchQuery: {
    queryParam: "query",
    optionalParams: ["locale", "tag", "limit", "mode"],
  },
  starterContentFiles: [
    "content/docs/meta.json",
    "content/docs/index.mdx",
    "openapi/dx-launch.yaml",
  ],
  requiredRuntimePackages: [
    "fumadocs-core",
    "fumadocs-ui",
    "fumadocs-mdx",
    "fumadocs-openapi",
    "zod",
    "next",
    "react",
    "react-dom",
  ],
  generatedRoutes: [
    "/docs",
    "/docs/[[...slug]]",
    "/docs/readiness",
    "/docs/api",
    "/api/openapi/proxy",
    "/llms.txt",
    "/llms-full.txt",
    "/llms.mdx/docs/[[...slug]]",
    "/api/search",
    "/api/search-static",
  ],
  appOwnedBoundaries: [
    "Dependency installation and version policy.",
    "framework.www.* and framework.fumadocs.* values in dx.",
    "Docs content governance and editorial workflow.",
    "Source plugin taxonomy, DX Icon catalog governance, status lifecycle, slug redirects, canonical URL policy, docs navigation policy, and toc policy.",
    "OpenAPI schema governance, proxy/auth policy, allowed origins, request playground safety, and deployed API drift review.",
    "OpenAPI request code sample policy, SDK naming, auth header redaction, generated sample QA, and media adapter policy.",
    "AI crawler exposure, private content exclusion, and generated text retention policy.",
    "Search UI, static index payload policy, analytics, multilingual/vector policy, deployment, and runtime verification.",
  ],
  verification: {
    sourceGuard:
      "dx run --test benchmarks/fumadocs-dashboard-workflow.test.ts benchmarks/fumadocs-toc-slice.test.ts benchmarks/fumadocs-navigation-slice.test.ts benchmarks/fumadocs-source-plugins-slice.test.ts benchmarks/fumadocs-openapi-proxy-slice.test.ts benchmarks/fumadocs-openapi-code-usage-slice.test.ts benchmarks/fumadocs-openapi-slice.test.ts benchmarks/fumadocs-llms-slice.test.ts benchmarks/fumadocs-search-slice.test.ts",
    runtimeStatus: "source-owned-readiness-route",
  },
} as const;

export type DxFumadocsRouteContract = typeof dxFumadocsRouteContract;
"#;

const FUMADOCS_READINESS_TS: &str = r#"import { dxFumadocsNextPackage } from "./metadata";
import { dxFumadocsRouteContract } from "./route-contract";

const requiredRoutes = [
  dxFumadocsRouteContract.docsRoute,
  dxFumadocsRouteContract.readinessRoute,
  dxFumadocsRouteContract.llmsIndexRoute,
  dxFumadocsRouteContract.llmsFullRoute,
  dxFumadocsRouteContract.searchRoute,
  dxFumadocsRouteContract.staticSearchRoute,
  dxFumadocsRouteContract.openApiProxyRoute,
] as const;

const requiredMaterializedFiles = [
  dxFumadocsRouteContract.docsPageFile,
  dxFumadocsRouteContract.readinessRouteFile,
  dxFumadocsRouteContract.sourcePluginFile,
  dxFumadocsRouteContract.metadataFile,
  dxFumadocsRouteContract.routeContractFile,
  dxFumadocsRouteContract.sourceFile,
  "lib/fumadocs/readiness.ts",
] as const;

export function createDxFumadocsReadinessReport() {
  const generatedRoutes = new Set(dxFumadocsRouteContract.generatedRoutes);
  const materializedFiles = new Set(dxFumadocsNextPackage.materializedFiles);
  const missingRoutes = requiredRoutes.filter((route) => !generatedRoutes.has(route));
  const missingMaterializedFiles = requiredMaterializedFiles.filter(
    (file) => !materializedFiles.has(file),
  );
  const status =
    missingRoutes.length === 0 && missingMaterializedFiles.length === 0
      ? "source-ready"
      : "source-incomplete";

  return {
    schema: "dx.fumadocs.runtime_readiness",
    packageId: dxFumadocsRouteContract.packageId,
    officialPackageName: dxFumadocsNextPackage.officialName,
    status,
    routeHandlerMaterialized: true,
    runtimeExecution: false,
    liveRouteExecutionProof: false,
    liveFumadocsRendererProof: false,
    hostedSearchProof: false,
    openApiProxyExecution: false,
    readinessRoute: dxFumadocsRouteContract.readinessRoute,
    docsRoute: dxFumadocsRouteContract.docsRoute,
    iconSurface: dxFumadocsRouteContract.sourcePluginIconSurface,
    iconComponentFile: dxFumadocsRouteContract.iconComponentFile,
    routeContractFile: dxFumadocsRouteContract.routeContractFile,
    dxConfigKeys: dxFumadocsRouteContract.dxConfigKeys,
    requiredRoutes,
    missingRoutes,
    requiredMaterializedFiles,
    missingMaterializedFiles,
    boundary:
      "This route proves source-owned Documentation System wiring only; live rendering, hosted search indexing, OpenAPI proxy execution, dependency installation, and browser QA stay app-owned.",
  } as const;
}
"#;

const FUMADOCS_MDX_COMPONENTS_TSX: &str = r#"import { APIPage } from "@/components/api-page";
import defaultMdxComponents from "fumadocs-ui/mdx";
import type { MDXComponents } from "mdx/types";

export function getMDXComponents(components?: MDXComponents): MDXComponents {
  return {
    ...defaultMdxComponents,
    APIPage,
    ...components,
  };
}

export function useMDXComponents(components: MDXComponents): MDXComponents {
  return getMDXComponents(components);
}

declare global {
  type MDXProvidedComponents = (components: MDXComponents) => MDXComponents;
}
"#;

const FUMADOCS_API_PAGE_TSX: &str = r#"import { dxFumadocsOpenAPI } from "@/lib/fumadocs/openapi";
import { dxFumadocsOpenAPICodeUsages } from "@/lib/fumadocs/openapi-code-usage";
import { createAPIPage } from "fumadocs-openapi/ui";
import dxFumadocsOpenAPIClientConfig from "./api-page.client";

export const APIPage = createAPIPage(dxFumadocsOpenAPI, {
  client: dxFumadocsOpenAPIClientConfig,
  codeUsages: dxFumadocsOpenAPICodeUsages,
});
"#;

const FUMADOCS_API_PAGE_CLIENT_TSX: &str = r#""use client";

import { dxFumadocsOpenAPICodeUsages } from "@/lib/fumadocs/openapi-code-usage";
import { defineClientConfig } from "fumadocs-openapi/ui/client";

export const dxFumadocsOpenAPIClientConfig = defineClientConfig({
  storageKeyPrefix: "dx-fumadocs-openapi-",
  codeUsages: dxFumadocsOpenAPICodeUsages,
});

export default dxFumadocsOpenAPIClientConfig;
"#;

const FUMADOCS_DASHBOARD_WORKFLOW_TSX: &str = r#""use client";

import * as React from "react";

import {
  createFumadocsNavigationReceipt,
  dxFumadocsDashboardPages,
  dxFumadocsDashboardWorkflowContract,
  getDxFumadocsDashboardPage,
  type DxFumadocsDashboardPageId,
} from "@/lib/fumadocs/dashboard-workflow";

export function FumadocsDocsWorkflow() {
  const [pageId, setPageId] =
    React.useState<DxFumadocsDashboardPageId>("overview");
  const [receipt, setReceipt] = React.useState<ReturnType<
    typeof createFumadocsNavigationReceipt
  > | null>(null);
  const activePage = getDxFumadocsDashboardPage(pageId);

  return (
    <section
      className="grid gap-3 rounded-lg border bg-card p-4 text-card-foreground"
      data-dx-package={dxFumadocsDashboardWorkflowContract.packageId}
      data-dx-component="dashboard-fumadocs-docs-workflow"
      data-dx-fumadocs-dashboard-workflow="docs-ops"
      data-dx-style-surface="documentation-system"
      data-dx-fumadocs-route={activePage.route}
      data-dx-fumadocs-selected-page={activePage.id}
      data-dx-node-modules="forbidden"
    >
      <header className="flex items-start gap-3">
        {React.createElement("dx-icon", {
          name: "pack:fumadocs",
          "aria-label": "Documentation System",
        })}
        <div>
          <h2 className="text-base font-semibold">Documentation System Workflow</h2>
          <p className="text-sm text-muted-foreground">
            Operate docs, OpenAPI, search, and LLM handoff routes from the
            source-owned package slice.
          </p>
        </div>
      </header>

      <div
        className="flex flex-wrap gap-2"
        data-dx-fumadocs-interaction="page-tree-selector"
      >
        {dxFumadocsDashboardPages.map((page) => {
          const selected = page.id === activePage.id;

          return (
            <button
              key={page.id}
              type="button"
              className={[
                "rounded-md border px-3 py-2 text-left text-sm transition",
                selected
                  ? "border-foreground bg-foreground text-background"
                  : "border-border bg-background text-muted-foreground hover:text-foreground",
              ].join(" ")}
              data-dx-fumadocs-page-option={page.id}
              data-dx-fumadocs-page-route={page.route}
              data-dx-fumadocs-page-selected={selected ? "true" : "false"}
              aria-pressed={selected}
              onClick={() => {
                setPageId(page.id);
                setReceipt(null);
              }}
            >
              <span className="block font-medium">{page.title}</span>
              <span className="block text-xs opacity-80">{page.route}</span>
            </button>
          );
        })}
      </div>

      <div
        className="grid gap-3 rounded-md border p-3 text-sm"
        data-dx-fumadocs-navigation-snapshot={activePage.breadcrumb.join("/")}
      >
        <p data-dx-fumadocs-route-contract={activePage.route}>
          {activePage.description}
        </p>
        <dl className="grid gap-2 md:grid-cols-2">
          <div>
            <dt className="text-xs text-muted-foreground">Breadcrumb</dt>
            <dd data-dx-fumadocs-breadcrumb={activePage.breadcrumb.join("/")}>
              {activePage.breadcrumb.join(" / ")}
            </dd>
          </div>
          <div>
            <dt className="text-xs text-muted-foreground">Required env</dt>
            <dd
              data-dx-fumadocs-required-env={
                dxFumadocsDashboardWorkflowContract.requiredEnv[0]
              }
            >
              {dxFumadocsDashboardWorkflowContract.requiredEnv[0]}
            </dd>
          </div>
        </dl>
      </div>

      <div
        className="grid gap-2 rounded-md border p-3 text-sm"
        data-dx-fumadocs-rendered-markdown="active-dashboard-page"
      >
        <div className="flex flex-wrap items-center justify-between gap-2">
          <h3 className="font-medium">{activePage.title}</h3>
          <span className="rounded-sm border px-2 py-1 text-xs text-muted-foreground">
            {activePage.route}
          </span>
        </div>
        <ul className="grid gap-1" data-dx-fumadocs-toc-list={activePage.id}>
          {activePage.toc.map((item) => (
            <li key={item} data-dx-fumadocs-toc-item={item}>
              {item}
            </li>
          ))}
        </ul>
        <p
          className="text-xs text-muted-foreground"
          data-dx-fumadocs-public-api={
            dxFumadocsDashboardWorkflowContract.upstreamPublicApis.join(",")
          }
        >
          APIs:{" "}
          {dxFumadocsDashboardWorkflowContract.upstreamPublicApis
            .slice(0, 3)
            .join(", ")}
          .
        </p>
      </div>

      <button
        type="button"
        className="inline-flex w-fit items-center gap-2 rounded-md border px-3 py-2 text-sm font-medium"
        data-dx-fumadocs-action="safe-local-route-preview"
        onClick={() => setReceipt(createFumadocsNavigationReceipt({ pageId }))}
      >
        {React.createElement("dx-icon", {
          name: "pack:search",
          "aria-hidden": "true",
        })}
        Preview route receipt
      </button>

      <p
        role="status"
        aria-live="polite"
        className="rounded-md border p-3 text-sm text-muted-foreground"
        data-dx-fumadocs-local-response={receipt ? receipt.status : "idle"}
        data-dx-fumadocs-receipt-route={receipt?.route ?? "none"}
      >
        {receipt
          ? `${receipt.packageId} selected ${receipt.route} with ${receipt.tocCount} TOC entries. ${receipt.nextAction}`
          : "Select a docs route, then preview the local Documentation System route receipt."}
      </p>
    </section>
  );
}

export default FumadocsDocsWorkflow;
"#;

const FUMADOCS_DOCS_LAYOUT_TSX: &str = r#"import { source } from "@/lib/fumadocs/source";
import { baseOptions } from "@/lib/fumadocs/layout";
import { DocsLayout } from "fumadocs-ui/layouts/docs";
import { RootProvider } from "fumadocs-ui/provider/next";
import "fumadocs-openapi/css/preset.css";
import "fumadocs-ui/css/neutral.css";
import "fumadocs-ui/css/preset.css";

export default function Layout({ children }: LayoutProps<"/docs">) {
  return (
    <RootProvider>
      <DocsLayout tree={source.getPageTree()} {...baseOptions()}>
        {children}
      </DocsLayout>
    </RootProvider>
  );
}
"#;

const FUMADOCS_DOCS_PAGE_TSX: &str = r#"import { APIPage } from "@/components/api-page";
import { getMDXComponents } from "@/components/mdx";
import { source } from "@/lib/fumadocs/source";
import { getDxFumadocsPageToc } from "@/lib/fumadocs/toc";
import { createRelativeLink } from "fumadocs-ui/mdx";
import {
  DocsBody,
  DocsDescription,
  DocsPage,
  DocsTitle,
} from "fumadocs-ui/layouts/docs/page";
import type { Metadata } from "next";
import { notFound } from "next/navigation";

export default async function Page(props: PageProps<"/docs/[[...slug]]">) {
  const params = await props.params;
  const page = source.getPage(params.slug);
  if (!page) notFound();

  if (page.type === "openapi") {
    return (
      <DocsPage toc={getDxFumadocsPageToc(page)} full>
        <DocsTitle>{page.data.title}</DocsTitle>
        <DocsDescription>{page.data.description}</DocsDescription>
        <DocsBody>
          <APIPage {...page.data.getAPIPageProps()} />
        </DocsBody>
      </DocsPage>
    );
  }

  const MDX = page.data.body;

  return (
    <DocsPage toc={getDxFumadocsPageToc(page)} full={page.data.full}>
      <DocsTitle>{page.data.title}</DocsTitle>
      <DocsDescription>{page.data.description}</DocsDescription>
      <DocsBody>
        <MDX
          components={getMDXComponents({
            a: createRelativeLink(source, page),
          })}
        />
      </DocsBody>
    </DocsPage>
  );
}

export async function generateStaticParams() {
  return source.generateParams();
}

export async function generateMetadata(
  props: PageProps<"/docs/[[...slug]]">,
): Promise<Metadata> {
  const params = await props.params;
  const page = source.getPage(params.slug);
  if (!page) notFound();

  return {
    title: page.data.title,
    description: page.data.description,
  };
}
"#;

const FUMADOCS_LLMS_TXT_ROUTE_TS: &str = r#"import { createDxFumadocsLLMsIndex } from "@/lib/fumadocs/llms";

export const revalidate = false;

export function GET() {
  return new Response(createDxFumadocsLLMsIndex().index(), {
    headers: {
      "Content-Type": "text/plain; charset=utf-8",
    },
  });
}
"#;

const FUMADOCS_LLMS_FULL_TXT_ROUTE_TS: &str = r#"import { getDxFumadocsLLMText } from "@/lib/fumadocs/llms";
import { source } from "@/lib/fumadocs/source";

export const revalidate = false;

export async function GET() {
  const pages = await Promise.all(source.getPages().map(getDxFumadocsLLMText));

  return new Response(pages.join("\n\n"), {
    headers: {
      "Content-Type": "text/plain; charset=utf-8",
    },
  });
}
"#;

const FUMADOCS_LLMS_MDX_ROUTE_TS: &str = r#"import {
  getDxFumadocsLLMText,
  getDxFumadocsPageMarkdownUrl,
} from "@/lib/fumadocs/llms";
import { source } from "@/lib/fumadocs/source";
import { notFound } from "next/navigation";

export const revalidate = false;

export async function GET(
  _request: Request,
  { params }: RouteContext<"/llms.mdx/docs/[[...slug]]">,
) {
  const { slug } = await params;
  const page = source.getPage(slug?.slice(0, -1));
  if (!page) notFound();

  return new Response(await getDxFumadocsLLMText(page), {
    headers: {
      "Content-Type": "text/markdown; charset=utf-8",
    },
  });
}

export function generateStaticParams() {
  return source.getPages().map((page) => ({
    slug: getDxFumadocsPageMarkdownUrl(page).segments,
  }));
}
"#;

const FUMADOCS_SEARCH_ROUTE_TS: &str = r#"import { createDxFumadocsSearchApi } from "@/lib/fumadocs/search";

const searchApi = createDxFumadocsSearchApi();

export const GET = searchApi.GET;
"#;

const FUMADOCS_STATIC_SEARCH_ROUTE_TS: &str = r#"import { createDxFumadocsSearchApi } from "@/lib/fumadocs/search";

export const revalidate = false;

const searchApi = createDxFumadocsSearchApi();

export const GET = searchApi.staticGET;
"#;

const FUMADOCS_OPENAPI_PROXY_ROUTE_TS: &str = r#"import {
  dxFumadocsOpenAPI,
  readDxFumadocsOpenAPIAllowedOrigins,
} from "@/lib/fumadocs/openapi";

const allowedOrigins = readDxFumadocsOpenAPIAllowedOrigins();

export const { GET, HEAD, PUT, POST, PATCH, DELETE } =
  dxFumadocsOpenAPI.createProxy({
    allowedOrigins,
    filterRequest(request) {
      return allowedOrigins.length > 0 && request.url.startsWith("https://");
    },
  });
"#;

const FUMADOCS_DOCS_READINESS_ROUTE_TS: &str = r#"import { createDxFumadocsReadinessReport } from "@/lib/fumadocs/readiness";

export const runtime = "nodejs";

export function GET() {
  return Response.json(createDxFumadocsReadinessReport(), {
    headers: {
      "x-dx-docs-readiness": "source-owned",
      "x-dx-docs-readiness-marker": "data-dx-docs-readiness",
    },
  });
}
"#;

const FUMADOCS_META_JSON: &str = r#"{
  "title": "DX Launch Docs",
  "pages": ["index"]
}
"#;

const FUMADOCS_INDEX_MDX: &str = r#"---
title: DX Launch Docs
description: Source-owned launch documentation powered by the Documentation System package.
icon: action:rocket
status: beta
slug: ""
---

# DX Launch Docs

This page is editable project content materialized by DX Forge.

## Launch Surface

- App Router docs route at `/docs`
- Documentation System LLM index at `/llms.txt`
- Full processed Markdown export at `/llms-full.txt`
- Per-page Markdown export under `/llms.mdx/docs`
- Documentation System search route at `/api/search?query=launch`
- Static Documentation System search index export at `/api/search-static`
- Documentation System OpenAPI virtual docs under `/docs/api`
- Documentation System OpenAPI playground proxy at `/api/openapi/proxy` gated by `DX_FUMADOCS_OPENAPI_ALLOWED_ORIGINS`
- Documentation System OpenAPI request code samples with default generators plus `dx-launch-fetch`
- Documentation System source plugins for DX Icon page-tree icons, status badges, and frontmatter slugs
- Documentation System TOC helpers for docs page outlines and Markdown heading summaries
- Documentation System MDX collection from `content/docs`
- Documentation System UI layout and page primitives
- Metadata that lets DX CLI and Zed discover the package later

## Next Step

Replace this page with the launch template's product docs, then run the project checks required by your repo before publishing.
"#;

const FUMADOCS_OPENAPI_YAML: &str = r#"openapi: 3.1.0
info:
  title: DX Launch Docs API
  version: 0.1.0
  description: Source-owned API contract for the generated DX launch docs slice.
servers:
  - url: https://example.com
    description: Replace with the deployed application origin before publishing.
tags:
  - name: Docs
    description: Documentation discovery endpoints generated by the Fumadocs slice.
paths:
  /api/search:
    get:
      tags:
        - Docs
      summary: Search launch docs
      description: Query the Fumadocs search index generated from the source-owned docs tree.
      operationId: searchDocs
      parameters:
        - name: query
          in: query
          required: true
          schema:
            type: string
          description: Search text entered by the docs user.
        - name: limit
          in: query
          required: false
          schema:
            type: integer
            minimum: 1
            maximum: 20
          description: Optional app-owned result limit.
      responses:
        "200":
          description: Search results from the Fumadocs search API.
          content:
            application/json:
              schema:
                type: object
                additionalProperties: true
  /api/search-static:
    get:
      tags:
        - Docs
      summary: Read static docs search index
      description: Download the static search index used by the Fumadocs static client preset.
      operationId: readStaticDocsSearchIndex
      responses:
        "200":
          description: Static Fumadocs search index payload.
          content:
            application/json:
              schema:
                type: object
                additionalProperties: true
  /api/openapi/proxy:
    get:
      tags:
        - Docs
      summary: Proxy allowed OpenAPI playground request
      description: Forwards an OpenAPI playground request only when the target origin is configured by DX_FUMADOCS_OPENAPI_ALLOWED_ORIGINS.
      operationId: proxyOpenAPIPlaygroundRequest
      parameters:
        - name: url
          in: query
          required: true
          schema:
            type: string
            format: uri
          description: HTTPS target URL allowed by the application-owned proxy origin policy.
      responses:
        "200":
          description: Proxied response from an allowed OpenAPI target.
          content:
            application/json:
              schema:
                type: object
                additionalProperties: true
"#;

const FUMADOCS_README_MD: &str = r#"# Documentation System

This package materializes a small, editable Documentation System surface for a Next.js App Router project.

Upstream package: `fumadocs`.

## Files

- `dx` is the only generated project config file; WWW/Fumadocs adapter settings stay in `framework.www.*` and `framework.fumadocs.*` instead of `next.config.*` or `source.config.ts`.
- `lib/fumadocs/source.ts` creates the Fumadocs source loader.
- `lib/fumadocs/source-plugins.tsx` wires Fumadocs source plugins for DX Icon page-tree icons, status badges, and `slug` frontmatter.
- `lib/fumadocs/navigation.ts` exposes Fumadocs breadcrumb, previous-next, page-tree count, and peer helpers from the generated docs tree.
- `lib/fumadocs/toc.ts` exposes Fumadocs `getTableOfContents()` and page TOC summary helpers for docs outlines.
- `lib/fumadocs/llms.ts` exposes the real Fumadocs `llms(source)` index and per-page processed Markdown helpers.
- `lib/fumadocs/openapi.ts` creates the Fumadocs OpenAPI server and virtual-docs contract with `createOpenAPI()`.
- `lib/fumadocs/openapi-code-usage.ts` registers Fumadocs OpenAPI default request generators plus the DX launch fetch generator.
- `lib/fumadocs/search.ts` creates the Fumadocs `createFromSource` search API config.
- `lib/fumadocs/search-client.ts` exposes typed `useDocsSearch` client presets for fetch and static modes.
- `lib/fumadocs/readiness.ts` builds the source-owned Documentation System readiness report without claiming live renderer, search, or OpenAPI proxy execution.
- `lib/fumadocs/dashboard-workflow.ts` exposes dashboard workflow pages, public API readiness metadata, and local route receipt construction for starter dashboards.
- `lib/fumadocs/route-contract.ts` records route, file, dependency, and verification ownership for DX CLI and Zed.
- `components/api-page.tsx` exposes the Fumadocs OpenAPI `createAPIPage()` component.
- `components/api-page.client.tsx` exposes the Fumadocs OpenAPI client config with request code usage generators.
- `components/dashboard/fumadocs-docs-workflow.tsx` exposes a visible dashboard workflow with page-tree selection, route readiness, OpenAPI env readiness, LLM export readiness, and a safe local route receipt.
- `app/docs/layout.tsx` mounts `RootProvider` and `DocsLayout`.
- `app/docs/[[...slug]]/page.tsx` renders MDX content and OpenAPI virtual docs with Fumadocs page primitives.
- `app/docs/readiness/route.ts` returns the source-owned readiness report for route, DX Icon, LLM, search, and OpenAPI boundary checks.
- `app/llms.txt/route.ts` renders the Fumadocs `llms(source).index()` Markdown index.
- `app/llms-full.txt/route.ts` renders processed Markdown for every generated docs page.
- `app/llms.mdx/docs/[[...slug]]/route.ts` renders processed Markdown for one generated docs page.
- `app/api/search/route.ts` mounts Fumadocs search with `createFromSource(source, config)`.
- `app/api/search-static/route.ts` exports Fumadocs `staticGET` for client-side static search indexes.
- `app/api/openapi/proxy/route.ts` mounts Fumadocs `openapi.createProxy()` behind an app-owned allowed-origin env boundary.
- `content/docs/index.mdx` is the starter launch-doc page.
- `openapi/dx-launch.yaml` is a starter OpenAPI contract for the generated docs routes.

## Dependencies

Install `fumadocs-core`, `fumadocs-ui`, `fumadocs-mdx`, `fumadocs-openapi`, `zod`, `next`, `react`, and `react-dom` in the app that consumes this slice. Forge writes source files only; it does not run installs or lifecycle scripts. Page-tree icons render through the source-owned DX Icon system in `components/icons/icon.tsx`.

## Application Ownership

Review the extensionless `dx` file before accepting this package into an app that already has custom framework settings. Forge owns the generated docs route, source loader, Fumadocs source plugin wiring, Fumadocs navigation snapshot helpers, Fumadocs TOC extraction helpers, Fumadocs LLM index routes, OpenAPI virtual docs wiring, OpenAPI proxy route wiring, OpenAPI request code usage registry wiring, default Orama-backed search route, static search-index export route, typed client search presets, dashboard workflow receipt helpers, and the dashboard workflow proof component. The application owns dependency installation, source plugin taxonomy, DX Icon catalog governance, status lifecycle, docs navigation policy, toc policy, slug/canonical URL policy, AI indexing policy, private content exclusion, OpenAPI schema governance, OpenAPI proxy allowed origins via `DX_FUMADOCS_OPENAPI_ALLOWED_ORIGINS`, auth/cookie forwarding policy, request-playground policy, request code sample policy, search UI, static-index payload budget, multilingual or vector search policy, deployment settings, content governance, and styling integration beyond the Fumadocs presets imported by the docs layout.
"#;
