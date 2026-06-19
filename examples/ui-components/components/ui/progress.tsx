import { cn } from "../../lib/utils";
import type { DxElementProps } from "./types";

function Progress({
  className,
  value = 0,
  max = 100,
  ...props
}: DxElementProps & { value?: number; max?: number }) {
  return (
    <div
      role="progressbar"
      aria-valuemin={0}
      aria-valuemax={max}
      aria-valuenow={value}
      data-slot="progress"
      data-value={value}
      className={cn("cn-progress", className)}
      {...props}
    >
      <div data-slot="progress-indicator" className="cn-progress-indicator" />
    </div>
  );
}

export { Progress };
