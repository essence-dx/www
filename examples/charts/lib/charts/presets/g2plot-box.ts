import type { ChartSpec, ChartTask, Datum, FieldEncoding, ValueFormat } from "../spec";

export type G2PlotBoxPreset = {
  id: string;
  title: string;
  description: string;
  task: ChartTask;
  data: Datum[];
  xField: string;
  lowField: string;
  q1Field: string;
  medianField: string;
  q3Field: string;
  highField: string;
  xLabel?: string;
  yLabel?: string;
  valueFormat?: ValueFormat;
  width?: number;
  height?: number;
};

const defaultSize = { width: 640, height: 380 } as const;

export function g2plotBox(config: G2PlotBoxPreset): ChartSpec {
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
      id: `${config.id}-boxplot`,
      type: "boxplot",
      encoding: {
        x: field(config.xField, "ordinal", config.xLabel),
        y: field(config.medianField, "quantitative", config.yLabel, config.valueFormat),
        low: field(config.lowField, "quantitative", undefined, config.valueFormat),
        q1: field(config.q1Field, "quantitative", undefined, config.valueFormat),
        median: field(config.medianField, "quantitative", undefined, config.valueFormat),
        q3: field(config.q3Field, "quantitative", undefined, config.valueFormat),
        high: field(config.highField, "quantitative", undefined, config.valueFormat),
      },
    }],
    axes: [{ channel: "x" }, { channel: "y", title: config.yLabel, grid: true }],
  };
}

function field(fieldName: string, type: FieldEncoding["type"], label?: string, format?: ValueFormat): FieldEncoding {
  return { field: fieldName, type, label, format };
}
