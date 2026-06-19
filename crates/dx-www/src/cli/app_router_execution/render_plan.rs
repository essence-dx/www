use dx_compiler::delivery::{DxComponentEdge, DxComponentNode, DxReactAppRouteProof};
use serde_json::{Value, json};

pub(super) fn build_tsx_render_plan(route: &str, proof: &DxReactAppRouteProof) -> Value {
    let nodes = &proof.page_graph.components.nodes;
    let edges = &proof.page_graph.components.edges;
    let forge_owned = nodes
        .iter()
        .filter(|node| node.package_id.is_some())
        .count();
    let source_owned = nodes.len().saturating_sub(forge_owned);
    let state = &proof.route_unit.state;
    let state_slot_count = state.slots.len();
    let event_slot_count = state.event_slots.len();
    let effect_count = state.effects.len();
    let server_action_count = state.server_actions.len();
    let has_effects = effect_count > 0;
    let has_server_actions = server_action_count > 0;
    let state_runtime_required =
        state_runtime_required(state_slot_count, event_slot_count, effect_count);
    let client_island_required = state_slot_count > 0 || event_slot_count > 0;
    let status = render_plan_status(
        nodes.len(),
        state_slot_count,
        event_slot_count,
        has_effects,
        has_server_actions,
    );

    json!({
        "schema": "dx.tsx.renderPlan",
        "schema_revision": 1,
        "contract_name": "TSX Render Plan",
        "route": route,
        "status": status,
        "public_authoring": "tsx",
        "public_story": "React-familiar apps with source-owned packages and no hidden dependency surface.",
        "source_owned": true,
        "source_owned_render_plan": true,
        "external_runtime_required": false,
        "external_runtime_executed": false,
        "node_modules_required": false,
        "component_graph": {
            "root_component_id": proof.page_graph.root_component_id,
            "node_count": nodes.len(),
            "edge_count": edges.len(),
            "source_owned_nodes": source_owned,
            "forge_owned_nodes": forge_owned,
            "nodes": nodes.iter().map(component_node).collect::<Vec<_>>(),
            "edges": edges.iter().map(component_edge).collect::<Vec<_>>(),
        },
        "runtime_ready": {
            "crawlable_shell": true,
            "metadata": true,
            "layouts_and_templates": true,
            "dynamic_route_params": true,
            "search_params": true,
            "state_graph": true,
            "state_runtime": state_runtime_required,
            "client_island_manifest": client_island_required,
            "generated_dom_action_binder": event_slot_count > 0,
            "effect_scheduler": has_effects,
            "context_runtime": "source-owned-provider-value-map",
            "route_handlers": "safe-boundary-interpreter",
        },
        "runtime_boundaries": {
            "state_slots": state_slot_count,
            "event_slots": event_slot_count,
            "effects": effect_count,
            "server_actions": server_action_count,
            "effect_only_runtime": has_effects && state_slot_count == 0 && event_slot_count == 0,
            "server_action_boundary": has_server_actions,
            "state_runtime_required": state_runtime_required,
            "client_island_required": client_island_required,
        },
        "claim_policy": {
            "allowed": [
                "React-familiar TSX",
                "source-owned package boundaries",
                "no node_modules required for the DX/Forge path",
                "designed for fast static/dev paths when receipts prove it"
            ],
            "blocked_until_measured_or_implemented": [
                "drop-in React replacement",
                "drop-in App Router replacement",
                "faster than Next.js globally",
                "Forge package is a full replacement when it is only a boundary"
            ]
        },
        "production_blockers": production_blockers(state_slot_count, event_slot_count, has_effects, has_server_actions),
    })
}

fn render_plan_status(
    component_nodes: usize,
    state_slots: usize,
    event_slots: usize,
    has_effects: bool,
    has_server_actions: bool,
) -> &'static str {
    if has_effects || has_server_actions {
        "semantic-runtime-with-boundaries"
    } else if component_nodes <= 1 && state_slots == 0 && event_slots == 0 {
        "static-source-renderable"
    } else {
        "semantic-runtime-ready"
    }
}

fn state_runtime_required(state_slots: usize, event_slots: usize, effects: usize) -> bool {
    state_slots > 0 || event_slots > 0 || effects > 0
}

fn component_node(node: &DxComponentNode) -> Value {
    json!({
        "id": &node.id,
        "name": &node.name,
        "package_id": &node.package_id,
        "ownership": if node.package_id.is_some() { "forge-owned" } else { "local-source-owned" },
        "content_hash": &node.content_hash,
    })
}

fn component_edge(edge: &DxComponentEdge) -> Value {
    json!({
        "from": &edge.from,
        "to": &edge.to,
    })
}

fn production_blockers(
    state_slots: usize,
    event_slots: usize,
    has_effects: bool,
    has_server_actions: bool,
) -> Vec<Value> {
    let mut blockers = vec![
        json!({
            "kind": "component-import-execution",
            "status": "partial",
            "next_fix": "Execute source-owned component imports through the TSX renderer instead of only reporting graph edges."
        }),
        json!({
            "kind": "props-evaluation",
            "status": "partial",
            "next_fix": "Evaluate common literal, object, array, boolean, and expression props for source-owned components."
        }),
        json!({
            "kind": "context-runtime",
            "status": "partial",
            "next_fix": "Propagate provider values through the compiled component tree instead of exposing only a source-owned context value map."
        }),
        json!({
            "kind": "client-island-dom-binding",
            "status": if event_slots == 0 { "not-needed-for-route" } else { "partial" },
            "next_fix": "Bind generated state/event runtime operations to real JSX DOM nodes."
        }),
    ];
    if state_slots > 0 {
        blockers.push(json!({
            "kind": "dx-native-state-runtime",
            "status": "partial",
            "next_fix": "Lower DX-native state()/derived()/effect()/action() graph records into browser event handlers and snapshots; React hook syntax stays adapter-only unless exactly lowerable."
        }));
    }
    if has_effects {
        blockers.push(json!({
            "kind": "effect-ordering",
            "status": "partial",
            "next_fix": "Execute effect callback bodies and cleanup functions after dependency scheduling is live."
        }));
    }
    if has_server_actions {
        blockers.push(json!({
            "kind": "server-action-execution",
            "status": "boundary",
            "next_fix": "Execute server action edges through typed app-owned handlers instead of metadata-only edges."
        }));
    }
    blockers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_plan_marks_effect_only_routes_as_runtime_boundary() {
        assert_eq!(
            render_plan_status(1, 0, 0, true, false),
            "semantic-runtime-with-boundaries"
        );
        assert!(state_runtime_required(0, 0, 1));
    }

    #[test]
    fn render_plan_keeps_server_action_routes_out_of_static_status() {
        assert_eq!(
            render_plan_status(1, 0, 0, false, true),
            "semantic-runtime-with-boundaries"
        );
        assert!(!state_runtime_required(0, 0, 0));
    }
}
