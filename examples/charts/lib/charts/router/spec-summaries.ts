import type { Datum } from "../spec";
import { primitiveLabel } from "./spec-fields";

export function summarizeBoxplotRows(rows: Datum[], categoryField: string, measureField: string): Datum[] {
  const groups = new Map<string, number[]>();
  rows.forEach((row) => {
    const category = primitiveLabel(row[categoryField], "Group");
    const value = Number(row[measureField]);
    if (!Number.isFinite(value)) return;
    groups.set(category, [...(groups.get(category) ?? []), value]);
  });

  return Array.from(groups.entries()).map(([category, values]) => {
    const sorted = values.length > 0 ? [...values].sort((left, right) => left - right) : [0];
    return { [categoryField]: category, low: sorted[0] ?? 0, q1: quantile(sorted, 0.25), median: quantile(sorted, 0.5), q3: quantile(sorted, 0.75), high: sorted[sorted.length - 1] ?? 0 };
  });
}

function quantile(sortedValues: number[], percentile: number): number {
  if (sortedValues.length === 0) return 0;
  const index = (sortedValues.length - 1) * percentile;
  const lower = Math.floor(index);
  const upper = Math.ceil(index);
  const weight = index - lower;
  return round((sortedValues[lower] ?? 0) * (1 - weight) + (sortedValues[upper] ?? sortedValues[lower] ?? 0) * weight);
}

function round(value: number): number {
  return Math.round(value * 1000) / 1000;
}
