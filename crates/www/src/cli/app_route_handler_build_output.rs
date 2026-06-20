use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

use dx_compiler::delivery::{
    DxReactRouteHandlerRequest, DxReactServerSource, DxReactServerSourceKind,
    compile_react_server_contracts, execute_react_route_handler,
};
use serde_json::{Value, json};

use crate::error::{DxError, DxResult};

use super::app_route_handler_receipt::{
    DxAppRouteHandlerReceiptInput, build_app_route_handler_receipt,
};

pub(super) const APP_ROUTE_HANDLER_RECEIPTS_JSON: &str = ".dx/build-cache/route-handler-receipts.json";
const ROUTE_HANDLER_METHOD_ORDER: &[&str] =
    &["GET", "HEAD", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"];

pub(super) fn app_route_handler_deploy_metadata(
    server_sources: &[DxReactServerSource],
) -> Vec<Value> {
    let mut routes = route_handler_contract_rows(server_sources)
        .into_iter()
        .map(|row| {
            let safe_methods = row
                .allowed_methods
                .iter()
                .filter(|method| is_safe_build_method(method))
                .cloned()
                .collect::<Vec<_>>();
            let skipped_methods = row
                .allowed_methods
                .iter()
                .filter(|method| !is_safe_build_method(method))
                .cloned()
                .collect::<Vec<_>>();
            json!({
                "path": row.endpoint,
                "source_path": row.source_path,
                "methods": row.allowed_methods,
                "declared_methods": row.declared_methods,
                "implicit_methods": row.implicit_methods,
                "safe_build_methods": safe_methods,
                "skipped_build_methods": skipped_methods,
                "build_execution": if safe_methods.is_empty() {
                    "skipped-build-execution"
                } else {
                    "safe-requestless-receipt"
                },
                "receipt": APP_ROUTE_HANDLER_RECEIPTS_JSON,
                "node_modules_required": false,
                "runtime_boundary": {
                    "source_owned": true,
                    "external_runtime_required": false,
                    "external_runtime_executed": false
                },
            })
        })
        .collect::<Vec<_>>();
    routes.sort_by(|left, right| left["path"].as_str().cmp(&right["path"].as_str()));
    routes
}

pub(super) fn app_route_handler_health_checks(
    server_sources: &[DxReactServerSource],
) -> Vec<Value> {
    let mut checks = Vec::new();
    for row in route_handler_contract_rows(server_sources) {
        for method in row
            .allowed_methods
            .iter()
            .filter(|method| is_safe_build_method(method))
        {
            let source_method = route_handler_source_method(&row.declared_methods, method);
            checks.push(json!({
                "path": row.endpoint.clone(),
                "method": method,
                "source_method": source_method,
                "implicit_method": row.implicit_methods.contains(method),
                "allowed_methods": row.allowed_methods.clone(),
                "declared_methods": row.declared_methods.clone(),
                "source_path": row.source_path.clone(),
                "receipt": APP_ROUTE_HANDLER_RECEIPTS_JSON,
            }));
        }
    }
    checks.sort_by(|left, right| {
        left["path"]
            .as_str()
            .cmp(&right["path"].as_str())
            .then(left["method"].as_str().cmp(&right["method"].as_str()))
    });
    checks
}

pub(super) fn write_app_route_handler_receipts(
    output_dir: &Path,
    server_sources: &[DxReactServerSource],
    node_modules_present: bool,
) -> DxResult<usize> {
    let route_sources = server_sources
        .iter()
        .filter(|source| source.kind == DxReactServerSourceKind::RouteHandler)
        .collect::<Vec<_>>();
    if route_sources.is_empty() {
        return Ok(0);
    }

    let mut receipts = Vec::new();
    let mut skipped = Vec::new();

    for row in route_handler_contract_rows(server_sources) {
        let Some(source) = route_sources
            .iter()
            .find(|source| source.source_path == row.source_path)
        else {
            continue;
        };

        for method in row.allowed_methods.iter() {
            if !matches!(method.as_str(), "GET" | "HEAD") {
                skipped.push(json!({
                    "source_path": row.source_path.clone(),
                    "method": method,
                    "request_path": row.endpoint.clone(),
                    "declared_methods": row.declared_methods.clone(),
                    "implicit_method": row.implicit_methods.contains(method),
                    "reason": "build receipts execute only safe requestless GET/HEAD route handlers"
                }));
                continue;
            }

            receipts.push(route_handler_receipt_for_build(
                source,
                method,
                &row.endpoint,
                node_modules_present,
            ));
        }
    }

    let receipt_count = receipts.len();
    let contract = json!({
        "schema": "dx.next.appRouteHandlerBuildReceipts",
        "format": 1,
        "receipt_count": receipt_count,
        "skipped_count": skipped.len(),
        "node_modules_required": false,
        "node_modules_present": node_modules_present,
        "lifecycle_scripts_executed": false,
        "receipts": receipts,
        "skipped": skipped,
    });
    let path = output_dir.join(APP_ROUTE_HANDLER_RECEIPTS_JSON);
    std::fs::write(
        &path,
        serde_json::to_string_pretty(&contract).map_err(|error| {
            DxError::ConfigValidationError {
                message: format!("Failed to serialize route handler receipts: {error}"),
                field: Some("route-handler-receipts".to_string()),
            }
        })?,
    )
    .map_err(|error| DxError::IoError {
        path: Some(path),
        message: error.to_string(),
    })?;

    Ok(receipt_count)
}

fn route_handler_receipt_for_build(
    source: &DxReactServerSource,
    method: &str,
    endpoint: &str,
    node_modules_present: bool,
) -> Value {
    let request = DxReactRouteHandlerRequest {
        method: method.to_string(),
        path: endpoint.to_string(),
        headers: BTreeMap::new(),
        body: Value::Null,
        route_params: BTreeMap::new(),
        search_params: BTreeMap::new(),
        runtime_env: BTreeMap::new(),
    };

    let (
        status,
        content_type,
        mut response_headers,
        execution_model,
        lifecycle_scripts_executed,
        error,
    ) = match execute_react_route_handler(source, request) {
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
    response_headers
        .entry("content-type".to_string())
        .or_insert_with(|| content_type.clone());
    let receipt_execution_model =
        if error.is_none() && execution_model == "source-owned-safe-interpreter" {
            "source-owned-route-handler-contract"
        } else {
            execution_model.as_str()
        };

    let mut receipt = build_app_route_handler_receipt(DxAppRouteHandlerReceiptInput {
        source_path: &source.source_path,
        method,
        request_path: endpoint,
        route_params: &BTreeMap::new(),
        search_params: &BTreeMap::new(),
        status,
        content_type: &content_type,
        response_headers: &response_headers,
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

struct RouteHandlerContractRow {
    endpoint: String,
    source_path: String,
    declared_methods: Vec<String>,
    allowed_methods: Vec<String>,
    implicit_methods: Vec<String>,
}

fn route_handler_contract_rows(
    server_sources: &[DxReactServerSource],
) -> Vec<RouteHandlerContractRow> {
    let mut rows = compile_react_server_contracts(server_sources)
        .into_iter()
        .filter(|contract| contract.kind == DxReactServerSourceKind::RouteHandler)
        .filter_map(|contract| {
            let endpoint = contract.endpoint?;
            let declared_methods = contract
                .exports
                .iter()
                .filter_map(|export| export.http_method.clone())
                .collect::<BTreeSet<_>>();
            if declared_methods.is_empty() {
                return None;
            }
            let declared_methods = ordered_route_handler_methods(&declared_methods);
            let allowed_methods = allowed_route_handler_methods(&declared_methods);
            let implicit_methods = allowed_methods
                .iter()
                .filter(|method| !declared_methods.contains(method))
                .cloned()
                .collect::<Vec<_>>();
            Some(RouteHandlerContractRow {
                endpoint,
                source_path: contract.source_path,
                declared_methods,
                allowed_methods,
                implicit_methods,
            })
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.endpoint.cmp(&right.endpoint));
    rows
}

fn is_safe_build_method(method: &str) -> bool {
    matches!(method, "GET" | "HEAD")
}

fn ordered_route_handler_methods(methods: &BTreeSet<String>) -> Vec<String> {
    ROUTE_HANDLER_METHOD_ORDER
        .iter()
        .filter(|method| methods.contains(**method))
        .map(|method| (*method).to_string())
        .collect()
}

fn allowed_route_handler_methods(declared_methods: &[String]) -> Vec<String> {
    let mut methods = Vec::new();
    let has_get = declared_methods.iter().any(|method| method == "GET");
    let has_head = declared_methods.iter().any(|method| method == "HEAD");

    if has_get {
        push_route_handler_method(&mut methods, "GET");
    }
    if has_get || has_head {
        push_route_handler_method(&mut methods, "HEAD");
    }
    for method in ["POST", "PUT", "PATCH", "DELETE"] {
        if declared_methods.iter().any(|declared| declared == method) {
            push_route_handler_method(&mut methods, method);
        }
    }
    if !methods.is_empty() {
        push_route_handler_method(&mut methods, "OPTIONS");
    }

    methods
}

fn push_route_handler_method(methods: &mut Vec<String>, method: &str) {
    if !methods.iter().any(|existing| existing == method) {
        methods.push(method.to_string());
    }
}

fn route_handler_source_method(declared_methods: &[String], request_method: &str) -> String {
    if request_method == "HEAD"
        && !declared_methods.iter().any(|method| method == "HEAD")
        && declared_methods.iter().any(|method| method == "GET")
    {
        "GET".to_string()
    } else {
        request_method.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn get_only_route_handler() -> DxReactServerSource {
        DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/health/route.ts".to_string(),
            source: r#"export function GET() {
  return {
    ok: true,
    method: "GET",
  };
}
"#
            .to_string(),
        }
    }

    #[test]
    fn route_handler_deploy_metadata_exposes_declared_and_implicit_methods() {
        let route = app_route_handler_deploy_metadata(&[get_only_route_handler()])
            .into_iter()
            .find(|route| route["path"] == "/api/health")
            .expect("route metadata");

        assert_eq!(route["declared_methods"], json!(["GET"]));
        assert_eq!(route["methods"], json!(["GET", "HEAD", "OPTIONS"]));
        assert_eq!(route["implicit_methods"], json!(["HEAD", "OPTIONS"]));
        assert_eq!(route["safe_build_methods"], json!(["GET", "HEAD"]));
        assert_eq!(route["skipped_build_methods"], json!(["OPTIONS"]));
    }

    #[test]
    fn route_handler_health_checks_include_head_fallback_source_method() {
        let checks = app_route_handler_health_checks(&[get_only_route_handler()]);
        let head = checks
            .iter()
            .find(|check| check["path"] == "/api/health" && check["method"] == "HEAD")
            .expect("HEAD health check");

        assert_eq!(head["source_method"], "GET");
        assert_eq!(head["implicit_method"], true);
        assert_eq!(head["declared_methods"], json!(["GET"]));
        assert_eq!(head["allowed_methods"], json!(["GET", "HEAD", "OPTIONS"]));
    }

    #[test]
    fn route_handler_build_receipts_include_implicit_head_fallback() {
        let output = tempdir().expect("tempdir");
        let count =
            write_app_route_handler_receipts(output.path(), &[get_only_route_handler()], false)
                .expect("write route handler receipts");
        assert_eq!(count, 2);

        let receipt_path = output.path().join(APP_ROUTE_HANDLER_RECEIPTS_JSON);
        let receipt: Value = serde_json::from_slice(
            &std::fs::read(receipt_path).expect("route handler receipt file"),
        )
        .expect("route handler receipt json");

        assert_eq!(receipt["receipt_count"], 2);
        assert_eq!(receipt["skipped_count"], 1);
        assert!(
            receipt["receipts"]
                .as_array()
                .expect("receipts")
                .iter()
                .any(|receipt| receipt["method"] == "HEAD"
                    && receipt["execution_model"] == "source-owned-route-handler-contract")
        );
        assert!(
            receipt["skipped"]
                .as_array()
                .expect("skipped")
                .iter()
                .any(|skipped| skipped["method"] == "OPTIONS"
                    && skipped["implicit_method"] == true)
        );
    }
}
