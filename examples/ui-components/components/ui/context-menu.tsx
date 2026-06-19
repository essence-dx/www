import { cn } from "../../lib/utils";
import { Icon } from "../icons/icon";
import type { DxElementProps } from "./types";

function ContextMenu(props: DxElementProps) {
  return <div data-slot="context-menu" data-adapter-boundary="context-menu" {...props} />;
}

function ContextMenuTrigger({ className, ...props }: DxElementProps) {
  return <button type="button" data-slot="context-menu-trigger" className={cn("cn-menu-trigger", className)} {...props} />;
}

function ContextMenuContent({ className, ...props }: DxElementProps) {
  return <div data-slot="context-menu-content" className={cn("cn-menu-content", className)} {...props} />;
}

function ContextMenuItem({ className, ...props }: DxElementProps) {
  return <div role="menuitem" data-slot="context-menu-item" className={cn("cn-menu-item", className)} {...props} />;
}

function ContextMenuCheckboxItem({ children, className, checked, ...props }: DxElementProps & { checked?: boolean }) {
  return (
    <div role="menuitemcheckbox" aria-checked={checked} data-slot="context-menu-checkbox-item" className={cn("cn-menu-item", className)} {...props}>
      <Icon name={checked ? "pack:check" : "pack:blank"} className="cn-slot-icon" />
      {children}
    </div>
  );
}

function ContextMenuRadioItem({ children, className, checked, ...props }: DxElementProps & { checked?: boolean }) {
  return (
    <div role="menuitemradio" aria-checked={checked} data-slot="context-menu-radio-item" className={cn("cn-menu-item", className)} {...props}>
      <Icon name={checked ? "pack:circle-dot" : "pack:circle"} className="cn-slot-icon" />
      {children}
    </div>
  );
}

function ContextMenuLabel({ className, ...props }: DxElementProps) {
  return <div data-slot="context-menu-label" className={cn("cn-menu-label", className)} {...props} />;
}

function ContextMenuSeparator({ className, ...props }: DxElementProps) {
  return <div role="separator" data-slot="context-menu-separator" className={cn("cn-menu-separator", className)} {...props} />;
}

function ContextMenuShortcut({ className, ...props }: DxElementProps) {
  return <span data-slot="context-menu-shortcut" className={cn("cn-menu-shortcut", className)} {...props} />;
}

function ContextMenuGroup(props: DxElementProps) {
  return <div role="group" data-slot="context-menu-group" {...props} />;
}

function ContextMenuPortal(props: DxElementProps) {
  return <div data-slot="context-menu-portal" {...props} />;
}

function ContextMenuSub(props: DxElementProps) {
  return <div data-slot="context-menu-sub" {...props} />;
}

function ContextMenuSubTrigger({ children, className, ...props }: DxElementProps) {
  return (
    <ContextMenuItem data-slot="context-menu-sub-trigger" className={className} {...props}>
      {children}
      <Icon name="pack:chevron-right" className="cn-slot-icon" />
    </ContextMenuItem>
  );
}

function ContextMenuSubContent(props: DxElementProps) {
  return <ContextMenuContent data-slot="context-menu-sub-content" {...props} />;
}

function ContextMenuRadioGroup(props: DxElementProps) {
  return <div role="radiogroup" data-slot="context-menu-radio-group" {...props} />;
}

export {
  ContextMenu,
  ContextMenuTrigger,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuCheckboxItem,
  ContextMenuRadioItem,
  ContextMenuLabel,
  ContextMenuSeparator,
  ContextMenuShortcut,
  ContextMenuGroup,
  ContextMenuPortal,
  ContextMenuSub,
  ContextMenuSubContent,
  ContextMenuSubTrigger,
  ContextMenuRadioGroup,
};
