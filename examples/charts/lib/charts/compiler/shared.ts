import { readField, toLabel } from "../format";
import type { ChartPadding, ChartSpec, Datum, FieldEncoding } from "../spec";
import { DEFAULT_PADDING } from "../spec";
import type { ChartScene, LegendItem, SceneElement } from "../scene";

export type PlotBounds = {
  left: number;
  top: number;
  right: number;
  bottom: number;
  width: number;
  height: number;
};

export function groupData(data: Datum[], field?: FieldEncoding): Array<{ key: string; rows: Datum[] }> {
  if (!field) {
    return [{ key: "Series", rows: data }];
  }

  const groups = new Map<string, Datum[]>();
  data.forEach((datum) => {
    const key = toLabel(readField(datum, field.field));
    groups.set(key, [...(groups.get(key) ?? []), datum]);
  });

  return Array.from(groups, ([key, rows]) => ({ key, rows }));
}

export function makeLegend(domain: string[], color: (value: string) => string): LegendItem[] {
  return domain.map((label) => ({ label, color: color(label) }));
}

export function scene(spec: ChartSpec, elements: SceneElement[], legend: LegendItem[]): ChartScene {
  const metadata = coordinateSceneMetadata(spec);

  return {
    id: spec.id,
    title: spec.title,
    description: spec.description,
    width: spec.width,
    height: spec.height,
    elements: elements.map((element) => ({ ...metadata, ...element })),
    legend,
    summary: `${spec.title}: ${spec.data.length} rows rendered by DX Charts.`,
  };
}

export function emptyScene(spec: ChartSpec): ChartScene {
  return scene(spec, [], []);
}

export function sceneId(...parts: Array<string | number | undefined>): string {
  const id = parts
    .map((part) => String(part ?? ""))
    .join("-")
    .toLowerCase()
    .replace(/[^a-z0-9_-]+/g, "-")
    .replace(/^-+|-+$/g, "");

  return id || "scene";
}

export function withPadding(padding?: Partial<ChartPadding>): ChartPadding {
  return { ...DEFAULT_PADDING, ...padding };
}

export function plotBounds(spec: ChartSpec, padding: ChartPadding): PlotBounds {
  const left = padding.left;
  const top = padding.top;
  const right = spec.width - padding.right;
  const bottom = spec.height - padding.bottom;

  return {
    left,
    top,
    right,
    bottom,
    width: right - left,
    height: bottom - top,
  };
}

export function coordinateSceneMetadata(spec: ChartSpec): Pick<SceneElement, "coordinateType" | "coordinateOptions"> {
  if (!spec.coordinate) return {};
  const options = [
    spec.coordinate.innerRadius === undefined ? undefined : `innerRadius=${spec.coordinate.innerRadius}`,
    spec.coordinate.startAngle === undefined ? undefined : `startAngle=${spec.coordinate.startAngle}`,
    spec.coordinate.endAngle === undefined ? undefined : `endAngle=${spec.coordinate.endAngle}`,
  ].filter(Boolean);

  return {
    coordinateType: spec.coordinate.type,
    coordinateOptions: options.length > 0 ? options.join(";") : undefined,
  };
}
