import { cn } from "../../lib/utils";
import type { DxElementProps } from "./types";

function Popover({ className, ...props }: DxElementProps) {
  return <div data-slot="popover" data-adapter-boundary="popover" className={cn("cn-popover", className)} {...props} />;
}

function PopoverTrigger({ className, ...props }: DxElementProps) {
  return <button type="button" data-slot="popover-trigger" className={cn("cn-popover-trigger", className)} {...props} />;
}

function PopoverContent({ className, ...props }: DxElementProps) {
  return <div data-slot="popover-content" className={cn("cn-popover-content", className)} {...props} />;
}

function PopoverAnchor({ className, ...props }: DxElementProps) {
  return <span data-slot="popover-anchor" className={cn("cn-popover-anchor", className)} {...props} />;
}

function PopoverHeader({ className, ...props }: DxElementProps) {
  return <div data-slot="popover-header" className={cn("cn-popover-header", className)} {...props} />;
}

function PopoverTitle({ className, ...props }: DxElementProps) {
  return <div data-slot="popover-title" className={cn("cn-popover-title", className)} {...props} />;
}

function PopoverDescription({ className, ...props }: DxElementProps) {
  return <p data-slot="popover-description" className={cn("cn-popover-description", className)} {...props} />;
}

export {
  Popover,
  PopoverTrigger,
  PopoverContent,
  PopoverAnchor,
  PopoverHeader,
  PopoverTitle,
  PopoverDescription,
};
