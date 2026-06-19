use std::collections::{BTreeMap, HashMap};
use std::io::{ErrorKind, Read};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

use super::dev_http::{
    DxCliHttpRequest, DxCliHttpResponse, dev_project_reload_token, parse_http_request,
    with_dev_html_injections_token,
};
use super::devtools;

#[derive(Debug, Clone)]
pub(super) struct DxDevCachedWireResponse {
    pub(super) bytes: Vec<u8>,
    pub(super) expires_at: Instant,
}

pub(super) const DX_DEV_RESPONSE_CACHE_TTL: Duration = Duration::from_millis(500);

pub(super) const DX_DEV_MAX_REQUEST_BYTES: usize = 1024 * 1024;

static DX_DEV_EXTENSION_TOOLCHAIN_TOKENS: OnceLock<Mutex<HashMap<String, String>>> =
    OnceLock::new();

pub(super) type DxDevHttpResponder =
    fn(&PathBuf, &DxCliHttpRequest, &HashMap<String, String>) -> DxCliHttpResponse;

pub(super) type DxDevResponseCache = std::sync::Mutex<HashMap<String, DxDevCachedWireResponse>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum DxDevWireRequestError {
    TooLarge { max_bytes: usize },
    Malformed { message: String },
    Io { message: String },
}

impl DxDevWireRequestError {
    fn status(&self) -> &'static str {
        match self {
            Self::TooLarge { .. } => "413 Payload Too Large",
            Self::Malformed { .. } | Self::Io { .. } => "400 Bad Request",
        }
    }

    fn code(&self) -> &'static str {
        match self {
            Self::TooLarge { .. } => "dx-dev-request-too-large",
            Self::Malformed { .. } => "dx-dev-request-malformed",
            Self::Io { .. } => "dx-dev-request-read-failed",
        }
    }

    fn message(&self) -> String {
        match self {
            Self::TooLarge { max_bytes } => {
                format!("dx dev request exceeded the {max_bytes} byte fallback HTTP limit")
            }
            Self::Malformed { message } | Self::Io { message } => message.clone(),
        }
    }
}

pub(super) fn new_dev_response_cache() -> DxDevResponseCache {
    std::sync::Mutex::new(HashMap::new())
}

pub(super) fn read_http_wire_request<R: Read>(
    reader: &mut R,
) -> Result<Option<String>, DxDevWireRequestError> {
    let mut request = Vec::new();
    let mut chunk = [0; 8192];

    loop {
        let read = match reader.read(&mut chunk) {
            Ok(read) => read,
            Err(error)
                if request.is_empty()
                    && matches!(error.kind(), ErrorKind::TimedOut | ErrorKind::WouldBlock) =>
            {
                return Ok(None);
            }
            Err(error) => {
                return Err(DxDevWireRequestError::Io {
                    message: format!("dx dev could not read HTTP request: {error}"),
                });
            }
        };

        if read == 0 {
            if request.is_empty() {
                return Ok(None);
            }
            if request_bytes_complete(&request)? {
                return Ok(Some(String::from_utf8_lossy(&request).to_string()));
            }
            return Err(DxDevWireRequestError::Malformed {
                message:
                    "dx dev request ended before the declared Content-Length body was received"
                        .to_string(),
            });
        }

        request.extend_from_slice(&chunk[..read]);
        if request.len() > DX_DEV_MAX_REQUEST_BYTES {
            return Err(DxDevWireRequestError::TooLarge {
                max_bytes: DX_DEV_MAX_REQUEST_BYTES,
            });
        }
        if request_bytes_complete(&request)? {
            return Ok(Some(String::from_utf8_lossy(&request).to_string()));
        }
    }
}

pub(super) fn dev_wire_request_error_response(error: &DxDevWireRequestError) -> Vec<u8> {
    let response = DxCliHttpResponse {
        status: error.status().to_string(),
        content_type: "application/json; charset=utf-8".to_string(),
        headers: BTreeMap::from([
            ("cache-control".to_string(), "no-store".to_string()),
            (
                "x-dx-dev-request-error".to_string(),
                error.code().to_string(),
            ),
        ]),
        body: serde_json::json!({
            "error": error.code(),
            "message": error.message(),
            "node_modules_required": false,
            "source_owned": true
        })
        .to_string(),
    };
    dev_wire_response_bytes(&response, false)
}

pub(super) fn dev_response_cache_key(request: &DxCliHttpRequest) -> Option<String> {
    if !matches!(request.method.as_str(), "GET" | "HEAD") {
        return None;
    }

    if request.path.contains("__no_cache") {
        return None;
    }

    let path_only = request.path.split('?').next().unwrap_or("/");
    if path_only == "/api" || path_only.starts_with("/api/") {
        return None;
    }
    if path_only.starts_with("/_dx/") && !path_only.starts_with("/_dx/styles/") {
        return None;
    }

    Some(format!("{} {}", request.method, request.path))
}

pub(super) fn handle_http_wire_response_cached(
    cwd: &PathBuf,
    request: &str,
    translations: &HashMap<String, String>,
    response_cache: &DxDevResponseCache,
    hot_reload: bool,
    devtools_enabled: bool,
    respond: DxDevHttpResponder,
) -> Vec<u8> {
    handle_http_wire_response_cached_with_connection(
        cwd,
        request,
        translations,
        response_cache,
        hot_reload,
        devtools_enabled,
        respond,
        false,
    )
}

#[allow(clippy::too_many_arguments)]
pub(super) fn handle_http_wire_response_cached_with_connection(
    cwd: &PathBuf,
    request: &str,
    translations: &HashMap<String, String>,
    response_cache: &DxDevResponseCache,
    hot_reload: bool,
    devtools_enabled: bool,
    respond: DxDevHttpResponder,
    keep_alive: bool,
) -> Vec<u8> {
    let request = parse_http_request(request);
    if let Some(response) = devtools::devtools_cli_response(cwd, &request, devtools_enabled) {
        return dev_wire_response_bytes_with_connection(
            &response,
            request.method == "HEAD",
            connection_header_value(keep_alive),
        );
    }

    if hot_reload {
        run_dev_extension_toolchain_on_reload_token_change(cwd);
    }
    let cache_key = if hot_reload || devtools_enabled {
        None
    } else {
        dev_response_cache_key(&request)
            .map(|key| format!("{} {}", key, connection_header_value(keep_alive)))
    };
    let now = Instant::now();

    if let Some(cache_key) = cache_key.as_ref() {
        if let Ok(cache) = response_cache.lock() {
            if let Some(cached) = cache.get(cache_key) {
                if cached.expires_at > now {
                    return cached.bytes.clone();
                }
            }
        }
    }

    let mut response = respond(cwd, &request, translations);
    if hot_reload || devtools_enabled {
        let token = hot_reload.then(|| dev_project_reload_token(cwd));
        response =
            with_dev_html_injections_token(response, token.as_deref(), devtools_enabled, false);
    }
    let bytes = dev_wire_response_bytes_with_connection(
        &response,
        request.method == "HEAD",
        connection_header_value(keep_alive),
    );

    if let Some(cache_key) = cache_key {
        if let Ok(mut cache) = response_cache.lock() {
            if cache.len() > 512 {
                cache.clear();
            }
            cache.insert(
                cache_key,
                DxDevCachedWireResponse {
                    bytes: bytes.clone(),
                    expires_at: now + DX_DEV_RESPONSE_CACHE_TTL,
                },
            );
        }
    }

    bytes
}

pub(super) fn dev_wire_request_keep_alive(request: &str) -> bool {
    let mut lines = request.lines();
    let request_line = lines.next().unwrap_or_default();
    let http_11 = request_line
        .split_whitespace()
        .nth(2)
        .is_some_and(|version| version.eq_ignore_ascii_case("HTTP/1.1"));

    for line in lines {
        if line.trim().is_empty() {
            break;
        }
        let Some((name, value)) = line.split_once(':') else {
            continue;
        };
        if !name.trim().eq_ignore_ascii_case("connection") {
            continue;
        }
        let tokens = value
            .split(',')
            .map(|token| token.trim().to_ascii_lowercase())
            .collect::<Vec<_>>();
        if tokens.iter().any(|token| token == "close") {
            return false;
        }
        if tokens.iter().any(|token| token == "keep-alive") {
            return true;
        }
    }

    http_11
}

fn run_dev_extension_toolchain_on_reload_token_change(cwd: &PathBuf) {
    let token = dev_project_reload_token(cwd);
    let key = cwd
        .canonicalize()
        .unwrap_or_else(|_| cwd.clone())
        .to_string_lossy()
        .replace('\\', "/");
    let cache = DX_DEV_EXTENSION_TOOLCHAIN_TOKENS.get_or_init(|| Mutex::new(HashMap::new()));
    if let Ok(mut cache) = cache.lock() {
        if cache
            .get(&key)
            .is_some_and(|last_token| last_token == &token)
        {
            return;
        }
        cache.insert(key, token.clone());
    }

    let commands = [
        ("imports", ["imports", "sync", "--json"].as_slice()),
        ("style", ["style", "build", "--json"].as_slice()),
        ("icons", ["icons", "sync", "--json"].as_slice()),
    ];
    let results = commands
        .iter()
        .map(|(name, args)| run_dev_extension_toolchain_command(cwd, name, args))
        .collect::<Vec<_>>();
    write_dev_extension_toolchain_fallback_receipt(cwd, &token, &results);
}

fn run_dev_extension_toolchain_command(
    cwd: &PathBuf,
    name: &str,
    args: &[&str],
) -> serde_json::Value {
    let executable = match std::env::current_exe() {
        Ok(executable) => executable,
        Err(error) => {
            return serde_json::json!({
                "name": name,
                "passed": false,
                "error": format!("resolve current dx executable: {error}")
            });
        }
    };

    match Command::new(executable)
        .args(args)
        .current_dir(cwd)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
    {
        Ok(output) => serde_json::json!({
            "name": name,
            "command": format!("dx {}", args.join(" ")),
            "passed": output.status.success(),
            "status": output.status.code(),
            "stderr": String::from_utf8_lossy(&output.stderr).trim()
        }),
        Err(error) => serde_json::json!({
            "name": name,
            "command": format!("dx {}", args.join(" ")),
            "passed": false,
            "error": error.to_string()
        }),
    }
}

fn write_dev_extension_toolchain_fallback_receipt(
    cwd: &Path,
    token: &str,
    results: &[serde_json::Value],
) {
    let passed = results
        .iter()
        .all(|result| result.get("passed").and_then(serde_json::Value::as_bool) == Some(true));
    let receipt = serde_json::json!({
        "schema": "dx.dev.extension_toolchain",
        "version": 2,
        "config_source": "dx",
        "generated_at": chrono::Utc::now().to_rfc3339(),
        "trigger": "fallback-hot-reload-token",
        "reload_token": token,
        "planned_commands": ["imports", "style", "icons"],
        "policy": "non-dev-server dx dev runs imports, style, and icon tools once per source reload token before the browser refreshes",
        "passed": passed,
        "commands": results,
    });
    let path = cwd.join(".dx/receipts/run/dev-extension-toolchain.json");
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(serialized) = serde_json::to_string_pretty(&receipt) {
        let _ = std::fs::write(path, format!("{serialized}\n"));
    }
}

fn request_bytes_complete(request: &[u8]) -> Result<bool, DxDevWireRequestError> {
    let Some((body_start, headers)) = split_http_header_bytes(request) else {
        return Ok(false);
    };
    let Some(content_length) = declared_content_length(headers)? else {
        return Ok(true);
    };
    Ok(request.len() >= body_start.saturating_add(content_length))
}

fn split_http_header_bytes(request: &[u8]) -> Option<(usize, &[u8])> {
    find_bytes(request, b"\r\n\r\n")
        .map(|index| (index + 4, &request[..index + 4]))
        .or_else(|| find_bytes(request, b"\n\n").map(|index| (index + 2, &request[..index + 2])))
}

fn declared_content_length(headers: &[u8]) -> Result<Option<usize>, DxDevWireRequestError> {
    let text = String::from_utf8_lossy(headers);
    for line in text.lines().skip(1) {
        let Some((name, value)) = line.split_once(':') else {
            continue;
        };
        if !name.trim().eq_ignore_ascii_case("content-length") {
            continue;
        }
        let value = value.trim();
        let parsed = value
            .parse::<usize>()
            .map_err(|_| DxDevWireRequestError::Malformed {
                message: format!("dx dev received an invalid Content-Length header: {value}"),
            })?;
        if parsed > DX_DEV_MAX_REQUEST_BYTES {
            return Err(DxDevWireRequestError::TooLarge {
                max_bytes: DX_DEV_MAX_REQUEST_BYTES,
            });
        }
        return Ok(Some(parsed));
    }
    Ok(None)
}

fn find_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

pub(super) fn dev_wire_response_bytes(response: &DxCliHttpResponse, head_request: bool) -> Vec<u8> {
    dev_wire_response_bytes_with_connection(response, head_request, "close")
}

pub(super) fn dev_wire_response_bytes_with_connection(
    response: &DxCliHttpResponse,
    head_request: bool,
    connection: &str,
) -> Vec<u8> {
    let body_bytes = response.body.as_bytes();
    let headers = dev_response_headers_with_connection(response, body_bytes.len(), connection);
    let status_and_headers = format!("HTTP/1.1 {}\r\n{}\r\n\r\n", response.status, headers);
    let mut bytes = Vec::with_capacity(status_and_headers.len() + body_bytes.len());
    bytes.extend_from_slice(status_and_headers.as_bytes());
    if !head_request {
        bytes.extend_from_slice(body_bytes);
    }
    bytes
}

pub(super) fn dev_response_headers(response: &DxCliHttpResponse, content_length: usize) -> String {
    dev_response_headers_with_connection(response, content_length, "close")
}

pub(super) fn dev_response_headers_with_connection(
    response: &DxCliHttpResponse,
    content_length: usize,
    connection: &str,
) -> String {
    let mut headers = vec![
        format!("Content-Type: {}", response.content_type),
        format!("Content-Length: {content_length}"),
        format!("Connection: {connection}"),
    ];
    headers.extend(
        response
            .headers
            .iter()
            .filter(|(name, _)| {
                !matches!(
                    name.as_str(),
                    "content-type" | "content-length" | "connection"
                )
            })
            .map(|(name, value)| format!("{}: {}", http_header_name(name), value)),
    );
    headers.join("\r\n")
}

fn http_header_name(name: &str) -> String {
    name.split('-')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => {
                    format!("{}{}", first.to_ascii_uppercase(), chars.as_str())
                }
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join("-")
}

fn connection_header_value(keep_alive: bool) -> &'static str {
    if keep_alive { "keep-alive" } else { "close" }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::{
        DxCliHttpResponse, dev_wire_request_keep_alive, dev_wire_response_bytes,
        dev_wire_response_bytes_with_connection,
    };

    #[test]
    fn wire_request_keep_alive_follows_http_connection_rules() {
        assert!(dev_wire_request_keep_alive(
            "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n"
        ));
        assert!(dev_wire_request_keep_alive(
            "GET / HTTP/1.0\r\nConnection: keep-alive\r\n\r\n"
        ));
        assert!(!dev_wire_request_keep_alive(
            "GET / HTTP/1.1\r\nConnection: close\r\n\r\n"
        ));
        assert!(!dev_wire_request_keep_alive(
            "GET / HTTP/1.0\r\nHost: localhost\r\n\r\n"
        ));
    }

    #[test]
    fn wire_response_can_keep_connection_alive_without_changing_default() {
        let response = DxCliHttpResponse {
            status: "200 OK".to_string(),
            content_type: "text/plain; charset=utf-8".to_string(),
            headers: BTreeMap::new(),
            body: "hello dx".to_string(),
        };

        let default_frame =
            String::from_utf8(dev_wire_response_bytes(&response, false)).expect("wire frame");
        let keep_alive_frame = String::from_utf8(dev_wire_response_bytes_with_connection(
            &response,
            false,
            "keep-alive",
        ))
        .expect("wire frame");

        assert!(default_frame.contains("Connection: close"));
        assert!(keep_alive_frame.contains("Connection: keep-alive"));
        assert!(keep_alive_frame.ends_with("hello dx"));
    }
}
