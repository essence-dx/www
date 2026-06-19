import type { ChartToolRouteSpec, Datum } from "../spec";

export function materializeToolRouteRows(route: ChartToolRouteSpec): Datum[] {
  return route.recommendations.map((recommendation, index) => ({
    choice: recommendation.choice,
    chartType: recommendation.chartType,
    confidence: recommendation.confidence,
    reason: recommendation.reason,
    ruleId: recommendation.ruleId,
    rationale: recommendation.rationale,
    requestId: route.requestId,
    prompt: route.prompt,
    selected: recommendation.toolId === route.selected.toolId,
    rank: index + 1,
  }));
}
