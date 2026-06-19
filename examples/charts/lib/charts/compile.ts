import { compileCartesian } from "./compiler/cartesian";
import { compileChord } from "./compiler/chord";
import { compileFacet, compileView } from "./compiler/composition";
import { compileFunnel } from "./compiler/funnel";
import { compileSunburst, compileTreemap } from "./compiler/hierarchy";
import { compileGraph, compileSankey } from "./compiler/network";
import { compileGauge, compilePie, compileRadar } from "./compiler/radial";
import { compileMap, compilePivot } from "./compiler/spatial";
import { emptyScene } from "./compiler/shared";
import { compileWaterfall } from "./compiler/waterfall";
import { compileWordCloud } from "./compiler/wordcloud";
import type { ChartCatalogItem } from "./spec";
import type { ChartScene } from "./scene";

export function compileChart(item: ChartCatalogItem): ChartScene {
  return compileChartSpec(item.spec);
}

function compileChartSpec(spec: ChartCatalogItem["spec"]): ChartScene {
  if (spec.composition?.type === "view") {
    return compileView(spec, compileChartSpec);
  }

  if (spec.composition?.type === "facet") {
    return compileFacet(spec, compileSingleChart);
  }

  return compileSingleChart(spec);
}

function compileSingleChart(spec: ChartCatalogItem["spec"]): ChartScene {
  const firstMark = spec.marks[0];
  if (!firstMark) {
    return emptyScene(spec);
  }

  if (firstMark.type === "pie") return compilePie(spec);
  if (firstMark.type === "radar") return compileRadar(spec);
  if (firstMark.type === "gauge") return compileGauge(spec);
  if (firstMark.type === "funnel") return compileFunnel(spec);
  if (firstMark.type === "waterfall") return compileWaterfall(spec);
  if (firstMark.type === "treemap") return compileTreemap(spec);
  if (firstMark.type === "sunburst") return compileSunburst(spec);
  if (firstMark.type === "sankey") return compileSankey(spec);
  if (firstMark.type === "chord") return compileChord(spec);
  if (firstMark.type === "graph") return compileGraph(spec);
  if (firstMark.type === "map") return compileMap(spec);
  if (firstMark.type === "pivot") return compilePivot(spec);
  if (firstMark.type === "wordcloud") return compileWordCloud(spec);

  return compileCartesian(spec);
}
