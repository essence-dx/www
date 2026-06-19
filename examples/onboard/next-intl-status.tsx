import {
  IntlErrorCode,
  useFormatter,
  useLocale,
  useMessages,
  useNow,
  useTimeZone,
  useTranslations,
} from "next-intl";

import { Link, getPathname } from "@/i18n/navigation";

export type LaunchStatusPhase =
  | "alpha"
  | "beta"
  | "release-candidate"
  | "production";

export type LaunchStatusMessageProps = {
  phase?: LaunchStatusPhase;
  releaseAt?: Date;
  relativeDays?: number;
};

export function LaunchStatusMessage({
  phase = "beta",
  releaseAt = new Date("2026-05-23T09:00:00.000Z"),
  relativeDays = 1,
}: LaunchStatusMessageProps) {
  const t = useTranslations("Launch");
  const format = useFormatter();
  const locale = useLocale();

  return (
    <div
      className="space-y-1 text-sm text-muted-foreground"
      data-launch-i18n-locale={locale}
      data-launch-i18n-phase={phase}
    >
      <p>{t("phaseStatus", { phase })}</p>
      <p data-launch-i18n-window={releaseAt.toISOString()}>
        {format.dateTime(releaseAt, {
          dateStyle: "medium",
          timeStyle: "short",
        })}
      </p>
      <p>{format.relativeTime(relativeDays, "day")}</p>
    </div>
  );
}

export function LaunchLocaleLinks() {
  const locale = useLocale();

  return (
    <nav aria-label="Launch language" data-launch-i18n-locale-links={locale}>
      <Link
        aria-current={locale === "en" ? "page" : undefined}
        href="/"
        locale="en"
      >
        EN
      </Link>
      <Link
        aria-current={locale === "bn" ? "page" : undefined}
        href="/"
        locale="bn"
      >
        BN
      </Link>
      <span
        data-launch-i18n-locale-href={getPathname({
          href: "/",
          locale: "en",
        })}
      />
    </nav>
  );
}

export function LaunchIntlRuntimeSnapshot() {
  const messages = useMessages();
  const now = useNow();
  const timeZone = useTimeZone();

  return (
    <output
      data-launch-i18n-message-count={Object.keys(messages).length}
      data-launch-i18n-now={now.toISOString()}
      data-launch-i18n-time-zone={timeZone}
    >
      {Object.keys(messages).length} message namespace(s)
    </output>
  );
}

export function getLaunchMessageFallbackPreview({
  code,
  key,
  namespace,
}: {
  code: IntlErrorCode;
  key: string;
  namespace?: string;
}) {
  const scope = namespace ? `${namespace}.${key}` : key;

  if (code === IntlErrorCode.MISSING_MESSAGE) {
    return `[missing:${scope}]`;
  }

  if (code === IntlErrorCode.FORMATTING_ERROR) {
    return `[format-error:${scope}]`;
  }

  return `[intl-error:${scope}]`;
}

export function LaunchIntlErrorPolicy() {
  const preview = getLaunchMessageFallbackPreview({
    code: IntlErrorCode.MISSING_MESSAGE,
    key: "status",
    namespace: "Launch",
  });

  return (
    <output data-launch-i18n-error-policy={IntlErrorCode.MISSING_MESSAGE}>
      {preview}
    </output>
  );
}
