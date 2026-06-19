import { cn } from "../../lib/utils";
import type { DxElementProps } from "./types";

function ScrollArea({ className, ...props }: DxElementProps) {
  return (
    <div
      data-slot="scroll-area"
      className={cn("cn-scroll-area", className)}
      {...props}
    />
  );
}

function ScrollBar({
  className,
  orientation = "vertical",
  ...props
}: DxElementProps & { orientation?: "vertical" | "horizontal" }) {
  return (
    <div
      role="scrollbar"
      data-slot="scroll-bar"
      data-orientation={orientation}
      className={cn("cn-scroll-bar", className)}
      {...props}
    />
  );
}

export { ScrollArea, ScrollBar };
