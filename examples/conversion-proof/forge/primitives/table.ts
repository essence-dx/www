import { cn } from "./class-merge";
import type { ForgeElementRecipe, ForgePrimitiveRecipe } from "./types";

export const tablePrimitive: ForgePrimitiveRecipe = {
  name: "table",
  status: "source-owned",
  sourceFiles: ["G:\\WWW\\inspirations\\shadcn-ui\\apps\\v4\\styles\\radix-vega\\ui\\table.tsx"],
  replacesRuntimePackages: [],
  defaultClass: "w-full border-collapse text-sm",
  variants: {
    density: {
      compact: "[&_td]:py-2 [&_th]:py-2",
      spacious: "[&_td]:py-4 [&_th]:py-4",
    },
  },
  accessibility: ["Keep real table markup for comparable source surfaces.", "Use th scope when the renderer supports it."],
  runtimeBoundary: "Static table recipe only; data fetching remains outside this proof.",
};

export function createTableRecipe(density: keyof typeof tablePrimitive.variants.density = "spacious"): ForgeElementRecipe {
  return {
    tag: "table",
    className: cn(tablePrimitive.defaultClass, tablePrimitive.variants.density[density]),
    attributes: {},
  };
}
