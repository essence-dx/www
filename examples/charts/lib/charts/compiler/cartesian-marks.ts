import { areaPath, clamp, linePath } from "../geometry";
import { extent, formatValue, readField, toLabel, toNumber, uniqueLabels } from "../format";
import {
  CHART_PALETTE,
  createBandScale,
  createLinearScale,
  type BandScale,
  type ContinuousScale,
  type OrdinalScale,
  type PointScale,
} from "../scales";
import type { ChartPrimitive, ChartSpec, Datum, FieldEncoding, MarkSpec } from "../spec";
import type { SceneElement } from "../scene";
import { DODGE_COUNT_FIELD, DODGE_INDEX_FIELD, STACK_END_FIELD, STACK_START_FIELD } from "../transforms";
import { groupData, type PlotBounds } from "./shared";

export type CartesianXScale = BandScale | PointScale | ContinuousScale;

export function yValuesForDomain(mark: MarkSpec, datum: Datum, fallback: FieldEncoding) {
  const stackStart = readField(datum, STACK_START_FIELD);
  const stackEnd = readField(datum, STACK_END_FIELD);
  if (stackStart !== null || stackEnd !== null) {
    return [stackStart, stackEnd];
  }

  const fields = [
    mark.encoding.y,
    mark.encoding.low,
    mark.encoding.q1,
    mark.encoding.median,
    mark.encoding.q3,
    mark.encoding.high,
    mark.encoding.target,
    fallback,
  ].filter((field): field is FieldEncoding => Boolean(field));

  return fields.map((field) => readField(datum, field.field));
}

export function addBars(
  elements: SceneElement[],
  data: Datum[],
  mark: MarkSpec,
  x: FieldEncoding,
  y: FieldEncoding,
  xScale: CartesianXScale,
  yScale: ContinuousScale,
  colorField: FieldEncoding | undefined,
  colorScale: OrdinalScale,
) {
  if (!isBandScale(xScale)) return;

  const zero = yScale.map(0);
  data.forEach((datum, index) => {
    const value = readField(datum, y.field);
    const colorValue = colorField ? readField(datum, colorField.field) : mark.id;
    const layout = barLayout(datum, x, xScale);
    const stackStart = readField(datum, STACK_START_FIELD);
    const stackEnd = readField(datum, STACK_END_FIELD);
    const startValue = stackStart === null ? 0 : stackStart;
    const endValue = stackEnd === null ? value : stackEnd;
    const labelValue = stackStart === null || stackEnd === null ? value : toNumber(stackEnd) - toNumber(stackStart);
    const startY = stackEnd === null ? zero : yScale.map(startValue);
    const endY = yScale.map(endValue);
    elements.push({
      kind: "rect",
      id: `${mark.id}-bar-${index}`,
      x: layout.x,
      y: Math.min(endY, startY),
      width: layout.width,
      height: Math.max(2, Math.abs(startY - endY)),
      fill: mark.style?.fill ?? colorScale.map(colorValue),
      opacity: mark.style?.opacity ?? 0.92,
      radius: mark.style?.radius ?? 5,
      label: `${toLabel(readField(datum, x.field))}: ${formatValue(labelValue, y.format)}`,
      className: "chart-mark chart-mark-bar",
      ...adviceMetadata(datum),
    });
  });
}

export function addLines(
  elements: SceneElement[],
  data: Datum[],
  mark: MarkSpec,
  x: FieldEncoding,
  y: FieldEncoding,
  xScale: CartesianXScale,
  yScale: ContinuousScale,
  colorField: FieldEncoding | undefined,
  colorScale: OrdinalScale,
) {
  for (const group of groupData(data, colorField)) {
    const points = group.rows.map((datum) => ({
      x: xScale.map(readField(datum, x.field)),
      y: yScale.map(readField(datum, y.field)),
    }));

    elements.push({
      kind: "path",
      id: `${mark.id}-line-${group.key}`,
      d: linePath(points),
      fill: "none",
      stroke: mark.style?.stroke ?? colorScale.map(group.key),
      strokeWidth: mark.style?.strokeWidth ?? 3,
      label: `${group.key} ${mark.type}`,
      className: "chart-mark chart-mark-line",
    });
  }
}

export function addAreas(
  elements: SceneElement[],
  data: Datum[],
  mark: MarkSpec,
  x: FieldEncoding,
  y: FieldEncoding,
  xScale: CartesianXScale,
  yScale: ContinuousScale,
  colorField: FieldEncoding | undefined,
  colorScale: OrdinalScale,
  baseline: number,
) {
  for (const group of groupData(data, colorField)) {
    const points = group.rows.map((datum) => ({
      x: xScale.map(readField(datum, x.field)),
      y: yScale.map(readField(datum, y.field)),
    }));

    elements.push({
      kind: "path",
      id: `${mark.id}-area-${group.key}`,
      d: areaPath(points, baseline),
      fill: mark.style?.fill ?? colorScale.map(group.key),
      opacity: mark.style?.opacity ?? 0.22,
      label: `${group.key} area`,
      className: "chart-mark chart-mark-area",
    });
  }
}

export function addPoints(
  elements: SceneElement[],
  data: Datum[],
  mark: MarkSpec,
  x: FieldEncoding,
  y: FieldEncoding,
  xScale: CartesianXScale,
  yScale: ContinuousScale,
  colorField: FieldEncoding | undefined,
  colorScale: OrdinalScale,
) {
  const size = mark.encoding.size;
  data.forEach((datum, index) => {
    const colorValue = colorField ? readField(datum, colorField.field) : mark.id;
    const radius = size ? clamp(Math.sqrt(toNumber(readField(datum, size.field))) * 1.6, 4, 16) : mark.style?.radius ?? 6;
    elements.push({
      kind: "circle",
      id: `${mark.id}-point-${index}`,
      cx: xScale.map(readField(datum, x.field)),
      cy: yScale.map(readField(datum, y.field)),
      r: radius,
      fill: mark.style?.fill ?? colorScale.map(colorValue),
      stroke: "hsl(var(--chart-stroke))",
      opacity: mark.style?.opacity ?? 0.9,
      label: `${toLabel(readField(datum, x.field))}: ${formatValue(readField(datum, y.field), y.format)}`,
      className: "chart-mark chart-mark-point",
    });
  });
}

export function addRules(elements: SceneElement[], data: Datum[], mark: MarkSpec, y: FieldEncoding, yScale: ContinuousScale, bounds: PlotBounds) {
  data.forEach((datum, index) => {
    const yy = yScale.map(readField(datum, y.field));
    elements.push({
      kind: "line",
      id: `${mark.id}-rule-${index}`,
      x1: bounds.left,
      y1: yy,
      x2: bounds.right,
      y2: yy,
      stroke: mark.style?.stroke ?? "hsl(var(--chart-warning))",
      strokeWidth: mark.style?.strokeWidth ?? 2,
      label: `${mark.id}: ${formatValue(readField(datum, y.field), y.format)}`,
      className: "chart-mark chart-mark-rule",
    });
  });
}

export function addHeatmap(elements: SceneElement[], data: Datum[], mark: MarkSpec, x: FieldEncoding, y: FieldEncoding, bounds: PlotBounds) {
  const color = mark.encoding.color;
  const size = mark.encoding.size;
  const xScale = createBandScale(uniqueLabels(data.map((datum) => readField(datum, x.field))), [bounds.left, bounds.right], 0.08);
  const yScale = createBandScale(uniqueLabels(data.map((datum) => readField(datum, y.field))), [bounds.top, bounds.bottom], 0.08);
  const values = color ? extent(data.map((datum) => readField(datum, color.field))) : [0, 1] as [number, number];
  const sizeDomain = size ? extent(data.map((datum) => readField(datum, size.field))) : [0, 1] as [number, number];
  const span = values[1] - values[0] || 1;
  const cellWidth = xScale.bandwidth();
  const cellHeight = yScale.bandwidth();

  data.forEach((datum, index) => {
    const intensity = color ? (toNumber(readField(datum, color.field)) - values[0]) / span : 0.5;
    const cellX = xScale.map(readField(datum, x.field));
    const cellY = yScale.map(readField(datum, y.field));
    const sizeRatio = size ? heatmapSizeRatio(readField(datum, size.field), sizeDomain) : 1;
    const drawWidth = cellWidth * sizeRatio;
    const drawHeight = cellHeight * sizeRatio;
    const labelParts = [`${toLabel(readField(datum, x.field))} ${toLabel(readField(datum, y.field))}`];
    if (color) labelParts.push(formatValue(readField(datum, color.field), color.format));
    if (size) labelParts.push(`size ${formatValue(readField(datum, size.field), size.format)}`);
    elements.push({
      kind: "rect",
      id: `${mark.id}-heat-${index}`,
      x: cellX + (cellWidth - drawWidth) / 2,
      y: cellY + (cellHeight - drawHeight) / 2,
      width: drawWidth,
      height: drawHeight,
      fill: CHART_PALETTE[Math.round(clamp(intensity, 0, 1) * (CHART_PALETTE.length - 1))],
      opacity: 0.88,
      radius: 5,
      label: labelParts.join(": "),
      className: "chart-mark chart-mark-heatmap",
    });
  });
}

export function addBoxplots(elements: SceneElement[], data: Datum[], mark: MarkSpec, x: FieldEncoding, y: FieldEncoding, xScale: CartesianXScale, yScale: ContinuousScale) {
  const low = mark.encoding.low;
  const q1 = mark.encoding.q1;
  const median = mark.encoding.median ?? y;
  const q3 = mark.encoding.q3;
  const high = mark.encoding.high;
  if (!low || !q1 || !q3 || !high || !isBandScale(xScale)) return;

  data.forEach((datum, index) => {
    const center = xScale.map(readField(datum, x.field)) + xScale.bandwidth() / 2;
    const boxWidth = xScale.bandwidth() * 0.58;
    const q1Y = yScale.map(readField(datum, q1.field));
    const q3Y = yScale.map(readField(datum, q3.field));
    const lowY = yScale.map(readField(datum, low.field));
    const highY = yScale.map(readField(datum, high.field));
    const medianY = yScale.map(readField(datum, median.field));
    const name = toLabel(readField(datum, x.field));

    elements.push({ kind: "line", id: `${mark.id}-whisker-${index}`, x1: center, y1: highY, x2: center, y2: lowY, stroke: "hsl(var(--chart-stroke))", strokeWidth: 2, label: `${name} range`, className: "chart-mark chart-mark-boxplot" });
    elements.push({ kind: "rect", id: `${mark.id}-box-${index}`, x: center - boxWidth / 2, y: Math.min(q1Y, q3Y), width: boxWidth, height: Math.max(4, Math.abs(q3Y - q1Y)), fill: mark.style?.fill ?? CHART_PALETTE[index % CHART_PALETTE.length], opacity: 0.42, radius: 5, label: `${name}: median ${formatValue(readField(datum, median.field), median.format)}`, className: "chart-mark chart-mark-boxplot" });
    elements.push({ kind: "line", id: `${mark.id}-median-${index}`, x1: center - boxWidth / 2, y1: medianY, x2: center + boxWidth / 2, y2: medianY, stroke: "hsl(var(--foreground))", strokeWidth: 2, label: `${name} median`, className: "chart-mark chart-mark-boxplot" });
  });
}

export function addBullets(elements: SceneElement[], data: Datum[], mark: MarkSpec, x: FieldEncoding, y: FieldEncoding, xScale: CartesianXScale, yScale: ContinuousScale, bounds: PlotBounds) {
  const target = mark.encoding.target;
  if (!isBandScale(xScale)) return;

  const zero = yScale.map(0);
  data.forEach((datum, index) => {
    const category = readField(datum, x.field);
    const left = xScale.map(category);
    const width = xScale.bandwidth();
    const valueY = yScale.map(readField(datum, y.field));
    const targetY = target ? yScale.map(readField(datum, target.field)) : undefined;
    elements.push({ kind: "rect", id: `${mark.id}-range-${index}`, x: left, y: bounds.top, width, height: bounds.height, fill: "hsl(var(--muted))", opacity: 0.18, radius: 6, className: "chart-bullet-range" });
    elements.push({ kind: "rect", id: `${mark.id}-value-${index}`, x: left + width * 0.24, y: Math.min(valueY, zero), width: width * 0.52, height: Math.max(3, Math.abs(zero - valueY)), fill: mark.style?.fill ?? CHART_PALETTE[index % CHART_PALETTE.length], radius: 4, label: `${toLabel(category)}: ${formatValue(readField(datum, y.field), y.format)}`, className: "chart-mark chart-mark-bullet" });
    if (targetY !== undefined) {
      elements.push({ kind: "line", id: `${mark.id}-target-${index}`, x1: left + width * 0.16, y1: targetY, x2: left + width * 0.84, y2: targetY, stroke: "hsl(var(--foreground))", strokeWidth: 2, label: `${toLabel(category)} target`, className: "chart-mark chart-mark-bullet-target" });
    }
  });
}

export function addGridAndAxes(elements: SceneElement[], spec: ChartSpec, bounds: PlotBounds, x: FieldEncoding, y: FieldEncoding, xScale: CartesianXScale, yTicks: number[]) {
  elements.push({ kind: "line", id: `${spec.id}-axis-x`, x1: bounds.left, y1: bounds.bottom, x2: bounds.right, y2: bounds.bottom, className: "chart-axis-line" });
  elements.push({ kind: "line", id: `${spec.id}-axis-y`, x1: bounds.left, y1: bounds.top, x2: bounds.left, y2: bounds.bottom, className: "chart-axis-line" });

  xScale.ticks().forEach((tick, index) => {
    elements.push({ kind: "text", id: `${spec.id}-x-tick-${index}`, x: xScale.map(tick), y: bounds.bottom + 22, text: formatValue(tick, x.format), anchor: "middle", className: "chart-axis-label" });
  });

  const yScale = createLinearScale([yTicks[0] ?? 0, yTicks[yTicks.length - 1] ?? 1], [bounds.bottom, bounds.top]);
  yTicks.forEach((tick, index) => {
    const ty = yScale.map(tick);
    elements.push({ kind: "line", id: `${spec.id}-grid-y-${index}`, x1: bounds.left, y1: ty, x2: bounds.right, y2: ty, className: "chart-grid-line" });
    elements.push({ kind: "text", id: `${spec.id}-y-tick-${index}`, x: bounds.left - 10, y: ty + 4, text: formatValue(tick, y.format), anchor: "end", className: "chart-axis-label" });
  });

  elements.push({ kind: "text", id: `${spec.id}-x-title`, x: (bounds.left + bounds.right) / 2, y: spec.height - 8, text: x.label ?? x.field, anchor: "middle", className: "chart-axis-title" });
  elements.push({ kind: "text", id: `${spec.id}-y-title`, x: 12, y: bounds.top - 10, text: y.label ?? y.field, anchor: "start", className: "chart-axis-title" });
}

function isBandScale(scale: CartesianXScale): scale is BandScale {
  return scale.kind === "band";
}

function heatmapSizeRatio(value: ChartPrimitive, domain: [number, number]) {
  const span = domain[1] - domain[0] || 1;
  return 0.35 + clamp((toNumber(value) - domain[0]) / span, 0, 1) * 0.65;
}

function adviceMetadata(datum: Datum) {
  const ruleId = toLabel(readField(datum, "ruleId"));
  if (!ruleId) {
    return {};
  }

  return {
    adviceRuleId: ruleId,
    adviceReason: toLabel(readField(datum, "reason")),
    adviceConfidence: toLabel(readField(datum, "confidence")),
  };
}

function barLayout(datum: Datum, x: FieldEncoding, xScale: BandScale) {
  const bandStart = xScale.map(readField(datum, x.field));
  const bandWidth = xScale.bandwidth();
  const dodgeCount = Math.max(1, toNumber(readField(datum, DODGE_COUNT_FIELD)));
  const dodgeIndex = Math.max(0, toNumber(readField(datum, DODGE_INDEX_FIELD)));

  if (dodgeCount <= 1) {
    return { x: bandStart, width: bandWidth };
  }

  const innerPadding = bandWidth * 0.08;
  const slotWidth = (bandWidth - innerPadding) / dodgeCount;
  return {
    x: bandStart + innerPadding / 2 + slotWidth * Math.min(dodgeIndex, dodgeCount - 1),
    width: Math.max(2, slotWidth * 0.9),
  };
}
