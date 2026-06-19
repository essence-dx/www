const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readRustRawStringConst(source, constName) {
  const prefix = `const ${constName}: &str = r#"`;
  const start = source.indexOf(prefix);
  assert.notStrictEqual(start, -1, `${constName} must exist`);

  const bodyStart = start + prefix.length;
  const end = source.indexOf('"#;', bodyStart);
  assert.notStrictEqual(end, -1, `${constName} must use a raw string body`);

  return source.slice(bodyStart, end);
}

test("Stripe package exposes env-safe checkout/contact helpers", () => {
  const stripeSource = read("core/src/ecosystem/forge_stripe_js.rs");
  const securitySource = read("core/src/ecosystem/forge_security.rs");
  const configSource = readRustRawStringConst(stripeSource, "STRIPE_CONFIG_TS");
  const paymentSource = readRustRawStringConst(stripeSource, "STRIPE_PAYMENT_TS");
  const checkoutSource = readRustRawStringConst(stripeSource, "STRIPE_CHECKOUT_TS");
  const dashboardCheckoutSource = readRustRawStringConst(
    stripeSource,
    "STRIPE_DASHBOARD_CHECKOUT_TS",
  );
  const serverSource = readRustRawStringConst(stripeSource, "STRIPE_SERVER_TS");
  const routeSource = readRustRawStringConst(stripeSource, "STRIPE_ROUTE_TS");
  const webhookRouteSource = readRustRawStringConst(
    stripeSource,
    "STRIPE_WEBHOOK_ROUTE_TS",
  );
  const metadataSource = readRustRawStringConst(stripeSource, "STRIPE_METADATA_TS");
  const readmeSource = readRustRawStringConst(stripeSource, "STRIPE_README_MD");

  for (const marker of [
    "STRIPE_CHECKOUT_TS",
    "STRIPE_SERVER_TS",
    "assertNoPublicStripeSecrets",
    "readDxStripeServerConfig",
    "createDxStripeServerClient",
    "createDxStripeCustomer",
    "listDxStripeCustomerSubscriptions",
    "createDxStripeCheckoutSession",
    "createDxStripeEmbeddedCheckoutSession",
    "createDxStripeBillingPortalSession",
    "retrieveDxStripeCheckoutSession",
    "createDxStripePaymentIntent",
    "retrieveDxStripePaymentIntent",
    "verifyDxStripeWebhookEvent",
    "verifyDxStripeWebhookRequest",
    "submitDxStripeCheckoutContact",
    "createDxStripeEmbeddedCheckoutClientSecretFetcher",
    "createDxStripeEmbeddedCheckout",
    "STRIPE_DASHBOARD_CHECKOUT_TS",
    "createDxStripeDashboardCheckoutRequest",
    "createDxStripeDashboardMissingConfigReceipt",
    "DxStripeCheckoutContact",
    "server-only",
    "STRIPE_SECRET_KEY",
    "NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY",
    "js/app/api/checkout/route.ts",
    "js/app/api/stripe/webhook/route.ts",
  ]) {
    assert.match(stripeSource, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(configSource, /Object\.entries\(env\)\.find/);
  assert.match(configSource, /name\.startsWith\("NEXT_PUBLIC_"\)/);
  assert.match(configSource, /isStripeSecretValue\(readStripeEnvValue\(value\)\)/);
  assert.match(configSource, /value\?\.startsWith\("sk_"\)/);
  assert.match(configSource, /value\?\.startsWith\("rk_"\)/);
  assert.match(configSource, /value\?\.startsWith\("whsec_"\)/);
  assert.match(configSource, /readStripeEnvString\(\s*env,\s*"NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY"/);
  assert.match(configSource, /readStripeAccountId\(env\)/);
  assert.match(configSource, /readStripeEnvString\(env, "NEXT_PUBLIC_STRIPE_ACCOUNT"\)/);
  assert.match(configSource, /!account\.startsWith\("acct_"\)/);
  assert.match(configSource, /NEXT_PUBLIC_STRIPE_ACCOUNT must be a Stripe account ID/);
  assert.match(configSource, /advancedFraudSignals: readStripeFraudSignals\(env\)/);
  assert.match(configSource, /function readStripeFraudSignals\(/);
  assert.match(configSource, /NEXT_PUBLIC_STRIPE_ADVANCED_FRAUD_SIGNALS/);
  assert.match(configSource, /value === "true"/);
  assert.match(configSource, /value === "false"/);
  assert.match(configSource, /must be true or false/);
  assert.match(configSource, /readStripeApiVersion\(env, "NEXT_PUBLIC_STRIPE_API_VERSION"\)/);
  assert.match(configSource, /STRIPE_API_VERSION_PATTERN/);
  assert.match(configSource, /must be a Stripe API version like 2026-02-25\.clover/);
  assert.match(paymentSource, /readDxStripePaymentReturnUrl\(intent\.returnUrl\)/);
  assert.match(paymentSource, /new URL\(trimmed\)/);
  assert.match(paymentSource, /url\.protocol === "https:"/);
  assert.match(paymentSource, /url\.protocol === "http:" && isLocalStripeReturnHost\(url\.hostname\)/);
  assert.match(paymentSource, /Stripe payment returnUrl must use HTTPS outside localhost/);
  assert.match(paymentSource, /const submitResult = await elements\.submit\(\)/);
  assert.match(paymentSource, /if \(submitResult\.error\)/);
  assert.match(paymentSource, /return \{\s*error: submitResult\.error,\s*\}/);
  assert.match(paymentSource, /stripeClient\.confirmPayment\(/);
  assert.match(paymentSource, /export type RetrieveDxStripePaymentIntentInput/);
  assert.match(paymentSource, /export async function retrieveDxStripePaymentIntent\(/);
  assert.match(paymentSource, /const normalizedClientSecret = readDxStripeClientSecret\(clientSecret\)/);
  assert.match(paymentSource, /stripeClient\.retrievePaymentIntent\(normalizedClientSecret\)/);
  assert.match(paymentSource, /const normalizedClientSecret = readDxStripeClientSecret\(clientSecret\)/);
  assert.match(paymentSource, /clientSecret: normalizedClientSecret/);
  assert.match(paymentSource, /readDxStripeClientSecret\(intent\.clientSecret\)/);
  assert.match(paymentSource, /!clientSecret\.startsWith\("pi_"\)/);
  assert.match(paymentSource, /!clientSecret\.includes\("_secret_"\)/);
  assert.match(paymentSource, /Stripe PaymentIntent client secret must come from your server-created PaymentIntent/);
  assert.doesNotMatch(checkoutSource, /kind: "payment-intent"/);
  assert.match(checkoutSource, /export type DxStripeCheckoutMode = "hosted" \| "embedded"/);
  assert.match(checkoutSource, /export type DxStripeCheckoutPlanSelection/);
  assert.match(checkoutSource, /kind: "embedded-checkout-session"/);
  assert.match(checkoutSource, /clientSecret: string/);
  assert.match(checkoutSource, /checkoutMode\?: DxStripeCheckoutMode/);
  assert.match(checkoutSource, /plan\?: DxStripeCheckoutPlanSelection/);
  assert.match(checkoutSource, /checkoutMode: checkoutMode \?\? "hosted"/);
  assert.match(checkoutSource, /const planSelection = readDxStripeCheckoutPlanSelection\(plan\)/);
  assert.match(checkoutSource, /plan: planSelection/);
  assert.match(checkoutSource, /function readDxStripeCheckoutPlanSelection/);
  assert.match(
    checkoutSource,
    /export type CreateDxStripeEmbeddedCheckoutClientSecretFetcherOptions/,
  );
  assert.match(
    checkoutSource,
    /export function createDxStripeEmbeddedCheckoutClientSecretFetcher\(/,
  );
  assert.match(checkoutSource, /return async \(\) => \{/);
  assert.match(checkoutSource, /checkoutMode: "embedded"/);
  assert.match(checkoutSource, /const body = await readJsonResponse\(response\)/);
  assert.match(checkoutSource, /const checkoutResponse = parseDxStripeCheckoutResponse\(body\)/);
  assert.match(checkoutSource, /checkoutResponse\.kind !== "embedded-checkout-session"/);
  assert.match(
    checkoutSource,
    /Embedded Checkout endpoint must return an embedded Checkout Session/,
  );
  assert.match(
    checkoutSource,
    /return readDxStripeCheckoutClientSecret\(checkoutResponse\.clientSecret\)/,
  );
  assert.match(checkoutSource, /readCheckoutSessionClientSecret\(value\)/);
  assert.match(checkoutSource, /Checkout Session response requires sessionId and clientSecret/);
  assert.match(checkoutSource, /function readCheckoutSessionClientSecret/);
  assert.match(checkoutSource, /readOptionalString\(value, "sessionId"\)/);
  assert.match(checkoutSource, /readCheckoutSessionUrl\(value\)/);
  assert.match(checkoutSource, /Checkout Session response requires sessionId and HTTPS url/);
  assert.match(checkoutSource, /new URL\(value\)/);
  assert.match(checkoutSource, /url\.protocol !== "https:"/);
  assert.match(checkoutSource, /Checkout Session url must use HTTPS/);
  assert.match(checkoutSource, /Checkout contact response requires a message/);
  assert.match(checkoutSource, /const checkoutEndpoint = readDxStripeCheckoutEndpoint\(endpoint\)/);
  assert.match(checkoutSource, /fetcher\(checkoutEndpoint,/);
  assert.match(checkoutSource, /import type \{\s*Stripe,\s*StripeEmbeddedCheckout,\s*StripeEmbeddedCheckoutOptions,\s*\} from "@stripe\/stripe-js"/);
  assert.match(checkoutSource, /export type DxStripeEmbeddedCheckoutClientSecretSource/);
  assert.match(checkoutSource, /clientSecret: string;\s*fetchClientSecret\?: never/);
  assert.match(checkoutSource, /clientSecret\?: never;\s*fetchClientSecret: \(\) => Promise<string>/);
  assert.match(checkoutSource, /export type CreateDxStripeEmbeddedCheckoutInput/);
  assert.match(checkoutSource, /export async function createDxStripeEmbeddedCheckout\(/);
  assert.match(checkoutSource, /const source = readDxStripeEmbeddedCheckoutClientSecretSource\(\{\s*clientSecret,\s*fetchClientSecret,\s*\}\)/);
  assert.match(checkoutSource, /stripeClient\.createEmbeddedCheckoutPage\(/);
  assert.match(checkoutSource, /\.\.\.source/);
  assert.match(checkoutSource, /function readDxStripeEmbeddedCheckoutClientSecretSource\(/);
  assert.match(checkoutSource, /clientSecret !== undefined/);
  assert.match(checkoutSource, /fetchClientSecret !== undefined/);
  assert.match(checkoutSource, /fetchClientSecret: async \(\) =>\s*readDxStripeCheckoutClientSecret\(await fetchClientSecret\(\)\)/);
  assert.match(checkoutSource, /Stripe Embedded Checkout accepts either clientSecret or fetchClientSecret, not both/);
  assert.match(checkoutSource, /Stripe Embedded Checkout requires either clientSecret or fetchClientSecret/);
  assert.match(checkoutSource, /function readDxStripeCheckoutClientSecret\(/);
  assert.match(checkoutSource, /typeof value !== "string"/);
  assert.match(checkoutSource, /!clientSecret\.startsWith\("cs_"\)/);
  assert.match(checkoutSource, /!clientSecret\.includes\("_secret_"\)/);
  assert.match(checkoutSource, /Stripe Embedded Checkout client secret must come from your server-created Checkout Session/);
  assert.match(checkoutSource, /function readDxStripeCheckoutEndpoint\(endpoint: string\)/);
  assert.match(checkoutSource, /trimmed\.startsWith\("\/"\) && !trimmed\.startsWith\("\/\/"\)/);
  assert.match(checkoutSource, /url\.protocol === "https:"/);
  assert.match(checkoutSource, /Checkout endpoint must be a same-origin path or HTTPS URL/);
  assert.match(dashboardCheckoutSource, /createDxStripeCheckoutContactPayload/);
  assert.match(dashboardCheckoutSource, /type DxStripeCheckoutMode/);
  assert.match(dashboardCheckoutSource, /export const dxStripeDashboardPlans/);
  assert.match(dashboardCheckoutSource, /export function createDxStripeDashboardCheckoutRequest/);
  assert.match(dashboardCheckoutSource, /export function createDxStripeDashboardMissingConfigReceipt/);
  assert.match(dashboardCheckoutSource, /stripe-dashboard-local-/);
  assert.match(dashboardCheckoutSource, /source: "dx-www-dashboard"/);
  assert.match(dashboardCheckoutSource, /STRIPE_PRICE_ID/);
  assert.match(serverSource, /readStripeCheckoutRedirectUrl\(input\.successUrl, "successUrl"\)/);
  assert.match(serverSource, /readStripeCheckoutRedirectUrl\(input\.cancelUrl, "cancelUrl"\)/);
  assert.match(serverSource, /url\.protocol === "http:"[\s\S]*isLocalStripeCheckoutRedirectHost\(url\.hostname\)/);
  assert.match(serverSource, /Stripe checkout \$\{label\} must use HTTPS outside localhost/);
  assert.match(serverSource, /const lineItems = readDxStripeCheckoutLineItems\(input\.lineItems\)/);
  assert.match(serverSource, /customerId\?: string/);
  assert.match(serverSource, /customerUpdate\?: Stripe\.Checkout\.SessionCreateParams\["customer_update"\]/);
  assert.match(serverSource, /allowPromotionCodes\?: boolean/);
  assert.match(serverSource, /\.\.\.toDxStripeCheckoutCustomerParams\(contact, input\)/);
  assert.match(serverSource, /allow_promotion_codes: input\.allowPromotionCodes/);
  assert.match(serverSource, /function toDxStripeCheckoutCustomerParams/);
  assert.match(serverSource, /input: Pick<CreateDxStripeCheckoutSessionInput, "customerId" \| "customerUpdate">/);
  assert.match(serverSource, /const customerId = readOptionalDxStripeCustomerId\(input\.customerId\)/);
  assert.match(serverSource, /customer: customerId/);
  assert.match(serverSource, /customer_email: undefined/);
  assert.match(serverSource, /customer_update: input\.customerUpdate/);
  assert.match(serverSource, /customer_email: contact\.email/);
  assert.match(serverSource, /function readOptionalDxStripeCustomerId/);
  assert.match(serverSource, /return readDxStripeCustomerId\(value\)/);
  assert.match(serverSource, /line_items: lineItems/);
  assert.match(serverSource, /export type CreateDxStripeEmbeddedCheckoutSessionInput = Omit</);
  assert.match(serverSource, /"successUrl" \| "cancelUrl"/);
  assert.match(serverSource, /returnUrl: string/);
  assert.match(serverSource, /redirectOnCompletion\?: Stripe\.Checkout\.SessionCreateParams\["redirect_on_completion"\]/);
  assert.match(serverSource, /export async function createDxStripeEmbeddedCheckoutSession\(/);
  assert.match(serverSource, /ui_mode: "embedded"/);
  assert.match(serverSource, /return_url: returnUrl/);
  assert.match(serverSource, /redirect_on_completion: input\.redirectOnCompletion \?\? "if_required"/);
  assert.match(serverSource, /if \(!session\.client_secret\)/);
  assert.match(serverSource, /Stripe Embedded Checkout Session did not include a client secret/);
  assert.match(serverSource, /export type CreateDxStripeBillingPortalSessionInput/);
  assert.match(serverSource, /customerId: string/);
  assert.match(serverSource, /returnUrl: string/);
  assert.match(serverSource, /configuration\?: string/);
  assert.match(serverSource, /flowData\?: Stripe\.BillingPortal\.SessionCreateParams\["flow_data"\]/);
  assert.match(serverSource, /locale\?: Stripe\.BillingPortal\.SessionCreateParams\["locale"\]/);
  assert.match(serverSource, /export async function createDxStripeBillingPortalSession\(/);
  assert.match(serverSource, /stripe\.billingPortal\.sessions\.create\(/);
  assert.match(serverSource, /customer: readDxStripeCustomerId\(input\.customerId\)/);
  assert.match(serverSource, /return_url: readStripeCheckoutRedirectUrl\(input\.returnUrl, "returnUrl"\)/);
  assert.match(serverSource, /configuration: readDxStripeBillingPortalConfiguration\(input\.configuration\)/);
  assert.match(serverSource, /flow_data: input\.flowData/);
  assert.match(serverSource, /locale: input\.locale/);
  assert.match(serverSource, /function readDxStripeCustomerId/);
  assert.match(serverSource, /!customerId\.startsWith\("cus_"\)/);
  assert.match(serverSource, /Stripe customer ID must start with cus_/);
  assert.match(serverSource, /function readDxStripeBillingPortalConfiguration/);
  assert.match(serverSource, /!configuration\.startsWith\("bpc_"\)/);
  assert.match(serverSource, /function readDxStripeCheckoutLineItems\(/);
  assert.match(serverSource, /Array\.isArray\(lineItems\)/);
  assert.match(serverSource, /return lineItems\.map\(readDxStripeCheckoutLineItem\)/);
  assert.match(serverSource, /function readDxStripeCheckoutLineItem\(/);
  assert.match(serverSource, /typeof lineItem\.price === "string"/);
  assert.match(serverSource, /const price = lineItem\.price\.trim\(\)/);
  assert.match(serverSource, /!price\.startsWith\("price_"\)/);
  assert.match(serverSource, /requires a Stripe Price ID/);
  assert.match(serverSource, /lineItem\.price_data/);
  assert.match(serverSource, /Number\.isInteger\(lineItem\.quantity\)/);
  assert.match(serverSource, /quantity must be a positive integer/);
  assert.match(serverSource, /Stripe Checkout requires at least one line item/);
  assert.match(serverSource, /readDxStripeServerEnvString\(env, "STRIPE_SECRET_KEY"\)/);
  assert.match(serverSource, /readDxStripeWebhookSecret\(env\)/);
  assert.match(serverSource, /STRIPE_WEBHOOK_SECRET must be a Stripe webhook signing secret/);
  assert.match(serverSource, /export type RetrieveDxStripeCheckoutSessionInput/);
  assert.match(serverSource, /export async function retrieveDxStripeCheckoutSession\(/);
  assert.match(serverSource, /stripe\.checkout\.sessions\.retrieve\(/);
  assert.match(serverSource, /readDxStripeCheckoutSessionId\(input\.sessionId\)/);
  assert.match(serverSource, /expand: input\.expand/);
  assert.match(serverSource, /function readDxStripeCheckoutSessionId\(/);
  assert.match(serverSource, /!sessionId\.startsWith\("cs_"\)/);
  assert.match(serverSource, /Stripe Checkout Session ID must start with cs_/);
  assert.match(serverSource, /export type CreateDxStripePaymentIntentInput/);
  assert.match(serverSource, /export async function createDxStripePaymentIntent\(/);
  assert.match(serverSource, /stripe\.paymentIntents\.create\(/);
  assert.match(serverSource, /amount: readDxStripePaymentIntentAmount\(input\.amount\)/);
  assert.match(serverSource, /currency: readDxStripePaymentIntentCurrency\(input\.currency\)/);
  assert.match(serverSource, /automatic_payment_methods: input\.automaticPaymentMethods \?\? \{\s*enabled: true,\s*\}/);
  assert.match(serverSource, /receipt_email: readDxStripePaymentIntentReceiptEmail\(input\.receiptEmail\)/);
  assert.match(serverSource, /metadata: stripEmptyMetadata\(input\.metadata\)/);
  assert.match(serverSource, /if \(!paymentIntent\.client_secret\)/);
  assert.match(serverSource, /Stripe PaymentIntent did not include a client secret/);
  assert.match(serverSource, /Number\.isInteger\(amount\)/);
  assert.match(serverSource, /PaymentIntent amount must be a positive integer/);
  assert.match(serverSource, /amount > 99999999/);
  assert.match(serverSource, /PaymentIntent amount must be 8 digits or fewer/);
  assert.match(serverSource, /currency !== currency\.toLowerCase\(\)/);
  assert.match(serverSource, /PaymentIntent currency must be a lowercase three-letter ISO currency code/);
  assert.match(serverSource, /export type VerifyDxStripeWebhookEventInput/);
  assert.match(serverSource, /export function verifyDxStripeWebhookEvent\(/);
  assert.match(serverSource, /export async function verifyDxStripeWebhookRequest\(/);
  assert.match(serverSource, /request\.headers\.get\("stripe-signature"\)/);
  assert.match(serverSource, /await request\.text\(\)/);
  assert.match(serverSource, /stripeClient\.webhooks\.constructEvent\(/);
  assert.match(serverSource, /readStripeWebhookPayload\(input\.payload\)/);
  assert.match(serverSource, /readStripeWebhookSignature\(input\.signature\)/);
  assert.match(serverSource, /readConfiguredWebhookSecret\(config\)/);
  assert.match(serverSource, /STRIPE_WEBHOOK_SECRET is required to verify Stripe webhook events/);
  assert.match(serverSource, /Stripe webhook raw request body is required/);
  assert.match(serverSource, /Stripe-Signature header is required/);
  assert.match(serverSource, /readDxStripeApiVersion\(env, "STRIPE_API_VERSION"\)/);
  assert.match(serverSource, /STRIPE_API_VERSION_PATTERN/);
  assert.match(serverSource, /must be a Stripe API version like 2026-02-25\.clover/);
  assert.match(serverSource, /readDxStripeAppUrl\(env\)/);
  assert.match(serverSource, /normalizeStripeAppUrl\(value: string \| undefined\)/);
  assert.match(serverSource, /isLocalStripeAppHost\(url\.hostname\)/);
  assert.match(serverSource, /Stripe app URL must use HTTPS outside localhost/);
  assert.match(serverSource, /export type CreateDxStripeCustomerInput/);
  assert.match(serverSource, /email\?: string/);
  assert.match(serverSource, /name\?: string/);
  assert.match(serverSource, /metadata\?: Record<string, string \| undefined>/);
  assert.match(serverSource, /export async function createDxStripeCustomer\(/);
  assert.match(serverSource, /const email = readDxStripeCustomerEmail\(input\.email\)/);
  assert.match(serverSource, /const name = readDxStripeCustomerName\(input\.name\)/);
  assert.match(serverSource, /const metadata = stripEmptyMetadata\(input\.metadata\)/);
  assert.match(serverSource, /Object\.keys\(metadata\)\.length === 0/);
  assert.match(serverSource, /Stripe customer creation requires email, name, or metadata/);
  assert.match(serverSource, /return stripe\.customers\.create\(\{/);
  assert.match(serverSource, /email,\s*name,\s*metadata,/);
  assert.match(serverSource, /function readDxStripeCustomerEmail/);
  assert.match(serverSource, /Stripe customer email must be a valid email address/);
  assert.match(serverSource, /function readDxStripeCustomerName/);
  assert.match(serverSource, /Stripe customer name must be 120 characters or fewer/);
  assert.match(serverSource, /export type ListDxStripeCustomerSubscriptionsInput/);
  assert.match(serverSource, /status\?: Stripe\.SubscriptionListParams\["status"\]/);
  assert.match(serverSource, /limit\?: number/);
  assert.match(serverSource, /expand\?: Stripe\.SubscriptionListParams\["expand"\]/);
  assert.match(serverSource, /export async function listDxStripeCustomerSubscriptions\(/);
  assert.match(serverSource, /return stripe\.subscriptions\.list\(\{/);
  assert.match(serverSource, /customer: readDxStripeCustomerId\(input\.customerId\)/);
  assert.match(serverSource, /status: input\.status/);
  assert.match(serverSource, /limit: readDxStripeListLimit\(input\.limit, "subscription limit"\)/);
  assert.match(serverSource, /expand: input\.expand/);
  assert.match(serverSource, /function readDxStripeListLimit/);
  assert.match(serverSource, /Stripe subscription limit must be a positive integer/);
  assert.match(serverSource, /Stripe subscription limit must be 100 or fewer/);
  assert.match(routeSource, /export async function POST\(request: Request\)/);
  assert.match(routeSource, /export const runtime = "nodejs"/);
  assert.match(routeSource, /export const dynamic = "force-dynamic"/);
  assert.match(routeSource, /createDxStripeCheckoutContactPayload\(\s*readCheckoutContact\(body\),\s*\)/);
  assert.match(routeSource, /readDxStripeServerConfig\(\)/);
  assert.match(routeSource, /createDxStripeServerClient\(config\)/);
  assert.match(routeSource, /createDxStripeCheckoutSession\(/);
  assert.match(routeSource, /createDxStripeEmbeddedCheckoutSession\(/);
  assert.match(routeSource, /const checkoutMode = readCheckoutMode\(body\)/);
  assert.match(routeSource, /const checkoutPlan = readCheckoutPlan\(body\)/);
  assert.match(routeSource, /const priceId = readStripePriceId\(checkoutPlan\)/);
  assert.match(routeSource, /const STRIPE_ROUTE_PLAN_PRICE_ENVS/);
  assert.match(routeSource, /STRIPE_PRICE_ID_STARTER/);
  assert.match(routeSource, /process\.env\[checkoutPlan\.priceEnv\]\?\.trim\(\)/);
  assert.match(routeSource, /if \(checkoutMode === "embedded"\)/);
  assert.match(routeSource, /returnUrl: buildStripeRouteUrl\(\s*appUrl,\s*"\/checkout\/return\?session_id=\{CHECKOUT_SESSION_ID\}",\s*\)/);
  assert.match(routeSource, /kind: "embedded-checkout-session"/);
  assert.match(routeSource, /clientSecret: session\.client_secret/);
  assert.match(routeSource, /function readCheckoutMode/);
  assert.match(routeSource, /function readCheckoutPlan/);
  assert.match(routeSource, /metadata: createCheckoutMetadata\(checkoutPlan\)/);
  assert.match(routeSource, /dx_plan_id: checkoutPlan\?\.id/);
  assert.match(routeSource, /dx_price_env: checkoutPlan\?\.priceEnv/);
  assert.match(routeSource, /readCheckoutAppUrl\(config, request\)/);
  assert.match(routeSource, /if \(config\.appUrl\)/);
  assert.match(routeSource, /isLocalCheckoutRequestHost\(url\.hostname\)/);
  assert.match(routeSource, /NEXT_PUBLIC_APP_URL, APP_URL, or VERCEL_URL is required before creating Stripe Checkout outside localhost/);
  assert.match(routeSource, /process\.env\.STRIPE_PRICE_ID\?\.trim\(\)/);
  assert.match(routeSource, /if \(!priceId\.startsWith\("price_"\)\)/);
  assert.match(routeSource, /STRIPE_PRICE_ID must be a Stripe Price ID/);
  assert.match(routeSource, /message\.includes\("STRIPE_PRICE_ID"\)/);
  assert.match(routeSource, /status: 202/);
  assert.match(routeSource, /Set STRIPE_PRICE_ID to create live Stripe Checkout Sessions/);
  assert.match(routeSource, /\{CHECKOUT_SESSION_ID\}/);
  assert.match(webhookRouteSource, /verifyDxStripeWebhookRequest/);
  assert.match(webhookRouteSource, /export const runtime = "nodejs"/);
  assert.match(webhookRouteSource, /export const dynamic = "force-dynamic"/);
  assert.match(webhookRouteSource, /verifyDxStripeWebhookRequest\(request\)/);
  assert.match(webhookRouteSource, /event\.type/);
  assert.match(webhookRouteSource, /event\.id/);
  assert.match(webhookRouteSource, /checkout\.session\.completed/);
  assert.match(webhookRouteSource, /checkout\.session\.expired/);
  assert.match(webhookRouteSource, /payment_intent\.succeeded/);
  assert.match(webhookRouteSource, /payment_intent\.payment_failed/);
  assert.match(webhookRouteSource, /fulfillmentStatus: "app-owned"/);
  assert.match(webhookRouteSource, /return Response\.json\(\{\s*received: true/);
  assert.match(webhookRouteSource, /status: 400/);
  assert.doesNotMatch(webhookRouteSource, /request\.json\(\)/);
  assert.match(metadataSource, /"verifyDxStripeWebhookEvent"/);
  assert.match(metadataSource, /"verifyDxStripeWebhookRequest"/);
  assert.match(metadataSource, /"elements\.submit"/);
  assert.match(metadataSource, /"createDxStripeCustomer"/);
  assert.match(metadataSource, /"listDxStripeCustomerSubscriptions"/);
  assert.match(metadataSource, /"createDxStripeEmbeddedCheckoutSession"/);
  assert.match(metadataSource, /"createDxStripeBillingPortalSession"/);
  assert.match(metadataSource, /"retrieveDxStripeCheckoutSession"/);
  assert.match(metadataSource, /"stripe\.retrievePaymentIntent"/);
  assert.match(metadataSource, /"createDxStripePaymentIntent"/);
  assert.match(metadataSource, /"retrieveDxStripePaymentIntent"/);
  assert.match(metadataSource, /"stripe\.createEmbeddedCheckoutPage"/);
  assert.match(metadataSource, /"StripeEmbeddedCheckoutOptions.fetchClientSecret"/);
  assert.match(metadataSource, /"createDxStripeEmbeddedCheckoutClientSecretFetcher"/);
  assert.match(metadataSource, /"createDxStripeEmbeddedCheckout"/);
  assert.match(metadataSource, /paymentSubmitGuard: "elements\.submit\(\)"/);
  assert.match(metadataSource, /serverCheckoutRetrieveHelper: "retrieveDxStripeCheckoutSession\(input\)"/);
  assert.match(metadataSource, /serverPaymentIntentHelper: "createDxStripePaymentIntent\(input\)"/);
  assert.match(metadataSource, /paymentStatusHelper: "retrieveDxStripePaymentIntent\(\{ clientSecret \}\)"/);
  assert.match(metadataSource, /embeddedCheckoutHelper: "createDxStripeEmbeddedCheckout\(\{ clientSecret \}\)"/);
  assert.match(metadataSource, /embeddedCheckoutFetchHelper: "createDxStripeEmbeddedCheckout\(\{ fetchClientSecret \}\)"/);
  assert.match(
    metadataSource,
    /embeddedCheckoutClientSecretFetcher: "createDxStripeEmbeddedCheckoutClientSecretFetcher\(\{ contact \}\)"/,
  );
  assert.match(metadataSource, /serverCheckoutCustomerLinking: "createDxStripeCheckoutSession\(\{ customerId, customerUpdate \}\)"/);
  assert.match(metadataSource, /serverCustomerHelper: "createDxStripeCustomer\(input\)"/);
  assert.match(metadataSource, /serverSubscriptionListHelper: "listDxStripeCustomerSubscriptions\(\{ customerId \}\)"/);
  assert.match(metadataSource, /serverEmbeddedCheckoutHelper: "createDxStripeEmbeddedCheckoutSession\(input\)"/);
  assert.match(metadataSource, /serverBillingPortalHelper: "createDxStripeBillingPortalSession\(input\)"/);
  assert.match(metadataSource, /webhookHelper: "verifyDxStripeWebhookRequest\(request\)"/);
  assert.match(metadataSource, /webhookRoute: "app\/api\/stripe\/webhook\/route\.ts"/);
  assert.match(readmeSource, /Create Checkout Sessions with `server\.ts` for checkout\/contact requests/);
  assert.match(readmeSource, /Create embedded Checkout Sessions with `createDxStripeEmbeddedCheckoutSession` and `ui_mode: "embedded"`/);
  assert.match(readmeSource, /Create Billing Portal Sessions with `createDxStripeBillingPortalSession` only after authenticated app-owned customer lookup/);
  assert.match(readmeSource, /Billing Portal customer IDs must come from your server-owned user or account record, not browser input/);
  assert.match(readmeSource, /Pass `customerId` from your authenticated server record into hosted or embedded Checkout helpers when you want Checkout and Billing Portal to share a Stripe customer/);
  assert.match(readmeSource, /The generated checkout route intentionally does not trust browser-provided Stripe customer IDs/);
  assert.match(readmeSource, /Create Stripe Customers with `createDxStripeCustomer` after authentication and persist the returned `customer\.id` in your app database/);
  assert.match(readmeSource, /Customer creation helpers do not own user lookup, deduplication policy, or database writes/);
  assert.match(readmeSource, /List customer subscriptions with `listDxStripeCustomerSubscriptions` before deciding whether to send a signed-in user to Checkout or Billing Portal/);
  assert.match(readmeSource, /Subscription listing helpers do not own plan mapping, entitlement checks, billing interval display, or database writes/);
  assert.match(readmeSource, /Retrieve Checkout Sessions with `retrieveDxStripeCheckoutSession` before trusting success-page state/);
  assert.match(readmeSource, /Create PaymentIntents with `createDxStripePaymentIntent` for Payment Element flows/);
  assert.match(readmeSource, /Retrieve PaymentIntents with `retrieveDxStripePaymentIntent` on Payment Element return pages/);
  assert.match(readmeSource, /Create Embedded Checkout instances with `createDxStripeEmbeddedCheckout` only after your server creates an embedded Checkout Session client secret/);
  assert.match(
    readmeSource,
    /Use `createDxStripeEmbeddedCheckoutClientSecretFetcher` to adapt the app-owned checkout endpoint into Stripe\.js `fetchClientSecret`/,
  );
  assert.match(readmeSource, /Use `fetchClientSecret` for Embedded Checkout when the client should lazily request a Checkout Session client secret from an app-owned endpoint/);
  assert.match(readmeSource, /Verify webhook events with `verifyDxStripeWebhookRequest` or `verifyDxStripeWebhookEvent`/);
  assert.match(readmeSource, /`app\/api\/stripe\/webhook\/route\.ts` verifies Stripe signatures from the raw request body/);
  assert.match(readmeSource, /`app\/api\/checkout\/route\.ts` validates contact requests and creates Checkout Sessions only when a plan-specific Price env or fallback `STRIPE_PRICE_ID` is configured/);
  assert.match(readmeSource, /The Checkout route pins `runtime = "nodejs"` because Stripe's server SDK is not an Edge runtime dependency/);
  assert.match(readmeSource, /Production Checkout redirects require `NEXT_PUBLIC_APP_URL`, `APP_URL`, or `VERCEL_URL`; request-origin fallback is local-only/);
  assert.match(readmeSource, /`NEXT_PUBLIC_STRIPE_ACCOUNT`, when set, must be a Stripe connected account ID/);
  assert.match(readmeSource, /`NEXT_PUBLIC_STRIPE_ADVANCED_FRAUD_SIGNALS`, when set, must be `true` or `false`/);
  assert.match(readmeSource, /Stripe API version env values must use Stripe's date-based version shape/);
  assert.match(readmeSource, /Client checkout contact submissions only allow same-origin paths or HTTPS endpoints/);
  assert.match(readmeSource, /Embedded Checkout helpers require a Checkout Session client secret, not a hosted Checkout URL/);
  assert.match(readmeSource, /Embedded Checkout Sessions use `return_url`; hosted Checkout Sessions use `success_url` and `cancel_url`/);
  assert.match(readmeSource, /Embedded Checkout fetchers require the endpoint to return `kind: "embedded-checkout-session"`/);
  assert.match(readmeSource, /Embedded Checkout `fetchClientSecret` callbacks are revalidated before Stripe.js receives the value/);
  assert.match(readmeSource, /Payment Element helpers require a server-created PaymentIntent client secret/);
  assert.match(readmeSource, /`confirmDxStripePayment` calls `elements\.submit\(\)` before `stripe\.confirmPayment`/);
  assert.match(readmeSource, /PaymentIntent status pages still own order lookup, access control, and fulfillment state/);
  assert.match(readmeSource, /PaymentIntent amounts must be positive integers in the smallest currency unit/);
  assert.match(readmeSource, /Checkout Sessions require at least one app-owned line item/);
  assert.match(readmeSource, /Success pages still own authorization, persistence checks, and fulfillment status/);
  assert.match(readmeSource, /Webhook handlers still own event routing, idempotency, persistence, and fulfillment/);
  assert.match(readmeSource, /The generated webhook route acknowledges verified delivery and leaves fulfillment app-owned/);
  assert.match(readmeSource, /Checkout line items require a Stripe Price ID or app-owned `price_data`/);
  assert.match(readmeSource, /`STRIPE_PRICE_ID` must be a Stripe Price ID from your app-owned product catalog/);
  assert.match(readmeSource, /Plan-specific env such as `STRIPE_PRICE_ID_STARTER` can override the fallback `STRIPE_PRICE_ID`/);
  assert.match(readmeSource, /For Payment Element flows, create PaymentIntents on your server before calling `confirmDxStripePayment`/);
  assert.doesNotMatch(readmeSource, /Create Checkout Sessions or PaymentIntents on your server/);
  assert.doesNotMatch(readmeSource, /price_\.\.\./);
});

test("launch payment panel uses React Hook Form submit state without fake card inputs", () => {
  const paymentsStatus = read("examples/template/payments-status.tsx");

  for (const marker of [
    "@/lib/payments/stripe-js/checkout",
    "@/lib/forms/react-hook-form/form",
    "createDxZodResolver",
    "LaunchCheckoutContactValues",
    "formSubmitState",
    "checkoutMode",
    "setCheckoutMode",
    "setEmbeddedCheckoutRequest",
    "isSubmitting",
    "submitDxStripeCheckoutContact",
    "createDxStripeEmbeddedCheckoutClientSecretFetcher",
    "createDxStripeEmbeddedCheckout",
    "data-dx-checkout-submit-state",
    "data-dx-checkout-mode",
    "data-dx-embedded-checkout-mount",
  ]) {
    assert.match(paymentsStatus, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(paymentsStatus, /type DxStripeCheckoutMode/);
  assert.match(paymentsStatus, /checkoutMode === "embedded" && !onCheckoutContact/);
  assert.match(paymentsStatus, /Preparing Stripe Embedded Checkout/);
  assert.match(paymentsStatus, /embeddedCheckout\.mount\(containerRef\.current\)/);
  assert.match(paymentsStatus, /embeddedCheckout\.destroy\(\)/);
  assert.match(paymentsStatus, /checkoutMode,\s*contact: values/);
  assert.match(paymentsStatus, /plan: dashboardRequest\.body\.plan/);
  assert.match(paymentsStatus, /plan: request\.plan/);
  assert.match(paymentsStatus, /response\.kind === "embedded-checkout-session"/);
  assert.match(paymentsStatus, /Embedded Checkout Session created/);
  assert.doesNotMatch(paymentsStatus, /cardNumber|4242|fake card/i);
  assert.doesNotMatch(paymentsStatus, /PaymentIntent|clientSecret/);
});

test("launch checkout return pages verify Stripe Checkout Session status", () => {
  const sessionStatus = read("examples/template/checkout-session-status.tsx");
  const hostedSuccessPage = read("examples/template/app/checkout/success/page.tsx");
  const embeddedReturnPage = read("examples/template/app/checkout/return/page.tsx");

  for (const marker of [
    "@/lib/payments/stripe-js/server",
    "retrieveDxStripeCheckoutSession",
    "readLaunchCheckoutSessionIdSearchParam",
    "data-dx-checkout-session-status",
    "data-dx-checkout-payment-status",
    "data-dx-checkout-mode",
    "session.payment_status",
    "session.status",
    "session.customer_details?.email",
    "Verify fulfillment from your server",
  ]) {
    assert.match(sessionStatus, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(sessionStatus, /!sessionId\.startsWith\("cs_"\)/);
  assert.match(sessionStatus, /expand: \["payment_intent"\]/);
  assert.match(hostedSuccessPage, /metadata: Metadata/);
  assert.match(hostedSuccessPage, /mode="hosted"/);
  assert.match(hostedSuccessPage, /readLaunchCheckoutSessionIdSearchParam\(params\)/);
  assert.match(embeddedReturnPage, /metadata: Metadata/);
  assert.match(embeddedReturnPage, /mode="embedded"/);
  assert.match(embeddedReturnPage, /readLaunchCheckoutSessionIdSearchParam\(params\)/);
  assert.doesNotMatch(
    `${sessionStatus}\n${hostedSuccessPage}\n${embeddedReturnPage}`,
    /cardNumber|4242|fake card|client_secret|clientSecret|PaymentIntent/i,
  );
});

test("dashboard starter consumes payments/stripe-js as a real plan checkout workflow", () => {
  const stripeSource = read("core/src/ecosystem/forge_stripe_js.rs");
  const securitySource = read("core/src/ecosystem/forge_security.rs");
  const metadataSource = readRustRawStringConst(stripeSource, "STRIPE_METADATA_TS");
  const readmeSource = readRustRawStringConst(stripeSource, "STRIPE_README_MD");
  const dashboard = read("examples/dashboard/src/pages/Dashboard.tsx");
  const panel = read("examples/dashboard/src/components/StripePlanCheckout.tsx");
  const helper = read("examples/dashboard/src/lib/stripePlanCheckout.ts");
  const dashboardReadme = read("examples/dashboard/README.md");

  assert.match(dashboard, /import \{ StripePlanCheckout \} from ['"]\.\.\/components\/StripePlanCheckout['"];/);
  assert.match(dashboard, /<StripePlanCheckout/);

  assert.match(panel, /data-dx-package="payments\/stripe-js"/);
  assert.match(panel, /data-dx-component="dashboard-stripe-plan-checkout"/);
  assert.match(panel, /data-dx-stripe-dashboard-workflow="plan-checkout"/);
  assert.match(panel, /data-dx-node-modules="forbidden"/);
  assert.match(panel, /<dx-icon name="pack:payments"/);
  assert.match(panel, /data-dx-stripe-action="request-checkout-session"/);
  assert.match(panel, /data-dx-stripe-submit-state=\{submitState\}/);
  assert.match(panel, /data-dx-stripe-local-receipt=\{receipt \? receipt\.receiptId : "none"\}/);
  assert.match(panel, /createDxStripeDashboardCheckoutRequest/);
  assert.match(panel, /planId: selectedPlan\.id/);
  assert.doesNotMatch(panel, /plan: selectedPlan/);
  assert.match(panel, /createDxStripeDashboardMissingConfigReceipt/);
  assert.match(panel, /dxStripeDashboardCheckoutReadiness/);
  assert.doesNotMatch(panel, /createStripeDashboardCheckoutRequest/);
  assert.doesNotMatch(panel, /createStripeDashboardLocalReceipt/);
  assert.doesNotMatch(panel, /4242|cardNumber|stripe\.svg/i);
  assert.doesNotMatch(panel, /#[0-9a-fA-F]{3,6}/);

  assert.match(helper, /export type DxStripeDashboardCheckoutMode = "hosted" \| "embedded"/);
  assert.match(helper, /planId: DxStripeDashboardPlan\["id"\]/);
  assert.match(helper, /export function createDxStripeDashboardCheckoutRequest/);
  assert.match(helper, /const plan = readDxStripeDashboardPlan\(planId\)/);
  assert.match(helper, /function readDxStripeDashboardPlan/);
  assert.match(helper, /export function createDxStripeDashboardMissingConfigReceipt/);
  assert.match(helper, /export const dxStripeDashboardPlans/);
  assert.match(helper, /export const dxStripeDashboardCheckoutReadiness/);
  assert.match(helper, /source: "dx-www-dashboard"/);
  assert.match(helper, /body: \{/);
  assert.doesNotMatch(helper, /createStripeDashboardCheckoutRequest/);
  assert.doesNotMatch(helper, /createStripeDashboardLocalReceipt/);
  assert.match(helper, /submitDxStripeCheckoutContact/);
  assert.match(helper, /createDxStripeEmbeddedCheckoutClientSecretFetcher/);
  assert.match(helper, /StripeEmbeddedCheckoutOptions\.fetchClientSecret/);
  assert.match(helper, /Checkout Session/);
  assert.match(helper, /NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY/);
  assert.match(helper, /STRIPE_SECRET_KEY/);
  assert.match(helper, /STRIPE_PRICE_ID/);
  assert.match(helper, /STRIPE_PRICE_ID_STARTER/);
  assert.match(helper, /STRIPE_PRICE_ID_TEAM/);
  assert.match(helper, /STRIPE_PRICE_ID_SCALE/);
  assert.match(
    helper,
    /requiredEnv:\s*\[\s*"NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY",\s*"STRIPE_SECRET_KEY",\s*"STRIPE_PRICE_ID",\s*"STRIPE_PRICE_ID_STARTER",\s*"STRIPE_PRICE_ID_TEAM",\s*"STRIPE_PRICE_ID_SCALE",\s*\]/,
  );
  assert.match(helper, /status: "missing-config"/);
  assert.match(helper, /stripe-dashboard-local-/);
  assert.match(helper, /endpoint: "\/api\/checkout"/);

  assert.match(metadataSource, /dashboardUsage: \{/);
  assert.match(metadataSource, /aliases: \["payments", "stripe-js", "@stripe\/stripe-js", "stripe", "payments\/stripe"\]/);
  assert.match(metadataSource, /sourceMirror: "G:\/WWW\/inspirations\/stripe-js"/);
  assert.match(metadataSource, /provenance: \{/);
  assert.match(metadataSource, /packageJson: "G:\/WWW\/inspirations\/stripe-js\/package\.json"/);
  assert.match(metadataSource, /exportedFiles: \{/);
  assert.match(metadataSource, /dashboardCheckout: "lib\/payments\/stripe-js\/dashboard-checkout\.ts"/);
  assert.match(metadataSource, /"lib\/payments\/stripe-js\/dashboard-checkout\.ts"/);
  assert.match(metadataSource, /appOwnedBoundaries: \[/);
  assert.match(metadataSource, /sourceFile: "examples\/dashboard\/src\/components\/StripePlanCheckout\.tsx"/);
  assert.match(metadataSource, /helperFile: "examples\/dashboard\/src\/lib\/stripePlanCheckout\.ts"/);
  assert.match(metadataSource, /visibleWorkflow: "plan checkout action with safe missing-config receipt"/);
  assert.match(metadataSource, /icon: "pack:payments"/);
  assert.match(metadataSource, /receiptPaths: \{/);
  assert.match(metadataSource, /dashboard: "\.dx\/forge\/docs\/dashboard-stripe-plan-checkout\.md"/);
  assert.match(readmeSource, /Dashboard starter usage/);
  assert.match(readmeSource, /examples\/dashboard\/src\/components\/StripePlanCheckout\.tsx/);
  assert.match(readmeSource, /safe missing-config receipt/);
  assert.match(dashboardReadme, /Stripe checkout workflow/);
  assert.match(dashboardReadme, /data-dx-component="dashboard-stripe-plan-checkout"/);
  assert.match(securitySource, /Dashboard starter usage/);
  assert.match(securitySource, /dashboard-stripe-plan-checkout/);
  assert.match(securitySource, /safe missing-config receipt instead of fake payment success/);
});
