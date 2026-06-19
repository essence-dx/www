import {
  ContextMenu,
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
  ContextMenuTrigger,
} from "./context-menu";
import type { DxElementProps } from "./types";

function DropdownMenu(props: DxElementProps) {
  return <ContextMenu data-slot="dropdown-menu" data-adapter-boundary="dropdown-menu" {...props} />;
}

function DropdownMenuTrigger(props: DxElementProps) {
  return <ContextMenuTrigger data-slot="dropdown-menu-trigger" {...props} />;
}

function DropdownMenuContent(props: DxElementProps) {
  return <ContextMenuContent data-slot="dropdown-menu-content" {...props} />;
}

function DropdownMenuGroup(props: DxElementProps) {
  return <ContextMenuGroup data-slot="dropdown-menu-group" {...props} />;
}

function DropdownMenuLabel(props: DxElementProps) {
  return <ContextMenuLabel data-slot="dropdown-menu-label" {...props} />;
}

function DropdownMenuItem(props: DxElementProps) {
  return <ContextMenuItem data-slot="dropdown-menu-item" {...props} />;
}

function DropdownMenuCheckboxItem(props: DxElementProps & { checked?: boolean }) {
  return <ContextMenuCheckboxItem data-slot="dropdown-menu-checkbox-item" {...props} />;
}

function DropdownMenuRadioGroup(props: DxElementProps) {
  return <ContextMenuRadioGroup data-slot="dropdown-menu-radio-group" {...props} />;
}

function DropdownMenuRadioItem(props: DxElementProps & { checked?: boolean }) {
  return <ContextMenuRadioItem data-slot="dropdown-menu-radio-item" {...props} />;
}

function DropdownMenuSeparator(props: DxElementProps) {
  return <ContextMenuSeparator data-slot="dropdown-menu-separator" {...props} />;
}

function DropdownMenuShortcut(props: DxElementProps) {
  return <ContextMenuShortcut data-slot="dropdown-menu-shortcut" {...props} />;
}

function DropdownMenuPortal(props: DxElementProps) {
  return <ContextMenuPortal data-slot="dropdown-menu-portal" {...props} />;
}

function DropdownMenuSub(props: DxElementProps) {
  return <ContextMenuSub data-slot="dropdown-menu-sub" {...props} />;
}

function DropdownMenuSubTrigger(props: DxElementProps) {
  return <ContextMenuSubTrigger data-slot="dropdown-menu-sub-trigger" {...props} />;
}

function DropdownMenuSubContent(props: DxElementProps) {
  return <ContextMenuSubContent data-slot="dropdown-menu-sub-content" {...props} />;
}

export {
  DropdownMenu,
  DropdownMenuPortal,
  DropdownMenuTrigger,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuLabel,
  DropdownMenuItem,
  DropdownMenuCheckboxItem,
  DropdownMenuRadioGroup,
  DropdownMenuRadioItem,
  DropdownMenuSeparator,
  DropdownMenuShortcut,
  DropdownMenuSub,
  DropdownMenuSubTrigger,
  DropdownMenuSubContent,
};
