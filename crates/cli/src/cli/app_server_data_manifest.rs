use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::build::SourceBuildServerDataRoute;
use crate::error::{DxError, DxResult};
use serde_json::{Value, json};

pub(super) fn collect_app_server_data_manifest(
    project_dir: &Path,
    output_dir: &Path,
    source_build_server_data_routes: &[SourceBuildServerDataRoute],
) -> DxResult<Vec<Value>> {
    let mut routes = collect_source_build_server_data_manifest(
        project_dir,
        output_dir,
        source_build_server_data_routes,
    )?;
    let source_build_route_keys = routes
        .iter()
        .filter_map(server_data_route_source_key)
        .collect::<BTreeSet<_>>();

    let app_output_dir = output_dir.join("app");
    if !app_output_dir.exists() {
        return Ok(sort_server_data_routes(routes));
    }

    for entry in walkdir::WalkDir::new(&app_output_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| {
            entry.file_type().is_file()
                && entry.file_name().to_string_lossy().as_ref() == "server-data.json"
        })
    {
        let path = entry.path();
        let raw = std::fs::read_to_string(path).map_err(|error| DxError::IoError {
            path: Some(path.to_path_buf()),
            message: error.to_string(),
        })?;
        let contract: Value =
            serde_json::from_str(&raw).map_err(|error| DxError::ConfigValidationError {
                message: format!("invalid app-router server-data artifact: {error}"),
                field: Some(relative_build_output_path(output_dir, path)),
            })?;
        let route = server_data_route_manifest_entry(output_dir, path, &contract, None);
        if server_data_route_source_key(&route)
            .as_ref()
            .is_some_and(|key| source_build_route_keys.contains(key))
        {
            continue;
        }
        routes.push(route);
    }

    Ok(sort_server_data_routes(routes))
}

pub(super) fn summarize_app_server_data_manifest_routes(
    routes: &[Value],
    source_build_server_data_routes: &[SourceBuildServerDataRoute],
) -> Value {
    let source_build_server_data_entry_count = source_build_server_data_routes
        .iter()
        .map(|route| route.entry_count)
        .sum::<usize>();
    let missing_source_build_routes =
        missing_source_build_server_data_routes(routes, source_build_server_data_routes);

    json!({
        "source_build_routes": source_build_server_data_routes.len(),
        "manifest_routes": routes.len(),
        "source_build_entries": source_build_server_data_entry_count,
        "manifest_entries": manifest_server_data_entry_count(routes),
        "routes_with_route_params": request_prop_route_count(routes, "route_params"),
        "routes_with_search_params": request_prop_route_count(routes, "search_params"),
        "route_param_keys": request_prop_keys(routes, "route_params"),
        "search_param_keys": request_prop_keys(routes, "search_params"),
        "manifest_includes_source_build_routes": missing_source_build_routes.is_empty(),
        "missing_source_build_routes": missing_source_build_routes,
    })
}

fn collect_source_build_server_data_manifest(
    project_dir: &Path,
    output_dir: &Path,
    source_build_server_data_routes: &[SourceBuildServerDataRoute],
) -> DxResult<Vec<Value>> {
    source_build_server_data_routes
        .iter()
        .map(|source_build_route| {
            let path = project_relative_output_path(project_dir, &source_build_route.output);
            let raw = std::fs::read_to_string(&path).map_err(|error| DxError::IoError {
                path: Some(path.clone()),
                message: error.to_string(),
            })?;
            let contract: Value =
                serde_json::from_str(&raw).map_err(|error| DxError::ConfigValidationError {
                    message: format!("invalid source-build server-data artifact: {error}"),
                    field: Some(source_build_route.output.clone()),
                })?;
            validate_source_build_server_data_contract(source_build_route, &contract)?;
            Ok(server_data_route_manifest_entry(
                output_dir,
                &path,
                &contract,
                Some(source_build_route),
            ))
        })
        .collect()
}

fn validate_source_build_server_data_contract(
    source_build_route: &SourceBuildServerDataRoute,
    contract: &Value,
) -> DxResult<()> {
    let artifact_route = contract.get("route").and_then(Value::as_str);
    if artifact_route != Some(source_build_route.route.as_str()) {
        return Err(source_build_server_data_contract_error(
            &source_build_route.output,
            "source-build server-data artifact route mismatch",
        ));
    }

    let artifact_source_path = contract
        .get("route_source_path")
        .and_then(Value::as_str)
        .map(normalized_manifest_path);
    let expected_source_path = normalized_manifest_path(&source_build_route.source_path);
    if artifact_source_path.as_deref() != Some(expected_source_path.as_str()) {
        return Err(source_build_server_data_contract_error(
            &source_build_route.output,
            "source-build server-data artifact route_source_path mismatch",
        ));
    }

    validate_source_build_server_data_bool(
        &source_build_route.output,
        contract,
        "node_modules_required",
        false,
        "source-build server-data artifact must declare node_modules_required=false",
    )?;
    validate_source_build_server_data_bool(
        &source_build_route.output,
        contract,
        "lifecycle_scripts_executed",
        false,
        "source-build server-data artifact must declare lifecycle_scripts_executed=false",
    )?;
    validate_source_build_server_data_bool(
        &source_build_route.output,
        contract,
        "source_owned_contract",
        true,
        "source-build server-data artifact must declare source_owned_contract=true",
    )?;
    validate_source_build_server_data_bool(
        &source_build_route.output,
        contract,
        "external_runtime_required",
        false,
        "source-build server-data artifact must not require external runtime",
    )?;
    validate_source_build_server_data_bool(
        &source_build_route.output,
        contract,
        "external_runtime_executed",
        false,
        "source-build server-data artifact must not execute external runtime",
    )?;
    validate_source_build_server_data_request(source_build_route, contract)?;
    Ok(())
}

fn validate_source_build_server_data_request(
    source_build_route: &SourceBuildServerDataRoute,
    contract: &Value,
) -> DxResult<()> {
    let expected = serde_json::to_value(&source_build_route.request).map_err(|error| {
        DxError::ConfigValidationError {
            message: error.to_string(),
            field: Some(source_build_route.output.clone()),
        }
    })?;
    if contract.get("request") == Some(&expected) {
        return Ok(());
    }

    Err(source_build_server_data_contract_error(
        &source_build_route.output,
        "source-build server-data artifact request props mismatch",
    ))
}

fn validate_source_build_server_data_bool(
    output: &str,
    contract: &Value,
    field: &str,
    expected: bool,
    message: &str,
) -> DxResult<()> {
    if contract.get(field).and_then(Value::as_bool) == Some(expected) {
        return Ok(());
    }

    Err(source_build_server_data_contract_error(output, message))
}

fn source_build_server_data_contract_error(output: &str, message: &str) -> DxError {
    DxError::ConfigValidationError {
        message: message.to_string(),
        field: Some(output.to_string()),
    }
}

fn missing_source_build_server_data_routes(
    routes: &[Value],
    source_build_server_data_routes: &[SourceBuildServerDataRoute],
) -> Vec<Value> {
    source_build_server_data_routes
        .iter()
        .filter(|source_build_route| {
            !routes
                .iter()
                .any(|route| manifest_route_matches_source_build_route(route, source_build_route))
        })
        .map(|route| {
            json!({
                "route": route.route.as_str(),
                "route_source_path": normalized_manifest_path(&route.source_path),
                "output": normalized_manifest_path(&route.output),
            })
        })
        .collect()
}

fn manifest_route_matches_source_build_route(
    route: &Value,
    source_build_route: &SourceBuildServerDataRoute,
) -> bool {
    let source_build_source_path = normalized_manifest_path(&source_build_route.source_path);
    let source_build_output = normalized_manifest_path(&source_build_route.output);
    route.get("route").and_then(Value::as_str) == Some(source_build_route.route.as_str())
        && manifest_route_source_path(route).as_deref() == Some(source_build_source_path.as_str())
        && route.get("output").and_then(Value::as_str) == Some(source_build_output.as_str())
}

fn manifest_route_source_path(route: &Value) -> Option<String> {
    route
        .get("route_source_path")
        .and_then(Value::as_str)
        .map(normalized_manifest_path)
}

fn manifest_server_data_entry_count(routes: &[Value]) -> usize {
    routes
        .iter()
        .filter_map(|route| route.get("entry_count").and_then(Value::as_u64))
        .map(|count| count as usize)
        .sum()
}

fn request_prop_route_count(routes: &[Value], field: &str) -> usize {
    routes
        .iter()
        .filter(|route| !request_prop_keys_for_route(route, field).is_empty())
        .count()
}

fn request_prop_keys(routes: &[Value], field: &str) -> Vec<String> {
    routes
        .iter()
        .flat_map(|route| request_prop_keys_for_route(route, field))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn request_prop_keys_for_route(route: &Value, field: &str) -> Vec<String> {
    route
        .get("request")
        .or_else(|| route.get("build_time_request_props"))
        .and_then(|request| request.get(field))
        .and_then(Value::as_object)
        .map(|props| props.keys().cloned().collect::<BTreeSet<_>>())
        .unwrap_or_default()
        .into_iter()
        .collect()
}

fn sort_server_data_routes(mut routes: Vec<Value>) -> Vec<Value> {
    routes.sort_by(|left, right| {
        route_sort_key(left)
            .cmp(&route_sort_key(right))
            .then_with(|| output_sort_key(left).cmp(&output_sort_key(right)))
    });
    routes
}

fn server_data_route_manifest_entry(
    output_dir: &Path,
    server_data_path: &Path,
    contract: &Value,
    source_build_route: Option<&SourceBuildServerDataRoute>,
) -> Value {
    let route = contract
        .get("route")
        .and_then(Value::as_str)
        .map(str::to_string)
        .unwrap_or_else(|| route_from_server_data_output(output_dir, server_data_path));
    let output = source_build_route
        .map(|route| normalized_manifest_path(&route.output))
        .unwrap_or_else(|| relative_build_output_path(output_dir, server_data_path));
    let entry_count = contract
        .get("entry_count")
        .and_then(Value::as_u64)
        .or_else(|| {
            contract
                .get("entries")
                .and_then(Value::as_array)
                .map(|entries| entries.len() as u64)
        })
        .unwrap_or(0);

    let mut route_entry = json!({
        "route": route,
        "output": output,
        "route_source_path": normalized_server_data_route_source_path(contract),
        "status": contract.get("status").cloned().unwrap_or_else(|| json!("unknown")),
        "entry_count": entry_count,
        "execution_model": contract
            .get("execution_model")
            .cloned()
            .unwrap_or_else(|| json!("unknown")),
        "node_modules_required": contract
            .get("node_modules_required")
            .and_then(Value::as_bool)
            .unwrap_or(false),
        "lifecycle_scripts_executed": contract
            .get("lifecycle_scripts_executed")
            .and_then(Value::as_bool)
            .unwrap_or(false),
        "source_owned_contract": contract
            .get("source_owned_contract")
            .and_then(Value::as_bool)
            .unwrap_or(true),
        "external_runtime_required": contract
            .get("external_runtime_required")
            .and_then(Value::as_bool)
            .unwrap_or(false),
        "external_runtime_executed": contract
            .get("external_runtime_executed")
            .and_then(Value::as_bool)
            .unwrap_or(false),
    });

    if let Some(object) = route_entry.as_object_mut() {
        for key in [
            "schema",
            "format",
            "revision",
            "request",
            "build_time_request_props",
            "adapter_boundary",
        ] {
            if let Some(value) = contract.get(key) {
                object.insert(key.to_string(), value.clone());
            }
        }
        if let Some(bindings) = server_data_entry_bindings(contract) {
            object.insert("bindings".to_string(), Value::Array(bindings));
        }
        if let Some(source_build_route) = source_build_route {
            let source_build_route = source_build_server_data_route_json(source_build_route);
            object.insert("source_build_route".to_string(), source_build_route);
        }
    }

    route_entry
}

fn source_build_server_data_route_json(route: &SourceBuildServerDataRoute) -> Value {
    json!({
        "route": route.route.as_str(),
        "route_source_path": normalized_manifest_path(&route.source_path),
        "output": normalized_manifest_path(&route.output),
        "status": route.status.as_str(),
        "entry_count": route.entry_count,
        "entry_source_paths": normalized_manifest_paths(&route.entry_source_paths),
        "request": &route.request,
        "execution_model": route.execution_model.as_str(),
        "node_modules_required": route.node_modules_required,
        "lifecycle_scripts_executed": route.lifecycle_scripts_executed,
        "source_owned_contract": route.source_owned_contract,
        "external_runtime_required": route.external_runtime_required,
        "external_runtime_executed": route.external_runtime_executed,
    })
}

fn normalized_server_data_route_source_path(contract: &Value) -> Value {
    contract
        .get("route_source_path")
        .and_then(Value::as_str)
        .map(normalized_manifest_path)
        .map(Value::String)
        .unwrap_or(Value::Null)
}

fn server_data_route_source_key(value: &Value) -> Option<String> {
    let source_path = value.get("route_source_path")?.as_str()?;
    (!source_path.is_empty()).then(|| normalized_manifest_path(source_path))
}

fn server_data_entry_bindings(contract: &Value) -> Option<Vec<Value>> {
    let bindings = contract
        .get("entries")?
        .as_array()?
        .iter()
        .filter_map(|entry| {
            Some(json!({
                "binding": entry.get("binding")?.clone(),
                "export_name": entry.get("export_name")?.clone(),
                "source_path": entry.get("source_path")?.clone(),
                "execution_model": entry.get("execution_model").cloned().unwrap_or_else(|| json!("unknown")),
            }))
        })
        .collect::<Vec<_>>();
    (!bindings.is_empty()).then_some(bindings)
}

fn route_from_server_data_output(output_dir: &Path, server_data_path: &Path) -> String {
    let relative = relative_build_output_path(output_dir, server_data_path);
    let mut route = relative
        .strip_prefix("app/")
        .unwrap_or(relative.as_str())
        .to_string();
    if let Some(stripped) = route.strip_suffix("/server-data.json") {
        route = stripped.to_string();
    } else if let Some(stripped) = route.strip_suffix("server-data.json") {
        route = stripped.to_string();
    }
    let route = route.trim_matches('/');
    if route.is_empty() {
        "/".to_string()
    } else {
        format!("/{route}")
    }
}

fn relative_build_output_path(output_dir: &Path, path: &Path) -> String {
    path.strip_prefix(output_dir)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn normalized_manifest_path(path: &str) -> String {
    path.replace('\\', "/")
}

fn normalized_manifest_paths(paths: &[String]) -> Vec<String> {
    paths
        .iter()
        .map(|path| normalized_manifest_path(path))
        .collect()
}

fn project_relative_output_path(project_dir: &Path, output: &str) -> PathBuf {
    let mut path = project_dir.to_path_buf();
    for segment in output.replace('\\', "/").split('/') {
        if segment.is_empty() || segment == "." || segment == ".." {
            continue;
        }
        path.push(segment);
    }
    path
}

fn route_sort_key(value: &Value) -> String {
    value
        .get("route")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string()
}

fn output_sort_key(value: &Value) -> String {
    value
        .get("output")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string()
}
