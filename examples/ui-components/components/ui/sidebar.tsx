import { cn } from "../../lib/utils";
import { Icon } from "../icons/icon";
import { Button } from "./button";
import { Input } from "./input";
import { Separator } from "./separator";
import type { ButtonProps } from "./button";
import type { DxElementProps, DxInputProps } from "./types";

function useSidebar() {
  return {
    state: "expanded",
    open: true,
    setOpen: () => undefined,
    openMobile: false,
    setOpenMobile: () => undefined,
    isMobile: false,
    toggleSidebar: () => undefined,
  };
}

function SidebarProvider({ className, ...props }: DxElementProps) {
  return <div data-slot="sidebar-provider" className={cn("cn-sidebar-provider", className)} {...props} />;
}

function Sidebar({ className, side = "left", variant = "sidebar", ...props }: DxElementProps & { side?: "left" | "right"; variant?: string }) {
  return <aside data-slot="sidebar" data-side={side} data-variant={variant} className={cn("cn-sidebar", className)} {...props} />;
}

function SidebarContent({ className, ...props }: DxElementProps) {
  return <div data-slot="sidebar-content" className={cn("cn-sidebar-content", className)} {...props} />;
}

function SidebarFooter({ className, ...props }: DxElementProps) {
  return <div data-slot="sidebar-footer" className={cn("cn-sidebar-footer", className)} {...props} />;
}

function SidebarGroup({ className, ...props }: DxElementProps) {
  return <div data-slot="sidebar-group" className={cn("cn-sidebar-group", className)} {...props} />;
}

function SidebarGroupAction({ className, ...props }: ButtonProps) {
  return <Button data-slot="sidebar-group-action" size="icon-sm" variant="ghost" className={cn("cn-sidebar-group-action", className)} {...props} />;
}

function SidebarGroupContent({ className, ...props }: DxElementProps) {
  return <div data-slot="sidebar-group-content" className={cn("cn-sidebar-group-content", className)} {...props} />;
}

function SidebarGroupLabel({ className, ...props }: DxElementProps) {
  return <div data-slot="sidebar-group-label" className={cn("cn-sidebar-group-label", className)} {...props} />;
}

function SidebarHeader({ className, ...props }: DxElementProps) {
  return <div data-slot="sidebar-header" className={cn("cn-sidebar-header", className)} {...props} />;
}

function SidebarInput({ className, ...props }: DxInputProps) {
  return <Input data-slot="sidebar-input" className={cn("cn-sidebar-input", className)} {...props} />;
}

function SidebarInset({ className, ...props }: DxElementProps) {
  return <main data-slot="sidebar-inset" className={cn("cn-sidebar-inset", className)} {...props} />;
}

function SidebarMenu({ className, ...props }: DxElementProps) {
  return <ul data-slot="sidebar-menu" className={cn("cn-sidebar-menu", className)} {...props} />;
}

function SidebarMenuAction({ className, ...props }: ButtonProps) {
  return <Button data-slot="sidebar-menu-action" size="icon-sm" variant="ghost" className={cn("cn-sidebar-menu-action", className)} {...props} />;
}

function SidebarMenuBadge({ className, ...props }: DxElementProps) {
  return <span data-slot="sidebar-menu-badge" className={cn("cn-sidebar-menu-badge", className)} {...props} />;
}

function SidebarMenuButton({ className, isActive = false, ...props }: ButtonProps & { isActive?: boolean }) {
  return <Button data-slot="sidebar-menu-button" data-active={isActive} variant={isActive ? "secondary" : "ghost"} className={cn("cn-sidebar-menu-button", className)} {...props} />;
}

function SidebarMenuItem({ className, ...props }: DxElementProps) {
  return <li data-slot="sidebar-menu-item" className={cn("cn-sidebar-menu-item", className)} {...props} />;
}

function SidebarMenuSkeleton({ className, ...props }: DxElementProps) {
  return <div data-slot="sidebar-menu-skeleton" className={cn("cn-sidebar-menu-skeleton", className)} {...props} />;
}

function SidebarMenuSub({ className, ...props }: DxElementProps) {
  return <ul data-slot="sidebar-menu-sub" className={cn("cn-sidebar-menu-sub", className)} {...props} />;
}

function SidebarMenuSubButton({ className, isActive = false, ...props }: ButtonProps & { isActive?: boolean }) {
  return <Button data-slot="sidebar-menu-sub-button" data-active={isActive} variant="ghost" size="sm" className={cn("cn-sidebar-menu-sub-button", className)} {...props} />;
}

function SidebarMenuSubItem({ className, ...props }: DxElementProps) {
  return <li data-slot="sidebar-menu-sub-item" className={cn("cn-sidebar-menu-sub-item", className)} {...props} />;
}

function SidebarRail({ className, ...props }: DxElementProps) {
  return <button type="button" data-slot="sidebar-rail" className={cn("cn-sidebar-rail", className)} {...props} />;
}

function SidebarSeparator(props: DxElementProps) {
  return <Separator data-slot="sidebar-separator" {...props} />;
}

function SidebarTrigger({ className, ...props }: ButtonProps) {
  return (
    <Button data-slot="sidebar-trigger" size="icon" variant="ghost" className={className} {...props}>
      <Icon name="pack:panel-left" className="cn-slot-icon" />
    </Button>
  );
}

export {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupAction,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarInput,
  SidebarInset,
  SidebarMenu,
  SidebarMenuAction,
  SidebarMenuBadge,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarMenuSkeleton,
  SidebarMenuSub,
  SidebarMenuSubButton,
  SidebarMenuSubItem,
  SidebarProvider,
  SidebarRail,
  SidebarSeparator,
  SidebarTrigger,
  useSidebar,
};
