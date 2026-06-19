import { cn } from "./class-merge";
import { createSlot } from "./slot";
import type { ForgeElementRecipe, ForgePrimitiveRecipe } from "./types";

export const buttonPrimitive: ForgePrimitiveRecipe = {
  name: "button",
  status: "source-owned",
  sourceFiles: [
    "G:\\WWW\\inspirations\\shadcn-ui\\apps\\v4\\styles\\radix-vega\\ui\\button.tsx",
  ],
  replacesRuntimePackages: ["@radix-ui/react-slot", "class-variance-authority"],
  defaultClass:
    "inline-flex items-center justify-center rounded-md border px-3 py-2 text-sm font-medium transition-colors",
  variants: {
    intent: {
      primary: "border-transparent bg-foreground text-background",
      secondary: "border-border bg-surface text-foreground",
      ghost: "border-transparent bg-transparent text-foreground",
    },
    size: {
      sm: "h-8 px-2",
      md: "h-10 px-3",
      lg: "h-11 px-4",
    },
  },
  accessibility: ["Use button for actions and anchors for navigation.", "Disabled state must keep aria-disabled visible."],
  runtimeBoundary: "The proof emits class and slot recipes without React, Radix, or CVA runtime imports.",
};

export function createButtonRecipe(options: {
  intent?: keyof typeof buttonPrimitive.variants.intent;
  size?: keyof typeof buttonPrimitive.variants.size;
  asChild?: boolean;
  child?: Partial<ForgeElementRecipe>;
} = {}): ForgeElementRecipe {
  const className = cn(
    buttonPrimitive.defaultClass,
    buttonPrimitive.variants.intent[options.intent ?? "primary"],
    buttonPrimitive.variants.size[options.size ?? "md"],
  );

  return createSlot({
    fallbackTag: options.asChild ? "a" : "button",
    fallbackClassName: className,
    child: options.child,
  });
}
