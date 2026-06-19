export type ForgePrimitiveStatus = "source-owned" | "launch-shim" | "blocked-runtime";

export type ForgeAttributeValue = string | number | boolean | null | undefined;

export type ForgeAttributes = Record<string, ForgeAttributeValue>;

export interface ForgePrimitiveRecipe {
  name: string;
  status: ForgePrimitiveStatus;
  sourceFiles: string[];
  replacesRuntimePackages: string[];
  defaultClass: string;
  variants: Record<string, Record<string, string>>;
  accessibility: string[];
  runtimeBoundary: string;
}

export interface ForgeElementRecipe {
  tag: string;
  className: string;
  attributes: ForgeAttributes;
  children?: string;
}

export function cleanAttributes(attributes: ForgeAttributes): Record<string, string> {
  return Object.fromEntries(
    Object.entries(attributes)
      .filter(([, value]) => value !== null && value !== undefined && value !== false)
      .map(([key, value]) => [key, value === true ? "" : String(value)]),
  );
}
