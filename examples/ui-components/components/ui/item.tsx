import { cn } from "../../lib/utils";
import { Separator, type SeparatorProps } from "./separator";
import { Slot } from "./slot";
import type { DxElementProps } from "./types";

type ItemVariant = "default" | "outline" | "muted";
type ItemSize = "default" | "sm" | "xs";
type ItemMediaVariant = "default" | "icon" | "image";

type ItemVariantOptions = {
  variant?: ItemVariant | null;
  size?: ItemSize | null;
  className?: string;
};

type ItemMediaVariantOptions = {
  variant?: ItemMediaVariant | null;
  className?: string;
};

const itemVariantClasses: Record<ItemVariant, string> = {
  default: "cn-item-variant-default",
  outline: "cn-item-variant-outline",
  muted: "cn-item-variant-muted",
};

const itemSizeClasses: Record<ItemSize, string> = {
  default: "cn-item-size-default",
  sm: "cn-item-size-sm",
  xs: "cn-item-size-xs",
};

const itemMediaVariantClasses: Record<ItemMediaVariant, string> = {
  default: "cn-item-media-variant-default",
  icon: "cn-item-media-variant-icon",
  image: "cn-item-media-variant-image",
};

function ItemGroup({ className, ...props }: DxElementProps) {
  return (
    <div
      role="list"
      data-slot="item-group"
      className={cn(
        "cn-item-group group/item-group flex w-full flex-col",
        className,
      )}
      {...props}
    />
  );
}

function ItemSeparator({ className, ...props }: SeparatorProps) {
  return (
    <Separator
      data-slot="item-separator"
      orientation="horizontal"
      className={cn("cn-item-separator", className)}
      {...props}
    />
  );
}

function itemVariants({
  variant = "default",
  size = "default",
  className,
}: ItemVariantOptions = {}) {
  return cn(
    "cn-item flex w-full flex-wrap items-center transition-colors duration-100 outline-none focus-visible:border-ring",
    itemVariantClasses[variant ?? "default"],
    itemSizeClasses[size ?? "default"],
    className,
  );
}

function Item({
  className,
  variant = "default",
  size = "default",
  asChild = false,
  ...props
}: DxElementProps &
  ItemVariantOptions & {
    asChild?: boolean;
  }) {
  const classNames = itemVariants({ variant, size, className });

  if (asChild) {
    return (
      <Slot.Root
        data-slot="item"
        data-variant={variant}
        data-size={size}
        className={classNames}
        {...props}
      />
    );
  }

  return (
    <div
      data-slot="item"
      data-variant={variant}
      data-size={size}
      className={classNames}
      {...props}
    />
  );
}

function itemMediaVariants({
  variant = "default",
  className,
}: ItemMediaVariantOptions = {}) {
  return cn(
    "cn-item-media flex shrink-0 items-center justify-center",
    itemMediaVariantClasses[variant ?? "default"],
    className,
  );
}

function ItemMedia({
  className,
  variant = "default",
  ...props
}: DxElementProps & ItemMediaVariantOptions) {
  return (
    <div
      data-slot="item-media"
      data-variant={variant}
      className={itemMediaVariants({ variant, className })}
      {...props}
    />
  );
}

function ItemContent({ className, ...props }: DxElementProps) {
  return (
    <div
      data-slot="item-content"
      className={cn("cn-item-content flex flex-1 flex-col", className)}
      {...props}
    />
  );
}

function ItemTitle({ className, ...props }: DxElementProps) {
  return (
    <div
      data-slot="item-title"
      className={cn("cn-item-title flex w-fit items-center", className)}
      {...props}
    />
  );
}

function ItemDescription({ className, ...props }: DxElementProps) {
  return (
    <p
      data-slot="item-description"
      className={cn("cn-item-description font-normal", className)}
      {...props}
    />
  );
}

function ItemActions({ className, ...props }: DxElementProps) {
  return (
    <div
      data-slot="item-actions"
      className={cn("cn-item-actions flex items-center", className)}
      {...props}
    />
  );
}

function ItemHeader({ className, ...props }: DxElementProps) {
  return (
    <div
      data-slot="item-header"
      className={cn(
        "cn-item-header flex basis-full items-center justify-between",
        className,
      )}
      {...props}
    />
  );
}

function ItemFooter({ className, ...props }: DxElementProps) {
  return (
    <div
      data-slot="item-footer"
      className={cn(
        "cn-item-footer flex basis-full items-center justify-between",
        className,
      )}
      {...props}
    />
  );
}

export {
  Item,
  ItemMedia,
  ItemContent,
  ItemActions,
  ItemGroup,
  ItemSeparator,
  ItemTitle,
  ItemDescription,
  ItemHeader,
  ItemFooter,
};
