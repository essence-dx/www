export type DxWwwFrameworkCompletenessLane =
  | "routing-parity"
  | "server-client-model"
  | "dev-experience"
  | "production-template"
  | "package-ecosystem";

export type DxWwwFrameworkCompletenessStatus =
  | "full"
  | "source-owned"
  | "adapter-boundary"
  | "partial"
  | "missing";

export type DxWwwFrameworkCompletenessItem = {
  id: string;
  lane: DxWwwFrameworkCompletenessLane;
  label: string;
  status: DxWwwFrameworkCompletenessStatus;
  score: number;
  evidenceFiles: readonly string[];
  checkSignals: readonly string[];
  packageIds: readonly string[];
  nextAction: string;
};

const statusScore = {
  full: 100,
  "source-owned": 85,
  "adapter-boundary": 70,
  partial: 45,
  missing: 0,
} as const satisfies Record<DxWwwFrameworkCompletenessStatus, number>;

function item(
  lane: DxWwwFrameworkCompletenessLane,
  id: string,
  label: string,
  status: DxWwwFrameworkCompletenessStatus,
  evidenceFiles: readonly string[],
  checkSignals: readonly string[],
  nextAction: string,
  packageIds: readonly string[] = [],
): DxWwwFrameworkCompletenessItem {
  return {
    id,
    lane,
    label,
    status,
    score: statusScore[status],
    evidenceFiles,
    checkSignals,
    packageIds,
    nextAction,
  };
}

export const dxWwwFrameworkCompleteness = [
  item(
    "routing-parity",
    "nested-layouts",
    "Nested layouts",
    "source-owned",
    ["app/layout.tsx", "dx-www/src/cli/app_router_execution.rs"],
    ["composition_chain", "layouts"],
    "Keep nested layout composition in the generic App Router renderer and add browser proof for nested child routes.",
  ),
  item(
    "routing-parity",
    "loading-error-not-found-boundaries",
    "Loading, error, and not-found boundaries",
    "source-owned",
    ["app/loading.tsx", "app/error.tsx", "app/not-found.tsx"],
    ["boundaries", "not_found_boundary", "error_boundary", "loading_boundary"],
    "Promote boundary screenshots into dx-check receipts after runtime QA.",
  ),
  item(
    "routing-parity",
    "route-groups",
    "Route groups",
    "source-owned",
    ["dx-www/src/cli/app_router_execution.rs"],
    ["route_groups", "public_authoring:tsx"],
    "Add a generated route-group fixture to the launch starter once the public dashboard routes settle.",
  ),
  item(
    "routing-parity",
    "dynamic-params",
    "Dynamic params",
    "source-owned",
    ["dx-www/src/cli/app_router_execution.rs"],
    ["route_params", "dynamic_segments", "data-dx-route-params"],
    "Add one public docs or dashboard detail route that proves params without bridge-only rendering.",
  ),
  item(
    "routing-parity",
    "metadata-seo",
    "Metadata and SEO",
    "source-owned",
    ["app/layout.tsx", "app/page.tsx", "dx-www/src/cli/app_router_execution.rs"],
    ["effective_metadata", "metadata_sources"],
    "Generate social/OG metadata receipts for static export and Vercel deploy.",
  ),
  item(
    "routing-parity",
    "route-handlers",
    "Route handlers",
    "source-owned",
    ["app/api/health/route.ts", "core/src/delivery/server_contract.rs"],
    ["source-owned-route-handler-boundary", "request_serialization"],
    "Keep handler execution source-owned and add route-handler fixtures for auth, AI, and checkout.",
  ),
  item(
    "server-client-model",
    "server-actions-equivalent",
    "Server actions equivalent",
    "source-owned",
    ["server/actions.ts", "core/src/delivery/server_contract.rs"],
    ["DxReactServerActionProtocol", "server/actions.ts#recordWelcomeView"],
    "Expose the action protocol through one obvious docs page and one generated starter receipt.",
  ),
  item(
    "server-client-model",
    "form-actions",
    "Form actions",
    "source-owned",
    ["components/forms/SettingsForm.tsx", "examples/template/zod-dashboard-settings.tsx"],
    ["safeParseDxDashboardSettingsForm", "data-dx-zod-settings-action"],
    "Bind form actions to the production dashboard settings route instead of only the launch route.",
    ["validation/zod", "forms/react-hook-form"],
  ),
  item(
    "server-client-model",
    "cookies-headers-session-helpers",
    "Cookies, headers, and session helpers",
    "adapter-boundary",
    ["core/src/delivery/server_contract.rs", "dx-www/src/cli/mod.rs"],
    ["next/headers", "next/cookies", "auth/better-auth"],
    "Materialize safe helper files in new projects so apps do not import framework internals.",
    ["auth/better-auth"],
  ),
  item(
    "server-client-model",
    "streaming-response-boundary",
    "Streaming response boundary",
    "partial",
    ["dx-www/src/cli/app_router_execution.rs", "dx-www/src/cli/mod.rs"],
    ["loading_boundary", "response frame"],
    "Add one AI or docs streaming fixture before calling this production parity.",
    ["ai/vercel-ai"],
  ),
  item(
    "server-client-model",
    "cache-revalidate-story",
    "Cache and revalidate story",
    "partial",
    ["dx-www/src/cli/mod.rs", "examples/template/query-cache-status.tsx"],
    ["cache-control", "query-cache-refresh", "revalidate"],
    "Turn cache headers and query invalidation into a documented public `dx check` receipt.",
    ["tanstack/query"],
  ),
  item(
    "dev-experience",
    "reliable-hot-reload",
    "Reliable hot reload",
    "source-owned",
    ["dx-www/src/dev/watcher.rs", "dx-www/src/dev/axum_server.rs"],
    ["data-dx-hot-reload-target", "_dx/hot-reload/version"],
    "Keep hot reload focused on generated CSS, TSX route changes, and Studio edit markers.",
  ),
  item(
    "dev-experience",
    "tsx-first-templates",
    "TSX-first templates",
    "source-owned",
    ["app/page.tsx", "app/page.tsx", "dx-www/src/cli/mod.rs"],
    ["publicAuthoring:tsx-app-router", "sourceSyntax:tsx-app-router"],
    "Keep App Router TSX as the only public starter authoring path.",
  ),
  item(
    "dev-experience",
    "auto-imports",
    "Auto imports",
    "source-owned",
    ["dx-www/src/cli/public_framework_tools.rs", ".dx/imports/import-map.json"],
    ["dx imports sync", "components/auto-imports.ts"],
    "Wire auto imports into `dx add` so package installs update the import map by default.",
  ),
  item(
    "dev-experience",
    "dx-style-css-generation",
    "dx-style CSS generation",
    "source-owned",
    ["styles/globals.css", "styles/globals.css", "related-crates/style/src/core/engine/mod.rs"],
    ["dx style build", "dx style watch", "generated-css"],
    "Keep CSS output as the public default and leave binary style snapshots internal/experimental.",
  ),
  item(
    "dev-experience",
    "dx-check-receipts",
    "dx-check receipts",
    "source-owned",
    ["core/src/ecosystem/project_check.rs", ".dx/receipts/check/web-perf/report.json"],
    ["dx check", "web-perf", "project-contract"],
    "Add this completeness matrix to project-contract metrics so launch QA can score it.",
  ),
  item(
    "dev-experience",
    "obvious-cli-path",
    "One obvious CLI path",
    "source-owned",
    ["dx-www/src/cli/mod.rs"],
    ["dx new", "dx dev", "dx add", "dx check", "dx deploy"],
    "Hide internal evidence commands behind advanced docs and keep the public path short.",
  ),
  item(
    "production-template",
    "real-dashboard-starter",
    "Real dashboard starter",
    "source-owned",
    ["components/dashboard/LaunchDashboard.tsx", "examples/template/template-shell.tsx"],
    ["data-dx-dashboard-workflow", "data-dx-component"],
    "Keep replacing proof cards with workflows that mutate visible state and receipts.",
  ),
  item(
    "production-template",
    "auth-page",
    "Auth page",
    "adapter-boundary",
    ["app/auth/page.tsx", "examples/template/auth-session-status.tsx"],
    ["data-dx-auth-session-source", "missing-provider safe"],
    "Add real Better Auth route handlers once credentials and storage are configured.",
    ["auth/better-auth"],
  ),
  item(
    "production-template",
    "settings-validation-form",
    "Settings form with validation",
    "source-owned",
    ["app/settings/page.tsx", "components/forms/SettingsForm.tsx", "examples/template/zod-dashboard-settings.tsx"],
    ["safeParseDxDashboardSettingsForm", "data-dx-zod-settings-output"],
    "Keep settings as the canonical form-validation route for package QA.",
    ["validation/zod", "forms/react-hook-form"],
  ),
  item(
    "production-template",
    "payment-plan-page",
    "Payment plan page",
    "adapter-boundary",
    ["app/billing/page.tsx", "examples/template/payments-status.tsx"],
    ["api/checkout", "data-dx-stripe-receipt-path"],
    "Keep payment honest as a safe Stripe-shaped boundary until secrets exist.",
    ["payments/stripe-js"],
  ),
  item(
    "production-template",
    "database-backed-table-boundary",
    "Database-backed table boundary",
    "adapter-boundary",
    ["examples/template/data-status.tsx", "examples/template/drizzle-query-proof.tsx"],
    ["data-dx-drizzle-query-plan-id", "data-dx-supabase-receipt-path"],
    "Add a tiny SQLite-backed local proof before claiming full database runtime.",
    ["db/drizzle-sqlite", "supabase/client", "instantdb/react"],
  ),
  item(
    "production-template",
    "docs-content-route",
    "Docs/content route",
    "adapter-boundary",
    ["examples/template/docs-status.tsx", "examples/template/react-markdown-preview.tsx"],
    ["data-dx-fumadocs-action", "content/react-markdown"],
    "Add a generated `/docs` route with markdown source and Fumadocs navigation proof.",
    ["content/fumadocs-next", "content/react-markdown"],
  ),
  item(
    "production-template",
    "ai-chat-route",
    "adapter-boundary",
    ["examples/template/ai-chat-status.tsx"],
    ["api/ai/chat", "missing-provider"],
    "Keep missing-key state honest and add provider-approved streaming when credentials exist.",
    ["ai/vercel-ai"],
  ),
  item(
    "production-template",
    "visual-studio-markers",
    "Visual Studio markers",
    "source-owned",
    ["examples/template/dx-studio-edit-contract.ts", "examples/template/template-surface-registry.ts"],
    ["data-dx-edit-id", "data-dx-template-slot", "data-dx-edit-ops"],
    "Connect Zed Web Preview edit operations to these selectors in the editor integration lane.",
  ),
  item(
    "package-ecosystem",
    "authentication",
    "Authentication",
    "adapter-boundary",
    ["examples/template/auth-session-status.tsx", "docs/packages/authentication.md"],
    ["auth/better-auth", "session", "google-oauth-provider"],
    "Promote route handlers and credential storage after keychain/provider UX is ready.",
    ["auth/better-auth"],
  ),
  item(
    "package-ecosystem",
    "validation-schemas",
    "Validation & Schemas",
    "source-owned",
    ["examples/template/zod-dashboard-settings.tsx", "docs/packages/validation-zod.md"],
    ["safeParseDxDashboardSettingsForm", "receipt"],
    "Keep validation schemas as first-class package docs and generated starter imports.",
    ["validation/zod"],
  ),
  item(
    "package-ecosystem",
    "forms",
    "Forms",
    "source-owned",
    [
      "examples/template/template-lead-form.tsx",
      "docs/packages/forms-react-hook-form.md",
      "examples/template/.dx/forge/receipts/2026-05-22-forms-dashboard-workflow.json",
    ],
    ["forms/react-hook-form", "data-dx-component=\"template-lead-form\""],
    "Wire the Forms receipt into generated-project dx-check freshness if the next pass needs materialized receipt status.",
    ["forms/react-hook-form"],
  ),
  item(
    "package-ecosystem",
    "state-management",
    "State Management",
    "source-owned",
    ["examples/template/state-zustand-dashboard.tsx", "docs/packages/state-zustand.md"],
    ["data-dx-zustand-store", "state/zustand"],
    "Keep state mutations visible in the dashboard and Studio manifest.",
    ["state/zustand"],
  ),
  item(
    "package-ecosystem",
    "data-fetching-cache",
    "Data Fetching & Cache",
    "source-owned",
    ["examples/template/query-cache-status.tsx", "docs/packages/tanstack-query.md"],
    ["data-dx-query-cache-state", "tanstack/query"],
    "Keep query refresh, cache state, and invalidation visible in the generated starter.",
    ["tanstack/query"],
  ),
  item(
    "package-ecosystem",
    "database-orm",
    "Database ORM",
    "adapter-boundary",
    ["examples/template/drizzle-query-proof.tsx", "docs/packages/db-drizzle-sqlite.md"],
    ["db/drizzle-sqlite", "query plan"],
    "Add a tiny local SQLite execution proof when package runtime dependencies are available.",
    ["db/drizzle-sqlite"],
  ),
  item(
    "package-ecosystem",
    "backend-platform-client",
    "Backend Platform Client",
    "adapter-boundary",
    ["examples/template/supabase-profile-workflow.tsx", "docs/packages/supabase-client.md"],
    ["supabase/client", "missing config"],
    "Keep hosted credentials explicit and never fake remote writes.",
    ["supabase/client"],
  ),
  item(
    "package-ecosystem",
    "payments",
    "Payments",
    "adapter-boundary",
    ["examples/template/payments-status.tsx", "docs/packages/payments-stripe-js.md"],
    ["payments/stripe-js", "checkout boundary"],
    "Add real Checkout session creation only behind explicit secret configuration.",
    ["payments/stripe-js"],
  ),
  item(
    "package-ecosystem",
    "internationalization",
    "Internationalization",
    "source-owned",
    ["examples/template/next-intl-dashboard-locale.tsx", "docs/packages/next-intl.md"],
    ["data-launch-i18n-phase", "i18n/next-intl"],
    "Review launch copy quality and extend formatter/cache helpers.",
    ["i18n/next-intl"],
  ),
  item(
    "package-ecosystem",
    "markdown-mdx",
    "Markdown & MDX Content",
    "adapter-boundary",
    ["examples/template/react-markdown-preview.tsx", "examples/template/docs-status.tsx"],
    ["content/react-markdown", "content/fumadocs-next"],
    "Add a separate MDX receipt and route-level docs page.",
    ["content/react-markdown", "content/fumadocs-next"],
  ),
  item(
    "package-ecosystem",
    "motion-animation",
    "Motion & Animation",
    "source-owned",
    ["examples/template/motion-interaction-proof.tsx", "docs/packages/animation-motion.md"],
    ["data-dx-motion-interaction", "animation/motion"],
    "Keep reduced-motion and animation budget checks in dx-check.",
    ["animation/motion"],
  ),
  item(
    "package-ecosystem",
    "ui-components-icons",
    "UI Components and Icons",
    "source-owned",
    ["examples/template/shadcn-dashboard-controls.tsx", "examples/template/icon-status.tsx"],
    ["data-dx-icon", "shadcn/ui", "dx/icon/search"],
    "Make icon markers reliable in runtime DOM and add SVG icon CLI receipts.",
    ["shadcn/ui/button", "dx/icon/search"],
  ),
] as const satisfies readonly DxWwwFrameworkCompletenessItem[];

export function dxWwwFrameworkCompletenessSummary() {
  const lanes = dxWwwFrameworkCompleteness.reduce<
    Record<
      DxWwwFrameworkCompletenessLane,
      {
        itemCount: number;
        averageScore: number;
        fullOrSourceOwned: number;
        adapterBoundaries: number;
        partialOrMissing: number;
      }
    >
  >(
    (summary, current) => {
      const lane = summary[current.lane];
      lane.itemCount += 1;
      lane.averageScore += current.score;
      if (current.status === "full" || current.status === "source-owned") {
        lane.fullOrSourceOwned += 1;
      }
      if (current.status === "adapter-boundary") {
        lane.adapterBoundaries += 1;
      }
      if (current.status === "partial" || current.status === "missing") {
        lane.partialOrMissing += 1;
      }
      return summary;
    },
    {
      "routing-parity": emptyLaneSummary(),
      "server-client-model": emptyLaneSummary(),
      "dev-experience": emptyLaneSummary(),
      "production-template": emptyLaneSummary(),
      "package-ecosystem": emptyLaneSummary(),
    },
  );

  for (const lane of Object.values(lanes)) {
    lane.averageScore =
      lane.itemCount === 0 ? 0 : Math.round(lane.averageScore / lane.itemCount);
  }

  return {
    schema: "dx.www.framework_completeness",
    publicAuthoring: "tsx-app-router",
    packagePolicy: "forge-source-owned-visible-files",
    stylePolicy: "dx-style-generated-css",
    checkPolicy: "dx-check-receipts",
    cliPath: ["dx new", "dx dev", "dx add", "dx check", "dx deploy"],
    itemCount: dxWwwFrameworkCompleteness.length,
    averageScore: dxWwwFrameworkCompletenessScore(),
    sourceOwnedCount: dxWwwFrameworkCompleteness.filter(
      (entry) => entry.status === "full" || entry.status === "source-owned",
    ).length,
    adapterBoundaryCount: dxWwwFrameworkCompleteness.filter(
      (entry) => entry.status === "adapter-boundary",
    ).length,
    partialOrMissingCount: dxWwwFrameworkCompleteness.filter(
      (entry) => entry.status === "partial" || entry.status === "missing",
    ).length,
    lanes,
  } as const;
}

export function dxWwwFrameworkCompletenessScore() {
  if (dxWwwFrameworkCompleteness.length === 0) {
    return 0;
  }

  const total = dxWwwFrameworkCompleteness.reduce(
    (sum, current) => sum + current.score,
    0,
  );
  return Math.round(total / dxWwwFrameworkCompleteness.length);
}

function emptyLaneSummary() {
  return {
    itemCount: 0,
    averageScore: 0,
    fullOrSourceOwned: 0,
    adapterBoundaries: 0,
    partialOrMissing: 0,
  };
}
