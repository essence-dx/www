use std::collections::{BTreeMap, VecDeque};

use crate::dx_parser::{
    AttributeValue, ComponentInstance, DxFile, ElementNode, IfBlockNode, SlotNode, TemplateNode,
};

/// Parsed source-owned component available to the vertical fallback renderer.
pub(super) struct DxVerticalAstComponent {
    pub(super) name: String,
    pub(super) file: DxFile,
}

/// Render-time hooks used by compiler-derived tiny interaction programs.
#[derive(Debug, Clone, Default)]
pub(super) struct DxVerticalRenderRuntime {
    pub(super) expression_targets: BTreeMap<String, DxVerticalExpressionTarget>,
    pub(super) event_targets: BTreeMap<String, VecDeque<String>>,
    pub(super) script: Option<String>,
}

/// DOM target for a state expression mirrored by a runtime program.
#[derive(Debug, Clone)]
pub(super) struct DxVerticalExpressionTarget {
    pub(super) element_id: String,
    pub(super) initial_value: String,
}

/// Render a crawlable fallback document from the parsed route/component tree.
pub(super) fn render_fallback_document(
    route: &str,
    page: &DxFile,
    components: &[DxVerticalAstComponent],
    runtime: Option<DxVerticalRenderRuntime>,
) -> String {
    let mut renderer = VerticalRenderer::new(components, runtime.unwrap_or_default());
    let body = renderer.render_nodes(&page.template, &RenderContext::default());
    let script = renderer.runtime.script.as_deref().unwrap_or("");

    format!(
        r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>DX-WWW {route}</title>
  </head>
  <body>{body}{script}</body>
</html>"#
    )
}

struct VerticalRenderer<'a> {
    components: BTreeMap<&'a str, &'a DxFile>,
    runtime: DxVerticalRenderRuntime,
}

#[derive(Default)]
struct RenderContext {
    values: BTreeMap<String, String>,
    slots: BTreeMap<String, Vec<TemplateNode>>,
}

impl<'a> VerticalRenderer<'a> {
    fn new(components: &'a [DxVerticalAstComponent], runtime: DxVerticalRenderRuntime) -> Self {
        let components = components
            .iter()
            .map(|component| (component.name.as_str(), &component.file))
            .collect();

        Self {
            components,
            runtime,
        }
    }

    fn render_nodes(&mut self, nodes: &[TemplateNode], context: &RenderContext) -> String {
        nodes
            .iter()
            .map(|node| self.render_node(node, context))
            .collect::<String>()
    }

    fn render_node(&mut self, node: &TemplateNode, context: &RenderContext) -> String {
        match node {
            TemplateNode::Text(text) => escape_text(text),
            TemplateNode::Element(element) => self.render_element(element, context),
            TemplateNode::Expression(expression) => {
                self.render_expression_value(&expression.expression, context)
            }
            TemplateNode::IfBlock(block) => self.render_if_block(block, context),
            TemplateNode::EachBlock(block) => format!(
                r#"<template data-dx-each="{}">{}</template>"#,
                escape_attr(&block.iterable),
                self.render_nodes(&block.body, context)
            ),
            TemplateNode::AwaitBlock(block) => format!(
                r#"<template data-dx-await="{}">{}</template>"#,
                escape_attr(&block.promise),
                self.render_nodes(&block.pending_branch, context)
            ),
            TemplateNode::KeyBlock(block) => format!(
                r#"<template data-dx-key="{}">{}</template>"#,
                escape_attr(&block.key),
                self.render_nodes(&block.body, context)
            ),
            TemplateNode::Slot(slot) => self.render_slot(slot, context),
            TemplateNode::Component(component) => self.render_component(component, context),
        }
    }

    fn render_element(&mut self, element: &ElementNode, context: &RenderContext) -> String {
        let mut html = String::new();
        html.push('<');
        html.push_str(&element.tag);

        let has_static_id = element.attributes.contains_key("id");
        let generated_id = if has_static_id {
            None
        } else {
            self.take_event_target_id(element)
        };

        for (name, value) in sorted_map(&element.attributes) {
            match value {
                AttributeValue::Static(value) if value == "true" => {
                    html.push(' ');
                    html.push_str(name);
                }
                AttributeValue::Static(value) => {
                    html.push_str(&format!(r#" {name}="{}""#, escape_attr(value)));
                }
                AttributeValue::Dynamic(expression) => {
                    if let Some(value) = context.values.get(expression.trim()) {
                        html.push_str(&format!(r#" {name}="{}""#, escape_attr(value)));
                    } else {
                        html.push_str(&format!(
                            r#" data-dx-attr-{}="{}""#,
                            escape_attr(name),
                            escape_attr(expression)
                        ));
                    }
                }
            }
        }

        if let Some(id) = generated_id {
            html.push_str(&format!(r#" id="{}""#, escape_attr(&id)));
        }

        for (event, handler) in sorted_map(&element.events) {
            html.push_str(&format!(
                r#" data-dx-on-{}="{}""#,
                escape_attr(event),
                escape_attr(handler)
            ));
        }

        for (binding, expression) in sorted_map(&element.bindings) {
            html.push_str(&format!(
                r#" data-dx-bind-{}="{}""#,
                escape_attr(binding),
                escape_attr(expression)
            ));
        }

        for (class_name, expression) in sorted_map(&element.class_directives) {
            html.push_str(&format!(
                r#" data-dx-class-{}="{}""#,
                escape_attr(class_name),
                escape_attr(expression)
            ));
        }

        if let Some(transition) = &element.transition {
            html.push_str(&format!(
                r#" data-dx-transition="{}""#,
                escape_attr(transition)
            ));
        }

        for directive in &element.use_directives {
            html.push_str(&format!(r#" data-dx-use="{}""#, escape_attr(directive)));
        }

        if element.self_closing || is_void_element(&element.tag) {
            html.push('>');
            return html;
        }

        html.push('>');
        html.push_str(&self.render_nodes(&element.children, context));
        html.push_str("</");
        html.push_str(&element.tag);
        html.push('>');
        html
    }

    fn render_component(
        &mut self,
        component: &ComponentInstance,
        context: &RenderContext,
    ) -> String {
        let Some(file) = self.components.get(component.name.as_str()).copied() else {
            return format!(
                r#"<dx-component data-component="{}"></dx-component>"#,
                escape_attr(&component.name)
            );
        };

        let mut child_context = RenderContext {
            values: component
                .props
                .iter()
                .map(|(name, value)| {
                    let value = match value {
                        AttributeValue::Static(value) => value.clone(),
                        AttributeValue::Dynamic(expression) => context
                            .values
                            .get(expression.trim())
                            .cloned()
                            .unwrap_or_default(),
                    };
                    (name.clone(), value)
                })
                .collect(),
            ..Default::default()
        };

        if !component.children.is_empty() {
            child_context
                .slots
                .insert("default".to_string(), component.children.clone());
        }

        for (name, nodes) in &component.slots {
            child_context.slots.insert(name.clone(), nodes.clone());
        }

        self.render_nodes(&file.template, &child_context)
    }

    fn render_slot(&mut self, slot: &SlotNode, context: &RenderContext) -> String {
        let name = slot.name.as_deref().unwrap_or("default");
        if let Some(nodes) = context.slots.get(name) {
            return self.render_nodes(nodes, context);
        }
        self.render_nodes(&slot.fallback, context)
    }

    fn render_if_block(&mut self, block: &IfBlockNode, context: &RenderContext) -> String {
        let mut html = format!(
            r#"<template data-dx-if="{}">{}"#,
            escape_attr(&block.condition),
            self.render_nodes(&block.then_branch, context)
        );

        for (condition, branch) in &block.else_if_branches {
            html.push_str(&format!(
                r#"<template data-dx-else-if="{}">{}</template>"#,
                escape_attr(condition),
                self.render_nodes(branch, context)
            ));
        }

        if let Some(branch) = &block.else_branch {
            html.push_str(&format!(
                r#"<template data-dx-else>{}</template>"#,
                self.render_nodes(branch, context)
            ));
        }

        html.push_str("</template>");
        html
    }

    fn render_expression_value(&self, expression: &str, context: &RenderContext) -> String {
        if let Some(target) = self.runtime.expression_targets.get(expression.trim()) {
            return format!(
                r#"<span id="{}">{}</span>"#,
                escape_attr(&target.element_id),
                escape_text(&target.initial_value)
            );
        }

        context
            .values
            .get(expression.trim())
            .map(|value| escape_text(value))
            .unwrap_or_else(|| {
                format!(
                    r#"<span data-dx-expression="{}"></span>"#,
                    escape_attr(expression)
                )
            })
    }

    fn take_event_target_id(&mut self, element: &ElementNode) -> Option<String> {
        for (_, handler) in sorted_map(&element.events) {
            let Some(ids) = self.runtime.event_targets.get_mut(handler.trim()) else {
                continue;
            };
            if let Some(id) = ids.pop_front() {
                return Some(id);
            }
        }
        None
    }
}

fn sorted_map<T>(map: &std::collections::HashMap<String, T>) -> Vec<(&str, &T)> {
    let mut entries = map
        .iter()
        .map(|(key, value)| (key.as_str(), value))
        .collect::<Vec<_>>();
    entries.sort_by(|left, right| left.0.cmp(right.0));
    entries
}

fn is_void_element(tag: &str) -> bool {
    matches!(
        tag,
        "area"
            | "base"
            | "br"
            | "col"
            | "embed"
            | "hr"
            | "img"
            | "input"
            | "link"
            | "meta"
            | "param"
            | "source"
            | "track"
            | "wbr"
    )
}

fn escape_text(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn escape_attr(value: &str) -> String {
    escape_text(value).replace('"', "&quot;")
}
