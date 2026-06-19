import { cn } from "../../lib/utils";
import type { DxElementProps } from "./types";

function Skeleton({ className, ...props }: DxElementProps) {
  return (
    <div
      data-slot="skeleton"
      className={cn("cn-skeleton", className)}
      {...props}
    />
  );
}

export { Skeleton };
