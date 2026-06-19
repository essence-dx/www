import type { ChartPrimitive, Datum, ValueFormat } from "./spec";

export function readField(datum: Datum, field: string): ChartPrimitive {
  return Object.prototype.hasOwnProperty.call(datum, field) ? datum[field] : null;
}

export function toNumber(value: ChartPrimitive): number {
  if (value instanceof Date) {
    return value.getTime();
  }

  if (typeof value === "number" && Number.isFinite(value)) {
    return value;
  }

  if (typeof value === "string") {
    const parsed = Number(value);
    return Number.isFinite(parsed) ? parsed : 0;
  }

  return value === true ? 1 : 0;
}

export function toLabel(value: ChartPrimitive): string {
  if (value instanceof Date) {
    return value.toISOString().slice(0, 10);
  }

  if (value === null || value === undefined) {
    return "";
  }

  return String(value);
}

export function formatValue(value: ChartPrimitive, format?: ValueFormat): string {
  const numeric = toNumber(value);

  if (format === "currency") {
    return `$${numeric.toLocaleString("en-US", { maximumFractionDigits: 0 })}`;
  }

  if (format === "compact") {
    return Intl.NumberFormat("en-US", { notation: "compact", maximumFractionDigits: 1 }).format(numeric);
  }

  if (format === "percent") {
    return `${Math.round(numeric * 100)}%`;
  }

  if (format === "date") {
    return toLabel(value);
  }

  if (typeof value === "number") {
    return numeric.toLocaleString("en-US", { maximumFractionDigits: 1 });
  }

  return toLabel(value);
}

export function extent(values: ChartPrimitive[]): [number, number] {
  const numbers = values.map(toNumber).filter((value) => Number.isFinite(value));
  if (numbers.length === 0) {
    return [0, 1];
  }

  const min = Math.min(...numbers);
  const max = Math.max(...numbers);
  if (min === max) {
    return [0, max === 0 ? 1 : max * 1.1];
  }

  return [Math.min(0, min), max * 1.08];
}

export function uniqueLabels(values: ChartPrimitive[]): string[] {
  return Array.from(new Set(values.map(toLabel))).filter((value) => value.length > 0);
}
