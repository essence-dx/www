import type { ChartFamily, ChartSpec, ChartTask, Datum, FieldEncoding, ValueFormat } from "../spec";

type G2PlotHierarchyPresetBase = {
  id: string;
  title: string;
  description: string;
  task: ChartTask;
  data: Datum[];
  family?: ChartFamily;
  width?: number;
  height?: number;
};

export type G2PlotTreemapPreset = G2PlotHierarchyPresetBase & {
  labelField: string;
  valueField: string;
  colorField?: string;
  valueFormat?: ValueFormat;
};

export type G2PlotSunburstPreset = G2PlotHierarchyPresetBase & {
  labelField: string;
  parentField: string;
  levelField: string;
  valueField: string;
  colorField?: string;
  valueFormat?: ValueFormat;
};

export type G2PlotSankeyPreset = G2PlotHierarchyPresetBase & {
  sourceField: string;
  targetField: string;
  weightField: string;
  sourceLabel?: string;
  targetLabel?: string;
  weightLabel?: string;
  valueFormat?: ValueFormat;
};

const defaultSize = { width: 640, height: 380 } as const;

export function g2plotTreemap(config: G2PlotTreemapPreset): ChartSpec {
  return {
    ...baseSpec(config),
    marks: [{
      id: `${config.id}-treemap`,
      type: "treemap",
      encoding: {
        label: field(config.labelField, "nominal"),
        size: field(config.valueField, "quantitative", config.valueFormat),
        color: config.colorField ? field(config.colorField, "nominal") : undefined,
      },
    }],
  };
}

export function g2plotSunburst(config: G2PlotSunburstPreset): ChartSpec {
  return {
    ...baseSpec(config),
    coordinate: { type: "polar" },
    marks: [{
      id: `${config.id}-sunburst`,
      type: "sunburst",
      encoding: {
        label: field(config.labelField, "nominal"),
        parent: field(config.parentField, "nominal"),
        series: field(config.levelField, "ordinal"),
        color: config.colorField ? field(config.colorField, "nominal") : undefined,
        size: field(config.valueField, "quantitative", config.valueFormat),
      },
    }],
  };
}

export function g2plotSankey(config: G2PlotSankeyPreset): ChartSpec {
  return {
    ...baseSpec(config),
    axes: [],
    marks: [{
      id: `${config.id}-sankey`,
      type: "sankey",
      encoding: {
        source: field(config.sourceField, "nominal", undefined, config.sourceLabel),
        target: field(config.targetField, "nominal", undefined, config.targetLabel),
        size: field(config.weightField, "quantitative", config.valueFormat, config.weightLabel),
      },
    }],
  };
}

function baseSpec(config: G2PlotHierarchyPresetBase): Omit<ChartSpec, "marks"> {
  return {
    id: config.id,
    title: config.title,
    description: config.description,
    task: config.task,
    family: config.family ?? "G2Plot",
    width: config.width ?? defaultSize.width,
    height: config.height ?? defaultSize.height,
    data: config.data,
  };
}

function field(fieldName: string, type: FieldEncoding["type"], format?: ValueFormat, label?: string): FieldEncoding {
  return { field: fieldName, type, format, label };
}
