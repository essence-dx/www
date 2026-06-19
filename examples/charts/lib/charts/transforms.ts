import type { ChartPrimitive, Datum, MarkSpec, ReducerKind, TransformSpec } from "./spec";
import { readField, toLabel, toNumber } from "./format";

export const STACK_START_FIELD = "__dxStackStart";
export const STACK_END_FIELD = "__dxStackEnd";
export const DODGE_INDEX_FIELD = "__dxDodgeIndex";
export const DODGE_COUNT_FIELD = "__dxDodgeCount";

export function applyMarkTransforms(data: Datum[], mark: MarkSpec): Datum[] {
  return applyTransforms(data, mark.transforms ?? []);
}

export function applyTransforms(data: Datum[], transforms: TransformSpec[]): Datum[] {
  return transforms.reduce((rows, transform) => applyTransform(rows, transform), data);
}

function applyTransform(data: Datum[], transform: TransformSpec): Datum[] {
  switch (transform.type) {
    case "filter":
      return filterRows(data, transform);
    case "sort":
      return sortRows(data, transform.field, transform.order ?? "asc");
    case "group":
      return groupRows(data, transform.groupBy, transform.field, transform.as, transform.reducer);
    case "bin":
      return binRows(data, transform.field, transform.as, transform.valueAs, transform.count ?? 8);
    case "stackY":
      return stackRows(data, transform.x, transform.y, transform.series, false);
    case "normalizeY":
      return stackRows(data, transform.x, transform.y, transform.series, true);
    case "dodgeX":
      return dodgeRows(data, transform.x, transform.series);
  }

  return assertNever(transform);
}

function filterRows(data: Datum[], transform: Extract<TransformSpec, { type: "filter" }>): Datum[] {
  return data.filter((datum) => {
    const value = readField(datum, transform.field);
    if (transform.equals !== undefined && toLabel(value) !== toLabel(transform.equals)) return false;
    if (transform.min !== undefined && toNumber(value) < transform.min) return false;
    if (transform.max !== undefined && toNumber(value) > transform.max) return false;
    return true;
  });
}

function sortRows(data: Datum[], field: string, order: "asc" | "desc"): Datum[] {
  const direction = order === "asc" ? 1 : -1;
  return [...data].sort((left, right) => {
    const leftValue = readField(left, field);
    const rightValue = readField(right, field);
    const numeric = toNumber(leftValue) - toNumber(rightValue);
    if (numeric !== 0) return numeric * direction;
    return toLabel(leftValue).localeCompare(toLabel(rightValue)) * direction;
  });
}

function groupRows(data: Datum[], groupBy: string[], field: string | undefined, as: string, reducer: ReducerKind): Datum[] {
  const groups = new Map<string, Datum[]>();
  data.forEach((datum) => {
    const key = groupBy.map((name) => toLabel(readField(datum, name))).join("\u001f");
    groups.set(key, [...(groups.get(key) ?? []), datum]);
  });

  return Array.from(groups.entries()).map(([key, rows]) => {
    const labels = key.split("\u001f");
    const grouped: Datum = {};
    groupBy.forEach((name, index) => {
      grouped[name] = labels[index] ?? "";
    });
    grouped[as] = reduceRows(rows, field, reducer);
    return grouped;
  });
}

function reduceRows(rows: Datum[], field: string | undefined, reducer: ReducerKind): ChartPrimitive {
  if (reducer === "count" || !field) return rows.length;
  const values = rows.map((row) => toNumber(readField(row, field))).filter(Number.isFinite);
  if (values.length === 0) return 0;
  if (reducer === "sum") return values.reduce((sum, value) => sum + value, 0);
  if (reducer === "mean") return values.reduce((sum, value) => sum + value, 0) / values.length;
  if (reducer === "min") return Math.min(...values);
  return Math.max(...values);
}

function binRows(data: Datum[], field: string, as: string, valueAs: string, count: number): Datum[] {
  const values = data.map((datum) => toNumber(readField(datum, field))).filter(Number.isFinite);
  if (values.length === 0) return [];
  const min = Math.min(...values);
  const max = Math.max(...values);
  const binCount = Math.max(1, count);
  const span = max - min || 1;
  const width = span / binCount;
  const bins = Array.from({ length: binCount }, (_, index) => {
    const start = min + width * index;
    const end = index === binCount - 1 ? max : start + width;
    return { start, end, count: 0 };
  });

  values.forEach((value) => {
    const index = Math.min(binCount - 1, Math.max(0, Math.floor((value - min) / width)));
    bins[index].count += 1;
  });

  return bins.map((bin) => ({
    [as]: `${formatBinEdge(bin.start)}-${formatBinEdge(bin.end)}`,
    [valueAs]: bin.count,
    binStart: bin.start,
    binEnd: bin.end,
  }));
}

function formatBinEdge(value: number): string {
  return Number.isInteger(value) ? String(value) : value.toFixed(1);
}

function stackRows(data: Datum[], x: string, y: string, series: string | undefined, normalize: boolean): Datum[] {
  const groups = rowsByField(data, x);
  const stacked: Datum[] = [];

  for (const rows of groups.values()) {
    const ordered = series ? sortRows(rows, series, "asc") : rows;
    const positiveTotal = ordered.reduce((sum, row) => sum + Math.max(0, toNumber(readField(row, y))), 0) || 1;
    const negativeTotal = Math.abs(ordered.reduce((sum, row) => sum + Math.min(0, toNumber(readField(row, y))), 0)) || 1;
    let positiveCursor = 0;
    let negativeCursor = 0;

    ordered.forEach((row) => {
      const raw = toNumber(readField(row, y));
      const total = raw < 0 ? negativeTotal : positiveTotal;
      const value = normalize ? raw / total : raw;
      const cursor = raw < 0 ? negativeCursor : positiveCursor;
      const next = cursor + value;
      stacked.push({ ...row, [STACK_START_FIELD]: cursor, [STACK_END_FIELD]: next });

      if (raw < 0) {
        negativeCursor = next;
      } else {
        positiveCursor = next;
      }
    });
  }

  return stacked;
}

function dodgeRows(data: Datum[], x: string, series: string): Datum[] {
  const groups = rowsByField(data, x);
  const seriesDomain = Array.from(new Set(data.map((row) => toLabel(readField(row, series)))));
  const dodged: Datum[] = [];

  for (const rows of groups.values()) {
    const count = Math.max(1, seriesDomain.length);
    rows.forEach((row) => {
      dodged.push({
        ...row,
        [DODGE_INDEX_FIELD]: Math.max(0, seriesDomain.indexOf(toLabel(readField(row, series)))),
        [DODGE_COUNT_FIELD]: count,
      });
    });
  }

  return dodged;
}

function rowsByField(data: Datum[], field: string): Map<string, Datum[]> {
  const groups = new Map<string, Datum[]>();
  data.forEach((datum) => {
    const key = toLabel(readField(datum, field));
    groups.set(key, [...(groups.get(key) ?? []), datum]);
  });
  return groups;
}

function assertNever(value: never): never {
  throw new Error(`Unsupported chart transform: ${JSON.stringify(value)}`);
}
