import { cn } from "../../lib/utils";
import type { DxElementProps } from "./types";

function TooltipProvider({ className, ...props }: DxElementProps) {
  return <div data-slot="tooltip-provider" data-adapter-boundary="tooltip" className={cn("cn-tooltip-provider", className)} {...props} />;
}

function Tooltip({ className, ...props }: DxElementProps) {
  return <span data-slot="tooltip" className={cn("cn-tooltip", className)} {...props} />;
}

function TooltipTrigger({ className, ...props }: DxElementProps) {
  return <button type="button" data-slot="tooltip-trigger" className={cn("cn-tooltip-trigger", className)} {...props} />;
}

function TooltipContent({ className, ...props }: DxElementProps) {
  return <span role="tooltip" data-slot="tooltip-content" className={cn("cn-tooltip-content", className)} {...props} />;
}

export { Tooltip, TooltipTrigger, TooltipContent, TooltipProvider };
