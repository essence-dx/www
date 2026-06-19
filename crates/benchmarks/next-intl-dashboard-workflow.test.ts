const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const sourceRoot = "G:\\WWW\\inspirations\\next-intl";

function read(relativePath) {
  const fullPath = path.join(root, relativePath);
  assert.ok(fs.existsSync(fullPath), `expected ${relativePath} to exist`);
  return fs.readFileSync(fullPath, "utf8");
}

function readMirror(relativePath) {
  const fullPath = path.join(sourceRoot, relativePath);
  assert.ok(
    fs.existsSync(fullPath),
    `expected next-intl source mirror file ${relativePath} to exist`,
  );
  return fs.readFileSync(fullPath, "utf8");
}

const mojibakeSequences = [
  String.fromCodePoint(0x00e0, 0x00a6),
  String.fromCodePoint(0x00e0, 0x00a7),
  String.fromCodePoint(0x00e0, 0x00a5),
  String.fromCodePoint(0x00c3),
  String.fromCodePoint(0x00c2),
];

const staleBanglaEnglishCopy = ["Review launch copy", " in ", "Bangla"].join("");

function assertNoMojibake(label, text) {
  for (const sequence of mojibakeSequences) {
    assert.ok(
      !text.includes(sequence),
      `${label} contains mojibake instead of UTF-8 source copy`,
    );
  }
}

test("next-intl dashboard workflow is grounded in upstream public APIs", () => {
  const provider = readMirror(
    "packages/next-intl/src/shared/NextIntlClientProvider.tsx",
  );
  const hooks = readMirror("packages/use-intl/src/react/index.tsx");
  const routing = readMirror(
    "packages/next-intl/src/routing/defineRouting.tsx",
  );
  const formatter = readMirror("packages/use-intl/src/core/createFormatter.tsx");

  assert.match(provider, /NextIntlClientProvider/);
  assert.match(hooks, /useTranslations/);
  assert.match(hooks, /useLocale/);
  assert.match(hooks, /useFormatter/);
  assert.match(routing, /defineRouting/);
  assert.match(formatter, /function dateTime/);
  assert.match(formatter, /DateTimeFormatOptions/);
  assert.match(formatter, /function number/);
  assert.match(formatter, /NumberFormatOptions/);
});

test("next-intl package slice exports dashboard locale workflow files and metadata", () => {
  const source = read("core/src/ecosystem/forge_next_intl.rs");

  assertNoMojibake("core next-intl Forge slice", source);
  assert.match(source, /"js\/i18n\/dashboard-copy\.ts",\s*NEXT_INTL_DASHBOARD_COPY_TS/);
  assert.match(
    source,
    /"js\/i18n\/dashboard-locale-workflow\.tsx",\s*NEXT_INTL_DASHBOARD_LOCALE_WORKFLOW_TSX/,
  );
  assert.match(source, /const NEXT_INTL_DASHBOARD_COPY_TS: &str = r#"/);
  assert.match(source, /const NEXT_INTL_DASHBOARD_LOCALE_WORKFLOW_TSX: &str = r#"/);
  assert.match(source, /packageId: "i18n\/next-intl"/);
  assert.match(source, /officialPackageName: "Internationalization"/);
  assert.match(source, /aliases: \["next-intl", "intl", "i18n\/next"\]/);
  assert.match(source, /upstreamPackage: "next-intl"/);
  assert.match(source, /upstreamVersion: "4\.12\.0"/);
  assert.match(source, /sourceMirror: "G:\/WWW\/inspirations\/next-intl"/);
  assert.match(source, /provenance:/);
  assert.match(source, /exportedFiles:/);
  assert.match(source, /requiredEnv: \[\]/);
  assert.match(source, /appOwnedBoundaries:/);
  assert.match(source, /receiptPaths:/);
  assert.match(source, /dashboardUsage:/);
  assert.match(source, /dxIcon: "pack:i18n"/);
  assert.match(source, /NextIntlClientProvider/);
  assert.match(source, /useTranslations/);
  assert.match(source, /useLocale/);
  assert.match(source, /useFormatter/);
  assert.match(source, /defineRouting/);
  assert.match(source, /createNavigation/);
  assert.match(source, /getRequestConfig/);
  assert.match(source, /createMiddleware/);
  assert.match(source, /createDxDashboardIntlReceipt/);
  assert.match(source, /DxDashboardIntlFormatPreview/);
  assert.match(source, /createDxDashboardIntlFormatPreview/);
  assert.match(source, /dxDashboardIntlFormatOptions/);
  assert.match(source, /DxDashboardIntlNumberPreview/);
  assert.match(source, /createDxDashboardIntlNumberPreview/);
  assert.match(source, /dxDashboardIntlNumberOptions/);
  assert.match(source, /DxDashboardLocaleAlternateLink/);
  assert.match(source, /DxDashboardLocaleRoutePreview/);
  assert.match(source, /dxDashboardLocaleAlternateLinks/);
  assert.match(source, /dxDashboardLocaleRoutePreviews/);
  assert.match(source, /getDxDashboardLocaleAlternateLinks/);
  assert.match(source, /getDxDashboardLocaleRoutePreview/);
  assert.match(source, /data-dx-intl-action="review-alternate-link"/);
  assert.match(source, /data-dx-intl-action="preview-localized-route"/);
  assert.match(source, /data-dx-intl-format-preview/);
  assert.match(source, /data-dx-intl-format-source-api/);
  assert.match(source, /data-dx-intl-number-preview/);
  assert.match(source, /data-dx-intl-number-source-api/);
  assert.match(source, /dxDashboardIntlMessages/);
  assert.match(source, /DxNextIntlDashboardLocaleWorkflow/);
  assert.match(source, /LaunchDashboard/);
});

test("launch shell imports next-intl as a dashboard workflow, not only a package card", () => {
  const component = read("examples/template/next-intl-dashboard-locale.tsx");
  const contract = read("examples/template/next-intl-dashboard-locale-contract.ts");
  const shell = read("examples/template/template-shell.tsx");
  const editContract = read("examples/template/dx-studio-edit-contract.ts");
  const routeContract = read("examples/template/template-route-contract.ts");
  const cli = read("dx-www/src/cli/mod.rs");
  const newCommand = read("dx-www/src/cli/new_command.rs");
  const studio = read("dx-www/src/cli/studio_manifest.rs");

  assertNoMojibake("next-intl dashboard component", component);
  assertNoMojibake("next-intl dashboard contract", contract);
  assert.match(component, /from "\.\/next-intl-dashboard-locale-contract";/);
  assert.doesNotMatch(component, /const dashboardCopy = \{/);
  assert.match(contract, /export const dashboardCopy = \{/);
  assert.match(
    contract,
    /export const localeOptions = Object\.keys\(\s*dashboardCopy,\s*\) as LaunchDashboardLocale\[\]/,
  );
  assert.match(contract, /export function normalizeLocale/);
  assert.match(contract, /export type DashboardIntlFormatPreview/);
  assert.match(contract, /export const dashboardIntlPreviewWindowIso/);
  assert.match(contract, /export const dashboardIntlFormatOptions/);
  assert.match(contract, /export function createDashboardIntlFormatPreview/);
  assert.match(contract, /export type DashboardIntlNumberPreview/);
  assert.match(contract, /export const dashboardIntlPlanMonthlyUsd/);
  assert.match(contract, /export const dashboardIntlNumberOptions/);
  assert.match(contract, /export function createDashboardIntlNumberPreview/);
  assert.match(contract, /export type DashboardLocaleAlternateLink/);
  assert.match(contract, /export const dashboardLocaleAlternateLinks/);
  assert.match(contract, /export function getDashboardLocaleAlternateLinks/);
  assert.match(contract, /export type DashboardLocaleRoutePreview/);
  assert.match(contract, /export const dashboardLocaleRoutePreviews/);
  assert.match(contract, /export function getDashboardLocaleRoutePreview/);
  assert.match(contract, /export function createDxDashboardIntlReceipt/);
  assert.match(contract, /formatPreview: DashboardIntlFormatPreview/);
  assert.match(contract, /numberPreview: DashboardIntlNumberPreview/);
  assert.match(contract, /alternateLinks: readonly DashboardLocaleAlternateLink\[\]/);
  assert.match(contract, /sourceApi: "useFormatter.dateTime"/);
  assert.match(contract, /sourceApi: "useFormatter.number"/);
  assert.match(contract, /currency: "USD"/);
  assert.match(contract, /timeZone: "UTC"/);
  assert.match(contract, /routePreviewDetails: DashboardLocaleRoutePreview/);
  assert.match(contract, /rel: "alternate"/);
  assert.match(contract, /hrefLang: "bn-BD"/);
  assert.match(contract, /localePrefix: "as-needed"/);
  assert.match(contract, /label: "বাংলা"/);
  assert.match(contract, /title: "লঞ্চ কপি বাংলায় পর্যালোচনা করুন"/);
  assert.match(contract, /readiness: "বাংলা কপি পণ্য পর্যালোচনার জন্য প্রস্তুত।"/);
  assert.ok(!contract.includes(staleBanglaEnglishCopy));
  assert.match(component, /data-dx-package="i18n\/next-intl"/);
  assert.match(
    component,
    /data-dx-component="next-intl-dashboard-locale-workflow"/,
  );
  assert.match(
    component,
    /data-dx-intl-dashboard-workflow="locale-copy-boundary"/,
  );
  assert.match(component, /data-dx-dashboard-copy-locale=/);
  assert.match(component, /data-dx-intl-message-namespace="LaunchDashboard"/);
  assert.match(component, /data-dx-intl-hreflang={routePreview\.hrefLang}/);
  assert.match(component, /data-dx-intl-locale-prefix={routePreview\.localePrefix}/);
  assert.match(component, /data-dx-intl-alternate-links="launch-dashboard"/);
  assert.match(component, /data-dx-intl-format-preview={formatPreview\.formatted}/);
  assert.match(component, /data-dx-intl-format-source-api={formatPreview\.sourceApi}/);
  assert.match(component, /data-dx-intl-format-time-zone={formatPreview\.timeZone}/);
  assert.match(component, /data-dx-intl-number-preview={numberPreview\.formatted}/);
  assert.match(component, /data-dx-intl-number-source-api={numberPreview\.sourceApi}/);
  assert.match(component, /data-dx-intl-number-currency={numberPreview\.currency}/);
  assert.match(component, /data-dx-intl-readiness-copy={translatedCopy\.readiness}/);
  assert.match(component, /dashboardCopy\[locale\]\.label/);
  assert.ok(!component.includes(staleBanglaEnglishCopy));
  assert.match(component, /data-dx-icon-search="i18n:locale"/);
  assert.match(component, /<dx-icon name="pack:i18n"/);
  assert.match(component, /useTranslations/);
  assert.match(component, /useLocale/);
  assert.match(component, /useFormatter/);
  assert.match(component, /createDashboardIntlFormatPreview/);
  assert.match(component, /createDashboardIntlNumberPreview/);
  assert.match(component, /dashboardIntlFormatOptions/);
  assert.match(component, /dashboardIntlNumberOptions/);
  assert.match(component, /dashboardIntlPlanMonthlyUsd/);
  assert.match(component, /dashboardIntlPreviewWindowIso/);
  assert.match(component, /format\.number\(\s*dashboardIntlPlanMonthlyUsd,\s*dashboardIntlNumberOptions,\s*\)/);
  assert.match(component, /createDxDashboardIntlReceipt/);
  assert.match(component, /getDashboardLocaleAlternateLinks/);
  assert.match(component, /getDashboardLocaleRoutePreview/);
  assert.match(component, /launchT\("phaseStatus", { phase: "release-candidate" }\)/);
  assert.match(component, /data-dx-intl-copy-target="format-preview"/);
  assert.match(component, /{formatPreview\.formatted}/);
  assert.match(component, /data-dx-intl-copy-target="price-preview"/);
  assert.match(component, /{numberPreview\.formatted}/);
  assert.match(component, /data-dx-intl-copy-target="readiness"/);
  assert.match(component, /{translatedCopy\.readiness}/);
  assert.match(component, /data-dx-intl-action="preview-localized-route"/);
  assert.match(component, /data-dx-intl-action="review-alternate-link"/);
  assert.match(component, /data-dx-intl-alternate-locale={link\.locale}/);
  assert.match(component, /data-dx-intl-alternate-href={link\.href}/);
  assert.match(component, /href={routePreview\.href}/);
  assert.match(component, /hrefLang={routePreview\.hrefLang}/);
  assert.doesNotMatch(component, /const t = launchT/);
  assert.doesNotMatch(component, /[\u00c2\u00b7]/);
  assert.doesNotMatch(component, /text-(emerald|red|blue|purple|orange|amber)-/);
  assert.doesNotMatch(component, /bg-(emerald|red|blue|purple|orange|amber)-/);

  assert.match(
    shell,
    /import { LaunchDashboardIntlWorkflow } from "\.\/next-intl-dashboard-locale";/,
  );
  assert.match(shell, /<LaunchDashboardIntlWorkflow \/>/);
  assert.match(
    routeContract,
    /"components\/template-app\/next-intl-dashboard-locale\.tsx"/,
  );
  assert.match(
    routeContract,
    /"components\/template-app\/next-intl-dashboard-locale-contract\.ts"/,
  );
  assert.match(editContract, /id: "next-intl-dashboard-locale-workflow"/);
  assert.match(
    editContract,
    /selector: '\[data-dx-component="next-intl-dashboard-locale-workflow"\]'/,
  );
  assert.match(
    editContract,
    /sourceFile: "examples\/template\/next-intl-dashboard-locale\.tsx"/,
  );
  assert.match(
    editContract,
    /materializedFile: "components\/template-app\/next-intl-dashboard-locale\.tsx"/,
  );
  assert.match(editContract, /packageIds: \["i18n\/next-intl"\]/);
  assert.match(editContract, /id: "next-intl-dashboard-message-contract"/);
  assert.match(
    editContract,
    /selector: '\[data-dx-intl-message-namespace="LaunchDashboard"\]'/,
  );
  assert.match(
    editContract,
    /sourceFile:\s*"examples\/template\/next-intl-dashboard-locale-contract\.ts"/,
  );
  assert.match(
    editContract,
    /materializedFile:\s*"components\/template-app\/next-intl-dashboard-locale-contract\.ts"/,
  );
  assert.match(editContract, /stateMarkers: \[\s*"data-dx-intl-message-namespace"/);
  assert.match(
    studio,
    /"examples\/template\/next-intl-dashboard-locale\.tsx"/,
  );
  assert.match(
    studio,
    /"components\/template-app\/next-intl-dashboard-locale\.tsx"/,
  );
  assert.match(
    studio,
    /studio_edit_surface\(\s*"next-intl-dashboard-locale-workflow"/,
  );
  assert.match(
    studio,
    /studio_marker\(\s*"data-dx-intl-dashboard-workflow"/,
  );
  assert.match(studio, /studio_marker\(\s*"data-dx-intl-action"/);
  assert.match(
    studio,
    /studio_marker\(\s*"data-dx-dashboard-copy-locale"/,
  );
  assert.match(
    studio,
    /studio_marker\(\s*"data-dx-intl-receipt-state"/,
  );
  assert.match(
    studio,
    /studio_marker\(\s*"data-dx-intl-message-namespace"/,
  );
  assert.match(studio, /studio_marker\(\s*"data-dx-intl-copy-target"/);
  assert.match(studio, /studio_marker\(\s*"data-dx-intl-readiness-copy"/);
  assert.match(studio, /studio_marker\(\s*"data-dx-intl-locale-option"/);
  assert.match(studio, /studio_marker\(\s*"data-dx-intl-preview-locale"/);
  assert.match(studio, /studio_marker\(\s*"data-dx-intl-provider-locale"/);
  assert.match(studio, /studio_marker\(\s*"data-dx-intl-route-preview"/);
  assert.match(studio, /studio_marker\(\s*"data-dx-intl-format-preview"/);
  assert.match(studio, /studio_marker\(\s*"data-dx-intl-format-source-api"/);
  assert.match(studio, /studio_marker\(\s*"data-dx-intl-format-time-zone"/);
  assert.match(studio, /studio_marker\(\s*"data-dx-intl-number-preview"/);
  assert.match(studio, /studio_marker\(\s*"data-dx-intl-number-source-api"/);
  assert.match(studio, /studio_marker\(\s*"data-dx-intl-number-currency"/);
  assert.match(studio, /studio_marker\(\s*"data-dx-intl-alternate-links"/);
  assert.match(studio, /studio_marker\(\s*"data-dx-intl-alternate-locale"/);
  assert.match(studio, /studio_marker\(\s*"data-dx-intl-alternate-href"/);
  assert.match(studio, /studio_marker\(\s*"data-dx-intl-hreflang"/);
  assert.match(studio, /studio_marker\(\s*"data-dx-intl-locale-prefix"/);
  assert.match(studio, /studio_marker\(\s*"data-dx-intl-plan-label"/);
  assert.match(studio, /studio_marker\(\s*"data-dx-intl-support-sla"/);
  assert.match(studio, /studio_marker\(\s*"data-dx-intl-receipt-locale"/);
  assert.match(studio, /studio_marker\(\s*"data-dx-intl-receipt-route"/);
  assert.match(studio, /studio_marker\(\s*"data-dx-intl-receipt-hreflang"/);
  assert.match(
    studio,
    /"interaction_selectors": route_string_array\(surface, "interaction_selectors"\)/,
  );
  assert.match(studio, /\[data-dx-intl-action\]/);
  assert.match(studio, /\[data-dx-intl-copy-target\]/);
  assert.match(studio, /\[data-dx-intl-readiness-copy\]/);
  assert.match(studio, /\[data-dx-intl-locale-option\]/);
  assert.match(
    studio,
    /"package": "i18n\/next-intl"[\s\S]*"front_facing_name": "Internationalization Dashboard Locale Workflow"[\s\S]*"source_file": "examples\/template\/next-intl-dashboard-locale\.tsx"/,
  );
  assert.doesNotMatch(
    studio,
    /"front_facing_name": "next-intl Dashboard Locale Workflow"/,
  );
  assert.match(studio, /"data-dx-intl-action"/);
  assert.match(cli, /NEXT_FAMILIAR_INTL_DASHBOARD_LOCALE_TSX/);
  assert.match(cli, /NEXT_FAMILIAR_INTL_DASHBOARD_LOCALE_CONTRACT_TS/);
  assert.match(
    cli,
    /"components\/template-app\/next-intl-dashboard-locale\.tsx"/,
  );
  assert.match(
    cli,
    /"components\/template-app\/next-intl-dashboard-locale-contract\.ts"/,
  );
  assert.match(cli, /NEXT_FAMILIAR_INTL_DASHBOARD_RECEIPT_JSON/);
  assert.match(
    cli,
    /include_str!\(\s*"..\/..\/..\/examples\/template\/\.dx\/forge\/receipts\/2026-05-22-i18n-next-intl-dashboard-locale\.json"\s*\)/,
  );
  assert.match(
    newCommand,
    /"\.dx\/forge\/receipts\/2026-05-22-i18n-next-intl-dashboard-locale\.json"/,
  );
  assert.match(
    cli,
    /"package_id": "i18n\/next-intl", "official_name": "Internationalization", "upstream_package": "next-intl", "upstream_version": "4\.12\.0", "aliases": \["next-intl", "intl", "i18n\/next"\]/,
  );
  assert.match(cli, /"source_mirror": "G:\/WWW\/inspirations\/next-intl"/);
  assert.match(
    cli,
    /"exported_files": \["i18n\/dashboard-copy\.ts", "i18n\/dashboard-locale-workflow\.tsx", "components\/template-app\/next-intl-dashboard-locale-contract\.ts", "components\/template-app\/next-intl-dashboard-locale\.tsx"/,
  );
  assert.match(
    cli,
    /"dashboard_usage": "The root route imports LaunchDashboardIntlWorkflow/,
  );
  assert.match(
    cli,
    /"receipt_paths": \["\.dx\/forge\/receipts\/\*-i18n-next-intl\.json", "\.dx\/forge\/docs\/i18n-next-intl\.md", "docs\/packages\/next-intl\.md", "examples\/template\/\.dx\/forge\/receipts\/2026-05-22-i18n-next-intl-dashboard-locale\.json"\]/,
  );
  assert.match(
    cli,
    /"i18n\/next-intl" => vec!\[\s*"DxIntlProvider",\s*"loadDxMessages",\s*"LaunchDashboardIntlWorkflow",\s*"createDxDashboardIntlReceipt",\s*"createDashboardIntlFormatPreview",\s*"createDxDashboardIntlNumberPreview",\s*"createDxDashboardIntlFormatPreview",\s*"createDxDashboardIntlNumberPreview",\s*"getDxDashboardLocaleAlternateLinks",\s*"getDashboardLocaleAlternateLinks",\s*"getDxDashboardLocaleRoutePreview",\s*"getDashboardLocaleRoutePreview",?\s*\]/,
  );
  assert.match(
    cli,
    /launch_companion_doc_receipt\(\s*"i18n\/next-intl",\s*"i18n",\s*"next-intl-dashboard-locale",\s*"dx add next-intl --write",\s*"examples\/template\/next-intl-dashboard-locale\.tsx",\s*"components\/template-app\/next-intl-dashboard-locale\.tsx",\s*"\.dx\/forge\/docs\/i18n-next-intl\.md",\s*"LaunchDashboardIntlWorkflow",\s*&\[\s*"useTranslations",\s*"useLocale",\s*"useFormatter",\s*"createDxDashboardIntlReceipt",?\s*\]/,
  );
});

test("runtime launch route visibly exposes next-intl dashboard interaction", () => {
  const page = read("tools/launch/runtime-template/pages/index.html");
  const runtime = read("tools/launch/runtime-template/assets/launch-runtime.ts");

  assertNoMojibake("runtime launch page next-intl bridge", page);
  assertNoMojibake("runtime launch JavaScript next-intl bridge", runtime);
  assert.match(page, /data-dx-package="i18n\/next-intl"/);
  assert.match(
    page,
    /data-dx-component="next-intl-dashboard-locale-workflow"/,
  );
  assert.match(page, /data-dx-dashboard-card="locale"/);
  assert.match(
    page,
    /data-dx-intl-dashboard-workflow="locale-copy-boundary"/,
  );
  assert.match(page, /data-dx-intl-message-namespace="LaunchDashboard"/);
  assert.match(page, /data-dx-intl-locale-option="en"/);
  assert.match(page, /data-dx-intl-locale-option="bn"/);
  assert.match(page, /data-dx-intl-action="switch-dashboard-locale"/);
  assert.match(page, /data-dx-intl-action="prepare-dashboard-intl-receipt"/);
  assert.match(page, /data-dx-intl-action="preview-localized-route"/);
  assert.match(page, /data-dx-intl-action="review-alternate-link"/);
  assert.match(page, /data-dx-intl-alternate-links="launch-dashboard"/);
  assert.match(page, /data-dx-intl-alternate-locale="bn"/);
  assert.match(page, /data-dx-intl-alternate-href="\/\?locale=bn"/);
  assert.match(page, /data-dx-intl-format-preview/);
  assert.match(page, /data-dx-intl-format-source-api="useFormatter\.dateTime"/);
  assert.match(page, /data-dx-intl-format-time-zone="UTC"/);
  assert.match(page, /id="mission-locale-format" data-dx-intl-copy-target="format-preview"/);
  assert.match(page, /data-dx-intl-number-preview="\$49"/);
  assert.match(page, /data-dx-intl-number-source-api="useFormatter\.number"/);
  assert.match(page, /data-dx-intl-number-currency="USD"/);
  assert.match(page, /id="mission-locale-price" data-dx-intl-copy-target="price-preview"/);
  assert.match(page, /data-dx-intl-hreflang="en"/);
  assert.match(page, /data-dx-intl-locale-prefix="as-needed"/);
  assert.match(page, /data-dx-intl-readiness-copy="English copy is ready for product review\."/);
  assert.match(page, /id="mission-locale-readiness" data-dx-intl-copy-target="readiness"/);
  assert.match(page, /<dx-icon name="pack:i18n"/);

  assert.match(runtime, /const dashboardIntlMessages = {/);
  assert.match(runtime, /status: "বাংলা"/);
  assert.match(runtime, /copy: "ড্যাশবোর্ড কপি বাংলায় বদলেছে; কোনো credential লোড করা হয়নি।"/);
  assert.match(runtime, /locale: "en"/);
  assert.match(runtime, /hrefLang: "bn-BD"/);
  assert.match(runtime, /localePrefix: "as-needed"/);
  assert.match(runtime, /formatPreview:/);
  assert.match(runtime, /formatSourceApi: "useFormatter\.dateTime"/);
  assert.match(runtime, /formatTimeZone: "UTC"/);
  assert.match(runtime, /numberPreview: "\$49"/);
  assert.match(runtime, /numberSourceApi: "useFormatter\.number"/);
  assert.match(runtime, /numberCurrency: "USD"/);
  assert.match(runtime, /alternateLinks: \[/);
  assert.match(runtime, /rel: "alternate"/);
  assert.match(runtime, /function bindIntlDashboard\(\)/);
  assert.match(runtime, /dashboard\.dataset\.dxDashboardLocale = state\.locale/);
  assert.match(runtime, /document\.documentElement\.lang = state\.locale/);
  assert.match(runtime, /dataset\.dxIntlReadinessCopy = intl\.readiness/);
  assert.match(runtime, /setText\("#mission-locale-readiness", intl\.readiness\)/);
  assert.match(runtime, /setText\("#mission-locale-format", `Formatter preview: \$\{intl\.formatPreview\}\.`\)/);
  assert.match(runtime, /setText\("#mission-locale-price", `Localized plan price: \$\{intl\.numberPreview\}\/mo\.`\)/);
  assert.match(runtime, /mission-locale-route-link/);
  assert.match(runtime, /routeLink\.href = intl\.routePreview/);
  assert.match(runtime, /routeLink\.hreflang = intl\.hrefLang/);
  assert.match(runtime, /dataset\.dxIntlLocaleSelected/);
  assert.match(runtime, /Dashboard copy switched/);
  assert.match(runtime, /createDxDashboardIntlReceipt/);
});

test("next-intl catalog, package docs, and status docs report dashboard usage", () => {
  const catalog = read("examples/template/package-catalog.ts");
  const docs = read("docs/packages/next-intl.md");
  const routeContract = read("examples/template/template-route-contract.ts");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");
  const dx = read("DX.md");
  const receiptText = read(
    "examples/template/.dx/forge/receipts/2026-05-22-i18n-next-intl-dashboard-locale.json",
  );
  const receipt = JSON.parse(
    receiptText,
  );

  for (const [label, text] of [
    ["next-intl package catalog", catalog],
    ["next-intl package docs", docs],
    ["next-intl route contract", routeContract],
    ["next-intl launch receipt", receiptText],
    ["TODO next-intl status", todo],
    ["CHANGELOG next-intl status", changelog],
    ["DX next-intl status", dx],
  ]) {
    assertNoMojibake(label, text);
  }

  assert.match(catalog, /packageId: "i18n\/next-intl"/);
  assert.match(catalog, /officialName: "Internationalization"/);
  assert.match(catalog, /upstreamPackage: "next-intl"/);
  assert.match(catalog, /upstreamVersion: "4\.12\.0"/);
  assert.match(catalog, /aliases: \["next-intl", "intl", "i18n\/next"\]/);
  assert.match(catalog, /sourceMirror: "G:\/WWW\/inspirations\/next-intl"/);
  assert.match(catalog, /provenance:/);
  assert.match(catalog, /exportedFiles:/);
  assert.match(catalog, /requiredEnv: \[\]/);
  assert.match(catalog, /receiptPaths:/);
  assert.match(catalog, /contractFile:/);
  assert.match(
    catalog,
    /materializedContractFile:\s*"components\/template-app\/next-intl-dashboard-locale-contract\.ts"/,
  );
  assert.match(
    catalog,
    /examples\/template\/\.dx\/forge\/receipts\/2026-05-22-i18n-next-intl-dashboard-locale\.json/,
  );
  assert.match(catalog, /dashboardUsage:/);
  assert.match(catalog, /dxIcon: "pack:i18n"/);
  assert.match(catalog, /'data-dx-intl-action="switch-dashboard-locale"'/);
  assert.match(catalog, /'data-dx-intl-action="prepare-dashboard-intl-receipt"'/);
  assert.match(catalog, /'data-dx-intl-action="preview-localized-route"'/);
  assert.match(catalog, /'data-dx-intl-action="review-alternate-link"'/);
  assert.match(catalog, /'data-dx-intl-format-preview'/);
  assert.match(catalog, /'data-dx-intl-format-source-api'/);
  assert.match(catalog, /'data-dx-intl-format-time-zone'/);
  assert.match(catalog, /'data-dx-intl-number-preview'/);
  assert.match(catalog, /'data-dx-intl-number-source-api'/);
  assert.match(catalog, /'data-dx-intl-number-currency'/);
  assert.match(catalog, /'data-dx-intl-alternate-links'/);
  assert.match(catalog, /'data-dx-intl-alternate-locale'/);
  assert.match(catalog, /'data-dx-intl-alternate-href'/);
  assert.match(catalog, /'data-dx-intl-message-namespace'/);
  assert.match(catalog, /'data-dx-intl-hreflang'/);
  assert.match(catalog, /'data-dx-intl-locale-prefix'/);
  assert.match(catalog, /'data-dx-intl-locale-option'/);
  assert.match(catalog, /'data-dx-intl-copy-target'/);
  assert.match(catalog, /'data-dx-intl-readiness-copy'/);
  assert.match(catalog, /'data-dx-intl-receipt-state="idle"'/);
  assert.match(routeContract, /nextIntlDashboardLocale/);
  assert.match(routeContract, /dx\.forge\.package_dashboard_workflow_receipt/);
  assert.match(
    routeContract,
    /examples\/template\/\.dx\/forge\/receipts\/2026-05-22-i18n-next-intl-dashboard-locale\.json/,
  );
  assert.match(routeContract, /"data-dx-intl-locale-option"/);
  assert.match(routeContract, /"data-dx-intl-copy-target"/);
  assert.match(routeContract, /"data-dx-intl-readiness-copy"/);
  assert.match(routeContract, /"data-dx-intl-format-preview"/);
  assert.match(routeContract, /"data-dx-intl-format-source-api"/);
  assert.match(routeContract, /"data-dx-intl-format-time-zone"/);
  assert.match(routeContract, /"data-dx-intl-alternate-links"/);
  assert.match(routeContract, /"data-dx-intl-alternate-locale"/);
  assert.match(routeContract, /"data-dx-intl-alternate-href"/);
  assert.match(routeContract, /"data-dx-intl-hreflang"/);
  assert.match(routeContract, /"data-dx-intl-locale-prefix"/);
  assert.match(routeContract, /"data-dx-intl-receipt-locale"/);

  assert.equal(receipt.schema, "dx.forge.package_dashboard_workflow_receipt");
  assert.equal(receipt.package_id, "i18n/next-intl");
  assert.equal(receipt.package_name, "Internationalization");
  assert.equal(receipt.upstream_package, "next-intl");
  assert.equal(receipt.upstream_version, "4.12.0");
  assert.equal(receipt.route, "/");
  assert.equal(receipt.component, "next-intl-dashboard-locale-workflow");
  assert.equal(receipt.no_runtime_execution, true);
  assert.equal(receipt.reality_audit.verdict, "REAL");
  assert.equal(receipt.reality_audit.dashboard_consumer, "examples/template/next-intl-dashboard-locale.tsx");
  assert.equal(receipt.reality_audit.runtime_consumer, "tools/launch/runtime-template/pages/index.html");
  assert.equal(receipt.reality_audit.content_contract, "examples/template/next-intl-dashboard-locale-contract.ts");
  assert.equal(receipt.reality_audit.format_preview_contract, "DashboardIntlFormatPreview");
  assert.equal(receipt.reality_audit.number_preview_contract, "DashboardIntlNumberPreview");
  assert.equal(receipt.reality_audit.alternate_link_contract, "DashboardLocaleAlternateLink");
  assert.equal(receipt.reality_audit.route_preview_contract, "DashboardLocaleRoutePreview");
  assert.equal(receipt.reality_audit.guard, "benchmarks/next-intl-dashboard-workflow.test.ts");
  assert.ok(receipt.reality_audit.upstream_source.includes("packages/next-intl/package.json"));
  assert.ok(receipt.reality_audit.upstream_public_apis.includes("useTranslations"));
  assert.ok(receipt.reality_audit.forge_files.includes("core/src/ecosystem/forge_next_intl.rs"));
  assert.ok(receipt.reality_audit.dashboard_markers.includes('data-dx-package="i18n/next-intl"'));
  assert.ok(receipt.reality_audit.partial_boundaries.includes("governed browser proof"));
  assert.ok(receipt.reality_audit.partial_boundaries.includes("production runtime dependency installation"));
  assert.deepEqual(receipt.local_readiness_interactions, [
    "switch-dashboard-locale",
    "prepare-dashboard-intl-receipt",
    "preview-localized-route",
    "review-alternate-link",
  ]);
  assert.equal(receipt.local_demo_interactions, undefined);
  assert.equal(receipt.visible_locale_copy.bn, "বাংলা");
  assert.ok(
    receipt.stable_markers.includes('data-dx-package="i18n/next-intl"'),
  );
  assert.ok(
    receipt.stable_markers.includes(
      'data-dx-intl-action="switch-dashboard-locale"',
    ),
  );
  assert.ok(
    receipt.stable_markers.includes(
      'data-dx-intl-action="prepare-dashboard-intl-receipt"',
    ),
  );
  assert.ok(
    receipt.stable_markers.includes(
      'data-dx-intl-action="preview-localized-route"',
    ),
  );
  assert.ok(
    receipt.stable_markers.includes(
      'data-dx-intl-action="review-alternate-link"',
    ),
  );
  assert.ok(receipt.stable_markers.includes("data-dx-intl-format-preview"));
  assert.ok(receipt.stable_markers.includes("data-dx-intl-format-source-api"));
  assert.ok(receipt.stable_markers.includes("data-dx-intl-format-time-zone"));
  assert.equal(receipt.format_preview_contract.sourceApi, "useFormatter.dateTime");
  assert.equal(receipt.format_preview_contract.timeZone, "UTC");
  assert.ok(receipt.stable_markers.includes("data-dx-intl-number-preview"));
  assert.ok(receipt.stable_markers.includes("data-dx-intl-number-source-api"));
  assert.ok(receipt.stable_markers.includes("data-dx-intl-number-currency"));
  assert.equal(receipt.number_preview_contract.sourceApi, "useFormatter.number");
  assert.equal(receipt.number_preview_contract.currency, "USD");
  assert.ok(receipt.stable_markers.includes("data-dx-intl-alternate-links"));
  assert.ok(receipt.stable_markers.includes("data-dx-intl-alternate-locale"));
  assert.ok(receipt.stable_markers.includes("data-dx-intl-alternate-href"));
  assert.equal(receipt.alternate_link_contract.length, 2);
  assert.equal(receipt.alternate_link_contract[1].hrefLang, "bn-BD");
  assert.equal(receipt.alternate_link_contract[1].rel, "alternate");
  assert.ok(receipt.stable_markers.includes("data-dx-intl-hreflang"));
  assert.ok(receipt.stable_markers.includes("data-dx-intl-locale-prefix"));
  assert.ok(receipt.stable_markers.includes("data-dx-intl-receipt-hreflang"));
  assert.equal(receipt.route_preview_contract.en.hrefLang, "en");
  assert.equal(receipt.route_preview_contract.bn.hrefLang, "bn-BD");
  assert.equal(receipt.route_preview_contract.bn.href, "/?locale=bn");
  assert.ok(receipt.stable_markers.includes("data-dx-intl-locale-option"));
  assert.ok(receipt.stable_markers.includes("data-dx-intl-copy-target"));
  assert.ok(receipt.stable_markers.includes("data-dx-intl-readiness-copy"));
  assert.ok(receipt.stable_markers.includes("data-dx-intl-receipt-locale"));
  assert.ok(receipt.source_files.includes("examples/template/next-intl-dashboard-locale.tsx"));
  assert.ok(receipt.source_files.includes("examples/template/next-intl-dashboard-locale-contract.ts"));
  assert.ok(receipt.materialized_files.includes("components/template-app/next-intl-dashboard-locale-contract.ts"));
  assert.ok(receipt.guards.includes("node --test .\\benchmarks\\next-intl-dashboard-workflow.test.ts"));

  for (const text of [docs, todo, changelog, dx]) {
    assert.match(text, /Internationalization dashboard locale workflow/i);
    assert.match(text, /i18n\/next-intl/);
    assert.match(
      text,
      /2026-05-22-i18n-next-intl-dashboard-locale\.json/,
    );
  }
});
