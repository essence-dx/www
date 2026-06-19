struct ComponentInvocationChildHtml {
    html: String,
    text_children: usize,
    literal_child_expressions: usize,
    skipped_child_expressions: usize,
}

const NATIVE_EVENT_BROWSER_BINDER_RECEIPT_CONTRACT: &str =
    "dx.www.readiness.native_event_browser_binder_receipt_contract";
const NATIVE_EVENT_REACT_STYLE_EVENT_EXAMPLES: &[&str] = &["onClick", "onInput", "onPointerMove"];
const NATIVE_EVENT_DOM_EVENT_EXAMPLES: &[&str] = &["click", "input", "pointermove"];
const NATIVE_EVENT_UNSUPPORTED_EVENT_POLICY: &str =
    "diagnose unsupported React-style event attributes without attaching listeners or claiming React synthetic event parity";

fn component_invocation_child_html(
    document: &LoweredSourceDocument,
    element: &DxReactJsxElement,
    state_graph: &DxStateGraph,
    request_prop_bindings: &[ComponentPropBinding],
) -> ComponentInvocationChildHtml {
    let StaticChildSnapshot {
        html,
        literal_expressions,
        skipped_expressions,
        ..
    } = render_static_child_nodes(document, element, state_graph, request_prop_bindings, None);
    ComponentInvocationChildHtml {
        html,
        text_children: element.child_text.len(),
        literal_child_expressions: literal_expressions,
        skipped_child_expressions: skipped_expressions,
    }
}

fn component_runtime_binding_plan(
    document: &LoweredSourceDocument,
    state_graph: &DxStateGraph,
) -> Value {
    let state_slots = component_state_slot_bindings(document, state_graph);
    let event_slots = component_event_slot_bindings(document, state_graph);
    let intrinsic_forms = form_surfaces(document);
    let dom_action_descriptors = component_dom_action_descriptors(document, state_graph);
    let form_count = intrinsic_forms.len();
    let dom_action_descriptor_count = dom_action_descriptors.len();
    let field_count = intrinsic_forms
        .iter()
        .filter_map(|form| form.get("fields").and_then(Value::as_array))
        .map(Vec::len)
        .sum::<usize>();
    let submit_handler_count = intrinsic_forms
        .iter()
        .filter(|form| {
            form.get("has_submit_handler")
                .and_then(Value::as_bool)
                .unwrap_or(false)
        })
        .count();
    let state_slot_count = state_slots.len();
    let event_slot_count = event_slots.len();
    let has_bindings = state_slot_count > 0
        || event_slot_count > 0
        || form_count > 0
        || dom_action_descriptor_count > 0;

    json!({
        "schema": "dx.tsx.componentRuntimeBindingPlan",
        "schema_revision": 1,
        "contract_name": "Component Runtime Binding Plan",
        "status": if has_bindings { "bindings-discovered" } else { "static-preview-only" },
        "source_path": &document.source_path,
        "full_dom_binding": false,
        "event_listener_attachment": if event_slot_count == 0 && dom_action_descriptor_count == 0 { "not-needed" } else { "planned-from-state-graph" },
        "state_slot_count": state_slot_count,
        "event_slot_count": event_slot_count,
        "state_slots": state_slots,
        "event_slots": event_slots,
        "dom_action_descriptors": {
            "schema": "dx.tsx.domActionDescriptors",
            "schema_revision": 1,
            "contract_name": "DOM Action Descriptors",
            "status": if dom_action_descriptor_count == 0 { "no-safe-intrinsic-actions" } else { "safe-intrinsic-actions-planned" },
            "source_path": &document.source_path,
            "control_tags": ["button", "input", "textarea", "select", "form"],
            "descriptor_count": dom_action_descriptor_count,
            "descriptors": dom_action_descriptors,
            "browser_listeners_attached": false,
            "full_react_event_execution": false,
            "limits": [
                "Describes safe intrinsic button/input/form action targets for a future generated JS binder.",
                "Matches React-style event attributes and compiler state event slots without executing handler code.",
                "Does not attach browser listeners, run React synthetic events, or claim full React event parity."
            ],
        },
        "intrinsic_form_lowering": {
            "schema": "dx.tsx.intrinsicFormLowering",
            "schema_revision": 1,
            "contract_name": "Intrinsic Form Lowering",
            "status": if form_count == 0 { "no-forms" } else { "intrinsic-forms-discovered" },
            "forms": intrinsic_forms,
            "form_count": form_count,
            "field_count": field_count,
            "submit_handler_count": submit_handler_count,
            "native_submit_supported": true,
            "react_submit_handler_execution": false,
        },
        "limits": [
            "Matches state/event slots by source-owned component file before real DOM listener attachment.",
            "Carries intrinsic form metadata into the component return preview for source/check/Studio consumers.",
            "Does not attach browser events, run submit handlers, execute component functions, or claim React hydration parity."
        ],
    })
}

fn component_dom_action_descriptors(
    document: &LoweredSourceDocument,
    state_graph: &DxStateGraph,
) -> Vec<Value> {
    document
        .document
        .elements
        .iter()
        .filter_map(|element| intrinsic_dom_action_descriptor(document, element, state_graph))
        .collect()
}

fn source_dom_action_descriptor_sets(
    documents: &[LoweredSourceDocument],
    state_graph: &DxStateGraph,
) -> Vec<Value> {
    documents
        .iter()
        .filter_map(|document| {
            let descriptors = component_dom_action_descriptors(document, state_graph);
            if descriptors.is_empty() {
                return None;
            }
            Some(json!({
                "source_path": &document.source_path,
                "role": document.role,
                "descriptor_count": descriptors.len(),
                "descriptors": descriptors,
            }))
        })
        .collect()
}

fn build_dom_action_binder(descriptor_sets: &[Value], descriptor_count: usize) -> Value {
    let supported_events = dx_compiler::delivery::native_dom_event_names();
    let catalog_integrity = dx_compiler::delivery::native_dom_event_catalog_integrity();
    let script = if descriptor_count == 0 {
        None
    } else {
        Some(generated_dom_action_binder_script(descriptor_sets))
    };
    json!({
        "schema": "dx.tsx.domActionBinder",
        "schema_revision": 1,
        "contract_name": "DOM Action Binder",
        "status": if descriptor_count == 0 { "no-safe-actions" } else { "generated-js-binder-ready" },
        "script_id": "__DX_DOM_ACTION_BINDER__",
        "descriptor_count": descriptor_count,
        "descriptor_sets": descriptor_sets,
        "supported_events": supported_events,
        "supported_event_count": supported_events.len(),
        "catalog_source": catalog_integrity.source_of_truth,
        "catalog_hash": catalog_integrity.catalog_hash.clone(),
        "compiler_integrity": catalog_integrity,
        "readiness_receipt_contract": NATIVE_EVENT_BROWSER_BINDER_RECEIPT_CONTRACT,
        "react_style_event_examples": NATIVE_EVENT_REACT_STYLE_EVENT_EXAMPLES,
        "dom_event_examples": NATIVE_EVENT_DOM_EVENT_EXAMPLES,
        "unsupported_event_policy": NATIVE_EVENT_UNSUPPORTED_EVENT_POLICY,
        "receipt_ready_fields": [
            "binder_global_present",
            "supported_event_count",
            "catalog_hash",
            "listener_events",
            "unsupported_listener_attached",
            "preview_event_count",
            "state_dispatch_count",
        ],
        "node_modules_required": false,
        "browser_api": "addEventListener + CustomEvent",
        "ready_event": "dx:dom-action-binder-ready",
        "preview_event": "dx:dom-action-preview",
        "state_runtime_bridge": build_dom_action_state_bridge(descriptor_count),
        "script": script,
        "full_react_event_parity": false,
        "react_synthetic_events": false,
        "limits": [
            "Generates a no-node_modules browser binder for safe intrinsic action previews.",
            "Attaches preview listeners for the source-owned native DOM event catalog when React-style attributes are present.",
            "Dispatches DX preview events instead of executing React handler bodies or React synthetic events."
        ],
    })
}

fn build_client_island_manifest(
    route: &str,
    descriptor_sets: &[Value],
    descriptor_count: usize,
    state_graph: &DxStateGraph,
) -> Value {
    let mut islands = Vec::new();
    for (set_index, set) in descriptor_sets.iter().enumerate() {
        let set_source_path = set
            .get("source_path")
            .and_then(Value::as_str)
            .unwrap_or("unknown");
        let Some(descriptors) = set.get("descriptors").and_then(Value::as_array) else {
            continue;
        };
        for (descriptor_index, descriptor) in descriptors.iter().enumerate() {
            let source_path = descriptor
                .get("source_path")
                .and_then(Value::as_str)
                .unwrap_or(set_source_path);
            islands.push(json!({
                "schema": "dx.tsx.clientIsland",
                "schema_revision": 1,
                "id": format!("client-island-{set_index}-{descriptor_index}"),
                "kind": "intrinsic-dom-action-island",
                "route": route,
                "source_path": source_path,
                "target": client_island_target_descriptor(descriptor),
                "events": client_island_events(descriptor),
                "event_slot_ids": client_island_event_slot_ids(descriptor),
                "state_slots": client_island_state_slots(source_path, state_graph),
                "binder": "generated-dom-action-binder",
                "binder_global": "__DX_DOM_ACTION_BINDER__",
                "state_runtime_global": "__DX_STATE_GRAPH_RUNTIME__",
                "node_modules_required": false,
                "full_react_hydration": false,
                "react_synthetic_events": false,
            }));
        }
    }

    if islands.is_empty() {
        let mut state_sources = BTreeSet::new();
        for slot in &state_graph.slots {
            state_sources.insert(normalize_source_path(&slot.source_path));
        }
        for (index, source_path) in state_sources.into_iter().enumerate() {
            islands.push(json!({
                "schema": "dx.tsx.clientIsland",
                "schema_revision": 1,
                "id": format!("client-island-state-{index}"),
                "kind": "state-runtime-only-island",
                "route": route,
                "source_path": source_path,
                "target": null,
                "events": [],
                "event_slot_ids": [],
                "state_slots": client_island_state_slots(&source_path, state_graph),
                "binder": "state-runtime",
                "state_runtime_global": "__DX_STATE_GRAPH_RUNTIME__",
                "node_modules_required": false,
                "full_react_hydration": false,
                "react_synthetic_events": false,
            }));
        }
    }

    let island_count = islands.len();
    json!({
        "schema": "dx.tsx.clientIslandManifest",
        "schema_revision": 1,
        "contract_name": "TSX Client Island Manifest",
        "abi": client_island_abi_metadata(),
        "route": route,
        "status": if island_count == 0 {
            "no-client-islands"
        } else if descriptor_count == 0 {
            "state-runtime-only"
        } else {
            "generated-client-islands-ready"
        },
        "island_count": island_count,
        "descriptor_count": descriptor_count,
        "state_slot_count": state_graph.slots.len(),
        "event_slot_count": state_graph.event_slots.len(),
        "islands": islands,
        "binder": "generated-dom-action-binder",
        "binder_script_id": "__DX_DOM_ACTION_BINDER__",
        "manifest_script_id": "__DX_TSX_CLIENT_ISLANDS__",
        "state_runtime_global": "__DX_STATE_GRAPH_RUNTIME__",
        "node_modules_required": false,
        "full_react_hydration": false,
        "full_react_event_parity": false,
        "limits": [
            "Publishes safe source-owned client island targets for generated DOM action binding.",
            "Connects intrinsic events to the compiler-owned state runtime without React synthetic events.",
            "Does not execute arbitrary client components, effects, context providers, or full React hydration."
        ],
    })
}

fn client_island_abi_metadata() -> Value {
    let capabilities = dx_compiler::delivery::react_client_island_abi_capabilities();
    json!({
        "schema": capabilities.compiler_abi_schema,
        "schema_revision": capabilities.schema_revision,
        "capabilities_schema": capabilities.schema,
        "directive_style": capabilities.directive_style,
        "directive_style_id": capabilities.directive_style_id,
        "supported_directives": capabilities.supported_directives,
        "unsupported_directive_syntax": capabilities.unsupported_directive_syntax,
        "source_owned_runtime": capabilities.source_owned_runtime,
        "node_modules_required": capabilities.node_modules_required,
        "full_react_hydration": capabilities.full_react_hydration,
        "no_js_fallback_required": capabilities.no_js_fallback_required,
        "adapter_boundary_required": capabilities.adapter_boundary_required,
        "explicit_frameworks_policy": capabilities.explicit_frameworks_policy,
        "framework_adapters": "preview-only",
        "readiness_release_ready": capabilities.readiness_release_ready,
        "browser_proof_status": capabilities.browser_proof_status,
    })
}

fn client_island_target_descriptor(descriptor: &Value) -> Value {
    json!({
        "kind": descriptor.get("kind").cloned().unwrap_or(Value::Null),
        "tag": descriptor.get("tag").cloned().unwrap_or(Value::Null),
        "name": descriptor.get("name").cloned().unwrap_or(Value::Null),
        "type": descriptor.get("type").cloned().unwrap_or(Value::Null),
        "text": descriptor.get("text").cloned().unwrap_or(Value::Null),
    })
}

fn client_island_events(descriptor: &Value) -> Vec<String> {
    let mut events = BTreeSet::new();
    if let Some(attributes) = descriptor.get("event_attributes").and_then(Value::as_array) {
        for attribute in attributes {
            if let Some(event) = attribute.get("dom_event").and_then(Value::as_str) {
                events.insert(event.to_string());
            }
        }
    }
    if let Some(slots) = descriptor.get("event_slots").and_then(Value::as_array) {
        for slot in slots {
            if let Some(event) = slot.get("event").and_then(Value::as_str) {
                events.insert(event.to_string());
            }
        }
    }
    match descriptor.get("kind").and_then(Value::as_str) {
        Some("button-event") if events.is_empty() => {
            events.insert("click".to_string());
        }
        Some("input-binding") if events.is_empty() => {
            events.insert("change".to_string());
            events.insert("input".to_string());
        }
        Some("form-submit") => {
            events.insert("submit".to_string());
        }
        _ => {}
    }
    events.into_iter().collect()
}

fn client_island_event_slot_ids(descriptor: &Value) -> Vec<String> {
    descriptor
        .get("event_slots")
        .and_then(Value::as_array)
        .map(|slots| {
            slots
                .iter()
                .filter_map(|slot| slot.get("id").and_then(Value::as_str))
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

fn client_island_state_slots(source_path: &str, state_graph: &DxStateGraph) -> Vec<Value> {
    state_graph
        .slots
        .iter()
        .filter(|slot| same_source_path(&slot.source_path, source_path))
        .map(state_slot_binding)
        .collect()
}

fn build_dom_action_state_bridge(descriptor_count: usize) -> Value {
    json!({
        "schema": "dx.tsx.domActionStateBridge",
        "schema_revision": 1,
        "contract_name": "DOM Action State Bridge",
        "status": if descriptor_count == 0 { "no-safe-actions" } else { "preview-events-dispatch-to-state-runtime" },
        "state_runtime_global": "__DX_STATE_GRAPH_RUNTIME__",
        "state_runtime_dispatcher": "window.__DX_STATE_GRAPH_RUNTIME__.dispatch",
        "dispatcher": "dispatchDomActionPreviewToStateRuntime",
        "dispatch_event": "dx:state-runtime-dispatch",
        "preview_event": "dx:dom-action-preview",
        "node_modules_required": false,
        "full_react_hook_parity": false,
        "react_synthetic_events": false,
        "limits": [
            "Forwards safe DOM action previews to compiler-owned state event slots.",
            "Uses the TSX state runtime dispatcher instead of executing React handler bodies.",
            "Does not claim full React hook ordering, synthetic events, effects, or component execution."
        ],
    })
}

fn generated_dom_action_binder_script(descriptor_sets: &[Value]) -> String {
    let descriptor_json =
        serde_json::to_string(descriptor_sets).unwrap_or_else(|_| "[]".to_string());
    let supported_events_json =
        serde_json::to_string(dx_compiler::delivery::native_dom_event_names())
            .unwrap_or_else(|_| "[]".to_string());
    let catalog_integrity = dx_compiler::delivery::native_dom_event_catalog_integrity();
    let catalog_hash_json = serde_json::to_string(&catalog_integrity.catalog_hash)
        .unwrap_or_else(|_| "\"blake3:unknown\"".to_string());
    let receipt_contract_json = serde_json::to_string(NATIVE_EVENT_BROWSER_BINDER_RECEIPT_CONTRACT)
        .unwrap_or_else(|_| {
            "\"dx.www.readiness.native_event_browser_binder_receipt_contract\"".to_string()
        });
    let react_style_event_examples_json =
        serde_json::to_string(NATIVE_EVENT_REACT_STYLE_EVENT_EXAMPLES)
            .unwrap_or_else(|_| "[]".to_string());
    let dom_event_examples_json =
        serde_json::to_string(NATIVE_EVENT_DOM_EVENT_EXAMPLES).unwrap_or_else(|_| "[]".to_string());
    let unsupported_event_policy_json =
        serde_json::to_string(NATIVE_EVENT_UNSUPPORTED_EVENT_POLICY).unwrap_or_else(|_| {
            "\"diagnose unsupported React-style event attributes without attaching listeners or claiming React synthetic event parity\"".to_string()
        });
    r#"(function () {
  const descriptorSets = __DX_DESCRIPTOR_SETS__;
  const supportedEvents = __DX_SUPPORTED_DOM_EVENTS__;
  const contract = {
    schema: "dx.tsx.domActionBinder",
    schema_revision: 1,
    source: "generated-js-dom-binder",
    node_modules_required: false,
    supported_events: supportedEvents,
    supported_event_count: supportedEvents.length,
    catalog_hash: __DX_SUPPORTED_DOM_EVENT_CATALOG_HASH__,
    catalog_source: "core/src/delivery/dom_events.rs::NATIVE_DOM_EVENT_NAMES",
    readiness_receipt_contract: __DX_NATIVE_EVENT_BROWSER_BINDER_RECEIPT_CONTRACT__,
    react_style_event_examples: __DX_REACT_STYLE_EVENT_EXAMPLES__,
    dom_event_examples: __DX_DOM_EVENT_EXAMPLES__,
    unsupported_event_policy: __DX_UNSUPPORTED_EVENT_POLICY__,
    full_react_event_parity: false,
    react_synthetic_events: false
  };
  const stateRuntimeBridge = {
    schema: "dx.tsx.domActionStateBridge",
    schema_revision: 1,
    status: "preview-events-dispatch-to-state-runtime",
    state_runtime_global: "__DX_STATE_GRAPH_RUNTIME__",
    state_runtime_dispatcher: "window.__DX_STATE_GRAPH_RUNTIME__.dispatch",
    dispatcher: "dispatchDomActionPreviewToStateRuntime",
    dispatch_event: "dx:state-runtime-dispatch",
    full_react_hook_parity: false,
    react_synthetic_events: false
  };
  const listenerEvents = new Set();
  let unsupportedListenerAttached = false;
  let previewEventCount = 0;
  let stateDispatchCount = 0;
  let interactionClassApplicationCount = 0;
  function descriptorList() {
    return descriptorSets.reduce(function (output, set) {
      const descriptors = Array.isArray(set.descriptors) ? set.descriptors : [];
      descriptors.forEach(function (descriptor) {
        output.push(Object.assign({ source_path: set.source_path || null }, descriptor));
      });
      return output;
    }, []);
  }
  function eventNames(descriptor) {
    const names = new Set();
    const eventAttributeCount = Array.isArray(descriptor.event_attributes) ? descriptor.event_attributes.length : 0;
    (descriptor.event_attributes || []).forEach(function (attribute) {
      if (attribute && attribute.dom_event) names.add(attribute.dom_event);
    });
    (descriptor.event_slots || []).forEach(function (slot) {
      if (slot && slot.event) names.add(slot.event);
    });
    if (descriptor.kind === "button-event" && names.size === 0 && eventAttributeCount === 0) names.add("click");
    if (descriptor.kind === "input-binding" && names.size === 0) {
      names.add("input");
      names.add("change");
    }
    if (descriptor.kind === "form-submit") names.add("submit");
    return Array.from(names).filter(function (name) {
      return supportedEvents.includes(name);
    });
  }
  function eventSlotIdsFor(descriptor, eventName) {
    const slots = Array.isArray(descriptor.event_slots) ? descriptor.event_slots : [];
    return slots.filter(function (slot) {
      return slot && slot.id && (!slot.event || slot.event === eventName);
    }).map(function (slot) {
      return slot.id;
    });
  }
  function interactionClassesFor(descriptor, eventName) {
    const attributes = Array.isArray(descriptor.event_attributes) ? descriptor.event_attributes : [];
    return attributes.reduce(function (classes, attribute) {
      if (!attribute || attribute.dom_event !== eventName || attribute.handler_kind !== "interaction-classes") return classes;
      const attributeClasses = Array.isArray(attribute.interaction_classes) ? attribute.interaction_classes : [];
      attributeClasses.forEach(function (className) {
        if (typeof className === "string" && className && !classes.includes(className)) classes.push(className);
      });
      return classes;
    }, []);
  }
  function applyInteractionClasses(element, classes) {
    if (!classes.length) return;
    if (element.classList && typeof element.classList.add === "function") {
      element.classList.add.apply(element.classList, classes);
    } else {
      const existing = (element.getAttribute("class") || "").split(/\s+/).filter(Boolean);
      classes.forEach(function (className) {
        if (!existing.includes(className)) existing.push(className);
      });
      element.setAttribute("class", existing.join(" "));
    }
    interactionClassApplicationCount += 1;
    element.setAttribute("data-dx-interaction-class-applied", classes.join(" "));
  }
  function matchesDescriptor(element, descriptor) {
    if (!element || (descriptor.tag && element.localName !== descriptor.tag)) return false;
    if (descriptor.name && element.getAttribute("name") !== descriptor.name) return false;
    if (descriptor.type && descriptor.tag === "input") {
      return (element.getAttribute("type") || "text") === descriptor.type;
    }
    return true;
  }
  function dispatchDomActionPreviewToStateRuntime(detail, element) {
    const runtime = window.__DX_STATE_GRAPH_RUNTIME__;
    const eventSlotIds = Array.isArray(detail.event_slot_ids) ? detail.event_slot_ids : [];
    if (!runtime || typeof runtime.dispatch !== "function") {
      element.setAttribute("data-dx-state-runtime-dispatch", "missing-runtime");
      return { ok: false, reason: "missing-state-runtime" };
    }
    if (eventSlotIds.length === 0) {
      const interactionClasses = Array.isArray(detail.interaction_classes) ? detail.interaction_classes : [];
      if (interactionClasses.length > 0) {
        element.setAttribute("data-dx-state-runtime-dispatch", "not-needed");
        return { ok: true, reason: "interaction-classes-only" };
      }
      element.setAttribute("data-dx-state-runtime-dispatch", "no-event-slot");
      return { ok: false, reason: "no-event-slot" };
    }
    const payload = {
      schema: "dx.tsx.domActionStateDispatch",
      schema_revision: 1,
      source: "dom-action-binder",
      kind: detail.kind,
      event: detail.event,
      source_path: detail.source_path,
      name: detail.name,
      value: detail.value,
      checked: detail.checked,
      full_react_hook_parity: false
    };
    const results = eventSlotIds.map(function (eventSlotId) {
      return runtime.dispatch(eventSlotId, payload);
    });
    stateDispatchCount += results.length;
    const ok = results.some(function (result) {
      return result && result.ok;
    });
    element.setAttribute("data-dx-state-runtime-dispatch", ok ? "dispatched" : "not-dispatched");
    window.dispatchEvent(new CustomEvent("dx:state-runtime-dispatch", {
      detail: {
        schema: "dx.tsx.domActionStateDispatch",
        schema_revision: 1,
        event_slot_ids: eventSlotIds,
        preview: detail,
        results,
        full_react_hook_parity: false
      }
    }));
    return { ok, results };
  }
  function emitPreview(element, descriptor, eventName, event) {
    if (descriptor.kind === "form-submit") event.preventDefault();
    previewEventCount += 1;
    const eventSlotIds = eventSlotIdsFor(descriptor, eventName);
    const interactionClasses = interactionClassesFor(descriptor, eventName);
    applyInteractionClasses(element, interactionClasses);
    const detail = {
      schema: "dx.tsx.domActionPreview",
      schema_revision: 1,
      kind: descriptor.kind,
      tag: descriptor.tag,
      event: eventName,
      event_slot_ids: eventSlotIds,
      source_path: descriptor.source_path || null,
      name: descriptor.name || null,
      value: "value" in element ? element.value : null,
      checked: "checked" in element ? element.checked : null,
      interaction_classes: interactionClasses,
      interaction_class_count: interactionClasses.length
    };
    element.setAttribute("data-dx-dom-action-last-event", eventName);
    window.dispatchEvent(new CustomEvent("dx:dom-action-preview", { detail }));
    dispatchDomActionPreviewToStateRuntime(detail, element);
  }
  function attachSafeDomActionBinder(root) {
    const descriptors = descriptorList();
    const bound = [];
    descriptors.forEach(function (descriptor, index) {
      if (!descriptor.tag) return;
      root.querySelectorAll(descriptor.tag).forEach(function (element) {
        if (!matchesDescriptor(element, descriptor)) return;
        const key = index + ":" + descriptor.kind;
        const existing = element.getAttribute("data-dx-dom-action-bound") || "";
        if (existing.split(" ").includes(key)) return;
        element.setAttribute("data-dx-dom-action-bound", [existing, key].filter(Boolean).join(" "));
        const events = eventNames(descriptor);
        events.forEach(function (eventName) {
          if (!supportedEvents.includes(eventName)) {
            unsupportedListenerAttached = true;
            return;
          }
          listenerEvents.add(eventName);
          element.addEventListener(eventName, function (event) {
            emitPreview(element, descriptor, eventName, event);
          });
        });
        bound.push({
          kind: descriptor.kind,
          tag: descriptor.tag,
          source_path: descriptor.source_path || null,
          events
        });
      });
    });
    return {
      contract,
      stateRuntimeBridge,
      descriptorSets,
      bound,
      getSnapshot() {
        return {
          schema: "dx.tsx.domActionBinderSnapshot",
          schema_revision: 1,
          readiness_receipt_contract: contract.readiness_receipt_contract,
          descriptor_count: descriptors.length,
          bound_count: bound.length,
          binder_global_present: window.__DX_DOM_ACTION_BINDER__ === api,
          node_modules_required: false,
          catalog_source: contract.catalog_source,
          catalog_hash: contract.catalog_hash,
          supported_event_count: contract.supported_event_count,
          listener_events: Array.from(listenerEvents).sort(),
          unsupported_listener_attached: unsupportedListenerAttached,
          preview_event_count: previewEventCount,
          state_dispatch_count: stateDispatchCount,
          interaction_class_application_count: interactionClassApplicationCount,
          state_runtime_bridge: stateRuntimeBridge.status,
          release_ready: false,
          fastest_world_claim: false,
          full_react_hook_parity: false,
          full_react_event_parity: false,
          proof_scope: "local-browser-native-event-binder-replay-required"
        };
      }
    };
  }
  const api = attachSafeDomActionBinder(document);
  window.__DX_DOM_ACTION_BINDER__ = api;
  window.dispatchEvent(new CustomEvent("dx:dom-action-binder-ready", { detail: api.getSnapshot() }));
})();"#
    .replace("__DX_DESCRIPTOR_SETS__", &descriptor_json)
    .replace("__DX_SUPPORTED_DOM_EVENTS__", &supported_events_json)
    .replace("__DX_SUPPORTED_DOM_EVENT_CATALOG_HASH__", &catalog_hash_json)
    .replace(
        "__DX_NATIVE_EVENT_BROWSER_BINDER_RECEIPT_CONTRACT__",
        &receipt_contract_json,
    )
    .replace(
        "__DX_REACT_STYLE_EVENT_EXAMPLES__",
        &react_style_event_examples_json,
    )
    .replace("__DX_DOM_EVENT_EXAMPLES__", &dom_event_examples_json)
    .replace("__DX_UNSUPPORTED_EVENT_POLICY__", &unsupported_event_policy_json)
    .replace("</script", "<\\/script")
}

fn intrinsic_dom_action_descriptor(
    document: &LoweredSourceDocument,
    element: &DxReactJsxElement,
    state_graph: &DxStateGraph,
) -> Option<Value> {
    match element.name.as_str() {
        "button" => button_dom_action_descriptor(document, element, state_graph),
        "input" | "textarea" | "select" => {
            input_dom_action_descriptor(document, element, state_graph)
        }
        "form" => Some(form_dom_action_descriptor(document, element, state_graph)),
        _ => None,
    }
}

fn button_dom_action_descriptor(
    document: &LoweredSourceDocument,
    element: &DxReactJsxElement,
    state_graph: &DxStateGraph,
) -> Option<Value> {
    let event_attributes = element_event_attributes(element);
    let event_slots = element_event_slot_bindings(document, element, state_graph);
    if event_attributes.is_empty() && event_slots.is_empty() {
        return None;
    }
    Some(json!({
        "kind": "button-event",
        "tag": &element.name,
        "source_path": &document.source_path,
        "text": element.text_content(),
        "type": element.attribute("type").unwrap_or("button"),
        "event_attributes": event_attributes,
        "event_slots": event_slots,
        "browser_listener_attached": false,
        "react_synthetic_event_execution": false,
    }))
}

fn input_dom_action_descriptor(
    document: &LoweredSourceDocument,
    element: &DxReactJsxElement,
    state_graph: &DxStateGraph,
) -> Option<Value> {
    let event_attributes = element_event_attributes(element);
    let event_slots = element_event_slot_bindings(document, element, state_graph);
    let controlled = element
        .attributes
        .iter()
        .any(|attribute| attribute.name == "value" || attribute.name == "checked");
    let named = element.attribute("name").is_some();
    let required = element
        .attributes
        .iter()
        .any(|attribute| attribute.name == "required");
    if event_attributes.is_empty() && event_slots.is_empty() && !controlled && !named && !required {
        return None;
    }
    Some(json!({
        "kind": "input-binding",
        "tag": &element.name,
        "source_path": &document.source_path,
        "name": element.attribute("name"),
        "type": element.attribute("type").unwrap_or("text"),
        "required": required,
        "controlled": controlled,
        "event_attributes": event_attributes,
        "event_slots": event_slots,
        "browser_listener_attached": false,
        "react_synthetic_event_execution": false,
    }))
}

fn form_dom_action_descriptor(
    document: &LoweredSourceDocument,
    element: &DxReactJsxElement,
    state_graph: &DxStateGraph,
) -> Value {
    let event_attributes = element_event_attributes(element);
    let event_slots = element_event_slot_bindings(document, element, state_graph);
    json!({
        "kind": "form-submit",
        "tag": &element.name,
        "source_path": &document.source_path,
        "method": element.attribute("method").unwrap_or("get"),
        "action": element.attribute("action"),
        "has_submit_handler": element.attributes.iter().any(|attribute| attribute.name == "onSubmit"),
        "event_attributes": event_attributes,
        "event_slots": event_slots,
        "native_submit_supported": true,
        "browser_listener_attached": false,
        "react_submit_handler_execution": false,
    })
}

fn element_event_attributes(element: &DxReactJsxElement) -> Vec<Value> {
    element
        .attributes
        .iter()
        .filter(|attribute| is_event_attribute(&attribute.name))
        .map(|attribute| {
            let dom_event = dom_event_name_from_react_attribute(&attribute.name);
            let support_status = event_attribute_support_status(dom_event.as_deref());
            let interaction_classes = event_interaction_classes(attribute);
            let handler_kind = if !interaction_classes.is_empty() {
                "interaction-classes"
            } else if attribute.expression.is_some() {
                "expression-handler"
            } else {
                "literal-handler"
            };
            json!({
                "name": &attribute.name,
                "dom_event": dom_event,
                "support_status": support_status,
                "next_action": event_attribute_next_action(support_status),
                "kind": attribute_kind(attribute),
                "handler_kind": handler_kind,
                "expression": attribute.expression.as_deref().or(attribute.value.as_deref()),
                "interaction_classes": interaction_classes,
                "interaction_class_runtime": if handler_kind == "interaction-classes" { "class-list-application" } else { "none" },
            })
        })
        .collect()
}

fn event_interaction_classes(attribute: &DxReactJsxAttribute) -> Vec<String> {
    if attribute.expression.is_some() {
        return Vec::new();
    }
    attribute
        .value
        .as_deref()
        .and_then(safe_interaction_class_tokens)
        .unwrap_or_default()
}

fn safe_interaction_class_tokens(value: &str) -> Option<Vec<String>> {
    let tokens = value
        .split_whitespace()
        .filter(|token| !token.is_empty())
        .collect::<Vec<_>>();
    if tokens.is_empty()
        || !tokens
            .iter()
            .all(|token| token.chars().all(is_safe_interaction_class_char))
    {
        return None;
    }
    Some(tokens.into_iter().map(str::to_string).collect())
}

fn is_safe_interaction_class_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric()
        || matches!(
            ch,
            '-' | '_' | ':' | '/' | '[' | ']' | '(' | ')' | '.' | '%' | '#' | '@' | '!' | '&'
                | '*' | '+' | '~' | ','
        )
}

fn element_event_slot_bindings(
    document: &LoweredSourceDocument,
    element: &DxReactJsxElement,
    state_graph: &DxStateGraph,
) -> Vec<Value> {
    state_graph
        .event_slots
        .iter()
        .filter(|event| same_source_path(&event.source_path, &document.source_path))
        .filter(|event| event.element == element.name)
        .map(event_slot_binding)
        .collect()
}

fn dom_event_name_from_react_attribute(attribute_name: &str) -> Option<String> {
    dx_compiler::delivery::react_style_event_attribute_to_dom_event(attribute_name)
}

fn event_attribute_support_status(dom_event: Option<&str>) -> &'static str {
    if dom_event.is_some() {
        "native-dom-event-supported"
    } else {
        "unsupported-react-event-diagnostic"
    }
}

fn event_attribute_next_action(support_status: &str) -> &'static str {
    if support_status == "native-dom-event-supported" {
        "Attach through the generated native DOM action binder."
    } else {
        "Use a native DOM event from the DX catalog or add an explicit adapter boundary."
    }
}

fn component_state_slot_bindings(
    document: &LoweredSourceDocument,
    state_graph: &DxStateGraph,
) -> Vec<Value> {
    state_graph
        .slots
        .iter()
        .filter(|slot| same_source_path(&slot.source_path, &document.source_path))
        .map(state_slot_binding)
        .collect()
}

fn state_slot_binding(slot: &DxStateSlot) -> Value {
    json!({
        "id": &slot.id,
        "name": &slot.name,
        "setter": &slot.setter,
        "scope": &slot.scope,
        "initial_source": &slot.initial_source,
        "value_kind": &slot.value_kind,
    })
}

fn component_event_slot_bindings(
    document: &LoweredSourceDocument,
    state_graph: &DxStateGraph,
) -> Vec<Value> {
    state_graph
        .event_slots
        .iter()
        .filter(|event| same_source_path(&event.source_path, &document.source_path))
        .map(event_slot_binding)
        .collect()
}

fn event_slot_binding(event: &DxStateEventSlot) -> Value {
    json!({
        "id": &event.id,
        "element": &event.element,
        "event": &event.event,
        "handler": &event.handler,
        "state_dependencies": &event.state_dependencies,
        "server_action": &event.action,
    })
}

fn same_source_path(left: &str, right: &str) -> bool {
    normalize_source_path(left) == normalize_source_path(right)
}

fn normalize_source_path(path: &str) -> String {
    path.replace('\\', "/").trim_start_matches("./").to_string()
}
