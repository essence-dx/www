import {
  createDxStripeDashboardCheckoutRequest,
  createDxStripeDashboardMissingConfigReceipt,
  dxStripeDashboardCheckoutReadiness,
  dxStripeDashboardPlans,
} from "@/lib/payments/stripe-js/dashboard-checkout";

export const runtime = "nodejs";
export const dynamic = "force-dynamic";

type CheckoutMode = "hosted" | "embedded";
type DashboardPlanId = (typeof dxStripeDashboardPlans)[number]["id"];

const baseRequiredEnv = [
  "NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY",
  "STRIPE_SECRET_KEY",
] as const;
const fallbackPriceEnv = "STRIPE_PRICE_ID";

export async function GET() {
  const readiness = buildDxStripeReadiness();

  return Response.json({
    ok: true,
    packageId: "payments/stripe-js",
    status: readiness.status,
    readiness,
    plans: dxStripeDashboardPlans.map((plan) => ({
      id: plan.id,
      label: plan.label,
      priceLabel: plan.priceLabel,
      priceEnv: plan.priceEnv,
    })),
    runtimeExecution: false,
    stripeLiveExecution: false,
    secretValues: [],
    boundary:
      "This route exposes Payments readiness only. Stripe keys, Price IDs, Checkout redirects, webhook delivery, and fulfillment stay app-owned.",
  });
}

export async function POST(request: Request) {
  try {
    const body = await readJsonBody(request);
    const checkoutRequest = createDxStripeDashboardCheckoutRequest({
      planId: readPlanId(body),
      checkoutMode: readCheckoutMode(body),
      contact: readCheckoutContact(body),
    });
    const readiness = buildDxStripeReadiness(checkoutRequest.plan.priceEnv);
    const missingConfigReceipt =
      createDxStripeDashboardMissingConfigReceipt(checkoutRequest);
    const receipt =
      readiness.status === "missing-config"
        ? missingConfigReceipt
        : {
            ...missingConfigReceipt,
            status: "dry-run-ready",
            message:
              "Checkout request is provider-configured; this readiness route still did not call Stripe.",
          };

    return Response.json(
      {
        ok: readiness.status !== "missing-config",
        packageId: "payments/stripe-js",
        status: readiness.status,
        httpStatus: readiness.status === "missing-config" ? 501 : 202,
        providerBoundary: readiness.status === "missing-config",
        request: checkoutRequest,
        receipt,
        readiness,
        runtimeExecution: false,
        stripeLiveExecution: false,
        secretValues: [],
        boundary:
          "The response is a local readiness record. It does not create Checkout Sessions, read secret values, redirect users, or process webhooks.",
      },
      { status: readiness.status === "missing-config" ? 501 : 202 },
    );
  } catch (error) {
    return Response.json(
      {
        ok: false,
        packageId: "payments/stripe-js",
        status: "bad-request",
        message:
          error instanceof Error
            ? error.message
            : "Payments readiness request failed.",
        runtimeExecution: false,
        stripeLiveExecution: false,
        secretValues: [],
      },
      { status: 400 },
    );
  }
}

function buildDxStripeReadiness(selectedPriceEnv?: string) {
  const env = readEnv();
  const selectedPriceConfigured = selectedPriceEnv
    ? isEnvConfigured(env, selectedPriceEnv) || isEnvConfigured(env, fallbackPriceEnv)
    : isEnvConfigured(env, fallbackPriceEnv) ||
      dxStripeDashboardPlans.some((plan) => isEnvConfigured(env, plan.priceEnv));
  const missingRequiredEnv = [
    ...baseRequiredEnv.filter((name) => !isEnvConfigured(env, name)),
    ...(selectedPriceConfigured
      ? []
      : [selectedPriceEnv ?? "STRIPE_PRICE_ID or one plan-specific Price env"]),
  ];

  return {
    schema: "dx.payments.stripe_js.readiness",
    packageId: "payments/stripe-js",
    status:
      missingRequiredEnv.length === 0
        ? "provider-configured-dry-run-only"
        : "missing-config",
    endpoint: "/api/payments/stripe-js/readiness",
    liveCheckoutEndpoint: dxStripeDashboardCheckoutReadiness.endpoint,
    requiredEnv: selectedPriceEnv
      ? [...baseRequiredEnv, selectedPriceEnv, fallbackPriceEnv]
      : dxStripeDashboardCheckoutReadiness.requiredEnv,
    selectedPriceEnv: selectedPriceEnv ?? null,
    configuredEnv: envReadinessRows(env, selectedPriceEnv),
    missingRequiredEnv,
    runtimeExecution: false,
    stripeLiveExecution: false,
    secretValues: [],
  };
}

function envReadinessRows(
  env: Record<string, string | undefined>,
  selectedPriceEnv?: string,
) {
  const names = new Set<string>([
    ...dxStripeDashboardCheckoutReadiness.requiredEnv,
    ...(selectedPriceEnv ? [selectedPriceEnv] : []),
  ]);

  return [...names].map((name) => ({
    name,
    configured: isEnvConfigured(env, name),
    valueExposed: false,
  }));
}

async function readJsonBody(request: Request) {
  try {
    const value = await request.json();
    return isRecord(value) ? value : {};
  } catch {
    return {};
  }
}

function readPlanId(body: Record<string, unknown>): DashboardPlanId {
  const value = isRecord(body.plan) ? body.plan.id : body.planId;

  if (value === "starter" || value === "team" || value === "scale") {
    return value;
  }

  return "starter";
}

function readCheckoutMode(body: Record<string, unknown>): CheckoutMode {
  if (body.checkoutMode === "embedded") {
    return "embedded";
  }

  return "hosted";
}

function readCheckoutContact(body: Record<string, unknown>) {
  if (isRecord(body.contact)) {
    return body.contact;
  }

  throw new Error("Payments readiness dry-run requires contact details.");
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

function isEnvConfigured(
  env: Record<string, string | undefined>,
  name: string,
) {
  return Boolean(env[name]?.trim());
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return Boolean(value) && typeof value === "object" && !Array.isArray(value);
}
