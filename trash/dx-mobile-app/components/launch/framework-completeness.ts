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
  nativeTarget: "tauri-webview",
  nativeBridge: "dx-native-mobile-companion",
  stylePolicy: "dx-style-generated-css",
  checkPolicy: "dx-check-receipts",
} as const;

export const dxMobileAppFrameworkCompleteness = [
  item("routing-parity", "nested-layouts", "Single route mounted in the App Router shell", "source-owned"),
  item("routing-parity", "loading-error-not-found-boundaries", "Route boundaries are still a launch follow-up", "partial"),
  item("routing-parity", "route-groups", "Mobile companion can move behind a route group without changing the surface", "partial"),
  item("routing-parity", "dynamic-params", "Session identity is delegated to the DX Agents gateway", "adapter-boundary"),
  item("routing-parity", "metadata-seo", "DX Mobile App metadata", "source-owned"),
  item("routing-parity", "route-handlers", "Gateway pairing and chat use DX Agents runtime endpoints", "adapter-boundary", [
    "auth/better-auth",
  ]),
  item("server-client-model", "server-actions-equivalent", "Pairing mutations are delegated to the dx-agents gateway", "adapter-boundary", [
    "auth/better-auth",
  ]),
  item("server-client-model", "form-actions", "QR pairing form is source-owned and native-ready", "source-owned", [
    "auth/better-auth",
  ]),
  item("server-client-model", "cookies-headers-session-helpers", "Session cookies and adapter policy are runtime-owned", "adapter-boundary", [
    "auth/better-auth",
  ]),
  item("server-client-model", "streaming-response-boundary", "Live chat streams through dx-agents WebSocket or ACP", "adapter-boundary"),
  item("server-client-model", "cache-revalidate-story", "Companion screen is static source with session refresh delegated", "adapter-boundary"),
  item("server-client-model", "native-bridge-contract", "Native handoff uses dx-native-mobile-companion", "source-owned"),
  item("dev-experience", "reliable-hot-reload", "DX dev route can hot-reload source modules", "source-owned"),
  item("dev-experience", "tsx-first-templates", "Companion surface is TSX-first", "source-owned"),
  item("dev-experience", "auto-imports", "DX imports receipts", "source-owned"),
  item("dev-experience", "dx-style-css-generation", "DX Style generated CSS and authored modules", "source-owned"),
  item("dev-experience", "dx-check-receipts", "DX check, style, icon, import, and Forge receipts", "source-owned"),
  item("dev-experience", "obvious-cli-path", "Created with dx-www new, native-shell, and Forge authentication provenance", "source-owned"),
  item("production-template", "real-dashboard-starter", "Zed chat companion replaces the placeholder dashboard lane", "source-owned"),
  item("production-template", "auth-page", "Responsive QR pairing screen", "source-owned", [
    "auth/better-auth",
  ]),
  item("production-template", "settings-validation-form", "Validation belongs to post-auth settings", "adapter-boundary"),
  item("production-template", "payment-plan-page", "Payments are outside the first mobile auth lane", "adapter-boundary"),
  item("production-template", "database-backed-table-boundary", "Gateway sessions and device registry are deployment-owned", "adapter-boundary", [
    "auth/better-auth",
  ]),
  item("production-template", "docs-content-route", "Docs route is not part of the mobile auth screen", "partial"),
  item("production-template", "ai-chat-route", "zed-chat-companion consumes dx-agents gateway chat contracts", "source-owned"),
  item("package-ecosystem", "visual-studio-markers", "Style, Check, Icon, Forge, and native markers are visible in source", "source-owned"),
  item("package-ecosystem", "style-check-icon-forge", "Style, Check, Icon, Forge, and dx-agents gateway markers surfaced in source", "source-owned", [
    "auth/better-auth",
    "dx/icon/search",
  ]),
] satisfies readonly FrameworkCompletenessItem[];

export function dxMobileAppFrameworkCompletenessSummary() {
  return {
    ...frameworkCompletenessContract,
    itemCount: dxMobileAppFrameworkCompleteness.length,
    sourceOwnedCount: dxMobileAppFrameworkCompleteness.filter(
      (entry) => entry.status === "source-owned",
    ).length,
    adapterBoundaryCount: dxMobileAppFrameworkCompleteness.filter(
      (entry) => entry.status === "adapter-boundary",
    ).length,
    partialCount: dxMobileAppFrameworkCompleteness.filter((entry) => entry.status === "partial")
      .length,
  };
}

export function dxMobileAppFrameworkCompletenessScore() {
  const weighted = dxMobileAppFrameworkCompleteness.reduce((score, entry) => {
    if (entry.status === "source-owned") return score + 100;
    if (entry.status === "adapter-boundary") return score + 70;
    return score + 55;
  }, 0);

  return Math.round(weighted / dxMobileAppFrameworkCompleteness.length);
}
