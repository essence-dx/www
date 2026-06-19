use std::collections::BTreeSet;

use dx_compiler::delivery::{DxReactAppSegmentKind, DxReactAppSegmentSource, DxStateGraph};
use serde_json::{Value, json};

pub(super) fn build_tsx_app_router_semantics(
    route_source_path: &str,
    route_source: &str,
    segments: &[DxReactAppSegmentSource],
    state_graph: &DxStateGraph,
) -> Value {
    let mut sources = segments
        .iter()
        .map(|segment| SemanticSource {
            kind: segment_kind_label(segment.kind),
            source_path: segment.source_path.as_str(),
            source: segment.source.as_str(),
        })
        .collect::<Vec<_>>();
    sources.push(SemanticSource {
        kind: "page",
        source_path: route_source_path,
        source: route_source,
    });

    let source_summaries = sources
        .iter()
        .map(|source| source_semantics(source, state_graph))
        .collect::<Vec<_>>();
    let composition_chain = sources
        .iter()
        .filter(|source| matches!(source.kind, "layout" | "template" | "page"))
        .map(|source| {
            json!({
                "kind": source.kind,
                "source_path": source.source_path,
                "accepts_children": accepts_children(source),
                "default_export": has_default_export(source.source),
            })
        })
        .collect::<Vec<_>>();
    let imports = sources
        .iter()
        .flat_map(|source| {
            collect_imports(source.source)
                .into_iter()
                .map(move |import| {
                    json!({
                        "source_path": source.source_path,
                        "specifier": import.clone(),
                        "kind": import_kind(&import),
                        "source_owned": import_is_source_owned(&import),
                    })
                })
        })
        .collect::<Vec<_>>();
    let hooks = unique_strings(
        sources
            .iter()
            .flat_map(|source| collect_hook_usage(source.source))
            .collect(),
    );
    let event_handlers = unique_strings(
        sources
            .iter()
            .flat_map(|source| collect_event_handlers(source.source))
            .collect(),
    );
    let react_event_support = react_event_support_manifest(&sources);
    let client_boundary_sources = sources
        .iter()
        .filter(|source| source_needs_client_boundary(source))
        .map(|source| source.source_path.to_string())
        .collect::<Vec<_>>();
    let server_boundary_sources = sources
        .iter()
        .filter(|source| has_directive(source.source, "use server"))
        .map(|source| source.source_path.to_string())
        .collect::<Vec<_>>();
    let effect_context_boundaries = effect_context_boundary_manifest(&sources, state_graph);
    let react_hook_support = react_hook_support_manifest(&sources, state_graph);
    let warnings = semantic_warnings(&sources, state_graph);

    json!({
        "schema": "dx.tsx.appRouterSemantics",
        "schema_revision": 1,
        "contract_name": "App Router Semantics",
        "public_authoring": "tsx",
        "react_compatibility_claim": "react-familiar-compiler-owned",
        "full_react_runtime_parity": false,
        "composition_chain": composition_chain,
        "source_summaries": source_summaries,
        "import_graph": imports,
        "hooks_detected": hooks,
        "event_handlers_detected": event_handlers,
        "react_event_support": react_event_support,
        "client_boundaries": {
            "count": client_boundary_sources.len(),
            "sources": client_boundary_sources,
        },
        "server_boundaries": {
            "count": server_boundary_sources.len(),
            "sources": server_boundary_sources,
        },
        "effect_context_boundaries": effect_context_boundaries,
        "react_hook_support": react_hook_support,
        "state_graph": state_graph_semantics(state_graph),
        "supported_now": [
            "app route matching",
            "dynamic params",
            "search params",
            "metadata discovery",
            "layout/template/boundary discovery",
            "source-owned semantic contract",
            "compiler-owned state graph ABI"
        ],
        "still_needed_for_nextjs_runtime_parity": [
            "general JSX execution",
            "component import execution",
            "props evaluation",
            "browser runtime lowering for common hook semantics",
            "client island bundling",
            "route handler execution beyond safe boundaries"
        ],
        "warnings": warnings,
    })
}

struct SemanticSource<'a> {
    kind: &'static str,
    source_path: &'a str,
    source: &'a str,
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

fn source_semantics(source: &SemanticSource<'_>, state_graph: &DxStateGraph) -> Value {
    let hooks = collect_hook_usage(source.source);
    let hook_support = hooks
        .iter()
        .map(|hook| hook_support_finding(source.source_path, source.source, hook, state_graph))
        .collect::<Vec<_>>();
    let event_handlers = collect_event_handlers(source.source);
    let event_support = event_handlers
        .iter()
        .map(|event| event_support_finding(source.source_path, event))
        .collect::<Vec<_>>();
    json!({
        "kind": source.kind,
        "source_path": source.source_path,
        "use_client": has_directive(source.source, "use client"),
        "use_server": has_directive(source.source, "use server"),
        "default_export": has_default_export(source.source),
        "accepts_children": accepts_children(source),
        "uses_params": source.source.contains("params"),
        "uses_search_params": source.source.contains("searchParams"),
        "hook_count": hooks.len(),
        "hooks": hooks,
        "hook_support": hook_support,
        "event_handler_count": event_handlers.len(),
        "event_handlers": event_handlers,
        "event_support": event_support,
        "client_boundary_needed": source_needs_client_boundary(source),
    })
}

fn react_event_support_manifest(sources: &[SemanticSource<'_>]) -> Value {
    let findings = sources
        .iter()
        .flat_map(|source| {
            collect_event_handlers(source.source)
                .into_iter()
                .map(|event| event_support_finding(source.source_path, &event))
        })
        .collect::<Vec<_>>();
    let unsupported_count = findings
        .iter()
        .filter(|finding| finding["status"] == "unsupported-react-event-diagnostic")
        .count();

    json!({
        "schema": "dx.tsx.reactEventSupport",
        "schema_revision": 1,
        "contract_name": "React-Style Native Event Support",
        "native_dom_events_direct": true,
        "react_synthetic_events": false,
        "unsupported_count": unsupported_count,
        "findings": findings,
        "policy": [
            "React-style onClick/onInput authoring lowers to native addEventListener names when the event exists in the DX native DOM event catalog.",
            "Unsupported onX attributes are diagnostics, not silent drops or synthetic React parity claims."
        ],
    })
}

fn event_support_finding(source_path: &str, attribute_name: &str) -> Value {
    let dom_event = dx_compiler::delivery::react_style_event_attribute_to_dom_event(attribute_name);
    let status = if dom_event.is_some() {
        "native-dom-event-supported"
    } else {
        "unsupported-react-event-diagnostic"
    };
    let failure_reason = react_event_failure_reason(attribute_name, dom_event.is_some());
    json!({
        "source_path": source_path,
        "attribute": attribute_name,
        "dom_event": dom_event,
        "status": status,
        "diagnostic_code": react_event_diagnostic_code(attribute_name, status),
        "failure_reason": failure_reason,
        "browser_api": "addEventListener",
        "listener_attached": status == "native-dom-event-supported",
        "react_synthetic_event": false,
        "adapter_boundary_required": status != "native-dom-event-supported",
        "binder_proof_status": if status == "native-dom-event-supported" {
            "native-add-event-listener-eligible"
        } else {
            "diagnostic-only-no-listener"
        },
        "next_action": if status == "native-dom-event-supported" {
            "No adapter required for native listener lowering."
        } else {
            "Use a native DOM event from the catalog or add an explicit adapter boundary."
        },
    })
}

fn react_event_failure_reason(attribute_name: &str, supported: bool) -> Option<&'static str> {
    if supported {
        return None;
    }
    let Some(rest) = attribute_name.strip_prefix("on") else {
        return Some("missing-react-style-on-prefix");
    };
    let Some(first) = rest.chars().next() else {
        return Some("missing-event-name-after-on-prefix");
    };
    if !first.is_ascii_uppercase() {
        return Some("event-name-must-start-uppercase-after-on-prefix");
    }
    Some("not-in-native-dom-event-catalog")
}

fn react_event_diagnostic_code(attribute_name: &str, status: &str) -> String {
    let attribute = diagnostic_identifier(attribute_name);
    match status {
        "native-dom-event-supported" => {
            format!("dx.dom-event.{attribute}.native-listener-lowered")
        }
        "unsupported-react-event-diagnostic" => {
            format!("dx.dom-event.unsupported.{attribute}")
        }
        _ => format!("dx.dom-event.{attribute}.{status}"),
    }
}

fn diagnostic_identifier(value: &str) -> String {
    value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || character == '-' || character == '_' {
                character
            } else {
                '-'
            }
        })
        .collect()
}

fn react_hook_support_manifest(
    sources: &[SemanticSource<'_>],
    state_graph: &DxStateGraph,
) -> Value {
    let findings = sources
        .iter()
        .flat_map(|source| {
            collect_hook_usage(source.source).into_iter().map(|hook| {
                hook_support_finding(source.source_path, source.source, &hook, state_graph)
            })
        })
        .collect::<Vec<_>>();
    let unsupported_count = findings
        .iter()
        .filter(|finding| finding["status"] == "unsupported-react-hook-diagnostic")
        .count();

    json!({
        "schema": "dx.tsx.reactHookSupport",
        "schema_revision": 1,
        "contract_name": "React Hook Support Diagnostics",
        "full_react_hook_runtime": false,
        "react_api_shim_executed": false,
        "unsupported_count": unsupported_count,
        "findings": findings,
        "policy": [
            "useState is compatibility sugar only when the compiler can lower it exactly into DX state slots.",
            "Effect hooks are boundary records until source-owned scheduling can execute the supported subset.",
            "Unsupported hooks require a precise diagnostic or an explicit adapter boundary before any runtime behavior."
        ],
    })
}

fn hook_support_finding(
    source_path: &str,
    source: &str,
    hook: &str,
    state_graph: &DxStateGraph,
) -> Value {
    let (status, runtime_behavior, next_action) =
        hook_support_status(source_path, source, hook, state_graph);
    json!({
        "source_path": source_path,
        "hook": hook,
        "status": status,
        "diagnostic_code": react_hook_diagnostic_code(hook, status),
        "runtime_behavior": runtime_behavior,
        "next_action": next_action,
        "react_runtime_required": false,
        "adapter_boundary_required": status != "compatibility-lowered",
    })
}

fn hook_support_status(
    source_path: &str,
    source: &str,
    hook: &str,
    state_graph: &DxStateGraph,
) -> (&'static str, &'static str, &'static str) {
    match hook {
        "useState"
            if state_graph_has_exact_use_state_lowering(state_graph, source_path, source) =>
        {
            (
                "compatibility-lowered",
                "Lowered into DX state slots because every useState binding has an exact compiler-owned state slot.",
                "Use DX-native state()/derived()/effect()/action() for new code that needs guaranteed release-readiness semantics.",
            )
        }
        "useState" => (
            "unsupported-react-hook-diagnostic",
            "useState was detected, but the compiler did not find an exact DX state slot for this source.",
            "Rewrite to DX-native state() or keep this component behind an explicit client island adapter until the pattern is lowerable.",
        ),
        "useEffect" | "useLayoutEffect" | "useInsertionEffect" => (
            "effect-boundary-scheduled",
            "Recorded as an effect boundary; callback bodies and cleanup are not executed as React runtime semantics and not treated as a no-op.",
            "Move deterministic browser behavior to DX-native effect() or an explicit client island adapter.",
        ),
        "useMemo" | "useCallback" | "useRef" => (
            "semantic-boundary",
            "Detected as author intent, but not executed through a hidden React runtime.",
            "Prefer compiler-visible derived()/state() values or isolate React-specific behavior behind an adapter boundary.",
        ),
        "useReducer" | "useTransition" | "useOptimistic" | "useActionState" | "useContext" => (
            "unsupported-react-hook-diagnostic",
            "Not lowered by the DX-native runtime and never treated as a no-op compatibility shim.",
            "Use a DX-native primitive, wait for a source-owned lowering, or mark this component as an explicit framework island.",
        ),
        _ => (
            "unsupported-react-hook-diagnostic",
            "Unknown React-style hook is detected but not lowered by DX WWW.",
            "Use a supported DX-native primitive or an explicit adapter boundary.",
        ),
    }
}

fn react_hook_diagnostic_code(hook: &str, status: &str) -> String {
    match (hook, status) {
        ("useState", "compatibility-lowered") => {
            "dx.react-hook.useState.exact-dx-state-slot-lowering".to_string()
        }
        ("useState", "unsupported-react-hook-diagnostic") => {
            "dx.react-hook.useState.missing-exact-state-slot".to_string()
        }
        ("useEffect" | "useLayoutEffect" | "useInsertionEffect", "effect-boundary-scheduled") => {
            format!("dx.react-hook.{hook}.effect-boundary-scheduled")
        }
        (_, "unsupported-react-hook-diagnostic") => format!("dx.react-hook.unsupported.{hook}"),
        _ => format!("dx.react-hook.{hook}.{status}"),
    }
}

struct UseStateBinding {
    state_name: String,
    setter_name: Option<String>,
}

fn state_graph_has_exact_use_state_lowering(
    state_graph: &DxStateGraph,
    source_path: &str,
    source: &str,
) -> bool {
    let bindings = collect_use_state_bindings(source);
    if bindings.is_empty() {
        return false;
    }

    let source_path = normalize_semantic_source_path(source_path);
    let source_slots = state_graph
        .slots
        .iter()
        .filter(|slot| normalize_semantic_source_path(&slot.source_path) == source_path)
        .collect::<Vec<_>>();

    let source_slots_match = source_slots.len() == bindings.len()
        && bindings.iter().all(|binding| {
            source_slots.iter().any(|slot| {
                slot.name.as_str() == binding.state_name.as_str()
                    && slot.setter.as_deref() == binding.setter_name.as_deref()
                    && !slot.initial_source.trim().is_empty()
            })
        });

    source_slots_match
        && source_events_have_exact_use_state_lowering(state_graph, &source_path, source, &bindings)
}

fn collect_use_state_bindings(source: &str) -> Vec<UseStateBinding> {
    let searchable = source_without_comments_and_strings(source);
    let use_state_calls = react_hook_call_patterns(&searchable)
        .into_iter()
        .filter(|call| call.canonical == "useState")
        .map(|call| call.regex_source())
        .collect::<Vec<_>>();
    if use_state_calls.is_empty() {
        return Vec::new();
    }
    let Ok(re) = regex::Regex::new(&format!(
        r#"\b(?:const|let)\s*\[\s*([A-Za-z_$][A-Za-z0-9_$]*)\s*(?:,\s*([A-Za-z_$][A-Za-z0-9_$]*))?\s*\]\s*=\s*(?:{})\s*\("#,
        use_state_calls.join("|")
    )) else {
        return Vec::new();
    };

    re.captures_iter(&searchable)
        .filter_map(|capture| {
            let state_name = capture.get(1)?.as_str().to_string();
            let setter_name = capture.get(2).map(|setter| setter.as_str().to_string());
            Some(UseStateBinding {
                state_name,
                setter_name,
            })
        })
        .collect()
}

fn source_events_have_exact_use_state_lowering(
    state_graph: &DxStateGraph,
    source_path: &str,
    source: &str,
    bindings: &[UseStateBinding],
) -> bool {
    let source_events = state_graph
        .event_slots
        .iter()
        .filter(|event| normalize_semantic_source_path(&event.source_path) == source_path)
        .collect::<Vec<_>>();

    bindings.iter().all(|binding| {
        let Some(setter) = binding.setter_name.as_deref() else {
            return true;
        };
        let setter_call_count = count_use_state_setter_calls(source, setter);
        if setter_call_count == 0 {
            return true;
        }

        let matching_events = source_events
            .iter()
            .filter(|event| count_use_state_setter_calls(&event.handler, setter) > 0)
            .collect::<Vec<_>>();
        matching_events.len() == setter_call_count
            && matching_events.iter().all(|event| {
                event_handler_has_exact_use_state_setter_operation(&event.handler, binding)
            })
    })
}

fn count_use_state_setter_calls(source: &str, setter: &str) -> usize {
    let mut count = 0usize;
    let mut search_from = 0usize;
    while let Some(offset) = source[search_from..].find(setter) {
        let index = search_from + offset;
        if has_identifier_boundary(source, index, setter.len()) {
            let open_paren = skip_ascii_whitespace(source, index + setter.len());
            if source[open_paren..].starts_with('(') {
                count += 1;
            }
        }
        search_from = index + setter.len();
    }
    count
}

fn event_handler_has_exact_use_state_setter_operation(
    handler: &str,
    binding: &UseStateBinding,
) -> bool {
    let Some(setter) = binding.setter_name.as_deref() else {
        return false;
    };
    let Some(arguments) = exact_setter_call_arguments(handler, setter) else {
        return false;
    };
    is_lowerable_use_state_setter_argument(arguments, &binding.state_name)
}

fn exact_setter_call_arguments<'a>(source: &'a str, setter: &str) -> Option<&'a str> {
    let expression = exact_state_handler_expression(source)?;
    if !expression.starts_with(setter) || !has_identifier_boundary(expression, 0, setter.len()) {
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

fn setter_call_argument_span<'a>(source: &'a str, setter: &str) -> Option<(&'a str, usize)> {
    let mut index = setter.len();
    index = skip_ascii_whitespace(source, index);
    if !source[index..].starts_with('(') {
        return None;
    }
    let start = index + 1;
    let mut depth = 1usize;
    index = start;
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

fn is_lowerable_use_state_setter_argument(arguments: &str, state_name: &str) -> bool {
    let compact = compact_js(arguments);
    compact == "event.target.value"
        || compact == "event.currentTarget.value"
        || compact == format!("!{state_name}")
        || infer_functional_toggle(&compact)
        || infer_delta(&compact, state_name)
        || infer_literal_set(&compact)
}

fn compact_js(source: &str) -> String {
    source.chars().filter(|ch| !ch.is_whitespace()).collect()
}

fn infer_functional_toggle(compact_arguments: &str) -> bool {
    let Some(arrow) = compact_arguments.find("=>") else {
        return false;
    };
    let parameter = compact_arguments[..arrow]
        .trim()
        .trim_start_matches('(')
        .trim_end_matches(')');
    !parameter.is_empty() && compact_arguments[arrow + 2..] == format!("!{parameter}")
}

fn infer_delta(compact_arguments: &str, state_name: &str) -> bool {
    compact_arguments
        .strip_prefix(&format!("{state_name}+"))
        .or_else(|| compact_arguments.strip_prefix(&format!("{state_name}-")))
        .is_some_and(|value| !value.is_empty() && value.chars().all(|ch| ch.is_ascii_digit()))
}

fn infer_literal_set(compact_arguments: &str) -> bool {
    matches!(compact_arguments, "true" | "false")
        || compact_arguments.parse::<i64>().is_ok()
        || is_complete_js_string_literal(compact_arguments)
}

fn is_complete_js_string_literal(source: &str) -> bool {
    let Some(quote) = source.chars().next() else {
        return false;
    };
    if quote != '"' && quote != '\'' && quote != '`' {
        return false;
    }
    skip_js_string(source, 0, quote) == source.len()
}

fn skip_js_string(source: &str, mut index: usize, quote: char) -> usize {
    index += quote.len_utf8();
    let mut escaped = false;
    while index < source.len() {
        let Some(ch) = source[index..].chars().next() else {
            break;
        };
        index += ch.len_utf8();
        if escaped {
            escaped = false;
        } else if ch == '\\' {
            escaped = true;
        } else if ch == quote {
            break;
        }
    }
    index
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

fn normalize_semantic_source_path(path: &str) -> String {
    path.replace('\\', "/").trim_start_matches("./").to_string()
}

fn effect_context_boundary_manifest(
    sources: &[SemanticSource<'_>],
    state_graph: &DxStateGraph,
) -> Value {
    let context_sources = sources
        .iter()
        .filter_map(context_source_boundary)
        .collect::<Vec<_>>();
    let effect_sources = state_graph
        .effects
        .iter()
        .map(|effect| {
            json!({
                "id": &effect.id,
                "kind": &effect.kind,
                "source_path": &effect.source_path,
                "dependencies": &effect.dependencies,
                "effect_scheduler_status": "dependency-scheduler-ready",
                "full_react_effect_ordering": false,
            })
        })
        .collect::<Vec<_>>();
    let context_provider_count = context_sources
        .iter()
        .filter_map(|source| source.get("provider_count").and_then(Value::as_u64))
        .sum::<u64>() as usize;
    let context_consumer_count = context_sources
        .iter()
        .filter_map(|source| source.get("consumer_count").and_then(Value::as_u64))
        .sum::<u64>() as usize;
    let context_runtime = context_runtime_manifest(&context_sources);
    json!({
        "schema": "dx.tsx.effectContextBoundaryManifest",
        "schema_revision": 1,
        "contract_name": "Effect And Context Boundary Manifest",
        "status": if state_graph.effects.is_empty() && context_sources.is_empty() {
            "no-effect-context-boundaries"
        } else {
            "effect-context-boundaries-detected"
        },
        "effect_scheduler_status": if state_graph.effects.is_empty() {
            "not-needed"
        } else {
            "dependency-scheduler-ready"
        },
        "effect_count": state_graph.effects.len(),
        "context_source_count": context_sources.len(),
        "context_provider_count": context_provider_count,
        "context_consumer_count": context_consumer_count,
        "effects": effect_sources,
        "contexts": context_sources,
        "context_runtime": context_runtime,
        "full_react_effect_ordering": false,
        "full_react_context_runtime": false,
        "node_modules_required": false,
        "limits": [
            "Makes useEffect/useLayoutEffect and React context explicit before runtime lowering.",
            "Schedules dependency records through the generated effect scheduler but does not execute callback bodies.",
            "Registers detected context providers and consumers through the generated context runtime but does not execute useContext.",
            "Does not run cleanup functions, emulate Strict Mode double invocation, or execute full React provider propagation.",
            "Keeps full React parity blocked until effect ordering and context propagation are implemented."
        ],
    })
}

fn context_source_boundary(source: &SemanticSource<'_>) -> Option<Value> {
    let provider_count = source.source.matches(".Provider").count()
        + source.source.matches("Provider>").count()
        + source.source.matches("Provider ").count();
    let consumer_count = count_occurrences(source.source, "useContext(");
    let create_context_count = count_occurrences(source.source, "createContext(");
    if provider_count == 0 && consumer_count == 0 && create_context_count == 0 {
        return None;
    }
    let contexts = collect_context_names(source.source)
        .into_iter()
        .map(|name| {
            let initial_value_source = initial_context_value_source(source.source, &name);
            let provider_value_source = provider_context_value_source(source.source, &name);
            let safe_literal_context_value = provider_value_source
                .as_deref()
                .or(initial_value_source.as_deref())
                .and_then(safe_literal_context_value);
            json!({
                "name": name,
                "source_path": source.source_path,
                "initial_value_source": initial_value_source,
                "provider_value_source": provider_value_source,
                "safe_literal_context_value": safe_literal_context_value,
            })
        })
        .collect::<Vec<_>>();
    Some(json!({
        "source_path": source.source_path,
        "kind": source.kind,
        "create_context_count": create_context_count,
        "provider_count": provider_count,
        "consumer_count": consumer_count,
        "context_runtime_status": "provider-value-map-ready",
        "contexts": contexts,
        "full_react_context_runtime": false,
    }))
}

fn context_runtime_manifest(context_sources: &[Value]) -> Value {
    let mut contexts = BTreeSet::new();
    let mut initial_values = serde_json::Map::new();
    let mut provider_values = Vec::new();
    for source in context_sources {
        let Some(source_contexts) = source.get("contexts").and_then(Value::as_array) else {
            continue;
        };
        for context in source_contexts {
            let Some(name) = context.get("name").and_then(Value::as_str) else {
                continue;
            };
            contexts.insert(name.to_string());
            let safe_value = context.get("safe_literal_context_value").cloned();
            let provider_value_source = context
                .get("provider_value_source")
                .and_then(Value::as_str)
                .map(str::to_string);
            if let Some(value) = safe_value {
                if provider_value_source.is_some() || !initial_values.contains_key(name) {
                    initial_values.insert(name.to_string(), value);
                }
            }
            if let Some(source) = provider_value_source {
                provider_values.push(json!({
                    "name": name,
                    "source": source,
                    "safe_literal": context.get("safe_literal_context_value").is_some(),
                }));
            }
        }
    }
    let context_initial_values = initial_values.len();
    json!({
        "schema": "dx.tsx.contextRuntime",
        "schema_revision": 1,
        "contract_name": "Context Runtime",
        "status": if contexts.is_empty() {
            "no-contexts"
        } else {
            "provider-value-map-ready"
        },
        "strategy": "source-owned-provider-value-map",
        "event": "dx:context-value",
        "ready_event": "dx:context-runtime-ready",
        "context_names": contexts.into_iter().collect::<Vec<_>>(),
        "initial_values": initial_values,
        "context_initial_values": context_initial_values,
        "provider_values": provider_values,
        "node_modules_required": false,
        "full_react_context_runtime": false,
        "limits": [
            "Exposes detected contexts through getContext, setContext, and resolveContextValue.",
            "Seeds safe literal createContext defaults and Provider value attributes into the generated value map.",
            "Reflects values to data-dx-context-read markers for Studio/browser tooling.",
            "Does not execute dynamic provider expressions, React useContext, or provider tree reconciliation."
        ],
    })
}

fn collect_context_names(source: &str) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    collect_assignment_context_names(source, "createContext(", &mut names);
    collect_member_context_names(source, ".Provider", &mut names);
    collect_use_context_names(source, &mut names);
    names
}

fn collect_assignment_context_names(source: &str, marker: &str, output: &mut BTreeSet<String>) {
    let mut cursor = source;
    while let Some(index) = cursor.find(marker) {
        let before = &cursor[..index];
        if let Some(name) = before
            .rsplit_once('=')
            .map(|(left, _)| left)
            .and_then(last_identifier)
        {
            output.insert(name.to_string());
        }
        cursor = &cursor[index + marker.len()..];
    }
}

fn collect_member_context_names(source: &str, marker: &str, output: &mut BTreeSet<String>) {
    let mut cursor = source;
    while let Some(index) = cursor.find(marker) {
        if let Some(name) = last_identifier(&cursor[..index]) {
            output.insert(name.to_string());
        }
        cursor = &cursor[index + marker.len()..];
    }
}

fn collect_use_context_names(source: &str, output: &mut BTreeSet<String>) {
    let mut cursor = source;
    while let Some(index) = cursor.find("useContext(") {
        let candidate = cursor[index + "useContext(".len()..].trim_start();
        let name = candidate
            .chars()
            .take_while(|ch| ch.is_ascii_alphanumeric() || *ch == '_' || *ch == '$')
            .collect::<String>();
        if !name.is_empty() {
            output.insert(name);
        }
        cursor = candidate;
    }
}

fn last_identifier(source: &str) -> Option<&str> {
    let trimmed =
        source.trim_end_matches(|ch: char| !ch.is_ascii_alphanumeric() && ch != '_' && ch != '$');
    let start = trimmed
        .rfind(|ch: char| !ch.is_ascii_alphanumeric() && ch != '_' && ch != '$')
        .map(|index| index + 1)
        .unwrap_or(0);
    let candidate = &trimmed[start..];
    (!candidate.is_empty()).then_some(candidate)
}

fn initial_context_value_source(source: &str, name: &str) -> Option<String> {
    let assignment = format!("{name} = createContext(");
    let start = source.find(&assignment)? + assignment.len();
    let rest = &source[start..];
    let value = rest
        .chars()
        .take_while(|ch| *ch != ')')
        .collect::<String>()
        .trim()
        .to_string();
    (!value.is_empty()).then_some(value)
}

fn provider_context_value_source(source: &str, name: &str) -> Option<String> {
    let marker = format!("<{name}.Provider");
    let start = source.find(&marker)? + marker.len();
    let tag = source[start..]
        .split_once('>')
        .map(|(tag, _)| tag)
        .unwrap_or(&source[start..]);
    jsx_attribute_value_source(tag, "value")
}

fn jsx_attribute_value_source(tag: &str, attribute: &str) -> Option<String> {
    let mut cursor = tag;
    while let Some(index) = cursor.find(attribute) {
        let before = cursor[..index].chars().last();
        let after_name = &cursor[index + attribute.len()..];
        if before.is_some_and(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-')
            || !after_name.trim_start().starts_with('=')
        {
            cursor = after_name;
            continue;
        }
        let value = after_name.trim_start().strip_prefix('=')?.trim_start();
        return if let Some(quote) = value.chars().next().filter(|ch| *ch == '"' || *ch == '\'') {
            let rest = &value[quote.len_utf8()..];
            let end = rest.find(quote)?;
            Some(format!("{quote}{}{quote}", &rest[..end]))
        } else if let Some(rest) = value.strip_prefix('{') {
            let end = rest.find('}')?;
            Some(rest[..end].trim().to_string())
        } else {
            Some(
                value
                    .chars()
                    .take_while(|ch| !ch.is_whitespace() && *ch != '/' && *ch != '>')
                    .collect::<String>(),
            )
            .filter(|value| !value.is_empty())
        };
    }
    None
}

fn safe_literal_context_value(source: &str) -> Option<Value> {
    let value = source.trim();
    if value.is_empty() {
        return None;
    }
    let unwrapped = value
        .strip_prefix('{')
        .and_then(|value| value.strip_suffix('}'))
        .map(str::trim)
        .unwrap_or(value);
    if let Some(string) = quoted_value(unwrapped) {
        return Some(json!(string));
    }
    match unwrapped {
        "true" => Some(json!(true)),
        "false" => Some(json!(false)),
        "null" => Some(Value::Null),
        _ => unwrapped
            .parse::<i64>()
            .map(|value| json!(value))
            .or_else(|_| unwrapped.parse::<f64>().map(|value| json!(value)))
            .ok(),
    }
}

fn count_occurrences(source: &str, needle: &str) -> usize {
    source.match_indices(needle).count()
}

fn state_graph_semantics(state_graph: &DxStateGraph) -> Value {
    json!({
        "schema": "dx.tsx.stateGraph",
        "schema_revision": 1,
        "contract_name": "TSX State Graph",
        "compiler_owned": true,
        "react_authoring_source": "useState/useMemo/events/server-actions",
        "default_scope": state_graph.default_scope,
        "runtime_lowering": {
            "status": "source-graph-ready",
            "full_react_hook_runtime": false,
            "next_step": "lower state/event graph into generated JS client islands"
        },
        "counts": {
            "slots": state_graph.slots.len(),
            "derived_slots": state_graph.derived_slots.len(),
            "event_slots": state_graph.event_slots.len(),
            "effects": state_graph.effects.len(),
            "server_actions": state_graph.server_actions.len(),
        },
        "slots": &state_graph.slots,
        "derived_slots": &state_graph.derived_slots,
        "event_slots": &state_graph.event_slots,
        "effects": &state_graph.effects,
        "server_actions": &state_graph.server_actions,
    })
}

fn has_directive(source: &str, directive: &str) -> bool {
    source
        .lines()
        .take_while(|line| {
            let trimmed = line.trim();
            trimmed.is_empty()
                || trimmed.starts_with("//")
                || trimmed.starts_with('"')
                || trimmed.starts_with('\'')
        })
        .any(|line| {
            let trimmed = line.trim().trim_end_matches(';');
            trimmed == format!("\"{directive}\"") || trimmed == format!("'{directive}'")
        })
}

fn has_default_export(source: &str) -> bool {
    source.contains("export default")
}

fn accepts_children(source: &SemanticSource<'_>) -> bool {
    matches!(source.kind, "layout" | "template") && source.source.contains("children")
}

fn source_needs_client_boundary(source: &SemanticSource<'_>) -> bool {
    has_directive(source.source, "use client")
        || !collect_hook_usage(source.source).is_empty()
        || !collect_event_handlers(source.source).is_empty()
}

fn collect_imports(source: &str) -> Vec<String> {
    source
        .lines()
        .filter_map(import_specifier)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn import_specifier(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if !trimmed.starts_with("import ") {
        return None;
    }
    if let Some((_, after_from)) = trimmed.rsplit_once(" from ") {
        return quoted_value(after_from.trim().trim_end_matches(';'));
    }
    let rest = trimmed
        .strip_prefix("import ")?
        .trim()
        .trim_end_matches(';');
    quoted_value(rest)
}

fn quoted_value(value: &str) -> Option<String> {
    let quote = value
        .chars()
        .next()
        .filter(|ch| *ch == '"' || *ch == '\'')?;
    let rest = &value[quote.len_utf8()..];
    let end = rest.find(quote)?;
    Some(rest[..end].to_string())
}

fn import_kind(specifier: &str) -> &'static str {
    if specifier.starts_with("./") || specifier.starts_with("../") {
        "local"
    } else if specifier.starts_with("@/forge") || specifier.starts_with("forge/") {
        "forge"
    } else if specifier.starts_with("@/") {
        "workspace"
    } else if specifier.starts_with("dx:") {
        "dx-runtime"
    } else {
        "external"
    }
}

fn import_is_source_owned(specifier: &str) -> bool {
    matches!(
        import_kind(specifier),
        "local" | "forge" | "workspace" | "dx-runtime"
    )
}

fn collect_hook_usage(source: &str) -> Vec<String> {
    let searchable = source_without_comments_and_strings(source);
    react_hook_call_patterns(&searchable)
        .into_iter()
        .filter(|call| call.is_called_in(&searchable))
        .map(|call| call.canonical.to_string())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ReactHookCallPattern {
    canonical: &'static str,
    local: String,
    namespace: Option<String>,
}

impl ReactHookCallPattern {
    fn regex_source(&self) -> String {
        match self.namespace.as_deref() {
            Some(namespace) => format!(
                r#"\b{}\s*\.\s*{}"#,
                regex::escape(namespace),
                regex::escape(&self.local)
            ),
            None => format!(r#"\b{}"#, regex::escape(&self.local)),
        }
    }

    fn is_called_in(&self, source: &str) -> bool {
        regex::Regex::new(&format!(r#"{}\s*\("#, self.regex_source()))
            .ok()
            .is_some_and(|regex| regex.is_match(source))
    }
}

fn react_hook_names() -> &'static [&'static str] {
    &[
        "useState",
        "useEffect",
        "useLayoutEffect",
        "useInsertionEffect",
        "useMemo",
        "useCallback",
        "useReducer",
        "useRef",
        "useTransition",
        "useOptimistic",
        "useActionState",
        "useContext",
    ]
}

fn react_hook_call_patterns(source: &str) -> Vec<ReactHookCallPattern> {
    let hook_names = react_hook_names();
    let mut calls = BTreeSet::new();
    for hook in hook_names {
        calls.insert(ReactHookCallPattern {
            canonical: hook,
            local: (*hook).to_string(),
            namespace: None,
        });
    }
    for namespace in react_namespace_imports(source) {
        for hook in hook_names {
            calls.insert(ReactHookCallPattern {
                canonical: hook,
                local: (*hook).to_string(),
                namespace: Some(namespace.clone()),
            });
        }
    }
    for (canonical, local) in react_named_hook_imports(source) {
        calls.insert(ReactHookCallPattern {
            canonical,
            local,
            namespace: None,
        });
    }
    calls.into_iter().collect()
}

fn react_named_hook_imports(source: &str) -> Vec<(&'static str, String)> {
    let Ok(re) = regex::Regex::new(r#"import\s*\{([^}]*)\}\s*from\s*["']react["']"#) else {
        return Vec::new();
    };
    re.captures_iter(source)
        .flat_map(|capture| {
            capture
                .get(1)
                .map(|imports| imports.as_str())
                .unwrap_or_default()
                .split(',')
                .filter_map(react_named_hook_import)
                .collect::<Vec<_>>()
        })
        .collect()
}

fn react_named_hook_import(specifier: &str) -> Option<(&'static str, String)> {
    let mut parts = specifier.split_whitespace();
    let imported = parts.next()?;
    let imported = react_hook_names()
        .iter()
        .copied()
        .find(|hook| *hook == imported)?;
    let local = match (parts.next(), parts.next(), parts.next()) {
        (Some("as"), Some(alias), None) => alias,
        (None, None, None) => imported,
        _ => return None,
    };
    is_identifier(local).then(|| (imported, local.to_string()))
}

fn react_namespace_imports(source: &str) -> Vec<String> {
    let mut namespaces = BTreeSet::new();
    if let Ok(namespace_re) =
        regex::Regex::new(r#"import\s+\*\s+as\s+([A-Za-z_$][A-Za-z0-9_$]*)\s+from\s*["']react["']"#)
    {
        for capture in namespace_re.captures_iter(source) {
            if let Some(namespace) = capture.get(1) {
                namespaces.insert(namespace.as_str().to_string());
            }
        }
    }
    if let Ok(default_re) =
        regex::Regex::new(r#"import\s+([A-Za-z_$][A-Za-z0-9_$]*)(?:\s*,|\s+from\s*["']react["'])"#)
    {
        for capture in default_re.captures_iter(source) {
            if let Some(namespace) = capture.get(1) {
                namespaces.insert(namespace.as_str().to_string());
            }
        }
    }
    namespaces.into_iter().collect()
}

fn is_identifier(value: &str) -> bool {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    (first.is_ascii_alphabetic() || first == '_' || first == '$')
        && chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '$')
}

fn source_without_comments_and_strings(source: &str) -> String {
    let mut output = String::with_capacity(source.len());
    let mut chars = source.chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            '"' | '\'' | '`' => {
                output.push(' ');
                consume_string_like(&mut chars, ch, &mut output);
            }
            '/' if chars.peek() == Some(&'/') => {
                output.push(' ');
                output.push(' ');
                chars.next();
                for comment in chars.by_ref() {
                    if comment == '\n' {
                        output.push('\n');
                        break;
                    }
                    output.push(' ');
                }
            }
            '/' if chars.peek() == Some(&'*') => {
                output.push(' ');
                output.push(' ');
                chars.next();
                let mut previous = '\0';
                for comment in chars.by_ref() {
                    output.push(if comment == '\n' { '\n' } else { ' ' });
                    if previous == '*' && comment == '/' {
                        break;
                    }
                    previous = comment;
                }
            }
            _ => output.push(ch),
        }
    }
    output
}

fn consume_string_like<I>(chars: &mut std::iter::Peekable<I>, quote: char, output: &mut String)
where
    I: Iterator<Item = char>,
{
    let mut escaped = false;
    for ch in chars.by_ref() {
        output.push(if ch == '\n' { '\n' } else { ' ' });
        if escaped {
            escaped = false;
            continue;
        }
        if ch == '\\' {
            escaped = true;
            continue;
        }
        if ch == quote {
            break;
        }
    }
}

fn collect_event_handlers(source: &str) -> Vec<String> {
    let Ok(re) = regex::Regex::new(r#"\bon[A-Z][A-Za-z0-9_]*\s*="#) else {
        return Vec::new();
    };
    re.find_iter(source)
        .filter_map(|event| event.as_str().split_once('=').map(|(name, _)| name.trim()))
        .map(str::to_string)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn semantic_warnings(sources: &[SemanticSource<'_>], state_graph: &DxStateGraph) -> Vec<Value> {
    let mut warnings = Vec::new();
    for source in sources {
        for hook in collect_hook_usage(source.source) {
            let (status, _, next_action) =
                hook_support_status(source.source_path, source.source, &hook, state_graph);
            let diagnostic_code = react_hook_diagnostic_code(&hook, status);
            match status {
                "unsupported-react-hook-diagnostic" => {
                    warnings.push(json!({
                        "source_path": source.source_path,
                        "kind": "unsupported-react-hook",
                        "hook": hook,
                        "diagnostic_code": diagnostic_code,
                        "message": "React hook usage needs an explicit DX-native lowering or framework island; DX-WWW will not pretend hidden React runtime parity.",
                        "next_action": next_action,
                    }));
                }
                "effect-boundary-scheduled" => {
                    warnings.push(json!({
                        "source_path": source.source_path,
                        "kind": "react-effect-boundary",
                        "hook": hook,
                        "diagnostic_code": diagnostic_code,
                        "message": "React effect hook usage is recorded as a DX effect boundary; callback bodies and cleanup are not executed with hidden React semantics.",
                        "next_action": next_action,
                    }));
                }
                "semantic-boundary" => {
                    warnings.push(json!({
                        "source_path": source.source_path,
                        "kind": "react-semantic-boundary",
                        "hook": hook,
                        "diagnostic_code": diagnostic_code,
                        "message": "React memo/ref/callback intent is recorded as a semantic boundary; DX-WWW will not execute it through an implicit React runtime.",
                        "next_action": next_action,
                    }));
                }
                _ => {}
            }
        }
        for event in collect_event_handlers(source.source) {
            if dx_compiler::delivery::react_style_event_attribute_to_dom_event(&event).is_none() {
                warnings.push(json!({
                    "source_path": source.source_path,
                    "kind": "unsupported-react-event",
                    "attribute": event,
                    "failure_reason": react_event_failure_reason(&event, false),
                    "message": "React-style event attribute is not in the native DOM event catalog; DX-WWW will not attach a fake listener or claim React synthetic event parity.",
                    "next_action": "Use a supported native DOM event or mark the component as an explicit framework island.",
                }));
            }
        }
        if source.source.contains("import(") {
            warnings.push(json!({
                "source_path": source.source_path,
                "kind": "dynamic-import",
                "message": "Dynamic imports need an explicit compiler/runtime lane before www can claim full App Router parity."
            }));
        }
        if source.source.contains("createContext(") || source.source.contains("useContext(") {
            warnings.push(json!({
                "source_path": source.source_path,
                "kind": "react-context",
                "message": "React context usage is detected; www must lower or execute it explicitly instead of pretending generic React parity."
            }));
        }
        if source_needs_client_boundary(source) && !has_directive(source.source, "use client") {
            warnings.push(json!({
                "source_path": source.source_path,
                "kind": "implicit-client-boundary",
                "message": "Hooks or event handlers were detected without an explicit use client directive."
            }));
        }
    }
    warnings
}

fn unique_strings(values: Vec<String>) -> Vec<String> {
    values
        .into_iter()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

#[cfg(test)]
mod tests {
    use dx_compiler::delivery::{DxStateEventSlot, DxStateGraph, DxStateScope, DxStateSlot};

    use super::*;

    #[test]
    fn react_hook_support_reports_dx_native_policy_without_dummy_runtime() {
        let mut graph = state_graph_with_count_slot();
        graph.event_slots.push(DxStateEventSlot {
            id: "event:count-click".to_string(),
            source_path: "app/page.tsx".to_string(),
            element: "button".to_string(),
            event: "click".to_string(),
            handler: "() => setCount(count + 1)".to_string(),
            state_dependencies: vec!["count".to_string()],
            action: None,
        });
        graph.event_slots.push(DxStateEventSlot {
            id: "event:count-double-click".to_string(),
            source_path: "app/page.tsx".to_string(),
            element: "button".to_string(),
            event: "dblclick".to_string(),
            handler: "() => setCount(count + 2)".to_string(),
            state_dependencies: vec!["count".to_string()],
            action: None,
        });
        let report = build_tsx_app_router_semantics(
            "app/page.tsx",
            r#"import { useEffect, useReducer, useState } from "react";

export default function Page() {
  const [count, setCount] = useState(0);
  useEffect(() => console.log(count), [count]);
  useReducer((state) => state, 0);
  return (
    <button
      onClick={() => setCount(count + 1)}
      onDoubleClick={() => setCount(count + 2)}
      onMagicGesture={() => console.log(count)}
      onOnce_per={() => console.log(count)}
    >
      {count}
    </button>
  );
}
"#,
            &[],
            &graph,
        );
        let findings = report["react_hook_support"]["findings"]
            .as_array()
            .expect("hook findings");

        assert_hook_status(findings, "useState", "compatibility-lowered");
        assert_hook_status(findings, "useEffect", "effect-boundary-scheduled");
        assert_hook_status(findings, "useReducer", "unsupported-react-hook-diagnostic");
        let event_findings = report["react_event_support"]["findings"]
            .as_array()
            .expect("event findings");
        assert_event_status(event_findings, "onClick", "native-dom-event-supported");
        assert_event_status(
            event_findings,
            "onDoubleClick",
            "native-dom-event-supported",
        );
        assert_event_status(
            event_findings,
            "onMagicGesture",
            "unsupported-react-event-diagnostic",
        );
        assert_event_status(
            event_findings,
            "onOnce_per",
            "unsupported-react-event-diagnostic",
        );
        assert_eq!(
            report["react_hook_support"]["react_api_shim_executed"],
            false
        );
        assert!(
            report["warnings"]
                .as_array()
                .expect("warnings")
                .iter()
                .any(|warning| warning["kind"] == "unsupported-react-hook"
                    && warning["hook"] == "useReducer")
        );
        assert!(
            report["warnings"]
                .as_array()
                .expect("warnings")
                .iter()
                .any(|warning| warning["kind"] == "react-effect-boundary"
                    && warning["hook"] == "useEffect")
        );
        assert!(
            report["warnings"]
                .as_array()
                .expect("warnings")
                .iter()
                .any(|warning| warning["kind"] == "unsupported-react-event"
                    && warning["attribute"] == "onMagicGesture")
        );
        assert!(report["warnings"].as_array().expect("warnings").iter().any(
            |warning| warning["kind"] == "unsupported-react-event"
                && warning["attribute"] == "onOnce_per"
                && warning["failure_reason"] == "not-in-native-dom-event-catalog"
        ));
    }

    #[test]
    fn use_state_without_exact_state_graph_slot_is_diagnostic() {
        let report = build_tsx_app_router_semantics(
            "app/page.tsx",
            r#"import { useState } from "react";

export default function Page() {
  const [count] = useState(0);
  return <p>{count}</p>;
}
"#,
            &[],
            &DxStateGraph::default(),
        );
        let findings = report["react_hook_support"]["findings"]
            .as_array()
            .expect("hook findings");

        assert_hook_status(findings, "useState", "unsupported-react-hook-diagnostic");
        assert!(
            report["warnings"]
                .as_array()
                .expect("warnings")
                .iter()
                .any(|warning| warning["kind"] == "unsupported-react-hook"
                    && warning["hook"] == "useState")
        );
    }

    #[test]
    fn use_state_with_untracked_setter_call_is_diagnostic() {
        let report = build_tsx_app_router_semantics(
            "app/page.tsx",
            r#"import { useState } from "react";

export default function Page() {
  const [count, setCount] = useState(0);
  return <button onClick={() => setCount(count + 1)}>{count}</button>;
}
"#,
            &[],
            &state_graph_with_count_slot(),
        );
        let findings = report["react_hook_support"]["findings"]
            .as_array()
            .expect("hook findings");

        assert_hook_status(findings, "useState", "unsupported-react-hook-diagnostic");
        assert!(
            report["warnings"]
                .as_array()
                .expect("warnings")
                .iter()
                .any(|warning| warning["kind"] == "unsupported-react-hook"
                    && warning["hook"] == "useState")
        );
    }

    #[test]
    fn use_state_with_unlowerable_setter_event_is_diagnostic() {
        let mut graph = state_graph_with_count_slot();
        graph.event_slots.push(DxStateEventSlot {
            id: "event:count".to_string(),
            source_path: "app/page.tsx".to_string(),
            element: "button".to_string(),
            event: "click".to_string(),
            handler: "() => setCount(expensive(count))".to_string(),
            state_dependencies: vec!["count".to_string()],
            action: None,
        });
        let report = build_tsx_app_router_semantics(
            "app/page.tsx",
            r#"import { useState } from "react";

export default function Page() {
  const [count, setCount] = useState(0);
  return <button onClick={() => setCount(expensive(count))}>{count}</button>;
}
"#,
            &[],
            &graph,
        );
        let findings = report["react_hook_support"]["findings"]
            .as_array()
            .expect("hook findings");

        assert_hook_status(findings, "useState", "unsupported-react-hook-diagnostic");
        assert!(
            report["warnings"]
                .as_array()
                .expect("warnings")
                .iter()
                .any(|warning| warning["kind"] == "unsupported-react-hook"
                    && warning["hook"] == "useState")
        );
    }

    #[test]
    fn react_hook_detection_is_import_aware_and_ignores_comments_and_strings() {
        let hooks = collect_hook_usage(
            r#"import { useState as useS, useEffect } from "react";
import * as R from "react";

const fake = "useReducer()";
// useOptimistic()
/* useActionState() */

export default function Page() {
  const [count, setCount] = useS(0);
  R.useContext(ThemeContext);
  useEffect(() => {}, []);
  return <button onClick={() => setCount(count + 1)}>{count}</button>;
}
"#,
        );

        assert_eq!(hooks, vec!["useContext", "useEffect", "useState"]);
        let bindings = collect_use_state_bindings(
            r#"import { useState as useS } from "react";

export default function Page() {
  const [count, setCount] = useS(0);
  return count;
}
"#,
        );
        assert_eq!(bindings.len(), 1);
        assert_eq!(bindings[0].state_name, "count");
        assert_eq!(bindings[0].setter_name.as_deref(), Some("setCount"));
    }

    fn state_graph_with_count_slot() -> DxStateGraph {
        let mut graph = DxStateGraph::default();
        graph.slots.push(DxStateSlot {
            id: "slot:count".to_string(),
            name: "count".to_string(),
            setter: Some("setCount".to_string()),
            scope: DxStateScope::Local,
            source_path: "app/page.tsx".to_string(),
            initial_source: "0".to_string(),
            value_kind: "number".to_string(),
        });
        graph
    }

    fn assert_hook_status(findings: &[Value], hook: &str, status: &str) {
        assert!(
            findings
                .iter()
                .any(|finding| finding["hook"] == hook && finding["status"] == status),
            "expected {hook} to have status {status}"
        );
    }

    fn assert_event_status(findings: &[Value], attribute: &str, status: &str) {
        let finding = findings
            .iter()
            .find(|finding| finding["attribute"] == attribute && finding["status"] == status)
            .unwrap_or_else(|| panic!("expected {attribute} to have status {status}"));
        assert_eq!(
            finding["diagnostic_code"],
            react_event_diagnostic_code(attribute, status)
        );
        assert_eq!(
            finding["listener_attached"],
            status == "native-dom-event-supported"
        );
        assert_eq!(
            finding["adapter_boundary_required"],
            status != "native-dom-event-supported"
        );
        if status == "native-dom-event-supported" {
            assert!(finding["failure_reason"].is_null());
            assert_eq!(
                finding["binder_proof_status"],
                "native-add-event-listener-eligible"
            );
        } else {
            assert_eq!(finding["failure_reason"], "not-in-native-dom-event-catalog");
            assert_eq!(
                finding["binder_proof_status"],
                "diagnostic-only-no-listener"
            );
        }
    }
}
