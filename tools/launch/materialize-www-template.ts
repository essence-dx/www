#!/usr/bin/env node
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

import {
  dashboardNavIconNames,
  dashboardNavItems,
  dashboardProjects,
  dashboardReports,
} from "../../examples/template/components/template-app/template-data.ts";
import { createDashboardQueryCacheStatus } from "../../examples/template/components/template-app/dashboard-query-cache.ts";
import { createDashboardReactiveStoreReadiness } from "../../examples/template/components/template-app/dashboard-reactive-store.ts";
import { createWasmBindgenTemplateReadiness } from "../../examples/template/wasm/bindgen/readiness.ts";
import {
  dashboardForgePackageIdList,
  dashboardForgePackageIds,
  forgeRealityRows,
  forgeRealitySummary,
  interactiveForgePackageRows,
  lane7ForgePackageIdList,
  lane7ForgePackageRows,
  launchEvidenceSummaryRows,
  providerGatedReadinessRows,
  templateReadinessExecutionProofRows,
} from "../../examples/template/components/template-app/package-reality.ts";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const repoRoot = path.resolve(__dirname, "..", "..");
const templateRoot = path.join(repoRoot, "examples", "template");
const runtimeTemplateRoot = path.join(repoRoot, "tools", "launch", "runtime-template");
const runtimePagesRoot = path.join(runtimeTemplateRoot, "pages");
const runtimeAssetsRoot = path.join(runtimeTemplateRoot, "assets");
const forgeReceiptsRoot = path.join(templateRoot, ".dx", "forge", "receipts");
const forgeTemplateReadinessRoot = path.join(templateRoot, ".dx", "forge", "template-readiness");
const forgeCacheRoot = path.join(templateRoot, ".dx", "forge", "cache");
const forgeCacheArchiveRoot = path.join(templateRoot, ".dx", "forge", "cache-archive");
const sourceWwwTemplateRoot = path.resolve(templateRoot);
const DX_STYLE_BROWSER_COMPAT_PREVIEW_EVIDENCE = {
  schema: "dx.style.browser_compat.preview_evidence",
  rowId: "dx-style-browser-compat",
  title: "dx-style browser compatibility",
  status: "missing",
  receiptPath: ".dx/receipts/style/check.json",
  fixturePath: "related-crates/style/fixtures/tailwind-postcss-browser-compat.json",
  zedVisibility: "dx-style:browser-compat",
  canaryClassCount: 0,
  selectorCanaryClassCount: 0,
  selectorClassExamples: [],
  tailwindParityStateAliasSupportedClassCount: 0,
  tailwindParitySupportedStateAliasExamples: [],
  fullAutoprefixerParity: false,
  fullTailwindPostcssOutputParity: false,
  source: "check_panel.view_model.style_evidence_rows",
  runtimeProof: false,
  nextAction:
    "Run dx style check, then dx check --json, and reload the check-panel receipt.",
};
const DX_STYLE_PACKAGE_PANEL_DRIFT_READ_MODEL =
  "dx.www.template.preview_style_package_panel_with_drift_read_model";
const DX_STYLE_BROWSER_COMPAT_DRIFT_FIXTURE = {
  schema: "dx.style.browser_compat.drift_fixture",
  rowId: "dx-style-browser-compat",
  route: "/",
  status: "source-guarded",
  packagePanelReadModel: DX_STYLE_PACKAGE_PANEL_DRIFT_READ_MODEL,
  loaderFile: "components/template-app/template-shell-evidence-loader.ts",
  markerHelperFile: "components/template-app/template-shell-style-evidence-drift.ts",
  fixturePath: "related-crates/style/fixtures/tailwind-postcss-browser-compat.json",
  states: ["unknown", "false", "true"],
  fullAutoprefixerParity: false,
  fullTailwindPostcssOutputParity: false,
};
const DX_STYLE_PACKAGE_OWNERSHIP_ROWS = [
  {
    schema: "dx.style.package_ownership",
    packageId: "shadcn/ui/button",
    packageName: "UI Components",
    styleScope: "ui-components",
    sourceFiles: ["examples/template/components/ui/button.tsx"],
    requiredTokens: ["primary", "primary-foreground", "ring", "surface"],
    generatedClasses: [
      "inline-flex",
      "items-center",
      "justify-center",
      "rounded-md",
      "text-sm",
      "font-medium",
      "transition-colors",
      "bg-token(surface)",
    ],
    unsupportedClasses: [],
    tokenSource: "styles/globals.css",
    generatedCss: "styles/generated.css",
    receiptPath: ".dx/forge/receipts/packages/shadcn-ui-button.json",
    zedVisibility: "shadcn-ui-button:style-ownership",
    runtimeProof: false,
  },
  {
    schema: "dx.style.package_ownership",
    packageId: "animation/motion",
    packageName: "Motion Animation",
    styleScope: "motion-animation",
    sourceFiles: [
      "examples/template/components/template-app/motion.tsx",
      "examples/template/motion/dashboard-workflow.ts",
    ],
    requiredTokens: ["accent", "accent-foreground", "surface"],
    generatedClasses: [
      "motion-safe:animate-pulse",
      "motion-reduce:transition-none",
      "opacity-100",
      "animate-[var(--package-animation)]",
    ],
    unsupportedClasses: [],
    tokenSource: "styles/globals.css",
    generatedCss: "styles/generated.css",
    receiptPath: ".dx/forge/receipts/packages/animation-motion.json",
    zedVisibility: "animation-motion:style-ownership",
    runtimeProof: false,
  },
];
const FORGE_SAFETY_ARCHIVE_SOURCE_GUARD_RUNBOOK_FIXTURE = {
  packageId: "forge/safety-archive",
  officialPackageName: "Forge Safety Archive",
  upstreamPackage: "dx-forge",
  upstreamVersion: "launch-local",
  sourceMirror: "G:/Dx/www",
  route: "/",
  fixture: "docs/packages/forge-safety-archive.source-guard-runbook.json",
  guardId: "forge-safety-archive-rollback-coverage",
  schema: "dx.forge.safety_archive.source_guard_runbook_fixture",
  honestyLabel: "LOCK-BACKED SOURCE-OWNED",
  runtimeProof: false,
  zedVisibility: "forge-safety-archive:rollback-coverage",
  command: "dx run --test .\\benchmarks\\www-forge-package-lock.test.ts",
  commandPurpose:
    "Validate source-owned Forge package lock, archive receipt, rollback receipt, cache-file, and safety_archive status coverage for the app template.",
  scope: "source-only",
  writesFiles: false,
  startsServer: false,
  runsPackageInstall: false,
  runsFullBuild: false,
  nodeModulesRequired: false,
};
const AUTHENTICATION_SOURCE_GUARD_RUNBOOK_FIXTURE = {
  packageId: "auth/better-auth",
  officialPackageName: "Authentication",
  upstreamPackage: "better-auth",
  upstreamVersion: "1.6.11",
  sourceMirror: "G:/WWW/inspirations/better-auth",
  route: "/",
  fixture: "docs/packages/authentication.source-guard-runbook.json",
  guardId: "authentication-package-lane-panel",
  schema: "dx.forge.package.source_guard_runbook_fixture",
  honestyLabel: "ADAPTER-BOUNDARY",
  runtimeProof: false,
  zedVisibility: "authentication:receipt-hash-refresh",
};
const STATE_MANAGEMENT_SOURCE_GUARD_RUNBOOK_FIXTURE = {
  packageId: "state/zustand",
  officialPackageName: "State Management",
  upstreamPackage: "zustand",
  upstreamVersion: "5.0.13",
  sourceMirror: "G:/WWW/inspirations/zustand",
  route: "/",
  fixture: "docs/packages/state-zustand.source-guard-runbook.json",
  guardId: "state-management-generated-starter-materialization",
  schema: "dx.forge.package.source_guard_runbook_fixture",
  honestyLabel: "SOURCE-ONLY",
  runtimeProof: false,
  zedVisibility: "state-management:receipt-hash-refresh",
};
const REACTIVE_STORE_SOURCE_GUARD_RUNBOOK_FIXTURE = {
  packageId: "reactive/store",
  officialPackageName: "Reactive Store",
  upstreamPackage: "@tanstack/store",
  upstreamVersion: "0.11.0",
  sourceMirror: "G:/WWW/inspirations/tanstack-store",
  route: "/",
  fixture: "docs/packages/reactive-store.source-guard-runbook.json",
  guardId: "reactive-store-lower-dx-check-helper-freshness",
  schema: "dx.forge.package.source_guard_runbook_fixture",
  honestyLabel: "SOURCE-ONLY",
  runtimeProof: false,
  zedVisibility: "reactive-store:receipt-hash-refresh",
};
const DATA_FETCHING_CACHE_SOURCE_GUARD_RUNBOOK_FIXTURE = {
  packageId: "tanstack/query",
  officialPackageName: "Data Fetching & Cache",
  upstreamPackage: "@tanstack/react-query",
  upstreamVersion: "5.100.10",
  sourceMirror: "G:/WWW/inspirations/tanstack-query",
  route: "/",
  fixture: "docs/packages/data-fetching-cache.source-guard-runbook.json",
  guardId: "data-fetching-cache-generated-starter-materialization",
  schema: "dx.forge.package.source_guard_runbook_fixture",
  honestyLabel: "SOURCE-ONLY",
  runtimeProof: false,
  zedVisibility: "data-fetching-cache:receipt-hash-refresh",
};
const VALIDATION_SCHEMAS_SOURCE_GUARD_RUNBOOK_FIXTURE = {
  packageId: "validation/zod",
  officialPackageName: "Validation & Schemas",
  upstreamPackage: "zod",
  upstreamVersion: "4.4.3",
  sourceMirror: "G:/WWW/inspirations/zod",
  route: "/",
  fixture: "docs/packages/validation-schemas.source-guard-runbook.json",
  guardId: "validation-schemas-generated-starter-materialization",
  schema: "dx.forge.package.source_guard_runbook_fixture",
  honestyLabel: "SOURCE-ONLY",
  runtimeProof: false,
  zedVisibility: "validation-schemas:receipt-hash-refresh",
};
const PAYMENTS_SOURCE_GUARD_RUNBOOK_FIXTURE = {
  packageId: "payments/stripe-js",
  officialPackageName: "Payments",
  upstreamPackage: "@stripe/stripe-js",
  upstreamVersion: "9.6.0",
  sourceMirror: "G:/WWW/inspirations/stripe-js",
  route: "/",
  fixture: "docs/packages/payments.source-guard-runbook.json",
  guardId: "payments-generated-starter-materialization",
  schema: "dx.forge.package.source_guard_runbook_fixture",
  honestyLabel: "SOURCE-ONLY",
  runtimeProof: false,
  zedVisibility: "payments:receipt-hash-refresh",
};
const FORMS_SOURCE_GUARD_RUNBOOK_FIXTURE = {
  packageId: "forms/react-hook-form",
  officialPackageName: "Forms",
  upstreamPackage: "react-hook-form",
  upstreamVersion: "7.75.0",
  sourceMirror: "G:/WWW/inspirations/react-hook-form",
  route: "/",
  fixture: "docs/packages/forms.source-guard-runbook.json",
  guardId: "forms-generated-starter-materialization",
  schema: "dx.forge.package.source_guard_runbook_fixture",
  honestyLabel: "SOURCE-ONLY",
  runtimeProof: false,
  zedVisibility: "forms:receipt-hash-refresh",
};
const WEBASSEMBLY_BRIDGE_SOURCE_GUARD_RUNBOOK_FIXTURE = {
  packageId: "wasm/bindgen",
  officialPackageName: "WebAssembly Bridge",
  upstreamPackage: "wasm-bindgen",
  upstreamVersion: "0.2.121",
  sourceMirror: "G:/WWW/inspirations/wasm-bindgen",
  route: "/",
  fixture: "docs/packages/wasm-bindgen.source-guard-runbook.json",
  guardId: "webassembly-bridge-generated-starter-materialization",
  schema: "dx.forge.package.source_guard_runbook_fixture",
  honestyLabel: "SOURCE-ONLY",
  runtimeProof: false,
  zedVisibility: "webassembly-bridge:source-guard-runbook",
};
const MOTION_ANIMATION_SOURCE_GUARD_RUNBOOK_FIXTURE = {
  packageId: "animation/motion",
  officialPackageName: "Motion & Animation",
  upstreamPackage: "motion",
  upstreamVersion: "12.38.0",
  sourceMirror: "G:/WWW/inspirations/motion",
  route: "/",
  fixture: "docs/packages/motion-animation.source-guard-runbook.json",
  guardId: "motion-animation-generated-starter-materialization",
  schema: "dx.forge.package.source_guard_runbook_fixture",
  honestyLabel: "SOURCE-ONLY",
  runtimeProof: false,
  zedVisibility: "motion-animation:receipt-hash-refresh",
};
const THREE_SCENE_SYSTEM_SOURCE_GUARD_RUNBOOK_FIXTURE = {
  packageId: "3d/launch-scene",
  officialPackageName: "3D Scene System",
  upstreamPackage: "three + @react-three/fiber + @react-three/drei",
  upstreamVersion:
    "three 0.184.0; @react-three/fiber 9.6.1; @react-three/drei local mirror",
  sourceMirror:
    "G:/WWW/inspirations/three.js; G:/WWW/inspirations/react-three-fiber; G:/WWW/inspirations/drei",
  route: "/",
  fixture: "docs/packages/3d-scene-system.source-guard-runbook.json",
  guardId: "three-scene-system-lower-dx-check-helper-freshness",
  schema: "dx.forge.package.source_guard_runbook_fixture",
  honestyLabel: "SOURCE-ONLY",
  runtimeProof: false,
  zedVisibility: "3d-scene-system:receipt-hash-refresh",
};
const DOCUMENTATION_SYSTEM_SOURCE_GUARD_RUNBOOK_FIXTURE = {
  packageId: "content/fumadocs-next",
  officialPackageName: "Documentation System",
  upstreamPackage: "fumadocs",
  upstreamVersion: "16.8.12",
  sourceMirror: "G:/WWW/inspirations/fumadocs",
  route: "/",
  fixture: "docs/packages/content-fumadocs-next.source-guard-runbook.json",
  guardId: "documentation-system-generated-starter-materialization",
  schema: "dx.forge.package.source_guard_runbook_fixture",
  honestyLabel: "SOURCE-ONLY",
  runtimeProof: false,
  zedVisibility: "documentation-system:receipt-hash-refresh",
};
const AUTOMATION_CONNECTORS_SOURCE_GUARD_RUNBOOK_FIXTURE = {
  packageId: "automations/n8n",
  officialPackageName: "Automation Connectors",
  upstreamPackage: "n8n-nodes-base",
  upstreamVersion: "2.22.0",
  sourceMirror: "G:/WWW/inspirations/n8n/packages/nodes-base",
  route: "/",
  fixture: "docs/packages/automation-connectors.source-guard-runbook.json",
  guardId: "automation-connectors-package-lane-panel",
  schema: "dx.forge.package.source_guard_runbook_fixture",
  honestyLabel: "ADAPTER-BOUNDARY",
  runtimeProof: false,
  zedVisibility: "automation-connectors:receipt-hash-refresh",
};
const INTERNATIONALIZATION_SOURCE_GUARD_RUNBOOK_FIXTURE = {
  packageId: "i18n/next-intl",
  officialPackageName: "Internationalization",
  upstreamPackage: "next-intl",
  upstreamVersion: "4.12.0",
  sourceMirror: "G:/WWW/inspirations/next-intl",
  route: "/",
  fixture: "docs/packages/next-intl.source-guard-runbook.json",
  guardId: "internationalization-launch-package-lane-template",
  schema: "dx.forge.package.source_guard_runbook_fixture",
  honestyLabel: "SOURCE-ONLY",
  runtimeProof: false,
  zedVisibility: "internationalization:receipt-hash-refresh",
};
const TYPE_SAFE_API_SOURCE_GUARD_RUNBOOK_FIXTURE = {
  packageId: "api/trpc",
  officialPackageName: "Type-Safe API",
  upstreamPackage: "@trpc/server",
  upstreamVersion: "11.17.0",
  sourceMirror: "G:/WWW/inspirations/trpc",
  route: "/",
  fixture: "docs/packages/api-trpc.source-guard-runbook.json",
  guardId: "type-safe-api-unsupported-surface-context",
  schema: "dx.forge.package.source_guard_runbook_fixture",
  honestyLabel: "SOURCE-ONLY",
  runtimeProof: false,
  zedVisibility: "type-safe-api:receipt-hash-refresh",
};
const DATABASE_ORM_SOURCE_GUARD_RUNBOOK_FIXTURE = {
  packageId: "db/drizzle-sqlite",
  officialPackageName: "Database ORM",
  upstreamPackage: "drizzle-orm",
  upstreamVersion: "0.45.3",
  sourceMirror: "G:/WWW/inspirations/drizzle-orm",
  route: "/",
  fixture: "docs/packages/database-orm.source-guard-runbook.json",
  guardId: "database-orm-lower-dx-check-helper-freshness",
  schema: "dx.forge.package.source_guard_runbook_fixture",
  honestyLabel: "SOURCE-ONLY",
  runtimeProof: false,
  zedVisibility: "database-orm:receipt-hash-refresh",
};
const AI_SDK_SOURCE_GUARD_RUNBOOK_FIXTURE = {
  packageId: "ai/vercel-ai",
  officialPackageName: "AI SDK",
  upstreamPackage: "ai",
  upstreamVersion: "7.0.0-canary.146",
  sourceMirror: "G:/WWW/inspirations/vercel-ai",
  route: "/",
  fixture: "docs/packages/ai-sdk.source-guard-runbook.json",
  guardId: "ai-sdk-check-panel-helper-freshness",
  schema: "dx.forge.package.source_guard_runbook_fixture",
  honestyLabel: "SOURCE-ONLY",
  runtimeProof: false,
  zedVisibility: "ai-sdk:receipt-hash-refresh",
};
const BACKEND_PLATFORM_CLIENT_SOURCE_GUARD_RUNBOOK_FIXTURE = {
  packageId: "supabase/client",
  officialPackageName: "Backend Platform Client",
  upstreamPackage: "@supabase/ssr + @supabase/supabase-js",
  upstreamVersion: "@supabase/ssr latest; @supabase/supabase-js ^2",
  sourceMirror: "G:/WWW/inspirations/supabase",
  route: "/",
  fixture: "docs/packages/backend-platform-client.source-guard-runbook.json",
  guardId: "backend-platform-client-lower-dx-check-helper-freshness",
  schema: "dx.forge.package.source_guard_runbook_fixture",
  honestyLabel: "SOURCE-ONLY",
  runtimeProof: false,
  zedVisibility: "backend-platform-client:receipt-hash-refresh",
};
const UI_COMPONENTS_SOURCE_GUARD_RUNBOOK_FIXTURE = {
  packageId: "shadcn/ui/button",
  officialPackageName: "UI Components",
  upstreamPackage: "shadcn-ui",
  upstreamVersion: "0.0.1",
  sourceMirror: "G:/WWW/inspirations/shadcn-ui; G:/WWW/inspirations/radix-primitives",
  route: "/",
  fixture: "docs/packages/ui-components.source-guard-runbook.json",
  guardId: "ui-components-generated-starter-materialization",
  schema: "dx.forge.package.source_guard_runbook_fixture",
  honestyLabel: "SOURCE-ONLY",
  runtimeProof: false,
  zedVisibility: "ui-components:receipt-hash-refresh",
};

const DASHBOARD_FORGE_PACKAGE_IDS = dashboardForgePackageIds;

function usage() {
  console.error("Usage: node tools/launch/materialize-www-template.ts <project-dir>");
  process.exit(2);
}

function ensureDir(dir) {
  fs.mkdirSync(dir, { recursive: true });
}

function copyFile(source, target) {
  ensureDir(path.dirname(target));
  fs.copyFileSync(source, target);
}

function copyTextFile(source, target, transform = (text) => text) {
  ensureDir(path.dirname(target));
  fs.writeFileSync(target, transform(fs.readFileSync(source, "utf8")), "utf8");
}

function writeText(target, text) {
  ensureDir(path.dirname(target));
  fs.writeFileSync(target, text, "utf8");
}

function removeFileIfExists(target) {
  if (fs.existsSync(target)) {
    fs.unlinkSync(target);
  }
}

function dxIconName(attrs) {
  return attrs.match(/\bname="([^"]+)"/)?.[1] || "pack:unknown";
}

function ensureAttribute(attrs, name, value) {
  return new RegExp(`\\b${name}=`).test(attrs) ? attrs : `${attrs} ${name}="${value}"`;
}

function annotateNativeScrollMarkers(html) {
  return html.replace(/<main\b([^>]*)>/g, (tag, attrs) => {
    if (!tag.includes('data-dx-scroll-proof="document-flow-no-lock"')) {
      return tag;
    }
    if (tag.includes("data-dx-scroll-content=")) {
      return tag;
    }
    const closingIndent = attrs.match(/\n(\s*)$/)?.[1];
    if (closingIndent !== undefined) {
      const trimmedAttrs = attrs.replace(/\s+$/, "");
      return `<main${trimmedAttrs}\n${closingIndent}  data-dx-scroll-content="viewport-plus"\n${closingIndent}>`;
    }
    return `<main${attrs} data-dx-scroll-content="viewport-plus">`;
  });
}

function annotateDxIconMarkers(html) {
  let annotated = annotateNativeScrollMarkers(html).replace(/<dx-icon\s+([^>]*?)(\/?)>/g, (_, attrs, slash) => {
    const iconName = dxIconName(attrs);
    let nextAttrs = ensureAttribute(attrs.trim(), "data-icon-source", "dx-icons");
    nextAttrs = ensureAttribute(nextAttrs, "data-dx-icon", iconName);
    return `<dx-icon ${nextAttrs}${slash || ""}>`;
  });

  if (annotated.includes('data-dx-component="dx-icon-runtime-markers"')) {
    return annotated;
  }

  const markerBlock = `\n      <div class="dx-icon-runtime-markers" data-dx-component="dx-icon-runtime-markers" data-dx-icon-proof="runtime-markers" aria-hidden="true">\n        <span data-dx-icon="pack:auth" data-icon-source="dx-icons"></span>\n        <span data-dx-icon="pack:payments" data-icon-source="dx-icons"></span>\n        <span data-dx-icon="pack:database" data-icon-source="dx-icons"></span>\n        <span data-dx-icon="pack:fumadocs" data-icon-source="dx-icons"></span>\n        <span data-dx-icon="pack:wasm-bindgen" data-icon-source="dx-icons"></span>\n      </div>\n`;
  return annotated.replace(/(<main\b[^>]*>)/, `$1${markerBlock}`);
}

function disableConflictingAppRoute(projectDir, routeParts = ["launch"]) {
  if (path.resolve(projectDir) === sourceWwwTemplateRoot) {
    return null;
  }

  const appRoute = path.join(projectDir, "app", ...routeParts, "page.tsx");
  const sourceOnlyRoute = path.join(projectDir, "app", ...routeParts, "page.tsx.source-only");
  if (!fs.existsSync(appRoute)) return null;

  if (!fs.existsSync(sourceOnlyRoute)) {
    fs.renameSync(appRoute, sourceOnlyRoute);
    return path.relative(projectDir, sourceOnlyRoute).replaceAll("\\", "/");
  }

  fs.unlinkSync(appRoute);
  return path.relative(projectDir, sourceOnlyRoute).replaceAll("\\", "/");
}

function runtimePageSource(name) {
  const source = path.join(runtimePagesRoot, `${name}.html`);
  if (fs.existsSync(source)) {
    return source;
  }
  throw new Error(`Missing tool-owned page fixture: ${name}.html`);
}

function escapeHtml(value) {
  return String(value).replace(/[&<>"']/g, (character) => {
    const entities = {
      "&": "&amp;",
      "<": "&lt;",
      ">": "&gt;",
      '"': "&quot;",
      "'": "&#39;",
    };
    return entities[character];
  });
}

function publicReadinessLabel(row) {
  const labels = {
    "lock-backed-package": "Installed",
    "source-owned-limited-proof": "Source-ready",
    "adapter-boundary-readiness": "Adapter-ready",
    "provider-gated": "Setup needed",
    "source-guard-only": "Guarded source",
  };
  return labels[row.maturityKind] ?? row.maturityLabel;
}

const dxIconBodies = {
  "lucide:arrow-up-right": '<path d="M7 7h10v10M7 17 17 7" />',
  "lucide:badge-check": '<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.77 4 4 0 0 1 0 6.76 4 4 0 0 1-4.78 4.77 4 4 0 0 1-6.74 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z" /><path d="m9 12 2 2 4-4" />',
  "lucide:bar-chart-3": '<path d="M3 3v18h18" /><path d="M18 17V9" /><path d="M13 17V5" /><path d="M8 17v-3" />',
  "lucide:bolt": '<path d="M13 2 3 14h7l-1 8 10-12h-7l1-8Z" />',
  "lucide:brain-circuit": '<path d="M12 5a3 3 0 0 0-5.83-1M12 5a3 3 0 0 1 5.83-1M12 5v14" /><path d="M7 8H4a2 2 0 0 0 0 4h1" /><path d="M17 8h3a2 2 0 0 1 0 4h-1" /><path d="M7 16H5a2 2 0 1 0 0 4h2" /><path d="M17 16h2a2 2 0 1 1 0 4h-2" />',
  "lucide:boxes": '<path d="M2.97 12.92 12 18.15l9.03-5.23" /><path d="M2.97 7.08 12 12.31l9.03-5.23L12 1.85 2.97 7.08Z" /><path d="M2.97 7.08v5.84L12 18.15v-5.84" /><path d="M21.03 7.08v5.84L12 18.15" /><path d="M12 22.15v-4" />',
  "lucide:check-circle": '<path d="M22 11.08V12a10 10 0 1 1-5.93-9.14" /><path d="m9 11 3 3L22 4" />',
  "lucide:clock": '<circle cx="12" cy="12" r="10" /><path d="M12 6v6l4 2" />',
  "lucide:database": '<ellipse cx="12" cy="5" rx="9" ry="3" /><path d="M3 5v14c0 1.66 4.03 3 9 3s9-1.34 9-3V5" /><path d="M3 12c0 1.66 4.03 3 9 3s9-1.34 9-3" />',
  "lucide:folder-check": '<path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.7-.9L9.6 3.9A2 2 0 0 0 7.9 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z" /><path d="m9 13 2 2 4-4" />',
  "lucide:languages": '<path d="m5 8 6 6" /><path d="m4 14 6-6 2-3" /><path d="M2 5h12" /><path d="M7 2h1" /><path d="m22 22-5-10-5 10" /><path d="M14 18h6" />',
  "lucide:layout-dashboard": '<rect width="7" height="9" x="3" y="3" rx="1" /><rect width="7" height="5" x="14" y="3" rx="1" /><rect width="7" height="9" x="14" y="12" rx="1" /><rect width="7" height="5" x="3" y="16" rx="1" />',
  "lucide:list-checks": '<path d="m3 7 2 2 4-4" /><path d="m3 17 2 2 4-4" /><path d="M13 6h8" /><path d="M13 12h8" /><path d="M13 18h8" />',
  "lucide:log-in": '<path d="M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4" /><path d="m10 17 5-5-5-5" /><path d="M15 12H3" />',
  "lucide:log-out": '<path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4" /><path d="m16 17 5-5-5-5" /><path d="M21 12H9" />',
  "lucide:menu": '<path d="M4 6h16" /><path d="M4 12h16" /><path d="M4 18h16" />',
  "lucide:moon": '<path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" />',
  "lucide:rocket": '<path d="M4.5 16.5c-1.5 1.26-2 5-2 5s3.74-.5 5-2c.71-.84.7-2.13-.09-2.91a2.18 2.18 0 0 0-2.91-.09Z" /><path d="m12 15-3-3a22 22 0 0 1 2-3.95A12.88 12.88 0 0 1 22 2c0 2.72-.78 7.5-6 11a22.35 22.35 0 0 1-4 2Z" /><path d="M9 12H4s.55-3.03 2-4c1.62-1.08 5 0 5 0" /><path d="M12 15v5s3.03-.55 4-2c1.08-1.62 0-5 0-5" />',
  "lucide:settings": '<path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.38a2 2 0 0 0-.73-2.73l-.15-.09a2 2 0 0 1-1-1.74v-.51a2 2 0 0 1 1-1.72l.15-.1a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2Z" /><circle cx="12" cy="12" r="3" />',
  "lucide:sun": '<circle cx="12" cy="12" r="4" /><path d="M12 2v2" /><path d="M12 20v2" /><path d="m4.93 4.93 1.41 1.41" /><path d="m17.66 17.66 1.41 1.41" /><path d="M2 12h2" /><path d="M20 12h2" /><path d="m6.34 17.66-1.41 1.41" /><path d="m19.07 4.93-1.41 1.41" />',
  "lucide:users": '<path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" /><circle cx="9" cy="7" r="4" /><path d="M22 21v-2a4 4 0 0 0-3-3.87" /><path d="M16 3.13a4 4 0 0 1 0 7.75" />',
  "lucide:wrench": '<path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94Z" />',
};

const dxIconAliases = {
  "lucide:arrow-up-right": "action:open",
  "lucide:badge-check": "pack:validation",
  "lucide:bar-chart-3": "nav:reports",
  "lucide:bolt": "state:bolt",
  "lucide:brain-circuit": "ai:brain",
  "lucide:boxes": "pack:workspace",
  "lucide:check-circle": "status:check",
  "lucide:clock": "pack:query",
  "lucide:database": "pack:database",
  "lucide:folder-check": "content:folder-check",
  "lucide:languages": "i18n:languages",
  "lucide:layout-dashboard": "nav:dashboard",
  "lucide:list-checks": "status:list-checks",
  "lucide:log-in": "action:login",
  "lucide:log-out": "action:logout",
  "lucide:menu": "action:menu",
  "lucide:moon": "theme:moon",
  "lucide:rocket": "action:rocket",
  "lucide:settings": "nav:settings",
  "lucide:sun": "theme:sun",
  "lucide:users": "nav:team",
  "lucide:wrench": "action:tools",
};

for (const [legacyName, canonicalName] of Object.entries(dxIconAliases)) {
  dxIconBodies[canonicalName] = dxIconBodies[canonicalName] || dxIconBodies[legacyName];
}

Object.assign(dxIconBodies, {
  "pack:forms": dxIconBodies["lucide:list-checks"],
  "pack:motion": dxIconBodies["lucide:rocket"],
  "pack:state": dxIconBodies["lucide:bar-chart-3"],
  "pack:three-scene": dxIconBodies["lucide:boxes"],
  "pack:ui-components": dxIconBodies["lucide:layout-dashboard"],
  "pack:wasm-bindgen": dxIconBodies["lucide:bolt"],
});

function canonicalDxIconName(name = "nav:dashboard") {
  return dxIconAliases[name] || name;
}

function renderDxIcon(name, className = "dx-icon", label = "") {
  const canonicalName = canonicalDxIconName(name);
  const [set = "dx", iconName = "circle"] = canonicalName.split(":");
  const labelAttrs = label
    ? ` aria-label="${escapeHtml(label)}"`
    : ' aria-hidden="true"';
  const body = dxIconBodies[canonicalName] || dxIconBodies[name] || '<circle cx="12" cy="12" r="9" />';
  return `<svg class="${escapeHtml(className)}" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"${labelAttrs} data-icon-source="dx-icons" data-dx-icon="${escapeHtml(canonicalName)}" data-dx-icon-set="${escapeHtml(set)}" data-dx-icon-name="${escapeHtml(iconName)}">${body}</svg>`;
}

function renderLane7ForgeSystems() {
  const wasmReadiness = createWasmBindgenTemplateReadiness();
  const lane7Score = Math.round(
    lane7ForgePackageRows.reduce((total, row) => total + row.score, 0) /
      lane7ForgePackageRows.length,
  );
  const lane7PackageCards = lane7ForgePackageRows
    .map(
      (row) => `<article data-dx-package="${escapeHtml(row.packageId)}" data-dx-lane7-package-reality="${escapeHtml(row.realityLevelId)}" data-dx-lane7-package-score="${row.score}">
                ${renderDxIcon(row.iconName)}
                <span>${escapeHtml(row.packageName)}</span>
                <strong data-dx-score-scope="lane7-package-readiness-row" data-dx-package-score="${row.score}">${row.score}/100</strong>
              </article>`,
    )
    .join("\n              ");

  return `<section
            id="lane7"
            class="template-card dashboard-panel lane7-forge-systems"
            data-dx-component="lane7-forge-systems"
            data-dx-package="${lane7ForgePackageIdList}"
            data-dx-style-surface="lane7-forge-systems"
            data-dx-node-modules="forbidden"
            data-dx-lane="7"
          >
            <div class="dashboard-panel-header">
              <div>
                <h2>UI, Motion, 3D, and WASM</h2>
                <p>Interactive UI systems stay ready while renderer and compute checks remain gated.</p>
              </div>
              <output class="lane7-score" data-dx-score-scope="lane7-package-summary" data-dx-lane7-score="${lane7Score}">${lane7Score}/100</output>
            </div>
            <div class="lane7-package-strip" aria-label="Lane 7 package status">
              ${lane7PackageCards}
            </div>
            <div class="lane7-boundary-grid">
              <article
                class="lane7-motion-control"
                data-dx-component="lane7-motion-control"
                data-dx-dashboard-workflow="lane7-motion-local-state"
                data-dx-motion-interaction="dashboard-stage-toggle"
                data-dx-motion-state="source-owned"
                data-dx-package="animation/motion"
                data-template-module="lane7-motion-stage"
              >
                <div>
                  ${renderDxIcon("pack:motion")}
                  <strong>Motion & Animation</strong>
                  <output data-template-module-status>Source-owned motion stage ready</output>
                </div>
                <button class="template-button template-button-primary" type="button" data-template-module-action="lane7-motion-stage">${renderDxIcon("action:rocket")}<span>Advance stage</span></button>
              </article>
              <article
                data-dx-lane7-boundary="lock-backed-source-owned-3d"
                data-dx-package="3d/launch-scene"
                data-dx-runtime-proof="false"
                data-dx-style-surface="launch-scene"
              >
                ${renderDxIcon("pack:three-scene")}
                <strong>3D Scene System</strong>
                <span>Scene controls are ready for review; live renderer checks stay gated.</span>
              </article>
              <article
                data-dx-lane7-boundary="source-only-wasm"
                data-dx-package="wasm/bindgen"
                data-dx-runtime-proof="false"
                data-dx-style-surface="theme-token"
                data-dx-wasm-readiness="${escapeHtml(wasmReadiness.status)}"
                data-dx-wasm-source-file="${escapeHtml(wasmReadiness.sourceFile)}"
              >
                ${renderDxIcon("pack:wasm-bindgen")}
                <strong>WebAssembly Bridge</strong>
                <span>${escapeHtml(wasmReadiness.summary)}</span>
                <small>${escapeHtml(wasmReadiness.nextAction)}</small>
              </article>
            </div>
          </section>`;
}

function countPhysicalCacheManifests() {
  if (!fs.existsSync(forgeCacheRoot)) {
    return 0;
  }

  let manifestCount = 0;
  const visit = (directory) => {
    for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
      const fullPath = path.join(directory, entry.name);
      if (entry.isDirectory()) {
        visit(fullPath);
      } else if (entry.name === ".dx/build-cache/manifest.json") {
        manifestCount += 1;
      }
    }
  };

  visit(forgeCacheRoot);
  return manifestCount;
}

function listCacheArchiveManifestPaths() {
  if (!fs.existsSync(forgeCacheArchiveRoot)) {
    return [];
  }

  const manifestPaths = [];
  const visit = (directory) => {
    for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
      const fullPath = path.join(directory, entry.name);
      if (entry.isDirectory()) {
        visit(fullPath);
      } else if (entry.name === ".dx/build-cache/manifest.json") {
        manifestPaths.push(
          path.relative(templateRoot, fullPath).replace(/\\/g, "/"),
        );
      }
    }
  };

  visit(forgeCacheArchiveRoot);
  return manifestPaths.sort();
}

const nativeScrollProofRoutes = [
  { route: "/", sourceFile: "pages/index.html" },
  { route: "/dashboard", sourceFile: "pages/dashboard.html" },
  { route: "/login", sourceFile: "pages/login.html" },
];

function firstTag(html, tagName) {
  return html.match(new RegExp(`<${tagName}\\b[^>]*>`, "i"))?.[0] ?? "";
}

function tagInnerHtml(html, tagName) {
  return html.match(new RegExp(`<${tagName}\\b[^>]*>([\\s\\S]*?)</${tagName}>`, "i"))?.[1] ?? "";
}

function tagHasAttribute(tag, name, value) {
  return new RegExp(`\\b${name}="${value}"`).test(tag);
}

function tagAttributeValue(tag, name) {
  return tag.match(new RegExp(`\\b${name}="([^"]*)"`))?.[1] ?? "";
}

function tagHasInlineScrollLock(tag) {
  const style = tagAttributeValue(tag, "style");
  return /overflow(?:-y)?\s*:\s*(?:hidden|clip)\b|position\s*:\s*fixed\b/.test(style);
}

function tagHasScrollLockClass(tag) {
  const className = tagAttributeValue(tag, "class");
  return className
    .split(/\s+/)
    .some((part) => ["no-scroll", "scroll-lock", "is-scroll-locked"].includes(part));
}

function tagHasScrollLockDataAttribute(tag) {
  return /\bdata-(?:scroll-lock|body-scroll-lock)="(?:true|locked)"/.test(tag);
}

function hasCustomScrollbarSurface(source) {
  return /::-webkit-scrollbar|scrollbar-color|scrollbar-width|data-custom-scrollbar|dashboard-scrollbar|data-scroll-reveal/.test(
    source,
  );
}

function cssRules(source, selector) {
  const selectorPattern = selector
    .trim()
    .split(/\s+/)
    .map((part) => part.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"))
    .join("\\s+");
  return source.match(new RegExp(`(?:^|\\n)${selectorPattern}\\s*\\{[\\s\\S]*?\\}`, "g")) ?? [];
}

function hasFixedPosition(rules) {
  return rules.some((rule) => /position:\s*fixed\b/.test(rule));
}

function collectNativeScrollProof(projectDir) {
  const css = fs.readFileSync(path.join(projectDir, "styles", "globals.css"), "utf8");
  const htmlBodyRules = css.match(/(?:^|\n)(?:html|body)\s*\{[\s\S]*?\}/g) ?? [];
  const bodyNativeRule =
    css.match(/body\[data-dx-scroll-system="native"\]\s*\{[\s\S]*?\}/)?.[0] ?? "";
  const mobileMenuTriggerRules = [
    ...cssRules(css, ".template-mobile-menu"),
    ...cssRules(css, ".dashboard-mobile-menu"),
  ];
  const mobileMenuPanelRules = [
    ...cssRules(css, ".template-mobile-menu nav"),
    ...cssRules(css, ".dashboard-mobile-sheet"),
  ];
  const routes = nativeScrollProofRoutes.map(({ route, sourceFile }) => {
    const html = fs.readFileSync(path.join(projectDir, sourceFile), "utf8");
    const htmlTag = firstTag(html, "html");
    const bodyTag = firstTag(html, "body");
    const mainTag = firstTag(html, "main");
    const mainContentBytes = tagInnerHtml(html, "main").replace(/\s+/g, " ").trim().length;
    const mobileMenuTag =
      html.match(/<(?:details|div)\b[^>]*class="[^"]*(?:template-mobile-menu|dashboard-mobile-menu)[^"]*"[^>]*>/)?.[0] ??
      "";
    const hasMobileMenu = mobileMenuTag.length > 0;

    return {
      route,
      sourceFile,
      bodyScrollRoot: tagHasAttribute(bodyTag, "data-dx-scroll-root", "document"),
      bodyScrollSystem: tagHasAttribute(bodyTag, "data-dx-scroll-system", "native"),
      mainScrollSurface: tagHasAttribute(mainTag, "data-dx-scroll-surface", "document"),
      mainScrollLock: tagHasAttribute(mainTag, "data-dx-scroll-lock", "none"),
      wheelScroll: tagHasAttribute(mainTag, "data-dx-wheel-scroll", "native"),
      documentFlowProof: tagHasAttribute(
        mainTag,
        "data-dx-scroll-proof",
        "document-flow-no-lock",
      ),
      viewportPlusContent: tagHasAttribute(mainTag, "data-dx-scroll-content", "viewport-plus"),
      documentFlowContent: mainContentBytes > 400,
      mainContentBytes,
      inlineScrollLock: tagHasInlineScrollLock(htmlTag) || tagHasInlineScrollLock(bodyTag),
      classScrollLock: tagHasScrollLockClass(htmlTag) || tagHasScrollLockClass(bodyTag),
      dataScrollLock: tagHasScrollLockDataAttribute(htmlTag) || tagHasScrollLockDataAttribute(bodyTag),
      customScrollbarRuntime: hasCustomScrollbarSurface(html),
      mobileMenuScrollTrap: hasMobileMenu
        ? !tagHasAttribute(mobileMenuTag, "data-dx-scroll-trap", "false")
        : null,
    };
  });

  const cssContract = {
    bodyNativeOverflowYAuto: /overflow-y:\s*auto/.test(bodyNativeRule),
    htmlBodyOverflowDisabled: htmlBodyRules.some((rule) =>
      /overflow(?:-y)?:\s*(?:hidden|clip)\b|position:\s*fixed\b/.test(rule),
    ),
    customScrollbarCss: hasCustomScrollbarSurface(css),
    documentHeightPolicy: "min-height-100vh-no-body-lock",
    mobileMenuTriggerPositionFixed: hasFixedPosition(mobileMenuTriggerRules),
    mobileMenuPanelsPositionFixed: hasFixedPosition(mobileMenuPanelRules),
    mobileMenuPanelsScrollable:
      mobileMenuPanelRules.length >= 2 &&
      mobileMenuPanelRules.every(
        (rule) => /max-height:\s*min\(/.test(rule) && /overflow-y:\s*auto/.test(rule),
      ),
  };
  const routesWithNativeDocumentFlow = routes.filter(
    (route) =>
      route.bodyScrollRoot &&
      route.bodyScrollSystem &&
      route.mainScrollSurface &&
      route.mainScrollLock &&
      route.wheelScroll &&
      route.documentFlowProof,
  ).length;
  const routesWithDocumentFlowContent = routes.filter(
    (route) => route.documentFlowContent && route.viewportPlusContent,
  ).length;
  const routesWithoutDocumentRootScrollLock = routes.filter(
    (route) => !route.inlineScrollLock && !route.classScrollLock && !route.dataScrollLock,
  ).length;
  const routesWithoutCustomScrollbarRuntime = routes.filter(
    (route) => !route.customScrollbarRuntime,
  ).length;
  const routesWithoutScrollTrap = routes.filter(
    (route) => route.mobileMenuScrollTrap !== true,
  ).length;

  return {
    schema: "dx.template.native_scroll_source_guard",
    sourceGuard: true,
    browserRuntimeProof: false,
    routesChecked: routes.length,
    coverage: {
      expectedRoutes: nativeScrollProofRoutes.map((route) => route.route),
      routesCovered: routes.length,
      routesWithNativeDocumentFlow,
      routesWithDocumentFlowContent,
      routesWithoutDocumentRootScrollLock,
      routesWithoutCustomScrollbarRuntime,
      routesWithoutScrollTrap,
      allRoutesNativeDocumentFlow: routesWithNativeDocumentFlow === routes.length,
      allRoutesHaveDocumentFlowContent: routesWithDocumentFlowContent === routes.length,
      allRoutesDocumentRootUnlocked: routesWithoutDocumentRootScrollLock === routes.length,
      allRoutesCustomScrollbarFree: routesWithoutCustomScrollbarRuntime === routes.length,
      allMobileMenusNonTrapping: routesWithoutScrollTrap === routes.length,
      cssDocumentRootUnlocked:
        cssContract.bodyNativeOverflowYAuto &&
        !cssContract.htmlBodyOverflowDisabled &&
        !cssContract.customScrollbarCss,
      sourceOnlyProof: true,
      browserRuntimeProof: false,
    },
    cssContract,
    routes,
  };
}

function renderTemplateDashboardRuntimePage() {
  const initialProjectFilter = "all";
  const visibleProjectCount = dashboardProjects.length;
  const physicalCacheManifestCount = countPhysicalCacheManifests();
  const stalePhysicalCacheManifestCount = Math.max(
    0,
    physicalCacheManifestCount - forgeRealitySummary.currentCacheManifestCount,
  );
  const queryCacheStatus = createDashboardQueryCacheStatus({
    optimisticState: "idle",
    lastReceiptState: "Local cache ready",
    filter: initialProjectFilter,
    visibleProjectCount,
  });
  const reactiveStoreReadiness = createDashboardReactiveStoreReadiness({
    activeModule: "reactive-context",
    filter: initialProjectFilter,
    optimisticState: "idle",
    queryCacheStatus: queryCacheStatus.status,
    theme: "dark",
    visibleProjectCount,
  });
  const navLinks = dashboardNavItems
    .map((item, index) => {
      const id = item.toLowerCase();
      const current = index === 0 ? ' aria-current="page"' : "";
      return `<a href="#${id}"${current}>${renderDxIcon(dashboardNavIconNames[item], "dx-icon dashboard-nav-icon")}<span>${escapeHtml(item)}</span></a>`;
    })
    .join("\n          ");
  const projectRows = dashboardProjects
    .map(
      (project) => `<div role="row" data-project-state="${escapeHtml(project.state)}">
                <strong role="cell">${escapeHtml(project.name)}</strong>
                <span role="cell">${escapeHtml(project.owner)}</span>
                <span role="cell">${escapeHtml(project.state)}</span>
              </div>`,
    )
    .join("\n              ");
  const reportRows = dashboardReports
    .map(
      (report) => `<div><span>${escapeHtml(report.label)}</span><strong>${report.value}%</strong><div class="template-progress"><span style="width: ${report.value}%"></span></div></div>`,
    )
    .join("\n                ");
  const realControls = interactiveForgePackageRows
    .map(
      (row, index) => `<article class="package-module forge-reality-control" data-dx-package-id="${escapeHtml(row.packageId)}" data-dx-package-control="source-owned-template-control" data-dx-package-maturity="${escapeHtml(row.maturityKind)}" data-template-module="${escapeHtml(row.controlId ?? row.packageId)}" data-module-active="${index === 1 ? "true" : "false"}" style="--module-index: ${index}">
                <strong>${renderDxIcon(row.iconName)}<span>${escapeHtml(row.controlTitle ?? row.packageName)}</span></strong>
                <span>${escapeHtml(row.templateUsage)}</span>
                <div class="forge-reality-badges"><span class="forge-reality-level" data-dx-forge-reality-level="${escapeHtml(row.realityLevelId)}">${escapeHtml(publicReadinessLabel(row))}</span><span class="forge-maturity-level" data-dx-package-maturity="${escapeHtml(row.maturityKind)}">${escapeHtml(row.maturityLabel)}</span></div>
                <footer><button class="template-button" type="button" data-template-module-action="${escapeHtml(row.controlId ?? row.packageId)}">${renderDxIcon("action:open")}<span>${escapeHtml(row.controlAction ?? "Review")}</span></button><output data-template-module-status>${escapeHtml(row.initialStatus ?? "Ready")}</output></footer>
              </article>`,
    )
    .join("\n              ");
  const providerReadinessCards = providerGatedReadinessRows
    .map(
      (provider) => `<article class="package-module forge-reality-control provider-gated-card lane5-provider-card" data-dx-package-maturity="provider-gated" data-dx-package-id="${escapeHtml(provider.packageId)}" data-dx-provider-endpoint="${escapeHtml(provider.endpoint)}" data-dx-provider-boundary-endpoints="${escapeHtml(provider.boundaryEndpoints.join(" "))}" data-dx-provider-method="${escapeHtml(provider.method)}" data-dx-provider-boundary="${escapeHtml(provider.boundary)}" data-dx-provider-runtime-execution="${escapeHtml(provider.runtimeEvidence)}" data-dx-provider-secret-values="${escapeHtml(provider.secretEvidence)}" data-dx-template-readiness-receipt="${escapeHtml(provider.readinessReceipt)}" data-dx-runtime-proof="false">
                <strong>${renderDxIcon(provider.iconName)}<span>${escapeHtml(provider.packageName)}</span></strong>
                <span>${escapeHtml(provider.summary)}</span>
                <small>${escapeHtml(provider.endpoint)}</small>
                <small>${escapeHtml(provider.boundary)}</small>
                <footer><span class="forge-reality-level" data-dx-forge-reality-level="provider-boundary">Provider-boundary</span><output>${escapeHtml(provider.statusLabel)}</output></footer>
              </article>`,
    )
    .join("\n                ");
  const maturitySummaryCards = [
    {
      kind: "lock-backed-package",
      label: "Ready modules",
      count: forgeRealitySummary.lockBackedPackageMaturityCount,
      description: "Controls that are usable without provider setup.",
    },
    {
      kind: "provider-gated",
      label: "Provider setup",
      count: forgeRealitySummary.providerGatedCount,
      description: "Credentials are required before live provider execution.",
    },
    {
      kind: "adapter-boundary-readiness",
      label: "Adapter boundary",
      count: forgeRealitySummary.adapterBoundaryReadinessCount,
      description: "Readiness is visible while app-owned adapters remain gated.",
    },
    {
      kind: "source-owned-limited-proof",
      label: "Source-ready modules",
      count: forgeRealitySummary.sourceOwnedLimitedProofCount,
      description: "Editable files are present; browser evidence is pending.",
    },
    {
      kind: "source-guard-only",
      label: "Guarded source",
      count: forgeRealitySummary.sourceGuardOnlyCount,
      description: "Guarded source evidence only, with no active package lane.",
    },
  ]
    .map(
      (row) => `<article class="forge-maturity-card" data-dx-package-maturity-kind="${escapeHtml(row.kind)}" data-dx-package-maturity-count="${row.count}">
                <strong>${escapeHtml(row.label)}</strong>
                <span>${escapeHtml(row.description)}</span>
                <output>${row.count}</output>
              </article>`,
    )
    .join("\n              ");
  const launchEvidenceSummaryCards = launchEvidenceSummaryRows
    .map((row) => {
      const routes = "routes" in row ? row.routes.join(" ") : "";
      const packageIds = "packageIds" in row ? row.packageIds.join(" ") : "";

      return `<article class="package-module forge-reality-control" data-dx-launch-evidence-id="${escapeHtml(row.id)}" data-dx-launch-evidence-status="${escapeHtml(row.status)}" data-dx-launch-evidence-score-ceiling="${forgeRealitySummary.scoreGate.ceilingWithoutLiveProof}" data-dx-launch-evidence-browser-proof="${forgeRealitySummary.scoreGate.browserRuntimeProof}" data-dx-launch-evidence-provider-proof="${forgeRealitySummary.scoreGate.liveProviderProof}" data-dx-launch-evidence-routes="${escapeHtml(routes)}" data-dx-launch-evidence-packages="${escapeHtml(packageIds)}">
                <strong>${renderDxIcon(row.iconName)}<span>${escapeHtml(row.label)}</span></strong>
                <span>${escapeHtml(row.description)}</span>
                <small>${escapeHtml(row.scoreImpact)}</small>
                <footer><span class="forge-reality-level">${escapeHtml(row.statusLabel)}</span><output>${escapeHtml(row.value)}</output></footer>
              </article>`;
    })
    .join("\n              ");
  const realityRows = forgeRealityRows
    .map(
      (row) => `<div class="forge-reality-row" role="row" data-dx-package-maturity="${escapeHtml(row.maturityKind)}" data-dx-package-id="${escapeHtml(row.packageId)}" data-dx-forge-status-row="${row.realityLevelId !== "real-lock-backed"}" data-dx-forge-reality-level="${escapeHtml(row.realityLevelId)}">
                <span role="cell"><strong>${renderDxIcon(row.iconName)}${escapeHtml(row.packageName)}</strong><small>${escapeHtml(row.packageId)}</small><small>${escapeHtml(row.upstreamPackage)}</small></span>
                <span role="cell"><div class="forge-reality-badges"><span class="forge-reality-level" data-dx-forge-reality-level="${escapeHtml(row.realityLevelId)}">${escapeHtml(publicReadinessLabel(row))}</span><span class="forge-maturity-level" data-dx-package-maturity="${escapeHtml(row.maturityKind)}">${escapeHtml(row.maturityLabel)}</span></div></span>
                <span role="cell">${escapeHtml(row.runtimeProof)}</span>
                <span role="cell" class="forge-reality-score" data-dx-score-scope="package-readiness-row" data-dx-package-score="${row.score}">${row.score}/100</span>
                <span role="cell">${escapeHtml(row.missingToReach90)}</span>
              </div>`,
    )
    .join("\n              ");
  const staleCacheManifestPaths =
    forgeRealitySummary.stalePhysicalCacheManifestPaths ?? [];
  const staleCacheManifestPathAttr = staleCacheManifestPaths
    .map((manifestPath) => escapeHtml(manifestPath))
    .join(" ");
  const archivedCacheManifestPaths = listCacheArchiveManifestPaths();
  const archivedCacheManifestCount = archivedCacheManifestPaths.length;
  const archivedCacheManifestPathAttr = archivedCacheManifestPaths
    .map((manifestPath) => escapeHtml(manifestPath))
    .join(" ");
  const staleCacheManifestList =
    staleCacheManifestPaths.length > 0
      ? `<ul class="forge-reality-stale-cache-list" data-dx-stale-cache-manifest-list>${staleCacheManifestPaths
          .map((manifestPath) => `<li>${escapeHtml(manifestPath)}</li>`)
          .join("")}</ul>`
      : "";
  const archivedCacheManifestList =
    archivedCacheManifestPaths.length > 0
      ? `<ul class="forge-reality-stale-cache-list" data-dx-cache-archive-manifest-list>${archivedCacheManifestPaths
          .map((manifestPath) => `<li>${escapeHtml(manifestPath)}</li>`)
          .join("")}</ul>`
      : "";
  const scoreComponentList = `<ul class="forge-score-component-list" data-dx-forge-score-components>${forgeRealitySummary.scoreComponents
    .map(
      (component) =>
        `<li data-dx-forge-score-component="${escapeHtml(component.id)}"><strong>${escapeHtml(component.label)}</strong><span>${escapeHtml(component.evidence)}</span><output>${component.points}</output></li>`,
    )
    .join("")}</ul>`;
  const scoreGateList = `<div class="forge-score-gate-list" data-dx-forge-score-gate="${escapeHtml(forgeRealitySummary.scoreGate.schema)}" data-dx-forge-score-gate-current="${forgeRealitySummary.scoreGate.currentScore}" data-dx-forge-score-gate-target="${forgeRealitySummary.scoreGate.targetScore}" data-dx-forge-score-gate-ceiling="${forgeRealitySummary.scoreGate.ceilingWithoutLiveProof}" data-dx-forge-score-gate-browser-proof="${forgeRealitySummary.scoreGate.browserRuntimeProof}" data-dx-forge-score-gate-provider-proof="${forgeRealitySummary.scoreGate.liveProviderProof}"><strong>Launch score gate</strong><span>${escapeHtml(forgeRealitySummary.scoreGate.honesty)}</span><ul>${forgeRealitySummary.scoreGate.requiredProofs
    .map((proof) => {
      const routes = "routes" in proof ? proof.routes.join(" ") : "";
      const packageIds = "packageIds" in proof ? proof.packageIds.join(" ") : "";
      return `<li data-dx-forge-score-gate-proof="${escapeHtml(proof.id)}" data-dx-forge-score-gate-proof-status="${escapeHtml(proof.status)}" data-dx-forge-score-gate-proof-routes="${escapeHtml(routes)}" data-dx-forge-score-gate-proof-packages="${escapeHtml(packageIds)}"><strong>${escapeHtml(proof.label)}</strong><span>${escapeHtml(proof.requiredEvidence)}</span></li>`;
    })
    .join("")}</ul></div>`;
  const readinessExecutionProofList = `<div class="forge-reality-route-proof-list" data-dx-component="template-readiness-execution-proof" data-dx-runtime-proof="false" data-dx-live-provider-execution="false"><strong>Route helper proof</strong><span>Focused tests execute source-owned readiness helpers. Browser runtime and live provider execution remain gated.</span><ul>${templateReadinessExecutionProofRows
    .map(
      (row) =>
        `<li data-dx-readiness-execution-route="${escapeHtml(row.route)}" data-dx-readiness-execution-packages="${escapeHtml(row.packageIds.join(" "))}" data-dx-readiness-execution-http-status="${escapeHtml(row.httpStatus.join(" "))}" data-dx-readiness-execution-proof="${escapeHtml(row.proofKind)}" data-dx-runtime-proof="false" data-dx-live-provider-execution="false" data-dx-network-calls="false" data-dx-secret-values="[]"><strong>${escapeHtml(row.route)}</strong><span>${escapeHtml(row.boundary)}</span><small>${escapeHtml(row.exercisedBy)}</small></li>`,
    )
    .join("")}</ul></div>`;

  return `<!DOCTYPE html>
<html lang="en" data-theme="dark">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Dashboard | www</title>
    <link rel="icon" href="/favicon.svg" type="image/svg+xml" />
    <link rel="stylesheet" href="/styles/globals.css" />
    <script src="/public/launch-runtime.js" defer></script>
  </head>
  <body
    class="template-app-body"
    data-dx-scroll-root="document"
    data-dx-scroll-system="native"
  >
    <!-- Generated from examples/template/components/template-app/dashboard-page.tsx and template-data.ts. -->
    <main
      id="dashboard-scroll-content"
      class="dashboard-shell"
      data-dx-route="/dashboard"
      data-dx-hot-reload-target="route:/dashboard"
      data-dx-source="examples/template/components/template-app/dashboard-page.tsx"
      data-dx-component="template-dashboard-page"
      data-dx-package="${dashboardForgePackageIdList}"
      data-dx-style-surface="template-dashboard"
      data-dx-check-surface="template-dashboard"
      data-dx-forge-reality-score="${forgeRealitySummary.score}"
      data-dx-forge-score-ceiling="${forgeRealitySummary.scoreGate.ceilingWithoutLiveProof}"
      data-dx-forge-unbounded-source-score="${forgeRealitySummary.scoreGate.unboundedSourceScore}"
      data-dx-forge-score-can-exceed-ceiling="${forgeRealitySummary.scoreGate.canExceedCeiling ? "true" : "false"}"
      data-dx-forge-lock-backed-package-count="${forgeRealitySummary.lockBackedPackageCount}"
      data-dx-forge-current-cache-manifest-count="${forgeRealitySummary.currentCacheManifestCount}"
      data-dx-forge-current-lock-backed-manifests="${forgeRealitySummary.currentLockBackedManifests}"
      data-dx-forge-physical-cache-manifest-count="${physicalCacheManifestCount}"
      data-dx-forge-physical-cache-manifests="${physicalCacheManifestCount}"
      data-dx-forge-stale-physical-cache-manifest-count="${stalePhysicalCacheManifestCount}"
      data-dx-forge-stale-physical-cache-manifests="${stalePhysicalCacheManifestCount}"
      data-dx-forge-stale-cache-manifest-paths="${staleCacheManifestPathAttr}"
      data-dx-forge-cache-manifest-source="${forgeRealitySummary.cacheManifestSource}"
      data-dx-forge-cache-manifest-caveat="${forgeRealitySummary.cacheManifestCaveatId}"
      data-dx-forge-cache-archive-root="${escapeHtml(forgeRealitySummary.cacheArchiveRoot)}"
      data-dx-forge-cache-archive-manifest-count="${archivedCacheManifestCount}"
      data-dx-forge-cache-archive-manifest-paths="${archivedCacheManifestPathAttr}"
      data-dx-forge-cache-archive-caveat="${forgeRealitySummary.cacheArchiveCaveatId}"
      data-dx-forge-real-control-count="${forgeRealitySummary.realControlCount}"
      data-dx-forge-real-lock-backed-count="${forgeRealitySummary.realLockBackedControlCount}"
      data-dx-forge-interactive-surface-count="${forgeRealitySummary.realControlCount}"
      data-dx-forge-status-lane-count="${forgeRealitySummary.statusOnlyPackageCount}"
      data-dx-lane-three-state="state-data-fetching"
      data-dx-dashboard-filter-state="${initialProjectFilter}"
      data-dx-optimistic-ui-state="idle"
      data-dx-reactive-store-runtime="${reactiveStoreReadiness.runtimeBoundary}"
      data-dx-reactive-store-provider-boundary="${reactiveStoreReadiness.providerBoundary}"
      data-dx-reactive-store-snapshot-key="${reactiveStoreReadiness.snapshotKey}"
      data-dx-reactive-store-subscription="${reactiveStoreReadiness.subscriptionState}"
      data-dx-reactive-store-visible-count="${reactiveStoreReadiness.visibleProjectCount}"
      data-dx-query-cache-status="${queryCacheStatus.status}"
      data-dx-query-cache-runtime="${queryCacheStatus.runtimeBoundary}"
      data-dx-query-cache-key="${queryCacheStatus.queryKey}"
      data-dx-query-cache-readiness-route="/api/query-cache/readiness"
      data-dx-query-cache-action-state="no-cache-action-requested"
      data-dx-query-cache-last-action="none"
      data-dx-query-cache-ready-count="${queryCacheStatus.readyEntryCount}"
      data-dx-query-cache-invalidated-count="${queryCacheStatus.invalidatedEntryCount}"
      data-dx-query-cache-optimistic-count="${queryCacheStatus.optimisticEntryCount}"
      data-dx-tanstack-query-runtime-boundary="${queryCacheStatus.upstreamAdapterBoundary}"
      data-dx-theme="dark"
      data-dx-auth-state="unknown"
      data-dx-state-storage-key="dx-template-workspace-state"
      data-dx-form-validation="idle"
      data-dx-node-modules="forbidden"
      data-dx-ui-density="comfortable"
      data-dx-locale="en"
      data-dx-native-scroll="page"
      data-dx-scroll-surface="document"
      data-dx-scroll-lock="none"
      data-dx-scroll-proof="document-flow-no-lock"
      data-dx-wheel-scroll="native"
    >
      <header class="dashboard-header" data-dx-component="template-dashboard-header">
        <a class="template-brand" href="/" aria-label="www home">
          <span class="template-brand-mark" aria-hidden="true">${renderDxIcon("pack:workspace")}</span>
          <span>www</span>
        </a>
        <nav class="dashboard-header-nav" aria-label="Primary dashboard navigation">
          ${navLinks}
        </nav>
        <div class="dashboard-header-actions">
          <a class="template-button" href="/login">${renderDxIcon("action:login")}<span>Sign in</span></a>
          <a class="template-button template-button-primary" href="/logout">${renderDxIcon("action:logout")}<span>Clear review</span></a>
        </div>
        <details
          class="dashboard-mobile-menu"
          data-dx-mobile-menu="native-details"
          data-dx-mobile-menu-scroll-lock="none"
          data-dx-scroll-trap="false"
        >
          <summary class="dashboard-mobile-toggle" aria-label="Open navigation">${renderDxIcon("action:menu")}<span>Menu</span></summary>
          <div class="dashboard-mobile-sheet">
            <nav class="dashboard-mobile-nav" aria-label="Mobile dashboard navigation">
              ${navLinks}
            </nav>
            <div class="dashboard-mobile-actions">
              <a class="template-button" href="/login">${renderDxIcon("action:login")}<span>Sign in</span></a>
              <a class="template-button template-button-primary" href="/logout">${renderDxIcon("action:logout")}<span>Clear review</span></a>
            </div>
          </div>
        </details>
      </header>

      <div class="dashboard-workspace">
        <section class="dashboard-main">
          <header class="dashboard-topbar">
            <div>
              <p class="template-kicker">Today</p>
              <h1>Launch readiness</h1>
            </div>
            <div class="dashboard-account">
              <span data-app-auth-label>Sign-in readiness</span>
              <strong data-app-session-email>No hosted session</strong>
            </div>
          </header>

          <section
            class="template-card dashboard-panel"
            data-dx-component="template-auth-session-panel"
            data-dx-package="auth/better-auth"
            data-dx-auth-config="missing-config"
            data-dx-auth-session-source="missing-config"
            data-dx-auth-profile-gate="missing-config"
            data-dx-auth-provider="google"
            data-dx-auth-readiness-endpoint="/api/auth/readiness"
            data-dx-auth-missing-config="BETTER_AUTH_SECRET,BETTER_AUTH_URL,GOOGLE_CLIENT_ID,GOOGLE_CLIENT_SECRET"
          >
            <div class="dashboard-panel-header">
              <div>
                <h2>Authentication</h2>
                <p>Authentication source is present; login, logout, session, and profile actions stay gated until app secrets exist.</p>
              </div>
              <a class="template-button" href="/login">${renderDxIcon("action:login")}<span>Open login</span></a>
            </div>
            <div class="activity-list">
              <div><strong>Google provider</strong><span>Configured as the only selected OAuth plugin surface.</span></div>
              <div><strong>Profile gate</strong><span>No hosted session or account mutation is claimed while config is missing.</span></div>
            </div>
          </section>

          <section id="overview" class="dashboard-grid" data-dx-component="template-dashboard-overview">
            <article class="template-card stat-card"><span class="stat-card-title">${renderDxIcon("status:check")}<span>Launch readiness</span></span><strong id="dashboard-readiness">${forgeRealitySummary.score}%</strong><p>Current readiness across the workspace modules.</p></article>
            <article class="template-card stat-card"><span class="stat-card-title">${renderDxIcon("status:list-checks")}<span>Package set</span></span><strong id="dashboard-project-count">${forgeRealitySummary.lockBackedPackageCount}/${forgeRealitySummary.totalVisiblePackageLanes}</strong><p>The installed package set is aligned; live integrations stay gated.</p></article>
            <article class="template-card stat-card"><span class="stat-card-title">${renderDxIcon("nav:team")}<span>Template surfaces</span></span><strong id="dashboard-profile-state">${forgeRealitySummary.realControlCount}</strong><p id="dashboard-profile-detail">Interactive surfaces with current template evidence.</p></article>
          </section>

          ${renderLane7ForgeSystems()}

          <section id="tasks" class="template-card dashboard-panel" data-dx-component="template-project-panel">
            <div class="dashboard-panel-header">
              <div><h2>Task queue</h2><p>Filter real dashboard rows and keep priority items visible.</p></div>
              <div class="template-segmented" role="group" aria-label="Project filter">
                <button type="button" data-dashboard-filter="all" aria-pressed="true">All</button>
                <button type="button" data-dashboard-filter="ready" aria-pressed="false">Ready</button>
                <button type="button" data-dashboard-filter="review" aria-pressed="false">Review</button>
              </div>
            </div>
            <div class="priority-row"><div><strong>Dashboard polish</strong><p>Keep the next release focused on ready modules and gated integrations.</p></div><button class="template-button template-button-primary" type="button" data-template-module-action="query-refresh">${renderDxIcon("action:rocket")}<span>Refresh status</span></button></div>
            <div class="project-table" role="table" aria-label="Project queue">
              <div role="row" class="project-table-head"><span role="columnheader">Project</span><span role="columnheader">Owner</span><span role="columnheader">State</span></div>
              ${projectRows}
            </div>
          </section>

          <section class="dashboard-split">
            <article id="team" class="template-card dashboard-panel">
              <div><h2>Team rhythm</h2><p>A short operating view for the next standup.</p></div>
              <div class="activity-list">
                <div><strong>Design review</strong><span>Header, footer, and mobile navigation are ready for review.</span></div>
                <div><strong>API contract</strong><span>Dashboard actions return clear states without leaving the page.</span></div>
                <div><strong>Mobile QA</strong><span>Responsive sheets, filters, and forms stay reachable on small screens.</span></div>
              </div>
            </article>
            <article id="reports" class="template-card dashboard-panel">
              <h2>Weekly signal</h2>
              <div class="report-list">
                ${reportRows}
              </div>
            </article>
          </section>

          <section
            id="state"
            class="template-card dashboard-panel lane-three-state-panel"
            data-dx-component="lane-three-state-data-panel"
            data-dx-lane-three-state="state-data-fetching"
            data-dx-package="state/zustand reactive/store tanstack/query"
            data-dx-reactive-store-provider="dashboard-context"
            data-dx-reactive-store-runtime="${reactiveStoreReadiness.runtimeBoundary}"
            data-dx-reactive-store-provider-boundary="${reactiveStoreReadiness.providerBoundary}"
            data-dx-reactive-store-snapshot-key="${reactiveStoreReadiness.snapshotKey}"
            data-dx-reactive-store-subscription="${reactiveStoreReadiness.subscriptionState}"
            data-dx-reactive-store-visible-count="${reactiveStoreReadiness.visibleProjectCount}"
            data-dx-query-cache-status="${queryCacheStatus.status}"
            data-dx-query-cache-runtime="${queryCacheStatus.runtimeBoundary}"
            data-dx-query-cache-key="${queryCacheStatus.queryKey}"
            data-dx-query-cache-readiness-route="/api/query-cache/readiness"
            data-dx-query-cache-action-state="no-cache-action-requested"
            data-dx-query-cache-last-action="none"
            data-dx-query-cache-ready-count="${queryCacheStatus.readyEntryCount}"
            data-dx-query-cache-invalidated-count="${queryCacheStatus.invalidatedEntryCount}"
            data-dx-query-cache-optimistic-count="${queryCacheStatus.optimisticEntryCount}"
            data-dx-tanstack-query-runtime-boundary="${queryCacheStatus.upstreamAdapterBoundary}"
            data-dx-dashboard-filter-state="${initialProjectFilter}"
            data-dx-optimistic-ui-state="idle"
          >
            <div class="dashboard-panel-header">
              <div><h2>State and cache readiness</h2><p>Dashboard state, reactive snapshots, and cache readiness are visible without hosted services.</p></div>
              <output id="lane-three-cache-meter" class="lane-three-cache-meter">${queryCacheStatus.readyEntryCount}/${queryCacheStatus.cacheEntryCount}</output>
            </div>
            <div class="lane-three-state-grid">
              <div>${renderDxIcon("pack:state")}<strong>Dashboard filter</strong><span id="lane-three-filter">${initialProjectFilter}</span></div>
              <div>${renderDxIcon("state:bolt")}<strong>Reactive snapshot</strong><span id="lane-three-visible-rows">${visibleProjectCount} visible rows</span></div>
              <div>${renderDxIcon("pack:query")}<strong>Query cache</strong><span id="lane-three-query-label">${escapeHtml(queryCacheStatus.readinessLabel)}</span></div>
              <div>${renderDxIcon("status:check")}<strong>Optimistic update</strong><span id="lane-three-optimistic">No optimistic receipt queued</span></div>
            </div>
          </section>

          <section id="tools" class="template-card dashboard-panel package-module-board" data-dx-component="forge-package-reality-dashboard-shell" data-dx-package="${dashboardForgePackageIdList}">
            <section class="forge-reality-dashboard" data-dx-component="forge-package-reality-dashboard" data-dx-forge-reality-score="${forgeRealitySummary.score}" data-dx-forge-lock-backed-package-count="${forgeRealitySummary.lockBackedPackageCount}" data-dx-forge-current-cache-manifest-count="${forgeRealitySummary.currentCacheManifestCount}" data-dx-forge-current-lock-backed-manifests="${forgeRealitySummary.currentLockBackedManifests}" data-dx-forge-physical-cache-manifest-count="${physicalCacheManifestCount}" data-dx-forge-physical-cache-manifests="${physicalCacheManifestCount}" data-dx-forge-stale-physical-cache-manifest-count="${stalePhysicalCacheManifestCount}" data-dx-forge-stale-physical-cache-manifests="${stalePhysicalCacheManifestCount}" data-dx-forge-stale-cache-manifest-paths="${staleCacheManifestPathAttr}" data-dx-forge-cache-manifest-source="${forgeRealitySummary.cacheManifestSource}" data-dx-forge-cache-manifest-caveat="${forgeRealitySummary.cacheManifestCaveatId}" data-dx-forge-cache-archive-root="${escapeHtml(forgeRealitySummary.cacheArchiveRoot)}" data-dx-forge-cache-archive-manifest-count="${archivedCacheManifestCount}" data-dx-forge-cache-archive-manifest-paths="${archivedCacheManifestPathAttr}" data-dx-forge-cache-archive-caveat="${forgeRealitySummary.cacheArchiveCaveatId}" data-dx-forge-real-control-count="${forgeRealitySummary.realControlCount}" data-dx-forge-real-lock-backed-count="${forgeRealitySummary.realLockBackedControlCount}" data-dx-forge-interactive-surface-count="${forgeRealitySummary.realControlCount}" data-dx-forge-status-lane-count="${forgeRealitySummary.statusOnlyPackageCount}" data-dx-forge-provider-gated-count="${forgeRealitySummary.providerGatedCount}" data-dx-forge-provider-boundary-coverage="${forgeRealitySummary.providerBoundaryCoverage}" data-dx-forge-readiness-execution-proof-count="${forgeRealitySummary.readinessExecutionProofCount}" data-dx-forge-readiness-execution-proof-package-count="${forgeRealitySummary.readinessExecutionProofPackageCount}" data-dx-forge-readiness-execution-proof-packages="${escapeHtml(forgeRealitySummary.readinessExecutionProofPackageIds.join(" "))}" data-dx-forge-adapter-boundary-readiness-count="${forgeRealitySummary.adapterBoundaryReadinessCount}" data-dx-forge-source-owned-limited-proof-count="${forgeRealitySummary.sourceOwnedLimitedProofCount}" data-dx-forge-source-guard-only-count="${forgeRealitySummary.sourceGuardOnlyCount}" data-dx-forge-public-summary-first="true" data-dx-forge-audit-details-default="collapsed">
              <div class="forge-reality-summary" aria-label="Package readiness summary">
                <div><span>Launch readiness</span><strong>${forgeRealitySummary.score}/100</strong></div>
                <div><span>Package set</span><strong>${forgeRealitySummary.lockBackedPackageCount}/${forgeRealitySummary.totalVisiblePackageLanes}</strong></div>
                <div><span>Current packages</span><strong>${forgeRealitySummary.currentCacheManifestCount}</strong></div>
                <div><span>Interactive surfaces</span><strong>${forgeRealitySummary.realControlCount}</strong></div>
                <div><span>Setup required</span><strong>${forgeRealitySummary.providerGatedCount}</strong></div>
                <div><span>Runtime model</span><strong>${forgeRealitySummary.noNodeModulesRequired ? "App source" : "Adapter required"}</strong></div>
                <div><span>Route checks</span><strong>${forgeRealitySummary.readinessExecutionProofPackageCount}</strong></div>
              </div>
              <div class="forge-maturity-strip" aria-label="Package maturity summary" data-dx-component="forge-package-maturity-summary" data-dx-package-maturity-summary="visible">
                ${maturitySummaryCards}
              </div>
              <div class="forge-reality-controls" data-dx-component="launch-evidence-summary" data-dx-launch-evidence-score-ceiling="${forgeRealitySummary.scoreGate.ceilingWithoutLiveProof}" data-dx-launch-evidence-browser-proof="${forgeRealitySummary.scoreGate.browserRuntimeProof}" data-dx-launch-evidence-provider-proof="${forgeRealitySummary.scoreGate.liveProviderProof}">
                ${launchEvidenceSummaryCards}
              </div>
              <div class="dashboard-panel-header"><div><h2>Workspace controls</h2><p>Controls ready in this template are active here. Setup-gated integrations stay visible in details.</p></div></div>
              <div class="forge-reality-note provider-gated-readiness lane5-provider-readiness" data-dx-component="provider-gated-readiness" data-dx-provider-readiness="all-provider-gated-packages" data-dx-lane5-provider-readiness="payments-ai-automations" data-dx-provider-runtime-proof="false" data-dx-provider-secret-values="[]">
                <strong>Provider readiness</strong>
                <span>Authentication, data, payments, AI, and automations expose setup checks and dry-run states. Live provider work starts only after app-owned credentials are configured.</span>
                <div class="forge-reality-controls">
                  ${providerReadinessCards}
                </div>
              </div>
              <div class="forge-reality-controls">
                ${realControls}
              </div>
              <details class="forge-reality-audit-details" data-dx-component="forge-package-maturity-details" data-dx-forge-audit-details-default="collapsed">
                <summary>Package readiness details</summary>
                <div class="dashboard-panel-header"><div><h2>Integration readiness details</h2><p>Credentialed providers and browser checks remain gated until the app supplies evidence.</p></div></div>
                <div class="forge-reality-table" role="table" aria-label="Forge package reality table">
                  <div class="forge-reality-row forge-reality-row-head" role="row"><span role="columnheader">Package</span><span role="columnheader">Readiness</span><span role="columnheader">Evidence</span><span role="columnheader">Score</span><span role="columnheader">Next proof needed</span></div>
                  ${realityRows}
                </div>
                <div class="forge-reality-note" data-dx-provider-gated-count="${forgeRealitySummary.providerGatedCount}" data-dx-adapter-boundary-readiness-count="${forgeRealitySummary.adapterBoundaryReadinessCount}" data-dx-source-owned-limited-proof-count="${forgeRealitySummary.sourceOwnedLimitedProofCount}" data-dx-source-guard-only-count="${forgeRealitySummary.sourceGuardOnlyCount}"><strong>All ${forgeRealitySummary.totalVisiblePackageLanes} packages have visible template surfaces.</strong><span>${forgeRealitySummary.providerGatedCount} provider integrations still need credentials. Adapter-boundary and source-proof packages stay labeled until browser or live-provider evidence lands. ${escapeHtml(forgeRealitySummary.cacheManifestCaveat)} ${escapeHtml(forgeRealitySummary.cacheArchiveCaveat)}</span>${archivedCacheManifestList}${scoreComponentList}${scoreGateList}${readinessExecutionProofList}${staleCacheManifestList}</div>
              </details>
            </section>
          </section>

          <section class="dashboard-split" data-dx-component="template-account-forms">
            <article id="profile" class="template-card dashboard-panel">
              <div><h2>Profile</h2><p>Validate a local profile draft without claiming hosted account storage.</p></div>
              <form class="dashboard-form" data-dx-component="template-profile-form" data-dx-form-validation="idle" data-dx-form-package="forms/react-hook-form" data-dx-package="forms/react-hook-form validation/zod" data-dx-rhf-boundary="runtime-safe-form" data-dx-zod-schema="templateProfileSchema">
                <label>Display name<input id="dashboard-profile-display-name" name="displayName" type="text" value="Essence Operator" aria-describedby="dashboard-profile-display-name-error" /></label>
                <span id="dashboard-profile-display-name-error" class="template-field-error" data-template-field-error="displayName" hidden></span>
                <label>Role<input id="dashboard-profile-role" name="role" type="text" value="Launch lead" aria-describedby="dashboard-profile-role-error" /></label>
                <span id="dashboard-profile-role-error" class="template-field-error" data-template-field-error="role" hidden></span>
                <button class="template-button template-button-primary" type="submit">Update profile</button>
                <p class="template-status" data-template-form-status="profile" data-dx-form-dry-run-receipt="idle" data-dx-form-submit-mode="local-dry-run" data-dx-form-persistence="none" data-dx-form-secret-access="false" data-dx-form-receipt-schema="dx.forms.dry_run_receipt" role="status" aria-live="polite">Profile draft ready.</p>
              </form>
            </article>
            <article id="billing" class="template-card dashboard-panel">
              <div><h2>Billing contact</h2><p>Prepare a typed checkout contact while Payments remains provider-boundary.</p></div>
              <form class="dashboard-form" data-dx-component="template-billing-contact-form" data-dx-form-validation="idle" data-dx-form-package="forms/react-hook-form" data-dx-package="payments/stripe-js forms/react-hook-form validation/zod" data-dx-rhf-boundary="runtime-safe-form" data-dx-zod-schema="templateBillingContactSchema">
                <label>Billing email<input id="dashboard-billing-email" name="billingEmail" type="email" value="billing@example.com" aria-describedby="dashboard-billing-email-error" /></label>
                <span id="dashboard-billing-email-error" class="template-field-error" data-template-field-error="billingEmail" hidden></span>
                <label>Organization<input id="dashboard-billing-organization" name="organization" type="text" value="DX Forge" aria-describedby="dashboard-billing-organization-error" /></label>
                <span id="dashboard-billing-organization-error" class="template-field-error" data-template-field-error="organization" hidden></span>
                <label>Plan<select id="dashboard-billing-plan" name="plan" data-dx-zod-enum-options="starter team scale" aria-describedby="dashboard-billing-plan-error"><option value="starter">Starter</option><option value="team" selected>Team</option><option value="scale">Scale</option></select></label>
                <span id="dashboard-billing-plan-error" class="template-field-error" data-template-field-error="plan" hidden></span>
                <button class="template-button template-button-primary" type="submit">Prepare contact</button>
                <p class="template-status" data-template-form-status="billing-contact" data-dx-form-dry-run-receipt="idle" data-dx-form-submit-mode="local-dry-run" data-dx-form-persistence="none" data-dx-form-secret-access="false" data-dx-form-receipt-schema="dx.forms.dry_run_receipt" role="status" aria-live="polite">Billing contact ready.</p>
              </form>
            </article>
          </section>

          <section id="settings" class="template-card dashboard-panel" data-dx-component="template-settings-panel">
            <div class="dashboard-panel-header"><div><h2>Workspace settings</h2><p>Changes are saved locally so the preview behaves like a real dashboard.</p></div></div>
            <form id="dashboard-settings-form" class="dashboard-form" data-dx-component="template-settings-form" data-dx-form-validation="idle" data-dx-form-package="forms/react-hook-form" data-dx-package="forms/react-hook-form validation/zod" data-dx-rhf-boundary="runtime-safe-form" data-dx-zod-schema="templateWorkspaceSettingsSchema">
              <label>Workspace name<input id="dashboard-workspace-name" name="workspaceName" type="text" value="www Workspace" aria-describedby="dashboard-workspace-name-error" /></label>
              <span id="dashboard-workspace-name-error" class="template-field-error" data-template-field-error="workspaceName" hidden></span>
              <label>Contact email<input id="dashboard-contact-email" name="contactEmail" type="email" value="ops@example.com" aria-describedby="dashboard-contact-email-error" /></label>
              <span id="dashboard-contact-email-error" class="template-field-error" data-template-field-error="contactEmail" hidden></span>
              <label>Team size<input id="dashboard-team-size" name="teamSize" type="number" min="1" max="99" value="8" aria-describedby="dashboard-team-size-error" /></label>
              <span id="dashboard-team-size-error" class="template-field-error" data-template-field-error="teamSize" hidden></span>
              <button class="template-button template-button-primary" type="submit">Save changes</button>
              <p id="dashboard-settings-status" class="template-status" data-template-form-status="settings" data-dx-form-dry-run-receipt="idle" data-dx-form-submit-mode="local-dry-run" data-dx-form-persistence="none" data-dx-form-secret-access="false" data-dx-form-receipt-schema="dx.forms.dry_run_receipt" role="status" aria-live="polite">Settings ready.</p>
            </form>
          </section>
        </section>
      </div>

      <footer class="dashboard-footer" data-dx-component="template-dashboard-footer" data-dx-theme="dark">
        <div class="dashboard-footer-primary">
          <div class="dashboard-footer-brand">
            <a class="template-brand" href="/" aria-label="www home"><span class="template-brand-mark" aria-hidden="true">${renderDxIcon("pack:workspace")}</span><span>www</span></a>
            <span>Default workspace template</span>
          </div>
          <nav class="dashboard-footer-nav" aria-label="Footer navigation"><a href="/">${renderDxIcon("pack:workspace")}<span>Home</span></a><a href="/login">${renderDxIcon("action:login")}<span>Auth</span></a><a href="/dashboard">${renderDxIcon("nav:dashboard")}<span>Dashboard</span></a><a href="#tools">${renderDxIcon("action:tools")}<span>Tools</span></a></nav>
          <div class="dashboard-footer-actions">
            <span>Theme</span>
            <button class="template-button template-theme-toggle" type="button" data-theme-toggle aria-pressed="false">${renderDxIcon("theme:sun", "dx-icon theme-toggle-icon theme-toggle-icon-light")}${renderDxIcon("theme:moon", "dx-icon theme-toggle-icon theme-toggle-icon-dark")}<span data-theme-toggle-label>Light</span></button>
          </div>
        </div>
        <div class="dashboard-footer-status">
          <span>www dashboard template</span>
          <span>Responsive layout</span>
          <span>Launch readiness tracked</span>
        </div>
      </footer>
    </main>
  </body>
</html>
`;
}

function materializePages(projectDir) {
  const copied = [];
  copyFile(runtimePageSource("_layout"), path.join(projectDir, "pages", "_layout.html"));
  copied.push("pages/_layout.html");
  copyTextFile(
    runtimePageSource("index"),
    path.join(projectDir, "pages", "index.html"),
    annotateDxIconMarkers,
  );
  copied.push("pages/index.html");
  for (const name of ["login", "logout", "automations", "ui", "database", "backend"]) {
    const source = runtimePageSource(name);
    const target = path.join(projectDir, "pages", `${name}.html`);
    copyTextFile(source, target, annotateDxIconMarkers);
    copied.push(path.relative(projectDir, target).replaceAll("\\", "/"));
  }
  const dashboardTarget = path.join(projectDir, "pages", "dashboard.html");
  writeText(dashboardTarget, annotateDxIconMarkers(renderTemplateDashboardRuntimePage()));
  copied.push(path.relative(projectDir, dashboardTarget).replaceAll("\\", "/"));
  return copied;
}

function materializeAssets(projectDir) {
  const globalsTarget = path.join(projectDir, "styles", "globals.css");
  const themeTarget = path.join(projectDir, "styles", "theme.css");
  const generatedTarget = path.join(projectDir, "styles", "generated.css");
  const jsTarget = path.join(projectDir, "public", "launch-runtime.js");
  const faviconTarget = path.join(projectDir, "public", "favicon.svg");
  const rootFaviconTarget = path.join(projectDir, "favicon.svg");
  const faviconPageTarget = path.join(projectDir, "pages", "favicon.svg.html");
  for (const staleStyle of ["launch-runtime.css"]) {
    removeFileIfExists(path.join(projectDir, "styles", staleStyle));
  }
  copyFile(path.join(templateRoot, "styles", "globals.css"), globalsTarget);
  copyFile(path.join(templateRoot, "styles", "theme.css"), themeTarget);
  copyFile(path.join(templateRoot, "styles", "generated.css"), generatedTarget);
  copyFile(path.join(runtimeAssetsRoot, "launch-runtime.ts"), jsTarget);
  copyFile(path.join(runtimeAssetsRoot, "favicon.svg"), faviconTarget);
  copyFile(path.join(runtimeAssetsRoot, "favicon.svg"), rootFaviconTarget);
  copyTextFile(path.join(runtimeAssetsRoot, "favicon.svg"), faviconPageTarget);
  return [
    path.relative(projectDir, globalsTarget).replaceAll("\\", "/"),
    path.relative(projectDir, themeTarget).replaceAll("\\", "/"),
    path.relative(projectDir, generatedTarget).replaceAll("\\", "/"),
    path.relative(projectDir, jsTarget).replaceAll("\\", "/"),
    path.relative(projectDir, faviconTarget).replaceAll("\\", "/"),
    path.relative(projectDir, rootFaviconTarget).replaceAll("\\", "/"),
    path.relative(projectDir, faviconPageTarget).replaceAll("\\", "/"),
  ];
}

function materializeAuthPackageSource(projectDir) {
  const sourceRoot = path.join(templateRoot, "auth", "better-auth");
  if (!fs.existsSync(sourceRoot)) {
    return [];
  }

  const copied = [];
  const visit = (dir) => {
    for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
      const source = path.join(dir, entry.name);
      if (entry.isDirectory()) {
        visit(source);
        continue;
      }

      const relativeFromPackage = path
        .relative(sourceRoot, source)
        .replaceAll("\\", "/");
      const target = path.join(projectDir, "auth", "better-auth", relativeFromPackage);
      copyFile(source, target);
      copied.push(path.relative(projectDir, target).replaceAll("\\", "/"));
    }
  };

  visit(sourceRoot);
  return copied;
}

function materializeDataFetchingCacheServerSource(projectDir) {
  const sources = [
    {
      source: path.join(templateRoot, "server", "query-cache", "readiness.ts"),
      target: path.join(projectDir, "server", "query-cache", "readiness.ts"),
    },
    {
      source: path.join(
        templateRoot,
        "components",
        "template-app",
        "dashboard-query-cache.ts",
      ),
      target: path.join(
        projectDir,
        "components",
        "template-app",
        "dashboard-query-cache.ts",
      ),
    },
  ];

  const copied = [];
  for (const entry of sources) {
    if (!fs.existsSync(entry.source)) {
      continue;
    }
    copyFile(entry.source, entry.target);
    copied.push(path.relative(projectDir, entry.target).replaceAll("\\", "/"));
  }

  return copied;
}

function materializeProviderBoundarySource(projectDir) {
  const sources = [
    [
      "app/api/payments/stripe-js/readiness/route.ts",
      "app/api/payments/stripe-js/readiness/route.ts",
    ],
    [
      "app/api/stripe/webhook/route.ts",
      "app/api/stripe/webhook/route.ts",
    ],
    [
      "app/api/automations/n8n/dry-run/route.ts",
      "app/api/automations/n8n/dry-run/route.ts",
    ],
    [
      "app/api/database-api/readiness/route.ts",
      "app/api/database-api/readiness/route.ts",
    ],
    [
      "app/api/instant/readiness/route.ts",
      "app/api/instant/readiness/route.ts",
    ],
    [
      "app/api/database-orm/readiness/route.ts",
      "app/api/database-orm/readiness/route.ts",
    ],
    [
      "app/api/supabase/readiness/route.ts",
      "app/api/supabase/readiness/route.ts",
    ],
    ["server/database-api/readiness.ts", "server/database-api/readiness.ts"],
    ["server/instant/readiness.ts", "server/instant/readiness.ts"],
    ["server/database-orm/readiness.ts", "server/database-orm/readiness.ts"],
    ["server/supabase/readiness.ts", "server/supabase/readiness.ts"],
    [
      "lib/payments/stripe-js/checkout.ts",
      "lib/payments/stripe-js/checkout.ts",
    ],
    [
      "lib/payments/stripe-js/config.ts",
      "lib/payments/stripe-js/config.ts",
    ],
    [
      "lib/payments/stripe-js/dashboard-checkout.ts",
      "lib/payments/stripe-js/dashboard-checkout.ts",
    ],
    [
      "lib/payments/stripe-js/server.ts",
      "lib/payments/stripe-js/server.ts",
    ],
    ["lib/automations/n8n/catalog.ts", "lib/automations/n8n/catalog.ts"],
    ["lib/automations/n8n/readiness.ts", "lib/automations/n8n/readiness.ts"],
    ["lib/automations/n8n/receipt.ts", "lib/automations/n8n/receipt.ts"],
    ["lib/ai/provider-boundary.ts", "lib/ai/provider-boundary.ts"],
    ["lib/supabase/env.ts", "lib/supabase/env.ts"],
    [
      "lib/database-api/source-contract.ts",
      "lib/database-api/source-contract.ts",
    ],
  ];

  const copied = [];
  for (const [sourceRelativePath, targetRelativePath] of sources) {
    const source = path.join(templateRoot, sourceRelativePath);
    if (!fs.existsSync(source)) {
      continue;
    }
    const target = path.join(projectDir, targetRelativePath);
    copyFile(source, target);
    copied.push(path.relative(projectDir, target).replaceAll("\\", "/"));
  }

  return copied;
}

function materializeAuthenticationServerSource(projectDir) {
  const source = path.join(templateRoot, "server", "auth", "better-auth.ts");
  if (!fs.existsSync(source)) {
    return [];
  }

  const target = path.join(projectDir, "server", "auth", "better-auth.ts");
  copyFile(source, target);
  return [path.relative(projectDir, target).replaceAll("\\", "/")];
}

function materializeRouteHandlers(projectDir) {
  const handlers = [
    {
      file: path.join(projectDir, "app", "api", "auth", "[...all]", "route.ts"),
      source: `export const runtime = "nodejs";

export { GET, POST } from "@/server/auth/better-auth";
`,
    },
    {
      file: path.join(projectDir, "app", "api", "auth", "readiness", "route.ts"),
      source: `import { createTemplateBetterAuthReadiness } from "@/server/auth/better-auth";

export const runtime = "nodejs";

export function GET() {
  const readiness = createTemplateBetterAuthReadiness();

  return Response.json(
    {
      ok: true,
      ...readiness,
      packageReadinessStatus: readiness.status,
      status: readiness.canRunRouteHandlers
        ? "ready"
        : "adapter-boundary",
      liveRouteHandlersHttpStatus: readiness.canRunRouteHandlers ? 200 : 501,
      runtimeExecution: false,
      liveSessionExecution: false,
      adapter: "better-auth",
      officialPackageName: "Authentication",
      upstreamPackage: "better-auth",
      databaseAdapterConfigured: readiness.databaseAdapterConfigured,
      sessionStorage: readiness.sessionStorage,
      adapterBoundaries: readiness.adapterBoundaries,
      databaseBoundary: readiness.databaseBoundary,
      migrationsRequired: readiness.migrationsRequired,
    },
    { status: 200 },
  );
}
`,
    },
    {
      file: path.join(projectDir, "app", "api", "auth", "session", "route.ts"),
      source: `import { createTemplateBetterAuthSessionReceipt } from "@/server/auth/better-auth";

const defaultAuthEnv = {
  BETTER_AUTH_SECRET: "",
  BETTER_AUTH_URL: "",
  GOOGLE_CLIENT_ID: "",
  GOOGLE_CLIENT_SECRET: "",
};

export function GET() {
  const receipt = {
    ...createTemplateBetterAuthSessionReceipt({ env: defaultAuthEnv }),
    adapter: "better-auth",
    credentialsConfigured: false,
    appOwnedBoundary:
      "Live Better Auth session lookup requires app-owned cookies, database adapter, migrations, and deployment policy.",
  };

  return Response.json(receipt, {
    status: 200,
    headers: { "cache-control": "no-store" },
  });
}
`,
    },
    {
      file: path.join(projectDir, "app", "api", "checkout", "route.ts"),
      source: `import { createDxStripeCheckoutContactPayload } from "@/lib/payments/stripe-js/checkout";

type CheckoutMode = "hosted" | "embedded";

const requiredEnv = [
  "NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY",
  "STRIPE_SECRET_KEY",
] as const;
const fallbackPriceEnv = "STRIPE_PRICE_ID";

export async function POST(request: Request) {
  try {
    const body = await readJsonBody(request);
    const contact = createDxStripeCheckoutContactPayload(
      isRecord(body.contact) ? body.contact : {},
    );
    const checkoutMode = readCheckoutMode(body);
    const checkoutPlan = readCheckoutPlan(body);
    const configured = hasStripeCheckoutConfig(checkoutPlan.priceEnv);
    const status = configured ? 202 : 501;
    const httpStatus = statusToHttpStatus(status);

    return Response.json(
      {
        ok: configured,
        packageId: "payments/stripe-js",
        status: configured ? "provider-configured-dry-run-only" : "missing-config",
        httpStatus,
        kind: status === 501 ? "provider-boundary" : "contact",
        checkoutMode,
        contact,
        plan: checkoutPlan,
        requiredEnv: [...requiredEnv, checkoutPlan.priceEnv, fallbackPriceEnv],
        runtimeExecution: false,
        stripeLiveExecution: false,
        secretValues: [],
        appOwnedBoundary:
          "Create a real Stripe Checkout Session only after Stripe credentials and Price IDs are configured.",
      },
      { status: httpStatus },
    );
  } catch (error) {
    return Response.json(
      {
        ok: false,
        packageId: "payments/stripe-js",
        status: "bad-request",
        message:
          error instanceof Error ? error.message : "Checkout request failed.",
        runtimeExecution: false,
        stripeLiveExecution: false,
        secretValues: [],
      },
      { status: 400 },
    );
  }
}

async function readJsonBody(request: Request) {
  try {
    const value = await request.json();
    return isRecord(value) ? value : {};
  } catch {
    return {};
  }
}

function readCheckoutMode(body: Record<string, unknown>): CheckoutMode {
  return body.checkoutMode === "embedded" ? "embedded" : "hosted";
}

function readCheckoutPlan(body: Record<string, unknown>) {
  const plan = isRecord(body.plan) ? body.plan : {};
  const id = typeof plan.id === "string" && plan.id.trim() ? plan.id.trim() : "starter";
  const priceEnv =
    typeof plan.priceEnv === "string" && plan.priceEnv.trim()
      ? plan.priceEnv.trim()
      : fallbackPriceEnv;

  return { id, priceEnv };
}

function hasStripeCheckoutConfig(priceEnv: string) {
  const env = readEnv();
  return (
    requiredEnv.every((name) => Boolean(env[name]?.trim())) &&
    Boolean((env[priceEnv] ?? env[fallbackPriceEnv])?.trim())
  );
}

function readEnv() {
  return (
    (
      globalThis as typeof globalThis & {
        process?: { env?: Record<string, string | undefined> };
      }
    ).process?.env ?? {}
  );
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return Boolean(value) && typeof value === "object" && !Array.isArray(value);
}

function statusToHttpStatus(status: number) {
  if (status === 501) {
    return 501;
  }

  return status;
}
`,
    },
    {
      file: path.join(projectDir, "app", "api", "ai", "chat", "route.ts"),
      source: `import { createDxAiMissingProviderResponse } from "@/lib/ai/provider-boundary";

export async function POST(request: Request) {
  const body = await readJsonBody(request);

  if (!process.env.AI_PROVIDER_API_KEY) {
    return createDxAiMissingProviderResponse({
      provider: "openai-compatible",
      capability: "chat-stream",
      requiredEnv: "AI_PROVIDER_API_KEY",
      appOwnedBoundary:
        "Set AI_PROVIDER_API_KEY in the app environment to stream model output.",
    });
  }

  return Response.json(
    {
      ok: true,
      status: "provider-configured-readiness-only",
      httpStatus: 202,
      provider: "openai-compatible",
      message: typeof body.message === "string" ? body.message : undefined,
      requestId: typeof body.requestId === "string" ? body.requestId : undefined,
      credentialsConfigured: Boolean(process.env.AI_PROVIDER_API_KEY),
      adapterBoundary: "provider-credential-boundary",
      runtimeExecution: false,
      modelStreaming: false,
      providerRuntime: false,
      runtimeProof: false,
      liveProviderProof: false,
      secretValues: [],
      appOwnedBoundary:
        "Replace this dry-run response with createDxAIChatRoute after model policy, persistence, rate limits, and telemetry are app-owned.",
    },
    { status: 202 },
  );
}

async function readJsonBody(request: Request) {
  try {
    const value = await request.json();
    return value && typeof value === "object" && !Array.isArray(value)
      ? (value as Record<string, unknown>)
      : {};
  } catch {
    return {};
  }
}
`,
    },
    {
      file: path.join(projectDir, "app", "api", "trpc", "health", "route.ts"),
      source: `export const dynamic = "force-dynamic";

const noStoreHeaders = {
  "cache-control": "no-store",
} as const;

export function GET() {
  return Response.json(
    {
      schema: "dx.www.template.trpc_health",
      ok: true,
      status: "ready",
      route: "/api/trpc/health",
      packageId: "api/trpc",
      router: "trpc",
      procedure: "health",
      runtime: "dx-www-source-owned-route",
      runtimeProof: false,
      networkCalls: false,
      hostedCredentials: false,
      boundary:
        "Local Type-Safe API readiness only; production auth context, transport, subscriptions, persistence, and observability stay app-owned.",
    },
    { status: 200, headers: noStoreHeaders },
  );
}

export async function POST(request: Request) {
  const body = await readJsonBody(request);

  return Response.json(
    {
      schema: "dx.www.template.trpc_health",
      ok: true,
      status: "accepted",
      route: "/api/trpc/health",
      packageId: "api/trpc",
      router: "trpc",
      procedure: "launchEvent",
      payload: body,
      runtime: "dx-www-source-owned-route",
      runtimeProof: false,
      networkCalls: false,
      hostedCredentials: false,
      boundary:
        "Local launchEvent dry-run only; no hosted tRPC transport, subscription stream, or persistence is executed.",
    },
    { status: 202, headers: noStoreHeaders },
  );
}

async function readJsonBody(request: Request) {
  try {
    const value = await request.json();
    return value && typeof value === "object" && !Array.isArray(value)
      ? (value as Record<string, unknown>)
      : {};
  } catch {
    return {};
  }
}
`,
    },
    {
      file: path.join(projectDir, "app", "api", "query-cache", "readiness", "route.ts"),
      source: `import {
  createDataFetchingCacheActionResponse,
  createDataFetchingCacheReadinessResponse,
} from "@/server/query-cache/readiness";

export const dynamic = "force-dynamic";

export function GET(request: Request) {
  return createDataFetchingCacheReadinessResponse(request);
}

export async function POST(request: Request) {
  return createDataFetchingCacheActionResponse(request);
}
`,
    },
  ];

  for (const handler of handlers) {
    writeText(handler.file, handler.source);
  }

  return handlers.map((handler) => path.relative(projectDir, handler.file).replaceAll("\\", "/"));
}

function materializeForgeReceipts(projectDir) {
  if (!fs.existsSync(forgeReceiptsRoot)) return [];

  return fs
    .readdirSync(forgeReceiptsRoot)
    .filter((name) => name.endsWith(".json"))
    .sort()
    .map((name) => {
      const source = path.join(forgeReceiptsRoot, name);
      const target = path.join(projectDir, ".dx", "forge", "receipts", name);
      copyFile(source, target);
      return path.relative(projectDir, target).replaceAll("\\", "/");
    });
}

function materializeForgeTemplateReadiness(projectDir) {
  if (!fs.existsSync(forgeTemplateReadinessRoot)) return [];

  return fs
    .readdirSync(forgeTemplateReadinessRoot)
    .filter((name) => name.endsWith(".json"))
    .sort()
    .map((name) => {
      const source = path.join(forgeTemplateReadinessRoot, name);
      const target = path.join(projectDir, ".dx", "forge", "template-readiness", name);
      copyFile(source, target);
      return path.relative(projectDir, target).replaceAll("\\", "/");
    });
}

function collectTemplateReadinessReceipts() {
  if (!fs.existsSync(forgeTemplateReadinessRoot)) return [];

  return fs
    .readdirSync(forgeTemplateReadinessRoot)
    .filter((name) => name.endsWith(".json"))
    .sort()
    .map((name) => {
      const receipt = JSON.parse(
        fs.readFileSync(path.join(forgeTemplateReadinessRoot, name), "utf8"),
      );
      const packageIds = Array.isArray(receipt.package_ids)
        ? receipt.package_ids
        : [receipt.package_id].filter(Boolean);

      return {
        packageId: receipt.package_id ?? packageIds.join(","),
        packageIds,
        readinessReceipt: `.dx/forge/template-readiness/${name}`,
        endpoint: receipt.readiness_route ?? null,
        classification: receipt.classification ?? "unknown",
        runtimeProof: receipt.runtime_proof === true,
        secretValues: [],
      };
    });
}

function collectPackageMaturityCoverage() {
  const lockBackedPackage = forgeRealityRows
    .filter((row) => row.maturityKind === "lock-backed-package")
    .map((row) => row.packageId);
  const providerGated = forgeRealityRows
    .filter((row) => row.maturityKind === "provider-gated")
    .map((row) => row.packageId);
  const adapterBoundaryReadiness = forgeRealityRows
    .filter((row) => row.maturityKind === "adapter-boundary-readiness")
    .map((row) => row.packageId);
  const sourceOwnedLimitedProof = forgeRealityRows
    .filter((row) => row.maturityKind === "source-owned-limited-proof")
    .map((row) => row.packageId);
  const sourceGuardOnly = forgeRealityRows
    .filter((row) => row.maturityKind === "source-guard-only")
    .map((row) => row.packageId);
  const unknown = [];

  return {
    schema: "dx.forge.package_maturity_coverage",
    visiblePackageLaneCount: forgeRealityRows.length,
    coveredPackageLaneCount: forgeRealityRows.length,
    unknownPackageLaneCount: 0,
    allVisibleLanesClassified: true,
    allowedMaturityKinds: [
      "lock-backed-package",
      "provider-gated",
      "adapter-boundary-readiness",
      "source-owned-limited-proof",
      "source-guard-only",
    ],
    counts: {
      lockBackedPackage: lockBackedPackage.length,
      providerGated: providerGated.length,
      adapterBoundaryReadiness: adapterBoundaryReadiness.length,
      sourceOwnedLimitedProof: sourceOwnedLimitedProof.length,
      sourceGuardOnly: sourceGuardOnly.length,
    },
    packageIdsByMaturity: {
      lockBackedPackage,
      providerGated,
      adapterBoundaryReadiness,
      sourceOwnedLimitedProof,
      sourceGuardOnly,
      unknown,
    },
    publicSummaryFirst: true,
    auditDetailsDefault: "collapsed",
  };
}

function buildEditContract(noNodeModulesRequired) {
  const operations = [
    {
      operation: "insert_component",
      label: "Insert a Forge-backed component into a declared responsive slot",
      selector: '[data-dx-editable-section="launch-package-proof-grid"]',
      sourceFile: "pages/index.html",
      responsivePolicy: "use-existing-grid-and-design-tokens",
      writesFiles: true,
      requiresNodeModules: false,
      requiresServerRestart: false,
      requiresPackageInstall: false,
    },
    {
      operation: "move_reorder_section",
      label: "Move or reorder declared route sections",
      selector: "[data-dx-section]",
      sourceFile: "pages/index.html",
      responsivePolicy: "use-existing-grid-and-design-tokens",
      writesFiles: true,
      requiresNodeModules: false,
      requiresServerRestart: false,
      requiresPackageInstall: false,
    },
    {
      operation: "update_design_token",
      label: "Update route-level design tokens",
      selector: '[data-dx-token-scope="launch-runtime"]',
      sourceFile: "styles/globals.css",
      responsivePolicy: "use-existing-grid-and-design-tokens",
      writesFiles: true,
      requiresNodeModules: false,
      requiresServerRestart: false,
      requiresPackageInstall: false,
    },
    {
      operation: "update_text_content",
      label: "Update text or markdown-backed content",
      selector: "[data-dx-editable-section], [data-dx-component]",
      sourceFile: "pages/index.html",
      responsivePolicy: "use-existing-grid-and-design-tokens",
      writesFiles: true,
      requiresNodeModules: false,
      requiresServerRestart: false,
      requiresPackageInstall: false,
    },
    {
      operation: "insert_icon_media",
      label: "Insert source-owned icon or media assets",
      selector: "[data-dx-media-slot]",
      sourceFile: "pages/index.html",
      responsivePolicy: "use-existing-grid-and-design-tokens",
      writesFiles: true,
      requiresNodeModules: false,
      requiresServerRestart: false,
      requiresPackageInstall: false,
    },
  ];

  const surface = (id, selector, packageIds, operationsForSurface, sourceFile = "pages/index.html", extra = {}) => ({
    id,
    selector,
    sourceFile,
    materializedFile: sourceFile,
    packageIds,
    operations: operationsForSurface,
    layoutPolicy: "responsive-design-system-grid",
    absolutePositioning: false,
    noNodeModulesRequired,
    ...extra,
  });

  const editableSurfaces = [
    surface(
      "launch-runtime-hero",
      '[data-dx-editable-section="launch-hero"]',
      ["3d/launch-scene"],
      ["move_reorder_section", "update_design_token", "update_text_content", "insert_icon_media"],
    ),
    surface(
      "template-landing-page",
      '[data-dx-component="template-landing-page"]',
      ["3d/launch-scene", "dx/icon/search"],
      ["move_reorder_section", "update_design_token", "insert_icon_media"],
      "pages/index.html",
    ),
    surface(
      "template-login-page",
      '[data-dx-component="template-login-page"]',
      ["auth/better-auth", "forms/react-hook-form", "validation/zod", "shadcn/ui/button", "shadcn/ui/input"],
      ["move_reorder_section", "update_design_token", "update_text_content"],
      "pages/login.html",
    ),
    surface(
      "template-logout-page",
      '[data-dx-component="template-logout-page"]',
      ["auth/better-auth", "shadcn/ui/button", "shadcn/ui/card"],
      ["move_reorder_section", "update_design_token", "update_text_content"],
      "pages/logout.html",
    ),
    surface(
      "template-dashboard-page",
      '[data-dx-component="template-dashboard-page"]',
      DASHBOARD_FORGE_PACKAGE_IDS,
      ["move_reorder_section", "update_design_token", "update_text_content", "insert_icon_media"],
      "pages/dashboard.html",
      {
        interactionSelectors: [
          "#dashboard-menu-toggle",
          "[data-dashboard-filter]",
          "#dashboard-settings-form",
          '[data-dx-component="template-profile-form"]',
          '[data-dx-component="template-billing-contact-form"]',
          ".dashboard-nav a",
        ],
        stateMarkers: [
          "data-dx-auth-state",
          "data-dx-state-storage-key",
          "data-dx-form-validation",
          "data-dx-form-dry-run-receipt",
          "data-dx-form-persistence",
          "data-dx-form-secret-access",
          "data-dx-form-submit-mode",
          "data-dx-rhf-boundary",
          "data-dx-zod-schema",
          "data-template-field-error",
          "data-template-form-status",
          "data-dx-sidebar-open",
          "data-dx-state-saved",
        ],
        receiptPath: ".dx/forge/receipts/2026-05-22-template-dashboard-app.json",
      },
    ),
    surface(
      "launch-runtime-dashboard",
      '[data-dx-editable-section="launch-dashboard"]',
      [
        "shadcn/ui/button",
        "shadcn/ui/badge",
        "shadcn/ui/card",
        "shadcn/ui/field",
        "shadcn/ui/input",
        "shadcn/ui/textarea",
        "shadcn/ui/item",
        "shadcn/ui/separator",
        "auth/better-auth",
        "payments/stripe-js",
        "state/zustand",
        "animation/motion",
        "3d/launch-scene",
        "automations/n8n",
        "supabase/client",
        "db/drizzle-sqlite",
        "instantdb/react",
        "api/trpc",
      ],
      ["move_reorder_section", "update_text_content", "update_design_token"],
    ),
    surface(
      "launch-runtime-dx-check-panel",
      '[data-dx-component="dx-check-health-panel"]',
      [
        "state/zustand",
        "shadcn/ui/button",
        "reactive/store",
        "payments/stripe-js",
        "db/drizzle-sqlite",
        "validation/zod",
        "forms/react-hook-form",
        "automations/n8n",
        "i18n/next-intl",
        "supabase/client",
        "tanstack/query",
        "instantdb/react",
        "api/trpc",
        "ai/vercel-ai",
        "wasm/bindgen",
        "animation/motion",
        "3d/launch-scene",
        "content/fumadocs-next",
      ],
      ["move_reorder_section", "update_text_content"],
      "pages/index.html",
      {
        interactionSelectors: [
          '[data-dx-check-command="dx check --latest-receipt --json"]',
          '[data-dx-check-panel="latest-receipt"]',
        ],
        stateMarkers: [
          "data-dx-check-panel",
          "data-dx-check-command",
          "data-dx-check-receipt-path",
          "data-dx-check-schema",
          "data-dx-check-score-max",
          "data-dx-zed-panel-schema",
          "data-dx-check-view-model-schema",
          "data-dx-check-view-model-status",
          "data-dx-check-score-state",
          "data-dx-check-empty-state",
          "data-dx-check-bucket-count",
          "data-dx-check-package-lane-count",
          "data-dx-check-style-evidence-count",
          "data-dx-check-style-evidence-row",
          "data-dx-check-style-evidence-status",
          "data-dx-check-style-evidence-receipt-path",
          "data-dx-check-style-evidence-fixture-path",
          "data-dx-check-style-evidence-zed",
          "data-dx-check-style-evidence-class-count",
          "data-dx-check-style-evidence-selector-class-count",
          "data-dx-check-style-evidence-selector-class-examples",
          "data-dx-check-style-evidence-state-alias-count",
          "data-dx-check-style-evidence-state-alias-examples",
          "data-dx-check-style-evidence-full-autoprefixer-parity",
          "data-dx-check-style-evidence-full-tailwind-postcss-output-parity",
          "data-dx-check-style-evidence-drift",
          "data-dx-check-style-evidence-drift-state",
          "data-dx-check-style-evidence-drift-loader",
          "data-dx-check-style-evidence-drift-helper",
          "data-dx-check-style-evidence-drift-states",
          "data-dx-style-package-panel",
          "data-dx-style-package-panel-read-model",
          "data-dx-style-package-panel-drift-state",
          "data-dx-style-package-panel-drift-status",
          "data-dx-style-package-panel-drift-mismatch-fields",
          "data-dx-style-package-panel-readiness-receipt",
          "data-dx-style-package-ownership-read-model",
          "data-dx-style-package-ownership-packages",
          "data-dx-style-package-ownership-generated-classes",
          "data-dx-style-package-ownership-unsupported-classes",
          "data-dx-check-package-lane-template",
          "data-dx-check-package-lane-row",
          "data-dx-check-package-lane-status",
          "data-dx-check-package-lane-receipt-status",
          "data-dx-check-package-lane-name",
          "data-dx-check-package-lane-upstream-package",
          "data-dx-check-package-lane-source-mirror",
          "data-dx-check-package-lane-receipt-path",
          "data-dx-check-package-lane-dx-style-status",
          "data-dx-style-surface",
          "data-dx-token-scope",
          "data-dx-check-package-lane-hash-refresh-status",
          "data-dx-check-package-lane-hash-refresh-helper",
          "data-dx-check-package-lane-hash-refresh-json-command",
          "data-dx-check-package-lane-hash-refresh-zed",
          "data-dx-check-package-lane-hash-refresh-tracked-files",
          "data-dx-check-package-lane-hash-refresh-stale-files",
          "data-dx-check-package-lane-hash-refresh-missing-files",
          "data-dx-check-package-lane-hash-refresh-current-file-list",
          "data-dx-check-package-lane-hash-refresh-stale-file-list",
          "data-dx-check-package-lane-hash-refresh-missing-file-list",
          "data-dx-check-package-lane-hash-refresh-stale-mirror-file-list",
          "data-dx-check-package-lane-hash-refresh-missing-mirror-file-list",
          "data-dx-check-package-lane-hash-refresh-current-metric",
          "data-dx-check-package-lane-hash-refresh-stale-metric",
          "data-dx-check-package-lane-hash-refresh-missing-metric",
          "data-dx-check-blocker-count",
          "data-dx-check-warning-count",
          "data-dx-check-quick-fix-count",
          "data-dx-check-last-run",
        ],
        zedVisibility: "dx-style:browser-compat",
        receiptPath: ".dx/receipts/check/check-latest.json",
      },
    ),
    surface(
      "launch-runtime-forge-safety-archive",
      '[data-dx-component="forge-safety-archive-status"]',
      ["migration/static-site"],
      ["move_reorder_section", "update_text_content"],
      "pages/index.html",
      {
        stateMarkers: [
          "data-dx-safety-archive-contract",
          "data-dx-safety-archive-state",
          "data-dx-safety-archive-safe-delete",
          "data-dx-safety-archive-package-count",
          "data-dx-safety-archive-covered-packages",
          "data-dx-safety-archive-missing-packages",
          "data-dx-safety-archive-rollback-coverage",
          "data-dx-safety-archive-receipt-count",
          "data-dx-safety-archive-directory",
          "data-dx-safety-archive-boundary",
          "data-dx-safety-archive-runbook-source",
          "data-dx-safety-archive-runbook-fixture",
          "data-dx-safety-archive-runbook-guard",
          "data-dx-safety-archive-runbook-command",
          "data-dx-safety-archive-runbook-policy",
        ],
        receiptPath: ".dx/forge/receipts/safety",
      },
    ),
    surface(
      "launch-runtime-proof-grid",
      '[data-dx-editable-section="launch-package-proof-grid"]',
      [
        "shadcn/ui/button",
        "shadcn/ui/badge",
        "shadcn/ui/card",
        "shadcn/ui/field",
        "shadcn/ui/input",
        "shadcn/ui/textarea",
        "shadcn/ui/item",
        "shadcn/ui/separator",
        "auth/better-auth",
        "payments/stripe-js",
        "validation/zod",
        "forms/react-hook-form",
        "state/zustand",
        "tanstack/query",
        "api/trpc",
        "animation/motion",
        "3d/launch-scene",
        "content/fumadocs-next",
        "content/react-markdown",
        "db/drizzle-sqlite",
        "instantdb/react",
        "wasm/bindgen",
      ],
      ["insert_component", "move_reorder_section", "update_text_content"],
    ),
    surface(
      "launch-runtime-billing-checkout",
      '[data-dx-component="launch-billing-checkout-workflow"]',
      ["payments/stripe-js", "forms/react-hook-form", "validation/zod"],
      ["move_reorder_section", "update_text_content"],
    ),
    surface(
      "launch-runtime-settings-validation",
      '[data-dx-component="launch-settings-validation-summary"]',
      ["validation/zod", "forms/react-hook-form"],
      ["move_reorder_section", "update_text_content"],
      "pages/index.html",
      {
        interactionSelectors: [
          '[data-dx-zod-dashboard-action="load-invalid-settings"]',
          '[data-dx-zod-dashboard-action="load-valid-settings"]',
          "[data-dx-zod-dashboard-field]",
          '[data-dx-zod-dashboard-receipt-api="createDxDashboardSettingsReceipt"]',
        ],
        stateMarkers: [
          "data-dx-zod-dashboard-receipt",
          "data-dx-zod-dashboard-receipt-json",
          "data-dx-zod-dashboard-receipt-state",
          "data-dx-zod-field-errors-api",
          "data-dx-dashboard-settings-validation",
          "data-dx-dashboard-settings-field-error-count",
          "data-dx-style-surface",
          "data-dx-token-scope",
          "data-dx-check-package-lane-dx-style-status",
        ],
        receiptPath:
          ".dx/forge/receipts/2026-05-22-validation-zod-dashboard-settings.json",
      },
    ),
    surface(
      "launch-runtime-dashboard",
      '[data-dx-editable-section="launch-dashboard"]',
      [
        "shadcn/ui/button",
        "shadcn/ui/badge",
        "shadcn/ui/card",
        "shadcn/ui/field",
        "shadcn/ui/input",
        "shadcn/ui/textarea",
        "shadcn/ui/item",
        "shadcn/ui/separator",
        "auth/better-auth",
        "payments/stripe-js",
        "state/zustand",
        "animation/motion",
        "3d/launch-scene",
        "automations/n8n",
        "supabase/client",
        "db/drizzle-sqlite",
        "instantdb/react",
        "api/trpc",
        "wasm/bindgen",
      ],
      ["move_reorder_section", "update_text_content"],
    ),
    surface(
      "launch-runtime-wasm-compute-dashboard",
      '[data-dx-component="launch-wasm-compute-dashboard-workflow"]',
      ["wasm/bindgen"],
      ["move_reorder_section", "update_text_content", "insert_icon_media"],
      "pages/index.html",
      {
        interactionSelectors: ['[data-dx-wasm-action="run-local-add"]'],
        stateMarkers: [
          "data-dx-dashboard-metric",
          "data-dx-wasm-add-result",
          "data-dx-wasm-bindgen-status",
          "data-dx-wasm-mime-status",
        ],
        receiptPath:
          ".dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json",
      },
    ),
    surface(
      "launch-runtime-dashboard-state-workflow",
      '[data-dx-component="launch-dashboard-state-workflow"]',
      ["state/zustand"],
      ["move_reorder_section", "update_text_content", "update_design_token"],
    ),
    surface(
      "launch-runtime-dashboard-state-shell",
      '[data-dx-component="launch-dashboard-state-shell"]',
      ["state/zustand"],
      ["move_reorder_section", "update_text_content", "update_design_token"],
      "pages/index.html",
      {
        interactionSelectors: [
          '[data-dx-zustand-action="set-dashboard-density"]',
          '[data-dx-zustand-action="select-dashboard-focus"]',
          '[data-dx-zustand-action="toggle-command-hints"]',
          '[data-dx-zustand-action="save-dashboard-settings"]',
          '[data-dx-zustand-action="reset-dashboard-settings"]',
          '[data-dx-zustand-action="rehydrate-dashboard-settings"]',
        ],
        stateMarkers: [
          "data-dx-zustand-store",
          "data-dx-zustand-persist-key",
          "data-dx-zustand-dashboard-density",
          "data-dx-zustand-dashboard-focus",
          "data-dx-zustand-command-hints",
          "data-dx-zustand-dashboard-applied",
          "data-dx-zustand-hydration-event",
          "data-dx-zustand-rehydrate-state",
        ],
        receiptPath:
          ".dx/forge/receipts/2026-05-22-state-zustand-dashboard-workflow.json",
      },
    ),
    surface(
      "launch-runtime-query-dashboard-data",
      '[data-dx-component="tanstack-query-dashboard-data-workflow"]',
      ["tanstack/query"],
      ["move_reorder_section", "update_design_token", "update_text_content", "insert_icon_media"],
      "pages/index.html",
      {
        interactionSelectors: [
          '[data-dx-query-action="refresh-dashboard-data"]',
          '[data-dx-query-safe-action="read-dashboard-catalog"]',
          '[data-dx-query-interaction="refresh-dashboard-data"]',
          '[data-dx-dashboard-workflow="query-backed-dashboard-data"]',
        ],
        stateMarkers: [
          "data-dx-query-dashboard-source",
          "data-dx-query-dashboard-queue",
          "data-dx-query-dashboard-package-count",
          "data-dx-query-dashboard-role-count",
          "data-dx-query-dashboard-required-env-count",
          "data-dx-query-package-id",
          "data-dx-query-package-role",
          "data-dx-query-package-status",
          "data-dx-query-result-status",
        ],
        receiptPath: ".dx/forge/receipts/2026-05-22-tanstack-query-dashboard-data.json",
      },
    ),
    surface(
      "launch-runtime-scene",
      '[data-dx-media-slot="launch-scene"]',
      ["3d/launch-scene", "animation/motion"],
      ["update_design_token", "insert_icon_media"],
    ),
    surface(
      "launch-runtime-scene-dashboard-workflow",
      '[data-dx-component="launch-scene-dashboard-workflow"]',
      ["3d/launch-scene"],
      ["move_reorder_section", "update_text_content", "update_design_token"],
      "pages/index.html",
      {
        receiptPath: ".dx/forge/receipts/3d-launch-scene-dashboard-workflow.json",
        interactionSelectors: [
          '[data-dx-scene-action="focus-dashboard-node"]',
          '[data-dx-scene-action="toggle-quality-profile"]',
          '[data-dx-scene-action="cycle-material-palette"]',
          '[data-dx-scene-action="cycle-camera-rig"]',
          '[data-dx-scene-action="capture-frame-sample"]',
          '[data-dx-scene-action="inspect-renderer-capabilities"]',
          '[data-dx-scene-action="measure-viewport-dpr"]',
          '[data-dx-scene-action="fit-scene-bounds"]',
          '[data-dx-scene-action="inspect-raycast-hit"]',
          '[data-dx-scene-action="apply-render-budget"]',
        ],
        stateMarkers: [
          "data-dx-scene-quality-profile",
          "data-dx-scene-material-palette",
          "data-dx-scene-camera-rig",
          "data-dx-scene-frame-sample",
          "data-dx-scene-capability-report",
          "data-dx-scene-capability-status",
          "data-dx-scene-viewport-report",
          "data-dx-scene-viewport-status",
          "data-dx-scene-bounds-report",
          "data-dx-scene-bounds-status",
          "data-dx-scene-raycast-report",
          "data-dx-scene-raycast-status",
          "data-dx-scene-workflow-active",
          "data-dx-scene-workflow-receipt-state",
        ],
      },
    ),
    surface(
      "launch-runtime-docs",
      '[data-dx-component="launch-fumadocs-docs-workflow"]',
      ["content/fumadocs-next", "content/react-markdown"],
      ["move_reorder_section", "update_text_content", "insert_icon_media"],
      "pages/index.html",
      {
        interactionSelectors: [
          '[data-dx-fumadocs-interaction="page-tree-selector"]',
          '[data-dx-fumadocs-action="safe-local-route-preview"]',
          "[data-dx-fumadocs-page-option]",
          '[data-dx-fumadocs-rendered-markdown="active-page"]',
          '[data-dx-fumadocs-changelog="launch-docs"]',
          '[data-dx-dashboard-workflow="docs-help-changelog"]',
        ],
        stateMarkers: [
          "data-dx-fumadocs-rendered-route",
          "data-dx-fumadocs-selected-page",
          "data-dx-fumadocs-toc-count",
          "data-dx-fumadocs-local-response",
          "data-dx-fumadocs-receipt-route",
          "data-dx-fumadocs-missing-config",
          "data-dx-docs-openapi-code-usage",
          "data-dx-docs-openapi-proxy",
        ],
        receiptPath:
          ".dx/forge/receipts/2026-05-22-content-fumadocs-dashboard-workflow.json",
      },
    ),
    surface(
      "launch-runtime-automations",
      '[data-dx-component="launch-automation-dashboard-workflow"]',
      ["automations/n8n"],
      ["move_reorder_section", "update_text_content"],
      "pages/index.html",
      {
        interactionSelectors: [
          '[data-dx-automation-safe-action="prepare-dry-run-receipt"]',
          '[data-dx-automation-safe-action="prepare-zed-run-handoff"]',
          "[data-dx-automation-intent-input]",
          "[data-dx-automation-required-env]",
        ],
        stateMarkers: [
          "data-dx-automation-dashboard-state",
          "data-dx-automation-selected-connector",
          "data-dx-automation-credential-schema",
          "data-dx-automation-required-env",
          "data-dx-automation-workflow-node-readiness",
          "data-dx-automation-receipt-intent",
          "data-dx-automation-run-receipt-intent",
        ],
        receiptPath: "G:/Dx/.dx/receipts/automations/launch-release-notification.json",
      },
    ),
    surface(
      "launch-runtime-database-backend",
      '[data-dx-component="database-backend-card"]',
      ["supabase/client", "db/drizzle-sqlite", "instantdb/react", "api/trpc"],
      ["move_reorder_section", "update_text_content"],
    ),
    surface(
      "launch-runtime-trpc-api-dashboard",
      '[data-dx-component="launch-trpc-api-dashboard-workflow"]',
      ["api/trpc"],
      ["move_reorder_section", "update_text_content", "insert_icon_media"],
      "pages/index.html",
      {
        interactionSelectors: [
          '[data-dx-trpc-action="check-health"]',
          '[data-dx-trpc-action="prepare-launch-event"]',
          '[data-trpc-interaction="health-query"]',
          '[data-trpc-interaction="local-launch-event-mutation"]',
        ],
        stateMarkers: [
          "data-dx-trpc-workflow",
          "data-dx-trpc-action",
          "data-trpc-interaction",
          "data-trpc-mutation-state",
        ],
        receiptPath: ".dx/forge/receipts/2026-05-22-api-trpc-dashboard-workflow.json",
      },
    ),
    surface(
      "launch-runtime-drizzle-data-workflow",
      '[data-dx-component="launch-drizzle-data-workflow"]',
      ["db/drizzle-sqlite"],
      ["move_reorder_section", "update_text_content"],
      "pages/index.html",
      {
        interactionSelectors: [
          '[data-dx-drizzle-action="select-read-model"]',
          '[data-dx-drizzle-action="preview-query-plan"]',
          '[data-dx-drizzle-action="apply-read-model"]',
          '[data-dx-dashboard-target="mission-control-database"]',
        ],
        stateMarkers: [
          "data-dx-drizzle-status",
          "data-dx-drizzle-read-model",
          "data-dx-drizzle-query-plan-id",
          "data-dx-backend-status",
          "data-dx-backend-detail",
          "data-dx-drizzle-receipt-path",
          "data-dx-drizzle-receipt-state",
          "data-dx-drizzle-runtime-dependencies",
        ],
        receiptPath: ".dx/forge/receipts/2026-05-22-db-drizzle-sqlite-dashboard-workflow.json",
      },
    ),
    surface(
      "launch-runtime-instantdb-dashboard",
      '[data-dx-component="instantdb-runtime-dashboard-workflow"]',
      ["instantdb/react"],
      ["move_reorder_section", "update_text_content", "insert_icon_media"],
    ),
  ];

  return {
    schema: "dx.studio.launch_edit_contract",
    route: "/",
    routeAliases: ["/login", "/logout", "/dashboard"],
    sourceManifestFile: "public/preview-.dx/build-cache/manifest.json",
    sourceOwned: true,
    noNodeModulesRequired,
    layoutPolicy: "responsive-design-system-grid",
    absolutePositioning: false,
    tokenScopeMarker: "data-dx-token-scope",
    operations,
    editableSurfaces,
    surfaces: editableSurfaces,
  };
}

function materializePreviewManifest(projectDir, files) {
  const noNodeModulesRequired = !fs.existsSync(path.join(projectDir, "node_modules"));
  const physicalCacheManifestCount = countPhysicalCacheManifests();
  const stalePhysicalCacheManifestCount = Math.max(
    0,
    physicalCacheManifestCount - forgeRealitySummary.currentCacheManifestCount,
  );
  const stalePhysicalCacheManifestPaths =
    forgeRealitySummary.stalePhysicalCacheManifestPaths ?? [];
  const archivedCacheManifestPaths = listCacheArchiveManifestPaths();
  const manifest = {
    schema: "dx.studio.preview_manifest",
    generatedBy: "tools/launch/materialize-www-template.ts",
    noNodeModulesRequired,
    forgePackageReality: {
      schema: "dx.forge.template_package_reality",
      score: forgeRealitySummary.score,
      packageAverageScore: forgeRealitySummary.packageAverageScore,
      scoreCeilingWithoutLiveProof: forgeRealitySummary.scoreCeilingWithoutLiveProof,
      unboundedSourceScore: forgeRealitySummary.unboundedSourceScore,
      scoreComponents: forgeRealitySummary.scoreComponents,
      scoreGate: forgeRealitySummary.scoreGate,
      launchEvidenceSummaryRows: launchEvidenceSummaryRows.map((row) => ({
        id: row.id,
        label: row.label,
        value: row.value,
        status: row.status,
        statusLabel: row.statusLabel,
        description: row.description,
        scoreImpact: row.scoreImpact,
        iconName: row.iconName,
        routes: "routes" in row ? row.routes : [],
        packageIds: "packageIds" in row ? row.packageIds : [],
      })),
      lockBackedPackageCount: forgeRealitySummary.lockBackedPackageCount,
      visiblePackageLaneCount: forgeRealitySummary.totalVisiblePackageLanes,
      realControlCount: forgeRealitySummary.realControlCount,
      readinessOnlyLaneCount: forgeRealitySummary.statusOnlyPackageCount,
      currentLockBackedManifests: forgeRealitySummary.currentLockBackedManifests,
      physicalCacheManifestCount,
      stalePhysicalCacheManifestCount,
      stalePhysicalCacheManifestPaths,
      cacheArchiveRoot: forgeRealitySummary.cacheArchiveRoot,
      archivedCacheManifestCount: archivedCacheManifestPaths.length,
      archivedCacheManifestPaths,
      providerGatedCount: forgeRealitySummary.providerGatedCount,
      providerBoundaryCoverage: forgeRealitySummary.providerBoundaryCoverage,
      readinessExecutionProofCount:
        forgeRealitySummary.readinessExecutionProofCount,
      readinessExecutionProofPackageCount:
        forgeRealitySummary.readinessExecutionProofPackageCount,
      readinessExecutionProofPackageIds:
        forgeRealitySummary.readinessExecutionProofPackageIds,
      readinessExecutionProofCoverage:
        forgeRealitySummary.readinessExecutionProofCoverage,
      adapterBoundaryReadinessCount:
        forgeRealitySummary.adapterBoundaryReadinessCount,
      sourceOwnedLimitedProofCount:
        forgeRealitySummary.sourceOwnedLimitedProofCount,
      sourceGuardOnlyCount: forgeRealitySummary.sourceGuardOnlyCount,
      dummyOrMisleadingCount: forgeRealitySummary.dummyOrMisleadingCount,
      cacheManifestSource: forgeRealitySummary.cacheManifestSource,
      cacheManifestCaveatId: forgeRealitySummary.cacheManifestCaveatId,
      cacheArchiveCaveatId: forgeRealitySummary.cacheArchiveCaveatId,
      maturityCoverage: collectPackageMaturityCoverage(),
      templateReadinessReceipts: collectTemplateReadinessReceipts(),
    },
    forgePackageRealityRows: forgeRealityRows.map((row) => ({
      packageId: row.packageId,
      packageName: row.packageName,
      maturityKind: row.maturityKind,
      realityLevelId: row.realityLevelId,
      score: row.score,
      receiptStatus: row.receiptStatus,
      templateSurface: row.controlId ? "interactive-control" : "readiness-only",
      hasInteractiveControl: Boolean(row.controlId),
      controlId: row.controlId ?? null,
    })),
    templateReadinessRouteHandlers: [
      {
        route: "/api/payments/stripe-js/readiness",
        packageId: "payments/stripe-js",
        readinessKind: "provider-gated",
        sourceFile: "app/api/payments/stripe-js/readiness/route.ts",
        methods: ["GET", "POST"],
        runtimeExecution: false,
        liveProviderExecution: false,
        missingConfigHttpStatus: 501,
      },
      {
        route: "/api/automations/n8n/dry-run",
        packageId: "automations/n8n",
        readinessKind: "provider-gated",
        sourceFile: "app/api/automations/n8n/dry-run/route.ts",
        methods: ["GET", "POST"],
        runtimeExecution: false,
        liveProviderExecution: false,
        missingConfigHttpStatus: 501,
      },
      {
        route: "/api/database-api/readiness",
        packageId: "api/trpc",
        readinessKind: "source-owned-adapter-boundary",
        sourceFile: "app/api/database-api/readiness/route.ts",
        methods: ["GET"],
        runtimeExecution: false,
        liveProviderExecution: false,
        readinessHttpStatus: 200,
      },
      {
        route: "/api/query-cache/readiness",
        packageId: "tanstack/query",
        readinessKind: "source-owned-adapter-boundary",
        sourceFile: "app/api/query-cache/readiness/route.ts",
        methods: ["GET", "POST"],
        runtimeExecution: false,
        liveProviderExecution: false,
        readinessHttpStatus: 200,
        unsupportedActionHttpStatus: 400,
      },
      {
        route: "/api/instant/readiness",
        packageId: "instantdb/react",
        readinessKind: "provider-gated",
        sourceFile: "app/api/instant/readiness/route.ts",
        methods: ["GET"],
        runtimeExecution: false,
        liveProviderExecution: false,
        missingConfigHttpStatus: 501,
      },
      {
        route: "/api/database-orm/readiness",
        packageId: "db/drizzle-sqlite",
        readinessKind: "runtime-gated",
        sourceFile: "app/api/database-orm/readiness/route.ts",
        methods: ["GET"],
        runtimeExecution: false,
        liveProviderExecution: false,
        missingConfigHttpStatus: 501,
      },
      {
        route: "/api/supabase/readiness",
        packageId: "supabase/client",
        readinessKind: "provider-gated",
        sourceFile: "app/api/supabase/readiness/route.ts",
        methods: ["GET"],
        runtimeExecution: false,
        liveProviderExecution: false,
        missingConfigHttpStatus: 501,
      },
    ],
    nativeScrollProof: collectNativeScrollProof(projectDir),
    styleEvidenceRows: [DX_STYLE_BROWSER_COMPAT_PREVIEW_EVIDENCE],
    styleEvidenceDriftFixtures: [DX_STYLE_BROWSER_COMPAT_DRIFT_FIXTURE],
    stylePackageOwnershipRows: DX_STYLE_PACKAGE_OWNERSHIP_ROWS,
    sourceGuardRunbookFixtures: [
      FORGE_SAFETY_ARCHIVE_SOURCE_GUARD_RUNBOOK_FIXTURE,
      AUTHENTICATION_SOURCE_GUARD_RUNBOOK_FIXTURE,
      UI_COMPONENTS_SOURCE_GUARD_RUNBOOK_FIXTURE,
      STATE_MANAGEMENT_SOURCE_GUARD_RUNBOOK_FIXTURE,
      REACTIVE_STORE_SOURCE_GUARD_RUNBOOK_FIXTURE,
      DATA_FETCHING_CACHE_SOURCE_GUARD_RUNBOOK_FIXTURE,
      VALIDATION_SCHEMAS_SOURCE_GUARD_RUNBOOK_FIXTURE,
      PAYMENTS_SOURCE_GUARD_RUNBOOK_FIXTURE,
      FORMS_SOURCE_GUARD_RUNBOOK_FIXTURE,
      WEBASSEMBLY_BRIDGE_SOURCE_GUARD_RUNBOOK_FIXTURE,
      MOTION_ANIMATION_SOURCE_GUARD_RUNBOOK_FIXTURE,
      THREE_SCENE_SYSTEM_SOURCE_GUARD_RUNBOOK_FIXTURE,
      DOCUMENTATION_SYSTEM_SOURCE_GUARD_RUNBOOK_FIXTURE,
      AUTOMATION_CONNECTORS_SOURCE_GUARD_RUNBOOK_FIXTURE,
      INTERNATIONALIZATION_SOURCE_GUARD_RUNBOOK_FIXTURE,
      TYPE_SAFE_API_SOURCE_GUARD_RUNBOOK_FIXTURE,
      DATABASE_ORM_SOURCE_GUARD_RUNBOOK_FIXTURE,
      BACKEND_PLATFORM_CLIENT_SOURCE_GUARD_RUNBOOK_FIXTURE,
      AI_SDK_SOURCE_GUARD_RUNBOOK_FIXTURE,
    ],
    routes: [
      {
        route: "/login",
        sourceFile: "pages/login.html",
        forgePackages: [
          "auth/better-auth",
          "forms/react-hook-form",
          "validation/zod",
          "shadcn/ui/button",
          "shadcn/ui/input",
        ],
        hotReloadTarget: "route:/login",
      },
      {
        route: "/logout",
        sourceFile: "pages/logout.html",
        forgePackages: ["auth/better-auth", "shadcn/ui/button", "shadcn/ui/card"],
        hotReloadTarget: "route:/logout",
      },
      {
        route: "/dashboard",
        sourceFile: "pages/dashboard.html",
        forgePackages: DASHBOARD_FORGE_PACKAGE_IDS,
        hotReloadTarget: "route:/dashboard",
      },
      {
        route: "/",
        sourceFile: "pages/index.html",
        forgePackages: [
          "shadcn/ui/button",
          "shadcn/ui/badge",
          "shadcn/ui/card",
          "shadcn/ui/field",
          "shadcn/ui/input",
          "shadcn/ui/textarea",
          "shadcn/ui/item",
          "shadcn/ui/separator",
          "auth/better-auth",
          "i18n/next-intl",
          "payments/stripe-js",
          "validation/zod",
          "forms/react-hook-form",
          "state/zustand",
          "reactive/store",
          "tanstack/query",
          "animation/motion",
          "3d/launch-scene",
          "content/react-markdown",
          "content/fumadocs-next",
          "automations/n8n",
          "supabase/client",
          "db/drizzle-sqlite",
          "instantdb/react",
          "api/trpc",
          "ai/vercel-ai",
          "wasm/bindgen",
        ],
        hotReloadTarget: "route:/",
        styleEvidenceRows: [DX_STYLE_BROWSER_COMPAT_PREVIEW_EVIDENCE],
        styleEvidenceDriftFixtures: [DX_STYLE_BROWSER_COMPAT_DRIFT_FIXTURE],
        stylePackageOwnershipRows: DX_STYLE_PACKAGE_OWNERSHIP_ROWS,
        sourceGuardRunbookFixtures: [
          FORGE_SAFETY_ARCHIVE_SOURCE_GUARD_RUNBOOK_FIXTURE.fixture,
          AUTHENTICATION_SOURCE_GUARD_RUNBOOK_FIXTURE.fixture,
          UI_COMPONENTS_SOURCE_GUARD_RUNBOOK_FIXTURE.fixture,
          STATE_MANAGEMENT_SOURCE_GUARD_RUNBOOK_FIXTURE.fixture,
          REACTIVE_STORE_SOURCE_GUARD_RUNBOOK_FIXTURE.fixture,
          DATA_FETCHING_CACHE_SOURCE_GUARD_RUNBOOK_FIXTURE.fixture,
          VALIDATION_SCHEMAS_SOURCE_GUARD_RUNBOOK_FIXTURE.fixture,
          PAYMENTS_SOURCE_GUARD_RUNBOOK_FIXTURE.fixture,
          FORMS_SOURCE_GUARD_RUNBOOK_FIXTURE.fixture,
          WEBASSEMBLY_BRIDGE_SOURCE_GUARD_RUNBOOK_FIXTURE.fixture,
          MOTION_ANIMATION_SOURCE_GUARD_RUNBOOK_FIXTURE.fixture,
          THREE_SCENE_SYSTEM_SOURCE_GUARD_RUNBOOK_FIXTURE.fixture,
          DOCUMENTATION_SYSTEM_SOURCE_GUARD_RUNBOOK_FIXTURE.fixture,
          AUTOMATION_CONNECTORS_SOURCE_GUARD_RUNBOOK_FIXTURE.fixture,
          INTERNATIONALIZATION_SOURCE_GUARD_RUNBOOK_FIXTURE.fixture,
          TYPE_SAFE_API_SOURCE_GUARD_RUNBOOK_FIXTURE.fixture,
          DATABASE_ORM_SOURCE_GUARD_RUNBOOK_FIXTURE.fixture,
          BACKEND_PLATFORM_CLIENT_SOURCE_GUARD_RUNBOOK_FIXTURE.fixture,
          AI_SDK_SOURCE_GUARD_RUNBOOK_FIXTURE.fixture,
        ],
        dataDxMarkers: [
          "data-dx-route",
          "data-dx-component",
          "data-dx-package",
          "data-dx-editable-section",
          "data-dx-operation",
          "data-dx-dashboard-flow",
          "data-dx-dashboard-target",
          "data-dx-product-surface",
          "data-dx-check-style-evidence-count",
          "data-dx-check-style-evidence-row",
          "data-dx-check-style-evidence-status",
          "data-dx-check-style-evidence-receipt-path",
          "data-dx-check-style-evidence-fixture-path",
          "data-dx-check-style-evidence-zed",
          "data-dx-check-style-evidence-class-count",
          "data-dx-check-style-evidence-selector-class-count",
          "data-dx-check-style-evidence-selector-class-examples",
          "data-dx-check-style-evidence-state-alias-count",
          "data-dx-check-style-evidence-state-alias-examples",
          "data-dx-check-style-evidence-full-autoprefixer-parity",
          "data-dx-check-style-evidence-full-tailwind-postcss-output-parity",
          "data-dx-check-style-evidence-drift",
          "data-dx-check-style-evidence-drift-state",
          "data-dx-check-style-evidence-drift-loader",
          "data-dx-check-style-evidence-drift-helper",
          "data-dx-check-style-evidence-drift-states",
          "data-dx-style-package-panel",
          "data-dx-style-package-panel-read-model",
          "data-dx-style-package-panel-drift-state",
          "data-dx-style-package-panel-drift-status",
          "data-dx-style-package-panel-drift-mismatch-fields",
          "data-dx-style-package-panel-readiness-receipt",
          "data-dx-check-package-lane-template",
          "data-dx-check-package-lane-row",
          "data-dx-check-package-lane-dx-style-status",
          "data-dx-check-package-lane-hash-refresh-status",
          "data-dx-check-package-lane-hash-refresh-helper",
          "data-dx-check-package-lane-hash-refresh-json-command",
          "data-dx-check-package-lane-hash-refresh-zed",
          "data-dx-check-package-lane-hash-refresh-tracked-files",
          "data-dx-check-package-lane-hash-refresh-stale-files",
          "data-dx-check-package-lane-hash-refresh-missing-files",
          "data-dx-check-package-lane-hash-refresh-current-file-list",
          "data-dx-check-package-lane-hash-refresh-stale-file-list",
          "data-dx-check-package-lane-hash-refresh-missing-file-list",
          "data-dx-check-package-lane-hash-refresh-stale-mirror-file-list",
          "data-dx-check-package-lane-hash-refresh-missing-mirror-file-list",
          "data-dx-check-package-lane-hash-refresh-current-metric",
          "data-dx-check-package-lane-hash-refresh-stale-metric",
          "data-dx-check-package-lane-hash-refresh-missing-metric",
          "data-dx-safety-archive-contract",
          "data-dx-safety-archive-state",
          "data-dx-safety-archive-rollback-coverage",
          "data-dx-safety-archive-receipt-count",
          "data-dx-safety-archive-safe-delete",
          "data-dx-style-surface",
          "data-dx-token-scope",
          "data-dx-intl-dashboard-workflow",
          "data-dx-query-dashboard-source",
          "data-dx-query-dashboard-queue",
          "data-dx-query-dashboard-package-count",
          "data-dx-query-dashboard-role-count",
          "data-dx-query-action",
          "data-dx-query-safe-action",
          "data-dx-query-result-status",
          "data-dx-scene-quality-profile",
          "data-dx-scene-material-palette",
          "data-dx-scene-camera-rig",
          "data-dx-scene-frame-sample",
          "data-dx-scene-capability-report",
          "data-dx-scene-capability-status",
          "data-dx-scene-viewport-report",
          "data-dx-scene-viewport-status",
          "data-dx-scene-bounds-report",
          "data-dx-scene-bounds-status",
          "data-dx-scene-raycast-report",
          "data-dx-scene-raycast-status",
          "data-dx-scene-workflow-active",
          "data-dx-scene-workflow-receipt-state",
          "data-dx-stripe-dashboard-workflow",
          "data-dx-stripe-action",
          "data-dx-stripe-receipt-path",
          "data-dx-drizzle-action",
          "data-dx-drizzle-status",
          "data-dx-drizzle-read-model",
          "data-dx-drizzle-query-plan-id",
          "data-dx-drizzle-mission-control",
          "data-dx-drizzle-sql-preview",
          "data-dx-drizzle-fixture-row",
          "data-dx-drizzle-receipt-path",
          "data-dx-drizzle-receipt-state",
          "data-dx-drizzle-runtime-dependencies",
          "data-dx-query-dashboard-source",
          "data-dx-query-dashboard-queue",
          "data-dx-query-dashboard-package-count",
          "data-dx-query-dashboard-role-count",
          "data-dx-query-dashboard-required-env-count",
          "data-dx-query-package-id",
          "data-dx-query-package-role",
          "data-dx-query-package-status",
          "data-dx-query-action",
          "data-dx-query-safe-action",
          "data-dx-query-result-status",
          "data-dx-backend-status",
          "data-dx-backend-detail",
          "data-dx-trpc-workflow",
          "data-dx-trpc-action",
          "data-trpc-interaction",
          "data-dx-fumadocs-action",
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
      },
      { route: "/automations", sourceFile: "pages/automations.html", forgePackages: ["automations/n8n"], hotReloadTarget: "route:/automations" },
      { route: "/ui", sourceFile: "pages/ui.html", forgePackages: ["shadcn/ui/card", "shadcn/ui/button", "shadcn/ui/input", "dx/icon/search"], hotReloadTarget: "route:/ui" },
      { route: "/database", sourceFile: "pages/database.html", forgePackages: ["supabase/client", "db/drizzle-sqlite", "instantdb/react"], hotReloadTarget: "route:/database" },
      { route: "/backend", sourceFile: "pages/backend.html", forgePackages: ["api/trpc", "ai/vercel-ai"], hotReloadTarget: "route:/backend" },
    ],
    editableOperations: [
      "insert_component",
      "move_reorder_section",
      "update_design_token",
      "update_text_content",
      "insert_icon_media",
    ],
    editContract: buildEditContract(noNodeModulesRequired),
    files,
  };
  const target = path.join(projectDir, "public", "preview-.dx/build-cache/manifest.json");
  writeText(target, `${JSON.stringify(manifest, null, 2)}\n`);
  return path.relative(projectDir, target).replaceAll("\\", "/");
}

function main() {
  const projectArg = process.argv[2];
  if (!projectArg) usage();
  const projectDir = path.resolve(projectArg);
  if (!fs.existsSync(projectDir)) {
    console.error(`Project directory does not exist: ${projectDir}`);
    process.exit(1);
  }

  const shouldDisableConflictingRoutes =
    path.relative(templateRoot, projectDir) !== "";
  const disabledRoutes = shouldDisableConflictingRoutes
    ? [
        disableConflictingAppRoute(projectDir, []),
        disableConflictingAppRoute(projectDir, ["launch"]),
      ].filter(Boolean)
    : [];
  const files = [
    ...materializePages(projectDir),
    ...materializeAssets(projectDir),
    ...materializeAuthPackageSource(projectDir),
    ...materializeAuthenticationServerSource(projectDir),
    ...materializeDataFetchingCacheServerSource(projectDir),
    ...materializeProviderBoundarySource(projectDir),
    ...materializeRouteHandlers(projectDir),
    ...materializeForgeReceipts(projectDir),
    ...materializeForgeTemplateReadiness(projectDir),
  ];
  const previewManifest = materializePreviewManifest(projectDir, files);

  const result = {
    ok: true,
    projectDir,
    disabledRoute: disabledRoutes[0] ?? null,
    disabledRoutes,
    files: [...files, previewManifest],
    noNodeModules: !fs.existsSync(path.join(projectDir, "node_modules")),
  };
  console.log(JSON.stringify(result, null, 2));
}

main();
