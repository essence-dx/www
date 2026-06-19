import type { ChartSpec, ChartTask, Datum, FieldEncoding, ValueFormat } from "../spec";

type G2PlotRadialBase = {
  id: string;
  title: string;
  description: string;
  task: ChartTask;
  data: Datum[];
  width?: number;
  height?: number;
};

export type G2PlotPiePreset = G2PlotRadialBase & {
  angleField: string;
  colorField: string;
  valueFormat?: ValueFormat;
};

export type G2PlotRadarPreset = G2PlotRadialBase & {
  xField: string;
  yField: string;
  seriesField?: string;
  yFormat?: ValueFormat;
};

export type G2PlotRosePreset = G2PlotRadialBase & {
  xField: string;
  yField: string;
  colorField?: string;
  yFormat?: ValueFormat;
  radius?: number;
  innerRadius?: number;
};

const defaultSize = { width: 640, height: 380 } as const;

export function g2plotPie(config: G2PlotPiePreset): ChartSpec {
  return {
    id: config.id,
    title: config.title,
    description: config.description,
    task: config.task,
    family: "G2Plot",
    width: config.width ?? defaultSize.width,
    height: config.height ?? defaultSize.height,
    data: config.data,
    marks: [{
      id: `${config.id}-pie`,
      type: "pie",
      encoding: {
        label: field(config.colorField, "nominal"),
        color: field(config.colorField, "nominal"),
        theta: field(config.angleField, "quantitative", config.valueFormat),
      },
    }],
    legend: { channel: "color", title: labelFor(config.colorField) },
  };
}

export function g2plotRadar(config: G2PlotRadarPreset): ChartSpec {
  return {
    id: config.id,
    title: config.title,
    description: config.description,
    task: config.task,
    family: "G2Plot",
    width: config.width ?? defaultSize.width,
    height: config.height ?? defaultSize.height,
    data: config.data,
    marks: [{
      id: `${config.id}-radar`,
      type: "radar",
      encoding: {
        x: field(config.xField, "ordinal"),
        y: field(config.yField, "quantitative", config.yFormat),
        series: config.seriesField ? field(config.seriesField, "nominal") : undefined,
      },
    }],
    legend: config.seriesField ? { channel: "series", title: labelFor(config.seriesField) } : undefined,
  };
}

export function g2plotRose(config: G2PlotRosePreset): ChartSpec {
  const colorField = config.colorField ?? config.xField;

  return {
    id: config.id,
    title: config.title,
    description: config.description,
    task: config.task,
    family: "G2Plot",
    width: config.width ?? defaultSize.width,
    height: config.height ?? defaultSize.height,
    data: config.data,
    coordinate: { type: "polar", radius: config.radius ?? 0.92, innerRadius: config.innerRadius ?? 0.18 },
    marks: [{
      id: `${config.id}-rose`,
      type: "pie",
      encoding: {
        label: field(config.xField, "nominal"),
        color: field(colorField, "nominal"),
        theta: field(config.yField, "quantitative", config.yFormat),
      },
    }],
    legend: { channel: "color", title: labelFor(colorField) },
  };
}

function field(fieldName: string, type: FieldEncoding["type"], format?: ValueFormat): FieldEncoding {
  return { field: fieldName, type, format };
}

function labelFor(fieldName: string): string {
  return fieldName.replace(/([a-z])([A-Z])/g, "$1 $2").replace(/[-_]/g, " ").replace(/\b\w/g, (value) => value.toUpperCase());
}
