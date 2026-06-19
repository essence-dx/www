import type { ChartSpec, ChartTask, Datum, FieldEncoding, ValueFormat } from "../spec";

export type AntDesignColumnPreset = {
  id: string;
  title: string;
  description: string;
  data: Datum[];
  xField: string;
  yField: string;
  colorField?: string;
  xLabel?: string;
  yLabel?: string;
  width?: number;
  height?: number;
};

export type AntDesignProgressPreset = {
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

export function antDesignColumn(config: AntDesignColumnPreset): ChartSpec {
  const x: FieldEncoding = { field: config.xField, type: "ordinal", label: config.xLabel };
  const y: FieldEncoding = { field: config.yField, type: "quantitative", label: config.yLabel };

  return {
    id: config.id,
    title: config.title,
    description: config.description,
    task: "comparison",
    family: "AntDesignCharts",
    width: config.width ?? 640,
    height: config.height ?? 380,
    data: config.data,
    marks: [
      {
        id: `${config.id}-column`,
        type: "bar",
        encoding: {
          x,
          y,
          ...(config.colorField ? { color: { field: config.colorField, type: "nominal" as const } } : {}),
        },
      },
    ],
    ...(config.colorField ? { legend: { channel: "color" as const, title: "Family" } } : {}),
  };
}

export function antDesignProgress(config: AntDesignProgressPreset): ChartSpec {
  return {
    id: config.id,
    title: config.title,
    description: config.description,
    task: config.task ?? "proportion",
    family: "AntDesignCharts",
    width: config.width ?? 360,
    height: config.height ?? 240,
    data: config.data,
    coordinate: { type: "theta", innerRadius: 0.72 },
    marks: [
      {
        id: `${config.id}-progress`,
        type: "gauge",
        encoding: {
          theta: field(config.valueField, "quantitative", config.valueFormat),
          target: config.targetField ? field(config.targetField, "quantitative", config.valueFormat) : undefined,
          label: config.labelField ? field(config.labelField, "nominal") : undefined,
        },
      },
    ],
  };
}

function field(fieldName: string, type: FieldEncoding["type"], format?: ValueFormat): FieldEncoding {
  return { field: fieldName, type, format };
}
