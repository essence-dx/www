import { formatValue, readField, toLabel, toNumber } from "../format";
import { CHART_PALETTE } from "../scales";
import type { ChartSpec, Datum, FieldEncoding } from "../spec";
import type { SceneElement } from "../scene";
import { applyMarkTransforms } from "../transforms";
import { emptyScene, plotBounds, scene, sceneId, withPadding } from "./shared";

export function compileFunnel(spec: ChartSpec) {
  const mark = spec.marks[0];
  const label = mark.encoding.label ?? mark.encoding.x;
  const value = mark.encoding.size ?? mark.encoding.y ?? mark.encoding.theta;
  if (!label || !value) return emptyScene(spec);

  const rows = applyMarkTransforms(spec.data, mark);
  if (rows.length === 0) return emptyScene(spec);

  const bounds = plotBounds(spec, withPadding({ left: 48, right: 48, top: 34, bottom: 34 }));
  const maxValue = Math.max(1, ...rows.map((datum) => toNumber(readField(datum, value.field))));
  const gap = 7;
  const stageHeight = (bounds.height - gap * Math.max(0, rows.length - 1)) / rows.length;
  const elements: SceneElement[] = [];

  rows.forEach((datum, index) => {
    const next = rows[index + 1] ?? datum;
    const y = bounds.top + index * (stageHeight + gap);
    const currentWidth = stageWidth(datum, value, maxValue, bounds.width);
    const nextWidth = stageWidth(next, value, maxValue, bounds.width);
    const points = trapezoidPoints(bounds.left + bounds.width / 2, y, stageHeight, currentWidth, nextWidth);
    const name = toLabel(readField(datum, label.field));
    const raw = readField(datum, value.field);

    elements.push({
      kind: "polygon",
      id: sceneId(mark.id, "stage", index),
      points,
      fill: CHART_PALETTE[index % CHART_PALETTE.length],
      opacity: mark.style?.opacity ?? 0.9,
      label: `${name}: ${formatValue(raw, value.format)}`,
      className: "chart-mark chart-mark-funnel",
    });
    elements.push({
      kind: "text",
      id: sceneId(mark.id, "stage-label", index),
      x: bounds.left + bounds.width / 2,
      y: y + stageHeight / 2 + 5,
      text: `${name} ${formatValue(raw, value.format)}`,
      anchor: "middle",
      className: "chart-inside-label",
    });
  });

  return scene(spec, elements, []);
}

function stageWidth(datum: Datum, value: FieldEncoding, maxValue: number, maxWidth: number): number {
  return Math.max(42, (toNumber(readField(datum, value.field)) / maxValue) * maxWidth);
}

function trapezoidPoints(cx: number, y: number, height: number, topWidth: number, bottomWidth: number): string {
  const topLeft = cx - topWidth / 2;
  const topRight = cx + topWidth / 2;
  const bottomLeft = cx - bottomWidth / 2;
  const bottomRight = cx + bottomWidth / 2;

  return `${round(topLeft)},${round(y)} ${round(topRight)},${round(y)} ${round(bottomRight)},${round(y + height)} ${round(bottomLeft)},${round(y + height)}`;
}

function round(value: number): number {
  return Math.round(value * 100) / 100;
}
