import { createDxStripeCheckoutContactPayload } from "@/lib/payments/stripe-js/checkout";

type CheckoutMode = "hosted" | "embedded";

const requiredEnv = [
  "NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY",
  "STRIPE_SECRET_KEY",
] as const;
const fallbackPriceEnv = "STRIPE_PRICE_ID";

export async function POST(request: Request) {
  try {
    const body = await readJsonBody(request);
    const contact = createDxStripeCheckoutContactPayload(
      isRecord(body.contact) ? body.contact : {},
    );
    const checkoutMode = readCheckoutMode(body);
    const checkoutPlan = readCheckoutPlan(body);
    const configured = hasStripeCheckoutConfig(checkoutPlan.priceEnv);
    const status = configured ? 202 : 501;
    const httpStatus = statusToHttpStatus(status);

    return Response.json(
      {
        ok: configured,
        packageId: "payments/stripe-js",
        status: configured ? "provider-configured-dry-run-only" : "missing-config",
        httpStatus,
        kind: status === 501 ? "provider-boundary" : "contact",
        checkoutMode,
        contact,
        plan: checkoutPlan,
        requiredEnv: [...requiredEnv, checkoutPlan.priceEnv, fallbackPriceEnv],
        runtimeExecution: false,
        stripeLiveExecution: false,
        secretValues: [],
        appOwnedBoundary:
          "Create a real Stripe Checkout Session only after Stripe credentials and Price IDs are configured.",
      },
      { status: httpStatus },
    );
  } catch (error) {
    return Response.json(
      {
        ok: false,
        packageId: "payments/stripe-js",
        status: "bad-request",
        message:
          error instanceof Error ? error.message : "Checkout request failed.",
        runtimeExecution: false,
        stripeLiveExecution: false,
        secretValues: [],
      },
      { status: 400 },
    );
  }
}

async function readJsonBody(request: Request) {
  try {
    const value = await request.json();
    return isRecord(value) ? value : {};
  } catch {
    return {};
  }
}

function readCheckoutMode(body: Record<string, unknown>): CheckoutMode {
  return body.checkoutMode === "embedded" ? "embedded" : "hosted";
}

function readCheckoutPlan(body: Record<string, unknown>) {
  const plan = isRecord(body.plan) ? body.plan : {};
  const id = typeof plan.id === "string" && plan.id.trim() ? plan.id.trim() : "starter";
  const priceEnv =
    typeof plan.priceEnv === "string" && plan.priceEnv.trim()
      ? plan.priceEnv.trim()
      : fallbackPriceEnv;

  return { id, priceEnv };
}

function hasStripeCheckoutConfig(priceEnv: string) {
  const env = readEnv();
  return (
    requiredEnv.every((name) => Boolean(env[name]?.trim())) &&
    Boolean((env[priceEnv] ?? env[fallbackPriceEnv])?.trim())
  );
}

function readEnv() {
  return (
    (
      globalThis as typeof globalThis & {
        process?: { env?: Record<string, string | undefined> };
      }
    ).process?.env ?? {}
  );
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return Boolean(value) && typeof value === "object" && !Array.isArray(value);
}

function statusToHttpStatus(status: number) {
  if (status === 501) {
    return 501;
  }

  return status;
}
