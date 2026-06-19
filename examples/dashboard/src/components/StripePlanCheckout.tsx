import { useState } from 'dx';
import {
    createDxStripeDashboardCheckoutRequest,
    createDxStripeDashboardMissingConfigReceipt,
    dxStripeDashboardCheckoutReadiness,
    dxStripeDashboardPlans,
    type DxStripeDashboardCheckoutMode,
    type DxStripeDashboardCheckoutReceipt,
} from '../lib/stripePlanCheckout';

type StripeSubmitState = 'idle' | 'submitting' | 'missing-config' | 'error';

export function StripePlanCheckout() {
    const [planId, setPlanId] = useState(dxStripeDashboardPlans[1].id);
    const [checkoutMode, setCheckoutMode] = useState<DxStripeDashboardCheckoutMode>('hosted');
    const [email, setEmail] = useState('billing@example.com');
    const [name, setName] = useState('DX Launch Operator');
    const [organization, setOrganization] = useState('DX WWW');
    const [submitState, setSubmitState] = useState<StripeSubmitState>('idle');
    const [receipt, setReceipt] = useState<DxStripeDashboardCheckoutReceipt | null>(null);
    const [error, setError] = useState('');

    const selectedPlan =
        dxStripeDashboardPlans.find((plan) => plan.id === planId) || dxStripeDashboardPlans[0];

    const requestCheckout = (event: Event) => {
        event.preventDefault();
        setSubmitState('submitting');
        setError('');

        try {
            const request = createDxStripeDashboardCheckoutRequest({
                planId: selectedPlan.id,
                checkoutMode,
                contact: {
                    email,
                    name,
                    organization,
                },
            });
            const localReceipt = createDxStripeDashboardMissingConfigReceipt(request);
            setReceipt(localReceipt);
            setSubmitState(localReceipt.status);
        } catch (checkoutError) {
            setReceipt(null);
            setSubmitState('error');
            setError(
                checkoutError instanceof Error
                    ? checkoutError.message
                    : 'Checkout request could not be prepared.',
            );
        }
    };

    return (
        <section
            class="stripe-checkout-panel"
            data-dx-package="payments/stripe-js"
            data-dx-component="dashboard-stripe-plan-checkout"
            data-dx-stripe-dashboard-workflow="plan-checkout"
            data-dx-stripe-config-state={receipt ? receipt.status : dxStripeDashboardCheckoutReadiness.status}
            data-dx-stripe-submit-state={submitState}
            data-dx-stripe-selected-plan={selectedPlan.id}
            data-dx-stripe-checkout-mode={checkoutMode}
            data-dx-stripe-local-receipt={receipt ? receipt.receiptId : "none"}
            data-dx-node-modules="forbidden"
        >
            <header class="panel-header">
                <dx-icon name="pack:payments" aria-label="Payments" />
                <div>
                    <h2>Stripe plan checkout</h2>
                    <p>Prepare a Checkout Session request with an app-owned credential boundary.</p>
                </div>
            </header>

            <dl class="readiness-list" data-dx-stripe-readiness="app-owned">
                <div>
                    <dt>Required env</dt>
                    <dd data-dx-stripe-required-env={dxStripeDashboardCheckoutReadiness.requiredEnv.join(',')}>
                        {dxStripeDashboardCheckoutReadiness.requiredEnv.join(', ')}
                    </dd>
                </div>
                <div>
                    <dt>Public APIs</dt>
                    <dd data-dx-stripe-public-api={dxStripeDashboardCheckoutReadiness.publicApi.join(',')}>
                        {dxStripeDashboardCheckoutReadiness.publicApi.join(', ')}
                    </dd>
                </div>
            </dl>

            <div class="provider-options" data-dx-stripe-interaction="plan-picker">
                {dxStripeDashboardPlans.map((plan) => (
                    <button
                        key={plan.id}
                        type="button"
                        class={plan.id === selectedPlan.id ? 'active' : ''}
                        data-dx-stripe-plan-id={plan.id}
                        data-dx-stripe-plan-selected={plan.id === selectedPlan.id ? 'true' : 'false'}
                        onClick={() => {
                            setPlanId(plan.id);
                            setReceipt(null);
                            setSubmitState('idle');
                        }}
                    >
                        <dx-icon name="pack:package" aria-hidden="true" />
                        {plan.label}
                    </button>
                ))}
            </div>

            <div class="query-box" data-dx-stripe-plan-summary={selectedPlan.id}>
                <strong>{selectedPlan.priceLabel}</strong>
                <span>{selectedPlan.description}</span>
                <span data-dx-stripe-price-env={selectedPlan.priceEnv}>
                    Price env: {selectedPlan.priceEnv}
                </span>
            </div>

            <form
                class="form-grid"
                data-dx-stripe-interaction="checkout-contact-form"
                onSubmit={requestCheckout}
            >
                <label>
                    Billing email
                    <input
                        value={email}
                        autocomplete="email"
                        data-dx-stripe-input="email"
                        onChange={(event) => setEmail((event.target as HTMLInputElement).value)}
                    />
                </label>
                <label>
                    Contact name
                    <input
                        value={name}
                        data-dx-stripe-input="name"
                        onChange={(event) => setName((event.target as HTMLInputElement).value)}
                    />
                </label>
                <label>
                    Organization
                    <input
                        value={organization}
                        data-dx-stripe-input="organization"
                        onChange={(event) =>
                            setOrganization((event.target as HTMLInputElement).value)
                        }
                    />
                </label>
                <div class="provider-options" data-dx-stripe-interaction="checkout-mode-picker">
                    <button
                        type="button"
                        class={checkoutMode === 'hosted' ? 'active' : ''}
                        data-dx-stripe-checkout-mode-option="hosted"
                        onClick={() => {
                            setCheckoutMode('hosted');
                            setReceipt(null);
                        }}
                    >
                        Hosted Checkout
                    </button>
                    <button
                        type="button"
                        class={checkoutMode === 'embedded' ? 'active' : ''}
                        data-dx-stripe-checkout-mode-option="embedded"
                        onClick={() => {
                            setCheckoutMode('embedded');
                            setReceipt(null);
                        }}
                    >
                        Embedded Checkout
                    </button>
                </div>
                <button
                    type="submit"
                    class="primary-action"
                    data-dx-stripe-action="request-checkout-session"
                    data-dx-stripe-submit-state={submitState}
                >
                    <dx-icon name="pack:play" aria-hidden="true" />
                    Prepare checkout request
                </button>
            </form>

            <p
                class="assistant-receipt"
                data-dx-stripe-local-result={receipt ? receipt.status : submitState}
                data-dx-stripe-local-receipt={receipt ? receipt.receiptId : "none"}
                role={submitState === 'error' ? 'alert' : undefined}
            >
                {receipt
                    ? `${receipt.receiptId}: ${receipt.message}`
                    : error || 'No checkout request has been prepared yet.'}
            </p>
        </section>
    );
}
