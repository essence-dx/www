import { cn } from "../../lib/utils";
import type { DxElementProps } from "./types";

function Toaster({ className, ...props }: DxElementProps) {
  return (
    <div
      aria-live="polite"
      data-slot="toaster"
      data-adapter-boundary="sonner"
      className={cn("cn-toaster", className)}
      {...props}
    />
  );
}

export { Toaster };
