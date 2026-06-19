use std::collections::BTreeMap;

use serde_json::{Value, json};

pub(super) const APP_ROUTE_HANDLER_RECEIPT_SCHEMA: &str = "dx.next.appRouteHandlerReceipt";
const APP_ROUTE_HANDLER_RECEIPT_FORMAT: u8 = 1;

pub(super) struct DxAppRouteHandlerReceiptInput<'a> {
    pub source_path: &'a str,
    pub method: &'a str,
    pub request_path: &'a str,
    pub route_params: &'a BTreeMap<String, String>,
    pub search_params: &'a BTreeMap<String, String>,
    pub status: u16,
    pub content_type: &'a str,
    pub response_headers: &'a BTreeMap<String, String>,
    pub execution_model: &'a str,
    pub lifecycle_scripts_executed: bool,
    pub node_modules_present: bool,
}

pub(super) fn build_app_route_handler_receipt(input: DxAppRouteHandlerReceiptInput<'_>) -> Value {
    json!({
        "schema": APP_ROUTE_HANDLER_RECEIPT_SCHEMA,
        "format": APP_ROUTE_HANDLER_RECEIPT_FORMAT,
        "source_path": input.source_path,
        "method": input.method,
        "request_path": input.request_path,
        "route_params": input.route_params,
        "search_params": input.search_params,
        "route_param_count": input.route_params.len(),
        "search_param_count": input.search_params.len(),
        "response": {
            "status": input.status,
            "content_type": input.content_type,
            "header_count": input.response_headers.len(),
        },
        "response_header_count": input.response_headers.len(),
        "execution_model": input.execution_model,
        "lifecycle_scripts_executed": input.lifecycle_scripts_executed,
        "node_modules_required": false,
        "node_modules_present": input.node_modules_present,
        "runtime_boundary": {
            "source_owned": true,
            "external_runtime_required": false,
            "external_runtime_executed": false
        },
        "adapter_boundary": [
            "Does not import Next.js Route Handler runtime.",
            "Does not require React/RSC, Node/NAPI, npm, or node_modules.",
            "Does not claim unbounded NextRequest, streaming bodies, or middleware handoff coverage."
        ],
    })
}

pub(super) fn app_route_handler_receipt_headers(receipt: &Value) -> BTreeMap<String, String> {
    let mut headers = BTreeMap::new();
    headers.insert(
        "x-dx-route-handler-receipt".to_string(),
        APP_ROUTE_HANDLER_RECEIPT_SCHEMA.to_string(),
    );
    headers.insert(
        "x-dx-route-handler-request-maps".to_string(),
        format!(
            "params={};searchParams={}",
            receipt_u64(receipt, "route_param_count"),
            receipt_u64(receipt, "search_param_count")
        ),
    );
    headers.insert(
        "x-dx-route-handler-source".to_string(),
        receipt_string(receipt, "source_path"),
    );
    headers.insert(
        "x-dx-node-modules-required".to_string(),
        "false".to_string(),
    );
    headers.insert(
        "x-dx-route-handler-source-owned".to_string(),
        receipt_runtime_boundary(receipt, "source_owned").to_string(),
    );
    headers.insert(
        "x-dx-external-runtime-required".to_string(),
        receipt_runtime_boundary(receipt, "external_runtime_required").to_string(),
    );
    headers.insert(
        "x-dx-external-runtime-executed".to_string(),
        receipt_runtime_boundary(receipt, "external_runtime_executed").to_string(),
    );
    headers
}

fn receipt_string(receipt: &Value, key: &str) -> String {
    receipt
        .get(key)
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string()
}

fn receipt_u64(receipt: &Value, key: &str) -> u64 {
    receipt.get(key).and_then(Value::as_u64).unwrap_or(0)
}

fn receipt_runtime_boundary(receipt: &Value, key: &str) -> bool {
    receipt
        .get("runtime_boundary")
        .and_then(|boundary| boundary.get(key))
        .and_then(Value::as_bool)
        .unwrap_or(false)
}
