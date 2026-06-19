import { cn } from "../../lib/utils";
import type { DxElementProps } from "./types";

type AlertVariant = "default" | "destructive";

function Alert({
  className,
  variant = "default",
  ...props
}: DxElementProps & { variant?: AlertVariant }) {
  return (
    <div
      role="alert"
      data-slot="alert"
      data-variant={variant}
      className={cn("cn-alert", `cn-alert-${variant}`, className)}
      {...props}
    />
  );
}

function AlertTitle({ className, ...props }: DxElementProps) {
  return (
    <div
      data-slot="alert-title"
      className={cn("cn-alert-title", className)}
      {...props}
    />
  );
}

function AlertDescription({ className, ...props }: DxElementProps) {
  return (
    <div
      data-slot="alert-description"
      className={cn("cn-alert-description", className)}
      {...props}
    />
  );
}

export { Alert, AlertTitle, AlertDescription };
