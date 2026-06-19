pub(super) const STRIPE_JS_VERSION: &str = "9.6.0-dx.1";

pub(super) fn stripe_js_templates() -> Vec<(&'static str, &'static str)> {
    vec![
        ("js/payments/stripe-js/config.ts", STRIPE_CONFIG_TS),
        ("js/payments/stripe-js/client.ts", STRIPE_CLIENT_TS),
        ("js/payments/stripe-js/payment.ts", STRIPE_PAYMENT_TS),
        ("js/payments/stripe-js/checkout.ts", STRIPE_CHECKOUT_TS),
        (
            "js/payments/stripe-js/dashboard-checkout.ts",
            STRIPE_DASHBOARD_CHECKOUT_TS,
        ),
        ("js/payments/stripe-js/server.ts", STRIPE_SERVER_TS),
        ("js/app/api/checkout/route.ts", STRIPE_ROUTE_TS),
        (
            "js/app/api/stripe/webhook/route.ts",
            STRIPE_WEBHOOK_ROUTE_TS,
        ),
        ("js/payments/stripe-js/metadata.ts", STRIPE_METADATA_TS),
        ("js/payments/stripe-js/README.md", STRIPE_README_MD),
    ]
}

const STRIPE_CONFIG_TS: &str = r#"import type { StripeConstructorOptions } from "@stripe/stripe-js";

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
"#;

const STRIPE_CLIENT_TS: &str = r#"import { loadStripe } from "@stripe/stripe-js/pure";
import type {
  Stripe,
  StripeConstructorOptions,
} from "@stripe/stripe-js";

import {
  readDxStripeClientConfig,
  type DxStripeClientConfig,
} from "./config";

let stripePromise: Promise<Stripe | null> | undefined;
let configuredFraudSignals: boolean | undefined;

export function configureDxStripeLoadParameters(
  config: Pick<DxStripeClientConfig, "advancedFraudSignals">,
) {
  if (config.advancedFraudSignals === undefined) return;
  if (configuredFraudSignals === config.advancedFraudSignals) return;

  loadStripe.setLoadParameters({
    advancedFraudSignals: config.advancedFraudSignals,
  });
  configuredFraudSignals = config.advancedFraudSignals;
}

export function getDxStripeOptions(
  config: DxStripeClientConfig,
): StripeConstructorOptions {
  return {
    stripeAccount: config.account,
    apiVersion: config.apiVersion,
    locale: config.locale,
  };
}

export function getDxStripe(
  config: DxStripeClientConfig = readDxStripeClientConfig(),
) {
  configureDxStripeLoadParameters(config);
  stripePromise ??= loadStripe(config.publishableKey, getDxStripeOptions(config));
  return stripePromise;
}

export async function requireDxStripe(
  config?: DxStripeClientConfig,
): Promise<Stripe> {
  const stripe = await getDxStripe(config);

  if (!stripe) {
    throw new Error("Stripe.js is not available in this environment.");
  }

  return stripe;
}
"#;

const STRIPE_PAYMENT_TS: &str = r#"import type {
  PaymentIntentResult,
  Stripe,
  StripeElements,
  StripeElementsOptionsClientSecret,
} from "@stripe/stripe-js";

import { requireDxStripe } from "./client";
import type { DxStripeClientConfig } from "./config";

export type DxStripePaymentIntent = {
  clientSecret: string;
  returnUrl: string;
};

export type RetrieveDxStripePaymentIntentInput = {
  clientSecret: string;
  config?: DxStripeClientConfig;
  stripe?: Stripe;
};

export type DxStripeElementsOptions = {
  clientSecret: string;
} & Omit<StripeElementsOptionsClientSecret, "clientSecret">;

export function createDxStripeElementsOptions({
  clientSecret,
  ...options
}: DxStripeElementsOptions): StripeElementsOptionsClientSecret {
  const normalizedClientSecret = readDxStripeClientSecret(clientSecret);

  return {
    appearance: {
      theme: "stripe",
      labels: "above",
      ...options.appearance,
    },
    ...options,
    clientSecret: normalizedClientSecret,
  };
}

export async function retrieveDxStripePaymentIntent({
  clientSecret,
  config,
  stripe,
}: RetrieveDxStripePaymentIntentInput): Promise<PaymentIntentResult> {
  const stripeClient = stripe ?? (await requireDxStripe(config));
  const normalizedClientSecret = readDxStripeClientSecret(clientSecret);

  return stripeClient.retrievePaymentIntent(normalizedClientSecret);
}

export async function confirmDxStripePayment({
  config,
  elements,
  intent,
  stripe,
}: {
  config?: DxStripeClientConfig;
  elements: StripeElements;
  intent: DxStripePaymentIntent;
  stripe?: Stripe;
}): Promise<PaymentIntentResult> {
  const stripeClient = stripe ?? (await requireDxStripe(config));
  const returnUrl = readDxStripePaymentReturnUrl(intent.returnUrl);

  readDxStripeClientSecret(intent.clientSecret);

  const submitResult = await elements.submit();

  if (submitResult.error) {
    return {
      error: submitResult.error,
    };
  }

  return stripeClient.confirmPayment({
    elements,
    confirmParams: {
      return_url: returnUrl,
    },
    redirect: "if_required",
  });
}

function readDxStripeClientSecret(value: string) {
  const clientSecret = value.trim();

  if (!clientSecret) {
    throw new Error("Stripe PaymentIntent client secret is required.");
  }

  if (!clientSecret.startsWith("pi_") || !clientSecret.includes("_secret_")) {
    throw new Error(
      "Stripe PaymentIntent client secret must come from your server-created PaymentIntent.",
    );
  }

  return clientSecret;
}

function readDxStripePaymentReturnUrl(value: string) {
  const trimmed = value.trim();

  if (!trimmed) {
    throw new Error("Stripe payment returnUrl is required.");
  }

  let url: URL;
  try {
    url = new URL(trimmed);
  } catch {
    throw new Error("Stripe payment returnUrl must be an absolute URL.");
  }

  if (
    url.protocol === "https:" ||
    (url.protocol === "http:" && isLocalStripeReturnHost(url.hostname))
  ) {
    return url.toString();
  }

  throw new Error("Stripe payment returnUrl must use HTTPS outside localhost.");
}

function isLocalStripeReturnHost(hostname: string) {
  return hostname === "localhost" || hostname === "127.0.0.1" || hostname === "[::1]";
}
"#;

const STRIPE_CHECKOUT_TS: &str = r#"import type {
  Stripe,
  StripeEmbeddedCheckout,
  StripeEmbeddedCheckoutOptions,
} from "@stripe/stripe-js";

import { requireDxStripe } from "./client";
import type { DxStripeClientConfig } from "./config";

export type DxStripeCheckoutContact = {
  email: string;
  name: string;
  organization?: string;
  message?: string;
};

export type DxStripeCheckoutContactIssue = {
  field: keyof DxStripeCheckoutContact;
  message: string;
};

export type DxStripeCheckoutContactValidationResult =
  | {
      success: true;
      data: DxStripeCheckoutContact;
    }
  | {
      success: false;
      issues: DxStripeCheckoutContactIssue[];
    };

export type DxStripeCheckoutMode = "hosted" | "embedded";

export type DxStripeCheckoutPlanSelection = {
  id: string;
  priceEnv: string;
};

export type DxStripeCheckoutSubmitResponse =
  | {
      kind: "checkout-session";
      sessionId: string;
      url: string;
      message?: string;
    }
  | {
      kind: "embedded-checkout-session";
      sessionId: string;
      clientSecret: string;
      message?: string;
    }
  | {
      kind: "contact";
      message: string;
    };

export type DxStripeCheckoutSubmitState =
  | {
      kind: "idle";
      message: string;
    }
  | {
      kind: "submitting";
      message: string;
    }
  | {
      kind: "success";
      message: string;
      checkoutUrl?: string;
    }
  | {
      kind: "error";
      message: string;
    };

export type SubmitDxStripeCheckoutContactOptions = {
  endpoint?: string;
  contact: Partial<Record<keyof DxStripeCheckoutContact, unknown>>;
  checkoutMode?: DxStripeCheckoutMode;
  plan?: DxStripeCheckoutPlanSelection;
  source?: "dx-www-launch" | "dx-www-dashboard";
  fetcher?: typeof fetch;
  signal?: AbortSignal;
};

export type CreateDxStripeEmbeddedCheckoutClientSecretFetcherOptions = Omit<
  SubmitDxStripeCheckoutContactOptions,
  "checkoutMode"
>;

export type DxStripeEmbeddedCheckoutClientSecretSource =
  | {
      clientSecret: string;
      fetchClientSecret?: never;
    }
  | {
      clientSecret?: never;
      fetchClientSecret: () => Promise<string>;
    };

export type CreateDxStripeEmbeddedCheckoutInput = Omit<
  StripeEmbeddedCheckoutOptions,
  "clientSecret" | "fetchClientSecret"
> &
  DxStripeEmbeddedCheckoutClientSecretSource & {
    config?: DxStripeClientConfig;
    stripe?: Stripe;
  };

const EMAIL_PATTERN = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;

export function validateDxStripeCheckoutContact(
  input: Partial<Record<keyof DxStripeCheckoutContact, unknown>>,
): DxStripeCheckoutContactValidationResult {
  const email = readContactString(input, "email").toLowerCase();
  const name = readContactString(input, "name");
  const organization = readContactString(input, "organization");
  const message = readContactString(input, "message");
  const issues: DxStripeCheckoutContactIssue[] = [];

  if (!EMAIL_PATTERN.test(email)) {
    issues.push({
      field: "email",
      message: "Enter a valid checkout email.",
    });
  }

  if (name.length < 2) {
    issues.push({
      field: "name",
      message: "Enter the checkout contact name.",
    });
  }

  if (name.length > 80) {
    issues.push({
      field: "name",
      message: "Checkout contact name must be 80 characters or fewer.",
    });
  }

  if (organization.length > 120) {
    issues.push({
      field: "organization",
      message: "Organization must be 120 characters or fewer.",
    });
  }

  if (message.length > 500) {
    issues.push({
      field: "message",
      message: "Checkout notes must be 500 characters or fewer.",
    });
  }

  if (issues.length > 0) {
    return {
      success: false,
      issues,
    };
  }

  return {
    success: true,
    data: {
      email,
      name,
      organization: organization || undefined,
      message: message || undefined,
    },
  };
}

export function createDxStripeCheckoutContactPayload(
  input: Partial<Record<keyof DxStripeCheckoutContact, unknown>>,
): DxStripeCheckoutContact {
  const result = validateDxStripeCheckoutContact(input);

  if (!result.success) {
    throw new Error(
      result.issues.map((issue) => issue.message).join(" "),
    );
  }

  return result.data;
}

export async function submitDxStripeCheckoutContact({
  endpoint = "/api/checkout",
  contact,
  checkoutMode,
  plan,
  source = "dx-www-launch",
  fetcher = fetch,
  signal,
}: SubmitDxStripeCheckoutContactOptions): Promise<DxStripeCheckoutSubmitResponse> {
  const payload = createDxStripeCheckoutContactPayload(contact);
  const planSelection = readDxStripeCheckoutPlanSelection(plan);
  const checkoutEndpoint = readDxStripeCheckoutEndpoint(endpoint);
  const response = await fetcher(checkoutEndpoint, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      contact: payload,
      checkoutMode: checkoutMode ?? "hosted",
      source,
      plan: planSelection,
    }),
    signal,
  });
  const body = await readJsonResponse(response);

  if (!response.ok) {
    throw new Error(
      readResponseMessage(body) ?? `Checkout request failed with ${response.status}.`,
    );
  }

  return parseDxStripeCheckoutResponse(body);
}

export function createDxStripeEmbeddedCheckoutClientSecretFetcher({
  endpoint = "/api/checkout",
  contact,
  plan,
  source = "dx-www-launch",
  fetcher = fetch,
  signal,
}: CreateDxStripeEmbeddedCheckoutClientSecretFetcherOptions): () => Promise<string> {
  const payload = createDxStripeCheckoutContactPayload(contact);
  const planSelection = readDxStripeCheckoutPlanSelection(plan);
  const checkoutEndpoint = readDxStripeCheckoutEndpoint(endpoint);

  return async () => {
    const response = await fetcher(checkoutEndpoint, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        contact: payload,
        checkoutMode: "embedded",
        source,
        plan: planSelection,
      }),
      signal,
    });
    const body = await readJsonResponse(response);

    if (!response.ok) {
      throw new Error(
        readResponseMessage(body) ??
          `Embedded Checkout request failed with ${response.status}.`,
      );
    }

    const checkoutResponse = parseDxStripeCheckoutResponse(body);

    if (checkoutResponse.kind !== "embedded-checkout-session") {
      throw new Error(
        "Embedded Checkout endpoint must return an embedded Checkout Session.",
      );
    }

    return readDxStripeCheckoutClientSecret(checkoutResponse.clientSecret);
  };
}

export async function createDxStripeEmbeddedCheckout({
  clientSecret,
  fetchClientSecret,
  config,
  stripe,
  ...options
}: CreateDxStripeEmbeddedCheckoutInput): Promise<StripeEmbeddedCheckout> {
  const stripeClient = stripe ?? (await requireDxStripe(config));
  const source = readDxStripeEmbeddedCheckoutClientSecretSource({
    clientSecret,
    fetchClientSecret,
  });

  return stripeClient.createEmbeddedCheckoutPage({
    ...options,
    ...source,
  });
}

function readDxStripeEmbeddedCheckoutClientSecretSource({
  clientSecret,
  fetchClientSecret,
}: {
  clientSecret?: string;
  fetchClientSecret?: () => Promise<string>;
}): Pick<StripeEmbeddedCheckoutOptions, "clientSecret" | "fetchClientSecret"> {
  const hasClientSecret = clientSecret !== undefined;
  const hasFetchClientSecret = fetchClientSecret !== undefined;

  if (hasClientSecret && hasFetchClientSecret) {
    throw new Error(
      "Stripe Embedded Checkout accepts either clientSecret or fetchClientSecret, not both.",
    );
  }

  if (hasClientSecret) {
    return {
      clientSecret: readDxStripeCheckoutClientSecret(clientSecret),
    };
  }

  if (hasFetchClientSecret) {
    return {
      fetchClientSecret: async () =>
        readDxStripeCheckoutClientSecret(await fetchClientSecret()),
    };
  }

  throw new Error(
    "Stripe Embedded Checkout requires either clientSecret or fetchClientSecret.",
  );
}

function readDxStripeCheckoutClientSecret(value: unknown) {
  if (typeof value !== "string") {
    throw new Error("Stripe Embedded Checkout client secret must be a string.");
  }

  const clientSecret = value.trim();

  if (!clientSecret) {
    throw new Error("Stripe Embedded Checkout client secret is required.");
  }

  if (!clientSecret.startsWith("cs_") || !clientSecret.includes("_secret_")) {
    throw new Error(
      "Stripe Embedded Checkout client secret must come from your server-created Checkout Session.",
    );
  }

  return clientSecret;
}

function readDxStripeCheckoutEndpoint(endpoint: string) {
  const trimmed = endpoint.trim();

  if (!trimmed) {
    throw new Error("Checkout endpoint must be a same-origin path or HTTPS URL.");
  }

  if (trimmed.startsWith("/") && !trimmed.startsWith("//")) {
    return trimmed;
  }

  let url: URL;
  try {
    url = new URL(trimmed);
  } catch {
    throw new Error("Checkout endpoint must be a same-origin path or HTTPS URL.");
  }

  if (url.protocol === "https:") {
    return url.toString();
  }

  throw new Error("Checkout endpoint must be a same-origin path or HTTPS URL.");
}

function readDxStripeCheckoutPlanSelection(
  value: DxStripeCheckoutPlanSelection | undefined,
) {
  if (value === undefined) {
    return undefined;
  }

  const id = readDxStripeCheckoutPlanString(value.id, "checkout plan id");
  const priceEnv = readDxStripeCheckoutPlanString(
    value.priceEnv,
    "checkout plan price env",
  );

  if (!/^[a-z0-9-]+$/.test(id)) {
    throw new Error("Checkout plan id must be a lowercase app-owned plan id.");
  }

  if (!/^STRIPE_PRICE_ID(_[A-Z0-9]+)*$/.test(priceEnv)) {
    throw new Error(
      "Checkout plan price env must be a STRIPE_PRICE_ID env key.",
    );
  }

  return {
    id,
    priceEnv,
  };
}

function readDxStripeCheckoutPlanString(value: unknown, label: string) {
  if (typeof value !== "string") {
    throw new Error(`Stripe ${label} must be a string.`);
  }

  const trimmed = value.trim();

  if (!trimmed) {
    throw new Error(`Stripe ${label} is required.`);
  }

  return trimmed;
}

function parseDxStripeCheckoutResponse(
  value: unknown,
): DxStripeCheckoutSubmitResponse {
  if (!isRecord(value)) {
    throw new Error("Checkout response must be a JSON object.");
  }

  if (value.kind === "checkout-session") {
    const sessionId = readOptionalString(value, "sessionId");
    const url = readCheckoutSessionUrl(value);

    if (!sessionId || !url) {
      throw new Error("Checkout Session response requires sessionId and HTTPS url.");
    }

    return {
      kind: "checkout-session",
      sessionId,
      url,
      message: readOptionalString(value, "message"),
    };
  }

  if (value.kind === "embedded-checkout-session") {
    const sessionId = readOptionalString(value, "sessionId");
    const clientSecret = readCheckoutSessionClientSecret(value);

    if (!sessionId || !clientSecret) {
      throw new Error(
        "Checkout Session response requires sessionId and clientSecret.",
      );
    }

    return {
      kind: "embedded-checkout-session",
      sessionId,
      clientSecret,
      message: readOptionalString(value, "message"),
    };
  }

  if (value.kind === "contact") {
    const message = readOptionalString(value, "message");

    if (!message) {
      throw new Error("Checkout contact response requires a message.");
    }

    return {
      kind: "contact",
      message,
    };
  }

  throw new Error("Checkout response is missing a supported Stripe result.");
}

async function readJsonResponse(response: Response): Promise<unknown> {
  const text = await response.text();

  if (!text.trim()) {
    return {};
  }

  try {
    return JSON.parse(text) as unknown;
  } catch {
    throw new Error("Checkout response must be valid JSON.");
  }
}

function readResponseMessage(value: unknown): string | undefined {
  if (!isRecord(value)) {
    return undefined;
  }

  return readOptionalString(value, "message") ?? readOptionalString(value, "error");
}

function readContactString(
  input: Partial<Record<keyof DxStripeCheckoutContact, unknown>>,
  key: keyof DxStripeCheckoutContact,
): string {
  const value = input[key];
  return typeof value === "string" ? value.trim() : "";
}

function readOptionalString(
  input: Record<string, unknown>,
  key: string,
): string | undefined {
  const value = input[key];
  return typeof value === "string" && value.trim() ? value.trim() : undefined;
}

function readCheckoutSessionUrl(input: Record<string, unknown>): string | undefined {
  const value = readOptionalString(input, "url");

  if (!value) {
    return undefined;
  }

  let url: URL;
  try {
    url = new URL(value);
  } catch {
    throw new Error("Checkout Session url must be an absolute HTTPS URL.");
  }

  if (url.protocol !== "https:") {
    throw new Error("Checkout Session url must use HTTPS.");
  }

  return url.toString();
}

function readCheckoutSessionClientSecret(
  input: Record<string, unknown>,
): string | undefined {
  const clientSecret = readOptionalString(input, "clientSecret");

  if (!clientSecret) {
    return undefined;
  }

  return readDxStripeCheckoutClientSecret(clientSecret);
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return Boolean(value) && typeof value === "object" && !Array.isArray(value);
}
"#;

const STRIPE_DASHBOARD_CHECKOUT_TS: &str = r#"import {
  createDxStripeCheckoutContactPayload,
  type DxStripeCheckoutContact,
  type DxStripeCheckoutMode,
} from "./checkout";

export type DxStripeDashboardPlan = {
  id: "starter" | "team" | "scale";
  label: string;
  priceLabel: string;
  description: string;
  priceEnv: string;
};

export type CreateDxStripeDashboardCheckoutRequestInput = {
  planId: DxStripeDashboardPlan["id"];
  checkoutMode: DxStripeCheckoutMode;
  contact: Partial<Record<keyof DxStripeCheckoutContact, unknown>>;
};

export type DxStripeDashboardCheckoutRequest = {
  endpoint: "/api/checkout";
  method: "POST";
  checkoutMode: DxStripeCheckoutMode;
  contact: DxStripeCheckoutContact;
  plan: DxStripeDashboardPlan;
  body: {
    checkoutMode: DxStripeCheckoutMode;
    contact: DxStripeCheckoutContact;
    source: "dx-www-dashboard";
    plan: {
      id: DxStripeDashboardPlan["id"];
      priceEnv: string;
    };
  };
};

export type DxStripeDashboardCheckoutReceipt = {
  receiptId: string;
  status: "missing-config";
  message: string;
  requiredEnv: readonly string[];
  request: DxStripeDashboardCheckoutRequest;
};

export const dxStripeDashboardPlans: readonly DxStripeDashboardPlan[] = [
  {
    id: "starter",
    label: "Starter",
    priceLabel: "$29/mo",
    description: "For a small DX-WWW launch dashboard with one app-owned product price.",
    priceEnv: "STRIPE_PRICE_ID_STARTER",
  },
  {
    id: "team",
    label: "Team",
    priceLabel: "$99/mo",
    description: "For teams that need hosted Checkout plus Billing Portal readiness.",
    priceEnv: "STRIPE_PRICE_ID_TEAM",
  },
  {
    id: "scale",
    label: "Scale",
    priceLabel: "Custom",
    description: "For embedded Checkout and app-owned subscription entitlement review.",
    priceEnv: "STRIPE_PRICE_ID_SCALE",
  },
];

export const dxStripeDashboardCheckoutReadiness = {
  packageId: "payments/stripe-js",
  sourceMirror: "G:/WWW/inspirations/stripe-js",
  endpoint: "/api/checkout",
  status: "missing-config",
  requiredEnv: [
    "NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY",
    "STRIPE_SECRET_KEY",
    "STRIPE_PRICE_ID",
    "STRIPE_PRICE_ID_STARTER",
    "STRIPE_PRICE_ID_TEAM",
    "STRIPE_PRICE_ID_SCALE",
  ],
  publicApi: [
    "loadStripe",
    "submitDxStripeCheckoutContact",
    "createDxStripeEmbeddedCheckoutClientSecretFetcher",
    "StripeEmbeddedCheckoutOptions.fetchClientSecret",
    "stripe.confirmPayment",
    "stripe.retrievePaymentIntent",
  ],
} as const;

export function createDxStripeDashboardCheckoutRequest({
  planId,
  checkoutMode,
  contact,
}: CreateDxStripeDashboardCheckoutRequestInput): DxStripeDashboardCheckoutRequest {
  const plan = readDxStripeDashboardPlan(planId);
  const checkoutContact = createDxStripeCheckoutContactPayload(contact);

  return {
    endpoint: "/api/checkout",
    method: "POST",
    checkoutMode,
    contact: checkoutContact,
    plan,
    body: {
      checkoutMode,
      contact: checkoutContact,
      source: "dx-www-dashboard",
      plan: {
        id: plan.id,
        priceEnv: plan.priceEnv,
      },
    },
  };
}

export function createDxStripeDashboardMissingConfigReceipt(
  request: DxStripeDashboardCheckoutRequest,
): DxStripeDashboardCheckoutReceipt {
  return {
    receiptId: `stripe-dashboard-local-${request.plan.id}-${request.checkoutMode}-${slugifyStripeDashboardEmail(request.contact.email)}`,
    status: "missing-config",
    message:
      "Checkout request is ready, but Stripe credentials and a product Price ID must be app-owned before creating a Checkout Session.",
    requiredEnv: dxStripeDashboardCheckoutReadiness.requiredEnv,
    request,
  };
}

function readDxStripeDashboardPlan(planId: DxStripeDashboardPlan["id"]) {
  const plan = dxStripeDashboardPlans.find((item) => item.id === planId);

  if (!plan) {
    throw new Error("Dashboard checkout plan must be one of the app-owned Stripe plans.");
  }

  return plan;
}

function slugifyStripeDashboardEmail(email: string) {
  return email.replace(/[^a-z0-9]+/g, "-").replace(/^-|-$/g, "") || "contact";
}
"#;

const STRIPE_SERVER_TS: &str = r#"import "server-only";

import Stripe from "stripe";

import {
  createDxStripeCheckoutContactPayload,
  type DxStripeCheckoutContact,
} from "./checkout";
import { assertNoPublicStripeSecrets } from "./config";

declare const process:
  | {
      env?: Record<string, string | undefined>;
    }
  | undefined;

export const DX_STRIPE_API_VERSION = "2026-02-25.clover";
const STRIPE_API_VERSION_PATTERN = /^\d{4}-\d{2}-\d{2}(\.[a-z][a-z0-9_-]*)?$/;
const DX_STRIPE_EMAIL_PATTERN = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;

export type DxStripeServerConfig = {
  secretKey: string;
  webhookSecret?: string;
  apiVersion: string;
  appUrl?: string;
};

export type CreateDxStripeCustomerInput = {
  email?: string;
  name?: string;
  metadata?: Record<string, string | undefined>;
};

export type CreateDxStripeCheckoutSessionInput = {
  contact: DxStripeCheckoutContact;
  lineItems: Stripe.Checkout.SessionCreateParams.LineItem[];
  successUrl: string;
  cancelUrl: string;
  mode?: Stripe.Checkout.SessionCreateParams.Mode;
  customerId?: string;
  customerUpdate?: Stripe.Checkout.SessionCreateParams["customer_update"];
  allowPromotionCodes?: boolean;
  clientReferenceId?: string;
  metadata?: Record<string, string | undefined>;
};

export type CreateDxStripeEmbeddedCheckoutSessionInput = Omit<
  CreateDxStripeCheckoutSessionInput,
  "successUrl" | "cancelUrl"
> & {
  returnUrl: string;
  redirectOnCompletion?: Stripe.Checkout.SessionCreateParams["redirect_on_completion"];
};

export type CreateDxStripeBillingPortalSessionInput = {
  customerId: string;
  returnUrl: string;
  configuration?: string;
  flowData?: Stripe.BillingPortal.SessionCreateParams["flow_data"];
  locale?: Stripe.BillingPortal.SessionCreateParams["locale"];
};

export type ListDxStripeCustomerSubscriptionsInput = {
  customerId: string;
  status?: Stripe.SubscriptionListParams["status"];
  limit?: number;
  expand?: Stripe.SubscriptionListParams["expand"];
};

export type RetrieveDxStripeCheckoutSessionInput = {
  sessionId: string;
  expand?: Stripe.Checkout.SessionRetrieveParams["expand"];
};

export type CreateDxStripePaymentIntentInput = {
  amount: number;
  currency: string;
  receiptEmail?: string;
  automaticPaymentMethods?: Stripe.PaymentIntentCreateParams.AutomaticPaymentMethods;
  metadata?: Record<string, string | undefined>;
};

export type VerifyDxStripeWebhookEventInput = {
  payload: string;
  signature: string | null | undefined;
  config?: DxStripeServerConfig;
};

export function readDxStripeServerConfig(
  env: Record<string, string | undefined> = readDxStripeServerEnv(),
): DxStripeServerConfig {
  assertNoPublicStripeSecrets(env);
  const secretKey = readDxStripeServerEnvString(env, "STRIPE_SECRET_KEY");

  if (!secretKey) {
    throw new Error("Missing STRIPE_SECRET_KEY.");
  }

  if (!secretKey.startsWith("sk_")) {
    throw new Error("STRIPE_SECRET_KEY must be a Stripe secret key.");
  }

  return {
    secretKey,
    webhookSecret: readDxStripeWebhookSecret(env),
    apiVersion:
      readDxStripeApiVersion(env, "STRIPE_API_VERSION") ??
      DX_STRIPE_API_VERSION,
    appUrl: readDxStripeAppUrl(env),
  };
}

export function readDxStripeServerEnv(): Record<string, string | undefined> {
  return typeof process !== "undefined" && process.env ? process.env : {};
}

function readDxStripeServerEnvString(
  env: Record<string, string | undefined>,
  name: string,
): string | undefined {
  const value = env[name]?.trim();
  return value ? value : undefined;
}

function readDxStripeWebhookSecret(
  env: Record<string, string | undefined>,
): string | undefined {
  const webhookSecret = readDxStripeServerEnvString(env, "STRIPE_WEBHOOK_SECRET");

  if (!webhookSecret) {
    return undefined;
  }

  if (!webhookSecret.startsWith("whsec_")) {
    throw new Error(
      "STRIPE_WEBHOOK_SECRET must be a Stripe webhook signing secret.",
    );
  }

  return webhookSecret;
}

function readDxStripeApiVersion(
  env: Record<string, string | undefined>,
  name: string,
): string | undefined {
  const version = readDxStripeServerEnvString(env, name);

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

function readDxStripeAppUrl(env: Record<string, string | undefined>) {
  return normalizeStripeAppUrl(
    readDxStripeServerEnvString(env, "NEXT_PUBLIC_APP_URL") ??
      readDxStripeServerEnvString(env, "APP_URL") ??
      readDxStripeServerEnvString(env, "VERCEL_URL"),
  );
}

export function createDxStripeServerClient(
  config: DxStripeServerConfig = readDxStripeServerConfig(),
) {
  return new Stripe(config.secretKey, {
    apiVersion: config.apiVersion as Stripe.StripeConfig["apiVersion"],
  });
}

export async function createDxStripeCustomer(
  input: CreateDxStripeCustomerInput,
  stripe = createDxStripeServerClient(),
) {
  const email = readDxStripeCustomerEmail(input.email);
  const name = readDxStripeCustomerName(input.name);
  const metadata = stripEmptyMetadata(input.metadata);

  if (!email && !name && Object.keys(metadata).length === 0) {
    throw new Error(
      "Stripe customer creation requires email, name, or metadata.",
    );
  }

  return stripe.customers.create({
    email,
    name,
    metadata,
  });
}

export async function retrieveDxStripeCheckoutSession(
  input: RetrieveDxStripeCheckoutSessionInput,
  stripe = createDxStripeServerClient(),
) {
  return stripe.checkout.sessions.retrieve(
    readDxStripeCheckoutSessionId(input.sessionId),
    {
      expand: input.expand,
    },
  );
}

export async function listDxStripeCustomerSubscriptions(
  input: ListDxStripeCustomerSubscriptionsInput,
  stripe = createDxStripeServerClient(),
) {
  return stripe.subscriptions.list({
    customer: readDxStripeCustomerId(input.customerId),
    status: input.status,
    limit: readDxStripeListLimit(input.limit, "subscription limit"),
    expand: input.expand,
  });
}

export async function createDxStripePaymentIntent(
  input: CreateDxStripePaymentIntentInput,
  stripe = createDxStripeServerClient(),
) {
  const paymentIntent = await stripe.paymentIntents.create({
    amount: readDxStripePaymentIntentAmount(input.amount),
    currency: readDxStripePaymentIntentCurrency(input.currency),
    automatic_payment_methods: input.automaticPaymentMethods ?? {
      enabled: true,
    },
    receipt_email: readDxStripePaymentIntentReceiptEmail(input.receiptEmail),
    metadata: stripEmptyMetadata(input.metadata),
  });

  if (!paymentIntent.client_secret) {
    throw new Error("Stripe PaymentIntent did not include a client secret.");
  }

  return paymentIntent;
}

function readDxStripeCheckoutSessionId(value: string) {
  const sessionId = value.trim();

  if (!sessionId) {
    throw new Error("Stripe Checkout Session ID is required.");
  }

  if (!sessionId.startsWith("cs_")) {
    throw new Error("Stripe Checkout Session ID must start with cs_.");
  }

  return sessionId;
}

export function verifyDxStripeWebhookEvent(
  input: VerifyDxStripeWebhookEventInput,
  stripe?: Stripe,
): Stripe.Event {
  const config = input.config ?? readDxStripeServerConfig();
  const stripeClient = stripe ?? createDxStripeServerClient(config);

  return stripeClient.webhooks.constructEvent(
    readStripeWebhookPayload(input.payload),
    readStripeWebhookSignature(input.signature),
    readConfiguredWebhookSecret(config),
  );
}

export async function verifyDxStripeWebhookRequest(
  request: Request,
  config: DxStripeServerConfig = readDxStripeServerConfig(),
  stripe = createDxStripeServerClient(config),
): Promise<Stripe.Event> {
  return verifyDxStripeWebhookEvent(
    {
      payload: await request.text(),
      signature: request.headers.get("stripe-signature"),
      config,
    },
    stripe,
  );
}

export async function createDxStripeCheckoutSession(
  input: CreateDxStripeCheckoutSessionInput,
  stripe = createDxStripeServerClient(),
) {
  const contact = createDxStripeCheckoutContactPayload(input.contact);
  const lineItems = readDxStripeCheckoutLineItems(input.lineItems);

  const successUrl = readStripeCheckoutRedirectUrl(input.successUrl, "successUrl");
  const cancelUrl = readStripeCheckoutRedirectUrl(input.cancelUrl, "cancelUrl");

  return stripe.checkout.sessions.create({
    mode: input.mode ?? "payment",
    ...toDxStripeCheckoutCustomerParams(contact, input),
    line_items: lineItems,
    allow_promotion_codes: input.allowPromotionCodes,
    success_url: successUrl,
    cancel_url: cancelUrl,
    client_reference_id: input.clientReferenceId,
    metadata: toDxStripeCheckoutMetadata(contact, input.metadata),
  });
}

export async function createDxStripeEmbeddedCheckoutSession(
  input: CreateDxStripeEmbeddedCheckoutSessionInput,
  stripe = createDxStripeServerClient(),
) {
  const contact = createDxStripeCheckoutContactPayload(input.contact);
  const lineItems = readDxStripeCheckoutLineItems(input.lineItems);
  const returnUrl = readStripeCheckoutRedirectUrl(input.returnUrl, "returnUrl");

  const session = await stripe.checkout.sessions.create({
    mode: input.mode ?? "payment",
    ...toDxStripeCheckoutCustomerParams(contact, input),
    line_items: lineItems,
    allow_promotion_codes: input.allowPromotionCodes,
    ui_mode: "embedded",
    return_url: returnUrl,
    redirect_on_completion: input.redirectOnCompletion ?? "if_required",
    client_reference_id: input.clientReferenceId,
    metadata: toDxStripeCheckoutMetadata(contact, input.metadata),
  });

  if (!session.client_secret) {
    throw new Error(
      "Stripe Embedded Checkout Session did not include a client secret.",
    );
  }

  return session;
}

export async function createDxStripeBillingPortalSession(
  input: CreateDxStripeBillingPortalSessionInput,
  stripe = createDxStripeServerClient(),
) {
  return stripe.billingPortal.sessions.create({
    customer: readDxStripeCustomerId(input.customerId),
    return_url: readStripeCheckoutRedirectUrl(input.returnUrl, "returnUrl"),
    configuration: readDxStripeBillingPortalConfiguration(input.configuration),
    flow_data: input.flowData,
    locale: input.locale,
  });
}

function readDxStripePaymentIntentAmount(amount: number) {
  if (!Number.isInteger(amount) || amount < 1) {
    throw new Error(
      "PaymentIntent amount must be a positive integer in the smallest currency unit.",
    );
  }

  if (amount > 99999999) {
    throw new Error("PaymentIntent amount must be 8 digits or fewer.");
  }

  return amount;
}

function readDxStripePaymentIntentCurrency(value: string) {
  const currency = value.trim();

  if (
    currency.length !== 3 ||
    currency !== currency.toLowerCase() ||
    !/^[a-z]+$/.test(currency)
  ) {
    throw new Error(
      "PaymentIntent currency must be a lowercase three-letter ISO currency code.",
    );
  }

  return currency;
}

function readDxStripePaymentIntentReceiptEmail(value: string | undefined) {
  const receiptEmail = value?.trim();
  return receiptEmail ? receiptEmail : undefined;
}

function readConfiguredWebhookSecret(config: DxStripeServerConfig) {
  if (!config.webhookSecret) {
    throw new Error(
      "STRIPE_WEBHOOK_SECRET is required to verify Stripe webhook events.",
    );
  }

  return config.webhookSecret;
}

function readStripeWebhookPayload(payload: string) {
  if (!payload) {
    throw new Error("Stripe webhook raw request body is required.");
  }

  return payload;
}

function readStripeWebhookSignature(signature: string | null | undefined) {
  const value = signature?.trim();

  if (!value) {
    throw new Error("Stripe-Signature header is required.");
  }

  return value;
}

function readDxStripeCustomerEmail(value: string | undefined) {
  const email = value?.trim().toLowerCase();

  if (!email) {
    return undefined;
  }

  if (!DX_STRIPE_EMAIL_PATTERN.test(email)) {
    throw new Error("Stripe customer email must be a valid email address.");
  }

  return email;
}

function readDxStripeCustomerName(value: string | undefined) {
  const name = value?.trim();

  if (!name) {
    return undefined;
  }

  if (name.length > 120) {
    throw new Error("Stripe customer name must be 120 characters or fewer.");
  }

  return name;
}

function readDxStripeListLimit(value: number | undefined, label: string) {
  if (value === undefined) {
    return undefined;
  }

  if (!Number.isInteger(value) || value < 1) {
    if (label === "subscription limit") {
      throw new Error(
        "Stripe subscription limit must be a positive integer.",
      );
    }

    throw new Error(`Stripe ${label} must be a positive integer.`);
  }

  if (value > 100) {
    if (label === "subscription limit") {
      throw new Error("Stripe subscription limit must be 100 or fewer.");
    }

    throw new Error(`Stripe ${label} must be 100 or fewer.`);
  }

  return value;
}

function readDxStripeCustomerId(value: string) {
  const customerId = value.trim();

  if (!customerId) {
    throw new Error("Stripe customer ID is required.");
  }

  if (!customerId.startsWith("cus_")) {
    throw new Error("Stripe customer ID must start with cus_.");
  }

  return customerId;
}

function readOptionalDxStripeCustomerId(value: string | undefined) {
  if (!value) {
    return undefined;
  }

  return readDxStripeCustomerId(value);
}

function readDxStripeBillingPortalConfiguration(value: string | undefined) {
  const configuration = value?.trim();

  if (!configuration) {
    return undefined;
  }

  if (!configuration.startsWith("bpc_")) {
    throw new Error("Stripe billing portal configuration ID must start with bpc_.");
  }

  return configuration;
}

function readDxStripeCheckoutLineItems(
  lineItems: Stripe.Checkout.SessionCreateParams.LineItem[],
) {
  if (!Array.isArray(lineItems) || lineItems.length === 0) {
    throw new Error("Stripe Checkout requires at least one line item.");
  }

  return lineItems.map(readDxStripeCheckoutLineItem);
}

function readDxStripeCheckoutLineItem(
  lineItem: Stripe.Checkout.SessionCreateParams.LineItem,
  index: number,
) {
  const label = `Stripe Checkout line item ${index + 1}`;

  if (
    lineItem.quantity !== undefined &&
    (!Number.isInteger(lineItem.quantity) || lineItem.quantity < 1)
  ) {
    throw new Error(`${label} quantity must be a positive integer.`);
  }

  if (typeof lineItem.price === "string") {
    const price = lineItem.price.trim();

    if (!price.startsWith("price_")) {
      throw new Error(`${label} requires a Stripe Price ID.`);
    }

    return {
      ...lineItem,
      price,
    };
  }

  if (lineItem.price_data) {
    return lineItem;
  }

  throw new Error(
    `${label} requires a Stripe Price ID or app-owned price_data.`,
  );
}

function toDxStripeCheckoutMetadata(
  contact: DxStripeCheckoutContact,
  metadata: Record<string, string | undefined> = {},
) {
  return stripEmptyMetadata({
    ...metadata,
    dx_contact_name: contact.name,
    dx_contact_organization: contact.organization,
    dx_contact_message: contact.message,
  });
}

function toDxStripeCheckoutCustomerParams(
  contact: DxStripeCheckoutContact,
  input: Pick<CreateDxStripeCheckoutSessionInput, "customerId" | "customerUpdate">,
): Pick<
  Stripe.Checkout.SessionCreateParams,
  "customer" | "customer_email" | "customer_update"
> {
  const customerId = readOptionalDxStripeCustomerId(input.customerId);

  if (customerId) {
    return {
      customer: customerId,
      customer_email: undefined,
      customer_update: input.customerUpdate,
    };
  }

  return {
    customer: undefined,
    customer_email: contact.email,
    customer_update: undefined,
  };
}

function stripEmptyMetadata(metadata: Record<string, string | undefined> = {}) {
  return Object.fromEntries(
    Object.entries(metadata)
      .filter(([, value]) => typeof value === "string" && value.trim())
      .map(([key, value]) => [key, String(value).slice(0, 500)]),
  );
}

function readStripeCheckoutRedirectUrl(value: string, label: string) {
  let url: URL;
  try {
    url = new URL(value);
  } catch {
    throw new Error(`Stripe checkout ${label} must be an absolute URL.`);
  }

  if (
    url.protocol === "https:" ||
    (url.protocol === "http:" &&
      isLocalStripeCheckoutRedirectHost(url.hostname))
  ) {
    return url.toString();
  }

  throw new Error(`Stripe checkout ${label} must use HTTPS outside localhost.`);
}

function isLocalStripeCheckoutRedirectHost(hostname: string) {
  return hostname === "localhost" || hostname === "127.0.0.1" || hostname === "[::1]";
}

function normalizeStripeAppUrl(value: string | undefined) {
  if (!value) {
    return undefined;
  }

  const normalized = /^https?:\/\//.test(value) ? value : `https://${value}`;
  const url = new URL(normalized);

  if (
    url.protocol === "https:" ||
    (url.protocol === "http:" && isLocalStripeAppHost(url.hostname))
  ) {
    return url.toString();
  }

  throw new Error("Stripe app URL must use HTTPS outside localhost.");
}

function isLocalStripeAppHost(hostname: string) {
  return hostname === "localhost" || hostname === "127.0.0.1" || hostname === "[::1]";
}
"#;

const STRIPE_ROUTE_TS: &str = r#"import {
  createDxStripeCheckoutContactPayload,
  type DxStripeCheckoutContact,
} from "@/lib/payments/stripe-js/checkout";
import {
  createDxStripeEmbeddedCheckoutSession,
  createDxStripeCheckoutSession,
  createDxStripeServerClient,
  readDxStripeServerConfig,
  type DxStripeServerConfig,
} from "@/lib/payments/stripe-js/server";

export const runtime = "nodejs";
export const dynamic = "force-dynamic";

const STRIPE_ROUTE_PLAN_PRICE_ENVS = {
  starter: "STRIPE_PRICE_ID_STARTER",
  team: "STRIPE_PRICE_ID_TEAM",
  scale: "STRIPE_PRICE_ID_SCALE",
} as const;

type StripeRoutePlanId = keyof typeof STRIPE_ROUTE_PLAN_PRICE_ENVS;

type CheckoutPlanSelection = {
  id: StripeRoutePlanId;
  priceEnv: string;
};

export async function POST(request: Request) {
  try {
    const body = await readCheckoutRequestBody(request);
    const contact = createDxStripeCheckoutContactPayload(
      readCheckoutContact(body),
    );
    const checkoutMode = readCheckoutMode(body);
    const checkoutPlan = readCheckoutPlan(body);
    const priceId = readStripePriceId(checkoutPlan);

    if (!priceId) {
      return createDxStripeCheckoutBoundaryResponse(
        202,
        "Checkout contact received. Set STRIPE_PRICE_ID to create live Stripe Checkout Sessions, or configure the selected plan Price env.",
      );
    }

    const config = readDxStripeServerConfig();
    const appUrl = readCheckoutAppUrl(config, request);
    const stripeClient = createDxStripeServerClient(config);

    if (checkoutMode === "embedded") {
      const session = await createDxStripeEmbeddedCheckoutSession(
        {
          contact,
          lineItems: [{ price: priceId, quantity: 1 }],
          returnUrl: buildStripeRouteUrl(
            appUrl,
            "/checkout/return?session_id={CHECKOUT_SESSION_ID}",
          ),
          metadata: createCheckoutMetadata(checkoutPlan),
        },
        stripeClient,
      );

      return Response.json({
        kind: "embedded-checkout-session",
        sessionId: session.id,
        clientSecret: session.client_secret,
        message: "Embedded Checkout Session created.",
      });
    }

    const session = await createDxStripeCheckoutSession(
      {
        contact,
        lineItems: [{ price: priceId, quantity: 1 }],
        successUrl: buildStripeRouteUrl(
          appUrl,
          "/checkout/success?session_id={CHECKOUT_SESSION_ID}",
        ),
        cancelUrl: buildStripeRouteUrl(appUrl, "/"),
        metadata: createCheckoutMetadata(checkoutPlan),
      },
      stripeClient,
    );

    if (!session.url) {
      throw new Error("Stripe Checkout Session did not include a URL.");
    }

    return Response.json({
      kind: "checkout-session",
      sessionId: session.id,
      url: session.url,
      message: "Checkout Session created.",
    });
  } catch (error) {
    const message =
      error instanceof Error ? error.message : "Checkout request failed.";

    return createDxStripeCheckoutBoundaryResponse(
      checkoutErrorStatus(message),
      message,
    );
  }
}

function createDxStripeCheckoutBoundaryResponse(status: number, message: string) {
  return Response.json(
    {
      kind: status === 501 ? "provider-boundary" : "contact",
      message,
      stripeLiveExecution: false,
      runtimeProof: false,
      networkCalls: false,
    },
    { status },
  );
}

async function readCheckoutRequestBody(request: Request) {
  try {
    return (await request.json()) as unknown;
  } catch {
    throw new Error("Checkout request body must be valid JSON.");
  }
}

function readCheckoutContact(
  value: unknown,
): Partial<Record<keyof DxStripeCheckoutContact, unknown>> {
  if (!isRecord(value) || !isRecord(value.contact)) {
    throw new Error("Checkout request requires contact details.");
  }

  return value.contact as Partial<Record<keyof DxStripeCheckoutContact, unknown>>;
}

function readCheckoutMode(value: unknown) {
  if (!isRecord(value)) {
    return "hosted";
  }

  const checkoutMode = value.checkoutMode;

  if (checkoutMode === undefined || checkoutMode === "hosted") {
    return "hosted";
  }

  if (checkoutMode === "embedded") {
    return "embedded";
  }

  throw new Error("Checkout mode must be hosted or embedded.");
}

function readCheckoutPlan(value: unknown): CheckoutPlanSelection | undefined {
  if (!isRecord(value) || value.plan === undefined) {
    return undefined;
  }

  if (!isRecord(value.plan)) {
    throw new Error("Checkout plan must be an app-owned plan object.");
  }

  const id = value.plan.id;

  if (id !== "starter" && id !== "team" && id !== "scale") {
    throw new Error("Checkout plan must be one of the app-owned Stripe plans.");
  }

  const priceEnv = STRIPE_ROUTE_PLAN_PRICE_ENVS[id];
  const requestedPriceEnv = value.plan.priceEnv;

  if (
    requestedPriceEnv !== undefined &&
    requestedPriceEnv !== priceEnv
  ) {
    throw new Error("Checkout plan price env must match the app-owned catalog.");
  }

  return {
    id,
    priceEnv,
  };
}

function readStripePriceId(checkoutPlan?: CheckoutPlanSelection) {
  const planPriceId = checkoutPlan
    ? process.env[checkoutPlan.priceEnv]?.trim()
    : undefined;
  const priceId = planPriceId || process.env.STRIPE_PRICE_ID?.trim();
  const priceEnv = planPriceId && checkoutPlan
    ? checkoutPlan.priceEnv
    : "STRIPE_PRICE_ID";

  if (!priceId) {
    return undefined;
  }

  if (!priceId.startsWith("price_")) {
    if (priceEnv === "STRIPE_PRICE_ID") {
      throw new Error("STRIPE_PRICE_ID must be a Stripe Price ID.");
    }

    throw new Error(`${priceEnv} must be a Stripe Price ID.`);
  }

  return priceId;
}

function createCheckoutMetadata(checkoutPlan?: CheckoutPlanSelection) {
  return {
    source: "dx-www-launch",
    dx_plan_id: checkoutPlan?.id,
    dx_price_env: checkoutPlan?.priceEnv,
  };
}

function readCheckoutAppUrl(config: DxStripeServerConfig, request: Request) {
  if (config.appUrl) {
    return config.appUrl;
  }

  const url = new URL(request.url);

  if (isLocalCheckoutRequestHost(url.hostname)) {
    return url.origin;
  }

  throw new Error(
    "NEXT_PUBLIC_APP_URL, APP_URL, or VERCEL_URL is required before creating Stripe Checkout outside localhost.",
  );
}

function buildStripeRouteUrl(appUrl: string, path: string) {
  const base = appUrl.endsWith("/") ? appUrl.slice(0, -1) : appUrl;
  return `${base}${path}`;
}

function checkoutErrorStatus(message: string) {
  if (
    message.includes("STRIPE_SECRET_KEY") ||
    message.includes("STRIPE_PRICE_ID") ||
    message.includes("Stripe Embedded Checkout Session") ||
    message.includes("Stripe Checkout Session")
  ) {
    return 500;
  }

  return 400;
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return Boolean(value) && typeof value === "object" && !Array.isArray(value);
}

function isLocalCheckoutRequestHost(hostname: string) {
  return hostname === "localhost" || hostname === "127.0.0.1" || hostname === "[::1]";
}
"#;

const STRIPE_WEBHOOK_ROUTE_TS: &str = r#"import type Stripe from "stripe";

import { verifyDxStripeWebhookRequest } from "@/lib/payments/stripe-js/server";

export const runtime = "nodejs";
export const dynamic = "force-dynamic";

export async function POST(request: Request) {
  try {
    const event = await verifyDxStripeWebhookRequest(request);
    const summary = createDxStripeWebhookDeliverySummary(event);
    const action = routeDxStripeWebhookEvent(event);

    return Response.json({
      received: true,
      ...summary,
      ...action,
    });
  } catch (error) {
    if (isDxStripeWebhookProviderBoundaryError(error)) {
      return createDxStripeWebhookProviderBoundaryResponse(error);
    }

    return Response.json(
      {
        ok: false,
        received: false,
        status: "bad-request",
        packageId: "payments/stripe-js",
        webhookVerified: false,
        stripeLiveExecution: false,
        secretValues: [],
        error:
          error instanceof Error
            ? error.message
            : "Stripe webhook verification failed.",
      },
      { status: 400 },
    );
  }
}

const stripeWebhookProviderBoundaryMessages = [
  "Missing STRIPE_SECRET_KEY.",
  "STRIPE_SECRET_KEY must be a Stripe secret key.",
  "STRIPE_WEBHOOK_SECRET is required to verify Stripe webhook events.",
  "STRIPE_WEBHOOK_SECRET must be a Stripe webhook signing secret.",
] as const;

const stripeWebhookRequiredEnv = [
  "STRIPE_SECRET_KEY",
  "STRIPE_WEBHOOK_SECRET",
] as const;

function createDxStripeWebhookProviderBoundaryResponse(error: unknown) {
  const message = readDxStripeWebhookErrorMessage(error);

  return Response.json(
    {
      schema: "dx.payments.stripe_js.webhook_boundary",
      ok: false,
      received: false,
      packageId: "payments/stripe-js",
      officialPackageName: "Payments",
      upstreamPackage: "stripe",
      status: isDxStripeWebhookMissingConfigMessage(message)
        ? "missing-config"
        : "invalid-provider-config",
      httpStatus: 501,
      kind: "provider-boundary",
      endpoint: "/api/stripe/webhook",
      requiredEnv: stripeWebhookRequiredEnv,
      providerBoundary: true,
      runtimeExecution: false,
      stripeLiveExecution: false,
      webhookVerified: false,
      eventRouted: false,
      secretValues: [],
      fulfillmentStatus: "app-owned",
      error: message,
      appOwnedBoundary:
        "Set Stripe server and webhook secrets before accepting signed webhook delivery.",
    },
    { status: 501 },
  );
}

function isDxStripeWebhookProviderBoundaryError(error: unknown) {
  const message = readDxStripeWebhookErrorMessage(error);

  return stripeWebhookProviderBoundaryMessages.some(
    (candidate) => candidate === message,
  );
}

function isDxStripeWebhookMissingConfigMessage(message: string) {
  return (
    message === "Missing STRIPE_SECRET_KEY." ||
    message === "STRIPE_WEBHOOK_SECRET is required to verify Stripe webhook events."
  );
}

function readDxStripeWebhookErrorMessage(error: unknown) {
  return error instanceof Error
    ? error.message
    : "Stripe webhook verification failed.";
}

function routeDxStripeWebhookEvent(event: Stripe.Event) {
  switch (event.type) {
    case "checkout.session.completed":
      return {
        eventAction: "checkout-session-completed",
        fulfillmentStatus: "app-owned",
      };
    case "checkout.session.expired":
      return {
        eventAction: "checkout-session-expired",
        fulfillmentStatus: "app-owned",
      };
    case "payment_intent.succeeded":
      return {
        eventAction: "payment-intent-succeeded",
        fulfillmentStatus: "app-owned",
      };
    case "payment_intent.payment_failed":
      return {
        eventAction: "payment-intent-payment-failed",
        fulfillmentStatus: "app-owned",
      };
    default:
      return {
        eventAction: "unhandled",
        fulfillmentStatus: "app-owned",
      };
  }
}

function createDxStripeWebhookDeliverySummary(event: Stripe.Event) {
  return {
    eventId: event.id,
    eventType: event.type,
    livemode: event.livemode,
    created: event.created,
  };
}
"#;

const STRIPE_METADATA_TS: &str = r#"export const dxStripeJsForgePackage = {
  packageId: "payments/stripe-js",
  officialPackageName: "Payments",
  upstreamPackage: "@stripe/stripe-js",
  upstreamVersion: "9.6.0",
  forgeVersion: "9.6.0-dx.1",
  docsPath: "docs/packages/payments-stripe-js.md",
  aliases: ["payments", "stripe-js", "@stripe/stripe-js", "stripe", "payments/stripe"],
  sourceMirror: "G:/WWW/inspirations/stripe-js",
  provenance: {
    packageJson: "G:/WWW/inspirations/stripe-js/package.json",
    docs: [
      "G:/WWW/inspirations/stripe-js/README.md",
      "G:/WWW/inspirations/stripe-js/src/shared.ts",
      "G:/WWW/inspirations/stripe-js/types/shared.d.ts",
      "G:/WWW/inspirations/stripe-js/types/stripe-js/stripe.d.ts",
      "G:/WWW/inspirations/stripe-js/types/stripe-js/checkout.d.ts",
    ],
    inspectedSourceFiles: [
      "package.json",
      "README.md",
      "src/pure.ts",
      "src/shared.ts",
      "types/shared.d.ts",
      "types/stripe-js/stripe.d.ts",
      "types/stripe-js/checkout.d.ts",
    ],
    upstreamApis: [
      "loadStripe",
      "loadStripe.setLoadParameters",
      "stripe.confirmPayment",
      "stripe.retrievePaymentIntent",
      "stripe.createEmbeddedCheckoutPage",
      "StripeEmbeddedCheckoutOptions.fetchClientSecret",
    ],
  },
  publicApi: [
    "loadStripe",
    "loadStripe.setLoadParameters",
    "Stripe",
    "StripeElements",
    "StripeConstructorOptions",
    "elements.submit",
    "stripe.confirmPayment",
    "stripe.retrievePaymentIntent",
    "retrieveDxStripePaymentIntent",
    "readDxStripeServerConfig",
    "createDxStripeServerClient",
    "createDxStripeCustomer",
    "listDxStripeCustomerSubscriptions",
    "createDxStripeCheckoutSession",
    "createDxStripeEmbeddedCheckoutSession",
    "createDxStripeBillingPortalSession",
    "retrieveDxStripeCheckoutSession",
    "createDxStripePaymentIntent",
    "verifyDxStripeWebhookEvent",
    "verifyDxStripeWebhookRequest",
    "validateDxStripeCheckoutContact",
    "submitDxStripeCheckoutContact",
    "createDxStripeDashboardCheckoutRequest",
    "createDxStripeDashboardMissingConfigReceipt",
    "stripe.createEmbeddedCheckoutPage",
    "StripeEmbeddedCheckoutOptions.fetchClientSecret",
    "createDxStripeEmbeddedCheckoutClientSecretFetcher",
    "createDxStripeEmbeddedCheckout",
  ],
  materializedFiles: [
    "lib/payments/stripe-js/config.ts",
    "lib/payments/stripe-js/client.ts",
    "lib/payments/stripe-js/payment.ts",
    "lib/payments/stripe-js/checkout.ts",
    "lib/payments/stripe-js/dashboard-checkout.ts",
    "lib/payments/stripe-js/server.ts",
    "app/api/checkout/route.ts",
    "app/api/stripe/webhook/route.ts",
    "lib/payments/stripe-js/metadata.ts",
    "lib/payments/stripe-js/README.md",
  ],
  exportedFiles: {
    config: "lib/payments/stripe-js/config.ts",
    client: "lib/payments/stripe-js/client.ts",
    payment: "lib/payments/stripe-js/payment.ts",
    checkout: "lib/payments/stripe-js/checkout.ts",
    dashboardCheckout: "lib/payments/stripe-js/dashboard-checkout.ts",
    server: "lib/payments/stripe-js/server.ts",
    checkoutRoute: "app/api/checkout/route.ts",
    webhookRoute: "app/api/stripe/webhook/route.ts",
    metadata: "lib/payments/stripe-js/metadata.ts",
    docs: "lib/payments/stripe-js/README.md",
  },
  requiredDependencies: [
    {
      name: "@stripe/stripe-js",
      version: "^9.6.0",
      reason: "Provides the real Stripe.js loader, types, and client payment APIs.",
    },
    {
      name: "stripe",
      version: "^18.0.0",
      reason: "Creates Checkout Sessions from server-only code without exposing secret keys.",
    },
  ],
  requiredEnv: ["NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY", "STRIPE_SECRET_KEY"],
  optionalEnv: [
    "NEXT_PUBLIC_STRIPE_ACCOUNT",
    "NEXT_PUBLIC_STRIPE_API_VERSION",
    "NEXT_PUBLIC_STRIPE_LOCALE",
    "NEXT_PUBLIC_STRIPE_ADVANCED_FRAUD_SIGNALS",
    "STRIPE_API_VERSION",
    "STRIPE_WEBHOOK_SECRET",
    "STRIPE_PRICE_ID",
    "STRIPE_PRICE_ID_STARTER",
    "STRIPE_PRICE_ID_TEAM",
    "STRIPE_PRICE_ID_SCALE",
    "APP_URL",
    "NEXT_PUBLIC_APP_URL",
  ],
  optionalIntegrations: [
    {
      packageId: "forms/react-hook-form",
      reason: "Launch checkout contact forms use React Hook Form submit state.",
    },
    {
      packageId: "validation/zod",
      reason: "Launch checkout contact forms use Zod-compatible validation.",
    },
  ],
  surfaces: [
    "launch-billing-checkout-workflow",
    "dashboard-stripe-plan-checkout",
    "checkout-session-route",
    "stripe-webhook-route",
    "payment-element-helper",
    "embedded-checkout-helper",
    "billing-portal-helper",
  ],
  dxCheckVisibility: {
    schema: "dx.forge.package.dx_check_visibility",
    currentStatus: "present",
    statuses: [
      "present",
      "stale",
      "missing-receipt",
      "blocked",
      "unsupported-surface",
    ],
    receiptPath: "examples/template/.dx/forge/receipts/2026-05-22-payments-stripe-js-billing-workflow.json",
    monitoredSurfaces: [
      "payments-launch-billing-checkout-workflow",
      "payments-checkout-session-route",
      "payments-webhook-route",
    ],
  },
  honestyLabel: "ADAPTER-BOUNDARY",
  appOwnedBoundaries: [
    "Stripe account, products, prices, tax, fraud, dispute, and refund policy",
    "STRIPE_SECRET_KEY, STRIPE_WEBHOOK_SECRET, and server-only secret rotation",
    "Authenticated customer lookup before Checkout or Billing Portal sessions",
    "Checkout success authorization, fulfillment, and webhook idempotency",
    "Dashboard plan catalog, entitlement mapping, and billing interval copy",
  ],
  receiptPaths: {
    package: ".dx/forge/receipts/payments-stripe-js.json",
    launch: ".dx/forge/docs/launch-companions/payments-status.md",
    dashboard: ".dx/forge/docs/dashboard-stripe-plan-checkout.md",
    dashboardWorkflow: "examples/template/.dx/forge/receipts/2026-05-22-payments-stripe-js-billing-workflow.json",
    previewManifest: "examples/conversion-proof/public/preview-manifest.json#launch-runtime-billing-checkout",
    sourceGuard: "benchmarks/stripe-rhf-checkout-flow.test.ts",
  },
  dashboardUsage: {
    sourceFile: "examples/dashboard/src/components/StripePlanCheckout.tsx",
    helperFile: "examples/dashboard/src/lib/stripePlanCheckout.ts",
    visibleWorkflow: "plan checkout action with safe missing-config receipt",
    packageMarker: 'data-dx-package="payments/stripe-js"',
    componentMarker: 'data-dx-component="dashboard-stripe-plan-checkout"',
    actionMarker: 'data-dx-stripe-action="request-checkout-session"',
    icon: "pack:payments",
  },
  launchUsage: {
    sourceFile: "examples/template/payments-status.tsx",
    materializedFile: "components/launch/payments-status.tsx",
    visibleWorkflow: "launch billing checkout workflow with plan selection and safe missing-config receipt",
    receiptFile: "examples/template/.dx/forge/receipts/2026-05-22-payments-stripe-js-billing-workflow.json",
    packageMarker: 'data-dx-package="payments/stripe-js"',
    componentMarker: 'data-dx-component="launch-billing-checkout-workflow"',
    dashboardFlowMarker: 'data-dx-dashboard-flow="billing-checkout"',
    actionMarker: 'data-dx-stripe-action="request-checkout-intent"',
    receiptPathMarker: 'data-dx-stripe-receipt-path="examples/template/.dx/forge/receipts/2026-05-22-payments-stripe-js-billing-workflow.json"',
    previewManifestSurface: "launch-runtime-billing-checkout",
    materializerFile: "tools/launch/materialize-www-template.ts",
    sourceGuard: "benchmarks/stripe-payment-launch-proof.test.ts",
  },
  discovery: {
    dxAdd: "dx add payments --write",
    canonicalPackage: "payments/stripe-js",
    clientHelper: "getDxStripe()",
    paymentSubmitGuard: "elements.submit()",
    paymentHelper: "confirmDxStripePayment({ elements, intent })",
    paymentStatusHelper: "retrieveDxStripePaymentIntent({ clientSecret })",
    checkoutHelper: "submitDxStripeCheckoutContact({ contact })",
    dashboardCheckoutHelper: "createDxStripeDashboardCheckoutRequest({ planId, checkoutMode, contact })",
    dashboardMissingConfigReceipt: "createDxStripeDashboardMissingConfigReceipt(request)",
    embeddedCheckoutHelper: "createDxStripeEmbeddedCheckout({ clientSecret })",
    embeddedCheckoutFetchHelper: "createDxStripeEmbeddedCheckout({ fetchClientSecret })",
    embeddedCheckoutClientSecretFetcher: "createDxStripeEmbeddedCheckoutClientSecretFetcher({ contact })",
    serverCustomerHelper: "createDxStripeCustomer(input)",
    serverSubscriptionListHelper: "listDxStripeCustomerSubscriptions({ customerId })",
    serverCheckoutHelper: "createDxStripeCheckoutSession(input)",
    serverCheckoutCustomerLinking: "createDxStripeCheckoutSession({ customerId, customerUpdate })",
    serverEmbeddedCheckoutHelper: "createDxStripeEmbeddedCheckoutSession(input)",
    serverBillingPortalHelper: "createDxStripeBillingPortalSession(input)",
    serverCheckoutRetrieveHelper: "retrieveDxStripeCheckoutSession(input)",
    serverPaymentIntentHelper: "createDxStripePaymentIntent(input)",
    webhookHelper: "verifyDxStripeWebhookRequest(request)",
    checkoutRoute: "app/api/checkout/route.ts",
    webhookRoute: "app/api/stripe/webhook/route.ts",
  },
} as const;

export type DxStripeJsForgePackage = typeof dxStripeJsForgePackage;
"#;

const STRIPE_README_MD: &str = r#"# DX Forge Payments Slice

This Payments package materializes a small source-owned adapter around the real `@stripe/stripe-js` 9.6 public API and server-only Stripe Checkout, Billing Portal, and webhook boundaries. It imports from `@stripe/stripe-js/pure` so Stripe.js is loaded when your app asks for it, not as an import-time side effect.

## Owned Files

- `config.ts` reads public browser Stripe configuration and rejects missing or leaked secret keys.
- `client.ts` wraps `loadStripe`, `loadStripe.setLoadParameters`, and Stripe constructor options.
- `payment.ts` provides typed `confirmPayment` and `retrievePaymentIntent` helpers for Payment Element flows.
- `checkout.ts` validates checkout contact payloads, submits them to an app-owned checkout endpoint, and creates Embedded Checkout instances from Checkout Session client secrets.
- `dashboard-checkout.ts` reuses the checkout contact validation to prepare dashboard plan checkout requests and safe missing-config receipts.
- `server.ts` imports `server-only`, validates `STRIPE_SECRET_KEY`, verifies webhook signatures, and creates hosted Checkout, embedded Checkout, or Billing Portal Sessions with the official `stripe` server SDK.
- `app/api/checkout/route.ts` validates contact requests and creates Checkout Sessions only when a plan-specific Price env or fallback `STRIPE_PRICE_ID` is configured.
- `app/api/stripe/webhook/route.ts` verifies Stripe signatures from the raw request body and acknowledges verified event delivery.
- `metadata.ts` lets DX CLI, Zed, and launch templates discover dependencies, env, and helper names.

## Required App Dependency

Install or provide `@stripe/stripe-js` in the host app and `stripe` on the server. Forge owns these adapter files and receipts; Stripe.js and Stripe's server SDK remain the runtime payment SDKs.

## Application-Owned Work

- Create Checkout Sessions with `server.ts` for checkout/contact requests.
- Create embedded Checkout Sessions with `createDxStripeEmbeddedCheckoutSession` and `ui_mode: "embedded"`.
- Create Billing Portal Sessions with `createDxStripeBillingPortalSession` only after authenticated app-owned customer lookup.
- Billing Portal customer IDs must come from your server-owned user or account record, not browser input.
- Pass `customerId` from your authenticated server record into hosted or embedded Checkout helpers when you want Checkout and Billing Portal to share a Stripe customer.
- The generated checkout route intentionally does not trust browser-provided Stripe customer IDs.
- Create Stripe Customers with `createDxStripeCustomer` after authentication and persist the returned `customer.id` in your app database.
- Customer creation helpers do not own user lookup, deduplication policy, or database writes.
- List customer subscriptions with `listDxStripeCustomerSubscriptions` before deciding whether to send a signed-in user to Checkout or Billing Portal.
- Subscription listing helpers do not own plan mapping, entitlement checks, billing interval display, or database writes.
- Retrieve Checkout Sessions with `retrieveDxStripeCheckoutSession` before trusting success-page state.
- Create PaymentIntents with `createDxStripePaymentIntent` for Payment Element flows.
- Retrieve PaymentIntents with `retrieveDxStripePaymentIntent` on Payment Element return pages.
- Create Embedded Checkout instances with `createDxStripeEmbeddedCheckout` only after your server creates an embedded Checkout Session client secret.
- Use `createDxStripeEmbeddedCheckoutClientSecretFetcher` to adapt the app-owned checkout endpoint into Stripe.js `fetchClientSecret`.
- Use `fetchClientSecret` for Embedded Checkout when the client should lazily request a Checkout Session client secret from an app-owned endpoint.
- Verify webhook events with `verifyDxStripeWebhookRequest` or `verifyDxStripeWebhookEvent`.
- The generated webhook route acknowledges verified delivery and leaves fulfillment app-owned.
- Set plan-specific `STRIPE_PRICE_ID_STARTER`, `STRIPE_PRICE_ID_TEAM`, `STRIPE_PRICE_ID_SCALE`, fallback `STRIPE_PRICE_ID`, or replace the route's line item lookup with your own catalog.
- `STRIPE_PRICE_ID` must be a Stripe Price ID from your app-owned product catalog.
- Plan-specific env such as `STRIPE_PRICE_ID_STARTER` can override the fallback `STRIPE_PRICE_ID` when the dashboard sends an app-owned plan selection.
- Checkout Sessions require at least one app-owned line item.
- Success pages still own authorization, persistence checks, and fulfillment status.
- Checkout line items require a Stripe Price ID or app-owned `price_data`.
- The Checkout route pins `runtime = "nodejs"` because Stripe's server SDK is not an Edge runtime dependency.
- Production Checkout redirects require `NEXT_PUBLIC_APP_URL`, `APP_URL`, or `VERCEL_URL`; request-origin fallback is local-only.
- `NEXT_PUBLIC_STRIPE_ACCOUNT`, when set, must be a Stripe connected account ID.
- `NEXT_PUBLIC_STRIPE_ADVANCED_FRAUD_SIGNALS`, when set, must be `true` or `false`.
- Stripe API version env values must use Stripe's date-based version shape, for example `2026-02-25.clover`.
- Client checkout contact submissions only allow same-origin paths or HTTPS endpoints.
- Embedded Checkout helpers require a Checkout Session client secret, not a hosted Checkout URL.
- Embedded Checkout Sessions use `return_url`; hosted Checkout Sessions use `success_url` and `cancel_url`.
- Embedded Checkout fetchers require the endpoint to return `kind: "embedded-checkout-session"`.
- Embedded Checkout `fetchClientSecret` callbacks are revalidated before Stripe.js receives the value.
- Payment Element helpers require a server-created PaymentIntent client secret.
- `confirmDxStripePayment` calls `elements.submit()` before `stripe.confirmPayment`.
- PaymentIntent status pages still own order lookup, access control, and fulfillment state.
- PaymentIntent amounts must be positive integers in the smallest currency unit.
- For Payment Element flows, create PaymentIntents on your server before calling `confirmDxStripePayment`.
- Webhook handlers still own event routing, idempotency, persistence, and fulfillment.
- Keep secret keys and webhook secrets out of browser code.
- Review pricing, tax, fraud, dispute, refund, and compliance policy.
- Mount real Stripe Checkout or Payment Element UI and handle failure states.

## Dashboard starter usage

The dashboard starter consumes this package through `examples/dashboard/src/components/StripePlanCheckout.tsx` and `examples/dashboard/src/lib/stripePlanCheckout.ts`. The visible workflow lets an operator choose a plan, choose hosted or embedded Checkout mode, validate contact details, and create a safe missing-config receipt for `/api/checkout` through the same `createDxStripeDashboardCheckoutRequest({ planId, checkoutMode, contact })` and `createDxStripeDashboardMissingConfigReceipt` API names without collecting card data or pretending Stripe credentials exist.

The dashboard surface uses `<dx-icon name="pack:payments" />`, `data-dx-package="payments/stripe-js"`, and `data-dx-component="dashboard-stripe-plan-checkout"` so DX Studio and dx-check can find the package behavior directly in the starter UI.

## Launch billing workflow

The `/launch` template consumes the same plan-checkout boundary through `examples/template/payments-status.tsx`. The visible billing workflow selects an app-owned plan, chooses hosted or embedded Checkout, validates contact details with React Hook Form plus Zod, and creates a safe missing-config receipt through `createDxStripeDashboardCheckoutRequest` and `createDxStripeDashboardMissingConfigReceipt` when Stripe credentials are absent.

The launch surface uses `data-dx-component="launch-billing-checkout-workflow"`, `data-dx-dashboard-flow="billing-checkout"`, `data-dx-stripe-dashboard-workflow="plan-checkout"`, `data-dx-stripe-action="request-checkout-intent"`, and `data-dx-stripe-receipt-path="examples/template/.dx/forge/receipts/2026-05-22-payments-stripe-js-billing-workflow.json"` so Zed Web Preview can map the payment interaction back to source and receipt proof. The runtime materializer preserves that selector as `launch-runtime-billing-checkout` in `public/preview-manifest.json`, keeping the generated/static Web Preview handoff pointed at the billing workflow instead of the generic package proof grid.

## Template Usage

The slice materializes `app/api/checkout/route.ts`. Configure real app values before expecting a hosted Checkout URL:

```env
STRIPE_SECRET_KEY=<set in deployment environment>
STRIPE_PRICE_ID=<Stripe Price ID from your product catalog>
STRIPE_PRICE_ID_TEAM=<optional selected-plan override>
NEXT_PUBLIC_APP_URL=https://your-app.example
```

Without a plan-specific or fallback Price ID, the route returns an honest contact receipt so the launch form never pretends a Checkout Session exists.
"#;
