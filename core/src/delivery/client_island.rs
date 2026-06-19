use serde::{Deserialize, Serialize};

use super::app_route::{DxReactAppSegmentSource, DxReactComponentSource};
use super::client_boundary::{DxReactClientSource, analyze_react_client_boundaries};
use super::dom_events::react_style_event_attribute_to_dom_event;
use super::jsx_lowering::{
    DxReactJsxAttribute, DxReactJsxDocument, DxReactJsxElement, DxReactJsxKeyHint,
    lower_react_jsx_source,
};
use super::micro_js::DxMicroJsEmitter;
use super::react_state::{ReactStateBinding, react_state_bindings};
use super::types::{DxDeliveryMode, DxMicroJsAction, DxMicroJsOp, DxMicroJsProgram};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DxReactClientIslandInput {
    pub route: String,
    pub route_source_path: String,
    pub route_source: String,
    pub segments: Vec<DxReactAppSegmentSource>,
    pub components: Vec<DxReactComponentSource>,
    pub route_delivery_mode: DxDeliveryMode,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DxReactClientIslandManifest {
    pub version: u32,
    pub route: String,
    pub runtime: String,
    pub abi: DxReactClientIslandAbi,
    pub node_modules_required: bool,
    pub deterministic: bool,
    pub islands: Vec<DxReactClientIsland>,
    pub dynamic_imports: Vec<DxReactClientIslandDynamicImport>,
    pub hydration_runtime: Option<DxReactClientIslandHydrationRuntime>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactClientIslandAbi {
    pub schema: String,
    pub schema_revision: u32,
    pub directive_style: String,
    pub supported_directives: Vec<String>,
    pub unsupported_directive_syntax: Vec<String>,
    pub source_owned_runtime: bool,
    pub node_modules_required: bool,
    pub full_react_hydration: bool,
    pub no_js_fallback_required: bool,
    pub island_count: usize,
    pub source_owned_island_count: usize,
    pub framework_adapter_count: usize,
    pub client_load_count: usize,
    pub client_visible_count: usize,
    pub client_idle_count: usize,
    pub client_only_count: usize,
    pub client_media_count: usize,
    pub client_interaction_count: usize,
    pub dynamic_import_count: usize,
    pub explicit_frameworks: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactClientIslandAbiCapabilities {
    pub schema: String,
    pub schema_revision: u32,
    pub compiler_abi_schema: String,
    pub directive_style: String,
    pub directive_style_id: String,
    pub supported_directives: Vec<String>,
    pub unsupported_directive_syntax: Vec<String>,
    pub source_owned_runtime: bool,
    pub node_modules_required: bool,
    pub full_react_hydration: bool,
    pub no_js_fallback_required: bool,
    pub adapter_boundary_required: Vec<String>,
    pub explicit_frameworks_policy: String,
    pub readiness_release_ready: bool,
    pub browser_proof_status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DxReactClientIsland {
    pub id: String,
    pub source_path: String,
    pub source_kind: String,
    pub package_id: Option<String>,
    pub directives: Vec<DxReactClientIslandDirective>,
    pub use_client: bool,
    pub delivery_mode: DxDeliveryMode,
    pub event_handlers: usize,
    pub state_vars: usize,
    pub keyed_updates: Vec<DxReactClientIslandKeyedUpdate>,
    pub state: Vec<DxReactClientIslandState>,
    pub events: Vec<DxReactClientIslandEvent>,
    pub hydration: DxReactClientIslandHydration,
    pub micro_js: Option<DxReactClientIslandMicroJs>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DxReactClientIslandHydration {
    pub deterministic: bool,
    pub strategy: String,
    pub directives: Vec<DxReactClientIslandDirective>,
    pub props: Vec<DxReactClientIslandProp>,
    pub events: Vec<DxReactClientIslandHydrationEvent>,
    pub forms: Vec<DxReactClientIslandForm>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactClientIslandDirective {
    pub name: String,
    pub source: String,
    pub value: Option<String>,
    pub expression: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactClientIslandProp {
    pub name: String,
    pub source: String,
    pub value: Option<String>,
    pub expression: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactClientIslandHydrationEvent {
    pub event_id: String,
    pub element: String,
    pub event: String,
    pub handler: String,
    pub state: Option<String>,
    pub operation: Option<String>,
    pub form_id: Option<String>,
    pub prevent_default: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactClientIslandForm {
    pub form_id: String,
    pub submit_event: Option<String>,
    pub fields: Vec<DxReactClientIslandFormField>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactClientIslandFormField {
    pub name: String,
    pub input_type: Option<String>,
    pub value_state: Option<String>,
    pub change_event: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactClientIslandDynamicImport {
    pub source_path: String,
    pub source: String,
    pub chunk_id: String,
    pub preload: bool,
    pub ssr: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactClientIslandHydrationRuntime {
    pub source_owned: bool,
    pub deterministic: bool,
    pub event_count: usize,
    pub form_count: usize,
    pub dynamic_import_count: usize,
    pub script: String,
    pub script_hash: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DxReactClientIslandState {
    pub name: String,
    pub setter: String,
    pub initial_source: String,
    pub initial_value: Option<i64>,
    pub value_kind: String,
    pub target_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DxReactClientIslandEvent {
    pub element: String,
    pub attribute: String,
    pub event: String,
    pub handler: String,
    pub element_id: String,
    pub operation: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DxReactClientIslandKeyedUpdate {
    pub element: String,
    pub value: Option<String>,
    pub expression: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DxReactClientIslandMicroJs {
    pub deterministic: bool,
    pub program_id: String,
    pub target_id: String,
    pub action_count: usize,
    pub script: String,
    pub script_hash: String,
}

struct ClientIslandCandidate {
    source_path: String,
    source: String,
    source_kind: String,
    package_id: Option<String>,
    component_name: Option<String>,
    props: Vec<DxReactClientIslandProp>,
    directives: Vec<DxReactClientIslandDirective>,
}

type StateBinding = ReactStateBinding;

pub fn compile_react_client_islands(
    input: DxReactClientIslandInput,
) -> DxReactClientIslandManifest {
    let dynamic_imports = client_dynamic_imports(&input);
    let mut islands = client_island_candidates(&input)
        .into_iter()
        .filter_map(compile_client_island)
        .collect::<Vec<_>>();
    islands.sort_by(|left, right| left.source_path.cmp(&right.source_path));

    let runtime = islands
        .iter()
        .map(|island| island.delivery_mode)
        .find(|mode| *mode == DxDeliveryMode::WasmCore)
        .or_else(|| {
            islands
                .iter()
                .map(|island| island.delivery_mode)
                .find(|mode| *mode == DxDeliveryMode::MicroJs)
        })
        .unwrap_or(input.route_delivery_mode)
        .as_str()
        .to_string();
    let hydration_runtime = (!islands.is_empty())
        .then(|| client_hydration_runtime(&input.route, &islands, &dynamic_imports));
    let abi = client_island_abi(&islands, &dynamic_imports);

    DxReactClientIslandManifest {
        version: 1,
        route: input.route,
        runtime,
        abi,
        node_modules_required: false,
        deterministic: true,
        islands,
        dynamic_imports,
        hydration_runtime,
    }
}

pub fn react_client_island_abi_capabilities() -> DxReactClientIslandAbiCapabilities {
    DxReactClientIslandAbiCapabilities {
        schema: "dx.react.clientIsland.abi.capabilities".to_string(),
        schema_revision: 1,
        compiler_abi_schema: "dx.react.clientIsland.abi".to_string(),
        directive_style: "camelCase-jsx-props".to_string(),
        directive_style_id: "camelCase-jsx-props".to_string(),
        supported_directives: client_island_directive_names()
            .iter()
            .map(|directive| (*directive).to_string())
            .collect(),
        unsupported_directive_syntax: unsupported_client_island_directive_syntax()
            .iter()
            .map(|directive| (*directive).to_string())
            .collect(),
        source_owned_runtime: true,
        node_modules_required: false,
        full_react_hydration: false,
        no_js_fallback_required: true,
        adapter_boundary_required: vec![
            "explicit clientOnly framework runtime".to_string(),
            "arbitrary React hydration semantics".to_string(),
            "Svelte/Vue/other framework component execution".to_string(),
            "third-party island runtime imports".to_string(),
        ],
        explicit_frameworks_policy:
            "framework adapters are preview-only and only allowed through explicit clientOnly directives until executable framework adapter receipts exist".to_string(),
        readiness_release_ready: false,
        browser_proof_status: "foundation-not-release-proof".to_string(),
    }
}

fn client_island_abi(
    islands: &[DxReactClientIsland],
    dynamic_imports: &[DxReactClientIslandDynamicImport],
) -> DxReactClientIslandAbi {
    let capabilities = react_client_island_abi_capabilities();
    let directive_count = |name: &str| {
        islands
            .iter()
            .flat_map(|island| island.directives.iter())
            .filter(|directive| directive.name == name)
            .count()
    };
    let mut explicit_frameworks = islands
        .iter()
        .flat_map(|island| island.directives.iter())
        .filter(|directive| directive.name == "clientOnly")
        .filter_map(|directive| {
            directive
                .value
                .clone()
                .or_else(|| directive.expression.clone())
        })
        .collect::<Vec<_>>();
    explicit_frameworks.sort();
    explicit_frameworks.dedup();

    let framework_adapter_count = directive_count("clientOnly");

    DxReactClientIslandAbi {
        schema: capabilities.compiler_abi_schema,
        schema_revision: capabilities.schema_revision,
        directive_style: capabilities.directive_style,
        supported_directives: capabilities.supported_directives,
        unsupported_directive_syntax: capabilities.unsupported_directive_syntax,
        source_owned_runtime: capabilities.source_owned_runtime,
        node_modules_required: capabilities.node_modules_required,
        full_react_hydration: capabilities.full_react_hydration,
        no_js_fallback_required: capabilities.no_js_fallback_required,
        island_count: islands.len(),
        source_owned_island_count: islands.len().saturating_sub(framework_adapter_count),
        framework_adapter_count,
        client_load_count: directive_count("clientLoad"),
        client_visible_count: directive_count("clientVisible"),
        client_idle_count: directive_count("clientIdle"),
        client_only_count: framework_adapter_count,
        client_media_count: directive_count("clientMedia"),
        client_interaction_count: directive_count("clientInteraction"),
        dynamic_import_count: dynamic_imports.len(),
        explicit_frameworks,
    }
}

pub fn react_client_island_micro_js_bundle(
    manifest: &DxReactClientIslandManifest,
) -> Option<String> {
    let scripts = manifest
        .islands
        .iter()
        .filter_map(|island| island.micro_js.as_ref())
        .map(|micro_js| micro_js.script.as_str())
        .collect::<Vec<_>>();
    if let Some(runtime) = manifest.hydration_runtime.as_ref() {
        let mut bundle = runtime.script.clone();
        if !scripts.is_empty() {
            bundle.push('\n');
            bundle.push_str(&scripts.join(";\n"));
        }
        return Some(bundle);
    }
    if scripts.is_empty() {
        return None;
    }
    Some(format!(
        "/* www client islands v{} route={} */\n{}",
        manifest.version,
        manifest.route,
        scripts.join(";\n")
    ))
}

fn client_island_candidates(input: &DxReactClientIslandInput) -> Vec<ClientIslandCandidate> {
    let route_doc = lower_react_jsx_source(&input.route_source_path, &input.route_source);
    let segment_docs = input
        .segments
        .iter()
        .map(|segment| lower_react_jsx_source(&segment.source_path, &segment.source))
        .collect::<Vec<_>>();
    let mut candidates = vec![ClientIslandCandidate {
        source_path: input.route_source_path.clone(),
        source: input.route_source.clone(),
        source_kind: "route".to_string(),
        package_id: None,
        component_name: None,
        props: Vec::new(),
        directives: Vec::new(),
    }];
    candidates.extend(input.segments.iter().map(|segment| ClientIslandCandidate {
        source_path: segment.source_path.clone(),
        source: segment.source.clone(),
        source_kind: "segment".to_string(),
        package_id: None,
        component_name: None,
        props: Vec::new(),
        directives: Vec::new(),
    }));
    candidates.extend(input.components.iter().filter_map(|component| {
        let referenced = source_references_component(&route_doc, &component.name)
            || segment_docs
                .iter()
                .any(|doc| source_references_component(doc, &component.name));
        referenced.then(|| ClientIslandCandidate {
            source_path: component.source_path.clone(),
            source: component.source.clone(),
            source_kind: "component".to_string(),
            package_id: component.package_id.clone(),
            component_name: Some(component.name.clone()),
            props: component_route_props(&route_doc, &segment_docs, &component.name),
            directives: component_route_directives(&route_doc, &segment_docs, &component.name),
        })
    }));
    candidates
}

fn compile_client_island(candidate: ClientIslandCandidate) -> Option<DxReactClientIsland> {
    let source = DxReactClientSource {
        source_path: candidate.source_path.clone(),
        source: candidate.source.clone(),
    };
    let boundary = analyze_react_client_boundaries(&[source])
        .into_iter()
        .next()?;
    let lowered = lower_react_jsx_source(&candidate.source_path, &candidate.source);
    let id = client_island_id(&candidate.source_path);
    let state = state_bindings(&candidate.source)
        .into_iter()
        .map(|binding| DxReactClientIslandState {
            target_id: state_target_id(&id, &binding.name),
            name: binding.name,
            setter: binding.setter,
            initial_source: binding.initial_source,
            initial_value: binding.initial_value,
            value_kind: binding.value_kind,
        })
        .collect::<Vec<_>>();
    let raw_state = state_bindings(&candidate.source);
    let events = lowered
        .event_attributes
        .iter()
        .enumerate()
        .filter_map(|(index, event)| {
            let dom_event = react_style_event_attribute_to_dom_event(&event.name)?;
            let operation = raw_state.iter().find_map(|state| {
                infer_micro_op(&candidate.source, &event.expression, state).map(micro_op_label)
            });
            Some(DxReactClientIslandEvent {
                element: event.element.clone(),
                attribute: event.name.clone(),
                event: dom_event.clone(),
                handler: normalize_handler(&event.expression),
                element_id: format!("{id}-event-{dom_event}-{index}"),
                operation,
            })
        })
        .collect::<Vec<_>>();
    let micro_js = if boundary.delivery_mode == DxDeliveryMode::MicroJs {
        compile_micro_js(&id, &candidate.source, &raw_state, &events)
    } else {
        None
    };
    let hydration = compile_hydration_contract(
        &candidate,
        &id,
        &lowered,
        &raw_state,
        &events,
        &candidate.directives,
        boundary.delivery_mode,
    );

    Some(DxReactClientIsland {
        id,
        source_path: candidate.source_path,
        source_kind: candidate.source_kind,
        package_id: candidate.package_id,
        directives: candidate.directives,
        use_client: boundary.use_client,
        delivery_mode: boundary.delivery_mode,
        event_handlers: boundary.event_handlers,
        state_vars: boundary.state_vars,
        keyed_updates: keyed_updates(&lowered.keyed_update_hints),
        state,
        events,
        hydration,
        micro_js,
    })
}

fn compile_hydration_contract(
    candidate: &ClientIslandCandidate,
    island_id: &str,
    lowered: &DxReactJsxDocument,
    state: &[StateBinding],
    events: &[DxReactClientIslandEvent],
    directives: &[DxReactClientIslandDirective],
    delivery_mode: DxDeliveryMode,
) -> DxReactClientIslandHydration {
    let base_name = candidate
        .component_name
        .clone()
        .or_else(|| exported_component_name(&candidate.source))
        .unwrap_or_else(|| island_id.to_string())
        .to_ascii_lowercase();
    let events = hydration_events(&candidate.source, state, events, &base_name);
    let forms = hydration_forms(lowered, state, &events, &base_name);
    DxReactClientIslandHydration {
        deterministic: true,
        strategy: hydration_strategy(delivery_mode, directives).to_string(),
        directives: directives.to_vec(),
        props: candidate.props.clone(),
        events,
        forms,
    }
}

fn hydration_strategy(
    delivery_mode: DxDeliveryMode,
    directives: &[DxReactClientIslandDirective],
) -> &'static str {
    if directives
        .iter()
        .any(|directive| directive.name == "clientOnly")
    {
        return "framework-adapter-client-only";
    }
    if directives
        .iter()
        .any(|directive| directive.name == "clientVisible")
    {
        return "visible";
    }
    if directives
        .iter()
        .any(|directive| directive.name == "clientIdle")
    {
        return "idle";
    }
    if directives
        .iter()
        .any(|directive| directive.name == "clientLoad")
    {
        return "load";
    }
    if directives
        .iter()
        .any(|directive| directive.name == "clientMedia")
    {
        return "media-recognized-not-executed";
    }
    if directives
        .iter()
        .any(|directive| directive.name == "clientInteraction")
    {
        return "interaction-recognized-not-executed";
    }
    match delivery_mode {
        DxDeliveryMode::Static => "static",
        DxDeliveryMode::MicroJs => "js-event-replay",
        DxDeliveryMode::WasmCore | DxDeliveryMode::WasmSplit => "wasm-core-resume",
        _ => "source-owned-resume",
    }
}

fn hydration_events(
    source: &str,
    state: &[StateBinding],
    events: &[DxReactClientIslandEvent],
    form_base: &str,
) -> Vec<DxReactClientIslandHydrationEvent> {
    let mut form_index = 0usize;
    events
        .iter()
        .map(|event| {
            let body =
                handler_body(source, &event.handler).unwrap_or_else(|| event.handler.clone());
            let state_binding = state.iter().find(|state| body.contains(&state.setter));
            let form_id = if event.element == "form" {
                let id = format!("{form_base}-form-{form_index}");
                form_index += 1;
                Some(id)
            } else {
                None
            };
            DxReactClientIslandHydrationEvent {
                event_id: event.element_id.clone(),
                element: event.element.clone(),
                event: event.event.clone(),
                handler: event.handler.clone(),
                state: state_binding.map(|state| state.name.clone()),
                operation: hydration_operation(&body, state_binding, event.operation.as_deref()),
                form_id,
                prevent_default: body.contains(".preventDefault("),
            }
        })
        .collect()
}

fn hydration_operation(
    body: &str,
    state: Option<&StateBinding>,
    fallback: Option<&str>,
) -> Option<String> {
    if state.is_some() && body.contains("event.target.value") {
        return Some("set-from-input".to_string());
    }
    fallback.map(str::to_string)
}

fn hydration_forms(
    lowered: &DxReactJsxDocument,
    state: &[StateBinding],
    events: &[DxReactClientIslandHydrationEvent],
    form_base: &str,
) -> Vec<DxReactClientIslandForm> {
    let form_count = lowered
        .elements
        .iter()
        .filter(|element| element.name == "form")
        .count();
    (0..form_count)
        .map(|index| {
            let form_id = format!("{form_base}-form-{index}");
            let submit_event = events
                .iter()
                .find(|event| {
                    event.event == "submit" && event.form_id.as_deref() == Some(form_id.as_str())
                })
                .map(|event| event.event_id.clone());
            DxReactClientIslandForm {
                form_id,
                submit_event,
                fields: hydration_form_fields(lowered, state, events),
            }
        })
        .collect()
}

fn hydration_form_fields(
    lowered: &DxReactJsxDocument,
    state: &[StateBinding],
    events: &[DxReactClientIslandHydrationEvent],
) -> Vec<DxReactClientIslandFormField> {
    let mut change_events = events
        .iter()
        .filter(|event| event.event == "change" || event.event == "input")
        .collect::<Vec<_>>();
    lowered
        .elements
        .iter()
        .filter(|element| matches!(element.name.as_str(), "input" | "textarea" | "select"))
        .enumerate()
        .map(|(index, element)| {
            let value_expression = element_expression(element, "value");
            let value_state = value_expression.as_deref().and_then(|expression| {
                state
                    .iter()
                    .find(|state| state.name == expression)
                    .map(|state| state.name.clone())
            });
            let change_event = change_events
                .iter()
                .position(|event| event.element == element.name)
                .map(|position| change_events.remove(position).event_id.clone());
            DxReactClientIslandFormField {
                name: element
                    .attribute("name")
                    .map(str::to_string)
                    .unwrap_or_else(|| format!("field-{index}")),
                input_type: element.attribute("type").map(str::to_string),
                value_state,
                change_event,
            }
        })
        .collect()
}

fn compile_micro_js(
    island_id: &str,
    source: &str,
    state: &[StateBinding],
    events: &[DxReactClientIslandEvent],
) -> Option<DxReactClientIslandMicroJs> {
    let first_state = state.first()?;
    let target_id = state_target_id(island_id, &first_state.name);
    let actions = events
        .iter()
        .filter_map(|event| {
            let (state, op) = infer_micro_action(source, &event.handler, state)?;
            Some(DxMicroJsAction {
                element_id: event.element_id.clone(),
                event: event.event.clone(),
                target_id: Some(state_target_id(island_id, &state.name)),
                initial_value: state.initial_value,
                op,
            })
        })
        .collect::<Vec<_>>();
    if actions.is_empty() {
        return None;
    }
    let program = DxMicroJsProgram {
        initial_value: first_state.initial_value.unwrap_or_default(),
        target_id: target_id.clone(),
        actions,
    };
    let script = DxMicroJsEmitter::emit(&program);
    Some(DxReactClientIslandMicroJs {
        deterministic: true,
        program_id: format!("{island_id}-js"),
        target_id,
        action_count: program.actions.len(),
        script_hash: short_hash(&script),
        script,
    })
}

fn state_bindings(source: &str) -> Vec<StateBinding> {
    react_state_bindings(source)
}

fn infer_micro_action<'a>(
    source: &str,
    handler: &str,
    state: &'a [StateBinding],
) -> Option<(&'a StateBinding, DxMicroJsOp)> {
    state
        .iter()
        .find_map(|state| infer_micro_op(source, handler, state).map(|op| (state, op)))
}

fn infer_micro_op(source: &str, handler: &str, state: &StateBinding) -> Option<DxMicroJsOp> {
    let body = handler_body(source, handler).unwrap_or_else(|| handler.to_string());
    let compact = body
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .collect::<String>();
    if let Some(op) = infer_functional_update(&compact, state) {
        return Some(op);
    }
    if let Some(op) = infer_named_update(&compact, state) {
        return Some(op);
    }
    let Ok(set_re) = regex::Regex::new(&format!(
        r#"\b{}\s*\(\s*(-?\d+|true|false)\s*\)"#,
        regex::escape(&state.setter)
    )) else {
        return None;
    };
    set_re
        .captures(&compact)
        .and_then(|capture| capture.get(1))
        .and_then(|value| match value.as_str() {
            "true" => Some(1),
            "false" => Some(0),
            other => other.parse::<i64>().ok(),
        })
        .map(DxMicroJsOp::Set)
}

fn infer_functional_update(compact: &str, state: &StateBinding) -> Option<DxMicroJsOp> {
    let Ok(delta_re) = regex::Regex::new(&format!(
        r#"\b{}\(\(?([A-Za-z_$][A-Za-z0-9_$]*)\)?=>([A-Za-z_$][A-Za-z0-9_$]*)([+-])(\d+)\)"#,
        regex::escape(&state.setter)
    )) else {
        return None;
    };
    if let Some(capture) = delta_re.captures(compact) {
        if capture.get(1)?.as_str() != capture.get(2)?.as_str() {
            return None;
        }
        let sign = capture.get(3)?.as_str();
        let amount = capture.get(4)?.as_str().parse::<i64>().ok()?;
        return Some(DxMicroJsOp::Add(if sign == "-" { -amount } else { amount }));
    }
    let Ok(toggle_re) = regex::Regex::new(&format!(
        r#"\b{}\(\(?([A-Za-z_$][A-Za-z0-9_$]*)\)?=>!([A-Za-z_$][A-Za-z0-9_$]*)\)"#,
        regex::escape(&state.setter)
    )) else {
        return None;
    };
    toggle_re.captures(compact).and_then(|capture| {
        (capture.get(1)?.as_str() == capture.get(2)?.as_str()).then_some(DxMicroJsOp::Toggle)
    })
}

fn infer_named_update(compact: &str, state: &StateBinding) -> Option<DxMicroJsOp> {
    let Ok(delta_re) = regex::Regex::new(&format!(
        r#"\b{}\({}([+-])(\d+)\)"#,
        regex::escape(&state.setter),
        regex::escape(&state.name)
    )) else {
        return None;
    };
    if let Some(capture) = delta_re.captures(compact) {
        let sign = capture.get(1)?.as_str();
        let amount = capture.get(2)?.as_str().parse::<i64>().ok()?;
        return Some(DxMicroJsOp::Add(if sign == "-" { -amount } else { amount }));
    }
    let direct_toggle = format!("{}(!{})", state.setter, state.name);
    compact
        .contains(&direct_toggle)
        .then_some(DxMicroJsOp::Toggle)
}

fn handler_body(source: &str, handler: &str) -> Option<String> {
    let handler = normalize_handler(handler);
    if handler.contains("=>") || handler.contains('(') || handler.is_empty() {
        return Some(handler);
    }
    let marker = format!("function {handler}");
    let start = source.find(&marker)?;
    let brace = source[start..].find('{').map(|offset| start + offset)?;
    let end = find_balanced_brace(source, brace)?;
    Some(source[brace + 1..end].to_string())
}

fn find_balanced_brace(source: &str, start: usize) -> Option<usize> {
    let mut depth = 0usize;
    let mut quote = None;
    let mut cursor = start;
    while cursor < source.len() {
        let ch = source[cursor..].chars().next()?;
        if let Some(active_quote) = quote {
            if ch == active_quote {
                quote = None;
            }
            cursor += ch.len_utf8();
            continue;
        }
        match ch {
            '"' | '\'' | '`' => quote = Some(ch),
            '{' => depth += 1,
            '}' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some(cursor);
                }
            }
            _ => {}
        }
        cursor += ch.len_utf8();
    }
    None
}

fn keyed_updates(hints: &[DxReactJsxKeyHint]) -> Vec<DxReactClientIslandKeyedUpdate> {
    hints
        .iter()
        .map(|hint| DxReactClientIslandKeyedUpdate {
            element: hint.element.clone(),
            value: hint.value.clone(),
            expression: hint.expression.clone(),
        })
        .collect()
}

fn component_route_props(
    route_doc: &DxReactJsxDocument,
    segment_docs: &[DxReactJsxDocument],
    component_name: &str,
) -> Vec<DxReactClientIslandProp> {
    let mut props = component_props_from_doc(route_doc, component_name, "route-prop");
    for doc in segment_docs {
        props.extend(component_props_from_doc(
            doc,
            component_name,
            "segment-prop",
        ));
    }
    props.sort_by(|left, right| {
        left.name
            .cmp(&right.name)
            .then(left.source.cmp(&right.source))
    });
    props.dedup_by(|left, right| left.name == right.name && left.source == right.source);
    props
}

fn component_route_directives(
    route_doc: &DxReactJsxDocument,
    segment_docs: &[DxReactJsxDocument],
    component_name: &str,
) -> Vec<DxReactClientIslandDirective> {
    let mut directives =
        component_directives_from_doc(route_doc, component_name, "route-directive");
    for doc in segment_docs {
        directives.extend(component_directives_from_doc(
            doc,
            component_name,
            "segment-directive",
        ));
    }
    directives.sort_by(|left, right| {
        left.name
            .cmp(&right.name)
            .then(left.source.cmp(&right.source))
    });
    directives.dedup_by(|left, right| left.name == right.name && left.source == right.source);
    directives
}

fn component_props_from_doc(
    doc: &DxReactJsxDocument,
    component_name: &str,
    source: &str,
) -> Vec<DxReactClientIslandProp> {
    doc.elements
        .iter()
        .filter(|element| element.name == component_name)
        .flat_map(|element| {
            element.attributes.iter().filter_map(|attribute| {
                if attribute.name == "key"
                    || attribute.name.starts_with("on")
                    || is_client_island_directive(&attribute.name)
                    || is_unsupported_client_island_directive_syntax(&attribute.name)
                {
                    return None;
                }
                Some(DxReactClientIslandProp {
                    name: attribute.name.clone(),
                    source: source.to_string(),
                    value: attribute.value.clone(),
                    expression: attribute.expression.clone(),
                })
            })
        })
        .collect()
}

fn component_directives_from_doc(
    doc: &DxReactJsxDocument,
    component_name: &str,
    source: &str,
) -> Vec<DxReactClientIslandDirective> {
    doc.elements
        .iter()
        .filter(|element| element.name == component_name)
        .flat_map(|element| {
            element
                .attributes
                .iter()
                .filter(|&attribute| is_client_island_directive(&attribute.name))
                .map(|attribute| DxReactClientIslandDirective {
                    name: attribute.name.clone(),
                    source: source.to_string(),
                    value: attribute.value.clone(),
                    expression: attribute.expression.clone(),
                })
        })
        .collect()
}

fn is_client_island_directive(name: &str) -> bool {
    client_island_directive_names().contains(&name)
}

fn is_unsupported_client_island_directive_syntax(name: &str) -> bool {
    unsupported_client_island_directive_syntax().contains(&name)
}

fn client_island_directive_names() -> &'static [&'static str] {
    &[
        "clientLoad",
        "clientVisible",
        "clientIdle",
        "clientOnly",
        "clientMedia",
        "clientInteraction",
    ]
}

fn unsupported_client_island_directive_syntax() -> &'static [&'static str] {
    &[
        "client:load",
        "client:visible",
        "client:idle",
        "client:only",
    ]
}

fn client_dynamic_imports(
    input: &DxReactClientIslandInput,
) -> Vec<DxReactClientIslandDynamicImport> {
    let mut imports = Vec::new();
    collect_dynamic_imports_from_source(
        &mut imports,
        &input.route_source_path,
        &input.route_source,
    );
    for segment in &input.segments {
        collect_dynamic_imports_from_source(&mut imports, &segment.source_path, &segment.source);
    }
    for component in &input.components {
        collect_dynamic_imports_from_source(
            &mut imports,
            &component.source_path,
            &component.source,
        );
    }
    imports.sort_by(|left, right| {
        left.source_path
            .cmp(&right.source_path)
            .then(left.source.cmp(&right.source))
    });
    imports.dedup_by(|left, right| {
        left.source_path == right.source_path && left.source == right.source
    });
    imports
}

fn collect_dynamic_imports_from_source(
    imports: &mut Vec<DxReactClientIslandDynamicImport>,
    source_path: &str,
    source: &str,
) {
    let Ok(re) = regex::Regex::new(r#"import\s*\(\s*["']([^"']+)["']\s*\)"#) else {
        return;
    };
    for capture in re.captures_iter(source) {
        let Some(import_source) = capture.get(1).map(|value| value.as_str().to_string()) else {
            continue;
        };
        let import_start = capture
            .get(0)
            .map(|value| value.start())
            .unwrap_or_default();
        let import_window = &source[import_start..source.len().min(import_start + 220)];
        imports.push(DxReactClientIslandDynamicImport {
            source_path: source_path.to_string(),
            chunk_id: format!(
                "dxchunk-{}",
                short_hash(&format!("{source_path}:{import_source}")).trim_start_matches("blake3:")
            ),
            source: import_source,
            preload: true,
            ssr: dynamic_import_ssr(import_window),
        });
    }
}

fn dynamic_import_ssr(source: &str) -> Option<bool> {
    if source.contains("ssr: false") {
        Some(false)
    } else if source.contains("ssr: true") {
        Some(true)
    } else {
        None
    }
}

fn client_hydration_runtime(
    route: &str,
    islands: &[DxReactClientIsland],
    dynamic_imports: &[DxReactClientIslandDynamicImport],
) -> DxReactClientIslandHydrationRuntime {
    let event_count = islands
        .iter()
        .map(|island| island.hydration.events.len())
        .sum::<usize>();
    let form_count = islands
        .iter()
        .map(|island| island.hydration.forms.len())
        .sum::<usize>();
    let script = hydration_runtime_script(route, islands, dynamic_imports);
    DxReactClientIslandHydrationRuntime {
        source_owned: true,
        deterministic: true,
        event_count,
        form_count,
        dynamic_import_count: dynamic_imports.len(),
        script_hash: short_hash(&script),
        script,
    }
}

fn hydration_runtime_script(
    route: &str,
    islands: &[DxReactClientIsland],
    dynamic_imports: &[DxReactClientIslandDynamicImport],
) -> String {
    let mut script = format!(
        "/* www client islands v1 route={} */\n(() => {{\n  const islands = document.querySelectorAll('[data-dx-island]');\n",
        route
    );
    script.push_str(
        "  islands.forEach((island) => {\n    island.querySelectorAll('[data-dx-event]').forEach((node) => {\n      const type = node.getAttribute('data-dx-event');\n      node.addEventListener(type, (event) => {\n        if (node.getAttribute('data-dx-prevent-default') === 'true') event.preventDefault();\n      });\n    });\n  });\n",
    );
    for island in islands {
        for event in &island.hydration.events {
            let state = event
                .state
                .as_deref()
                .map(js_string_literal)
                .unwrap_or_else(|| "null".to_string());
            let operation = event
                .operation
                .as_deref()
                .map(js_string_literal)
                .unwrap_or_else(|| "null".to_string());
            let form_id = event
                .form_id
                .as_deref()
                .map(js_string_literal)
                .unwrap_or_else(|| "null".to_string());
            let prevent_default = if event.prevent_default {
                "true"
            } else {
                "false"
            };
            script.push_str(&format!(
                "  document.querySelectorAll('[data-dx-event-id=\"{}\"]').forEach((node) => node.addEventListener({}, (event) => {{\n    if ({}) event.preventDefault();\n    const detail = {{ islandId: {}, sourcePath: {}, eventId: {}, event: {}, element: {}, handler: {}, state: {}, operation: {}, formId: {} }};\n    node.dispatchEvent(new CustomEvent('dx:client-island-event', {{ bubbles: true, detail }}));\n    document.dispatchEvent(new CustomEvent('dx:client-island-event', {{ detail }}));\n  }}));\n",
                event.event_id,
                js_string_literal(&event.event),
                prevent_default,
                js_string_literal(&island.id),
                js_string_literal(&island.source_path),
                js_string_literal(&event.event_id),
                js_string_literal(&event.event),
                js_string_literal(&event.element),
                js_string_literal(&event.handler),
                state,
                operation,
                form_id,
            ));
        }
    }
    for import in dynamic_imports {
        script.push_str(&format!(
            "  document.dispatchEvent(new CustomEvent('dx:preload', {{ detail: {{ chunk: '{}', source: '{}' }} }}));\n",
            import.chunk_id, import.source
        ));
    }
    script.push_str("})();");
    script
}

fn js_string_literal(value: &str) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "\"\"".to_string())
}

fn source_references_component(doc: &DxReactJsxDocument, component_name: &str) -> bool {
    let element_names = doc
        .elements
        .iter()
        .map(|element| element.name.as_str())
        .collect::<Vec<_>>();
    if element_names.contains(&component_name) {
        return true;
    }

    doc.imports.iter().any(|import| {
        import.specifiers.iter().any(|specifier| {
            !specifier.type_only
                && specifier.imported == component_name
                && element_names.contains(&specifier.local.as_str())
        }) || (!import.type_only
            && import
                .default
                .as_deref()
                .is_some_and(|local| local == component_name && element_names.contains(&local)))
    })
}

fn exported_component_name(source: &str) -> Option<String> {
    let re = regex::Regex::new(r#"\bexport\s+function\s+([A-Z][A-Za-z0-9_]*)\s*\("#).ok()?;
    re.captures(source)
        .and_then(|capture| capture.get(1).map(|name| name.as_str().to_string()))
}

fn element_expression(element: &DxReactJsxElement, name: &str) -> Option<String> {
    element
        .attributes
        .iter()
        .find(|attribute| attribute.name == name)
        .and_then(attribute_expression)
}

fn attribute_expression(attribute: &DxReactJsxAttribute) -> Option<String> {
    attribute
        .expression
        .clone()
        .or_else(|| attribute.value.clone())
}

fn normalize_handler(handler: &str) -> String {
    handler
        .trim()
        .trim_matches('{')
        .trim_matches('}')
        .trim()
        .to_string()
}

fn micro_op_label(op: DxMicroJsOp) -> String {
    match op {
        DxMicroJsOp::Add(1) => "add".to_string(),
        DxMicroJsOp::Add(-1) => "subtract".to_string(),
        DxMicroJsOp::Add(delta) if delta > 0 => format!("add:{delta}"),
        DxMicroJsOp::Add(delta) => format!("subtract:{}", delta.abs()),
        DxMicroJsOp::Set(_) => "set".to_string(),
        DxMicroJsOp::Toggle => "toggle".to_string(),
    }
}

fn state_target_id(island_id: &str, state_name: &str) -> String {
    format!("{island_id}-state-{state_name}")
}

fn client_island_id(source_path: &str) -> String {
    format!(
        "dxi-{}",
        short_hash(source_path).trim_start_matches("blake3:")
    )
}

fn short_hash(value: &str) -> String {
    format!(
        "blake3:{}",
        blake3::hash(value.as_bytes())
            .to_hex()
            .as_str()
            .chars()
            .take(16)
            .collect::<String>()
    )
}
