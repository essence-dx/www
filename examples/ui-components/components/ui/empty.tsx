import { cn } from "../../lib/utils";
import type { DxElementProps } from "./types";

function Empty({ className, ...props }: DxElementProps) {
  return (
    <div data-slot="empty" className={cn("cn-empty", className)} {...props} />
  );
}

function EmptyHeader({ className, ...props }: DxElementProps) {
  return (
    <div
      data-slot="empty-header"
      className={cn("cn-empty-header", className)}
      {...props}
    />
  );
}

function EmptyTitle({ className, ...props }: DxElementProps) {
  return (
    <div
      data-slot="empty-title"
      className={cn("cn-empty-title", className)}
      {...props}
    />
  );
}

function EmptyDescription({ className, ...props }: DxElementProps) {
  return (
    <p
      data-slot="empty-description"
      className={cn("cn-empty-description", className)}
      {...props}
    />
  );
}

function EmptyContent({ className, ...props }: DxElementProps) {
  return (
    <div
      data-slot="empty-content"
      className={cn("cn-empty-content", className)}
      {...props}
    />
  );
}

function EmptyMedia({ className, ...props }: DxElementProps) {
  return (
    <div
      data-slot="empty-media"
      className={cn("cn-empty-media", className)}
      {...props}
    />
  );
}

export {
  Empty,
  EmptyHeader,
  EmptyTitle,
  EmptyDescription,
  EmptyContent,
  EmptyMedia,
};
