use std::collections::{BTreeMap, VecDeque};

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::dx_parser::{AttributeValue, DxFile, ElementNode, TemplateNode};

use super::micro_js::DxMicroJsEmitter;
use super::types::{DxDeliveryMode, DxMicroJsAction, DxMicroJsOp, DxMicroJsProgram};
use super::vertical_render::{
    DxVerticalAstComponent, DxVerticalExpressionTarget, DxVerticalRenderRuntime,
};

/// Compiler-derived interaction proof for a vertical slice.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DxVerticalInteractionProof {
    /// Selected browser delivery mode.
    pub delivery_mode: DxDeliveryMode,
    /// State variable mirrored into the DOM.
    pub state_name: String,
    /// Initial numeric state value.
    pub initial_value: i64,
    /// DOM id whose text mirrors the state value.
    pub target_id: String,
    /// Tiny generated runtime program.
    pub program: DxMicroJsProgram,
    /// Inline script bytes.
    pub script_bytes: usize,
    /// Honest caveats for this early proof path.
    pub warnings: Vec<String>,
}

pub(super) struct DxVerticalInteractionOutput {
    pub(super) proof: DxVerticalInteractionProof,
    pub(super) runtime: DxVerticalRenderRuntime,
}

struct EventCandidate {
    handler: String,
    element_id: Option<String>,
}

/// Detect the first v0 counter interaction that can run without WASM.
pub(super) fn compile_vertical_interaction(
    page: &DxFile,
    components: &[DxVerticalAstComponent],
) -> Option<DxVerticalInteractionOutput> {
    let state = page
        .scripts
        .iter()
        .flat_map(|script| script.state_vars.iter())
        .filter_map(|state| {
            state
                .initial_value
                .as_deref()
                .and_then(parse_i64_literal)
                .map(|initial| (state.name.clone(), initial))
        })
        .next()?;

    let mut collector = InteractionCollector::new(components);
    collector.collect_nodes(&page.template);

    let (state_name, initial_value) = state;
    if !collector.expressions.iter().any(|expr| expr == &state_name) {
        return None;
    }

    let operations = page
        .scripts
        .iter()
        .map(|script| script.content.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    let mut event_targets: BTreeMap<String, VecDeque<String>> = BTreeMap::new();
    let mut actions = Vec::new();
    let mut generated_index = 0usize;

    for event in collector.events {
        let Some(op) = handler_operation(&operations, &state_name, &event.handler) else {
            continue;
        };
        let element_id = event.element_id.unwrap_or_else(|| {
            generated_index += 1;
            let id = format!(
                "dx-action-{}-{generated_index}",
                sanitize_id(&event.handler)
            );
            event_targets
                .entry(event.handler.clone())
                .or_default()
                .push_back(id.clone());
            id
        });
        actions.push(DxMicroJsAction {
            element_id,
            event: "click".to_string(),
            target_id: None,
            initial_value: None,
            op,
        });
    }

    if actions.is_empty() {
        return None;
    }

    let target_id = format!("dx-state-{}", sanitize_id(&state_name));
    let program = DxMicroJsProgram {
        initial_value,
        target_id: target_id.clone(),
        actions,
    };
    let script = DxMicroJsEmitter::emit_script_tag(&program);
    let script_bytes = script.len();
    let runtime = DxVerticalRenderRuntime {
        expression_targets: BTreeMap::from([(
            state_name.clone(),
            DxVerticalExpressionTarget {
                element_id: target_id.clone(),
                initial_value: initial_value.to_string(),
            },
        )]),
        event_targets,
        script: Some(script),
    };
    let proof = DxVerticalInteractionProof {
        delivery_mode: DxDeliveryMode::MicroJs,
        state_name,
        initial_value,
        target_id,
        program,
        script_bytes,
        warnings: vec![
            "Phase 1 interaction proof supports simple numeric counter state and click handlers only."
                .to_string(),
        ],
    };

    Some(DxVerticalInteractionOutput { proof, runtime })
}

struct InteractionCollector<'a> {
    components: BTreeMap<&'a str, &'a DxFile>,
    expressions: Vec<String>,
    events: Vec<EventCandidate>,
}

impl<'a> InteractionCollector<'a> {
    fn new(components: &'a [DxVerticalAstComponent]) -> Self {
        Self {
            components: components
                .iter()
                .map(|component| (component.name.as_str(), &component.file))
                .collect(),
            expressions: Vec::new(),
            events: Vec::new(),
        }
    }

    fn collect_nodes(&mut self, nodes: &[TemplateNode]) {
        for node in nodes {
            self.collect_node(node);
        }
    }

    fn collect_node(&mut self, node: &TemplateNode) {
        match node {
            TemplateNode::Expression(expression) => {
                self.expressions
                    .push(expression.expression.trim().to_string());
            }
            TemplateNode::Element(element) => self.collect_element(element),
            TemplateNode::IfBlock(block) => {
                self.collect_nodes(&block.then_branch);
                for (_, branch) in &block.else_if_branches {
                    self.collect_nodes(branch);
                }
                if let Some(branch) = &block.else_branch {
                    self.collect_nodes(branch);
                }
            }
            TemplateNode::EachBlock(block) => {
                self.collect_nodes(&block.body);
                if let Some(branch) = &block.empty_branch {
                    self.collect_nodes(branch);
                }
            }
            TemplateNode::AwaitBlock(block) => {
                self.collect_nodes(&block.pending_branch);
                self.collect_nodes(&block.then_branch);
                self.collect_nodes(&block.catch_branch);
            }
            TemplateNode::KeyBlock(block) => self.collect_nodes(&block.body),
            TemplateNode::Slot(slot) => self.collect_nodes(&slot.fallback),
            TemplateNode::Component(component) => {
                if let Some(file) = self.components.get(component.name.as_str()).copied() {
                    self.collect_nodes(&file.template);
                }
                self.collect_nodes(&component.children);
                for nodes in component.slots.values() {
                    self.collect_nodes(nodes);
                }
            }
            TemplateNode::Text(_) => {}
        }
    }

    fn collect_element(&mut self, element: &ElementNode) {
        let element_id = static_id(element);
        for (event, handler) in &element.events {
            if event == "click" {
                self.events.push(EventCandidate {
                    handler: handler.trim().to_string(),
                    element_id: element_id.clone(),
                });
            }
        }
        self.collect_nodes(&element.children);
    }
}

fn static_id(element: &ElementNode) -> Option<String> {
    match element.attributes.get("id") {
        Some(AttributeValue::Static(value)) if !value.trim().is_empty() => {
            Some(value.trim().to_string())
        }
        _ => None,
    }
}

fn handler_operation(script: &str, state_name: &str, handler: &str) -> Option<DxMicroJsOp> {
    let body = function_body(script, handler)?;
    let escaped_state = regex::escape(state_name);

    let add = Regex::new(&format!(r"{escaped_state}\s*\+=\s*(-?\d+)")).ok()?;
    if let Some(value) = add
        .captures(&body)
        .and_then(|captures| captures.get(1))
        .and_then(|value| value.as_str().parse::<i64>().ok())
    {
        return Some(DxMicroJsOp::Add(value));
    }

    let subtract = Regex::new(&format!(r"{escaped_state}\s*-=\s*(\d+)")).ok()?;
    if let Some(value) = subtract
        .captures(&body)
        .and_then(|captures| captures.get(1))
        .and_then(|value| value.as_str().parse::<i64>().ok())
    {
        return Some(DxMicroJsOp::Add(-value));
    }

    let set = Regex::new(&format!(r"{escaped_state}\s*=\s*(-?\d+)")).ok()?;
    set.captures(&body)
        .and_then(|captures| captures.get(1))
        .and_then(|value| value.as_str().parse::<i64>().ok())
        .map(DxMicroJsOp::Set)
}

fn function_body(script: &str, handler: &str) -> Option<String> {
    let escaped_handler = regex::escape(handler);
    let rust = Regex::new(&format!(
        r"(?s)fn\s+{escaped_handler}\s*\([^)]*\)(?:\s*->\s*\w+)?\s*\{{(?P<body>.*?)\}}"
    ))
    .ok()?;
    if let Some(body) = rust
        .captures(script)
        .and_then(|captures| captures.name("body"))
        .map(|body| body.as_str().to_string())
    {
        return Some(body);
    }

    let js_function = Regex::new(&format!(
        r"(?s)function\s+{escaped_handler}\s*\([^)]*\)\s*\{{(?P<body>.*?)\}}"
    ))
    .ok()?;
    if let Some(body) = js_function
        .captures(script)
        .and_then(|captures| captures.name("body"))
        .map(|body| body.as_str().to_string())
    {
        return Some(body);
    }

    let js_arrow = Regex::new(&format!(
        r"(?s)(?:const|let|var)\s+{escaped_handler}\s*=\s*\([^)]*\)\s*=>\s*\{{(?P<body>.*?)\}}"
    ))
    .ok()?;
    js_arrow
        .captures(script)
        .and_then(|captures| captures.name("body"))
        .map(|body| body.as_str().to_string())
}

fn parse_i64_literal(value: &str) -> Option<i64> {
    value.trim().trim_end_matches(';').parse::<i64>().ok()
}

fn sanitize_id(value: &str) -> String {
    let id = value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string();

    if id.is_empty() {
        "item".to_string()
    } else {
        id
    }
}
