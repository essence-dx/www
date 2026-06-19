use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::path::Path;

use dx_compiler::delivery::{
    DxReactAppRouteProof, DxReactAppSegmentKind, DxReactAppSegmentSource, DxReactServerSource,
    compile_react_server_data_manifest,
};
use serde_json::{Value, json};

use crate::app_router_segments::{AppRouteSegmentKind, classify_app_route_segment};

use super::app_page_routes;
use super::app_router_semantics::build_tsx_app_router_semantics;
use super::app_router_server_data::{
    DxServerDataSurfaceStatus, insert_server_data_adapter_boundary,
    insert_server_data_surface_metadata, server_data_request_contract,
};

mod client_island_runtime;
mod directives;
mod metadata;
mod next_custom_transforms;
mod next_navigation;
mod render_plan;
mod request_props;
mod source_render;
mod state_runtime;

use self::client_island_runtime::client_island_dev_runtime;
use self::metadata::{
    effective_metadata, metadata_head_tag_count, metadata_head_tags, metadata_sources,
};
use self::next_custom_transforms::build_next_custom_transform_receipt;
use self::next_navigation::{build_next_navigation_control_flow, next_navigation_head_tags};
use self::render_plan::build_tsx_render_plan;
use self::source_render::build_tsx_source_render_surface;
use self::state_runtime::build_state_runtime;
pub(super) use self::state_runtime::dx_native_reactivity_capabilities;

const TINY_STATIC_NO_JS_PACKET_GUARD: &str = "tiny_static_route_proof.no_js_capable";

pub(super) struct DxAppRouterExecutionInput<'a> {
    pub cwd: &'a Path,
    pub app_route_path: &'a Path,
    pub route: String,
    pub source_path: String,
    pub route_source: String,
    pub segments: Vec<DxReactAppSegmentSource>,
    pub proof: &'a DxReactAppRouteProof,
    pub source_manifest_hash: Option<String>,
    pub node_modules_present: bool,
    pub route_params: &'a BTreeMap<String, String>,
    pub search_params: &'a BTreeMap<String, String>,
    pub server_sources: &'a [DxReactServerSource],
    pub request_value_mode: DxAppRouterRequestValueMode,
}

pub(super) struct DxAppRouterRuntimeRenderInput<'a> {
    pub cwd: &'a Path,
    pub app_route_path: &'a Path,
    pub route: String,
    pub source_path: String,
    pub route_source: String,
    pub segments: Vec<DxReactAppSegmentSource>,
    pub proof: &'a DxReactAppRouteProof,
    pub source_manifest_hash: Option<String>,
    pub node_modules_present: bool,
    pub route_params: &'a BTreeMap<String, String>,
    pub search_params: &'a BTreeMap<String, String>,
    pub server_sources: &'a [DxReactServerSource],
    pub request_value_mode: DxAppRouterRequestValueMode,
}

#[derive(Clone, Copy)]
pub(super) enum DxAppRouterRequestValueMode {
    Runtime,
    BuildTimeContractInputs,
}

impl DxAppRouterRequestValueMode {
    fn is_runtime(self) -> bool {
        matches!(self, Self::Runtime)
    }

    fn is_build_time_contract_inputs(self) -> bool {
        matches!(self, Self::BuildTimeContractInputs)
    }

    fn label(self) -> &'static str {
        match self {
            Self::Runtime => "runtime-request-values",
            Self::BuildTimeContractInputs => "static-route-contract-inputs",
        }
    }
}

pub(super) fn render_app_router_runtime(input: DxAppRouterRuntimeRenderInput<'_>) -> String {
    let layout_count = segment_count(&input.segments, DxReactAppSegmentKind::Layout);
    let template_count = segment_count(&input.segments, DxReactAppSegmentKind::Template);
    let boundary_count = segment_count(&input.segments, DxReactAppSegmentKind::Loading)
        + segment_count(&input.segments, DxReactAppSegmentKind::Error)
        + segment_count(&input.segments, DxReactAppSegmentKind::NotFound);
    let state_slot_count = input.proof.route_unit.state.slots.len();
    let event_slot_count = input.proof.route_unit.state.event_slots.len();
    let server_data = build_server_data_surface(
        &input.route,
        &input.source_path,
        &input.route_source,
        input.server_sources,
        input.route_params,
        input.search_params,
        input.request_value_mode,
    );
    let state_runtime = build_state_runtime(&input.route, &input.proof.route_unit.state);
    let render_plan = build_tsx_render_plan(&input.route, input.proof);
    let source_render = build_tsx_source_render_surface(
        input.cwd,
        &input.route,
        &input.source_path,
        &input.route_source,
        &input.segments,
        input.proof,
        input.route_params,
        input.search_params,
    );
    let mut contract = build_app_router_execution_contract(DxAppRouterExecutionInput {
        cwd: input.cwd,
        app_route_path: input.app_route_path,
        route: input.route.clone(),
        source_path: input.source_path.clone(),
        route_source: input.route_source,
        segments: input.segments,
        proof: input.proof,
        source_manifest_hash: input.source_manifest_hash.clone(),
        node_modules_present: input.node_modules_present,
        route_params: input.route_params,
        search_params: input.search_params,
        server_sources: input.server_sources,
        request_value_mode: input.request_value_mode,
    });
    if let Some(object) = contract.as_object_mut() {
        object.insert(
            "renderer".to_string(),
            json!({
                "kind": "tsx-app-router-generic",
                "public_authoring": "tsx",
                "legacy_page_formats": "internal-only",
            }),
        );
        object.insert(
            "request".to_string(),
            json!({
                "route_params": input.route_params,
                "search_params": input.search_params,
            }),
        );
        object.insert("state_runtime".to_string(), state_runtime.program.clone());
        object.insert("tsx_render_plan".to_string(), render_plan.clone());
        object.insert("tsx_source_render".to_string(), source_render.clone());
        object.insert("server_data".to_string(), server_data.clone());
    }

    let effect_context_boundaries = contract
        .get("tsx_semantics")
        .and_then(|semantics| semantics.get("effect_context_boundaries"))
        .cloned()
        .unwrap_or_else(|| {
            json!({
                "schema": "dx.tsx.effectContextBoundaryManifest",
                "status": "effect-context-boundaries-unavailable",
            })
        });
    let effect_boundary_count = effect_context_boundaries
        .get("effect_count")
        .and_then(Value::as_u64)
        .unwrap_or(0);
    let context_boundary_count = effect_context_boundaries
        .get("context_source_count")
        .and_then(Value::as_u64)
        .unwrap_or(0);
    let context_runtime_status = json_nested_string(
        &effect_context_boundaries,
        &["context_runtime", "status"],
        "unknown",
    );
    let context_initial_value_count = json_nested_count(
        &effect_context_boundaries,
        &["context_runtime", "context_initial_values"],
    );
    let effect_scheduler_status = json_nested_string(
        &state_runtime.program,
        &["effect_scheduler", "status"],
        "unknown",
    );
    let next_navigation_control_flow = contract
        .get("next_navigation_control_flow")
        .cloned()
        .unwrap_or_else(|| {
            json!({
                "schema": "dx.next.appRouterControlFlow",
                "status": "control-flow-unavailable",
                "node_modules_required": false,
                "source_owned_control_flow": false,
                "external_runtime_required": false,
            })
        });
    let next_navigation_status =
        json_nested_string(&next_navigation_control_flow, &["status"], "unknown");
    let next_redirect_attr = optional_json_string_attr(
        "data-dx-next-redirect",
        &next_navigation_control_flow,
        &["redirect", "destination"],
    );
    let next_not_found_attr = bool_json_attr(
        "data-dx-next-not-found",
        &next_navigation_control_flow,
        &["not_found", "detected"],
    );
    let execution_json = serde_json::to_string(&contract)
        .unwrap_or_else(|_| "{\"error\":\"app-router-contract-serialization\"}".to_string())
        .replace("</script", "<\\/script");
    let source_render_json = serde_json::to_string(&source_render)
        .unwrap_or_else(|_| "{\"error\":\"tsx-source-render-serialization\"}".to_string())
        .replace("</script", "<\\/script");
    let route_params_attr = format_kv_attr("data-dx-route-params", input.route_params);
    let search_params_attr = format_kv_attr("data-dx-search-params", input.search_params);
    let server_data_attrs = server_data_dom_attrs(&server_data);
    let renderable_count = json_array_count(&source_render, "renderable_elements");
    let component_ref_count = json_array_count(&source_render, "component_references");
    let component_composition_count = json_nested_count(
        &source_render,
        &["counts", "source_owned_component_compositions"],
    );
    let prop_binding_count = json_array_count(&source_render, "prop_bindings");
    let page_prop_binding_count =
        json_nested_count(&source_render, &["counts", "page_prop_bindings"]);
    let form_surface_count = json_array_count(&source_render, "form_surfaces");
    let source_import_count =
        json_nested_count(&source_render, &["counts", "source_owned_imports_scanned"]);
    let skipped_import_count =
        json_nested_count(&source_render, &["counts", "source_owned_imports_skipped"]);
    let static_snapshot_count =
        json_nested_count(&source_render, &["counts", "static_dom_snapshot_elements"]);
    let static_snapshot_status = json_nested_string(
        &source_render,
        &["static_dom_snapshot", "status"],
        "unknown",
    );
    let literal_expression_count = json_nested_count(
        &source_render,
        &["counts", "static_dom_snapshot_literal_expressions"],
    );
    let dom_action_binder_status =
        json_nested_string(&source_render, &["dom_action_binder", "status"], "unknown");
    let dom_action_descriptor_count =
        json_nested_count(&source_render, &["counts", "dom_action_descriptors"]);
    let client_island_count =
        json_nested_count(&source_render, &["client_islands", "island_count"]);
    let attrs = format!(
        r#"data-dx-renderer="tsx-app-router-generic" data-dx-app-router-runtime="source-owned-app-router" data-dx-render-plan="{}" data-dx-render-components="{}" data-dx-render-edges="{}" data-dx-tsx-source-render="{}" data-dx-tsx-renderable-elements="{}" data-dx-tsx-component-refs="{}" data-dx-tsx-component-compositions="{}" data-dx-tsx-prop-bindings="{}" data-dx-tsx-page-prop-bindings="{}" data-dx-tsx-form-surfaces="{}" data-dx-tsx-source-imports="{}" data-dx-tsx-skipped-imports="{}" data-dx-tsx-static-snapshot="{}" data-dx-tsx-static-snapshot-elements="{}" data-dx-tsx-literal-expressions="{}" data-dx-client-islands="{}" data-dx-effect-boundaries="{}" data-dx-context-boundaries="{}" data-dx-context-runtime="{}" data-dx-context-initial-values="{}" data-dx-effect-scheduler="{}" data-dx-next-navigation-control-flow="{}" data-dx-dom-action-binder="{}" data-dx-dom-action-descriptors="{}" data-dx-state-runtime="{}" data-dx-route-source="{}" data-dx-layout-count="{}" data-dx-template-count="{}" data-dx-boundary-count="{}" data-dx-state-slots="{}" data-dx-event-slots="{}" data-dx-node-modules-required="false" data-dx-node-modules-present="{}"{}{}{}{}{}"#,
        render_plan["status"].as_str().unwrap_or("unknown"),
        input.proof.page_graph.components.nodes.len(),
        input.proof.page_graph.components.edges.len(),
        source_render["status"].as_str().unwrap_or("unknown"),
        renderable_count,
        component_ref_count,
        component_composition_count,
        prop_binding_count,
        page_prop_binding_count,
        form_surface_count,
        source_import_count,
        skipped_import_count,
        escape_attr(&static_snapshot_status),
        static_snapshot_count,
        literal_expression_count,
        client_island_count,
        effect_boundary_count,
        context_boundary_count,
        escape_attr(&context_runtime_status),
        context_initial_value_count,
        escape_attr(&effect_scheduler_status),
        escape_attr(&next_navigation_status),
        escape_attr(&dom_action_binder_status),
        dom_action_descriptor_count,
        state_runtime.status,
        escape_attr(&input.source_path),
        layout_count,
        template_count,
        boundary_count,
        state_slot_count,
        event_slot_count,
        input.node_modules_present,
        server_data_attrs,
        route_params_attr,
        search_params_attr,
        next_redirect_attr,
        next_not_found_attr
    );
    let html = input.proof.fallback.html.replacen(
        r#"data-dx-template="next-familiar""#,
        &format!(r#"data-dx-template="next-familiar" {attrs}"#),
        1,
    );
    let metadata_head_tags = contract
        .get("metadata_head")
        .and_then(|head| head.get("tags"))
        .and_then(Value::as_str)
        .unwrap_or("");
    let next_navigation_head = next_navigation_head_tags(&next_navigation_control_flow);
    let head_tags = format!("{metadata_head_tags}{next_navigation_head}");
    let html = if head_tags.is_empty() {
        html
    } else {
        html.replacen("</head>", &format!("{head_tags}</head>"), 1)
    };
    let inline_dev_diagnostics = inline_dev_diagnostics_enabled();
    let inline_dev_runtime = inline_dev_runtime_enabled();
    let contract_script = if inline_dev_diagnostics {
        format!(
            r#"<script type="application/json" id="__DX_APP_ROUTER_EXECUTION__">{execution_json}</script>"#
        )
    } else {
        externalized_dev_payload_marker("__DX_APP_ROUTER_EXECUTION__", &input.route)
    };
    let source_render_script = if inline_dev_diagnostics {
        format!(
            r#"<script type="application/json" id="__DX_TSX_SOURCE_RENDER__">{source_render_json}</script>"#
        )
    } else {
        externalized_dev_payload_marker("__DX_TSX_SOURCE_RENDER__", &input.route)
    };
    let state_script = if inline_dev_runtime {
        state_runtime.script_tag.unwrap_or_default()
    } else {
        externalized_dev_payload_marker("__DX_STATE_GRAPH_RUNTIME__", &input.route)
    };
    let client_island_manifest_script = if inline_dev_diagnostics {
        client_island_manifest_script_tag(&source_render)
    } else {
        externalized_dev_payload_marker("__DX_TSX_CLIENT_ISLANDS__", &input.route)
    };
    let effect_context_manifest_script = if inline_dev_diagnostics {
        effect_context_manifest_script_tag(&effect_context_boundaries)
    } else {
        externalized_dev_payload_marker("__DX_TSX_EFFECT_CONTEXT_BOUNDARIES__", &input.route)
    };
    let context_runtime_script = if inline_dev_runtime {
        context_runtime_script_tag(&input.route, &effect_context_boundaries)
    } else {
        externalized_dev_payload_marker("__DX_CONTEXT_RUNTIME__", &input.route)
    };
    let dom_action_binder_script = if inline_dev_runtime {
        dom_action_binder_script_tag(&source_render)
    } else {
        externalized_dev_payload_marker("__DX_DOM_ACTION_BINDER__", &input.route)
    };
    let client_island_dev_runtime = client_island_dev_runtime(&source_render);
    let app_router_runtime_shell = app_router_runtime_shell_preview(&source_render);
    let html = if app_router_runtime_shell.is_empty() {
        html
    } else {
        replace_visible_body_with_app_router_shell(&html, &app_router_runtime_shell)
    };
    let static_dom_preview = static_dom_snapshot_preview(&source_render);
    let asset_script_tags = dev_asset_script_tags(&input.proof.fallback.html);
    let injection = format!(
        "{static_dom_preview}{contract_script}{source_render_script}{client_island_manifest_script}{effect_context_manifest_script}{context_runtime_script}{state_script}{dom_action_binder_script}{client_island_dev_runtime}{asset_script_tags}"
    );
    // Use rfind so we target the real document </body>, not any </body> inside
    // a <template> element (static dom preview etc.) which appears earlier.
    if let Some(last_body_close) = html.rfind("</body>") {
        let mut result = String::with_capacity(html.len() + injection.len() + 7);
        result.push_str(&html[..last_body_close]);
        result.push_str(&injection);
        result.push_str("</body>");
        result.push_str(&html[last_body_close + 7..]);
        result
    } else {
        html
    }
}

pub(super) fn build_app_router_execution_contract(input: DxAppRouterExecutionInput<'_>) -> Value {
    let source_segments = relative_app_route_segments(&input.source_path);
    let route_groups = source_segments
        .iter()
        .filter(|segment| is_route_group_segment(segment))
        .cloned()
        .collect::<Vec<_>>();
    let visible_segments = source_segments
        .iter()
        .filter(|segment| !is_route_group_segment(segment))
        .cloned()
        .collect::<Vec<_>>();
    let dynamic_segments = visible_segments
        .iter()
        .filter_map(|segment| dynamic_segment(segment))
        .collect::<Vec<_>>();
    let page_params = dynamic_segments
        .iter()
        .filter_map(|segment| segment["name"].as_str().map(str::to_string))
        .collect::<BTreeSet<_>>();
    let search_params = search_params_used_by_page(&input.route_source);
    let metadata_sources = metadata_sources(
        &input.segments,
        &input.source_path,
        &input.route_source,
        input.route_params,
        input.search_params,
    );
    let effective_metadata = effective_metadata(&metadata_sources);
    let metadata_head_tags = metadata_head_tags(&effective_metadata);
    let metadata_head_tag_count = metadata_head_tag_count(&metadata_head_tags);
    let next_navigation_control_flow =
        build_next_navigation_control_flow(&input.source_path, &input.route_source);
    let next_custom_transform_receipt = build_next_custom_transform_receipt(
        &input.source_path,
        &input.route_source,
        &input.segments,
    );
    let tsx_semantics = build_tsx_app_router_semantics(
        &input.source_path,
        &input.route_source,
        &input.segments,
        &input.proof.route_unit.state,
    );
    let server_data = build_server_data_surface(
        &input.route,
        &input.source_path,
        &input.route_source,
        input.server_sources,
        input.route_params,
        input.search_params,
        input.request_value_mode,
    );
    let state_runtime = build_state_runtime(&input.route, &input.proof.route_unit.state);
    let render_plan = build_tsx_render_plan(&input.route, input.proof);
    let source_render = build_tsx_source_render_surface(
        input.cwd,
        &input.route,
        &input.source_path,
        &input.route_source,
        &input.segments,
        input.proof,
        input.route_params,
        input.search_params,
    );
    let no_js_capable = input
        .proof
        .route_unit
        .runtime_report
        .tiny_static_route_proof
        .no_js_capable;
    let public_packet_path = if no_js_capable {
        Value::Null
    } else {
        json!("index.dxpk")
    };

    json!({
        "version": 1,
        "route": input.route,
        "source_path": input.source_path,
        "source_path_from_cwd": input.app_route_path.strip_prefix(input.cwd).ok().map(|path| {
            path.components()
                .map(|component| component.as_os_str().to_string_lossy())
                .collect::<Vec<_>>()
                .join("/")
        }),
        "runtime": "source-owned-app-router",
        "compiler_owns_runtime": true,
        "runtime_boundary": {
            "source_owned": true,
            "external_runtime_required": false,
            "external_runtime_executed": false,
        },
        "source_owned_files": true,
        "node_modules_required": false,
        "node_modules_present": input.node_modules_present,
        "route_groups": route_groups,
        "visible_segments": visible_segments,
        "dynamic_segments": dynamic_segments,
        "page_props": {
            "params": page_params.into_iter().collect::<Vec<_>>(),
            "search_params": search_params.into_iter().collect::<Vec<_>>(),
        },
        "metadata_sources": metadata_sources,
        "effective_metadata": effective_metadata,
        "metadata_head": {
            "status": if metadata_head_tag_count > 0 { "source-owned-head-tags-ready" } else { "no-source-owned-head-tags" },
            "tags": metadata_head_tags,
            "tag_count": metadata_head_tag_count,
            "source_owned_head_tags": true,
            "node_modules_required": false,
            "external_runtime_required": false,
            "external_runtime_executed": false,
            "full_next_head_runtime": false,
            "limits": [
                "Emits bounded title, description, canonical, openGraph, and viewport tags from source-owned metadata extraction.",
                "Does not execute Next.js head manager, Metadata API streaming, resource hints, dynamic image generation, cookies, headers, or external runtime code."
            ],
        },
        "next_navigation_control_flow": next_navigation_control_flow,
        "next_custom_transform_receipt": next_custom_transform_receipt,
        "layouts": layout_segments(&input.segments),
        "templates": template_segments(&input.segments),
        "composition_chain": composition_chain(&input.segments),
        "boundaries": boundary_segments(&input.segments),
        "tsx_semantics": tsx_semantics,
        "state_runtime": state_runtime.program,
        "tsx_render_plan": render_plan,
        "tsx_source_render": source_render,
        "server_data": server_data,
        "proof": {
            "page_graph_path": "page-graph.json",
            "packet_path": public_packet_path,
            "tiny_static_no_js_packet_guard": TINY_STATIC_NO_JS_PACKET_GUARD,
            "root_component_id": input.proof.page_graph.root_component_id,
            "delivery_mode": input.proof.delivery_mode.as_str(),
            "layout_count": segment_count(&input.segments, DxReactAppSegmentKind::Layout),
            "template_count": segment_count(&input.segments, DxReactAppSegmentKind::Template),
            "packet_sections": input.proof.packet.section_count,
            "fallback_bytes": input.proof.fallback.bytes,
            "packet_bytes": input.proof.packet.bytes,
            "roundtrip_matches": input.proof.packet.roundtrip_matches,
        },
        "source_manifest_hash": input.source_manifest_hash,
    })
}

fn escape_attr(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn format_kv_attr(name: &str, values: &BTreeMap<String, String>) -> String {
    if values.is_empty() {
        return String::new();
    }
    let value = values
        .iter()
        .map(|(key, value)| format!("{key}={value}"))
        .collect::<Vec<_>>()
        .join("&");
    format!(r#" {name}="{}""#, escape_attr(&value))
}

fn build_server_data_surface(
    route: &str,
    source_path: &str,
    route_source: &str,
    server_sources: &[DxReactServerSource],
    route_params: &BTreeMap<String, String>,
    search_params: &BTreeMap<String, String>,
    request_value_mode: DxAppRouterRequestValueMode,
) -> Value {
    match compile_react_server_data_manifest(route, source_path, route_source, server_sources) {
        Ok(manifest) => {
            let entry_count = manifest.entries.len();
            let entries = serde_json::to_value(&manifest.entries).unwrap_or_else(|_| json!([]));
            let mut surface = json!({
                "route": route,
                "route_source_path": source_path,
                "entries": entries,
                "node_modules_required": manifest.node_modules_required,
                "lifecycle_scripts_executed": manifest.lifecycle_scripts_executed,
                "request": server_data_request_contract(
                    request_value_mode.label(),
                    route_params,
                    search_params,
                    request_value_mode.is_build_time_contract_inputs(),
                    request_value_mode.is_runtime(),
                ),
            });
            if let Some(object) = surface.as_object_mut() {
                insert_server_data_surface_metadata(
                    object,
                    DxServerDataSurfaceStatus::for_entry_count(entry_count),
                    entry_count,
                );
            }
            surface
        }
        Err(error) => {
            let mut surface = json!({
                "route": route,
                "route_source_path": source_path,
                "entries": [],
                "error": error,
                "node_modules_required": false,
                "lifecycle_scripts_executed": false,
                "request": server_data_request_contract(
                    request_value_mode.label(),
                    route_params,
                    search_params,
                    request_value_mode.is_build_time_contract_inputs(),
                    request_value_mode.is_runtime(),
                ),
            });
            if let Some(object) = surface.as_object_mut() {
                insert_server_data_surface_metadata(
                    object,
                    DxServerDataSurfaceStatus::AdapterBoundary,
                    0,
                );
                insert_server_data_adapter_boundary(
                    object,
                    &error,
                    false,
                    request_value_mode.is_runtime(),
                );
            }
            surface
        }
    }
}

fn server_data_dom_attrs(server_data: &Value) -> String {
    let Some(entries) = server_data.get("entries").and_then(Value::as_array) else {
        return String::new();
    };

    let bindings = entries
        .iter()
        .filter_map(|entry| entry.get("binding").and_then(Value::as_str))
        .collect::<Vec<_>>()
        .join(",");
    let status = server_data
        .get("status")
        .and_then(Value::as_str)
        .unwrap_or("unknown");
    let adapter_boundary = status == "adapter-boundary";
    let entry_count = server_data
        .get("entry_count")
        .and_then(Value::as_u64)
        .unwrap_or(entries.len() as u64);
    let schema = json_attr_value(server_data.get("schema"));
    let format = json_attr_value(server_data.get("format"));
    let schema_revision = json_attr_value(server_data.get("schema_revision"));
    let request = server_data.get("request").unwrap_or(&Value::Null);
    let request_mode = json_attr_value(request.get("mode"));
    let runtime_request_values = json_attr_value(request.get("runtime_request_values"));
    let build_time_contract_inputs = json_attr_value(request.get("build_time_contract_inputs"));
    let source_owned_contract = json_attr_value(
        request
            .get("source_owned_contract")
            .or_else(|| server_data.get("source_owned_contract")),
    );
    let external_runtime_required = json_attr_value(server_data.get("external_runtime_required"));
    let external_runtime_executed = json_attr_value(server_data.get("external_runtime_executed"));
    let node_modules_required = json_attr_value(server_data.get("node_modules_required"));
    let lifecycle_scripts_executed = json_attr_value(server_data.get("lifecycle_scripts_executed"));
    let adapter_boundary_details = server_data.get("adapter_boundary").unwrap_or(&Value::Null);
    let adapter_boundary_kind = json_attr_value(adapter_boundary_details.get("kind"));
    let adapter_boundary_reason = json_attr_value(adapter_boundary_details.get("reason"));
    let adapter_boundary_build_output =
        json_attr_value(adapter_boundary_details.get("build_output_emitted"));
    let adapter_boundary_runtime_values =
        json_attr_value(adapter_boundary_details.get("runtime_request_values"));
    let execution_model = entries
        .iter()
        .find_map(|entry| entry.get("execution_model").and_then(Value::as_str))
        .or_else(|| server_data.get("execution_model").and_then(Value::as_str))
        .unwrap_or("unknown");
    let metrics = entries
        .iter()
        .find(|entry| entry.get("binding").and_then(Value::as_str) == Some("metrics"))
        .and_then(|entry| entry.get("value"));

    let mut attrs = vec![
        format!(r#" data-dx-server-data="{}""#, escape_attr(execution_model)),
        format!(r#" data-dx-server-data-status="{}""#, escape_attr(status)),
        format!(
            r#" data-dx-server-data-entries="{}""#,
            escape_attr(&entry_count.to_string())
        ),
    ];
    if let Some(schema) = schema {
        attrs.push(format!(
            r#" data-dx-server-data-schema="{}""#,
            escape_attr(&schema)
        ));
    }
    if let Some(format) = format {
        attrs.push(format!(
            r#" data-dx-server-data-format="{}""#,
            escape_attr(&format)
        ));
    }
    if let Some(schema_revision) = schema_revision {
        attrs.push(format!(
            r#" data-dx-server-data-revision="{}""#,
            escape_attr(&schema_revision)
        ));
    }
    if let Some(request_mode) = request_mode {
        attrs.push(format!(
            r#" data-dx-server-data-request-mode="{}""#,
            escape_attr(&request_mode)
        ));
    }
    if let Some(build_time_contract_inputs) = build_time_contract_inputs {
        attrs.push(format!(
            r#" data-dx-server-data-build-contract-inputs="{}""#,
            escape_attr(&build_time_contract_inputs)
        ));
    }
    if let Some(runtime_request_values) = runtime_request_values {
        attrs.push(format!(
            r#" data-dx-server-data-runtime-values="{}""#,
            escape_attr(&runtime_request_values)
        ));
    }
    if let Some(source_owned_contract) = source_owned_contract {
        attrs.push(format!(
            r#" data-dx-server-data-source-owned-contract="{}""#,
            escape_attr(&source_owned_contract)
        ));
    }
    if let Some(external_runtime_required) = external_runtime_required {
        attrs.push(format!(
            r#" data-dx-server-data-external-runtime-required="{}""#,
            escape_attr(&external_runtime_required)
        ));
    }
    if let Some(external_runtime_executed) = external_runtime_executed {
        attrs.push(format!(
            r#" data-dx-server-data-external-runtime-executed="{}""#,
            escape_attr(&external_runtime_executed)
        ));
    }
    if let Some(node_modules_required) = node_modules_required {
        attrs.push(format!(
            r#" data-dx-server-data-node-modules-required="{}""#,
            escape_attr(&node_modules_required)
        ));
    }
    if let Some(lifecycle_scripts_executed) = lifecycle_scripts_executed {
        attrs.push(format!(
            r#" data-dx-server-data-lifecycle-scripts-executed="{}""#,
            escape_attr(&lifecycle_scripts_executed)
        ));
    }
    if adapter_boundary {
        attrs.push(r#" data-dx-server-data-adapter-boundary="true""#.to_string());
        if let Some(adapter_boundary_kind) = adapter_boundary_kind {
            attrs.push(format!(
                r#" data-dx-server-data-adapter-boundary-kind="{}""#,
                escape_attr(&adapter_boundary_kind)
            ));
        }
        if let Some(adapter_boundary_reason) = adapter_boundary_reason {
            attrs.push(format!(
                r#" data-dx-server-data-adapter-boundary-reason="{}""#,
                escape_attr(&adapter_boundary_reason)
            ));
        }
        if let Some(adapter_boundary_build_output) = adapter_boundary_build_output {
            attrs.push(format!(
                r#" data-dx-server-data-adapter-boundary-build-output="{}""#,
                escape_attr(&adapter_boundary_build_output)
            ));
        }
        if let Some(adapter_boundary_runtime_values) = adapter_boundary_runtime_values {
            attrs.push(format!(
                r#" data-dx-server-data-adapter-boundary-runtime-values="{}""#,
                escape_attr(&adapter_boundary_runtime_values)
            ));
        }
    }
    if !bindings.is_empty() {
        attrs.push(format!(
            r#" data-dx-server-data-bindings="{}""#,
            escape_attr(&bindings)
        ));
    }

    if let Some(metrics) = metrics {
        if let Some(routes) = json_attr_value(metrics.get("routes")) {
            attrs.push(format!(
                r#" data-dx-route-count="{}""#,
                escape_attr(&routes)
            ));
        }
        if let Some(packages) = json_attr_value(metrics.get("packages")) {
            attrs.push(format!(
                r#" data-dx-package-count="{}""#,
                escape_attr(&packages)
            ));
        }
        if let Some(runtime) = json_attr_value(metrics.get("runtime")) {
            attrs.push(format!(
                r#" data-dx-server-data-runtime="{}""#,
                escape_attr(&runtime)
            ));
        }
    }

    attrs.join("")
}

fn json_attr_value(value: Option<&Value>) -> Option<String> {
    match value? {
        Value::String(value) => Some(value.clone()),
        Value::Number(value) => Some(value.to_string()),
        Value::Bool(value) => Some(value.to_string()),
        _ => None,
    }
}

fn optional_json_string_attr(name: &str, value: &Value, path: &[&str]) -> String {
    let Some(attr_value) = path
        .iter()
        .try_fold(value, |current, key| current.get(*key))
        .and_then(Value::as_str)
    else {
        return String::new();
    };
    format!(r#" {name}="{}""#, escape_attr(attr_value))
}

fn bool_json_attr(name: &str, value: &Value, path: &[&str]) -> String {
    let enabled = path
        .iter()
        .try_fold(value, |current, key| current.get(*key))
        .and_then(Value::as_bool)
        .unwrap_or(false);
    if enabled {
        format!(r#" {name}="true""#)
    } else {
        String::new()
    }
}

fn json_array_count(value: &Value, key: &str) -> usize {
    value.get(key).and_then(Value::as_array).map_or(0, Vec::len)
}

fn json_nested_count(value: &Value, path: &[&str]) -> usize {
    path.iter()
        .try_fold(value, |current, key| current.get(*key))
        .and_then(Value::as_u64)
        .unwrap_or(0) as usize
}

fn json_nested_string(value: &Value, path: &[&str], fallback: &str) -> String {
    path.iter()
        .try_fold(value, |current, key| current.get(*key))
        .and_then(Value::as_str)
        .unwrap_or(fallback)
        .to_string()
}

fn inline_dev_diagnostics_enabled() -> bool {
    env_flag_enabled("DX_WWW_INLINE_DEV_DIAGNOSTICS")
        || env_flag_enabled("DX_WWW_INLINE_APP_ROUTER_EXECUTION")
}

fn inline_dev_runtime_enabled() -> bool {
    inline_dev_diagnostics_enabled() || env_flag_enabled("DX_WWW_INLINE_DEV_RUNTIME")
}

fn env_flag_enabled(name: &str) -> bool {
    env::var(name).is_ok_and(|value| {
        matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "1" | "true" | "yes" | "on" | "inline"
        )
    })
}

fn externalized_dev_payload_marker(script_id: &str, route: &str) -> String {
    format!(
        r#"<template id="{script_id}" data-dx-dev-payload="externalized" data-dx-route="{}"></template>"#,
        escape_attr(route)
    )
}

fn dom_action_binder_script_tag(source_render: &Value) -> String {
    let Some(script) = source_render
        .get("dom_action_binder")
        .and_then(|binder| binder.get("script"))
        .and_then(Value::as_str)
        .filter(|script| !script.is_empty())
    else {
        return String::new();
    };
    let script = script.replace("</script", "<\\/script");
    format!(r#"<script id="__DX_DOM_ACTION_BINDER__">{script}</script>"#)
}

fn client_island_manifest_script_tag(source_render: &Value) -> String {
    let Some(manifest) = source_render.get("client_islands") else {
        return String::new();
    };
    let manifest_json = serde_json::to_string(manifest)
        .unwrap_or_else(|_| "{\"error\":\"tsx-client-island-manifest-serialization\"}".to_string())
        .replace("</script", "<\\/script");
    format!(
        r#"<script type="application/json" id="__DX_TSX_CLIENT_ISLANDS__">{manifest_json}</script>"#
    )
}

fn effect_context_manifest_script_tag(effect_context_boundaries: &Value) -> String {
    let manifest_json = serde_json::to_string(effect_context_boundaries)
        .unwrap_or_else(|_| "{\"error\":\"tsx-effect-context-manifest-serialization\"}".to_string())
        .replace("</script", "<\\/script");
    format!(
        r#"<script type="application/json" id="__DX_TSX_EFFECT_CONTEXT_BOUNDARIES__">{manifest_json}</script>"#
    )
}

fn context_runtime_script_tag(route: &str, effect_context_boundaries: &Value) -> String {
    let Some(runtime) = effect_context_boundaries.get("context_runtime") else {
        return String::new();
    };
    let mut runtime_program = runtime.clone();
    if let Some(object) = runtime_program.as_object_mut() {
        object.insert("route".to_string(), json!(route));
    }
    let runtime_json = serde_json::to_string(&runtime_program)
        .unwrap_or_else(|_| "{\"error\":\"tsx-context-runtime-serialization\"}".to_string())
        .replace("</script", "<\\/script");
    format!(
        r#"<script type="module" id="__DX_CONTEXT_RUNTIME__" data-dx-context-runtime="source-owned-provider-value-map">
(() => {{
  const program = {runtime_json};
  const values = Object.create(null);
  const initialValues = program.initial_values || {{}};
  for (const name of program.context_names || []) {{
    values[name] = Object.prototype.hasOwnProperty.call(initialValues, name) ? initialValues[name] : null;
  }}
  function hasContext(name) {{
    return Object.prototype.hasOwnProperty.call(values, name);
  }}
  function contextSelector(name) {{
    const safe = String(name).replace(/\\/g, "\\\\").replace(/"/g, '\\"');
    return '[data-dx-context-read~="' + safe + '"]';
  }}
  function resolveContextValue(name, fallback = null) {{
    return hasContext(name) ? values[name] : fallback;
  }}
  function reflectContextValue(name, value) {{
    const updates = [];
    document.querySelectorAll(contextSelector(name)).forEach((element) => {{
      element.textContent = value == null ? "" : String(value);
      element.setAttribute("data-dx-context-value", value == null ? "" : String(value));
      updates.push({{ target: "text-content", tag: element.localName }});
    }});
    document.dispatchEvent(new CustomEvent("dx:context-value", {{
      detail: {{ route: program.route || null, name, value, updates, full_react_context_runtime: false }}
    }}));
    return updates;
  }}
  function setContext(name, value) {{
    if (!hasContext(name)) values[name] = null;
    values[name] = value;
    return {{ ok: true, name, value, reflected_count: reflectContextValue(name, value).length }};
  }}
  function attachContextRuntime(root = document) {{
    void root;
    for (const name of Object.keys(values)) {{
      reflectContextValue(name, values[name]);
    }}
    return runtime;
  }}
  const runtime = {{
    schema: program.schema,
    schema_revision: program.schema_revision,
    program,
    getSnapshot() {{
      return {{ ...values }};
    }},
    getContext: resolveContextValue,
    resolveContextValue,
    setContext,
    attachContextRuntime
  }};
  window.__DX_CONTEXT_RUNTIME__ = runtime;
  attachContextRuntime(document);
  document.dispatchEvent(new CustomEvent("dx:context-runtime-ready", {{
    detail: {{ status: program.status, context_count: Object.keys(values).length, full_react_context_runtime: false }}
  }}));
}})();
</script>"#
    )
}

fn app_router_runtime_shell_preview(source_render: &Value) -> String {
    let shell = source_render
        .get("composed_static_dom_snapshot")
        .and_then(|snapshot| snapshot.get("app_router_shell"));
    let Some(shell) = shell else {
        return String::new();
    };
    let html = shell
        .get("html")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .trim();
    if html.is_empty() {
        return String::new();
    }
    let html = body_inner_app_router_shell_preview(html);
    if html.is_empty() {
        return String::new();
    }
    let status = shell
        .get("status")
        .and_then(Value::as_str)
        .unwrap_or("unknown");
    let wrappers = shell
        .get("wrapper_count")
        .and_then(Value::as_u64)
        .unwrap_or(0);
    let child_insertions = shell
        .get("child_insertions")
        .and_then(Value::as_u64)
        .unwrap_or(0);
    format!(
        r#"<section data-dx-tsx-app-router-shell="layout-template-page-composition" data-dx-app-router-shell-visible="true" data-dx-app-router-shell-status="{}" data-dx-app-router-shell-wrappers="{}" data-dx-app-router-shell-child-insertions="{}">{html}</section>"#,
        escape_attr(status),
        wrappers,
        child_insertions,
    )
}

fn body_inner_app_router_shell_preview(html: &str) -> String {
    let trimmed = html.trim();
    let normalized = trimmed.to_ascii_lowercase();
    if let Some(body_start) = normalized.find("<body") {
        let Some(open_end) = normalized[body_start..]
            .find('>')
            .map(|index| body_start + index + 1)
        else {
            return trimmed.to_string();
        };
        let close_start = normalized[open_end..]
            .rfind("</body>")
            .map(|index| open_end + index)
            .unwrap_or(trimmed.len());
        return trimmed[open_end..close_start].trim().to_string();
    }
    if normalized.starts_with("<html") {
        let Some(open_end) = normalized.find('>').map(|index| index + 1) else {
            return trimmed.to_string();
        };
        let close_start = normalized.rfind("</html>").unwrap_or(trimmed.len());
        return trimmed[open_end..close_start].trim().to_string();
    }
    trimmed.to_string()
}

/// Extracts JS asset paths from the `data-dx-assets` attribute in the fallback
/// HTML and returns them as `<script defer>` tags so they actually execute in
/// `dx dev` (where scripts inside TSX are stripped and only serialised into
/// `data-dx-assets`, never re-injected by the dev runtime).
fn dev_asset_script_tags(fallback_html: &str) -> String {
    let marker = "data-dx-assets=\"";
    let Some(start) = fallback_html.find(marker) else {
        return String::new();
    };
    let rest = &fallback_html[start + marker.len()..];
    let Some(end) = rest.find('"') else {
        return String::new();
    };
    let raw = &rest[..end];
    // The value may be HTML-entity-encoded (e.g. &#x2F; for /)
    let decoded = raw.replace("&#x2F;", "/").replace("&amp;", "&");
    decoded
        .split(',')
        .map(str::trim)
        .filter(|path| path.ends_with(".js"))
        .map(|path| format!(r#"<script defer src="{}"></script>"#, path))
        .collect::<String>()
}

fn replace_visible_body_with_app_router_shell(html: &str, shell: &str) -> String {
    let normalized = html.to_ascii_lowercase();
    let Some(body_start) = normalized.find("<body") else {
        return format!("{shell}{html}");
    };
    let Some(open_end) = normalized[body_start..]
        .find('>')
        .map(|index| body_start + index + 1)
    else {
        return format!("{shell}{html}");
    };
    let close_start = normalized[open_end..]
        .rfind("</body>")
        .map(|index| open_end + index)
        .unwrap_or(html.len());
    let fallback = html[open_end..close_start].trim();
    let fallback_preview = if fallback.is_empty() {
        String::new()
    } else {
        format!(
            r#"<template data-dx-no-js-fallback-preview="true" data-dx-static-dom-preview-hidden="true">{fallback}</template>"#
        )
    };

    let mut output = String::with_capacity(html.len() + shell.len() + fallback_preview.len());
    output.push_str(&html[..open_end]);
    output.push_str(shell);
    output.push_str(&fallback_preview);
    output.push_str(&html[close_start..]);
    output
}

fn static_dom_snapshot_preview(source_render: &Value) -> String {
    let snapshot = source_render
        .get("composed_static_dom_snapshot")
        .or_else(|| source_render.get("static_dom_snapshot"));
    let Some(snapshot) = snapshot else {
        return String::new();
    };
    let html = snapshot
        .get("html")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .trim();
    if html.is_empty() {
        return String::new();
    }
    let status = snapshot
        .get("status")
        .and_then(Value::as_str)
        .unwrap_or("unknown");
    let element_count = snapshot
        .get("element_count")
        .and_then(Value::as_u64)
        .unwrap_or(0);
    let mode = snapshot
        .get("mode")
        .and_then(Value::as_str)
        .unwrap_or("safe-common-subset");
    format!(
        r#"<template data-dx-tsx-static-dom-preview="{}" data-dx-static-dom-preview-hidden="true" data-dx-tsx-static-snapshot="{}" data-dx-tsx-static-snapshot-elements="{}">{html}</template>"#,
        escape_attr(mode),
        escape_attr(status),
        element_count,
    )
}

fn relative_app_route_segments(source_path: &str) -> Vec<String> {
    app_page_routes::page_route_segments_from_source_path(source_path)
        .map(|segments| {
            segments
                .into_iter()
                .filter(|segment| !app_page_routes::is_parallel_route_slot_segment(segment))
                .collect()
        })
        .unwrap_or_default()
}

fn is_route_group_segment(segment: &str) -> bool {
    matches!(
        classify_app_route_segment(segment),
        AppRouteSegmentKind::RouteGroup
    )
}

fn dynamic_segment(segment: &str) -> Option<Value> {
    let (name, kind) = match classify_app_route_segment(segment) {
        AppRouteSegmentKind::OptionalCatchAll(name) => (name, "optional-catchall"),
        AppRouteSegmentKind::RequiredCatchAll(name) => (name, "catchall"),
        AppRouteSegmentKind::Dynamic(name) => (name, "dynamic"),
        AppRouteSegmentKind::Static(_)
        | AppRouteSegmentKind::RouteGroup
        | AppRouteSegmentKind::ParallelSlot
        | AppRouteSegmentKind::Private
        | AppRouteSegmentKind::Intercepting
        | AppRouteSegmentKind::Malformed => return None,
    };
    Some(json!({
        "name": name,
        "kind": kind,
        "segment": segment,
    }))
}

fn search_params_used_by_page(source: &str) -> BTreeSet<String> {
    let mut params = BTreeSet::new();
    collect_search_param_accesses(source, "searchParams", &mut params);
    collect_search_param_accesses(source, "(await searchParams)", &mut params);
    params
}

fn collect_search_param_accesses(source: &str, receiver: &str, output: &mut BTreeSet<String>) {
    let dot_marker = format!("{receiver}.");
    let optional_dot_marker = format!("{receiver}?.");
    let bracket_marker = format!("{receiver}[");
    let optional_bracket_marker = format!("{receiver}?.[");

    collect_dot_accesses(source, &dot_marker, output);
    collect_dot_accesses(source, &optional_dot_marker, output);
    collect_bracket_accesses(source, &bracket_marker, output);
    collect_bracket_accesses(source, &optional_bracket_marker, output);
}

fn collect_dot_accesses(source: &str, marker: &str, output: &mut BTreeSet<String>) {
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

fn collect_bracket_accesses(source: &str, marker: &str, output: &mut BTreeSet<String>) {
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

fn layout_segments(segments: &[DxReactAppSegmentSource]) -> Vec<Value> {
    typed_segments(segments, DxReactAppSegmentKind::Layout, "layout")
}

fn template_segments(segments: &[DxReactAppSegmentSource]) -> Vec<Value> {
    typed_segments(segments, DxReactAppSegmentKind::Template, "template")
}

fn typed_segments(
    segments: &[DxReactAppSegmentSource],
    kind: DxReactAppSegmentKind,
    label: &str,
) -> Vec<Value> {
    segments
        .iter()
        .filter(|segment| segment.kind == kind)
        .enumerate()
        .map(|(depth, segment)| {
            json!({
                "kind": label,
                "source_path": segment.source_path,
                "depth": segment_route_depth(&segment.source_path).unwrap_or(depth),
            })
        })
        .collect()
}

fn composition_chain(segments: &[DxReactAppSegmentSource]) -> Vec<Value> {
    segments
        .iter()
        .filter_map(|segment| {
            let kind = match segment.kind {
                DxReactAppSegmentKind::Layout => "layout",
                DxReactAppSegmentKind::Template => "template",
                DxReactAppSegmentKind::Loading
                | DxReactAppSegmentKind::Error
                | DxReactAppSegmentKind::NotFound => return None,
            };
            Some(json!({
                "kind": kind,
                "source_path": segment.source_path,
                "depth": segment_route_depth(&segment.source_path).unwrap_or(0),
            }))
        })
        .collect()
}

fn boundary_segments(segments: &[DxReactAppSegmentSource]) -> Value {
    json!({
        "loading": segment_paths(segments, DxReactAppSegmentKind::Loading),
        "error": segment_paths(segments, DxReactAppSegmentKind::Error),
        "not_found": segment_paths(segments, DxReactAppSegmentKind::NotFound),
    })
}

fn segment_paths(segments: &[DxReactAppSegmentSource], kind: DxReactAppSegmentKind) -> Vec<String> {
    segments
        .iter()
        .filter(|segment| segment.kind == kind)
        .map(|segment| segment.source_path.clone())
        .collect()
}

fn segment_count(segments: &[DxReactAppSegmentSource], kind: DxReactAppSegmentKind) -> usize {
    segments
        .iter()
        .filter(|segment| segment.kind == kind)
        .count()
}

fn segment_route_depth(source_path: &str) -> Option<usize> {
    let directory = source_path.rsplit_once('/').map(|(dir, _)| dir)?;
    let relative = strip_app_route_root_directory(directory);
    Some(
        relative
            .split('/')
            .filter(|segment| {
                !segment.is_empty() && (!segment.starts_with('(') || !segment.ends_with(')'))
            })
            .count(),
    )
}

fn strip_app_route_root_directory(directory: &str) -> &str {
    if directory == "src/app" || directory == "app" {
        return "";
    }
    directory
        .strip_prefix("src/app/")
        .or_else(|| directory.strip_prefix("app/"))
        .unwrap_or(directory)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn metadata_sources_resolve_safe_generate_metadata_literal_return() {
        let route_params = BTreeMap::new();
        let search_params = BTreeMap::new();
        let sources = metadata_sources(
            &[],
            "app/blog/[slug]/page.tsx",
            r#"export async function generateMetadata() {
  return {
    title: "Blog Post",
    description: "Static metadata from generateMetadata",
    alternates: {
      canonical: "/blog/acme"
    }
  };
}

export default function Page() {
  return <main>Blog</main>;
}
"#,
            &route_params,
            &search_params,
        );

        assert_eq!(sources.len(), 1);
        assert_eq!(sources[0]["source_path"], "app/blog/[slug]/page.tsx");
        assert_eq!(sources[0]["source_kind"], "generateMetadata");
        assert_eq!(sources[0]["title"], "Blog Post");
        assert_eq!(
            sources[0]["description"],
            "Static metadata from generateMetadata"
        );
        assert_eq!(sources[0]["canonical"], "/blog/acme");
        assert_eq!(sources[0]["node_modules_required"], false);
        assert_eq!(sources[0]["source_owned_metadata"], true);
        assert_eq!(sources[0]["external_runtime_required"], false);
        assert_eq!(sources[0]["external_runtime_executed"], false);

        let effective = effective_metadata(&sources);
        assert_eq!(effective["title"], "Blog Post");
        assert_eq!(effective["canonical"], "/blog/acme");
    }

    #[test]
    fn metadata_sources_resolve_safe_generate_metadata_request_bindings() {
        let route_params = BTreeMap::from([("slug".to_string(), "acme".to_string())]);
        let search_params = BTreeMap::from([("preview".to_string(), "draft".to_string())]);
        let sources = metadata_sources(
            &[],
            "app/blog/[slug]/page.tsx",
            r#"export async function generateMetadata({ params, searchParams }) {
  return {
    title: `Post ${params.slug}`,
    description: searchParams["preview"],
    alternates: {
      canonical: `/blog/${params["slug"]}`
    }
  };
}

export default function Page() {
  return <main>Blog</main>;
}
"#,
            &route_params,
            &search_params,
        );

        assert_eq!(sources.len(), 1);
        assert_eq!(sources[0]["source_kind"], "generateMetadata");
        assert_eq!(
            sources[0]["mode"],
            "safe-generate-metadata-request-literal-return"
        );
        assert_eq!(sources[0]["title"], "Post acme");
        assert_eq!(sources[0]["description"], "draft");
        assert_eq!(sources[0]["canonical"], "/blog/acme");
        assert_eq!(sources[0]["request_bound"], true);
        assert_eq!(sources[0]["node_modules_required"], false);
        assert_eq!(sources[0]["source_owned_metadata"], true);
        assert_eq!(sources[0]["external_runtime_required"], false);
        assert_eq!(sources[0]["external_runtime_executed"], false);

        let bindings = json_string_array(&sources[0], "request_prop_bindings");
        assert!(bindings.contains(&"params.slug".to_string()));
        assert!(bindings.contains(&"searchParams.preview".to_string()));

        let expressions = json_string_array(&sources[0], "supported_expressions");
        assert!(expressions.contains(&"params.slug".to_string()));
        assert!(expressions.contains(&"searchParams[\"preview\"]".to_string()));
        assert!(expressions.contains(&"params[\"slug\"]".to_string()));
        assert!(expressions.contains(&"`Post ${params.slug}`".to_string()));
        assert!(expressions.contains(&"`/blog/${params[\"slug\"]}`".to_string()));
    }

    #[test]
    fn segment_route_depth_counts_src_app_segments_like_root_app() {
        assert_eq!(segment_route_depth("app/layout.tsx"), Some(0));
        assert_eq!(segment_route_depth("src/app/layout.tsx"), Some(0));
        assert_eq!(
            segment_route_depth("app/(marketing)/dashboard/template.tsx"),
            Some(1)
        );
        assert_eq!(
            segment_route_depth("src/app/(marketing)/dashboard/template.tsx"),
            Some(1)
        );
        assert_eq!(segment_route_depth("src/app/blog/[slug]/page.tsx"), Some(2));
    }

    #[test]
    fn execution_route_params_use_shared_app_router_segments() {
        assert_eq!(
            dynamic_segment("[team]").and_then(|value| value["name"].as_str().map(str::to_string)),
            Some("team".to_string())
        );
        assert_eq!(
            dynamic_segment("[...slug]")
                .and_then(|value| value["kind"].as_str().map(str::to_string)),
            Some("catchall".to_string())
        );
        assert_eq!(
            dynamic_segment("[[...rest]]")
                .and_then(|value| value["kind"].as_str().map(str::to_string)),
            Some("optional-catchall".to_string())
        );
        assert!(dynamic_segment("(marketing)").is_none());
        assert!(dynamic_segment("@modal").is_none());
        assert!(dynamic_segment("[[bad]]").is_none());
        assert!(is_route_group_segment("(marketing)"));
        assert!(!is_route_group_segment("(.)photo"));
    }

    #[test]
    fn search_params_used_by_page_records_optional_and_awaited_reads() {
        let params = search_params_used_by_page(
            r#"export default async function Page({ searchParams }) {
  const preview = searchParams?.preview;
  const tab = (await searchParams)?.tab;
  const mode = searchParams?.["mode"];
  const view = (await searchParams)["view"];
  const ignored = searchParams?.[dynamicKey];
  return <main>{preview}{tab}{mode}{view}{ignored}</main>;
}
"#,
        );

        assert!(params.contains("preview"));
        assert!(params.contains("tab"));
        assert!(params.contains("mode"));
        assert!(params.contains("view"));
        assert!(!params.contains("dynamicKey"));
    }

    #[test]
    fn server_data_dom_attrs_expose_loader_metrics_as_source_owned() {
        let server_data = json!({
            "status": "source-owned-safe-loader-data",
            "entry_count": 1,
            "entries": [
                {
                    "binding": "metrics",
                    "execution_model": "source-owned-safe-interpreter",
                    "value": {
                        "routes": 6,
                        "packages": 24,
                        "runtime": "js"
                    }
                }
            ],
            "source_owned_contract": true,
            "external_runtime_required": false,
            "external_runtime_executed": false
        });

        let attrs = server_data_dom_attrs(&server_data);

        assert!(attrs.contains(r#"data-dx-server-data="source-owned-safe-interpreter""#));
        assert!(attrs.contains(r#"data-dx-server-data-status="source-owned-safe-loader-data""#));
        assert!(attrs.contains(r#"data-dx-server-data-entries="1""#));
        assert!(attrs.contains(r#"data-dx-server-data-bindings="metrics""#));
        assert!(attrs.contains(r#"data-dx-route-count="6""#));
        assert!(attrs.contains(r#"data-dx-package-count="24""#));
        assert!(attrs.contains(r#"data-dx-server-data-runtime="js""#));
        assert_eq!(server_data["source_owned_contract"], true);
        assert_eq!(server_data["external_runtime_required"], false);
        assert_eq!(server_data["external_runtime_executed"], false);
    }

    #[test]
    fn server_data_dom_attrs_expose_schema_revision_for_dev_build_consistency() {
        let server_data = json!({
            "schema": "dx.appRouter.serverData",
            "format": 1,
            "schema_revision": 1,
            "status": "no-loader-bindings",
            "entry_count": 0,
            "entries": [],
            "execution_model": "not-required",
            "source_owned_contract": true,
            "external_runtime_required": false,
            "external_runtime_executed": false
        });

        let attrs = server_data_dom_attrs(&server_data);

        assert!(attrs.contains(r#"data-dx-server-data-schema="dx.appRouter.serverData""#));
        assert!(attrs.contains(r#"data-dx-server-data-format="1""#));
        assert!(attrs.contains(r#"data-dx-server-data-revision="1""#));
        assert!(attrs.contains(r#"data-dx-server-data-status="no-loader-bindings""#));
        assert!(attrs.contains(r#"data-dx-server-data-source-owned-contract="true""#));
        assert!(attrs.contains(r#"data-dx-server-data-external-runtime-required="false""#));
        assert!(attrs.contains(r#"data-dx-server-data-external-runtime-executed="false""#));
        assert_eq!(server_data["source_owned_contract"], true);
    }

    #[test]
    fn server_data_dom_attrs_expose_numeric_format_for_stable_schema_name() {
        let server_data = json!({
            "schema": "dx.appRouter.serverData",
            "format": 1,
            "status": "no-loader-bindings",
            "entry_count": 0,
            "execution_model": "not-required",
            "source_owned_contract": true,
            "external_runtime_required": false,
            "external_runtime_executed": false,
            "entries": [],
        });

        let attrs = server_data_dom_attrs(&server_data);

        assert!(attrs.contains(r#"data-dx-server-data-schema="dx.appRouter.serverData""#));
        assert!(attrs.contains(r#"data-dx-server-data-format="1""#));
        assert_eq!(server_data["format"], 1);
    }

    #[test]
    fn server_data_dom_attrs_expose_request_mode_as_source_owned() {
        let server_data = json!({
            "status": "source-owned-safe-loader-data",
            "entry_count": 1,
            "entries": [
                {
                    "binding": "metrics",
                    "execution_model": "source-owned-safe-interpreter",
                    "value": {
                        "routes": 6,
                        "packages": 24,
                        "runtime": "js"
                    }
                }
            ],
            "request": {
                "mode": "runtime-request-values",
                "build_time_contract_inputs": false,
                "runtime_request_values": true,
                "source_owned_contract": true,
                "external_runtime_request_values": false
            }
        });

        let attrs = server_data_dom_attrs(&server_data);

        assert!(attrs.contains(r#"data-dx-server-data-request-mode="runtime-request-values""#));
        assert!(attrs.contains(r#"data-dx-server-data-build-contract-inputs="false""#));
        assert!(attrs.contains(r#"data-dx-server-data-runtime-values="true""#));
        assert!(attrs.contains(r#"data-dx-server-data-source-owned-contract="true""#));
    }

    #[test]
    fn server_data_dom_attrs_expose_source_safe_loader_flags() {
        let server_data = json!({
            "status": "source-owned-safe-loader-data",
            "entry_count": 1,
            "entries": [
                {
                    "binding": "metrics",
                    "execution_model": "source-owned-safe-interpreter",
                    "value": {
                        "routes": 6,
                        "packages": 24,
                        "runtime": "js"
                    }
                }
            ],
            "node_modules_required": false,
            "lifecycle_scripts_executed": false,
            "source_owned_contract": true,
            "external_runtime_required": false,
            "external_runtime_executed": false
        });

        let attrs = server_data_dom_attrs(&server_data);

        assert!(
            attrs.contains(r#"data-dx-server-data-node-modules-required="false""#),
            "{attrs}"
        );
        assert!(
            attrs.contains(r#"data-dx-server-data-lifecycle-scripts-executed="false""#),
            "{attrs}"
        );
    }

    #[test]
    fn server_data_dom_attrs_expose_adapter_boundary_without_fake_runtime() {
        let server_data = json!({
            "status": "adapter-boundary",
            "entry_count": 0,
            "entries": [],
            "execution_model": "unsupported-safe-loader-shape",
            "node_modules_required": false,
            "lifecycle_scripts_executed": false,
            "source_owned_contract": true,
            "external_runtime_required": false,
            "external_runtime_executed": false
        });

        let attrs = server_data_dom_attrs(&server_data);

        assert!(attrs.contains(r#"data-dx-server-data="unsupported-safe-loader-shape""#));
        assert!(attrs.contains(r#"data-dx-server-data-status="adapter-boundary""#));
        assert!(attrs.contains(r#"data-dx-server-data-adapter-boundary="true""#));
        assert!(attrs.contains(r#"data-dx-server-data-source-owned-contract="true""#));
    }

    #[test]
    fn server_data_dom_attrs_expose_adapter_boundary_reason() {
        let server_data = json!({
            "status": "adapter-boundary",
            "entry_count": 0,
            "entries": [],
            "execution_model": "unsupported-safe-loader-shape",
            "node_modules_required": false,
            "lifecycle_scripts_executed": false,
            "source_owned_contract": true,
            "external_runtime_required": false,
            "external_runtime_executed": false,
            "adapter_boundary": {
                "kind": "server-data-loader",
                "reason": "server loader must return a supported object literal",
                "build_output_emitted": false,
                "runtime_request_values": true,
                "source_owned_contract": true,
                "external_runtime_required": false,
                "external_runtime_executed": false
            }
        });

        let attrs = server_data_dom_attrs(&server_data);

        assert!(attrs.contains(r#"data-dx-server-data-adapter-boundary="true""#));
        assert!(
            attrs.contains(r#"data-dx-server-data-adapter-boundary-kind="server-data-loader""#)
        );
        assert!(attrs.contains(
            r#"data-dx-server-data-adapter-boundary-reason="server loader must return a supported object literal""#
        ));
        assert!(attrs.contains(r#"data-dx-server-data-adapter-boundary-build-output="false""#));
        assert!(attrs.contains(r#"data-dx-server-data-adapter-boundary-runtime-values="true""#));
    }

    #[test]
    fn server_data_surface_marks_adapter_boundary_contract_without_fake_runtime() {
        let route_params = BTreeMap::from([("team".to_string(), "launch".to_string())]);
        let search_params = BTreeMap::from([("tab".to_string(), "metrics".to_string())]);
        let server_sources = vec![DxReactServerSource {
            kind: dx_compiler::delivery::DxReactServerSourceKind::Loader,
            source_path: "server/loaders.ts".to_string(),
            source: r#"export async function loadHomeMetrics() {
  return computeMetrics();
}
"#
            .to_string(),
        }];

        let server_data = build_server_data_surface(
            "/dashboard/[team]",
            "app/dashboard/[team]/page.tsx",
            r#"import { loadHomeMetrics } from "@/server/loaders";

export default async function Page() {
  const metrics = await loadHomeMetrics();
  return <main>{metrics.routes}</main>;
}
"#,
            &server_sources,
            &route_params,
            &search_params,
            DxAppRouterRequestValueMode::Runtime,
        );
        let attrs = server_data_dom_attrs(&server_data);

        assert_eq!(server_data["status"], "adapter-boundary");
        assert_eq!(server_data["entry_count"], 0);
        assert_eq!(
            server_data["execution_model"],
            "unsupported-safe-loader-shape"
        );
        assert_eq!(server_data["source_owned_contract"], true);
        assert_eq!(server_data["external_runtime_required"], false);
        assert_eq!(server_data["external_runtime_executed"], false);
        assert_eq!(
            server_data["adapter_boundary"]["kind"],
            "server-data-loader"
        );
        assert_eq!(
            server_data["adapter_boundary"]["build_output_emitted"],
            false
        );
        assert_eq!(
            server_data["adapter_boundary"]["runtime_request_values"],
            true
        );
        assert_eq!(server_data["request"]["mode"], "runtime-request-values");
        assert!(attrs.contains(r#"data-dx-server-data-adapter-boundary="true""#));
        assert!(
            attrs.contains(r#"data-dx-server-data-adapter-boundary-kind="server-data-loader""#)
        );
        assert!(attrs.contains(r#"data-dx-server-data-adapter-boundary-build-output="false""#));
    }

    #[test]
    fn server_data_dom_attrs_expose_no_loader_status_without_fake_runtime() {
        let server_data = json!({
            "status": "no-loader-bindings",
            "entry_count": 0,
            "entries": [],
            "execution_model": "not-required",
            "source_owned_contract": true,
            "external_runtime_required": false,
            "external_runtime_executed": false
        });

        let attrs = server_data_dom_attrs(&server_data);

        assert!(attrs.contains(r#"data-dx-server-data="not-required""#));
        assert!(attrs.contains(r#"data-dx-server-data-status="no-loader-bindings""#));
        assert!(attrs.contains(r#"data-dx-server-data-entries="0""#));
        assert!(!attrs.contains("data-dx-server-data-bindings"));
        assert_eq!(server_data["source_owned_contract"], true);
    }

    #[test]
    fn server_data_surface_marks_runtime_request_values_as_source_owned() {
        let route_params = BTreeMap::from([("team".to_string(), "launch".to_string())]);
        let search_params = BTreeMap::from([("tab".to_string(), "metrics".to_string())]);
        let server_sources = vec![DxReactServerSource {
            kind: dx_compiler::delivery::DxReactServerSourceKind::Loader,
            source_path: "server/loaders.ts".to_string(),
            source: r#"export async function loadHomeMetrics() {
  return { routes: 6, packages: 24, runtime: "js" };
}
"#
            .to_string(),
        }];

        let server_data = build_server_data_surface(
            "/dashboard/[team]",
            "app/dashboard/[team]/page.tsx",
            r#"import { loadHomeMetrics } from "@/server/loaders";

export default async function Page() {
  const metrics = await loadHomeMetrics();
  return <main>{metrics.routes}</main>;
}
"#,
            &server_sources,
            &route_params,
            &search_params,
            DxAppRouterRequestValueMode::Runtime,
        );

        assert_eq!(server_data["status"], "source-owned-safe-loader-data");
        assert_eq!(server_data["request"]["mode"], "runtime-request-values");
        assert_eq!(server_data["request"]["route_params"]["team"], "launch");
        assert_eq!(server_data["request"]["search_params"]["tab"], "metrics");
        assert_eq!(server_data["request"]["build_time_contract_inputs"], false);
        assert_eq!(server_data["request"]["runtime_request_values"], true);
        assert_eq!(server_data["request"]["source_owned_contract"], true);
        assert_eq!(
            server_data["request"]["external_runtime_request_values"],
            false
        );
        assert_eq!(server_data["entries"][0]["binding"], "metrics");
        assert_eq!(server_data["entries"][0]["value"]["routes"], 6);
        assert_eq!(server_data["node_modules_required"], false);
        assert_eq!(server_data["lifecycle_scripts_executed"], false);
    }

    fn json_string_array(value: &Value, key: &str) -> Vec<String> {
        value
            .get(key)
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .map(str::to_string)
            .collect()
    }
}
