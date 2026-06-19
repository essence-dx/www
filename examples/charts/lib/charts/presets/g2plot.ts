import type { ChartFamily, ChartSpec, ChartTask, Datum, FieldEncoding, MarkKind, TransformSpec, ValueFormat } from "../spec";

type FieldTypeHint = FieldEncoding["type"];

type G2PlotPresetBase = {
  id: string;
  title: string;
  description: string;
  task: ChartTask;
  data: Datum[];
  family?: ChartFamily;
  width?: number;
  height?: number;
};

export type G2PlotCartesianPreset = G2PlotPresetBase & {
  preset: "column" | "bar" | "line" | "area" | "scatter";
  xField: string;
  yField: string;
  xLabel?: string;
  yLabel?: string;
  xType?: FieldTypeHint;
  yType?: FieldTypeHint;
  valueFormat?: ValueFormat;
  seriesField?: string;
  colorField?: string;
  sizeField?: string;
  isStack?: boolean;
  isPercent?: boolean;
  isGroup?: boolean;
};

export type G2PlotHistogramPreset = G2PlotPresetBase & {
  preset: "histogram";
  binField: string;
  binNumber?: number;
  min?: number;
  xLabel?: string;
  yLabel?: string;
};

export type G2PlotFacetPreset = G2PlotPresetBase & {
  preset: "facet";
  field: string;
  columns?: number;
  label?: string;
  child: Omit<G2PlotCartesianPreset, "id" | "title" | "description" | "task" | "data" | "family" | "width" | "height">;
};

export type G2PlotHeatmapPreset = G2PlotPresetBase & {
  preset: "heatmap";
  xField: string;
  yField: string;
  colorField?: string;
  sizeField?: string;
  xLabel?: string;
  yLabel?: string;
  valueFormat?: ValueFormat;
};

export type G2PlotBulletPreset = G2PlotPresetBase & {
  xField: string;
  valueField: string;
  targetField?: string;
  xLabel?: string;
  valueLabel?: string;
  targetLabel?: string;
  valueFormat?: ValueFormat;
};

const defaultSize = { width: 640, height: 380 } as const;

export function g2plotCartesian(config: G2PlotCartesianPreset): ChartSpec {
  const colorField = config.colorField ?? config.seriesField;
  const transforms = cartesianTransforms(config);
  const mark: ChartSpec["marks"][number] = {
    id: markId(config.id, config.preset),
    type: cartesianMark(config.preset),
    encoding: {
      x: field(config.xField, config.xType ?? defaultXType(config.preset), config.xLabel),
      y: field(config.yField, config.yType ?? "quantitative", config.yLabel, config.valueFormat),
      color: colorField ? field(colorField, "nominal") : undefined,
      series: config.seriesField ? field(config.seriesField, "nominal") : undefined,
      size: config.sizeField ? field(config.sizeField, "quantitative") : undefined,
    },
    transforms: transforms.length > 0 ? transforms : undefined,
  };

  return {
    id: config.id,
    title: config.title,
    description: config.description,
    task: config.task,
    family: config.family ?? "G2Plot",
    width: config.width ?? defaultSize.width,
    height: config.height ?? defaultSize.height,
    data: config.data,
    marks: [mark],
    coordinate: config.preset === "bar" ? { type: "transpose" } : undefined,
    axes: [{ channel: "x" }, { channel: "y", grid: true }],
    legend: colorField || config.seriesField ? { channel: colorField ? "color" : "series", title: labelFor(colorField ?? config.seriesField) } : undefined,
  };
}

export function g2plotHistogram(config: G2PlotHistogramPreset): ChartSpec {
  const transforms: TransformSpec[] = [
    ...(config.min === undefined ? [] : [{ type: "filter" as const, field: config.binField, min: config.min }]),
    { type: "bin", field: config.binField, as: "bin", valueAs: "count", count: config.binNumber ?? 8 },
  ];

  return {
    id: config.id,
    title: config.title,
    description: config.description,
    task: config.task,
    family: config.family ?? "G2Plot",
    width: config.width ?? defaultSize.width,
    height: config.height ?? defaultSize.height,
    data: config.data,
    marks: [{
      id: markId(config.id, "histogram"),
      type: "bar",
      transforms,
      encoding: {
        x: field("bin", "ordinal", config.xLabel ?? labelFor(config.binField)),
        y: field("count", "quantitative", config.yLabel ?? "Count"),
      },
    }],
    axes: [{ channel: "x" }, { channel: "y", grid: true }],
  };
}

export function g2plotHeatmap(config: G2PlotHeatmapPreset): ChartSpec {
  return {
    id: config.id,
    title: config.title,
    description: config.description,
    task: config.task,
    family: config.family ?? "G2Plot",
    width: config.width ?? defaultSize.width,
    height: config.height ?? defaultSize.height,
    data: config.data,
    marks: [{
      id: markId(config.id, "heatmap"),
      type: "heatmap",
      encoding: {
        x: field(config.xField, "ordinal", config.xLabel),
        y: field(config.yField, "ordinal", config.yLabel),
        color: config.colorField ? field(config.colorField, "quantitative", undefined, config.valueFormat) : undefined,
        size: config.sizeField ? field(config.sizeField, "quantitative", undefined, config.valueFormat) : undefined,
      },
    }],
    axes: [{ channel: "x" }, { channel: "y" }],
    legend: config.colorField ? { channel: "color", title: labelFor(config.colorField) } : undefined,
  };
}

export function g2plotBullet(config: G2PlotBulletPreset): ChartSpec {
  return {
    id: config.id,
    title: config.title,
    description: config.description,
    task: config.task,
    family: config.family ?? "G2Plot",
    width: config.width ?? defaultSize.width,
    height: config.height ?? defaultSize.height,
    data: config.data,
    marks: [{
      id: markId(config.id, "bullet"),
      type: "bullet",
      encoding: {
        x: field(config.xField, "ordinal", config.xLabel),
        y: field(config.valueField, "quantitative", config.valueLabel, config.valueFormat),
        target: config.targetField ? field(config.targetField, "quantitative", config.targetLabel ?? "Target", config.valueFormat) : undefined,
      },
    }],
    axes: [{ channel: "x" }, { channel: "y", grid: true, title: config.valueLabel ?? labelFor(config.valueField) }],
  };
}

export function g2plotFacet(config: G2PlotFacetPreset): ChartSpec {
  const child = g2plotCartesian({
    id: config.id,
    title: config.title,
    description: config.description,
    task: config.task,
    family: config.family ?? "G2Plot",
    width: config.width ?? defaultSize.width,
    height: config.height ?? defaultSize.height,
    data: config.data,
    ...config.child,
  });

  return {
    ...child,
    composition: { type: "facet", field: config.field, columns: config.columns, label: config.label },
  };
}

function cartesianTransforms(config: G2PlotCartesianPreset): TransformSpec[] {
  if (!config.seriesField || (config.preset !== "bar" && config.preset !== "column")) return [];
  if (config.isPercent) return [{ type: "normalizeY", x: config.xField, y: config.yField, series: config.seriesField }];
  if (config.isStack) return [{ type: "stackY", x: config.xField, y: config.yField, series: config.seriesField }];
  if (config.isGroup) return [{ type: "dodgeX", x: config.xField, series: config.seriesField }];
  return [];
}

function cartesianMark(preset: G2PlotCartesianPreset["preset"]): MarkKind {
  if (preset === "line") return "line";
  if (preset === "area") return "area";
  if (preset === "scatter") return "point";
  return "bar";
}

function defaultXType(preset: G2PlotCartesianPreset["preset"]): FieldTypeHint {
  return preset === "scatter" ? "quantitative" : "ordinal";
}

function field(fieldName: string, type: FieldTypeHint, label?: string, format?: ValueFormat): FieldEncoding {
  return { field: fieldName, type, label, format };
}

function markId(id: string, preset: string): string {
  return `${id}-${preset}`;
}

function labelFor(fieldName: string | undefined): string {
  if (!fieldName) return "Series";
  return fieldName.replace(/([a-z])([A-Z])/g, "$1 $2").replace(/[-_]/g, " ").replace(/\b\w/g, (value) => value.toUpperCase());
}
