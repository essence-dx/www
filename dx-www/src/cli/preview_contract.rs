use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};

use super::build_observability::PRODUCTION_OBSERVABILITY_JSON;
use super::dev_http::{DxCliHttpRequest, parse_http_request};

pub(super) type DxProductionServerActionExecutor =
    fn(&Path, &serde_json::Value, &DxCliHttpRequest, &str) -> Result<String, String>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxProductionPreviewResponse {
    pub(super) status: String,
    pub(super) content_type: String,
    pub(super) content_encoding: Option<String>,
    pub(super) cache_control: Option<String>,
    pub(super) last_modified: Option<String>,
    pub(super) vary_accept_encoding: bool,
    pub(super) allow: Option<String>,
    pub(super) contract_path: String,
    pub(super) body: Vec<u8>,
}

#[derive(Clone, Debug)]
pub(super) struct DxProductionPreviewCache {
    build_dir: PathBuf,
    contract: serde_json::Value,
    files: BTreeMap<String, Vec<u8>>,
}

pub(super) fn load_production_preview_cache(
    build_dir: &Path,
) -> Result<DxProductionPreviewCache, String> {
    let contract = read_deploy_adapter_contract(build_dir)?;
    let mut files = BTreeMap::new();
    for relative_path in production_contract_static_paths(&contract) {
        let path = safe_production_contract_file(build_dir, &relative_path)?;
        let body = std::fs::read(&path).map_err(|error| {
            format!(
                "Failed to read contract file {relative_path} from deploy-adapter.json: {error}"
            )
        })?;
        files.insert(relative_path, body);
    }
    Ok(DxProductionPreviewCache {
        build_dir: build_dir.to_path_buf(),
        contract,
        files,
    })
}

pub(super) fn production_contract_wire_response(
    build_dir: &Path,
    request: &str,
    execute_server_action: DxProductionServerActionExecutor,
) -> Vec<u8> {
    let request_meta = parse_http_request(request);
    let omit_body = request_meta.method == "HEAD";
    let if_none_match = request_meta
        .headers
        .get("if-none-match")
        .map(String::as_str);
    let if_modified_since = request_meta
        .headers
        .get("if-modified-since")
        .map(String::as_str);
    let range = request_meta.headers.get("range").map(String::as_str);
    let if_range = request_meta.headers.get("if-range").map(String::as_str);
    let response =
        handle_production_contract_http_request(build_dir, request, execute_server_action)
            .unwrap_or_else(|error| DxProductionPreviewResponse {
                status: "500 Internal Server Error".to_string(),
                content_type: "text/plain; charset=utf-8".to_string(),
                content_encoding: None,
                cache_control: Some("no-store".to_string()),
                last_modified: None,
                vary_accept_encoding: false,
                allow: None,
                contract_path: "deploy-adapter.json".to_string(),
                body: error.into_bytes(),
            });
    wire_response_bytes(
        response,
        &request_meta.method,
        omit_body,
        if_none_match,
        if_modified_since,
        range,
        if_range,
        "close",
    )
}

pub(super) fn production_contract_wire_response_cached(
    cache: &DxProductionPreviewCache,
    request: &str,
    execute_server_action: DxProductionServerActionExecutor,
) -> Vec<u8> {
    production_contract_wire_response_cached_with_connection(
        cache,
        request,
        execute_server_action,
        "close",
    )
}

pub(super) fn production_contract_wire_response_cached_with_connection(
    cache: &DxProductionPreviewCache,
    request: &str,
    execute_server_action: DxProductionServerActionExecutor,
    connection: &str,
) -> Vec<u8> {
    let request_meta = parse_http_request(request);
    let omit_body = request_meta.method == "HEAD";
    let if_none_match = request_meta
        .headers
        .get("if-none-match")
        .map(String::as_str);
    let if_modified_since = request_meta
        .headers
        .get("if-modified-since")
        .map(String::as_str);
    let range = request_meta.headers.get("range").map(String::as_str);
    let if_range = request_meta.headers.get("if-range").map(String::as_str);
    let response =
        handle_production_contract_http_request_cached(cache, request, execute_server_action)
            .unwrap_or_else(|error| DxProductionPreviewResponse {
                status: "500 Internal Server Error".to_string(),
                content_type: "text/plain; charset=utf-8".to_string(),
                content_encoding: None,
                cache_control: Some("no-store".to_string()),
                last_modified: None,
                vary_accept_encoding: false,
                allow: None,
                contract_path: "deploy-adapter.json".to_string(),
                body: error.into_bytes(),
            });
    wire_response_bytes(
        response,
        &request_meta.method,
        omit_body,
        if_none_match,
        if_modified_since,
        range,
        if_range,
        connection,
    )
}

#[cfg(feature = "dev-server")]
pub(super) fn production_contract_axum_response_cached(
    cache: &DxProductionPreviewCache,
    request: crate::dev::axum_server::DxDevAxumRequest,
    execute_server_action: DxProductionServerActionExecutor,
) -> crate::dev::axum_server::DxDevAxumResponse {
    let raw_request = production_axum_request_to_raw_http(&request);
    let wire = production_contract_wire_response_cached(cache, &raw_request, execute_server_action);
    production_wire_response_to_axum_response(&wire)
}

#[cfg(feature = "dev-server")]
fn production_axum_request_to_raw_http(
    request: &crate::dev::axum_server::DxDevAxumRequest,
) -> String {
    let mut head = format!("{} {} HTTP/1.1\r\n", request.method, request.path);
    let mut has_content_length = false;
    for (name, value) in &request.headers {
        if name.eq_ignore_ascii_case("content-length") {
            has_content_length = true;
        }
        head.push_str(name);
        head.push_str(": ");
        head.push_str(value);
        head.push_str("\r\n");
    }
    if !request.body.is_empty() && !has_content_length {
        head.push_str("content-length: ");
        head.push_str(&request.body.len().to_string());
        head.push_str("\r\n");
    }
    head.push_str("\r\n");

    let mut bytes = head.into_bytes();
    bytes.extend_from_slice(&request.body);
    String::from_utf8_lossy(&bytes).to_string()
}

#[cfg(feature = "dev-server")]
fn production_wire_response_to_axum_response(
    wire: &[u8],
) -> crate::dev::axum_server::DxDevAxumResponse {
    let header_end = wire
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .map(|position| position + 4)
        .unwrap_or(wire.len());
    let header_bytes = &wire[..header_end];
    let body = if header_end < wire.len() {
        wire[header_end..].to_vec()
    } else {
        Vec::new()
    };
    let header_text = String::from_utf8_lossy(header_bytes);
    let mut lines = header_text.lines();
    let status = lines
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|status| status.parse::<u16>().ok())
        .unwrap_or(500);
    let mut content_type = "application/octet-stream".to_string();
    let mut headers = BTreeMap::new();
    for line in lines {
        let Some((name, value)) = line.split_once(':') else {
            continue;
        };
        let name = name.trim().to_ascii_lowercase();
        let value = value.trim().to_string();
        if name == "content-type" {
            content_type = value;
        } else if !name.is_empty() {
            headers.insert(name, value);
        }
    }

    crate::dev::axum_server::DxDevAxumResponse {
        status,
        content_type,
        headers,
        body: body.into(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ByteRangeSelection {
    Satisfiable(usize, usize),
    Unsatisfiable,
    Unsupported,
}

#[allow(clippy::too_many_arguments)]
fn wire_response_bytes(
    response: DxProductionPreviewResponse,
    request_method: &str,
    omit_body: bool,
    if_none_match: Option<&str>,
    if_modified_since: Option<&str>,
    range: Option<&str>,
    if_range: Option<&str>,
    connection: &str,
) -> Vec<u8> {
    let cache_control = response.cache_control.as_deref().unwrap_or("no-store");
    let etag = production_response_etag(&response);
    let last_modified = response.last_modified.as_deref();
    let not_modified = request_method_allows_conditional_not_modified(request_method)
        && response.status == "200 OK"
        && if let Some(header) = if_none_match {
            etag_matches(header, &etag)
        } else {
            if_modified_since
                .zip(last_modified)
                .is_some_and(|(header, modified)| http_date_not_after(modified, header))
        };
    let byte_range = match range {
        Some(header)
            if !not_modified
                && response.status == "200 OK"
                && if_range_allows_range(if_range, &etag, last_modified) =>
        {
            parse_single_byte_range(header, response.body.len())
        }
        _ => ByteRangeSelection::Unsupported,
    };
    let status = if not_modified {
        "304 Not Modified"
    } else {
        match byte_range {
            ByteRangeSelection::Satisfiable(_, _) => "206 Partial Content",
            ByteRangeSelection::Unsatisfiable => "416 Range Not Satisfiable",
            ByteRangeSelection::Unsupported => response.status.as_str(),
        }
    };
    let body = match byte_range {
        ByteRangeSelection::Satisfiable(start, end) => &response.body[start..=end],
        ByteRangeSelection::Unsatisfiable => &[],
        ByteRangeSelection::Unsupported => response.body.as_slice(),
    };
    let content_length = if not_modified { 0 } else { body.len() };
    let content_range = match byte_range {
        ByteRangeSelection::Satisfiable(start, end) => {
            format!(
                "Content-Range: bytes {start}-{end}/{}\r\n",
                response.body.len()
            )
        }
        ByteRangeSelection::Unsatisfiable => {
            format!("Content-Range: bytes */{}\r\n", response.body.len())
        }
        ByteRangeSelection::Unsupported => String::new(),
    };
    let content_encoding = response
        .content_encoding
        .as_deref()
        .map(|encoding| format!("Content-Encoding: {encoding}\r\n"))
        .unwrap_or_default();
    let last_modified = last_modified
        .map(|value| format!("Last-Modified: {value}\r\n"))
        .unwrap_or_default();
    let vary = if response.vary_accept_encoding || response.content_encoding.is_some() {
        "Vary: Accept-Encoding\r\n"
    } else {
        ""
    };
    let allow = response
        .allow
        .as_deref()
        .map(|value| format!("Allow: {value}\r\n"))
        .unwrap_or_default();
    let header = format!(
        "HTTP/1.1 {}\r\nContent-Type: {}\r\n{}{}{}Cache-Control: {}\r\nETag: {}\r\n{}Accept-Ranges: bytes\r\n{}Content-Length: {}\r\nConnection: {}\r\n\r\n",
        status,
        response.content_type,
        content_encoding,
        vary,
        allow,
        cache_control,
        etag,
        last_modified,
        content_range,
        content_length,
        connection,
    );

    let mut bytes = header.into_bytes();
    if !omit_body && !not_modified && byte_range != ByteRangeSelection::Unsatisfiable {
        bytes.extend_from_slice(body);
    }
    bytes
}

fn request_method_allows_conditional_not_modified(method: &str) -> bool {
    method.eq_ignore_ascii_case("GET") || method.eq_ignore_ascii_case("HEAD")
}

fn production_response_etag(response: &DxProductionPreviewResponse) -> String {
    let hash = blake3::hash(&response.body).to_hex().to_string();
    format!(r#""dx-{}-{}""#, &hash[..16], response.body.len())
}

fn etag_matches(if_none_match: &str, etag: &str) -> bool {
    if_none_match.split(',').any(|candidate| {
        let candidate = candidate.trim();
        candidate == "*"
            || candidate == etag
            || candidate
                .strip_prefix("W/")
                .is_some_and(|weak| weak.trim() == etag)
    })
}

fn if_range_allows_range(if_range: Option<&str>, etag: &str, last_modified: Option<&str>) -> bool {
    let Some(if_range) = if_range.map(str::trim).filter(|value| !value.is_empty()) else {
        return true;
    };
    strong_etag_matches(if_range, etag)
        || (!if_range.starts_with("W/")
            && last_modified.is_some_and(|modified| http_date_not_after(modified, if_range)))
}

fn strong_etag_matches(candidate: &str, etag: &str) -> bool {
    let candidate = candidate.trim();
    !candidate.starts_with("W/") && candidate == etag
}

fn http_date_not_after(candidate: &str, boundary: &str) -> bool {
    let Ok(candidate) = DateTime::parse_from_rfc2822(candidate) else {
        return candidate.trim() == boundary.trim();
    };
    let Ok(boundary) = DateTime::parse_from_rfc2822(boundary) else {
        return false;
    };
    candidate <= boundary
}

fn parse_single_byte_range(range: &str, body_len: usize) -> ByteRangeSelection {
    if body_len == 0 {
        return ByteRangeSelection::Unsatisfiable;
    }
    let Some(range) = range.trim().strip_prefix("bytes=") else {
        return ByteRangeSelection::Unsupported;
    };
    if range.contains(',') {
        return ByteRangeSelection::Unsupported;
    }
    let Some((start, end)) = range.split_once('-') else {
        return ByteRangeSelection::Unsupported;
    };
    if start.is_empty() {
        let Ok(suffix_len) = end.trim().parse::<usize>() else {
            return ByteRangeSelection::Unsupported;
        };
        let suffix_len = suffix_len.min(body_len);
        if suffix_len == 0 {
            return ByteRangeSelection::Unsatisfiable;
        }
        return ByteRangeSelection::Satisfiable(body_len - suffix_len, body_len - 1);
    }
    let Ok(start) = start.trim().parse::<usize>() else {
        return ByteRangeSelection::Unsupported;
    };
    if start >= body_len {
        return ByteRangeSelection::Unsatisfiable;
    }
    let end = if end.trim().is_empty() {
        body_len - 1
    } else {
        let Ok(end) = end.trim().parse::<usize>() else {
            return ByteRangeSelection::Unsupported;
        };
        end.min(body_len - 1)
    };
    if start <= end {
        ByteRangeSelection::Satisfiable(start, end)
    } else {
        ByteRangeSelection::Unsatisfiable
    }
}

pub(super) fn handle_production_contract_http_request(
    build_dir: &Path,
    request: &str,
    execute_server_action: DxProductionServerActionExecutor,
) -> Result<DxProductionPreviewResponse, String> {
    let request = parse_http_request(request);
    let contract = read_deploy_adapter_contract(build_dir)?;
    if let Some(response) = production_contract_server_action_response(
        build_dir,
        &contract,
        &request,
        execute_server_action,
    )? {
        return Ok(response);
    }

    handle_production_contract_request_with_contract(
        build_dir,
        &contract,
        &request.path,
        &request.method,
        request.headers.get("accept-encoding").map(String::as_str),
    )
}

fn handle_production_contract_http_request_cached(
    cache: &DxProductionPreviewCache,
    request: &str,
    execute_server_action: DxProductionServerActionExecutor,
) -> Result<DxProductionPreviewResponse, String> {
    let request = parse_http_request(request);
    if let Some(response) = production_contract_server_action_response(
        &cache.build_dir,
        &cache.contract,
        &request,
        execute_server_action,
    )? {
        return Ok(response);
    }

    handle_production_contract_request_with_cached_files(
        &cache.build_dir,
        &cache.contract,
        &request.path,
        &request.method,
        request.headers.get("accept-encoding").map(String::as_str),
        Some(&cache.files),
    )
}

#[cfg(test)]
pub(super) fn handle_production_contract_request(
    build_dir: &Path,
    request_path: &str,
) -> Result<DxProductionPreviewResponse, String> {
    let contract = read_deploy_adapter_contract(build_dir)?;
    handle_production_contract_request_with_contract(
        build_dir,
        &contract,
        request_path,
        "GET",
        None,
    )
}

fn handle_production_contract_request_with_contract(
    build_dir: &Path,
    contract: &serde_json::Value,
    request_path: &str,
    request_method: &str,
    accept_encoding: Option<&str>,
) -> Result<DxProductionPreviewResponse, String> {
    handle_production_contract_request_with_cached_files(
        build_dir,
        contract,
        request_path,
        request_method,
        accept_encoding,
        None,
    )
}

fn handle_production_contract_request_with_cached_files(
    build_dir: &Path,
    contract: &serde_json::Value,
    request_path: &str,
    request_method: &str,
    accept_encoding: Option<&str>,
    cached_files: Option<&BTreeMap<String, Vec<u8>>>,
) -> Result<DxProductionPreviewResponse, String> {
    let path = normalize_preview_contract_path(request_path)?;

    if let Some(response) =
        production_contract_ready_response(build_dir, contract, &path, request_method)?
    {
        return Ok(response);
    }

    if let Some(response) =
        production_contract_observability_response(build_dir, contract, &path, request_method)?
    {
        return Ok(response);
    }

    if let Some(response) = production_contract_health_response(contract, &path, request_method)? {
        return Ok(response);
    }

    if let Some(route) = contract["routes"].as_array().and_then(|routes| {
        routes
            .iter()
            .find(|route| route["path"].as_str() == Some(path.as_str()))
    }) {
        if let Some(response) = production_contract_static_method_response(
            &path,
            request_method,
            "static-route-method-not-allowed",
        )? {
            return Ok(response);
        }
        let html = route["html"]
            .as_str()
            .ok_or_else(|| format!("Route {path} is missing html in deploy-adapter.json"))?;
        return read_production_contract_file(
            build_dir,
            html,
            production_contract_content_type(html),
            Some(production_contract_cache_control(contract, html)),
            false,
            cached_files,
        );
    }

    let asset_path = path.trim_start_matches('/');
    if let Some((selected_asset_path, asset, vary_accept_encoding)) =
        production_contract_immutable_asset_for_request(contract, asset_path, accept_encoding)
    {
        if let Some(response) = production_contract_static_method_response(
            &path,
            request_method,
            "static-asset-method-not-allowed",
        )? {
            return Ok(response);
        }
        let cache_control = asset["cache_control"]
            .as_str()
            .unwrap_or("public, max-age=31536000, immutable")
            .to_string();
        return read_production_contract_file(
            build_dir,
            &selected_asset_path,
            production_contract_content_type(&selected_asset_path),
            Some(cache_control),
            vary_accept_encoding,
            cached_files,
        );
    }

    Ok(production_contract_not_found(&path))
}

const PRODUCTION_STATIC_ALLOWED_METHODS: &str = "GET, HEAD, OPTIONS";

fn production_contract_static_method_response(
    path: &str,
    request_method: &str,
    error_code: &str,
) -> Result<Option<DxProductionPreviewResponse>, String> {
    match request_method {
        "GET" | "HEAD" => Ok(None),
        "OPTIONS" => Ok(Some(production_contract_static_options_response())),
        method => {
            production_contract_static_method_guard_response(path, method, error_code).map(Some)
        }
    }
}

fn production_contract_static_options_response() -> DxProductionPreviewResponse {
    DxProductionPreviewResponse {
        status: "204 No Content".to_string(),
        content_type: "text/plain; charset=utf-8".to_string(),
        content_encoding: None,
        cache_control: Some("no-store".to_string()),
        last_modified: None,
        vary_accept_encoding: false,
        allow: Some(PRODUCTION_STATIC_ALLOWED_METHODS.to_string()),
        contract_path: "deploy-adapter.json".to_string(),
        body: Vec::new(),
    }
}

fn production_contract_static_method_guard_response(
    path: &str,
    method: &str,
    error_code: &str,
) -> Result<DxProductionPreviewResponse, String> {
    let body = serde_json::json!({
        "ok": false,
        "error": error_code,
        "path": path,
        "method": method,
        "allowed_methods": ["GET", "HEAD", "OPTIONS"],
    });
    Ok(DxProductionPreviewResponse {
        status: "405 Method Not Allowed".to_string(),
        content_type: "application/json; charset=utf-8".to_string(),
        content_encoding: None,
        cache_control: Some("no-store".to_string()),
        last_modified: None,
        vary_accept_encoding: false,
        allow: Some(PRODUCTION_STATIC_ALLOWED_METHODS.to_string()),
        contract_path: "deploy-adapter.json".to_string(),
        body: serde_json::to_vec(&body).map_err(|error| {
            format!("Failed to serialize static method guard response: {error}")
        })?,
    })
}

fn production_contract_server_action_response(
    build_dir: &Path,
    contract: &serde_json::Value,
    request: &DxCliHttpRequest,
    execute_server_action: DxProductionServerActionExecutor,
) -> Result<Option<DxProductionPreviewResponse>, String> {
    let path = normalize_preview_contract_path(&request.path)?;
    let Some(action) = contract["server_actions"].as_array().and_then(|actions| {
        actions
            .iter()
            .find(|action| action["endpoint"].as_str() == Some(path.as_str()))
    }) else {
        return Ok(None);
    };
    let action_id = action["action_id"].as_str().ok_or_else(|| {
        format!("Server action {path} is missing action_id in deploy-adapter.json")
    })?;
    let method = action["method"].as_str().unwrap_or("POST");
    if request.method == "OPTIONS" {
        return Ok(Some(server_action_options_response(path, method)?));
    }
    if request.method != method {
        return Ok(Some(server_action_method_not_allowed_response(
            path, method, action_id,
        )?));
    }
    let body = match execute_server_action(build_dir, contract, request, action_id) {
        Ok(body) => body,
        Err(error) => {
            return Ok(Some(server_action_failed_response(
                path, action_id, &error,
            )?));
        }
    };
    Ok(Some(DxProductionPreviewResponse {
        status: "200 OK".to_string(),
        content_type: "application/json; charset=utf-8".to_string(),
        content_encoding: None,
        cache_control: Some("no-store".to_string()),
        last_modified: None,
        vary_accept_encoding: false,
        allow: None,
        contract_path: "deploy-adapter.json".to_string(),
        body: body.into_bytes(),
    }))
}

fn server_action_options_response(
    _path: String,
    method: &str,
) -> Result<DxProductionPreviewResponse, String> {
    Ok(DxProductionPreviewResponse {
        status: "204 No Content".to_string(),
        content_type: "text/plain; charset=utf-8".to_string(),
        content_encoding: None,
        cache_control: Some("no-store".to_string()),
        last_modified: None,
        vary_accept_encoding: false,
        allow: Some(server_action_allowed_methods(method)),
        contract_path: "deploy-adapter.json".to_string(),
        body: Vec::new(),
    })
}

fn server_action_method_not_allowed_response(
    path: String,
    method: &str,
    action_id: &str,
) -> Result<DxProductionPreviewResponse, String> {
    let body = serde_json::json!({
        "error": "server-action-method-not-allowed",
        "message": format!("Server action endpoint {path} requires {method}"),
        "action_id": action_id,
        "method": method,
        "allowed_methods": [method, "OPTIONS"],
        "replay_safe": false,
    });
    Ok(DxProductionPreviewResponse {
        status: "405 Method Not Allowed".to_string(),
        content_type: "application/json; charset=utf-8".to_string(),
        content_encoding: None,
        cache_control: Some("no-store".to_string()),
        last_modified: None,
        vary_accept_encoding: false,
        allow: Some(server_action_allowed_methods(method)),
        contract_path: "deploy-adapter.json".to_string(),
        body: serde_json::to_vec(&body)
            .map_err(|error| format!("Failed to serialize server action 405 response: {error}"))?,
    })
}

fn server_action_allowed_methods(method: &str) -> String {
    format!("{method}, OPTIONS")
}

fn server_action_failed_response(
    path: String,
    action_id: &str,
    error: &str,
) -> Result<DxProductionPreviewResponse, String> {
    let body = serde_json::json!({
        "error": "server-action-failed",
        "message": super::server_action_runtime::server_action_redacted_error(error),
        "path": path,
        "action_id": action_id,
        "replay_safe": false,
        "receipt_written": false,
    });
    Ok(DxProductionPreviewResponse {
        status: super::server_action_runtime::server_action_error_status(error).to_string(),
        content_type: "application/json; charset=utf-8".to_string(),
        content_encoding: None,
        cache_control: Some("no-store".to_string()),
        last_modified: None,
        vary_accept_encoding: false,
        allow: None,
        contract_path: "deploy-adapter.json".to_string(),
        body: serde_json::to_vec(&body).map_err(|error| {
            format!("Failed to serialize server action error response: {error}")
        })?,
    })
}

pub(super) fn read_deploy_adapter_contract(build_dir: &Path) -> Result<serde_json::Value, String> {
    let contract_path = build_dir.join("deploy-adapter.json");
    let contract_json = std::fs::read_to_string(&contract_path)
        .map_err(|error| format!("Failed to read deploy-adapter.json: {error}"))?;
    serde_json::from_str(&contract_json)
        .map_err(|error| format!("Failed to parse deploy-adapter.json: {error}"))
}

fn read_production_observability_contract(
    build_dir: &Path,
    contract: &serde_json::Value,
) -> Result<serde_json::Value, String> {
    let metadata_path = contract["observability"]["metadata_path"]
        .as_str()
        .unwrap_or(PRODUCTION_OBSERVABILITY_JSON);
    let path = safe_production_contract_file(build_dir, metadata_path)?;
    let json = std::fs::read_to_string(&path).map_err(|error| {
        format!("Failed to read {metadata_path} from deploy-adapter.json: {error}")
    })?;
    serde_json::from_str(&json).map_err(|error| format!("Failed to parse {metadata_path}: {error}"))
}

pub(super) fn normalize_preview_contract_path(request_path: &str) -> Result<String, String> {
    let path = request_path.split('?').next().unwrap_or(request_path);
    if !path.starts_with('/') {
        return Err(format!(
            "Production preview request path must start with /: {request_path}"
        ));
    }
    if path.contains('\\') {
        return Err(format!(
            "Production preview request path cannot contain backslashes: {request_path}"
        ));
    }
    let normalized = path.trim_end_matches('/');
    let normalized = if normalized.is_empty() {
        "/".to_string()
    } else {
        normalized.to_string()
    };
    if normalized
        .split('/')
        .any(|segment| segment == "." || segment == "..")
    {
        return Err(format!(
            "Production preview request path cannot escape the contract: {request_path}"
        ));
    }
    Ok(normalized)
}

fn production_contract_health_response(
    contract: &serde_json::Value,
    path: &str,
    request_method: &str,
) -> Result<Option<DxProductionPreviewResponse>, String> {
    let allowed_methods = production_contract_health_allowed_methods(contract, path);
    let path_matched = contract["health_checks"].as_array().is_some_and(|checks| {
        checks
            .iter()
            .any(|check| check["path"].as_str() == Some(path))
    });
    if path_matched && request_method == "OPTIONS" {
        return Ok(Some(production_contract_health_options_response(
            allowed_methods,
        )));
    }
    let Some(check) = contract["health_checks"].as_array().and_then(|checks| {
        checks.iter().find(|check| {
            check["path"].as_str() == Some(path)
                && production_contract_health_method_matches(
                    check["method"].as_str().unwrap_or("GET"),
                    request_method,
                )
        })
    }) else {
        if path_matched {
            return Ok(Some(
                production_contract_health_method_not_allowed_response(
                    path,
                    request_method,
                    allowed_methods,
                )?,
            ));
        }
        return Ok(None);
    };
    let body = serde_json::json!({
        "ok": true,
        "runtime": "dx-www-preview",
        "production_contract": true,
        "path": path,
        "method": request_method,
        "source_path": check["source_path"].as_str().unwrap_or("unknown"),
    });
    Ok(Some(DxProductionPreviewResponse {
        status: "200 OK".to_string(),
        content_type: "application/json; charset=utf-8".to_string(),
        content_encoding: None,
        cache_control: Some("no-store".to_string()),
        last_modified: None,
        vary_accept_encoding: false,
        allow: Some(allowed_methods.join(", ")),
        contract_path: "deploy-adapter.json".to_string(),
        body: serde_json::to_vec(&body)
            .map_err(|error| format!("Failed to serialize health response: {error}"))?,
    }))
}

fn production_contract_health_method_matches(declared: &str, request_method: &str) -> bool {
    declared == request_method || (declared == "GET" && request_method == "HEAD")
}

fn production_contract_health_allowed_methods(
    contract: &serde_json::Value,
    path: &str,
) -> Vec<String> {
    let mut allowed_methods = Vec::new();
    for method in contract["health_checks"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|check| check["path"].as_str() == Some(path))
        .filter_map(|check| check["method"].as_str().or(Some("GET")))
    {
        push_unique_http_method(&mut allowed_methods, method);
        if method == "GET" {
            push_unique_http_method(&mut allowed_methods, "HEAD");
        }
    }
    push_unique_http_method(&mut allowed_methods, "OPTIONS");
    allowed_methods
}

fn push_unique_http_method(methods: &mut Vec<String>, method: &str) {
    let stable_method = method.trim();
    if stable_method.is_empty()
        || !stable_method
            .chars()
            .all(|value| value.is_ascii_uppercase())
    {
        return;
    }
    if !methods.iter().any(|existing| existing == stable_method) {
        methods.push(stable_method.to_string());
    }
}

fn production_contract_health_options_response(
    allowed_methods: Vec<String>,
) -> DxProductionPreviewResponse {
    DxProductionPreviewResponse {
        status: "204 No Content".to_string(),
        content_type: "text/plain; charset=utf-8".to_string(),
        content_encoding: None,
        cache_control: Some("no-store".to_string()),
        last_modified: None,
        vary_accept_encoding: false,
        allow: Some(allowed_methods.join(", ")),
        contract_path: "deploy-adapter.json".to_string(),
        body: Vec::new(),
    }
}

fn production_contract_health_method_not_allowed_response(
    path: &str,
    request_method: &str,
    allowed_methods: Vec<String>,
) -> Result<DxProductionPreviewResponse, String> {
    let body = serde_json::json!({
        "ok": false,
        "error": "production-health-method-not-allowed",
        "path": path,
        "method": request_method,
        "allowed_methods": allowed_methods,
    });
    Ok(DxProductionPreviewResponse {
        status: "405 Method Not Allowed".to_string(),
        content_type: "application/json; charset=utf-8".to_string(),
        content_encoding: None,
        cache_control: Some("no-store".to_string()),
        last_modified: None,
        vary_accept_encoding: false,
        allow: Some(allowed_methods.join(", ")),
        contract_path: "deploy-adapter.json".to_string(),
        body: serde_json::to_vec(&body)
            .map_err(|error| format!("Failed to serialize health 405 response: {error}"))?,
    })
}

fn production_contract_ready_response(
    build_dir: &Path,
    contract: &serde_json::Value,
    path: &str,
    request_method: &str,
) -> Result<Option<DxProductionPreviewResponse>, String> {
    let ready_path = contract["observability"]["ready_path"]
        .as_str()
        .unwrap_or("/.dx/ready");
    if path != ready_path {
        return Ok(None);
    }
    if request_method == "OPTIONS" {
        return Ok(Some(production_contract_internal_options_response(path)));
    }
    if request_method != "GET" && request_method != "HEAD" {
        return production_contract_internal_method_not_allowed_response(
            path,
            request_method,
            "production-ready-method-not-allowed",
        )
        .map(Some);
    }
    let observability = read_production_observability_contract(build_dir, contract)?;
    let required_artifacts_ready = observability["ready_check"]["required_artifacts"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|artifact| artifact.as_str())
        .all(|artifact| {
            safe_production_contract_file(build_dir, artifact)
                .map(|path| path.is_file())
                .unwrap_or(false)
        });
    let body = serde_json::json!({
        "ok": required_artifacts_ready,
        "ready": required_artifacts_ready,
        "runtime": "dx-www-preview",
        "production_contract": true,
        "manifest_hash": observability["manifest_hash"],
        "route_count": observability["route_timings"].as_array().map_or(0, Vec::len),
        "packet_bytes_total": observability["packet_bytes_total"].as_u64().unwrap_or(0),
        "server_action_receipts": observability["server_action_receipts"].as_array().map_or(0, Vec::len),
        "secret_fields_collected": false,
    });
    Ok(Some(DxProductionPreviewResponse {
        status: "200 OK".to_string(),
        content_type: "application/json; charset=utf-8".to_string(),
        content_encoding: None,
        cache_control: Some("no-store".to_string()),
        last_modified: None,
        vary_accept_encoding: false,
        allow: Some(PRODUCTION_STATIC_ALLOWED_METHODS.to_string()),
        contract_path: "deploy-adapter.json".to_string(),
        body: serde_json::to_vec(&body)
            .map_err(|error| format!("Failed to serialize ready response: {error}"))?,
    }))
}

fn production_contract_observability_response(
    build_dir: &Path,
    contract: &serde_json::Value,
    path: &str,
    request_method: &str,
) -> Result<Option<DxProductionPreviewResponse>, String> {
    let metrics_path = contract["observability"]["metrics_path"]
        .as_str()
        .unwrap_or("/.dx/observability");
    if path != metrics_path {
        return Ok(None);
    }
    if request_method == "OPTIONS" {
        return Ok(Some(production_contract_internal_options_response(path)));
    }
    if request_method != "GET" && request_method != "HEAD" {
        return production_contract_internal_method_not_allowed_response(
            path,
            request_method,
            "production-observability-method-not-allowed",
        )
        .map(Some);
    }
    let observability = read_production_observability_contract(build_dir, contract)?;
    Ok(Some(DxProductionPreviewResponse {
        status: "200 OK".to_string(),
        content_type: "application/json; charset=utf-8".to_string(),
        content_encoding: None,
        cache_control: Some("no-store".to_string()),
        last_modified: None,
        vary_accept_encoding: false,
        allow: Some(PRODUCTION_STATIC_ALLOWED_METHODS.to_string()),
        contract_path: "deploy-adapter.json".to_string(),
        body: serde_json::to_vec(&observability)
            .map_err(|error| format!("Failed to serialize observability response: {error}"))?,
    }))
}

fn production_contract_internal_options_response(_path: &str) -> DxProductionPreviewResponse {
    DxProductionPreviewResponse {
        status: "204 No Content".to_string(),
        content_type: "text/plain; charset=utf-8".to_string(),
        content_encoding: None,
        cache_control: Some("no-store".to_string()),
        last_modified: None,
        vary_accept_encoding: false,
        allow: Some(PRODUCTION_STATIC_ALLOWED_METHODS.to_string()),
        contract_path: "deploy-adapter.json".to_string(),
        body: Vec::new(),
    }
}

fn production_contract_internal_method_not_allowed_response(
    path: &str,
    request_method: &str,
    error_code: &str,
) -> Result<DxProductionPreviewResponse, String> {
    let body = serde_json::json!({
        "ok": false,
        "error": error_code,
        "path": path,
        "method": request_method,
        "allowed_methods": ["GET", "HEAD", "OPTIONS"],
    });
    Ok(DxProductionPreviewResponse {
        status: "405 Method Not Allowed".to_string(),
        content_type: "application/json; charset=utf-8".to_string(),
        content_encoding: None,
        cache_control: Some("no-store".to_string()),
        last_modified: None,
        vary_accept_encoding: false,
        allow: Some(PRODUCTION_STATIC_ALLOWED_METHODS.to_string()),
        contract_path: "deploy-adapter.json".to_string(),
        body: serde_json::to_vec(&body).map_err(|error| {
            format!("Failed to serialize production internal method guard response: {error}")
        })?,
    })
}

fn read_production_contract_file(
    build_dir: &Path,
    relative_path: &str,
    content_type: String,
    cache_control: Option<String>,
    vary_accept_encoding: bool,
    cached_files: Option<&BTreeMap<String, Vec<u8>>>,
) -> Result<DxProductionPreviewResponse, String> {
    let file_path = safe_production_contract_file(build_dir, relative_path)?;
    let last_modified = production_contract_file_last_modified(&file_path);
    let body = if let Some(body) = cached_files.and_then(|files| files.get(relative_path)) {
        body.clone()
    } else {
        std::fs::read(&file_path).map_err(|error| {
            format!(
                "Failed to read contract file {relative_path} from deploy-adapter.json: {error}"
            )
        })?
    };
    Ok(DxProductionPreviewResponse {
        status: "200 OK".to_string(),
        content_type,
        content_encoding: production_contract_content_encoding(relative_path).map(str::to_string),
        cache_control,
        last_modified,
        vary_accept_encoding,
        allow: None,
        contract_path: "deploy-adapter.json".to_string(),
        body,
    })
}

fn production_contract_file_last_modified(file_path: &Path) -> Option<String> {
    let modified = std::fs::metadata(file_path).ok()?.modified().ok()?;
    let modified: DateTime<Utc> = modified.into();
    Some(modified.format("%a, %d %b %Y %H:%M:%S GMT").to_string())
}

fn production_contract_static_paths(contract: &serde_json::Value) -> Vec<String> {
    let mut paths = Vec::new();
    if let Some(routes) = contract["routes"].as_array() {
        paths.extend(
            routes
                .iter()
                .filter_map(|route| route["html"].as_str())
                .map(str::to_string),
        );
    }
    if let Some(assets) = contract["immutable_assets"].as_array() {
        paths.extend(
            assets
                .iter()
                .filter_map(|asset| asset["path"].as_str())
                .map(str::to_string),
        );
    }
    paths.sort();
    paths.dedup();
    paths
}

fn production_contract_immutable_asset_for_request<'a>(
    contract: &'a serde_json::Value,
    asset_path: &str,
    accept_encoding: Option<&str>,
) -> Option<(String, &'a serde_json::Value, bool)> {
    let assets = contract["immutable_assets"].as_array()?;
    if production_contract_content_encoding(asset_path).is_none() {
        for candidate_path in immutable_asset_request_candidates(asset_path) {
            let varies_by_accept_encoding =
                immutable_precompressed_asset_exists(assets, &candidate_path, "br")
                    || immutable_precompressed_asset_exists(assets, &candidate_path, "gz");
            for suffix in accepted_precompressed_asset_suffixes(accept_encoding) {
                let candidate = format!("{candidate_path}.{suffix}");
                if let Some(asset) = immutable_asset_by_path(assets, &candidate) {
                    return Some((candidate, asset, true));
                }
            }
            if let Some(asset) = immutable_asset_by_path(assets, &candidate_path) {
                return Some((candidate_path, asset, varies_by_accept_encoding));
            }
        }
        return None;
    }
    for candidate_path in immutable_asset_request_candidates(asset_path) {
        if let Some(asset) = immutable_asset_by_path(assets, &candidate_path) {
            return Some((candidate_path, asset, false));
        }
    }
    None
}

fn immutable_asset_request_candidates(asset_path: &str) -> Vec<String> {
    let mut candidates = vec![asset_path.to_string()];
    if !asset_path.starts_with("public/") {
        candidates.push(format!("public/{asset_path}"));
    }
    candidates
}

fn immutable_asset_by_path<'a>(
    assets: &'a [serde_json::Value],
    path: &str,
) -> Option<&'a serde_json::Value> {
    assets
        .iter()
        .find(|asset| asset["path"].as_str() == Some(path))
}

fn immutable_precompressed_asset_exists(
    assets: &[serde_json::Value],
    asset_path: &str,
    suffix: &str,
) -> bool {
    let candidate = format!("{asset_path}.{suffix}");
    immutable_asset_by_path(assets, &candidate).is_some()
}

fn accepted_precompressed_asset_suffixes(accept_encoding: Option<&str>) -> Vec<&'static str> {
    let Some(accept_encoding) = accept_encoding.filter(|value| !value.trim().is_empty()) else {
        return Vec::new();
    };
    let mut candidates = Vec::new();
    let br_quality = accept_encoding_quality(accept_encoding, "br");
    let gzip_quality = accept_encoding_quality(accept_encoding, "gzip");
    if br_quality > 0 {
        candidates.push(("br", br_quality, 0_u8));
    }
    if gzip_quality > 0 {
        candidates.push(("gz", gzip_quality, 1_u8));
    }
    candidates.sort_by(|left, right| right.1.cmp(&left.1).then(left.2.cmp(&right.2)));
    candidates
        .into_iter()
        .map(|(suffix, _quality, _preference)| suffix)
        .collect()
}

fn accept_encoding_quality(header: &str, encoding: &str) -> u16 {
    let mut exact = None;
    let mut wildcard = None;
    for item in header.split(',') {
        let mut parts = item.split(';').map(str::trim);
        let token = parts.next().unwrap_or_default().to_ascii_lowercase();
        if token.is_empty() {
            continue;
        }
        let quality = parts
            .find_map(|part| part.strip_prefix("q=").map(parse_http_quality))
            .unwrap_or(1000);
        if token == encoding {
            exact = Some(quality);
        } else if token == "*" {
            wildcard = Some(quality);
        }
    }
    exact.or(wildcard).unwrap_or(0)
}

fn parse_http_quality(value: &str) -> u16 {
    let value = value.trim();
    if value == "1" || value == "1.0" || value == "1.00" || value == "1.000" {
        return 1000;
    }
    if value == "0" {
        return 0;
    }
    let Some(fraction) = value.strip_prefix("0.") else {
        return 0;
    };
    let mut digits = fraction
        .chars()
        .filter(|character| character.is_ascii_digit())
        .take(3)
        .collect::<String>();
    while digits.len() < 3 {
        digits.push('0');
    }
    digits.parse::<u16>().unwrap_or(0).min(999)
}

fn safe_production_contract_file(build_dir: &Path, relative_path: &str) -> Result<PathBuf, String> {
    if relative_path.is_empty() || relative_path.contains('\\') {
        return Err(format!(
            "Unsafe deploy-adapter.json file path: {relative_path}"
        ));
    }
    let path = Path::new(relative_path);
    if path.is_absolute() {
        return Err(format!(
            "deploy-adapter.json file paths must be relative: {relative_path}"
        ));
    }
    let mut output = build_dir.to_path_buf();
    for component in path.components() {
        match component {
            std::path::Component::Normal(part) => output.push(part),
            _ => {
                return Err(format!(
                    "deploy-adapter.json file path cannot escape build output: {relative_path}"
                ));
            }
        }
    }
    Ok(output)
}

fn production_contract_cache_control(contract: &serde_json::Value, relative_path: &str) -> String {
    if relative_path.ends_with(".html") {
        return "public, max-age=0, must-revalidate".to_string();
    }
    contract["immutable_assets"]
        .as_array()
        .and_then(|assets| {
            assets
                .iter()
                .find(|asset| asset["path"].as_str() == Some(relative_path))
        })
        .and_then(|asset| asset["cache_control"].as_str())
        .unwrap_or("no-store")
        .to_string()
}

fn production_contract_content_type(relative_path: &str) -> String {
    let decoded_path = production_contract_decoded_path(relative_path);
    if decoded_path.ends_with(".html") {
        "text/html; charset=utf-8"
    } else if decoded_path.ends_with(".css") {
        "text/css; charset=utf-8"
    } else if decoded_path.ends_with(".js") || decoded_path.ends_with(".mjs") {
        "application/javascript; charset=utf-8"
    } else if decoded_path.ends_with(".json") {
        "application/json; charset=utf-8"
    } else if decoded_path.ends_with(".webmanifest") {
        "application/manifest+json"
    } else if decoded_path.ends_with(".svg") {
        "image/svg+xml"
    } else if decoded_path.ends_with(".png") {
        "image/png"
    } else if decoded_path.ends_with(".ico") {
        "image/x-icon"
    } else if decoded_path.ends_with(".jpg") || decoded_path.ends_with(".jpeg") {
        "image/jpeg"
    } else if decoded_path.ends_with(".webp") {
        "image/webp"
    } else if decoded_path.ends_with(".avif") {
        "image/avif"
    } else if decoded_path.ends_with(".gif") {
        "image/gif"
    } else if decoded_path.ends_with(".wasm") {
        "application/wasm"
    } else if decoded_path.ends_with(".woff2") {
        "font/woff2"
    } else if decoded_path.ends_with(".woff") {
        "font/woff"
    } else if decoded_path.ends_with(".ttf") {
        "font/ttf"
    } else if decoded_path.ends_with(".otf") {
        "font/otf"
    } else {
        "application/octet-stream"
    }
    .to_string()
}

fn production_contract_decoded_path(relative_path: &str) -> &str {
    relative_path
        .strip_suffix(".br")
        .or_else(|| relative_path.strip_suffix(".gz"))
        .unwrap_or(relative_path)
}

fn production_contract_content_encoding(relative_path: &str) -> Option<&'static str> {
    if relative_path.ends_with(".br") {
        Some("br")
    } else if relative_path.ends_with(".gz") {
        Some("gzip")
    } else {
        None
    }
}

fn production_contract_not_found(path: &str) -> DxProductionPreviewResponse {
    DxProductionPreviewResponse {
        status: "404 Not Found".to_string(),
        content_type: "text/plain; charset=utf-8".to_string(),
        content_encoding: None,
        cache_control: Some("no-store".to_string()),
        last_modified: None,
        vary_accept_encoding: false,
        allow: None,
        contract_path: "deploy-adapter.json".to_string(),
        body: format!("{path} is not listed in deploy-adapter.json").into_bytes(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn noop_server_action(
        _build_dir: &Path,
        _contract: &serde_json::Value,
        _request: &DxCliHttpRequest,
        _action_id: &str,
    ) -> Result<String, String> {
        Ok("{}".to_string())
    }

    fn missing_csrf_server_action(
        _build_dir: &Path,
        _contract: &serde_json::Value,
        _request: &DxCliHttpRequest,
        _action_id: &str,
    ) -> Result<String, String> {
        Err("missing csrf token".to_string())
    }

    fn response_header<'a>(response: &'a str, name: &str) -> Option<&'a str> {
        let prefix = format!("{name}: ");
        response.lines().find_map(|line| line.strip_prefix(&prefix))
    }

    #[test]
    fn production_contract_cached_response_can_keep_connection_alive() {
        let dir = tempfile::tempdir().expect("tempdir");
        let output = dir.path();
        std::fs::create_dir_all(output.join("app")).expect("app dir");
        std::fs::write(output.join("app/index.html"), "<!doctype html><h1>DX</h1>").expect("html");
        std::fs::write(
            output.join("deploy-adapter.json"),
            serde_json::json!({
                "routes": [{"path": "/", "html": "app/index.html"}],
                "immutable_assets": [],
                "server_actions": [],
                "health_checks": []
            })
            .to_string(),
        )
        .expect("contract");
        let cache = load_production_preview_cache(output).expect("preview cache");

        let response = production_contract_wire_response_cached_with_connection(
            &cache,
            "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n",
            noop_server_action,
            "keep-alive",
        );
        let text = String::from_utf8(response).expect("utf8");

        assert!(text.contains("HTTP/1.1 200 OK"));
        assert!(text.contains("Connection: keep-alive"));
        assert!(text.ends_with("\r\n\r\n<!doctype html><h1>DX</h1>"));
    }

    #[test]
    fn production_contract_head_response_keeps_length_and_omits_body() {
        let dir = tempfile::tempdir().expect("tempdir");
        let output = dir.path();
        std::fs::create_dir_all(output.join("app")).expect("app dir");
        std::fs::write(output.join("app/index.html"), "<!doctype html><h1>DX</h1>").expect("html");
        std::fs::write(
            output.join("deploy-adapter.json"),
            serde_json::json!({
                "routes": [{"path": "/", "html": "app/index.html"}],
                "immutable_assets": [],
                "server_actions": [],
                "health_checks": []
            })
            .to_string(),
        )
        .expect("contract");

        let response = production_contract_wire_response(
            output,
            "HEAD / HTTP/1.1\r\nHost: localhost\r\n\r\n",
            noop_server_action,
        );
        let text = String::from_utf8(response).expect("utf8");

        assert!(text.contains("HTTP/1.1 200 OK"));
        assert!(text.contains("Content-Length: 26"));
        assert!(text.ends_with("\r\n\r\n"));
        assert!(!text.contains("<h1>DX</h1>"));
    }

    #[test]
    fn production_contract_static_routes_and_assets_guard_methods() {
        let dir = tempfile::tempdir().expect("tempdir");
        let output = dir.path();
        std::fs::create_dir_all(output.join("app")).expect("app dir");
        std::fs::create_dir_all(output.join("chunks")).expect("chunks dir");
        std::fs::write(output.join("app/index.html"), "<!doctype html><h1>DX</h1>").expect("html");
        std::fs::write(output.join("chunks/app.mjs"), b"console.log('dx');").expect("asset");
        std::fs::write(
            output.join("deploy-adapter.json"),
            serde_json::json!({
                "routes": [{"path": "/", "html": "app/index.html"}],
                "immutable_assets": [
                    {
                        "path": "chunks/app.mjs",
                        "cache_control": "public, max-age=31536000, immutable"
                    }
                ],
                "server_actions": [],
                "health_checks": []
            })
            .to_string(),
        )
        .expect("contract");

        let options_response = production_contract_wire_response(
            output,
            "OPTIONS / HTTP/1.1\r\nHost: localhost\r\n\r\n",
            noop_server_action,
        );
        let options_text = String::from_utf8(options_response).expect("options utf8");
        assert!(options_text.contains("HTTP/1.1 204 No Content"));
        assert!(options_text.contains("Allow: GET, HEAD, OPTIONS"));
        assert!(options_text.contains("Content-Length: 0"));
        assert!(options_text.ends_with("\r\n\r\n"));

        let post_route_response = production_contract_wire_response(
            output,
            "POST / HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\n\r\n",
            noop_server_action,
        );
        let post_route_text = String::from_utf8(post_route_response).expect("post route utf8");
        assert!(post_route_text.contains("HTTP/1.1 405 Method Not Allowed"));
        assert!(post_route_text.contains("Allow: GET, HEAD, OPTIONS"));
        assert!(post_route_text.contains(r#""error":"static-route-method-not-allowed""#));

        let post_asset_response = production_contract_wire_response(
            output,
            "POST /chunks/app.mjs HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\n\r\n",
            noop_server_action,
        );
        let post_asset_text = String::from_utf8(post_asset_response).expect("post asset utf8");
        assert!(post_asset_text.contains("HTTP/1.1 405 Method Not Allowed"));
        assert!(post_asset_text.contains("Allow: GET, HEAD, OPTIONS"));
        assert!(post_asset_text.contains(r#""error":"static-asset-method-not-allowed""#));
    }

    #[test]
    fn production_contract_serves_public_assets_from_root_alias() {
        let dir = tempfile::tempdir().expect("tempdir");
        let output = dir.path();
        std::fs::create_dir_all(output.join("public")).expect("public dir");
        std::fs::write(
            output.join("public/logo.svg"),
            "<svg><title>DX</title></svg>",
        )
        .expect("logo");
        std::fs::write(output.join("public/logo.svg.br"), b"br-logo").expect("br logo");
        std::fs::write(
            output.join("deploy-adapter.json"),
            serde_json::json!({
                "routes": [],
                "immutable_assets": [
                    {
                        "path": "public/logo.svg",
                        "cache_control": "public, max-age=31536000, immutable"
                    },
                    {
                        "path": "public/logo.svg.br",
                        "cache_control": "public, max-age=31536000, immutable"
                    }
                ],
                "server_actions": [],
                "health_checks": []
            })
            .to_string(),
        )
        .expect("contract");

        let plain_response = production_contract_wire_response(
            output,
            "GET /logo.svg HTTP/1.1\r\nHost: localhost\r\n\r\n",
            noop_server_action,
        );
        let plain_text = String::from_utf8(plain_response).expect("plain utf8");
        assert!(plain_text.contains("HTTP/1.1 200 OK"));
        assert!(plain_text.contains("Content-Type: image/svg+xml"));
        assert!(plain_text.contains("<svg><title>DX</title></svg>"));

        let br_response = production_contract_wire_response(
            output,
            "GET /logo.svg HTTP/1.1\r\nHost: localhost\r\nAccept-Encoding: br\r\n\r\n",
            noop_server_action,
        );
        let br_text = String::from_utf8(br_response).expect("br utf8");
        assert!(br_text.contains("HTTP/1.1 200 OK"));
        assert!(br_text.contains("Content-Type: image/svg+xml"));
        assert!(br_text.contains("Content-Encoding: br"));
        assert!(br_text.contains("Vary: Accept-Encoding"));
        assert!(br_text.ends_with("\r\n\r\nbr-logo"));
    }

    #[test]
    fn production_contract_conditional_get_returns_304_for_matching_etag() {
        let dir = tempfile::tempdir().expect("tempdir");
        let output = dir.path();
        std::fs::create_dir_all(output.join("app")).expect("app dir");
        std::fs::write(output.join("app/index.html"), "<!doctype html><h1>DX</h1>").expect("html");
        std::fs::write(
            output.join("deploy-adapter.json"),
            serde_json::json!({
                "routes": [{"path": "/", "html": "app/index.html"}],
                "immutable_assets": [],
                "server_actions": [],
                "health_checks": []
            })
            .to_string(),
        )
        .expect("contract");

        let first = production_contract_wire_response(
            output,
            "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n",
            noop_server_action,
        );
        let first = String::from_utf8(first).expect("utf8");
        let etag = first
            .lines()
            .find_map(|line| line.strip_prefix("ETag: "))
            .expect("etag")
            .to_string();
        let second = production_contract_wire_response(
            output,
            &format!("GET / HTTP/1.1\r\nHost: localhost\r\nIf-None-Match: {etag}\r\n\r\n"),
            noop_server_action,
        );
        let second = String::from_utf8(second).expect("utf8");

        assert!(second.contains("HTTP/1.1 304 Not Modified"));
        assert!(second.contains(&format!("ETag: {etag}")));
        assert!(second.contains("Content-Length: 0"));
        assert!(second.ends_with("\r\n\r\n"));
        assert!(!second.contains("<h1>DX</h1>"));
    }

    #[test]
    fn production_contract_cached_conditional_get_returns_304_for_matching_etag() {
        let dir = tempfile::tempdir().expect("tempdir");
        let output = dir.path();
        std::fs::create_dir_all(output.join("app")).expect("app dir");
        std::fs::write(output.join("app/index.html"), "<!doctype html><h1>DX</h1>").expect("html");
        std::fs::write(
            output.join("deploy-adapter.json"),
            serde_json::json!({
                "routes": [{"path": "/", "html": "app/index.html"}],
                "immutable_assets": [],
                "server_actions": [],
                "health_checks": []
            })
            .to_string(),
        )
        .expect("contract");

        let cache = load_production_preview_cache(output).expect("preview cache");
        let first = production_contract_wire_response_cached(
            &cache,
            "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n",
            noop_server_action,
        );
        let first = String::from_utf8(first).expect("utf8");
        let etag = response_header(&first, "ETag").expect("etag").to_string();
        let second = production_contract_wire_response_cached(
            &cache,
            &format!("GET / HTTP/1.1\r\nHost: localhost\r\nIf-None-Match: {etag}\r\n\r\n"),
            noop_server_action,
        );
        let second = String::from_utf8(second).expect("utf8");

        assert!(second.contains("HTTP/1.1 304 Not Modified"));
        assert!(second.contains(&format!("ETag: {etag}")));
        assert!(second.contains("Content-Length: 0"));
        assert!(second.ends_with("\r\n\r\n"));
        assert!(!second.contains("<h1>DX</h1>"));
    }

    #[test]
    fn production_contract_conditional_get_supports_last_modified() {
        let dir = tempfile::tempdir().expect("tempdir");
        let output = dir.path();
        std::fs::create_dir_all(output.join("app")).expect("app dir");
        std::fs::write(output.join("app/index.html"), "<!doctype html><h1>DX</h1>").expect("html");
        std::fs::write(
            output.join("deploy-adapter.json"),
            serde_json::json!({
                "routes": [{"path": "/", "html": "app/index.html"}],
                "immutable_assets": [],
                "server_actions": [],
                "health_checks": []
            })
            .to_string(),
        )
        .expect("contract");

        let first = production_contract_wire_response(
            output,
            "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n",
            noop_server_action,
        );
        let first = String::from_utf8(first).expect("utf8");
        let last_modified = response_header(&first, "Last-Modified")
            .expect("last modified")
            .to_string();
        let second = production_contract_wire_response(
            output,
            &format!(
                "GET / HTTP/1.1\r\nHost: localhost\r\nIf-Modified-Since: {last_modified}\r\n\r\n"
            ),
            noop_server_action,
        );
        let second = String::from_utf8(second).expect("utf8");

        assert!(second.contains("HTTP/1.1 304 Not Modified"));
        assert!(second.contains(&format!("Last-Modified: {last_modified}")));
        assert!(second.contains("Content-Length: 0"));
        assert!(second.ends_with("\r\n\r\n"));
    }

    #[test]
    fn production_contract_range_request_returns_partial_body() {
        let dir = tempfile::tempdir().expect("tempdir");
        let output = dir.path();
        std::fs::create_dir_all(output.join("app")).expect("app dir");
        std::fs::write(output.join("app/index.html"), "abcdef").expect("html");
        std::fs::write(
            output.join("deploy-adapter.json"),
            serde_json::json!({
                "routes": [{"path": "/", "html": "app/index.html"}],
                "immutable_assets": [],
                "server_actions": [],
                "health_checks": []
            })
            .to_string(),
        )
        .expect("contract");

        let response = production_contract_wire_response(
            output,
            "GET / HTTP/1.1\r\nHost: localhost\r\nRange: bytes=1-3\r\n\r\n",
            noop_server_action,
        );
        let text = String::from_utf8(response).expect("utf8");

        assert!(text.contains("HTTP/1.1 206 Partial Content"));
        assert!(text.contains("Accept-Ranges: bytes"));
        assert!(text.contains("Content-Range: bytes 1-3/6"));
        assert!(text.contains("Content-Length: 3"));
        assert!(text.ends_with("\r\n\r\nbcd"));
    }

    #[test]
    fn production_contract_invalid_range_returns_416() {
        let dir = tempfile::tempdir().expect("tempdir");
        let output = dir.path();
        std::fs::create_dir_all(output.join("app")).expect("app dir");
        std::fs::write(output.join("app/index.html"), "abcdef").expect("html");
        std::fs::write(
            output.join("deploy-adapter.json"),
            serde_json::json!({
                "routes": [{"path": "/", "html": "app/index.html"}],
                "immutable_assets": [],
                "server_actions": [],
                "health_checks": []
            })
            .to_string(),
        )
        .expect("contract");

        let response = production_contract_wire_response(
            output,
            "GET / HTTP/1.1\r\nHost: localhost\r\nRange: bytes=99-120\r\n\r\n",
            noop_server_action,
        );
        let text = String::from_utf8(response).expect("utf8");

        assert!(text.contains("HTTP/1.1 416 Range Not Satisfiable"));
        assert!(text.contains("Content-Range: bytes */6"));
        assert!(text.contains("Content-Length: 0"));
        assert!(text.ends_with("\r\n\r\n"));
    }

    #[test]
    fn production_contract_if_range_controls_partial_body() {
        let dir = tempfile::tempdir().expect("tempdir");
        let output = dir.path();
        std::fs::create_dir_all(output.join("app")).expect("app dir");
        std::fs::write(output.join("app/index.html"), "abcdef").expect("html");
        std::fs::write(
            output.join("deploy-adapter.json"),
            serde_json::json!({
                "routes": [{"path": "/", "html": "app/index.html"}],
                "immutable_assets": [],
                "server_actions": [],
                "health_checks": []
            })
            .to_string(),
        )
        .expect("contract");

        let first = production_contract_wire_response(
            output,
            "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n",
            noop_server_action,
        );
        let first = String::from_utf8(first).expect("utf8");
        let etag = response_header(&first, "ETag").expect("etag").to_string();

        let matching = production_contract_wire_response(
            output,
            &format!(
                "GET / HTTP/1.1\r\nHost: localhost\r\nRange: bytes=1-3\r\nIf-Range: {etag}\r\n\r\n"
            ),
            noop_server_action,
        );
        let matching = String::from_utf8(matching).expect("matching utf8");
        assert!(matching.contains("HTTP/1.1 206 Partial Content"));
        assert!(matching.contains("Content-Range: bytes 1-3/6"));
        assert!(matching.ends_with("\r\n\r\nbcd"));

        let stale = production_contract_wire_response(
            output,
            "GET / HTTP/1.1\r\nHost: localhost\r\nRange: bytes=1-3\r\nIf-Range: \"stale\"\r\n\r\n",
            noop_server_action,
        );
        let stale = String::from_utf8(stale).expect("stale utf8");
        assert!(stale.contains("HTTP/1.1 200 OK"));
        assert!(!stale.contains("Content-Range:"));
        assert!(stale.ends_with("\r\n\r\nabcdef"));

        let weak = production_contract_wire_response(
            output,
            &format!(
                "GET / HTTP/1.1\r\nHost: localhost\r\nRange: bytes=1-3\r\nIf-Range: W/{etag}\r\n\r\n"
            ),
            noop_server_action,
        );
        let weak = String::from_utf8(weak).expect("weak if-range utf8");
        assert!(weak.contains("HTTP/1.1 200 OK"));
        assert!(!weak.contains("Content-Range:"));
        assert!(weak.ends_with("\r\n\r\nabcdef"));
    }

    #[test]
    fn production_contract_route_handler_health_check_requires_matching_method() {
        let dir = tempfile::tempdir().expect("tempdir");
        let output = dir.path();
        std::fs::write(
            output.join("deploy-adapter.json"),
            serde_json::json!({
                "routes": [],
                "immutable_assets": [],
                "server_actions": [],
                "health_checks": [
                    {
                        "path": "/api/health",
                        "method": "GET",
                        "source_path": "app/api/health/route.ts"
                    }
                ]
            })
            .to_string(),
        )
        .expect("contract");

        let get_response = production_contract_wire_response(
            output,
            "GET /api/health HTTP/1.1\r\nHost: localhost\r\n\r\n",
            noop_server_action,
        );
        let get_text = String::from_utf8(get_response).expect("get utf8");
        assert!(get_text.contains("HTTP/1.1 200 OK"));
        assert!(get_text.contains("Allow: GET, HEAD, OPTIONS"));
        assert!(get_text.contains(r#""method":"GET""#));

        let head_response = production_contract_wire_response(
            output,
            "HEAD /api/health HTTP/1.1\r\nHost: localhost\r\n\r\n",
            noop_server_action,
        );
        let head_text = String::from_utf8(head_response).expect("head utf8");
        assert!(head_text.contains("HTTP/1.1 200 OK"));
        assert!(head_text.contains("Allow: GET, HEAD, OPTIONS"));
        assert!(head_text.ends_with("\r\n\r\n"));
        assert!(!head_text.contains(r#""source_path":"app/api/health/route.ts""#));

        let options_response = production_contract_wire_response(
            output,
            "OPTIONS /api/health HTTP/1.1\r\nHost: localhost\r\n\r\n",
            noop_server_action,
        );
        let options_text = String::from_utf8(options_response).expect("options utf8");
        assert!(options_text.contains("HTTP/1.1 204 No Content"));
        assert!(options_text.contains("Allow: GET, HEAD, OPTIONS"));
        assert!(options_text.contains("Content-Length: 0"));

        let post_response = production_contract_wire_response(
            output,
            "POST /api/health HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\n\r\n",
            noop_server_action,
        );
        let post_text = String::from_utf8(post_response).expect("post utf8");
        assert!(post_text.contains("HTTP/1.1 405 Method Not Allowed"));
        assert!(post_text.contains(r#""error":"production-health-method-not-allowed""#));
        assert!(post_text.contains(r#""method":"POST""#));
        assert!(post_text.contains("Allow: GET, HEAD, OPTIONS"));
        assert!(post_text.contains(r#""allowed_methods":["GET","HEAD","OPTIONS"]"#));
    }

    #[test]
    fn production_contract_content_type_covers_runtime_assets() {
        assert_eq!(
            production_contract_content_type("chunks/app.mjs"),
            "application/javascript; charset=utf-8"
        );
        assert_eq!(
            production_contract_content_type("public/manifest.webmanifest"),
            "application/manifest+json"
        );
        assert_eq!(
            production_contract_content_type("runtime/app.wasm"),
            "application/wasm"
        );
        assert_eq!(
            production_contract_content_type("fonts/mono.woff2"),
            "font/woff2"
        );
    }

    #[test]
    fn production_contract_precompressed_asset_sets_encoding_and_decoded_type() {
        let dir = tempfile::tempdir().expect("tempdir");
        let output = dir.path();
        std::fs::create_dir_all(output.join("chunks")).expect("chunks dir");
        std::fs::write(output.join("chunks/app.mjs.br"), b"compressed-js").expect("asset");
        std::fs::write(
            output.join("deploy-adapter.json"),
            serde_json::json!({
                "routes": [],
                "immutable_assets": [
                    {
                        "path": "chunks/app.mjs.br",
                        "cache_control": "public, max-age=31536000, immutable"
                    }
                ],
                "server_actions": [],
                "health_checks": []
            })
            .to_string(),
        )
        .expect("contract");

        let response = production_contract_wire_response(
            output,
            "GET /chunks/app.mjs.br HTTP/1.1\r\nHost: localhost\r\n\r\n",
            noop_server_action,
        );
        let text = String::from_utf8(response).expect("utf8");

        assert!(text.contains("HTTP/1.1 200 OK"));
        assert!(text.contains("Content-Type: application/javascript; charset=utf-8"));
        assert!(text.contains("Content-Encoding: br"));
        assert!(text.contains("Vary: Accept-Encoding"));
        assert!(text.ends_with("\r\n\r\ncompressed-js"));
    }

    #[test]
    fn production_contract_negotiates_precompressed_assets_from_accept_encoding() {
        let dir = tempfile::tempdir().expect("tempdir");
        let output = dir.path();
        std::fs::create_dir_all(output.join("chunks")).expect("chunks dir");
        std::fs::write(output.join("chunks/app.mjs"), b"plain-js").expect("plain asset");
        std::fs::write(output.join("chunks/app.mjs.br"), b"br-js").expect("br asset");
        std::fs::write(output.join("chunks/app.mjs.gz"), b"gzip-js").expect("gzip asset");
        std::fs::write(
            output.join("deploy-adapter.json"),
            serde_json::json!({
                "routes": [],
                "immutable_assets": [
                    {
                        "path": "chunks/app.mjs",
                        "cache_control": "public, max-age=31536000, immutable"
                    },
                    {
                        "path": "chunks/app.mjs.br",
                        "cache_control": "public, max-age=31536000, immutable"
                    },
                    {
                        "path": "chunks/app.mjs.gz",
                        "cache_control": "public, max-age=31536000, immutable"
                    }
                ],
                "server_actions": [],
                "health_checks": []
            })
            .to_string(),
        )
        .expect("contract");

        let br_response = production_contract_wire_response(
            output,
            "GET /chunks/app.mjs HTTP/1.1\r\nHost: localhost\r\nAccept-Encoding: gzip;q=0.5, br\r\n\r\n",
            noop_server_action,
        );
        let br_text = String::from_utf8(br_response).expect("br utf8");
        assert!(br_text.contains("Content-Encoding: br"));
        assert!(br_text.ends_with("\r\n\r\nbr-js"));

        let gzip_response = production_contract_wire_response(
            output,
            "GET /chunks/app.mjs HTTP/1.1\r\nHost: localhost\r\nAccept-Encoding: br;q=0, gzip;q=1\r\n\r\n",
            noop_server_action,
        );
        let gzip_text = String::from_utf8(gzip_response).expect("gzip utf8");
        assert!(gzip_text.contains("Content-Encoding: gzip"));
        assert!(gzip_text.ends_with("\r\n\r\ngzip-js"));

        let plain_response = production_contract_wire_response(
            output,
            "GET /chunks/app.mjs HTTP/1.1\r\nHost: localhost\r\n\r\n",
            noop_server_action,
        );
        let plain_text = String::from_utf8(plain_response).expect("plain utf8");
        assert!(!plain_text.contains("Content-Encoding:"));
        assert!(plain_text.contains("Vary: Accept-Encoding"));
        assert!(plain_text.ends_with("\r\n\r\nplain-js"));
    }

    #[cfg(feature = "dev-server")]
    #[test]
    fn production_contract_axum_static_responder_matches_wire_semantics() {
        use crate::dev::axum_server::DxDevAxumRequest;

        let dir = tempfile::tempdir().expect("tempdir");
        let output = dir.path();
        std::fs::create_dir_all(output.join("chunks")).expect("chunks dir");
        std::fs::create_dir_all(output.join("app")).expect("app dir");
        std::fs::write(output.join("app/index.html"), b"abcdef").expect("html");
        std::fs::write(output.join("chunks/app.mjs"), b"plain-js").expect("plain asset");
        std::fs::write(output.join("chunks/app.mjs.br"), b"br-js").expect("br asset");
        std::fs::write(output.join("chunks/app.mjs.gz"), b"gzip-js").expect("gzip asset");
        std::fs::write(
            output.join("deploy-adapter.json"),
            serde_json::json!({
                "routes": [
                    {
                        "path": "/",
                        "html": "app/index.html"
                    }
                ],
                "immutable_assets": [
                    {
                        "path": "chunks/app.mjs",
                        "cache_control": "public, max-age=31536000, immutable"
                    },
                    {
                        "path": "chunks/app.mjs.br",
                        "cache_control": "public, max-age=31536000, immutable"
                    },
                    {
                        "path": "chunks/app.mjs.gz",
                        "cache_control": "public, max-age=31536000, immutable"
                    }
                ],
                "server_actions": [],
                "health_checks": []
            })
            .to_string(),
        )
        .expect("contract");
        let cache = load_production_preview_cache(output).expect("preview cache");

        let br = production_contract_axum_response_cached(
            &cache,
            DxDevAxumRequest {
                method: "GET".to_string(),
                path: "/chunks/app.mjs".to_string(),
                headers: BTreeMap::from([(
                    "accept-encoding".to_string(),
                    "gzip;q=0.5, br".to_string(),
                )]),
                body: Vec::new(),
            },
            noop_server_action,
        );
        assert_eq!(br.status, 200);
        assert_eq!(br.content_type, "application/javascript; charset=utf-8");
        assert_eq!(br.headers.get("content-encoding"), Some(&"br".to_string()));
        assert_eq!(br.headers.get("vary"), Some(&"Accept-Encoding".to_string()));
        assert_eq!(br.body.as_ref(), b"br-js");

        let range = production_contract_axum_response_cached(
            &cache,
            DxDevAxumRequest {
                method: "GET".to_string(),
                path: "/".to_string(),
                headers: BTreeMap::from([("range".to_string(), "bytes=1-3".to_string())]),
                body: Vec::new(),
            },
            noop_server_action,
        );
        assert_eq!(range.status, 206);
        assert_eq!(
            range.headers.get("content-range"),
            Some(&"bytes 1-3/6".to_string())
        );
        assert_eq!(range.body.as_ref(), b"bcd");

        let head = production_contract_axum_response_cached(
            &cache,
            DxDevAxumRequest {
                method: "HEAD".to_string(),
                path: "/".to_string(),
                headers: BTreeMap::new(),
                body: Vec::new(),
            },
            noop_server_action,
        );
        assert_eq!(head.status, 200);
        assert_eq!(head.headers.get("content-length"), Some(&"6".to_string()));
        assert!(head.body.is_empty());

        let post = production_contract_axum_response_cached(
            &cache,
            DxDevAxumRequest {
                method: "POST".to_string(),
                path: "/chunks/app.mjs".to_string(),
                headers: BTreeMap::from([("content-length".to_string(), "0".to_string())]),
                body: Vec::new(),
            },
            noop_server_action,
        );
        assert_eq!(post.status, 405);
        assert_eq!(
            post.headers.get("allow"),
            Some(&"GET, HEAD, OPTIONS".to_string())
        );
        let post_body = String::from_utf8(post.body.to_vec()).expect("post body utf8");
        assert!(post_body.contains(r#""error":"static-asset-method-not-allowed""#));
    }

    #[test]
    fn production_contract_server_action_get_returns_405_without_executor() {
        let dir = tempfile::tempdir().expect("tempdir");
        let output = dir.path();
        std::fs::write(
            output.join("deploy-adapter.json"),
            serde_json::json!({
                "routes": [],
                "immutable_assets": [],
                "server_actions": [
                    {
                        "endpoint": "/.dx/actions/server%2Factions.ts%23save",
                        "action_id": "server/actions.ts#save",
                        "method": "POST"
                    }
                ],
                "health_checks": []
            })
            .to_string(),
        )
        .expect("contract");

        let response = handle_production_contract_http_request(
            output,
            "GET /.dx/actions/server%2Factions.ts%23save HTTP/1.1\r\nHost: localhost\r\n\r\n",
            |_build_dir, _contract, _request, _action_id| {
                panic!("method guard should run before executor")
            },
        )
        .expect("response");
        let body: serde_json::Value = serde_json::from_slice(&response.body).expect("json");

        assert_eq!(response.status, "405 Method Not Allowed");
        assert_eq!(body["error"], "server-action-method-not-allowed");
        assert_eq!(body["method"], "POST");
        assert_eq!(body["replay_safe"], false);
    }

    #[test]
    fn production_contract_server_action_post_does_not_return_304_for_if_none_match() {
        let dir = tempfile::tempdir().expect("tempdir");
        let output = dir.path();
        std::fs::write(
            output.join("deploy-adapter.json"),
            serde_json::json!({
                "routes": [],
                "immutable_assets": [],
                "server_actions": [
                    {
                        "endpoint": "/.dx/actions/server%2Factions.ts%23save",
                        "action_id": "server/actions.ts#save",
                        "method": "POST"
                    }
                ],
                "health_checks": []
            })
            .to_string(),
        )
        .expect("contract");

        let response = production_contract_wire_response(
            output,
            "POST /.dx/actions/server%2Factions.ts%23save HTTP/1.1\r\nHost: localhost\r\nIf-None-Match: *\r\nContent-Length: 2\r\n\r\n{}",
            noop_server_action,
        );
        let text = String::from_utf8(response).expect("utf8");

        assert!(text.contains("HTTP/1.1 200 OK"));
        assert!(text.contains("Cache-Control: no-store"));
        assert!(text.ends_with("\r\n\r\n{}"));
        assert!(!text.contains("HTTP/1.1 304 Not Modified"));
    }

    #[test]
    fn production_contract_server_action_validation_error_is_structured_bad_request() {
        let dir = tempfile::tempdir().expect("tempdir");
        let output = dir.path();
        std::fs::write(
            output.join("deploy-adapter.json"),
            serde_json::json!({
                "routes": [],
                "immutable_assets": [],
                "server_actions": [
                    {
                        "endpoint": "/.dx/actions/server%2Factions.ts%23save",
                        "action_id": "server/actions.ts#save",
                        "method": "POST"
                    }
                ],
                "health_checks": []
            })
            .to_string(),
        )
        .expect("contract");

        let response = handle_production_contract_http_request(
            output,
            "POST /.dx/actions/server%2Factions.ts%23save HTTP/1.1\r\nHost: localhost\r\nContent-Length: 2\r\n\r\n{}",
            missing_csrf_server_action,
        )
        .expect("response");
        let body: serde_json::Value = serde_json::from_slice(&response.body).expect("json");

        assert_eq!(response.status, "400 Bad Request");
        assert_eq!(response.content_type, "application/json; charset=utf-8");
        assert_eq!(response.cache_control.as_deref(), Some("no-store"));
        assert_eq!(body["error"], "server-action-failed");
        assert_eq!(body["message"], "missing csrf token");
        assert_eq!(body["receipt_written"], false);
        assert_eq!(body["replay_safe"], false);
    }
}
