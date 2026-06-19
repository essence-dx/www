export type FrameworkCompletenessStatus =
  | "source-owned"
  | "adapter-boundary"
  | "partial";

export type FrameworkCompletenessItem = {
  id: string;
  lane: string;
  label: string;
  status: FrameworkCompletenessStatus;
  packageIds: readonly string[];
};

const item = (
  lane: string,
  id: string,
  label: string,
  status: FrameworkCompletenessStatus,
  packageIds: readonly string[] = [],
): FrameworkCompletenessItem => ({ id, lane, label, status, packageIds });

export const frameworkCompletenessContract = {
  schema: "dx.www.framework_completeness",
  publicAuthoring: "tsx-app-router",
  packagePolicy: "forge-source-owned-visible-files",
  legacyPositioningAliases: ["tsx-first", "forge-first-no-node_modules"],
  stylePolicy: "dx-style-generated-css",
  checkPolicy: "dx-check-receipts",
} as const;

export const officialPackageLabels = [
  "Authentication",
  "Validation & Schemas",
  "Forms",
  "State Management",
  "Data Fetching & Cache",
  "Database ORM",
  "Backend Platform Client",
  "Payments",
  "Internationalization",
  "Markdown & MDX Content",
  "Motion & Animation",
  "UI Components and Icons",
] as const;

export const dxWwwFrameworkCompleteness = [
  item("routing-parity", "nested-layouts", "Nested layouts", "source-owned"),
  item(
    "routing-parity",
    "loading-error-not-found-boundaries",
    "Loading, error, and not-found boundaries",
    "partial",
  ),
  item("routing-parity", "route-groups", "Route groups", "partial"),
  item("routing-parity", "dynamic-params", "Dynamic params", "source-owned"),
  item("routing-parity", "metadata-seo", "Metadata and SEO", "source-owned"),
  item("routing-parity", "route-handlers", "Route handlers", "source-owned"),
  item(
    "server-client-model",
    "server-actions-equivalent",
    "Server actions equivalent",
    "adapter-boundary",
  ),
  item("server-client-model", "form-actions", "Form actions", "source-owned", [
    "forms/react-hook-form",
    "validation/zod",
  ]),
  item(
    "server-client-model",
    "cookies-headers-session-helpers",
    "Cookies, headers, and session helpers",
    "adapter-boundary",
    ["auth/better-auth"],
  ),
  item(
    "server-client-model",
    "streaming-response-boundary",
    "Streaming response boundary",
    "partial",
  ),
  item("server-client-model", "cache-revalidate-story", "Cache and revalidate story", "partial", [
    "tanstack/query",
  ]),
  item("dev-experience", "reliable-hot-reload", "Reliable hot reload", "source-owned"),
  item("dev-experience", "tsx-first-templates", "TSX-first templates", "source-owned"),
  item("dev-experience", "auto-imports", "Auto imports", "source-owned"),
  item("dev-experience", "dx-style-css-generation", "DX Style CSS generation", "source-owned"),
  item("dev-experience", "dx-check-receipts", "DX Check receipts", "source-owned"),
  item("dev-experience", "obvious-cli-path", "Obvious CLI path", "source-owned"),
  item("production-template", "real-dashboard-starter", "Real dashboard starter", "partial"),
  item("production-template", "auth-page", "Authentication", "adapter-boundary", [
    "auth/better-auth",
  ]),
  item("production-template", "settings-validation-form", "Validation & Schemas", "source-owned", [
    "validation/zod",
  ]),
  item("production-template", "payment-plan-page", "Payments", "adapter-boundary", [
    "payments/stripe-js",
  ]),
  item(
    "production-template",
    "database-backed-table-boundary",
    "Database ORM",
    "adapter-boundary",
    ["db/drizzle-sqlite", "supabase/client", "instantdb/react"],
  ),
  item("production-template", "docs-content-route", "Markdown & MDX Content", "source-owned", [
    "content/fumadocs-next",
    "content/react-markdown",
  ]),
  item("production-template", "ai-chat-route", "AI SDK", "adapter-boundary", ["ai/vercel-ai"]),
  item("package-ecosystem", "visual-studio-markers", "Motion & Animation", "source-owned", [
    "animation/motion",
    "dx/icon/search",
  ]),
  item("package-ecosystem", "state-management", "State Management", "source-owned", [
    "state/zustand",
  ]),
  item("package-ecosystem", "backend-platform-client", "Backend Platform Client", "adapter-boundary", [
    "supabase/client",
  ]),
  item("package-ecosystem", "ui-components-icons", "UI Components and Icons", "source-owned", [
    "shadcn/ui/button",
    "dx/icon/search",
  ]),
  item("package-ecosystem", "internationalization", "Internationalization", "adapter-boundary", [
    "i18n/next-intl",
  ]),
] satisfies readonly FrameworkCompletenessItem[];

export function dxWwwFrameworkCompletenessSummary() {
  const sourceOwnedCount = dxWwwFrameworkCompleteness.filter(
    (entry) => entry.status === "source-owned",
  ).length;

  return {
    ...frameworkCompletenessContract,
    itemCount: dxWwwFrameworkCompleteness.length,
    sourceOwnedCount,
    adapterBoundaryCount: dxWwwFrameworkCompleteness.filter(
      (entry) => entry.status === "adapter-boundary",
    ).length,
    partialCount: dxWwwFrameworkCompleteness.filter((entry) => entry.status === "partial").length,
  };
}

export function dxWwwFrameworkCompletenessScore() {
  const weighted = dxWwwFrameworkCompleteness.reduce((score, entry) => {
    if (entry.status === "source-owned") return score + 100;
    if (entry.status === "adapter-boundary") return score + 70;
    return score + 55;
  }, 0);

  return Math.round(weighted / dxWwwFrameworkCompleteness.length);
}
