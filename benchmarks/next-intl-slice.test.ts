const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("next-intl slice exposes formatter and server translation helpers", () => {
  const source = read("core/src/ecosystem/forge_next_intl.rs");

  assert.match(source, /"js\/i18n\/formatting\.tsx",\s*NEXT_INTL_FORMATTING_TSX/);
  assert.match(source, /const NEXT_INTL_FORMATTING_TSX: &str = r#"/);
  assert.match(source, /useFormatter/);
  assert.match(source, /useLocale/);
  assert.match(source, /getFormatter/);
  assert.match(source, /getTranslations/);
  assert.match(source, /setRequestLocale/);
  assert.match(source, /DxLaunchIntlSummary/);
  assert.match(source, /getDxLaunchIntlMetadata/);
  assert.match(source, /"i18n\/formatting\.tsx"/);
  assert.match(source, /`i18n\/formatting\.tsx` adds launch-ready client and server formatting helpers/);
});

test("launch template uses next-intl formatter APIs beyond translated copy", () => {
  const status = read("examples/template/next-intl-status.tsx");

  assert.match(status, /useFormatter/);
  assert.match(status, /useLocale/);
  assert.match(status, /format\.dateTime/);
  assert.match(status, /format\.relativeTime/);
  assert.match(status, /data-launch-i18n-locale=/);
  assert.match(status, /data-launch-i18n-window=/);
});

test("next-intl slice exposes the real Next config plugin boundary", () => {
  const source = read("core/src/ecosystem/forge_next_intl.rs");

  assert.match(source, /"js\/i18n\/next-config\.ts",\s*NEXT_INTL_NEXT_CONFIG_TS/);
  assert.match(source, /const NEXT_INTL_NEXT_CONFIG_TS: &str = r#"/);
  assert.match(source, /createNextIntlPlugin/);
  assert.match(source, /next-intl\/plugin/);
  assert.match(source, /requestConfig: "\.\/i18n\/request\.ts"/);
  assert.match(source, /createDxNextIntlPlugin/);
  assert.match(source, /withDxNextIntl/);
  assert.match(source, /DxNextIntlPluginConfig/);
  assert.match(source, /"i18n\/next-config\.ts"/);
  assert.match(source, /`i18n\/next-config\.ts` exposes the real next-intl\/plugin boundary/);
});

test("next-intl slice exposes extractor configuration without running extraction", () => {
  const source = read("core/src/ecosystem/forge_next_intl.rs");

  assert.match(source, /"js\/i18n\/extraction\.ts",\s*NEXT_INTL_EXTRACTION_TS/);
  assert.match(source, /const NEXT_INTL_EXTRACTION_TS: &str = r#"/);
  assert.match(source, /next-intl\/extractor/);
  assert.match(source, /ExtractorConfigInput/);
  assert.match(source, /Locale/);
  assert.match(source, /createDxMessageExtractionConfig/);
  assert.match(source, /dxLaunchMessageCatalogs/);
  assert.match(source, /format: "json"/);
  assert.match(source, /sourceLocale: "en"/);
  assert.match(source, /extract: true/);
  assert.match(source, /"i18n\/extraction\.ts"/);
  assert.match(source, /`i18n\/extraction\.ts` exposes typed extraction config/);
  assert.match(source, /does not run extraction/);
});

test("next-intl slice exposes localized navigation links and permanent redirects", () => {
  const source = read("core/src/ecosystem/forge_next_intl.rs");
  const status = read("examples/template/next-intl-status.tsx");

  assert.match(source, /permanentRedirect/);
  assert.match(source, /"js\/i18n\/locale-links\.tsx",\s*NEXT_INTL_LOCALE_LINKS_TSX/);
  assert.match(source, /const NEXT_INTL_LOCALE_LINKS_TSX: &str = r#"/);
  assert.match(source, /DxLocaleLinkList/);
  assert.match(source, /getDxLocaleHref/);
  assert.match(source, /Link/);
  assert.match(source, /usePathname/);
  assert.match(source, /"i18n\/locale-links\.tsx"/);
  assert.match(source, /`i18n\/locale-links\.tsx` adds locale switch links backed by createNavigation/);

  assert.match(status, /LaunchLocaleLinks/);
  assert.match(status, /Link/);
  assert.match(status, /data-launch-i18n-locale-links/);
  assert.match(status, /getPathname/);
});

test("next-intl slice exposes runtime context diagnostics", () => {
  const source = read("core/src/ecosystem/forge_next_intl.rs");
  const status = read("examples/template/next-intl-status.tsx");

  assert.match(source, /"js\/i18n\/context-status\.tsx",\s*NEXT_INTL_CONTEXT_STATUS_TSX/);
  assert.match(source, /const NEXT_INTL_CONTEXT_STATUS_TSX: &str = r#"/);
  assert.match(source, /useMessages/);
  assert.match(source, /useNow/);
  assert.match(source, /useTimeZone/);
  assert.match(source, /DxIntlContextStatus/);
  assert.match(source, /getDxMessageNamespaceCount/);
  assert.match(source, /data-launch-i18n-time-zone=/);
  assert.match(source, /"i18n\/context-status\.tsx"/);
  assert.match(source, /`i18n\/context-status\.tsx` exposes runtime context diagnostics/);

  assert.match(status, /LaunchIntlRuntimeSnapshot/);
  assert.match(status, /useMessages/);
  assert.match(status, /useNow/);
  assert.match(status, /useTimeZone/);
  assert.match(status, /data-launch-i18n-message-count=/);
});

test("next-intl slice exposes error and fallback policy hooks", () => {
  const source = read("core/src/ecosystem/forge_next_intl.rs");
  const status = read("examples/template/next-intl-status.tsx");

  assert.match(source, /"js\/i18n\/error-policy\.ts",\s*NEXT_INTL_ERROR_POLICY_TS/);
  assert.match(source, /const NEXT_INTL_ERROR_POLICY_TS: &str = r#"/);
  assert.match(source, /IntlErrorCode/);
  assert.match(source, /type IntlError/);
  assert.match(source, /createDxIntlErrorPolicy/);
  assert.match(source, /getDxMessageFallback/);
  assert.match(source, /onError/);
  assert.match(source, /getMessageFallback/);
  assert.match(source, /MISSING_MESSAGE/);
  assert.match(source, /FORMATTING_ERROR/);
  assert.match(source, /"i18n\/error-policy\.ts"/);
  assert.match(source, /`i18n\/error-policy\.ts` exposes missing-message and formatting fallback policy/);

  assert.match(status, /LaunchIntlErrorPolicy/);
  assert.match(status, /IntlErrorCode/);
  assert.match(status, /getLaunchMessageFallbackPreview/);
  assert.match(status, /data-launch-i18n-error-policy=/);
});

test("next-intl slice exposes server request context helpers", () => {
  const source = read("core/src/ecosystem/forge_next_intl.rs");
  const status = read("examples/template/next-intl-server-status.tsx");

  assert.match(source, /"js\/i18n\/server-context\.ts",\s*NEXT_INTL_SERVER_CONTEXT_TS/);
  assert.match(source, /const NEXT_INTL_SERVER_CONTEXT_TS: &str = r#"/);
  assert.match(source, /getLocale/);
  assert.match(source, /getMessages/);
  assert.match(source, /getNow/);
  assert.match(source, /getTimeZone/);
  assert.match(source, /getDxIntlServerContext/);
  assert.match(source, /DxIntlServerContext/);
  assert.match(source, /"i18n\/server-context\.ts"/);
  assert.match(source, /`i18n\/server-context\.ts` exposes server request context helpers/);

  assert.match(status, /LaunchIntlServerStatus/);
  assert.match(status, /getDxIntlServerContext/);
  assert.match(status, /data-launch-i18n-server-locale=/);
  assert.match(status, /data-launch-i18n-server-time-zone=/);
});

test("next-intl slice exposes static route and metadata helpers", () => {
  const source = read("core/src/ecosystem/forge_next_intl.rs");
  const route = read("examples/template/next-intl-route-boundary.tsx");

  assert.match(source, /"js\/i18n\/route-boundary\.ts",\s*NEXT_INTL_ROUTE_BOUNDARY_TS/);
  assert.match(source, /const NEXT_INTL_ROUTE_BOUNDARY_TS: &str = r#"/);
  assert.match(source, /generateDxLocaleStaticParams/);
  assert.match(source, /createDxLaunchMetadata/);
  assert.match(source, /setDxRequestLocale/);
  assert.match(source, /getTranslations/);
  assert.match(source, /setRequestLocale/);
  assert.match(source, /getPathname/);
  assert.match(source, /Metadata/);
  assert.match(source, /"i18n\/route-boundary\.ts"/);
  assert.match(source, /`i18n\/route-boundary\.ts` exposes static route and metadata helpers/);

  assert.match(route, /generateStaticParams/);
  assert.match(route, /generateMetadata/);
  assert.match(route, /createDxLaunchMetadata/);
  assert.match(route, /setDxRequestLocale/);
  assert.match(route, /data-launch-i18n-route-boundary=/);
});

test("next-intl slice exposes core translator and formatter helpers", () => {
  const source = read("core/src/ecosystem/forge_next_intl.rs");
  const status = read("examples/template/next-intl-runtime-core-status.tsx");

  assert.match(source, /"js\/i18n\/runtime-core\.ts",\s*NEXT_INTL_RUNTIME_CORE_TS/);
  assert.match(source, /const NEXT_INTL_RUNTIME_CORE_TS: &str = r#"/);
  assert.match(source, /createTranslator/);
  assert.match(source, /createFormatter/);
  assert.match(source, /createDxLaunchIntlRuntime/);
  assert.match(source, /DxLaunchCoreMessages/);
  assert.match(source, /format\.dateTime/);
  assert.match(source, /t\("phaseStatus"/);
  assert.match(source, /"i18n\/runtime-core\.ts"/);
  assert.match(source, /`i18n\/runtime-core\.ts` exposes core translator and formatter helpers/);

  assert.match(status, /LaunchIntlRuntimeCoreStatus/);
  assert.match(status, /createDxLaunchIntlRuntime/);
  assert.match(status, /data-launch-i18n-runtime-core=/);
  assert.match(status, /phaseStatus/);
});

test("next-intl slice exposes rich copy translation helpers", () => {
  const source = read("core/src/ecosystem/forge_next_intl.rs");
  const status = read("examples/template/next-intl-rich-copy-status.tsx");

  assert.match(source, /"js\/i18n\/rich-copy\.tsx",\s*NEXT_INTL_RICH_COPY_TSX/);
  assert.match(source, /const NEXT_INTL_RICH_COPY_TSX: &str = r#"/);
  assert.match(source, /DxLaunchRichCopy/);
  assert.match(source, /useTranslations/);
  assert.match(source, /t\.rich/);
  assert.match(source, /t\.markup/);
  assert.match(source, /t\.raw/);
  assert.match(source, /richDescription/);
  assert.match(source, /"i18n\/rich-copy\.tsx"/);
  assert.match(source, /`i18n\/rich-copy\.tsx` exposes rich, markup, and raw translation helpers/);

  assert.match(status, /LaunchIntlRichCopyStatus/);
  assert.match(status, /DxLaunchRichCopy/);
  assert.match(status, /data-launch-i18n-rich-copy=/);
});

test("next-intl slice exposes route locale guard helpers", () => {
  const source = read("core/src/ecosystem/forge_next_intl.rs");
  const guard = read("examples/template/next-intl-locale-guard-status.tsx");

  assert.match(source, /"js\/i18n\/locale-guard\.ts",\s*NEXT_INTL_LOCALE_GUARD_TS/);
  assert.match(source, /const NEXT_INTL_LOCALE_GUARD_TS: &str = r#"/);
  assert.match(source, /assertDxLocale/);
  assert.match(source, /getDxLocaleOrNotFound/);
  assert.match(source, /hasLocale/);
  assert.match(source, /notFound/);
  assert.match(source, /routing\.locales/);
  assert.match(source, /"i18n\/locale-guard\.ts"/);
  assert.match(source, /`i18n\/locale-guard\.ts` exposes route locale guard helpers/);

  assert.match(guard, /LaunchIntlLocaleGuardStatus/);
  assert.match(guard, /getDxLocaleOrNotFound/);
  assert.match(guard, /data-launch-i18n-locale-guard=/);
});

test("next-intl slice exposes routing policy helpers", () => {
  const source = read("core/src/ecosystem/forge_next_intl.rs");
  const status = read("examples/template/next-intl-routing-policy-status.tsx");

  assert.match(source, /"js\/i18n\/routing-policy\.ts",\s*NEXT_INTL_ROUTING_POLICY_TS/);
  assert.match(source, /const NEXT_INTL_ROUTING_POLICY_TS: &str = r#"/);
  assert.match(source, /createDxRoutingPolicy/);
  assert.match(source, /DxRoutingPolicyOptions/);
  assert.match(source, /localeCookie/);
  assert.match(source, /alternateLinks/);
  assert.match(source, /localeDetection/);
  assert.match(source, /domains/);
  assert.match(source, /defineDxRouting/);
  assert.match(source, /"i18n\/routing-policy\.ts"/);
  assert.match(source, /`i18n\/routing-policy\.ts` exposes routing policy helpers/);

  assert.match(status, /LaunchIntlRoutingPolicyStatus/);
  assert.match(status, /createDxRoutingPolicy/);
  assert.match(status, /data-launch-i18n-routing-policy=/);
});

test("next-intl slice exposes global format presets", () => {
  const source = read("core/src/ecosystem/forge_next_intl.rs");
  const status = read("examples/template/next-intl-formats-status.tsx");

  assert.match(source, /"js\/i18n\/formats\.ts",\s*NEXT_INTL_FORMATS_TS/);
  assert.match(source, /const NEXT_INTL_FORMATS_TS: &str = r#"/);
  assert.match(source, /type Formats/);
  assert.match(source, /dxIntlFormats/);
  assert.match(source, /createDxIntlFormats/);
  assert.match(source, /dateTime/);
  assert.match(source, /number/);
  assert.match(source, /formats\?: Formats/);
  assert.match(source, /formats={formats}/);
  assert.match(source, /"i18n\/formats\.ts"/);
  assert.match(source, /`i18n\/formats\.ts` exposes global date and number format presets/);

  assert.match(status, /LaunchIntlFormatsStatus/);
  assert.match(status, /dxIntlFormats/);
  assert.match(status, /data-launch-i18n-formats=/);
});

test("next-intl slice exposes request runtime configuration", () => {
  const source = read("core/src/ecosystem/forge_next_intl.rs");
  const status = read("examples/template/next-intl-request-runtime-status.tsx");

  assert.match(source, /"js\/i18n\/request-runtime\.ts",\s*NEXT_INTL_REQUEST_RUNTIME_TS/);
  assert.match(source, /const NEXT_INTL_REQUEST_RUNTIME_TS: &str = r#"/);
  assert.match(source, /DxIntlRequestRuntime/);
  assert.match(source, /createDxIntlRequestRuntime/);
  assert.match(source, /dxIntlRequestRuntime/);
  assert.match(source, /dxIntlErrorPolicy/);
  assert.match(source, /dxIntlFormats/);
  assert.match(source, /timeZone/);
  assert.match(source, /formats/);
  assert.match(source, /getMessageFallback/);
  assert.match(source, /onError/);
  assert.match(source, /createDxRequestConfig\(\s*loadMessages: \(locale: DxLocale\) => Promise<DxMessages> = loadDxMessages,\s*runtime: DxIntlRequestRuntime = createDxIntlRequestRuntime\(\),/);
  assert.match(source, /\.\.\.runtime/);
  assert.match(source, /"i18n\/request-runtime\.ts"/);
  assert.match(source, /`i18n\/request-runtime\.ts` exposes request runtime configuration/);

  assert.match(status, /LaunchIntlRequestRuntimeStatus/);
  assert.match(status, /dxIntlRequestRuntime/);
  assert.match(status, /data-launch-i18n-request-runtime=/);
});

test("next-intl slice exposes localized redirect action helpers", () => {
  const source = read("core/src/ecosystem/forge_next_intl.rs");
  const status = read("examples/template/next-intl-redirect-status.tsx");

  assert.match(source, /"js\/i18n\/navigation-actions\.ts",\s*NEXT_INTL_NAVIGATION_ACTIONS_TS/);
  assert.match(source, /const NEXT_INTL_NAVIGATION_ACTIONS_TS: &str = r#"/);
  assert.match(source, /getDxLaunchRedirectHref/);
  assert.match(source, /redirectToDxLaunch/);
  assert.match(source, /permanentRedirectToDxLaunch/);
  assert.match(source, /redirect\(/);
  assert.match(source, /permanentRedirect\(/);
  assert.match(source, /getPathname/);
  assert.match(source, /"i18n\/navigation-actions\.ts"/);
  assert.match(source, /`i18n\/navigation-actions\.ts` exposes localized redirect action helpers/);

  assert.match(status, /LaunchIntlRedirectStatus/);
  assert.match(status, /getDxLaunchRedirectHref/);
  assert.match(status, /data-launch-i18n-redirect=/);
});

test("next-intl slice exposes a server provider for App Router layouts", () => {
  const source = read("core/src/ecosystem/forge_next_intl.rs");
  const status = read("examples/template/next-intl-server-provider-status.tsx");

  assert.match(source, /"js\/i18n\/server-provider\.tsx",\s*NEXT_INTL_SERVER_PROVIDER_TSX/);
  assert.match(source, /const NEXT_INTL_SERVER_PROVIDER_TSX: &str = r#"/);
  assert.match(source, /DxIntlServerProvider/);
  assert.match(source, /NextIntlClientProvider/);
  assert.match(source, /getLocale/);
  assert.match(source, /getMessages/);
  assert.match(source, /getNow/);
  assert.match(source, /getTimeZone/);
  assert.match(source, /dxIntlFormats/);
  assert.match(source, /"i18n\/server-provider\.tsx"/);
  assert.match(source, /`i18n\/server-provider\.tsx` exposes an App Router server provider/);

  assert.match(status, /LaunchIntlServerProviderStatus/);
  assert.match(status, /DxIntlServerProvider/);
  assert.match(status, /data-launch-i18n-server-provider=/);
});

test("next-intl slice exposes reusable middleware helpers", () => {
  const source = read("core/src/ecosystem/forge_next_intl.rs");
  const status = read("examples/template/next-intl-middleware-status.tsx");

  assert.match(source, /createDxIntlMiddleware/);
  assert.match(source, /dxIntlMiddlewareConfig/);
  assert.match(source, /createMiddleware\(config\)/);
  assert.match(source, /typeof routing/);
  assert.match(source, /matcher: dxIntlMiddlewareMatcher/);
  assert.match(source, /"i18n\/middleware\.ts"/);
  assert.match(source, /`i18n\/middleware\.ts` exposes `createMiddleware\(routing\)`/);

  assert.match(status, /LaunchIntlMiddlewareStatus/);
  assert.match(status, /dxIntlMiddlewareConfig/);
  assert.match(status, /data-launch-i18n-middleware=/);
});

test("next-intl slice exposes client locale navigation actions", () => {
  const source = read("core/src/ecosystem/forge_next_intl.rs");
  const status = read("examples/template/next-intl-client-navigation-status.tsx");

  assert.match(source, /"js\/i18n\/navigation-client\.tsx",\s*NEXT_INTL_NAVIGATION_CLIENT_TSX/);
  assert.match(source, /const NEXT_INTL_NAVIGATION_CLIENT_TSX: &str = r#"/);
  assert.match(source, /"use client"/);
  assert.match(source, /useDxLocaleRouter/);
  assert.match(source, /DxLocaleSwitchButtons/);
  assert.match(source, /useRouter/);
  assert.match(source, /usePathname/);
  assert.match(source, /router\.replace/);
  assert.match(source, /router\.prefetch/);
  assert.match(source, /useTransition/);
  assert.match(source, /"i18n\/navigation-client\.tsx"/);
  assert.match(source, /`i18n\/navigation-client\.tsx` exposes client locale navigation actions/);

  assert.match(status, /LaunchIntlClientNavigationStatus/);
  assert.match(status, /DxLocaleSwitchButtons/);
  assert.match(status, /data-launch-i18n-client-navigation=/);
});

test("next-intl slice exposes inline extracted copy helpers", () => {
  const source = read("core/src/ecosystem/forge_next_intl.rs");
  const status = read("examples/template/next-intl-extracted-copy-status.tsx");

  assert.match(source, /"js\/i18n\/extracted-copy\.tsx",\s*NEXT_INTL_EXTRACTED_COPY_TSX/);
  assert.match(source, /const NEXT_INTL_EXTRACTED_COPY_TSX: &str = r#"/);
  assert.match(source, /useExtracted/);
  assert.match(source, /DxLaunchExtractedCopy/);
  assert.match(source, /getDxLaunchExtractedCopyFallback/);
  assert.match(source, /launch\.extractedStatus/);
  assert.match(source, /launch\.extractedRichStatus/);
  assert.match(source, /"i18n\/extracted-copy\.tsx"/);
  assert.match(source, /`i18n\/extracted-copy\.tsx` exposes inline extracted copy helpers/);
  assert.match(source, /useExtracted/);

  assert.match(status, /LaunchIntlExtractedCopyStatus/);
  assert.match(status, /DxLaunchExtractedCopy/);
  assert.match(status, /data-launch-i18n-extracted-copy=/);
});

test("next-intl slice exposes domain routing helpers", () => {
  const source = read("core/src/ecosystem/forge_next_intl.rs");
  const status = read("examples/template/next-intl-domain-routing-status.tsx");

  assert.match(source, /"js\/i18n\/domain-routing\.ts",\s*NEXT_INTL_DOMAIN_ROUTING_TS/);
  assert.match(source, /const NEXT_INTL_DOMAIN_ROUTING_TS: &str = r#"/);
  assert.match(source, /DxDomainRoutingConfig/);
  assert.match(source, /createDxDomainRouting/);
  assert.match(source, /getDxPrimaryDomainForLocale/);
  assert.match(source, /getDxDomainRoutingLocales/);
  assert.match(source, /defineDxRouting/);
  assert.match(source, /RoutingConfig/);
  assert.match(source, /domains/);
  assert.match(source, /"i18n\/domain-routing\.ts"/);
  assert.match(source, /`i18n\/domain-routing\.ts` exposes domain routing helpers/);

  assert.match(status, /LaunchIntlDomainRoutingStatus/);
  assert.match(status, /createDxDomainRouting/);
  assert.match(status, /data-launch-i18n-domain-routing=/);
});

test("next-intl slice exposes catalog config validation helpers", () => {
  const source = read("core/src/ecosystem/forge_next_intl.rs");
  const status = read("examples/template/next-intl-catalog-validation-status.tsx");

  assert.match(source, /"js\/i18n\/catalog-validation\.ts",\s*NEXT_INTL_CATALOG_VALIDATION_TS/);
  assert.match(source, /const NEXT_INTL_CATALOG_VALIDATION_TS: &str = r#"/);
  assert.match(source, /initializeConfig/);
  assert.match(source, /AbstractIntlMessages/);
  assert.match(source, /IntlConfig/);
  assert.match(source, /DxIntlCatalogValidationResult/);
  assert.match(source, /validateDxIntlCatalog/);
  assert.match(source, /validateDxLaunchCatalogs/);
  assert.match(source, /dxIntlErrorPolicy/);
  assert.match(source, /"i18n\/catalog-validation\.ts"/);
  assert.match(source, /`i18n\/catalog-validation\.ts` exposes catalog config validation helpers/);

  assert.match(status, /LaunchIntlCatalogValidationStatus/);
  assert.match(status, /validateDxLaunchCatalogs/);
  assert.match(status, /data-launch-i18n-catalog-validation=/);
});

test("next-intl slice exposes typed message contract helpers", () => {
  const source = read("core/src/ecosystem/forge_next_intl.rs");
  const status = read("examples/template/next-intl-type-contract-status.tsx");

  assert.match(source, /"js\/i18n\/type-contracts\.ts",\s*NEXT_INTL_TYPE_CONTRACTS_TS/);
  assert.match(source, /const NEXT_INTL_TYPE_CONTRACTS_TS: &str = r#"/);
  assert.match(source, /MessageKeys/);
  assert.match(source, /NamespaceKeys/);
  assert.match(source, /NestedKeyOf/);
  assert.match(source, /NestedValueOf/);
  assert.match(source, /TranslationValues/);
  assert.match(source, /RichTranslationValues/);
  assert.match(source, /MarkupTranslationValues/);
  assert.match(source, /DxLaunchMessageKey/);
  assert.match(source, /dxLaunchMessageKeys/);
  assert.match(source, /"i18n\/type-contracts\.ts"/);
  assert.match(source, /`i18n\/type-contracts\.ts` exposes typed message contract helpers/);

  assert.match(status, /LaunchIntlTypeContractStatus/);
  assert.match(status, /dxLaunchMessageKeys/);
  assert.match(status, /data-launch-i18n-type-contract=/);
});

test("next-intl slice exposes typed request config helpers", () => {
  const source = read("core/src/ecosystem/forge_next_intl.rs");
  const status = read("examples/template/next-intl-request-config-status.tsx");

  assert.match(source, /"js\/i18n\/request-config\.ts",\s*NEXT_INTL_REQUEST_CONFIG_TS/);
  assert.match(source, /const NEXT_INTL_REQUEST_CONFIG_TS: &str = r#"/);
  assert.match(source, /GetRequestConfigParams/);
  assert.match(source, /RequestConfig/);
  assert.match(source, /resolveDxRequestConfigLocale/);
  assert.match(source, /createDxRequestConfigResult/);
  assert.match(source, /requestLocale/);
  assert.match(source, /createDxIntlRequestRuntime/);
  assert.match(source, /loadDxMessages/);
  assert.match(source, /"i18n\/request-config\.ts"/);
  assert.match(source, /`i18n\/request-config\.ts` exposes typed request config helpers/);

  assert.match(status, /LaunchIntlRequestConfigStatus/);
  assert.match(status, /createDxRequestConfigResult/);
  assert.match(status, /data-launch-i18n-request-config=/);
});

test("next-intl slice exposes formatter cache helpers", () => {
  const source = read("core/src/ecosystem/forge_next_intl.rs");
  const status = read("examples/template/next-intl-formatter-cache-status.tsx");

  assert.match(source, /"js\/i18n\/formatter-cache\.ts",\s*NEXT_INTL_FORMATTER_CACHE_TS/);
  assert.match(source, /const NEXT_INTL_FORMATTER_CACHE_TS: &str = r#"/);
  assert.match(source, /_createCache/);
  assert.match(source, /_createIntlFormatters/);
  assert.match(source, /createFormatter/);
  assert.match(source, /createDxIntlFormatterCache/);
  assert.match(source, /createDxLaunchFormatter/);
  assert.match(source, /getDxFormatterCacheSummary/);
  assert.match(source, /dxIntlFormats/);
  assert.match(source, /"i18n\/formatter-cache\.ts"/);
  assert.match(source, /`i18n\/formatter-cache\.ts` exposes formatter cache helpers/);

  assert.match(status, /LaunchIntlFormatterCacheStatus/);
  assert.match(status, /createDxLaunchFormatter/);
  assert.match(status, /data-launch-i18n-formatter-cache=/);
});

test("next-intl slice exposes server extracted copy helpers", () => {
  const source = read("core/src/ecosystem/forge_next_intl.rs");
  const status = read("examples/template/next-intl-server-extracted-status.tsx");

  assert.match(source, /"js\/i18n\/server-extracted\.ts",\s*NEXT_INTL_SERVER_EXTRACTED_TS/);
  assert.match(source, /const NEXT_INTL_SERVER_EXTRACTED_TS: &str = r#"/);
  assert.match(source, /getExtracted/);
  assert.match(source, /getDxServerExtractedTranslator/);
  assert.match(source, /getDxLaunchServerExtractedCopy/);
  assert.match(source, /DxServerExtractedOptions/);
  assert.match(source, /namespace/);
  assert.match(source, /locale/);
  assert.match(source, /"i18n\/server-extracted\.ts"/);
  assert.match(source, /`i18n\/server-extracted\.ts` exposes server extracted copy helpers/);

  assert.match(status, /LaunchIntlServerExtractedStatus/);
  assert.match(status, /getDxLaunchServerExtractedCopy/);
  assert.match(status, /data-launch-i18n-server-extracted=/);
});

test("next-intl slice exposes extractor runner helpers", () => {
  const source = read("core/src/ecosystem/forge_next_intl.rs");
  const status = read("examples/template/next-intl-extractor-runner-status.tsx");

  assert.match(source, /"js\/i18n\/extraction-runner\.ts",\s*NEXT_INTL_EXTRACTION_RUNNER_TS/);
  assert.match(source, /const NEXT_INTL_EXTRACTION_RUNNER_TS: &str = r#"/);
  assert.match(source, /unstable_extractMessages/);
  assert.match(source, /defineCodec/);
  assert.match(source, /runDxMessageExtraction/);
  assert.match(source, /createDxMessageExtractionPlan/);
  assert.match(source, /dxJsonExtractorCodec/);
  assert.match(source, /createDxMessageExtractionConfig/);
  assert.match(source, /"i18n\/extraction-runner\.ts"/);
  assert.match(source, /`i18n\/extraction-runner\.ts` exposes opt-in extractor runner helpers/);

  assert.match(status, /LaunchIntlExtractorRunnerStatus/);
  assert.match(status, /createDxMessageExtractionPlan/);
  assert.match(status, /data-launch-i18n-extractor-runner=/);
});

test("next-intl slice exposes ICU message argument helpers", () => {
  const source = read("core/src/ecosystem/forge_next_intl.rs");
  const status = read("examples/template/next-intl-message-arguments-status.tsx");

  assert.match(source, /"js\/i18n\/message-arguments\.ts",\s*NEXT_INTL_MESSAGE_ARGUMENTS_TS/);
  assert.match(source, /const NEXT_INTL_MESSAGE_ARGUMENTS_TS: &str = r#"/);
  assert.match(source, /ICUArgs/);
  assert.match(source, /ICUTags/);
  assert.match(source, /RichTagsFunction/);
  assert.match(source, /MarkupTagsFunction/);
  assert.match(source, /DxLaunchPhaseArgs/);
  assert.match(source, /DxLaunchRichDescriptionTags/);
  assert.match(source, /createDxLaunchPhaseValues/);
  assert.match(source, /dxLaunchIcuMessages/);
  assert.match(source, /"i18n\/message-arguments\.ts"/);
  assert.match(source, /`i18n\/message-arguments\.ts` exposes ICU message argument helpers/);

  assert.match(status, /LaunchIntlMessageArgumentsStatus/);
  assert.match(status, /createDxLaunchPhaseValues/);
  assert.match(status, /data-launch-i18n-message-arguments=/);
});

test("next-intl slice exposes typed format option helpers", () => {
  const source = read("core/src/ecosystem/forge_next_intl.rs");
  const status = read("examples/template/next-intl-format-options-status.tsx");

  assert.match(source, /"js\/i18n\/format-options\.ts",\s*NEXT_INTL_FORMAT_OPTIONS_TS/);
  assert.match(source, /const NEXT_INTL_FORMAT_OPTIONS_TS: &str = r#"/);
  assert.match(source, /DateTimeFormatOptions/);
  assert.match(source, /NumberFormatOptions/);
  assert.match(source, /RelativeTimeFormatOptions/);
  assert.match(source, /Timezone/);
  assert.match(source, /DxLaunchFormatOptions/);
  assert.match(source, /createDxLaunchFormatOptions/);
  assert.match(source, /dxLaunchFormatOptions/);
  assert.match(source, /"i18n\/format-options\.ts"/);
  assert.match(source, /`i18n\/format-options\.ts` exposes typed format option helpers/);

  assert.match(status, /LaunchIntlFormatOptionsStatus/);
  assert.match(status, /createDxLaunchFormatOptions/);
  assert.match(status, /data-launch-i18n-format-options=/);
});

test("next-intl slice exposes app config type helpers", () => {
  const source = read("core/src/ecosystem/forge_next_intl.rs");
  const status = read("examples/template/next-intl-app-config-status.tsx");

  assert.match(source, /"js\/i18n\/app-config\.ts",\s*NEXT_INTL_APP_CONFIG_TS/);
  assert.match(source, /const NEXT_INTL_APP_CONFIG_TS: &str = r#"/);
  assert.match(source, /AppConfig/);
  assert.match(source, /AbstractIntlMessages/);
  assert.match(source, /type Locale/);
  assert.match(source, /Messages/);
  assert.match(source, /DxNextIntlAppConfig/);
  assert.match(source, /DxLaunchAppMessages/);
  assert.match(source, /getDxNextIntlAppConfigSummary/);
  assert.match(source, /dxIntlFormats/);
  assert.match(source, /"i18n\/app-config\.ts"/);
  assert.match(source, /`i18n\/app-config\.ts` exposes AppConfig, Locale, and Messages helpers/);

  assert.match(status, /LaunchIntlAppConfigStatus/);
  assert.match(status, /getDxNextIntlAppConfigSummary/);
  assert.match(status, /data-launch-i18n-app-config=/);
});

test("next-intl slice exposes route type helpers", () => {
  const source = read("core/src/ecosystem/forge_next_intl.rs");
  const status = read("examples/template/next-intl-route-types-status.tsx");

  assert.match(source, /"js\/i18n\/route-types\.ts",\s*NEXT_INTL_ROUTE_TYPES_TS/);
  assert.match(source, /const NEXT_INTL_ROUTE_TYPES_TS: &str = r#"/);
  assert.match(source, /Pathnames/);
  assert.match(source, /LocalePrefix/);
  assert.match(source, /DomainsConfig/);
  assert.match(source, /LocalePrefixMode/);
  assert.match(source, /QueryParams/);
  assert.match(source, /DxLaunchPathnames/);
  assert.match(source, /DxLaunchQueryParams/);
  assert.match(source, /createDxLaunchRouteTypeSummary/);
  assert.match(source, /"i18n\/route-types\.ts"/);
  assert.match(source, /`i18n\/route-types\.ts` exposes route type helpers/);

  assert.match(status, /LaunchIntlRouteTypesStatus/);
  assert.match(status, /createDxLaunchRouteTypeSummary/);
  assert.match(status, /data-launch-i18n-route-types=/);
});
