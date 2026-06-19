import {
  launchPackageCatalog,
  requiredLaunchEnv,
  type LaunchPackageRole,
} from "./package-catalog";
import {
  dxWwwFrameworkCompletenessScore,
  dxWwwFrameworkCompletenessSummary,
} from "./framework-completeness";

export type WwwTemplateSurfaceKind =
  | "route-shell"
  | "dashboard-workflow"
  | "form-workflow"
  | "data-workflow"
  | "visual-workflow"
  | "studio-contract";

export type WwwTemplateSurface = {
  id: string;
  label: string;
  kind: WwwTemplateSurfaceKind;
  route: "/";
  slot: string;
  sourceFile: string;
  materializedFile: string;
  sectionSelector: `[data-dx-section="${string}"]`;
  componentSelector: `[data-dx-component="${string}"]`;
  packageIds: readonly string[];
  packageRoles: readonly LaunchPackageRole[];
  requiredEnv: readonly string[];
  providerEnv?: readonly string[];
  receiptPaths: readonly string[];
  dataDxMarkers: readonly string[];
  packageWorkerContract: {
    ownerLane: string;
    acceptedExports: readonly string[];
    mustProvide: readonly string[];
    forbidden: readonly string[];
    sourceGuard: string;
  };
  studio: {
    editableSurfaceId: string;
    hotReloadTarget: "route:/";
    operations: readonly [
      "insert_component",
      "move_reorder_section",
      "update_design_token",
      "update_text_content",
      "insert_icon_media",
    ];
  };
};

const defaultStudioOperations = [
  "insert_component",
  "move_reorder_section",
  "update_design_token",
  "update_text_content",
  "insert_icon_media",
] as const;

const noNodeModulesRule = "must not create or require template-local node_modules";

export const wwwTemplateSurfaces = [
  {
    id: "template-shell",
    label: "Launch dashboard shell",
    kind: "route-shell",
    route: "/",
    slot: "app-shell",
    sourceFile: "examples/template/template-shell.tsx",
    materializedFile: "components/template-app/template-shell.tsx",
    sectionSelector: '[data-dx-section="hero"]',
    componentSelector: '[data-dx-component="launch-hero"]',
    packageIds: ["3d/launch-scene", "dx/icon/search"],
    packageRoles: ["scene", "selected-asset"],
    requiredEnv: [],
    receiptPaths: [".dx/forge/template-readiness/launch-route.json"],
    dataDxMarkers: [
      "data-dx-route",
      "data-dx-source",
      "data-dx-forge",
      "data-dx-hot-reload-target",
      "data-dx-node-modules",
    ],
    packageWorkerContract: {
      ownerLane: "www-core",
      acceptedExports: ["TemplateShell"],
      mustProvide: [
        "React-familiar TSX entrypoint",
        "provider-owned shell composition",
        "stable data-dx route markers",
      ],
      forbidden: ["package proof cards without a shell slot", noNodeModulesRule],
      sourceGuard: "dx run --test .\\benchmarks\\template-shell.test.ts",
    },
    studio: {
      editableSurfaceId: "launch.root",
      hotReloadTarget: "route:/",
      operations: defaultStudioOperations,
    },
  },
  {
    id: "account-access",
    label: "Authentication",
    kind: "dashboard-workflow",
    route: "/",
    slot: "account-access-dashboard",
    sourceFile: "examples/template/auth-session-status.tsx",
    materializedFile: "components/template-app/auth-session-status.tsx",
    sectionSelector: '[data-dx-section="account-access-dashboard"]',
    componentSelector: '[data-dx-component="better-auth-account-dashboard-workflow"]',
    packageIds: ["auth/better-auth"],
    packageRoles: ["auth"],
    requiredEnv: ["BETTER_AUTH_SECRET", "BETTER_AUTH_URL"],
    providerEnv: ["GOOGLE_CLIENT_ID", "GOOGLE_CLIENT_SECRET"],
    receiptPaths: [
      ".dx/forge/receipts/auth-better-auth.json",
      ".dx/forge/receipts/*-auth-better-auth.json",
    ],
    dataDxMarkers: [
      "data-dx-package",
      "data-dx-component",
      "data-dx-auth-session-source",
      "data-dx-auth-network-state",
      "data-dx-auth-provider",
      "data-dx-auth-provider-state",
      "data-dx-auth-missing-provider",
    ],
    packageWorkerContract: {
      ownerLane: "auth-package",
      acceptedExports: ["LaunchAuthSessionStatus"],
      mustProvide: [
        "session state",
        "safe sign-in/sign-out boundary",
        "provider readiness",
        "env contract",
      ],
      forbidden: ["fake signed-in success", noNodeModulesRule],
      sourceGuard: "dx run --test .\\benchmarks\\better-auth-dashboard-workflow.test.ts",
    },
    studio: {
      editableSurfaceId: "launch.account-access",
      hotReloadTarget: "route:/",
      operations: defaultStudioOperations,
    },
  },
  {
    id: "settings-validation",
    label: "Settings form and validation",
    kind: "form-workflow",
    route: "/",
    slot: "settings-validation-dashboard",
    sourceFile: "examples/template/zod-dashboard-settings.tsx",
    materializedFile: "components/template-app/zod-dashboard-settings.tsx",
    sectionSelector: '[data-dx-section="forms-validation"]',
    componentSelector: '[data-dx-component="zod-dashboard-settings-form"]',
    packageIds: ["forms/react-hook-form", "validation/zod", "shadcn/ui/field"],
    packageRoles: ["forms", "validation", "ui-primitive"],
    requiredEnv: [],
    receiptPaths: [
      "examples/template/.dx/forge/receipts/2026-05-22-validation-zod-dashboard-settings.json",
      ".dx/forge/receipts/*-validation-zod.json",
    ],
    dataDxMarkers: [
      "data-dx-package",
      "data-dx-component",
      "data-dx-zod-settings-state",
      "data-dx-zod-settings-issues",
      "data-dx-zod-settings-output",
      "data-dx-zod-settings-field",
      "data-dx-zod-settings-action",
    ],
    packageWorkerContract: {
      ownerLane: "forms-validation-package",
      acceptedExports: ["LaunchLeadForm", "LaunchZodDashboardSettings"],
      mustProvide: [
        "typed submit payload",
        "visible validation errors",
        "success state",
        "createDxDashboardSettingsReceipt output",
      ],
      forbidden: ["unvalidated readiness submit", noNodeModulesRule],
      sourceGuard: "dx run --test .\\benchmarks\\zod-dashboard-settings-workflow.test.ts",
    },
    studio: {
      editableSurfaceId: "launch.forms-validation",
      hotReloadTarget: "route:/",
      operations: defaultStudioOperations,
    },
  },
  {
    id: "billing",
    label: "Billing and payment boundary",
    kind: "dashboard-workflow",
    route: "/",
    slot: "billing-checkout-dashboard",
    sourceFile: "examples/template/payments-status.tsx",
    materializedFile: "components/template-app/payments-status.tsx",
    sectionSelector: '[data-dx-section="billing-workflow"]',
    componentSelector: '[data-dx-component="launch-billing-checkout-workflow"]',
    packageIds: ["payments/stripe-js"],
    packageRoles: ["payments"],
    requiredEnv: ["NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY", "STRIPE_SECRET_KEY"],
    receiptPaths: [
      "examples/template/.dx/forge/receipts/2026-05-22-payments-stripe-js-billing-workflow.json",
    ],
    dataDxMarkers: [
      "data-dx-package",
      "data-dx-component",
      "data-dx-dashboard-flow",
      "data-dx-stripe-receipt-path",
    ],
    packageWorkerContract: {
      ownerLane: "payments-package",
      acceptedExports: ["LaunchPaymentStatus"],
      mustProvide: ["plan selection", "checkout/session boundary", "missing-config state"],
      forbidden: ["fake payment success", "card-field collection in template", noNodeModulesRule],
      sourceGuard: "dx run --test .\\benchmarks\\stripe-rhf-checkout-flow.test.ts",
    },
    studio: {
      editableSurfaceId: "launch.payments",
      hotReloadTarget: "route:/",
      operations: defaultStudioOperations,
    },
  },
  {
    id: "data-backend",
    label: "Database and backend",
    kind: "data-workflow",
    route: "/",
    slot: "account-data-dashboard",
    sourceFile: "examples/template/data-status.tsx",
    materializedFile: "components/template-app/data-status.tsx",
    sectionSelector: '[data-dx-section="account-data-dashboard"]',
    componentSelector: '[data-dx-component="launch-account-data-dashboard"]',
    packageIds: ["supabase/client", "db/drizzle-sqlite", "instantdb/react", "api/trpc"],
    packageRoles: ["backend-client", "database", "realtime-data", "api"],
    requiredEnv: ["NEXT_PUBLIC_SUPABASE_URL", "NEXT_PUBLIC_INSTANT_APP_ID"],
    receiptPaths: [
      "examples/template/.dx/forge/receipts/2026-05-22-supabase-client-dashboard-workflow.json",
      "examples/template/.dx/forge/receipts/2026-05-22-db-drizzle-sqlite-dashboard-workflow.json",
      "examples/template/.dx/forge/receipts/2026-05-22-instantdb-realtime-dashboard.json",
      "examples/template/.dx/forge/receipts/2026-05-22-api-trpc-dashboard-workflow.json",
    ],
    dataDxMarkers: [
      "data-dx-dashboard-workflow",
      "data-dx-supabase-receipt-path",
      "data-dx-drizzle-receipt-path",
      "data-dx-drizzle-query-plan-id",
      "data-dx-drizzle-runtime-dependencies",
      "data-dx-instant-runtime-status",
      "data-dx-trpc-workflow",
    ],
    packageWorkerContract: {
      ownerLane: "data-api-packages",
      acceptedExports: [
        "LaunchDataStatus",
        "LaunchSupabaseProfileWorkflow",
        "LaunchDrizzleDashboardData",
        "LaunchInstantStatus",
        "TrpcLaunchHealth",
      ],
      mustProvide: [
        "typed schema or route boundary",
        "safe fixture/readiness state",
        "receipt path",
        "runtime dependency boundary",
      ],
      forbidden: ["pretend hosted data writes", noNodeModulesRule],
      sourceGuard: "dx run --test .\\benchmarks\\launch-runtime-materializer.test.ts",
    },
    studio: {
      editableSurfaceId: "launch.account-data",
      hotReloadTarget: "route:/",
      operations: defaultStudioOperations,
    },
  },
  {
    id: "state-query",
    label: "State, query, token and money meters",
    kind: "dashboard-workflow",
    route: "/",
    slot: "mission-control",
    sourceFile: "examples/template/template-shell.tsx",
    materializedFile: "components/template-app/template-shell.tsx",
    sectionSelector: '[data-dx-section="launch-dashboard-controls"]',
    componentSelector: '[data-dx-component="launch-dashboard-state-shell"]',
    packageIds: ["state/zustand", "tanstack/query"],
    packageRoles: ["launch-state", "server-state"],
    requiredEnv: [],
    receiptPaths: [
      "examples/template/.dx/forge/receipts/2026-05-22-state-zustand-dashboard-workflow.json",
      "examples/template/.dx/forge/receipts/2026-05-22-tanstack-query-dashboard-data.json",
    ],
    dataDxMarkers: [
      "data-dx-zustand-store",
      "data-dx-zustand-action",
      "data-dx-query-dashboard-source",
      "data-dx-dashboard-metric",
    ],
    packageWorkerContract: {
      ownerLane: "state-query-packages",
      acceptedExports: [
        "LaunchDashboardStateShell",
        "LaunchDashboardStateControl",
        "LaunchQueryCacheStatus",
      ],
      mustProvide: ["visible state mutation", "query refresh action", "dashboard metric update"],
      forbidden: ["static counters without state", noNodeModulesRule],
      sourceGuard: "dx run --test .\\benchmarks\\zustand-launch-materialized.test.ts",
    },
    studio: {
      editableSurfaceId: "launch.dashboard",
      hotReloadTarget: "route:/",
      operations: defaultStudioOperations,
    },
  },
  {
    id: "content-docs",
    label: "Docs, markdown, and i18n",
    kind: "dashboard-workflow",
    route: "/",
    slot: "docs-help-changelog",
    sourceFile: "examples/template/docs-status.tsx",
    materializedFile: "components/template-app/docs-status.tsx",
    sectionSelector: '[data-dx-section="docs-content"]',
    componentSelector: '[data-dx-component="launch-fumadocs-docs-workflow"]',
    packageIds: ["content/fumadocs-next", "content/react-markdown", "i18n/next-intl"],
    packageRoles: ["docs", "content-rendering", "i18n"],
    requiredEnv: ["DX_FUMADOCS_OPENAPI_ALLOWED_ORIGINS"],
    receiptPaths: [
      "examples/template/.dx/forge/receipts/2026-05-22-content-fumadocs-dashboard-workflow.json",
      "examples/template/.dx/forge/receipts/2026-05-22-i18n-next-intl-dashboard-locale.json",
    ],
    dataDxMarkers: [
      "data-dx-fumadocs-action",
      "data-dx-docs-status",
      "data-launch-i18n-phase",
      "data-dx-package",
    ],
    packageWorkerContract: {
      ownerLane: "content-i18n-packages",
      acceptedExports: [
        "LaunchDocsStatus",
        "LaunchMarkdownPreview",
        "LaunchDashboardIntlWorkflow",
      ],
      mustProvide: ["rendered markdown/docs preview", "locale state", "safe docs receipt"],
      forbidden: ["static docs-only cards", noNodeModulesRule],
      sourceGuard: "dx run --test .\\benchmarks\\fumadocs-dashboard-workflow.test.ts",
    },
    studio: {
      editableSurfaceId: "launch.docs",
      hotReloadTarget: "route:/",
      operations: defaultStudioOperations,
    },
  },
  {
    id: "automation-ai-visuals",
    label: "Automation, AI, Motion, 3D, WASM, and icons",
    kind: "visual-workflow",
    route: "/",
    slot: "advanced-runtime-dashboard",
    sourceFile: "examples/template/template-shell.tsx",
    materializedFile: "components/template-app/template-shell.tsx",
    sectionSelector: '[data-dx-section="studio-proof-flow"]',
    componentSelector: '[data-dx-component="studio-web-preview-proof-flow"]',
    packageIds: [
      "automations/n8n",
      "ai/vercel-ai",
      "animation/motion",
      "3d/launch-scene",
      "wasm/bindgen",
      "dx/icon/search",
    ],
    packageRoles: ["automations", "ai", "animation", "scene", "wasm", "selected-asset"],
    requiredEnv: ["AI_PROVIDER_API_KEY", "SLACK_BOT_TOKEN"],
    receiptPaths: [
      "examples/template/.dx/forge/receipts/2026-05-22-ai-vercel-ai-launch-assistant.json",
      "examples/template/.dx/forge/receipts/2026-05-22-animation-motion-dashboard-workflow.json",
      "examples/template/.dx/forge/receipts/3d-launch-scene-dashboard-workflow.json",
      "examples/template/.dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json",
    ],
    dataDxMarkers: [
      "data-dx-automation-dashboard-state",
      "data-dx-ai-action-state",
      "data-dx-motion-interaction",
      "data-dx-scene-action",
      "data-dx-scene-capability-report",
      "data-dx-scene-capability-status",
      "data-dx-scene-viewport-report",
      "data-dx-scene-viewport-status",
      "data-dx-scene-bounds-report",
      "data-dx-scene-bounds-status",
      "data-dx-scene-raycast-report",
      "data-dx-scene-raycast-status",
      "data-dx-wasm-action",
      "data-dx-icon",
    ],
    packageWorkerContract: {
      ownerLane: "automation-ai-visual-runtime-packages",
      acceptedExports: [
        "LaunchAutomationBridgeStatus",
        "LaunchAiChatStatus",
        "LaunchMotionInteractionProof",
        "LaunchScene",
        "LaunchWasmInteropStatus",
        "IconLaunchStatus",
      ],
      mustProvide: ["visible action", "safe missing-config state", "Studio-selectable marker"],
      forbidden: ["decorative-only motion or canvas", "fake provider success", noNodeModulesRule],
      sourceGuard: "dx run --test .\\benchmarks\\dx-studio-preview-manifest.test.ts",
    },
    studio: {
      editableSurfaceId: "launch.studio-proof-flow",
      hotReloadTarget: "route:/",
      operations: defaultStudioOperations,
    },
  },
] as const satisfies readonly WwwTemplateSurface[];

export const launchPackageWorkerRegistry = wwwTemplateSurfaces.map((surface) => ({
  id: surface.id,
  label: surface.label,
  route: surface.route,
  slot: surface.slot,
  ownerLane: surface.packageWorkerContract.ownerLane,
  sourceFile: surface.sourceFile,
  materializedFile: surface.materializedFile,
  sectionSelector: surface.sectionSelector,
  componentSelector: surface.componentSelector,
  packageIds: surface.packageIds,
  requiredEnv: surface.requiredEnv,
  providerEnv: surface.providerEnv ?? [],
  sourceGuard: surface.packageWorkerContract.sourceGuard,
  hotReloadTarget: surface.studio.hotReloadTarget,
}));

export function wwwTemplateSlotSummary() {
  const packageIds = new Set<string>();
  const roles = new Set<LaunchPackageRole>();
  const requiredEnvVars = new Set<string>();
  const providerEnvVars = new Set<string>();
  const catalogIds = new Set(launchPackageCatalog.map((item) => item.packageId));
  const frameworkCompleteness = dxWwwFrameworkCompletenessSummary();

  for (const surface of wwwTemplateSurfaces) {
    for (const packageId of surface.packageIds) {
      packageIds.add(packageId);
    }
    for (const role of surface.packageRoles) {
      roles.add(role);
    }
    for (const env of surface.requiredEnv) {
      requiredEnvVars.add(env);
    }
    for (const env of surface.providerEnv ?? []) {
      providerEnvVars.add(env);
    }
  }

  const missingCatalogPackages = [...packageIds].filter(
    (packageId) => !catalogIds.has(packageId),
  );

  return {
    schema: "dx.www.template_surface_registry",
    route: "/",
    routeAliases: [],
    surfaceCount: wwwTemplateSurfaces.length,
    packageCount: packageIds.size,
    roleCount: roles.size,
    requiredEnvCount: requiredEnv().length,
    explicitSurfaceEnvCount: requiredEnvVars.size,
    providerEnvCount: providerEnvVars.size,
    missingCatalogPackages,
    noNodeModulesRequired: true,
    publicAuthoring: "tsx-app-router",
    styling: "dx-style-generated-css",
    frameworkCompleteness,
    frameworkCompletenessScore: dxWwwFrameworkCompletenessScore(),
    frameworkCompletenessSchema: frameworkCompleteness.schema,
  } as const;
}

export function findWwwTemplateSurface(id: string) {
  return wwwTemplateSurfaces.find((surface) => surface.id === id);
}
