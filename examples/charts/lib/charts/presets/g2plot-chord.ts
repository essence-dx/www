import type { ChartFamily, ChartSpec, ChartTask, ChordNodeSort, Datum, FieldEncoding, ValueFormat } from "../spec";

type G2PlotChordPresetBase = {
  id: string;
  title: string;
  description: string;
  task: ChartTask;
  data: Datum[];
  family?: ChartFamily;
  width?: number;
  height?: number;
};

export type G2PlotChordPreset = G2PlotChordPresetBase & {
  sourceField: string;
  targetField: string;
  weightField: string;
  sourceLabel?: string;
  targetLabel?: string;
  weightLabel?: string;
  valueFormat?: ValueFormat;
  nodePaddingRatio?: number;
  nodeWidthRatio?: number;
  nodeSort?: ChordNodeSort;
};

const defaultSize = { width: 640, height: 380 } as const;

export function g2plotChord(config: G2PlotChordPreset): ChartSpec {
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
      id: `${config.id}-chord`,
      type: "chord",
      encoding: {
        source: field(config.sourceField, "nominal", config.sourceLabel),
        target: field(config.targetField, "nominal", config.targetLabel),
        size: field(config.weightField, "quantitative", config.weightLabel, config.valueFormat),
        color: field(config.sourceField, "nominal", config.sourceLabel),
      },
      chord: {
        nodePaddingRatio: config.nodePaddingRatio ?? 0.024,
        nodeWidthRatio: config.nodeWidthRatio ?? 0.08,
        nodeSort: config.nodeSort ?? "weight-desc",
      },
    }],
    axes: [],
    coordinate: { type: "polar" },
    legend: { channel: "color", title: config.sourceLabel ?? labelFor(config.sourceField) },
  };
}

function field(fieldName: string, type: FieldEncoding["type"], label?: string, format?: ValueFormat): FieldEncoding {
  return { field: fieldName, type, label, format };
}

function labelFor(fieldName: string): string {
  return fieldName.replace(/([a-z])([A-Z])/g, "$1 $2").replace(/[-_]/g, " ").replace(/\b\w/g, (value) => value.toUpperCase());
}
