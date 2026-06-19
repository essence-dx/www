import { inferPromptAdviceIntent } from "./prompt-model";
import { fallbackRecommendation, rankChartTools } from "./router/scoring";
import { chartCatalogItemFromRoute } from "./router/catalog-item";
import { chartSpecFromToolRecommendation } from "./router/spec-builder";
import { gptVisSyntaxFromRoute } from "./router/syntax";
import type { ChartCatalogItem, ChartPromptRequestSpec, ChartSpec, ChartToolRouteSpec } from "./spec";

export { GPT_VIS_CHART_TOOLS } from "./router/chart-tools";
export { chartCatalogItemFromRoute } from "./router/catalog-item";
export { materializeToolRouteRows } from "./router/route-rows";
export { rankChartTools } from "./router/scoring";
export { chartSpecFromToolRecommendation } from "./router/spec-builder";
export { gptVisSyntaxFromRoute } from "./router/syntax";

export type ChartRouterMaterializationSpec = {
  route: ChartToolRouteSpec;
  chart: ChartSpec;
  catalogItem: ChartCatalogItem;
};

export function routeChartRequest(request: ChartPromptRequestSpec): ChartRouterMaterializationSpec {
  const recommendations = rankChartTools(request).slice(0, request.maxRecommendations ?? 5);
  const selected = recommendations[0] ?? fallbackRecommendation(request);
  const chart = chartSpecFromToolRecommendation(request, selected);
  const routeWithoutSyntax = {
    requestId: request.id,
    prompt: request.prompt,
    intent: inferPromptAdviceIntent(request),
    selected,
    recommendations,
    generatedSpecId: chart.id,
  };
  const syntax = gptVisSyntaxFromRoute(routeWithoutSyntax, chart);
  const route: ChartToolRouteSpec = { ...routeWithoutSyntax, syntax };

  const routedChart = { ...chart, router: route };
  return { route, chart: routedChart, catalogItem: chartCatalogItemFromRoute(routedChart) };
}
