import type { ChartSpec, ChartTask, Datum, FieldEncoding, ValueFormat } from "../spec";

export type G2PlotWaterfallPreset = {
  id: string;
  title: string;
  description: string;
  task: ChartTask;
  data: Datum[];
  xField: string;
  yField: string;
  totalField?: string;
  xLabel?: string;
  yLabel?: string;
  valueFormat?: ValueFormat;
  width?: number;
  height?: number;
};

const defaultSize = { width: 640, height: 380 } as const;

export function g2plotWaterfall(config: G2PlotWaterfallPreset): ChartSpec {
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
      id: `${config.id}-waterfall`,
      type: "waterfall",
      encoding: {
        x: field(config.xField, "ordinal", config.xLabel),
        y: field(config.yField, "quantitative", config.yLabel, config.valueFormat),
      },
      waterfall: config.totalField ? { totalField: config.totalField } : undefined,
    }],
    axes: [{ channel: "x" }, { channel: "y", grid: true }],
  };
}

function field(fieldName: string, type: FieldEncoding["type"], label?: string, format?: ValueFormat): FieldEncoding {
  return { field: fieldName, type, label, format };
}
