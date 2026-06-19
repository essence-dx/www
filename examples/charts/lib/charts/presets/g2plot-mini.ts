import type { ChartSpec, ChartTask, Datum, FieldEncoding, ValueFormat } from "../spec";

type G2PlotMiniBase = {
  id: string;
  title: string;
  description: string;
  task: ChartTask;
  data: Datum[];
  width?: number;
  height?: number;
};

export type G2PlotTinyPreset = G2PlotMiniBase & {
  preset: "tiny-line" | "tiny-area" | "tiny-column";
  xField: string;
  yField: string;
  seriesField?: string;
  yFormat?: ValueFormat;
};

export type G2PlotRingProgressPreset = G2PlotMiniBase & {
  valueField: string;
  targetField?: string;
  labelField?: string;
  valueFormat?: ValueFormat;
};

export type G2PlotGaugePreset = G2PlotMiniBase & {
  valueField: string;
  targetField?: string;
  labelField?: string;
  valueFormat?: ValueFormat;
};

export type G2PlotProgressPreset = G2PlotMiniBase & {
  valueField: string;
  targetField?: string;
  labelField?: string;
  valueFormat?: ValueFormat;
};

const tinySize = { width: 320, height: 160 } as const;
const progressSize = { width: 360, height: 240 } as const;

export function g2plotTiny(config: G2PlotTinyPreset): ChartSpec {
  return {
    id: config.id,
    title: config.title,
    description: config.description,
    task: config.task,
    family: "G2Plot",
    width: config.width ?? tinySize.width,
    height: config.height ?? tinySize.height,
    data: config.data,
    marks: [{
      id: `${config.id}-${config.preset}`,
      type: tinyMark(config.preset),
      encoding: {
        x: field(config.xField, "ordinal"),
        y: field(config.yField, "quantitative", config.yFormat),
        series: config.seriesField ? field(config.seriesField, "nominal") : undefined,
      },
      style: config.preset === "tiny-area" ? { opacity: 0.28 } : undefined,
    }],
    axes: [],
    padding: { left: 10, right: 10, top: 10, bottom: 10 },
    legend: config.seriesField ? { channel: "series", title: labelFor(config.seriesField) } : undefined,
  };
}

export function g2plotRingProgress(config: G2PlotRingProgressPreset): ChartSpec {
  return {
    id: config.id,
    title: config.title,
    description: config.description,
    task: config.task,
    family: "G2Plot",
    width: config.width ?? progressSize.width,
    height: config.height ?? progressSize.height,
    data: config.data,
    coordinate: { type: "theta", innerRadius: 0.64 },
    marks: [{
      id: `${config.id}-ring-progress`,
      type: "gauge",
      encoding: {
        theta: field(config.valueField, "quantitative", config.valueFormat),
        target: config.targetField ? field(config.targetField, "quantitative", config.valueFormat) : undefined,
        label: config.labelField ? field(config.labelField, "nominal") : undefined,
      },
    }],
  };
}

export function g2plotGauge(config: G2PlotGaugePreset): ChartSpec {
  return {
    id: config.id,
    title: config.title,
    description: config.description,
    task: config.task,
    family: "G2Plot",
    width: config.width ?? progressSize.width,
    height: config.height ?? progressSize.height,
    data: config.data,
    coordinate: { type: "theta", innerRadius: 0.58 },
    marks: [{
      id: `${config.id}-gauge`,
      type: "gauge",
      encoding: {
        theta: field(config.valueField, "quantitative", config.valueFormat),
        target: config.targetField ? field(config.targetField, "quantitative", config.valueFormat) : undefined,
        label: config.labelField ? field(config.labelField, "nominal") : undefined,
      },
    }],
  };
}

export function g2plotProgress(config: G2PlotProgressPreset): ChartSpec {
  return {
    id: config.id,
    title: config.title,
    description: config.description,
    task: config.task,
    family: "G2Plot",
    width: config.width ?? progressSize.width,
    height: config.height ?? progressSize.height,
    data: config.data,
    coordinate: { type: "theta", innerRadius: 0.78 },
    marks: [{
      id: `${config.id}-progress`,
      type: "gauge",
      encoding: {
        theta: field(config.valueField, "quantitative", config.valueFormat),
        target: config.targetField ? field(config.targetField, "quantitative", config.valueFormat) : undefined,
        label: config.labelField ? field(config.labelField, "nominal") : undefined,
      },
    }],
  };
}

function tinyMark(preset: G2PlotTinyPreset["preset"]): ChartSpec["marks"][number]["type"] {
  if (preset === "tiny-column") return "bar";
  if (preset === "tiny-area") return "area";
  return "line";
}

function field(fieldName: string, type: FieldEncoding["type"], format?: ValueFormat): FieldEncoding {
  return { field: fieldName, type, format };
}

function labelFor(fieldName: string): string {
  return fieldName.replace(/([a-z])([A-Z])/g, "$1 $2").replace(/[-_]/g, " ").replace(/\b\w/g, (value) => value.toUpperCase());
}
