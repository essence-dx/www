import type Stripe from "stripe";

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
