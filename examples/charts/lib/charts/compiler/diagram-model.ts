import type {
  DiagramConnectorSpec,
  DiagramEdgeSpec,
  DiagramInteractionSpec,
  DiagramModelSpec,
  DiagramNodeSpec,
  DiagramPortPosition,
  DiagramPortSpec,
  DiagramPortStateKind,
  DiagramRoutingSpec,
  GraphBehaviorSpec,
  GraphModelSpec,
} from "../spec";

const DEFAULT_NODE_WIDTH = 124;
const DEFAULT_NODE_HEIGHT = 56;
const DEFAULT_ROUTER: DiagramRoutingSpec = { name: "orth", padding: 12 };
const DEFAULT_CONNECTOR: DiagramConnectorSpec = { name: "rounded", radius: 8 };

export type NormalizedDiagramNode = Omit<DiagramNodeSpec, "height" | "ports" | "width"> & {
  width: number;
  height: number;
  ports: NormalizedDiagramPort[];
};

export type NormalizedDiagramEdge = Omit<DiagramEdgeSpec, "connector" | "router"> & {
  router: DiagramRoutingSpec;
  connector: DiagramConnectorSpec;
};

export type NormalizedDiagramPort = DiagramPortSpec & {
  position: DiagramPortPosition;
  state: DiagramPortStateKind;
};

export type NormalizedDiagramModel = {
  nodes: NormalizedDiagramNode[];
  edges: NormalizedDiagramEdge[];
  interactions: DiagramInteractionSpec[];
};

export function buildDiagramModel(spec: DiagramModelSpec): NormalizedDiagramModel {
  const nodeIds = new Set(spec.nodes.map((node) => node.id));
  const edges = spec.edges
    .filter((edge) => nodeIds.has(edge.source.cell) && nodeIds.has(edge.target.cell))
    .map((edge) => ({
      ...edge,
      router: edge.router ?? DEFAULT_ROUTER,
      connector: edge.connector ?? DEFAULT_CONNECTOR,
    }));
  const connectedPorts = connectedPortKeys(edges);
  const nodes = spec.nodes.map((node) => normalizeNode(node, connectedPorts));

  return {
    nodes,
    edges,
    interactions: (spec.interactions ?? []).filter((interaction) => interaction.enabled !== false),
  };
}

export function diagramModelToGraph(spec: DiagramModelSpec): GraphModelSpec {
  const model = buildDiagramModel(spec);
  const behaviors = model.interactions.map((interaction) => toGraphBehavior(interaction, model)).filter((behavior): behavior is GraphBehaviorSpec => Boolean(behavior));

  return {
    nodes: model.nodes.map((node) => ({
      id: node.id,
      label: node.label,
      type: node.shape ?? "rounded-rect",
      value: Math.max(4, node.ports.length + Math.round(node.width / 40)),
    })),
    edges: model.edges.map((edge) => ({
      id: edge.id,
      source: edge.source.cell,
      target: edge.target.cell,
      relation: edge.label ?? `${edge.source.cell} to ${edge.target.cell}`,
      weight: 3,
    })),
    layout: { type: "dagre-lite", rankDirection: "LR" },
    behaviors,
    plugins: [{ type: "tooltip" }, { type: "minimap" }],
  };
}

export function diagramSceneMetadata(model: NormalizedDiagramModel) {
  return {
    diagramInteractions: `${model.interactions.map((interaction) => interaction.type).join(",")}|ports=${diagramPortStateMetadata(model)}`,
  };
}

export function diagramPortStateMetadata(model: NormalizedDiagramModel): string {
  return model.nodes
    .flatMap((node) => node.ports.map((port) => `${node.id}.${port.id}:${port.group}:${port.position}:${port.state}`))
    .join("|") || "none";
}

export function connectedPortKeys(edges: Array<Pick<DiagramEdgeSpec, "source" | "target">>): Set<string> {
  const keys = new Set<string>();
  edges.forEach((edge) => {
    if (edge.source.port) keys.add(portKey(edge.source.cell, edge.source.port));
    if (edge.target.port) keys.add(portKey(edge.target.cell, edge.target.port));
  });
  return keys;
}

function normalizeNode(node: DiagramNodeSpec, connectedPorts: Set<string>): NormalizedDiagramNode {
  const ports = (node.ports && node.ports.length > 0 ? node.ports : defaultPorts()).map((port) => ({
    ...port,
    position: port.position ?? defaultPortPosition(port),
    state: diagramPortState(node.id, port, connectedPorts),
  }));

  return {
    ...node,
    shape: node.shape ?? "rounded-rect",
    width: node.width ?? DEFAULT_NODE_WIDTH,
    height: node.height ?? DEFAULT_NODE_HEIGHT,
    ports,
  };
}

function defaultPorts(): DiagramPortSpec[] {
  return [
    { id: "in", group: "input", label: "Input", position: "left" },
    { id: "out", group: "output", label: "Output", position: "right" },
  ];
}

function defaultPortPosition(port: DiagramPortSpec): DiagramPortPosition {
  if (port.group === "input") return "left";
  if (port.group === "output") return "right";
  return "bottom";
}

function diagramPortState(nodeId: string, port: DiagramPortSpec, connectedPorts: Set<string>): DiagramPortStateKind {
  if (port.state) return port.state;
  return connectedPorts.has(portKey(nodeId, port.id)) ? "connected" : "available";
}

function portKey(nodeId: string, portId: string): string {
  return `${nodeId}:${portId}`;
}

function toGraphBehavior(interaction: DiagramInteractionSpec, model: NormalizedDiagramModel): GraphBehaviorSpec | null {
  const firstNodeId = model.nodes[0]?.id;
  if (interaction.type === "drag-node") return { type: "drag-node" };
  if (interaction.type === "pan-canvas") return { type: "drag-canvas" };
  if (interaction.type === "zoom-canvas") return { type: "zoom-canvas" };
  if (interaction.type === "select-node" && firstNodeId) return { type: "focus-node", focus: { nodeId: firstNodeId, relationDepth: 1 } };
  if (interaction.type === "connect-port" && firstNodeId) return { type: "activate-relations", activation: { nodeId: firstNodeId } };
  return null;
}
