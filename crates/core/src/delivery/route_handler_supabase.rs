use std::collections::BTreeMap;

use serde_json::{Value, json};

use super::server_contract::{DxReactRouteHandlerRequest, DxReactRouteHandlerResponse};

pub(super) fn supabase_readiness_route_handler_response(
    source: &str,
    function_body: &str,
    request: &DxReactRouteHandlerRequest,
) -> Option<DxReactRouteHandlerResponse> {
    let path = request.path_for_match().trim_end_matches('/');
    if request.method != "GET"
        || path != "/api/supabase/readiness"
        || !source.contains("createSupabaseReadinessResponse")
        || !function_body.contains("createSupabaseReadinessResponse()")
        || !source.contains("server/supabase/readiness.ts")
    {
        return None;
    }

    let body = supabase_readiness_body(&request.runtime_env);
    let status = readiness_http_status(&body);

    Some(DxReactRouteHandlerResponse {
        status,
        content_type: "application/json; charset=utf-8".to_string(),
        headers: BTreeMap::from([
            ("cache-control".to_string(), "no-store".to_string()),
            (
                "x-dx-supabase-readiness".to_string(),
                "source-owned-safe-interpreter".to_string(),
            ),
        ]),
        redirect_url: None,
        body,
        execution_model: "source-owned-supabase-readiness-interpreter".to_string(),
        lifecycle_scripts_executed: false,
    })
}

fn supabase_readiness_body(runtime_env: &BTreeMap<String, String>) -> Value {
    let missing_env = supabase_missing_env(runtime_env);
    let (validation_error, local_project) =
        supabase_validation(runtime_env, missing_env.is_empty());
    let configured = missing_env.is_empty() && validation_error.is_none();

    json!({
        "schema": "dx.www.template.supabase_readiness",
        "packageId": "supabase/client",
        "officialName": "Backend Platform Client",
        "route": "/api/supabase/readiness",
        "status": if configured {
            "configured-source-owned-adapter-boundary"
        } else {
            "provider-gated"
        },
        "httpStatus": if configured { 200 } else { 501 },
        "runtimeProof": false,
        "networkCalls": false,
        "hostedCredentials": false,
        "requiredEnv": [
            "NEXT_PUBLIC_SUPABASE_URL",
            "NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY",
        ],
        "missingEnv": missing_env,
        "validationError": validation_error,
        "localProject": local_project,
        "sourceOwnedSurfaces": [
            "lib/supabase/env.ts",
            "lib/supabase/profiles.ts",
            "lib/supabase/profile-workflow.ts",
            "server/supabase/readiness.ts",
            "app/api/supabase/readiness/route.ts",
        ],
        "appOwnedBoundary": [
            "Supabase project URL and publishable key",
            "RLS policy migration",
            "Auth redirect allow-list",
            "hosted read/write/realtime proof",
        ],
        "message": if configured {
            "Supabase public configuration validates locally; hosted reads, writes, auth, RLS, and realtime remain app-owned proof."
        } else {
            "This route performs local configuration validation only; configure Supabase public env before enabling hosted reads, writes, auth, RLS, or realtime."
        },
    })
}

fn supabase_missing_env(runtime_env: &BTreeMap<String, String>) -> Vec<&'static str> {
    [
        "NEXT_PUBLIC_SUPABASE_URL",
        "NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY",
    ]
    .into_iter()
    .filter(|name| !runtime_env_value_present(runtime_env, name))
    .collect()
}

fn supabase_validation(
    runtime_env: &BTreeMap<String, String>,
    should_validate: bool,
) -> (Option<String>, Option<bool>) {
    if !should_validate {
        return (None, None);
    }

    let url = runtime_env
        .get("NEXT_PUBLIC_SUPABASE_URL")
        .map(String::as_str)
        .unwrap_or_default();
    if !is_http_url(url) {
        return (
            Some("NEXT_PUBLIC_SUPABASE_URL must be a valid URL.".to_string()),
            None,
        );
    }

    let publishable_key = runtime_env
        .get("NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY")
        .map(String::as_str)
        .unwrap_or_default();
    let key_lower = publishable_key.to_ascii_lowercase();
    if key_lower.contains("service_role")
        || key_lower.contains("service-role")
        || key_lower.contains("servicerole")
        || key_lower.contains("secret")
    {
        return (
            Some(
                "NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY must be a publishable key, not a service-role secret."
                    .to_string(),
            ),
            None,
        );
    }

    (None, Some(is_local_http_url(url)))
}

fn runtime_env_value_present(runtime_env: &BTreeMap<String, String>, name: &str) -> bool {
    runtime_env
        .get(name)
        .map(|value| !value.trim().is_empty())
        .unwrap_or(false)
}

fn is_http_url(value: &str) -> bool {
    let Some((scheme, rest)) = value.split_once("://") else {
        return false;
    };
    matches!(scheme, "http" | "https") && !rest.trim_matches('/').is_empty()
}

fn is_local_http_url(value: &str) -> bool {
    value
        .split_once("://")
        .map(|(_, rest)| {
            let host = rest.split('/').next().unwrap_or_default();
            let host = host.split(':').next().unwrap_or_default();
            matches!(host, "localhost" | "127.0.0.1")
        })
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

    use super::supabase_readiness_route_handler_response;
    use crate::delivery::server_contract::DxReactRouteHandlerRequest;

    #[test]
    fn supabase_readiness_accepts_absolute_url_for_endpoint_match() {
        let response = supabase_readiness_route_handler_response(
            r#"import { createSupabaseReadinessResponse } from "../../../../server/supabase/readiness.ts";

export function GET() {
  return createSupabaseReadinessResponse();
}
"#,
            "createSupabaseReadinessResponse()",
            &DxReactRouteHandlerRequest {
                method: "GET".to_string(),
                path: "https://example.test/api/supabase/readiness?source=dx#ready".to_string(),
                headers: BTreeMap::new(),
                body: Value::Null,
                route_params: BTreeMap::new(),
                search_params: BTreeMap::from([("source".to_string(), "dx".to_string())]),
                runtime_env: BTreeMap::from([
                    (
                        "NEXT_PUBLIC_SUPABASE_URL".to_string(),
                        "https://project.supabase.co".to_string(),
                    ),
                    (
                        "NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY".to_string(),
                        "sb_publishable_redacted".to_string(),
                    ),
                ]),
            },
        )
        .expect("Supabase readiness route should match absolute request URL");

        assert_eq!(response.status, 200);
        assert_eq!(
            response.execution_model,
            "source-owned-supabase-readiness-interpreter"
        );
        assert_eq!(response.body["route"], "/api/supabase/readiness");
        assert_eq!(response.body["networkCalls"], false);
    }
}
