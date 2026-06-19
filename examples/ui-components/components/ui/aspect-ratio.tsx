import { cn } from "../../lib/utils";
import type { DxElementProps } from "./types";

function AspectRatio({
  className,
  ratio = 16 / 9,
  ...props
}: DxElementProps & { ratio?: number }) {
  return (
    <div
      data-slot="ratio-box"
      data-ratio={ratio}
      className={cn("ui-aspect-frame", className)}
      {...props}
    />
  );
}

export { AspectRatio };
