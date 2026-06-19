import { inferPromptAdviceIntent } from "../prompt-model";
import type {
  ChartPromptRequestSpec,
  ChartSpec,
  ChartToolRecommendationSpec,
  Datum,
  MarkKind,
} from "../spec";
import { field, fieldBuckets, firstField, labelFor, primitiveLabel, rowsForRequest, secondField, type FieldBuckets } from "./spec-fields";
import { summarizeBoxplotRows } from "./spec-summaries";

export function chartSpecFromToolRecommendation(request: ChartPromptRequestSpec, recommendation: ChartToolRecommendationSpec): ChartSpec {
  const buckets = fieldBuckets(request);
  const rows = rowsForRequest(request);
  const base = baseChart(request, recommendation, rows);

  switch (recommendation.chartType) {
    case "line":
      return cartesianChart(base, "line", buckets, rows, "line");
    case "area":
      return cartesianChart(base, "area", buckets, rows, "area");
    case "bar":
      return cartesianChart({ ...base, coordinate: { type: "transpose" } }, "bar", buckets, rows, "bar");
    case "column":
      return cartesianChart(base, "bar", buckets, rows, "column");
    case "scatter":
      return scatterChart(base, buckets, rows);
    case "histogram":
      return histogramChart(base, buckets, rows);
    case "pie":
      return pieChart(base, buckets, rows);
    case "radar":
      return radarChart(base, buckets, rows);
    case "boxplot":
      return boxplotChart(base, buckets, rows);
    case "funnel":
      return funnelChart(base, buckets, rows);
    case "waterfall":
      return waterfallChart(base, buckets, rows);
    case "treemap":
      return treemapChart(base, buckets, rows);
    case "chord":
      return chordChart(base, buckets, rows);
    case "sankey":
      return sankeyChart(base, buckets, rows);
    case "network-graph":
      return networkGraphChart(base, buckets, rows);
    case "table":
      return tableChart(base, buckets, rows);
    case "word-cloud":
      return wordCloudChart(base, buckets, rows);
    default:
      return cartesianChart(base, "bar", buckets, rows, "column");
  }
}

function baseChart(request: ChartPromptRequestSpec, recommendation: ChartToolRecommendationSpec, rows: Datum[]): ChartSpec {
  return {
    id: `${request.id}-${recommendation.chartType}`,
    title: `${recommendation.choice} route`,
    description: request.prompt,
    task: inferPromptAdviceIntent(request).task,
    family: "GPTVis",
    width: 640,
    height: 380,
    data: rows,
    marks: [],
  };
}

function cartesianChart(base: ChartSpec, mark: MarkKind, buckets: FieldBuckets, rows: Datum[], chartType: string): ChartSpec {
  const x = firstField([...buckets.time, ...buckets.dimensions], rows, "category");
  const y = firstField(buckets.measures, rows, "value");
  const series = buckets.dimensions.find((fieldSpec) => fieldSpec.name !== x.name);
  const lineLike = mark === "line" || mark === "area";

  return {
    ...base,
    marks: [{
      id: `${base.id}-${chartType}`,
      type: mark,
      encoding: {
        x: field(x.name, buckets.time.includes(x) ? "temporal" : "ordinal", x.label),
        y: field(y.name, "quantitative", y.label),
        color: series ? field(series.name, "nominal", series.label) : undefined,
        series: series && lineLike ? field(series.name, "nominal", series.label) : undefined,
      },
    }],
    axes: [{ channel: "x" }, { channel: "y", grid: true }],
    legend: series ? { channel: lineLike ? "series" : "color", title: series.label ?? labelFor(series.name) } : undefined,
  };
}

function scatterChart(base: ChartSpec, buckets: FieldBuckets, rows: Datum[]): ChartSpec {
  const x = firstField(buckets.measures, rows, "x");
  const y = buckets.measures.find((fieldSpec) => fieldSpec.name !== x.name) ?? firstField(buckets.measures, rows, "y");
  const color = buckets.dimensions[0];

  return {
    ...base,
    marks: [{
      id: `${base.id}-scatter`,
      type: "point",
      encoding: {
        x: field(x.name, "quantitative", x.label),
        y: field(y.name, "quantitative", y.label),
        color: color ? field(color.name, "nominal", color.label) : undefined,
      },
    }],
    axes: [{ channel: "x" }, { channel: "y", grid: true }],
    legend: color ? { channel: "color", title: color.label ?? labelFor(color.name) } : undefined,
  };
}

function histogramChart(base: ChartSpec, buckets: FieldBuckets, rows: Datum[]): ChartSpec {
  const measure = firstField(buckets.measures, rows, "value");
  return {
    ...base,
    marks: [{
      id: `${base.id}-histogram`,
      type: "bar",
      transforms: [{ type: "bin", field: measure.name, as: "bin", valueAs: "count", count: 8 }],
      encoding: { x: field("bin", "ordinal", measure.label ?? labelFor(measure.name)), y: field("count", "quantitative", "Count") },
    }],
    axes: [{ channel: "x" }, { channel: "y", grid: true }],
  };
}

function pieChart(base: ChartSpec, buckets: FieldBuckets, rows: Datum[]): ChartSpec {
  const label = firstField(buckets.dimensions, rows, "category");
  const value = firstField(buckets.measures, rows, "value");
  return {
    ...base,
    marks: [{
      id: `${base.id}-pie`,
      type: "pie",
      encoding: { label: field(label.name, "nominal", label.label), color: field(label.name, "nominal", label.label), theta: field(value.name, "quantitative", value.label) },
    }],
    legend: { channel: "color", title: label.label ?? labelFor(label.name) },
  };
}

function radarChart(base: ChartSpec, buckets: FieldBuckets, rows: Datum[]): ChartSpec {
  const axis = firstField(buckets.dimensions, rows, "axis");
  const value = firstField(buckets.measures, rows, "value");
  const series = buckets.dimensions.find((fieldSpec) => fieldSpec.name !== axis.name);
  return {
    ...base,
    marks: [{
      id: `${base.id}-radar`,
      type: "radar",
      encoding: { x: field(axis.name, "ordinal", axis.label), y: field(value.name, "quantitative", value.label), series: series ? field(series.name, "nominal", series.label) : undefined },
    }],
    legend: series ? { channel: "series", title: series.label ?? labelFor(series.name) } : undefined,
  };
}

function boxplotChart(base: ChartSpec, buckets: FieldBuckets, rows: Datum[]): ChartSpec {
  const category = firstField(buckets.dimensions, rows, "category");
  const measure = firstField(buckets.measures, rows, "value");
  return {
    ...base,
    data: summarizeBoxplotRows(rows, category.name, measure.name),
    marks: [{
      id: `${base.id}-boxplot`,
      type: "boxplot",
      encoding: {
        x: field(category.name, "ordinal", category.label),
        y: field("median", "quantitative", measure.label),
        low: field("low", "quantitative"),
        q1: field("q1", "quantitative"),
        median: field("median", "quantitative"),
        q3: field("q3", "quantitative"),
        high: field("high", "quantitative"),
      },
    }],
    axes: [{ channel: "x" }, { channel: "y", grid: true }],
  };
}

function funnelChart(base: ChartSpec, buckets: FieldBuckets, rows: Datum[]): ChartSpec {
  const label = firstField(buckets.dimensions, rows, "stage");
  const value = firstField(buckets.measures, rows, "value");
  return { ...base, marks: [{ id: `${base.id}-funnel`, type: "funnel", encoding: { label: field(label.name, "nominal", label.label), size: field(value.name, "quantitative", value.label) } }] };
}

function waterfallChart(base: ChartSpec, buckets: FieldBuckets, rows: Datum[]): ChartSpec {
  const x = firstField(buckets.dimensions, rows, "stage");
  const y = firstField(buckets.measures, rows, "delta");
  return { ...base, marks: [{ id: `${base.id}-waterfall`, type: "waterfall", encoding: { x: field(x.name, "ordinal", x.label), y: field(y.name, "quantitative", y.label) } }], axes: [{ channel: "x" }, { channel: "y", grid: true }] };
}

function treemapChart(base: ChartSpec, buckets: FieldBuckets, rows: Datum[]): ChartSpec {
  const label = firstField([...buckets.hierarchy, ...buckets.dimensions], rows, "label");
  const value = firstField(buckets.measures, rows, "value");
  return { ...base, marks: [{ id: `${base.id}-treemap`, type: "treemap", encoding: { label: field(label.name, "nominal", label.label), size: field(value.name, "quantitative", value.label) } }] };
}

function sankeyChart(base: ChartSpec, buckets: FieldBuckets, rows: Datum[]): ChartSpec {
  const source = firstField([...buckets.network, ...buckets.dimensions], rows, "source");
  const target = secondField([...buckets.network, ...buckets.dimensions], source, rows, "target");
  const value = firstField(buckets.measures, rows, "value");
  return { ...base, marks: [{ id: `${base.id}-sankey`, type: "sankey", encoding: { source: field(source.name, "nominal"), target: field(target.name, "nominal"), size: field(value.name, "quantitative") } }] };
}

function chordChart(base: ChartSpec, buckets: FieldBuckets, rows: Datum[]): ChartSpec {
  const source = firstField([...buckets.network, ...buckets.dimensions], rows, "source");
  const target = secondField([...buckets.network, ...buckets.dimensions], source, rows, "target");
  const value = firstField(buckets.measures, rows, "value");
  return {
    ...base,
    coordinate: { type: "polar" },
    marks: [{
      id: `${base.id}-chord`,
      type: "chord",
      encoding: {
        source: field(source.name, "nominal", source.label),
        target: field(target.name, "nominal", target.label),
        size: field(value.name, "quantitative", value.label),
        color: field(source.name, "nominal", source.label),
      },
      chord: { nodePaddingRatio: 0.024, nodeWidthRatio: 0.08, nodeSort: "weight-desc" },
    }],
    legend: { channel: "color", title: source.label ?? labelFor(source.name) },
  };
}

function networkGraphChart(base: ChartSpec, buckets: FieldBuckets, rows: Datum[]): ChartSpec {
  const source = firstField([...buckets.network, ...buckets.dimensions], rows, "source");
  const target = secondField([...buckets.network, ...buckets.dimensions], source, rows, "target");
  const edgeRows = rows.map((row, index) => ({ source: primitiveLabel(row[source.name], `source-${index + 1}`), target: primitiveLabel(row[target.name], `target-${index + 1}`), value: 1 }));
  const nodeIds = Array.from(new Set(edgeRows.flatMap((row) => [String(row.source), String(row.target)])));

  return {
    ...base,
    data: edgeRows,
    graph: {
      nodes: nodeIds.map((id) => ({ id, label: id, value: 5 })),
      edges: edgeRows.map((row, index) => ({ id: `${base.id}-edge-${index}`, source: String(row.source), target: String(row.target), weight: 2 })),
      layout: { type: "dagre-lite", rankDirection: "LR" },
      behaviors: [{ type: "zoom-canvas" }, { type: "focus-node", focus: { nodeId: nodeIds[0] ?? "source" } }],
      plugins: [{ type: "tooltip" }],
    },
    marks: [{ id: `${base.id}-network`, type: "graph", encoding: { source: field("source", "nominal"), target: field("target", "nominal") } }],
  };
}

function tableChart(base: ChartSpec, buckets: FieldBuckets, rows: Datum[]): ChartSpec {
  const row = firstField(buckets.dimensions, rows, "category");
  const column = buckets.dimensions.find((fieldSpec) => fieldSpec.name !== row.name);
  const value = firstField(buckets.measures, rows, "value");
  return {
    ...base,
    marks: [{ id: `${base.id}-table`, type: "pivot", encoding: { x: field(column?.name ?? row.name, "nominal"), y: field(row.name, "nominal"), color: field(value.name, "quantitative") } }],
    table: {
      rows: [{ field: row.name, label: row.label }],
      columns: column ? [{ field: column.name, label: column.label }] : [],
      values: [{ field: value.name, label: value.label, reducer: "sum" }],
      hierarchyType: "grid",
      interactions: [{ type: "cell-hover" }, { type: "sort-header", sort: { target: "row", valueField: value.name, order: "desc" } }],
      totals: { row: "right", grand: true },
      sort: [{ target: "row", valueField: value.name, order: "desc" }],
    },
  };
}

function wordCloudChart(base: ChartSpec, buckets: FieldBuckets, rows: Datum[]): ChartSpec {
  const term = firstField(buckets.dimensions, rows, "term");
  const weight = firstField(buckets.measures, rows, "weight");
  return { ...base, marks: [{ id: `${base.id}-word-cloud`, type: "wordcloud", encoding: { label: field(term.name, "nominal"), size: field(weight.name, "quantitative") } }] };
}
