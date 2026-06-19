import { cn } from "../../lib/utils";
import type { DxInputProps } from "./types";

function Switch({ className, type: _type, ...props }: DxInputProps) {
  return (
    <input
      type="checkbox"
      role="switch"
      data-slot="switch"
      className={cn("cn-switch", className)}
      {...props}
    />
  );
}

export { Switch };
