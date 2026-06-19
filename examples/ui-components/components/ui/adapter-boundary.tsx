import { cn } from "../../lib/utils";
import type { DxElementProps } from "./types";

export type AdapterBoundaryProps = DxElementProps & {
  boundary: string;
  part: string;
};

export function AdapterBoundary({
  boundary,
  part,
  className,
  ...props
}: AdapterBoundaryProps) {
  return (
    <div
      data-slot={`${boundary}-${part}`}
      data-adapter-boundary={boundary}
      className={cn("cn-adapter-boundary", className)}
      {...props}
    />
  );
}
