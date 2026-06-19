import { materializeAdviceRows, recommendCharts } from "./advice-model";
import type { ChartAdviceCandidateSpec, ChartAdviceIntentSpec, ChartPromptFieldRole, ChartPromptRequestSpec, ChartTask, Datum, PromptChartMaterializationSpec } from "./spec";

const promptTaskHints: Array<{ task: ChartTask; words: string[] }> = [
  { task: "trend", words: ["trend", "time", "timeline", "over time", "weekly", "monthly"] },
  { task: "map", words: ["map", "geo", "location", "region", "city"] },
  { task: "relation", words: ["network", "graph", "relationship", "dependency", "correlation", "chord"] },
  { task: "table", words: ["pivot", "table", "matrix", "cross tab"] },
  { task: "distribution", words: ["spread", "distribution", "density", "histogram"] },
  { task: "proportion", words: ["share", "ratio", "percentage", "part to whole"] },
  { task: "flow", words: ["flow", "pipeline", "journey", "path"] },
  { task: "composition", words: ["stack", "composition", "breakdown"] },
  { task: "comparison", words: ["compare", "rank", "readiness", "score"] },
];

export function inferPromptAdviceIntent(request: ChartPromptRequestSpec): ChartAdviceIntentSpec {
  const roles = new Set(request.fields.map((field) => field.role));

  return {
    task: request.task ?? inferTaskFromPrompt(request.prompt),
    recordCount: request.recordCount,
    dimensions: fieldsByRole(request, ["dimension", "time", "geo", "network", "hierarchy"]),
    measures: fieldsByRole(request, ["measure"]),
    hasTime: roles.has("time") || promptIncludes(request.prompt, ["time", "trend", "weekly", "monthly"]),
    hasGeo: roles.has("geo") || promptIncludes(request.prompt, ["map", "geo", "location"]),
    hasNetwork: roles.has("network") || promptIncludes(request.prompt, ["network", "graph", "dependency", "chord", "source", "target"]),
    hasHierarchy: roles.has("hierarchy") || promptIncludes(request.prompt, ["tree", "hierarchy", "nested"]),
  };
}

export function materializePromptRecommendations(
  request: ChartPromptRequestSpec,
  candidates?: ChartAdviceCandidateSpec[],
): PromptChartMaterializationSpec {
  const intent = inferPromptAdviceIntent(request);
  const recommendations = recommendCharts(intent, candidates).slice(0, request.maxRecommendations ?? 5);

  return { request, intent, recommendations };
}

export function promptRecommendationRows(request: ChartPromptRequestSpec, candidates?: ChartAdviceCandidateSpec[]): Datum[] {
  const materialized = materializePromptRecommendations(request, candidates);

  return materializeAdviceRows(materialized.recommendations).map((row) => ({
    ...row,
    requestId: request.id,
    prompt: request.prompt,
    inferredTask: materialized.intent.task,
    fieldCount: request.fields.length,
  }));
}

function fieldsByRole(request: ChartPromptRequestSpec, roles: ChartPromptFieldRole[]): string[] {
  const accepted = new Set(roles);
  return request.fields.filter((field) => accepted.has(field.role)).map((field) => field.name);
}

function inferTaskFromPrompt(prompt: string): ChartTask {
  const normalized = prompt.toLowerCase();
  const match = promptTaskHints.find((hint) => hint.words.some((word) => normalized.includes(word)));
  return match?.task ?? "comparison";
}

function promptIncludes(prompt: string, words: string[]): boolean {
  const normalized = prompt.toLowerCase();
  return words.some((word) => normalized.includes(word));
}
