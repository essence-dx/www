use std::collections::BTreeMap;

use serde_json::{Value, json};

use super::server_contract::{DxReactRouteHandlerRequest, DxReactRouteHandlerResponse};

pub(super) fn payments_stripe_checkout_route_handler_response(
    source: &str,
    function_body: &str,
    request: &DxReactRouteHandlerRequest,
) -> Option<DxReactRouteHandlerResponse> {
    let path = request.path_for_match().trim_end_matches('/');
    if path != "/api/checkout"
        || request.method != "POST"
        || !source.contains("@/lib/payments/stripe-js/checkout")
        || !source.contains("createDxStripeCheckoutContactPayload")
        || !source.contains("hasStripeCheckoutConfig")
        || !function_body.contains("createDxStripeCheckoutContactPayload")
    {
        return None;
    }

    let (status, body) = stripe_checkout_boundary_body(&request.body, &request.runtime_env);

    Some(DxReactRouteHandlerResponse {
        status,
        content_type: "application/json; charset=utf-8".to_string(),
        headers: BTreeMap::from([(
            "x-dx-payments-stripe-checkout".to_string(),
            "source-owned-safe-interpreter".to_string(),
        )]),
        redirect_url: None,
        body,
        execution_model: "source-owned-payments-stripe-checkout-boundary-interpreter".to_string(),
        lifecycle_scripts_executed: false,
    })
}

pub(super) fn payments_stripe_webhook_route_handler_response(
    source: &str,
    function_body: &str,
    request: &DxReactRouteHandlerRequest,
) -> Option<DxReactRouteHandlerResponse> {
    let path = request.path_for_match().trim_end_matches('/');
    if path != "/api/stripe/webhook"
        || request.method != "POST"
        || !source.contains("@/lib/payments/stripe-js/server")
        || !source.contains("verifyDxStripeWebhookRequest")
        || !source.contains("routeDxStripeWebhookEvent")
        || !function_body.contains("verifyDxStripeWebhookRequest")
    {
        return None;
    }

    let (status, body) = stripe_webhook_boundary_body(request, &request.runtime_env);

    Some(DxReactRouteHandlerResponse {
        status,
        content_type: "application/json; charset=utf-8".to_string(),
        headers: BTreeMap::from([(
            "x-dx-payments-stripe-webhook".to_string(),
            "source-owned-safe-interpreter".to_string(),
        )]),
        redirect_url: None,
        body,
        execution_model: "source-owned-payments-stripe-webhook-boundary-interpreter".to_string(),
        lifecycle_scripts_executed: false,
    })
}

pub(super) fn payments_stripe_readiness_route_handler_response(
    source: &str,
    function_body: &str,
    request: &DxReactRouteHandlerRequest,
) -> Option<DxReactRouteHandlerResponse> {
    let path = request.path_for_match().trim_end_matches('/');
    if path != "/api/payments/stripe-js/readiness"
        || !matches!(request.method.as_str(), "GET" | "POST")
        || !source.contains("@/lib/payments/stripe-js/dashboard-checkout")
        || !source.contains("createDxStripeDashboardCheckoutRequest")
        || !source.contains("createDxStripeDashboardMissingConfigReceipt")
    {
        return None;
    }

    if request.method == "GET" && !function_body.contains("buildDxStripeReadiness") {
        return None;
    }

    if request.method == "POST"
        && (!function_body.contains("createDxStripeDashboardCheckoutRequest")
            || !function_body.contains("createDxStripeDashboardMissingConfigReceipt"))
    {
        return None;
    }

    let (status, body) = if request.method == "GET" {
        (200, stripe_get_readiness_body(&request.runtime_env))
    } else {
        stripe_post_readiness_body(&request.body, &request.runtime_env)
    };

    Some(DxReactRouteHandlerResponse {
        status,
        content_type: "application/json; charset=utf-8".to_string(),
        headers: BTreeMap::from([(
            "x-dx-payments-stripe-readiness".to_string(),
            "source-owned-safe-interpreter".to_string(),
        )]),
        redirect_url: None,
        body,
        execution_model: "source-owned-payments-stripe-readiness-interpreter".to_string(),
        lifecycle_scripts_executed: false,
    })
}

fn stripe_webhook_boundary_body(
    request: &DxReactRouteHandlerRequest,
    runtime_env: &BTreeMap<String, String>,
) -> (u16, Value) {
    let missing_required_env = stripe_missing_webhook_env(runtime_env);
    let credentials_configured = missing_required_env.is_empty();
    let signature_header_present = stripe_signature_header_present(request);
    let payload_received = !request.body.is_null();
    let local_receipt_ready =
        credentials_configured && signature_header_present && payload_received;
    let http_status = if local_receipt_ready { 202 } else { 501 };

    (
        http_status,
        json!({
            "schema": "dx.payments.stripe_js.webhook_boundary",
            "ok": local_receipt_ready,
            "received": local_receipt_ready,
            "packageId": "payments/stripe-js",
            "officialPackageName": "Payments",
            "upstreamPackage": "stripe",
            "status": stripe_webhook_status(
                credentials_configured,
                signature_header_present,
                payload_received,
            ),
            "httpStatus": http_status,
            "kind": if local_receipt_ready {
                "configured-webhook-receipt"
            } else {
                "provider-boundary"
            },
            "endpoint": "/api/stripe/webhook",
            "method": "POST",
            "requiredEnv": stripe_webhook_required_env(),
            "missingRequiredEnv": missing_required_env,
            "missingDeliveryEvidence": stripe_webhook_missing_delivery_evidence(
                signature_header_present,
                payload_received,
            ),
            "signatureHeaderPresent": signature_header_present,
            "payloadReceived": payload_received,
            "rawBodyRequired": true,
            "credentialsConfigured": credentials_configured,
            "providerConfigured": credentials_configured,
            "providerBoundary": true,
            "webhookVerificationBoundary": true,
            "runtimeExecution": false,
            "liveProviderExecution": false,
            "stripeLiveExecution": false,
            "webhookVerified": false,
            "eventRouted": false,
            "eventClassified": local_receipt_ready,
            "eventAction": stripe_webhook_event_action(&request.body),
            "event": stripe_webhook_event_summary(&request.body),
            "secretValues": [],
            "fulfillmentStatus": "app-owned",
            "appOwnedBoundary": "Stripe webhook delivery and fulfillment stay app-owned until signature verification and fulfillment handlers are configured.",
            "boundary": if local_receipt_ready {
                "Stripe webhook delivery was observed locally with configured secret presence; this DX receipt does not verify signatures, mutate billing state, or process fulfillment."
            } else {
                "This response is a local DX/Zed webhook-boundary receipt. Configure Stripe server secret presence, webhook signing secret presence, signature header delivery, and payload delivery before accepting the webhook receipt."
            },
        }),
    )
}

fn stripe_get_readiness_body(runtime_env: &BTreeMap<String, String>) -> Value {
    let readiness = stripe_readiness_body(None, runtime_env);
    let status = readiness
        .get("status")
        .cloned()
        .unwrap_or_else(|| json!("missing-config"));
    let provider_configured = stripe_missing_required_env(None, runtime_env).is_empty();

    json!({
        "ok": true,
        "packageId": "payments/stripe-js",
        "status": status,
        "providerBoundary": true,
        "providerConfigured": provider_configured,
        "readiness": readiness,
        "plans": stripe_plan_rows(),
        "runtimeExecution": false,
        "liveProviderExecution": false,
        "stripeLiveExecution": false,
        "secretValues": [],
        "boundary": "This route exposes Payments readiness only. Stripe keys, Price IDs, Checkout redirects, webhook delivery, and fulfillment stay app-owned."
    })
}

fn stripe_post_readiness_body(
    body: &Value,
    runtime_env: &BTreeMap<String, String>,
) -> (u16, Value) {
    let plan = stripe_plan(read_plan_id(body));
    let checkout_mode = read_checkout_mode(body);
    let contact = match read_contact(body) {
        Ok(contact) => contact,
        Err(message) => return (400, stripe_bad_request_body(message)),
    };
    let request = json!({
        "endpoint": "/api/checkout",
        "method": "POST",
        "checkoutMode": checkout_mode,
        "contact": contact.clone(),
        "plan": plan.clone(),
        "body": {
            "checkoutMode": checkout_mode,
            "contact": contact.clone(),
            "source": "dx-www-dashboard",
            "plan": {
                "id": plan["id"],
                "priceEnv": plan["priceEnv"],
            },
        },
    });
    let price_env = plan["priceEnv"].as_str().unwrap_or("STRIPE_PRICE_ID");
    let configured = stripe_checkout_configured(price_env, runtime_env);
    let http_status = if configured { 202 } else { 501 };
    let readiness_status = if configured {
        "provider-configured-dry-run-only"
    } else {
        "missing-config"
    };
    let receipt = json!({
        "receiptId": stripe_receipt_id(
            plan["id"].as_str().unwrap_or("starter"),
            checkout_mode,
            contact["email"].as_str().unwrap_or("contact"),
        ),
        "status": if configured { "dry-run-ready" } else { "missing-config" },
        "message": if configured {
            "Checkout request is provider-configured; this readiness route still did not call Stripe."
        } else {
            "Checkout request is ready, but Stripe credentials and a product Price ID must be app-owned before creating a Checkout Session."
        },
        "requiredEnv": stripe_required_env(),
        "request": request.clone(),
    });
    let readiness = stripe_readiness_body(Some(price_env), runtime_env);

    (
        http_status,
        json!({
            "ok": configured,
            "packageId": "payments/stripe-js",
            "status": readiness_status,
            "httpStatus": http_status,
            "providerBoundary": true,
            "providerConfigured": configured,
            "request": request.clone(),
            "receipt": receipt,
            "readiness": readiness,
            "runtimeExecution": false,
            "liveProviderExecution": false,
            "stripeLiveExecution": false,
            "secretValues": [],
            "boundary": "The response is a local DX/Zed readiness receipt. It does not create Checkout Sessions, read secret values, redirect users, or process webhooks."
        }),
    )
}

fn stripe_checkout_boundary_body(
    body: &Value,
    runtime_env: &BTreeMap<String, String>,
) -> (u16, Value) {
    let contact = match read_contact(body) {
        Ok(contact) => contact,
        Err(message) => return (400, stripe_bad_request_body(message)),
    };
    let checkout_mode = read_checkout_mode(body);
    let plan = stripe_plan(read_plan_id(body));
    let price_env = plan["priceEnv"].as_str().unwrap_or("STRIPE_PRICE_ID");
    let configured = stripe_checkout_configured(price_env, runtime_env);
    let http_status = if configured { 202 } else { 501 };

    (
        http_status,
        json!({
            "schema": "dx.payments.stripe_js.checkout_boundary",
            "ok": configured,
            "packageId": "payments/stripe-js",
            "status": if configured { "provider-configured-dry-run-only" } else { "missing-config" },
            "httpStatus": http_status,
            "kind": if configured { "contact" } else { "provider-boundary" },
            "checkoutMode": checkout_mode,
            "contact": contact,
            "plan": plan.clone(),
            "requiredEnv": [
                "NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY",
                "STRIPE_SECRET_KEY",
                price_env,
                "STRIPE_PRICE_ID",
            ],
            "missingRequiredEnv": stripe_missing_required_env(Some(price_env), runtime_env),
            "credentialsConfigured": configured,
            "providerConfigured": configured,
            "providerBoundary": true,
            "runtimeExecution": false,
            "liveProviderExecution": false,
            "stripeLiveExecution": false,
            "secretValues": [],
            "appOwnedBoundary": "Create a real Stripe Checkout Session only after Stripe credentials and Price IDs are configured.",
            "boundary": "This response is a local DX/Zed checkout-boundary receipt. It does not create Checkout Sessions, read Stripe secrets, redirect users, or process webhooks."
        }),
    )
}

fn stripe_bad_request_body(message: String) -> Value {
    json!({
        "ok": false,
        "packageId": "payments/stripe-js",
        "status": "bad-request",
        "message": message,
        "runtimeExecution": false,
        "stripeLiveExecution": false,
        "secretValues": [],
    })
}

fn stripe_readiness_body(
    selected_price_env: Option<&str>,
    runtime_env: &BTreeMap<String, String>,
) -> Value {
    let missing_required_env = stripe_missing_required_env(selected_price_env, runtime_env);

    json!({
        "schema": "dx.payments.stripe_js.readiness",
        "packageId": "payments/stripe-js",
        "status": if missing_required_env.is_empty() { "provider-configured-dry-run-only" } else { "missing-config" },
        "endpoint": "/api/payments/stripe-js/readiness",
        "liveCheckoutEndpoint": "/api/checkout",
        "requiredEnv": match selected_price_env {
            Some(price_env) => json!([
                "NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY",
                "STRIPE_SECRET_KEY",
                price_env,
                "STRIPE_PRICE_ID",
            ]),
            None => json!(stripe_required_env()),
        },
        "selectedPriceEnv": selected_price_env,
        "configuredEnv": stripe_configured_env_rows(selected_price_env, runtime_env),
        "missingRequiredEnv": missing_required_env,
        "runtimeExecution": false,
        "stripeLiveExecution": false,
        "secretValues": [],
    })
}

fn stripe_signature_header_present(request: &DxReactRouteHandlerRequest) -> bool {
    request.headers.iter().any(|(name, value)| {
        name.eq_ignore_ascii_case("stripe-signature") && !value.trim().is_empty()
    })
}

fn stripe_webhook_status(
    credentials_configured: bool,
    signature_header_present: bool,
    payload_received: bool,
) -> &'static str {
    if credentials_configured && signature_header_present && payload_received {
        "configured-source-owned-webhook-receipt"
    } else if credentials_configured {
        "missing-webhook-delivery-evidence"
    } else {
        "missing-config"
    }
}

fn stripe_webhook_required_env() -> Vec<&'static str> {
    vec!["STRIPE_SECRET_KEY", "STRIPE_WEBHOOK_SECRET"]
}

fn stripe_missing_webhook_env(runtime_env: &BTreeMap<String, String>) -> Vec<&'static str> {
    stripe_webhook_required_env()
        .into_iter()
        .filter(|name| !stripe_env_value_present(runtime_env, name))
        .collect()
}

fn stripe_webhook_missing_delivery_evidence(
    signature_header_present: bool,
    payload_received: bool,
) -> Vec<&'static str> {
    let mut missing = Vec::new();
    if !signature_header_present {
        missing.push("stripe-signature header");
    }
    if !payload_received {
        missing.push("JSON request payload");
    }
    missing
}

fn stripe_webhook_event_summary(body: &Value) -> Value {
    json!({
        "id": stripe_webhook_event_string(body, "id"),
        "type": stripe_webhook_event_string(body, "type"),
        "livemode": body.get("livemode").and_then(Value::as_bool),
        "created": body.get("created").and_then(Value::as_u64),
    })
}

fn stripe_webhook_event_action(body: &Value) -> &'static str {
    match body.get("type").and_then(Value::as_str).unwrap_or_default() {
        "checkout.session.completed" => "checkout-session-completed",
        "checkout.session.expired" => "checkout-session-expired",
        "payment_intent.succeeded" => "payment-intent-succeeded",
        "payment_intent.payment_failed" => "payment-intent-payment-failed",
        _ => "unhandled",
    }
}

fn stripe_webhook_event_string(body: &Value, key: &str) -> Option<String> {
    body.get(key)
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

fn stripe_plan_rows() -> Value {
    json!([
        stripe_plan("starter"),
        stripe_plan("team"),
        stripe_plan("scale"),
    ])
}

fn stripe_plan(plan_id: &str) -> Value {
    match plan_id {
        "team" => json!({
            "id": "team",
            "label": "Team",
            "priceLabel": "$99/mo",
            "description": "For teams that need hosted Checkout plus Billing Portal readiness.",
            "priceEnv": "STRIPE_PRICE_ID_TEAM",
        }),
        "scale" => json!({
            "id": "scale",
            "label": "Scale",
            "priceLabel": "Custom",
            "description": "For embedded Checkout and app-owned subscription entitlement review.",
            "priceEnv": "STRIPE_PRICE_ID_SCALE",
        }),
        _ => json!({
            "id": "starter",
            "label": "Starter",
            "priceLabel": "$29/mo",
            "description": "For a small DX-WWW launch dashboard with one app-owned product price.",
            "priceEnv": "STRIPE_PRICE_ID_STARTER",
        }),
    }
}

fn read_plan_id(body: &Value) -> &str {
    body.get("plan")
        .and_then(|plan| plan.get("id"))
        .or_else(|| body.get("planId"))
        .and_then(Value::as_str)
        .filter(|value| matches!(*value, "starter" | "team" | "scale"))
        .unwrap_or("starter")
}

fn read_checkout_mode(body: &Value) -> &str {
    match body.get("checkoutMode").and_then(Value::as_str) {
        Some("embedded") => "embedded",
        _ => "hosted",
    }
}

fn read_contact(body: &Value) -> Result<Value, String> {
    let Some(contact) = body.get("contact").and_then(Value::as_object) else {
        return Err("Payments readiness dry-run requires contact details.".to_string());
    };
    let email = contact
        .get("email")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .trim()
        .to_ascii_lowercase();
    let name = contact
        .get("name")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .trim();
    let organization = contact
        .get("organization")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .trim();
    let message = contact
        .get("message")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .trim();

    let mut issues = Vec::new();
    if !is_checkout_email(&email) {
        issues.push("Enter a valid checkout email.");
    }
    if name.len() < 2 {
        issues.push("Enter the checkout contact name.");
    }
    if name.len() > 80 {
        issues.push("Checkout contact name must be 80 characters or fewer.");
    }
    if organization.len() > 120 {
        issues.push("Organization must be 120 characters or fewer.");
    }
    if message.len() > 500 {
        issues.push("Checkout notes must be 500 characters or fewer.");
    }
    if !issues.is_empty() {
        return Err(issues.join(" "));
    }

    let mut contact_body = serde_json::Map::from_iter([
        ("email".to_string(), json!(email)),
        ("name".to_string(), json!(name)),
    ]);
    if !organization.is_empty() {
        contact_body.insert("organization".to_string(), json!(organization));
    }
    if !message.is_empty() {
        contact_body.insert("message".to_string(), json!(message));
    }

    Ok(Value::Object(contact_body))
}

fn is_checkout_email(email: &str) -> bool {
    let Some((local, domain)) = email.split_once('@') else {
        return false;
    };
    !local.is_empty()
        && domain.contains('.')
        && !domain.starts_with('.')
        && !domain.ends_with('.')
        && !email.chars().any(char::is_whitespace)
}

fn stripe_receipt_id(plan_id: &str, checkout_mode: &str, email: &str) -> String {
    format!(
        "stripe-dashboard-local-{plan_id}-{checkout_mode}-{}",
        stripe_email_slug(email)
    )
}

fn stripe_email_slug(email: &str) -> String {
    let mut slug = String::new();
    let mut previous_dash = false;
    for character in email.chars().flat_map(char::to_lowercase) {
        if character.is_ascii_alphanumeric() {
            slug.push(character);
            previous_dash = false;
        } else if !previous_dash && !slug.is_empty() {
            slug.push('-');
            previous_dash = true;
        }
    }
    while slug.ends_with('-') {
        slug.pop();
    }
    if slug.is_empty() {
        "contact".to_string()
    } else {
        slug
    }
}

fn stripe_required_env() -> Vec<&'static str> {
    vec![
        "NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY",
        "STRIPE_SECRET_KEY",
        "STRIPE_PRICE_ID",
        "STRIPE_PRICE_ID_STARTER",
        "STRIPE_PRICE_ID_TEAM",
        "STRIPE_PRICE_ID_SCALE",
    ]
}

fn stripe_configured_env_rows(
    selected_price_env: Option<&str>,
    runtime_env: &BTreeMap<String, String>,
) -> Value {
    let mut names = stripe_required_env();
    if let Some(price_env) = selected_price_env {
        names.push(price_env);
    }
    names.sort_unstable();
    names.dedup();

    Value::Array(
        names
            .into_iter()
            .map(|name| {
                json!({
                    "name": name,
                    "configured": stripe_env_value_present(runtime_env, name),
                    "valueExposed": false,
                })
            })
            .collect(),
    )
}

fn stripe_checkout_configured(price_env: &str, runtime_env: &BTreeMap<String, String>) -> bool {
    stripe_env_value_present(runtime_env, "NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY")
        && stripe_env_value_present(runtime_env, "STRIPE_SECRET_KEY")
        && (stripe_env_value_present(runtime_env, price_env)
            || stripe_env_value_present(runtime_env, "STRIPE_PRICE_ID"))
}

fn stripe_missing_required_env(
    selected_price_env: Option<&str>,
    runtime_env: &BTreeMap<String, String>,
) -> Vec<String> {
    let mut missing = Vec::new();

    for name in ["NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY", "STRIPE_SECRET_KEY"] {
        if !stripe_env_value_present(runtime_env, name) {
            missing.push(name.to_string());
        }
    }

    if !stripe_price_env_configured(selected_price_env, runtime_env) {
        missing.push(
            selected_price_env
                .unwrap_or("STRIPE_PRICE_ID or one plan-specific Price env")
                .to_string(),
        );
    }

    missing
}

fn stripe_price_env_configured(
    selected_price_env: Option<&str>,
    runtime_env: &BTreeMap<String, String>,
) -> bool {
    match selected_price_env {
        Some(price_env) => {
            stripe_env_value_present(runtime_env, price_env)
                || stripe_env_value_present(runtime_env, "STRIPE_PRICE_ID")
        }
        None => {
            stripe_env_value_present(runtime_env, "STRIPE_PRICE_ID")
                || [
                    "STRIPE_PRICE_ID_STARTER",
                    "STRIPE_PRICE_ID_TEAM",
                    "STRIPE_PRICE_ID_SCALE",
                ]
                .into_iter()
                .any(|name| stripe_env_value_present(runtime_env, name))
        }
    }
}

fn stripe_env_value_present(runtime_env: &BTreeMap<String, String>, name: &str) -> bool {
    runtime_env
        .get(name)
        .map(|value| !value.trim().is_empty())
        .unwrap_or(false)
}
