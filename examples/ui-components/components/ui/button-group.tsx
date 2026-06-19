import { cn } from "../../lib/utils";
import type { DxElementProps } from "./types";

type ButtonGroupOrientation = "horizontal" | "vertical";

function buttonGroupVariants({
  orientation = "horizontal",
  className,
}: {
  orientation?: ButtonGroupOrientation;
  className?: string;
} = {}) {
  return cn(
    "cn-button-group",
    orientation === "vertical" ? "cn-button-group-vertical" : "cn-button-group-horizontal",
    className,
  );
}

function ButtonGroup({
  className,
  orientation = "horizontal",
  ...props
}: DxElementProps & { orientation?: ButtonGroupOrientation }) {
  return (
    <div
      role="group"
      data-slot="button-group"
      data-orientation={orientation}
      className={buttonGroupVariants({ orientation, className })}
      {...props}
    />
  );
}

function ButtonGroupSeparator({ className, ...props }: DxElementProps) {
  return (
    <div
      role="separator"
      data-slot="button-group-separator"
      className={cn("cn-button-group-separator", className)}
      {...props}
    />
  );
}

function ButtonGroupText({ className, ...props }: DxElementProps) {
  return (
    <span
      data-slot="button-group-text"
      className={cn("cn-button-group-text", className)}
      {...props}
    />
  );
}

export {
  ButtonGroup,
  ButtonGroupSeparator,
  ButtonGroupText,
  buttonGroupVariants,
};
