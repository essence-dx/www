import { annularArcPath, polarPoint } from "../geometry";
import { formatValue, readField, toLabel, toNumber } from "../format";
import { CHART_PALETTE } from "../scales";
import type { ChartSpec, Datum } from "../spec";
import type { ChartScene, SceneElement } from "../scene";
import { applyMarkTransforms } from "../transforms";
import { emptyScene, plotBounds, scene, sceneId, type PlotBounds, withPadding } from "./shared";

type HierarchyNode = {
  datum: Datum;
  name: string;
  parent: string;
  level: number;
  value: number;
};

type TreemapTile = {
  datum: Datum;
  x: number;
  y: number;
  width: number;
  height: number;
};

export function compileTreemap(spec: ChartSpec): ChartScene {
  const mark = spec.marks[0];
  const value = mark.encoding.size ?? mark.encoding.y ?? mark.encoding.theta;
  const label = mark.encoding.label ?? mark.encoding.x;
  if (!value || !label) return emptyScene(spec);

  const data = applyMarkTransforms(spec.data, mark);
  const bounds = plotBounds(spec, withPadding({ left: 24, right: 24, top: 28, bottom: 24 }));
  const total = data.reduce((sum, datum) => sum + Math.max(1, toNumber(readField(datum, value.field))), 0);
  const elements: SceneElement[] = [];

  sliceDiceTreemapTiles(data, bounds, value.field, total).forEach((tile, index) => {
    const labelText = toLabel(readField(tile.datum, label.field));
    elements.push({
      kind: "rect",
      id: sceneId(mark.id, "tile", index),
      x: tile.x,
      y: tile.y,
      width: tile.width,
      height: tile.height,
      fill: CHART_PALETTE[index % CHART_PALETTE.length],
      opacity: 0.88,
      radius: 7,
      label: `${labelText}: ${formatValue(readField(tile.datum, value.field), value.format)}`,
      className: "chart-mark chart-mark-treemap",
    });
    if (tile.width >= 48 && tile.height >= 36) {
      elements.push({ kind: "text", id: sceneId(mark.id, "tile-label", index), x: tile.x + 12, y: tile.y + 24, text: labelText, className: "chart-inside-label" });
    }
  });

  return scene(spec, elements, []);
}

export function compileSunburst(spec: ChartSpec): ChartScene {
  const mark = spec.marks[0];
  const value = mark.encoding.size ?? mark.encoding.y ?? mark.encoding.theta;
  const label = mark.encoding.label ?? mark.encoding.x;
  const level = mark.encoding.series ?? mark.encoding.color;
  const parent = mark.encoding.parent;
  if (!value || !label || !level) return emptyScene(spec);

  const data = applyMarkTransforms(spec.data, mark);
  const nodes = data
    .map((datum) => ({
      datum,
      name: toLabel(readField(datum, label.field)),
      parent: parent ? toLabel(readField(datum, parent.field)) : "",
      level: toNumber(readField(datum, level.field)),
      value: Math.max(1, toNumber(readField(datum, value.field))),
    }))
    .filter((node) => node.name.length > 0)
    .sort((left, right) => left.level - right.level || left.name.localeCompare(right.name));
  const cx = spec.width / 2;
  const cy = spec.height / 2 + 16;
  const baseRadius = Math.min(spec.width, spec.height) * 0.12;
  const ringWidth = Math.min(spec.width, spec.height) * 0.095;
  const elements: SceneElement[] = [];
  const childrenByParent = new Map<string, HierarchyNode[]>();
  nodes.forEach((node) => {
    childrenByParent.set(node.parent, [...(childrenByParent.get(node.parent) ?? []), node]);
  });
  const roots = childrenByParent.get("") ?? nodes.filter((node) => node.level === Math.min(...nodes.map((item) => item.level)));

  addSunburstPartition(elements, roots, childrenByParent, mark.id, value.field, value.format, 0, -Math.PI / 2, Math.PI * 1.5, cx, cy, baseRadius, ringWidth);

  const levels = Array.from(new Set(nodes.map((node) => String(node.level)))).sort(compareLevels);

  return scene(spec, elements, levels.map((levelName, index) => ({ label: `Level ${levelName}`, color: CHART_PALETTE[index % CHART_PALETTE.length] })));
}

function sliceDiceTreemapTiles(data: Datum[], bounds: PlotBounds, valueField: string, total: number): TreemapTile[] {
  if (data.length === 0 || total <= 0) {
    return [];
  }

  const gap = 6;
  const horizontal = bounds.width >= bounds.height;
  const available = Math.max(0, (horizontal ? bounds.width : bounds.height) - gap * (data.length - 1));
  let cursor = horizontal ? bounds.left : bounds.top;
  let allocated = 0;

  return data.map((datum, index) => {
    const raw = Math.max(1, toNumber(readField(datum, valueField)));
    const span = index === data.length - 1 ? Math.max(0, available - allocated) : Math.max(1, (raw / total) * available);
    allocated += span;
    const tile = horizontal
      ? { datum, x: cursor, y: bounds.top, width: span, height: bounds.height }
      : { datum, x: bounds.left, y: cursor, width: bounds.width, height: span };
    cursor += span + gap;
    return tile;
  });
}

function addSunburstPartition(
  elements: SceneElement[],
  nodes: HierarchyNode[],
  childrenByParent: Map<string, HierarchyNode[]>,
  markId: string,
  valueField: string,
  format: Parameters<typeof formatValue>[1],
  levelIndex: number,
  startAngle: number,
  endAngle: number,
  cx: number,
  cy: number,
  baseRadius: number,
  ringWidth: number,
) {
  const total = nodes.reduce((sum, node) => sum + node.value, 0) || 1;
  const innerRadius = baseRadius + levelIndex * ringWidth;
  const outerRadius = innerRadius + ringWidth * 0.88;
  let start = startAngle;

  nodes.forEach((node, index) => {
    const angle = (node.value / total) * (endAngle - startAngle);
    const color = CHART_PALETTE[(levelIndex + index) % CHART_PALETTE.length];
    elements.push({
      kind: "path",
      id: sceneId(markId, "sunburst", levelIndex, node.parent, node.name),
      d: annularArcPath(cx, cy, innerRadius, outerRadius, start, start + angle),
      fill: color,
      stroke: "hsl(var(--chart-stroke))",
      strokeWidth: 1,
      label: `${node.name}: ${formatValue(readField(node.datum, valueField), format)}`,
      className: "chart-mark chart-mark-sunburst",
    });

    if (angle > 0.42) {
      const point = polarPoint(cx, cy, (innerRadius + outerRadius) / 2, start + angle / 2);
      elements.push({ kind: "text", id: sceneId(markId, "sunburst-label", levelIndex, node.parent, node.name), x: point.x, y: point.y, text: node.name, anchor: "middle", className: "chart-inside-label" });
    }

    const children = childrenByParent.get(node.name) ?? [];
    if (children.length > 0) {
      addSunburstPartition(elements, children, childrenByParent, markId, valueField, format, levelIndex + 1, start, start + angle, cx, cy, baseRadius, ringWidth);
    }

    start += angle;
  });
}

function compareLevels(left: string, right: string) {
  const a = Number(left);
  const b = Number(right);
  if (Number.isFinite(a) && Number.isFinite(b)) return a - b;
  return left.localeCompare(right);
}
