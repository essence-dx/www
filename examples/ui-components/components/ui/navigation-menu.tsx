import { cn } from "../../lib/utils";
import { buttonVariants } from "./button";
import type { DxElementProps } from "./types";

function NavigationMenu({ className, ...props }: DxElementProps) {
  return <nav data-slot="navigation-menu" data-adapter-boundary="navigation-menu" className={cn("cn-navigation-menu", className)} {...props} />;
}

function NavigationMenuList({ className, ...props }: DxElementProps) {
  return <ul data-slot="navigation-menu-list" className={cn("cn-navigation-menu-list", className)} {...props} />;
}

function NavigationMenuItem({ className, ...props }: DxElementProps) {
  return <li data-slot="navigation-menu-item" className={cn("cn-navigation-menu-item", className)} {...props} />;
}

function NavigationMenuContent({ className, ...props }: DxElementProps) {
  return <div data-slot="navigation-menu-content" className={cn("cn-navigation-menu-content", className)} {...props} />;
}

function NavigationMenuTrigger({ className, ...props }: DxElementProps) {
  return <button type="button" data-slot="navigation-menu-trigger" className={cn(navigationMenuTriggerStyle(), className)} {...props} />;
}

function NavigationMenuLink({ className, ...props }: DxElementProps) {
  return <a data-slot="navigation-menu-link" className={cn("cn-navigation-menu-link", className)} {...props} />;
}

function NavigationMenuIndicator({ className, ...props }: DxElementProps) {
  return <div data-slot="navigation-menu-indicator" className={cn("cn-navigation-menu-indicator", className)} {...props} />;
}

function NavigationMenuViewport({ className, ...props }: DxElementProps) {
  return <div data-slot="navigation-menu-viewport" className={cn("cn-navigation-menu-viewport", className)} {...props} />;
}

function navigationMenuTriggerStyle() {
  return cn(buttonVariants({ variant: "ghost", size: "sm" }), "cn-navigation-menu-trigger-style");
}

export {
  NavigationMenu,
  NavigationMenuList,
  NavigationMenuItem,
  NavigationMenuContent,
  NavigationMenuTrigger,
  NavigationMenuLink,
  NavigationMenuIndicator,
  NavigationMenuViewport,
  navigationMenuTriggerStyle,
};
