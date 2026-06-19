import type { ChartSpec, ChartTask, Datum, FieldEncoding, ValueFormat } from "../spec";

export type G2PlotFunnelPreset = {
  id: string;
  title: string;
  description: string;
  task: ChartTask;
  data: Datum[];
  labelField: string;
  valueField: string;
  valueFormat?: ValueFormat;
  width?: number;
  height?: number;
};

const defaultSize = { width: 640, height: 380 } as const;

export function g2plotFunnel(config: G2PlotFunnelPreset): ChartSpec {
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
      id: `${config.id}-funnel`,
      type: "funnel",
      encoding: {
        label: field(config.labelField, "nominal"),
        size: field(config.valueField, "quantitative", config.valueFormat),
      },
    }],
  };
}

function field(fieldName: string, type: FieldEncoding["type"], format?: ValueFormat): FieldEncoding {
  return { field: fieldName, type, format };
}
