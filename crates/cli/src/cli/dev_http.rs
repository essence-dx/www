use std::collections::{BTreeMap, HashMap};
use std::path::Path;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant, UNIX_EPOCH};

use super::{dev_hot_reload_client, devtools};

#[derive(Debug, Clone)]
struct CachedReloadToken {
    token: String,
    expires_at: Instant,
}

static DX_DEV_RELOAD_TOKEN_CACHE: OnceLock<Mutex<HashMap<String, CachedReloadToken>>> =
    OnceLock::new();

const DX_DEV_RELOAD_TOKEN_CACHE_TTL: Duration = Duration::from_millis(250);

#[derive(Debug, Clone, PartialEq)]
pub(super) struct DxCliHttpRequest {
    pub(super) method: String,
    pub(super) path: String,
    pub(super) headers: BTreeMap<String, String>,
    pub(super) body: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxCliHttpResponse {
    pub(super) status: String,
    pub(super) content_type: String,
    pub(super) headers: BTreeMap<String, String>,
    pub(super) body: String,
}

pub(super) fn parse_http_request(request: &str) -> DxCliHttpRequest {
    let request_line = request.lines().next().unwrap_or_default();
    let mut request_parts = request_line.split_whitespace();
    let method = request_parts
        .next()
        .unwrap_or("GET")
        .trim()
        .to_ascii_uppercase();
    let path = request_parts.next().unwrap_or("/").to_string();
    let (headers, body) = split_http_headers_and_body(request);
    let headers = parse_http_headers(headers);
    let content_type = headers
        .get("content-type")
        .map(|value| value.to_ascii_lowercase())
        .unwrap_or_default();
    let body = parse_http_body_value(body, &content_type);

    DxCliHttpRequest {
        method,
        path,
        headers,
        body,
    }
}

#[cfg(feature = "dev-server")]
pub(super) fn dx_cli_request_from_axum(
    request: crate::dev::axum_server::DxDevAxumRequest,
) -> DxCliHttpRequest {
    let content_type = request
        .headers
        .get("content-type")
        .map(|value| value.to_ascii_lowercase())
        .unwrap_or_default();
    let body_text = String::from_utf8_lossy(&request.body);
    let body = parse_http_body_value(&body_text, &content_type);

    DxCliHttpRequest {
        method: request.method.to_ascii_uppercase(),
        path: request.path,
        headers: request.headers,
        body,
    }
}

#[cfg(feature = "dev-server")]
pub(super) fn dx_cli_response_to_axum(
    response: DxCliHttpResponse,
) -> crate::dev::axum_server::DxDevAxumResponse {
    crate::dev::axum_server::DxDevAxumResponse {
        status: response
            .status
            .split_whitespace()
            .next()
            .and_then(|status| status.parse::<u16>().ok())
            .unwrap_or(500),
        content_type: response.content_type,
        headers: response.headers,
        body: response.body.into_bytes().into(),
    }
}

pub(super) fn apply_dev_cache_headers(
    request: &DxCliHttpRequest,
    response: &mut DxCliHttpResponse,
) {
    if matches!(request.method.as_str(), "GET" | "HEAD")
        && response.status.starts_with("200")
        && is_dev_cache_sensitive_path(&request.path)
    {
        response
            .headers
            .entry("cache-control".to_string())
            .or_insert_with(|| "no-store".to_string());
    }
}

fn is_dev_cache_sensitive_path(request_path: &str) -> bool {
    let path = dev_lookup_path(request_path);
    path == "/"
        || path.starts_with("/styles/")
        || path.starts_with("/public/")
        || path.starts_with("/_dx/styles/")
        || path.ends_with(".css")
        || path.ends_with(".js")
        || path.ends_with(".mjs")
        || path.ends_with(".json")
        || path.ends_with(".svg")
        || path.ends_with(".png")
        || path.ends_with(".jpg")
        || path.ends_with(".jpeg")
        || path.ends_with(".webp")
        || path.ends_with(".wasm")
}

pub(super) fn dev_lookup_path(request_path: &str) -> String {
    let path = request_path
        .split('?')
        .next()
        .unwrap_or(request_path)
        .trim();
    let path = if path.is_empty() { "/" } else { path };
    if path.starts_with('/') {
        path.to_string()
    } else {
        format!("/{path}")
    }
}

fn dev_reload_source_roots() -> [&'static str; 15] {
    [
        "app",
        "src/app",
        "pages",
        "src/pages",
        "components",
        "server",
        "api",
        "styles",
        "public",
        "forge",
        ".dx/forge/routes",
        ".dx/forge/route-discovery",
        ".dx/forge/source-manifests",
        ".dx/forge/source-surfaces",
        ".dx/forge/preview",
    ]
}

pub(super) fn dev_project_reload_token(cwd: &Path) -> String {
    let cache_key = cwd
        .canonicalize()
        .unwrap_or_else(|_| cwd.to_path_buf())
        .to_string_lossy()
        .replace('\\', "/");
    let now = Instant::now();

    if let Some(token) = DX_DEV_RELOAD_TOKEN_CACHE
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .ok()
        .and_then(|cache| {
            cache
                .get(&cache_key)
                .filter(|cached| cached.expires_at > now)
                .map(|cached| cached.token.clone())
        })
    {
        return token;
    }

    let mut latest_modified = 0u128;
    let mut file_count = 0u64;
    let mut byte_fingerprint = 0u64;

    for root in dev_reload_source_roots() {
        let root = cwd.join(root);
        if !root.exists() {
            continue;
        }

        for entry in walkdir::WalkDir::new(root)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_file())
        {
            if is_dev_reload_generated_output(cwd, entry.path()) {
                continue;
            }
            let Ok(metadata) = entry.metadata() else {
                continue;
            };
            file_count = file_count.saturating_add(1);
            byte_fingerprint = byte_fingerprint
                .wrapping_mul(16777619)
                .wrapping_add(metadata.len());

            let Ok(modified) = metadata.modified() else {
                continue;
            };
            let Ok(since_epoch) = modified.duration_since(UNIX_EPOCH) else {
                continue;
            };
            latest_modified = latest_modified.max(since_epoch.as_nanos());
        }
    }

    let token = format!("{file_count:x}-{latest_modified:x}-{byte_fingerprint:x}");
    if let Ok(mut cache) = DX_DEV_RELOAD_TOKEN_CACHE
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
    {
        if cache.len() > 128 {
            cache.clear();
        }
        cache.insert(
            cache_key,
            CachedReloadToken {
                token: token.clone(),
                expires_at: now + DX_DEV_RELOAD_TOKEN_CACHE_TTL,
            },
        );
    }
    token
}

fn is_dev_reload_generated_output(cwd: &Path, path: &Path) -> bool {
    let relative = path.strip_prefix(cwd).unwrap_or(path);
    let normalized = relative.to_string_lossy().replace('\\', "/");
    normalized == "styles/generated.css"
        || normalized == "components/auto-imports.ts"
        || normalized.starts_with("components/icons/")
        || normalized.starts_with(".dx/serializer/")
        || normalized.starts_with(".dx/receipts/")
        || normalized.starts_with(".dx/run/")
        || normalized.starts_with(".dx/www/")
        || normalized.starts_with(".dx/build/")
}

pub(super) fn with_dev_hot_reload(cwd: &Path, response: DxCliHttpResponse) -> DxCliHttpResponse {
    let token = dev_project_reload_token(cwd);
    with_dev_hot_reload_token(response, &token, false)
}

pub(super) fn with_dev_hot_reload_token(
    response: DxCliHttpResponse,
    token: &str,
    event_stream_supported: bool,
) -> DxCliHttpResponse {
    with_dev_html_injections_token(response, Some(token), false, event_stream_supported)
}

pub(super) fn with_dev_html_injections_token(
    mut response: DxCliHttpResponse,
    reload_token: Option<&str>,
    devtools: bool,
    event_stream_supported: bool,
) -> DxCliHttpResponse {
    if !response.status.starts_with("200") || !response.content_type.contains("text/html") {
        return response;
    }

    let mut injected = false;

    if let Some(token) = reload_token {
        if !response
            .body
            .contains("<script type=\"module\" data-dx-hot-reload")
        {
            let script =
                dev_hot_reload_client::dev_hot_reload_client_script(token, event_stream_supported);
            inject_dev_html_script(&mut response.body, &script);
            injected = true;
        }
    }

    if devtools && !response.body.contains("data-dx-devtools-runtime") {
        inject_dev_html_script(&mut response.body, devtools::devtools_injection_tags());
        injected = true;
    }

    if injected {
        response
            .headers
            .insert("cache-control".to_string(), "no-store".to_string());
    }
    response
}

fn inject_dev_html_script(body: &mut String, script: &str) {
    if let Some(index) = body.rfind("</body>") {
        body.insert_str(index, script);
    } else {
        body.push_str(script);
    }
}

fn split_http_headers_and_body(request: &str) -> (&str, &str) {
    request
        .split_once("\r\n\r\n")
        .or_else(|| request.split_once("\n\n"))
        .unwrap_or((request, ""))
}

fn parse_http_headers(headers: &str) -> BTreeMap<String, String> {
    headers
        .lines()
        .skip(1)
        .filter_map(|line| line.split_once(':'))
        .map(|(name, value)| (name.trim().to_ascii_lowercase(), value.trim().to_string()))
        .collect()
}

fn parse_http_body_value(body: &str, content_type: &str) -> serde_json::Value {
    let media_type = content_type_media_type(content_type);
    if body.trim().is_empty() {
        serde_json::Value::Null
    } else if media_type == "application/json" || media_type.ends_with("+json") {
        serde_json::from_str(body).unwrap_or_else(|_| {
            serde_json::json!({
                "raw": body,
                "parse_error": "invalid-json"
            })
        })
    } else if media_type == "application/x-www-form-urlencoded" {
        parse_urlencoded_form_body(body)
    } else {
        serde_json::Value::String(body.to_string())
    }
}

fn content_type_media_type(content_type: &str) -> &str {
    content_type.split(';').next().unwrap_or("").trim()
}

fn parse_urlencoded_form_body(body: &str) -> serde_json::Value {
    let mut fields = serde_json::Map::new();
    for entry in body.split('&') {
        if entry.is_empty() {
            continue;
        }
        let (name, value) = entry.split_once('=').unwrap_or((entry, ""));
        let name = decode_urlencoded_component(name);
        if name.is_empty() {
            continue;
        }
        fields
            .entry(name)
            .or_insert_with(|| serde_json::Value::String(decode_urlencoded_component(value)));
    }
    serde_json::Value::Object(fields)
}

fn decode_urlencoded_component(component: &str) -> String {
    let bytes = component.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut cursor = 0usize;
    while cursor < bytes.len() {
        match bytes[cursor] {
            b'+' => {
                decoded.push(b' ');
                cursor += 1;
            }
            b'%' if cursor + 2 < bytes.len() => {
                if let (Some(high), Some(low)) =
                    (hex_value(bytes[cursor + 1]), hex_value(bytes[cursor + 2]))
                {
                    decoded.push((high << 4) | low);
                    cursor += 3;
                } else {
                    decoded.push(bytes[cursor]);
                    cursor += 1;
                }
            }
            byte => {
                decoded.push(byte);
                cursor += 1;
            }
        }
    }
    String::from_utf8_lossy(&decoded).into_owned()
}

fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{DxCliHttpResponse, parse_http_request, with_dev_html_injections_token};
    use std::collections::BTreeMap;

    #[test]
    fn parses_urlencoded_form_body_for_route_handler_form_data() {
        let request = parse_http_request(
            "POST /api/signup HTTP/1.1\r\n\
content-type: application/x-www-form-urlencoded;charset=UTF-8\r\n\
\r\n\
email=ada%2Blaunch%40example.com&plan=team+space&empty=&plan=enterprise",
        );

        assert_eq!(
            request.body,
            serde_json::json!({
                "email": "ada+launch@example.com",
                "plan": "team space",
                "empty": ""
            })
        );
    }

    #[test]
    fn parses_json_suffix_media_type_with_charset_for_route_handlers() {
        let request = parse_http_request(
            "POST /api/custom-json HTTP/1.1\r\n\
content-type: application/vnd.dx.route+json; charset=UTF-8\r\n\
\r\n\
{\"message\":\"ship\",\"count\":2}",
        );

        assert_eq!(
            request.body,
            serde_json::json!({
                "message": "ship",
                "count": 2
            })
        );
    }

    #[test]
    fn preserves_plain_text_body_for_request_text() {
        let request = parse_http_request(
            "POST /api/text HTTP/1.1\r\n\
content-type: text/plain; charset=UTF-8\r\n\
\r\n\
DX route handler text body",
        );

        assert_eq!(
            request.body,
            serde_json::Value::String("DX route handler text body".to_string())
        );
    }

    #[test]
    fn dev_html_injections_compose_hot_reload_and_devtools() {
        let response = DxCliHttpResponse {
            status: "200 OK".to_string(),
            content_type: "text/html; charset=utf-8".to_string(),
            headers: BTreeMap::new(),
            body: "<!doctype html><html><body><main>DX</main></body></html>".to_string(),
        };

        let response = with_dev_html_injections_token(response, Some("reload-token"), true, true);

        assert!(response.body.contains("data-dx-hot-reload"));
        assert!(response.body.contains("data-dx-devtools-runtime"));
        assert!(response.body.contains("/_dx/devtools/runtime.js"));
        assert!(response.body.contains("/_dx/devtools/devtools.css"));
        assert!(
            response
                .body
                .find("data-dx-hot-reload")
                .expect("hot reload marker")
                < response
                    .body
                    .find("data-dx-devtools")
                    .expect("devtools marker")
        );
        assert_eq!(
            response.headers.get("cache-control").map(String::as_str),
            Some("no-store")
        );
    }

    #[test]
    fn dev_html_injections_target_live_body_after_inert_templates() {
        let response = DxCliHttpResponse {
            status: "200 OK".to_string(),
            content_type: "text/html; charset=utf-8".to_string(),
            headers: BTreeMap::new(),
            body: "<!doctype html><html><body><main>DX</main><template><html><body>preview</body></html></template></body></html>".to_string(),
        };

        let response = with_dev_html_injections_token(response, Some("reload-token"), true, true);
        let template_end = response
            .body
            .find("</template>")
            .expect("template should remain inert");
        let devtools_marker = response
            .body
            .find("data-dx-devtools-runtime")
            .expect("devtools runtime marker");

        assert!(template_end < devtools_marker);
        assert!(
            response
                .body
                .contains("</template><script type=\"module\" data-dx-hot-reload")
        );
        assert!(response.body.contains("/_dx/devtools/runtime.js"));
    }
}
