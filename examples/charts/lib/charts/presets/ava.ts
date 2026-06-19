import { materializeAdviceRows, recommendCharts } from "../advice-model";
import type { ChartAdviceCandidateSpec, ChartAdviceIntentSpec, ChartSpec } from "../spec";

export type AvaAdviceChartPreset = {
  id: string;
  title: string;
  description: string;
  intent: ChartAdviceIntentSpec;
  candidates?: ChartAdviceCandidateSpec[];
  maxRecommendations?: number;
  width?: number;
  height?: number;
};

export function avaAdviceChart(config: AvaAdviceChartPreset): ChartSpec {
  const maxRecommendations = config.maxRecommendations ?? 5;
  const recommendations = recommendCharts(config.intent, config.candidates).slice(0, maxRecommendations);

  return {
    id: config.id,
    title: config.title,
    description: config.description,
    task: "ai",
    family: "AVA",
    width: config.width ?? 640,
    height: config.height ?? 380,
    data: materializeAdviceRows(recommendations),
    advice: { intent: config.intent, candidates: config.candidates, maxRecommendations },
    marks: [
      {
        id: `${config.id}-advice`,
        type: "bar",
        encoding: {
          x: { field: "choice", type: "ordinal" },
          y: { field: "confidence", type: "quantitative", format: "percent" },
          color: { field: "reason", type: "nominal" },
        },
      },
    ],
    legend: { channel: "color", title: "Reason" },
  };
}
