import { cn } from "./class-merge";
import type { ForgeElementRecipe, ForgePrimitiveRecipe } from "./types";

export const dialogPrimitive: ForgePrimitiveRecipe = {
  name: "dialog",
  status: "source-owned",
  sourceFiles: ["G:\\WWW\\inspirations\\shadcn-ui\\apps\\v4\\registry\\bases\\base\\blocks\\dashboard-01\\components\\data-table.tsx"],
  replacesRuntimePackages: ["@radix-ui/react-dialog"],
  defaultClass: "rounded-lg border border-border bg-surface p-5 shadow-lg",
  variants: {
    size: {
      sm: "max-w-sm",
      md: "max-w-lg",
      lg: "max-w-2xl",
    },
  },
  accessibility: ["Dialog markup must expose aria-modal when made interactive.", "Focus trapping is blocked until runtime approval."],
  runtimeBoundary: "Dialog shell is represented as metadata; focus trapping is a missing runtime boundary.",
};

export function createDialogRecipe(size: keyof typeof dialogPrimitive.variants.size = "md"): ForgeElementRecipe {
  return {
    tag: "div",
    className: cn(dialogPrimitive.defaultClass, dialogPrimitive.variants.size[size]),
    attributes: { role: "dialog", "aria-modal": "true" },
  };
}
