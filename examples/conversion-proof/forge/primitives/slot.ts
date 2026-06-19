import { cn } from "./class-merge";
import type { ForgeAttributes, ForgeElementRecipe } from "./types";
import { cleanAttributes } from "./types";

export interface ForgeSlotInput {
  fallbackTag: string;
  fallbackClassName?: string;
  child?: Partial<ForgeElementRecipe>;
  attributes?: ForgeAttributes;
}

export function createSlot(input: ForgeSlotInput): ForgeElementRecipe {
  const tag = input.child?.tag ?? input.fallbackTag;
  const attributes = cleanAttributes({
    ...(input.attributes ?? {}),
    ...(input.child?.attributes ?? {}),
  });

  return {
    tag,
    attributes,
    className: cn(input.fallbackClassName, input.child?.className),
    children: input.child?.children,
  };
}
