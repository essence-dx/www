export type LaunchDashboardLocale = "en" | "bn";

export type DashboardLocaleCopy = {
  label: string;
  planLabel: string;
  readiness: string;
  routePreview: string;
  summary: string;
  supportSla: string;
  title: string;
};

export const dashboardIntlPreviewWindowIso = "2026-05-23T10:00:00.000Z";

export const dashboardIntlFormatOptions = {
  dateStyle: "medium",
  timeStyle: "short",
  timeZone: "UTC",
} as const satisfies Intl.DateTimeFormatOptions;

export const dashboardIntlPlanMonthlyUsd = 49;

export const dashboardIntlNumberOptions = {
  currency: "USD",
  maximumFractionDigits: 0,
  style: "currency",
} as const satisfies Intl.NumberFormatOptions;

export type DashboardIntlFormatPreview = {
  dateIso: typeof dashboardIntlPreviewWindowIso;
  dateStyle: typeof dashboardIntlFormatOptions.dateStyle;
  formatted: string;
  sourceApi: "useFormatter.dateTime";
  timeStyle: typeof dashboardIntlFormatOptions.timeStyle;
  timeZone: typeof dashboardIntlFormatOptions.timeZone;
};

export type DashboardIntlNumberPreview = {
  amount: typeof dashboardIntlPlanMonthlyUsd;
  currency: typeof dashboardIntlNumberOptions.currency;
  formatted: string;
  maximumFractionDigits: typeof dashboardIntlNumberOptions.maximumFractionDigits;
  sourceApi: "useFormatter.number";
  style: typeof dashboardIntlNumberOptions.style;
};

export type DashboardLocaleRoutePreview = {
  href: string;
  hrefLang: "en" | "bn-BD";
  locale: LaunchDashboardLocale;
  localePrefix: "as-needed";
  seoBoundary: "app-owned-alternate-link-review";
};

export type DashboardLocaleAlternateLink = {
  href: string;
  hrefLang: DashboardLocaleRoutePreview["hrefLang"];
  label: string;
  locale: LaunchDashboardLocale;
  rel: "alternate";
};

export type DashboardIntlReceipt = {
  packageId: "i18n/next-intl";
  component: "next-intl-dashboard-locale-workflow";
  alternateLinks: readonly DashboardLocaleAlternateLink[];
  formatPreview: DashboardIntlFormatPreview;
  numberPreview: DashboardIntlNumberPreview;
  locale: LaunchDashboardLocale;
  messageNamespace: "LaunchDashboard";
  providerLocale: string;
  previewWindow: string;
  routePreview: string;
  routePreviewDetails: DashboardLocaleRoutePreview;
  sourceApis: readonly string[];
  appOwnedBoundaries: readonly string[];
};

const nextIntlDashboardSourceApis = [
  "NextIntlClientProvider",
  "useTranslations",
  "useLocale",
  "useFormatter",
  "defineRouting",
  "createNavigation",
  "getRequestConfig",
  "createMiddleware",
] as const;

const appOwnedDashboardIntlBoundaries = [
  "Message quality",
  "Locale routing policy",
  "Middleware placement",
  "SEO metadata",
] as const;

export const dashboardCopy = {
  en: {
    label: "English",
    planLabel: "Launch Pro",
    routePreview: "/",
    title: "Review launch copy in English",
    summary:
      "Mission control, plan copy, route preview, and support response text are using the English dashboard lane.",
    readiness: "English copy is ready for product review.",
    supportSla: "Same business day",
  },
  bn: {
    label: "বাংলা",
    planLabel: "লঞ্চ প্রো",
    routePreview: "/bn",
    title: "লঞ্চ কপি বাংলায় পর্যালোচনা করুন",
    summary:
      "মিশন কন্ট্রোল বাংলা কপি, প্ল্যান, রুট প্রিভিউ ও সহায়তা SLA একই ড্যাশবোর্ডে দেখাচ্ছে।",
    readiness: "বাংলা কপি পণ্য পর্যালোচনার জন্য প্রস্তুত।",
    supportSla: "পরবর্তী কর্মদিবস",
  },
} as const satisfies Record<LaunchDashboardLocale, DashboardLocaleCopy>;

export const localeOptions = Object.keys(
  dashboardCopy,
) as LaunchDashboardLocale[];

export const dashboardLocaleRoutePreviews = {
  en: {
    href: dashboardCopy.en.routePreview,
    hrefLang: "en",
    locale: "en",
    localePrefix: "as-needed",
    seoBoundary: "app-owned-alternate-link-review",
  },
  bn: {
    href: dashboardCopy.bn.routePreview,
    hrefLang: "bn-BD",
    locale: "bn",
    localePrefix: "as-needed",
    seoBoundary: "app-owned-alternate-link-review",
  },
} as const satisfies Record<
  LaunchDashboardLocale,
  DashboardLocaleRoutePreview
>;

export const dashboardLocaleAlternateLinks = localeOptions.map((locale) => ({
  href: dashboardLocaleRoutePreviews[locale].href,
  hrefLang: dashboardLocaleRoutePreviews[locale].hrefLang,
  label: dashboardCopy[locale].label,
  locale,
  rel: "alternate" as const,
})) satisfies readonly DashboardLocaleAlternateLink[];

export function normalizeLocale(locale: string): LaunchDashboardLocale {
  return locale === "bn" ? "bn" : "en";
}

export function getDashboardLocaleAlternateLinks(): readonly DashboardLocaleAlternateLink[] {
  return dashboardLocaleAlternateLinks;
}

export function createDashboardIntlFormatPreview(
  formatted: string,
): DashboardIntlFormatPreview {
  return {
    dateIso: dashboardIntlPreviewWindowIso,
    dateStyle: dashboardIntlFormatOptions.dateStyle,
    formatted,
    sourceApi: "useFormatter.dateTime",
    timeStyle: dashboardIntlFormatOptions.timeStyle,
    timeZone: dashboardIntlFormatOptions.timeZone,
  };
}

export function createDashboardIntlNumberPreview(
  formatted: string,
): DashboardIntlNumberPreview {
  return {
    amount: dashboardIntlPlanMonthlyUsd,
    currency: dashboardIntlNumberOptions.currency,
    formatted,
    maximumFractionDigits: dashboardIntlNumberOptions.maximumFractionDigits,
    sourceApi: "useFormatter.number",
    style: dashboardIntlNumberOptions.style,
  };
}

export function getDashboardLocaleRoutePreview(
  locale: LaunchDashboardLocale,
): DashboardLocaleRoutePreview {
  return dashboardLocaleRoutePreviews[locale];
}

export function createDxDashboardIntlReceipt({
  locale,
  planPricePreview = "$49",
  previewWindow,
  providerLocale,
}: {
  locale: LaunchDashboardLocale;
  planPricePreview?: string;
  previewWindow: string;
  providerLocale: string;
}): DashboardIntlReceipt {
  const copy = dashboardCopy[locale];
  const routePreviewDetails = getDashboardLocaleRoutePreview(locale);
  const formatPreview = createDashboardIntlFormatPreview(previewWindow);
  const numberPreview = createDashboardIntlNumberPreview(planPricePreview);

  return {
    packageId: "i18n/next-intl",
    component: "next-intl-dashboard-locale-workflow",
    alternateLinks: getDashboardLocaleAlternateLinks(),
    formatPreview,
    numberPreview,
    locale,
    messageNamespace: "LaunchDashboard",
    providerLocale,
    previewWindow,
    routePreview: copy.routePreview,
    routePreviewDetails,
    sourceApis: nextIntlDashboardSourceApis,
    appOwnedBoundaries: appOwnedDashboardIntlBoundaries,
  };
}
