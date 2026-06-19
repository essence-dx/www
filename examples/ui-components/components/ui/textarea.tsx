import { cn } from "../../lib/utils";
import type { DxTextareaProps } from "./types";

function Textarea({ className, ...props }: DxTextareaProps) {
  return (
    <textarea
      data-slot="textarea"
      className={cn("cn-textarea flex min-h-16 w-full", className)}
      {...props}
    />
  );
}

export { Textarea };
