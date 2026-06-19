use dx_compiler::delivery::{
    DxDerivedStateSlot, DxGlobalStore, DxGlobalStoreAction, DxStateEventSlot, DxStateGraph,
    DxStateSlot,
};
use serde_json::{Value, json};

pub(super) struct DxAppRouterStateRuntime {
    pub(super) program: Value,
    pub(super) script_tag: Option<String>,
    pub(super) status: &'static str,
}

fn react_hook_policy() -> Value {
    json!({
        "status": "adapter-only-or-exact-compatibility-lowering",
        "runtime_api": false,
        "compatibility_authoring_only": true,
        "react_api_shim_executed": false,
        "full_react_hook_runtime": false,
        "exact_state_slot_lowering": {
            "adapter_syntax": "useState",
            "status": "exact-dx-state-slot-compatibility-only",
            "diagnostic_when_unlowerable": "dx.react-hook.useState.missing-exact-state-slot",
            "adapter_boundary_required_when_unlowerable": true,
        },
        "unsupported_hooks": [
            {
                "hook": "useEffect",
                "status": "adapter-boundary-required",
                "diagnostic_code": "dx.react-hook.useEffect.adapter-boundary-required",
                "next_action": "Rewrite deterministic behavior to DX-native effect() or keep it behind an explicit client island adapter."
            },
            {
                "hook": "useReducer",
                "status": "adapter-boundary-required",
                "diagnostic_code": "dx.react-hook.useReducer.adapter-boundary-required",
                "next_action": "Rewrite transitions to DX-native state()/action() or keep reducer semantics behind an adapter."
            },
            {
                "hook": "useContext",
                "status": "adapter-boundary-required",
                "diagnostic_code": "dx.react-hook.useContext.adapter-boundary-required",
                "next_action": "Use compiler-visible provider values or keep React context behind an adapter boundary."
            },
            {
                "hook": "useTransition/useOptimistic/useActionState",
                "status": "adapter-boundary-required",
                "diagnostic_code": "dx.react-hook.advanced.adapter-boundary-required",
                "next_action": "Keep concurrent or optimistic React runtime semantics adapter-owned until DX-native equivalents are implemented."
            }
        ],
    })
}

pub(crate) fn dx_native_reactivity_capabilities() -> Value {
    json!({
        "schema": "dx.tsx.dxNativeReactivityCapabilities",
        "schema_revision": 1,
        "contract_name": "DX Native Reactivity Capabilities",
        "status": "foundation-contract",
        "runtime": "fine-grained compiler-owned state graph",
        "source_owned": true,
        "node_modules_required": false,
        "full_react_hook_runtime": false,
        "readiness_release_ready": false,
        "dx_native_api": ["state()", "derived()", "effect()", "action()"],
        "react_familiar_authoring": true,
        "capability_matrix": [
            {
                "api": "state()",
                "status": "compiler-owned-foundation",
                "current_lowering": "DX-native state() slots, app-global store slots, and explicit state graph slots",
                "readiness_gap": "public DX-native state() authoring syntax and browser replay receipts"
            },
            {
                "api": "derived()",
                "status": "compiler-owned-foundation",
                "current_lowering": "safe derived slot recomputation for literal, identity, boolean, binary, and comparison operations",
                "readiness_gap": "complete derived expression catalog and no-JS fallback proof"
            },
            {
                "api": "effect()",
                "status": "scheduled-boundary-foundation",
                "current_lowering": "dependency scheduler records without arbitrary callback execution",
                "readiness_gap": "supported effect body execution, cleanup policy, cancellation, and browser receipts"
            },
            {
                "api": "action()",
                "status": "edge-metadata-foundation",
                "current_lowering": "event operation labels and server-action edge metadata",
                "readiness_gap": "typed action authoring, optimistic/error states, and replay proof"
            }
        ],
        "react_hook_policy": react_hook_policy(),
        "unsupported_react_api_policy": "React hooks are adapter-only authoring syntax; unsupported hooks must emit diagnostics or require adapter-boundary islands",
        "react_api_shim_executed": false,
        "adapter_boundary_required": [
            "arbitrary React hook execution",
            "React effect cleanup and ordering semantics",
            "React context/reducer/transition runtime parity",
            "component imports that require React runtime execution"
        ],
        "browser_proof_status": "foundation-not-release-proof",
    })
}

pub(super) fn build_state_runtime(
    route: &str,
    state_graph: &DxStateGraph,
) -> DxAppRouterStateRuntime {
    let lowerable_events = state_graph
        .event_slots
        .iter()
        .filter(|event| {
            infer_event_operation(event, state_graph)
                .as_ref()
                .is_some_and(is_lowerable_event_operation)
        })
        .count();
    let lowerable_derived_slots = state_graph
        .derived_slots
        .iter()
        .filter(|slot| infer_derived_operation(slot).is_some())
        .count();
    let status = if state_graph.slots.is_empty()
        && state_graph.derived_slots.is_empty()
        && state_graph.event_slots.is_empty()
        && state_graph.effects.is_empty()
        && state_graph.global_stores.is_empty()
    {
        "no-client-state"
    } else if state_graph.slots.is_empty()
        && state_graph.derived_slots.is_empty()
        && state_graph.event_slots.is_empty()
        && !state_graph.effects.is_empty()
    {
        "effect-scheduler-emitted"
    } else if lowerable_events == state_graph.event_slots.len()
        && lowerable_derived_slots == state_graph.derived_slots.len()
    {
        "runtime-emitted"
    } else {
        "partial-runtime-emitted"
    };
    let program = json!({
        "schema": "dx.tsx.stateRuntime",
        "schema_revision": 1,
        "contract_name": "TSX State Runtime",
        "route": route,
        "runtime": "generated-js-client-islands",
        "source_owned": true,
        "node_modules_required": false,
        "full_react_hook_runtime": false,
        "lowering_status": status,
        "react_hook_policy": react_hook_policy(),
        "supported_now": [
            "DX-native state() and app-global store slot metadata",
            "DX-native app-global store slot metadata",
            "state snapshot inspection",
            "safe setSlot state updates",
            "lowerable event operation application",
            "event-to-state dependency mapping",
            "simple add/subtract/toggle/input event operation labels",
            "deterministic effect dependency scheduling events",
            "server-action edge metadata dispatch events",
            "safe derived slot recomputation",
            "derived state DOM reflection events"
        ],
        "adapter_boundary_gaps": [
            "execute arbitrary component imports",
            "browser-replay supported native DOM listener binding across all lowered event shapes",
            "execute DX-native effect() callback bodies and cleanup functions with deterministic ordering",
            "React reducer/context/transition hooks require adapter boundaries or DX-native rewrites",
            "bundle client islands with full source imports"
        ],
        "counts": {
            "slots": state_graph.slots.len(),
            "global_stores": state_graph.global_stores.len(),
            "global_store_actions": state_graph.global_stores.iter().map(|store| store.actions.len()).sum::<usize>(),
            "derived_slots": state_graph.derived_slots.len(),
            "lowerable_derived_slots": lowerable_derived_slots,
            "events": state_graph.event_slots.len(),
            "lowerable_events": lowerable_events,
            "effects": state_graph.effects.len(),
            "server_actions": state_graph.server_actions.len(),
        },
        "global_stores": state_graph.global_stores.iter().map(global_store_program).collect::<Vec<_>>(),
        "slots": state_graph.slots.iter().map(state_slot_program).collect::<Vec<_>>(),
        "derived_slots": state_graph.derived_slots.iter().map(derived_slot_program).collect::<Vec<_>>(),
        "events": state_graph.event_slots.iter().map(|event| event_program(event, state_graph)).collect::<Vec<_>>(),
        "effects": &state_graph.effects,
        "effect_scheduler": {
            "schema": "dx.tsx.effectScheduler",
            "schema_revision": 1,
            "contract_name": "Effect Scheduler",
            "status": if state_graph.effects.is_empty() { "no-effects" } else { "dependency-scheduler-ready" },
            "strategy": "mount-and-state-dependency-dispatch",
            "event": "dx:effect-scheduled",
            "source_owned": true,
            "node_modules_required": false,
            "full_react_effect_body_execution": false,
            "full_react_effect_cleanup": false,
            "full_react_effect_ordering": false,
            "effects": state_graph.effects.iter().enumerate().map(|(order, effect)| json!({
                "order": order,
                "id": &effect.id,
                "kind": &effect.kind,
                "source_path": &effect.source_path,
                "dependencies": &effect.dependencies,
                "execution": "scheduled-boundary-only",
            })).collect::<Vec<_>>(),
            "limits": [
                "Schedules effect records on mount and after compiler-owned dependency slot changes.",
                "Does not execute callback bodies, cleanup functions, timers, subscriptions, or arbitrary JavaScript.",
                "React effect hook ordering remains adapter-boundary unless the behavior is rewritten to DX-native effect()."
            ],
        },
        "server_actions": &state_graph.server_actions,
        "action_runtime": {
            "schema": "dx.tsx.serverActionEdgeRuntime",
            "schema_revision": 1,
            "contract_name": "Server Action Edge Runtime",
            "status": if state_graph.server_actions.is_empty() { "no-server-actions" } else { "metadata-dispatch-ready" },
            "event": "dx:server-action-edge",
            "execution": "metadata-only",
            "edge_matching": "match by action symbol, event id, and source path when server_actions metadata is present",
            "unsafe_action_lowering_policy": "reject server-action operations unless they match source-owned server_actions metadata",
            "unmatched_action_status": "diagnostic-only",
            "unmatched_action_diagnostic_code": "dx.state-runtime.action.unmatched-source-edge",
            "source_owned": true,
            "node_modules_required": false,
            "server_action_invoked": false,
            "full_react_hook_runtime": false,
        },
        "state_operation_diagnostics": {
            "schema": "dx.tsx.stateOperationDiagnostics",
            "schema_revision": 1,
            "contract_name": "State Operation Diagnostics",
            "event": "dx:state-runtime-diagnostic",
            "unsupported_operation_status": "diagnostic-only",
            "unsupported_react_like_operation_diagnostic_code": "dx.state-runtime.operation.unsupported-react-like-operation",
            "react_api_shim_executed": false,
            "adapter_boundary_required": true,
            "full_react_hook_runtime": false,
            "react_hook_policy": react_hook_policy(),
        },
        "public_reactivity_model": dx_native_reactivity_capabilities(),
        "state_dom_reflection": {
            "schema": "dx.tsx.stateDomReflection",
            "schema_revision": 1,
            "contract_name": "State DOM Reflection",
            "status": if state_graph.slots.is_empty() && state_graph.derived_slots.is_empty() { "no-state-slots" } else { "runtime-hook-ready" },
            "event": "dx:state-dom-reflection",
            "derived_event": "dx:derived-state-slot",
            "derived_dom_reflection": "derived slots use same data-dx-state-* markers after safe runtime lowering",
            "markers": [
                "data-dx-state-read",
                "data-dx-state-value",
                "data-dx-state-checked",
                "data-dx-state-disabled",
                "data-dx-state-aria-*"
            ],
            "supported_targets": [
                "text-content",
                "form-control-value",
                "form-control-checked",
                "boolean-attribute",
                "aria-attribute"
            ],
            "full_react_reconciliation": false,
        },
        "browser_evidence": {
            "schema": "dx.tsx.stateRuntimeBrowserEvidence",
            "schema_revision": 1,
            "contract_name": "State Runtime Browser Evidence",
            "status": if state_graph.slots.is_empty() && state_graph.derived_slots.is_empty() { "no-state-slots" } else { "runtime-evidence-accessor-ready" },
            "runtime_global": "window.__DX_STATE_GRAPH_RUNTIME__",
            "script_id": "__DX_STATE_GRAPH_RUNTIME__",
            "ready_event": "dx:state-runtime-ready",
            "accessor": "window.__DX_STATE_GRAPH_RUNTIME__.getBrowserEvidence(reason)",
            "proof_surface": "browser-context DOM target counts, current state snapshot, event contract names, and explicit unsupported-runtime claims",
            "source_owned": true,
            "node_modules_required": false,
            "full_react_hook_runtime": false,
            "full_react_reconciliation": false,
        },
    });
    let script_tag = (!state_graph.slots.is_empty()
        || !state_graph.global_stores.is_empty()
        || !state_graph.derived_slots.is_empty()
        || !state_graph.event_slots.is_empty()
        || !state_graph.effects.is_empty())
    .then(|| state_runtime_script_tag(&program));

    DxAppRouterStateRuntime {
        program,
        script_tag,
        status,
    }
}

fn global_store_program(store: &DxGlobalStore) -> Value {
    json!({
        "id": &store.id,
        "name": &store.name,
        "source_path": &store.source_path,
        "scope": "global",
        "slots": store.slots.iter().map(state_slot_program).collect::<Vec<_>>(),
        "derived_slots": store.derived_slots.iter().map(derived_slot_program).collect::<Vec<_>>(),
        "actions": store.actions.iter().map(|action| json!({
            "id": &action.id,
            "name": &action.name,
            "source_path": &action.source_path,
            "handler": &action.handler,
            "state_dependencies": &action.state_dependencies,
            "execution": "compiler-owned-action-boundary",
        })).collect::<Vec<_>>(),
        "effects": &store.effects,
        "no_props_drilling_required": true,
        "node_modules_required": false,
    })
}

fn state_slot_program(slot: &DxStateSlot) -> Value {
    json!({
        "id": &slot.id,
        "name": &slot.name,
        "setter": &slot.setter,
        "scope": slot.scope,
        "source_path": &slot.source_path,
        "initial_source": &slot.initial_source,
        "initial_value": initial_runtime_value(&slot.initial_source, &slot.value_kind),
        "value_kind": &slot.value_kind,
    })
}

fn derived_slot_program(slot: &DxDerivedStateSlot) -> Value {
    let operation = infer_derived_operation(slot);
    let evaluation_status = if operation.is_some() {
        "safe-runtime-lowered"
    } else {
        "preview-only / not writable"
    };
    json!({
        "id": &slot.id,
        "name": &slot.name,
        "source_path": &slot.source_path,
        "expression": &slot.expression,
        "dependencies": &slot.dependencies,
        "operation": operation,
        "evaluation_status": evaluation_status,
        "dom_reflection": {
            "event": "dx:derived-state-slot",
            "markers": "derived slots use same data-dx-state-* markers by derived name",
            "full_react_reconciliation": false,
        },
    })
}

fn infer_derived_operation(slot: &DxDerivedStateSlot) -> Option<Value> {
    let expression = slot.expression.trim();
    for dependency in &slot.dependencies {
        if expression == dependency {
            return Some(json!({
                "kind": "identity",
                "dependency": dependency,
            }));
        }
        if expression
            .strip_prefix('!')
            .is_some_and(|tail| tail.trim() == dependency)
        {
            return Some(json!({
                "kind": "boolean-not",
                "dependency": dependency,
            }));
        }
        for operator in ["===", "!==", ">=", "<=", ">", "<", "+", "-", "*", "/", "%"] {
            let Some((left, right)) = split_binary_expression(expression, operator) else {
                continue;
            };
            if left == dependency {
                let literal = derived_literal_value(right)?;
                return Some(json!({
                    "kind": if is_comparison_operator(operator) { "comparison" } else { "binary" },
                    "dependency": dependency,
                    "operator": operator,
                    "right": literal,
                }));
            }
            if right == dependency && matches!(operator, "+" | "*") {
                let literal = derived_literal_value(left)?;
                return Some(json!({
                    "kind": "binary",
                    "dependency": dependency,
                    "operator": operator,
                    "left": literal,
                    "commuted": true,
                }));
            }
        }
    }
    None
}

fn split_binary_expression<'a>(expression: &'a str, operator: &str) -> Option<(&'a str, &'a str)> {
    let (left, right) = expression.split_once(operator)?;
    let left = left.trim();
    let right = right.trim();
    (!left.is_empty() && !right.is_empty()).then_some((left, right))
}

fn is_comparison_operator(operator: &str) -> bool {
    matches!(operator, "===" | "!==" | ">=" | "<=" | ">" | "<")
}

fn derived_literal_value(source: &str) -> Option<Value> {
    let source = source.trim();
    if source == "true" {
        return Some(json!(true));
    }
    if source == "false" {
        return Some(json!(false));
    }
    if source == "null" {
        return Some(Value::Null);
    }
    if let Ok(value) = source.parse::<i64>() {
        return Some(json!(value));
    }
    if let Ok(value) = source.parse::<f64>() {
        return serde_json::Number::from_f64(value).map(Value::Number);
    }
    if (source.starts_with('"') && source.ends_with('"'))
        || (source.starts_with('\'') && source.ends_with('\''))
        || (source.starts_with('`') && source.ends_with('`'))
    {
        return Some(json!(&source[1..source.len().saturating_sub(1)]));
    }
    None
}

fn event_program(event: &DxStateEventSlot, state_graph: &DxStateGraph) -> Value {
    json!({
        "id": &event.id,
        "source_path": &event.source_path,
        "element": &event.element,
        "event": &event.event,
        "handler": &event.handler,
        "state_dependencies": &event.state_dependencies,
        "action": &event.action,
        "operation": infer_event_operation(event, state_graph),
    })
}

fn is_lowerable_event_operation(operation: &Value) -> bool {
    let Some(kind) = operation.get("kind").and_then(Value::as_str) else {
        return false;
    };
    let has_slot = operation.get("slot").and_then(Value::as_str).is_some();
    has_slot
        && matches!(
            kind,
            "set-from-input" | "toggle" | "add" | "subtract" | "set-literal"
        )
}

fn infer_event_operation(event: &DxStateEventSlot, state_graph: &DxStateGraph) -> Option<Value> {
    for slot in &state_graph.slots {
        let Some(setter) = slot.setter.as_deref() else {
            continue;
        };
        let Some(arguments) = exact_setter_call_arguments(&event.handler, setter) else {
            continue;
        };
        let compact_arguments = compact_js(arguments);
        if compact_arguments == "event.target.value"
            || compact_arguments == "event.currentTarget.value"
        {
            return Some(json!({
                "kind": "set-from-input",
                "slot": &slot.name,
                "setter": setter,
                "lowering": "exact-useState-setter-call",
            }));
        }
        if compact_arguments == format!("!{}", slot.name) {
            return Some(json!({
                "kind": "toggle",
                "slot": &slot.name,
                "setter": setter,
                "lowering": "exact-useState-setter-call",
            }));
        }
        if infer_functional_toggle(arguments) {
            return Some(json!({
                "kind": "toggle",
                "slot": &slot.name,
                "setter": setter,
                "form": "functional-updater",
                "lowering": "exact-useState-setter-call",
            }));
        }
        if let Some(delta) = infer_delta(arguments, &slot.name) {
            return Some(json!({
                "kind": if delta >= 0 { "add" } else { "subtract" },
                "slot": &slot.name,
                "setter": setter,
                "delta": delta,
                "lowering": "exact-useState-setter-call",
            }));
        }
        if let Some(value) = infer_literal_set(arguments) {
            return Some(json!({
                "kind": "set-literal",
                "slot": &slot.name,
                "setter": setter,
                "value": value,
                "lowering": "exact-useState-setter-call",
            }));
        }
    }
    if let Some(action) = event
        .action
        .as_deref()
        .and_then(|name| global_store_action_by_name(name, &state_graph.global_stores))
    {
        if let Some(operation) = infer_global_store_action_operation(action, &state_graph.slots) {
            return Some(operation);
        }
    }

    event.action.as_ref().map(|action| {
        json!({
            "kind": "server-action",
            "action": action,
        })
    })
}

fn global_store_action_by_name<'a>(
    name: &str,
    stores: &'a [DxGlobalStore],
) -> Option<&'a DxGlobalStoreAction> {
    stores
        .iter()
        .flat_map(|store| store.actions.iter())
        .find(|action| action.name == name)
}

fn infer_global_store_action_operation(
    action: &DxGlobalStoreAction,
    slots: &[DxStateSlot],
) -> Option<Value> {
    let compact_handler = compact_js(&action.handler);
    for slot in slots {
        if slot.scope != dx_compiler::delivery::DxStateScope::Global {
            continue;
        }
        let Some(local_name) = slot.name.rsplit('.').next() else {
            continue;
        };
        let member = format!(".{local_name}");
        if !compact_handler.contains(&member) {
            continue;
        }
        if compact_handler.contains(&format!("{member}+=1"))
            || compact_handler.contains(&format!("{member}++"))
        {
            return Some(json!({
                "kind": "add",
                "slot": &slot.name,
                "delta": 1,
                "action": &action.name,
                "lowering": "dx-global-store-action",
            }));
        }
        if compact_handler.contains(&format!("{member}-=1"))
            || compact_handler.contains(&format!("{member}--"))
        {
            return Some(json!({
                "kind": "subtract",
                "slot": &slot.name,
                "delta": -1,
                "action": &action.name,
                "lowering": "dx-global-store-action",
            }));
        }
        if let Some(value) = assigned_global_store_literal(&compact_handler, &member) {
            return Some(json!({
                "kind": "set-literal",
                "slot": &slot.name,
                "value": value,
                "action": &action.name,
                "lowering": "dx-global-store-action",
            }));
        }
    }
    None
}

fn assigned_global_store_literal(handler: &str, member: &str) -> Option<Value> {
    let assignment = format!("{member}=");
    let start = handler.find(&assignment)? + assignment.len();
    let tail = &handler[start..];
    let end = tail.find([';', ',', '}']).unwrap_or(tail.len());
    infer_literal_set(&tail[..end])
}

fn exact_setter_call_arguments<'a>(source: &'a str, setter: &str) -> Option<&'a str> {
    let expression = exact_state_handler_expression(source)?;
    if !expression.starts_with(setter) || !has_identifier_boundary(expression, 0, setter.len()) {
        return None;
    }
    let open_paren = skip_ascii_whitespace(expression, setter.len());
    if !expression[open_paren..].starts_with('(') {
        return None;
    }
    let (arguments, close_index) = setter_call_argument_span(expression, setter)?;
    expression[close_index..]
        .trim()
        .is_empty()
        .then_some(arguments)
}

fn exact_state_handler_expression(source: &str) -> Option<&str> {
    let mut body = source.trim();
    if let Some(arrow) = body.find("=>") {
        body = body[arrow + 2..].trim();
    }
    if body.starts_with('{') {
        body = body.strip_prefix('{')?.trim();
        body = body.strip_suffix('}')?.trim();
    }
    body = body.strip_suffix(';').map(str::trim).unwrap_or(body);
    if body.contains(';') {
        return None;
    }
    (!body.is_empty()).then_some(body)
}

fn compact_js(source: &str) -> String {
    source.chars().filter(|ch| !ch.is_whitespace()).collect()
}

fn infer_functional_toggle(arguments: &str) -> bool {
    let arguments = compact_js(arguments);
    let Some(arrow) = arguments.find("=>") else {
        return false;
    };
    let parameter = arguments[..arrow]
        .trim()
        .trim_start_matches('(')
        .trim_end_matches(')');
    if parameter.is_empty() {
        return false;
    }
    arguments[arrow + 2..] == format!("!{parameter}")
}

fn infer_delta(arguments: &str, state_name: &str) -> Option<i64> {
    let compact = compact_js(arguments);
    let direct_add = format!("{state_name}+");
    if let Some(value) = compact.strip_prefix(&direct_add) {
        return parse_exact_positive_i64(value);
    }
    let direct_subtract = format!("{state_name}-");
    if let Some(value) = compact.strip_prefix(&direct_subtract) {
        return parse_exact_positive_i64(value).map(|value| -value);
    }
    let arrow = compact.find("=>")?;
    let parameter = compact[..arrow]
        .trim()
        .trim_start_matches('(')
        .trim_end_matches(')');
    if parameter.is_empty() {
        return None;
    }
    let expression = &compact[arrow + 2..];
    let functional_add = format!("{parameter}+");
    if let Some(value) = expression.strip_prefix(&functional_add) {
        return parse_exact_positive_i64(value);
    }
    let functional_subtract = format!("{parameter}-");
    if let Some(value) = expression.strip_prefix(&functional_subtract) {
        return parse_exact_positive_i64(value).map(|value| -value);
    }
    None
}

fn parse_exact_positive_i64(source: &str) -> Option<i64> {
    (!source.is_empty() && source.chars().all(|ch| ch.is_ascii_digit()))
        .then(|| source.parse::<i64>().ok())
        .flatten()
}

fn infer_literal_set(arguments: &str) -> Option<Value> {
    let value = arguments.trim();
    match value {
        "true" => Some(json!(true)),
        "false" => Some(json!(false)),
        _ => value
            .parse::<i64>()
            .map(|value| json!(value))
            .ok()
            .or_else(|| read_complete_js_string_literal(value).map(|value| json!(value))),
    }
}

fn setter_call_argument_span<'a>(source: &'a str, setter: &str) -> Option<(&'a str, usize)> {
    let open_paren = setter_call_open_paren(source, setter)?;
    let start = open_paren + 1;
    let mut depth = 1usize;
    let mut index = start;

    while index < source.len() {
        let ch = source[index..].chars().next()?;
        if ch == '"' || ch == '\'' || ch == '`' {
            index = skip_js_string(source, index, ch);
            continue;
        }
        match ch {
            '(' => depth += 1,
            ')' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some((&source[start..index], index + ch.len_utf8()));
                }
            }
            _ => {}
        }
        index += ch.len_utf8();
    }

    None
}

fn setter_call_open_paren(source: &str, setter: &str) -> Option<usize> {
    let mut search_from = 0;
    while let Some(offset) = source[search_from..].find(setter) {
        let index = search_from + offset;
        if !has_identifier_boundary(source, index, setter.len()) {
            search_from = index + setter.len();
            continue;
        }
        let open_paren = skip_ascii_whitespace(source, index + setter.len());
        if source[open_paren..].starts_with('(') {
            return Some(open_paren);
        }
        search_from = index + setter.len();
    }
    None
}

fn has_identifier_boundary(source: &str, index: usize, len: usize) -> bool {
    let bytes = source.as_bytes();
    let before = index == 0 || !is_identifier_byte(bytes[index - 1]);
    let after_index = index + len;
    let after = after_index >= source.len() || !is_identifier_byte(bytes[after_index]);
    before && after
}

fn is_identifier_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'$'
}

fn skip_ascii_whitespace(source: &str, mut index: usize) -> usize {
    while index < source.len() && source.as_bytes()[index].is_ascii_whitespace() {
        index += 1;
    }
    index
}

fn read_complete_js_string_literal(source: &str) -> Option<String> {
    let source = source.trim();
    let quote = source.chars().next()?;
    if quote != '"' && quote != '\'' && quote != '`' {
        return None;
    }

    let mut value = String::new();
    let mut escaped = false;
    let mut index = quote.len_utf8();
    while index < source.len() {
        let ch = source[index..].chars().next()?;
        index += ch.len_utf8();
        if escaped {
            value.push(ch);
            escaped = false;
            continue;
        }
        if ch == '\\' {
            escaped = true;
            continue;
        }
        if ch == quote {
            if quote == '`' && value.contains("${") {
                return None;
            }
            return source[index..].trim().is_empty().then_some(value);
        }
        value.push(ch);
    }
    None
}

fn skip_js_string(source: &str, index: usize, quote: char) -> usize {
    let mut cursor = index + quote.len_utf8();
    let mut escaped = false;
    while cursor < source.len() {
        let Some(ch) = source[cursor..].chars().next() else {
            return source.len();
        };
        cursor += ch.len_utf8();
        if escaped {
            escaped = false;
            continue;
        }
        if ch == '\\' {
            escaped = true;
            continue;
        }
        if ch == quote {
            return cursor;
        }
    }
    source.len()
}

fn initial_runtime_value(initial_source: &str, value_kind: &str) -> Value {
    let scalar = initial_source
        .trim()
        .strip_prefix("() =>")
        .map(str::trim)
        .unwrap_or_else(|| initial_source.trim());
    match value_kind {
        "number" => scalar
            .parse::<i64>()
            .map(|value| json!(value))
            .unwrap_or(Value::Null),
        "boolean" => match scalar {
            "true" => json!(true),
            "false" => json!(false),
            _ => Value::Null,
        },
        "string" => json!(scalar.trim_matches(['"', '\'', '`'])),
        _ => Value::Null,
    }
}

fn state_runtime_script_tag(program: &Value) -> String {
    let program_json = serde_json::to_string(program)
        .unwrap_or_else(|_| {
            "{\"schema\":\"dx.tsx.stateRuntime\",\"schema_revision\":1}".to_string()
        })
        .replace("</script", "<\\/script");
    format!(
        r#"<script type="module" id="__DX_STATE_GRAPH_RUNTIME__" data-dx-state-runtime="generated-js-client-islands">
(() => {{
  const program = {program_json};
  const values = Object.create(null);
  const derivedValues = Object.create(null);
  for (const slot of program.slots || []) {{
    values[slot.name] = slot.initial_value;
  }}
  function hasSlot(name) {{
    return Object.prototype.hasOwnProperty.call(values, name);
  }}
  function hasDerivedSlot(name) {{
    return Object.prototype.hasOwnProperty.call(derivedValues, name);
  }}
  function readReactiveValue(name) {{
    if (hasSlot(name)) return values[name];
    if (hasDerivedSlot(name)) return derivedValues[name];
    return undefined;
  }}
  function stateSelector(attribute, name, token) {{
    const safe = String(name).replace(/\\/g, "\\\\").replace(/"/g, '\\"');
    return "[" + attribute + (token ? "~=" : "=") + '"' + safe + '"]';
  }}
  function setBooleanAttribute(element, attribute, value) {{
    const enabled = Boolean(value);
    element[attribute] = enabled;
    if (enabled) {{
      element.setAttribute(attribute, "");
    }} else {{
      element.removeAttribute(attribute);
    }}
  }}
  function reflectStateSlotToDom(name, value) {{
    const updates = [];
    document.querySelectorAll(stateSelector("data-dx-state-read", name, true)).forEach((element) => {{
      element.textContent = value == null ? "" : String(value);
      updates.push({{ target: "text-content", tag: element.localName }});
    }});
    document.querySelectorAll(stateSelector("data-dx-state-value", name, false)).forEach((element) => {{
      if ("value" in element) element.value = value == null ? "" : String(value);
      element.setAttribute("value", value == null ? "" : String(value));
      updates.push({{ target: "form-control-value", tag: element.localName }});
    }});
    document.querySelectorAll(stateSelector("data-dx-state-checked", name, false)).forEach((element) => {{
      setBooleanAttribute(element, "checked", value);
      updates.push({{ target: "form-control-checked", tag: element.localName }});
    }});
    ["disabled", "required", "selected", "multiple", "readOnly"].forEach((attribute) => {{
      const marker = "data-dx-state-" + (attribute === "readOnly" ? "readonly" : attribute);
      document.querySelectorAll(stateSelector(marker, name, false)).forEach((element) => {{
        setBooleanAttribute(element, attribute, value);
        updates.push({{ target: "boolean-attribute", attribute: marker, tag: element.localName }});
      }});
    }});
    ["aria-expanded", "aria-pressed", "aria-selected", "aria-disabled", "aria-checked", "aria-current"].forEach((attribute) => {{
      document.querySelectorAll(stateSelector("data-dx-state-" + attribute, name, false)).forEach((element) => {{
        element.setAttribute(attribute, value == null ? "false" : String(value));
        updates.push({{ target: "aria-attribute", attribute, tag: element.localName }});
      }});
    }});
    document.dispatchEvent(new CustomEvent("dx:state-dom-reflection", {{
      detail: {{ route: program.route, name, value, updates, full_react_reconciliation: false }}
    }}));
    return updates;
  }}
  function setRuntimeSlot(name, value) {{
    if (!hasSlot(name)) return false;
    values[name] = value;
    const reflected = reflectStateSlotToDom(name, value);
    const derived = refreshDerivedSlots(name, "dependency-change");
    const scheduledEffects = scheduleEffectsForState(name, "state-change");
    document.dispatchEvent(new CustomEvent("dx:state-slot", {{ detail: {{ route: program.route, name, value, reflected_count: reflected.length, derived_count: derived.length }} }}));
    if (scheduledEffects.length) {{
      document.dispatchEvent(new CustomEvent("dx:effect-scheduler-flush", {{ detail: {{ route: program.route, reason: "state-change", changed_slot: name, scheduled_count: scheduledEffects.length }} }}));
    }}
    return true;
  }}
  function applyDerivedBinary(operator, current, literal) {{
    if (operator === "+") return current + literal;
    if (operator === "-") return Number(current || 0) - Number(literal || 0);
    if (operator === "*") return Number(current || 0) * Number(literal || 0);
    if (operator === "/") return Number(current || 0) / Number(literal || 1);
    if (operator === "%") return Number(current || 0) % Number(literal || 1);
    return undefined;
  }}
  function applyDerivedComparison(operator, current, literal) {{
    if (operator === "===") return current === literal;
    if (operator === "!==") return current !== literal;
    if (operator === ">=") return Number(current) >= Number(literal);
    if (operator === "<=") return Number(current) <= Number(literal);
    if (operator === ">") return Number(current) > Number(literal);
    if (operator === "<") return Number(current) < Number(literal);
    return undefined;
  }}
  function evaluateDerivedSlot(slot) {{
    const operation = slot && slot.operation;
    if (!operation || !operation.kind || !operation.dependency) {{
      return {{ ok: false, reason: "preview-only / not writable", slot }};
    }}
    const current = readReactiveValue(operation.dependency);
    let value;
    if (operation.kind === "identity") {{
      value = current;
    }} else if (operation.kind === "boolean-not") {{
      value = !Boolean(current);
    }} else if (operation.kind === "binary") {{
      const literal = Object.prototype.hasOwnProperty.call(operation, "right") ? operation.right : operation.left;
      value = applyDerivedBinary(operation.operator, current, literal);
    }} else if (operation.kind === "comparison") {{
      value = applyDerivedComparison(operation.operator, current, operation.right);
    }} else {{
      return {{ ok: false, reason: "unsupported-derived-operation", slot }};
    }}
    if (typeof value === "undefined" || Number.isNaN(value)) {{
      return {{ ok: false, reason: "derived-evaluation-failed", slot }};
    }}
    return {{ ok: true, value, operation }};
  }}
  function refreshDerivedSlots(changedSlot, reason = "state-change") {{
    const updates = [];
    for (const slot of program.derived_slots || []) {{
      const dependencies = Array.isArray(slot.dependencies) ? slot.dependencies : [];
      if (changedSlot && !dependencies.includes(changedSlot)) continue;
      const result = evaluateDerivedSlot(slot);
      if (!result.ok) {{
        continue;
      }}
      derivedValues[slot.name] = result.value;
      const reflected = reflectStateSlotToDom(slot.name, result.value);
      const detail = {{
        route: program.route,
        name: slot.name,
        value: result.value,
        dependencies,
        changed_slot: changedSlot || null,
        reason,
        evaluation_status: slot.evaluation_status || "safe-runtime-lowered",
        operation: result.operation,
        reflected_count: reflected.length,
        full_react_reconciliation: false
      }};
      document.dispatchEvent(new CustomEvent("dx:derived-state-slot", {{ detail }}));
      updates.push(detail);
    }}
    return updates;
  }}
  function effectDependencies(effect) {{
    return Array.isArray(effect && effect.dependencies) ? effect.dependencies : [];
  }}
  function shouldScheduleEffect(effect, changedSlot, reason) {{
    const dependencies = effectDependencies(effect);
    if (reason === "mount") {{
      return dependencies.length === 0 || dependencies.every((dependency) => hasSlot(dependency));
    }}
    return Boolean(changedSlot) && dependencies.includes(changedSlot);
  }}
  function scheduleEffectRecord(effect, reason, changedSlot) {{
    const detail = {{
      route: program.route,
      effect_id: effect.id,
      source_path: effect.source_path,
      dependencies: effectDependencies(effect),
      reason,
      changed_slot: changedSlot || null,
      order: typeof effect.order === "number" ? effect.order : null,
      executed: false,
      full_react_effect_body_execution: false,
      full_react_effect_cleanup: false
    }};
    document.dispatchEvent(new CustomEvent("dx:effect-scheduled", {{ detail }}));
    return detail;
  }}
  function scheduleEffectsForState(changedSlot, reason = "state-change") {{
    const scheduled = [];
    const effects = (program.effect_scheduler && program.effect_scheduler.effects) || program.effects || [];
    for (const effect of effects) {{
      if (shouldScheduleEffect(effect, changedSlot, reason)) {{
        scheduled.push(scheduleEffectRecord(effect, reason, changedSlot));
      }}
    }}
    return scheduled;
  }}
  function snapshotReactiveValues() {{
    return {{ ...values, ...derivedValues }};
  }}
  function domTargetCount(attribute, name, token = false) {{
    return document.querySelectorAll(stateSelector(attribute, name, token)).length;
  }}
  function booleanDomTargetCounts(name) {{
    const counts = Object.create(null);
    ["disabled", "required", "selected", "multiple", "readOnly"].forEach((attribute) => {{
      const marker = "data-dx-state-" + (attribute === "readOnly" ? "readonly" : attribute);
      const count = domTargetCount(marker, name, false);
      if (count) counts[marker] = count;
    }});
    return counts;
  }}
  function ariaDomTargetCounts(name) {{
    const counts = Object.create(null);
    ["aria-expanded", "aria-pressed", "aria-selected", "aria-disabled", "aria-checked", "aria-current"].forEach((attribute) => {{
      const count = domTargetCount("data-dx-state-" + attribute, name, false);
      if (count) counts[attribute] = count;
    }});
    return counts;
  }}
  function reactiveSlotBrowserEvidence(slot, kind) {{
    const name = slot && slot.name ? slot.name : "";
    return {{
      name,
      kind,
      value: readReactiveValue(name),
      dependencies: Array.isArray(slot && slot.dependencies) ? slot.dependencies : [],
      targets: {{
        text_content: domTargetCount("data-dx-state-read", name, true),
        form_control_value: domTargetCount("data-dx-state-value", name, false),
        form_control_checked: domTargetCount("data-dx-state-checked", name, false),
        boolean_attributes: booleanDomTargetCounts(name),
        aria_attributes: ariaDomTargetCounts(name)
      }}
    }};
  }}
  function stateRuntimeBrowserEvidence(reason = "manual") {{
    const effects = (program.effect_scheduler && program.effect_scheduler.effects) || program.effects || [];
    return {{
      schema: "dx.tsx.stateRuntimeBrowserEvidence",
      schema_revision: 1,
      route: program.route,
      reason,
      status: "browser-context-observed",
      runtime_global: "window.__DX_STATE_GRAPH_RUNTIME__",
      script_id: "__DX_STATE_GRAPH_RUNTIME__",
      source_owned: true,
      node_modules_required: false,
      full_react_hook_runtime: false,
      full_react_reconciliation: false,
      lowering_status: program.lowering_status,
      snapshot: snapshotReactiveValues(),
      counts: {{
        state_slots: (program.slots || []).length,
        derived_slots: (program.derived_slots || []).length,
        event_operations: (program.events || []).length,
        effects: effects.length,
        server_actions: (program.server_actions || []).length
      }},
      dom: {{
        state_slots: (program.slots || []).map((slot) => reactiveSlotBrowserEvidence(slot, "state")),
        derived_slots: (program.derived_slots || []).map((slot) => reactiveSlotBrowserEvidence(slot, "derived"))
      }},
      event_contracts: {{
        state_dom_reflection: "dx:state-dom-reflection",
        derived_state_slot: "dx:derived-state-slot",
        state_slot: "dx:state-slot",
        state_event: "dx:state-event",
        runtime_ready: "dx:state-runtime-ready",
        effect_scheduled: "dx:effect-scheduled",
        effect_scheduler_flush: "dx:effect-scheduler-flush",
        server_action_edge: "dx:server-action-edge",
        diagnostic: "dx:state-runtime-diagnostic"
      }},
      unsupported_runtime_claims: {{
        native_listener_binding_complete: false,
        full_react_effect_execution: false,
        arbitrary_react_hook_shims: false,
        server_action_invocation: false
      }}
    }};
  }}
  function emitStateRuntimeDiagnostic(event, operation, payload, reason, diagnosticCode) {{
    const detail = {{
      schema: "dx.tsx.stateRuntimeDiagnostic",
      schema_revision: 1,
      route: program.route,
      event_id: event && event.id ? event.id : null,
      event: event && event.event ? event.event : null,
      source_path: event && event.source_path ? event.source_path : null,
      operation_kind: operation && operation.kind ? operation.kind : null,
      operation,
      payload,
      status: "diagnostic-only",
      reason,
      diagnostic_code: diagnosticCode,
      source_owned: true,
      node_modules_required: false,
      full_react_hook_runtime: false,
      react_api_shim_executed: false,
      adapter_boundary_required: true
    }};
    document.dispatchEvent(new CustomEvent("dx:state-runtime-diagnostic", {{ detail }}));
    return {{
      ok: false,
      status: detail.status,
      reason,
      diagnostic_code: diagnosticCode,
      operation,
      full_react_hook_runtime: false,
      react_api_shim_executed: false,
      adapter_boundary_required: true
    }};
  }}
  function matchingServerActionEdge(event, action) {{
    const edges = Array.isArray(program.server_actions) ? program.server_actions : [];
    const exact = edges.find((edge) => edge && edge.action === action
      && (!edge.event_id || !event || edge.event_id === event.id)
      && (!edge.source_path || !event || edge.source_path === event.source_path));
    if (exact) return exact;
    return edges.find((edge) => edge && edge.action === action) || null;
  }}
  function emitServerActionEdge(event, operation, payload = {{}}) {{
    const action = operation.action || event.action || null;
    const edge = matchingServerActionEdge(event, action);
    const matched = Boolean(edge);
    const detail = {{
      schema: "dx.tsx.serverActionEdgeDispatch",
      schema_revision: 1,
      route: program.route,
      event_id: event && event.id ? event.id : null,
      event: event && event.event ? event.event : null,
      source_path: event && event.source_path ? event.source_path : null,
      action,
      edge_status: edge ? "matched-source-owned-edge" : "unmatched-action-metadata",
      edge_id: edge && edge.id ? edge.id : null,
      import_source: edge && Object.prototype.hasOwnProperty.call(edge, "import_source") ? edge.import_source : null,
      edge,
      payload,
      status: matched ? "metadata-only" : "diagnostic-only",
      diagnostic_code: matched ? null : "dx.state-runtime.action.unmatched-source-edge",
      reason: matched ? null : "unmatched-source-owned-server-action-edge",
      source_owned: true,
      node_modules_required: false,
      server_action_invoked: false,
      full_react_hook_runtime: false
    }};
    document.dispatchEvent(new CustomEvent("dx:server-action-edge", {{ detail }}));
    return {{
      ok: matched,
      action,
      edge,
      operation,
      status: detail.status,
      diagnostic_code: detail.diagnostic_code,
      reason: detail.reason,
      server_action_invoked: false
    }};
  }}
  function applyStateOperation(event, payload = {{}}) {{
    const operation = event && event.operation;
    if (!operation || !operation.kind) {{
      return {{ ok: false, reason: "no-lowerable-operation" }};
    }}
    if (operation.kind === "server-action") {{
      return emitServerActionEdge(event, operation, payload);
    }}
    if (!operation.slot) {{
      return {{ ok: false, reason: "no-lowerable-operation" }};
    }}
    const slot = operation.slot;
    if (!hasSlot(slot)) {{
      return {{ ok: false, reason: "unknown-slot", slot }};
    }}
    let value = values[slot];
    if (operation.kind === "set-from-input") {{
      if (typeof value === "boolean" && Object.prototype.hasOwnProperty.call(payload, "checked")) {{
        value = Boolean(payload.checked);
      }} else {{
        value = Object.prototype.hasOwnProperty.call(payload, "value") ? payload.value : "";
      }}
    }} else if (operation.kind === "toggle") {{
      value = !Boolean(value);
    }} else if (operation.kind === "add" || operation.kind === "subtract") {{
      value = Number(value || 0) + Number(operation.delta || 0);
    }} else if (operation.kind === "set-literal") {{
      value = operation.value;
    }} else {{
      return emitStateRuntimeDiagnostic(
        event,
        operation,
        payload,
        "unsupported-react-like-state-operation",
        "dx.state-runtime.operation.unsupported-react-like-operation"
      );
    }}
    const updated = setRuntimeSlot(slot, value);
    return {{ ok: updated, slot, value, operation }};
  }}
  const runtime = {{
    schema: program.schema,
    schema_revision: program.schema_revision,
    route: program.route,
    program,
    state_dom_reflection: program.state_dom_reflection,
    derived_dom_reflection: program.state_dom_reflection && program.state_dom_reflection.derived_dom_reflection,
    effect_scheduler: program.effect_scheduler,
    getSnapshot() {{
      return snapshotReactiveValues();
    }},
    getBrowserEvidence(reason = "manual") {{
      return stateRuntimeBrowserEvidence(reason);
    }},
    setSlot(name, value) {{
      return setRuntimeSlot(name, value);
    }},
    reflectStateSlotToDom,
    refreshDerivedSlots,
    scheduleEffectsForState,
    dispatch(eventId, payload = {{}}) {{
      const event = (program.events || []).find((candidate) => candidate.id === eventId);
      if (!event) return {{ ok: false, reason: "unknown-event" }};
      const state_result = applyStateOperation(event, payload);
      document.dispatchEvent(new CustomEvent("dx:state-event", {{ detail: {{ route: program.route, event, payload, state_result }} }}));
      return {{ ok: Boolean(state_result && state_result.ok), event, state_result }};
    }}
  }};
  window.__DX_STATE_GRAPH_RUNTIME__ = runtime;
  for (const slot of program.slots || []) {{
    reflectStateSlotToDom(slot.name, values[slot.name]);
  }}
  refreshDerivedSlots(null, "initial");
  const mountedEffects = scheduleEffectsForState(null, "mount");
  if (mountedEffects.length) {{
    document.dispatchEvent(new CustomEvent("dx:effect-scheduler-flush", {{ detail: {{ route: program.route, reason: "mount", scheduled_count: mountedEffects.length }} }}));
  }}
  document.dispatchEvent(new CustomEvent("dx:state-runtime-ready", {{
    detail: {{ route: program.route, slots: program.counts.slots, events: program.counts.events, status: program.lowering_status, browser_evidence: stateRuntimeBrowserEvidence("ready") }}
  }}));
}})();
</script>"#
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use dx_compiler::delivery::DxStateScope;

    fn slot(name: &str, setter: &str, value_kind: &str, initial_source: &str) -> DxStateSlot {
        DxStateSlot {
            id: format!("slot:{name}"),
            name: name.to_string(),
            setter: Some(setter.to_string()),
            scope: DxStateScope::Local,
            source_path: "app/page.tsx".to_string(),
            initial_source: initial_source.to_string(),
            value_kind: value_kind.to_string(),
        }
    }

    fn global_slot(name: &str, value_kind: &str, initial_source: &str) -> DxStateSlot {
        DxStateSlot {
            id: format!("global-slot:{name}"),
            name: name.to_string(),
            setter: None,
            scope: DxStateScope::Global,
            source_path: "lib/stores/counter.ts".to_string(),
            initial_source: initial_source.to_string(),
            value_kind: value_kind.to_string(),
        }
    }

    fn event(id: &str, handler: &str, dependencies: &[&str]) -> DxStateEventSlot {
        DxStateEventSlot {
            id: id.to_string(),
            source_path: "app/page.tsx".to_string(),
            element: "button".to_string(),
            event: "click".to_string(),
            handler: handler.to_string(),
            state_dependencies: dependencies
                .iter()
                .map(|dependency| dependency.to_string())
                .collect(),
            action: None,
        }
    }

    fn action_event(id: &str, handler: &str, action: &str) -> DxStateEventSlot {
        let mut event = event(id, handler, &[]);
        event.action = Some(action.to_string());
        event
    }

    fn derived_slot(name: &str, expression: &str, dependencies: &[&str]) -> DxDerivedStateSlot {
        DxDerivedStateSlot {
            id: format!("derived:{name}"),
            name: name.to_string(),
            source_path: "app/page.tsx".to_string(),
            expression: expression.to_string(),
            dependencies: dependencies
                .iter()
                .map(|dependency| dependency.to_string())
                .collect(),
        }
    }

    #[test]
    fn state_runtime_lowers_safe_string_literal_setters() {
        let mut graph = DxStateGraph::default();
        graph
            .slots
            .push(slot("status", "setStatus", "string", "\"idle\""));
        graph.event_slots.push(event(
            "event:set-status",
            r#"() => setStatus ( "ready" )"#,
            &["status"],
        ));

        let runtime = build_state_runtime("/settings", &graph);
        let operation = &runtime.program["events"][0]["operation"];

        assert_eq!(runtime.program["lowering_status"], "runtime-emitted");
        assert_eq!(operation["kind"], "set-literal");
        assert_eq!(operation["slot"], "status");
        assert_eq!(operation["value"], "ready");
        assert_eq!(runtime.program["slots"][0]["value_kind"], "string");
    }

    #[test]
    fn state_runtime_lowers_functional_boolean_toggles() {
        let mut graph = DxStateGraph::default();
        graph
            .slots
            .push(slot("open", "setOpen", "boolean", "false"));
        graph.event_slots.push(event(
            "event:toggle-open",
            "() => setOpen((current) => !current)",
            &["open"],
        ));

        let runtime = build_state_runtime("/menu", &graph);
        let operation = &runtime.program["events"][0]["operation"];

        assert_eq!(runtime.program["lowering_status"], "runtime-emitted");
        assert_eq!(operation["kind"], "toggle");
        assert_eq!(operation["slot"], "open");
        assert_eq!(operation["form"], "functional-updater");
    }

    #[test]
    fn state_runtime_rejects_non_exact_use_state_handler_bodies() {
        let mut graph = DxStateGraph::default();
        graph.slots.push(slot("count", "setCount", "number", "0"));
        graph.event_slots.push(event(
            "event:count-side-effect",
            "() => { setCount(count + 1); console.log(count); }",
            &["count"],
        ));

        let runtime = build_state_runtime("/counter", &graph);
        let operation = &runtime.program["events"][0]["operation"];

        assert_eq!(
            runtime.program["lowering_status"],
            "partial-runtime-emitted"
        );
        assert_eq!(operation, &Value::Null);
    }

    #[test]
    fn state_runtime_reflects_safe_derived_slots() {
        let mut graph = DxStateGraph::default();
        graph.slots.push(slot("count", "setCount", "number", "1"));
        graph
            .derived_slots
            .push(derived_slot("doubled", "count * 2", &["count"]));
        graph
            .derived_slots
            .push(derived_slot("mapped", "items.map(renderItem)", &["items"]));

        let runtime = build_state_runtime("/counter", &graph);
        let derived_slots = runtime.program["derived_slots"]
            .as_array()
            .expect("derived slots should be present");
        let script = runtime
            .script_tag
            .expect("derived runtime should emit script");

        assert_eq!(
            runtime.program["lowering_status"],
            "partial-runtime-emitted"
        );
        assert_eq!(runtime.program["counts"]["derived_slots"], 2);
        assert_eq!(runtime.program["counts"]["lowerable_derived_slots"], 1);
        assert_eq!(derived_slots[0]["operation"]["kind"], "binary");
        assert_eq!(derived_slots[0]["operation"]["operator"], "*");
        assert_eq!(
            derived_slots[0]["evaluation_status"],
            "safe-runtime-lowered"
        );
        assert_eq!(derived_slots[1]["operation"], Value::Null);
        assert_eq!(
            derived_slots[1]["evaluation_status"],
            "preview-only / not writable"
        );
        assert!(script.contains("refreshDerivedSlots"));
        assert!(script.contains("dx:derived-state-slot"));
        assert!(script.contains("derivedValues"));
    }

    #[test]
    fn state_runtime_lowers_global_store_actions_to_state_operations() {
        let mut graph = DxStateGraph::default();
        let slot = global_slot("counterStore.count", "number", "1");
        let action = DxGlobalStoreAction {
            id: "store-counter-action-increment".to_string(),
            name: "counterStore.increment".to_string(),
            source_path: "lib/stores/counter.ts".to_string(),
            handler: "(store) => { store.count += 1; }".to_string(),
            state_dependencies: vec!["counterStore.count".to_string()],
        };
        graph.slots.push(slot.clone());
        graph.global_stores.push(DxGlobalStore {
            id: "store-counter".to_string(),
            name: "counterStore".to_string(),
            source_path: "lib/stores/counter.ts".to_string(),
            slots: vec![slot],
            derived_slots: Vec::new(),
            actions: vec![action],
            effects: Vec::new(),
        });
        graph.event_slots.push(action_event(
            "event:global-increment",
            "() => counterStore.increment(counterStore)",
            "counterStore.increment",
        ));

        let runtime = build_state_runtime("/counter", &graph);
        let operation = &runtime.program["events"][0]["operation"];

        assert_eq!(runtime.program["counts"]["global_stores"], 1);
        assert_eq!(runtime.program["counts"]["global_store_actions"], 1);
        assert_eq!(runtime.program["lowering_status"], "runtime-emitted");
        assert_eq!(operation["kind"], "add");
        assert_eq!(operation["slot"], "counterStore.count");
        assert_eq!(operation["delta"], 1);
        assert_eq!(operation["action"], "counterStore.increment");
        assert_eq!(operation["lowering"], "dx-global-store-action");
    }

    #[test]
    fn dx_native_reactivity_capabilities_keep_unsupported_hooks_diagnosed() {
        let capabilities = dx_native_reactivity_capabilities();

        assert_eq!(
            capabilities["schema"],
            "dx.tsx.dxNativeReactivityCapabilities"
        );
        assert_eq!(capabilities["source_owned"], true);
        assert_eq!(capabilities["full_react_hook_runtime"], false);
        assert_eq!(capabilities["readiness_release_ready"], false);
        assert_eq!(
            capabilities["unsupported_react_api_policy"],
            "React hooks are adapter-only authoring syntax; unsupported hooks must emit diagnostics or require adapter-boundary islands"
        );
        assert_eq!(
            capabilities["react_hook_policy"]["compatibility_authoring_only"],
            true
        );
        assert_eq!(
            capabilities["react_hook_policy"]["unsupported_hooks"][0]["diagnostic_code"],
            "dx.react-hook.useEffect.adapter-boundary-required"
        );
        assert_eq!(capabilities["react_api_shim_executed"], false);
        assert_eq!(
            capabilities["browser_proof_status"],
            "foundation-not-release-proof"
        );
    }
}
