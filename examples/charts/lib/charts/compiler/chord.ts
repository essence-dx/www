import { annularArcPath, clamp, polarPoint, round } from "../geometry";
import { formatValue, readField, toLabel, toNumber } from "../format";
import { createOrdinalScale } from "../scales";
import type { ChartSpec, Datum, MarkSpec, ValueFormat } from "../spec";
import type { ChartScene, LegendItem, SceneElement } from "../scene";
import { applyMarkTransforms } from "../transforms";
import { emptyScene, plotBounds, scene, sceneId, withPadding } from "./shared";

type ChordLink = {
  source: string;
  target: string;
  weight: number;
  rows: Datum[];
};

type ChordNode = {
  id: string;
  label: string;
  total: number;
  startAngle: number;
  endAngle: number;
  color: string;
};

const TAU = Math.PI * 2;

export function compileChord(spec: ChartSpec): ChartScene {
  const mark = spec.marks[0];
  const source = mark.encoding.source;
  const target = mark.encoding.target;
  const weight = mark.encoding.size ?? mark.encoding.y;
  if (!source || !target) return emptyScene(spec);

  const rows = applyMarkTransforms(spec.data, mark);
  const links = aggregateChordLinks(rows, source.field, target.field, weight?.field);
  if (links.length === 0) return emptyScene(spec);

  const bounds = plotBounds(spec, withPadding({ left: 58, right: 58, top: 42, bottom: 46 }));
  const cx = bounds.left + bounds.width / 2;
  const cy = bounds.top + bounds.height / 2;
  const outerRadius = Math.max(42, Math.min(bounds.width, bounds.height) / 2 - 20);
  const nodeWidth = clamp(outerRadius * (mark.chord?.nodeWidthRatio ?? 0.08), 10, 24);
  const ribbonRadius = outerRadius - nodeWidth - 8;
  const nodes = layoutChordNodes(links, mark);
  const nodeById = new Map(nodes.map((node) => [node.id, node]));
  const maxWeight = Math.max(...links.map((link) => link.weight), 1);
  const elements: SceneElement[] = [];

  links.forEach((link, index) => {
    const sourceNode = nodeById.get(link.source);
    const targetNode = nodeById.get(link.target);
    if (!sourceNode || !targetNode) return;

    const sourceAngle = midAngle(sourceNode);
    const targetAngle = link.source === link.target ? sourceAngle + 0.28 : midAngle(targetNode);
    elements.push({
      kind: "path",
      id: sceneId(mark.id, "ribbon", link.source, link.target, index),
      d: ribbonPath(cx, cy, ribbonRadius, sourceAngle, targetAngle, link.weight, maxWeight),
      fill: sourceNode.color,
      stroke: "hsl(var(--chart-muted-line))",
      strokeWidth: 1,
      opacity: 0.42 + Math.min(0.2, link.weight / maxWeight / 5),
      label: `${link.source} to ${link.target}: ${formatWeight(link.weight, weight?.format)}`,
      className: "chart-mark chart-mark-chord-ribbon",
      chordSource: link.source,
      chordTarget: link.target,
      chordWeight: String(round(link.weight)),
    });
  });

  nodes.forEach((node) => {
    elements.push({
      kind: "path",
      id: sceneId(mark.id, "node", node.id),
      d: annularArcPath(cx, cy, outerRadius - nodeWidth, outerRadius, node.startAngle, node.endAngle),
      fill: node.color,
      stroke: "hsl(var(--background))",
      strokeWidth: 1,
      opacity: 0.94,
      label: `${node.label}: ${formatWeight(node.total, weight?.format)}`,
      className: "chart-mark chart-mark-chord-node",
      chordNodeId: node.id,
      chordWeight: String(round(node.total)),
    });
  });

  nodes.forEach((node) => {
    const angle = midAngle(node);
    const position = polarPoint(cx, cy, outerRadius + 18, angle);
    elements.push({
      kind: "text",
      id: sceneId(mark.id, "label", node.id),
      x: position.x,
      y: position.y + 4,
      text: node.label,
      anchor: labelAnchor(angle),
      className: "chart-axis-label",
      chordNodeId: node.id,
      chordWeight: String(round(node.total)),
    });
  });

  return scene(spec, elements, legendFor(nodes));
}

export function aggregateChordLinks(rows: Datum[], sourceField: string, targetField: string, weightField?: string): ChordLink[] {
  const links = new Map<string, ChordLink>();

  rows.forEach((row) => {
    const source = toLabel(readField(row, sourceField));
    const target = toLabel(readField(row, targetField));
    if (!source || !target) return;

    const weight = weightField ? Math.max(0, toNumber(readField(row, weightField))) : 1;
    if (weight <= 0) return;

    const key = `${source}\u001f${target}`;
    const current = links.get(key);
    if (current) {
      current.weight += weight;
      current.rows.push(row);
      return;
    }

    links.set(key, { source, target, weight, rows: [row] });
  });

  return Array.from(links.values());
}

function layoutChordNodes(links: ChordLink[], mark: MarkSpec): ChordNode[] {
  const totals = new Map<string, number>();
  links.forEach((link) => {
    totals.set(link.source, (totals.get(link.source) ?? 0) + link.weight);
    totals.set(link.target, (totals.get(link.target) ?? 0) + link.weight);
  });

  const ordered = Array.from(totals, ([id, total]) => ({ id, total })).sort((left, right) => {
    if (mark.chord?.nodeSort === "weight-asc") return left.total - right.total || left.id.localeCompare(right.id);
    if (mark.chord?.nodeSort === "name-asc") return left.id.localeCompare(right.id);
    return right.total - left.total || left.id.localeCompare(right.id);
  });
  const color = createOrdinalScale(ordered.map((node) => node.id));
  const paddingAngle = clamp(mark.chord?.nodePaddingRatio ?? 0.024, 0.006, 0.09);
  const availableAngle = Math.max(TAU * 0.5, TAU - ordered.length * paddingAngle);
  const totalWeight = ordered.reduce((sum, node) => sum + node.total, 0) || 1;
  let cursor = -Math.PI / 2;

  return ordered.map((node) => {
    const span = Math.max(0.035, (node.total / totalWeight) * availableAngle);
    const startAngle = cursor;
    const endAngle = cursor + span;
    cursor = endAngle + paddingAngle;

    return {
      id: node.id,
      label: node.id,
      total: node.total,
      startAngle,
      endAngle,
      color: color.map(node.id),
    };
  });
}

function ribbonPath(cx: number, cy: number, radius: number, sourceAngle: number, targetAngle: number, weight: number, maxWeight: number): string {
  const angularWidth = clamp(0.012 + (weight / maxWeight) * 0.05, 0.014, 0.07);
  const sourceStart = polarPoint(cx, cy, radius, sourceAngle - angularWidth);
  const sourceEnd = polarPoint(cx, cy, radius, sourceAngle + angularWidth);
  const targetStart = polarPoint(cx, cy, radius, targetAngle - angularWidth);
  const targetEnd = polarPoint(cx, cy, radius, targetAngle + angularWidth);
  const sourceControl = polarPoint(cx, cy, radius * 0.16, sourceAngle);
  const targetControl = polarPoint(cx, cy, radius * 0.16, targetAngle);

  return [
    `M ${round(sourceStart.x)} ${round(sourceStart.y)}`,
    `C ${round(sourceControl.x)} ${round(sourceControl.y)}, ${round(targetControl.x)} ${round(targetControl.y)}, ${round(targetStart.x)} ${round(targetStart.y)}`,
    `L ${round(targetEnd.x)} ${round(targetEnd.y)}`,
    `C ${round(targetControl.x)} ${round(targetControl.y)}, ${round(sourceControl.x)} ${round(sourceControl.y)}, ${round(sourceEnd.x)} ${round(sourceEnd.y)}`,
    "Z",
  ].join(" ");
}

function legendFor(nodes: ChordNode[]): LegendItem[] {
  return nodes.map((node) => ({ label: node.label, color: node.color }));
}

function midAngle(node: Pick<ChordNode, "startAngle" | "endAngle">): number {
  return (node.startAngle + node.endAngle) / 2;
}

function labelAnchor(angle: number): "start" | "middle" | "end" {
  const horizontal = Math.cos(angle);
  if (horizontal > 0.2) return "start";
  if (horizontal < -0.2) return "end";
  return "middle";
}

function formatWeight(value: number, format?: ValueFormat): string {
  return formatValue(value, format ?? "compact");
}
