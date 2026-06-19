use std::collections::BTreeMap;

use serde_json::{Value, json};

use super::server_contract::{DxReactRouteHandlerRequest, DxReactRouteHandlerResponse};

const DOC_TITLE: &str = "DX Launch Docs";
const DOC_ROUTE: &str = "/docs";
const DOC_DESCRIPTION: &str =
    "Source-owned launch documentation powered by the Documentation System package.";
const DOC_SEARCH_TEXT: &str = "dx launch docs source-owned launch documentation documentation system app router docs route llm export openapi search static index forge";
const OPENAPI_ALLOWED_ORIGINS_ENV: &str = "DX_FUMADOCS_OPENAPI_ALLOWED_ORIGINS";

pub(super) fn fumadocs_openapi_proxy_route_handler_response(
    source: &str,
    expression: &str,
    request: &DxReactRouteHandlerRequest,
) -> Option<DxReactRouteHandlerResponse> {
    if !fumadocs_openapi_proxy_method(&request.method)
        || request.path_for_match().trim_end_matches('/') != "/api/openapi/proxy"
        || !source.contains("@/lib/fumadocs/openapi")
        || !source.contains("readDxFumadocsOpenAPIAllowedOrigins")
        || !expression.contains("dxFumadocsOpenAPI.createProxy")
    {
        return None;
    }

    let policy = openapi_proxy_status(request);

    Some(DxReactRouteHandlerResponse {
        status: policy.status,
        content_type: "application/json; charset=utf-8".to_string(),
        headers: BTreeMap::from([
            (
                "cache-control".to_string(),
                "no-store, max-age=0".to_string(),
            ),
            (
                "x-dx-fumadocs-openapi-proxy".to_string(),
                "source-owned-safe-interpreter".to_string(),
            ),
        ]),
        redirect_url: None,
        body: fumadocs_openapi_proxy_boundary_body(request, &policy),
        execution_model: "source-owned-fumadocs-openapi-proxy-boundary-interpreter".to_string(),
        lifecycle_scripts_executed: false,
    })
}

pub(super) fn fumadocs_search_route_handler_response(
    source: &str,
    expression: &str,
    request: &DxReactRouteHandlerRequest,
) -> Option<DxReactRouteHandlerResponse> {
    if request.method != "GET"
        || !source.contains("@/lib/fumadocs/search")
        || !source.contains("createDxFumadocsSearchApi")
    {
        return None;
    }

    let path = request.path_for_match().trim_end_matches('/');
    let mode = match (path, expression.trim()) {
        ("/api/search", "searchApi.GET") => "dynamic",
        ("/api/search-static", "searchApi.staticGET") => "static-index",
        _ => return None,
    };

    Some(DxReactRouteHandlerResponse {
        status: 200,
        content_type: "application/json; charset=utf-8".to_string(),
        headers: BTreeMap::from([
            (
                "cache-control".to_string(),
                "no-store, max-age=0".to_string(),
            ),
            (
                "x-dx-fumadocs-search".to_string(),
                "source-owned-safe-interpreter".to_string(),
            ),
        ]),
        redirect_url: None,
        body: fumadocs_search_body(path, mode, request),
        execution_model: "source-owned-fumadocs-search-interpreter".to_string(),
        lifecycle_scripts_executed: false,
    })
}

pub(super) fn fumadocs_llms_route_handler_response(
    source: &str,
    function_body: &str,
    request: &DxReactRouteHandlerRequest,
) -> Option<DxReactRouteHandlerResponse> {
    if request.method != "GET" || !source.contains("@/lib/fumadocs/llms") {
        return None;
    }

    let path = request.path_for_match().trim_end_matches('/');
    let mode = match path {
        "/llms.txt"
            if source.contains("createDxFumadocsLLMsIndex")
                && function_body.contains("createDxFumadocsLLMsIndex().index()") =>
        {
            "index"
        }
        "/llms-full.txt"
            if source.contains("getDxFumadocsLLMText")
                && source.contains("@/lib/fumadocs/source")
                && function_body.contains("pages.join(\"\\n\\n\")") =>
        {
            "full"
        }
        _ => return None,
    };

    Some(DxReactRouteHandlerResponse {
        status: 200,
        content_type: "text/plain; charset=utf-8".to_string(),
        headers: BTreeMap::from([
            (
                "cache-control".to_string(),
                "no-store, max-age=0".to_string(),
            ),
            (
                "x-dx-fumadocs-llms".to_string(),
                "source-owned-safe-interpreter".to_string(),
            ),
        ]),
        redirect_url: None,
        body: Value::String(fumadocs_llms_text(path, mode)),
        execution_model: "source-owned-fumadocs-llms-interpreter".to_string(),
        lifecycle_scripts_executed: false,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OpenApiProxyPolicy {
    status: u16,
    status_label: &'static str,
    message: &'static str,
    configured_allowed_origins: Vec<String>,
    requested_origin: Option<String>,
    upstream_request_allowed: bool,
}

fn fumadocs_openapi_proxy_boundary_body(
    request: &DxReactRouteHandlerRequest,
    policy: &OpenApiProxyPolicy,
) -> Value {
    let status = if policy.upstream_request_allowed {
        policy.status_label
    } else {
        "openapi-proxy-runtime-adapter-boundary"
    };

    json!({
        "ok": policy.upstream_request_allowed,
        "schema": "dx.fumadocs.openapi_proxy_boundary",
        "packageId": "content/fumadocs-next",
        "route": "/api/openapi/proxy",
        "method": request.method,
        "status": status,
        "policyStatus": policy.status_label,
        "httpStatus": policy.status,
        "message": policy.message,
        "requiredEnv": [OPENAPI_ALLOWED_ORIGINS_ENV],
        "allowedOriginsConfigured": !policy.configured_allowed_origins.is_empty(),
        "configuredAllowedOrigins": policy.configured_allowed_origins,
        "requestedOrigin": policy.requested_origin,
        "upstreamRequestAllowed": policy.upstream_request_allowed,
        "proxyPolicyAccepted": policy.upstream_request_allowed,
        "proxyRequestForwarded": false,
        "adapterBoundary": "fumadocs-openapi-createProxy-adapter-boundary",
        "runtimeExecution": false,
        "policyExecution": true,
        "fumadocsRuntime": false,
        "nodeModulesRequired": false,
        "networkCalls": false,
        "recognizedFactory": "dxFumadocsOpenAPI.createProxy",
        "filterRequest": "allowedOrigins.length > 0 && request.url.startsWith(\"https://\")",
        "source": {
            "routeFile": "app/api/openapi/proxy/route.ts",
            "configFile": "lib/fumadocs/openapi.ts",
            "schemaFile": "openapi/dx-launch.yaml",
        },
        "appOwnedBoundaries": [
            "OpenAPI proxy target origins must be configured by the app, not inferred from source.",
            "Auth header forwarding, request playground safety, and production request limits remain app-owned.",
            "Fumadocs createProxy runtime execution is adapter-boundary until DX owns a safe proxy adapter.",
        ],
    })
}

fn openapi_proxy_status(request: &DxReactRouteHandlerRequest) -> OpenApiProxyPolicy {
    let configured_allowed_origins = openapi_allowed_origins(request);
    if configured_allowed_origins.is_empty() {
        return OpenApiProxyPolicy {
            status: 501,
            status_label: "openapi-proxy-missing-allowed-origins",
            message: "DX-WWW recognized the Fumadocs OpenAPI proxy factory, but the app has not configured an allowed-origin policy.",
            configured_allowed_origins,
            requested_origin: None,
            upstream_request_allowed: false,
        };
    }

    let requested_url = requested_openapi_proxy_url(request);
    let Some(requested_url) = requested_url else {
        return OpenApiProxyPolicy {
            status: 400,
            status_label: "openapi-proxy-missing-upstream-url",
            message: "DX-WWW can evaluate the Fumadocs OpenAPI proxy policy, but the request did not include a url query/body field.",
            configured_allowed_origins,
            requested_origin: None,
            upstream_request_allowed: false,
        };
    };

    let Some(requested_origin) = https_origin(&requested_url) else {
        return OpenApiProxyPolicy {
            status: 400,
            status_label: "openapi-proxy-invalid-upstream-url",
            message: "DX-WWW rejected the Fumadocs OpenAPI proxy request before network execution because the upstream URL is not a clean HTTPS origin.",
            configured_allowed_origins,
            requested_origin: None,
            upstream_request_allowed: false,
        };
    };

    if !configured_allowed_origins
        .iter()
        .any(|origin| origin == &requested_origin)
    {
        return OpenApiProxyPolicy {
            status: 403,
            status_label: "openapi-proxy-origin-not-allowed",
            message: "DX-WWW rejected the Fumadocs OpenAPI proxy request before network execution because the requested origin is not in the app-owned allowlist.",
            configured_allowed_origins,
            requested_origin: Some(requested_origin),
            upstream_request_allowed: false,
        };
    }

    OpenApiProxyPolicy {
        status: 202,
        status_label: "proxy-policy-accepted",
        message: "DX-WWW accepted the Fumadocs OpenAPI proxy policy without forwarding a network request; the app-owned runtime adapter must perform the actual proxy execution.",
        configured_allowed_origins,
        requested_origin: Some(requested_origin),
        upstream_request_allowed: true,
    }
}

fn openapi_allowed_origins(request: &DxReactRouteHandlerRequest) -> Vec<String> {
    request
        .runtime_env
        .get(OPENAPI_ALLOWED_ORIGINS_ENV)
        .map(|value| {
            value
                .split(',')
                .filter_map(https_origin)
                .collect::<Vec<String>>()
        })
        .unwrap_or_default()
}

fn requested_openapi_proxy_url(request: &DxReactRouteHandlerRequest) -> Option<String> {
    search_param(request, "url").or_else(|| json_string_field(&request.body, "url"))
}

fn json_string_field(value: &Value, key: &str) -> Option<String> {
    value
        .as_object()
        .and_then(|object| object.get(key))
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
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

fn fumadocs_openapi_proxy_method(method: &str) -> bool {
    matches!(method, "GET" | "HEAD" | "PUT" | "POST" | "PATCH" | "DELETE")
}

fn fumadocs_search_body(path: &str, mode: &str, request: &DxReactRouteHandlerRequest) -> Value {
    let query = search_param(request, "query").unwrap_or_default();
    let limit = requested_limit(request);
    let include_doc = mode == "static-index" || matches_launch_doc(&query);
    let results = if include_doc && limit > 0 {
        vec![json!({
            "id": "content/docs/index",
            "title": DOC_TITLE,
            "route": DOC_ROUTE,
            "description": DOC_DESCRIPTION,
            "section": "Launch Surface",
            "score": if query.trim().is_empty() { 1.0 } else { 0.94 },
            "sourceFile": "content/docs/index.mdx",
            "frontmatter": {
                "status": "beta",
                "slug": "",
            },
        })]
    } else {
        Vec::new()
    };

    json!({
        "ok": true,
        "schema": "dx.fumadocs.search_receipt",
        "packageId": "content/fumadocs-next",
        "route": path,
        "mode": mode,
        "query": query,
        "language": "english",
        "queryParam": "query",
        "optionalParams": ["locale", "tag", "limit", "mode"],
        "adapterBoundary": "createFromSource-adapter-boundary",
        "runtimeExecution": false,
        "fumadocsRuntime": false,
        "nodeModulesRequired": false,
        "networkCalls": false,
        "staticIndex": mode == "static-index",
        "totalIndexedPages": 1,
        "limit": limit,
        "results": results,
        "source": {
            "contentDir": "content/docs",
            "sourceFile": "lib/fumadocs/source.ts",
            "searchConfigFile": "lib/fumadocs/search.ts",
            "routeContractFile": "lib/fumadocs/route-contract.ts",
        },
        "appOwnedBoundaries": [
            "Search UI placement, empty states, analytics, and abuse limits stay app-owned.",
            "The Fumadocs createFromSource runtime remains an adapter boundary until node_modules and deployment policy are app-owned.",
            "Multilingual/vector search policy is not claimed by this local DX receipt.",
        ],
    })
}

fn fumadocs_llms_text(path: &str, mode: &str) -> String {
    let scope = if mode == "full" {
        "Full source-owned documentation text projection."
    } else {
        "LLM index projection for the source-owned docs route."
    };

    format!(
        "# {DOC_TITLE}\n\n{DOC_DESCRIPTION}\n\n{scope}\n\n- route: {path}\n- packageId: content/fumadocs-next\n- sourceFile: content/docs/index.mdx\n- docsRoute: {DOC_ROUTE}\n- runtimeExecution: false\n- fumadocsRuntime: false\n- nodeModulesRequired: false\n- networkCalls: false\n- adapterBoundary: createFromSource-adapter-boundary\n\n{DOC_SEARCH_TEXT}\n",
    )
}

fn search_param(request: &DxReactRouteHandlerRequest, key: &str) -> Option<String> {
    request
        .search_params
        .get(key)
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .or_else(|| search_param_from_path(&request.path, key))
}

fn search_param_from_path(path: &str, key: &str) -> Option<String> {
    let (_, query_with_fragment) = path.split_once('?')?;
    let query = query_with_fragment
        .split_once('#')
        .map(|(query, _)| query)
        .unwrap_or(query_with_fragment);

    for pair in query.split('&').filter(|pair| !pair.is_empty()) {
        let (raw_key, raw_value) = pair.split_once('=').unwrap_or((pair, ""));
        if percent_decode_query_component(raw_key) != key {
            continue;
        }

        let value = percent_decode_query_component(raw_value);
        let value = value.trim();
        if !value.is_empty() {
            return Some(value.to_string());
        }
    }

    None
}

fn percent_decode_query_component(value: &str) -> String {
    let bytes = value.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;

    while index < bytes.len() {
        match bytes[index] {
            b'+' => {
                decoded.push(b' ');
                index += 1;
            }
            b'%' if index + 2 < bytes.len() => {
                if let Some(byte) = decode_hex_pair(bytes[index + 1], bytes[index + 2]) {
                    decoded.push(byte);
                    index += 3;
                } else {
                    decoded.push(bytes[index]);
                    index += 1;
                }
            }
            byte => {
                decoded.push(byte);
                index += 1;
            }
        }
    }

    String::from_utf8_lossy(&decoded).into_owned()
}

fn decode_hex_pair(high: u8, low: u8) -> Option<u8> {
    Some(hex_value(high)? << 4 | hex_value(low)?)
}

fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

fn requested_limit(request: &DxReactRouteHandlerRequest) -> usize {
    search_param(request, "limit")
        .and_then(|value| value.parse::<usize>().ok())
        .map(|value| value.clamp(1, 10))
        .unwrap_or(5)
}

fn matches_launch_doc(query: &str) -> bool {
    let query = query.trim().to_ascii_lowercase();
    query.is_empty()
        || query
            .split_whitespace()
            .any(|term| DOC_SEARCH_TEXT.contains(term))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn proxy_request(path: &str) -> DxReactRouteHandlerRequest {
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: path.to_string(),
            headers: BTreeMap::new(),
            body: Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::from([(
                OPENAPI_ALLOWED_ORIGINS_ENV.to_string(),
                "https://api.example.com".to_string(),
            )]),
        }
    }

    fn get_request(path: &str) -> DxReactRouteHandlerRequest {
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
    fn openapi_proxy_accepts_encoded_query_url_from_request_path() {
        let request = proxy_request(
            "https://docs.example.test/api/openapi/proxy?url=https%3A%2F%2Fapi.example.com%2Fopenapi.json#playground",
        );

        let policy = openapi_proxy_status(&request);

        assert_eq!(policy.status, 202);
        assert_eq!(
            policy.requested_origin.as_deref(),
            Some("https://api.example.com")
        );
        assert!(policy.upstream_request_allowed);
    }

    #[test]
    fn search_param_from_path_decodes_query_components_without_hiding_bad_escape_sequences() {
        let path = "/api/search?query=launch+docs&encoded=%7Bvalue%7D&bad=%zz";

        assert_eq!(
            search_param_from_path(path, "query").as_deref(),
            Some("launch docs")
        );
        assert_eq!(
            search_param_from_path(path, "encoded").as_deref(),
            Some("{value}")
        );
        assert_eq!(search_param_from_path(path, "bad").as_deref(), Some("%zz"));
    }

    #[test]
    fn fumadocs_search_accepts_absolute_url_for_endpoint_match() {
        let response = fumadocs_search_route_handler_response(
            r#"import { createDxFumadocsSearchApi } from "@/lib/fumadocs/search";

const searchApi = createDxFumadocsSearchApi();
export const GET = searchApi.GET;"#,
            "searchApi.GET",
            &DxReactRouteHandlerRequest {
                method: "GET".to_string(),
                path: "https://example.test/api/search?query=launch&limit=1#results".to_string(),
                headers: BTreeMap::new(),
                body: Value::Null,
                route_params: BTreeMap::new(),
                search_params: BTreeMap::from([
                    ("query".to_string(), "launch".to_string()),
                    ("limit".to_string(), "1".to_string()),
                ]),
                runtime_env: BTreeMap::new(),
            },
        )
        .expect("Fumadocs search route should match absolute request URL");

        assert_eq!(response.status, 200);
        assert_eq!(
            response.execution_model,
            "source-owned-fumadocs-search-interpreter"
        );
        assert_eq!(response.body["route"], "/api/search");
        assert_eq!(response.body["results"].as_array().map(Vec::len), Some(1));
        assert_eq!(response.body["networkCalls"], false);
    }

    #[test]
    fn fumadocs_llms_index_route_returns_source_owned_text() {
        let response = fumadocs_llms_route_handler_response(
            r#"import { createDxFumadocsLLMsIndex } from "@/lib/fumadocs/llms";

export function GET() {
  return new Response(createDxFumadocsLLMsIndex().index(), {
    headers: { "Content-Type": "text/plain; charset=utf-8" },
  });
}"#,
            r#"return new Response(createDxFumadocsLLMsIndex().index(), {
    headers: { "Content-Type": "text/plain; charset=utf-8" },
  });"#,
            &get_request("/llms.txt"),
        )
        .expect("Fumadocs llms index route");

        assert_eq!(response.status, 200);
        assert_eq!(response.content_type, "text/plain; charset=utf-8");
        assert_eq!(
            response.execution_model,
            "source-owned-fumadocs-llms-interpreter"
        );
        let body = response.body.as_str().expect("text body");
        assert!(body.contains("# DX Launch Docs"));
        assert!(body.contains("nodeModulesRequired: false"));
        assert!(body.contains("networkCalls: false"));
    }

    #[test]
    fn fumadocs_llms_full_route_returns_source_owned_text() {
        let response = fumadocs_llms_route_handler_response(
            r#"import { getDxFumadocsLLMText } from "@/lib/fumadocs/llms";
import { source } from "@/lib/fumadocs/source";

export async function GET() {
  const pages = await Promise.all(source.getPages().map(getDxFumadocsLLMText));

  return new Response(pages.join("\n\n"), {
    headers: { "Content-Type": "text/plain; charset=utf-8" },
  });
}"#,
            r#"const pages = await Promise.all(source.getPages().map(getDxFumadocsLLMText));

  return new Response(pages.join("\n\n"), {
    headers: { "Content-Type": "text/plain; charset=utf-8" },
  });"#,
            &get_request("/llms-full.txt"),
        )
        .expect("Fumadocs full llms route");

        assert_eq!(response.status, 200);
        assert_eq!(response.content_type, "text/plain; charset=utf-8");
        let body = response.body.as_str().expect("text body");
        assert!(body.contains("Full source-owned documentation text projection."));
        assert!(body.contains("adapterBoundary: createFromSource-adapter-boundary"));
    }
}
