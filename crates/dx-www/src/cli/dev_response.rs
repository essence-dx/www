use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};

use crate::hot_reload_protocol::{
    DX_HOT_RELOAD_EVENT_NAME, DX_HOT_RELOAD_EVENT_RETRY_MS, DX_HOT_RELOAD_EVENT_STREAM_ENDPOINT,
    DX_HOT_RELOAD_EVENT_STREAM_TRANSPORT, DX_HOT_RELOAD_TRANSPORT,
    dx_hot_reload_resource_from_path, dx_hot_reload_version_payload,
};

#[cfg(test)]
use super::dev_http::parse_http_request;
use super::dev_http::{
    DxCliHttpRequest, DxCliHttpResponse, apply_dev_cache_headers, dev_project_reload_token,
};
use super::devtools;
use crate::dev_feedback::dev_feedback_response;

pub(super) type DxDevRouteHandlerResponder =
    fn(&Path, &DxCliHttpRequest) -> Option<DxCliHttpResponse>;

pub(super) type DxDevPageResponder =
    fn(&PathBuf, &DxCliHttpRequest, &HashMap<String, String>) -> (String, String, String);

#[cfg(test)]
pub(super) fn handle_http_response(
    cwd: &PathBuf,
    request: &str,
    translations: &HashMap<String, String>,
    route_handler: DxDevRouteHandlerResponder,
    page_response: DxDevPageResponder,
) -> DxCliHttpResponse {
    let request = parse_http_request(request);
    handle_parsed_http_response(cwd, &request, translations, route_handler, page_response)
}

pub(super) fn handle_parsed_http_response(
    cwd: &PathBuf,
    request: &DxCliHttpRequest,
    translations: &HashMap<String, String>,
    route_handler: DxDevRouteHandlerResponder,
    page_response: DxDevPageResponder,
) -> DxCliHttpResponse {
    if let Some(response) = devtools::devtools_cli_response(cwd, request, true) {
        return response;
    }

    if is_hot_reload_version_request(request) {
        return hot_reload_version_response(cwd, request);
    }

    if is_hot_reload_event_stream_request(request) {
        return hot_reload_event_stream_response(cwd, request);
    }

    if let Some(response) = dev_feedback_cli_response(cwd, request) {
        return response;
    }

    if let Some(mut response) = route_handler(cwd, request) {
        apply_dev_cache_headers(request, &mut response);
        return response;
    }

    if !app_router_page_exists(cwd, request) {
        if let Some(mut response) = static_page_cli_response(cwd, request) {
            apply_dev_cache_headers(request, &mut response);
            return response;
        }
    }

    let (status, content_type, body) = page_response(cwd, request, translations);
    let mut response = DxCliHttpResponse {
        status,
        content_type,
        headers: BTreeMap::new(),
        body,
    };
    apply_dev_cache_headers(request, &mut response);
    response
}

fn app_router_page_exists(cwd: &Path, request: &DxCliHttpRequest) -> bool {
    if !matches!(request.method.as_str(), "GET" | "HEAD") {
        return false;
    }

    super::app_page_routes::route_match(cwd, &request.path)
        .is_some_and(|route_match| route_match.path.exists())
}

fn static_page_cli_response(cwd: &Path, request: &DxCliHttpRequest) -> Option<DxCliHttpResponse> {
    if !matches!(request.method.as_str(), "GET" | "HEAD") {
        return None;
    }

    for relative in static_page_candidates(&request.path)? {
        for root in ["pages"] {
            let file = cwd.join(root).join(&relative);
            if !file.is_file() {
                continue;
            }
            let body = if request.method == "HEAD" {
                String::new()
            } else {
                std::fs::read_to_string(&file).ok()?
            };
            return Some(DxCliHttpResponse {
                status: "200 OK".to_string(),
                content_type: "text/html; charset=utf-8".to_string(),
                headers: BTreeMap::new(),
                body,
            });
        }
    }

    None
}

fn static_page_candidates(request_path: &str) -> Option<Vec<PathBuf>> {
    let path = super::dev_http::dev_lookup_path(request_path);
    let path = path.trim_end_matches('/');
    if path.is_empty() || path == "/" {
        return Some(vec![PathBuf::from("index.html")]);
    }

    let relative = path.strip_prefix('/')?;
    let mut route = PathBuf::new();
    for segment in relative.split('/') {
        if segment.is_empty()
            || segment == "."
            || segment == ".."
            || segment.contains('\\')
            || segment.contains(':')
        {
            return None;
        }
        route.push(segment);
    }

    let mut direct = route.clone();
    direct.set_extension("html");

    let mut nested = route;
    nested.push("index.html");

    Some(vec![direct, nested])
}

fn is_hot_reload_version_request(request: &DxCliHttpRequest) -> bool {
    request.method == "GET"
        && request
            .path
            .split('?')
            .next()
            .is_some_and(|path| path == "/_dx/hot-reload/version")
}

fn is_hot_reload_event_stream_request(request: &DxCliHttpRequest) -> bool {
    request.method == "GET"
        && request
            .path
            .split('?')
            .next()
            .is_some_and(|path| path == DX_HOT_RELOAD_EVENT_STREAM_ENDPOINT)
}

fn hot_reload_version_response(cwd: &Path, request: &DxCliHttpRequest) -> DxCliHttpResponse {
    DxCliHttpResponse {
        status: "200 OK".to_string(),
        content_type: "application/json; charset=utf-8".to_string(),
        headers: BTreeMap::from([
            ("cache-control".to_string(), "no-store".to_string()),
            ("x-dx-hot-reload".to_string(), "poll".to_string()),
        ]),
        body: dx_hot_reload_version_payload(
            true,
            dev_project_reload_token(cwd),
            0,
            &dx_hot_reload_resource_from_path(&request.path),
        )
        .to_string(),
    }
}

fn hot_reload_event_stream_response(cwd: &Path, request: &DxCliHttpRequest) -> DxCliHttpResponse {
    let resource = dx_hot_reload_resource_from_path(&request.path);
    let mut payload =
        dx_hot_reload_version_payload(true, dev_project_reload_token(cwd), 0, &resource);
    let event_stream = serde_json::json!({
        "endpoint": DX_HOT_RELOAD_EVENT_STREAM_ENDPOINT,
        "event": DX_HOT_RELOAD_EVENT_NAME,
        "retry_ms": DX_HOT_RELOAD_EVENT_RETRY_MS,
        "transport": DX_HOT_RELOAD_EVENT_STREAM_TRANSPORT,
        "fallback_transport": DX_HOT_RELOAD_TRANSPORT,
        "initial": true
    });
    if let Some(payload) = payload.as_object_mut() {
        payload.insert(
            "transport".to_string(),
            serde_json::json!(DX_HOT_RELOAD_EVENT_STREAM_TRANSPORT),
        );
        payload.insert(
            "fallback_transport".to_string(),
            serde_json::json!(DX_HOT_RELOAD_TRANSPORT),
        );
        payload.insert("event_stream".to_string(), event_stream.clone());
        payload.insert("event_stream_initial".to_string(), serde_json::json!(true));
        if let Some(receipt) = payload
            .get_mut("receipt")
            .and_then(serde_json::Value::as_object_mut)
        {
            receipt.insert("event_stream".to_string(), event_stream);
            receipt.insert(
                "fallback_transport".to_string(),
                serde_json::json!(DX_HOT_RELOAD_TRANSPORT),
            );
        }
    }

    DxCliHttpResponse {
        status: "200 OK".to_string(),
        content_type: "text/event-stream; charset=utf-8".to_string(),
        headers: BTreeMap::from([
            ("cache-control".to_string(), "no-store".to_string()),
            ("connection".to_string(), "keep-alive".to_string()),
            (
                "x-dx-hot-reload".to_string(),
                DX_HOT_RELOAD_EVENT_STREAM_TRANSPORT.to_string(),
            ),
        ]),
        body: format!(
            "retry: {DX_HOT_RELOAD_EVENT_RETRY_MS}\nevent: {DX_HOT_RELOAD_EVENT_NAME}\ndata: {}\n\n",
            payload
        ),
    }
}

fn dev_feedback_cli_response(cwd: &Path, request: &DxCliHttpRequest) -> Option<DxCliHttpResponse> {
    let response = dev_feedback_response(
        cwd,
        true,
        &request.path,
        &request.method,
        request.method != "HEAD",
    )?;

    Some(DxCliHttpResponse {
        status: http_status_line(response.status),
        content_type: response.content_type.to_string(),
        headers: response.headers,
        body: String::from_utf8_lossy(&response.body).into_owned(),
    })
}

fn http_status_line(status: u16) -> String {
    let reason = match status {
        200 => "OK",
        201 => "Created",
        202 => "Accepted",
        204 => "No Content",
        301 => "Moved Permanently",
        302 => "Found",
        303 => "See Other",
        307 => "Temporary Redirect",
        308 => "Permanent Redirect",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        409 => "Conflict",
        422 => "Unprocessable Entity",
        500 => "Internal Server Error",
        503 => "Service Unavailable",
        _ => "OK",
    };
    format!("{status} {reason}")
}
