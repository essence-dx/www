import { cn } from "../../lib/utils";
import { Icon } from "../icons/icon";
import { Dialog, DialogContent } from "./dialog";
import type { DxElementProps, DxInputProps } from "./types";

function Command({ className, ...props }: DxElementProps) {
  return (
    <div
      data-slot="command"
      data-adapter-boundary="command"
      className={cn("cn-command", className)}
      {...props}
    />
  );
}

function CommandDialog({ children, ...props }: DxElementProps) {
  return (
    <Dialog data-slot="command-dialog">
      <DialogContent data-slot="command-dialog-content" {...props}>
        <Command>{children}</Command>
      </DialogContent>
    </Dialog>
  );
}

function CommandInput({ className, ...props }: DxInputProps) {
  return (
    <div data-slot="command-input-wrapper" className="cn-command-input-wrapper">
      <Icon name="pack:search" className="cn-slot-icon" />
      <input
        data-slot="command-input"
        className={cn("cn-command-input", className)}
        {...props}
      />
    </div>
  );
}

function CommandList({ className, ...props }: DxElementProps) {
  return <div data-slot="command-list" className={cn("cn-command-list", className)} {...props} />;
}

function CommandEmpty({ className, ...props }: DxElementProps) {
  return <div data-slot="command-empty" className={cn("cn-command-empty", className)} {...props} />;
}

function CommandGroup({ className, ...props }: DxElementProps) {
  return <div data-slot="command-group" className={cn("cn-command-group", className)} {...props} />;
}

function CommandItem({ className, ...props }: DxElementProps) {
  return <div role="option" data-slot="command-item" className={cn("cn-command-item", className)} {...props} />;
}

function CommandShortcut({ className, ...props }: DxElementProps) {
  return <span data-slot="command-shortcut" className={cn("cn-command-shortcut", className)} {...props} />;
}

function CommandSeparator({ className, ...props }: DxElementProps) {
  return <div role="separator" data-slot="command-separator" className={cn("cn-command-separator", className)} {...props} />;
}

export {
  Command,
  CommandDialog,
  CommandInput,
  CommandList,
  CommandEmpty,
  CommandGroup,
  CommandItem,
  CommandShortcut,
  CommandSeparator,
};
