import type { ChartAdviceCandidateSpec, ChartAdviceIntentSpec, ChartAdviceRecommendationSpec, Datum } from "./spec";

export const DEFAULT_CHART_CANDIDATES: ChartAdviceCandidateSpec[] = [
  { id: "bar", label: "Bar", mark: "bar", strengths: ["comparison", "composition"] },
  { id: "line", label: "Line", mark: "line", strengths: ["trend"] },
  { id: "heatmap", label: "Heatmap", mark: "heatmap", strengths: ["distribution", "table"] },
  { id: "graph", label: "Graph", mark: "graph", strengths: ["relation"] },
  { id: "map", label: "Map", mark: "map", strengths: ["map"] },
  { id: "pie", label: "Pie", mark: "pie", strengths: ["proportion"] },
  { id: "point", label: "Scatter", mark: "point", strengths: ["relation", "distribution"] },
  { id: "pivot", label: "Pivot", mark: "pivot", strengths: ["table"] },
];

export function recommendCharts(
  intent: ChartAdviceIntentSpec,
  candidates: ChartAdviceCandidateSpec[] = DEFAULT_CHART_CANDIDATES,
): ChartAdviceRecommendationSpec[] {
  return scoreChartCandidates(intent, candidates)
    .sort((left, right) => right.confidence - left.confidence)
    .map((recommendation) => ({
      ...recommendation,
      rationale: explainRecommendation(intent, recommendation),
    }));
}

export function scoreChartCandidates(intent: ChartAdviceIntentSpec, candidates: ChartAdviceCandidateSpec[]): ChartAdviceRecommendationSpec[] {
  return candidates.map((candidate) => {
    const score = candidateScore(intent, candidate);
    const confidence = Math.max(0.12, Math.min(0.98, score));
    return {
      choice: candidate.label,
      confidence,
      reason: primaryReason(intent, candidate),
      ruleId: `${candidate.id}-${intent.task}`,
      rationale: "",
    };
  });
}

export function explainRecommendation(intent: ChartAdviceIntentSpec, recommendation: ChartAdviceRecommendationSpec): string {
  const shape = `${intent.dimensions.length} dimensions, ${intent.measures.length} measures, ${intent.recordCount} rows`;
  return `${recommendation.choice} matches ${intent.task} intent with ${shape}.`;
}

export function materializeAdviceRows(recommendations: ChartAdviceRecommendationSpec[]): Datum[] {
  return recommendations.map((recommendation) => ({
    choice: recommendation.choice,
    confidence: recommendation.confidence,
    reason: recommendation.reason,
    ruleId: recommendation.ruleId,
    rationale: recommendation.rationale,
  }));
}

function candidateScore(intent: ChartAdviceIntentSpec, candidate: ChartAdviceCandidateSpec): number {
  let score = candidate.strengths.includes(intent.task) ? 0.72 : 0.32;
  if (candidate.mark === "line" && intent.hasTime) score += 0.2;
  if (candidate.mark === "map" && intent.hasGeo) score += 0.3;
  if (candidate.mark === "graph" && intent.hasNetwork) score += 0.28;
  if (candidate.mark === "treemap" && intent.hasHierarchy) score += 0.18;
  if (candidate.mark === "heatmap" && intent.dimensions.length >= 2) score += 0.16;
  if (candidate.mark === "pivot" && intent.dimensions.length >= 2 && intent.recordCount > 8) score += 0.2;
  if (candidate.mark === "point" && intent.measures.length >= 2) score += 0.18;
  if (candidate.mark === "bar" && intent.dimensions.length >= 1 && intent.measures.length >= 1) score += 0.14;
  if (candidate.mark === "pie" && intent.task !== "proportion") score -= 0.18;
  if (candidate.mark === "map" && !intent.hasGeo) score -= 0.18;
  if (candidate.mark === "graph" && !intent.hasNetwork) score -= 0.14;
  return score;
}

function primaryReason(intent: ChartAdviceIntentSpec, candidate: ChartAdviceCandidateSpec): string {
  if (candidate.mark === "line" && intent.hasTime) return "Trend";
  if (candidate.mark === "map" && intent.hasGeo) return "Location";
  if (candidate.mark === "graph" && intent.hasNetwork) return "Relation";
  if (candidate.mark === "heatmap" && intent.dimensions.length >= 2) return "Matrix";
  if (candidate.mark === "pivot" && intent.dimensions.length >= 2) return "Table";
  if (candidate.mark === "point" && intent.measures.length >= 2) return "Correlation";
  if (candidate.mark === "bar") return "Comparison";
  return candidate.strengths[0] ?? intent.task;
}
