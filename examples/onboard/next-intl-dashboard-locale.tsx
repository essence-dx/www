"use client";

import * as React from "react";
import { useFormatter, useLocale, useTranslations } from "next-intl";

import {
  createDashboardIntlFormatPreview,
  createDashboardIntlNumberPreview,
  createDxDashboardIntlReceipt,
  dashboardCopy,
  dashboardIntlFormatOptions,
  dashboardIntlNumberOptions,
  dashboardIntlPlanMonthlyUsd,
  dashboardIntlPreviewWindowIso,
  getDashboardLocaleAlternateLinks,
  getDashboardLocaleRoutePreview,
  localeOptions,
  normalizeLocale,
  type DashboardIntlReceipt,
  type LaunchDashboardLocale,
} from "./next-intl-dashboard-locale-contract";

export function LaunchDashboardIntlWorkflow() {
  const providerLocale = useLocale();
  const launchT = useTranslations("Launch");
  const dashboardT = useTranslations("LaunchDashboard");
  const format = useFormatter();
  const [selectedLocale, setSelectedLocale] =
    React.useState<LaunchDashboardLocale>(() => normalizeLocale(providerLocale));
  const previewWindow = format.dateTime(
    new Date(dashboardIntlPreviewWindowIso),
    dashboardIntlFormatOptions,
  );
  const pricePreview = format.number(
    dashboardIntlPlanMonthlyUsd,
    dashboardIntlNumberOptions,
  );
  const formatPreview = createDashboardIntlFormatPreview(previewWindow);
  const numberPreview = createDashboardIntlNumberPreview(pricePreview);
  const [receipt, setReceipt] = React.useState<DashboardIntlReceipt>(() =>
    createDxDashboardIntlReceipt({
      locale: normalizeLocale(providerLocale),
      planPricePreview: pricePreview,
      previewWindow,
      providerLocale,
    }),
  );
  const copy = dashboardCopy[selectedLocale];
  const translatedCopy = {
    planLabel: dashboardT(`${selectedLocale}.planLabel`) || copy.planLabel,
    readiness: dashboardT(`${selectedLocale}.readiness`) || copy.readiness,
    routePreview:
      dashboardT(`${selectedLocale}.routePreview`) || copy.routePreview,
    summary: dashboardT(`${selectedLocale}.copySummary`) || copy.summary,
    supportSla: dashboardT(`${selectedLocale}.supportSla`) || copy.supportSla,
    title: dashboardT(`${selectedLocale}.title`) || copy.title,
  };
  const routePreview = getDashboardLocaleRoutePreview(selectedLocale);
  const alternateLinks = getDashboardLocaleAlternateLinks();

  function switchLocale(locale: LaunchDashboardLocale) {
    setSelectedLocale(locale);
    setReceipt(
      createDxDashboardIntlReceipt({
        locale,
        planPricePreview: pricePreview,
        previewWindow,
        providerLocale,
      }),
    );
  }

  return (
    <section
      className="grid gap-4 rounded-md border bg-card p-4 text-card-foreground lg:grid-cols-[minmax(0,0.72fr)_minmax(0,1.28fr)]"
      data-dx-package="i18n/next-intl"
      data-dx-component="next-intl-dashboard-locale-workflow"
      data-dx-intl-dashboard-workflow="locale-copy-boundary"
      data-dx-dashboard-card="locale"
      data-dx-dashboard-copy-locale={selectedLocale}
      data-dx-intl-message-namespace="LaunchDashboard"
      data-dx-intl-route-preview={translatedCopy.routePreview}
      data-dx-intl-hreflang={routePreview.hrefLang}
      data-dx-intl-locale-prefix={routePreview.localePrefix}
      data-dx-intl-format-preview={formatPreview.formatted}
      data-dx-intl-format-source-api={formatPreview.sourceApi}
      data-dx-intl-format-time-zone={formatPreview.timeZone}
      data-dx-intl-number-preview={numberPreview.formatted}
      data-dx-intl-number-source-api={numberPreview.sourceApi}
      data-dx-intl-number-currency={numberPreview.currency}
      data-dx-intl-plan-label={translatedCopy.planLabel}
      data-dx-intl-support-sla={translatedCopy.supportSla}
      data-dx-intl-readiness-copy={translatedCopy.readiness}
      data-dx-icon-search="i18n:locale"
      data-dx-node-modules="forbidden"
      data-dx-product-surface="launch-dashboard"
      data-dx-source-file="examples/template/next-intl-dashboard-locale.tsx"
      data-dx-style-surface="internationalization"
    >
      <div className="grid content-start gap-3">
        <div className="flex items-center gap-2">
          <dx-icon name="pack:i18n" aria-hidden="true" />
          <span className="text-sm font-medium">Locale and copy</span>
        </div>
        <h2
          className="text-2xl font-semibold tracking-normal"
          data-dx-editable-text="launch.intl-dashboard-title"
        >
          {translatedCopy.title}
        </h2>
        <p
          className="text-sm leading-6 text-muted-foreground"
          data-dx-editable-text="launch.intl-dashboard-copy"
          data-dx-intl-copy-target="mission-summary"
        >
          {translatedCopy.summary}
        </p>
        <p className="text-xs text-muted-foreground">
          {launchT("phaseStatus", { phase: "release-candidate" })} -{" "}
          {previewWindow}
        </p>
        <p
          className="text-xs text-muted-foreground"
          data-dx-intl-copy-target="format-preview"
          data-dx-intl-format-preview={formatPreview.formatted}
          data-dx-intl-format-source-api={formatPreview.sourceApi}
          data-dx-intl-format-time-zone={formatPreview.timeZone}
        >
          Formatter preview: {formatPreview.formatted}
        </p>
        <p
          className="text-xs text-muted-foreground"
          data-dx-intl-copy-target="price-preview"
          data-dx-intl-number-preview={numberPreview.formatted}
          data-dx-intl-number-source-api={numberPreview.sourceApi}
          data-dx-intl-number-currency={numberPreview.currency}
        >
          Localized plan price: {numberPreview.formatted}/mo
        </p>
      </div>

      <div className="grid gap-3 rounded-md border bg-muted p-3">
        <div
          className="flex flex-wrap gap-2"
          data-dx-intl-interaction="locale-switcher"
        >
          {localeOptions.map((locale) => (
            <button
              aria-pressed={selectedLocale === locale}
              className="rounded-md border bg-background px-3 py-2 text-sm text-foreground data-[selected=true]:bg-primary data-[selected=true]:text-primary-foreground"
              data-dx-intl-action="switch-dashboard-locale"
              data-dx-intl-locale-option={locale}
              data-dx-intl-locale-selected={selectedLocale === locale}
              data-selected={selectedLocale === locale}
              key={locale}
              onClick={() => switchLocale(locale)}
              type="button"
            >
              {dashboardCopy[locale].label}
            </button>
          ))}
        </div>

        <div
          className="rounded-md border bg-card p-3 text-sm"
          data-dx-intl-preview-locale={selectedLocale}
          data-dx-intl-provider-locale={providerLocale}
        >
          <strong aria-live="polite" data-dx-intl-copy-target="readiness">
            {translatedCopy.readiness}
          </strong>
          <p className="mt-2 text-muted-foreground">
            Provider locale: {providerLocale}. Selected dashboard copy lane:{" "}
            {selectedLocale}.
          </p>
          <dl className="mt-3 grid gap-2 text-xs">
            <div>
              <dt className="font-medium text-foreground">Plan copy</dt>
              <dd data-dx-intl-copy-target="plan-label">
                {translatedCopy.planLabel}
              </dd>
            </div>
            <div>
              <dt className="font-medium text-foreground">Support SLA</dt>
              <dd data-dx-intl-copy-target="support-sla">
                {translatedCopy.supportSla}
              </dd>
            </div>
            <div>
              <dt className="font-medium text-foreground">Route preview</dt>
              <dd data-dx-intl-copy-target="route-preview">
                <a
                  className="underline underline-offset-4"
                  data-dx-intl-action="preview-localized-route"
                  data-dx-intl-hreflang={routePreview.hrefLang}
                  data-dx-intl-locale-prefix={routePreview.localePrefix}
                  href={routePreview.href}
                  hrefLang={routePreview.hrefLang}
                >
                  {translatedCopy.routePreview}
                </a>
              </dd>
            </div>
          </dl>
          <div
            className="mt-3 rounded-md border bg-background p-2"
            data-dx-intl-alternate-links="launch-dashboard"
          >
            <span className="text-xs font-medium text-foreground">
              Alternate links
            </span>
            <div className="mt-2 flex flex-wrap gap-2">
              {alternateLinks.map((link) => (
                <a
                  className="rounded-md border px-2 py-1 text-xs text-foreground underline-offset-4 hover:underline"
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
          </div>
        </div>

        <button
          className="rounded-md border bg-background px-3 py-2 text-sm text-foreground"
          data-dx-intl-action="prepare-dashboard-intl-receipt"
          onClick={() =>
            setReceipt(
              createDxDashboardIntlReceipt({
                locale: selectedLocale,
                planPricePreview: pricePreview,
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
          className="rounded-md border bg-card p-3 text-xs text-muted-foreground"
          data-dx-intl-receipt-state="ready"
          data-dx-intl-receipt-locale={receipt.locale}
          data-dx-intl-receipt-route={receipt.routePreview}
          data-dx-intl-receipt-hreflang={receipt.routePreviewDetails.hrefLang}
          data-dx-intl-receipt-format-source={receipt.formatPreview.sourceApi}
          data-dx-intl-receipt-number-source={receipt.numberPreview.sourceApi}
          aria-live="polite"
        >
          {receipt.packageId} - {receipt.component} - {receipt.messageNamespace}
        </output>
      </div>
    </section>
  );
}
