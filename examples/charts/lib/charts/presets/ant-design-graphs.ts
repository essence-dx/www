import type { ChartSpec, Datum, GraphBehaviorSpec, GraphComboSpec, GraphEdgeSpec, GraphLayoutSpec, GraphNodeSpec, GraphModelSpec, GraphPluginSpec, GraphRelationActivationSpec } from "../spec";

export type AntDesignGraphPreset = {
  id: string;
  title: string;
  description: string;
  nodes: GraphNodeSpec[];
  edges: GraphEdgeSpec[];
  combos?: GraphComboSpec[];
  layout?: GraphLayoutSpec;
  behaviors?: GraphBehaviorSpec[];
  plugins?: GraphPluginSpec[];
  data?: Datum[];
  width?: number;
  height?: number;
};

export function antDesignGraph(config: AntDesignGraphPreset): ChartSpec {
  const graph: GraphModelSpec = {
    nodes: config.nodes,
    edges: config.edges,
    combos: config.combos,
    layout: config.layout ?? { type: "combo-cluster" },
    behaviors: config.behaviors ?? defaultGraphBehaviors(config),
    plugins: config.plugins ?? defaultGraphPlugins(),
  };

  return {
    id: config.id,
    title: config.title,
    description: config.description,
    task: "relation",
    family: "AntDesignGraphs",
    width: config.width ?? 640,
    height: config.height ?? 380,
    data: config.data ?? graph.edges.map((edge) => ({ source: edge.source, target: edge.target, relation: edge.relation ?? edge.label ?? edge.id ?? "", value: edge.weight ?? 1 })),
    graph,
    marks: [{ id: `${config.id}-graph`, type: "graph", encoding: { source: { field: "source", type: "nominal" }, target: { field: "target", type: "nominal" } } }],
  };
}

export function defaultGraphBehaviors(config: AntDesignGraphPreset): GraphBehaviorSpec[] {
  const behaviors: GraphBehaviorSpec[] = [{ type: "drag-node" }, { type: "zoom-canvas" }];
  const focusNodeId = config.nodes[0]?.id;
  const activationEdge = config.edges[0];

  if (focusNodeId) {
    behaviors.push({ type: "focus-node", focus: { nodeId: focusNodeId, relationDepth: 1 } });
  }

  if (activationEdge) {
    const activation: GraphRelationActivationSpec = {};
    const relation = activationEdge.relation ?? activationEdge.label;
    if (focusNodeId) activation.nodeId = focusNodeId;
    if (relation) activation.relation = relation;
    if (activationEdge.id) activation.edgeIds = [activationEdge.id];
    behaviors.push({ type: "activate-relations", activation });
  }

  return behaviors;
}

export function defaultGraphPlugins(): GraphPluginSpec[] {
  return [
    { type: "minimap", position: "bottom-right", target: "canvas" },
    { type: "tooltip", target: "selection" },
    { type: "legend", position: "top-right", target: "canvas" },
  ];
}
