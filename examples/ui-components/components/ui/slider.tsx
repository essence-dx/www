import { cn } from "../../lib/utils";
import type { DxInputProps } from "./types";

function Slider({ className, type: _type, ...props }: DxInputProps) {
  return (
    <input
      type="range"
      data-slot="slider"
      className={cn("cn-slider", className)}
      {...props}
    />
  );
}

export { Slider };
