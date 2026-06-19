import { cn } from "../../lib/utils";
import type { DxElementProps } from "./types";

function Card({
  className,
  size = "default",
  ...props
}: DxElementProps & { size?: "default" | "sm" }) {
  return (
    <div
      data-slot="card"
      data-size={size}
      className={cn("cn-card group/card flex flex-col", className)}
      {...props}
    />
  );
}

function CardHeader({ className, ...props }: DxElementProps) {
  return (
    <div
      data-slot="card-header"
      className={cn(
        "cn-card-header group/card-header @container/card-header grid auto-rows-min items-start",
        className,
      )}
      {...props}
    />
  );
}

function CardTitle({ className, ...props }: DxElementProps) {
  return (
    <div
      data-slot="card-title"
      className={cn("cn-card-title cn-font-heading", className)}
      {...props}
    />
  );
}

function CardDescription({ className, ...props }: DxElementProps) {
  return (
    <div
      data-slot="card-description"
      className={cn("cn-card-description", className)}
      {...props}
    />
  );
}

function CardAction({ className, ...props }: DxElementProps) {
  return (
    <div
      data-slot="card-action"
      className={cn(
        "cn-card-action col-start-2 row-span-2 row-start-1 self-start justify-self-end",
        className,
      )}
      {...props}
    />
  );
}

function CardContent({ className, ...props }: DxElementProps) {
  return (
    <div
      data-slot="card-content"
      className={cn("cn-card-content", className)}
      {...props}
    />
  );
}

function CardFooter({ className, ...props }: DxElementProps) {
  return (
    <div
      data-slot="card-footer"
      className={cn("cn-card-footer flex items-center", className)}
      {...props}
    />
  );
}

export {
  Card,
  CardHeader,
  CardFooter,
  CardTitle,
  CardAction,
  CardDescription,
  CardContent,
};
