use dx_compiler::delivery::{DxReactAppSegmentKind, DxReactAppSegmentSource};
use serde_json::{Value, json};

mod conflicts;
mod contract;
mod dynamic_imports;
mod font_loaders;
mod inline_server_actions;
mod metadata_exports;
mod page_config_exports;
mod rsc_boundaries;
mod server_actions;

use self::conflicts::collect_next_custom_transform_conflicts;
use self::contract::{
    NEXT_CUSTOM_TRANSFORM_CONTRACT_NAME, NEXT_CUSTOM_TRANSFORM_SCHEMA,
    next_custom_transform_adapter_contract, next_custom_transform_contract_booleans,
    next_custom_transform_limits, next_custom_transform_runtime_generation_contract,
    next_custom_transform_upstream_evidence,
};
use self::dynamic_imports::collect_dynamic_import_detections;
pub(super) use self::font_loaders::collect_font_loader_detections;
use self::metadata_exports::{collect_metadata_export_detections, collect_metadata_export_names};
use self::page_config_exports::collect_page_config_exports as collect_page_config_export_descriptors;
use self::rsc_boundaries::collect_rsc_boundary_detections;
use self::server_actions::collect_server_action_detections;

pub(super) fn build_next_custom_transform_receipt(
    route_source_path: &str,
    route_source: &str,
    segments: &[DxReactAppSegmentSource],
) -> Value {
    let sources = transform_sources(route_source_path, route_source, segments);
    let rsc_boundaries = collect_rsc_boundaries(&sources);
    let server_actions = collect_server_actions(&sources);
    let page_config_exports = collect_page_config_exports(&sources);
    let dynamic_imports = collect_dynamic_imports(&sources);
    let font_loaders = collect_font_loaders(&sources);
    let metadata_exports = collect_metadata_exports(&sources);
    let conflicts = collect_next_custom_transform_conflicts(
        &rsc_boundaries,
        &server_actions,
        &page_config_exports,
        &dynamic_imports,
        &font_loaders,
        &metadata_exports,
    );
    let contract_booleans = next_custom_transform_contract_booleans();
    let node_modules_required = contract_booleans["node_modules_required"].clone();
    let full_nextjs_runtime_parity = contract_booleans["full_nextjs_runtime_parity"].clone();
    let source_owned_receipt = contract_booleans["source_owned_receipt"].clone();

    json!({
        "schema": NEXT_CUSTOM_TRANSFORM_SCHEMA,
        "schema_revision": 1,
        "contract_name": NEXT_CUSTOM_TRANSFORM_CONTRACT_NAME,
        "status": if has_any_detection(&[
            &rsc_boundaries,
            &server_actions,
            &page_config_exports,
            &dynamic_imports,
            &font_loaders,
            &metadata_exports,
        ]) {
            "next-custom-transform-surfaces-detected"
        } else {
            "no-next-custom-transform-surfaces"
        },
        "adapter": next_custom_transform_adapter_contract(),
        "upstream": next_custom_transform_upstream_evidence(),
        "runtime_generation": next_custom_transform_runtime_generation_contract(
            &rsc_boundaries,
            &server_actions,
            &dynamic_imports,
            &font_loaders,
        ),
        "counts": {
            "sources": sources.len(),
            "rsc_boundaries": rsc_boundaries.len(),
            "server_actions": server_actions.len(),
            "page_config_exports": page_config_exports.len(),
            "dynamic_imports": dynamic_imports.len(),
            "font_loaders": font_loaders.len(),
            "metadata_exports": metadata_exports.len(),
            "conflicts": conflicts.len(),
        },
        "rsc_boundaries": rsc_boundaries,
        "server_actions": server_actions,
        "page_config_exports": page_config_exports,
        "dynamic_imports": dynamic_imports,
        "font_loaders": font_loaders,
        "metadata_exports": metadata_exports,
        "conflicts": conflicts,
        "node_modules_required": node_modules_required,
        "full_nextjs_runtime_parity": full_nextjs_runtime_parity,
        "source_owned_receipt": source_owned_receipt,
        "contract_booleans": contract_booleans,
        "limits": next_custom_transform_limits(),
    })
}

struct TransformSource<'a> {
    kind: &'static str,
    source_path: &'a str,
    source: &'a str,
}

fn transform_sources<'a>(
    route_source_path: &'a str,
    route_source: &'a str,
    segments: &'a [DxReactAppSegmentSource],
) -> Vec<TransformSource<'a>> {
    let mut sources = segments
        .iter()
        .map(|segment| TransformSource {
            kind: segment_kind_label(segment.kind),
            source_path: segment.source_path.as_str(),
            source: segment.source.as_str(),
        })
        .collect::<Vec<_>>();
    sources.push(TransformSource {
        kind: "page",
        source_path: route_source_path,
        source: route_source,
    });
    sources
}

fn collect_rsc_boundaries(sources: &[TransformSource<'_>]) -> Vec<Value> {
    let mut boundaries = Vec::new();
    for source in sources {
        let metadata_export_names =
            collect_metadata_export_names(source.source_path, source.source);
        let Some(boundary) = collect_rsc_boundary_detections(
            source.source_path,
            source.kind,
            source.source,
            &metadata_export_names,
        ) else {
            continue;
        };
        boundaries.push(json!({
                "source_path": source.source_path,
                "kind": source.kind,
                "directives": boundary.directives,
                "use_client": boundary.use_client,
                "use_server": boundary.use_server,
                "use_cache": boundary.use_cache,
                "cache_directives": boundary.cache_directives,
                "client_entry": boundary.client_entry,
                "server_action_file": boundary.server_action_file,
                "client_boundary_needed": boundary.client_boundary_needed,
                "metadata_export_in_client": boundary.metadata_export_in_client,
                "metadata_export_names": boundary.metadata_export_names,
                "metadata_export_count": boundary.metadata_export_count,
                "error_file_requires_client": boundary.error_file_requires_client,
                "hook_or_event_boundary": boundary.hook_or_event_boundary,
                "client_entry_reasons": boundary.client_entry_reasons,
                "source_owned_boundary": boundary.source_owned_boundary,
                "runtime_proxy_generated": boundary.runtime_proxy_generated,
        }));
    }
    boundaries
}

fn collect_server_actions(sources: &[TransformSource<'_>]) -> Vec<Value> {
    let mut actions = Vec::new();

    for source in sources {
        for action in collect_server_action_detections(source.source) {
            actions.push(json!({
                "source_path": source.source_path,
                "kind": source.kind,
                "export_name": action.export_name,
                "async_export": action.async_export,
                "directive_location": action.directive_location,
                "export_kind": action.export_kind,
                "action_id_strategy": action.action_id_strategy,
                "next_proxy_generated": action.next_proxy_generated,
                "source_owned_detection": action.source_owned_detection,
            }));
        }
    }

    actions
}

fn collect_page_config_exports(sources: &[TransformSource<'_>]) -> Vec<Value> {
    let mut configs = Vec::new();
    for source in sources {
        for config in collect_page_config_export_descriptors(source.source) {
            configs.push(json!({
                "source_path": source.source_path,
                "kind": source.kind,
                "name": config.name,
                "value_source": config.value_source,
                "value_kind": config.value_kind,
                "export_kind": config.export_kind,
                "compatibility_issue": config.compatibility_issue,
                "source_owned_detection": true,
                "stripped_from_client_bundle": false,
            }));
        }
    }
    configs
}

fn collect_dynamic_imports(sources: &[TransformSource<'_>]) -> Vec<Value> {
    let mut imports = Vec::new();
    for source in sources {
        for detection in collect_dynamic_import_detections(source.source_path, source.source) {
            imports.push(json!({
                "source_path": source.source_path,
                "kind": source.kind,
                "specifier": detection.specifier,
                "call": detection.call,
                "binding_name": detection.binding_name,
                "call_count": detection.call_count,
                "tracked_export_names": detection.tracked_export_names,
                "ssr_false": detection.ssr_false,
                "loadable_generated_added": detection.loadable_generated_added,
                "loadable_generated_field": detection.loadable_generated_field,
                "transition_added": detection.transition_added,
                "track_helper": detection.track_helper,
                "compatibility_issue": detection.compatibility_issue,
                "source_owned_detection": true,
            }));
        }
    }
    imports
}

fn collect_font_loaders(sources: &[TransformSource<'_>]) -> Vec<Value> {
    let mut loaders = Vec::new();
    for source in sources {
        for font in collect_font_loader_detections(source.source_path, source.source) {
            loaders.push(json!({
                "source_path": source.source_path,
                "kind": source.kind,
                "loader": font.loader,
                "imported": font.imported,
                "local": font.local,
                "namespace_import": font.namespace_import,
                "call_count": font.call_count,
                "call_scope": font.call_scope,
                "module_scope": font.module_scope,
                "assigned_to_const": font.assigned_to_const,
                "variable_names": font.variable_names,
                "css_variable_receipt": font.css_variable_receipt,
                "generated_css_import": font.generated_css_import,
                "compatibility_issue": font.compatibility_issue,
                "module_scope_enforced": false,
                "source_owned_detection": true,
            }));
        }
    }
    loaders
}

fn collect_metadata_exports(sources: &[TransformSource<'_>]) -> Vec<Value> {
    let mut exports = Vec::new();
    for source in sources {
        for metadata in collect_metadata_export_detections(source.source_path, source.source) {
            let surfaces = metadata
                .exports
                .iter()
                .map(|surface| {
                    json!({
                        "name": surface.name,
                        "export_kind": surface.export_kind,
                        "async_export": surface.async_export,
                        "value_source": surface.value_source,
                        "value_kind": surface.value_kind,
                        "compatibility_issue": surface.compatibility_issue,
                    })
                })
                .collect::<Vec<_>>();
            exports.push(json!({
                "source_path": source.source_path,
                "kind": source.kind,
                "static_metadata": metadata.static_metadata,
                "generate_metadata": metadata.generate_metadata,
                "metadata_conflict": metadata.metadata_conflict,
                "static_viewport": metadata.static_viewport,
                "generate_viewport": metadata.generate_viewport,
                "viewport_conflict": metadata.viewport_conflict,
                "parsed_static_metadata": metadata.parsed_static_metadata,
                "static_metadata_value_source": metadata.static_metadata_value_source,
                "static_metadata_value_kind": metadata.static_metadata_value_kind,
                "generate_metadata_return_source": metadata.generate_metadata_return_source,
                "generate_metadata_return_kind": metadata.generate_metadata_return_kind,
                "static_viewport_value_source": metadata.static_viewport_value_source,
                "static_viewport_value_kind": metadata.static_viewport_value_kind,
                "generate_viewport_return_source": metadata.generate_viewport_return_source,
                "generate_viewport_return_kind": metadata.generate_viewport_return_kind,
                "exports": surfaces,
                "compatibility_issues": metadata.compatibility_issues,
                "source_owned_detection": true,
                "server_component_only_enforced": metadata.server_component_only_enforced,
            }));
        }
    }
    exports
}

fn has_any_detection(groups: &[&[Value]]) -> bool {
    groups.iter().any(|group| !group.is_empty())
}

fn segment_kind_label(kind: DxReactAppSegmentKind) -> &'static str {
    match kind {
        DxReactAppSegmentKind::Layout => "layout",
        DxReactAppSegmentKind::Template => "template",
        DxReactAppSegmentKind::Loading => "loading",
        DxReactAppSegmentKind::Error => "error",
        DxReactAppSegmentKind::NotFound => "not-found",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn layout(source: &str) -> DxReactAppSegmentSource {
        DxReactAppSegmentSource {
            kind: DxReactAppSegmentKind::Layout,
            source_path: "app/layout.tsx".to_string(),
            source: source.to_string(),
        }
    }

    fn assert_receipt_stays_source_owned_contract(receipt: &Value) {
        assert_eq!(receipt["adapter"]["runtime_takeover"], false);
        assert_eq!(receipt["adapter"]["react_required"], false);
        assert_eq!(receipt["adapter"]["rsc_required"], false);
        assert_eq!(receipt["adapter"]["node_required"], false);
        assert_eq!(receipt["adapter"]["swc_transform_execution"], false);
        assert_eq!(
            receipt["contract_booleans"]["node_modules_required"],
            receipt["node_modules_required"]
        );
        assert_eq!(
            receipt["contract_booleans"]["full_nextjs_runtime_parity"],
            receipt["full_nextjs_runtime_parity"]
        );
        assert_eq!(
            receipt["contract_booleans"]["source_owned_receipt"],
            receipt["source_owned_receipt"]
        );
        assert_eq!(
            receipt["contract_booleans"]["does_not_claim_nextjs_parity"],
            true
        );
        assert_eq!(
            receipt["contract_booleans"]["does_not_require_react_or_rsc"],
            true
        );
        assert_eq!(
            receipt["contract_booleans"]["does_not_require_node_modules"],
            true
        );
        assert_eq!(
            receipt["runtime_generation"]["runtime_generation_contract"],
            "source-receipt-only"
        );
        assert_eq!(
            receipt["runtime_generation"]["runtime_generation_surface_counts"]["rsc_boundaries"],
            receipt["counts"]["rsc_boundaries"]
        );
        assert_eq!(
            receipt["runtime_generation"]["runtime_generation_surface_counts"]["server_actions"],
            receipt["counts"]["server_actions"]
        );
        assert_eq!(
            receipt["runtime_generation"]["runtime_generation_surface_counts"]["dynamic_imports"],
            receipt["counts"]["dynamic_imports"]
        );
        assert_eq!(
            receipt["runtime_generation"]["runtime_generation_surface_counts"]["font_loaders"],
            receipt["counts"]["font_loaders"]
        );
        assert_eq!(
            receipt["runtime_generation"]["detected_generation_attempts"]["next_proxy_generated"],
            0
        );
        assert_eq!(
            receipt["runtime_generation"]["detected_generation_attempts"]["rsc_runtime_proxy_generated"],
            0
        );
        assert_eq!(
            receipt["runtime_generation"]["detected_generation_attempts"]["font_css_import_generated"],
            0
        );
        assert_eq!(
            receipt["runtime_generation"]["detected_generation_attempts"]["dynamic_loadable_generated"],
            0
        );
        assert_eq!(
            receipt["runtime_generation"]["runtime_generation_detected"],
            false
        );
        assert_eq!(
            receipt["runtime_generation"]["source_rewrite_performed"],
            false
        );
        assert_eq!(receipt["runtime_generation"]["next_proxy_generated"], false);
        assert_eq!(
            receipt["runtime_generation"]["rsc_runtime_proxy_generated"],
            false
        );
        assert_eq!(
            receipt["runtime_generation"]["font_css_import_generated"],
            false
        );
        assert_eq!(
            receipt["runtime_generation"]["dynamic_loadable_generated"],
            false
        );
    }

    #[test]
    fn next_custom_transform_receipt_detects_selected_boundaries() {
        let receipt = build_next_custom_transform_receipt(
            "app/dashboard/page.tsx",
            r##""use client";
import dynamic from "next/dynamic";
import { Inter } from "next/font/google";

export const metadata = { title: "Dashboard" };
export async function generateMetadata() { return { title: "Dashboard" }; }
export const viewport = { width: "device-width" };
export function generateViewport() { return { themeColor: "#101827" }; }
export const dynamic = "force-dynamic";

const Chart = dynamic(() => import("../components/Chart"), { ssr: false });

async function saveDashboard() {
  "use server";
  return { ok: true };
}

export default function Page() {
  return <Chart />;
}
"##,
            &[layout(
                r#""use server";
export async function recordLayoutAction() {
  return { ok: true };
}
"#,
            )],
        );

        assert_eq!(receipt["schema"], "dx.next.customTransformReceipt");
        assert_eq!(receipt["adapter"]["mode"], "source-owned-detection-only");
        assert_eq!(receipt["node_modules_required"], false);
        assert_eq!(receipt["full_nextjs_runtime_parity"], false);
        assert_receipt_stays_source_owned_contract(&receipt);
        assert_eq!(receipt["counts"]["rsc_boundaries"], 2);
        assert_eq!(receipt["counts"]["server_actions"], 2);
        assert_eq!(receipt["counts"]["page_config_exports"], 1);
        assert_eq!(receipt["counts"]["dynamic_imports"], 2);
        assert_eq!(receipt["counts"]["font_loaders"], 1);
        assert_eq!(receipt["counts"]["metadata_exports"], 1);
        assert!(
            receipt["conflicts"]
                .as_array()
                .expect("conflicts")
                .iter()
                .any(|conflict| conflict["kind"] == "metadata-and-generateMetadata")
        );
        assert!(
            receipt["conflicts"]
                .as_array()
                .expect("conflicts")
                .iter()
                .any(|conflict| conflict["kind"] == "viewport-and-generateViewport")
        );
        assert!(
            receipt["upstream"]["inspected_files"]
                .as_array()
                .expect("upstream files")
                .iter()
                .any(|file| file == "src/transforms/server_actions.rs")
        );
    }

    #[test]
    fn next_custom_transform_receipt_stays_adapter_boundary_without_detections() {
        let receipt = build_next_custom_transform_receipt(
            "app/page.tsx",
            "export default function Page() { return <main>Hello</main>; }",
            &[],
        );

        assert_eq!(receipt["status"], "no-next-custom-transform-surfaces");
        assert_eq!(receipt["adapter"]["runtime_takeover"], false);
        assert_eq!(receipt["adapter"]["react_required"], false);
        assert_receipt_stays_source_owned_contract(&receipt);
        assert_eq!(receipt["counts"]["conflicts"], 0);
    }
}
