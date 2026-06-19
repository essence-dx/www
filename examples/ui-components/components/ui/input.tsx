import { cn } from "../../lib/utils";
import type { DxInputProps } from "./types";

function Input({ className, type, ...props }: DxInputProps) {
  return (
    <input
      type={type}
      data-slot="input"
      className={cn("cn-input flex w-full", className)}
      {...props}
    />
  );
}

export { Input };
