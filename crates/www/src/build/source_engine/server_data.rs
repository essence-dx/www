use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use dx_compiler::delivery::{
    DxReactServerDataManifest, DxReactServerSource, DxReactServerSourceKind,
    compile_react_server_data_manifest,
};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};

use crate::app_router_segments::{AppRouteSegmentKind, classify_app_route_segment};
use crate::error::{DxError, DxResult};

use super::graph::{
    SourceBuildRoute, SourceBuildRouteOutput, normalize_path, read_file, relative_path, write_file,
};
use super::route_paths::{source_route_key, source_route_output_slugs, source_route_slug};

const DX_APP_ROUTER_SERVER_DATA_SCHEMA: &str = "dx.appRouter.serverData";
const DX_APP_ROUTER_SERVER_DATA_FORMAT: u32 = 1;
const DX_APP_ROUTER_SERVER_DATA_SCHEMA_REVISION: u32 = 1;

/// Route-local server-data artifact emitted by the source-owned build engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceBuildServerDataRoute {
    /// URL route represented by the server-data artifact.
    pub route: String,
    /// Project-relative App Router page source path.
    #[serde(rename = "route_source_path", alias = "source_path")]
    pub source_path: String,
    /// Project-relative server-data artifact path.
    pub output: String,
    /// Current source-owned server-data status.
    pub status: String,
    /// Number of loader entries compiled into the artifact.
    pub entry_count: usize,
    /// Project-relative server loader source paths that produced route entries.
    #[serde(default)]
    pub entry_source_paths: Vec<String>,
    /// Static build-time request inputs represented by this route artifact.
    #[serde(default)]
    pub request: SourceBuildServerDataRequest,
    /// Execution model used for this route's server-data surface.
    pub execution_model: String,
    /// Source server-data never requires template-local `node_modules`.
    pub node_modules_required: bool,
    /// Source server-data never executes package lifecycle scripts.
    pub lifecycle_scripts_executed: bool,
    /// The artifact is a DX-owned contract emitted by the source build engine.
    pub source_owned_contract: bool,
    /// Source server-data does not require an external framework runtime.
    pub external_runtime_required: bool,
    /// Source server-data does not execute an external framework runtime.
    pub external_runtime_executed: bool,
}

/// Source-owned request-prop inputs for one server-data route artifact.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SourceBuildServerDataRequest {
    /// Request value mode used by this artifact.
    pub mode: String,
    /// Build-time contract route params keyed by dynamic segment name.
    pub route_params: BTreeMap<String, Value>,
    /// Build-time contract search params keyed by source-read query name.
    pub search_params: BTreeMap<String, String>,
    /// Whether values are static build-time contract inputs.
    pub build_time_contract_inputs: bool,
    /// Whether values came from a live runtime request.
    pub runtime_request_values: bool,
    /// Request props are emitted by DX-owned source analysis.
    pub source_owned_contract: bool,
    /// Request props do not come from an external framework runtime.
    pub external_runtime_request_values: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ServerDataStatus {
    SourceOwnedSafeLoaderData,
    NoLoaderBindings,
    AdapterBoundary,
}

impl ServerDataStatus {
    fn for_entry_count(entry_count: usize) -> Self {
        if entry_count == 0 {
            Self::NoLoaderBindings
        } else {
            Self::SourceOwnedSafeLoaderData
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::SourceOwnedSafeLoaderData => "source-owned-safe-loader-data",
            Self::NoLoaderBindings => "no-loader-bindings",
            Self::AdapterBoundary => "adapter-boundary",
        }
    }

    fn execution_model(self) -> &'static str {
        match self {
            Self::SourceOwnedSafeLoaderData => "source-owned-safe-interpreter",
            Self::NoLoaderBindings => "not-required",
            Self::AdapterBoundary => "unsupported-safe-loader-shape",
        }
    }
}

struct StaticRouteRequestProps {
    route_params: BTreeMap<String, Value>,
    search_params: BTreeMap<String, String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum StaticRouteParamShape {
    Dynamic,
    RequiredCatchAll,
    OptionalCatchAll,
}

pub fn emit_server_data_routes(
    project_root: &Path,
    output_dir: &Path,
    routes: &[SourceBuildRoute],
) -> DxResult<Vec<SourceBuildServerDataRoute>> {
    let server_sources = collect_server_sources(project_root)?;
    let mut route_slugs = source_route_output_slugs(routes);
    let mut outputs = Vec::new();

    for route in routes {
        let route_path = project_root.join(&route.path);
        let route_source = read_utf8(&route_path)?;
        let request_props = build_static_route_request_props(&route.path, &route_source);
        let request = source_build_server_data_request(&request_props);
        let compiled = compile_react_server_data_manifest(
            &route.route,
            &route.path,
            &route_source,
            &server_sources,
        );
        let (contract, status, entry_count, entry_source_paths) = match compiled {
            Ok(manifest) => {
                let entry_count = manifest.entries.len();
                let entry_source_paths = entry_source_paths(&manifest);
                (
                    server_data_manifest_contract_json(&manifest, &request_props)?,
                    ServerDataStatus::for_entry_count(entry_count),
                    entry_count,
                    entry_source_paths,
                )
            }
            Err(error) => (
                server_data_adapter_boundary_contract_json(
                    &route.route,
                    &route.path,
                    &error,
                    &request_props,
                )?,
                ServerDataStatus::AdapterBoundary,
                0,
                Vec::new(),
            ),
        };

        let server_data_path = output_dir
            .join("source-routes")
            .join(
                route_slugs
                    .remove(&source_route_key(route))
                    .unwrap_or_else(|| source_route_slug(&route.route)),
            )
            .join("server-data.json");
        write_json(&server_data_path, &contract)?;
        outputs.push(SourceBuildServerDataRoute {
            route: route.route.clone(),
            source_path: route.path.clone(),
            output: normalize_path(&relative_path(project_root, &server_data_path)),
            status: status.as_str().to_string(),
            entry_count,
            entry_source_paths,
            request,
            execution_model: status.execution_model().to_string(),
            node_modules_required: false,
            lifecycle_scripts_executed: false,
            source_owned_contract: true,
            external_runtime_required: false,
            external_runtime_executed: false,
        });
    }

    outputs.sort_by(|left, right| {
        left.route
            .cmp(&right.route)
            .then(left.source_path.cmp(&right.source_path))
    });
    Ok(outputs)
}

pub fn attach_server_data_outputs(
    route_outputs: &mut [SourceBuildRouteOutput],
    server_data_routes: &[SourceBuildServerDataRoute],
) {
    for output in route_outputs {
        output.server_data_output = server_data_routes
            .iter()
            .find(|server_data| {
                server_data.route == output.route && server_data.source_path == output.source_path
            })
            .map(|server_data| server_data.output.clone());
    }
}

fn source_build_server_data_request(
    request_props: &StaticRouteRequestProps,
) -> SourceBuildServerDataRequest {
    SourceBuildServerDataRequest {
        mode: "static-route-contract-inputs".to_string(),
        route_params: request_props.route_params.clone(),
        search_params: request_props.search_params.clone(),
        build_time_contract_inputs: true,
        runtime_request_values: false,
        source_owned_contract: true,
        external_runtime_request_values: false,
    }
}

fn server_data_manifest_contract_json(
    manifest: &DxReactServerDataManifest,
    request_props: &StaticRouteRequestProps,
) -> DxResult<Value> {
    let mut contract =
        serde_json::to_value(manifest).map_err(|error| DxError::ConfigValidationError {
            message: error.to_string(),
            field: Some("source-build server-data".to_string()),
        })?;
    let entry_count = manifest.entries.len();
    if let Some(object) = contract.as_object_mut() {
        insert_server_data_surface_metadata(object, ServerDataStatus::for_entry_count(entry_count));
        object.insert(
            "entry_source_paths".to_string(),
            json!(entry_source_paths(manifest)),
        );
    }
    insert_build_time_request_props(&mut contract, request_props);
    Ok(contract)
}

fn server_data_adapter_boundary_contract_json(
    route: &str,
    route_source_path: &str,
    error: &str,
    request_props: &StaticRouteRequestProps,
) -> DxResult<Value> {
    let mut contract = json!({
        "version": 1,
        "route": route,
        "route_source_path": route_source_path,
        "node_modules_required": false,
        "lifecycle_scripts_executed": false,
        "entry_source_paths": [],
        "entries": [],
        "error": error,
    });
    if let Some(object) = contract.as_object_mut() {
        insert_server_data_surface_metadata(object, ServerDataStatus::AdapterBoundary);
        object.insert(
            "adapter_boundary".to_string(),
            json!({
                "kind": "server-data-loader",
                "reason": error,
                "build_output_emitted": true,
                "runtime_request_values": false,
                "source_owned_contract": true,
                "external_runtime_required": false,
                "external_runtime_executed": false,
            }),
        );
    }
    insert_build_time_request_props(&mut contract, request_props);
    Ok(contract)
}

fn entry_source_paths(manifest: &DxReactServerDataManifest) -> Vec<String> {
    manifest
        .entries
        .iter()
        .map(|entry| entry.source_path.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn insert_server_data_surface_metadata(object: &mut Map<String, Value>, status: ServerDataStatus) {
    object.insert(
        "schema".to_string(),
        json!(DX_APP_ROUTER_SERVER_DATA_SCHEMA),
    );
    object.insert(
        "format".to_string(),
        json!(DX_APP_ROUTER_SERVER_DATA_FORMAT),
    );
    object.insert(
        "schema_revision".to_string(),
        json!(DX_APP_ROUTER_SERVER_DATA_SCHEMA_REVISION),
    );
    object.insert("status".to_string(), json!(status.as_str()));
    object.insert(
        "entry_count".to_string(),
        json!(if status == ServerDataStatus::SourceOwnedSafeLoaderData {
            object
                .get("entries")
                .and_then(Value::as_array)
                .map_or(0, Vec::len)
        } else {
            0
        }),
    );
    object.insert(
        "execution_model".to_string(),
        json!(status.execution_model()),
    );
    object.insert("source_owned_contract".to_string(), json!(true));
    object.insert("external_runtime_required".to_string(), json!(false));
    object.insert("external_runtime_executed".to_string(), json!(false));
    object.insert(
        "limits".to_string(),
        json!([
            "Records DX-owned safe loader data only.",
            "Does not execute arbitrary JavaScript, React Server Components, Node APIs, package lifecycle scripts, or Next.js runtime loaders."
        ]),
    );
}

fn insert_build_time_request_props(contract: &mut Value, request_props: &StaticRouteRequestProps) {
    let Some(object) = contract.as_object_mut() else {
        return;
    };
    let request = json!({
        "mode": "static-route-contract-inputs",
        "route_params": request_props.route_params,
        "search_params": request_props.search_params,
        "build_time_contract_inputs": true,
        "runtime_request_values": false,
        "source_owned_contract": true,
        "external_runtime_request_values": false,
    });
    object.insert("request".to_string(), request.clone());
    object.insert("build_time_request_props".to_string(), request);
}

fn build_static_route_request_props(
    route_source_path: &str,
    route_source: &str,
) -> StaticRouteRequestProps {
    StaticRouteRequestProps {
        route_params: static_route_param_samples(route_source_path),
        search_params: static_search_param_names(route_source)
            .into_iter()
            .map(|name| {
                let value = static_request_sample_value(&name);
                (name, value)
            })
            .collect(),
    }
}

fn static_route_param_samples(route_source_path: &str) -> BTreeMap<String, Value> {
    static_route_param_segments(route_source_path)
        .into_iter()
        .map(|(name, shape)| {
            let value = static_route_param_sample_value(&name, shape);
            (name, value)
        })
        .collect()
}

#[cfg(test)]
fn static_route_param_names(route_source_path: &str) -> BTreeSet<String> {
    static_route_param_segments(route_source_path)
        .into_iter()
        .map(|(name, _kind)| name)
        .collect()
}

fn static_route_param_segments(route_source_path: &str) -> BTreeMap<String, StaticRouteParamShape> {
    let normalized = route_source_path.replace('\\', "/");
    let route = app_route_source_without_app_root(&normalized);
    let route = strip_app_page_suffix(route);
    route
        .split('/')
        .filter_map(static_route_param_shape)
        .map(|(name, shape)| (name.to_string(), shape))
        .collect()
}

fn static_route_param_shape(segment: &str) -> Option<(&str, StaticRouteParamShape)> {
    match classify_app_route_segment(segment) {
        AppRouteSegmentKind::Dynamic(param) => Some((param, StaticRouteParamShape::Dynamic)),
        AppRouteSegmentKind::RequiredCatchAll(param) => {
            Some((param, StaticRouteParamShape::RequiredCatchAll))
        }
        AppRouteSegmentKind::OptionalCatchAll(param) => {
            Some((param, StaticRouteParamShape::OptionalCatchAll))
        }
        AppRouteSegmentKind::Static(_)
        | AppRouteSegmentKind::RouteGroup
        | AppRouteSegmentKind::ParallelSlot
        | AppRouteSegmentKind::Private
        | AppRouteSegmentKind::Intercepting
        | AppRouteSegmentKind::Malformed => None,
    }
}

fn static_route_param_sample_value(name: &str, shape: StaticRouteParamShape) -> Value {
    match shape {
        StaticRouteParamShape::Dynamic => json!(static_request_sample_value(name)),
        StaticRouteParamShape::RequiredCatchAll => json!([static_request_sample_value(name)]),
        StaticRouteParamShape::OptionalCatchAll => json!([]),
    }
}

fn app_route_source_without_app_root(normalized_source_path: &str) -> &str {
    normalized_source_path
        .strip_prefix("src/app/")
        .or_else(|| normalized_source_path.strip_prefix("app/"))
        .unwrap_or(normalized_source_path)
}

fn strip_app_page_suffix(route_source_path_without_app_root: &str) -> &str {
    ["tsx", "jsx", "ts", "js"]
        .iter()
        .find_map(|extension| {
            route_source_path_without_app_root.strip_suffix(&format!("/page.{extension}"))
        })
        .unwrap_or(route_source_path_without_app_root)
}

fn static_search_param_names(route_source: &str) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    collect_static_dot_accesses(route_source, "searchParams.", &mut names);
    collect_static_dot_accesses(route_source, "searchParams?.", &mut names);
    collect_static_dot_accesses(route_source, "(await searchParams).", &mut names);
    collect_static_dot_accesses(route_source, "(await searchParams)?.", &mut names);
    collect_static_bracket_accesses(route_source, "searchParams[", &mut names);
    collect_static_bracket_accesses(route_source, "searchParams?.[", &mut names);
    collect_static_bracket_accesses(route_source, "(await searchParams)[", &mut names);
    collect_static_bracket_accesses(route_source, "(await searchParams)?.[", &mut names);
    collect_static_destructured_named_search_param_names(route_source, "searchParams", &mut names);
    for alias in static_search_param_aliases(route_source) {
        collect_static_identifier_dot_accesses(route_source, &alias, &mut names);
        collect_static_identifier_optional_dot_accesses(route_source, &alias, &mut names);
        collect_static_identifier_bracket_accesses(route_source, &alias, &mut names);
        collect_static_identifier_optional_bracket_accesses(route_source, &alias, &mut names);
        collect_static_destructured_named_search_param_names(route_source, &alias, &mut names);
    }
    names
}

fn static_search_param_aliases(route_source: &str) -> BTreeSet<String> {
    let mut aliases = BTreeSet::new();
    let Ok(alias_re) = regex::Regex::new(
        r#"(?m)\b(?:const|let)\s+([A-Za-z_$][A-Za-z0-9_$]*)(?:\s*:\s*[^=;\n]+)?\s*=\s*\(?\s*(?:await\s+)?searchParams\s*\)?"#,
    ) else {
        return aliases;
    };
    for capture in alias_re.captures_iter(route_source) {
        let Some(alias) = capture.get(1).map(|value| value.as_str()) else {
            continue;
        };
        if alias != "searchParams" {
            aliases.insert(alias.to_string());
        }
    }
    aliases
}

fn collect_static_dot_accesses(source: &str, marker: &str, output: &mut BTreeSet<String>) {
    let mut cursor = source;
    while let Some(index) = cursor.find(marker) {
        let candidate = &cursor[index + marker.len()..];
        let name = candidate
            .chars()
            .take_while(|ch| ch.is_ascii_alphanumeric() || *ch == '_' || *ch == '-')
            .collect::<String>();
        let name_len = name.len();
        if !name.is_empty() {
            output.insert(name);
        }
        cursor = &candidate[name_len..];
    }
}

fn collect_static_identifier_dot_accesses(
    source: &str,
    identifier: &str,
    output: &mut BTreeSet<String>,
) {
    collect_static_identifier_named_accesses(source, identifier, ".", output);
}

fn collect_static_identifier_optional_dot_accesses(
    source: &str,
    identifier: &str,
    output: &mut BTreeSet<String>,
) {
    collect_static_identifier_named_accesses(source, identifier, "?.", output);
}

fn collect_static_identifier_named_accesses(
    source: &str,
    identifier: &str,
    suffix: &str,
    output: &mut BTreeSet<String>,
) {
    let marker = format!("{identifier}{suffix}");
    let mut cursor = 0usize;
    while let Some(offset) = source[cursor..].find(&marker) {
        let index = cursor + offset;
        let candidate = &source[index + marker.len()..];
        if has_identifier_boundaries(source, index, identifier.len()) {
            let name = candidate
                .chars()
                .take_while(|ch| ch.is_ascii_alphanumeric() || *ch == '_' || *ch == '-')
                .collect::<String>();
            if !name.is_empty() {
                output.insert(name);
            }
        }
        cursor = index + marker.len();
    }
}

fn collect_static_bracket_accesses(source: &str, marker: &str, output: &mut BTreeSet<String>) {
    let mut cursor = source;
    while let Some(index) = cursor.find(marker) {
        let candidate = cursor[index + marker.len()..].trim_start();
        let Some(quote) = candidate
            .chars()
            .next()
            .filter(|ch| *ch == '"' || *ch == '\'')
        else {
            cursor = candidate;
            continue;
        };
        let rest = &candidate[quote.len_utf8()..];
        if let Some(end) = rest.find(quote) {
            let name = &rest[..end];
            if !name.is_empty() {
                output.insert(name.to_string());
            }
            cursor = &rest[end + quote.len_utf8()..];
        } else {
            break;
        }
    }
}

fn collect_static_identifier_bracket_accesses(
    source: &str,
    identifier: &str,
    output: &mut BTreeSet<String>,
) {
    collect_static_identifier_quoted_accesses(source, identifier, "[", output);
}

fn collect_static_identifier_optional_bracket_accesses(
    source: &str,
    identifier: &str,
    output: &mut BTreeSet<String>,
) {
    collect_static_identifier_quoted_accesses(source, identifier, "?.[", output);
}

fn collect_static_identifier_quoted_accesses(
    source: &str,
    identifier: &str,
    suffix: &str,
    output: &mut BTreeSet<String>,
) {
    let marker = format!("{identifier}{suffix}");
    let mut cursor = 0usize;
    while let Some(offset) = source[cursor..].find(&marker) {
        let index = cursor + offset;
        let candidate = source[index + marker.len()..].trim_start();
        if !has_identifier_boundaries(source, index, identifier.len()) {
            cursor = index + marker.len();
            continue;
        }
        let Some(quote) = candidate
            .chars()
            .next()
            .filter(|ch| *ch == '"' || *ch == '\'')
        else {
            cursor = index + marker.len();
            continue;
        };
        let rest = &candidate[quote.len_utf8()..];
        if let Some(end) = rest.find(quote) {
            let name = &rest[..end];
            if !name.is_empty() {
                output.insert(name.to_string());
            }
        }
        cursor = index + marker.len();
    }
}

fn collect_static_destructured_named_search_param_names(
    source: &str,
    target: &str,
    output: &mut BTreeSet<String>,
) {
    let mut cursor = 0usize;
    while let Some(offset) = source[cursor..].find(target) {
        let marker_index = cursor + offset;
        if !has_identifier_boundaries(source, marker_index, target.len()) {
            cursor = marker_index + target.len();
            continue;
        }
        let Some(close_brace) = source[..marker_index].rfind('}') else {
            cursor = marker_index + target.len();
            continue;
        };
        let between = &source[close_brace + 1..marker_index];
        if !is_search_params_destructure_assignment(between) {
            cursor = marker_index + target.len();
            continue;
        }
        let Some(open_brace) = source[..close_brace].rfind('{') else {
            cursor = marker_index + target.len();
            continue;
        };
        for binding in source[open_brace + 1..close_brace].split(',') {
            if let Some(name) = search_param_destructured_name(binding) {
                output.insert(name);
            }
        }
        cursor = marker_index + target.len();
    }
}

fn is_search_params_destructure_assignment(between: &str) -> bool {
    let mut value = between.trim();
    let Some(after_assign) = value.strip_prefix('=') else {
        return false;
    };
    value = after_assign.trim();
    value = value.strip_prefix('(').unwrap_or(value).trim();
    value = value.strip_prefix("await").unwrap_or(value).trim();
    value.is_empty()
}

fn search_param_destructured_name(binding: &str) -> Option<String> {
    let binding = binding.trim();
    if binding.is_empty() || binding.starts_with("...") || binding.starts_with('[') {
        return None;
    }
    let name = binding
        .split(':')
        .next()
        .unwrap_or(binding)
        .split('=')
        .next()
        .unwrap_or(binding)
        .trim()
        .trim_matches('"')
        .trim_matches('\'');
    if name.is_empty()
        || !name
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-')
    {
        return None;
    }
    Some(name.to_string())
}

fn has_identifier_boundaries(source: &str, start: usize, len: usize) -> bool {
    let before_ok = source[..start]
        .chars()
        .next_back()
        .is_none_or(|ch| !is_js_identifier_char(ch));
    let after = start + len;
    let after_ok = source[after..]
        .chars()
        .next()
        .is_none_or(|ch| !is_js_identifier_char(ch));
    before_ok && after_ok
}

fn is_js_identifier_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_' || ch == '$'
}

fn static_request_sample_value(name: &str) -> String {
    let slug = name
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
                ch
            } else {
                '-'
            }
        })
        .collect::<String>();
    format!("sample-{slug}")
}

fn collect_server_sources(project_root: &Path) -> DxResult<Vec<DxReactServerSource>> {
    let mut sources = Vec::new();
    push_server_source(
        project_root,
        &mut sources,
        &project_root.join("server/loaders.ts"),
        DxReactServerSourceKind::Loader,
    )?;
    push_server_source(
        project_root,
        &mut sources,
        &project_root.join("server/actions.ts"),
        DxReactServerSourceKind::Action,
    )?;

    let api_dir = project_root.join("app/api");
    if api_dir.exists() {
        for entry in walkdir::WalkDir::new(&api_dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_file() && entry.file_name() == "route.ts")
        {
            push_server_source(
                project_root,
                &mut sources,
                entry.path(),
                DxReactServerSourceKind::RouteHandler,
            )?;
        }
    }

    sources.sort_by(|left, right| left.source_path.cmp(&right.source_path));
    Ok(sources)
}

fn push_server_source(
    project_root: &Path,
    sources: &mut Vec<DxReactServerSource>,
    path: &Path,
    kind: DxReactServerSourceKind,
) -> DxResult<()> {
    if !path.is_file() {
        return Ok(());
    }
    sources.push(DxReactServerSource {
        kind,
        source_path: normalize_path(&relative_path(project_root, path)),
        source: read_utf8(path)?,
    });
    Ok(())
}

fn read_utf8(path: &Path) -> DxResult<String> {
    String::from_utf8(read_file(path)?).map_err(|error| DxError::CompilationError {
        message: error.to_string(),
        file: path.to_path_buf(),
        src: None,
        span: None,
    })
}

fn write_json<T: serde::Serialize>(path: &Path, value: &T) -> DxResult<()> {
    let json = serde_json::to_string_pretty(value)?;
    write_file(path, json.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn src_app_dynamic_params_are_sampled() {
        let props = build_static_route_request_props(
            "src/app/dashboard/[team]/page.tsx",
            "export default function Page({ searchParams }) { return searchParams.tab }",
        );

        assert_eq!(
            props.route_params.get("team"),
            Some(&serde_json::json!("sample-team"))
        );
        assert_eq!(
            props.search_params.get("tab").map(String::as_str),
            Some("sample-tab")
        );
    }

    #[test]
    fn optional_search_param_reads_are_sampled() {
        let props = build_static_route_request_props(
            "app/dashboard/[team]/page.tsx",
            r#"export default async function Page({ searchParams }) {
  const resolvedSearchParams = await searchParams;
  const query = searchParams;
  return <main>
    {searchParams?.preview}
    {searchParams?.["view"]}
    {(await searchParams)?.mode}
    {(await searchParams)?.["draft"]}
    {resolvedSearchParams?.tab}
    {query?.["panel"]}
  </main>;
}
"#,
        );

        assert_eq!(
            props.search_params.get("preview").map(String::as_str),
            Some("sample-preview")
        );
        assert_eq!(
            props.search_params.get("view").map(String::as_str),
            Some("sample-view")
        );
        assert_eq!(
            props.search_params.get("mode").map(String::as_str),
            Some("sample-mode")
        );
        assert_eq!(
            props.search_params.get("draft").map(String::as_str),
            Some("sample-draft")
        );
        assert_eq!(
            props.search_params.get("tab").map(String::as_str),
            Some("sample-tab")
        );
        assert_eq!(
            props.search_params.get("panel").map(String::as_str),
            Some("sample-panel")
        );
    }

    #[test]
    fn static_route_param_names_use_shared_app_router_segments() {
        let params = static_route_param_names(
            "src/app/(docs)/@modal/docs/[...slug]/[[...rest]]/[id]/page.tsx",
        );

        assert_eq!(
            params,
            BTreeSet::from(["id".to_string(), "rest".to_string(), "slug".to_string()])
        );

        let malformed = static_route_param_names("app/docs/[...]/[[bad]]/[ok]/page.jsx");

        assert_eq!(malformed, BTreeSet::from(["ok".to_string()]));
    }

    #[test]
    fn catch_all_route_params_use_array_samples() {
        let required = build_static_route_request_props("app/docs/[...slug]/page.tsx", "");
        assert_eq!(
            required.route_params.get("slug"),
            Some(&serde_json::json!(["sample-slug"]))
        );

        let optional = build_static_route_request_props("app/files/[[...path]]/page.tsx", "");
        assert_eq!(
            optional.route_params.get("path"),
            Some(&serde_json::json!([]))
        );
    }
}
