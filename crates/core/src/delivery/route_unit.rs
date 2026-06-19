use std::collections::{BTreeMap, BTreeSet};

use super::app_route::{
    DxReactAppRouteInput, DxReactAppRoutePacketProof, DxReactGeneratedStyleAsset,
    DxReactStreamingProof,
};
use super::contract::{
    DxAdaptiveRuntimeReport, DxDerivedStateSlot, DxFallbackHtml, DxPageGraph,
    DxRouteClientIslandAbiReceipt, DxRoutePacketRef, DxRouteReceipt, DxRouteShell, DxRouteUnit,
    DxRuntimeCandidate, DxRuntimeRejection, DxServerActionEdge, DxStateEffectSlot,
    DxStateEventSlot, DxStateGraph, DxStateScope, DxStateSlot, DxTinyStaticRouteProof,
};
use super::dom_events::react_style_event_attribute_to_dom_event;
use super::global_store::global_store_bindings;
use super::jsx_lowering::lower_react_jsx_source;
use super::react_state::react_state_bindings;
use super::tsx_ast::parse_tsx_module;
use super::types::DxDeliveryMode;

pub(super) fn compile_route_unit(
    input: &DxReactAppRouteInput,
    page_graph: &DxPageGraph,
    fallback: &DxFallbackHtml,
    delivery_mode: DxDeliveryMode,
    generated_styles: &[DxReactGeneratedStyleAsset],
    streaming: &DxReactStreamingProof,
    packet: &DxReactAppRoutePacketProof,
) -> DxRouteUnit {
    let state = react_state_graph(input);
    let unsupported_react_event_attributes = unsupported_react_event_attributes(input);
    let runtime_report = adaptive_runtime_report(
        delivery_mode,
        fallback,
        generated_styles,
        streaming,
        packet,
        &state,
        &unsupported_react_event_attributes,
    );
    DxRouteUnit {
        version: 1,
        route: input.route.clone(),
        source_path: input.route_source_path.clone(),
        shell: DxRouteShell {
            fallback: fallback.clone(),
            style_hrefs: generated_styles
                .iter()
                .map(|asset| asset.href.clone())
                .collect(),
            streaming_strategy: streaming.strategy.clone(),
            first_flush_bytes: streaming.shell_bytes,
            runtime_required: delivery_mode != DxDeliveryMode::Static,
        },
        graph: page_graph.clone(),
        state,
        packet: DxRoutePacketRef {
            kind: packet.decoded_kind,
            bytes: packet.bytes,
            payload_bytes: packet.payload_bytes,
            section_count: packet.section_count,
            roundtrip_matches: packet.roundtrip_matches,
        },
        receipt: route_receipt(input, streaming),
        runtime_report,
    }
}

fn unsupported_react_event_attributes(input: &DxReactAppRouteInput) -> Vec<String> {
    let mut unsupported = BTreeSet::new();
    for source in route_state_sources(input) {
        let lowered = lower_react_jsx_source(&source.source_path, &source.source);
        for element in lowered.elements {
            for attribute in element
                .attributes
                .iter()
                .filter(|attribute| attribute.name.starts_with("on"))
            {
                if react_style_event_attribute_to_dom_event(&attribute.name).is_none() {
                    unsupported.insert(format!("{}:{}", source.source_path, attribute.name));
                }
            }
        }
    }
    unsupported.into_iter().collect()
}

fn react_state_graph(input: &DxReactAppRouteInput) -> DxStateGraph {
    let mut sources = route_state_sources(input);
    sources.sort_by(|left, right| left.source_path.cmp(&right.source_path));

    let server_actions = server_action_edges(input);
    let server_action_names = server_actions
        .iter()
        .map(|edge| edge.action.as_str())
        .collect::<BTreeSet<_>>();
    let mut slots = Vec::new();
    let mut derived_slots = Vec::new();
    let mut event_slots = Vec::new();
    let mut effects = Vec::new();
    let mut global_stores = Vec::new();
    let global_stores_by_source = sources
        .iter()
        .map(|source| {
            (
                source.source_path.clone(),
                global_store_bindings(&source.source_path, &source.source),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let all_global_stores = global_stores_by_source
        .values()
        .flat_map(|stores| stores.iter())
        .collect::<Vec<_>>();
    let global_slot_names = all_global_stores
        .iter()
        .flat_map(|store| store.slots.iter().map(|slot| slot.name.as_str()))
        .collect::<BTreeSet<_>>();
    let global_action_names = all_global_stores
        .iter()
        .flat_map(|store| store.actions.iter().map(|action| action.name.as_str()))
        .collect::<Vec<_>>();

    for source in sources {
        let source_slots = state_slots_for_source(&source.source_path, &source.source);
        let source_global_stores = global_stores_by_source
            .get(&source.source_path)
            .cloned()
            .unwrap_or_default();
        let source_global_slots = source_global_stores
            .iter()
            .flat_map(|store| store.slots.iter())
            .collect::<Vec<_>>();
        let slot_names = source_slots
            .iter()
            .chain(source_global_slots.iter().copied())
            .map(|slot| slot.name.as_str())
            .chain(global_slot_names.iter().copied())
            .collect::<BTreeSet<_>>();
        let source_local_slot_names = source_slots
            .iter()
            .map(|slot| slot.name.clone())
            .collect::<Vec<_>>();
        derived_slots.extend(derived_slots_for_source(
            &source.source_path,
            &source.source,
            &source_local_slot_names,
        ));
        effects.extend(effect_slots_for_source(
            &source.source_path,
            &source.source,
            &source_local_slot_names,
        ));

        let lowered = lower_react_jsx_source(&source.source_path, &source.source);
        let mut source_event_index = 0usize;
        for element in &lowered.elements {
            for attribute in element
                .attributes
                .iter()
                .filter(|attribute| attribute.name.starts_with("on"))
            {
                let index = source_event_index;
                source_event_index += 1;
                let dom_event = match react_style_event_attribute_to_dom_event(&attribute.name) {
                    Some(event) => event,
                    None => continue,
                };
                if attribute.expression.is_none() && attribute.value.is_some() {
                    continue;
                }
                let handler = attribute
                    .expression
                    .as_deref()
                    .or(attribute.value.as_deref())
                    .unwrap_or_default()
                    .trim()
                    .to_string();
                let action = server_action_names
                    .iter()
                    .find(|name| handler.contains(*name))
                    .map(|name| (*name).to_string())
                    .or_else(|| {
                        global_action_names
                            .iter()
                            .find(|name| handler.contains(**name))
                            .map(|name| (*name).to_string())
                    });
                event_slots.push(DxStateEventSlot {
                    id: format!("event-{}-{}", stable_source_id(&source.source_path), index),
                    source_path: source.source_path.clone(),
                    element: element.name.clone(),
                    event: dom_event,
                    handler: handler.clone(),
                    state_dependencies: slot_names
                        .iter()
                        .filter(|name| handler.contains(**name))
                        .map(|name| (*name).to_string())
                        .collect(),
                    action,
                });
            }
        }

        slots.extend(source_slots);
        for store in &source_global_stores {
            slots.extend(store.slots.clone());
            derived_slots.extend(store.derived_slots.clone());
            effects.extend(store.effects.clone());
        }
        global_stores.extend(source_global_stores);
    }

    let default_scope = if global_stores.is_empty() {
        DxStateScope::Local
    } else {
        DxStateScope::Global
    };

    DxStateGraph {
        default_scope,
        global_stores,
        slots,
        derived_slots,
        event_slots,
        effects,
        server_actions,
    }
}

#[derive(Debug, Clone)]
struct RouteStateSource {
    source_path: String,
    source: String,
}

fn route_state_sources(input: &DxReactAppRouteInput) -> Vec<RouteStateSource> {
    let mut sources = vec![RouteStateSource {
        source_path: input.route_source_path.clone(),
        source: input.route_source.clone(),
    }];
    sources.extend(input.segments.iter().map(|segment| RouteStateSource {
        source_path: segment.source_path.clone(),
        source: segment.source.clone(),
    }));
    sources.extend(input.components.iter().map(|component| RouteStateSource {
        source_path: component.source_path.clone(),
        source: component.source.clone(),
    }));
    sources
}

fn state_slots_for_source(source_path: &str, source: &str) -> Vec<DxStateSlot> {
    react_state_bindings(source)
        .into_iter()
        .enumerate()
        .map(|(index, binding)| DxStateSlot {
            id: format!("state-{}-{index}", stable_source_id(source_path)),
            name: binding.name,
            setter: Some(binding.setter),
            scope: DxStateScope::Local,
            source_path: source_path.to_string(),
            value_kind: binding.value_kind,
            initial_source: binding.initial_source,
        })
        .collect()
}

fn derived_slots_for_source(
    source_path: &str,
    source: &str,
    state_names: &[String],
) -> Vec<DxDerivedStateSlot> {
    if state_names.is_empty() {
        return Vec::new();
    }
    let Ok(re) = regex::Regex::new(r#"\bconst\s+([A-Za-z_$][A-Za-z0-9_$]*)\s*=\s*([^;\n]+)"#)
    else {
        return Vec::new();
    };
    re.captures_iter(source)
        .filter_map(|capture| {
            let name = capture.get(1)?.as_str().to_string();
            if state_names.iter().any(|state| state == &name) {
                return None;
            }
            let expression = capture.get(2)?.as_str().trim().to_string();
            let dependencies = state_names
                .iter()
                .filter(|state| expression_contains_ident(&expression, state))
                .cloned()
                .collect::<Vec<_>>();
            (!dependencies.is_empty()).then(|| DxDerivedStateSlot {
                id: format!("derived-{}-{name}", stable_source_id(source_path)),
                name,
                expression,
                dependencies,
                source_path: source_path.to_string(),
            })
        })
        .collect()
}

fn effect_slots_for_source(
    source_path: &str,
    source: &str,
    state_names: &[String],
) -> Vec<DxStateEffectSlot> {
    if !source.contains("useEffect")
        && !source.contains("useLayoutEffect")
        && !source.contains("useInsertionEffect")
    {
        return Vec::new();
    }
    let dependencies = state_names
        .iter()
        .filter(|state| expression_contains_ident(source, state))
        .cloned()
        .collect::<Vec<_>>();
    vec![DxStateEffectSlot {
        id: format!("effect-{}", stable_source_id(source_path)),
        kind: "effect".to_string(),
        source_path: source_path.to_string(),
        dependencies,
    }]
}

fn server_action_edges(input: &DxReactAppRouteInput) -> Vec<DxServerActionEdge> {
    let mut edges = Vec::new();
    for source in route_state_sources(input) {
        let ast = parse_tsx_module(&source.source_path, &source.source);
        for import in ast.imports {
            if !is_server_action_import(&import.source) {
                continue;
            }
            let mut imported_names = import
                .specifiers
                .iter()
                .filter(|specifier| !specifier.type_only)
                .map(|specifier| specifier.local.clone())
                .collect::<Vec<_>>();
            if let Some(default) = import.default {
                imported_names.push(default);
            }
            for action in imported_names {
                edges.push(DxServerActionEdge {
                    id: format!("action-{}-{action}", stable_source_id(&source.source_path)),
                    action,
                    source_path: source.source_path.clone(),
                    import_source: Some(import.source.clone()),
                    event_id: None,
                });
            }
        }
    }
    edges.sort_by(|left, right| {
        left.source_path
            .cmp(&right.source_path)
            .then(left.action.cmp(&right.action))
    });
    edges.dedup_by(|left, right| {
        left.source_path == right.source_path && left.action == right.action
    });
    edges
}

fn route_receipt(
    input: &DxReactAppRouteInput,
    streaming: &DxReactStreamingProof,
) -> DxRouteReceipt {
    let mut source_paths = vec![input.route_source_path.clone()];
    source_paths.extend(
        input
            .segments
            .iter()
            .map(|segment| segment.source_path.clone()),
    );
    source_paths.extend(
        input
            .components
            .iter()
            .map(|component| component.source_path.clone()),
    );
    source_paths.extend(input.styles.iter().map(|style| style.source_path.clone()));
    source_paths.sort();
    source_paths.dedup();

    let mut forge_package_ids = input
        .components
        .iter()
        .filter_map(|component| component.package_id.clone())
        .collect::<Vec<_>>();
    forge_package_ids.sort();
    forge_package_ids.dedup();

    DxRouteReceipt {
        source_manifest_hash: input.source_manifest_hash.clone(),
        node_modules_required: streaming.node_modules_required,
        client_island_abi: route_client_island_abi_receipt(streaming),
        source_paths,
        forge_package_ids,
    }
}

fn route_client_island_abi_receipt(
    streaming: &DxReactStreamingProof,
) -> Option<DxRouteClientIslandAbiReceipt> {
    if streaming.resumable_islands.is_empty() {
        return None;
    }

    let mut hydration_strategies = streaming
        .resumable_islands
        .iter()
        .map(|island| island.hydration_strategy.clone())
        .collect::<Vec<_>>();
    hydration_strategies.sort();
    hydration_strategies.dedup();
    let framework_adapter_count = streaming
        .resumable_islands
        .iter()
        .filter(|island| island.framework_adapter == "preview-only")
        .count();
    let island_count = streaming.resumable_islands.len();

    Some(DxRouteClientIslandAbiReceipt {
        schema: "dx.route.clientIslandAbiReceipt".to_string(),
        schema_revision: 1,
        directive_style_id: "camelCase-jsx-props".to_string(),
        core_directives: vec![
            "clientLoad".to_string(),
            "clientVisible".to_string(),
            "clientIdle".to_string(),
            "clientOnly".to_string(),
        ],
        source_owned_runtime: true,
        node_modules_required: streaming.node_modules_required,
        full_react_hydration: false,
        no_js_fallback_required: true,
        island_count,
        source_owned_island_count: island_count.saturating_sub(framework_adapter_count),
        framework_adapter_count,
        client_only_adapter_count: framework_adapter_count,
        hydration_strategies,
        browser_proof_status: "foundation-not-release-proof".to_string(),
        proof_scope: "local-source-owned-island-abi-foundation".to_string(),
        release_ready: false,
        fastest_world_claim: false,
    })
}

fn adaptive_runtime_report(
    delivery_mode: DxDeliveryMode,
    fallback: &DxFallbackHtml,
    generated_styles: &[DxReactGeneratedStyleAsset],
    streaming: &DxReactStreamingProof,
    packet: &DxReactAppRoutePacketProof,
    state: &DxStateGraph,
    unsupported_react_event_attributes: &[String],
) -> DxAdaptiveRuntimeReport {
    let style_bytes = generated_styles
        .iter()
        .map(|asset| asset.bytes)
        .sum::<usize>();
    let static_bytes = fallback.bytes + style_bytes;
    let tiny_static_route_proof = tiny_static_route_proof(
        delivery_mode,
        fallback,
        style_bytes,
        streaming.enabled,
        state,
    );
    let interaction_weight = state.event_slots.len() * 48 + state.slots.len() * 18;
    let micro_js_bytes = static_bytes
        + if interaction_weight == 0 {
            0
        } else {
            96 + interaction_weight
        };
    let wasm_core_bytes = static_bytes + packet.bytes + 7_500;
    let wasm_split_bytes = static_bytes + packet.bytes + 1_500;
    let server_fragment_bytes = static_bytes
        + streaming
            .deferred_chunks
            .iter()
            .map(|chunk| chunk.bytes)
            .sum::<usize>()
        + 320;

    let candidates = vec![
        runtime_candidate(
            DxDeliveryMode::Static,
            static_bytes,
            delivery_mode,
            "HTML/CSS shell with no browser runtime",
        ),
        runtime_candidate(
            DxDeliveryMode::MicroJs,
            micro_js_bytes,
            delivery_mode,
            "generated event/state JavaScript for tiny islands",
        ),
        runtime_candidate(
            DxDeliveryMode::ServerFragment,
            server_fragment_bytes,
            delivery_mode,
            "server-rendered fragment endpoint path",
        ),
        runtime_candidate(
            DxDeliveryMode::WasmCore,
            wasm_core_bytes,
            delivery_mode,
            "DXPK plus shared WASM core runtime",
        ),
        runtime_candidate(
            DxDeliveryMode::WasmSplit,
            wasm_split_bytes,
            delivery_mode,
            "DXPK plus split boot runtime",
        ),
    ];
    let rejected_modes = candidates
        .iter()
        .filter(|candidate| !candidate.selected)
        .map(|candidate| DxRuntimeRejection {
            mode: candidate.mode,
            estimated_bytes: candidate.estimated_bytes,
            reason: if candidate.mode == DxDeliveryMode::Static && !state.event_slots.is_empty() {
                "route has event/state slots, so a pure static shell would lose behavior"
                    .to_string()
            } else if candidate.estimated_bytes > selected_estimate(&candidates, delivery_mode) {
                "estimated bytes are higher than the selected route mode".to_string()
            } else {
                "candidate does not satisfy the current interaction contract".to_string()
            },
        })
        .collect::<Vec<_>>();
    let mut reasons = vec![format!(
        "selected {} from {} state slot(s), {} event slot(s), {} server action edge(s)",
        delivery_mode.as_str(),
        state.slots.len(),
        state.event_slots.len(),
        state.server_actions.len()
    )];
    if delivery_mode == DxDeliveryMode::Static {
        reasons.push("no client state or event slots require browser runtime".to_string());
        reasons.push(
            "source-only no-JS shell; Astro parity is not claimed by route-unit proof".to_string(),
        );
    } else if delivery_mode == DxDeliveryMode::MicroJs {
        reasons.push("small local state/event graph can avoid the WASM core runtime".to_string());
    }
    let mut warnings = vec![
        "runtime bytes, DXPK bytes, generated CSS, and shell bytes must be counted together"
            .to_string(),
    ];
    if tiny_static_route_proof.no_js_capable {
        warnings.push(
            "tier-0 no-JS proof is source-only; Astro parity requires a separate live payload/paint/throughput receipt"
                .to_string(),
        );
    }
    warnings.extend(unsupported_react_event_attributes.iter().map(|attribute| {
        format!(
            "unsupported React-style event attribute {attribute}; no listener is attached and React synthetic event parity is not claimed"
        )
    }));

    DxAdaptiveRuntimeReport {
        selected_mode: delivery_mode,
        tiny_static_route_proof,
        candidates,
        rejected_modes,
        reasons,
        warnings,
    }
}

fn tiny_static_route_proof(
    delivery_mode: DxDeliveryMode,
    fallback: &DxFallbackHtml,
    css_bytes: usize,
    streaming_enabled: bool,
    state: &DxStateGraph,
) -> DxTinyStaticRouteProof {
    let lowercase_html = fallback.html.to_ascii_lowercase();
    let script_tag_count = lowercase_html.matches("<script").count();
    let browser_api_required = delivery_mode != DxDeliveryMode::Static
        || !state.slots.is_empty()
        || !state.event_slots.is_empty()
        || !state.effects.is_empty()
        || !state.server_actions.is_empty()
        || streaming_enabled;
    let meaningful_html = tiny_static_has_meaningful_html(&fallback.html);
    let semantic_landmark_present = tiny_static_semantic_landmark_present(&lowercase_html);
    let link_count = tiny_static_tag_count(&lowercase_html, "a");
    let form_count = tiny_static_tag_count(&lowercase_html, "form");
    let seo_title_present = lowercase_html.contains("<title")
        || lowercase_html.contains("property=\"og:title\"")
        || lowercase_html.contains("name=\"twitter:title\"");
    let accessibility_signal_count = tiny_static_accessibility_signal_count(&lowercase_html);
    let no_js_capable = delivery_mode == DxDeliveryMode::Static
        && !browser_api_required
        && script_tag_count == 0
        && meaningful_html;

    DxTinyStaticRouteProof {
        selected: delivery_mode == DxDeliveryMode::Static,
        output_mode: if delivery_mode == DxDeliveryMode::Static {
            "tiny-static".to_string()
        } else {
            delivery_mode.as_str().to_string()
        },
        js: if no_js_capable {
            "none".to_string()
        } else {
            "required-or-fallback-only".to_string()
        },
        html_bytes: fallback.bytes,
        css_bytes,
        total_public_bytes: fallback.bytes + css_bytes,
        script_tag_count,
        runtime_required: delivery_mode != DxDeliveryMode::Static,
        no_js_capable,
        meaningful_html,
        semantic_landmark_present,
        link_count,
        form_count,
        seo_title_present,
        accessibility_signal_count,
        browser_api_required,
        astro_parity_status: "not_yet_claimed".to_string(),
    }
}

fn tiny_static_has_meaningful_html(html: &str) -> bool {
    let has_semantic_shell = html.contains("<main")
        && (html.contains("<h1")
            || html.contains("<p")
            || html.contains("<a ")
            || html.contains("<article")
            || html.contains("<section"));
    has_semantic_shell && html_has_visible_text(html)
}

fn tiny_static_semantic_landmark_present(html_lower: &str) -> bool {
    [
        "<main", "<article", "<section", "<nav", "<header", "<footer",
    ]
    .iter()
    .any(|marker| html_lower.contains(marker))
}

fn tiny_static_tag_count(html_lower: &str, tag_name: &str) -> usize {
    html_lower.matches(&format!("<{tag_name}")).count()
}

fn tiny_static_accessibility_signal_count(html_lower: &str) -> usize {
    [
        "<main",
        "<h1",
        "<nav",
        " aria-label=",
        " aria-labelledby=",
        " alt=",
        " role=",
    ]
    .iter()
    .filter(|marker| html_lower.contains(**marker))
    .count()
}

fn html_has_visible_text(html: &str) -> bool {
    let mut in_tag = false;
    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag && !ch.is_whitespace() => return true,
            _ => {}
        }
    }
    false
}

fn runtime_candidate(
    mode: DxDeliveryMode,
    estimated_bytes: usize,
    selected_mode: DxDeliveryMode,
    reason: &str,
) -> DxRuntimeCandidate {
    DxRuntimeCandidate {
        mode,
        estimated_bytes,
        selected: mode == selected_mode,
        reason: reason.to_string(),
    }
}

fn selected_estimate(candidates: &[DxRuntimeCandidate], selected: DxDeliveryMode) -> usize {
    candidates
        .iter()
        .find(|candidate| candidate.mode == selected)
        .map(|candidate| candidate.estimated_bytes)
        .unwrap_or(usize::MAX)
}

fn is_server_action_import(source: &str) -> bool {
    source.contains("/server/")
        || source.starts_with("../server")
        || source.starts_with("../../server")
        || source.starts_with("@/server")
        || source == "server/actions"
}

fn expression_contains_ident(expression: &str, ident: &str) -> bool {
    let Ok(re) = regex::Regex::new(&format!(r#"\b{}\b"#, regex::escape(ident))) else {
        return false;
    };
    re.is_match(expression)
}

fn stable_source_id(source_path: &str) -> String {
    blake3::hash(source_path.as_bytes())
        .to_hex()
        .chars()
        .take(12)
        .collect()
}
