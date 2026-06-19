import type { ChartPrimitive, ChartPromptFieldSpec, ChartPromptRequestSpec, Datum, FieldEncoding } from "../spec";

export type FieldBuckets = {
  dimensions: ChartPromptFieldSpec[];
  measures: ChartPromptFieldSpec[];
  time: ChartPromptFieldSpec[];
  network: ChartPromptFieldSpec[];
  hierarchy: ChartPromptFieldSpec[];
};

export function field(name: string, type: FieldEncoding["type"], label?: string): FieldEncoding {
  return { field: name, type, label };
}

export function fieldBuckets(request: ChartPromptRequestSpec): FieldBuckets {
  return {
    dimensions: request.fields.filter((fieldSpec) => fieldSpec.role === "dimension" || fieldSpec.role === "geo"),
    measures: request.fields.filter((fieldSpec) => fieldSpec.role === "measure"),
    time: request.fields.filter((fieldSpec) => fieldSpec.role === "time"),
    network: request.fields.filter((fieldSpec) => fieldSpec.role === "network"),
    hierarchy: request.fields.filter((fieldSpec) => fieldSpec.role === "hierarchy"),
  };
}

export function rowsForRequest(request: ChartPromptRequestSpec): Datum[] {
  if (request.sampleRows && request.sampleRows.length > 0) return request.sampleRows;

  const fields = fieldsForSyntheticRows(request);
  const rowCount = Math.max(4, Math.min(8, request.recordCount || 5));
  return Array.from({ length: rowCount }, (_, index) => {
    const row: Datum = {};
    fields.forEach((fieldSpec) => {
      row[fieldSpec.name] = syntheticValue(fieldSpec, index);
    });
    return row;
  });
}

export function firstField(fields: ChartPromptFieldSpec[], rows: Datum[], fallback: string): ChartPromptFieldSpec {
  return fields[0] ?? inferredField(rows, fallback);
}

export function secondField(fields: ChartPromptFieldSpec[], first: ChartPromptFieldSpec, rows: Datum[], fallback: string): ChartPromptFieldSpec {
  return fields.find((fieldSpec) => fieldSpec.name !== first.name) ?? inferredField(rows, fallback);
}

export function primitiveLabel(value: ChartPrimitive | undefined, fallback: string): string {
  if (value === null || value === undefined || value === "") return fallback;
  return value instanceof Date ? value.toISOString() : String(value);
}

export function labelFor(fieldName: string): string {
  return fieldName.replace(/([a-z])([A-Z])/g, "$1 $2").replace(/[-_]/g, " ").replace(/\b\w/g, (value) => value.toUpperCase());
}

function inferredField(rows: Datum[], fallback: string): ChartPromptFieldSpec {
  const row = rows[0] ?? {};
  const keys = Object.keys(row);
  const wantsNumber = ["value", "x", "y", "weight", "delta"].includes(fallback);
  const numeric = keys.find((key) => typeof row[key] === "number");
  const text = keys.find((key) => typeof row[key] === "string");
  const name = wantsNumber ? numeric ?? fallback : text ?? fallback;
  return { name, role: numeric === name ? "measure" : "dimension", label: labelFor(name) };
}

function fieldsForSyntheticRows(request: ChartPromptRequestSpec): ChartPromptFieldSpec[] {
  const fields = request.fields.length > 0 ? [...request.fields] : [{ name: "category", role: "dimension" as const }, { name: "value", role: "measure" as const }];
  if (!fields.some((fieldSpec) => fieldSpec.role === "dimension" || fieldSpec.role === "geo" || fieldSpec.role === "time")) {
    fields.unshift({ name: "category", role: "dimension", label: "Category" });
  }
  if (!fields.some((fieldSpec) => fieldSpec.role === "measure")) {
    fields.push({ name: "value", role: "measure", label: "Value" });
  }
  return fields;
}

function syntheticValue(fieldSpec: ChartPromptFieldSpec, index: number): ChartPrimitive {
  if (fieldSpec.role === "measure") return 24 + index * 9;
  if (fieldSpec.role === "time") return index + 1;
  return `${labelFor(fieldSpec.name)} ${index + 1}`;
}
