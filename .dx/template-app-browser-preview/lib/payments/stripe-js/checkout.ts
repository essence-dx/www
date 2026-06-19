import type {
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
