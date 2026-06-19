import { inferPromptAdviceIntent } from "../prompt-model";
import type { ChartPromptFieldRole, ChartPromptRequestSpec, ChartToolRecommendationSpec, ChartToolSpec } from "../spec";
import { GPT_VIS_CHART_TOOLS } from "./chart-tools";

export function rankChartTools(request: ChartPromptRequestSpec, tools: ChartToolSpec[] = GPT_VIS_CHART_TOOLS): ChartToolRecommendationSpec[] {
  const intent = inferPromptAdviceIntent(request);

  return tools
    .filter((candidate) => candidate.supported)
    .map((candidate) => {
      const confidence = clamp(toolScore(request, candidate), 0.08, 0.98);
      return {
        toolId: candidate.id,
        choice: candidate.label,
        chartType: candidate.chartType,
        mark: candidate.mark,
        confidence,
        reason: toolReason(request, candidate),
        ruleId: `${candidate.id}-${intent.task}`,
        rationale: `${candidate.label} routes ${intent.task} intent from ${request.fields.length} fields and ${request.recordCount} rows.`,
      };
    })
    .sort((left, right) => right.confidence - left.confidence || left.choice.localeCompare(right.choice));
}

export function fallbackRecommendation(request: ChartPromptRequestSpec): ChartToolRecommendationSpec {
  return {
    toolId: "gptvis-column",
    choice: "Column",
    chartType: "column",
    mark: "bar",
    confidence: 0.42,
    reason: "Fallback",
    ruleId: `gptvis-column-${request.task ?? "comparison"}`,
    rationale: "Column is the safest fallback for dimensional comparison requests.",
  };
}

function toolScore(request: ChartPromptRequestSpec, candidate: ChartToolSpec): number {
  const intent = inferPromptAdviceIntent(request);
  const roles = request.fields.map((fieldSpec) => fieldSpec.role);
  const requiredMatches = countRoleMatches(roles, candidate.requiredRoles ?? []);
  const preferredMatches = countRoleMatches(roles, candidate.preferredRoles ?? []);
  const prompt = request.prompt.toLowerCase();
  let score = 0.24;

  if (candidate.tasks.includes(intent.task)) score += 0.42;
  score += requiredMatches * 0.08;
  score += preferredMatches * 0.06;
  if ((candidate.aliases ?? []).some((alias) => prompt.includes(alias))) score += 0.14;
  if (candidate.chartType === "line" && intent.hasTime) score += 0.14;
  if (candidate.chartType === "network-graph" && intent.hasNetwork) score += 0.2;
  if (candidate.chartType === "chord" && intent.hasNetwork) score += 0.16;
  if (candidate.chartType === "treemap" && intent.hasHierarchy) score += 0.12;
  if (candidate.chartType === "scatter" && intent.measures.length >= 2) score += 0.18;
  if (candidate.chartType === "pie" && request.recordCount > 8) score -= 0.14;
  if (candidate.chartType === "table" && request.recordCount > 24) score += 0.08;
  if (candidate.chartType === "boxplot" && request.sampleRows?.length) score += 0.08;
  if (candidate.requiredRoles && requiredMatches < candidate.requiredRoles.length) {
    score -= (candidate.requiredRoles.length - requiredMatches) * 0.1;
  }

  return score;
}

function toolReason(request: ChartPromptRequestSpec, candidate: ChartToolSpec): string {
  const prompt = request.prompt.toLowerCase();
  if ((candidate.aliases ?? []).some((alias) => prompt.includes(alias))) return "Prompt";
  if ((candidate.preferredRoles ?? []).some((role) => request.fields.some((fieldSpec) => fieldSpec.role === role))) return "Fields";
  return "Intent";
}

function countRoleMatches(roles: ChartPromptFieldRole[], requiredRoles: ChartPromptFieldRole[]): number {
  const available = [...roles];
  let matches = 0;
  requiredRoles.forEach((role) => {
    const index = available.indexOf(role);
    if (index >= 0) {
      matches += 1;
      available.splice(index, 1);
    }
  });
  return matches;
}

function clamp(value: number, min: number, max: number): number {
  return Math.max(min, Math.min(max, round(value)));
}

function round(value: number): number {
  return Math.round(value * 1000) / 1000;
}
