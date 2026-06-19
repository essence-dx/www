import { cn } from "./class-merge";
import type { ForgeElementRecipe, ForgePrimitiveRecipe } from "./types";

export const tabsPrimitive: ForgePrimitiveRecipe = {
  name: "tabs",
  status: "source-owned",
  sourceFiles: ["G:\\WWW\\inspirations\\shadcn-ui\\apps\\v4\\registry\\bases\\base\\blocks\\dashboard-01\\page.tsx"],
  replacesRuntimePackages: ["@radix-ui/react-tabs"],
  defaultClass: "flex items-center gap-1 rounded-md border border-border bg-muted p-1",
  variants: {
    orientation: {
      horizontal: "flex-row",
      vertical: "flex-col items-stretch",
    },
  },
  accessibility: ["Expose tablist and tab roles in rendered markup.", "Inactive panels should stay discoverable in source proof metadata."],
  runtimeBoundary: "Tabs are launch navigation recipes; keyboard state machine is a future runtime upgrade.",
};

export function createTabsRecipe(orientation: keyof typeof tabsPrimitive.variants.orientation = "horizontal"): ForgeElementRecipe {
  return {
    tag: "div",
    className: cn(tabsPrimitive.defaultClass, tabsPrimitive.variants.orientation[orientation]),
    attributes: { role: "tablist" },
  };
}
