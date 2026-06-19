export interface Point {
  x: number;
  y: number;
}

export function linePath(points: Point[]): string {
  return points
    .filter((point) => Number.isFinite(point.x) && Number.isFinite(point.y))
    .map((point, index) => `${index === 0 ? "M" : "L"} ${round(point.x)} ${round(point.y)}`)
    .join(" ");
}

export function areaPath(points: Point[], baseline: number): string {
  if (points.length === 0) {
    return "";
  }

  const top = linePath(points);
  const last = points[points.length - 1];
  const first = points[0];
  return `${top} L ${round(last.x)} ${round(baseline)} L ${round(first.x)} ${round(baseline)} Z`;
}

export function polarPoint(cx: number, cy: number, radius: number, angle: number): Point {
  return {
    x: cx + Math.cos(angle) * radius,
    y: cy + Math.sin(angle) * radius,
  };
}

export function arcPath(cx: number, cy: number, radius: number, startAngle: number, endAngle: number): string {
  const start = polarPoint(cx, cy, radius, startAngle);
  const end = polarPoint(cx, cy, radius, endAngle);
  const largeArc = endAngle - startAngle > Math.PI ? 1 : 0;
  return `M ${round(cx)} ${round(cy)} L ${round(start.x)} ${round(start.y)} A ${round(radius)} ${round(radius)} 0 ${largeArc} 1 ${round(end.x)} ${round(end.y)} Z`;
}

const FULL_CIRCLE_EPSILON = 0.001;
const TAU = Math.PI * 2;

export function annularArcPath(cx: number, cy: number, innerRadius: number, outerRadius: number, startAngle: number, endAngle: number): string {
  const sweep = endAngle - startAngle;
  if (sweep <= 0) return "";

  if (sweep >= TAU - FULL_CIRCLE_EPSILON) {
    const midAngle = startAngle + TAU / 2;
    const outerStart = polarPoint(cx, cy, outerRadius, startAngle);
    const innerStart = polarPoint(cx, cy, innerRadius, startAngle);

    return [
      `M ${round(outerStart.x)} ${round(outerStart.y)}`,
      annularOuterArc(cx, cy, outerRadius, startAngle, midAngle),
      annularOuterArc(cx, cy, outerRadius, midAngle, startAngle + TAU),
      `L ${round(innerStart.x)} ${round(innerStart.y)}`,
      annularInnerArc(cx, cy, innerRadius, startAngle + TAU, midAngle),
      annularInnerArc(cx, cy, innerRadius, midAngle, startAngle),
      "Z",
    ].join(" ");
  }

  const outerStart = polarPoint(cx, cy, outerRadius, startAngle);
  const outerEnd = polarPoint(cx, cy, outerRadius, endAngle);
  const innerEnd = polarPoint(cx, cy, innerRadius, endAngle);
  const innerStart = polarPoint(cx, cy, innerRadius, startAngle);
  const largeArc = sweep > Math.PI ? 1 : 0;

  return [
    `M ${round(outerStart.x)} ${round(outerStart.y)}`,
    `A ${round(outerRadius)} ${round(outerRadius)} 0 ${largeArc} 1 ${round(outerEnd.x)} ${round(outerEnd.y)}`,
    `L ${round(innerEnd.x)} ${round(innerEnd.y)}`,
    `A ${round(innerRadius)} ${round(innerRadius)} 0 ${largeArc} 0 ${round(innerStart.x)} ${round(innerStart.y)}`,
    "Z",
  ].join(" ");
}

function annularOuterArc(cx: number, cy: number, radius: number, startAngle: number, endAngle: number): string {
  const end = polarPoint(cx, cy, radius, endAngle);
  const largeArc = endAngle - startAngle > Math.PI ? 1 : 0;
  return `A ${round(radius)} ${round(radius)} 0 ${largeArc} 1 ${round(end.x)} ${round(end.y)}`;
}

function annularInnerArc(cx: number, cy: number, radius: number, startAngle: number, endAngle: number): string {
  const end = polarPoint(cx, cy, radius, endAngle);
  const largeArc = startAngle - endAngle > Math.PI ? 1 : 0;
  return `A ${round(radius)} ${round(radius)} 0 ${largeArc} 0 ${round(end.x)} ${round(end.y)}`;
}

export function clamp(value: number, min: number, max: number): number {
  return Math.max(min, Math.min(max, value));
}

export function round(value: number): number {
  return Math.round(value * 100) / 100;
}
