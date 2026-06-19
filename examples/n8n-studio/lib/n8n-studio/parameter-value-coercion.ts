import type { ParameterField } from "./types";

function parseBooleanInput(value: unknown) {
  if (typeof value === "boolean") {
    return value;
  }
  if (typeof value !== "string") {
    return Boolean(value);
  }

  const normalizedValue = value.trim().toLowerCase();
  if (["true", "1", "yes", "on"].includes(normalizedValue)) {
    return true;
  }
  if (["false", "0", "no", "off", ""].includes(normalizedValue)) {
    return false;
  }
  return value;
}

function parseNumberInput(value: unknown) {
  if (typeof value === "number") {
    return Number.isFinite(value) ? value : undefined;
  }
  if (typeof value !== "string") {
    return value;
  }

  const trimmedValue = value.trim();
  if (!trimmedValue) {
    return undefined;
  }

  const parsedValue = Number(trimmedValue);
  return Number.isFinite(parsedValue) ? parsedValue : value;
}

function parseStructuredInput(value: unknown) {
  if (typeof value !== "string") {
    return value;
  }

  const trimmedValue = value.trim();
  if (!trimmedValue || !["{", "["].includes(trimmedValue[0] ?? "")) {
    return value;
  }

  try {
    return JSON.parse(trimmedValue);
  } catch {
    return value;
  }
}

export function coerceParameterInputValue(
  field: ParameterField,
  value: unknown,
): unknown {
  if (field.type === "boolean") {
    return parseBooleanInput(value);
  }
  if (field.type === "number") {
    return parseNumberInput(value);
  }
  if (
    field.type === "json" ||
    field.type === "resourceLocator" ||
    field.type === "resourceMapper"
  ) {
    return parseStructuredInput(value);
  }
  return value;
}
