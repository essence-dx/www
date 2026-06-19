import { polarPoint, round } from "../geometry";
import { CHART_PALETTE } from "../scales";
import type { GraphComboSpec, GraphEdgeSpec, GraphLayoutKind, GraphNodeSpec } from "../spec";
import type { PlotBounds } from "./shared";
import type { NormalizedGraphModel } from "./graph-model";

export type GraphLayoutNode = GraphNodeSpec & {
  x: number;
  y: number;
  color: string;
};

export type GraphLayoutCombo = GraphComboSpec & {
  x: number;
  y: number;
  r: number;
};

export type GraphLayoutEdge = GraphEdgeSpec & {
  id: string;
  route: "line" | "orth" | "arc" | "self";
  d: string;
  labelX: number;
  labelY: number;
};

export type GraphLayoutResult = {
  nodes: Map<string, GraphLayoutNode>;
  combos: Map<string, GraphLayoutCombo>;
  edges: Map<string, GraphLayoutEdge>;
};

type GraphNodeLayoutResult = Omit<GraphLayoutResult, "edges">;
type GraphPoint = { x: number; y: number };

export function layoutGraph(model: NormalizedGraphModel, bounds: PlotBounds, width: number, height: number): GraphLayoutResult {
  let layout: GraphNodeLayoutResult;
  if (model.layout.type === "grid") layout = gridLayout(model, bounds);
  else if (model.layout.type === "radial") layout = radialLayout(model, width, height);
  else if (model.layout.type === "dagre-lite") layout = dagreLiteLayout(model, bounds);
  else if (model.layout.type === "combo-cluster") layout = comboClusterLayout(model, width, height);
  else layout = circularLayout(model, width, height);

  return { ...layout, edges: routeGraphEdges(model, layout, model.layout.type) };
}

function circularLayout(model: NormalizedGraphModel, width: number, height: number): GraphNodeLayoutResult {
  const cx = width / 2;
  const cy = height / 2 + 10;
  const radius = model.layout.radius ?? Math.min(width, height) * 0.32;
  const nodes = new Map<string, GraphLayoutNode>();

  model.nodes.forEach((node, index) => {
    const angle = -Math.PI / 2 + (index / Math.max(1, model.nodes.length)) * Math.PI * 2;
    const point = polarPoint(cx, cy, radius, angle);
    nodes.set(node.id, withNodeLayout(node, point.x, point.y, index));
  });

  return { nodes, combos: new Map() };
}

function gridLayout(model: NormalizedGraphModel, bounds: PlotBounds): GraphNodeLayoutResult {
  const columns = Math.max(1, Math.ceil(Math.sqrt(model.nodes.length)));
  const rows = Math.max(1, Math.ceil(model.nodes.length / columns));
  const nodes = new Map<string, GraphLayoutNode>();

  model.nodes.forEach((node, index) => {
    const column = index % columns;
    const row = Math.floor(index / columns);
    const x = bounds.left + ((column + 0.5) / columns) * bounds.width;
    const y = bounds.top + ((row + 0.5) / rows) * bounds.height;
    nodes.set(node.id, withNodeLayout(node, x, y, index));
  });

  return { nodes, combos: new Map() };
}

function radialLayout(model: NormalizedGraphModel, width: number, height: number): GraphNodeLayoutResult {
  const nodes = new Map<string, GraphLayoutNode>();
  const incoming = incomingCounts(model.edges);
  const hub = model.nodes.reduce((best, node) => (incoming.get(node.id) ?? 0) > (incoming.get(best.id) ?? 0) ? node : best, model.nodes[0]);
  if (!hub) return { nodes, combos: new Map() };

  const cx = width / 2;
  const cy = height / 2 + 10;
  const radius = model.layout.radius ?? Math.min(width, height) * 0.34;
  nodes.set(hub.id, withNodeLayout(hub, cx, cy, 0));

  model.nodes.filter((node) => node.id !== hub.id).forEach((node, index, outerNodes) => {
    const angle = -Math.PI / 2 + (index / Math.max(1, outerNodes.length)) * Math.PI * 2;
    const point = polarPoint(cx, cy, radius, angle);
    nodes.set(node.id, withNodeLayout(node, point.x, point.y, index + 1));
  });

  return { nodes, combos: new Map() };
}

function dagreLiteLayout(model: NormalizedGraphModel, bounds: PlotBounds): GraphNodeLayoutResult {
  const ranks = rankNodes(model);
  const rankCount = Math.max(1, ranks.length);
  const nodes = new Map<string, GraphLayoutNode>();
  const leftToRight = model.layout.rankDirection === "LR";

  ranks.forEach((rank, rankIndex) => {
    rank.forEach((node, nodeIndex) => {
      const orderSize = Math.max(1, rank.length);
      const primary = rankCount === 1 ? 0.5 : rankIndex / (rankCount - 1);
      const secondary = (nodeIndex + 0.5) / orderSize;
      const x = leftToRight ? bounds.left + primary * bounds.width : bounds.left + secondary * bounds.width;
      const y = leftToRight ? bounds.top + secondary * bounds.height : bounds.top + primary * bounds.height;
      nodes.set(node.id, withNodeLayout(node, x, y, rankIndex + nodeIndex));
    });
  });

  return { nodes, combos: new Map() };
}

function comboClusterLayout(model: NormalizedGraphModel, width: number, height: number): GraphNodeLayoutResult {
  if (model.combos.length === 0) return circularLayout(model, width, height);

  const cx = width / 2;
  const cy = height / 2 + 10;
  const comboRadius = Math.min(width, height) * 0.26;
  const nodes = new Map<string, GraphLayoutNode>();
  const combos = new Map<string, GraphLayoutCombo>();

  model.combos.forEach((combo, comboIndex) => {
    const comboAngle = -Math.PI / 2 + (comboIndex / Math.max(1, model.combos.length)) * Math.PI * 2;
    const comboPoint = polarPoint(cx, cy, comboRadius, comboAngle);
    const members = model.nodes.filter((node) => node.combo === combo.id);
    const memberRadius = 34 + members.length * 4;
    combos.set(combo.id, { ...combo, x: comboPoint.x, y: comboPoint.y, r: Math.max(44, memberRadius + 18) });

    members.forEach((node, memberIndex) => {
      const angle = -Math.PI / 2 + (memberIndex / Math.max(1, members.length)) * Math.PI * 2;
      const point = polarPoint(comboPoint.x, comboPoint.y, memberRadius, angle);
      nodes.set(node.id, withNodeLayout(node, point.x, point.y, comboIndex + memberIndex));
    });
  });

  model.nodes.filter((node) => !nodes.has(node.id)).forEach((node, index) => {
    const point = polarPoint(cx, cy, Math.min(width, height) * 0.12, index);
    nodes.set(node.id, withNodeLayout(node, point.x, point.y, index));
  });

  return { nodes, combos };
}

function rankNodes(model: NormalizedGraphModel): GraphNodeSpec[][] {
  const nodeById = new Map(model.nodes.map((node) => [node.id, node]));
  const incoming = incomingCounts(model.edges);
  const outgoing = new Map<string, string[]>();
  model.edges.forEach((edge) => outgoing.set(edge.source, [...(outgoing.get(edge.source) ?? []), edge.target]));

  const ranks: GraphNodeSpec[][] = [];
  let frontier = model.nodes.filter((node) => (incoming.get(node.id) ?? 0) === 0);
  const seen = new Set<string>();

  while (frontier.length > 0) {
    ranks.push(frontier);
    frontier.forEach((node) => seen.add(node.id));
    frontier = frontier.flatMap((node) => outgoing.get(node.id) ?? []).map((id) => nodeById.get(id)).filter((node): node is GraphNodeSpec => Boolean(node) && !seen.has(node.id));
  }

  const remaining = model.nodes.filter((node) => !seen.has(node.id));
  return ranks.length > 0 ? [...ranks, ...(remaining.length > 0 ? [remaining] : [])] : [model.nodes];
}

export function routeGraphEdges(model: NormalizedGraphModel, layout: GraphNodeLayoutResult, layoutType: GraphLayoutKind): Map<string, GraphLayoutEdge> {
  const routes = new Map<string, GraphLayoutEdge>();
  const parallelCounts = new Map<string, number>();

  model.edges.forEach((edge) => {
    const edgePairKey = `${edge.source}->${edge.target}`;
    const parallelIndex = parallelCounts.get(edgePairKey) ?? 0;
    parallelCounts.set(edgePairKey, parallelIndex + 1);
    const graphEdgeRoute = routeGraphEdge(edge, { ...layout, edges: routes }, parallelIndex, layoutType);
    if (graphEdgeRoute) routes.set(graphEdgeRoute.id, graphEdgeRoute);
  });

  return routes;
}

export function routeGraphEdge(edge: GraphEdgeSpec, layout: Pick<GraphLayoutResult, "nodes">, parallelIndex = 0, layoutType: GraphLayoutKind = "circular"): GraphLayoutEdge | null {
  const source = layout.nodes.get(edge.source);
  const target = layout.nodes.get(edge.target);
  if (!source || !target) return null;

  const id = edge.id ?? `${edge.source}-${edge.target}`;
  const route = graphEdgeRoute(layoutType, source, target, parallelIndex);
  const label = labelPoint(source, target, route.control);

  return {
    ...edge,
    id,
    route: route.kind,
    d: route.d,
    labelX: label.x,
    labelY: label.y,
  };
}

function graphEdgeRoute(layoutType: GraphLayoutKind, source: GraphPoint, target: GraphPoint, parallelIndex: number): { kind: GraphLayoutEdge["route"]; d: string; control?: GraphPoint } {
  if (source.x === target.x && source.y === target.y) {
    const lift = 36 + parallelIndex * 10;
    return {
      kind: "self",
      d: `M ${round(source.x - 18)} ${round(source.y)} C ${round(source.x - 58)} ${round(source.y - lift)}, ${round(source.x + 58)} ${round(source.y - lift)}, ${round(source.x + 18)} ${round(source.y)}`,
      control: { x: source.x, y: source.y - lift },
    };
  }

  if (layoutType === "dagre-lite" || layoutType === "combo-cluster") {
    const middleX = (source.x + target.x) / 2 + parallelIndex * 12;
    return {
      kind: "orth",
      d: `M ${round(source.x)} ${round(source.y)} L ${round(middleX)} ${round(source.y)} L ${round(middleX)} ${round(target.y)} L ${round(target.x)} ${round(target.y)}`,
      control: { x: middleX, y: (source.y + target.y) / 2 },
    };
  }

  if (layoutType === "circular" || layoutType === "radial") {
    const control = curvedControlPoint(source, target, 0.18 + parallelIndex * 0.05);
    return {
      kind: "arc",
      d: `M ${round(source.x)} ${round(source.y)} Q ${round(control.x)} ${round(control.y)} ${round(target.x)} ${round(target.y)}`,
      control,
    };
  }

  return {
    kind: "line",
    d: `M ${round(source.x)} ${round(source.y)} L ${round(target.x)} ${round(target.y)}`,
  };
}

function curvedControlPoint(source: GraphPoint, target: GraphPoint, bend: number): GraphPoint {
  const middleX = (source.x + target.x) / 2;
  const middleY = (source.y + target.y) / 2;
  const dx = target.x - source.x;
  const dy = target.y - source.y;
  return {
    x: middleX - dy * bend,
    y: middleY + dx * bend,
  };
}

function labelPoint(source: GraphPoint, target: GraphPoint, control: GraphPoint | undefined): GraphPoint {
  if (control) {
    return {
      x: (source.x + target.x + control.x * 2) / 4,
      y: (source.y + target.y + control.y * 2) / 4,
    };
  }

  return {
    x: (source.x + target.x) / 2,
    y: (source.y + target.y) / 2,
  };
}

function incomingCounts(edges: GraphEdgeSpec[]): Map<string, number> {
  const counts = new Map<string, number>();
  edges.forEach((edge) => counts.set(edge.target, (counts.get(edge.target) ?? 0) + 1));
  return counts;
}

function withNodeLayout(node: GraphNodeSpec, x: number, y: number, index: number): GraphLayoutNode {
  return {
    ...node,
    x,
    y,
    color: CHART_PALETTE[index % CHART_PALETTE.length],
  };
}
