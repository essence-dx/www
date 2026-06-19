const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

const cli = read("dx-www/src/cli/mod.rs");
const nextFamiliarTemplate = read("dx-www/src/cli/mod_parts/next_familiar_template.rs");
const ecosystemMod = read("core/src/ecosystem/mod.rs");
const registry = [
  "core/src/ecosystem/forge_registry.rs",
  "core/src/ecosystem/forge_registry_parts/registry_operations.rs",
  "core/src/ecosystem/forge_registry_parts/package_lanes.rs",
  "core/src/ecosystem/forge_registry_parts/package_templates.rs",
].map(read).join("\n");
const scorecard = read("core/src/ecosystem/forge_scorecard.rs");
const trustPolicy = read("core/src/ecosystem/forge_trust_policy.rs");
const security = read("core/src/ecosystem/forge_security.rs");
const packageCatalog = read("examples/template/package-catalog.ts");

const launchPackages = [
  "shadcn/ui/button",
  "shadcn/ui/badge",
  "shadcn/ui/card",
  "shadcn/ui/alert",
  "shadcn/ui/avatar",
  "shadcn/ui/skeleton",
  "shadcn/ui/label",
  "shadcn/ui/separator",
  "shadcn/ui/field",
  "shadcn/ui/item",
  "shadcn/ui/input",
  "shadcn/ui/textarea",
  "dx/icon/search",
  "auth/better-auth",
  "animation/motion",
  "i18n/next-intl",
  "tanstack/query",
  "validation/zod",
  "forms/react-hook-form",
  "payments/stripe-js",
  "automations/n8n",
  "state/zustand",
  "ai/vercel-ai",
  "api/trpc",
  "content/fumadocs-next",
  "content/react-markdown",
  "supabase/client",
  "db/drizzle-sqlite",
  "instantdb/react",
  "wasm/bindgen",
  "3d/launch-scene",
];

const sourceSlices = [
  {
    module: "forge_registry",
    file: "core/src/ecosystem/forge_registry.rs",
    packageId: "shadcn/ui/button",
    markers: [
      "shadcn-ui://apps/v4/registry/bases/radix/ui/button.tsx",
      "buttonVariants",
      'data-slot="button"',
      "data-variant={variant}",
      "data-size={size}",
      'asChild ? Slot.Root : "button"',
      "export { Button, buttonVariants }",
    ],
  },
  {
    module: "forge_registry",
    file: "core/src/ecosystem/forge_registry.rs",
    packageId: "shadcn/ui/badge",
    markers: [
      "shadcn-ui://apps/v4/registry/bases/radix/ui/badge.tsx",
      "badgeVariants",
      'data-slot="badge"',
      "data-variant={variant}",
      'asChild ? Slot.Root : "span"',
      "export { Badge, badgeVariants }",
    ],
  },
  {
    module: "forge_registry",
    file: "core/src/ecosystem/forge_registry.rs",
    packageId: "shadcn/ui/card",
    markers: [
      "shadcn-ui://apps/v4/registry/bases/radix/ui/card.tsx",
      'data-slot="card"',
      "data-size={size}",
      'data-slot="card-header"',
      'data-slot="card-action"',
      "function CardAction",
      "CardAction,",
    ],
  },
  {
    module: "forge_registry",
    file: "core/src/ecosystem/forge_registry_parts/package_templates.rs",
    packageId: "shadcn/ui/alert",
    markers: [
      "shadcn/ui alert",
      "function Alert",
      'data-slot="alert"',
      "data-variant={variant}",
      "role=\"alert\"",
      "function AlertTitle",
      "function AlertDescription",
      "export { Alert, AlertTitle, AlertDescription }",
    ],
  },
  {
    module: "forge_registry",
    file: "core/src/ecosystem/forge_registry_parts/package_templates.rs",
    packageId: "shadcn/ui/avatar",
    markers: [
      "shadcn/ui avatar",
      "function Avatar",
      "function AvatarImage",
      "function AvatarFallback",
      'data-slot="avatar"',
      'data-slot="avatar-image"',
      'data-slot="avatar-fallback"',
      "export { Avatar, AvatarImage, AvatarFallback }",
    ],
  },
  {
    module: "forge_registry",
    file: "core/src/ecosystem/forge_registry_parts/package_templates.rs",
    packageId: "shadcn/ui/skeleton",
    markers: [
      "shadcn/ui skeleton",
      "function Skeleton",
      'data-slot="skeleton"',
      "cn-skeleton animate-pulse",
      "export { Skeleton }",
    ],
  },
  {
    module: "forge_registry",
    file: "core/src/ecosystem/forge_registry.rs",
    packageId: "shadcn/ui/label",
    markers: [
      "shadcn-ui://apps/v4/registry/bases/radix/ui/label.tsx",
      "function Label",
      'data-slot="label"',
      "cn-label flex items-center select-none",
      'React.ComponentProps<"label">',
      "export { Label }",
    ],
  },
  {
    module: "forge_registry",
    file: "core/src/ecosystem/forge_registry.rs",
    packageId: "shadcn/ui/separator",
    markers: [
      "shadcn-ui://apps/v4/registry/bases/radix/ui/separator.tsx",
      "function Separator",
      'data-slot="separator"',
      'orientation = "horizontal"',
      "decorative = true",
      "data-orientation={orientation}",
      "export { Separator }",
    ],
  },
  {
    module: "forge_registry",
    file: "core/src/ecosystem/forge_registry.rs",
    packageId: "shadcn/ui/field",
    markers: [
      "shadcn-ui://apps/v4/registry/bases/radix/ui/field.tsx",
      "function FieldSet",
      "function FieldLegend",
      "function FieldGroup",
      "fieldVariants",
      'data-slot="field"',
      "data-orientation={orientation}",
      "function FieldLabel",
      "function FieldDescription",
      "function FieldSeparator",
      "function FieldError",
      "uniqueErrors",
      "export {",
      "FieldError,",
    ],
  },
  {
    module: "forge_registry",
    file: "core/src/ecosystem/forge_registry.rs",
    packageId: "shadcn/ui/item",
    markers: [
      "shadcn-ui://apps/v4/registry/bases/radix/ui/item.tsx",
      "function ItemGroup",
      "function ItemSeparator",
      "itemVariants",
      'data-slot="item"',
      "data-variant={variant}",
      "data-size={size}",
      'asChild ? Slot.Root : "div"',
      "function ItemMedia",
      "function ItemContent",
      "function ItemActions",
      "function ItemHeader",
      "function ItemFooter",
      "export {",
      "ItemFooter,",
    ],
  },
  {
    module: "forge_motion",
    file: "core/src/ecosystem/forge_motion.rs",
    packageId: "animation/motion",
    markers: [
      "motion/react",
      "MotionConfig",
      "useAnimationControls",
      "animationControls",
      "MotionControlledStatus",
      "LazyMotion",
      "domAnimation",
      "domMax",
      "domMin",
      "MotionLazyBox",
      "AnimatePresence",
      "LayoutGroup",
      "useInstantLayoutTransition",
      "layoutId",
      "useMotionValue",
      "useTransform",
      "useMotionTemplate",
      "useMotionValueEvent",
      "useVelocity",
      "MotionValueMeter",
      "Reorder",
      "useDragControls",
      "MotionReveal",
      "useReducedMotion",
      "useAnimate",
      "useScroll",
      "useSpring",
    ],
  },
  {
    module: "forge_next_intl",
    file: "core/src/ecosystem/forge_next_intl.rs",
    packageId: "i18n/next-intl",
    markers: ["NextIntlClientProvider", "createMiddleware", "defineRouting"],
  },
  {
    module: "forge_tanstack_query",
    file: "core/src/ecosystem/forge_tanstack_query.rs",
    packageId: "tanstack/query",
    markers: ["QueryClient", "HydrationBoundary", "queryOptions"],
  },
  {
    module: "forge_zod",
    file: "core/src/ecosystem/forge_zod.rs",
    packageId: "validation/zod",
    markers: ["safeParse", "toJSONSchema", "flattenError"],
  },
  {
    module: "forge_react_hook_form",
    file: "core/src/ecosystem/forge_react_hook_form.rs",
    packageId: "forms/react-hook-form",
    markers: ["useForm", "FormProvider", "useFieldArray"],
  },
  {
    module: "forge_stripe_js",
    file: "core/src/ecosystem/forge_stripe_js.rs",
    packageId: "payments/stripe-js",
    markers: [
      "loadStripe",
      "confirmPayment",
      "NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY",
      "assertNoPublicStripeSecrets",
      "readDxStripeServerConfig",
      "createDxStripeCheckoutSession",
      "submitDxStripeCheckoutContact",
      "server-only",
    ],
  },
  {
    module: "forge_zustand",
    file: "core/src/ecosystem/forge_zustand.rs",
    packageId: "state/zustand",
    markers: ["createStore", "subscribeWithSelector", "persist"],
  },
  {
    module: "forge_vercel_ai",
    file: "core/src/ecosystem/forge_vercel_ai.rs",
    packageId: "ai/vercel-ai",
    markers: [
      "streamText",
      "DefaultChatTransport",
      "convertToModelMessages",
      "generateText",
      "Output.object",
      "embedMany",
      "cosineSimilarity",
      "createProviderRegistry",
      "customProvider",
      "gateway",
    ],
  },
  {
    module: "forge_trpc",
    file: "core/src/ecosystem/forge_trpc.rs",
    packageId: "api/trpc",
    markers: [
      "initTRPC",
      "fetchRequestHandler",
      "httpBatchLink",
      "inferRouterInputs",
      "mutationOptions",
      "formatDxTrpcError",
      "getHTTPStatusCodeFromError",
      "createDxTrpcServerCaller",
      "readDxTrpcLaunchReadiness",
      "httpSubscriptionLink",
      "subscriptionOptions",
      "createDxTrpcSubscriptionClient",
      "httpBatchStreamLink",
      "loggerLink",
      "createDxTrpcResponseMeta",
      "ResponseMetaFn",
      "infiniteQueryOptions",
      "infiniteQueryKey",
      "infiniteQueryFilter",
      "dxTrpcTransformer",
      "TRPCCombinedDataTransformer",
      "createDxTrpcHttpLinkOptions",
      "HTTPBatchLinkOptions",
    ],
  },
  {
    module: "forge_fumadocs",
    file: "core/src/ecosystem/forge_fumadocs.rs",
    packageId: "content/fumadocs-next",
    markers: [
      "extensionless dx config owns WWW/Fumadocs adapter settings",
      "DocsLayout",
      "llms from fumadocs-core/source",
      "app/llms.txt/route.ts",
      "createOpenAPI from fumadocs-openapi/server",
      "openapi.createProxy from fumadocs-openapi/server",
      "app/api/openapi/proxy/route.ts",
      "createAPIPage from fumadocs-openapi/ui",
      "createCodeUsageGeneratorRegistry",
      "registerDefault",
      "iconPlugin from fumadocs-core/source/icon",
      "statusBadgesPlugin",
      "source-plugins.tsx",
      "getBreadcrumbItems",
      "findNeighbour",
      "navigation.ts",
      "getTableOfContents",
      "TOCItemType",
      "toc.ts",
      "app/docs/[[...slug]]/page.tsx",
      "createFromSource",
      "fumadocs-core/search/server",
      "app/api/search/route.ts",
      "staticGET",
      "useDocsSearch from fumadocs-core/search/client",
      "app/api/search-static/route.ts",
    ],
  },
  {
    module: "forge_react_markdown",
    file: "core/src/ecosystem/forge_react_markdown.rs",
    packageId: "content/react-markdown",
    markers: ["MarkdownAsync", "MarkdownHooks", "defaultUrlTransform"],
  },
  {
    module: "forge_supabase",
    file: "core/src/ecosystem/forge_supabase.rs",
    packageId: "supabase/client",
    markers: ["createBrowserClient", "createServerClient", "NEXT_PUBLIC_SUPABASE_URL"],
  },
  {
    module: "forge_drizzle",
    file: "core/src/ecosystem/forge_drizzle.rs",
    packageId: "db/drizzle-sqlite",
    markers: ["sqliteTable", "InferSelectModel", "better-sqlite3"],
  },
  {
    module: "forge_instantdb",
    file: "core/src/ecosystem/forge_instantdb.rs",
    packageId: "instantdb/react",
    markers: [
      "init",
      "db.useQuery",
      "NEXT_PUBLIC_INSTANT_APP_ID",
      "InstantDbDashboardWorkflow",
      "instantdb-runtime-dashboard-workflow",
      "docs/packages/instantdb-react.md",
    ],
  },
  {
    module: "forge_n8n_automations",
    file: "core/src/ecosystem/forge_n8n_automations.rs",
    packageId: "automations/n8n",
    markers: [
      "dx automations connectors --json",
      "automationSummary",
      "connectorMetadata",
      "createDxN8nRunReceipt",
    ],
  },
  {
    module: "forge_wasm_bindgen",
    file: "core/src/ecosystem/forge_wasm_bindgen.rs",
    packageId: "wasm/bindgen",
    markers: ["wasm-bindgen", "init(input)", "WebAssembly", "moduleCache.delete(cacheKey)"],
  },
  {
    module: "forge_three_scene",
    file: "core/src/ecosystem/forge_three_scene.rs",
    packageId: "3d/launch-scene",
    markers: ["launch-scene.tsx", "scene/webgl-runtime.ts", "scene/metadata.ts"],
  },
];

test("launch package slices stay registered, discoverable, and source-owned", () => {
  for (const packageId of launchPackages) {
    assert.match(
      `${cli}\n${nextFamiliarTemplate}`,
      new RegExp(`"${packageId.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}"`),
      `${packageId} missing from CLI launch metadata`,
    );
    assert.match(registry, new RegExp(`"${packageId.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}"`), `${packageId} missing from Forge registry`);
    assert.match(scorecard, new RegExp(`"${packageId.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}"`), `${packageId} missing from scorecard`);
    assert.match(trustPolicy, new RegExp(`"${packageId.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}"`), `${packageId} missing from trust policy`);
  }

  assert.match(registry, /"authentication" \| "better-auth" \| "auth\/betterauth" \| "auth\/better-auth-next" => \{\s+"auth\/better-auth"\s+\}/);
  assert.match(registry, /"ui\/input" => "shadcn\/ui\/input"/);
  assert.match(registry, /import \{ betterAuth, type BetterAuthOptions \} from "better-auth";/);
  assert.match(registry, /import \{ createAuthClient \} from "better-auth\/react";/);
  assert.match(registry, /import \* as React from "react";/);
  assert.match(registry, /data-slot="input"/);
  assert.match(security, /content\/fumadocs-next/);
  assert.match(security, /ai\/vercel-ai/);
});

test("launch package source files expose real public API markers", () => {
  for (const slice of sourceSlices) {
    assert.match(ecosystemMod, new RegExp(`mod ${slice.module};`), `${slice.module} is not wired in ecosystem mod`);
    const source =
      slice.file === "core/src/ecosystem/forge_registry.rs"
        ? registry
        : read(slice.file);
    assert.match(source, new RegExp(slice.packageId.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")), `${slice.packageId} missing package metadata`);
    for (const marker of slice.markers) {
      assert.match(source, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")), `${slice.packageId} missing marker ${marker}`);
    }
  }
});

test("launch package catalog labels Forge maturity honestly", () => {
  assert.match(packageCatalog, /export type LaunchPackageMaturity/);
  assert.match(packageCatalog, /\| "slice"/);
  assert.match(packageCatalog, /\| "adapter-boundary"/);
  assert.match(packageCatalog, /\| "boundary"/);
  assert.match(packageCatalog, /\| "full"/);

  for (const packageId of launchPackages) {
    const escaped = packageId.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
    assert.match(
      packageCatalog,
      new RegExp(`packageId: "${escaped}",[\\s\\S]*?maturity: "(slice|adapter-boundary|boundary|full)"`),
      `${packageId} must declare slice, adapter-boundary, boundary, or full maturity`,
    );
  }

  for (const packageId of [
    "auth/better-auth",
    "payments/stripe-js",
    "supabase/client",
    "instantdb/react",
    "ai/vercel-ai",
    "automations/n8n",
  ]) {
    const escaped = packageId.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
    assert.match(
      packageCatalog,
      new RegExp(`packageId: "${escaped}",[\\s\\S]*?maturity: "adapter-boundary"`),
      `${packageId} must not be marketed as a full replacement`,
    );
  }

  assert.doesNotMatch(packageCatalog, /full replacement|drop-in replacement/i);
});
