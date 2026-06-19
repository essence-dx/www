export type FrameworkCompletenessStatus = "source-owned" | "adapter-boundary";

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
  legacyPositioningAliases: ["tsx-first", "forge-first-no-node-modules"],
  stylePolicy: "dx-style-generated-css",
  checkPolicy: "dx-check-receipts",
} as const;

export const dxChartsFrameworkCompleteness = [
  item("routing-parity", "nested-layouts", "Gallery, docs, theme, examples, ecosystem, and playground layouts", "source-owned"),
  item("routing-parity", "loading-error-not-found-boundaries", "Route boundary contract for chart docs", "source-owned"),
  item("routing-parity", "route-groups", "Chart family grouping", "source-owned"),
  item("routing-parity", "dynamic-params", "Chart and task slug vocabulary", "source-owned"),
  item("routing-parity", "metadata-seo", "Visualization metadata surface", "source-owned"),
  item("routing-parity", "route-handlers", "Server chart manifest boundary", "source-owned"),
  item("server-client-model", "server-actions-equivalent", "Spec compilation is server-safe source", "adapter-boundary"),
  item("server-client-model", "form-actions", "Playground spec editing form shell", "source-owned", ["charts/playground"]),
  item("server-client-model", "cookies-headers-session-helpers", "No session runtime required for local chart previews", "adapter-boundary"),
  item("server-client-model", "streaming-response-boundary", "Manifest stream frame boundary", "source-owned"),
  item("server-client-model", "cache-revalidate-story", "Catalog is deterministic local source", "source-owned"),
  item("dev-experience", "reliable-hot-reload", "DX dev route coverage", "source-owned"),
  item("dev-experience", "tsx-first-templates", "TSX chart surfaces", "source-owned"),
  item("dev-experience", "auto-imports", "DX imports receipts", "source-owned"),
  item("dev-experience", "dx-style-css-generation", "DX Style generated CSS", "source-owned"),
  item("dev-experience", "dx-check-receipts", "DX check proof receipts", "source-owned"),
  item("dev-experience", "obvious-cli-path", "dx new charts plus dx check examples/charts", "source-owned"),
  item("production-template", "real-dashboard-starter", "Metric, gallery, and documentation starter", "source-owned"),
  item("production-template", "auth-page", "Auth-free visualization product lane", "adapter-boundary"),
  item("production-template", "settings-validation-form", "Theme token validation surface", "source-owned"),
  item("production-template", "payment-plan-page", "Package commercialization boundary", "adapter-boundary"),
  item("production-template", "database-backed-table-boundary", "S2-style pivot table source lane", "adapter-boundary", ["charts/s2"]),
  item("production-template", "docs-content-route", "Chart grammar docs", "source-owned", ["charts/docs"]),
  item("production-template", "ai-chat-route", "AVA and GPT-Vis recommendation boundary", "adapter-boundary", ["charts/ava", "charts/gpt-vis"]),
  item("package-ecosystem", "visual-studio-markers", "Chart selection and tooltip markers", "source-owned", ["dx/icon/search"]),
  item("package-ecosystem", "state-management", "Local runtime state without npm stores", "source-owned"),
  item("package-ecosystem", "backend-platform-client", "Map and graph data adapter boundary", "adapter-boundary"),
  item("package-ecosystem", "ui-components-icons", "DX Icon-only navigation", "source-owned", ["dx/icon/search"]),
  item("package-ecosystem", "internationalization", "Chart label formatting boundary", "adapter-boundary"),
] satisfies readonly FrameworkCompletenessItem[];

export function dxChartsFrameworkCompletenessSummary() {
  return {
    ...frameworkCompletenessContract,
    itemCount: dxChartsFrameworkCompleteness.length,
    sourceOwnedCount: dxChartsFrameworkCompleteness.filter((entry) => entry.status === "source-owned").length,
    adapterBoundaryCount: dxChartsFrameworkCompleteness.filter((entry) => entry.status === "adapter-boundary").length,
    partialCount: 0,
  };
}
