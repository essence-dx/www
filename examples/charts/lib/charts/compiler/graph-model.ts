import { readField, toLabel } from "../format";
import type { ChartSpec, GraphBehaviorSpec, GraphComboSpec, GraphEdgeSpec, GraphFocusSpec, GraphModelSpec, GraphNodeSpec, GraphPluginSpec, GraphRelationActivationSpec, MarkSpec } from "../spec";
import { applyMarkTransforms } from "../transforms";

export type NormalizedGraphModel = Required<Pick<GraphModelSpec, "nodes" | "edges">> & {
  combos: GraphComboSpec[];
  layout: NonNullable<GraphModelSpec["layout"]>;
  behaviors: NonNullable<GraphModelSpec["behaviors"]>;
  plugins: NonNullable<GraphModelSpec["plugins"]>;
};

export type GraphInteractionState = {
  focusNodeId?: string;
  focusedNodeIds: Set<string>;
  activatedEdgeIds: Set<string>;
  relationState: string;
};

export function graphModelFromSpec(spec: ChartSpec, mark: MarkSpec): NormalizedGraphModel {
  if (spec.graph) {
    return normalizeGraphModel(spec.graph);
  }

  const source = mark.encoding.source;
  const target = mark.encoding.target;
  if (!source || !target) return normalizeGraphModel({ nodes: [], edges: [] });

  const rows = applyMarkTransforms(spec.data, mark);
  const edges: GraphEdgeSpec[] = rows.map((datum, index) => ({
    id: toLabel(readField(datum, "id")) || `${mark.id}-edge-${index}`,
    source: toLabel(readField(datum, source.field)),
    target: toLabel(readField(datum, target.field)),
    label: toLabel(readField(datum, "label")) || toLabel(readField(datum, "relation")),
    relation: toLabel(readField(datum, "relation")),
  }));

  return normalizeGraphModel({ nodes: nodesFromEdges(edges), edges });
}

export function normalizeGraphModel(model: GraphModelSpec): NormalizedGraphModel {
  const nodeMap = new Map<string, GraphNodeSpec>();
  nodesFromEdges(model.edges).forEach((node) => nodeMap.set(node.id, node));
  model.nodes.forEach((node) => nodeMap.set(node.id, { ...nodeMap.get(node.id), ...node }));

  return {
    nodes: Array.from(nodeMap.values()),
    edges: model.edges.map((edge, index) => ({ ...edge, id: edge.id ?? `edge-${index}` })),
    combos: model.combos ?? [],
    layout: model.layout ?? { type: "circular" },
    behaviors: (model.behaviors ?? []).filter((behavior) => behavior.enabled !== false),
    plugins: (model.plugins ?? []).filter((plugin) => plugin.enabled !== false),
  };
}

export function graphInteractionState(model: NormalizedGraphModel): GraphInteractionState {
  const focus = model.behaviors.find((behavior) => behavior.type === "focus-node")?.focus;
  const activation = model.behaviors.find((behavior) => behavior.type === "activate-relations")?.activation;
  const focusNodeId = focus?.nodeId;
  const activatedEdgeIds = activatedGraphEdgeIds(model, activation, focusNodeId);

  return {
    focusNodeId,
    focusedNodeIds: focusedGraphNodeIds(model, focus, activatedEdgeIds),
    activatedEdgeIds,
    relationState: graphRelationState(model, activatedEdgeIds),
  };
}

export function focusedGraphNodeIds(model: NormalizedGraphModel, focus: GraphFocusSpec | undefined, activatedEdgeIds: Set<string>): Set<string> {
  const focused = new Set<string>();
  if (focus?.nodeId) {
    focused.add(focus.nodeId);
    addNeighborNodes(model, focused, focus.nodeId, focus.relationDepth ?? 0);
  }

  model.edges.forEach((edge) => {
    if (!activatedEdgeIds.has(edgeKey(edge))) return;
    focused.add(edge.source);
    focused.add(edge.target);
  });

  return focused;
}

export function activatedGraphEdgeIds(model: NormalizedGraphModel, activation: GraphRelationActivationSpec | undefined, focusNodeId: string | undefined): Set<string> {
  const active = new Set<string>();
  const activationNodeId = activation?.nodeId ?? focusNodeId;
  const activationEdgeIds = new Set(activation?.edgeIds ?? []);

  model.edges.forEach((edge) => {
    const edgeId = edgeKey(edge);
    if (activationEdgeIds.has(edgeId)) active.add(edgeId);
    if (activation?.relation && edge.relation === activation.relation) active.add(edgeId);
    if (activationNodeId && (edge.source === activationNodeId || edge.target === activationNodeId)) active.add(edgeId);
  });

  return active;
}

export function graphRelationState(model: NormalizedGraphModel, activatedEdgeIds: Set<string>): string {
  if (activatedEdgeIds.size === 0) return "none";
  return model.edges
    .filter((edge) => activatedEdgeIds.has(edgeKey(edge)))
    .map((edge) => `${edge.source}->${edge.target}:${edge.relation ?? "relation"}`)
    .join("|");
}

export function graphBehaviorState(model: NormalizedGraphModel): string {
  return model.behaviors.map(describeGraphBehavior).join(",") || "none";
}

export function describeGraphBehavior(behavior: GraphBehaviorSpec): string {
  if (behavior.type === "focus-node" && behavior.focus) {
    return `${behavior.type}:${behavior.focus.nodeId}:depth-${behavior.focus.relationDepth ?? 0}`;
  }
  if (behavior.type === "activate-relations" && behavior.activation) {
    const activation = behavior.activation;
    const parts = [
      activation.nodeId ? `node-${activation.nodeId}` : "",
      activation.relation ? `relation-${activation.relation}` : "",
      activation.edgeIds && activation.edgeIds.length > 0 ? `edges-${activation.edgeIds.join("+")}` : "",
    ].filter(Boolean);
    return `${behavior.type}:${parts.join(":") || "active"}`;
  }
  return behavior.type;
}

export function graphPluginState(model: NormalizedGraphModel): string {
  return model.plugins.map(describeGraphPlugin).join(",") || "none";
}

export function describeGraphPlugin(plugin: GraphPluginSpec): string {
  const placement = plugin.position ? `@${plugin.position}` : "";
  const target = plugin.target ? `:${plugin.target}` : "";
  return `${plugin.type}${placement}${target}`;
}

export function nodesFromEdges(edges: GraphEdgeSpec[]): GraphNodeSpec[] {
  const ids = new Set<string>();
  edges.forEach((edge) => {
    ids.add(edge.source);
    ids.add(edge.target);
  });
  return Array.from(ids).map((id) => ({ id, label: id }));
}

function addNeighborNodes(model: NormalizedGraphModel, focused: Set<string>, nodeId: string, depth: number) {
  if (depth <= 0) return;
  const neighbors = model.edges.flatMap((edge) => {
    if (edge.source === nodeId) return [edge.target];
    if (edge.target === nodeId) return [edge.source];
    return [];
  });

  neighbors.forEach((neighbor) => {
    if (focused.has(neighbor)) return;
    focused.add(neighbor);
    addNeighborNodes(model, focused, neighbor, depth - 1);
  });
}

export function edgeKey(edge: GraphEdgeSpec): string {
  return edge.id ?? `${edge.source}-${edge.target}`;
}
