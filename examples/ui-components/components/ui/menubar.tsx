import {
  ContextMenuCheckboxItem,
  ContextMenuContent,
  ContextMenuGroup,
  ContextMenuItem,
  ContextMenuLabel,
  ContextMenuPortal,
  ContextMenuRadioGroup,
  ContextMenuRadioItem,
  ContextMenuSeparator,
  ContextMenuShortcut,
  ContextMenuSub,
  ContextMenuSubContent,
  ContextMenuSubTrigger,
} from "./context-menu";
import { cn } from "../../lib/utils";
import type { DxElementProps } from "./types";

function Menubar({ className, ...props }: DxElementProps) {
  return <div role="menubar" data-slot="menubar" data-adapter-boundary="menubar" className={cn("cn-menubar", className)} {...props} />;
}

function MenubarMenu(props: DxElementProps) {
  return <div data-slot="menubar-menu" {...props} />;
}

function MenubarTrigger({ className, ...props }: DxElementProps) {
  return <button type="button" role="menuitem" data-slot="menubar-trigger" className={cn("cn-menubar-trigger", className)} {...props} />;
}

function MenubarContent(props: DxElementProps) {
  return <ContextMenuContent data-slot="menubar-content" {...props} />;
}

function MenubarGroup(props: DxElementProps) {
  return <ContextMenuGroup data-slot="menubar-group" {...props} />;
}

function MenubarSeparator(props: DxElementProps) {
  return <ContextMenuSeparator data-slot="menubar-separator" {...props} />;
}

function MenubarLabel(props: DxElementProps) {
  return <ContextMenuLabel data-slot="menubar-label" {...props} />;
}

function MenubarItem(props: DxElementProps) {
  return <ContextMenuItem data-slot="menubar-item" {...props} />;
}

function MenubarShortcut(props: DxElementProps) {
  return <ContextMenuShortcut data-slot="menubar-shortcut" {...props} />;
}

function MenubarCheckboxItem(props: DxElementProps & { checked?: boolean }) {
  return <ContextMenuCheckboxItem data-slot="menubar-checkbox-item" {...props} />;
}

function MenubarRadioGroup(props: DxElementProps) {
  return <ContextMenuRadioGroup data-slot="menubar-radio-group" {...props} />;
}

function MenubarRadioItem(props: DxElementProps & { checked?: boolean }) {
  return <ContextMenuRadioItem data-slot="menubar-radio-item" {...props} />;
}

function MenubarSub(props: DxElementProps) {
  return <ContextMenuSub data-slot="menubar-sub" {...props} />;
}

function MenubarSubTrigger(props: DxElementProps) {
  return <ContextMenuSubTrigger data-slot="menubar-sub-trigger" {...props} />;
}

function MenubarSubContent(props: DxElementProps) {
  return <ContextMenuSubContent data-slot="menubar-sub-content" {...props} />;
}

function MenubarPortal(props: DxElementProps) {
  return <ContextMenuPortal data-slot="menubar-portal" {...props} />;
}

export {
  Menubar,
  MenubarPortal,
  MenubarMenu,
  MenubarTrigger,
  MenubarContent,
  MenubarGroup,
  MenubarSeparator,
  MenubarLabel,
  MenubarItem,
  MenubarShortcut,
  MenubarCheckboxItem,
  MenubarRadioGroup,
  MenubarRadioItem,
  MenubarSub,
  MenubarSubTrigger,
  MenubarSubContent,
};
