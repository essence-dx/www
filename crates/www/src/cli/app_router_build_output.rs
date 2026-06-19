use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use dx_compiler::delivery::{
    DxReactAppRouteProof, DxReactAppSegmentSource, DxReactClientIsland, DxReactClientIslandInput,
    DxReactClientIslandManifest, DxReactComponentSource, DxReactServerDataManifest,
    DxReactServerSource, compile_react_client_islands, compile_react_server_data_manifest,
    react_client_island_micro_js_bundle,
};
use serde_json::{Value, json};

use crate::app_router_segments::{AppRouteSegmentKind, classify_app_route_segment};
use crate::error::{DxError, DxResult};

use super::app_page_routes;
use super::app_router_execution::{
    DxAppRouterExecutionInput, DxAppRouterRequestValueMode, build_app_router_execution_contract,
};
use super::app_router_server_data::{
    DxServerDataSurfaceStatus, insert_build_time_server_data_request_contracts,
    insert_server_data_adapter_boundary, insert_server_data_surface_metadata,
};

pub(super) struct DxAppRouterExecutionOutputInput<'a> {
    pub(super) cwd: &'a Path,
    pub(super) app_route_path: &'a Path,
    pub(super) app_output_dir: &'a Path,
    pub(super) route: &'a str,
    pub(super) route_source_path: &'a str,
    pub(super) segments: Vec<DxReactAppSegmentSource>,
    pub(super) proof: &'a DxReactAppRouteProof,
    pub(super) source_manifest_hash: Option<String>,
    pub(super) node_modules_present: bool,
    pub(super) server_sources: &'a [DxReactServerSource],
}

pub(super) struct DxAppClientIslandsOutputInput<'a> {
    pub(super) app_route_path: &'a Path,
    pub(super) app_output_dir: &'a Path,
    pub(super) route: &'a str,
    pub(super) route_source_path: &'a str,
    pub(super) segments: Vec<DxReactAppSegmentSource>,
    pub(super) components: Vec<DxReactComponentSource>,
    pub(super) proof: &'a DxReactAppRouteProof,
}

pub(super) struct DxAppStreamingPlanOutputInput<'a> {
    pub(super) app_output_dir: &'a Path,
    pub(super) proof: &'a DxReactAppRouteProof,
}

pub(super) struct DxAppGeneratedStyleAssetsOutputInput<'a> {
    pub(super) output_dir: &'a Path,
    pub(super) proof: &'a DxReactAppRouteProof,
}

pub(super) struct DxAppServerDataOutputInput<'a> {
    pub(super) app_route_path: &'a Path,
    pub(super) app_output_dir: &'a Path,
    pub(super) route: &'a str,
    pub(super) route_source_path: &'a str,
    pub(super) server_sources: &'a [DxReactServerSource],
}

struct DxAppStaticRouteRequestProps {
    route_params: BTreeMap<String, String>,
    search_params: BTreeMap<String, String>,
}

pub(super) fn write_app_router_execution_contract(
    input: DxAppRouterExecutionOutputInput<'_>,
) -> DxResult<()> {
    let route_source =
        std::fs::read_to_string(input.app_route_path).map_err(|error| DxError::IoError {
            path: Some(input.app_route_path.to_path_buf()),
            message: error.to_string(),
        })?;
    let request_props =
        build_static_app_route_request_props(input.route_source_path, &route_source);
    let mut contract = build_app_router_execution_contract(DxAppRouterExecutionInput {
        cwd: input.cwd,
        app_route_path: input.app_route_path,
        route: input.route.to_string(),
        source_path: input.route_source_path.to_string(),
        route_source,
        segments: input.segments,
        proof: input.proof,
        source_manifest_hash: input.source_manifest_hash,
        node_modules_present: input.node_modules_present,
        route_params: &request_props.route_params,
        search_params: &request_props.search_params,
        server_sources: input.server_sources,
        request_value_mode: DxAppRouterRequestValueMode::BuildTimeContractInputs,
    });
    insert_build_time_request_props(&mut contract, &request_props);
    let execution_path = input.app_output_dir.join("app-router-execution.json");
    let contract_json = serde_json::to_string_pretty(&contract).map_err(|error| {
        DxError::ConfigValidationError {
            message: error.to_string(),
            field: Some("app-router execution".to_string()),
        }
    })?;
    std::fs::write(&execution_path, contract_json).map_err(|error| DxError::IoError {
        path: Some(execution_path),
        message: error.to_string(),
    })?;
    Ok(())
}

fn build_static_app_route_request_props(
    route_source_path: &str,
    route_source: &str,
) -> DxAppStaticRouteRequestProps {
    let route_params = static_route_param_names(route_source_path)
        .into_iter()
        .map(|name| {
            let value = static_request_sample_value(&name);
            (name, value)
        })
        .collect::<BTreeMap<_, _>>();
    let search_params = static_search_param_names(route_source)
        .into_iter()
        .map(|name| {
            let value = static_request_sample_value(&name);
            (name, value)
        })
        .collect::<BTreeMap<_, _>>();

    DxAppStaticRouteRequestProps {
        route_params,
        search_params,
    }
}

fn insert_build_time_request_props(
    contract: &mut Value,
    request_props: &DxAppStaticRouteRequestProps,
) {
    let Some(object) = contract.as_object_mut() else {
        return;
    };
    insert_build_time_server_data_request_contracts(
        object,
        &request_props.route_params,
        &request_props.search_params,
    );
}

fn static_route_param_names(route_source_path: &str) -> BTreeSet<String> {
    app_page_routes::page_route_segments_from_source_path(route_source_path)
        .unwrap_or_default()
        .into_iter()
        .filter(|segment| !app_page_routes::is_app_router_non_path_segment(segment))
        .filter_map(|segment| route_param_name(&segment).map(str::to_string))
        .collect()
}

fn route_param_name(segment: &str) -> Option<&str> {
    match classify_app_route_segment(segment) {
        AppRouteSegmentKind::OptionalCatchAll(name)
        | AppRouteSegmentKind::RequiredCatchAll(name)
        | AppRouteSegmentKind::Dynamic(name) => Some(name),
        AppRouteSegmentKind::Static(_)
        | AppRouteSegmentKind::RouteGroup
        | AppRouteSegmentKind::ParallelSlot
        | AppRouteSegmentKind::Private
        | AppRouteSegmentKind::Intercepting
        | AppRouteSegmentKind::Malformed => None,
    }
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
    collect_static_destructured_search_param_names(route_source, &mut names);
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

fn collect_static_destructured_search_param_names(source: &str, output: &mut BTreeSet<String>) {
    collect_static_destructured_named_search_param_names(source, "searchParams", output);
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

pub(super) fn write_app_client_islands_contract(
    input: DxAppClientIslandsOutputInput<'_>,
) -> DxResult<usize> {
    let route_source =
        std::fs::read_to_string(input.app_route_path).map_err(|error| DxError::IoError {
            path: Some(input.app_route_path.to_path_buf()),
            message: error.to_string(),
        })?;
    let manifest = compile_react_client_islands(DxReactClientIslandInput {
        route: input.route.to_string(),
        route_source_path: input.route_source_path.to_string(),
        route_source,
        segments: input.segments,
        components: input.components,
        route_delivery_mode: input.proof.delivery_mode,
    });
    if manifest.islands.is_empty() {
        return Ok(0);
    }

    let islands_path = input.app_output_dir.join("client-islands.json");
    let manifest_json = serde_json::to_string_pretty(&manifest).map_err(|error| {
        DxError::ConfigValidationError {
            message: error.to_string(),
            field: Some("app-router client-islands".to_string()),
        }
    })?;
    std::fs::write(&islands_path, manifest_json).map_err(|error| DxError::IoError {
        path: Some(islands_path),
        message: error.to_string(),
    })?;

    if let Some(bundle) = react_client_island_micro_js_bundle(&manifest) {
        let runtime_path = input.app_output_dir.join("client-islands.js");
        std::fs::write(&runtime_path, bundle).map_err(|error| DxError::IoError {
            path: Some(runtime_path),
            message: error.to_string(),
        })?;
    }

    stamp_client_island_hydration_markers(input.app_output_dir, &manifest)?;

    Ok(manifest.islands.len())
}

fn stamp_client_island_hydration_markers(
    app_output_dir: &Path,
    manifest: &DxReactClientIslandManifest,
) -> DxResult<()> {
    let markers = client_island_hydration_markers(manifest);
    if markers.is_empty() {
        return Ok(());
    }

    let html_path = app_output_dir.join("index.html");
    let html = std::fs::read_to_string(&html_path).map_err(|error| DxError::IoError {
        path: Some(html_path.clone()),
        message: error.to_string(),
    })?;
    let updated = if let Some(index) = html.rfind("</body>") {
        let mut updated = String::with_capacity(html.len() + markers.len());
        updated.push_str(&html[..index]);
        updated.push_str(&markers);
        updated.push_str(&html[index..]);
        updated
    } else {
        format!("{html}{markers}")
    };
    std::fs::write(&html_path, updated).map_err(|error| DxError::IoError {
        path: Some(html_path),
        message: error.to_string(),
    })
}

fn client_island_hydration_markers(manifest: &DxReactClientIslandManifest) -> String {
    if manifest.islands.is_empty() {
        return String::new();
    }

    let mut html = String::from(
        r#"<div hidden data-dx-client-island-bridge="source-owned" data-dx-client-island-abi="camelCase" data-dx-island-abi-schema="dx.react.clientIsland.abi" data-dx-island-directive-style="camelCase-jsx-props" data-dx-no-js-fallback="preserved" data-dx-client-only-adapters="preview-only" data-dx-client-media-support="recognized-not-executed" data-dx-client-interaction-support="recognized-not-executed" data-dx-browser-proof="not-claimed" data-dx-browser-runtime-proof="not-claimed" data-dx-provider-runtime-proof="not-claimed" data-dx-provider-adapters="not-executed">"#,
    );
    for island in &manifest.islands {
        let directive_names = island_directive_names(island);
        html.push_str(&format!(
            r#"<div data-dx-island="{}" data-dx-island-source="{}" data-dx-delivery-mode="{}" data-dx-island-abi-schema="dx.react.clientIsland.abi" data-dx-island-directive-style="camelCase-jsx-props" data-dx-island-hydration-strategy="{}" data-dx-island-directives="{}" data-dx-client-only-adapter="{}" data-dx-client-load="{}" data-dx-client-visible="{}" data-dx-client-idle="{}" data-dx-client-media="{}" data-dx-client-interaction="{}" data-dx-no-js-fallback="preserved" data-dx-browser-proof="not-claimed" data-dx-browser-runtime-proof="not-claimed" data-dx-provider-runtime-proof="not-claimed" data-dx-provider-adapter="not-executed">"#,
            escape_html_attr(&island.id),
            escape_html_attr(&island.source_path),
            island.delivery_mode.as_str(),
            escape_html_attr(&island.hydration.strategy),
            escape_html_attr(&directive_names),
            island_client_only_adapter_status(island),
            island_client_load_status(island),
            island_client_visible_status(island),
            island_client_idle_status(island),
            island_client_media_status(island),
            island_client_interaction_status(island),
        ));
        for event in &island.hydration.events {
            let prevent_default = if event.prevent_default {
                r#" data-dx-prevent-default="true""#
            } else {
                ""
            };
            html.push_str(&format!(
                r#"<span data-dx-event-id="{}" data-dx-event="{}" data-dx-event-element="{}" data-dx-event-handler="{}"{}></span>"#,
                escape_html_attr(&event.event_id),
                escape_html_attr(&event.event),
                escape_html_attr(&event.element),
                escape_html_attr(&event.handler),
                prevent_default,
            ));
        }
        html.push_str("</div>");
    }
    html.push_str("</div>");
    html
}

fn island_directive_names(island: &DxReactClientIsland) -> String {
    let names = island
        .hydration
        .directives
        .iter()
        .map(|directive| directive.name.as_str())
        .collect::<Vec<_>>();
    if names.is_empty() {
        "none".to_string()
    } else {
        names.join(",")
    }
}

fn island_has_directive(island: &DxReactClientIsland, name: &str) -> bool {
    island
        .hydration
        .directives
        .iter()
        .any(|directive| directive.name.as_str() == name)
}

fn island_client_only_adapter_status(island: &DxReactClientIsland) -> &'static str {
    if island_has_directive(island, "clientOnly") {
        "preview-only"
    } else {
        "not-requested"
    }
}

fn island_client_load_status(island: &DxReactClientIsland) -> &'static str {
    if island_has_directive(island, "clientLoad") {
        "observed"
    } else {
        "not-requested"
    }
}

fn island_client_visible_status(island: &DxReactClientIsland) -> &'static str {
    if island_has_directive(island, "clientVisible") {
        "observed"
    } else {
        "not-requested"
    }
}

fn island_client_idle_status(island: &DxReactClientIsland) -> &'static str {
    if island_has_directive(island, "clientIdle") {
        "observed"
    } else {
        "not-requested"
    }
}

fn island_client_media_status(island: &DxReactClientIsland) -> &'static str {
    if island_has_directive(island, "clientMedia") {
        "recognized-not-executed"
    } else {
        "not-requested"
    }
}

fn island_client_interaction_status(island: &DxReactClientIsland) -> &'static str {
    if island_has_directive(island, "clientInteraction") {
        "recognized-not-executed"
    } else {
        "not-requested"
    }
}

fn escape_html_attr(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            _ => escaped.push(ch),
        }
    }
    escaped
}

pub(super) fn write_app_generated_style_assets(
    input: DxAppGeneratedStyleAssetsOutputInput<'_>,
) -> DxResult<usize> {
    let mut bytes = 0usize;
    for asset in &input.proof.generated_styles {
        let asset_path = safe_generated_asset_path(input.output_dir, &asset.output_path)?;
        if let Some(parent) = asset_path.parent() {
            std::fs::create_dir_all(parent).map_err(|error| DxError::IoError {
                path: Some(parent.to_path_buf()),
                message: error.to_string(),
            })?;
        }
        std::fs::write(&asset_path, &asset.css).map_err(|error| DxError::IoError {
            path: Some(asset_path.clone()),
            message: error.to_string(),
        })?;
        bytes += asset.bytes;
    }
    Ok(bytes)
}

fn safe_generated_asset_path(output_dir: &Path, relative_path: &str) -> DxResult<PathBuf> {
    let path = Path::new(relative_path);
    if relative_path.is_empty() || path.is_absolute() {
        return Err(DxError::ConfigValidationError {
            message: format!("Generated asset path must be relative: {relative_path}"),
            field: Some("generated styles".to_string()),
        });
    }
    let mut output = output_dir.to_path_buf();
    for component in path.components() {
        match component {
            std::path::Component::Normal(part) => output.push(part),
            _ => {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Generated asset path cannot escape build output: {relative_path}"
                    ),
                    field: Some("generated styles".to_string()),
                });
            }
        }
    }
    Ok(output)
}

pub(super) fn write_app_streaming_plan(input: DxAppStreamingPlanOutputInput<'_>) -> DxResult<bool> {
    if !input.proof.streaming.enabled {
        return Ok(false);
    }

    let streaming_path = input.app_output_dir.join("streaming-plan.json");
    let streaming_json = serde_json::to_string_pretty(&input.proof.streaming).map_err(|error| {
        DxError::ConfigValidationError {
            message: error.to_string(),
            field: Some("app-router streaming-plan".to_string()),
        }
    })?;
    std::fs::write(&streaming_path, streaming_json).map_err(|error| DxError::IoError {
        path: Some(streaming_path),
        message: error.to_string(),
    })?;
    Ok(true)
}

pub(super) fn write_app_server_data_contract(
    input: DxAppServerDataOutputInput<'_>,
) -> DxResult<usize> {
    let route_source =
        std::fs::read_to_string(input.app_route_path).map_err(|error| DxError::IoError {
            path: Some(input.app_route_path.to_path_buf()),
            message: error.to_string(),
        })?;
    let request_props =
        build_static_app_route_request_props(input.route_source_path, &route_source);
    let compiled = compile_react_server_data_manifest(
        input.route,
        input.route_source_path,
        &route_source,
        input.server_sources,
    );
    let (contract, entry_count) = match compiled {
        Ok(manifest) => {
            if manifest.entries.is_empty()
                && request_props.route_params.is_empty()
                && request_props.search_params.is_empty()
            {
                return Ok(0);
            }
            let entry_count = manifest.entries.len();
            (
                server_data_manifest_contract_json(&manifest, &request_props)?,
                entry_count,
            )
        }
        Err(error) => (
            server_data_adapter_boundary_contract_json(
                input.route,
                input.route_source_path,
                &error,
                &request_props,
            )?,
            0,
        ),
    };

    let server_data_path = input.app_output_dir.join("server-data.json");
    let manifest_json = serde_json::to_string_pretty(&contract).map_err(|error| {
        DxError::ConfigValidationError {
            message: error.to_string(),
            field: Some("app-router server-data".to_string()),
        }
    })?;
    std::fs::write(&server_data_path, manifest_json).map_err(|error| DxError::IoError {
        path: Some(server_data_path),
        message: error.to_string(),
    })?;
    Ok(entry_count)
}

fn server_data_manifest_contract_json(
    manifest: &DxReactServerDataManifest,
    request_props: &DxAppStaticRouteRequestProps,
) -> DxResult<Value> {
    let mut contract =
        serde_json::to_value(manifest).map_err(|error| DxError::ConfigValidationError {
            message: error.to_string(),
            field: Some("app-router server-data".to_string()),
        })?;
    let entry_count = manifest.entries.len();
    if let Some(object) = contract.as_object_mut() {
        insert_server_data_surface_metadata(
            object,
            DxServerDataSurfaceStatus::for_entry_count(entry_count),
            entry_count,
        );
    }
    insert_build_time_request_props(&mut contract, request_props);
    Ok(contract)
}

fn server_data_adapter_boundary_contract_json(
    route: &str,
    route_source_path: &str,
    error: &str,
    request_props: &DxAppStaticRouteRequestProps,
) -> DxResult<Value> {
    let mut contract = json!({
        "version": 1,
        "route": route,
        "route_source_path": route_source_path,
        "node_modules_required": false,
        "lifecycle_scripts_executed": false,
        "entries": [],
        "error": error,
    });
    if let Some(object) = contract.as_object_mut() {
        insert_server_data_surface_metadata(object, DxServerDataSurfaceStatus::AdapterBoundary, 0);
        insert_server_data_adapter_boundary(object, error, true, false);
    }
    insert_build_time_request_props(&mut contract, request_props);
    Ok(contract)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_request_props_cover_dynamic_segments_and_safe_search_params() {
        let props = build_static_app_route_request_props(
            "app/(marketing)/blog/[slug]/[...rest]/page.tsx",
            r#"export default function Page({ searchParams }) {
  return <main>{searchParams.ref}{searchParams["tab"]}{(await searchParams).draft}</main>;
}
"#,
        );

        assert_eq!(props.route_params["slug"], "sample-slug");
        assert_eq!(props.route_params["rest"], "sample-rest");
        assert_eq!(props.search_params["ref"], "sample-ref");
        assert_eq!(props.search_params["tab"], "sample-tab");
        assert_eq!(props.search_params["draft"], "sample-draft");
    }

    #[test]
    fn static_request_props_use_shared_app_router_segments() {
        let required_catch_all_props = build_static_app_route_request_props(
            "app/(shop)/@modal/docs/[section]/[...slug]/page.tsx",
            "export default function Page() { return <main />; }",
        );

        assert_eq!(
            required_catch_all_props.route_params["section"],
            "sample-section"
        );
        assert_eq!(required_catch_all_props.route_params["slug"], "sample-slug");

        let optional_catch_all_props = build_static_app_route_request_props(
            "app/(shop)/@modal/docs/[section]/[[...rest]]/page.tsx",
            "export default function Page() { return <main />; }",
        );

        assert_eq!(
            optional_catch_all_props.route_params["section"],
            "sample-section"
        );
        assert_eq!(optional_catch_all_props.route_params["rest"], "sample-rest");
        assert_eq!(route_param_name("(shop)"), None);
        assert_eq!(route_param_name("@modal"), None);
        assert_eq!(route_param_name("[[bad]]"), None);
    }

    #[test]
    fn static_request_props_cover_destructured_search_params() {
        let props = build_static_app_route_request_props(
            "app/dashboard/[team]/page.tsx",
            r#"export default async function Page({ searchParams }) {
  const { preview, tab: activeTab } = await searchParams;
  const { "view-mode": viewMode, ignored = "fallback" } = searchParams;
  return <main>{preview}{activeTab}{viewMode}{ignored}</main>;
}
"#,
        );

        assert_eq!(props.route_params["team"], "sample-team");
        assert_eq!(props.search_params["preview"], "sample-preview");
        assert_eq!(props.search_params["tab"], "sample-tab");
        assert_eq!(props.search_params["view-mode"], "sample-view-mode");
        assert_eq!(props.search_params["ignored"], "sample-ignored");
    }

    #[test]
    fn static_request_props_cover_awaited_search_param_aliases() {
        let props = build_static_app_route_request_props(
            "app/dashboard/[team]/page.tsx",
            r#"export default async function Page({ searchParams }) {
  const resolvedSearchParams = await searchParams;
  const query = searchParams;
  const { mode } = resolvedSearchParams;
  return <main>{resolvedSearchParams.tab}{query["view"]}{mode}</main>;
}
"#,
        );

        assert_eq!(props.route_params["team"], "sample-team");
        assert_eq!(props.search_params["tab"], "sample-tab");
        assert_eq!(props.search_params["view"], "sample-view");
        assert_eq!(props.search_params["mode"], "sample-mode");
    }

    #[test]
    fn static_request_props_cover_optional_search_param_reads() {
        let props = build_static_app_route_request_props(
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

        assert_eq!(props.route_params["team"], "sample-team");
        assert_eq!(props.search_params["preview"], "sample-preview");
        assert_eq!(props.search_params["view"], "sample-view");
        assert_eq!(props.search_params["mode"], "sample-mode");
        assert_eq!(props.search_params["draft"], "sample-draft");
        assert_eq!(props.search_params["tab"], "sample-tab");
        assert_eq!(props.search_params["panel"], "sample-panel");
    }

    #[test]
    fn build_time_request_props_surface_is_honest_sample_data() {
        let props = build_static_app_route_request_props(
            "app/docs/[slug]/page.tsx",
            r#"export default function Page({ searchParams }) {
  return <main>{searchParams.preview}</main>;
}
"#,
        );
        let mut contract = json!({
            "route": "/docs/[slug]",
        });

        insert_build_time_request_props(&mut contract, &props);

        assert_eq!(contract["request"]["mode"], "static-route-contract-inputs");
        assert_eq!(contract["request"]["route_params"]["slug"], "sample-slug");
        assert_eq!(
            contract["request"]["search_params"]["preview"],
            "sample-preview"
        );
        assert_eq!(contract["request"]["build_time_contract_inputs"], true);
        assert_eq!(contract["request"]["runtime_request_values"], false);
        assert_eq!(contract["request"]["source_owned_contract"], true);
        assert_eq!(
            contract["request"]["external_runtime_request_values"],
            false
        );
        assert_eq!(
            contract["build_time_request_props"]["mode"],
            "static-route-contract-inputs"
        );
        assert_eq!(
            contract["build_time_request_props"]["build_time_contract_inputs"],
            true
        );
    }

    #[test]
    fn server_data_contracts_can_exist_without_loader_entries_for_dynamic_request_props() {
        let props = build_static_app_route_request_props(
            "app/dashboard/[team]/page.tsx",
            r#"export default function Page({ params, searchParams }) {
  return <main>{params.team}{searchParams.tab}</main>;
}
"#,
        );
        let manifest = DxReactServerDataManifest {
            version: 1,
            route: "/dashboard/[team]".to_string(),
            route_source_path: "app/dashboard/[team]/page.tsx".to_string(),
            node_modules_required: false,
            lifecycle_scripts_executed: false,
            entries: Vec::new(),
        };

        let contract = server_data_manifest_contract_json(&manifest, &props).expect("contract");

        assert_eq!(contract["route"], "/dashboard/[team]");
        assert_eq!(contract["schema"], "dx.appRouter.serverData");
        assert_eq!(contract["format"], 1);
        assert_eq!(contract["schema_revision"], 1);
        assert_eq!(contract["status"], "no-loader-bindings");
        assert_eq!(contract["entry_count"], 0);
        assert_eq!(contract["execution_model"], "not-required");
        assert_eq!(contract["source_owned_contract"], true);
        assert_eq!(contract["external_runtime_required"], false);
        assert_eq!(contract["external_runtime_executed"], false);
        assert_eq!(contract["entries"].as_array().expect("entries").len(), 0);
        assert_eq!(contract["request"]["mode"], "static-route-contract-inputs");
        assert_eq!(contract["request"]["route_params"]["team"], "sample-team");
        assert_eq!(contract["request"]["search_params"]["tab"], "sample-tab");
        assert_eq!(contract["request"]["runtime_request_values"], false);
        assert_eq!(contract["request"]["source_owned_contract"], true);
        assert_eq!(
            contract["build_time_request_props"]["mode"],
            "static-route-contract-inputs"
        );
        assert_eq!(
            contract["build_time_request_props"]["build_time_contract_inputs"],
            true
        );
    }

    #[test]
    fn server_data_contract_marks_compile_errors_as_adapter_boundary_without_fake_runtime() {
        let props = build_static_app_route_request_props(
            "app/dashboard/[team]/page.tsx",
            r#"export default function Page({ searchParams }) {
  return <main>{searchParams.tab}</main>;
}
"#,
        );

        let contract = server_data_adapter_boundary_contract_json(
            "/dashboard/[team]",
            "app/dashboard/[team]/page.tsx",
            "server loader must return a supported object literal",
            &props,
        )
        .expect("adapter-boundary contract");

        assert_eq!(contract["route"], "/dashboard/[team]");
        assert_eq!(
            contract["route_source_path"],
            "app/dashboard/[team]/page.tsx"
        );
        assert_eq!(contract["status"], "adapter-boundary");
        assert_eq!(contract["format"], 1);
        assert_eq!(contract["entry_count"], 0);
        assert_eq!(contract["execution_model"], "unsupported-safe-loader-shape");
        assert_eq!(contract["source_owned_contract"], true);
        assert_eq!(contract["external_runtime_required"], false);
        assert_eq!(contract["external_runtime_executed"], false);
        assert_eq!(contract["node_modules_required"], false);
        assert_eq!(contract["lifecycle_scripts_executed"], false);
        assert_eq!(contract["adapter_boundary"]["kind"], "server-data-loader");
        assert_eq!(contract["adapter_boundary"]["build_output_emitted"], true);
        assert_eq!(contract["adapter_boundary"]["source_owned_contract"], true);
        assert_eq!(
            contract["adapter_boundary"]["external_runtime_required"],
            false
        );
        assert_eq!(
            contract["adapter_boundary"]["external_runtime_executed"],
            false
        );
        assert_eq!(contract["request"]["mode"], "static-route-contract-inputs");
        assert_eq!(contract["request"]["route_params"]["team"], "sample-team");
        assert_eq!(contract["request"]["search_params"]["tab"], "sample-tab");
        assert_eq!(contract["request"]["runtime_request_values"], false);
        assert_eq!(
            contract["build_time_request_props"]["build_time_contract_inputs"],
            true
        );
    }
}
