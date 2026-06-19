pub(super) use crate::hot_reload_protocol::{
    DX_HOT_RELOAD_ASSET_REFRESH_INSTRUCTION, DX_HOT_RELOAD_ASSET_REFRESH_MODE,
    DX_HOT_RELOAD_CLEAR_ISSUE_INSTRUCTION, DX_HOT_RELOAD_CLEAR_ISSUE_MODE,
    DX_HOT_RELOAD_EVENT_NAME, DX_HOT_RELOAD_EVENT_RETRY_MS, DX_HOT_RELOAD_EVENT_STREAM_ENDPOINT,
    DX_HOT_RELOAD_EVENT_STREAM_TRANSPORT, DX_HOT_RELOAD_FULL_PAGE_MODE,
    DX_HOT_RELOAD_ISSUE_INSTRUCTION, DX_HOT_RELOAD_ISSUE_MODE, DX_HOT_RELOAD_ISSUE_RECEIPT_FORMAT,
    DX_HOT_RELOAD_ISSUE_RECEIPT_SCHEMA, DX_HOT_RELOAD_NODE_MODULES_REQUIRED,
    DX_HOT_RELOAD_NODE_RUNTIME_BOUNDARY, DX_HOT_RELOAD_POLL_RECEIPT_FORMAT,
    DX_HOT_RELOAD_POLL_RECEIPT_SCHEMA, DX_HOT_RELOAD_PROTOCOL, DX_HOT_RELOAD_PROTOCOL_FORMAT,
    DX_HOT_RELOAD_RESOURCE_MARKER, DX_HOT_RELOAD_RESOURCE_QUERY_PARAM,
    DX_HOT_RELOAD_RESTART_INSTRUCTION, DX_HOT_RELOAD_SOURCE, DX_HOT_RELOAD_SOURCE_OWNED_CONTRACT,
    DX_HOT_RELOAD_STREAMING_BOUNDARY, DX_HOT_RELOAD_STYLE_REFRESH_INSTRUCTION,
    DX_HOT_RELOAD_STYLE_REFRESH_MODE, DX_HOT_RELOAD_TRANSPORT, DX_HOT_RELOAD_VERSION_ENDPOINT,
};
use serde_json::{Value, json};

pub(super) const DX_STUDIO_HOT_RELOAD_ROUTE_READ_MODEL_SCHEMA: &str =
    "dx.studio.hot_reload_route_read_model";
const DX_STUDIO_HOT_RELOAD_ROUTE_READ_MODEL_FORMAT: u64 = 1;
const DX_HOT_RELOAD_EVENT_STREAM_STATUS: &str = "active-dx-sse";
const DX_HOT_RELOAD_LEGACY_BROADCAST_HELPER_SCHEMA: &str =
    "dx.dev.hot_reload.legacy_broadcast_helper";
const DX_HOT_RELOAD_LEGACY_BROADCAST_HELPER_FORMAT: u64 = 1;
const DX_HOT_RELOAD_LEGACY_BROADCAST_HELPER_MODULE: &str = "dx-www/src/dev/hot_reload.rs";
const DX_HOT_RELOAD_ACTIVE_CLIENT_MODULE: &str = "dx-www/src/cli/dev_hot_reload_client.rs";

pub(super) fn studio_hot_reload_contract() -> Value {
    json!({
        "enabled_by_default": true,
        "protocol": DX_HOT_RELOAD_PROTOCOL,
        "protocol_format": DX_HOT_RELOAD_PROTOCOL_FORMAT,
        "transport": DX_HOT_RELOAD_TRANSPORT,
        "version_endpoint": DX_HOT_RELOAD_VERSION_ENDPOINT,
        "resource_query_param": DX_HOT_RELOAD_RESOURCE_QUERY_PARAM,
        "resource_marker": DX_HOT_RELOAD_RESOURCE_MARKER,
        "event_stream": event_stream_contract(),
        "hot_update": hot_update_contract(),
        "route_scope": "route-source-and-forge-package-slices",
        "poll_receipt": {
            "schema": DX_HOT_RELOAD_POLL_RECEIPT_SCHEMA,
            "format": DX_HOT_RELOAD_POLL_RECEIPT_FORMAT,
            "source": DX_HOT_RELOAD_SOURCE,
            "source_owned_contract": DX_HOT_RELOAD_SOURCE_OWNED_CONTRACT,
            "node_modules_required": DX_HOT_RELOAD_NODE_MODULES_REQUIRED,
            "endpoint": DX_HOT_RELOAD_VERSION_ENDPOINT,
            "resource_field": DX_HOT_RELOAD_RESOURCE_QUERY_PARAM,
            "instruction_field": "instruction",
            "boundary_field": "receipt.boundaries",
            "partial_module_updates": false,
            "streaming": DX_HOT_RELOAD_STREAMING_BOUNDARY,
            "node_runtime": DX_HOT_RELOAD_NODE_RUNTIME_BOUNDARY,
        },
        "legacy_broadcast_helper": legacy_broadcast_helper_contract(),
    })
}

pub(super) fn studio_route_hot_reload_read_model(route: &str) -> Value {
    json!({
        "schema": DX_STUDIO_HOT_RELOAD_ROUTE_READ_MODEL_SCHEMA,
        "format": DX_STUDIO_HOT_RELOAD_ROUTE_READ_MODEL_FORMAT,
        "protocol": DX_HOT_RELOAD_PROTOCOL,
        "protocol_format": DX_HOT_RELOAD_PROTOCOL_FORMAT,
        "transport": DX_HOT_RELOAD_TRANSPORT,
        "target": route_hot_reload_target(route),
        "version_endpoint": DX_HOT_RELOAD_VERSION_ENDPOINT,
        "resource_query_param": DX_HOT_RELOAD_RESOURCE_QUERY_PARAM,
        "resource_marker": DX_HOT_RELOAD_RESOURCE_MARKER,
        "poll_receipt_schema": DX_HOT_RELOAD_POLL_RECEIPT_SCHEMA,
        "poll_receipt_format": DX_HOT_RELOAD_POLL_RECEIPT_FORMAT,
        "source_owned_contract": DX_HOT_RELOAD_SOURCE_OWNED_CONTRACT,
        "node_modules_required": DX_HOT_RELOAD_NODE_MODULES_REQUIRED,
        "boundary_field": "receipt.boundaries",
        "partial_module_updates": false,
        "streaming": DX_HOT_RELOAD_STREAMING_BOUNDARY,
        "node_runtime": DX_HOT_RELOAD_NODE_RUNTIME_BOUNDARY,
        "event_stream": event_stream_contract(),
        "hot_update": hot_update_contract(),
        "legacy_broadcast_helper_active": false,
        "active_client": DX_HOT_RELOAD_ACTIVE_CLIENT_MODULE,
        "legacy_broadcast_helper": legacy_broadcast_helper_contract(),
    })
}

fn hot_update_contract() -> Value {
    json!({
        "css": {
            "active": true,
            "instruction": DX_HOT_RELOAD_STYLE_REFRESH_INSTRUCTION,
            "mode": DX_HOT_RELOAD_STYLE_REFRESH_MODE,
            "mechanism": "stylesheet-link-cache-bust",
            "client_module": DX_HOT_RELOAD_ACTIVE_CLIENT_MODULE,
        },
        "asset_hot_update": asset_hot_update_contract(),
        "route": {
            "active": true,
            "instruction": DX_HOT_RELOAD_RESTART_INSTRUCTION,
            "mode": DX_HOT_RELOAD_FULL_PAGE_MODE,
        },
        "partial_module_updates": false,
        "streaming": DX_HOT_RELOAD_STREAMING_BOUNDARY,
    })
}

fn asset_hot_update_contract() -> Value {
    json!({
        "active": true,
        "instruction": DX_HOT_RELOAD_ASSET_REFRESH_INSTRUCTION,
        "mode": DX_HOT_RELOAD_ASSET_REFRESH_MODE,
        "mechanism": "dom-asset-url-cache-bust",
        "client_module": DX_HOT_RELOAD_ACTIVE_CLIENT_MODULE,
    })
}

fn event_stream_contract() -> Value {
    json!({
        "endpoint": DX_HOT_RELOAD_EVENT_STREAM_ENDPOINT,
        "event": DX_HOT_RELOAD_EVENT_NAME,
        "retry_ms": DX_HOT_RELOAD_EVENT_RETRY_MS,
        "status": DX_HOT_RELOAD_EVENT_STREAM_STATUS,
        "transport": DX_HOT_RELOAD_EVENT_STREAM_TRANSPORT,
        "fallback_transport": DX_HOT_RELOAD_TRANSPORT,
        "public_protocol": true,
        "turbopack_protocol": false,
        "active_transport": DX_HOT_RELOAD_EVENT_STREAM_TRANSPORT,
        "active_client_module": DX_HOT_RELOAD_ACTIVE_CLIENT_MODULE,
        "issue_stream": issue_stream_contract(),
    })
}

fn issue_stream_contract() -> Value {
    json!({
        "active": true,
        "schema": DX_HOT_RELOAD_ISSUE_RECEIPT_SCHEMA,
        "format": DX_HOT_RELOAD_ISSUE_RECEIPT_FORMAT,
        "instruction": DX_HOT_RELOAD_ISSUE_INSTRUCTION,
        "mode": DX_HOT_RELOAD_ISSUE_MODE,
        "recovery_instruction": DX_HOT_RELOAD_CLEAR_ISSUE_INSTRUCTION,
        "recovery_mode": DX_HOT_RELOAD_CLEAR_ISSUE_MODE,
        "transport": DX_HOT_RELOAD_EVENT_STREAM_TRANSPORT,
        "fallback_transport": DX_HOT_RELOAD_TRANSPORT,
        "client_module": DX_HOT_RELOAD_ACTIVE_CLIENT_MODULE,
        "source": DX_HOT_RELOAD_SOURCE,
        "source_owned_contract": DX_HOT_RELOAD_SOURCE_OWNED_CONTRACT,
        "node_modules_required": DX_HOT_RELOAD_NODE_MODULES_REQUIRED,
        "partial_module_updates": false,
        "turbopack_protocol": false,
    })
}

fn legacy_broadcast_helper_contract() -> Value {
    json!({
        "schema": DX_HOT_RELOAD_LEGACY_BROADCAST_HELPER_SCHEMA,
        "format": DX_HOT_RELOAD_LEGACY_BROADCAST_HELPER_FORMAT,
        "module": DX_HOT_RELOAD_LEGACY_BROADCAST_HELPER_MODULE,
        "active": false,
        "public_protocol": false,
        "status": "legacy-internal-helper",
        "active_transport": DX_HOT_RELOAD_TRANSPORT,
        "active_endpoint": DX_HOT_RELOAD_VERSION_ENDPOINT,
        "active_client_module": DX_HOT_RELOAD_ACTIVE_CLIENT_MODULE,
    })
}

pub(super) fn route_hot_reload_target(route: &str) -> String {
    let route = route.trim();
    if route.is_empty() || !route.starts_with('/') || route.chars().any(char::is_control) {
        return "route:/".to_string();
    }

    format!("route:{route}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn route_hot_reload_read_model_keeps_protocol_dx_owned() {
        let model = studio_route_hot_reload_read_model("/");

        assert_eq!(
            model["schema"],
            DX_STUDIO_HOT_RELOAD_ROUTE_READ_MODEL_SCHEMA
        );
        assert_eq!(model["protocol"], DX_HOT_RELOAD_PROTOCOL);
        assert_eq!(model["target"], "route:/");
        assert_eq!(
            model["poll_receipt_schema"],
            DX_HOT_RELOAD_POLL_RECEIPT_SCHEMA
        );
        assert_eq!(model["partial_module_updates"], false);
        assert_eq!(model["source_owned_contract"], true);
        assert_eq!(model["node_modules_required"], false);
        assert_eq!(model["node_runtime"], DX_HOT_RELOAD_NODE_RUNTIME_BOUNDARY);
        assert_eq!(
            model["event_stream"]["endpoint"],
            DX_HOT_RELOAD_EVENT_STREAM_ENDPOINT
        );
        assert_eq!(model["event_stream"]["event"], DX_HOT_RELOAD_EVENT_NAME);
        assert_eq!(
            model["event_stream"]["status"],
            DX_HOT_RELOAD_EVENT_STREAM_STATUS
        );
        assert_eq!(
            model["event_stream"]["transport"],
            DX_HOT_RELOAD_EVENT_STREAM_TRANSPORT
        );
        assert_eq!(
            model["event_stream"]["retry_ms"],
            DX_HOT_RELOAD_EVENT_RETRY_MS
        );
        assert_eq!(
            model["event_stream"]["fallback_transport"],
            DX_HOT_RELOAD_TRANSPORT
        );
        assert_eq!(model["event_stream"]["public_protocol"], true);
        assert_eq!(model["event_stream"]["turbopack_protocol"], false);
        assert_eq!(model["event_stream"]["issue_stream"]["active"], true);
        assert_eq!(
            model["event_stream"]["issue_stream"]["source_owned_contract"],
            true
        );
        assert_eq!(
            model["event_stream"]["issue_stream"]["node_modules_required"],
            false
        );
        assert_eq!(
            model["event_stream"]["issue_stream"]["schema"],
            DX_HOT_RELOAD_ISSUE_RECEIPT_SCHEMA
        );
        assert_eq!(model["event_stream"]["issue_stream"]["format"], 1);
        assert_eq!(
            model["event_stream"]["issue_stream"]["instruction"],
            DX_HOT_RELOAD_ISSUE_INSTRUCTION
        );
        assert_eq!(
            model["event_stream"]["issue_stream"]["recovery_instruction"],
            DX_HOT_RELOAD_CLEAR_ISSUE_INSTRUCTION
        );
        assert_eq!(
            model["hot_update"]["css"]["instruction"],
            DX_HOT_RELOAD_STYLE_REFRESH_INSTRUCTION
        );
        assert_eq!(model["hot_update"]["css"]["active"], true);
        assert_eq!(
            model["hot_update"]["asset_hot_update"]["instruction"],
            DX_HOT_RELOAD_ASSET_REFRESH_INSTRUCTION
        );
        assert_eq!(model["hot_update"]["asset_hot_update"]["active"], true);
        assert_eq!(model["hot_update"]["partial_module_updates"], false);
        assert_eq!(model["legacy_broadcast_helper_active"], false);
        assert_eq!(model["active_client"], DX_HOT_RELOAD_ACTIVE_CLIENT_MODULE);
        assert_eq!(
            model["legacy_broadcast_helper"]["schema"],
            DX_HOT_RELOAD_LEGACY_BROADCAST_HELPER_SCHEMA
        );
        assert_eq!(model["legacy_broadcast_helper"]["active"], false);
    }

    #[test]
    fn studio_hot_reload_manifest_uses_stable_schema_names_with_numeric_formats() {
        let contract = studio_hot_reload_contract();
        let model = studio_route_hot_reload_read_model("/");

        assert_eq!(
            DX_STUDIO_HOT_RELOAD_ROUTE_READ_MODEL_SCHEMA,
            "dx.studio.hot_reload_route_read_model"
        );
        assert!(!DX_STUDIO_HOT_RELOAD_ROUTE_READ_MODEL_SCHEMA.contains(".v1"));
        assert_eq!(
            model["schema"],
            DX_STUDIO_HOT_RELOAD_ROUTE_READ_MODEL_SCHEMA
        );
        assert_eq!(model["format"], 1);
        assert_eq!(model["poll_receipt_format"], 1);
        assert_eq!(contract["poll_receipt"]["format"], 1);
        assert_eq!(contract["poll_receipt"]["source_owned_contract"], true);
        assert_eq!(contract["poll_receipt"]["node_modules_required"], false);
        assert_eq!(
            contract["legacy_broadcast_helper"]["schema"],
            "dx.dev.hot_reload.legacy_broadcast_helper"
        );
        assert_eq!(contract["legacy_broadcast_helper"]["format"], 1);
        assert_eq!(
            contract["event_stream"]["issue_stream"]["schema"],
            DX_HOT_RELOAD_ISSUE_RECEIPT_SCHEMA
        );
        assert_eq!(contract["event_stream"]["issue_stream"]["format"], 1);
        assert!(!contract.to_string().contains(".v1"));
        assert!(!model.to_string().contains(".v1"));
    }

    #[test]
    fn invalid_route_falls_back_to_root_target() {
        assert_eq!(route_hot_reload_target("dashboard"), "route:/");
        assert_eq!(route_hot_reload_target(""), "route:/");
    }
}
