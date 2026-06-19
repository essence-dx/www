import { materializeToolRouteRows, routeChartRequest } from "../chart-router";
import type { ChartPromptRequestSpec, ChartSpec } from "../spec";

export type GptVisPromptChartPreset = {
  id: string;
  title: string;
  description: string;
  request: ChartPromptRequestSpec;
  width?: number;
  height?: number;
};

export function gptVisPromptChart(config: GptVisPromptChartPreset): ChartSpec {
  const { route } = routeChartRequest(config.request);

  return {
    id: config.id,
    title: config.title,
    description: config.description,
    task: "ai",
    family: "GPTVis",
    width: config.width ?? 640,
    height: config.height ?? 380,
    data: materializeToolRouteRows(route),
    advice: { intent: route.intent, maxRecommendations: config.request.maxRecommendations ?? 5 },
    router: route,
    marks: [
      {
        id: `${config.id}-prompt-advice`,
        type: "bar",
        encoding: {
          x: { field: "choice", type: "ordinal" },
          y: { field: "confidence", type: "quantitative", format: "percent" },
          color: { field: "reason", type: "nominal" },
          tooltip: { field: "prompt", type: "nominal" },
        },
      },
    ],
    legend: { channel: "color", title: "Reason" },
  };
}
