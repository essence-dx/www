import type { ChartCatalogItem, ChartSpec } from "../spec";

export function chartCatalogItemFromRoute(chart: ChartSpec): ChartCatalogItem {
  return {
    slug: chart.id,
    title: chart.title,
    task: chart.task,
    family: chart.family,
    summary: chart.description,
    whenToUse: "Use when a prompt-routed GPTVis recommendation should compile through the DX chart catalog pipeline.",
    avoidWhen: "Avoid when the caller needs raw routing metadata only and does not need a compiled chart scene.",
    dataShape: "Prompt-routed rows with typed chart encodings and source-owned DX chart metadata.",
    encodingNotes: [
      "The router lowers GPTVis chart choices into the same ChartSpec shape as hand-authored catalog charts.",
      "The catalog wrapper lets generated charts pass through compileChart without importing package runtimes.",
    ],
    spec: chart,
  };
}
