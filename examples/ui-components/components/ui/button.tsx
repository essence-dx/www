import { cn } from "../../lib/utils";
import { Slot } from "./slot";
import type { DxElementProps } from "./types";

type ButtonVariant =
  | "default"
  | "outline"
  | "secondary"
  | "ghost"
  | "destructive"
  | "link";
type ButtonSize =
  | "default"
  | "xs"
  | "sm"
  | "lg"
  | "icon"
  | "icon-xs"
  | "icon-sm"
  | "icon-lg";

type ButtonVariantOptions = {
  variant?: ButtonVariant | null;
  size?: ButtonSize | null;
  className?: string;
};

const variants: Record<ButtonVariant, string> = {
  default: "cn-button-variant-default",
  outline: "cn-button-variant-outline",
  secondary: "cn-button-variant-secondary",
  ghost: "cn-button-variant-ghost",
  destructive: "cn-button-variant-destructive",
  link: "cn-button-variant-link",
};

const sizes: Record<ButtonSize, string> = {
  default: "cn-button-size-default",
  xs: "cn-button-size-xs",
  sm: "cn-button-size-sm",
  lg: "cn-button-size-lg",
  icon: "cn-button-size-icon",
  "icon-xs": "cn-button-size-icon-xs",
  "icon-sm": "cn-button-size-icon-sm",
  "icon-lg": "cn-button-size-icon-lg",
};

function buttonVariants({
  variant = "default",
  size = "default",
  className,
}: ButtonVariantOptions = {}) {
  return cn(
    "cn-button group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0",
    variants[variant ?? "default"],
    sizes[size ?? "default"],
    className,
  );
}

export type ButtonProps = DxElementProps &
  ButtonVariantOptions & {
    asChild?: boolean;
    disabled?: boolean;
    type?: string;
  };

function Button({
  className,
  variant = "default",
  size = "default",
  asChild = false,
  ...props
}: ButtonProps) {
  const classNames = buttonVariants({ variant, size, className });

  if (asChild) {
    return (
      <Slot.Root
        data-slot="button"
        data-variant={variant}
        data-size={size}
        className={classNames}
        {...props}
      />
    );
  }

  return (
    <button
      data-slot="button"
      data-variant={variant}
      data-size={size}
      className={classNames}
      {...props}
    />
  );
}

export { Button, buttonVariants };
