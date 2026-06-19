import { cn } from "../../lib/utils";
import { Slot } from "./slot";
import type { DxElementProps } from "./types";

type BadgeVariant =
  | "default"
  | "secondary"
  | "destructive"
  | "outline"
  | "ghost"
  | "link";

type BadgeVariantOptions = {
  variant?: BadgeVariant | null;
  className?: string;
};

const variants: Record<BadgeVariant, string> = {
  default: "cn-badge-variant-default",
  secondary: "cn-badge-variant-secondary",
  destructive: "cn-badge-variant-destructive",
  outline: "cn-badge-variant-outline",
  ghost: "cn-badge-variant-ghost",
  link: "cn-badge-variant-link",
};

function badgeVariants({
  variant = "default",
  className,
}: BadgeVariantOptions = {}) {
  return cn(
    "cn-badge group/badge inline-flex w-fit shrink-0 items-center justify-center overflow-hidden whitespace-nowrap focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 aria-invalid:border-destructive aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 [&>svg]:pointer-events-none",
    variants[variant ?? "default"],
    className,
  );
}

export type BadgeProps = DxElementProps &
  BadgeVariantOptions & {
    asChild?: boolean;
  };

function Badge({
  className,
  variant = "default",
  asChild = false,
  ...props
}: BadgeProps) {
  const classNames = badgeVariants({ variant, className });

  if (asChild) {
    return (
      <Slot.Root
        data-slot="badge"
        data-variant={variant}
        className={classNames}
        {...props}
      />
    );
  }

  return (
    <span
      data-slot="badge"
      data-variant={variant}
      className={classNames}
      {...props}
    />
  );
}

export { Badge, badgeVariants };
