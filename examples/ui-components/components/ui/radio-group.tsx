import { cn } from "../../lib/utils";
import type { DxElementProps, DxInputProps } from "./types";

function RadioGroup({ className, ...props }: DxElementProps) {
  return (
    <div
      role="radiogroup"
      data-slot="radio-group"
      className={cn("cn-radio-group", className)}
      {...props}
    />
  );
}

function RadioGroupItem({ className, type: _type, ...props }: DxInputProps) {
  return (
    <input
      type="radio"
      data-slot="radio-group-item"
      className={cn("cn-radio-group-item", className)}
      {...props}
    />
  );
}

export { RadioGroup, RadioGroupItem };
