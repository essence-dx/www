export type DxStripeDashboardCheckoutMode = "hosted" | "embedded";

export type DxStripeDashboardPlan = {
    id: "starter" | "team" | "scale";
    label: string;
    priceLabel: string;
    description: string;
    priceEnv: string;
};

export type DxStripeDashboardCheckoutContact = {
    email: string;
    name: string;
    organization?: string;
};

export type DxStripeDashboardCheckoutRequest = {
    endpoint: "/api/checkout";
    method: "POST";
    checkoutMode: DxStripeDashboardCheckoutMode;
    contact: DxStripeDashboardCheckoutContact;
    plan: DxStripeDashboardPlan;
    publicApi: string[];
    body: {
        checkoutMode: DxStripeDashboardCheckoutMode;
        contact: DxStripeDashboardCheckoutContact;
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
    requiredEnv: string[];
    request: DxStripeDashboardCheckoutRequest;
};

export const dxStripeDashboardPlans: DxStripeDashboardPlan[] = [
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
    upstreamPackage: "@stripe/stripe-js",
    upstreamVersion: "9.6.0",
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
}: {
    planId: DxStripeDashboardPlan["id"];
    checkoutMode: DxStripeDashboardCheckoutMode;
    contact: Partial<DxStripeDashboardCheckoutContact>;
}): DxStripeDashboardCheckoutRequest {
    const plan = readDxStripeDashboardPlan(planId);
    const normalizedContact = normalizeStripeDashboardContact(contact);

    return {
        endpoint: "/api/checkout",
        method: "POST",
        checkoutMode,
        contact: normalizedContact,
        plan,
        publicApi: [...dxStripeDashboardCheckoutReadiness.publicApi],
        body: {
            checkoutMode,
            contact: normalizedContact,
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
        receiptId: `stripe-dashboard-local-${request.plan.id}-${request.checkoutMode}-${slugifyEmail(request.contact.email)}`,
        status: "missing-config",
        message:
            "Checkout request is ready, but Stripe credentials and a product Price ID must be app-owned before creating a Checkout Session.",
        requiredEnv: [...dxStripeDashboardCheckoutReadiness.requiredEnv],
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

function normalizeStripeDashboardContact(
    contact: Partial<DxStripeDashboardCheckoutContact>,
): DxStripeDashboardCheckoutContact {
    const email = String(contact.email ?? "").trim().toLowerCase();
    const name = String(contact.name ?? "").trim();
    const organization = String(contact.organization ?? "").trim();

    if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email)) {
        throw new Error("Enter a valid checkout email.");
    }

    if (name.length < 2) {
        throw new Error("Enter the checkout contact name.");
    }

    return {
        email,
        name,
        organization: organization || undefined,
    };
}

function slugifyEmail(email: string) {
    return email.replace(/[^a-z0-9]+/g, "-").replace(/^-|-$/g, "") || "contact";
}
