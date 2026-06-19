import { cn } from "./class-merge";
import type { ForgeElementRecipe, ForgePrimitiveRecipe } from "./types";
import { cleanAttributes } from "./types";

export const inputPrimitive: ForgePrimitiveRecipe = {
  name: "input",
  status: "source-owned",
  sourceFiles: ["G:\\WWW\\inspirations\\shadcn-ui\\apps\\v4\\styles\\radix-vega\\ui\\input.tsx"],
  replacesRuntimePackages: [],
  defaultClass:
    "h-10 w-full rounded-md border border-border bg-surface px-3 text-sm text-foreground outline-none",
  variants: {
    tone: {
      default: "focus:border-foreground",
      invalid: "border-danger focus:border-danger",
      muted: "bg-muted text-muted-foreground",
    },
  },
  accessibility: ["Every input needs a visible label or aria-label.", "Invalid state must pair with aria-invalid."],
  runtimeBoundary: "Native input recipe only; no form library or validation runtime is imported.",
};

export function createInputRecipe(options: {
  tone?: keyof typeof inputPrimitive.variants.tone;
  label: string;
  name: string;
}): ForgeElementRecipe {
  return {
    tag: "input",
    className: cn(inputPrimitive.defaultClass, inputPrimitive.variants.tone[options.tone ?? "default"]),
    attributes: cleanAttributes({
      "aria-label": options.label,
      name: options.name,
      type: "text",
    }),
  };
}
