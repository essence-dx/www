import { formatValue, readField, toLabel, toNumber } from "../format";
import { CHART_PALETTE, createBandScale, createLinearScale } from "../scales";
import type { ChartPrimitive, ChartSpec, Datum, FieldEncoding } from "../spec";
import type { SceneElement } from "../scene";
import { applyMarkTransforms } from "../transforms";
import { addGridAndAxes } from "./cartesian-marks";
import { emptyScene, plotBounds, scene, sceneId, withPadding } from "./shared";

type WaterfallStage = {
  label: string;
  value: ChartPrimitive;
  start: number;
  end: number;
  cumulative: number;
  total: boolean;
};

export function compileWaterfall(spec: ChartSpec) {
  const mark = spec.marks[0];
  const x = mark.encoding.x ?? mark.encoding.label;
  const y = mark.encoding.y ?? mark.encoding.size;
  if (!x || !y) return emptyScene(spec);

  const rows = applyMarkTransforms(spec.data, mark);
  if (rows.length === 0) return emptyScene(spec);

  const stages = waterfallStages(rows, x, y, mark.waterfall?.totalField);
  const bounds = plotBounds(spec, withPadding(spec.padding ?? { left: 54, right: 28, top: 32, bottom: 54 }));
  const xScale = createBandScale(stages.map((stage) => stage.label), [bounds.left, bounds.right]);
  const yValues = stages.flatMap((stage) => [stage.start, stage.end, stage.cumulative, 0]);
  const yScale = createLinearScale([Math.min(...yValues), Math.max(...yValues)], [bounds.bottom, bounds.top]);
  const elements: SceneElement[] = [];

  addGridAndAxes(elements, spec, bounds, x, y, xScale, yScale.ticks(5));

  stages.forEach((stage, index) => {
    const left = xScale.map(stage.label);
    const width = xScale.bandwidth();
    const startY = yScale.map(stage.start);
    const endY = yScale.map(stage.end);
    const negative = stage.end < stage.start;
    const total = stage.total;
    const className = [
      "chart-mark",
      "chart-mark-waterfall",
      negative ? "chart-mark-waterfall-negative" : "chart-mark-waterfall-positive",
      total ? "chart-mark-waterfall-total" : undefined,
    ].filter(Boolean).join(" ");

    elements.push({
      kind: "rect",
      id: sceneId(mark.id, "bar", index),
      x: left,
      y: Math.min(startY, endY),
      width,
      height: Math.max(3, Math.abs(startY - endY)),
      fill: fillForStage(index, negative, total),
      radius: mark.style?.radius ?? 5,
      opacity: mark.style?.opacity ?? 0.92,
      label: `${stage.label}: ${formatValue(stage.value, y.format)}`,
      className,
    });
    elements.push({
      kind: "text",
      id: sceneId(mark.id, "label", index),
      x: left + width / 2,
      y: Math.min(startY, endY) - 8,
      text: formatValue(stage.value, y.format),
      anchor: "middle",
      className: "chart-axis-label",
    });

    const next = stages[index + 1];
    if (next) {
      const connectorY = yScale.map(stage.end);
      elements.push({
        kind: "line",
        id: sceneId(mark.id, "connector", index),
        x1: left + width,
        y1: connectorY,
        x2: xScale.map(next.label),
        y2: connectorY,
        stroke: "hsl(var(--chart-muted-line))",
        strokeWidth: 1.5,
        label: `${stage.label} cumulative ${formatValue(stage.end, y.format)}`,
        className: "chart-waterfall-connector",
      });
    }
  });

  return scene(spec, elements, []);
}

export function waterfallStages(rows: Datum[], x: FieldEncoding, y: FieldEncoding, totalField?: string): WaterfallStage[] {
  let cumulative = 0;

  return rows.map((datum) => {
    const value = readField(datum, y.field);
    const delta = toNumber(value);
    const total = totalField ? isWaterfallTotalValue(readField(datum, totalField)) : isTotalStageLabel(datum, x.field);
    const start = total ? 0 : cumulative;
    const end = total ? delta : cumulative + delta;
    cumulative = end;

    return {
      label: toLabel(readField(datum, x.field)),
      value,
      start,
      end,
      cumulative,
      total,
    };
  });
}

export function isWaterfallTotalValue(value: ChartPrimitive): boolean {
  if (value === null) return false;
  if (typeof value === "boolean") return value;
  if (typeof value === "number") return Number.isFinite(value) && value !== 0;
  if (value instanceof Date) return false;

  const normalized = value.trim().toLowerCase();
  if (normalized === "") return false;
  if (["true", "1", "yes", "y", "total", "grand total"].includes(normalized)) return true;
  if (["false", "0", "no", "n", "none", "null"].includes(normalized)) return false;
  return false;
}

function isTotalStageLabel(datum: Datum, labelField: string): boolean {
  const labels = [readField(datum, labelField), readField(datum, "stage")];
  return labels.some((value) => toLabel(value).toLowerCase().includes("total"));
}

function fillForStage(index: number, negative: boolean, total: boolean): string {
  if (total) return "hsl(var(--chart-blue))";
  if (negative) return "hsl(var(--chart-warning))";
  return CHART_PALETTE[index % CHART_PALETTE.length];
}
