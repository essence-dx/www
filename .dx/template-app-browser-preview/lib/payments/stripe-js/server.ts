import "server-only";

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
