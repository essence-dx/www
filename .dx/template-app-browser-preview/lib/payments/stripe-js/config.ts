import type { StripeConstructorOptions } from "@stripe/stripe-js";

declare const process:
  | {
      env?: Record<string, string | undefined>;
    }
  | undefined;

export type DxStripeClientConfig = {
  publishableKey: string;
  account?: StripeConstructorOptions["stripeAccount"];
  apiVersion?: StripeConstructorOptions["apiVersion"];
  locale?: StripeConstructorOptions["locale"];
  advancedFraudSignals?: boolean;
};

const PUBLIC_STRIPE_SECRET_ENV_NAMES = [
  "NEXT_PUBLIC_STRIPE_SECRET_KEY",
  "NEXT_PUBLIC_STRIPE_WEBHOOK_SECRET",
  "NEXT_PUBLIC_STRIPE_RESTRICTED_KEY",
] as const;
const STRIPE_API_VERSION_PATTERN = /^\d{4}-\d{2}-\d{2}(\.[a-z][a-z0-9_-]*)?$/;

export function readDxStripeClientConfig(
  env: Record<string, string | undefined> = readDxStripeEnv(),
): DxStripeClientConfig {
  assertNoPublicStripeSecrets(env);
  const publishableKey = readStripeEnvString(
    env,
    "NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY",
  );

  if (!publishableKey) {
    throw new Error("Missing NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY.");
  }

  if (!publishableKey.startsWith("pk_") || publishableKey.startsWith("sk_")) {
    throw new Error("NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY must be a publishable key.");
  }

  return {
    publishableKey,
    account: readStripeAccountId(env),
    apiVersion:
      readStripeApiVersion(env, "NEXT_PUBLIC_STRIPE_API_VERSION") as
        | StripeConstructorOptions["apiVersion"]
        | undefined,
    locale:
      (readStripeEnvString(
        env,
        "NEXT_PUBLIC_STRIPE_LOCALE",
      ) as StripeConstructorOptions["locale"]) ??
      "auto",
    advancedFraudSignals: readStripeFraudSignals(env),
  };
}

export function readDxStripeEnv(): Record<string, string | undefined> {
  return typeof process !== "undefined" && process.env ? process.env : {};
}

export function assertNoPublicStripeSecrets(
  env: Record<string, string | undefined>,
) {
  const leakedName =
    PUBLIC_STRIPE_SECRET_ENV_NAMES.find((name) =>
      Boolean(readStripeEnvString(env, name)),
    ) ??
    Object.entries(env).find(
      ([name, value]) =>
        name.startsWith("NEXT_PUBLIC_") &&
        isStripeSecretValue(readStripeEnvValue(value)),
    )?.[0];

  if (leakedName) {
    throw new Error(`${leakedName} must not be exposed to browser code.`);
  }
}

function readStripeApiVersion(
  env: Record<string, string | undefined>,
  name: string,
): string | undefined {
  const version = readStripeEnvString(env, name);

  if (!version) {
    return undefined;
  }

  if (!STRIPE_API_VERSION_PATTERN.test(version)) {
    throw new Error(
      `${name} must be a Stripe API version like 2026-02-25.clover.`,
    );
  }

  return version;
}

function readStripeAccountId(
  env: Record<string, string | undefined>,
): StripeConstructorOptions["stripeAccount"] | undefined {
  const account = readStripeEnvString(env, "NEXT_PUBLIC_STRIPE_ACCOUNT");

  if (!account) {
    return undefined;
  }

  if (!account.startsWith("acct_")) {
    throw new Error("NEXT_PUBLIC_STRIPE_ACCOUNT must be a Stripe account ID.");
  }

  return account;
}

function readStripeFraudSignals(
  env: Record<string, string | undefined>,
): boolean | undefined {
  const value = readStripeEnvString(
    env,
    "NEXT_PUBLIC_STRIPE_ADVANCED_FRAUD_SIGNALS",
  );

  if (!value) {
    return undefined;
  }

  if (value === "true") {
    return true;
  }

  if (value === "false") {
    return false;
  }

  throw new Error(
    "NEXT_PUBLIC_STRIPE_ADVANCED_FRAUD_SIGNALS must be true or false.",
  );
}

function readStripeEnvString(
  env: Record<string, string | undefined>,
  name: string,
): string | undefined {
  return readStripeEnvValue(env[name]);
}

function readStripeEnvValue(value: string | undefined): string | undefined {
  const trimmed = value?.trim();
  return trimmed ? trimmed : undefined;
}

function isStripeSecretValue(value: string | undefined) {
  return Boolean(
    value?.startsWith("sk_") ||
      value?.startsWith("rk_") ||
      value?.startsWith("whsec_"),
  );
}
