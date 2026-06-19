import { round } from "../geometry";
import type { DiagramConnectorSpec, DiagramPortPosition, DiagramPortSpec, DiagramRoutingSpec, DiagramTerminalSpec } from "../spec";
import type { NormalizedDiagramEdge, NormalizedDiagramModel, NormalizedDiagramNode, NormalizedDiagramPort } from "./diagram-model";

export type DiagramLayoutPort = NormalizedDiagramPort & {
  x: number;
  y: number;
  position: DiagramPortPosition;
};

export type DiagramLayoutNode = NormalizedDiagramNode & {
  x: number;
  y: number;
  ports: DiagramLayoutPort[];
};

export type DiagramLayoutResult = {
  nodes: Map<string, DiagramLayoutNode>;
};

type DiagramPoint = { x: number; y: number };

export function layoutDiagram(model: NormalizedDiagramModel, width: number, height: number): DiagramLayoutResult {
  const ranks = rankDiagramNodes(model);
  const nodes = new Map<string, DiagramLayoutNode>();
  const left = 88;
  const right = width - 88;
  const top = 76;
  const bottom = height - 76;
  const rankCount = Math.max(1, ranks.length);

  ranks.forEach((rank, rankIndex) => {
    rank.forEach((node, nodeIndex) => {
      const x = node.x ?? (rankCount === 1 ? width / 2 : left + (rankIndex / (rankCount - 1)) * (right - left));
      const y = node.y ?? top + ((nodeIndex + 0.5) / Math.max(1, rank.length)) * (bottom - top);
      const layoutNode: DiagramLayoutNode = { ...node, x, y, ports: [] };
      layoutNode.ports = layoutPorts(layoutNode);
      nodes.set(node.id, layoutNode);
    });
  });

  return { nodes };
}

export function routeDiagramEdge(edge: NormalizedDiagramEdge, layout: DiagramLayoutResult): string {
  const source = terminalPoint(layout, edge.source, "right");
  const target = terminalPoint(layout, edge.target, "left");
  const points = edge.vertices && edge.vertices.length > 0
    ? [source, ...edge.vertices, target]
    : edge.router.name === "manhattan"
      ? routeManhattanDiagramEdge(source, target, edge.router)
      : routePoints(source, target, edge.router);

  return connectPoints(points, edge.connector);
}

function layoutPorts(node: Omit<DiagramLayoutNode, "ports"> & { ports: NormalizedDiagramPort[] }): DiagramLayoutPort[] {
  return node.ports.map((port) => {
    const position = port.position ?? defaultPortPosition(port);
    const peers = node.ports.filter((candidate) => (candidate.position ?? defaultPortPosition(candidate)) === position);
    const ratio = (peers.findIndex((candidate) => candidate.id === port.id) + 1) / (peers.length + 1);
    const left = node.x - node.width / 2;
    const right = node.x + node.width / 2;
    const top = node.y - node.height / 2;
    const bottom = node.y + node.height / 2;

    if (position === "left") return { ...port, position, x: left, y: top + ratio * node.height };
    if (position === "right") return { ...port, position, x: right, y: top + ratio * node.height };
    if (position === "top") return { ...port, position, x: left + ratio * node.width, y: top };
    return { ...port, position, x: left + ratio * node.width, y: bottom };
  });
}

function terminalPoint(layout: DiagramLayoutResult, terminal: DiagramTerminalSpec, fallback: DiagramPortPosition): DiagramPoint {
  const node = layout.nodes.get(terminal.cell);
  if (!node) return { x: 0, y: 0 };

  const port = terminal.port ? node.ports.find((candidate) => candidate.id === terminal.port) : undefined;
  if (port) return { x: port.x, y: port.y };

  if (fallback === "right") return { x: node.x + node.width / 2, y: node.y };
  if (fallback === "left") return { x: node.x - node.width / 2, y: node.y };
  if (fallback === "top") return { x: node.x, y: node.y - node.height / 2 };
  return { x: node.x, y: node.y + node.height / 2 };
}

function routePoints(source: DiagramPoint, target: DiagramPoint, router: DiagramRoutingSpec): DiagramPoint[] {
  if (router.name === "normal") return [source, target];

  const offset = router.padding ?? 0;
  const direction = Math.sign(target.x - source.x || 1);
  const middleX = (source.x + target.x) / 2;
  return [
    source,
    { x: source.x + direction * offset, y: source.y },
    { x: middleX, y: source.y },
    { x: middleX, y: target.y },
    { x: target.x - direction * offset, y: target.y },
    target,
  ];
}

export function routeManhattanDiagramEdge(source: DiagramPoint, target: DiagramPoint, router: DiagramRoutingSpec): DiagramPoint[] {
  const offset = Math.max(10, router.padding ?? 16);
  const direction = Math.sign(target.x - source.x || 1);
  const middleY = (source.y + target.y) / 2;
  return [
    source,
    { x: source.x + direction * offset, y: source.y },
    { x: source.x + direction * offset, y: middleY },
    { x: target.x - direction * offset, y: middleY },
    { x: target.x - direction * offset, y: target.y },
    target,
  ];
}

function connectPoints(points: DiagramPoint[], connector: DiagramConnectorSpec): string {
  const [start, ...rest] = points;
  const end = points[points.length - 1];
  if (!start || !end) return "";

  if (connector.name === "smooth") {
    const controlX = (start.x + end.x) / 2;
    return `M ${round(start.x)} ${round(start.y)} C ${round(controlX)} ${round(start.y)}, ${round(controlX)} ${round(end.y)}, ${round(end.x)} ${round(end.y)}`;
  }

  if (connector.name === "rounded") {
    return connectRoundedPath(points, connector.radius ?? 8);
  }

  if (connector.name === "jumpover") {
    return connectJumpoverPath(points, connector.radius ?? 10);
  }

  return `M ${round(start.x)} ${round(start.y)} ${rest.map((point) => `L ${round(point.x)} ${round(point.y)}`).join(" ")}`;
}

export function connectRoundedPath(points: DiagramPoint[], radius: number): string {
  if (points.length <= 2) return connectPolyline(points);

  const [start] = points;
  const commands = [`M ${round(start.x)} ${round(start.y)}`];

  for (let index = 1; index < points.length - 1; index += 1) {
    const previous = points[index - 1];
    const current = points[index];
    const next = points[index + 1];
    const before = pointToward(current, previous, radius);
    const after = pointToward(current, next, radius);
    commands.push(`L ${round(before.x)} ${round(before.y)}`);
    commands.push(`Q ${round(current.x)} ${round(current.y)} ${round(after.x)} ${round(after.y)}`);
  }

  const end = points[points.length - 1];
  commands.push(`L ${round(end.x)} ${round(end.y)}`);
  return commands.join(" ");
}

export function connectJumpoverPath(points: DiagramPoint[], radius: number): string {
  if (points.length <= 2) return connectPolyline(points);

  const [start] = points;
  const commands = [`M ${round(start.x)} ${round(start.y)}`];

  for (let index = 1; index < points.length - 1; index += 1) {
    const previous = points[index - 1];
    const current = points[index];
    const next = points[index + 1];
    const before = pointToward(current, previous, radius);
    const after = pointToward(current, next, radius);
    commands.push(`L ${round(before.x)} ${round(before.y)}`);
    commands.push(`C ${round(current.x)} ${round(current.y - radius)}, ${round(current.x)} ${round(current.y + radius)}, ${round(after.x)} ${round(after.y)}`);
  }

  const end = points[points.length - 1];
  commands.push(`L ${round(end.x)} ${round(end.y)}`);
  return commands.join(" ");
}

function connectPolyline(points: DiagramPoint[]): string {
  const [start, ...rest] = points;
  if (!start) return "";
  return `M ${round(start.x)} ${round(start.y)} ${rest.map((point) => `L ${round(point.x)} ${round(point.y)}`).join(" ")}`;
}

function pointToward(point: DiagramPoint, target: DiagramPoint, distance: number): DiagramPoint {
  const dx = target.x - point.x;
  const dy = target.y - point.y;
  const length = Math.max(1, Math.sqrt(dx * dx + dy * dy));
  const offset = Math.min(distance, length / 2);
  return {
    x: point.x + (dx / length) * offset,
    y: point.y + (dy / length) * offset,
  };
}

function rankDiagramNodes(model: NormalizedDiagramModel): NormalizedDiagramNode[][] {
  const nodeById = new Map(model.nodes.map((node) => [node.id, node]));
  const incoming = new Map<string, number>();
  const outgoing = new Map<string, string[]>();

  model.edges.forEach((edge) => {
    incoming.set(edge.target.cell, (incoming.get(edge.target.cell) ?? 0) + 1);
    outgoing.set(edge.source.cell, [...(outgoing.get(edge.source.cell) ?? []), edge.target.cell]);
  });

  const ranks: NormalizedDiagramNode[][] = [];
  let frontier = model.nodes.filter((node) => (incoming.get(node.id) ?? 0) === 0);
  const seen = new Set<string>();

  while (frontier.length > 0) {
    ranks.push(frontier);
    frontier.forEach((node) => seen.add(node.id));
    frontier = frontier
      .flatMap((node) => outgoing.get(node.id) ?? [])
      .map((id) => nodeById.get(id))
      .filter((node): node is NormalizedDiagramNode => Boolean(node) && !seen.has(node.id));
  }

  const remaining = model.nodes.filter((node) => !seen.has(node.id));
  return ranks.length > 0 ? [...ranks, ...(remaining.length > 0 ? [remaining] : [])] : [model.nodes];
}

function defaultPortPosition(port: DiagramPortSpec): DiagramPortPosition {
  if (port.group === "input") return "left";
  if (port.group === "output") return "right";
  return "bottom";
}
