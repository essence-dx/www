import { annularArcPath, arcPath, clamp, polarPoint, round } from "../geometry";
import { extent, formatValue, readField, toLabel, toNumber, uniqueLabels } from "../format";
import { CHART_PALETTE, createLinearScale } from "../scales";
import type { ChartSpec } from "../spec";
import type { ChartScene, LegendItem, SceneElement } from "../scene";
import { applyMarkTransforms } from "../transforms";
import { emptyScene, groupData, makeLegend, scene, sceneId } from "./shared";

export function compilePie(spec: ChartSpec): ChartScene {
  const mark = spec.marks[0];
  const theta = mark.encoding.theta ?? mark.encoding.y;
  const label = mark.encoding.label ?? mark.encoding.color ?? mark.encoding.x;
  if (!theta || !label) return emptyScene(spec);

  const data = applyMarkTransforms(spec.data, mark);
  const total = data.reduce((sum, datum) => sum + Math.max(0, toNumber(readField(datum, theta.field))), 0) || 1;
  const radius = Math.min(spec.width, spec.height) * 0.32;
  const cx = spec.width / 2;
  const cy = spec.height / 2 + 10;
  const elements: SceneElement[] = [];
  const legend: LegendItem[] = [];
  let start = -Math.PI / 2;

  data.forEach((datum, index) => {
    const value = Math.max(0, toNumber(readField(datum, theta.field)));
    const angle = (value / total) * Math.PI * 2;
    const color = CHART_PALETTE[index % CHART_PALETTE.length];
    const name = toLabel(readField(datum, label.field));
    elements.push({
      kind: "path",
      id: sceneId(mark.id, "slice", index),
      d: arcPath(cx, cy, radius, start, start + angle),
      fill: color,
      stroke: "hsl(var(--chart-stroke))",
      strokeWidth: 1,
      label: `${name}: ${formatValue(value, theta.format)}`,
      className: "chart-mark chart-mark-pie",
    });
    legend.push({ label: name, color });
    start += angle;
  });

  return scene(spec, elements, legend);
}

export function compileRadar(spec: ChartSpec): ChartScene {
  const mark = spec.marks[0];
  const axis = mark.encoding.x;
  const value = mark.encoding.y;
  const series = mark.encoding.series ?? mark.encoding.color;
  if (!axis || !value) return emptyScene(spec);

  const data = applyMarkTransforms(spec.data, mark);
  const labels = uniqueLabels(data.map((datum) => readField(datum, axis.field)));
  const valueDomain = extent(data.map((datum) => readField(datum, value.field)));
  const scale = createLinearScale(valueDomain, [0, Math.min(spec.width, spec.height) * 0.28]);
  const cx = spec.width / 2;
  const cy = spec.height / 2 + 12;
  const groups = groupData(data, series);
  const elements: SceneElement[] = [];

  labels.forEach((labelText, index) => {
    const angle = -Math.PI / 2 + (index / labels.length) * Math.PI * 2;
    const end = polarPoint(cx, cy, scale.map(valueDomain[1]), angle);
    elements.push({ kind: "line", id: sceneId("radar-axis", index), x1: cx, y1: cy, x2: end.x, y2: end.y, className: "chart-axis-line" });
    elements.push({ kind: "text", id: sceneId("radar-label", index), x: end.x, y: end.y, text: labelText, anchor: "middle", className: "chart-axis-label" });
  });

  groups.forEach((group, groupIndex) => {
    const points = labels.map((labelText, index) => {
      const datum = group.rows.find((row) => toLabel(readField(row, axis.field)) === labelText) ?? group.rows[0];
      const angle = -Math.PI / 2 + (index / labels.length) * Math.PI * 2;
      return polarPoint(cx, cy, scale.map(readField(datum, value.field)), angle);
    });
    const color = CHART_PALETTE[groupIndex % CHART_PALETTE.length];
    elements.push({
      kind: "polygon",
      id: sceneId(mark.id, "radar", group.key),
      points: points.map((point) => `${round(point.x)},${round(point.y)}`).join(" "),
      fill: color,
      stroke: color,
      opacity: 0.28,
      label: `${group.key} radar area`,
      className: "chart-mark chart-mark-radar",
    });
  });

  return scene(spec, elements, makeLegend(groups.map((group) => group.key), (key) => CHART_PALETTE[Math.max(0, groups.findIndex((group) => group.key === key)) % CHART_PALETTE.length]));
}

export function compileGauge(spec: ChartSpec): ChartScene {
  const mark = spec.marks[0];
  const value = mark.encoding.theta ?? mark.encoding.y;
  const label = mark.encoding.label ?? mark.encoding.x;
  const target = mark.encoding.target;
  if (!value) return emptyScene(spec);

  const data = applyMarkTransforms(spec.data, mark);
  const datum = data[0];
  if (!datum) return emptyScene(spec);

  const raw = Math.max(0, toNumber(readField(datum, value.field)));
  const targetValue = target ? Math.max(0, toNumber(readField(datum, target.field))) : undefined;
  const domainMax = Math.max(1, raw, targetValue ?? 0);
  const ratio = clamp(raw / domainMax, 0, 1);
  const startAngle = Math.PI * 0.78;
  const endAngle = Math.PI * 2.22;
  const cx = spec.width / 2;
  const cy = spec.height * 0.63;
  const outerRadius = Math.min(spec.width, spec.height) * 0.32;
  const innerRadius = outerRadius * 0.64;
  const valueEnd = startAngle + (endAngle - startAngle) * ratio;
  const name = label ? toLabel(readField(datum, label.field)) : spec.title;
  const elements: SceneElement[] = [
    {
      kind: "path",
      id: sceneId(mark.id, "gauge-track"),
      d: annularArcPath(cx, cy, innerRadius, outerRadius, startAngle, endAngle),
      fill: "hsl(var(--muted))",
      opacity: 0.2,
      className: "chart-gauge-track",
    },
    {
      kind: "path",
      id: sceneId(mark.id, "gauge-value"),
      d: annularArcPath(cx, cy, innerRadius, outerRadius, startAngle, valueEnd),
      fill: mark.style?.fill ?? CHART_PALETTE[0],
      label: `${name}: ${formatValue(raw, value.format)}`,
      className: "chart-mark chart-mark-gauge",
    },
    {
      kind: "text",
      id: sceneId(mark.id, "gauge-label"),
      x: cx,
      y: cy - 4,
      text: formatValue(raw, value.format),
      anchor: "middle",
      className: "chart-gauge-value",
    },
    {
      kind: "text",
      id: sceneId(mark.id, "gauge-name"),
      x: cx,
      y: cy + 26,
      text: name,
      anchor: "middle",
      className: "chart-axis-label",
    },
  ];

  if (targetValue !== undefined) {
    const targetAngle = startAngle + (endAngle - startAngle) * clamp(targetValue / domainMax, 0, 1);
    const inner = polarPoint(cx, cy, innerRadius - 10, targetAngle);
    const outer = polarPoint(cx, cy, outerRadius + 10, targetAngle);
    elements.push({ kind: "line", id: sceneId(mark.id, "gauge-target"), x1: inner.x, y1: inner.y, x2: outer.x, y2: outer.y, stroke: "hsl(var(--foreground))", strokeWidth: 2, label: `Target ${formatValue(targetValue, value.format)}`, className: "chart-mark chart-mark-gauge-target" });
  }

  return scene(spec, elements, []);
}
