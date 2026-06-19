use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use dx_compiler::delivery::{
    DxReactRouteHandlerRequest, DxReactServerSource, DxReactServerSourceKind,
    execute_react_route_handler,
};
use serde_json::{Value, json};

use crate::error::{DxError, DxResult};

use super::graph::{SourceBuildRouteHandler, read_file, write_file};

const ROUTE_HANDLER_RECEIPTS_JSON: &str = "route-handler-receipts.json";

const ROUTE_HANDLER_BUILD_RECEIPTS_SCHEMA: &str = "dx.next.appRouteHandlerBuildReceipts";
const ROUTE_HANDLER_RECEIPT_SCHEMA: &str = "dx.next.appRouteHandlerReceipt";

pub(super) fn write_route_handler_receipts(
    project_root: &Path,
    output_dir: &Path,
    route_handlers: &[SourceBuildRouteHandler],
) -> DxResult<PathBuf> {
    let node_modules_present = project_root.join("node_modules").exists();
    let mut receipts = Vec::new();
    let mut skipped = Vec::new();

    for handler in route_handlers {
        let source = route_handler_source(project_root, handler)?;
        for method in &handler.methods {
            if is_safe_build_method(method) {
                receipts.push(route_handler_receipt_for_build(
                    &source,
                    handler,
                    method,
                    node_modules_present,
                ));
            } else {
                skipped.push(json!({
                    "source_path": handler.path,
                    "method": method,
                    "request_path": handler.route,
                    "reason": "build receipts execute only safe requestless GET/HEAD route handlers"
                }));
            }
        }
    }

    let path = output_dir.join(ROUTE_HANDLER_RECEIPTS_JSON);
    let contract = json!({
        "schema": ROUTE_HANDLER_BUILD_RECEIPTS_SCHEMA,
        "format": 1,
        "receipt_count": receipts.len(),
        "skipped_count": skipped.len(),
        "node_modules_required": false,
        "node_modules_present": node_modules_present,
        "lifecycle_scripts_executed": false,
        "receipts": receipts,
        "skipped": skipped
    });
    let serialized = serde_json::to_string_pretty(&contract).map_err(|error| {
        DxError::ConfigValidationError {
            message: format!("Failed to serialize route handler receipts: {error}"),
            field: Some("route-handler-receipts".to_string()),
        }
    })?;
    write_file(&path, serialized.as_bytes())?;
    Ok(path)
}

fn route_handler_source(
    project_root: &Path,
    handler: &SourceBuildRouteHandler,
) -> DxResult<DxReactServerSource> {
    let bytes = read_file(&project_root.join(&handler.path))?;
    let source = String::from_utf8(bytes).map_err(|error| DxError::CompilationError {
        message: error.to_string(),
        file: project_root.join(&handler.path),
        src: None,
        span: None,
    })?;
    Ok(DxReactServerSource {
        kind: DxReactServerSourceKind::RouteHandler,
        source_path: handler.path.clone(),
        source,
    })
}

fn route_handler_receipt_for_build(
    source: &DxReactServerSource,
    handler: &SourceBuildRouteHandler,
    method: &str,
    node_modules_present: bool,
) -> Value {
    let request = DxReactRouteHandlerRequest {
        method: method.to_string(),
        path: handler.route.clone(),
        headers: BTreeMap::new(),
        body: Value::Null,
        route_params: BTreeMap::new(),
        search_params: BTreeMap::new(),
        runtime_env: BTreeMap::new(),
    };
    let (status, content_type, mut headers, execution_model, lifecycle_scripts_executed, error) =
        match execute_react_route_handler(source, request) {
            Ok(response) => (
                response.status,
                response.content_type,
                response.headers,
                response.execution_model,
                response.lifecycle_scripts_executed,
                None,
            ),
            Err(error) => (
                501,
                "application/json; charset=utf-8".to_string(),
                BTreeMap::new(),
                "adapter-boundary".to_string(),
                false,
                Some(error),
            ),
        };
    headers
        .entry("content-type".to_string())
        .or_insert_with(|| content_type.clone());
    let receipt_execution_model =
        if error.is_none() && execution_model == "source-owned-safe-interpreter" {
            "source-owned-route-handler-contract"
        } else {
            execution_model.as_str()
        };
    let mut receipt = build_route_handler_receipt(RouteHandlerReceiptInput {
        source_path: &handler.path,
        method,
        request_path: &handler.route,
        status,
        content_type: &content_type,
        response_headers: &headers,
        execution_model: receipt_execution_model,
        lifecycle_scripts_executed,
        node_modules_present,
    });
    if let Some(error) = error {
        receipt["status"] = json!("adapter-boundary");
        receipt["adapter_boundary_error"] = json!(error);
    }
    receipt
}

struct RouteHandlerReceiptInput<'a> {
    source_path: &'a str,
    method: &'a str,
    request_path: &'a str,
    status: u16,
    content_type: &'a str,
    response_headers: &'a BTreeMap<String, String>,
    execution_model: &'a str,
    lifecycle_scripts_executed: bool,
    node_modules_present: bool,
}

fn build_route_handler_receipt(input: RouteHandlerReceiptInput<'_>) -> Value {
    json!({
        "schema": ROUTE_HANDLER_RECEIPT_SCHEMA,
        "format": 1,
        "source_path": input.source_path,
        "method": input.method,
        "request_path": input.request_path,
        "route_params": {},
        "search_params": {},
        "route_param_count": 0,
        "search_param_count": 0,
        "response": {
            "status": input.status,
            "content_type": input.content_type,
            "header_count": input.response_headers.len()
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
        ]
    })
}

fn is_safe_build_method(method: &str) -> bool {
    matches!(method, "GET" | "HEAD")
}
