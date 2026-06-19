use std::collections::BTreeMap;

use regex::Regex;
use serde_json::{Value, json};

use super::server_contract::{DxReactRouteHandlerRequest, DxReactRouteHandlerResponse};

const DX_HTTP_JSON_ALLOWED_ORIGINS: &str = "DX_HTTP_JSON_ALLOWED_ORIGINS";
const HELPER_FACTORY: &str = "createDxHttpJsonRoute";
const HELPER_RESPONSE: &str = "createDxHttpJsonRouteResponse";

pub(super) fn http_json_route_handler_response(
    source: &str,
    handler_source: &str,
    request: &DxReactRouteHandlerRequest,
) -> Option<DxReactRouteHandlerResponse> {
    if !http_json_helper_present(source, handler_source) {
        return None;
    }

    let config = HttpJsonRouteConfig::from_source(source, handler_source);
    let policy = http_json_route_policy(request, &config);
    let route = normalized_route_path(request);
    let cache_control = response_cache_control(config.cache.as_deref());

    Some(DxReactRouteHandlerResponse {
        status: policy.status,
        content_type: "application/json; charset=utf-8".to_string(),
        headers: BTreeMap::from([
            ("cache-control".to_string(), cache_control),
            (
                "x-dx-http-json-route".to_string(),
                "source-owned-safe-interpreter".to_string(),
            ),
        ]),
        redirect_url: None,
        body: http_json_route_policy_body(request, &route, &config, &policy),
        execution_model: "source-owned-http-json-route-policy-interpreter".to_string(),
        lifecycle_scripts_executed: false,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HttpJsonRouteConfig {
    target: Option<String>,
    method: Option<String>,
    allowed_origins: Vec<String>,
    required_search_params: Vec<String>,
    cache: Option<String>,
    timeout_ms: Option<u64>,
    response_mode: String,
    recognized_factory: &'static str,
}

impl HttpJsonRouteConfig {
    fn from_source(source: &str, handler_source: &str) -> Self {
        let target = string_field(handler_source, "target")
            .or_else(|| string_field(handler_source, "upstream"))
            .or_else(|| string_field(handler_source, "url"))
            .or_else(|| string_field(source, "target"))
            .or_else(|| string_field(source, "upstream"))
            .or_else(|| string_field(source, "url"));
        let method = string_field(handler_source, "method")
            .or_else(|| string_field(source, "method"))
            .map(|method| method.trim().to_ascii_uppercase())
            .filter(|method| http_json_method(method));
        let cache = string_field(handler_source, "cache").or_else(|| string_field(source, "cache"));
        let timeout_ms =
            number_field(handler_source, "timeoutMs").or_else(|| number_field(source, "timeoutMs"));
        let response_mode = string_field(handler_source, "responseMode")
            .or_else(|| string_field(source, "responseMode"))
            .unwrap_or_else(|| "json".to_string());
        let recognized_factory = if handler_source.contains(HELPER_RESPONSE) {
            HELPER_RESPONSE
        } else {
            HELPER_FACTORY
        };

        Self {
            target,
            method,
            allowed_origins: dedupe_strings(
                string_array_field(handler_source, "allowedOrigins")
                    .into_iter()
                    .chain(string_array_field(source, "allowedOrigins")),
            ),
            required_search_params: dedupe_strings(
                string_array_field(handler_source, "requiredSearchParams")
                    .into_iter()
                    .chain(string_array_field(source, "requiredSearchParams")),
            ),
            cache,
            timeout_ms,
            response_mode,
            recognized_factory,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HttpJsonRoutePolicy {
    status: u16,
    status_label: &'static str,
    message: &'static str,
    configured_allowed_origins: Vec<String>,
    target_origin: Option<String>,
    missing_search_params: Vec<String>,
    upstream_request_allowed: bool,
}

fn http_json_route_policy(
    request: &DxReactRouteHandlerRequest,
    config: &HttpJsonRouteConfig,
) -> HttpJsonRoutePolicy {
    let configured_allowed_origins = configured_allowed_origins(request, config);
    let Some(target) = config.target.as_deref() else {
        return HttpJsonRoutePolicy {
            status: 501,
            status_label: "missing-target",
            message: "DX-WWW recognized an HTTP JSON route helper, but the route did not declare an HTTPS target.",
            configured_allowed_origins,
            target_origin: None,
            missing_search_params: Vec::new(),
            upstream_request_allowed: false,
        };
    };
    let Some(target_origin) = https_origin(target) else {
        return HttpJsonRoutePolicy {
            status: 400,
            status_label: "invalid-target",
            message: "DX-WWW rejected the HTTP JSON route because the target is not a clean HTTPS URL.",
            configured_allowed_origins,
            target_origin: None,
            missing_search_params: Vec::new(),
            upstream_request_allowed: false,
        };
    };
    if configured_allowed_origins.is_empty() {
        return HttpJsonRoutePolicy {
            status: 501,
            status_label: "missing-allowed-origins",
            message: "DX-WWW recognized the HTTP JSON route helper, but no allowed origin policy is configured.",
            configured_allowed_origins,
            target_origin: Some(target_origin),
            missing_search_params: Vec::new(),
            upstream_request_allowed: false,
        };
    }
    if !configured_allowed_origins
        .iter()
        .any(|origin| origin == &target_origin)
    {
        return HttpJsonRoutePolicy {
            status: 403,
            status_label: "target-origin-not-allowed",
            message: "DX-WWW rejected the HTTP JSON route before network execution because the target origin is not in the route allowlist.",
            configured_allowed_origins,
            target_origin: Some(target_origin),
            missing_search_params: Vec::new(),
            upstream_request_allowed: false,
        };
    }
    if config
        .method
        .as_deref()
        .is_some_and(|method| method != request.method)
    {
        return HttpJsonRoutePolicy {
            status: 405,
            status_label: "method-not-allowed",
            message: "DX-WWW rejected the HTTP JSON route because the request method does not match the declared route method.",
            configured_allowed_origins,
            target_origin: Some(target_origin),
            missing_search_params: Vec::new(),
            upstream_request_allowed: false,
        };
    }

    let query_params = request.query_params();
    let missing_search_params = config
        .required_search_params
        .iter()
        .filter(|name| {
            query_params
                .get(name.as_str())
                .map(|value| value.trim().is_empty())
                .unwrap_or(true)
        })
        .cloned()
        .collect::<Vec<String>>();
    if !missing_search_params.is_empty() {
        return HttpJsonRoutePolicy {
            status: 400,
            status_label: "missing-required-search-params",
            message: "DX-WWW rejected the HTTP JSON route because required query parameters are missing.",
            configured_allowed_origins,
            target_origin: Some(target_origin),
            missing_search_params,
            upstream_request_allowed: false,
        };
    }

    HttpJsonRoutePolicy {
        status: 202,
        status_label: "proxy-policy-accepted",
        message: "DX-WWW accepted the HTTP JSON route policy. The app-owned runtime adapter must perform the actual upstream request.",
        configured_allowed_origins,
        target_origin: Some(target_origin),
        missing_search_params: Vec::new(),
        upstream_request_allowed: true,
    }
}

fn http_json_route_policy_body(
    request: &DxReactRouteHandlerRequest,
    route: &str,
    config: &HttpJsonRouteConfig,
    policy: &HttpJsonRoutePolicy,
) -> Value {
    json!({
        "ok": policy.upstream_request_allowed,
        "schema": "dx.www.http_json_route_policy",
        "route": route,
        "method": request.method,
        "target": config.target,
        "targetOrigin": policy.target_origin,
        "responseMode": config.response_mode,
        "cache": config.cache,
        "timeoutMs": config.timeout_ms,
        "status": policy.status_label,
        "policyStatus": policy.status_label,
        "httpStatus": policy.status,
        "message": policy.message,
        "requiredSearchParams": config.required_search_params,
        "missingSearchParams": policy.missing_search_params,
        "searchParams": request.query_params(),
        "allowedOriginsConfigured": !policy.configured_allowed_origins.is_empty(),
        "configuredAllowedOrigins": policy.configured_allowed_origins,
        "upstreamRequestAllowed": policy.upstream_request_allowed,
        "proxyPolicyAccepted": policy.upstream_request_allowed,
        "proxyRequestForwarded": false,
        "networkCalls": false,
        "runtimeExecution": false,
        "nodeModulesRequired": false,
        "recognizedFactory": config.recognized_factory,
        "requiredEnv": [DX_HTTP_JSON_ALLOWED_ORIGINS],
        "sourceOwned": true,
        "appOwnedBoundaries": [
            "The route must declare or provide allowed HTTPS origins before any upstream request can run.",
            "The safe DX-WWW interpreter validates route policy only; the deployment adapter owns network execution, response shaping, retries, and abuse limits.",
            "Secrets and authorization headers must be projected through app-owned environment policy, not inferred from route source.",
        ],
    })
}

fn http_json_helper_present(source: &str, handler_source: &str) -> bool {
    source.contains(HELPER_FACTORY)
        && (handler_source.contains(HELPER_FACTORY) || handler_source.contains(HELPER_RESPONSE))
}

fn configured_allowed_origins(
    request: &DxReactRouteHandlerRequest,
    config: &HttpJsonRouteConfig,
) -> Vec<String> {
    config
        .allowed_origins
        .iter()
        .filter_map(|origin| https_origin(origin))
        .chain(
            request
                .runtime_env
                .get(DX_HTTP_JSON_ALLOWED_ORIGINS)
                .into_iter()
                .flat_map(|value| value.split(',').filter_map(https_origin)),
        )
        .fold(Vec::<String>::new(), |mut origins, origin| {
            if !origins.contains(&origin) {
                origins.push(origin);
            }
            origins
        })
}

fn normalized_route_path(request: &DxReactRouteHandlerRequest) -> String {
    let path = request.path_for_match().trim_end_matches('/');
    if path.is_empty() {
        "/".to_string()
    } else {
        path.to_string()
    }
}

fn response_cache_control(cache: Option<&str>) -> String {
    match cache {
        Some("no-store") => "no-store, max-age=0".to_string(),
        _ => "private, max-age=0".to_string(),
    }
}

fn http_json_method(method: &str) -> bool {
    matches!(
        method,
        "GET" | "HEAD" | "POST" | "PUT" | "PATCH" | "DELETE" | "OPTIONS"
    )
}

fn string_field(source: &str, field: &str) -> Option<String> {
    let pattern = format!(r#"(?s)\b{}\s*:\s*["']([^"']+)["']"#, regex::escape(field));
    let regex = Regex::new(&pattern).ok()?;
    regex
        .captures(source)?
        .get(1)
        .map(|value| value.as_str().trim().to_string())
        .filter(|value| !value.is_empty())
}

fn number_field(source: &str, field: &str) -> Option<u64> {
    let pattern = format!(r#"(?s)\b{}\s*:\s*(\d+)"#, regex::escape(field));
    let regex = Regex::new(&pattern).ok()?;
    regex.captures(source)?.get(1)?.as_str().parse::<u64>().ok()
}

fn string_array_field(source: &str, field: &str) -> Vec<String> {
    let pattern = format!(r#"(?s)\b{}\s*:\s*\[([^\]]*)\]"#, regex::escape(field));
    let Some(regex) = Regex::new(&pattern).ok() else {
        return Vec::new();
    };
    let Some(values) = regex.captures(source).and_then(|capture| capture.get(1)) else {
        return Vec::new();
    };
    let Some(value_regex) = Regex::new(r#"["']([^"']+)["']"#).ok() else {
        return Vec::new();
    };

    value_regex
        .captures_iter(values.as_str())
        .filter_map(|capture| capture.get(1))
        .map(|value| value.as_str().trim().to_string())
        .filter(|value| !value.is_empty())
        .fold(Vec::<String>::new(), |mut values, value| {
            if !values.contains(&value) {
                values.push(value);
            }
            values
        })
}

fn dedupe_strings(values: impl IntoIterator<Item = String>) -> Vec<String> {
    values
        .into_iter()
        .fold(Vec::<String>::new(), |mut unique, value| {
            if !unique.contains(&value) {
                unique.push(value);
            }
            unique
        })
}

fn https_origin(value: &str) -> Option<String> {
    let trimmed = value.trim();
    let without_scheme = trimmed.strip_prefix("https://")?;
    let host = without_scheme
        .split(['/', '?', '#'])
        .next()
        .unwrap_or_default()
        .trim()
        .trim_end_matches('.');

    if host.is_empty()
        || host.contains('@')
        || host.contains('\\')
        || host.chars().any(char::is_whitespace)
    {
        return None;
    }

    Some(format!("https://{}", host.to_ascii_lowercase()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request(path: &str) -> DxReactRouteHandlerRequest {
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: path.to_string(),
            headers: BTreeMap::new(),
            body: Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        }
    }

    #[test]
    fn parses_http_json_route_options_from_source() {
        let source = r#"createDxHttpJsonRoute({
  target: "https://translate.googleapis.com/translate_a/single",
  method: "GET",
  allowedOrigins: ["https://translate.googleapis.com"],
  requiredSearchParams: ["q", "tl"],
  cache: "no-store",
  timeoutMs: 7000,
})"#;

        let config = HttpJsonRouteConfig::from_source(source, source);

        assert_eq!(
            config.target.as_deref(),
            Some("https://translate.googleapis.com/translate_a/single")
        );
        assert_eq!(config.method.as_deref(), Some("GET"));
        assert_eq!(config.allowed_origins, ["https://translate.googleapis.com"]);
        assert_eq!(config.required_search_params, ["q", "tl"]);
        assert_eq!(config.cache.as_deref(), Some("no-store"));
        assert_eq!(config.timeout_ms, Some(7000));
    }

    #[test]
    fn rejects_http_json_route_policy_without_allowlist() {
        let source = r#"createDxHttpJsonRoute({
  target: "https://translate.googleapis.com/translate_a/single",
  requiredSearchParams: ["q"],
})"#;
        let config = HttpJsonRouteConfig::from_source(source, source);

        let policy = http_json_route_policy(&request("/api/translate?q=dx"), &config);

        assert_eq!(policy.status, 501);
        assert_eq!(policy.status_label, "missing-allowed-origins");
        assert!(!policy.upstream_request_allowed);
    }

    #[test]
    fn accepts_http_json_route_policy_when_query_and_origin_match() {
        let source = r#"createDxHttpJsonRoute({
  target: "https://translate.googleapis.com/translate_a/single",
  allowedOrigins: ["https://translate.googleapis.com"],
  requiredSearchParams: ["q"],
})"#;
        let config = HttpJsonRouteConfig::from_source(source, source);

        let policy = http_json_route_policy(&request("/api/translate?q=dx"), &config);

        assert_eq!(policy.status, 202);
        assert!(policy.upstream_request_allowed);
        assert_eq!(
            policy.target_origin.as_deref(),
            Some("https://translate.googleapis.com")
        );
    }
}
