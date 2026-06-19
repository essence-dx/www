export type TemplatePackageStatus =
  | "source-owned"
  | "adapter-boundary"
  | "partial";

export type LaunchPackageMaturity =
  | "slice"
  | "adapter-boundary"
  | "boundary"
  | "full";

export type TemplatePackage = {
  packageId: string;
  officialName: string;
  providerSurfaces?: readonly string[];
  dxCheckVisibility?: {
    schema: "dx.forge.package.dx_check_visibility";
    currentStatus: TemplatePackageStatus;
  };
  dxStyleCompatibility?: {
    schema: "dx.forge.package.dx_style_compatibility";
    tokenSource: string;
  };
};

export type LaunchPackageMaturityEntry = {
  packageId: string;
  maturity: LaunchPackageMaturity;
  boundary: string;
};

export const launchPackageCatalog = [
  { packageId: "ui/button", officialName: "UI Components" },
  { packageId: "ui/badge", officialName: "UI Components" },
  { packageId: "ui/card", officialName: "UI Components" },
  { packageId: "ui/alert", officialName: "UI Components" },
  { packageId: "ui/avatar", officialName: "UI Components" },
  { packageId: "ui/skeleton", officialName: "UI Components" },
  { packageId: "ui/label", officialName: "UI Components" },
  { packageId: "ui/separator", officialName: "UI Components" },
  { packageId: "ui/field", officialName: "UI Components" },
  { packageId: "ui/item", officialName: "UI Components" },
  { packageId: "ui/input", officialName: "UI Components" },
  { packageId: "ui/textarea", officialName: "UI Components" },
  { packageId: "dx/icon/search", officialName: "UI Components and Icons" },
  {
    packageId: "auth/better-auth",
    officialName: "Authentication",
    providerSurfaces: ["google-oauth", "email-password", "session"],
  },
  { packageId: "animation/motion", officialName: "Motion & Animation" },
  { packageId: "i18n/next-intl", officialName: "Internationalization" },
  { packageId: "tanstack/query", officialName: "Data Fetching & Cache" },
  { packageId: "validation/zod", officialName: "Validation & Schemas" },
  { packageId: "forms/react-hook-form", officialName: "Forms" },
  { packageId: "payments/stripe-js", officialName: "Payments" },
  { packageId: "automations/n8n", officialName: "Automations" },
  { packageId: "state/zustand", officialName: "State Management" },
  { packageId: "ai/vercel-ai", officialName: "AI SDK" },
  { packageId: "api/trpc", officialName: "Type-Safe API" },
  { packageId: "content/fumadocs-next", officialName: "Markdown & MDX Content" },
  { packageId: "content/react-markdown", officialName: "Markdown & MDX Content" },
  { packageId: "supabase/client", officialName: "Backend Platform Client" },
  { packageId: "db/drizzle-sqlite", officialName: "Database ORM" },
  { packageId: "instantdb/react", officialName: "Realtime App Database" },
  { packageId: "wasm/bindgen", officialName: "WebAssembly Bridge" },
  { packageId: "3d/launch-scene", officialName: "Three Scene System" },
  { packageId: "migration/static-site", officialName: "Static Migration" },
] satisfies readonly TemplatePackage[];

export const launchPackageMaturityCatalog = [
  {
    packageId: "shadcn/ui/button",
    maturity: "slice",
    boundary: "Editable source-owned button primitive, not full registry parity.",
  },
  {
    packageId: "shadcn/ui/badge",
    maturity: "slice",
    boundary: "Editable source-owned badge primitive, not full registry parity.",
  },
  {
    packageId: "shadcn/ui/card",
    maturity: "slice",
    boundary: "Editable source-owned layout primitive, not full registry parity.",
  },
  {
    packageId: "shadcn/ui/alert",
    maturity: "slice",
    boundary: "Editable source-owned feedback primitive, not full registry parity.",
  },
  {
    packageId: "shadcn/ui/avatar",
    maturity: "slice",
    boundary: "Editable source-owned identity media primitive, not full registry parity.",
  },
  {
    packageId: "shadcn/ui/skeleton",
    maturity: "slice",
    boundary: "Editable source-owned loading primitive, not full registry parity.",
  },
  {
    packageId: "shadcn/ui/label",
    maturity: "slice",
    boundary: "Editable source-owned label primitive, not full registry parity.",
  },
  {
    packageId: "shadcn/ui/separator",
    maturity: "slice",
    boundary: "Editable source-owned separator primitive, not full registry parity.",
  },
  {
    packageId: "shadcn/ui/field",
    maturity: "slice",
    boundary: "Editable source-owned field primitive group, not full registry parity.",
  },
  {
    packageId: "shadcn/ui/item",
    maturity: "slice",
    boundary: "Editable source-owned item primitive group, not full registry parity.",
  },
  {
    packageId: "shadcn/ui/input",
    maturity: "slice",
    boundary: "Editable source-owned input primitive, not full registry parity.",
  },
  {
    packageId: "shadcn/ui/textarea",
    maturity: "slice",
    boundary: "Editable source-owned textarea primitive, not full registry parity.",
  },
  {
    packageId: "dx/icon/search",
    maturity: "slice",
    boundary: "Selected icon source slice, not the full icon catalog.",
  },
  {
    packageId: "auth/better-auth",
    maturity: "adapter-boundary",
    boundary: "Source-owned integration boundary; app auth policy and provider setup remain app-owned.",
  },
  {
    packageId: "animation/motion",
    maturity: "slice",
    boundary: "Source-owned launch animation slice, not every motion gesture and layout API.",
  },
  {
    packageId: "i18n/next-intl",
    maturity: "slice",
    boundary: "Source-owned i18n launch slice, not complete translation operations.",
  },
  {
    packageId: "tanstack/query",
    maturity: "slice",
    boundary: "Source-owned query launch slice, not every observer, persistence, or devtools API.",
  },
  {
    packageId: "validation/zod",
    maturity: "slice",
    boundary: "Source-owned validation slice, not universal schema governance.",
  },
  {
    packageId: "forms/react-hook-form",
    maturity: "slice",
    boundary: "Source-owned form integration slice, not every form state scenario.",
  },
  {
    packageId: "payments/stripe-js",
    maturity: "adapter-boundary",
    boundary: "Source-owned browser/server payment boundary; live Stripe account policy remains app-owned.",
  },
  {
    packageId: "automations/n8n",
    maturity: "adapter-boundary",
    boundary: "Source-owned automation connector boundary; credentials and live execution remain app-owned.",
  },
  {
    packageId: "state/zustand",
    maturity: "slice",
    boundary: "Source-owned store slice, not every middleware and ecosystem extension.",
  },
  {
    packageId: "ai/vercel-ai",
    maturity: "adapter-boundary",
    boundary: "Source-owned AI transport boundary; model safety, keys, and persistence remain app-owned.",
  },
  {
    packageId: "api/trpc",
    maturity: "slice",
    boundary: "Source-owned API slice, not every router, subscription, and deployment scenario.",
  },
  {
    packageId: "content/fumadocs-next",
    maturity: "slice",
    boundary: "Source-owned docs slice, not a complete content operations platform.",
  },
  {
    packageId: "content/react-markdown",
    maturity: "slice",
    boundary: "Source-owned markdown slice, not untrusted raw HTML governance.",
  },
  {
    packageId: "supabase/client",
    maturity: "adapter-boundary",
    boundary: "Source-owned Supabase client boundary; hosted RLS and auth configuration remain app-owned.",
  },
  {
    packageId: "db/drizzle-sqlite",
    maturity: "slice",
    boundary: "Source-owned SQLite ORM slice, not every Drizzle dialect and migration strategy.",
  },
  {
    packageId: "instantdb/react",
    maturity: "adapter-boundary",
    boundary: "Source-owned realtime app boundary; hosted rules and app ids remain app-owned.",
  },
  {
    packageId: "wasm/bindgen",
    maturity: "boundary",
    boundary: "Source-owned wasm-bindgen loader boundary, not Rust macro or binary generation.",
  },
  {
    packageId: "3d/launch-scene",
    maturity: "slice",
    boundary: "Source-owned 3D launch scene slice, not the full Three/R3F/Drei ecosystem.",
  },
  {
    packageId: "migration/static-site",
    maturity: "boundary",
    boundary: "Source-owned static migration seed, not a full CMS migration platform.",
  },
] satisfies readonly LaunchPackageMaturityEntry[];

export const templatePackageCatalogContract = {
  dxCheckVisibilitySchema: "dx.forge.package.dx_check_visibility",
  dxStyleCompatibilitySchema: "dx.forge.package.dx_style_compatibility",
  packageCount: launchPackageCatalog.length,
  maturityPackageCount: launchPackageMaturityCatalog.length,
} as const;
