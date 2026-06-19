import type { ChartSpec, Datum } from "../spec";

export type F2WordCloudPreset = {
  id: string;
  title: string;
  description: string;
  data: Datum[];
  wordField: string;
  weightField: string;
  width?: number;
  height?: number;
};

export function f2WordCloud(config: F2WordCloudPreset): ChartSpec {
  return {
    id: config.id,
    title: config.title,
    description: config.description,
    task: "mobile",
    family: "F2",
    width: config.width ?? 640,
    height: config.height ?? 380,
    data: config.data,
    marks: [
      {
        id: `${config.id}-terms`,
        type: "wordcloud",
        encoding: {
          label: { field: config.wordField, type: "nominal" },
          size: { field: config.weightField, type: "quantitative" },
        },
      },
    ],
    padding: { top: 18, right: 22, bottom: 18, left: 22 },
  };
}
