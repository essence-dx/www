import { cn } from "./class-merge";
import type { ForgeElementRecipe, ForgePrimitiveRecipe } from "./types";

export const dropdownPrimitive: ForgePrimitiveRecipe = {
  name: "dropdown",
  status: "source-owned",
  sourceFiles: ["G:\\WWW\\inspirations\\shadcn-ui\\apps\\v4\\registry\\bases\\base\\blocks\\dashboard-01\\components\\nav-user.tsx"],
  replacesRuntimePackages: ["@radix-ui/react-dropdown-menu"],
  defaultClass: "min-w-48 rounded-md border border-border bg-surface p-1 shadow-md",
  variants: {
    align: {
      start: "origin-top-left",
      end: "origin-top-right",
    },
  },
  accessibility: ["Menu trigger state must be explicit when interactive.", "Keyboard menu behavior is a future runtime upgrade."],
  runtimeBoundary: "Dropdown menu recipe only; Radix menu state is not imported.",
};

export function createDropdownRecipe(align: keyof typeof dropdownPrimitive.variants.align = "start"): ForgeElementRecipe {
  return {
    tag: "div",
    className: cn(dropdownPrimitive.defaultClass, dropdownPrimitive.variants.align[align]),
    attributes: { role: "menu" },
  };
}
