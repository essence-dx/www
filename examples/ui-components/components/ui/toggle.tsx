import { cn } from "../../lib/utils";
import { Button, type ButtonProps } from "./button";

type ToggleVariant = "default" | "outline";
type ToggleSize = "default" | "sm" | "lg";

function toggleVariants({
  variant = "default",
  size = "default",
  className,
}: {
  variant?: ToggleVariant;
  size?: ToggleSize;
  className?: string;
} = {}) {
  return cn(
    "cn-toggle",
    `cn-toggle-${variant}`,
    `cn-toggle-${size}`,
    className,
  );
}

export type ToggleProps = ButtonProps & {
  variant?: ToggleVariant;
  size?: ToggleSize;
  pressed?: boolean;
};

function Toggle({
  className,
  variant = "default",
  size = "default",
  pressed,
  ...props
}: ToggleProps) {
  return (
    <Button
      type="button"
      role="button"
      aria-pressed={pressed}
      data-slot="toggle"
      data-state={pressed ? "on" : "off"}
      variant={variant === "outline" ? "outline" : "ghost"}
      size={size}
      className={toggleVariants({ variant, size, className })}
      {...props}
    />
  );
}

export { Toggle, toggleVariants };
