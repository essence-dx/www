import { cn } from "../../lib/utils";
import type { DxElementProps } from "./types";

function Collapsible({ className, ...props }: DxElementProps) {
  return (
    <details
      data-slot="collapsible"
      className={cn("cn-collapsible", className)}
      {...props}
    />
  );
}

function CollapsibleTrigger({ className, ...props }: DxElementProps) {
  return (
    <summary
      data-slot="collapsible-trigger"
      className={cn("cn-collapsible-trigger", className)}
      {...props}
    />
  );
}

function CollapsibleContent({ className, ...props }: DxElementProps) {
  return (
    <div
      data-slot="collapsible-content"
      className={cn("cn-collapsible-content", className)}
      {...props}
    />
  );
}

export { Collapsible, CollapsibleTrigger, CollapsibleContent };
