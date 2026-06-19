import { cn } from "./class-merge";
import type { ForgeElementRecipe, ForgePrimitiveRecipe } from "./types";

export const sidebarPrimitive: ForgePrimitiveRecipe = {
  name: "sidebar",
  status: "source-owned",
  sourceFiles: [
    "G:\\WWW\\inspirations\\shadcn-ui\\apps\\v4\\styles\\radix-vega\\ui\\sidebar.tsx",
    "G:\\WWW\\inspirations\\shadcn-ui\\apps\\v4\\registry\\bases\\base\\blocks\\dashboard-01\\components\\app-sidebar.tsx",
  ],
  replacesRuntimePackages: ["@radix-ui/react-slot"],
  defaultClass: "flex min-h-0 flex-col border-r border-border bg-surface",
  variants: {
    width: {
      compact: "w-56",
      default: "w-72",
      wide: "w-80",
    },
  },
  accessibility: ["Navigation groups need labels.", "Collapsed state should not hide route names from assistive tech."],
  runtimeBoundary: "Sidebar collapse and drag state are launch shims until DX-WWW interaction state is promoted.",
};

export function createSidebarRecipe(width: keyof typeof sidebarPrimitive.variants.width = "default"): ForgeElementRecipe {
  return {
    tag: "aside",
    className: cn(sidebarPrimitive.defaultClass, sidebarPrimitive.variants.width[width]),
    attributes: { "aria-label": "DX-WWW converted navigation" },
  };
}
