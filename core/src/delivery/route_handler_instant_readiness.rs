use std::collections::BTreeMap;

use serde_json::{Value, json};

use super::server_contract::{DxReactRouteHandlerRequest, DxReactRouteHandlerResponse};

pub(super) fn instant_readiness_route_handler_response(
    source: &str,
    function_body: &str,
    request: &DxReactRouteHandlerRequest,
) -> Option<DxReactRouteHandlerResponse> {
    let path = request.path_for_match().trim_end_matches('/');
    if request.method != "GET"
        || path != "/api/instant/readiness"
        || !source.contains("createInstantReadinessResponse")
        || !function_body.contains("createInstantReadinessResponse()")
        || !source.contains("server/instant/readiness.ts")
    {
        return None;
    }

    let body = instant_readiness_body(&request.runtime_env);
    let status = readiness_http_status(&body);

    Some(DxReactRouteHandlerResponse {
        status,
        content_type: "application/json; charset=utf-8".to_string(),
        headers: BTreeMap::from([
            ("cache-control".to_string(), "no-store".to_string()),
            (
                "x-dx-instant-readiness".to_string(),
                "source-owned-safe-interpreter".to_string(),
            ),
        ]),
        redirect_url: None,
        body,
        execution_model: "source-owned-instant-readiness-interpreter".to_string(),
        lifecycle_scripts_executed: false,
    })
}

fn instant_readiness_body(runtime_env: &BTreeMap<String, String>) -> Value {
    let missing_env = instant_missing_env(runtime_env);
    let validation_error = instant_validation_error(runtime_env, missing_env.is_empty());
    let configured = missing_env.is_empty() && validation_error.is_none();

    json!({
        "schema": "dx.www.template.instant_readiness",
        "packageId": "instantdb/react",
        "officialName": "Realtime App Database",
        "route": "/api/instant/readiness",
        "status": if configured {
            "configured-source-owned-adapter-boundary"
        } else {
            "provider-gated"
        },
        "httpStatus": if configured { 200 } else { 501 },
        "runtimeProof": false,
        "networkCalls": false,
        "hostedCredentials": false,
        "requiredEnv": ["NEXT_PUBLIC_INSTANT_APP_ID"],
        "missingEnv": missing_env,
        "validationError": validation_error,
        "sourceOwnedSurfaces": [
            "lib/instant/env.ts",
            "lib/instant/schema.ts",
            "lib/instant/status.ts",
            "lib/instant/route.ts",
            "server/instant/readiness.ts",
            "app/api/instant/readiness/route.ts",
        ],
        "appOwnedBoundary": [
            "Instant hosted app id",
            "rules and auth policy",
            "realtime transport",
            "storage and stream runtime proof",
        ],
        "message": if configured {
            "InstantDB public configuration validates locally; hosted rules, auth, realtime transport, storage, and stream proof remain app-owned."
        } else {
            "This route performs local configuration validation only; configure the Instant app id before enabling hosted auth, realtime transport, storage, or streams."
        },
    })
}

fn instant_missing_env(runtime_env: &BTreeMap<String, String>) -> Vec<&'static str> {
    if runtime_env_value_present(runtime_env, "NEXT_PUBLIC_INSTANT_APP_ID") {
        Vec::new()
    } else {
        vec!["NEXT_PUBLIC_INSTANT_APP_ID"]
    }
}

fn instant_validation_error(
    runtime_env: &BTreeMap<String, String>,
    should_validate: bool,
) -> Option<String> {
    if !should_validate {
        return None;
    }

    for name in [
        "NEXT_PUBLIC_INSTANT_DEVTOOL",
        "NEXT_PUBLIC_INSTANT_DISABLE_VALIDATION",
        "NEXT_PUBLIC_INSTANT_VERBOSE",
    ] {
        if let Some(value) = runtime_env.get(name) {
            if !is_optional_boolean_env(value) {
                return Some(format!("Invalid boolean InstantDB env var: {name}"));
            }
        }
    }

    if let Some(value) = runtime_env.get("NEXT_PUBLIC_INSTANT_QUERY_CACHE_LIMIT") {
        if !is_non_negative_number(value) {
            return Some(
                "Invalid non-negative number InstantDB env var: NEXT_PUBLIC_INSTANT_QUERY_CACHE_LIMIT"
                    .to_string(),
            );
        }
    }

    None
}

fn runtime_env_value_present(runtime_env: &BTreeMap<String, String>, name: &str) -> bool {
    runtime_env
        .get(name)
        .map(|value| !value.trim().is_empty())
        .unwrap_or(false)
}

fn is_optional_boolean_env(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "1" | "true" | "yes" | "on" | "0" | "false" | "no" | "off"
    )
}

fn is_non_negative_number(value: &str) -> bool {
    value
        .trim()
        .parse::<f64>()
        .map(|value| value.is_finite() && value >= 0.0)
        .unwrap_or(false)
}

fn readiness_http_status(body: &Value) -> u16 {
    body.get("httpStatus")
        .and_then(Value::as_u64)
        .and_then(|status| u16::try_from(status).ok())
        .unwrap_or(501)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use serde_json::Value;

    use super::instant_readiness_route_handler_response;
    use crate::delivery::server_contract::DxReactRouteHandlerRequest;

    #[test]
    fn instant_readiness_accepts_absolute_url_for_endpoint_match() {
        let response = instant_readiness_route_handler_response(
            r#"import { createInstantReadinessResponse } from "../../../../server/instant/readiness.ts";

export function GET() {
  return createInstantReadinessResponse();
}
"#,
            "createInstantReadinessResponse()",
            &DxReactRouteHandlerRequest {
                method: "GET".to_string(),
                path: "https://example.test/api/instant/readiness?source=dx#ready".to_string(),
                headers: BTreeMap::new(),
                body: Value::Null,
                route_params: BTreeMap::new(),
                search_params: BTreeMap::from([("source".to_string(), "dx".to_string())]),
                runtime_env: BTreeMap::from([(
                    "NEXT_PUBLIC_INSTANT_APP_ID".to_string(),
                    "app_redacted".to_string(),
                )]),
            },
        )
        .expect("Instant readiness route should match absolute request URL");

        assert_eq!(response.status, 200);
        assert_eq!(
            response.execution_model,
            "source-owned-instant-readiness-interpreter"
        );
        assert_eq!(response.body["route"], "/api/instant/readiness");
        assert_eq!(response.body["networkCalls"], false);
    }
}
