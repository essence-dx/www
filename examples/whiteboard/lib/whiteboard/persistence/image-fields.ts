import { assertEmbeddedImageSource } from "../image-source";

export function embeddedImageSourceAt(value: unknown, path: string): string {
  if (typeof value !== "string" || value.trim().length === 0) {
    throw new Error(`${path} must be a non-empty string`);
  }

  assertEmbeddedImageSource(value, path);
  return value;
}

export function optionalPositiveNumberAt(value: unknown, path: string): number | undefined {
  if (value === undefined || value === null) {
    return undefined;
  }

  if (typeof value !== "number" || !Number.isFinite(value) || value <= 0) {
    throw new Error(`${path} must be greater than 0`);
  }

  return value;
}
