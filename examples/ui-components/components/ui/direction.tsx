import type { DxElementProps } from "./types";

function DirectionProvider({ dir = "ltr", ...props }: DxElementProps & { dir?: "ltr" | "rtl" }) {
  return <div data-slot="direction-provider" dir={dir} {...props} />;
}

function useDirection() {
  return "ltr";
}

export { DirectionProvider, useDirection };
