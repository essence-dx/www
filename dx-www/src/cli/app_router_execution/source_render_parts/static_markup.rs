struct ComponentReturnPreview {
    html: String,
    elements: Vec<Value>,
    element_count: usize,
    children_placeholder_count: usize,
    children_insertions: usize,
    prop_identifier_bindings: usize,
    skipped_attributes: usize,
    literal_expressions: usize,
    skipped_child_expressions: usize,
}

fn static_document_preview_with_children(
    document: &LoweredSourceDocument,
    invocation_children_html: &str,
    prop_bindings: &[ComponentPropBinding],
    prop_aliases: &ComponentDestructuredPropAliases,
) -> ComponentReturnPreview {
    let mut html_fragments = Vec::new();
    let mut elements = Vec::new();
    let mut children_placeholder_count = 0usize;
    let mut children_insertions = 0usize;
    let mut prop_identifier_bindings = 0usize;
    let mut skipped_attributes = 0usize;
    let mut literal_expressions = 0usize;
    let mut skipped_child_expressions = 0usize;

    for element_index in root_jsx_elements(document) {
        let Some(element) = document.document.elements.get(element_index) else {
            continue;
        };
        if !is_static_renderable_element(document, element) {
            continue;
        }
        let preview = static_element_preview_with_children(
            document,
            element,
            invocation_children_html,
            prop_bindings,
            prop_aliases,
        );
        children_placeholder_count += preview.children_placeholder_count;
        children_insertions += preview.children_insertions;
        prop_identifier_bindings += preview.prop_identifier_bindings;
        skipped_attributes += preview.skipped_attributes;
        literal_expressions += preview.literal_expressions;
        skipped_child_expressions += preview.skipped_child_expressions;
        let preview_html = preview.html;
        html_fragments.push(preview_html.clone());
        elements.push(json!({
            "source_path": &document.source_path,
            "role": document.role,
            "tag": &element.name,
            "html": preview_html,
            "text": element.text_content(),
            "children_placeholder_count": preview.children_placeholder_count,
            "children_insertions": preview.children_insertions,
            "prop_identifier_bindings": preview.prop_identifier_bindings,
            "literal_expressions": preview.literal_expressions,
            "skipped_attributes": preview.skipped_attributes,
            "skipped_child_expressions": preview.skipped_child_expressions,
        }));
    }

    ComponentReturnPreview {
        html: html_fragments.join(""),
        element_count: elements.len(),
        elements,
        children_placeholder_count,
        children_insertions,
        prop_identifier_bindings,
        skipped_attributes,
        literal_expressions,
        skipped_child_expressions,
    }
}

struct StaticElementPreview {
    html: String,
    children_placeholder_count: usize,
    children_insertions: usize,
    prop_identifier_bindings: usize,
    skipped_attributes: usize,
    literal_expressions: usize,
    skipped_child_expressions: usize,
}

fn static_element_preview_with_children(
    document: &LoweredSourceDocument,
    element: &DxReactJsxElement,
    invocation_children_html: &str,
    prop_bindings: &[ComponentPropBinding],
    prop_aliases: &ComponentDestructuredPropAliases,
) -> StaticElementPreview {
    if is_dx_icon_element(document, element) {
        let preview = static_dx_icon_element_html(element, prop_bindings, Some(prop_aliases));
        return StaticElementPreview {
            html: preview.html,
            children_placeholder_count: 0,
            children_insertions: 0,
            prop_identifier_bindings: preview.prop_identifier_bindings,
            skipped_attributes: preview.skipped_attributes,
            literal_expressions: preview.literal_expressions,
            skipped_child_expressions: 0,
        };
    }

    let StaticAttributeListSnapshot {
        html: mut attributes,
        skipped_attributes,
        literal_expressions: attribute_literal_expressions,
        prop_identifier_bindings: attribute_prop_identifier_bindings,
    } = static_attribute_list_snapshot_with_bindings(element, prop_bindings, Some(prop_aliases));
    let StaticChildPreview {
        html: child_html,
        children_placeholder_count,
        children_insertions,
        literal_expressions: child_literal_expressions,
        skipped_expressions: skipped_child_expressions,
        prop_identifier_bindings: child_prop_identifier_bindings,
    } = static_child_preview_with_children(
        document,
        element,
        invocation_children_html,
        prop_bindings,
        prop_aliases,
    );
    if let Some(component_name) = framework_static_component_name(document, element) {
        attributes.push(format!(
            r#"data-dx-framework-component="{}""#,
            escape_html_attr(component_name)
        ));
        if let Some(render_mode) = framework_static_render_mode(document, element) {
            attributes.push(format!(
                r#"data-dx-render-mode="{}""#,
                escape_html_attr(render_mode)
            ));
        }
    }
    apply_next_image_static_attributes(document, element, &mut attributes);
    apply_next_script_static_attributes(document, element, &mut attributes);
    apply_next_font_static_attributes(document, element, &mut attributes);
    let attrs = if attributes.is_empty() {
        String::new()
    } else {
        format!(" {}", attributes.join(" "))
    };
    let tag = static_html_tag_name(document, element);
    let html = if is_void_element(tag) {
        format!("<{tag}{attrs}>")
    } else {
        format!("<{tag}{attrs}>{child_html}</{tag}>")
    };
    StaticElementPreview {
        html,
        children_placeholder_count,
        children_insertions,
        prop_identifier_bindings: attribute_prop_identifier_bindings
            + child_prop_identifier_bindings,
        skipped_attributes,
        literal_expressions: attribute_literal_expressions + child_literal_expressions,
        skipped_child_expressions,
    }
}

struct StaticChildPreview {
    html: String,
    children_placeholder_count: usize,
    children_insertions: usize,
    literal_expressions: usize,
    skipped_expressions: usize,
    prop_identifier_bindings: usize,
}

fn static_child_preview_with_children(
    document: &LoweredSourceDocument,
    element: &DxReactJsxElement,
    invocation_children_html: &str,
    prop_bindings: &[ComponentPropBinding],
    prop_aliases: &ComponentDestructuredPropAliases,
) -> StaticChildPreview {
    if element.child_nodes.is_empty() {
        return static_child_preview_with_children_legacy(
            element,
            invocation_children_html,
            prop_bindings,
            prop_aliases,
        );
    }

    let mut html = String::new();
    let mut children_placeholder_count = 0usize;
    let mut children_insertions = 0usize;
    let mut literal_expressions = 0usize;
    let mut skipped_expressions = 0usize;
    let mut prop_identifier_bindings = 0usize;

    for child in &element.child_nodes {
        match child {
            DxReactJsxChildNode::Text { value } => html.push_str(&escape_html_text(value)),
            DxReactJsxChildNode::Expression { expression } => {
                if is_children_placeholder_expression(expression) {
                    children_placeholder_count += 1;
                    if !invocation_children_html.is_empty() {
                        children_insertions += 1;
                        html.push_str(invocation_children_html);
                    }
                    continue;
                }
                match static_literal_expression(expression) {
                    Some(StaticLiteralExpression::Nullish) => literal_expressions += 1,
                    Some(value) => {
                        literal_expressions += 1;
                        html.push_str(&escape_html_text(&value.to_text()));
                    }
                    None => {
                        if let Some(value) = resolve_static_class_call_with_prop_bindings(
                            expression,
                            prop_bindings,
                            Some(prop_aliases),
                        ) {
                            literal_expressions += 1;
                            prop_identifier_bindings += 1;
                            html.push_str(&escape_html_text(&value.to_text()));
                        } else if let Some(value) = resolve_static_class_list_with_prop_bindings(
                            expression,
                            prop_bindings,
                            Some(prop_aliases),
                        ) {
                            literal_expressions += 1;
                            prop_identifier_bindings += 1;
                            html.push_str(&escape_html_text(&value.to_text()));
                        } else if let Some(value) = resolve_static_conditional_with_prop_bindings(
                            expression,
                            prop_bindings,
                            Some(prop_aliases),
                        ) {
                            literal_expressions += 1;
                            prop_identifier_bindings += 1;
                            html.push_str(&escape_html_text(&value.to_text()));
                        } else if let Some(value) = resolve_static_template_with_prop_bindings(
                            expression,
                            prop_bindings,
                            Some(prop_aliases),
                        ) {
                            literal_expressions += 1;
                            prop_identifier_bindings += 1;
                            html.push_str(&escape_html_text(&value.to_text()));
                        } else if let Some(value) = resolve_component_prop_identifier(
                            expression,
                            prop_bindings,
                            Some(prop_aliases),
                        ) {
                            prop_identifier_bindings += 1;
                            html.push_str(&escape_html_text(&value.to_text()));
                        } else {
                            skipped_expressions += 1;
                        }
                    }
                }
            }
            DxReactJsxChildNode::Element { index } => {
                let Some(child_element) = document.document.elements.get(*index) else {
                    skipped_expressions += 1;
                    continue;
                };
                if !is_static_renderable_element(document, child_element) {
                    skipped_expressions += 1;
                    continue;
                }
                let preview = static_element_preview_with_children(
                    document,
                    child_element,
                    invocation_children_html,
                    prop_bindings,
                    prop_aliases,
                );
                html.push_str(&preview.html);
                children_placeholder_count += preview.children_placeholder_count;
                children_insertions += preview.children_insertions;
                literal_expressions += preview.literal_expressions;
                skipped_expressions += preview.skipped_child_expressions;
                prop_identifier_bindings += preview.prop_identifier_bindings;
            }
        }
    }

    StaticChildPreview {
        html,
        children_placeholder_count,
        children_insertions,
        literal_expressions,
        skipped_expressions,
        prop_identifier_bindings,
    }
}

fn static_child_preview_with_children_legacy(
    element: &DxReactJsxElement,
    invocation_children_html: &str,
    prop_bindings: &[ComponentPropBinding],
    prop_aliases: &ComponentDestructuredPropAliases,
) -> StaticChildPreview {
    let mut html = escape_html_text(&element.text_content());
    let mut children_placeholder_count = 0usize;
    let mut children_insertions = 0usize;
    let mut literal_expressions = 0usize;
    let mut skipped_expressions = 0usize;
    let mut prop_identifier_bindings = 0usize;

    for expression in &element.child_expressions {
        if is_children_placeholder_expression(expression) {
            children_placeholder_count += 1;
            if !invocation_children_html.is_empty() {
                children_insertions += 1;
                html.push_str(invocation_children_html);
            }
            continue;
        }
        match static_literal_expression(expression) {
            Some(StaticLiteralExpression::Nullish) => literal_expressions += 1,
            Some(value) => {
                literal_expressions += 1;
                html.push_str(&escape_html_text(&value.to_text()));
            }
            None => {
                if let Some(value) = resolve_static_class_call_with_prop_bindings(
                    expression,
                    prop_bindings,
                    Some(prop_aliases),
                ) {
                    literal_expressions += 1;
                    prop_identifier_bindings += 1;
                    html.push_str(&escape_html_text(&value.to_text()));
                } else if let Some(value) = resolve_static_class_list_with_prop_bindings(
                    expression,
                    prop_bindings,
                    Some(prop_aliases),
                ) {
                    literal_expressions += 1;
                    prop_identifier_bindings += 1;
                    html.push_str(&escape_html_text(&value.to_text()));
                } else if let Some(value) = resolve_static_conditional_with_prop_bindings(
                    expression,
                    prop_bindings,
                    Some(prop_aliases),
                ) {
                    literal_expressions += 1;
                    prop_identifier_bindings += 1;
                    html.push_str(&escape_html_text(&value.to_text()));
                } else if let Some(value) = resolve_static_template_with_prop_bindings(
                    expression,
                    prop_bindings,
                    Some(prop_aliases),
                ) {
                    literal_expressions += 1;
                    prop_identifier_bindings += 1;
                    html.push_str(&escape_html_text(&value.to_text()));
                } else if let Some(value) =
                    resolve_component_prop_identifier(expression, prop_bindings, Some(prop_aliases))
                {
                    prop_identifier_bindings += 1;
                    html.push_str(&escape_html_text(&value.to_text()));
                } else {
                    skipped_expressions += 1;
                }
            }
        }
    }

    StaticChildPreview {
        html,
        children_placeholder_count,
        children_insertions,
        literal_expressions,
        skipped_expressions,
        prop_identifier_bindings,
    }
}

fn is_children_placeholder_expression(expression: &str) -> bool {
    matches!(
        strip_static_parentheses(expression.trim()),
        "children" | "props.children"
    )
}

fn prop_bindings(document: &LoweredSourceDocument) -> Vec<Value> {
    document
        .document
        .elements
        .iter()
        .flat_map(|element| {
            element.attributes.iter().map(|attribute| {
                json!({
                    "source_path": &document.source_path,
                    "role": document.role,
                    "element": &element.name,
                    "name": &attribute.name,
                    "kind": attribute_kind(attribute),
                    "value": &attribute.value,
                    "expression": &attribute.expression,
                    "event": is_event_attribute(&attribute.name),
                })
            })
        })
        .collect()
}

fn form_surfaces(document: &LoweredSourceDocument) -> Vec<Value> {
    let fields = document
        .document
        .elements
        .iter()
        .filter(|element| matches!(element.name.as_str(), "input" | "textarea" | "select"))
        .map(|element| {
            json!({
                "tag": &element.name,
                "name": element.attribute("name"),
                "type": element.attribute("type"),
                "required": element.attributes.iter().any(|attribute| attribute.name == "required"),
                "controlled": element.attributes.iter().any(|attribute| attribute.name == "value" || attribute.name == "checked"),
            })
        })
        .collect::<Vec<_>>();

    document
        .document
        .elements
        .iter()
        .filter(|element| element.name == "form")
        .map(|element| {
            json!({
                "source_path": &document.source_path,
                "role": document.role,
                "method": element.attribute("method").unwrap_or("get"),
                "action": element.attribute("action"),
                "has_submit_handler": element.attributes.iter().any(|attribute| attribute.name == "onSubmit"),
                "fields": fields.clone(),
            })
        })
        .collect()
}

fn build_state_dom_reflection_plan(
    route: &str,
    documents: &[LoweredSourceDocument],
    state_graph: &DxStateGraph,
) -> Value {
    let reflections = documents
        .iter()
        .flat_map(|document| document_state_dom_reflections(document, state_graph))
        .collect::<Vec<_>>();

    json!({
        "schema": "dx.tsx.stateDomReflection",
        "schema_revision": 1,
        "contract_name": "State DOM Reflection",
        "route": route,
        "status": if reflections.is_empty() { "no-safe-state-reads" } else { "state-read-reflection-planned" },
        "mode": "static-common-subset-reflection-plan",
        "reflection_count": reflections.len(),
        "full_react_reconciliation": false,
        "browser_listeners_attached": false,
        "supported_targets": [
            "text-content",
            "form-control-value",
            "form-control-checked",
            "boolean-attribute",
            "aria-attribute"
        ],
        "marker_contract": [
            "data-dx-state-read",
            "data-dx-state-value",
            "data-dx-state-checked",
            "data-dx-state-{disabled|required|selected|multiple|readonly}",
            "data-dx-state-aria-*"
        ],
        "reflections": reflections,
        "limits": [
            "Reflects direct state identifier reads only, such as {count}, value={name}, checked={open}, disabled={busy}, and aria-expanded={open}.",
            "Reflects derived values when the expression has a safe runtime lowering and the DOM carries compiler-owned data-dx-state-* markers by derived name.",
            "Does not evaluate ternaries, maps, member expressions without a safe derived lowering, component execution, effects, or React reconciliation.",
            "The generated runtime hook updates only elements carrying compiler-owned data-dx-state-* markers."
        ],
    })
}

fn document_state_dom_reflections(
    document: &LoweredSourceDocument,
    state_graph: &DxStateGraph,
) -> Vec<Value> {
    let state_slots = state_graph
        .slots
        .iter()
        .filter(|slot| state_slot_visible_to_document(slot, document))
        .collect::<Vec<_>>();
    let derived_slots = state_graph
        .derived_slots
        .iter()
        .filter(|slot| same_source_path(&slot.source_path, &document.source_path))
        .collect::<Vec<_>>();
    if state_slots.is_empty() && derived_slots.is_empty() {
        return Vec::new();
    }

    document
        .document
        .elements
        .iter()
        .enumerate()
        .filter(|(_, element)| is_static_renderable_element(document, element))
        .flat_map(|(index, element)| {
            let mutable_reflections = state_slots.iter().flat_map(move |slot| {
                element_state_dom_reflections(document, element, index, slot)
            });
            let derived_reflections = derived_slots.iter().flat_map(move |slot| {
                element_derived_state_dom_reflections(document, element, index, slot)
            });
            mutable_reflections.chain(derived_reflections)
        })
        .collect()
}

fn element_state_dom_reflections(
    document: &LoweredSourceDocument,
    element: &DxReactJsxElement,
    element_index: usize,
    slot: &DxStateSlot,
) -> Vec<Value> {
    let mut reflections = Vec::new();

    if element
        .child_expressions
        .iter()
        .any(|expression| expression_refs_state_slot(expression, &slot.name))
    {
        reflections.push(state_dom_reflection_entry(
            document,
            element,
            element_index,
            slot,
            "text-content",
            None,
        ));
    }

    for attribute in &element.attributes {
        let Some(expression) = attribute.expression.as_deref() else {
            continue;
        };
        if !expression_refs_state_slot(expression, &slot.name) {
            continue;
        }
        let Some(attribute_name) = static_html_attribute_name(&attribute.name) else {
            continue;
        };
        let target_kind = match attribute_name {
            "value" => Some("form-control-value"),
            "checked" => Some("form-control-checked"),
            "disabled" | "required" | "selected" | "multiple" | "readonly" | "hidden" => {
                Some("boolean-attribute")
            }
            name if name.starts_with("aria-") => Some("aria-attribute"),
            _ => None,
        };
        if let Some(target_kind) = target_kind {
            reflections.push(state_dom_reflection_entry(
                document,
                element,
                element_index,
                slot,
                target_kind,
                Some(attribute_name),
            ));
        }
    }

    reflections
}

fn element_derived_state_dom_reflections(
    document: &LoweredSourceDocument,
    element: &DxReactJsxElement,
    element_index: usize,
    slot: &DxDerivedStateSlot,
) -> Vec<Value> {
    let mut reflections = Vec::new();

    if element
        .child_expressions
        .iter()
        .any(|expression| expression_refs_state_slot(expression, &slot.name))
    {
        reflections.push(derived_state_dom_reflection_entry(
            document,
            element,
            element_index,
            slot,
            "text-content",
            None,
        ));
    }

    for attribute in &element.attributes {
        let Some(expression) = attribute.expression.as_deref() else {
            continue;
        };
        if !expression_refs_state_slot(expression, &slot.name) {
            continue;
        }
        let Some(attribute_name) = static_html_attribute_name(&attribute.name) else {
            continue;
        };
        let target_kind = match attribute_name {
            "value" => Some("form-control-value"),
            "checked" => Some("form-control-checked"),
            "disabled" | "required" | "selected" | "multiple" | "readonly" | "hidden" => {
                Some("boolean-attribute")
            }
            name if name.starts_with("aria-") => Some("aria-attribute"),
            _ => None,
        };
        if let Some(target_kind) = target_kind {
            reflections.push(derived_state_dom_reflection_entry(
                document,
                element,
                element_index,
                slot,
                target_kind,
                Some(attribute_name),
            ));
        }
    }

    reflections
}

fn state_dom_reflection_entry(
    document: &LoweredSourceDocument,
    element: &DxReactJsxElement,
    element_index: usize,
    slot: &DxStateSlot,
    target_kind: &'static str,
    attribute_name: Option<&str>,
) -> Value {
    json!({
        "source_path": &document.source_path,
        "role": document.role,
        "element_index": element_index,
        "tag": &element.name,
        "target": target_kind,
        "attribute": attribute_name,
        "state_slot": {
            "id": &slot.id,
            "name": &slot.name,
            "value_kind": &slot.value_kind,
        },
        "selector_hints": {
            "id": element.attribute("id"),
            "name": element.attribute("name"),
            "type": element.attribute("type"),
            "text": element.text_content(),
        },
        "full_react_reconciliation": false,
    })
}

fn derived_state_dom_reflection_entry(
    document: &LoweredSourceDocument,
    element: &DxReactJsxElement,
    element_index: usize,
    slot: &DxDerivedStateSlot,
    target_kind: &'static str,
    attribute_name: Option<&str>,
) -> Value {
    json!({
        "source_path": &document.source_path,
        "role": document.role,
        "element_index": element_index,
        "tag": &element.name,
        "target": target_kind,
        "attribute": attribute_name,
        "state_slot": {
            "id": &slot.id,
            "name": &slot.name,
            "value_kind": "derived",
        },
        "derived_slot": {
            "id": &slot.id,
            "name": &slot.name,
            "dependencies": &slot.dependencies,
            "expression": &slot.expression,
        },
        "selector_hints": {
            "id": element.attribute("id"),
            "name": element.attribute("name"),
            "type": element.attribute("type"),
            "text": element.text_content(),
        },
        "full_react_reconciliation": false,
    })
}

fn build_composed_static_dom_snapshot(
    route: &str,
    documents: &[LoweredSourceDocument],
    component_compositions: &[Value],
    state_graph: &DxStateGraph,
    request_prop_bindings: &[ComponentPropBinding],
) -> Value {
    let document_snapshots = documents
        .iter()
        .filter(|document| document.role != "source-owned-import")
        .map(|document| {
            composed_static_document_snapshot(
                document,
                component_compositions,
                state_graph,
                request_prop_bindings,
            )
        })
        .collect::<Vec<_>>();
    let elements = document_snapshots
        .iter()
        .flat_map(|snapshot| snapshot.elements.clone())
        .collect::<Vec<_>>();
    let html_fragments = document_snapshots
        .iter()
        .map(|snapshot| snapshot.html.as_str())
        .collect::<Vec<_>>();
    let skipped_attributes = document_snapshots
        .iter()
        .map(|snapshot| snapshot.skipped_attributes)
        .sum::<usize>();
    let literal_expressions = document_snapshots
        .iter()
        .map(|snapshot| snapshot.literal_expressions)
        .sum::<usize>();
    let skipped_child_expressions = document_snapshots
        .iter()
        .map(|snapshot| snapshot.skipped_child_expressions)
        .sum::<usize>();
    let app_router_shell = compose_app_router_static_shell(
        documents,
        component_compositions,
        state_graph,
        request_prop_bindings,
    );
    let app_router_shell_html = if app_router_shell.html.is_empty() {
        html_fragments.join("")
    } else {
        app_router_shell.html.clone()
    };
    let component_preview_insertions = document_snapshots
        .iter()
        .map(|snapshot| snapshot.component_preview_insertions)
        .sum::<usize>();
    let component_prop_identifier_bindings = document_snapshots
        .iter()
        .map(|snapshot| snapshot.component_prop_identifier_bindings)
        .sum::<usize>();
    let skipped_component_references = document_snapshots
        .iter()
        .map(|snapshot| snapshot.skipped_component_references)
        .sum::<usize>();

    json!({
        "schema": "dx.tsx.composedStaticDomSnapshot",
        "schema_revision": 1,
        "contract_name": "Composed Static DOM Snapshot",
        "route": route,
        "status": if elements.is_empty() {
            "empty"
        } else if app_router_shell.skipped_wrappers == 0 {
            "ready"
        } else if skipped_component_references == 0 {
            "ready-with-partial-app-router-shell"
        } else {
            "partial-skipped-components"
        },
        "mode": "layout-template-page-composition",
        "tree_mode": "nested-static-dom-tree",
        "full_component_execution": false,
        "full_react_execution": false,
        "source_documents": document_snapshots.len(),
        "element_count": elements.len(),
        "app_router_shell_child_insertions": app_router_shell.child_insertions,
        "app_router_shell_wrappers": app_router_shell.wrapper_count,
        "component_preview_insertions": component_preview_insertions,
        "component_prop_identifier_bindings": component_prop_identifier_bindings,
        "skipped_component_references": skipped_component_references,
        "skipped_attributes": skipped_attributes,
        "literal_expressions": literal_expressions,
        "skipped_child_expressions": skipped_child_expressions,
        "html": app_router_shell_html,
        "app_router_shell": app_router_shell.to_json(),
        "elements": elements,
        "limits": [
            "Composes route and layout documents with bounded source-owned component return previews.",
            "wraps page HTML with layout/template children placeholders in the static App Router shell.",
            "Uses component_return_preview HTML generated from local TSX imports and literal caller props.",
            "Does not execute arbitrary JavaScript, hooks, effects, data loaders, or client reconciliation.",
            "Keeps source-owned component previews separate from full React/App Router parity claims."
        ],
    })
}

struct AppRouterStaticShell {
    html: String,
    status: &'static str,
    leaf_role: &'static str,
    leaf_source_path: Option<String>,
    redirect_boundary_selected: bool,
    redirect: Option<Value>,
    not_found_boundary_selected: bool,
    error_boundary_selected: bool,
    error_boundary_props: Option<Value>,
    loading_boundary: Option<Value>,
    template_boundaries: Vec<Value>,
    scope_skipped_wrappers: Vec<Value>,
    wrapper_count: usize,
    child_insertions: usize,
    skipped_wrappers: usize,
    documents: Vec<Value>,
}

impl AppRouterStaticShell {
    fn to_json(&self) -> Value {
        json!({
            "schema": "dx.tsx.appRouterStaticShell",
            "schema_revision": 1,
            "contract_name": "App Router Static Shell",
            "status": self.status,
            "mode": "layout-template-page-composition",
            "html": &self.html,
            "selected_leaf": {
                "role": self.leaf_role,
                "source_path": &self.leaf_source_path,
                "redirect_selected": self.redirect_boundary_selected,
                "not_found_boundary_selected": self.not_found_boundary_selected,
                "error_boundary_selected": self.error_boundary_selected,
            },
            "redirect": &self.redirect,
            "error_boundary_props": &self.error_boundary_props,
            "wrapper_count": self.wrapper_count,
            "child_insertions": self.child_insertions,
            "skipped_wrappers": self.skipped_wrappers,
            "scope_skipped_wrapper_count": self.scope_skipped_wrappers.len(),
            "scope_skipped_wrappers": &self.scope_skipped_wrappers,
            "loading_boundary": &self.loading_boundary,
            "template_boundary_count": self.template_boundaries.len(),
            "template_boundaries": &self.template_boundaries,
            "documents": &self.documents,
            "full_app_router_runtime": false,
            "limits": [
                "Composes page, not-found, error boundary, or loading boundary preview HTML through layout/template children placeholders for the bounded static TSX surface.",
                "Records template.tsx remount boundaries without executing React remount lifecycle, Server Components, Client Components, async data, streaming Suspense payloads, context providers, effects, or React reconciliation.",
            ],
        })
    }
}

fn compose_app_router_static_shell(
    documents: &[LoweredSourceDocument],
    component_compositions: &[Value],
    state_graph: &DxStateGraph,
    request_prop_bindings: &[ComponentPropBinding],
) -> AppRouterStaticShell {
    let page_document = documents
        .iter()
        .rev()
        .find(|document| document.role == "page");
    let redirect = page_document.and_then(|document| next_navigation_redirect(&document.source));
    let redirect_boundary_selected = redirect.is_some();
    let selected_leaf = if redirect_boundary_selected {
        page_document
    } else {
        selected_app_router_leaf_document(documents)
    };
    let not_found_boundary_selected = !redirect_boundary_selected
        && selected_leaf.is_some_and(|document| document.role == "not-found");
    let error_boundary_selected = !redirect_boundary_selected
        && selected_leaf.is_some_and(|document| document.role == "error");
    let loading_boundary = loading_boundary_preview(
        documents,
        component_compositions,
        state_graph,
        request_prop_bindings,
    );
    let error_boundary_prop_bindings = if error_boundary_selected {
        error_boundary_prop_bindings(documents)
    } else {
        Vec::new()
    };
    let error_boundary_props = if error_boundary_selected {
        error_boundary_props(documents, &error_boundary_prop_bindings)
    } else {
        None
    };
    let loading_boundary_selected = loading_boundary.is_some();
    let leaf_role = if redirect_boundary_selected {
        "redirect"
    } else {
        selected_leaf.map_or("page", |document| document.role)
    };
    let leaf_source_path = selected_leaf.map(|document| document.source_path.clone());
    let wrapper_scope_leaf = if redirect_boundary_selected {
        page_document
    } else {
        selected_leaf
    };
    let mut leaf_prop_bindings = request_prop_bindings.to_vec();
    leaf_prop_bindings.extend(error_boundary_prop_bindings.iter().cloned());
    let mut html = if let Some(redirect) = redirect.as_ref() {
        redirect_leaf_html(redirect)
    } else {
        selected_leaf.map_or_else(String::new, |document| {
            composed_static_document_snapshot(
                document,
                component_compositions,
                state_graph,
                &leaf_prop_bindings,
            )
            .html
        })
    };
    let mut wrapper_count = 0usize;
    let mut child_insertions = 0usize;
    let mut skipped_wrappers = 0usize;
    let mut template_boundaries = Vec::new();
    let mut scope_skipped_wrappers = Vec::new();
    let mut composed_documents = selected_leaf
        .map(|document| {
            json!({
                "role": leaf_role,
                "source_path": &document.source_path,
                "mode": if redirect_boundary_selected {
                    "redirect-control-flow-static-html"
                } else {
                    match document.role {
                    "not-found" => "not-found-boundary-static-html",
                    "error" => "error-boundary-static-html",
                    _ => "page-static-html",
                    }
                },
                "redirect": &redirect,
            })
        })
        .into_iter()
        .collect::<Vec<_>>();

    for document in documents
        .iter()
        .filter(|document| matches!(document.role, "layout" | "template"))
        .rev()
    {
        if !app_router_wrapper_scope_matches_leaf(document, wrapper_scope_leaf) {
            scope_skipped_wrappers.push(app_router_scope_skipped_wrapper_record(
                document,
                wrapper_scope_leaf,
                leaf_role,
            ));
            continue;
        }
        let empty_prop_aliases = empty_component_prop_aliases();
        let preview = static_document_preview_with_children(
            document,
            &html,
            request_prop_bindings,
            &empty_prop_aliases,
        );
        let wrapper_status = if preview.element_count == 0 {
            "empty-wrapper-preview"
        } else if preview.children_insertions > 0 {
            "children-inserted"
        } else {
            "children-placeholder-missing"
        };
        if let Some(boundary) = template_boundary_record(document, &preview, wrapper_status) {
            template_boundaries.push(boundary);
        }
        if preview.element_count == 0 {
            skipped_wrappers += 1;
            composed_documents.push(json!({
                "role": document.role,
                "source_path": &document.source_path,
                "mode": "layout-template-page-composition",
                "status": wrapper_status,
                "children_insertions": 0,
            }));
            continue;
        }
        wrapper_count += 1;
        if preview.children_insertions > 0 {
            child_insertions += preview.children_insertions;
            html = preview.html;
        } else {
            skipped_wrappers += 1;
            html = format!("{}{}", preview.html, html);
        }
        composed_documents.push(json!({
            "role": document.role,
            "source_path": &document.source_path,
            "mode": "layout-template-page-composition",
            "status": wrapper_status,
            "children_insertions": preview.children_insertions,
            "element_count": preview.element_count,
            "skipped_attributes": preview.skipped_attributes,
            "skipped_child_expressions": preview.skipped_child_expressions,
        }));
    }

    let status = if html.is_empty() {
        "empty"
    } else if redirect_boundary_selected && skipped_wrappers == 0 {
        "redirect-boundary-ready"
    } else if redirect_boundary_selected {
        "redirect-boundary-partial-app-router-shell"
    } else if not_found_boundary_selected && skipped_wrappers == 0 {
        "not-found-boundary-ready"
    } else if not_found_boundary_selected {
        "not-found-boundary-partial-app-router-shell"
    } else if error_boundary_selected && skipped_wrappers == 0 {
        "error-boundary-ready"
    } else if error_boundary_selected {
        "error-boundary-partial-app-router-shell"
    } else if loading_boundary_selected && skipped_wrappers == 0 {
        "app-router-shell-ready-with-loading-boundary"
    } else if loading_boundary_selected {
        "partial-app-router-shell-with-loading-boundary"
    } else if skipped_wrappers == 0 {
        "app-router-shell-ready"
    } else {
        "partial-app-router-shell"
    };
    AppRouterStaticShell {
        html,
        status,
        leaf_role,
        leaf_source_path,
        redirect_boundary_selected,
        redirect,
        not_found_boundary_selected,
        error_boundary_selected,
        error_boundary_props,
        loading_boundary,
        template_boundaries,
        scope_skipped_wrappers,
        wrapper_count,
        child_insertions,
        skipped_wrappers,
        documents: composed_documents,
    }
}

fn app_router_wrapper_scope_matches_leaf(
    document: &LoweredSourceDocument,
    leaf: Option<&LoweredSourceDocument>,
) -> bool {
    let Some(leaf) = leaf else {
        return true;
    };
    app_router_boundary_scope_matches_page(&document.source_path, &leaf.source_path)
}

fn app_router_scope_skipped_wrapper_record(
    document: &LoweredSourceDocument,
    leaf: Option<&LoweredSourceDocument>,
    selected_leaf_role: &'static str,
) -> Value {
    json!({
        "schema": "dx.tsx.appRouterSkippedWrapper",
        "schema_revision": 1,
        "role": document.role,
        "source_path": &document.source_path,
        "reason": "below-selected-boundary-scope",
        "selected_leaf_role": selected_leaf_role,
        "selected_leaf_source_path": leaf.map(|leaf| leaf.source_path.as_str()),
        "source_owned_scope_boundary": true,
        "full_app_router_runtime": false,
    })
}

fn redirect_leaf_html(redirect: &Value) -> String {
    let destination = redirect
        .get("destination")
        .and_then(Value::as_str)
        .unwrap_or("/");
    let redirect_type = redirect
        .get("type")
        .and_then(Value::as_str)
        .unwrap_or("replace");
    let helper = redirect
        .get("helper")
        .and_then(Value::as_str)
        .unwrap_or("redirect()");
    let status_code = redirect
        .get("status_code")
        .and_then(Value::as_u64)
        .unwrap_or(307);
    let digest_code = redirect
        .get("digest_code")
        .and_then(Value::as_str)
        .unwrap_or("NEXT_REDIRECT");
    let destination_attr = escape_html_attr(destination);
    let destination_text = escape_html_text(destination);

    format!(
        r#"<section data-dx-app-router-redirect="true" data-dx-next-redirect-destination="{destination_attr}" data-dx-next-redirect-status="{status_code}" data-dx-next-redirect-type="{}" data-dx-next-redirect-helper="{}" data-dx-next-redirect-digest="{}"><p>Redirecting to <a href="{destination_attr}">{destination_text}</a></p></section>"#,
        escape_html_attr(redirect_type),
        escape_html_attr(helper),
        escape_html_attr(digest_code)
    )
}

fn template_boundary_record(
    document: &LoweredSourceDocument,
    preview: &ComponentReturnPreview,
    wrapper_status: &'static str,
) -> Option<Value> {
    if document.role != "template" {
        return None;
    }
    let status = if preview.element_count == 0 {
        "template-boundary-empty"
    } else if preview.children_insertions > 0 {
        "template-boundary-ready"
    } else {
        "template-boundary-without-children-placeholder"
    };
    Some(json!({
        "schema": "dx.tsx.appRouterTemplateBoundary",
        "schema_revision": 1,
        "boundary_id": template_boundary_id(&document.source_path),
        "source_path": &document.source_path,
        "status": status,
        "composition_status": wrapper_status,
        "source_owned_template_boundary": true,
        "remount_on_navigation": true,
        "persistent_across_navigation": false,
        "full_react_template_runtime": false,
        "element_count": preview.element_count,
        "children_placeholder_count": preview.children_placeholder_count,
        "children_insertions": preview.children_insertions,
        "skipped_attributes": preview.skipped_attributes,
        "literal_expressions": preview.literal_expressions,
        "skipped_child_expressions": preview.skipped_child_expressions,
        "limits": [
            "Marks template.tsx as a remount boundary in the source-owned static shell contract.",
            "Does not execute React lifecycle, client effects, context providers, or navigation-time remount code."
        ],
    }))
}

fn template_boundary_id(source_path: &str) -> String {
    format!("template:{}", normalize_source_path(source_path))
}

fn error_boundary_props(
    documents: &[LoweredSourceDocument],
    prop_bindings: &[ComponentPropBinding],
) -> Option<Value> {
    let page_document = documents
        .iter()
        .rev()
        .find(|document| document.role == "page")?;
    let error_document = nearest_app_router_boundary_document(documents, page_document, "error")?;
    let message = prop_bindings
        .iter()
        .find(|binding| binding.name == "error.message")
        .map(|binding| binding.value.to_text())
        .unwrap_or_else(|| "App Router page threw a source-owned error.".to_string());
    Some(json!({
        "schema": "dx.tsx.appRouterErrorBoundaryProps",
        "schema_revision": 1,
        "source_path": &error_document.source_path,
        "status": "error-boundary-props-ready",
        "source_owned_error_boundary_props": true,
        "error": {
            "binding": "error",
            "message_binding": "error.message",
            "message": message,
        },
        "reset": {
            "binding": "reset",
            "kind": "function-boundary",
            "reset_invocable": false,
        },
        "props": prop_bindings.iter().map(|binding| json!({
            "name": &binding.name,
            "source_kind": binding.source_kind,
            "expression": &binding.expression,
            "value_type": binding.value.value_type(),
            "value": binding.value.to_json_value(),
        })).collect::<Vec<_>>(),
        "full_client_error_runtime": false,
        "full_react_error_boundary_runtime": false,
        "limits": [
            "Binds error.message for static error.tsx previews when the selected page throws a safe literal Error.",
            "Exposes reset as a non-invocable boundary marker; DX does not run client recovery or React error lifecycle."
        ],
    }))
}

fn error_boundary_prop_bindings(documents: &[LoweredSourceDocument]) -> Vec<ComponentPropBinding> {
    let message = documents
        .iter()
        .rev()
        .find(|document| document.role == "page")
        .and_then(|document| static_page_error_message(&document.source))
        .unwrap_or_else(|| "App Router page threw a source-owned error.".to_string());
    vec![
        ComponentPropBinding {
            name: "error.message".to_string(),
            value: StaticLiteralExpression::String(message),
            source_kind: "app-router-error-boundary-prop",
            expression: Some("error.message".to_string()),
        },
        ComponentPropBinding {
            name: "reset".to_string(),
            value: StaticLiteralExpression::String("[dx-error-boundary-reset]".to_string()),
            source_kind: "app-router-error-boundary-reset",
            expression: Some("reset".to_string()),
        },
    ]
}

fn selected_app_router_leaf_document(
    documents: &[LoweredSourceDocument],
) -> Option<&LoweredSourceDocument> {
    let page = documents
        .iter()
        .rev()
        .find(|document| document.role == "page")?;
    if detects_next_navigation_not_found(&page.source) {
        return nearest_app_router_boundary_document(documents, page, "not-found").or(Some(page));
    }
    if detects_static_page_error_throw(&page.source) {
        return nearest_app_router_boundary_document(documents, page, "error").or(Some(page));
    }
    Some(page)
}

fn nearest_app_router_boundary_document<'a>(
    documents: &'a [LoweredSourceDocument],
    page: &LoweredSourceDocument,
    role: &str,
) -> Option<&'a LoweredSourceDocument> {
    documents
        .iter()
        .filter(|document| {
            document.role == role
                && app_router_boundary_scope_matches_page(&document.source_path, &page.source_path)
        })
        .max_by_key(|document| {
            app_router_route_scope_depth(&app_router_route_directory(&document.source_path))
        })
}

fn app_router_boundary_scope_matches_page(boundary_path: &str, page_path: &str) -> bool {
    let boundary_directory = app_router_route_directory(boundary_path);
    if boundary_directory.is_empty() {
        return true;
    }
    let page_directory = app_router_route_directory(page_path);
    page_directory == boundary_directory
        || page_directory
            .strip_prefix(&boundary_directory)
            .is_some_and(|rest| rest.starts_with('/'))
}

fn app_router_route_directory(source_path: &str) -> String {
    let normalized = normalize_source_path(source_path);
    let directory = normalized
        .rsplit_once('/')
        .map(|(directory, _)| directory)
        .unwrap_or("");
    strip_app_route_source_root(directory)
        .trim_matches('/')
        .to_string()
}

fn strip_app_route_source_root(directory: &str) -> &str {
    if directory == "app" || directory == "src/app" {
        return "";
    }
    directory
        .strip_prefix("src/app/")
        .or_else(|| directory.strip_prefix("app/"))
        .unwrap_or(directory)
}

fn app_router_route_scope_depth(route_directory: &str) -> usize {
    route_directory
        .split('/')
        .filter(|segment| !segment.is_empty())
        .count()
}

fn loading_boundary_preview(
    documents: &[LoweredSourceDocument],
    component_compositions: &[Value],
    state_graph: &DxStateGraph,
    request_prop_bindings: &[ComponentPropBinding],
) -> Option<Value> {
    let page = documents
        .iter()
        .rev()
        .find(|document| document.role == "page")?;
    if !detects_deferred_page_render(&page.source) {
        return None;
    }
    let loading = nearest_app_router_boundary_document(documents, page, "loading")?;
    let snapshot = composed_static_document_snapshot(
        loading,
        component_compositions,
        state_graph,
        request_prop_bindings,
    );
    let status = if snapshot.html.is_empty() {
        "loading-boundary-empty"
    } else {
        "loading-boundary-ready"
    };
    Some(json!({
        "schema": "dx.tsx.appRouterLoadingBoundary",
        "schema_revision": 1,
        "status": status,
        "source_path": &loading.source_path,
        "html": snapshot.html,
        "element_count": snapshot.elements.len(),
        "skipped_attributes": snapshot.skipped_attributes,
        "literal_expressions": snapshot.literal_expressions,
        "skipped_child_expressions": snapshot.skipped_child_expressions,
        "component_preview_insertions": snapshot.component_preview_insertions,
        "source_owned_loading_boundary": true,
        "full_streaming_runtime": false,
        "full_react_suspense_runtime": false,
        "limits": [
            "Renders the static loading.tsx fallback for deferred pages.",
            "Does not stream RSC payloads, run async data, or claim Suspense runtime parity."
        ],
    }))
}

fn detects_deferred_page_render(source: &str) -> bool {
    source_contains_word_outside_comments_and_strings(source, "await")
}

fn detects_static_page_error_throw(source: &str) -> bool {
    let mut index = 0usize;
    while index < source.len() {
        let rest = &source[index..];
        if rest.starts_with("//") {
            index = skip_source_line_comment(source, index);
            continue;
        }
        if rest.starts_with("/*") {
            index = skip_source_block_comment(source, index);
            continue;
        }

        let Some(character) = rest.chars().next() else {
            break;
        };
        if is_source_string_quote(character) {
            index = skip_source_string_literal(source, index, character);
            continue;
        }

        if rest.starts_with("throw") && is_word_boundary(source, index, "throw".len()) {
            let expression_start = skip_ascii_whitespace(source, index + "throw".len());
            if is_safe_error_throw_expression(&source[expression_start..]) {
                return true;
            }
        }

        index += character.len_utf8();
    }
    false
}

fn static_page_error_message(source: &str) -> Option<String> {
    let mut index = 0usize;
    while index < source.len() {
        let rest = &source[index..];
        if rest.starts_with("//") {
            index = skip_source_line_comment(source, index);
            continue;
        }
        if rest.starts_with("/*") {
            index = skip_source_block_comment(source, index);
            continue;
        }

        let Some(character) = rest.chars().next() else {
            break;
        };
        if is_source_string_quote(character) {
            index = skip_source_string_literal(source, index, character);
            continue;
        }

        if rest.starts_with("throw") && is_word_boundary(source, index, "throw".len()) {
            let expression_start = skip_ascii_whitespace(source, index + "throw".len());
            if let Some(message) = safe_error_throw_message(&source[expression_start..]) {
                return Some(message);
            }
        }

        index += character.len_utf8();
    }
    None
}

fn safe_error_throw_message(source: &str) -> Option<String> {
    let source = source.trim_start();
    let arguments = source
        .strip_prefix("new Error(")
        .or_else(|| source.strip_prefix("Error("))?;
    let literal = first_static_string_argument(arguments)?;
    Some(literal.to_text())
}

fn first_static_string_argument(arguments: &str) -> Option<StaticLiteralExpression> {
    let arguments = arguments.trim_start();
    let quote = arguments.chars().next()?;
    if !is_source_string_quote(quote) {
        return None;
    }
    let literal_end = skip_source_string_literal(arguments, 0, quote);
    if literal_end > arguments.len() {
        return None;
    }
    let after_literal = arguments[literal_end..].trim_start();
    if !(after_literal.starts_with(')') || after_literal.starts_with(',')) {
        return None;
    }
    static_literal_expression(&arguments[..literal_end])
}

fn source_contains_word_outside_comments_and_strings(source: &str, word: &str) -> bool {
    let mut index = 0usize;
    while index < source.len() {
        let rest = &source[index..];
        if rest.starts_with("//") {
            index = skip_source_line_comment(source, index);
            continue;
        }
        if rest.starts_with("/*") {
            index = skip_source_block_comment(source, index);
            continue;
        }

        let Some(character) = rest.chars().next() else {
            break;
        };
        if is_source_string_quote(character) {
            index = skip_source_string_literal(source, index, character);
            continue;
        }

        if rest.starts_with(word) && is_word_boundary(source, index, word.len()) {
            return true;
        }

        index += character.len_utf8();
    }
    false
}

fn is_safe_error_throw_expression(source: &str) -> bool {
    let source = source.trim_start();
    source.starts_with("new Error(") || source.starts_with("Error(")
}

fn skip_source_line_comment(source: &str, index: usize) -> usize {
    source[index + 2..]
        .find('\n')
        .map(|offset| index + 2 + offset + 1)
        .unwrap_or(source.len())
}

fn skip_source_block_comment(source: &str, index: usize) -> usize {
    source[index + 2..]
        .find("*/")
        .map(|offset| index + 2 + offset + 2)
        .unwrap_or(source.len())
}

fn skip_source_string_literal(source: &str, index: usize, quote: char) -> usize {
    let mut cursor = index + quote.len_utf8();
    let mut escaped = false;
    while cursor < source.len() {
        let Some(character) = source[cursor..].chars().next() else {
            return source.len();
        };
        cursor += character.len_utf8();
        if escaped {
            escaped = false;
            continue;
        }
        if character == '\\' {
            escaped = true;
            continue;
        }
        if character == quote {
            return cursor;
        }
    }
    source.len()
}

fn is_source_string_quote(character: char) -> bool {
    character == '"' || character == '\'' || character == '`'
}

struct ComposedStaticDocumentSnapshot {
    html: String,
    elements: Vec<Value>,
    skipped_attributes: usize,
    literal_expressions: usize,
    skipped_child_expressions: usize,
    component_preview_insertions: usize,
    component_prop_identifier_bindings: usize,
    skipped_component_references: usize,
}

fn composed_static_document_snapshot(
    document: &LoweredSourceDocument,
    component_compositions: &[Value],
    state_graph: &DxStateGraph,
    request_prop_bindings: &[ComponentPropBinding],
) -> ComposedStaticDocumentSnapshot {
    let mut elements = Vec::new();
    let mut html_fragments = Vec::new();
    let mut skipped_attributes = 0usize;
    let mut literal_expressions = 0usize;
    let mut skipped_child_expressions = 0usize;
    let mut component_preview_insertions = 0usize;
    let mut component_prop_identifier_bindings = 0usize;
    let mut skipped_component_references = 0usize;

    for element_index in root_jsx_elements(document) {
        let Some(element) = document.document.elements.get(element_index) else {
            continue;
        };
        if is_static_renderable_element(document, element) {
            let StaticElementSnapshot {
                html,
                skipped_attributes: element_skipped_attributes,
                literal_expressions: element_literal_expressions,
                skipped_child_expressions: element_skipped_child_expressions,
                component_preview_insertions: element_component_preview_insertions,
                component_prop_identifier_bindings: element_component_prop_identifier_bindings,
                skipped_component_references: element_skipped_component_references,
                state_dom_reflections,
            } = static_element_snapshot_with_components(
                document,
                element_index,
                element,
                state_graph,
                request_prop_bindings,
                Some(component_compositions),
            );
            skipped_attributes += element_skipped_attributes;
            literal_expressions += element_literal_expressions;
            skipped_child_expressions += element_skipped_child_expressions;
            component_preview_insertions += element_component_preview_insertions;
            component_prop_identifier_bindings += element_component_prop_identifier_bindings;
            skipped_component_references += element_skipped_component_references;
            html_fragments.push(html.clone());
            elements.push(json!({
                "source_path": &document.source_path,
                "role": document.role,
                "element_index": element_index,
                "parent_index": element.parent_index,
                "tag": &element.name,
                "mode": "intrinsic-static-dom",
                "html": html,
                "text": element.text_content(),
                "child_node_count": element.child_nodes.len(),
                "child_element_indices": child_element_indices(element),
                "skipped_attributes": element_skipped_attributes,
                "literal_expressions": element_literal_expressions,
                "skipped_child_expressions": element_skipped_child_expressions,
                "component_preview_insertions": element_component_preview_insertions,
                "component_prop_identifier_bindings": element_component_prop_identifier_bindings,
                "skipped_component_references": element_skipped_component_references,
                "event_handlers_lowered": element.attributes.iter().filter(|attribute| is_event_attribute(&attribute.name)).count(),
                "state_dom_reflections": state_dom_reflections,
            }));
            continue;
        }

        if !is_component_reference(&element.name) {
            continue;
        }

        let Some(composition) = matching_component_composition(
            component_compositions,
            &document.source_path,
            element_index,
            &element.name,
        ) else {
            skipped_component_references += 1;
            continue;
        };
        let Some(return_preview) = composition.get("return_preview") else {
            skipped_component_references += 1;
            continue;
        };
        let preview_html = return_preview
            .get("html")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let preview_element_count = return_preview
            .get("element_count")
            .and_then(Value::as_u64)
            .unwrap_or(0) as usize;
        let preview_children_insertions = return_preview
            .get("children_insertions")
            .and_then(Value::as_u64)
            .unwrap_or(0) as usize;
        let preview_prop_identifier_bindings = return_preview
            .get("prop_identifier_binding_count")
            .and_then(Value::as_u64)
            .unwrap_or(0) as usize;
        let preview_literal_expressions = return_preview
            .get("literal_expressions")
            .and_then(Value::as_u64)
            .unwrap_or(0) as usize;
        let preview_skipped_attributes = return_preview
            .get("skipped_attributes")
            .and_then(Value::as_u64)
            .unwrap_or(0) as usize;
        let preview_skipped_child_expressions = return_preview
            .get("skipped_child_expressions")
            .and_then(Value::as_u64)
            .unwrap_or(0) as usize;

        if preview_html.is_empty() {
            skipped_component_references += 1;
        } else {
            html_fragments.push(preview_html.to_string());
            component_preview_insertions += 1;
        }
        skipped_attributes += preview_skipped_attributes;
        literal_expressions += preview_literal_expressions;
        skipped_child_expressions += preview_skipped_child_expressions;
        component_prop_identifier_bindings += preview_prop_identifier_bindings;
        elements.push(json!({
            "source_path": &document.source_path,
            "role": document.role,
            "element_index": element_index,
            "parent_index": element.parent_index,
            "tag": &element.name,
            "mode": "source-owned-component-preview",
            "status": if preview_html.is_empty() { "empty-component-preview" } else { "component-preview-inserted" },
            "component_source_path": composition.get("source_path").cloned().unwrap_or(Value::Null),
            "component_preview_html": preview_html,
            "component_preview_element_count": preview_element_count,
            "component_preview_insertions": if preview_html.is_empty() { 0 } else { 1 },
            "children_insertions": preview_children_insertions,
            "component_prop_identifier_bindings": preview_prop_identifier_bindings,
            "literal_expressions": preview_literal_expressions,
            "skipped_attributes": preview_skipped_attributes,
            "skipped_child_expressions": preview_skipped_child_expressions,
            "full_component_execution": false,
            "runtime_binding": return_preview.get("runtime_binding").cloned().unwrap_or(Value::Null),
            "preview_elements": return_preview.get("elements").cloned().unwrap_or_else(|| json!([])),
        }));
    }

    ComposedStaticDocumentSnapshot {
        html: html_fragments.join(""),
        elements,
        skipped_attributes,
        literal_expressions,
        skipped_child_expressions,
        component_preview_insertions,
        component_prop_identifier_bindings,
        skipped_component_references,
    }
}

fn matching_component_composition<'a>(
    component_compositions: &'a [Value],
    importer: &str,
    importer_element_index: usize,
    component: &str,
) -> Option<&'a Value> {
    matching_component_composition_for_element(
        component_compositions,
        importer,
        importer_element_index,
        component,
    )
}

fn matching_component_composition_for_element<'a>(
    component_compositions: &'a [Value],
    importer: &str,
    importer_element_index: usize,
    component: &str,
) -> Option<&'a Value> {
    component_compositions.iter().find(|composition| {
        composition
            .get("importer")
            .and_then(Value::as_str)
            .is_some_and(|candidate| same_source_path(candidate, importer))
            && composition
                .get("importer_element_index")
                .and_then(Value::as_u64)
                .is_some_and(|candidate| candidate as usize == importer_element_index)
            && composition
                .get("component")
                .and_then(Value::as_str)
                .is_some_and(|candidate| candidate == component)
    })
}

fn build_static_dom_snapshot(
    route: &str,
    documents: &[LoweredSourceDocument],
    state_graph: &DxStateGraph,
    request_prop_bindings: &[ComponentPropBinding],
) -> Value {
    let document_snapshots = documents
        .iter()
        .map(|document| static_document_snapshot(document, state_graph, request_prop_bindings))
        .collect::<Vec<_>>();
    let elements = document_snapshots
        .iter()
        .flat_map(|snapshot| snapshot.elements.clone())
        .collect::<Vec<_>>();
    let html_fragments = document_snapshots
        .iter()
        .map(|snapshot| snapshot.html.as_str())
        .collect::<Vec<_>>();
    let skipped_attributes = document_snapshots
        .iter()
        .map(|snapshot| snapshot.skipped_attributes)
        .sum::<usize>();
    let literal_expressions = document_snapshots
        .iter()
        .map(|snapshot| snapshot.literal_expressions)
        .sum::<usize>();
    let skipped_child_expressions = document_snapshots
        .iter()
        .map(|snapshot| snapshot.skipped_child_expressions)
        .sum::<usize>();

    json!({
        "schema": "dx.tsx.staticDomSnapshot",
        "schema_revision": 1,
        "contract_name": "Static DOM Snapshot",
        "route": route,
        "status": if elements.is_empty() { "empty" } else { "static-dom-snapshot-ready" },
        "mode": "safe-common-subset",
        "full_react_execution": false,
        "source_documents": documents.len(),
        "element_count": elements.len(),
        "skipped_attributes": skipped_attributes,
        "literal_expressions": literal_expressions,
        "skipped_child_expressions": skipped_child_expressions,
        "html": html_fragments.join(""),
        "elements": elements,
        "limits": [
            "Serializes safe intrinsic JSX elements from route, segment, and bounded source-owned import files.",
            "Preserves literal safe attributes, boolean form attributes, and simple literal expression props such as string, number, boolean, nullish, and static template literal values.",
            "Appends direct literal child expressions after lowered text because the current JSX surface does not preserve interleaved child order.",
            "Does not execute arbitrary JavaScript, evaluate component functions, run hooks, or claim hydration parity."
        ],
    })
}

struct StaticDocumentSnapshot {
    html: String,
    elements: Vec<Value>,
    element_count: usize,
    skipped_attributes: usize,
    literal_expressions: usize,
    skipped_child_expressions: usize,
}

fn static_document_snapshot(
    document: &LoweredSourceDocument,
    state_graph: &DxStateGraph,
    request_prop_bindings: &[ComponentPropBinding],
) -> StaticDocumentSnapshot {
    let mut elements = Vec::new();
    let mut html_fragments = Vec::new();
    let mut skipped_attributes = 0usize;
    let mut literal_expressions = 0usize;
    let mut skipped_child_expressions = 0usize;

    for element_index in root_jsx_elements(document) {
        let Some(element) = document.document.elements.get(element_index) else {
            continue;
        };
        if !is_static_renderable_element(document, element) {
            continue;
        }
        let StaticElementSnapshot {
            html,
            skipped_attributes: element_skipped_attributes,
            literal_expressions: element_literal_expressions,
            skipped_child_expressions: element_skipped_child_expressions,
            component_preview_insertions: _element_component_preview_insertions,
            component_prop_identifier_bindings: _element_component_prop_identifier_bindings,
            skipped_component_references: _element_skipped_component_references,
            state_dom_reflections,
        } = static_element_snapshot(
            document,
            element_index,
            element,
            state_graph,
            request_prop_bindings,
        );
        skipped_attributes += element_skipped_attributes;
        literal_expressions += element_literal_expressions;
        skipped_child_expressions += element_skipped_child_expressions;
        html_fragments.push(html.clone());
        elements.push(json!({
            "source_path": &document.source_path,
            "role": document.role,
            "element_index": element_index,
            "parent_index": element.parent_index,
            "tag": &element.name,
            "html": html,
            "text": element.text_content(),
            "child_node_count": element.child_nodes.len(),
            "child_element_indices": child_element_indices(element),
            "skipped_attributes": element_skipped_attributes,
            "literal_expressions": element_literal_expressions,
            "skipped_child_expressions": element_skipped_child_expressions,
            "event_handlers_lowered": element.attributes.iter().filter(|attribute| is_event_attribute(&attribute.name)).count(),
            "state_dom_reflections": state_dom_reflections,
        }));
    }

    StaticDocumentSnapshot {
        html: html_fragments.join(""),
        element_count: elements.len(),
        elements,
        skipped_attributes,
        literal_expressions,
        skipped_child_expressions,
    }
}

struct StaticElementSnapshot {
    html: String,
    skipped_attributes: usize,
    literal_expressions: usize,
    skipped_child_expressions: usize,
    component_preview_insertions: usize,
    component_prop_identifier_bindings: usize,
    skipped_component_references: usize,
    state_dom_reflections: Vec<Value>,
}

fn static_element_snapshot(
    document: &LoweredSourceDocument,
    element_index: usize,
    element: &DxReactJsxElement,
    state_graph: &DxStateGraph,
    request_prop_bindings: &[ComponentPropBinding],
) -> StaticElementSnapshot {
    static_element_snapshot_with_components(
        document,
        element_index,
        element,
        state_graph,
        request_prop_bindings,
        None,
    )
}

fn static_element_snapshot_with_components(
    document: &LoweredSourceDocument,
    element_index: usize,
    element: &DxReactJsxElement,
    state_graph: &DxStateGraph,
    request_prop_bindings: &[ComponentPropBinding],
    component_compositions: Option<&[Value]>,
) -> StaticElementSnapshot {
    if is_dx_icon_element(document, element) {
        let preview = static_dx_icon_element_html(element, request_prop_bindings, None);
        return StaticElementSnapshot {
            html: preview.html,
            skipped_attributes: preview.skipped_attributes,
            literal_expressions: preview.literal_expressions,
            skipped_child_expressions: 0,
            component_preview_insertions: 0,
            component_prop_identifier_bindings: preview.prop_identifier_bindings,
            skipped_component_references: 0,
            state_dom_reflections: Vec::new(),
        };
    }

    let StaticAttributeListSnapshot {
        html: mut attributes,
        skipped_attributes,
        literal_expressions: attribute_literal_expressions,
        prop_identifier_bindings: _attribute_prop_identifier_bindings,
    } = static_attribute_list_snapshot_with_bindings(element, request_prop_bindings, None);
    let state_dom_reflections =
        document_state_dom_reflections_for_element(document, element_index, element, state_graph);
    attributes.extend(state_reflection_html_attributes(&state_dom_reflections));
    let StaticChildSnapshot {
        html: child_html,
        literal_expressions: child_literal_expressions,
        skipped_expressions: skipped_child_expressions,
        component_preview_insertions,
        component_prop_identifier_bindings,
        skipped_component_references,
    } = render_static_child_nodes(
        document,
        element,
        state_graph,
        request_prop_bindings,
        component_compositions,
    );
    if let Some(component_name) = framework_static_component_name(document, element) {
        attributes.push(format!(
            r#"data-dx-framework-component="{}""#,
            escape_html_attr(component_name)
        ));
        if let Some(render_mode) = framework_static_render_mode(document, element) {
            attributes.push(format!(
                r#"data-dx-render-mode="{}""#,
                escape_html_attr(render_mode)
            ));
        }
    }
    apply_next_image_static_attributes(document, element, &mut attributes);
    apply_next_script_static_attributes(document, element, &mut attributes);
    apply_next_font_static_attributes(document, element, &mut attributes);
    let attrs = if attributes.is_empty() {
        String::new()
    } else {
        format!(" {}", attributes.join(" "))
    };
    let tag = static_html_tag_name(document, element);
    let html = if is_void_element(tag) {
        format!("<{tag}{attrs}>")
    } else {
        format!("<{tag}{attrs}>{child_html}</{tag}>")
    };
    StaticElementSnapshot {
        html,
        skipped_attributes,
        literal_expressions: attribute_literal_expressions + child_literal_expressions,
        skipped_child_expressions,
        component_preview_insertions,
        component_prop_identifier_bindings,
        skipped_component_references,
        state_dom_reflections,
    }
}

fn root_jsx_elements(document: &LoweredSourceDocument) -> Vec<usize> {
    document
        .document
        .elements
        .iter()
        .enumerate()
        .filter_map(|(index, element)| element.parent_index.is_none().then_some(index))
        .collect()
}

fn document_state_dom_reflections_for_element(
    document: &LoweredSourceDocument,
    element_index: usize,
    element: &DxReactJsxElement,
    state_graph: &DxStateGraph,
) -> Vec<Value> {
    let mutable_reflections = state_graph
        .slots
        .iter()
        .filter(|slot| state_slot_visible_to_document(slot, document))
        .flat_map(|slot| element_state_dom_reflections(document, element, element_index, slot));
    let derived_reflections = state_graph
        .derived_slots
        .iter()
        .filter(|slot| same_source_path(&slot.source_path, &document.source_path))
        .flat_map(|slot| {
            element_derived_state_dom_reflections(document, element, element_index, slot)
        });
    mutable_reflections.chain(derived_reflections).collect()
}

fn state_slot_visible_to_document(slot: &DxStateSlot, document: &LoweredSourceDocument) -> bool {
    same_source_path(&slot.source_path, &document.source_path)
        || matches!(slot.scope, DxStateScope::Global)
}

fn state_reflection_html_attributes(reflections: &[Value]) -> Vec<String> {
    let mut text_slots = Vec::new();
    let mut value_slots = Vec::new();
    let mut checked_slots = Vec::new();
    let mut boolean_attrs = Vec::new();
    let mut aria_attrs = Vec::new();

    for reflection in reflections {
        let Some(slot) = reflection
            .get("state_slot")
            .and_then(|slot| slot.get("name"))
            .and_then(Value::as_str)
        else {
            continue;
        };
        match reflection.get("target").and_then(Value::as_str) {
            Some("text-content") => push_unique(&mut text_slots, slot),
            Some("form-control-value") => push_unique(&mut value_slots, slot),
            Some("form-control-checked") => push_unique(&mut checked_slots, slot),
            Some("boolean-attribute") => {
                if let Some(attribute) = reflection.get("attribute").and_then(Value::as_str) {
                    boolean_attrs.push((attribute.to_string(), slot.to_string()));
                }
            }
            Some("aria-attribute") => {
                if let Some(attribute) = reflection.get("attribute").and_then(Value::as_str) {
                    aria_attrs.push((attribute.to_string(), slot.to_string()));
                }
            }
            _ => {}
        }
    }

    let mut attrs = Vec::new();
    if !text_slots.is_empty() {
        attrs.push(format!(
            r#"data-dx-state-read="{}""#,
            escape_html_attr(&text_slots.join(" "))
        ));
    }
    if let Some(slot) = value_slots.first() {
        attrs.push(format!(
            r#"data-dx-state-value="{}""#,
            escape_html_attr(slot)
        ));
    }
    if let Some(slot) = checked_slots.first() {
        attrs.push(format!(
            r#"data-dx-state-checked="{}""#,
            escape_html_attr(slot)
        ));
    }
    for (attribute, slot) in boolean_attrs {
        attrs.push(format!(
            r#"data-dx-state-{}="{}""#,
            escape_html_attr(&attribute),
            escape_html_attr(&slot)
        ));
    }
    for (attribute, slot) in aria_attrs {
        attrs.push(format!(
            r#"data-dx-state-{}="{}""#,
            escape_html_attr(&attribute),
            escape_html_attr(&slot)
        ));
    }
    attrs
}
