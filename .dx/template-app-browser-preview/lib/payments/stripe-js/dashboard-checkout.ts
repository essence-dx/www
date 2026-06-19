import {
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
