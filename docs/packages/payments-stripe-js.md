# Payments

Official DX package: Payments
official_package_name: Payments
package_id: payments/stripe-js
upstream_package: @stripe/stripe-js
source_mirror: G:/WWW/inspirations/stripe-js
upstream_version: 9.6.0
honesty_label: ADAPTER-BOUNDARY

Recommended command: `dx add payments --write`
aliases: payments, stripe-js, @stripe/stripe-js, stripe, payments/stripe

`payments/stripe-js` is the source-owned DX Forge Payments slice for Stripe.js browser payment wiring and server-owned Checkout Sessions handoff. The package imports real upstream APIs, materializes editable app files, and leaves credentials, pricing, fulfillment, and compliance policy app-owned.

## CLI Dispatch

The official user-facing install command is `dx add payments --write`. Forge canonicalizes `payments` to `payments/stripe-js`, while `stripe-js`, `stripe`, `@stripe/stripe-js`, and `payments/stripe` remain compatibility aliases. `core/src/ecosystem/forge_registry.rs` carries the source-owned registry fixture for that dispatch path, and `benchmarks/payments-dx-check-package-lane-panel.test.ts` guards the active docs, dashboard, CLI/template copy, and DX package registry summary against upstream-named command drift.

## Upstream Evidence

- `package.json` identifies `@stripe/stripe-js` version `9.6.0` under MIT.
- `README.md` documents `loadStripe`, the side-effect-free `@stripe/stripe-js/pure` import, and `loadStripe.setLoadParameters`.
- `src/pure.ts` implements deferred Stripe.js loading and guarded load-parameter changes.
- `src/shared.ts` registers the Stripe.js wrapper and shared script-loading behavior.
- `types/shared.d.ts` declares the `loadStripe` public signature.
- `types/stripe-js/stripe.d.ts` exposes `stripe.confirmPayment`, `stripe.retrievePaymentIntent`, and `stripe.createEmbeddedCheckoutPage`.
- `types/stripe-js/checkout.d.ts` exposes Embedded Checkout options, including `StripeEmbeddedCheckoutOptions.fetchClientSecret`.

## Public API Slice

- `readDxStripeClientConfig` reads publishable browser config and rejects leaked public secret values.
- `getDxStripe` and `requireDxStripe` load Stripe.js from `@stripe/stripe-js/pure`.
- `confirmDxStripePayment` calls `elements.submit()` before `stripe.confirmPayment`.
- `retrieveDxStripePaymentIntent` reads a server-created PaymentIntent client secret.
- `submitDxStripeCheckoutContact` posts validated contact and plan data to `/api/checkout`.
- `createDxStripeEmbeddedCheckoutClientSecretFetcher` adapts the app-owned checkout endpoint into Stripe.js `fetchClientSecret`.
- `createDxStripeEmbeddedCheckout` calls `stripe.createEmbeddedCheckoutPage`.
- `createDxStripeCheckoutSession`, `createDxStripeEmbeddedCheckoutSession`, `retrieveDxStripeCheckoutSession`, `createDxStripeBillingPortalSession`, `createDxStripeCustomer`, `listDxStripeCustomerSubscriptions`, and `verifyDxStripeWebhookRequest` are server-only helpers around the official Stripe server SDK boundary.

## Materialized Files

- `lib/payments/stripe-js/config.ts`
- `lib/payments/stripe-js/client.ts`
- `lib/payments/stripe-js/payment.ts`
- `lib/payments/stripe-js/checkout.ts`
- `lib/payments/stripe-js/dashboard-checkout.ts`
- `lib/payments/stripe-js/server.ts`
- `app/api/checkout/route.ts`
- `app/api/payments/stripe-js/readiness/route.ts`
- `app/api/stripe/webhook/route.ts`
- `lib/payments/stripe-js/metadata.ts`
- `lib/payments/stripe-js/README.md`
- `components/launch/payments-status.tsx`

## Dashboard Usage

The generated `/launch` template consumes Payments through `examples/template/payments-status.tsx`, not through a fake card-entry form. The visible workflow lets an operator select an app-owned plan, choose hosted or embedded Checkout, validate contact details with React Hook Form plus Zod, and prepare a safe missing-config receipt.

Stable markers include:

- `data-dx-package="payments/stripe-js"`
- `data-dx-component="launch-billing-checkout-workflow"`
- `data-dx-dashboard-flow="billing-checkout"`
- `data-dx-stripe-dashboard-workflow="plan-checkout"`
- `data-dx-stripe-action="select-plan"`
- `data-dx-stripe-action="request-checkout-intent"`
- `data-dx-stripe-receipt-path`
- `<dx-icon name="pack:payments" />`

The starter dashboard also consumes the same package through `examples/dashboard/src/components/StripePlanCheckout.tsx` and `examples/dashboard/src/lib/stripePlanCheckout.ts`, with hosted and embedded Checkout mode selection plus a safe local missing-config receipt.

Starter dashboard markers include `data-dx-component="dashboard-stripe-plan-checkout"` and `data-dx-stripe-action="request-checkout-session"`.

## Receipts And Check Visibility

- `examples/template/.dx/forge/receipts/2026-05-22-payments-stripe-js-billing-workflow.json`
- `.dx/forge/docs/payments-stripe-js.md`
- `.dx/forge/docs/dashboard-stripe-plan-checkout.md`
- `.dx/forge/receipts/*-payments-stripe-js.json`
- `examples/conversion-proof/public/preview-manifest.json#launch-runtime-billing-checkout`

Schema: `dx.forge.package.dx_check_visibility`

`dx-check` and Zed-facing package consumers should report the structured status legend as present, stale, missing-receipt, blocked, and unsupported-surface. Payments is `present` when the source files and receipt path exist, `stale` when hashes or selected surfaces drift, `missing-receipt` when the receipt path is absent, `blocked` when Stripe credentials or app-owned catalog values are absent, and `unsupported-surface` when a request asks Forge to generate a payment surface outside the selected Checkout, Payment Element, Billing Portal, webhook, or launch billing workflow scope. Human-facing copy may render `missing receipt` and `unsupported surface`, but receipts use the hyphenated machine labels.

Monitored surfaces:

- `payments-launch-billing-checkout-workflow`: `examples/template/payments-status.tsx` -> `components/launch/payments-status.tsx`
- `payments-checkout-session-route`: `core/src/ecosystem/forge_stripe_js.rs` -> `app/api/checkout/route.ts`
- `payments-readiness-route`: `examples/template/app/api/payments/stripe-js/readiness/route.ts`
- `payments-webhook-route`: `core/src/ecosystem/forge_stripe_js.rs` -> `app/api/stripe/webhook/route.ts`

## Shared Package Status

The shared package-status read model now consumes the Payments receipt through `paymentsPackageVisibility` in `examples/template/forge-package-status-read-model.ts` and exposes `paymentsVisibility` from `examples/template/forge-package-status.ts`. `examples/template/.dx/forge/package-status.json` includes the same `payments/stripe-js` row, `receipt_hash_refresh`, `payments_receipt_*` dx-check metrics, and Zed receipt surfaces for the launch billing workflow, Checkout Session route, readiness route, webhook route, and receipt hash refresh helper.

## DX Studio/check-panel Payments package row

The DX Studio/check-panel Payments package row now reads the shared package-status row from `core/src/ecosystem/dx_check_receipt.rs` and renders `receipt_hash_refresh` directly beside source hash and dx-style evidence. It carries `payments:receipt-hash-refresh`, `payments_hash_manifest_present`, `payments_hash_mismatch`, `payments_dx_style_compatibility_present`, and `payments_dx_style_compatibility_missing` so Studio and Zed can see helper freshness without opening package-status JSON and without claiming live Stripe Checkout or webhook runtime proof.

## Static /launch package-lane template

The receipt-less static `/launch` page now carries a hidden Payments package-lane template under `data-dx-component="dx-check-health-panel"` before `.dx/receipts/check/check-latest.json` exists. The marker exposes `data-dx-check-package-lane-template="payments/stripe-js"`, official package name `Payments`, upstream `@stripe/stripe-js` `9.6.0` provenance, the billing workflow receipt path, dx-style status, and the package-owned `payments:receipt-hash-refresh` helper with tracked/stale/missing file counts.

`examples/template/dx-studio-edit-contract.ts`, `tools/launch/materialize-www-template.ts`, and `dx-www/src/cli/studio_manifest.rs` include `payments/stripe-js` in the `dx-check-health-panel` package filter so DX Studio and Zed can discover stale helper state from static source/materialized manifests before a fresh dx-check receipt is loaded. This is SOURCE-ONLY visibility evidence; live Stripe Checkout redirects, webhook delivery, and payment runtime proof stay app-owned.

The generated-starter materialization guard for Payments runs the launch runtime materializer into a temporary starter and proves generated static launch HTML plus `public/preview-manifest.json` preserve the Payments package-lane row, receipt-hash helper markers, dx-style surface marker, and `launch-runtime-dx-check-panel` package scope without claiming live Stripe Checkout or webhook runtime proof.

The materialized `public/preview-manifest.json` now exposes the Payments runbook fixture through root `sourceGuardRunbookFixtures` metadata and the `/launch` route `routes[].sourceGuardRunbookFixtures` list. The manifest points to `docs/packages/payments.source-guard-runbook.json`, `payments-generated-starter-materialization`, `honestyLabel: "SOURCE-ONLY"`, `runtimeProof: false`, and `payments:receipt-hash-refresh` so Zed/DX Studio can connect generated preview metadata back to the package-owned runbook fixture without executing live Stripe runtime checks.

`app/api/payments/stripe-js/readiness/route.ts` gives the default template a non-executing provider-boundary route. GET reports configured env names, missing Price env coverage, `runtimeExecution: false`, `stripeLiveExecution: false`, and `secretValues: []`. POST validates the dashboard checkout request shape and emits a local DX/Zed readiness receipt without creating Checkout Sessions, reading secret values, redirecting users, or processing webhooks.

## Studio Source Guard Runbook

Payments Studio source-guard/runbook entry: `payments-generated-starter-materialization` is published through `dx-www/src/cli/studio_manifest.rs` in both `source_guard_index` and the `/launch` `source_guard_runbook_index`. The runbook command is `dx run --test .\benchmarks\payments-dx-check-package-lane-panel.test.ts`, and it proves the source-only generated-starter materialization, helper freshness markers, `data-dx-check-package-lane-row="payments/stripe-js"`, and `data-dx-token-scope="payments/stripe-js"` without claiming live Stripe Checkout or webhook runtime proof.

The same runbook now publishes the two source-only helper-freshness Rust fixtures as `payments-lower-dx-check-helper-freshness` and `payments-check-panel-helper-freshness`. The commands are `cargo test -q -p dx-www-compiler payments_hash_refresh_stale_helper_keeps_source_hash_clean --lib` and `cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_payments_stale_helper_without_source_hash_drift --lib`; both prove `payments_receipt_hash_refresh_*` metrics while keeping `payments_hash_mismatch` byte-derived and without claiming live Stripe Checkout or webhook runtime proof.

`docs/packages/payments.source-guard-runbook.json` is the package-owned JSON fixture for the same guards. It records the official package name, upstream Stripe.js provenance, inspected source files, public APIs used, exact guard commands, `/launch` `source_guard_runbook_index` contract, structured `source_guard_fixture_paths`, Zed/DX Studio markers, receipt hash helper, hash-backed `dx-www/src/cli/studio_manifest.rs` source, app-owned Stripe boundaries, and `SOURCE-ONLY` runtime limitations so tooling can read the Payments runbook contract without parsing raw Rust source.

The same fixture records its preview-manifest exposure contract: generated file `public/preview-manifest.json`, materializer `tools/launch/materialize-www-template.ts`, root field `sourceGuardRunbookFixtures`, route field `routes[].sourceGuardRunbookFixtures`, and `runtime_proof: false`.

## Rust dx-check output

`core/src/ecosystem/project_check/payments_dx_check.rs` consumes `.dx/forge/package-status.json` and emits the `payments_*` metrics into the Rust Forge `dx check` section. It reports `payments_package_present`, `payments_receipt_present`, `payments_receipt_stale`, `payments_missing_receipt`, `payments_blocked_surface`, and `payments_unsupported_surface` from the package-status row plus the billing workflow receipt path.

Payments helper freshness is now first-class Rust dx-check output through `payments_receipt_hash_refresh_current`, `payments_receipt_hash_refresh_stale`, and `payments_receipt_hash_refresh_missing`. The targetable fixture `payments_hash_refresh_stale_helper_keeps_source_hash_clean` flips only `receipt_hash_refresh.status` and `stale_file_count` while the selected SHA-256 file hash remains current, so helper drift is visible without confusing it with `payments_hash_mismatch` and without claiming live Stripe payment execution.

Finding codes are `payments-missing-package-status`, `payments-stale-receipt`, `payments-missing-receipt`, `payments-blocked-surface`, and `payments-unsupported-surface`. `payments-blocked-surface` is expected while Stripe credentials, Price IDs, webhook fulfillment, customer entitlement writes, and live Checkout execution remain app-owned, without claiming live Stripe payment execution.

## Hash Freshness

The billing workflow receipt carries `hash_algorithm: sha256`, `files`, and `file_hashes` for the selected Payments source files that back the launch billing workflow, Checkout Session route, webhook route, package docs, and `docs/packages/payments.source-guard-runbook.json`. Shared launch runtime outputs such as `the static launch runtime template` and `tools/launch/runtime-template/assets/launch-runtime.ts` are recorded as hash exclusions because multiple package lanes edit those generated surfaces; their freshness belongs to launch materialization receipts. The shared package-status row mirrors Payments-owned hashes onto each selected surface and publishes `payments_hash_manifest_present` plus `payments_hash_mismatch` so Rust dx-check can flag stale source evidence from current file bytes.

`examples/template/payments-receipt-hashes.ts` checks or refreshes the selected Payments SHA-256 hashes across the billing workflow receipt, shared package-status row, and typed launch read model. It now reports `source_guard_runbook_fixture: docs/packages/payments.source-guard-runbook.json`, `studio_manifest_source: dx-www/src/cli/studio_manifest.rs`, and tracks five files, making source-guard runbook fixture drift and Studio manifest declaration drift stale-detectable through `payments:receipt-hash-refresh`. It publishes `dx.forge.package.receipt_hash_refresh` for Zed/DX Studio visibility and does not run live Stripe Checkout or read Stripe secrets.

Hash freshness is still ADAPTER-BOUNDARY evidence. It proves the receipt matches selected source files; it does not prove live Stripe credentials, hosted Checkout redirects, webhook delivery, entitlement writes, or production billing policy.

## DX-Style Compatibility

The visible Payments launch billing workflow declares `data-dx-style-surface="payments"` and uses dx-style token classes such as `bg-background`, `bg-muted`, `text-muted-foreground`, `border`, and `text-destructive` instead of inline style objects or hardcoded hex colors. The billing workflow receipt and shared package-status row expose `dx.forge.package.dx_style_compatibility` with `styles/theme.css` as the token source and `styles/generated.css` as the generated CSS target.

Rust dx-check reports `payments_dx_style_compatibility_present` and `payments_dx_style_compatibility_missing`, and raises `payments-missing-dx-style-compatibility` when the Payments package-status row loses that compatibility evidence. This remains SOURCE-ONLY style evidence; live Stripe Checkout visual proof, browser QA, and app theme token review are app-owned.

## App-Owned Boundaries

- Stripe account setup, product catalog, Prices, tax, fraud, disputes, refunds, and compliance review.
- `NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY`, `STRIPE_SECRET_KEY`, `STRIPE_WEBHOOK_SECRET`, API version policy, and secret rotation.
- Authenticated customer lookup before hosted Checkout, embedded Checkout, Billing Portal, or subscription reads.
- Checkout success-page authorization, order lookup, persistence checks, webhook idempotency, and fulfillment.
- Entitlement mapping, billing interval copy, invoice lifecycle, cancellation policy, and production observability.
- Runtime dependency installation for `@stripe/stripe-js`, `stripe`, React, and Next.js.

## Source Guard

Run the narrow guards with:

```powershell
dx run --test .\benchmarks\payments-dx-check-visibility-receipt.test.ts
dx run --test .\benchmarks\payments-dx-check-package-lane-panel.test.ts
dx run --test .\benchmarks\payments-hash-receipt.test.ts
dx run --test .\benchmarks\payments-receipt-hash-refresh.test.ts
dx run --test .\benchmarks\payments-dx-style-compatibility.test.ts
dx run --test .\benchmarks\payments-stripe-js-package-doc.test.ts
dx run --test .\benchmarks\stripe-payment-launch-proof.test.ts
dx run --test .\benchmarks\stripe-rhf-checkout-flow.test.ts
node tools/launch/run-template-receipt-helper.js examples/template/payments-receipt-hashes.ts --check
node tools/launch/run-template-receipt-helper.js examples/template/payments-receipt-hashes.ts --write
node tools/launch/run-template-receipt-helper.js examples/template/payments-receipt-hashes.ts --check --json
cargo test -q -p dx-www-compiler payments_package_metrics_reports_missing_dx_style_compatibility --lib
cargo test -q -p dx-www-compiler payments_hash_refresh_stale_helper_keeps_source_hash_clean --lib
cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_payments_package_lane_hash_refresh_row --lib
cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_payments_stale_helper_without_source_hash_drift --lib
```

## Verdict

ADAPTER-BOUNDARY. Payments uses real upstream Stripe.js public APIs and server Checkout/Billing/Webhook boundaries, materializes editable app source, records provenance and receipt metadata, and keeps live credentials, external service calls, fulfillment, and policy proof app-owned until explicitly configured and verified.
