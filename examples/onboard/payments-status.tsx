"use client";

import * as React from "react";
import { z } from "zod";

import { Button } from "@/components/ui/button";
import { Textarea } from "@/components/ui/textarea";
import {
  createDxStripeEmbeddedCheckout,
  createDxStripeEmbeddedCheckoutClientSecretFetcher,
  submitDxStripeCheckoutContact,
  type DxStripeCheckoutMode,
  type DxStripeCheckoutSubmitResponse,
  type DxStripeCheckoutSubmitState,
} from "@/lib/payments/stripe-js/checkout";
import { readDxStripeClientConfig } from "@/lib/payments/stripe-js/config";
import {
  createDxStripeDashboardCheckoutRequest,
  createDxStripeDashboardMissingConfigReceipt,
  dxStripeDashboardCheckoutReadiness,
  dxStripeDashboardPlans,
  type DxStripeDashboardCheckoutReceipt,
  type DxStripeDashboardCheckoutRequest,
  type DxStripeDashboardPlan,
} from "@/lib/payments/stripe-js/dashboard-checkout";
import { dxStripeJsForgePackage } from "@/lib/payments/stripe-js/metadata";
import { DxInputField } from "@/lib/forms/react-hook-form/fields";
import {
  DxHookForm,
  useDxHookForm,
} from "@/lib/forms/react-hook-form/form";
import { createDxZodResolver } from "@/lib/forms/react-hook-form/resolver";

type PaymentBoundaryStatus =
  | {
      kind: "ready";
      message: string;
    }
  | {
      kind: "missing-config";
      message: string;
    };

const launchCheckoutContactSchema = z.object({
  email: z.email(),
  name: z.string().trim().min(2).max(80),
  organization: z.string().trim().max(120),
  message: z.string().trim().max(500),
});

export type LaunchCheckoutContactValues = z.infer<
  typeof launchCheckoutContactSchema
>;

export type LaunchPaymentStatusProps = {
  checkoutEndpoint?: string;
  workflowContext?: "launch-dashboard" | "route-billing";
  onCheckoutContact?: (
    values: LaunchCheckoutContactValues,
    checkoutMode: DxStripeCheckoutMode,
    request: DxStripeDashboardCheckoutRequest,
  ) => Promise<DxStripeCheckoutSubmitResponse>;
};

const launchCheckoutContactDefaults = {
  email: "",
  name: "",
  organization: "",
  message: "",
} satisfies LaunchCheckoutContactValues;

const idleSubmitState = {
  kind: "idle",
  message: "Submit contact details to request a real Stripe Checkout Session.",
} satisfies DxStripeCheckoutSubmitState;

const stripePublishableKeyEnv = "NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY";
const stripeBillingWorkflowReceiptPath =
  "examples/template/.dx/forge/receipts/2026-05-22-payments-stripe-js-billing-workflow.json";

type LaunchEmbeddedCheckoutRequest = {
  id: number;
  contact: LaunchCheckoutContactValues;
  plan: DxStripeDashboardCheckoutRequest["body"]["plan"];
};

type LaunchStripePreviewReceipt = {
  id: string;
  mode: DxStripeCheckoutMode;
  email: string;
  planId: DxStripeDashboardPlan["id"];
  priceEnv: string;
  status: "missing-config";
  message: string;
  dashboardReceipt: DxStripeDashboardCheckoutReceipt;
};

function readPaymentBoundaryStatus(): PaymentBoundaryStatus {
  try {
    const config = readDxStripeClientConfig();
    return {
      kind: "ready",
      message: `Stripe.js publishable key ${config.publishableKey.slice(0, 7)}... is available.`,
    };
  } catch (error) {
    return {
      kind: "missing-config",
      message: error instanceof Error ? error.message : "Stripe config is missing.",
    };
  }
}

function createLaunchStripePreviewReceipt(
  contact: LaunchCheckoutContactValues,
  mode: DxStripeCheckoutMode,
  plan: DxStripeDashboardPlan,
): LaunchStripePreviewReceipt {
  const dashboardRequest = createDxStripeDashboardCheckoutRequest({
    planId: plan.id,
    checkoutMode: mode,
    contact,
  });
  const dashboardReceipt =
    createDxStripeDashboardMissingConfigReceipt(dashboardRequest);

  return {
    id: dashboardReceipt.receiptId,
    mode,
    email: dashboardRequest.contact.email,
    planId: plan.id,
    priceEnv: plan.priceEnv,
    status: dashboardReceipt.status,
    message: dashboardReceipt.message,
    dashboardReceipt,
  };
}

function readLaunchDashboardPlan(
  planId: DxStripeDashboardPlan["id"],
): DxStripeDashboardPlan {
  const plan = dxStripeDashboardPlans.find((item) => item.id === planId);

  if (!plan) {
    throw new Error("Launch billing plan must be one of the app-owned Stripe plans.");
  }

  return plan;
}

function checkoutSubmitStateFromResponse(
  response: DxStripeCheckoutSubmitResponse,
): DxStripeCheckoutSubmitState {
  if (response.kind === "checkout-session") {
    return {
      kind: "success",
      message: response.message ?? "Checkout Session created.",
      checkoutUrl: response.url,
    };
  }

  if (response.kind === "embedded-checkout-session") {
    return {
      kind: "success",
      message: response.message ?? "Embedded Checkout Session created.",
    };
  }

  return {
    kind: "success",
    message: response.message,
  };
}

function LaunchCheckoutContactFields() {
  const form = useDxHookForm<LaunchCheckoutContactValues>();
  const messageError = form.formState.errors.message?.message;

  return (
    <div className="grid gap-3">
      <DxInputField<LaunchCheckoutContactValues, "email">
        name="email"
        type="email"
        label="Checkout email"
        autoComplete="email"
        placeholder="billing@example.com"
        description="Used by Stripe Checkout for receipts and payment recovery."
      />
      <DxInputField<LaunchCheckoutContactValues, "name">
        name="name"
        autoComplete="name"
        label="Contact name"
        placeholder="Avery Stone"
      />
      <DxInputField<LaunchCheckoutContactValues, "organization">
        name="organization"
        autoComplete="organization"
        label="Organization"
        placeholder="DX Studio"
      />
      <label
        className="grid gap-2 text-sm font-medium"
        htmlFor="checkout-message"
      >
        Checkout context
        <Textarea
          id="checkout-message"
          rows={3}
          maxLength={500}
          placeholder="Plan, billing owner, tax notes, or launch handoff context"
          aria-invalid={messageError ? true : undefined}
          aria-describedby={
            messageError ? "checkout-message-error" : "checkout-message-help"
          }
          {...form.register("message")}
        />
        <span
          id="checkout-message-help"
          className="text-xs text-muted-foreground"
        >
          This is metadata for your server route, not card data.
        </span>
        {typeof messageError === "string" ? (
          <span
            id="checkout-message-error"
            className="text-xs text-destructive"
            role="alert"
          >
            {messageError}
          </span>
        ) : null}
      </label>
    </div>
  );
}

function LaunchEmbeddedCheckoutMount({
  checkoutEndpoint,
  onStatusChange,
  request,
}: {
  checkoutEndpoint: string;
  onStatusChange: (state: DxStripeCheckoutSubmitState) => void;
  request: LaunchEmbeddedCheckoutRequest;
}) {
  const containerRef = React.useRef<HTMLDivElement | null>(null);

  React.useEffect(() => {
    let cancelled = false;
    let checkout:
      | Awaited<ReturnType<typeof createDxStripeEmbeddedCheckout>>
      | undefined;
    const fetchCheckoutSecret =
      createDxStripeEmbeddedCheckoutClientSecretFetcher({
        endpoint: checkoutEndpoint,
        contact: request.contact,
        plan: request.plan,
        source: "dx-www-dashboard",
      });

    onStatusChange({
      kind: "submitting",
      message: "Preparing Stripe Embedded Checkout.",
    });

    void createDxStripeEmbeddedCheckout({
      fetchClientSecret: fetchCheckoutSecret,
      onComplete: () => {
        onStatusChange({
          kind: "success",
          message:
            "Embedded Checkout completed. Verify fulfillment from your server.",
        });
      },
    })
      .then((embeddedCheckout) => {
        if (cancelled) {
          embeddedCheckout.destroy();
          return;
        }

        if (!containerRef.current) {
          embeddedCheckout.destroy();
          throw new Error("Embedded Checkout mount target is unavailable.");
        }

        checkout = embeddedCheckout;
        embeddedCheckout.mount(containerRef.current);
        onStatusChange({
          kind: "success",
          message: "Embedded Checkout mounted.",
        });
      })
      .catch((error) => {
        if (cancelled) return;

        onStatusChange({
          kind: "error",
          message:
            error instanceof Error
              ? error.message
              : "Embedded Checkout failed to mount.",
        });
      });

    return () => {
      cancelled = true;
      checkout?.destroy();
    };
  }, [checkoutEndpoint, onStatusChange, request]);

  return (
    <div
      ref={containerRef}
      className="min-h-96 rounded-md border bg-background p-3"
      data-dx-embedded-checkout-mount={request.id}
    />
  );
}

export function LaunchPaymentStatus({
  checkoutEndpoint = "/api/checkout",
  workflowContext = "launch-dashboard",
  onCheckoutContact,
}: LaunchPaymentStatusProps) {
  const [status, refresh] = React.useReducer(
    () => readPaymentBoundaryStatus(),
    undefined,
    readPaymentBoundaryStatus,
  );
  const [formSubmitState, setFormSubmitState] =
    React.useState<DxStripeCheckoutSubmitState>(idleSubmitState);
  const [checkoutMode, setCheckoutMode] =
    React.useState<DxStripeCheckoutMode>("hosted");
  const [selectedPlanId, setSelectedPlanId] =
    React.useState<DxStripeDashboardPlan["id"]>("team");
  const [embeddedCheckoutRequest, setEmbeddedCheckoutRequest] =
    React.useState<LaunchEmbeddedCheckoutRequest | null>(null);
  const [localReceipt, setLocalReceipt] =
    React.useState<LaunchStripePreviewReceipt | null>(null);
  const nextEmbeddedCheckoutId = React.useRef(0);
  const selectedPlan = React.useMemo(
    () => readLaunchDashboardPlan(selectedPlanId),
    [selectedPlanId],
  );

  const handleCheckoutContact = React.useCallback(
    async (values: LaunchCheckoutContactValues) => {
      const dashboardRequest = createDxStripeDashboardCheckoutRequest({
        planId: selectedPlan.id,
        checkoutMode,
        contact: values,
      });

      setFormSubmitState({
        kind: "submitting",
        message: `Requesting Stripe Checkout for ${selectedPlan.label} from the app-owned server route.`,
      });

      if (status.kind === "missing-config" && !onCheckoutContact) {
        const receipt = createLaunchStripePreviewReceipt(
          values,
          checkoutMode,
          selectedPlan,
        );

        setEmbeddedCheckoutRequest(null);
        setLocalReceipt(receipt);
        setFormSubmitState({
          kind: "success",
          message: receipt.message,
        });
        return;
      }

      if (checkoutMode === "embedded" && !onCheckoutContact) {
        setLocalReceipt(null);
        setEmbeddedCheckoutRequest({
          id: nextEmbeddedCheckoutId.current + 1,
          contact: values,
          plan: dashboardRequest.body.plan,
        });
        nextEmbeddedCheckoutId.current += 1;
        return;
      }

      try {
        setEmbeddedCheckoutRequest(null);
        setLocalReceipt(null);
        const response = onCheckoutContact
          ? await onCheckoutContact(values, checkoutMode, dashboardRequest)
            : await submitDxStripeCheckoutContact({
                endpoint: checkoutEndpoint,
                checkoutMode,
                contact: values,
                plan: dashboardRequest.body.plan,
                source: dashboardRequest.body.source,
              });
        setFormSubmitState(checkoutSubmitStateFromResponse(response));
      } catch (error) {
        setFormSubmitState({
          kind: "error",
          message:
            error instanceof Error
              ? error.message
              : "Checkout request failed.",
        });
      }
    },
    [checkoutEndpoint, checkoutMode, onCheckoutContact, selectedPlan, status.kind],
  );

  return (
    <div
      className="grid gap-4"
      data-dx-component="launch-billing-checkout-workflow"
      data-dx-package="payments/stripe-js"
      data-dx-dashboard-flow="billing-checkout"
      data-dx-style-surface="payments"
      data-dx-payment-status={status.kind}
      data-dx-stripe-config-state={status.kind}
      data-dx-stripe-dashboard-workflow="plan-checkout"
      data-dx-stripe-plan-id={selectedPlan.id}
      data-dx-stripe-price-env={selectedPlan.priceEnv}
      data-dx-stripe-publishable-key-env={stripePublishableKeyEnv}
      data-dx-stripe-local-receipt={localReceipt?.id ?? "none"}
      data-dx-stripe-receipt-path={stripeBillingWorkflowReceiptPath}
      data-dx-stripe-receipt-status={localReceipt?.status ?? "idle"}
      data-dx-stripe-submit-state={formSubmitState.kind}
      data-dx-workflow-context={workflowContext}
    >
      <div className="flex items-start justify-between gap-3">
        <div className="flex items-start gap-2">
          <dx-icon name="pack:payments" aria-hidden="true" className="mt-0.5 size-4" />
          <div>
            <p className="text-sm font-medium">Billing checkout</p>
            <p className="text-xs leading-5 text-muted-foreground">
              {dxStripeJsForgePackage.dashboardUsage.visibleWorkflow} uses a
              plan, checkout mode, and contact boundary before your server creates
              a Checkout Session.
            </p>
          </div>
        </div>
        <button
          className="rounded-md border px-3 py-2 text-xs font-medium"
          data-dx-stripe-action="refresh-config"
          type="button"
          onClick={refresh}
        >
          Recheck
        </button>
      </div>

      <div className="grid gap-2 rounded-md border p-3">
        <p className="text-xs text-muted-foreground">
          Required env: {dxStripeDashboardCheckoutReadiness.requiredEnv.join(", ")}
        </p>
        <p className="text-sm font-medium">{status.message}</p>
        <p className="text-xs leading-5 text-muted-foreground">
          Backend contract: create Checkout Sessions on your server, keep
          secret keys server-only, and return a real checkout URL or contact
          receipt.
        </p>
      </div>

      <div
        className="grid gap-3 rounded-md border p-3"
        data-dx-stripe-interaction="plan-picker"
      >
        <div className="flex flex-wrap items-end justify-between gap-2">
          <div>
            <p className="text-sm font-medium">Plan checkout</p>
            <p className="text-xs leading-5 text-muted-foreground">
              Pick the dashboard plan before preparing the Stripe-shaped
              checkout request. Price IDs stay app-owned env mappings.
            </p>
          </div>
          <span
            className="rounded-md bg-muted px-2 py-1 text-xs text-muted-foreground"
            data-dx-stripe-price-env={selectedPlan.priceEnv}
          >
            {selectedPlan.priceEnv}
          </span>
        </div>
        <div className="grid gap-2 md:grid-cols-3">
          {dxStripeDashboardPlans.map((plan) => (
            <button
              key={plan.id}
              type="button"
              className="grid gap-1 rounded-md border p-3 text-left text-sm"
              aria-pressed={selectedPlan.id === plan.id}
              data-dx-stripe-action="select-plan"
              data-dx-stripe-plan-id={plan.id}
              data-dx-stripe-plan-selected={
                selectedPlan.id === plan.id ? "true" : "false"
              }
              onClick={() => {
                setSelectedPlanId(plan.id);
                setLocalReceipt(null);
                setFormSubmitState(idleSubmitState);
              }}
            >
              <span className="font-medium">{plan.label}</span>
              <span className="text-xs text-muted-foreground">
                {plan.priceLabel}
              </span>
              <span className="text-xs leading-5 text-muted-foreground">
                {plan.description}
              </span>
            </button>
          ))}
        </div>
      </div>

      <DxHookForm<LaunchCheckoutContactValues>
        className="grid gap-4 rounded-md border p-3"
        options={{
          defaultValues: launchCheckoutContactDefaults,
          resolver: createDxZodResolver<
            LaunchCheckoutContactValues,
            LaunchCheckoutContactValues
          >(launchCheckoutContactSchema),
        }}
        onSubmit={handleCheckoutContact}
      >
        {({ isSubmitting }) => (
          <>
            <LaunchCheckoutContactFields />
            <fieldset
              className="grid gap-2"
              data-dx-checkout-mode={checkoutMode}
              data-dx-stripe-checkout-mode={checkoutMode}
            >
              <legend className="text-sm font-medium">Checkout mode</legend>
              <div className="grid gap-2 sm:grid-cols-2">
                {[
                  {
                    label: "Hosted page",
                    value: "hosted",
                    description: "Redirect to Stripe Checkout after the server creates the session.",
                  },
                  {
                    label: "Embedded form",
                    value: "embedded",
                    description: "Ask the same server route for Stripe's embedded Checkout session.",
                  },
                ].map((option) => (
                  <label
                    className="grid cursor-pointer gap-1 rounded-md border p-3 text-sm"
                    key={option.value}
                  >
                    <span className="flex items-center gap-2 font-medium">
                      <input
                        checked={checkoutMode === option.value}
                        name="checkout-mode"
                        onChange={() => {
                          const mode = option.value as DxStripeCheckoutMode;
                          setCheckoutMode(mode);
                          setLocalReceipt(null);

                          if (mode === "hosted") {
                            setEmbeddedCheckoutRequest(null);
                          }
                        }}
                        type="radio"
                        value={option.value}
                      />
                      {option.label}
                    </span>
                    <span className="text-xs leading-5 text-muted-foreground">
                      {option.description}
                    </span>
                  </label>
                ))}
              </div>
            </fieldset>
            <div className="flex flex-wrap items-center gap-3">
              <Button
                type="submit"
                disabled={isSubmitting || formSubmitState.kind === "submitting"}
                data-dx-stripe-action="request-checkout-intent"
              >
                {isSubmitting || formSubmitState.kind === "submitting"
                  ? "Requesting checkout"
                  : "Request checkout"}
              </Button>
              <p
                className="text-xs leading-5 text-muted-foreground"
                data-dx-checkout-submit-state={formSubmitState.kind}
                data-dx-stripe-submit-state={formSubmitState.kind}
              >
                {formSubmitState.message}
              </p>
              {localReceipt ? (
                <p
                  className="rounded-md bg-muted px-3 py-2 text-xs leading-5 text-muted-foreground"
                  data-dx-stripe-local-receipt={localReceipt.id}
                  role="status"
                >
                  Local preview receipt: {localReceipt.email} selected{" "}
                  {localReceipt.planId} via {localReceipt.mode} checkout using{" "}
                  {localReceipt.priceEnv}. No Checkout Session was created.
                </p>
              ) : null}
            </div>
            {formSubmitState.kind === "success" &&
            formSubmitState.checkoutUrl ? (
              <a
                className="w-fit text-xs font-medium underline underline-offset-4"
                href={formSubmitState.checkoutUrl}
              >
                Continue to Stripe Checkout
              </a>
            ) : null}
          </>
        )}
      </DxHookForm>
      {embeddedCheckoutRequest && checkoutMode === "embedded" ? (
        <LaunchEmbeddedCheckoutMount
          key={embeddedCheckoutRequest.id}
          checkoutEndpoint={checkoutEndpoint}
          onStatusChange={setFormSubmitState}
          request={embeddedCheckoutRequest}
        />
      ) : null}
    </div>
  );
}
