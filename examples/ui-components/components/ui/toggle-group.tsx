import { cn } from "../../lib/utils";
import { Toggle, type ToggleProps } from "./toggle";
import type { DxElementProps } from "./types";

function ToggleGroup({
  className,
  orientation = "horizontal",
  ...props
}: DxElementProps & { orientation?: "horizontal" | "vertical" }) {
  return (
    <div
      role="group"
      data-slot="toggle-group"
      data-orientation={orientation}
      className={cn("cn-toggle-group", className)}
      {...props}
    />
  );
}

function ToggleGroupItem({ className, ...props }: ToggleProps) {
  return (
    <Toggle
      data-slot="toggle-group-item"
      className={cn("cn-toggle-group-item", className)}
      {...props}
    />
  );
}

export { ToggleGroup, ToggleGroupItem };
