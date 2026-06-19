import { cn } from "../../lib/utils";
import type { DxInputProps } from "./types";

function Checkbox({ className, type: _type, ...props }: DxInputProps) {
  return (
    <input
      type="checkbox"
      data-slot="checkbox"
      className={cn("cn-checkbox", className)}
      {...props}
    />
  );
}

export { Checkbox };
