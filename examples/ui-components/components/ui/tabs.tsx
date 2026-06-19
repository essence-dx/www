import { cn } from "../../lib/utils";
import type { DxElementProps } from "./types";

function tabsListVariants({ className }: { className?: string } = {}) {
  return cn("cn-tabs-list", className);
}

function Tabs({ className, value, ...props }: DxElementProps & { value?: string }) {
  return <div data-slot="tabs" data-value={value} className={cn("cn-tabs", className)} {...props} />;
}

function TabsList({ className, ...props }: DxElementProps) {
  return <div role="tablist" data-slot="tabs-list" className={tabsListVariants({ className })} {...props} />;
}

function TabsTrigger({ className, value, ...props }: DxElementProps & { value?: string }) {
  return <button type="button" role="tab" data-slot="tabs-trigger" data-value={value} className={cn("cn-tabs-trigger", className)} {...props} />;
}

function TabsContent({ className, value, ...props }: DxElementProps & { value?: string }) {
  return <div role="tabpanel" data-slot="tabs-content" data-value={value} className={cn("cn-tabs-content", className)} {...props} />;
}

export { Tabs, TabsList, TabsTrigger, TabsContent, tabsListVariants };
