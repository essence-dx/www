pub(super) const NEXT_INTL_VERSION: &str = "4.12.0-dx.0";

pub(super) fn next_intl_templates() -> Vec<(&'static str, &'static str)> {
    vec![
        ("js/i18n/routing.ts", NEXT_INTL_ROUTING_TS),
        ("js/i18n/route-types.ts", NEXT_INTL_ROUTE_TYPES_TS),
        ("js/i18n/routing-policy.ts", NEXT_INTL_ROUTING_POLICY_TS),
        ("js/i18n/domain-routing.ts", NEXT_INTL_DOMAIN_ROUTING_TS),
        ("js/i18n/navigation.ts", NEXT_INTL_NAVIGATION_TS),
        (
            "js/i18n/navigation-actions.ts",
            NEXT_INTL_NAVIGATION_ACTIONS_TS,
        ),
        (
            "js/i18n/navigation-client.tsx",
            NEXT_INTL_NAVIGATION_CLIENT_TSX,
        ),
        ("js/i18n/request.ts", NEXT_INTL_REQUEST_TS),
        ("js/i18n/request-config.ts", NEXT_INTL_REQUEST_CONFIG_TS),
        ("js/i18n/request-runtime.ts", NEXT_INTL_REQUEST_RUNTIME_TS),
        ("js/i18n/app-config.ts", NEXT_INTL_APP_CONFIG_TS),
        ("js/i18n/locale-guard.ts", NEXT_INTL_LOCALE_GUARD_TS),
        ("js/i18n/server-context.ts", NEXT_INTL_SERVER_CONTEXT_TS),
        ("js/i18n/route-boundary.ts", NEXT_INTL_ROUTE_BOUNDARY_TS),
        ("js/i18n/middleware.ts", NEXT_INTL_MIDDLEWARE_TS),
        ("js/i18n/formats.ts", NEXT_INTL_FORMATS_TS),
        ("js/i18n/format-options.ts", NEXT_INTL_FORMAT_OPTIONS_TS),
        ("js/i18n/formatter-cache.ts", NEXT_INTL_FORMATTER_CACHE_TS),
        ("js/i18n/server-provider.tsx", NEXT_INTL_SERVER_PROVIDER_TSX),
        ("js/i18n/provider.tsx", NEXT_INTL_PROVIDER_TSX),
        ("js/i18n/formatting.tsx", NEXT_INTL_FORMATTING_TSX),
        ("js/i18n/runtime-core.ts", NEXT_INTL_RUNTIME_CORE_TS),
        ("js/i18n/rich-copy.tsx", NEXT_INTL_RICH_COPY_TSX),
        ("js/i18n/extracted-copy.tsx", NEXT_INTL_EXTRACTED_COPY_TSX),
        ("js/i18n/server-extracted.ts", NEXT_INTL_SERVER_EXTRACTED_TS),
        ("js/i18n/type-contracts.ts", NEXT_INTL_TYPE_CONTRACTS_TS),
        (
            "js/i18n/message-arguments.ts",
            NEXT_INTL_MESSAGE_ARGUMENTS_TS,
        ),
        ("js/i18n/next-config.ts", NEXT_INTL_NEXT_CONFIG_TS),
        ("js/i18n/extraction.ts", NEXT_INTL_EXTRACTION_TS),
        (
            "js/i18n/extraction-runner.ts",
            NEXT_INTL_EXTRACTION_RUNNER_TS,
        ),
        ("js/i18n/locale-links.tsx", NEXT_INTL_LOCALE_LINKS_TSX),
        ("js/i18n/context-status.tsx", NEXT_INTL_CONTEXT_STATUS_TSX),
        ("js/i18n/error-policy.ts", NEXT_INTL_ERROR_POLICY_TS),
        (
            "js/i18n/catalog-validation.ts",
            NEXT_INTL_CATALOG_VALIDATION_TS,
        ),
        ("js/i18n/dashboard-copy.ts", NEXT_INTL_DASHBOARD_COPY_TS),
        (
            "js/i18n/dashboard-locale-workflow.tsx",
            NEXT_INTL_DASHBOARD_LOCALE_WORKFLOW_TSX,
        ),
        ("js/i18n/messages/en.json", NEXT_INTL_MESSAGES_EN_JSON),
        ("js/i18n/messages/bn.json", NEXT_INTL_MESSAGES_BN_JSON),
        ("js/i18n/metadata.ts", NEXT_INTL_METADATA_TS),
        ("js/i18n/README.md", NEXT_INTL_README_MD),
    ]
}

const NEXT_INTL_ROUTING_TS: &str = r#"import { defineRouting } from "next-intl/routing";

export const dxLocales = ["en", "bn"] as const;
export type DxLocale = (typeof dxLocales)[number];

export const dxDefaultLocale = "en" satisfies DxLocale;

export const dxPathnames = {
  "/": "/",
  "/launch": {
    en: "/launch",
    bn: "/launch",
  },
} as const;

export type DxRoutingInput = Partial<Parameters<typeof defineRouting>[0]>;

export function defineDxRouting(input: DxRoutingInput = {}) {
  return defineRouting({
    locales: dxLocales,
    defaultLocale: dxDefaultLocale,
    localePrefix: "as-needed",
    pathnames: dxPathnames,
    ...input,
  });
}

export const routing = defineDxRouting();
"#;

const NEXT_INTL_ROUTE_TYPES_TS: &str = r#"import type { QueryParams } from "next-intl/navigation";
import type {
  DomainsConfig,
  LocalePrefix,
  LocalePrefixMode,
  Pathnames,
} from "next-intl/routing";

import { dxLocales, dxPathnames, type DxLocale } from "./routing";

export const dxLaunchPathnames = dxPathnames satisfies Pathnames<typeof dxLocales>;
export type DxLaunchPathnames = typeof dxLaunchPathnames;
export type DxLaunchLocalePrefixMode = LocalePrefixMode;
export type DxLaunchLocalePrefix = LocalePrefix<typeof dxLocales, "as-needed">;
export type DxLaunchDomains = DomainsConfig<typeof dxLocales>;
export type DxLaunchQueryParams = QueryParams & {
  source?: "zed" | "www" | "forge";
};

export function createDxLaunchRouteTypeSummary({
  locale = "en",
  query = {},
}: {
  locale?: DxLocale;
  query?: DxLaunchQueryParams;
} = {}) {
  return {
    locale,
    pathCount: Object.keys(dxPathnames).length,
    queryKeys: Object.keys(query),
  };
}
"#;

const NEXT_INTL_ROUTING_POLICY_TS: &str = r#"import type { RoutingConfig } from "next-intl/routing";

import { defineDxRouting, dxLocales, type DxLocale } from "./routing";

export type DxRoutingPolicyOptions = Pick<
  RoutingConfig<typeof dxLocales>,
  "alternateLinks" | "domains" | "localeCookie" | "localeDetection"
>;

export const dxRoutingPolicyDefaults = {
  alternateLinks: true,
  localeCookie: {
    maxAge: 200 * 24 * 60 * 60,
    sameSite: "lax",
  },
  localeDetection: true,
} as const satisfies DxRoutingPolicyOptions;

export function createDxRoutingPolicy({
  alternateLinks = dxRoutingPolicyDefaults.alternateLinks,
  domains,
  localeCookie = dxRoutingPolicyDefaults.localeCookie,
  localeDetection = dxRoutingPolicyDefaults.localeDetection,
}: DxRoutingPolicyOptions = {}) {
  return defineDxRouting({
    alternateLinks,
    domains,
    localeCookie,
    localeDetection,
  });
}

export function getDxDomainLocales(
  domains: NonNullable<DxRoutingPolicyOptions["domains"]> = [],
): DxLocale[] {
  return domains.flatMap((domain) => domain.locales);
}
"#;

const NEXT_INTL_DOMAIN_ROUTING_TS: &str = r#"import type { RoutingConfig } from "next-intl/routing";

import { defineDxRouting, dxLocales, type DxLocale } from "./routing";

export type DxDomainRoutingConfig = NonNullable<
  RoutingConfig<typeof dxLocales>["domains"]
>;

export type DxDomainRoutingOptions = {
  domains: DxDomainRoutingConfig;
  localeDetection?: boolean;
};

export function createDxDomainRouting({
  domains,
  localeDetection = true,
}: DxDomainRoutingOptions) {
  return defineDxRouting({
    domains,
    localeDetection,
  });
}

export function getDxDomainRoutingLocales(
  domains: DxDomainRoutingConfig,
): DxLocale[] {
  return Array.from(new Set(domains.flatMap((domain) => domain.locales)));
}

export function getDxPrimaryDomainForLocale(
  domains: DxDomainRoutingConfig,
  locale: DxLocale,
) {
  return domains.find((domain) => domain.defaultLocale === locale) ??
    domains.find((domain) => domain.locales.includes(locale));
}
"#;

const NEXT_INTL_NAVIGATION_TS: &str = r#"import { createNavigation } from "next-intl/navigation";

import { routing } from "./routing";

export const {
  Link,
  getPathname,
  permanentRedirect,
  redirect,
  usePathname,
  useRouter,
} = createNavigation(routing);
"#;

const NEXT_INTL_NAVIGATION_ACTIONS_TS: &str = r#"import { getPathname, permanentRedirect, redirect } from "./navigation";
import type { DxLocale } from "./routing";

export type DxLaunchRedirectTarget = {
  forcePrefix?: boolean;
  locale: DxLocale;
};

export function getDxLaunchRedirectHref({
  forcePrefix,
  locale,
}: DxLaunchRedirectTarget) {
  return getPathname({
    forcePrefix,
    href: "/launch",
    locale,
  });
}

export function redirectToDxLaunch(target: DxLaunchRedirectTarget) {
  return redirect({
    forcePrefix: target.forcePrefix,
    href: "/launch",
    locale: target.locale,
  });
}

export function permanentRedirectToDxLaunch(target: DxLaunchRedirectTarget) {
  return permanentRedirect({
    forcePrefix: target.forcePrefix,
    href: "/launch",
    locale: target.locale,
  });
}
"#;

const NEXT_INTL_NAVIGATION_CLIENT_TSX: &str = r#""use client";

import { useLocale } from "next-intl";
import { useTransition } from "react";

import { usePathname, useRouter } from "./navigation";
import { dxLocales, type DxLocale } from "./routing";

export type DxLocaleRouter = {
  activeLocale: DxLocale;
  isPending: boolean;
  pathname: ReturnType<typeof usePathname>;
  prefetchLocale: (locale: DxLocale) => void;
  switchLocale: (locale: DxLocale) => void;
};

export function useDxLocaleRouter(): DxLocaleRouter {
  const activeLocale = useLocale() as DxLocale;
  const pathname = usePathname();
  const router = useRouter();
  const [isPending, startTransition] = useTransition();

  function switchLocale(locale: DxLocale) {
    startTransition(() => {
      router.replace(pathname, { locale });
    });
  }

  function prefetchLocale(locale: DxLocale) {
    router.prefetch(pathname, { locale });
  }

  return {
    activeLocale,
    isPending,
    pathname,
    prefetchLocale,
    switchLocale,
  };
}

export type DxLocaleSwitchButtonsProps = {
  labels?: Partial<Record<DxLocale, string>>;
};

export function DxLocaleSwitchButtons({
  labels = {},
}: DxLocaleSwitchButtonsProps) {
  const { activeLocale, isPending, pathname, prefetchLocale, switchLocale } =
    useDxLocaleRouter();

  return (
    <div data-launch-i18n-client-navigation={pathname}>
      {dxLocales.map((locale) => (
        <button
          aria-current={locale === activeLocale ? "page" : undefined}
          disabled={isPending || locale === activeLocale}
          key={locale}
          onClick={() => switchLocale(locale)}
          onFocus={() => prefetchLocale(locale)}
          onMouseEnter={() => prefetchLocale(locale)}
          type="button"
        >
          {labels[locale] ?? locale.toUpperCase()}
        </button>
      ))}
    </div>
  );
}
"#;

const NEXT_INTL_REQUEST_TS: &str = r#"import { hasLocale } from "next-intl";
import { getRequestConfig } from "next-intl/server";

import {
  createDxIntlRequestRuntime,
  type DxIntlRequestRuntime,
} from "./request-runtime";
import { routing, type DxLocale } from "./routing";

export type DxMessages = Record<string, string | DxMessages>;

export async function loadDxMessages(locale: DxLocale): Promise<DxMessages> {
  return (await import(`./messages/${locale}.json`)).default as DxMessages;
}

export function resolveDxLocale(requested: string | undefined): DxLocale {
  return hasLocale(routing.locales, requested) ? requested : routing.defaultLocale;
}

export function createDxRequestConfig(
  loadMessages: (locale: DxLocale) => Promise<DxMessages> = loadDxMessages,
  runtime: DxIntlRequestRuntime = createDxIntlRequestRuntime(),
) {
  return getRequestConfig(async ({ requestLocale }) => {
    const locale = resolveDxLocale(await requestLocale);

    return {
      locale,
      messages: await loadMessages(locale),
      ...runtime,
    };
  });
}

export default createDxRequestConfig();
"#;

const NEXT_INTL_REQUEST_CONFIG_TS: &str = r#"import type {
  GetRequestConfigParams,
  RequestConfig,
} from "next-intl/server";

import {
  createDxIntlRequestRuntime,
  type DxIntlRequestRuntime,
} from "./request-runtime";
import { loadDxMessages, type DxMessages } from "./request";
import { resolveDxLocale } from "./request";
import type { DxLocale } from "./routing";

export type DxRequestConfigParams = Pick<
  GetRequestConfigParams,
  "locale" | "requestLocale"
>;

export async function resolveDxRequestConfigLocale({
  locale,
  requestLocale,
}: DxRequestConfigParams): Promise<DxLocale> {
  return resolveDxLocale(locale ?? (await requestLocale));
}

export async function createDxRequestConfigResult(
  params: DxRequestConfigParams,
  loadMessages: (locale: DxLocale) => Promise<DxMessages> = loadDxMessages,
  runtime: DxIntlRequestRuntime = createDxIntlRequestRuntime(),
): Promise<RequestConfig> {
  const locale = await resolveDxRequestConfigLocale(params);

  return {
    locale,
    messages: await loadMessages(locale),
    ...runtime,
  };
}
"#;

const NEXT_INTL_REQUEST_RUNTIME_TS: &str = r#"import type { Formats, IntlError } from "next-intl";

import { dxIntlErrorPolicy } from "./error-policy";
import { dxIntlFormats } from "./formats";

export type DxIntlRequestRuntime = {
  formats?: Formats;
  getMessageFallback?: (info: {
    error: IntlError;
    key: string;
    namespace?: string;
  }) => string;
  onError?: (error: IntlError) => void;
  timeZone?: string;
};

export const dxIntlRequestRuntime = {
  formats: dxIntlFormats,
  getMessageFallback: dxIntlErrorPolicy.getMessageFallback,
  onError: dxIntlErrorPolicy.onError,
  timeZone: "UTC",
} satisfies DxIntlRequestRuntime;

export function createDxIntlRequestRuntime(
  input: DxIntlRequestRuntime = {},
): DxIntlRequestRuntime {
  return {
    ...dxIntlRequestRuntime,
    ...input,
  };
}
"#;

const NEXT_INTL_APP_CONFIG_TS: &str = r#"import type {
  AbstractIntlMessages,
  AppConfig,
  Locale,
  Messages,
} from "next-intl";

import { dxIntlFormats } from "./formats";
import { dxDefaultLocale, dxLocales, type DxLocale } from "./routing";
import type { DxLaunchMessages } from "./type-contracts";

export type DxLaunchAppMessages = DxLaunchMessages & AbstractIntlMessages;

export type DxNextIntlAppConfig = AppConfig & {
  Formats: typeof dxIntlFormats;
  Locale: DxLocale;
  Messages: DxLaunchAppMessages;
};

export type DxNextIntlLocale = Locale;
export type DxNextIntlMessages = Messages;

export function getDxNextIntlAppConfigSummary() {
  return {
    defaultLocale: dxDefaultLocale,
    localeCount: dxLocales.length,
    hasFormats: Object.keys(dxIntlFormats).length > 0,
  };
}
"#;

const NEXT_INTL_LOCALE_GUARD_TS: &str = r#"import { notFound } from "next/navigation";
import { hasLocale } from "next-intl";

import { routing, type DxLocale } from "./routing";

export function assertDxLocale(candidate: unknown): candidate is DxLocale {
  return hasLocale(routing.locales, candidate);
}

export function getDxLocaleOrNotFound(candidate: unknown): DxLocale {
  if (!assertDxLocale(candidate)) {
    notFound();
  }

  return candidate;
}
"#;

const NEXT_INTL_SERVER_CONTEXT_TS: &str = r#"import {
  getLocale,
  getMessages,
  getNow,
  getTimeZone,
} from "next-intl/server";

export type DxIntlServerContext = {
  locale: string;
  messageNamespaceCount: number;
  now: string;
  timeZone: string | undefined;
};

export async function getDxIntlServerContext(): Promise<DxIntlServerContext> {
  const [locale, messages, now, timeZone] = await Promise.all([
    getLocale(),
    getMessages(),
    getNow(),
    getTimeZone(),
  ]);

  return {
    locale,
    messageNamespaceCount: Object.keys(messages).length,
    now: now.toISOString(),
    timeZone,
  };
}
"#;

const NEXT_INTL_ROUTE_BOUNDARY_TS: &str = r#"import type { Metadata } from "next";
import type { Locale } from "next-intl";
import { getTranslations, setRequestLocale } from "next-intl/server";

import { getPathname } from "./navigation";
import { routing, type DxLocale } from "./routing";

export type DxLaunchRouteParams = {
  locale: DxLocale;
};

export function generateDxLocaleStaticParams(): DxLaunchRouteParams[] {
  return routing.locales.map((locale) => ({ locale }));
}

export function setDxRequestLocale(locale: DxLocale) {
  setRequestLocale(locale);
}

export async function createDxLaunchMetadata(
  locale: DxLocale,
): Promise<Metadata> {
  const t = await getTranslations({
    locale: locale as Locale,
    namespace: "Launch",
  });

  return {
    title: t("title"),
    alternates: {
      canonical: getPathname({
        href: "/launch",
        locale,
      }),
    },
  };
}
"#;

const NEXT_INTL_MIDDLEWARE_TS: &str = r#"import createMiddleware from "next-intl/middleware";

import { routing } from "./routing";

export const dxIntlMiddlewareMatcher = "/((?!api|trpc|_next|_vercel|.*\\..*).*)";

export function createDxIntlMiddleware(config: typeof routing = routing) {
  return createMiddleware(config);
}

export const dxIntlMiddleware = createDxIntlMiddleware();

export default dxIntlMiddleware;

export const dxIntlMiddlewareConfig = {
  matcher: dxIntlMiddlewareMatcher,
};

export const config = dxIntlMiddlewareConfig;
"#;

const NEXT_INTL_FORMATS_TS: &str = r#"import type { Formats } from "next-intl";

export type DxIntlFormatPreset = "launchCompact" | "launchCurrency";

export function createDxIntlFormats(
  currency = "USD",
): Formats {
  return {
    dateTime: {
      launchCompact: {
        dateStyle: "medium",
        timeStyle: "short",
      },
    },
    number: {
      launchCurrency: {
        currency,
        style: "currency",
      },
    },
  };
}

export const dxIntlFormats = createDxIntlFormats();
"#;

const NEXT_INTL_FORMAT_OPTIONS_TS: &str = r#"import type {
  DateTimeFormatOptions,
  NumberFormatOptions,
  RelativeTimeFormatOptions,
  Timezone,
} from "next-intl";

export type DxLaunchFormatOptions = {
  dateTime: DateTimeFormatOptions;
  number: NumberFormatOptions;
  relativeTime: RelativeTimeFormatOptions;
  timeZone: Timezone;
};

export function createDxLaunchFormatOptions({
  timeZone = "UTC",
}: {
  timeZone?: Timezone;
} = {}): DxLaunchFormatOptions {
  return {
    dateTime: {
      dateStyle: "medium",
      timeStyle: "short",
      timeZone,
    },
    number: {
      maximumFractionDigits: 0,
      notation: "compact",
    },
    relativeTime: {
      now: new Date("2026-05-23T09:00:00.000Z"),
      style: "short",
      unit: "day",
    },
    timeZone,
  };
}

export const dxLaunchFormatOptions = createDxLaunchFormatOptions();
"#;

const NEXT_INTL_FORMATTER_CACHE_TS: &str = r#"import {
  _createCache,
  _createIntlFormatters,
  createFormatter,
  type Formats,
  type Locale,
} from "next-intl";

import { dxIntlFormats } from "./formats";

export type DxIntlFormatterCache = ReturnType<typeof _createCache>;
export type DxIntlFormatterFactory = ReturnType<typeof _createIntlFormatters>;

export type DxLaunchFormatterOptions = {
  formats?: Formats;
  locale?: Locale;
  now?: Date;
  timeZone?: string;
};

export function createDxIntlFormatterCache() {
  const cache = _createCache();
  const formatters = _createIntlFormatters(cache);

  return {
    cache,
    formatters,
  };
}

export function createDxLaunchFormatter({
  formats = dxIntlFormats,
  locale = "en",
  now = new Date("2026-05-23T09:00:00.000Z"),
  timeZone = "UTC",
}: DxLaunchFormatterOptions = {}) {
  const { cache, formatters } = createDxIntlFormatterCache();

  return createFormatter({
    _cache: cache,
    _formatters: formatters,
    formats,
    locale,
    now,
    timeZone,
  });
}

export function getDxFormatterCacheSummary() {
  const { cache, formatters } = createDxIntlFormatterCache();

  return {
    hasCache: Boolean(cache),
    hasFormatters: Boolean(formatters),
  };
}
"#;

const NEXT_INTL_PROVIDER_TSX: &str = r#""use client";

import * as React from "react";
import {
  type Formats,
  type IntlError,
  NextIntlClientProvider,
  type AbstractIntlMessages,
  type Locale,
} from "next-intl";

export type DxIntlProviderProps = {
  children: React.ReactNode;
  formats?: Formats;
  locale: Locale;
  messages: AbstractIntlMessages;
  onError?: (error: IntlError) => void;
  getMessageFallback?: (info: {
    error: IntlError;
    key: string;
    namespace?: string;
  }) => string;
  now?: Date;
  timeZone?: string;
};

export function DxIntlProvider({
  children,
  formats,
  getMessageFallback,
  locale,
  messages,
  now,
  onError,
  timeZone = "UTC",
}: DxIntlProviderProps) {
  return (
    <NextIntlClientProvider
      formats={formats}
      getMessageFallback={getMessageFallback}
      locale={locale}
      messages={messages}
      now={now}
      onError={onError}
      timeZone={timeZone}
    >
      {children}
    </NextIntlClientProvider>
  );
}
"#;

const NEXT_INTL_SERVER_PROVIDER_TSX: &str = r#"import * as React from "react";
import {
  type AbstractIntlMessages,
  type Formats,
  type Locale,
  NextIntlClientProvider,
} from "next-intl";
import { getLocale, getMessages, getNow, getTimeZone } from "next-intl/server";

import { dxIntlFormats } from "./formats";

export type DxIntlServerProviderProps = {
  children: React.ReactNode;
  formats?: Formats;
  locale?: Locale;
  messages?: AbstractIntlMessages;
  now?: Date;
  timeZone?: string;
};

export async function DxIntlServerProvider({
  children,
  formats = dxIntlFormats,
  locale,
  messages,
  now,
  timeZone,
}: DxIntlServerProviderProps) {
  const [resolvedLocale, resolvedMessages, resolvedNow, resolvedTimeZone] =
    await Promise.all([
      locale ?? getLocale(),
      messages ?? getMessages(locale ? { locale } : undefined),
      now ?? getNow(),
      timeZone ?? getTimeZone(),
    ]);

  return (
    <NextIntlClientProvider
      formats={formats}
      locale={resolvedLocale}
      messages={resolvedMessages}
      now={resolvedNow}
      timeZone={resolvedTimeZone}
    >
      {children}
    </NextIntlClientProvider>
  );
}
"#;

const NEXT_INTL_FORMATTING_TSX: &str = r#"import { useFormatter, useLocale, useTranslations } from "next-intl";
import { getFormatter, getTranslations, setRequestLocale } from "next-intl/server";

import type { DxLocale } from "./routing";

export type DxLaunchIntlSummaryProps = {
  phase?: string;
  releaseAt?: Date;
  relativeDays?: number;
};

export type DxLaunchIntlMetadata = {
  locale: DxLocale;
  title: string;
  status: string;
  generatedAt: string;
};

export function DxLaunchIntlSummary({
  phase = "beta",
  releaseAt = new Date("2026-05-23T09:00:00.000Z"),
  relativeDays = 1,
}: DxLaunchIntlSummaryProps) {
  const t = useTranslations("Launch");
  const format = useFormatter();
  const locale = useLocale();

  return (
    <dl data-launch-i18n-locale={locale}>
      <div>
        <dt>{t("status")}</dt>
        <dd>{t("phaseStatus", { phase })}</dd>
      </div>
      <div>
        <dt>Release window</dt>
        <dd data-launch-i18n-window={releaseAt.toISOString()}>
          {format.dateTime(releaseAt, {
            dateStyle: "medium",
            timeStyle: "short",
          })}
        </dd>
      </div>
      <div>
        <dt>Relative readiness</dt>
        <dd>{format.relativeTime(relativeDays, "day")}</dd>
      </div>
    </dl>
  );
}

export async function getDxLaunchIntlMetadata(
  locale: DxLocale,
): Promise<DxLaunchIntlMetadata> {
  setRequestLocale(locale);

  const [t, format] = await Promise.all([
    getTranslations({ locale, namespace: "Launch" }),
    getFormatter({ locale }),
  ]);

  return {
    locale,
    title: t("title"),
    status: t("status"),
    generatedAt: format.dateTime(new Date("2026-05-23T09:00:00.000Z"), {
      dateStyle: "medium",
      timeStyle: "short",
    }),
  };
}
"#;

const NEXT_INTL_RUNTIME_CORE_TS: &str = r#"import {
  createFormatter,
  createTranslator,
  type Locale,
} from "next-intl";

export type DxLaunchCoreMessages = {
  Launch: {
    title: string;
    status: string;
    phaseStatus: string;
  };
};

export type DxLaunchIntlRuntimeInput = {
  locale: Locale;
  messages: DxLaunchCoreMessages;
  now?: Date;
  phase?: string;
  timeZone?: string;
};

export function createDxLaunchIntlRuntime({
  locale,
  messages,
  now = new Date("2026-05-23T09:00:00.000Z"),
  phase = "beta",
  timeZone = "UTC",
}: DxLaunchIntlRuntimeInput) {
  const t = createTranslator({
    locale,
    messages,
    namespace: "Launch",
  });
  const format = createFormatter({
    locale,
    now,
    timeZone,
  });

  return {
    generatedAt: format.dateTime(now, {
      dateStyle: "medium",
      timeStyle: "short",
    }),
    phaseStatus: t("phaseStatus", { phase }),
    status: t("status"),
    title: t("title"),
  };
}
"#;

const NEXT_INTL_RICH_COPY_TSX: &str = r#"import type { ReactNode } from "react";
import { useTranslations } from "next-intl";

export type DxLaunchRichCopyProps = {
  guideLink?: (chunks: ReactNode) => ReactNode;
};

export function DxLaunchRichCopy({
  guideLink = (chunks) => <a href="/launch">{chunks}</a>,
}: DxLaunchRichCopyProps) {
  const t = useTranslations("Launch");

  return (
    <section data-launch-i18n-rich-copy={t.raw("richDescription")}>
      <p>
        {t.rich("richDescription", {
          guide: guideLink,
        })}
      </p>
      <span
        data-launch-i18n-rich-markup={t.markup("richDescription", {
          guide: (chunks) => `<a href="/launch">${chunks}</a>`,
        })}
      />
    </section>
  );
}
"#;

const NEXT_INTL_EXTRACTED_COPY_TSX: &str = r#"import type { ReactNode } from "react";
import { useExtracted } from "next-intl";

export type DxLaunchExtractedCopyProps = {
  phase?: string;
};

export type DxLaunchExtractedCopyFallback = {
  id: string;
  message: string;
};

export function getDxLaunchExtractedCopyFallback(
  phase = "beta",
): DxLaunchExtractedCopyFallback {
  return {
    id: "launch.extractedStatus",
    message: `Launch phase ${phase} is ready for extracted copy review.`,
  };
}

export function DxLaunchExtractedCopy({
  phase = "beta",
}: DxLaunchExtractedCopyProps): ReactNode {
  const t = useExtracted("Launch");
  const fallback = getDxLaunchExtractedCopyFallback(phase);

  return (
    <span data-launch-i18n-extracted-copy={fallback.id}>
      {t({
        id: fallback.id,
        message: "Launch phase {phase} is ready for extracted copy review.",
        values: { phase },
      })}
      {" "}
      {t.rich({
        id: "launch.extractedRichStatus",
        message: "Review <guide>inline extraction ownership</guide>.",
        values: {
          guide: (chunks) => <strong>{chunks}</strong>,
        },
      })}
    </span>
  );
}
"#;

const NEXT_INTL_SERVER_EXTRACTED_TS: &str = r#"import type { Locale } from "next-intl";
import { getExtracted } from "next-intl/server";

export type DxServerExtractedOptions = {
  locale?: Locale;
  namespace?: string;
};

export async function getDxServerExtractedTranslator({
  locale,
  namespace,
}: DxServerExtractedOptions = {}) {
  if (locale) {
    return getExtracted({
      locale,
      namespace,
    });
  }

  return getExtracted(namespace);
}

export async function getDxLaunchServerExtractedCopy(
  options: DxServerExtractedOptions = {},
) {
  const t = await getDxServerExtractedTranslator({
    namespace: "Launch",
    ...options,
  });

  return {
    phase: t("Launch phase {phase} is ready for application-owned translation review", {
      phase: "beta",
    }),
    status: t("Internationalized launch route is ready"),
    title: t("DX launch"),
  };
}
"#;

const NEXT_INTL_TYPE_CONTRACTS_TS: &str = r#"import type {
  MarkupTranslationValues,
  MessageKeys,
  NamespaceKeys,
  NestedKeyOf,
  NestedValueOf,
  RichTranslationValues,
  TranslationValues,
} from "next-intl";

export type DxLaunchMessages = {
  Launch: {
    phaseStatus: string;
    richDescription: string;
    status: string;
    title: string;
  };
  LaunchDashboard: {
    en: DxDashboardLocaleMessages;
    bn: DxDashboardLocaleMessages;
  };
};

export type DxDashboardLocaleMessages = {
  copySummary: string;
  planLabel: string;
  readiness: string;
  routePreview: string;
  supportSla: string;
  title: string;
};

export type DxLaunchMessagePath = NestedKeyOf<DxLaunchMessages>;
export type DxLaunchNamespace = NamespaceKeys<
  DxLaunchMessages,
  DxLaunchMessagePath
>;
export type DxLaunchMessageKey = MessageKeys<
  DxLaunchMessages,
  DxLaunchMessagePath
>;

export type DxLaunchMessageValue<Path extends DxLaunchMessagePath> =
  NestedValueOf<DxLaunchMessages, Path>;

export type DxLaunchTranslationValues = TranslationValues & {
  phase?: string;
};
export type DxLaunchRichTranslationValues = RichTranslationValues;
export type DxLaunchMarkupTranslationValues = MarkupTranslationValues;

export const dxLaunchMessageNamespaces = ["Launch"] as const satisfies readonly DxLaunchNamespace[];

export const dxLaunchMessageKeys = [
  "Launch.phaseStatus",
  "Launch.richDescription",
  "Launch.status",
  "Launch.title",
] as const satisfies readonly DxLaunchMessageKey[];

export function isDxLaunchMessageKey(
  candidate: string,
): candidate is DxLaunchMessageKey {
  return dxLaunchMessageKeys.includes(candidate as DxLaunchMessageKey);
}
"#;

const NEXT_INTL_MESSAGE_ARGUMENTS_TS: &str = r#"import type {
  ICUArgs,
  ICUTags,
  MarkupTagsFunction,
  RichTagsFunction,
} from "next-intl";

export type DxIcuArgumentOptions = {
  ICUArgument: string;
  ICUDateArgument: Date;
  ICUNumberArgument: number | bigint;
};

export type DxLaunchPhaseMessage =
  "Launch phase {phase} is ready for application-owned translation review";
export type DxLaunchRichDescriptionMessage =
  "Review <guide>translation ownership</guide> before launch.";

export type DxLaunchPhaseArgs = ICUArgs<
  DxLaunchPhaseMessage,
  DxIcuArgumentOptions
>;
export type DxLaunchRichDescriptionTags = ICUTags<
  DxLaunchRichDescriptionMessage,
  RichTagsFunction
>;
export type DxLaunchMarkupDescriptionTags = ICUTags<
  DxLaunchRichDescriptionMessage,
  MarkupTagsFunction
>;

export const dxLaunchIcuMessages = {
  phaseStatus:
    "Launch phase {phase} is ready for application-owned translation review",
  richDescription:
    "Review <guide>translation ownership</guide> before launch.",
} as const;

export function createDxLaunchPhaseValues(
  values: DxLaunchPhaseArgs,
): DxLaunchPhaseArgs {
  return values;
}
"#;

const NEXT_INTL_NEXT_CONFIG_TS: &str = r#"import createNextIntlPlugin from "next-intl/plugin";
import type { NextConfig } from "next";

export type DxNextIntlPluginConfig = {
  requestConfig?: string;
  nextConfig?: NextConfig;
};

export const dxNextIntlRequestConfigPath = "./i18n/request.ts";

export function createDxNextIntlPlugin(
  config: Pick<DxNextIntlPluginConfig, "requestConfig"> = {
    requestConfig: "./i18n/request.ts",
  },
) {
  return createNextIntlPlugin(config);
}

export function withDxNextIntl({
  requestConfig = "./i18n/request.ts",
  nextConfig = {},
}: DxNextIntlPluginConfig = {}) {
  return createDxNextIntlPlugin({ requestConfig })(nextConfig);
}
"#;

const NEXT_INTL_EXTRACTION_TS: &str = r#"import type { ExtractorConfigInput, Locale } from "next-intl/extractor";

import { dxLocales, type DxLocale } from "./routing";

export type DxLaunchMessageCatalog = {
  locale: DxLocale;
  path: `./i18n/messages/${DxLocale}.json`;
  source: boolean;
};

export type DxMessageExtractionOptions = {
  srcPath?: string | readonly string[];
  messagesPath?: string | readonly string[];
  locales?: "infer" | readonly Locale[];
  sourceLocale?: DxLocale;
};

export const dxLaunchMessageCatalogs = [
  {
    locale: "en",
    path: "./i18n/messages/en.json",
    source: true,
  },
  {
    locale: "bn",
    path: "./i18n/messages/bn.json",
    source: false,
  },
] as const satisfies readonly DxLaunchMessageCatalog[];

export const dxLaunchMessageExtractionDefaults = {
  srcPath: ["./app", "./components"],
  messagesPath: "./i18n/messages",
  locales: dxLocales,
  sourceLocale: "en",
} as const satisfies Required<DxMessageExtractionOptions>;

export function createDxMessageExtractionConfig({
  srcPath = dxLaunchMessageExtractionDefaults.srcPath,
  messagesPath = dxLaunchMessageExtractionDefaults.messagesPath,
  locales = dxLaunchMessageExtractionDefaults.locales,
  sourceLocale = dxLaunchMessageExtractionDefaults.sourceLocale,
}: DxMessageExtractionOptions = {}): ExtractorConfigInput {
  return {
    srcPath,
    messages: {
      format: "json",
      path: messagesPath,
      locales,
      sourceLocale,
    },
    extract: true,
  };
}

export const dxLaunchMessageExtractionConfig =
  createDxMessageExtractionConfig();
"#;

const NEXT_INTL_EXTRACTION_RUNNER_TS: &str = r#"import {
  defineCodec,
  unstable_extractMessages,
  type ExtractorConfigInput,
} from "next-intl/extractor";

import {
  createDxMessageExtractionConfig,
  dxLaunchMessageExtractionConfig,
  type DxMessageExtractionOptions,
} from "./extraction";

export const dxJsonExtractorCodec = defineCodec(() => ({
  decode(content) {
    const parsed = JSON.parse(content) as Record<string, string>;

    return Object.entries(parsed).map(([id, message]) => ({
      id,
      message,
    }));
  },
  encode(messages) {
    const entries = messages.map((message) => [message.id, message.message]);

    return `${JSON.stringify(Object.fromEntries(entries), null, 2)}\n`;
  },
  toJSONString(content) {
    return content;
  },
}));

export type DxMessageExtractionPlan = {
  config: ExtractorConfigInput;
  runsExtraction: false;
};

export function createDxMessageExtractionPlan(
  options: DxMessageExtractionOptions = {},
): DxMessageExtractionPlan {
  return {
    config:
      Object.keys(options).length > 0
        ? createDxMessageExtractionConfig(options)
        : dxLaunchMessageExtractionConfig,
    runsExtraction: false,
  };
}

export async function runDxMessageExtraction(
  options: DxMessageExtractionOptions = {},
) {
  return unstable_extractMessages(createDxMessageExtractionConfig(options));
}
"#;

const NEXT_INTL_LOCALE_LINKS_TSX: &str = r#"import { useLocale } from "next-intl";

import { Link, getPathname, usePathname } from "./navigation";
import { dxLocales, type DxLocale } from "./routing";

export type DxLocaleLink = {
  locale: DxLocale;
  href: string;
  active: boolean;
  label: string;
};

export type DxLocaleLinkListProps = {
  labels?: Partial<Record<DxLocale, string>>;
  className?: string;
};

export function getDxLocaleHref(locale: DxLocale, href: "/" | "/launch" = "/launch") {
  return getPathname({
    href,
    locale,
    forcePrefix: true,
  });
}

export function getDxLocaleLinks(
  activeLocale: DxLocale,
  labels: Partial<Record<DxLocale, string>> = {},
): DxLocaleLink[] {
  return dxLocales.map((locale) => ({
    locale,
    href: getDxLocaleHref(locale),
    active: locale === activeLocale,
    label: labels[locale] ?? locale.toUpperCase(),
  }));
}

export function DxLocaleLinkList({
  labels,
  className,
}: DxLocaleLinkListProps) {
  const activeLocale = useLocale() as DxLocale;
  const pathname = usePathname();
  const links = getDxLocaleLinks(activeLocale, labels);

  return (
    <nav
      aria-label="Launch language"
      className={className}
      data-launch-i18n-locale-links={pathname}
    >
      {links.map((link) => (
        <Link
          aria-current={link.active ? "page" : undefined}
          href="/launch"
          key={link.locale}
          locale={link.locale}
        >
          {link.label}
        </Link>
      ))}
    </nav>
  );
}
"#;

const NEXT_INTL_CONTEXT_STATUS_TSX: &str = r#"import { useMessages, useNow, useTimeZone } from "next-intl";

export type DxIntlContextStatusProps = {
  updateInterval?: number;
};

export type DxIntlRuntimeContext = {
  messageNamespaceCount: number;
  now: string;
  timeZone: string | undefined;
};

export function getDxMessageNamespaceCount(messages: Record<string, unknown>) {
  return Object.keys(messages).length;
}

export function useDxIntlRuntimeContext({
  updateInterval,
}: DxIntlContextStatusProps = {}): DxIntlRuntimeContext {
  const messages = useMessages();
  const now = useNow({ updateInterval });
  const timeZone = useTimeZone();

  return {
    messageNamespaceCount: getDxMessageNamespaceCount(messages),
    now: now.toISOString(),
    timeZone,
  };
}

export function DxIntlContextStatus(props: DxIntlContextStatusProps) {
  const context = useDxIntlRuntimeContext(props);

  return (
    <output
      data-launch-i18n-message-count={context.messageNamespaceCount}
      data-launch-i18n-now={context.now}
      data-launch-i18n-time-zone={context.timeZone}
    >
      {context.messageNamespaceCount} message namespace(s)
    </output>
  );
}
"#;

const NEXT_INTL_ERROR_POLICY_TS: &str = r#"import { IntlErrorCode, type IntlError } from "next-intl";

export type DxIntlErrorPolicyEvent = {
  code: IntlErrorCode;
  key?: string;
  message: string;
  namespace?: string;
};

export type DxMessageFallbackInfo = {
  error: IntlError;
  key: string;
  namespace?: string;
};

export type DxIntlErrorReporter = (event: DxIntlErrorPolicyEvent) => void;

export function getDxMessageFallback({
  error,
  key,
  namespace,
}: DxMessageFallbackInfo) {
  const scope = namespace ? `${namespace}.${key}` : key;

  if (error.code === IntlErrorCode.MISSING_MESSAGE) {
    return `[missing:${scope}]`;
  }

  if (error.code === IntlErrorCode.FORMATTING_ERROR) {
    return `[format-error:${scope}]`;
  }

  return `[intl-error:${scope}]`;
}

export function createDxIntlErrorPolicy(
  report: DxIntlErrorReporter = console.warn,
) {
  return {
    onError(error: IntlError) {
      report({
        code: error.code,
        message: error.message,
      });
    },
    getMessageFallback: getDxMessageFallback,
  };
}

export const dxIntlErrorPolicy = createDxIntlErrorPolicy();
"#;

const NEXT_INTL_CATALOG_VALIDATION_TS: &str = r#"import {
  type AbstractIntlMessages,
  initializeConfig,
  type IntlConfig,
  type IntlError,
} from "next-intl";

import { dxIntlErrorPolicy } from "./error-policy";
import { dxDefaultLocale, dxLocales, type DxLocale } from "./routing";

export type DxIntlCatalogValidationResult = {
  errors: IntlError[];
  locale: DxLocale;
  valid: boolean;
};

export type DxIntlCatalogValidationInput = {
  locale: DxLocale;
  messages: AbstractIntlMessages;
  timeZone?: IntlConfig["timeZone"];
};

export function validateDxIntlCatalog({
  locale,
  messages,
  timeZone = "UTC",
}: DxIntlCatalogValidationInput): DxIntlCatalogValidationResult {
  const errors: IntlError[] = [];

  initializeConfig({
    getMessageFallback: dxIntlErrorPolicy.getMessageFallback,
    locale,
    messages,
    onError(error) {
      errors.push(error);
      dxIntlErrorPolicy.onError?.(error);
    },
    timeZone,
  });

  return {
    errors,
    locale,
    valid: errors.length === 0,
  };
}

export function validateDxLaunchCatalogs(
  catalogs: Partial<Record<DxLocale, AbstractIntlMessages>>,
) {
  return dxLocales.map((locale) =>
    validateDxIntlCatalog({
      locale,
      messages: catalogs[locale] ?? catalogs[dxDefaultLocale] ?? {},
    }),
  );
}
"#;

const NEXT_INTL_DASHBOARD_COPY_TS: &str = r#"import { dxLocales, type DxLocale } from "./routing";

export type DxDashboardLocaleMessage = {
  copySummary: string;
  label: string;
  planLabel: string;
  readiness: string;
  routePreview: string;
  supportSla: string;
  title: string;
};

export type DxDashboardLocaleRoutePreview = {
  href: string;
  hrefLang: "en" | "bn-BD";
  locale: DxLocale;
  localePrefix: "as-needed";
  seoBoundary: "app-owned-alternate-link-review";
};

export type DxDashboardLocaleAlternateLink = {
  href: string;
  hrefLang: DxDashboardLocaleRoutePreview["hrefLang"];
  label: string;
  locale: DxLocale;
  rel: "alternate";
};

export const dxDashboardIntlPreviewWindowIso = "2026-05-23T10:00:00.000Z";

export const dxDashboardIntlFormatOptions = {
  dateStyle: "medium",
  timeStyle: "short",
  timeZone: "UTC",
} as const satisfies Intl.DateTimeFormatOptions;

export const dxDashboardIntlPlanMonthlyUsd = 49;

export const dxDashboardIntlNumberOptions = {
  currency: "USD",
  maximumFractionDigits: 0,
  style: "currency",
} as const satisfies Intl.NumberFormatOptions;

export type DxDashboardIntlFormatPreview = {
  dateIso: typeof dxDashboardIntlPreviewWindowIso;
  dateStyle: typeof dxDashboardIntlFormatOptions.dateStyle;
  formatted: string;
  sourceApi: "useFormatter.dateTime";
  timeStyle: typeof dxDashboardIntlFormatOptions.timeStyle;
  timeZone: typeof dxDashboardIntlFormatOptions.timeZone;
};

export type DxDashboardIntlNumberPreview = {
  amount: typeof dxDashboardIntlPlanMonthlyUsd;
  currency: typeof dxDashboardIntlNumberOptions.currency;
  formatted: string;
  maximumFractionDigits: typeof dxDashboardIntlNumberOptions.maximumFractionDigits;
  sourceApi: "useFormatter.number";
  style: typeof dxDashboardIntlNumberOptions.style;
};

export type DxDashboardIntlReceipt = {
  packageId: "i18n/next-intl";
  component: "next-intl-dashboard-locale-workflow";
  alternateLinks: readonly DxDashboardLocaleAlternateLink[];
  formatPreview: DxDashboardIntlFormatPreview;
  numberPreview: DxDashboardIntlNumberPreview;
  locale: DxLocale;
  messageNamespace: "LaunchDashboard";
  providerLocale: string;
  previewWindow: string;
  routePreview: string;
  routePreviewDetails: DxDashboardLocaleRoutePreview;
  sourceApis: readonly string[];
  appOwnedBoundaries: readonly string[];
};

export const dxDashboardIntlMessages = {
  en: {
    label: "English",
    title: "Review launch copy in English",
    copySummary:
      "Mission control, plan copy, route preview, and support response text are using the English dashboard lane.",
    readiness: "English copy is ready for product review.",
    planLabel: "Launch Pro",
    supportSla: "Same business day",
    routePreview: "/launch",
  },
  bn: {
    label: "বাংলা",
    title: "লঞ্চ কপি বাংলায় পর্যালোচনা করুন",
    copySummary:
      "মিশন কন্ট্রোল বাংলা কপি, প্ল্যান, রুট প্রিভিউ ও সহায়তা SLA একই ড্যাশবোর্ডে দেখাচ্ছে।",
    readiness: "বাংলা কপি পণ্য পর্যালোচনার জন্য প্রস্তুত।",
    planLabel: "লঞ্চ প্রো",
    supportSla: "পরবর্তী কর্মদিবস",
    routePreview: "/bn",
  },
} as const satisfies Record<DxLocale, DxDashboardLocaleMessage>;

export const dxDashboardLocaleRoutePreviews = {
  en: {
    href: dxDashboardIntlMessages.en.routePreview,
    hrefLang: "en",
    locale: "en",
    localePrefix: "as-needed",
    seoBoundary: "app-owned-alternate-link-review",
  },
  bn: {
    href: dxDashboardIntlMessages.bn.routePreview,
    hrefLang: "bn-BD",
    locale: "bn",
    localePrefix: "as-needed",
    seoBoundary: "app-owned-alternate-link-review",
  },
} as const satisfies Record<DxLocale, DxDashboardLocaleRoutePreview>;

export const dxDashboardLocaleAlternateLinks = dxLocales.map((locale) => ({
  href: dxDashboardLocaleRoutePreviews[locale].href,
  hrefLang: dxDashboardLocaleRoutePreviews[locale].hrefLang,
  label: dxDashboardIntlMessages[locale].label,
  locale,
  rel: "alternate" as const,
})) satisfies readonly DxDashboardLocaleAlternateLink[];

export function getDxDashboardIntlCopy(locale: DxLocale) {
  return dxDashboardIntlMessages[locale] ?? dxDashboardIntlMessages.en;
}

export function getDxDashboardLocaleAlternateLinks(): readonly DxDashboardLocaleAlternateLink[] {
  return dxDashboardLocaleAlternateLinks;
}

export function getDxDashboardLocaleRoutePreview(
  locale: DxLocale,
): DxDashboardLocaleRoutePreview {
  return dxDashboardLocaleRoutePreviews[locale] ?? dxDashboardLocaleRoutePreviews.en;
}

export function createDxDashboardIntlFormatPreview(
  formatted: string,
): DxDashboardIntlFormatPreview {
  return {
    dateIso: dxDashboardIntlPreviewWindowIso,
    dateStyle: dxDashboardIntlFormatOptions.dateStyle,
    formatted,
    sourceApi: "useFormatter.dateTime",
    timeStyle: dxDashboardIntlFormatOptions.timeStyle,
    timeZone: dxDashboardIntlFormatOptions.timeZone,
  };
}

export function createDxDashboardIntlNumberPreview(
  formatted: string,
): DxDashboardIntlNumberPreview {
  return {
    amount: dxDashboardIntlPlanMonthlyUsd,
    currency: dxDashboardIntlNumberOptions.currency,
    formatted,
    maximumFractionDigits: dxDashboardIntlNumberOptions.maximumFractionDigits,
    sourceApi: "useFormatter.number",
    style: dxDashboardIntlNumberOptions.style,
  };
}

export function createDxDashboardIntlReceipt({
  locale,
  planPricePreview = "$49",
  previewWindow,
  providerLocale,
}: {
  locale: DxLocale;
  planPricePreview?: string;
  previewWindow: string;
  providerLocale: string;
}): DxDashboardIntlReceipt {
  const copy = getDxDashboardIntlCopy(locale);
  const routePreviewDetails = getDxDashboardLocaleRoutePreview(locale);
  const formatPreview = createDxDashboardIntlFormatPreview(previewWindow);
  const numberPreview = createDxDashboardIntlNumberPreview(planPricePreview);

  return {
    packageId: "i18n/next-intl",
    component: "next-intl-dashboard-locale-workflow",
    alternateLinks: getDxDashboardLocaleAlternateLinks(),
    formatPreview,
    numberPreview,
    locale,
    messageNamespace: "LaunchDashboard",
    providerLocale,
    previewWindow,
    routePreview: copy.routePreview,
    routePreviewDetails,
    sourceApis: [
      "NextIntlClientProvider",
      "useTranslations",
      "useLocale",
      "useFormatter",
      "defineRouting",
      "createNavigation",
      "getRequestConfig",
      "createMiddleware",
    ],
    appOwnedBoundaries: [
      "Message quality",
      "Locale routing policy",
      "Middleware placement",
      "SEO metadata",
    ],
  };
}
"#;

const NEXT_INTL_DASHBOARD_LOCALE_WORKFLOW_TSX: &str = r#""use client";

import * as React from "react";
import { useFormatter, useLocale, useTranslations } from "next-intl";

import {
  createDxDashboardIntlFormatPreview,
  createDxDashboardIntlNumberPreview,
  createDxDashboardIntlReceipt,
  dxDashboardIntlFormatOptions,
  dxDashboardIntlMessages,
  dxDashboardIntlNumberOptions,
  dxDashboardIntlPlanMonthlyUsd,
  dxDashboardIntlPreviewWindowIso,
  getDxDashboardIntlCopy,
  getDxDashboardLocaleAlternateLinks,
  getDxDashboardLocaleRoutePreview,
  type DxDashboardIntlReceipt,
} from "./dashboard-copy";
import { type DxLocale, dxLocales } from "./routing";

export type DxNextIntlDashboardLocaleWorkflowProps = {
  initialLocale?: DxLocale;
};

function normalizeDxDashboardLocale(locale: string | undefined): DxLocale {
  return locale === "bn" ? "bn" : "en";
}

export function DxNextIntlDashboardLocaleWorkflow({
  initialLocale,
}: DxNextIntlDashboardLocaleWorkflowProps) {
  const providerLocale = useLocale();
  const launchT = useTranslations("Launch");
  const dashboardT = useTranslations("LaunchDashboard");
  const format = useFormatter();
  const [selectedLocale, setSelectedLocale] = React.useState<DxLocale>(() =>
    normalizeDxDashboardLocale(initialLocale ?? providerLocale),
  );
  const previewWindow = format.dateTime(
    new Date(dxDashboardIntlPreviewWindowIso),
    dxDashboardIntlFormatOptions,
  );
  const formatPreview = createDxDashboardIntlFormatPreview(previewWindow);
  const planPricePreview = format.number(
    dxDashboardIntlPlanMonthlyUsd,
    dxDashboardIntlNumberOptions,
  );
  const numberPreview = createDxDashboardIntlNumberPreview(planPricePreview);
  const [receipt, setReceipt] = React.useState<DxDashboardIntlReceipt>(() =>
    createDxDashboardIntlReceipt({
      locale: normalizeDxDashboardLocale(initialLocale ?? providerLocale),
      planPricePreview,
      previewWindow,
      providerLocale,
    }),
  );
  const fallbackCopy = getDxDashboardIntlCopy(selectedLocale);

  function message(key: keyof typeof fallbackCopy) {
    return dashboardT(`${selectedLocale}.${key}`) || fallbackCopy[key];
  }
  const routePreview = getDxDashboardLocaleRoutePreview(selectedLocale);
  const alternateLinks = getDxDashboardLocaleAlternateLinks();

  function switchLocale(locale: DxLocale) {
    setSelectedLocale(locale);
    setReceipt(
      createDxDashboardIntlReceipt({
        locale,
        planPricePreview,
        previewWindow,
        providerLocale,
      }),
    );
  }

  return (
    <section
      className="grid gap-4 rounded-md border bg-card p-4 text-card-foreground"
      data-dx-package="i18n/next-intl"
      data-dx-component="next-intl-dashboard-locale-workflow"
      data-dx-intl-dashboard-workflow="locale-copy-boundary"
      data-dx-dashboard-card="locale"
      data-dx-dashboard-copy-locale={selectedLocale}
      data-dx-intl-message-namespace="LaunchDashboard"
      data-dx-intl-route-preview={message("routePreview")}
      data-dx-intl-hreflang={routePreview.hrefLang}
      data-dx-intl-locale-prefix={routePreview.localePrefix}
      data-dx-intl-format-preview={formatPreview.formatted}
      data-dx-intl-format-source-api={formatPreview.sourceApi}
      data-dx-intl-format-time-zone={formatPreview.timeZone}
      data-dx-intl-number-preview={numberPreview.formatted}
      data-dx-intl-number-source-api={numberPreview.sourceApi}
      data-dx-intl-number-currency={numberPreview.currency}
      data-dx-intl-plan-label={message("planLabel")}
      data-dx-intl-support-sla={message("supportSla")}
      data-dx-intl-readiness-copy={message("readiness")}
      data-dx-icon-search="i18n:locale"
      data-dx-node-modules="forbidden"
      data-dx-product-surface="launch-dashboard"
    >
      <div className="flex items-center gap-2">
        <dx-icon name="pack:i18n" aria-hidden="true" />
        <span className="text-sm font-medium">Locale and copy</span>
      </div>
      <div className="grid gap-2">
        <h2 data-dx-editable-text="launch.intl-dashboard-title">
          {message("title")}
        </h2>
        <p data-dx-intl-copy-target="mission-summary">
          {message("copySummary")}
        </p>
        <p>{launchT("phaseStatus", { phase: "release-candidate" })}</p>
        <strong aria-live="polite" data-dx-intl-copy-target="readiness">
          {message("readiness")}
        </strong>
        <p
          data-dx-intl-copy-target="format-preview"
          data-dx-intl-format-preview={formatPreview.formatted}
          data-dx-intl-format-source-api={formatPreview.sourceApi}
          data-dx-intl-format-time-zone={formatPreview.timeZone}
        >
          Formatter preview: {formatPreview.formatted}
        </p>
        <p
          data-dx-intl-copy-target="price-preview"
          data-dx-intl-number-preview={numberPreview.formatted}
          data-dx-intl-number-source-api={numberPreview.sourceApi}
          data-dx-intl-number-currency={numberPreview.currency}
        >
          Localized plan price: {numberPreview.formatted}/mo.
        </p>
      </div>
      <div data-dx-intl-interaction="locale-switcher">
        {dxLocales.map((locale) => (
          <button
            aria-pressed={selectedLocale === locale}
            data-dx-intl-action="switch-dashboard-locale"
            data-dx-intl-locale-option={locale}
            data-dx-intl-locale-selected={selectedLocale === locale}
            key={locale}
            onClick={() => switchLocale(locale)}
            type="button"
          >
            {dxDashboardIntlMessages[locale].label}
          </button>
        ))}
      </div>
      <dl>
        <div>
          <dt>Plan copy</dt>
          <dd data-dx-intl-copy-target="plan-label">{message("planLabel")}</dd>
        </div>
        <div>
          <dt>Support SLA</dt>
          <dd data-dx-intl-copy-target="support-sla">{message("supportSla")}</dd>
        </div>
        <div>
          <dt>Route preview</dt>
          <dd data-dx-intl-copy-target="route-preview">
            <a
              data-dx-intl-action="preview-localized-route"
              data-dx-intl-hreflang={routePreview.hrefLang}
              data-dx-intl-locale-prefix={routePreview.localePrefix}
              href={routePreview.href}
              hrefLang={routePreview.hrefLang}
            >
              {message("routePreview")}
            </a>
          </dd>
        </div>
      </dl>
      <div data-dx-intl-alternate-links="launch-dashboard">
        <span>Alternate links</span>
        {alternateLinks.map((link) => (
          <a
            data-dx-intl-action="review-alternate-link"
            data-dx-intl-alternate-href={link.href}
            data-dx-intl-alternate-locale={link.locale}
            href={link.href}
            hrefLang={link.hrefLang}
            key={link.locale}
            rel={link.rel}
          >
            {link.hrefLang}
          </a>
        ))}
      </div>
      <button
        data-dx-intl-action="prepare-dashboard-intl-receipt"
        onClick={() =>
          setReceipt(
            createDxDashboardIntlReceipt({
              locale: selectedLocale,
              planPricePreview,
              previewWindow,
              providerLocale,
            }),
          )
        }
        type="button"
      >
        Prepare locale receipt
      </button>
      <output
        data-dx-intl-receipt-locale={receipt.locale}
        data-dx-intl-receipt-route={receipt.routePreview}
        data-dx-intl-receipt-hreflang={receipt.routePreviewDetails.hrefLang}
        data-dx-intl-receipt-format-source={receipt.formatPreview.sourceApi}
        data-dx-intl-receipt-number-source={receipt.numberPreview.sourceApi}
        data-dx-intl-receipt-state="ready"
      >
        {receipt.packageId} uses {receipt.messageNamespace} for{" "}
        {receipt.routePreview}.
      </output>
    </section>
  );
}
"#;

const NEXT_INTL_MESSAGES_EN_JSON: &str = r#"{
  "Launch": {
    "title": "DX launch",
    "status": "Internationalized launch route is ready",
    "phaseStatus": "Launch phase {phase} is ready for application-owned translation review",
    "richDescription": "Review <guide>translation ownership</guide> before launch."
  },
  "LaunchDashboard": {
    "en": {
      "title": "Review launch copy in English",
      "copySummary": "Mission control, plan copy, route preview, and support response text are using the English dashboard lane.",
      "readiness": "English copy is ready for product review.",
      "planLabel": "Launch Pro",
      "supportSla": "Same business day",
      "routePreview": "/launch"
    },
    "bn": {
      "title": "লঞ্চ কপি বাংলায় পর্যালোচনা করুন",
      "copySummary": "মিশন কন্ট্রোল বাংলা কপি, প্ল্যান, রুট প্রিভিউ ও সহায়তা SLA একই ড্যাশবোর্ডে দেখাচ্ছে।",
      "readiness": "বাংলা কপি পণ্য পর্যালোচনার জন্য প্রস্তুত।",
      "planLabel": "লঞ্চ প্রো",
      "supportSla": "পরবর্তী কর্মদিবস",
      "routePreview": "/bn"
    }
  }
}
"#;

const NEXT_INTL_MESSAGES_BN_JSON: &str = r#"{
  "Launch": {
    "title": "DX launch",
    "status": "Internationalized launch route is ready",
    "phaseStatus": "Launch phase {phase} is ready for application-owned translation review",
    "richDescription": "Review <guide>translation ownership</guide> before launch."
  },
  "LaunchDashboard": {
    "en": {
      "title": "Review launch copy in English",
      "copySummary": "Mission control, plan copy, route preview, and support response text are using the English dashboard lane.",
      "readiness": "English copy is ready for product review.",
      "planLabel": "Launch Pro",
      "supportSla": "Same business day",
      "routePreview": "/launch"
    },
    "bn": {
      "title": "লঞ্চ কপি বাংলায় পর্যালোচনা করুন",
      "copySummary": "মিশন কন্ট্রোল বাংলা কপি, প্ল্যান, রুট প্রিভিউ ও সহায়তা SLA একই ড্যাশবোর্ডে দেখাচ্ছে।",
      "readiness": "বাংলা কপি পণ্য পর্যালোচনার জন্য প্রস্তুত।",
      "planLabel": "লঞ্চ প্রো",
      "supportSla": "পরবর্তী কর্মদিবস",
      "routePreview": "/bn"
    }
  }
}
"#;

const NEXT_INTL_METADATA_TS: &str = r#"export const dxNextIntlForgePackage = {
  packageId: "i18n/next-intl",
  officialPackageName: "Internationalization",
  upstreamPackage: "next-intl",
  upstreamVersion: "4.12.0",
  forgeVersion: "4.12.0-dx.0",
  aliases: ["next-intl", "intl", "i18n/next"],
  sourceMirror: "G:/WWW/inspirations/next-intl",
  provenance: {
    provider: "packages/next-intl/src/shared/NextIntlClientProvider.tsx",
    hooks: "packages/use-intl/src/react/index.tsx",
    routing: "packages/next-intl/src/routing/defineRouting.tsx",
    navigation: "packages/next-intl/src/navigation/createNavigation.tsx",
    requestConfig: "./i18n/request.ts",
  },
  requiredEnv: [],
  appOwnedBoundaries: [
    "Message quality",
    "Locale routing policy",
    "Middleware placement",
    "SEO metadata",
  ],
  receiptPaths: [
    ".dx/forge/receipts/*-i18n-next-intl.json",
    ".dx/forge/docs/i18n-next-intl.md",
    "docs/packages/next-intl.md",
    "examples/template/.dx/forge/receipts/2026-05-22-i18n-next-intl-dashboard-locale.json",
    "tools/launch/runtime-template/pages/index.html#mission-control",
  ],
  dxCheckVisibility: {
    schema: "dx.forge.package.dx_check_visibility",
    currentStatus: "present",
    statuses: ["present", "stale", "missing-receipt", "blocked", "unsupported-surface"],
    receiptPath:
      "examples/template/.dx/forge/receipts/2026-05-22-i18n-next-intl-dashboard-locale.json",
    monitoredSurfaces: [
      {
        id: "next-intl-dashboard-locale-workflow",
        status: "present",
        sourceFile: "examples/template/next-intl-dashboard-locale.tsx",
        materializedFile: "components/launch/next-intl-dashboard-locale.tsx",
        nextAction:
          "Keep visible locale switching, formatter, route, and alternate-link markers receipt-backed.",
      },
      {
        id: "next-intl-dashboard-message-contract",
        status: "present",
        sourceFile: "examples/template/next-intl-dashboard-locale-contract.ts",
        materializedFile:
          "components/launch/next-intl-dashboard-locale-contract.ts",
        nextAction:
          "Keep translated LaunchDashboard copy and receipt fields in the typed Internationalization contract.",
      },
    ],
    blockedSurfaces: [],
    unsupportedSurfaces: [
      {
        id: "production-locale-routing-runtime",
        status: "unsupported-surface",
        reason:
          "Requires app-owned middleware placement, route deployment, dependency installation, and governed browser/SEO proof.",
      },
    ],
  },
  honestyLabel: "SOURCE-ONLY",
  dashboardUsage:
    "The root route imports LaunchDashboardIntlWorkflow for locale switching, mission copy, plan copy, route preview, formatter preview, localized plan-price preview, support SLA copy, and safe locale receipt preparation.",
  dxIcon: "pack:i18n",
  publicApi: [
    "defineRouting",
    "RoutingConfig",
    "Pathnames",
    "LocalePrefix",
    "DomainsConfig",
    "LocalePrefixMode",
    "QueryParams",
    "domains",
    "localeCookie",
    "alternateLinks",
    "localeDetection",
    "createNavigation",
    "useRouter",
    "usePathname",
    "redirect",
    "getRequestConfig",
    "RequestConfig",
    "GetRequestConfigParams",
    "createMiddleware",
    "permanentRedirect",
    "notFound",
    "NextIntlClientProvider",
    "AbstractIntlMessages",
    "AppConfig",
    "Locale",
    "Messages",
    "Formats",
    "DateTimeFormatOptions",
    "NumberFormatOptions",
    "RelativeTimeFormatOptions",
    "Timezone",
    "getLocale",
    "getMessages",
    "getNow",
    "getTimeZone",
    "Metadata",
    "createTranslator",
    "createFormatter",
    "_createCache",
    "_createIntlFormatters",
    "initializeConfig",
    "IntlConfig",
    "MessageKeys",
    "NamespaceKeys",
    "NestedKeyOf",
    "NestedValueOf",
    "TranslationValues",
    "RichTranslationValues",
    "MarkupTranslationValues",
    "ICUArgs",
    "ICUTags",
    "RichTagsFunction",
    "MarkupTagsFunction",
    "useTranslations",
    "t.rich",
    "t.markup",
    "t.raw",
    "useExtracted",
    "useFormatter",
    "useLocale",
    "useMessages",
    "useNow",
    "useTimeZone",
    "IntlError",
    "IntlErrorCode",
    "onError",
    "getMessageFallback",
    "getTranslations",
    "getFormatter",
    "getExtracted",
    "setRequestLocale",
    "hasLocale",
    "createNextIntlPlugin",
    "next-intl/plugin",
    "next-intl/extractor",
    "ExtractorConfigInput",
    "unstable_extractMessages",
    "defineCodec",
    "dxDashboardIntlMessages",
    "DxDashboardIntlFormatPreview",
    "createDxDashboardIntlFormatPreview",
    "dxDashboardIntlNumberOptions",
    "DxDashboardIntlNumberPreview",
    "createDxDashboardIntlNumberPreview",
    "createDxDashboardIntlReceipt",
    "DxDashboardLocaleAlternateLink",
    "getDxDashboardLocaleAlternateLinks",
    "DxDashboardLocaleRoutePreview",
    "getDxDashboardLocaleRoutePreview",
    "DxNextIntlDashboardLocaleWorkflow",
    "LaunchDashboard",
  ],
  exportedFiles: [
    "i18n/routing.ts",
    "i18n/navigation.ts",
    "i18n/request.ts",
    "i18n/provider.tsx",
    "i18n/dashboard-copy.ts",
    "i18n/dashboard-locale-workflow.tsx",
    "components/launch/next-intl-dashboard-locale.tsx",
    "examples/template/next-intl-dashboard-locale.tsx",
  ],
  materializedFiles: [
    "i18n/routing.ts",
    "i18n/route-types.ts",
    "i18n/routing-policy.ts",
    "i18n/domain-routing.ts",
    "i18n/navigation.ts",
    "i18n/navigation-actions.ts",
    "i18n/navigation-client.tsx",
    "i18n/request.ts",
    "i18n/request-config.ts",
    "i18n/request-runtime.ts",
    "i18n/app-config.ts",
    "i18n/locale-guard.ts",
    "i18n/server-context.ts",
    "i18n/route-boundary.ts",
    "i18n/middleware.ts",
    "i18n/formats.ts",
    "i18n/format-options.ts",
    "i18n/formatter-cache.ts",
    "i18n/server-provider.tsx",
    "i18n/provider.tsx",
    "i18n/formatting.tsx",
    "i18n/runtime-core.ts",
    "i18n/rich-copy.tsx",
    "i18n/extracted-copy.tsx",
    "i18n/server-extracted.ts",
    "i18n/type-contracts.ts",
    "i18n/message-arguments.ts",
    "i18n/next-config.ts",
    "i18n/extraction.ts",
    "i18n/extraction-runner.ts",
    "i18n/locale-links.tsx",
    "i18n/context-status.tsx",
    "i18n/error-policy.ts",
    "i18n/catalog-validation.ts",
    "i18n/dashboard-copy.ts",
    "i18n/dashboard-locale-workflow.tsx",
    "i18n/messages/en.json",
    "i18n/messages/bn.json",
    "i18n/metadata.ts",
    "i18n/README.md",
  ],
  requiredDependencies: [
    {
      name: "next-intl",
      version: "^4.12.0",
      reason: "Provides App Router routing, middleware, navigation, request config, provider, and translation APIs.",
    },
    {
      name: "next",
      version: "^12 || ^13 || ^14 || ^15 || ^16",
      reason: "Peer dependency required by next-intl route handlers, middleware, and App Router integration.",
    },
    {
      name: "react",
      version: "^18 || ^19",
      reason: "Peer dependency required by next-intl client provider and hooks.",
    },
  ],
  cli: {
    dxAdd: "dx add next-intl --write",
    dxDryRun: "dx add next-intl --dry-run --format json",
  },
} as const;

export type DxNextIntlForgePackageMetadata = typeof dxNextIntlForgePackage;
"#;

const NEXT_INTL_README_MD: &str = r#"# DX Forge next-intl Slice

This package materializes a small source-owned App Router i18n slice around the real `next-intl` 4 public API. It does not reimplement message formatting, hide routing behind fake helpers, or run package lifecycle scripts.

## Owned Files

- `i18n/routing.ts` defines launch locales through `defineRouting`.
- `i18n/route-types.ts` exposes route type helpers through `Pathnames`, `LocalePrefix`, `DomainsConfig`, `LocalePrefixMode`, and `QueryParams`.
- `i18n/routing-policy.ts` exposes routing policy helpers for `domains`, `localeCookie`, `alternateLinks`, and `localeDetection`.
- `i18n/domain-routing.ts` exposes domain routing helpers through `defineRouting` and `RoutingConfig` while keeping production hostnames app-owned.
- `i18n/navigation.ts` exports typed navigation helpers from `createNavigation`.
- `i18n/navigation-actions.ts` exposes localized redirect action helpers through `redirect`, `permanentRedirect`, and `getPathname`.
- `i18n/navigation-client.tsx` exposes client locale navigation actions through `useRouter`, `usePathname`, and transition-safe locale switching.
- `i18n/request.ts` wires `getRequestConfig`, `hasLocale`, and local message loading.
- `i18n/request-config.ts` exposes typed request config helpers through `GetRequestConfigParams`, `RequestConfig`, explicit locale overrides, and request-locale fallback.
- `i18n/request-runtime.ts` exposes request runtime configuration for `formats`, `timeZone`, `onError`, and `getMessageFallback`.
- `i18n/app-config.ts` exposes AppConfig, Locale, and Messages helpers for app-owned next-intl type configuration.
- `i18n/locale-guard.ts` exposes route locale guard helpers through `hasLocale` and Next.js `notFound`.
- `i18n/server-context.ts` exposes server request context helpers through `getLocale`, `getMessages`, `getNow`, and `getTimeZone`.
- `i18n/route-boundary.ts` exposes static route and metadata helpers through `setRequestLocale`, `getTranslations`, and `getPathname`.
- `i18n/middleware.ts` exposes `createMiddleware(routing)` through a reusable factory and launch-safe matcher config.
- `i18n/formats.ts` exposes global date and number format presets through the upstream `Formats` type.
- `i18n/format-options.ts` exposes typed format option helpers through `DateTimeFormatOptions`, `NumberFormatOptions`, `RelativeTimeFormatOptions`, and `Timezone`.
- `i18n/formatter-cache.ts` exposes formatter cache helpers through `_createCache`, `_createIntlFormatters`, and `createFormatter` for non-React launch surfaces.
- `i18n/server-provider.tsx` exposes an App Router server provider through `NextIntlClientProvider`, `getLocale`, `getMessages`, `getNow`, and `getTimeZone`.
- `i18n/provider.tsx` wraps `NextIntlClientProvider` for client layouts.
- `i18n/formatting.tsx` adds launch-ready client and server formatting helpers through `useFormatter`, `useLocale`, `getFormatter`, `getTranslations`, and `setRequestLocale`.
- `i18n/runtime-core.ts` exposes core translator and formatter helpers through `createTranslator` and `createFormatter` for non-React launch surfaces.
- `i18n/rich-copy.tsx` exposes rich, markup, and raw translation helpers through `useTranslations().rich`, `useTranslations().markup`, and `useTranslations().raw`.
- `i18n/extracted-copy.tsx` exposes inline extracted copy helpers through `useExtracted` for apps that opt into the extractor/compiler workflow.
- `i18n/server-extracted.ts` exposes server extracted copy helpers through `getExtracted` for apps that opt into the extractor/compiler workflow.
- `i18n/type-contracts.ts` exposes typed message contract helpers through `MessageKeys`, `NamespaceKeys`, `NestedKeyOf`, `NestedValueOf`, and translation value types.
- `i18n/message-arguments.ts` exposes ICU message argument helpers through `ICUArgs`, `ICUTags`, `RichTagsFunction`, and `MarkupTagsFunction`.
- `i18n/next-config.ts` exposes the real next-intl/plugin boundary through `createNextIntlPlugin` for apps that want a typed `next.config` merge point.
- `i18n/extraction.ts` exposes typed extraction config from `next-intl/extractor` for apps that want to opt into static message extraction later; it does not run extraction, loaders, watchers, or catalog writes.
- `i18n/extraction-runner.ts` exposes opt-in extractor runner helpers through `unstable_extractMessages` and `defineCodec`; importing the file does not run extraction.
- `i18n/locale-links.tsx` adds locale switch links backed by createNavigation, including `Link`, `getPathname`, and `usePathname`.
- `i18n/context-status.tsx` exposes runtime context diagnostics through `useMessages`, `useNow`, and `useTimeZone`.
- `i18n/error-policy.ts` exposes missing-message and formatting fallback policy through `IntlError`, `IntlErrorCode`, `onError`, and `getMessageFallback`.
- `i18n/catalog-validation.ts` exposes catalog config validation helpers through `initializeConfig`, `IntlConfig`, and the local error policy.
- `i18n/dashboard-copy.ts` exposes `dxDashboardIntlMessages`, `createDxDashboardIntlFormatPreview`, `createDxDashboardIntlNumberPreview`, `getDxDashboardLocaleAlternateLinks`, `getDxDashboardLocaleRoutePreview`, and `createDxDashboardIntlReceipt` for the launch dashboard locale workflow.
- `i18n/dashboard-locale-workflow.tsx` exposes `DxNextIntlDashboardLocaleWorkflow` for mission copy, plan copy, route preview, formatter preview, alternate-link review, support SLA copy, locale switching, and receipt preparation.
- `i18n/messages/*.json` provides editable starter messages.
- `i18n/metadata.ts` lets DX CLI, Zed, and launch templates discover the upstream package and supported public API surface.

## Real And App-Owned Boundaries

Real: Forge wires next-intl routing, route type helpers, routing policy options, domain routing helpers, navigation, localized redirect actions, client locale navigation, permanent redirects, locale links, middleware, request config, typed request config helpers, request runtime config, app config types, route locale guards, server request context, static locale params, route metadata, client and server providers, global format presets, typed format options, formatter cache helpers, translated copy, rich translation copy, inline and server extracted copy helpers, typed message contracts, ICU message argument helpers, catalog config validation, dashboard copy and receipt helpers, date formatting, relative-time formatting, core translator and formatter helpers, runtime context diagnostics, provider error handling, message fallback policy, server metadata helpers, the Next config plugin boundary, typed extractor configuration, and opt-in extractor runner helpers against the upstream public API.

App-owned: translated message quality, locale list, route localization policy, production domain names, middleware placement, static rendering policy, extractor compiler enablement, telemetry destination for intl errors, fallback copy tone, and SEO alternates.

Deferred: running extraction, catalog writes, PO/JSON compilation, extractor compilation for production `useExtracted` calls, and locale-management workflows stay outside this launch slice until an app opts into those upstream systems.

## Required App Dependency

Install or provide `next-intl`, Next.js, and React in the app runtime. Forge owns these adapter files and receipts; it does not vendor next-intl internals.

## Template Usage

```tsx
import { useTranslations } from "next-intl";
import { DxLaunchIntlSummary } from "@/i18n/formatting";

export function LaunchStatus() {
  const t = useTranslations("Launch");

  return (
    <section>
      <h1>{t("title")}</h1>
      <p>{t("status")}</p>
      <p>{t("phaseStatus", { phase: "beta" })}</p>
      <DxLaunchIntlSummary phase="beta" />
    </section>
  );
}
```

Mount `i18n/middleware.ts` from your app proxy or middleware entry and import `i18n/request.ts` from the location expected by your Next.js app.

For `next.config.ts`, apps can compose the plugin without letting Forge own unrelated Next settings:

```ts
import { withDxNextIntl } from "./i18n/next-config";

export default withDxNextIntl({
  nextConfig: {
    reactStrictMode: true,
  },
});
```

To opt into extraction later, import `dxLaunchMessageExtractionConfig` from `i18n/extraction.ts` and pass its fields into the `experimental` section of the next-intl plugin config after the app chooses catalog ownership.
"#;
