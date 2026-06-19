import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const materializer = path.join(root, "tools", "launch", "materialize-www-template.ts");
const dashboardForgePackageIds = [
  "shadcn/ui/button",
  "state/zustand",
  "tanstack/query",
  "reactive/store",
  "db/drizzle-sqlite",
  "validation/zod",
  "forms/react-hook-form",
  "instantdb/react",
  "content/fumadocs-next",
  "auth/better-auth",
  "i18n/next-intl",
  "api/trpc",
  "ai/vercel-ai",
  "payments/stripe-js",
  "supabase/client",
  "automations/n8n",
  "wasm/bindgen",
  "3d/launch-scene",
  "animation/motion",
  "content/react-markdown",
];
const realLockBackedForgePackageIds = [
  "shadcn/ui/button",
  "state/zustand",
  "tanstack/query",
  "validation/zod",
  "forms/react-hook-form",
];

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function cssRule(css, selector, label) {
  const normalized = css.replace(/\r\n/g, "\n");
  const escapedSelector = selector.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const rulePattern = new RegExp(`(?:^|\\n)\\s*${escapedSelector}\\s*\\{([^}]*)\\}`, "g");
  const match = rulePattern.exec(normalized);
  assert.ok(match, `${label} should define ${selector}`);
  return match[1];
}

function assertNativeScrollShell(html, label) {
  assert.match(html, /<body(?:\s|>)/, `${label} should render a body element`);
  assert.doesNotMatch(
    html,
    /<body[^>]*style=["'][^"']*(?:overflow|position)\s*:\s*(?:hidden|clip|fixed)/i,
    `${label} must not lock native body scrolling inline`,
  );
  assert.doesNotMatch(
    html,
    /data-dx-component="template-custom-scrollbar"|data-dashboard-scrollbar|data-custom-scrollbar/i,
    `${label} must not reintroduce custom scrollbar surfaces`,
  );
}

function assertNativeScrollCss(css, label) {
  const htmlRule = cssRule(css, "html", label);
  const bodyRule = cssRule(css, "body", label);

  assert.match(
    htmlRule,
    /overflow-y:\s*auto;/,
    `${label} should explicitly keep document vertical scrolling native`,
  );
  assert.match(
    bodyRule,
    /overflow-y:\s*auto;/,
    `${label} should explicitly keep body vertical scrolling native`,
  );
  assert.doesNotMatch(
    `${htmlRule}\n${bodyRule}`,
    /(?:^|[;\r\n])\s*overflow(?:-y)?:\s*(?:hidden|clip)\s*;/i,
    `${label} must not disable vertical page scrolling on html/body`,
  );
  assert.doesNotMatch(
    `${htmlRule}\n${bodyRule}`,
    /(?:^|[;\r\n])\s*(?:height|max-height):\s*100vh/i,
    `${label} must not cap html/body height to the viewport`,
  );
}

test("template exposes a real app shell without visible dev copy", () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-template-app-source-"));
  execFileSync(process.execPath, [materializer, dir], { cwd: root, stdio: "pipe" });

  const landing = read("tools/launch/runtime-template/pages/index.html");
  const materializedLanding = fs.readFileSync(path.join(dir, "pages", "index.html"), "utf8");
  const materializedLogin = fs.readFileSync(path.join(dir, "pages", "login.html"), "utf8");
  const login = read("tools/launch/runtime-template/pages/login.html");
  const logout = read("tools/launch/runtime-template/pages/logout.html");
  const dashboard = fs.readFileSync(path.join(dir, "pages", "dashboard.html"), "utf8");
  const runtime = read("tools/launch/runtime-template/assets/launch-runtime.ts");
  const runtimeCss = read("tools/launch/runtime-template/assets/launch-runtime.css");
  const globals = read("examples/template/styles/globals.css");
  const theme = read("examples/template/styles/theme.css");
  const homeRoute = read("examples/template/app/page.tsx");
  const loginRoute = read("examples/template/app/login/page.tsx");
  const logoutRoute = read("examples/template/app/logout/page.tsx");
  const dashboardRoute = read("examples/template/app/dashboard/page.tsx");
  const layoutRoute = read("examples/template/app/layout.tsx");
  const dxUi = read("examples/template/components/template-app/dx-ui.tsx");
  const materializerSource = read("tools/launch/materialize-www-template.ts");
  const iconSource = read("examples/template/components/icons/icon.tsx");
  const landingSource = read("examples/template/components/template-app/landing-page.tsx");
  const dashboardSource = read("examples/template/components/template-app/dashboard-page.tsx");
  const authSource = read("examples/template/components/template-app/auth-pages.tsx");
  const formsSource = read("examples/template/components/template-app/forms.tsx");
  const formsFieldsSource = read("examples/template/lib/forms/react-hook-form/fields.tsx");
  const templateFormsValidationSource = read(
    "examples/template/lib/validation/zod/template-forms.ts",
  );
  const templateData = read("examples/template/components/template-app/template-data.ts");
  const packageStatus = JSON.parse(read("examples/template/.dx/forge/package-status.json"));
  const selectPrimitivePath = path.join(root, "examples/template/components/ui/select.tsx");
  const formsDryRunReceiptPath = path.join(
    root,
    "examples/template/lib/forms/react-hook-form/dry-run-receipt.ts",
  );
  assert.ok(
    fs.existsSync(selectPrimitivePath),
    "Forms package should materialize a source-owned select primitive for enum fields",
  );
  assert.ok(
    fs.existsSync(formsDryRunReceiptPath),
    "Forms package should materialize a source-owned dry-run receipt helper",
  );
  const selectPrimitiveSource = fs.readFileSync(selectPrimitivePath, "utf8");
  const formsDryRunReceiptSource = fs.readFileSync(formsDryRunReceiptPath, "utf8");

  assert.equal(fs.existsSync(path.join(root, "tools/launch/runtime-template/pages/dashboard.html")), false);
  assert.equal(fs.existsSync(path.join(root, "tools/launch/runtime-template/assets/launch-runtime.js")), false);
  assert.match(landing, /data-dx-route="\/"/);
  assert.match(landing, /data-dx-component="launch-scene-webgl-proof"/);
  assert.match(landing, /data-dx-component="launch-operating-dashboard"/);
  assert.match(landing, /<canvas[^>]+id="dx-launch-scene"/);
  assert.doesNotMatch(
    landing,
    /template-hero-copy|template-actions|template-preview|template-feature-card|Start workspace|View dashboard|Workflows|Teams/,
  );
  assert.match(materializedLanding, /data-dx-route="\/"/);
  assert.match(materializedLanding, /data-dx-component="launch-scene-webgl-proof"/);
  assert.doesNotMatch(materializedLanding, /template-hero-copy|template-actions|template-preview/);
  assert.match(login, /id="app-login-form"/);
  assert.match(login, /data-dx-package="auth\/better-auth forms\/react-hook-form validation\/zod/);
  assert.match(login, /data-dx-component="template-login-form"/);
  assert.match(login, /data-dx-rhf-boundary="runtime-safe-form"/);
  assert.match(login, /data-dx-zod-schema="templateLoginSchema"/);
  assert.match(login, /data-template-field-error="email"/);
  assert.match(login, /data-template-field-error="password"/);
  assert.match(logout, /id="app-logout-action"/);
  assert.match(dashboard, /data-dx-component="template-dashboard-page"/);
  assert.match(dashboard, /Dashboard \| www/);
  assert.match(dashboard, /<html lang="en" data-theme="dark">/);
  assert.match(dashboard, /data-dx-component="forge-package-reality-dashboard"/);
  assert.match(dashboard, /Authentication source is present/);
  assert.match(dashboard, /Launch readiness tracked/);
  assert.match(dashboard, /Package set/);
  assert.match(dashboard, /Refresh status/);
  assert.match(dashboard, /The installed package set is aligned; live integrations stay gated\./);
  assert.doesNotMatch(
    dashboard,
    /Better Auth source is present|Forge reality score|Refresh proof|Status and lock agree; live provider proof is tracked separately|Current package readiness across the visible template lanes/,
  );
  const realLockBackedCount = Number(
    dashboard.match(/data-dx-forge-real-lock-backed-count="(\d+)"/)?.[1] ?? 0,
  );
  assert.ok(
    realLockBackedCount >= realLockBackedForgePackageIds.length,
    "dashboard should preserve the lock-backed Forge package baseline",
  );
  assert.match(dashboard, /data-dx-forge-status-lane-count="\d+"/);
  assert.match(dashboard, /class="dashboard-mobile-menu"/);
  assert.match(dashboard, /class="dashboard-header-nav"/);
  assert.doesNotMatch(dashboard, /data-dx-component="template-custom-scrollbar"/);
  assert.doesNotMatch(dashboard, /data-dashboard-scrollbar|data-dashboard-scrollbar-track|data-dashboard-scrollbar-thumb/);
  assert.doesNotMatch(dashboard, /data-scroll-reveal|dashboard-scrollbar-button|data-dashboard-scroll-action/);
  assert.match(dashboard, /data-icon-source="dx-icons"/);
  assert.match(dashboard, /data-dx-icon="nav:dashboard"/);
  assert.match(dashboard, /data-dx-icon="action:menu"/);
  assert.match(dashboard, /class="dashboard-footer-primary"/);
  assert.match(dashboard, /class="dashboard-footer-status"/);
  assert.match(dashboard, /data-theme-toggle/);
  assert.match(dashboard, /id="dashboard-settings-form"/);
  assert.match(dashboard, /data-dx-component="template-settings-form"/);
  assert.match(dashboard, /data-dx-component="template-profile-form"/);
  assert.match(dashboard, /data-dx-component="template-billing-contact-form"/);
  assert.match(dashboard, /data-dx-zod-schema="templateWorkspaceSettingsSchema"/);
  assert.match(dashboard, /data-dx-zod-schema="templateProfileSchema"/);
  assert.match(dashboard, /data-dx-zod-schema="templateBillingContactSchema"/);
  assert.match(dashboard, /<select[^>]+id="dashboard-billing-plan"[^>]+name="plan"/);
  assert.match(dashboard, /data-dx-zod-enum-options="starter team scale"/);
  assert.match(dashboard, /<option value="starter">Starter<\/option>/);
  assert.match(dashboard, /<option value="team" selected>Team<\/option>/);
  assert.match(dashboard, /<option value="scale">Scale<\/option>/);
  assert.match(dashboard, /data-template-field-error="workspaceName"/);
  assert.match(dashboard, /data-template-field-error="displayName"/);
  assert.match(dashboard, /data-template-field-error="billingEmail"/);
  assert.match(dashboard, /data-template-form-status="billing-contact"/);
  assert.match(dashboard, /data-dx-form-dry-run-receipt="idle"/);
  assert.match(dashboard, /data-dx-form-submit-mode="local-dry-run"/);
  assert.match(dashboard, /data-dx-form-persistence="none"/);
  assert.match(dashboard, /data-dx-form-secret-access="false"/);
  assert.match(dashboard, /data-dashboard-filter="ready"/);
  assert.match(dashboard, /data-dx-state-storage-key="dx-template-workspace-state"/);
  assert.match(dashboard, /data-dx-component="forge-package-reality-dashboard-shell"/);
  assert.match(dashboard, /id="tools"/);
  assert.match(dashboard, /data-dx-source="examples\/template\/components\/template-app\/dashboard-page\.tsx"/);
  assert.match(dashboard, /Generated from examples\/template\/components\/template-app\/dashboard-page\.tsx/);
  for (const packageId of dashboardForgePackageIds) {
    assert.match(dashboard, new RegExp(packageId.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }
  for (const packageId of realLockBackedForgePackageIds) {
    assert.match(templateData, new RegExp(packageId.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }
  assert.match(dashboard, /content\/react-markdown/);
  assert.doesNotMatch(dashboard, /Code Stack|codestack|DX Template/i);
  assert.match(dashboardSource, /export function TemplateDashboardPage/);
  assert.match(dashboardSource, /WorkspaceSettingsForm/);
  assert.match(dashboardSource, /ProfileSettingsForm/);
  assert.match(dashboardSource, /BillingContactForm/);
  assert.match(authSource, /LoginBoundaryForm/);
  assert.match(formsSource, /createDxZodResolver/);
  assert.match(formsSource, /DxHookForm/);
  assert.match(formsSource, /createDxFormDryRunReceipt/);
  assert.match(
    formsSource,
    /@\/lib\/validation\/zod\/template-forms/,
    "template forms must consume source-owned Validation & Schemas definitions",
  );
  assert.doesNotMatch(
    formsSource,
    /import \{ z \} from "zod"/,
    "template form schemas should not be embedded in the Forms component",
  );
  assert.match(formsSource, /templateLoginSchema/);
  assert.match(formsSource, /templateWorkspaceSettingsSchema/);
  assert.match(formsSource, /templateProfileSchema/);
  assert.match(formsSource, /templateBillingContactSchema/);
  assert.match(formsSource, /templateBillingPlanOptions/);
  assert.match(formsSource, /DxSelectField<TemplateBillingContactValues, "plan">/);
  assert.doesNotMatch(formsSource, /DxInputField<TemplateBillingContactValues, "plan">/);
  assert.match(formsFieldsSource, /export function DxSelectField/);
  assert.match(formsFieldsSource, /data-dx-field-kind="select"/);
  assert.match(formsDryRunReceiptSource, /export function createDxFormDryRunReceipt/);
  assert.match(formsDryRunReceiptSource, /schema: "dx\.forms\.dry_run_receipt"/);
  assert.match(formsDryRunReceiptSource, /persistence: "none"/);
  assert.match(formsDryRunReceiptSource, /secretAccess: false/);
  assert.match(templateFormsValidationSource, /export const templateLoginSchema/);
  assert.match(templateFormsValidationSource, /export const templateWorkspaceSettingsSchema/);
  assert.match(templateFormsValidationSource, /export const templateProfileSchema/);
  assert.match(templateFormsValidationSource, /export const templateBillingContactSchema/);
  assert.match(templateFormsValidationSource, /export function safeParseTemplateForm/);
  assert.match(templateFormsValidationSource, /schema: "dx\.validation\.template_forms"/);
  assert.match(templateFormsValidationSource, /z\.strictObject/);
  assert.match(templateFormsValidationSource, /z\.flattenError/);
  assert.match(templateFormsValidationSource, /packageId: "validation\/zod"/);
  assert.match(selectPrimitiveSource, /export const Select/);
  assert.match(selectPrimitiveSource, /className=\{cn\("cn-select"/);
  assert.match(dashboardSource, /ForgeRealityPanel/);
  assert.doesNotMatch(dashboardSource, /templatePackageModules\.map/);
  assert.match(dashboardSource, /className="dashboard-shell"/);
  assert.match(dashboardSource, /data-dx-forge-reality-score=\{forgeRealitySummary\.score\}/);
  assert.match(dashboardSource, /id="tools"/);
  assert.match(dashboard, /href="\/styles\/globals\.css"/);
  assert.match(homeRoute, /TemplateLandingPage/);
  assert.match(homeRoute, /App Router authoring/);
  assert.doesNotMatch(homeRoute, /TemplateShell|templateRouteContract|DxIntlProvider/);
  assert.match(materializerSource, /function disableConflictingAppRoute/);
  assert.match(materializerSource, /path\.resolve\(projectDir\) === sourceWwwTemplateRoot/);
  assert.match(materializerSource, /const shouldDisableConflictingRoutes/);
  assert.match(landingSource, /LandingSceneSurface/);
  assert.match(landingSource, /data-dx-scene-mode="pure-visual"/);
  assert.doesNotMatch(
    landingSource,
    /Build faster from source-owned DX packages|Start workspace|View dashboard|DxCard|DxLinkButton/,
  );
  assert.match(layoutRoute, /"\.\.\/styles\/globals\.css"/);
  assert.doesNotMatch(layoutRoute, /theme\.css|generated\.css|app\.generated\.css/);
  assert.match(loginRoute, /TemplateLoginPage/);
  assert.match(logoutRoute, /TemplateLogoutPage/);
  assert.match(dashboardRoute, /TemplateDashboardPage/);
  assert.match(dxUi, /export function DxButton/);
  assert.match(dxUi, /export \{ Icon \} from "\.\.\/icons\/icon"|export function Icon/);
  assert.match(iconSource, /data-icon-source="dx-icons"/);
  assert.match(iconSource, /data-dx-icon=\{canonicalName\}/);
  assert.match(dxUi, /"template-button"/);
  assert.match(dxUi, /"template-button-primary"/);
  assert.match(dxUi, /"template-card"/);
  assert.match(dxUi, /"template-input"/);
  assert.doesNotMatch(dashboardSource, /#[0-9a-fA-F]{3,8}|rgb\(|rgba\(/);
  assert.match(globals, /\.template-button-primary:hover,[\s\S]*color: var\(--dx-ui-primary-fg\)/);
  assert.match(globals, /\.cn-select/);
  assert.match(globals, /--dx-template-background: var\(--dx-bg\)/);
  assert.doesNotMatch(globals, /radial-gradient\(circle at 18%|radial-gradient\(circle at 82%/);

  for (const html of [landing, login, logout, dashboard]) {
    assert.doesNotMatch(html, />[^<]*(dx add|Forge package usage)/i);
  }

  const lanes = packageStatus.package_lane_visibility;
  const lockBackedPackageIds = packageStatus.locked_package_names;
  for (const packageId of realLockBackedForgePackageIds) {
    assert.ok(
      lockBackedPackageIds.includes(packageId),
      `${packageId} should stay in the lock-backed baseline set`,
    );
  }
  const markdownLane = lanes.find((lane) => lane.package_id === "content/react-markdown");
  assert.ok(markdownLane, "content/react-markdown lane should remain visible");
  assert.ok(
    ["present", "missing-receipt", "stale", "blocked"].includes(
      markdownLane.receipt_status,
    ),
    "content/react-markdown lane should report a known receipt state",
  );

  assert.match(runtime, /function bindTemplateApp\(\)/);
  assert.match(runtime, /templateFormsValidationContract/);
  assert.match(runtime, /function setTemplateFieldError/);
  assert.match(runtime, /function validateTemplateForm/);
  assert.match(runtime, /function createTemplateFormDryRunReceipt/);
  assert.match(runtime, /dx\.forms\.dry_run_receipt/);
  assert.match(runtime, /data-dx-form-dry-run-receipt/);
  assert.match(runtime, /dashboard-billing-plan/);
  assert.match(runtime, /\["starter", "team", "scale"\]/);
  assert.match(runtime, /dx-template-auth-boundary-review/);
  assert.doesNotMatch(runtime, /dx-template-workspace-session/);
  assert.match(runtime, /dx-template-workspace-state/);
  assert.match(runtime, /window\.location\.href = "\/dashboard"/);
  assert.match(runtime, /data-dashboard-filter/);
  assert.match(runtime, /function bindTemplateModules\(\)/);
  assert.match(runtime, /function bindTemplateTheme\(\)/);
  assert.doesNotMatch(runtime, /function bindTemplateCustomScrollbar\(\)/);
  assert.doesNotMatch(runtime, /data-dashboard-scrollbar-track/);
  assert.doesNotMatch(runtime, /scrollReveal/);
  assert.doesNotMatch(runtime, /data-dashboard-scroll-action/);
  assert.doesNotMatch(
    runtime,
    /addEventListener\(["'](?:wheel|touchmove)["'][\s\S]*?preventDefault\(/,
  );
  assert.doesNotMatch(runtime, /(?:document\.body|body)\.style\.overflow/);
  assert.doesNotMatch(runtimeCss, /::-webkit-scrollbar|scrollbar-color|scrollbar-width/);
  assertNativeScrollCss(runtimeCss, "runtime CSS");
  assertNativeScrollCss(globals, "template globals");
  assert.match(
    globals,
    /\.template-mobile-menu nav\s*\{[\s\S]*?max-height:\s*min\([\s\S]*?overflow-y:\s*auto;/,
  );
  assert.match(
    globals,
    /\.dashboard-mobile-sheet\s*\{[\s\S]*?max-height:\s*min\([\s\S]*?overflow-y:\s*auto;/,
  );
  for (const [label, html] of [
    ["/ source", landing],
    ["/ materialized", materializedLanding],
    ["/dashboard materialized", dashboard],
    ["/login source", login],
    ["/login materialized", materializedLogin],
  ]) {
    assertNativeScrollShell(html, label);
  }
  assert.match(runtime, /www-template-theme/);
  assert.match(runtime, /nextTemplateModuleStatus/);
  assert.match(runtime, /bindTemplateApp\(\);/);
  assert.match(runtime, /addEventListener\("pointermove"/);
  assert.match(runtime, /data-dx-component="template-landing-scene"|template-landing-scene/);
  assert.match(globals, /\.dashboard-shell/);
  assert.doesNotMatch(globals, /theme\.dx\.css|app\.generated\.css/);
  assert.match(theme, /--background: 0 0% 0%/);
  assert.match(theme, /\.light,[\s\S]*--background: 0 0% 100%/);
  assert.match(theme, /--font-mono: "JetBrains Mono"/);
  assert.match(globals, /font-family: var\(--font-mono\)/);
  assert.match(globals, /background: var\(--dx-template-background\)/);
  assert.match(globals, /\.dashboard-header[\s\S]*position: fixed/);
  assert.doesNotMatch(globals, /::-webkit-scrollbar|scrollbar-color|scrollbar-width/);
  assert.doesNotMatch(globals, /data-custom-scrollbar|\.dashboard-scrollbar/);
  assert.match(globals, /\.dashboard-footer-primary/);
  assert.match(globals, /\.forge-reality-dashboard/);
  assert.match(globals, /\.forge-reality-table/);
  assert.match(globals, /\.template-theme-toggle/);
  assert.match(theme, /--muted-foreground: 0 0% 86%/);
  assert.match(globals, /@media \(max-width: 900px\)/);
  assert.match(globals, /@keyframes dx-forge-motion-rise/);
  assert.match(globals, /\.package-module:hover/);
  assert.match(globals, /\.dashboard-mobile-sheet/);
  assert.match(globals, /\.dashboard-workspace/);
  assert.match(globals, /@import "\.\/theme\.css"/);
  assert.match(globals, /@import "\.\/generated\.css"/);
});

test("materialized app routes keep package receipts and avoid node_modules", () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-template-app-routes-"));
  try {
    execFileSync(process.execPath, [materializer, dir], { cwd: root, stdio: "pipe" });

    assert.equal(fs.existsSync(path.join(dir, "node_modules")), false);
    assert.ok(fs.existsSync(path.join(dir, "styles", "globals.css")), "globals.css should exist");
    assert.equal(fs.existsSync(path.join(dir, "styles", "theme.css")), false);
    assert.equal(fs.existsSync(path.join(dir, "styles", "generated.css")), false);
  for (const route of ["index", "login", "logout", "dashboard"]) {
    assert.ok(fs.existsSync(path.join(dir, "pages", `${route}.html`)), `${route}.html should exist`);
  }
  const providerBoundaryFiles = [
    [
      "app/api/payments/stripe-js/readiness/route.ts",
      /packageId: "payments\/stripe-js"|packageId: 'payments\/stripe-js'|packageId": "payments\/stripe-js"/,
      /stripeLiveExecution:\s*false/,
    ],
    [
      "app/api/automations/n8n/dry-run/route.ts",
      /packageId: "automations\/n8n"|packageId: 'automations\/n8n'|packageId": "automations\/n8n"/,
      /runtimeExecution:\s*false/,
    ],
    [
      "app/api/database-api/readiness/route.ts",
      /createDatabaseApiReadinessResponse/,
      /force-dynamic/,
    ],
    [
      "app/api/instant/readiness/route.ts",
      /createInstantReadinessResponse/,
      /force-dynamic/,
    ],
    [
      "app/api/database-orm/readiness/route.ts",
      /createDatabaseOrmReadinessResponse/,
      /force-dynamic/,
    ],
    [
      "app/api/supabase/readiness/route.ts",
      /createSupabaseReadinessResponse/,
      /force-dynamic/,
    ],
    [
      "server/database-api/readiness.ts",
      /dx\.www\.template\.database_api_readiness/,
      /runtimeProof:\s*false/,
    ],
    [
      "server/instant/readiness.ts",
      /dx\.www\.template\.instant_readiness/,
      /networkCalls:\s*false/,
    ],
    [
      "server/database-orm/readiness.ts",
      /dx\.www\.template\.database_orm_readiness/,
      /networkCalls:\s*false/,
    ],
    [
      "server/supabase/readiness.ts",
      /dx\.www\.template\.supabase_readiness/,
      /networkCalls:\s*false/,
    ],
    [
      "lib/payments/stripe-js/dashboard-checkout.ts",
      /dxStripeDashboardCheckoutReadiness/,
      /STRIPE_SECRET_KEY/,
    ],
    [
      "lib/automations/n8n/receipt.ts",
      /dx\.automation\.n8n\.run_receipt/,
      /runtimeExecution:\s*false/,
    ],
    [
      "lib/supabase/env.ts",
      /NEXT_PUBLIC_SUPABASE_URL/,
      /secretValueExposed:\s*false|valueExposed:\s*false|isLocal/,
    ],
    [
      "lib/database-api/source-contract.ts",
      /dx\.www\.template\.database_api_source_contract/,
      /hostedCredentials:\s*false/,
    ],
  ] as const;
  for (const [relativePath, primaryPattern, boundaryPattern] of providerBoundaryFiles) {
    const absolutePath = path.join(dir, relativePath);
    assert.ok(fs.existsSync(absolutePath), `${relativePath} should be materialized into the preview app`);
    const source = fs.readFileSync(absolutePath, "utf8");
    assert.match(source, primaryPattern, `${relativePath} should expose its package/readiness contract`);
    assert.match(source, boundaryPattern, `${relativePath} should keep live runtime/provider execution caveated`);
  }

  const dashboard = fs.readFileSync(path.join(dir, "pages", "dashboard.html"), "utf8");
  const manifest = JSON.parse(
    fs.readFileSync(path.join(dir, "public", "preview-manifest.json"), "utf8"),
  );

  assert.match(dashboard, /data-dx-component="template-dashboard-page"/);
  assert.match(dashboard, /data-icon-source="dx-icons"/);
  assert.equal(manifest.noNodeModulesRequired, true);
  assert.deepEqual(
    manifest.templateReadinessRouteHandlers.map((entry) => ({
      route: entry.route,
      packageId: entry.packageId,
      readinessKind: entry.readinessKind,
      runtimeExecution: entry.runtimeExecution,
      liveProviderExecution: entry.liveProviderExecution,
      sourceFile: entry.sourceFile,
    })),
    [
      {
        route: "/api/payments/stripe-js/readiness",
        packageId: "payments/stripe-js",
        readinessKind: "provider-gated",
        runtimeExecution: false,
        liveProviderExecution: false,
        sourceFile: "app/api/payments/stripe-js/readiness/route.ts",
      },
      {
        route: "/api/automations/n8n/dry-run",
        packageId: "automations/n8n",
        readinessKind: "provider-gated",
        runtimeExecution: false,
        liveProviderExecution: false,
        sourceFile: "app/api/automations/n8n/dry-run/route.ts",
      },
      {
        route: "/api/database-api/readiness",
        packageId: "api/trpc",
        readinessKind: "source-owned-adapter-boundary",
        runtimeExecution: false,
        liveProviderExecution: false,
        sourceFile: "app/api/database-api/readiness/route.ts",
      },
      {
        route: "/api/query-cache/readiness",
        packageId: "tanstack/query",
        readinessKind: "source-owned-adapter-boundary",
        runtimeExecution: false,
        liveProviderExecution: false,
        sourceFile: "app/api/query-cache/readiness/route.ts",
      },
      {
        route: "/api/instant/readiness",
        packageId: "instantdb/react",
        readinessKind: "provider-gated",
        runtimeExecution: false,
        liveProviderExecution: false,
        sourceFile: "app/api/instant/readiness/route.ts",
      },
      {
        route: "/api/database-orm/readiness",
        packageId: "db/drizzle-sqlite",
        readinessKind: "runtime-gated",
        runtimeExecution: false,
        liveProviderExecution: false,
        sourceFile: "app/api/database-orm/readiness/route.ts",
      },
      {
        route: "/api/supabase/readiness",
        packageId: "supabase/client",
        readinessKind: "provider-gated",
        runtimeExecution: false,
        liveProviderExecution: false,
        sourceFile: "app/api/supabase/readiness/route.ts",
      },
    ],
  );
  const dashboardRoute = manifest.routes.find((entry) => entry.route === "/dashboard");
  assert.ok(dashboardRoute, "dashboard route should exist");
  for (const packageId of dashboardForgePackageIds) {
    assert.ok(dashboardRoute.forgePackages.includes(packageId), `${packageId} should be in dashboard route packages`);
  }
  assert.equal(dashboardRoute.forgePackages.includes("content/react-markdown"), true);
  for (const route of ["/", "/login", "/logout", "/dashboard"]) {
    assert.ok(manifest.routes.some((entry) => entry.route === route), `${route} route should exist`);
  }
  assert.ok(
    manifest.editContract.surfaces.some(
      (surface) =>
        surface.id === "template-dashboard-page" &&
        dashboardForgePackageIds.every((packageId) => surface.packageIds.includes(packageId)),
    ),
    "preview manifest should expose the integrated dashboard app surface",
  );
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test("template routes preserve native page scrolling without a custom scrollbar runtime", () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-template-native-scroll-"));
  try {
    execFileSync(process.execPath, [materializer, dir], { cwd: root, stdio: "pipe" });

    const globals = read("examples/template/styles/globals.css");
    const runtime = read("tools/launch/runtime-template/assets/launch-runtime.ts");
    const routes = new Map([
      ["/", fs.readFileSync(path.join(dir, "pages", "index.html"), "utf8")],
      ["/dashboard", fs.readFileSync(path.join(dir, "pages", "dashboard.html"), "utf8")],
      ["/login", fs.readFileSync(path.join(dir, "pages", "login.html"), "utf8")],
    ]);

  assert.match(globals, /html\s*\{[\s\S]*overflow-y:\s*auto/);
  assert.match(globals, /body\s*\{[\s\S]*overflow-y:\s*auto/);
  assert.match(globals, /\.template-app-body\s*\{[\s\S]*overflow-y:\s*auto/);
  assert.match(globals, /\.dx-launch,\s*\.template-app,\s*\.dashboard-shell\s*\{[\s\S]*min-height:\s*100vh/);
  assert.match(globals, /\.dashboard-mobile-menu\s*\{[\s\S]*overflow:\s*visible/);
  assert.match(globals, /\.template-mobile-menu\s*\{[\s\S]*overflow:\s*visible/);
  assert.doesNotMatch(globals, /::-webkit-scrollbar|scrollbar-color|scrollbar-width/);
  assert.doesNotMatch(runtime, /bindTemplateCustomScrollbar|data-dashboard-scrollbar|scrollReveal/);

  for (const [route, html] of routes) {
    assert.match(
      html,
      /data-dx-native-scroll="page"/,
      `${route} must mark native page scrolling as the scroll owner`,
    );
    assert.match(
      html,
      /data-dx-scroll-lock="none"/,
      `${route} must not opt into body scroll locking`,
    );
    assert.doesNotMatch(
      html,
      /data-dx-component="template-custom-scrollbar"|data-dashboard-scrollbar|dashboard-scrollbar-button/,
      `${route} must not reintroduce the old custom scrollbar surface`,
    );
  }

  for (const [route, html] of [["/dashboard", routes.get("/dashboard") ?? ""]]) {
    assert.match(
      html,
      /data-dx-mobile-menu="native-details"/,
      `${route} mobile menu must use the native details disclosure`,
    );
    assert.match(
      html,
      /data-dx-mobile-menu-scroll-lock="none"/,
      `${route} mobile menu must not lock document scrolling`,
    );
  }
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test("backend route does not claim non-catalog Forge packages", () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-backend-packages-"));
  try {
    execFileSync(process.execPath, [materializer, dir], { cwd: root, stdio: "pipe" });

    const sourceBackend = read("tools/launch/runtime-template/pages/backend.html");
    const materializedBackend = fs.readFileSync(path.join(dir, "pages", "backend.html"), "utf8");
    const manifest = JSON.parse(
      fs.readFileSync(path.join(dir, "public", "preview-manifest.json"), "utf8"),
    );
    const backendRoute = manifest.routes.find(
      (route: { route: string }) => route.route === "/backend",
    );

    assert.ok(backendRoute, "preview manifest should include the backend route");
    assert.deepEqual(backendRoute.forgePackages, ["api/trpc", "ai/vercel-ai"]);

    for (const html of [sourceBackend, materializedBackend]) {
      assert.doesNotMatch(html, /backend\/convex-compatible/);
      assert.doesNotMatch(html, /convex-compatible-proof/);
      assert.match(html, /data-dx-package="api\/trpc,ai\/vercel-ai"/);
      assert.match(html, /data-dx-package-boundary="app-owned"/);
    }
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});
