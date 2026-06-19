import type { ChartPrimitive, ChartSpec, ChartToolRouteSpec, FieldEncoding } from "../spec";

export function gptVisSyntaxFromRoute(route: Omit<ChartToolRouteSpec, "syntax">, chart: ChartSpec): string {
  const fieldNames = route.selected.chartType === "network-graph" ? ["source", "target", "value"] : chartFieldNames(chart);
  const safeFields = fieldNames.length > 0 ? fieldNames : Object.keys(chart.data[0] ?? {});
  const [firstField, ...restFields] = safeFields.length > 0 ? safeFields : ["value"];
  const lines = [`vis ${route.selected.chartType}`, "data"];

  chart.data.slice(0, 12).forEach((row) => {
    lines.push(`  - ${firstField} ${syntaxValue(row[firstField])}`);
    restFields.forEach((fieldName) => {
      lines.push(`    ${fieldName} ${syntaxValue(row[fieldName])}`);
    });
  });

  lines.push(`title ${quoteIfNeeded(chart.title)}`);
  return lines.join("\n");
}

function chartFieldNames(chart: ChartSpec): string[] {
  const mark = chart.marks[0];
  if (!mark) return Object.keys(chart.data[0] ?? {});
  const encodings = Object.values(mark.encoding).filter((encoding): encoding is FieldEncoding => Boolean(encoding));
  return Array.from(new Set(encodings.map((encoding) => encoding.field)));
}

function syntaxValue(value: ChartPrimitive | undefined): string {
  if (value === null || value === undefined) return "null";
  if (typeof value === "number" || typeof value === "boolean") return String(value);
  if (value instanceof Date) return quoteIfNeeded(value.toISOString());
  return quoteIfNeeded(String(value));
}

function quoteIfNeeded(value: string): string {
  return /\s/.test(value) ? `"${value.replace(/"/g, '\\"')}"` : value;
}
