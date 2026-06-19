import { cn } from "../../lib/utils";
import { Icon } from "../icons/icon";
import { Button } from "./button";
import type { ButtonProps } from "./button";
import type { DxElementProps } from "./types";

function Dialog({ className, ...props }: DxElementProps) {
  return (
    <div
      data-slot="dialog"
      data-adapter-boundary="dialog"
      className={cn("cn-dialog", className)}
      {...props}
    />
  );
}

function DialogTrigger(props: ButtonProps) {
  return <Button data-slot="dialog-trigger" {...props} />;
}

function DialogClose(props: ButtonProps) {
  return <Button data-slot="dialog-close" variant="ghost" {...props} />;
}

function DialogPortal({ className, ...props }: DxElementProps) {
  return <div data-slot="dialog-portal" className={cn("cn-dialog-portal", className)} {...props} />;
}

function DialogOverlay({ className, ...props }: DxElementProps) {
  return <div data-slot="dialog-overlay" className={cn("cn-dialog-overlay", className)} {...props} />;
}

function DialogContent({ children, className, ...props }: DxElementProps) {
  return (
    <div
      role="dialog"
      data-slot="dialog-content"
      className={cn("cn-dialog-content", className)}
      {...props}
    >
      {children}
      <Button data-slot="dialog-close-button" aria-label="Close" size="icon-sm" variant="ghost">
        <Icon name="pack:x" className="cn-slot-icon" />
      </Button>
    </div>
  );
}

function DialogHeader({ className, ...props }: DxElementProps) {
  return <div data-slot="dialog-header" className={cn("cn-dialog-header", className)} {...props} />;
}

function DialogFooter({ className, ...props }: DxElementProps) {
  return <div data-slot="dialog-footer" className={cn("cn-dialog-footer", className)} {...props} />;
}

function DialogTitle({ className, ...props }: DxElementProps) {
  return <div data-slot="dialog-title" className={cn("cn-dialog-title", className)} {...props} />;
}

function DialogDescription({ className, ...props }: DxElementProps) {
  return <p data-slot="dialog-description" className={cn("cn-dialog-description", className)} {...props} />;
}

export {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogOverlay,
  DialogPortal,
  DialogTitle,
  DialogTrigger,
};
