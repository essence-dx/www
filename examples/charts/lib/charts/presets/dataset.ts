import { createDataSetView, materializeDataSetFlowRows } from "../dataset-model";
import type { ChartSpec, DataSetViewSpec } from "../spec";

export type DataSetFlowChartPreset = {
  id: string;
  title: string;
  description: string;
  dataset: DataSetViewSpec;
  width?: number;
  height?: number;
};

export function dataSetFlowChart(config: DataSetFlowChartPreset): ChartSpec {
  const view = createDataSetView(config.dataset);

  return {
    id: config.id,
    title: config.title,
    description: config.description,
    task: "flow",
    family: "DataSet",
    width: config.width ?? 640,
    height: config.height ?? 380,
    data: materializeDataSetFlowRows(view),
    dataset: config.dataset,
    marks: [
      {
        id: `${config.id}-flow`,
        type: "sankey",
        encoding: {
          source: { field: "source", type: "nominal" },
          target: { field: "target", type: "nominal" },
          size: { field: "value", type: "quantitative" },
        },
      },
    ],
  };
}
