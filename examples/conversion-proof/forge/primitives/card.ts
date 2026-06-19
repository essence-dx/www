import { cn } from "./class-merge";
import type { ForgeElementRecipe, ForgePrimitiveRecipe } from "./types";

export const cardPrimitive: ForgePrimitiveRecipe = {
  name: "card",
  status: "source-owned",
  sourceFiles: ["G:\\WWW\\inspirations\\shadcn-ui\\apps\\v4\\styles\\radix-vega\\ui\\card.tsx"],
  replacesRuntimePackages: [],
  defaultClass: "rounded-lg border border-border bg-surface text-foreground",
  variants: {
    density: {
      compact: "p-3",
      comfortable: "p-5",
    },
  },
  accessibility: ["Use semantic section/article roles before visual card wrappers.", "Avoid nested decorative cards."],
  runtimeBoundary: "Layout shell recipe only; launch pages use panels and tables for real source proof.",
};

export function createCardRecipe(density: keyof typeof cardPrimitive.variants.density = "comfortable"): ForgeElementRecipe {
  return {
    tag: "section",
    className: cn(cardPrimitive.defaultClass, cardPrimitive.variants.density[density]),
    attributes: {},
  };
}
