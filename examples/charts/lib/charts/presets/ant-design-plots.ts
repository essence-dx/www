import type { ChartSpec, ChartTask, Datum, FieldEncoding, ValueFormat } from "../spec";

export type AntDesignDualAxesPreset = {
  id: string;
  title: string;
  description: string;
  data: Datum[];
  xField: string;
  columnField: string;
  lineField: string;
  columnLabel?: string;
  lineLabel?: string;
  seriesField?: string;
  task?: ChartTask;
  valueFormat?: ValueFormat;
  width?: number;
  height?: number;
};

export type AntDesignHeatmapPreset = {
  id: string;
  title: string;
  description: string;
  data: Datum[];
  xField: string;
  yField: string;
  colorField: string;
  sizeField?: string;
  task?: ChartTask;
  valueFormat?: ValueFormat;
  width?: number;
  height?: number;
};

export type AntDesignBulletPreset = {
  id: string;
  title: string;
  description: string;
  data: Datum[];
  xField: string;
  valueField: string;
  targetField?: string;
  task?: ChartTask;
  valueFormat?: ValueFormat;
  width?: number;
  height?: number;
};

export type AntDesignGaugePreset = {
  id: string;
  title: string;
  description: string;
  data: Datum[];
  valueField: string;
  targetField?: string;
  labelField?: string;
  task?: ChartTask;
  valueFormat?: ValueFormat;
  width?: number;
  height?: number;
};

export function antDesignDualAxes(config: AntDesignDualAxesPreset): ChartSpec {
  const x: FieldEncoding = { field: config.xField, type: "ordinal" };
  const columnY: FieldEncoding = { field: config.columnField, type: "quantitative", label: config.columnLabel, format: config.valueFormat };
  const lineY: FieldEncoding = { field: config.lineField, type: "quantitative", label: config.lineLabel, format: config.valueFormat };

  return {
    id: config.id,
    title: config.title,
    description: config.description,
    task: config.task ?? "comparison",
    family: "AntDesignPlots",
    width: config.width ?? 640,
    height: config.height ?? 380,
    data: config.data,
    marks: [
      {
        id: `${config.id}-column`,
        type: "bar",
        encoding: { x, y: columnY, ...(config.seriesField ? { color: { field: config.seriesField, type: "nominal" as const } } : {}) },
      },
      {
        id: `${config.id}-line`,
        type: "line",
        encoding: { x, y: lineY, ...(config.seriesField ? { series: { field: config.seriesField, type: "nominal" as const } } : {}) },
      },
      {
        id: `${config.id}-point`,
        type: "point",
        encoding: { x, y: lineY, ...(config.seriesField ? { series: { field: config.seriesField, type: "nominal" as const } } : {}) },
      },
    ],
    axes: [{ channel: "x" }, { channel: "y", grid: true, title: config.columnLabel ?? config.lineLabel }],
    ...(config.seriesField ? { legend: { channel: "series" as const, title: "Series" } } : {}),
  };
}

export function antDesignHeatmap(config: AntDesignHeatmapPreset): ChartSpec {
  return {
    id: config.id,
    title: config.title,
    description: config.description,
    task: config.task ?? "distribution",
    family: "AntDesignPlots",
    width: config.width ?? 640,
    height: config.height ?? 380,
    data: config.data,
    marks: [{
      id: `${config.id}-heatmap`,
      type: "heatmap",
      encoding: {
        x: field(config.xField, "ordinal"),
        y: field(config.yField, "ordinal"),
        color: field(config.colorField, "quantitative", config.valueFormat),
        size: config.sizeField ? field(config.sizeField, "quantitative", config.valueFormat) : undefined,
      },
    }],
    axes: [],
    legend: { channel: "color", title: labelFor(config.colorField) },
  };
}

export function antDesignBullet(config: AntDesignBulletPreset): ChartSpec {
  return {
    id: config.id,
    title: config.title,
    description: config.description,
    task: config.task ?? "comparison",
    family: "AntDesignPlots",
    width: config.width ?? 640,
    height: config.height ?? 380,
    data: config.data,
    marks: [{
      id: `${config.id}-bullet`,
      type: "bullet",
      encoding: {
        x: field(config.xField, "ordinal"),
        y: field(config.valueField, "quantitative", config.valueFormat),
        target: config.targetField ? field(config.targetField, "quantitative", config.valueFormat) : undefined,
      },
    }],
    axes: [{ channel: "x" }, { channel: "y", grid: true, title: labelFor(config.valueField) }],
  };
}

export function antDesignGauge(config: AntDesignGaugePreset): ChartSpec {
  return {
    id: config.id,
    title: config.title,
    description: config.description,
    task: config.task ?? "proportion",
    family: "AntDesignPlots",
    width: config.width ?? 360,
    height: config.height ?? 240,
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

function field(fieldName: string, type: FieldEncoding["type"], format?: ValueFormat): FieldEncoding {
  return { field: fieldName, type, format };
}

function labelFor(fieldName: string): string {
  return fieldName.replace(/([a-z])([A-Z])/g, "$1 $2").replace(/[-_]/g, " ").replace(/\b\w/g, (value) => value.toUpperCase());
}
