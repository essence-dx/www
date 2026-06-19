import { cn } from "./class-merge";
import type { ForgeElementRecipe, ForgePrimitiveRecipe } from "./types";

export const badgePrimitive: ForgePrimitiveRecipe = {
  name: "badge",
  status: "source-owned",
  sourceFiles: ["G:\\WWW\\inspirations\\shadcn-ui\\apps\\v4\\registry\\styles.tsx"],
  replacesRuntimePackages: ["class-variance-authority"],
  defaultClass: "inline-flex items-center rounded-full border px-2 py-1 text-xs font-semibold",
  variants: {
    tone: {
      neutral: "border-border bg-surface text-foreground",
      success: "border-emerald-300 bg-emerald-50 text-emerald-900",
      warning: "border-amber-300 bg-amber-50 text-amber-900",
      blocked: "border-rose-300 bg-rose-50 text-rose-900",
    },
  },
  accessibility: ["Badges are status text, not buttons.", "Use clear words for partial and blocked states."],
  runtimeBoundary: "Static status badge recipe, no CVA runtime.",
};

export function createBadgeRecipe(tone: keyof typeof badgePrimitive.variants.tone = "neutral"): ForgeElementRecipe {
  return {
    tag: "span",
    className: cn(badgePrimitive.defaultClass, badgePrimitive.variants.tone[tone]),
    attributes: {},
  };
}
