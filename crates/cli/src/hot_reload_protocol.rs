//! Source-owned hot reload protocol helpers for the DX-WWW dev server.

use serde_json::{Value, json};

const DEFAULT_RESOURCE: &str = DX_HOT_RELOAD_DEFAULT_RESOURCE;

pub(crate) const DX_HOT_RELOAD_DEFAULT_RESOURCE: &str = "route:/";
pub(crate) const DX_HOT_RELOAD_PROTOCOL: &str = "dx.hot-reload.poll";
pub(crate) const DX_HOT_RELOAD_PROTOCOL_FORMAT: u64 = 1;
pub(crate) const DX_HOT_RELOAD_TRANSPORT: &str = "poll";
pub(crate) const DX_HOT_RELOAD_VERSION_ENDPOINT: &str = "/_dx/hot-reload/version";
pub(crate) const DX_HOT_RELOAD_EVENT_STREAM_TRANSPORT: &str = "sse";
pub(crate) const DX_HOT_RELOAD_EVENT_STREAM_ENDPOINT: &str = "/_dx/hot-reload/events";
pub(crate) const DX_HOT_RELOAD_EVENT_NAME: &str = "dx-hot-reload";
pub(crate) const DX_HOT_RELOAD_EVENT_RETRY_MS: u64 = 1000;
pub(crate) const DX_HOT_RELOAD_RESOURCE_QUERY_PARAM: &str = "resource";
pub(crate) const DX_HOT_RELOAD_RESOURCE_MARKER: &str = "data-dx-hot-reload-target";
pub(crate) const DX_HOT_RELOAD_POLL_RECEIPT_SCHEMA: &str = "dx.dev.hot_reload.poll_receipt";
pub(crate) const DX_HOT_RELOAD_POLL_RECEIPT_FORMAT: u64 = 1;
pub(crate) const DX_HOT_RELOAD_SOURCE: &str = "dx-www-rust-dev-server";
pub(crate) const DX_HOT_RELOAD_SOURCE_OWNED_CONTRACT: bool = true;
pub(crate) const DX_HOT_RELOAD_NODE_MODULES_REQUIRED: bool = false;
pub(crate) const DX_HOT_RELOAD_STREAMING_BOUNDARY: &str = "adapter-boundary";
pub(crate) const DX_HOT_RELOAD_NODE_RUNTIME_BOUNDARY: &str = "not-required";
pub(crate) const DX_HOT_RELOAD_RESTART_INSTRUCTION: &str = "restart";
pub(crate) const DX_HOT_RELOAD_FULL_PAGE_MODE: &str = "full-page";
pub(crate) const DX_HOT_RELOAD_STYLE_REFRESH_INSTRUCTION: &str = "refresh-style";
pub(crate) const DX_HOT_RELOAD_STYLE_REFRESH_MODE: &str = "stylesheet-link";
pub(crate) const DX_HOT_RELOAD_ASSET_REFRESH_INSTRUCTION: &str = "refresh-asset";
pub(crate) const DX_HOT_RELOAD_ASSET_REFRESH_MODE: &str = "dom-asset-url";
pub(crate) const DX_HOT_RELOAD_ISSUE_INSTRUCTION: &str = "report-issue";
pub(crate) const DX_HOT_RELOAD_ISSUE_MODE: &str = "diagnostic-overlay";
pub(crate) const DX_HOT_RELOAD_CLEAR_ISSUE_INSTRUCTION: &str = "clear-issue";
pub(crate) const DX_HOT_RELOAD_CLEAR_ISSUE_MODE: &str = "diagnostic-recovery";
pub(crate) const DX_HOT_RELOAD_ISSUE_RECEIPT_SCHEMA: &str = "dx.dev.hot_reload.issue_receipt";
pub(crate) const DX_HOT_RELOAD_ISSUE_RECEIPT_FORMAT: u64 = 1;
pub(crate) const DX_HOT_RELOAD_DISABLED_INSTRUCTION: &str = "disabled";
pub(crate) const DX_HOT_RELOAD_DISABLED_MODE: &str = "none";

#[cfg(any(feature = "dev-server", test))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DxHotReloadIssue {
    severity: String,
    code: String,
    message: String,
    file: Option<String>,
    line: Option<u64>,
    column: Option<u64>,
    code_frame: Option<String>,
    next_action: Option<String>,
}

#[cfg(any(feature = "dev-server", test))]
impl DxHotReloadIssue {
    #[must_use]
    pub(crate) fn new(
        severity: impl Into<String>,
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            severity: severity.into(),
            code: code.into(),
            message: message.into(),
            file: None,
            line: None,
            column: None,
            code_frame: None,
            next_action: None,
        }
    }

    #[must_use]
    pub(crate) fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new("error", code, message)
    }

    #[must_use]
    pub(crate) fn with_source_location(
        mut self,
        file: impl Into<String>,
        line: u64,
        column: u64,
    ) -> Self {
        self.file = Some(file.into());
        self.line = Some(line);
        self.column = Some(column);
        self
    }

    #[must_use]
    pub(crate) fn with_code_frame(mut self, code_frame: impl Into<String>) -> Self {
        self.code_frame = Some(code_frame.into());
        self
    }

    #[must_use]
    pub(crate) fn with_next_action(mut self, next_action: impl Into<String>) -> Self {
        self.next_action = Some(next_action.into());
        self
    }
}

/// Extract the route or asset resource requested by the dev hot-reload client.
#[must_use]
pub(crate) fn dx_hot_reload_resource_from_path(path: &str) -> String {
    query_value(path, DX_HOT_RELOAD_RESOURCE_QUERY_PARAM)
        .as_deref()
        .map(normalize_resource_id)
        .unwrap_or_else(|| DEFAULT_RESOURCE.to_string())
}

/// Build the JSON response returned by `/_dx/hot-reload/version`.
#[must_use]
pub(crate) fn dx_hot_reload_version_payload(
    hot_reload: bool,
    token: String,
    version: u64,
    resource_id: &str,
) -> Value {
    let resource_id = normalize_resource_id(resource_id);
    let resource_kind = resource_kind(&resource_id);
    let (instruction_type, mode) = instruction_for_resource(resource_kind, hot_reload);
    let token = if hot_reload {
        token
    } else {
        "disabled".to_string()
    };

    json!({
        "ok": true,
        "protocol": DX_HOT_RELOAD_PROTOCOL,
        "protocol_format": DX_HOT_RELOAD_PROTOCOL_FORMAT,
        "transport": DX_HOT_RELOAD_TRANSPORT,
        "source_owned_contract": DX_HOT_RELOAD_SOURCE_OWNED_CONTRACT,
        "token": token,
        "version": version,
        "resource": {
            "kind": resource_kind,
            "id": resource_id.as_str(),
        },
        "instruction": {
            "type": instruction_type,
            "mode": mode,
            "resource": {
                "kind": resource_kind,
                "id": resource_id.as_str(),
            },
        },
        "receipt": {
            "schema": DX_HOT_RELOAD_POLL_RECEIPT_SCHEMA,
            "format": DX_HOT_RELOAD_POLL_RECEIPT_FORMAT,
            "source": DX_HOT_RELOAD_SOURCE,
            "source_owned_contract": DX_HOT_RELOAD_SOURCE_OWNED_CONTRACT,
            "protocol": DX_HOT_RELOAD_PROTOCOL,
            "protocol_format": DX_HOT_RELOAD_PROTOCOL_FORMAT,
            "transport": DX_HOT_RELOAD_TRANSPORT,
            "version": version,
            "hot_reload_enabled": hot_reload,
            "resource": {
                "kind": resource_kind,
                "id": resource_id.as_str(),
            },
            "instruction": {
                "type": instruction_type,
                "mode": mode,
            },
            "studio": {
                "version_endpoint": DX_HOT_RELOAD_VERSION_ENDPOINT,
                "resource_query_param": DX_HOT_RELOAD_RESOURCE_QUERY_PARAM,
                "resource_marker": DX_HOT_RELOAD_RESOURCE_MARKER,
            },
            "boundaries": {
                "runtime": "dx-owned",
                "streaming": DX_HOT_RELOAD_STREAMING_BOUNDARY,
                "partial_module_updates": false,
                "node_runtime": DX_HOT_RELOAD_NODE_RUNTIME_BOUNDARY,
            },
        },
        "capabilities": {
            "route_scoped_resources": true,
            "css_hot_swap": resource_kind == "style" && hot_reload,
            "asset_hot_swap": resource_kind == "asset" && hot_reload,
            "partial_module_updates": false,
            "issue_stream": false,
        },
        "boundaries": {
            "runtime": "dx-owned",
            "server": "axum",
            "source_owned_contract": DX_HOT_RELOAD_SOURCE_OWNED_CONTRACT,
            "streaming": DX_HOT_RELOAD_STREAMING_BOUNDARY,
            "partial_updates": DX_HOT_RELOAD_STREAMING_BOUNDARY,
            "node_runtime": DX_HOT_RELOAD_NODE_RUNTIME_BOUNDARY,
        },
    })
}

/// Build the payload emitted over the Server-Sent Events hot-reload stream.
#[cfg(any(feature = "dev-server", test))]
#[must_use]
pub(crate) fn dx_hot_reload_event_stream_payload(
    hot_reload: bool,
    token: String,
    version: u64,
    resource_id: &str,
) -> Value {
    let mut payload = dx_hot_reload_version_payload(hot_reload, token, version, resource_id);
    let event_stream = json!({
        "endpoint": DX_HOT_RELOAD_EVENT_STREAM_ENDPOINT,
        "event": DX_HOT_RELOAD_EVENT_NAME,
        "retry_ms": DX_HOT_RELOAD_EVENT_RETRY_MS,
        "transport": DX_HOT_RELOAD_EVENT_STREAM_TRANSPORT,
        "fallback_transport": DX_HOT_RELOAD_TRANSPORT,
        "initial": false,
    });

    if let Some(payload) = payload.as_object_mut() {
        payload.insert(
            "transport".to_string(),
            json!(DX_HOT_RELOAD_EVENT_STREAM_TRANSPORT),
        );
        payload.insert(
            "fallback_transport".to_string(),
            json!(DX_HOT_RELOAD_TRANSPORT),
        );
        payload.insert("event_stream".to_string(), event_stream.clone());

        if let Some(receipt) = payload.get_mut("receipt").and_then(Value::as_object_mut) {
            receipt.insert("event_stream".to_string(), event_stream);
            receipt.insert(
                "fallback_transport".to_string(),
                json!(DX_HOT_RELOAD_TRANSPORT),
            );
        }
    }

    payload
}

/// Build the first Server-Sent Events payload sent after a client subscribes.
#[cfg(any(feature = "dev-server", test))]
#[must_use]
pub(crate) fn dx_hot_reload_event_stream_initial_payload(
    hot_reload: bool,
    token: String,
    version: u64,
    resource_id: &str,
) -> Value {
    let mut payload = dx_hot_reload_event_stream_payload(hot_reload, token, version, resource_id);
    let initial_marker = json!({
        "initial": true,
    });

    if let Some(payload) = payload.as_object_mut() {
        payload.insert("event_stream_initial".to_string(), json!(true));

        if let Some(event_stream) = payload
            .get_mut("event_stream")
            .and_then(Value::as_object_mut)
        {
            if let Some(initial_marker) = initial_marker.as_object() {
                for (key, value) in initial_marker {
                    event_stream.insert(key.clone(), value.clone());
                }
            }
        }

        if let Some(receipt_event_stream) = payload
            .get_mut("receipt")
            .and_then(Value::as_object_mut)
            .and_then(|receipt| receipt.get_mut("event_stream"))
            .and_then(Value::as_object_mut)
        {
            receipt_event_stream.insert("initial".to_string(), json!(true));
        }
    }

    payload
}

#[cfg(any(feature = "dev-server", test))]
#[must_use]
pub(crate) fn dx_hot_reload_issue_payload(
    hot_reload: bool,
    token: String,
    version: u64,
    resource_id: &str,
    issues: &[DxHotReloadIssue],
) -> Value {
    let resource_id = normalize_resource_id(resource_id);
    let resource_kind = resource_kind(&resource_id);
    let mut payload = dx_hot_reload_event_stream_payload(hot_reload, token, version, &resource_id);
    let issue_values = issues.iter().map(issue_to_value).collect::<Vec<_>>();
    let instruction_type = if hot_reload {
        DX_HOT_RELOAD_ISSUE_INSTRUCTION
    } else {
        DX_HOT_RELOAD_DISABLED_INSTRUCTION
    };
    let instruction_mode = if hot_reload {
        DX_HOT_RELOAD_ISSUE_MODE
    } else {
        DX_HOT_RELOAD_DISABLED_MODE
    };
    let instruction = json!({
        "type": instruction_type,
        "mode": instruction_mode,
        "resource": {
            "kind": resource_kind,
            "id": resource_id.as_str(),
        },
    });
    let issue_receipt = json!({
        "schema": DX_HOT_RELOAD_ISSUE_RECEIPT_SCHEMA,
        "format": DX_HOT_RELOAD_ISSUE_RECEIPT_FORMAT,
        "source": DX_HOT_RELOAD_SOURCE,
        "protocol": DX_HOT_RELOAD_PROTOCOL,
        "protocol_format": DX_HOT_RELOAD_PROTOCOL_FORMAT,
        "transport": DX_HOT_RELOAD_EVENT_STREAM_TRANSPORT,
        "fallback_transport": DX_HOT_RELOAD_TRANSPORT,
        "source_owned_contract": DX_HOT_RELOAD_SOURCE_OWNED_CONTRACT,
        "version": version,
        "hot_reload_enabled": hot_reload,
        "resource": {
            "kind": resource_kind,
            "id": resource_id.as_str(),
        },
        "instruction": {
            "type": instruction_type,
            "mode": instruction_mode,
        },
        "issue_count": issue_values.len(),
        "issues": issue_values,
        "boundaries": {
            "runtime": "dx-owned",
            "streaming": DX_HOT_RELOAD_STREAMING_BOUNDARY,
            "partial_module_updates": false,
            "node_runtime": DX_HOT_RELOAD_NODE_RUNTIME_BOUNDARY,
        },
    });

    if let Some(payload) = payload.as_object_mut() {
        payload.insert("instruction".to_string(), instruction.clone());
        payload.insert("issue_receipt".to_string(), issue_receipt.clone());
        payload.insert("issues".to_string(), issue_receipt["issues"].clone());

        if let Some(event_stream) = payload
            .get_mut("event_stream")
            .and_then(Value::as_object_mut)
        {
            event_stream.insert("issue_stream".to_string(), json!(hot_reload));
            event_stream.insert(
                "issue_receipt_schema".to_string(),
                json!(DX_HOT_RELOAD_ISSUE_RECEIPT_SCHEMA),
            );
        }

        if let Some(capabilities) = payload
            .get_mut("capabilities")
            .and_then(Value::as_object_mut)
        {
            capabilities.insert("issue_stream".to_string(), json!(hot_reload));
            capabilities.insert("partial_module_updates".to_string(), json!(false));
        }

        if let Some(receipt) = payload.get_mut("receipt").and_then(Value::as_object_mut) {
            receipt.insert("instruction".to_string(), instruction);
            receipt.insert(
                "issue_stream".to_string(),
                json!({
                    "schema": DX_HOT_RELOAD_ISSUE_RECEIPT_SCHEMA,
                    "format": DX_HOT_RELOAD_ISSUE_RECEIPT_FORMAT,
                    "issue_count": issue_receipt["issue_count"].clone(),
                    "transport": DX_HOT_RELOAD_EVENT_STREAM_TRANSPORT,
                }),
            );
        }
    }

    payload
}

#[cfg(any(feature = "dev-server", test))]
#[must_use]
pub(crate) fn dx_hot_reload_issue_recovery_payload(
    hot_reload: bool,
    token: String,
    version: u64,
    resource_id: &str,
) -> Value {
    let resource_id = normalize_resource_id(resource_id);
    let resource_kind = resource_kind(&resource_id);
    let mut payload = dx_hot_reload_event_stream_payload(hot_reload, token, version, &resource_id);
    let instruction_type = if hot_reload {
        DX_HOT_RELOAD_CLEAR_ISSUE_INSTRUCTION
    } else {
        DX_HOT_RELOAD_DISABLED_INSTRUCTION
    };
    let instruction_mode = if hot_reload {
        DX_HOT_RELOAD_CLEAR_ISSUE_MODE
    } else {
        DX_HOT_RELOAD_DISABLED_MODE
    };
    let instruction = json!({
        "type": instruction_type,
        "mode": instruction_mode,
        "resource": {
            "kind": resource_kind,
            "id": resource_id.as_str(),
        },
    });
    let issue_recovery = json!({
        "schema": DX_HOT_RELOAD_ISSUE_RECEIPT_SCHEMA,
        "format": DX_HOT_RELOAD_ISSUE_RECEIPT_FORMAT,
        "source": DX_HOT_RELOAD_SOURCE,
        "protocol": DX_HOT_RELOAD_PROTOCOL,
        "protocol_format": DX_HOT_RELOAD_PROTOCOL_FORMAT,
        "transport": DX_HOT_RELOAD_EVENT_STREAM_TRANSPORT,
        "fallback_transport": DX_HOT_RELOAD_TRANSPORT,
        "source_owned_contract": DX_HOT_RELOAD_SOURCE_OWNED_CONTRACT,
        "version": version,
        "hot_reload_enabled": hot_reload,
        "resource": {
            "kind": resource_kind,
            "id": resource_id.as_str(),
        },
        "instruction": {
            "type": instruction_type,
            "mode": instruction_mode,
        },
        "issue_count": 0,
        "recovered": hot_reload,
        "boundaries": {
            "runtime": "dx-owned",
            "streaming": DX_HOT_RELOAD_STREAMING_BOUNDARY,
            "partial_module_updates": false,
            "node_runtime": DX_HOT_RELOAD_NODE_RUNTIME_BOUNDARY,
        },
    });

    if let Some(payload) = payload.as_object_mut() {
        payload.insert("instruction".to_string(), instruction.clone());
        payload.insert("issues".to_string(), json!([]));
        payload.insert("issue_recovery".to_string(), issue_recovery.clone());

        if let Some(event_stream) = payload
            .get_mut("event_stream")
            .and_then(Value::as_object_mut)
        {
            event_stream.insert("issue_stream".to_string(), json!(hot_reload));
            event_stream.insert("issue_recovered".to_string(), json!(hot_reload));
            event_stream.insert(
                "issue_receipt_schema".to_string(),
                json!(DX_HOT_RELOAD_ISSUE_RECEIPT_SCHEMA),
            );
        }

        if let Some(capabilities) = payload
            .get_mut("capabilities")
            .and_then(Value::as_object_mut)
        {
            capabilities.insert("issue_stream".to_string(), json!(hot_reload));
            capabilities.insert("partial_module_updates".to_string(), json!(false));
        }

        if let Some(receipt) = payload.get_mut("receipt").and_then(Value::as_object_mut) {
            receipt.insert("instruction".to_string(), instruction);
            receipt.insert(
                "issue_stream".to_string(),
                json!({
                    "schema": DX_HOT_RELOAD_ISSUE_RECEIPT_SCHEMA,
                    "format": DX_HOT_RELOAD_ISSUE_RECEIPT_FORMAT,
                    "issue_count": 0,
                    "recovered": hot_reload,
                    "transport": DX_HOT_RELOAD_EVENT_STREAM_TRANSPORT,
                }),
            );
        }
    }

    payload
}

/// Serialize a hot-reload payload as one Server-Sent Events frame.
#[cfg(any(feature = "dev-server", test))]
#[must_use]
pub(crate) fn dx_hot_reload_sse_frame(payload: &Value) -> String {
    format!(
        "event: {DX_HOT_RELOAD_EVENT_NAME}\nretry: {DX_HOT_RELOAD_EVENT_RETRY_MS}\ndata: {}\n\n",
        payload
    )
}

fn instruction_for_resource(resource_kind: &str, hot_reload: bool) -> (&'static str, &'static str) {
    if !hot_reload {
        return (
            DX_HOT_RELOAD_DISABLED_INSTRUCTION,
            DX_HOT_RELOAD_DISABLED_MODE,
        );
    }

    if resource_kind == "style" {
        (
            DX_HOT_RELOAD_STYLE_REFRESH_INSTRUCTION,
            DX_HOT_RELOAD_STYLE_REFRESH_MODE,
        )
    } else if resource_kind == "asset" {
        (
            DX_HOT_RELOAD_ASSET_REFRESH_INSTRUCTION,
            DX_HOT_RELOAD_ASSET_REFRESH_MODE,
        )
    } else {
        (
            DX_HOT_RELOAD_RESTART_INSTRUCTION,
            DX_HOT_RELOAD_FULL_PAGE_MODE,
        )
    }
}

fn query_value(path: &str, key: &str) -> Option<String> {
    let query = path.split_once('?')?.1;
    query.split('&').find_map(|pair| {
        let (raw_key, raw_value) = pair.split_once('=').unwrap_or((pair, ""));
        (percent_decode_query_component(raw_key) == key)
            .then(|| percent_decode_query_component(raw_value))
    })
}

fn normalize_resource_id(value: &str) -> String {
    let value = value.trim();
    if value.is_empty() || value.chars().any(char::is_control) {
        return DEFAULT_RESOURCE.to_string();
    }

    if let Some(route) = value.strip_prefix("route:") {
        return normalize_route_resource(route);
    }
    if value.starts_with('/') {
        return format!("route:{value}");
    }
    if let Some(asset) = value.strip_prefix("asset:") {
        return normalize_static_resource_id("asset", asset, true);
    }
    if let Some(style) = value.strip_prefix("style:") {
        return normalize_static_resource_id("style", style, false);
    }

    DEFAULT_RESOURCE.to_string()
}

#[cfg(feature = "dev-server")]
#[must_use]
pub(crate) fn dx_hot_reload_normalize_resource_id(value: &str) -> String {
    normalize_resource_id(value)
}

fn normalize_static_resource_id(kind: &str, raw_path: &str, trim_public_prefix: bool) -> String {
    let path = strip_static_resource_suffix(raw_path)
        .trim()
        .replace('\\', "/")
        .trim_start_matches('/')
        .chars()
        .take(250)
        .collect::<String>();
    let path = if trim_public_prefix {
        path.strip_prefix("public/").unwrap_or(path.as_str())
    } else {
        path.as_str()
    };

    if path.is_empty()
        || path.chars().any(char::is_control)
        || path.split('/').any(|segment| {
            segment.is_empty() || segment == "." || segment == ".." || segment == "node_modules"
        })
    {
        return DEFAULT_RESOURCE.to_string();
    }

    format!("{kind}:{path}")
}

fn strip_static_resource_suffix(raw_path: &str) -> &str {
    raw_path
        .char_indices()
        .find(|(_, character)| *character == '?' || *character == '#')
        .map_or(raw_path, |(index, _)| &raw_path[..index])
}

fn normalize_route_resource(route: &str) -> String {
    let route = route.trim();
    if route.is_empty() {
        return DEFAULT_RESOURCE.to_string();
    }
    let route = route.chars().take(250).collect::<String>();
    if route.starts_with('/') {
        format!("route:{route}")
    } else {
        format!("route:/{route}")
    }
}

fn resource_kind(resource_id: &str) -> &'static str {
    if resource_id.starts_with("route:") {
        "route"
    } else if resource_id.starts_with("style:") {
        "style"
    } else if resource_id.starts_with("asset:") {
        "asset"
    } else {
        "resource"
    }
}

#[cfg(any(feature = "dev-server", test))]
fn issue_to_value(issue: &DxHotReloadIssue) -> Value {
    json!({
        "severity": issue.severity.as_str(),
        "code": issue.code.as_str(),
        "message": issue.message.as_str(),
        "file": issue.file.as_deref(),
        "line": issue.line,
        "column": issue.column,
        "code_frame": issue.code_frame.as_deref(),
        "next_action": issue.next_action.as_deref(),
    })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resource_from_path_decodes_route_scope() {
        assert_eq!(
            dx_hot_reload_resource_from_path(
                "/_dx/hot-reload/version?resource=route%3A%2Fdashboard"
            ),
            "route:/dashboard"
        );
        assert_eq!(
            dx_hot_reload_resource_from_path("/_dx/hot-reload/version?resource=%2Fdashboard"),
            "route:/dashboard"
        );
    }

    #[test]
    fn resource_from_path_accepts_style_and_asset_but_rejects_unknown() {
        assert_eq!(
            dx_hot_reload_resource_from_path("/_dx/hot-reload/version?resource=style%3Aglobal.css"),
            "style:global.css"
        );
        assert_eq!(
            dx_hot_reload_resource_from_path(
                "/_dx/hot-reload/version?resource=asset%3Apublic%2Flogo.svg"
            ),
            "asset:logo.svg"
        );
        assert_eq!(
            dx_hot_reload_resource_from_path(
                "/_dx/hot-reload/version?resource=javascript%3Aalert(1)"
            ),
            DX_HOT_RELOAD_DEFAULT_RESOURCE
        );
        assert_eq!(
            dx_hot_reload_resource_from_path("/_dx/hot-reload/version?resource=route%3A%0A"),
            DX_HOT_RELOAD_DEFAULT_RESOURCE
        );
    }

    #[test]
    fn static_resource_ids_strip_query_hash_and_public_prefixes() {
        assert_eq!(
            dx_hot_reload_version_payload(
                true,
                "asset-token".to_string(),
                1,
                "asset:logo.svg?v=1#icon"
            )["resource"]["id"],
            "asset:logo.svg"
        );
        assert_eq!(
            dx_hot_reload_resource_from_path(
                "/_dx/hot-reload/version?resource=asset%3Apublic%2Flogo.svg"
            ),
            "asset:logo.svg"
        );
        assert_eq!(
            dx_hot_reload_version_payload(
                true,
                "public-asset-token".to_string(),
                1,
                "asset:public/logo.svg",
            )["resource"]["id"],
            "asset:logo.svg"
        );
        assert_eq!(
            dx_hot_reload_version_payload(
                true,
                "style-token".to_string(),
                2,
                "style:styles/app.css?v=1#sheet",
            )["resource"]["id"],
            "style:styles/app.css"
        );
        assert_eq!(
            dx_hot_reload_resource_from_path(
                "/_dx/hot-reload/version?resource=asset%3A..%2Fsecret.svg"
            ),
            DX_HOT_RELOAD_DEFAULT_RESOURCE
        );
        assert_eq!(
            dx_hot_reload_resource_from_path(
                "/_dx/hot-reload/version?resource=style%3Anode_modules%2Fvendor.css"
            ),
            DX_HOT_RELOAD_DEFAULT_RESOURCE
        );
    }

    #[test]
    fn version_payload_keeps_streaming_and_partial_updates_adapter_bound() {
        let payload = dx_hot_reload_version_payload(
            true,
            "1-source-token".to_string(),
            1,
            "route:/dashboard",
        );

        assert_eq!(payload["protocol"], DX_HOT_RELOAD_PROTOCOL);
        assert_eq!(payload["protocol_format"], DX_HOT_RELOAD_PROTOCOL_FORMAT);
        assert_eq!(payload["instruction"]["type"], "restart");
        assert_eq!(payload["resource"]["kind"], "route");
        assert_eq!(payload["source_owned_contract"], true);
        assert_eq!(payload["capabilities"]["partial_module_updates"], false);
        assert_eq!(payload["boundaries"]["streaming"], "adapter-boundary");
        assert_eq!(payload["boundaries"]["source_owned_contract"], true);
        assert_eq!(
            payload["receipt"]["schema"],
            DX_HOT_RELOAD_POLL_RECEIPT_SCHEMA
        );
        assert_eq!(payload["receipt"]["source"], DX_HOT_RELOAD_SOURCE);
        assert_eq!(payload["receipt"]["source_owned_contract"], true);
        assert_eq!(payload["receipt"]["resource"]["id"], "route:/dashboard");
        assert_eq!(
            payload["receipt"]["studio"]["resource_marker"],
            DX_HOT_RELOAD_RESOURCE_MARKER
        );
        assert_eq!(
            payload["receipt"]["boundaries"]["streaming"],
            DX_HOT_RELOAD_STREAMING_BOUNDARY
        );
        assert!(!payload.to_string().contains("node_modules"));
    }

    #[test]
    fn poll_receipt_uses_stable_schema_name_with_numeric_format() {
        let payload = dx_hot_reload_version_payload(
            true,
            "1-source-token".to_string(),
            1,
            "route:/dashboard",
        );

        assert_eq!(DX_HOT_RELOAD_PROTOCOL, "dx.hot-reload.poll");
        assert!(!DX_HOT_RELOAD_PROTOCOL.contains(".v1"));
        assert_eq!(DX_HOT_RELOAD_PROTOCOL_FORMAT, 1);
        assert_eq!(
            DX_HOT_RELOAD_POLL_RECEIPT_SCHEMA,
            "dx.dev.hot_reload.poll_receipt"
        );
        assert!(!DX_HOT_RELOAD_POLL_RECEIPT_SCHEMA.contains(".v1"));
        assert_eq!(
            payload["receipt"]["schema"],
            DX_HOT_RELOAD_POLL_RECEIPT_SCHEMA
        );
        assert_eq!(payload["receipt"]["format"], 1);
        assert_eq!(
            payload["receipt"]["protocol_format"],
            DX_HOT_RELOAD_PROTOCOL_FORMAT
        );
    }

    #[test]
    fn style_payload_uses_stylesheet_refresh_without_claiming_module_hmr() {
        let payload = dx_hot_reload_version_payload(
            true,
            "2-source-token".to_string(),
            2,
            "style:global.css",
        );

        assert_eq!(payload["instruction"]["type"], "refresh-style");
        assert_eq!(payload["instruction"]["mode"], "stylesheet-link");
        assert_eq!(payload["instruction"]["resource"]["kind"], "style");
        assert_eq!(payload["capabilities"]["css_hot_swap"], true);
        assert_eq!(payload["capabilities"]["partial_module_updates"], false);
        assert_eq!(
            payload["receipt"]["boundaries"]["streaming"],
            DX_HOT_RELOAD_STREAMING_BOUNDARY
        );
    }

    #[test]
    fn asset_payload_uses_dom_asset_refresh_without_claiming_module_hmr() {
        let payload = dx_hot_reload_version_payload(
            true,
            "3-source-token".to_string(),
            3,
            "asset:favicon.svg",
        );

        assert_eq!(payload["instruction"]["type"], "refresh-asset");
        assert_eq!(payload["instruction"]["mode"], "dom-asset-url");
        assert_eq!(payload["instruction"]["resource"]["kind"], "asset");
        assert_eq!(payload["capabilities"]["asset_hot_swap"], true);
        assert_eq!(payload["capabilities"]["partial_module_updates"], false);
        assert_eq!(
            payload["receipt"]["boundaries"]["streaming"],
            DX_HOT_RELOAD_STREAMING_BOUNDARY
        );
    }

    #[test]
    fn sse_frame_serializes_single_event_payload() {
        let payload = dx_hot_reload_event_stream_payload(
            true,
            "3-source-token".to_string(),
            3,
            "style:app.css",
        );
        let frame = dx_hot_reload_sse_frame(&payload);

        assert!(frame.starts_with("event: dx-hot-reload\nretry: 1000\ndata: {"));
        assert!(frame.contains("\"instruction\":{\"mode\":\"stylesheet-link\",\"resource\""));
        assert!(frame.contains("\"fallback_transport\":\"poll\""));
        assert!(frame.contains("\"transport\":\"sse\""));
        assert!(frame.ends_with("\n\n"));
        assert!(!frame.contains("_next"));
    }

    #[test]
    fn event_stream_payload_marks_sse_transport_with_poll_fallback() {
        let payload = dx_hot_reload_event_stream_payload(
            true,
            "4-source-token".to_string(),
            4,
            "route:/dashboard",
        );

        assert_eq!(payload["transport"], DX_HOT_RELOAD_EVENT_STREAM_TRANSPORT);
        assert_eq!(payload["fallback_transport"], DX_HOT_RELOAD_TRANSPORT);
        assert_eq!(
            payload["event_stream"]["endpoint"],
            DX_HOT_RELOAD_EVENT_STREAM_ENDPOINT
        );
        assert_eq!(payload["event_stream"]["retry_ms"], 1000);
        assert_eq!(
            payload["event_stream"]["fallback_transport"],
            DX_HOT_RELOAD_TRANSPORT
        );
        assert_eq!(
            payload["receipt"]["event_stream"]["transport"],
            DX_HOT_RELOAD_EVENT_STREAM_TRANSPORT
        );
        assert_eq!(
            payload["receipt"]["fallback_transport"],
            DX_HOT_RELOAD_TRANSPORT
        );
        assert_eq!(payload["capabilities"]["partial_module_updates"], false);
        assert_eq!(payload["event_stream"]["initial"], false);
        assert_eq!(payload["receipt"]["event_stream"]["initial"], false);
    }

    #[test]
    fn event_stream_initial_payload_marks_sync_without_module_hmr() {
        let payload = dx_hot_reload_event_stream_initial_payload(
            true,
            "5-source-token".to_string(),
            5,
            "route:/settings",
        );

        assert_eq!(payload["event_stream_initial"], true);
        assert_eq!(payload["event_stream"]["initial"], true);
        assert_eq!(payload["receipt"]["event_stream"]["initial"], true);
        assert_eq!(payload["transport"], DX_HOT_RELOAD_EVENT_STREAM_TRANSPORT);
        assert_eq!(payload["fallback_transport"], DX_HOT_RELOAD_TRANSPORT);
        assert_eq!(payload["capabilities"]["partial_module_updates"], false);
    }

    #[test]
    fn issue_payload_reports_diagnostics_without_claiming_module_hmr() {
        let issue = DxHotReloadIssue::error(
            "dx::build::syntax_error",
            "Unexpected token in route source",
        )
        .with_source_location("app/dashboard/page.tsx", 4, 17)
        .with_code_frame("> 4 |   return <main>\n    |                 ^")
        .with_next_action("Fix the route source and save the file.");

        let payload = dx_hot_reload_issue_payload(
            true,
            "6-source-token".to_string(),
            6,
            "route:/dashboard",
            &[issue],
        );

        assert_eq!(payload["transport"], DX_HOT_RELOAD_EVENT_STREAM_TRANSPORT);
        assert_eq!(payload["fallback_transport"], DX_HOT_RELOAD_TRANSPORT);
        assert_eq!(
            payload["instruction"]["type"],
            DX_HOT_RELOAD_ISSUE_INSTRUCTION
        );
        assert_eq!(payload["instruction"]["mode"], DX_HOT_RELOAD_ISSUE_MODE);
        assert_eq!(payload["event_stream"]["issue_stream"], true);
        assert_eq!(
            payload["event_stream"]["issue_receipt_schema"],
            DX_HOT_RELOAD_ISSUE_RECEIPT_SCHEMA
        );
        assert_eq!(payload["capabilities"]["issue_stream"], true);
        assert_eq!(payload["capabilities"]["partial_module_updates"], false);
        assert_eq!(
            payload["issue_receipt"]["schema"],
            DX_HOT_RELOAD_ISSUE_RECEIPT_SCHEMA
        );
        assert_eq!(payload["issue_receipt"]["source_owned_contract"], true);
        assert_eq!(payload["issue_receipt"]["format"], 1);
        assert_eq!(payload["issue_receipt"]["issue_count"], 1);
        assert_eq!(
            payload["issue_receipt"]["issues"][0]["file"],
            "app/dashboard/page.tsx"
        );
        assert_eq!(payload["issue_receipt"]["issues"][0]["line"], 4);
        assert_eq!(payload["issue_receipt"]["issues"][0]["column"], 17);
        assert_eq!(
            payload["issue_receipt"]["issues"][0]["code_frame"],
            "> 4 |   return <main>\n    |                 ^"
        );
        assert_eq!(
            payload["issue_receipt"]["issues"][0]["next_action"],
            "Fix the route source and save the file."
        );
        assert_eq!(
            payload["receipt"]["issue_stream"]["schema"],
            DX_HOT_RELOAD_ISSUE_RECEIPT_SCHEMA
        );
        assert_eq!(
            payload["receipt"]["boundaries"]["streaming"],
            DX_HOT_RELOAD_STREAMING_BOUNDARY
        );
        assert_eq!(
            payload["boundaries"]["node_runtime"],
            DX_HOT_RELOAD_NODE_RUNTIME_BOUNDARY
        );
        assert!(!payload.to_string().contains(".v1"));
        assert!(!payload.to_string().contains("turbopack"));
        assert!(!payload.to_string().contains("node_modules"));
    }

    #[test]
    fn issue_recovery_payload_clears_diagnostics_without_reloading_module_hmr() {
        let payload = dx_hot_reload_issue_recovery_payload(
            true,
            "7-source-token".to_string(),
            7,
            "route:/dashboard",
        );

        assert_eq!(
            payload["instruction"]["type"],
            DX_HOT_RELOAD_CLEAR_ISSUE_INSTRUCTION
        );
        assert_eq!(
            payload["instruction"]["mode"],
            DX_HOT_RELOAD_CLEAR_ISSUE_MODE
        );
        assert_eq!(payload["event_stream"]["issue_stream"], true);
        assert_eq!(payload["event_stream"]["issue_recovered"], true);
        assert_eq!(payload["issue_recovery"]["issue_count"], 0);
        assert_eq!(payload["issue_recovery"]["recovered"], true);
        assert_eq!(payload["issue_recovery"]["source_owned_contract"], true);
        assert_eq!(payload["issues"].as_array().map(Vec::len), Some(0));
        assert_eq!(payload["capabilities"]["partial_module_updates"], false);
        assert_eq!(
            payload["receipt"]["issue_stream"]["schema"],
            DX_HOT_RELOAD_ISSUE_RECEIPT_SCHEMA
        );
        assert_eq!(payload["receipt"]["issue_stream"]["recovered"], true);
        assert!(payload["issue_receipt"].is_null());
        assert!(!payload.to_string().contains(".v1"));
        assert!(!payload.to_string().contains("turbopack"));
        assert!(!payload.to_string().contains("node_modules"));
    }

    #[test]
    fn disabled_payload_does_not_claim_active_reload() {
        let payload =
            dx_hot_reload_version_payload(false, "ignored-token".to_string(), 0, "route:/");

        assert_eq!(payload["token"], "disabled");
        assert_eq!(payload["instruction"]["type"], "disabled");
        assert_eq!(payload["instruction"]["mode"], "none");
        assert_eq!(payload["receipt"]["hot_reload_enabled"], false);
        assert_eq!(payload["receipt"]["instruction"]["type"], "disabled");
        assert_eq!(
            payload["receipt"]["boundaries"]["streaming"],
            DX_HOT_RELOAD_STREAMING_BOUNDARY
        );
        assert_eq!(
            payload["receipt"]["boundaries"]["node_runtime"],
            DX_HOT_RELOAD_NODE_RUNTIME_BOUNDARY
        );
    }
}
