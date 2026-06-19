fn shadcn_button_templates() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "js/lib/utils.ts",
            r#"export function cn(...inputs: Array<string | false | null | undefined>) {
  return inputs.filter(Boolean).join(" ");
}
"#,
        ),
        (
            "js/ui/slot.tsx",
            r#"import * as React from "react";

export type SlotProps = React.HTMLAttributes<HTMLElement> & {
  children?: React.ReactNode;
};

const SlotRoot = React.forwardRef<HTMLElement, SlotProps>(
  ({ children, ...props }, ref) => {
    if (React.isValidElement(children)) {
      return React.cloneElement(children, {
        ...props,
        ref,
        className: [props.className, (children.props as { className?: string }).className]
          .filter(Boolean)
          .join(" "),
      } as React.HTMLAttributes<HTMLElement>);
    }

    return null;
  },
);

SlotRoot.displayName = "Slot.Root";

export const Slot = {
  Root: SlotRoot,
};
"#,
        ),
        (
            "js/ui/button.tsx",
            r#"import * as React from "react";

import { cn } from "../../lib/utils";
import { Slot } from "./slot";

type ButtonVariant = "default" | "outline" | "secondary" | "ghost" | "destructive" | "link";
type ButtonSize = "default" | "xs" | "sm" | "lg" | "icon" | "icon-xs" | "icon-sm" | "icon-lg";

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

export type ButtonProps = React.ComponentProps<"button"> &
  ButtonVariantOptions & {
  asChild?: boolean;
};

function Button({
  className,
  variant = "default",
  size = "default",
  asChild = false,
  ...props
}: ButtonProps) {
  const Comp = asChild ? Slot.Root : "button";

  return (
    <Comp
      data-slot="button"
      data-variant={variant}
      data-size={size}
      className={buttonVariants({ variant, size, className })}
      {...props}
    />
  );
}

export { Button, buttonVariants };
"#,
        ),
        (
            "js/ui/README.md",
            r#"# DX Forge UI Components: Button

This source-owned UI Components button package was materialized by DX Forge.
Compatibility id: `shadcn/ui/button`.
Upstream provenance: shadcn-ui v4 and Radix Slot.
It is editable project code and does not require `node_modules` lifecycle execution.
"#,
        ),
    ]
}

fn shadcn_badge_templates() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "js/lib/utils.ts",
            r#"export function cn(...inputs: Array<string | false | null | undefined>) {
  return inputs.filter(Boolean).join(" ");
}
"#,
        ),
        (
            "js/ui/slot.tsx",
            r#"import * as React from "react";

export type SlotProps = React.HTMLAttributes<HTMLElement> & {
  children?: React.ReactNode;
};

const SlotRoot = React.forwardRef<HTMLElement, SlotProps>(
  ({ children, ...props }, ref) => {
    if (React.isValidElement(children)) {
      return React.cloneElement(children, {
        ...props,
        ref,
        className: [props.className, (children.props as { className?: string }).className]
          .filter(Boolean)
          .join(" "),
      } as React.HTMLAttributes<HTMLElement>);
    }

    return null;
  },
);

SlotRoot.displayName = "Slot.Root";

export const Slot = {
  Root: SlotRoot,
};
"#,
        ),
        (
            "js/ui/badge.tsx",
            r#"import * as React from "react";

import { cn } from "../../lib/utils";
import { Slot } from "./slot";

type BadgeVariant = "default" | "secondary" | "destructive" | "outline" | "ghost" | "link";

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

export type BadgeProps = React.ComponentProps<"span"> &
  BadgeVariantOptions & {
    asChild?: boolean;
  };

function Badge({
  className,
  variant = "default",
  asChild = false,
  ...props
}: BadgeProps) {
  const Comp = asChild ? Slot.Root : "span";

  return (
    <Comp
      data-slot="badge"
      data-variant={variant}
      className={badgeVariants({ variant, className })}
      {...props}
    />
  );
}

export { Badge, badgeVariants };
"#,
        ),
        (
            "js/ui/README.md",
            r#"# DX Forge UI Components: Badge

This source-owned UI Components badge package was materialized by DX Forge.
Compatibility id: `shadcn/ui/badge`.
Upstream provenance: shadcn-ui v4 and Radix Slot.
It preserves the real Badge, badgeVariants, variant, asChild, Slot.Root, data-slot, and data-variant API shape while keeping app-owned label taxonomy and tone outside the package.

Deferred intentionally: full upstream registry sync, class-variance-authority as a template dependency, and project-specific status semantics.
"#,
        ),
    ]
}

fn shadcn_label_templates() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "js/lib/utils.ts",
            r#"export function cn(...inputs: Array<string | false | null | undefined>) {
  return inputs.filter(Boolean).join(" ");
}
"#,
        ),
        (
            "js/ui/label.tsx",
            r#"import * as React from "react";

import { cn } from "../../lib/utils";

export type LabelProps = React.ComponentProps<"label">;

function Label({ className, ...props }: LabelProps) {
  return (
    <label
      data-slot="label"
      className={cn(
        "cn-label flex items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed",
        className,
      )}
      {...props}
    />
  );
}

export { Label };
"#,
        ),
        (
            "js/ui/README.md",
            r#"# DX Forge UI Components: Label

This source-owned UI Components label package was materialized by DX Forge.
Compatibility id: `shadcn/ui/label`.
Upstream provenance: shadcn-ui v4 and the Radix Label primitive.
It preserves the real Label export, label props, data-slot marker, local `cn` helper, and upstream registry class contract while keeping app-owned form copy and validation relationships outside the package.

Deferred intentionally: full upstream registry sync, a hosted Radix dependency install, and app-specific accessibility copy review.
"#,
        ),
    ]
}

fn shadcn_separator_templates() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "js/lib/utils.ts",
            r#"export function cn(...inputs: Array<string | false | null | undefined>) {
  return inputs.filter(Boolean).join(" ");
}
"#,
        ),
        (
            "js/ui/separator.tsx",
            r#"import * as React from "react";

import { cn } from "../../lib/utils";

export type SeparatorOrientation = "horizontal" | "vertical";

export type SeparatorProps = React.ComponentProps<"div"> & {
  orientation?: SeparatorOrientation;
  decorative?: boolean;
};

function Separator({
  className,
  orientation = "horizontal",
  decorative = true,
  ...props
}: SeparatorProps) {
  return (
    <div
      data-slot="separator"
      data-orientation={orientation}
      role={decorative ? "none" : "separator"}
      aria-orientation={decorative ? undefined : orientation}
      className={cn(
        "cn-separator shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch",
        className,
      )}
      {...props}
    />
  );
}

export { Separator };
"#,
        ),
        (
            "js/ui/README.md",
            r#"# DX Forge UI Components: Separator

This source-owned UI Components separator package was materialized by DX Forge.
Compatibility id: `shadcn/ui/separator`.
Upstream provenance: shadcn-ui v4 and the Radix Separator primitive.
It preserves the real Separator export, `orientation`, `decorative`, `data-slot`, `data-orientation`, local `cn` helper, and upstream registry class contract while keeping app-owned information hierarchy outside the package.

Deferred intentionally: full upstream registry sync, a hosted Radix dependency install, and app-specific decorative-versus-semantic divider review.
"#,
        ),
    ]
}

fn shadcn_field_templates() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "js/lib/utils.ts",
            r#"export function cn(...inputs: Array<string | false | null | undefined>) {
  return inputs.filter(Boolean).join(" ");
}
"#,
        ),
        (
            "js/ui/label.tsx",
            r#"import * as React from "react";

import { cn } from "../../lib/utils";

export type LabelProps = React.ComponentProps<"label">;

function Label({ className, ...props }: LabelProps) {
  return (
    <label
      data-slot="label"
      className={cn(
        "cn-label flex items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed",
        className,
      )}
      {...props}
    />
  );
}

export { Label };
"#,
        ),
        (
            "js/ui/separator.tsx",
            r#"import * as React from "react";

import { cn } from "../../lib/utils";

export type SeparatorOrientation = "horizontal" | "vertical";

export type SeparatorProps = React.ComponentProps<"div"> & {
  orientation?: SeparatorOrientation;
  decorative?: boolean;
};

function Separator({
  className,
  orientation = "horizontal",
  decorative = true,
  ...props
}: SeparatorProps) {
  return (
    <div
      data-slot="separator"
      data-orientation={orientation}
      role={decorative ? "none" : "separator"}
      aria-orientation={decorative ? undefined : orientation}
      className={cn(
        "cn-separator shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch",
        className,
      )}
      {...props}
    />
  );
}

export { Separator };
"#,
        ),
        (
            "js/ui/field.tsx",
            r#"import * as React from "react";
import { useMemo } from "react";

import { cn } from "../../lib/utils";
import { Label } from "./label";
import { Separator } from "./separator";

export type FieldOrientation = "vertical" | "horizontal" | "responsive";

export type FieldVariantOptions = {
  orientation?: FieldOrientation;
  className?: string;
};

const fieldOrientationClasses: Record<FieldOrientation, string> = {
  vertical:
    "cn-field-orientation-vertical flex-col *:w-full [&>.sr-only]:w-auto",
  horizontal:
    "cn-field-orientation-horizontal flex-row items-center has-[>[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[>[data-slot=field-content]]:[&>[role=checkbox],[role=radio]]:mt-px",
  responsive:
    "cn-field-orientation-responsive flex-col *:w-full @md/field-group:flex-row @md/field-group:items-center @md/field-group:*:w-auto @md/field-group:has-[>[data-slot=field-content]]:items-start @md/field-group:*:data-[slot=field-label]:flex-auto [&>.sr-only]:w-auto @md/field-group:has-[>[data-slot=field-content]]:[&>[role=checkbox],[role=radio]]:mt-px",
};

function fieldVariants({
  orientation = "vertical",
  className,
}: FieldVariantOptions = {}) {
  return cn(
    "cn-field group/field flex w-full",
    fieldOrientationClasses[orientation],
    className,
  );
}

function FieldSet({ className, ...props }: React.ComponentProps<"fieldset">) {
  return (
    <fieldset
      data-slot="field-set"
      className={cn("cn-field-set flex flex-col", className)}
      {...props}
    />
  );
}

function FieldLegend({
  className,
  variant = "legend",
  ...props
}: React.ComponentProps<"legend"> & { variant?: "legend" | "label" }) {
  return (
    <legend
      data-slot="field-legend"
      data-variant={variant}
      className={cn("cn-field-legend", className)}
      {...props}
    />
  );
}

function FieldGroup({ className, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="field-group"
      className={cn(
        "cn-field-group group/field-group @container/field-group flex w-full flex-col",
        className,
      )}
      {...props}
    />
  );
}

function Field({
  className,
  orientation = "vertical",
  ...props
}: React.ComponentProps<"div"> & { orientation?: FieldOrientation }) {
  return (
    <div
      role="group"
      data-slot="field"
      data-orientation={orientation}
      className={cn(fieldVariants({ orientation }), className)}
      {...props}
    />
  );
}

function FieldContent({ className, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="field-content"
      className={cn(
        "cn-field-content group/field-content flex flex-1 flex-col leading-snug",
        className,
      )}
      {...props}
    />
  );
}

function FieldLabel({
  className,
  ...props
}: React.ComponentProps<typeof Label>) {
  return (
    <Label
      data-slot="field-label"
      className={cn(
        "cn-field-label group/field-label peer/field-label flex w-fit",
        "has-[>[data-slot=field]]:w-full has-[>[data-slot=field]]:flex-col",
        className,
      )}
      {...props}
    />
  );
}

function FieldTitle({ className, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="field-label"
      className={cn("cn-field-title flex w-fit items-center", className)}
      {...props}
    />
  );
}

function FieldDescription({ className, ...props }: React.ComponentProps<"p">) {
  return (
    <p
      data-slot="field-description"
      className={cn(
        "cn-field-description",
        className,
      )}
      {...props}
    />
  );
}

function FieldSeparator({
  children,
  className,
  ...props
}: React.ComponentProps<"div"> & {
  children?: React.ReactNode;
}) {
  return (
    <div
      data-slot="field-separator"
      data-content={Boolean(children)}
      className={cn("cn-field-separator relative", className)}
      {...props}
    >
      <Separator className="absolute inset-0 top-1/2" />
      {children ? (
        <span
          className="cn-field-separator-content relative mx-auto block w-fit bg-background"
          data-slot="field-separator-content"
        >
          {children}
        </span>
      ) : null}
    </div>
  );
}

function FieldError({
  className,
  children,
  errors,
  ...props
}: React.ComponentProps<"div"> & {
  errors?: Array<{ message?: string } | undefined>;
}) {
  const content = useMemo(() => {
    if (children) {
      return children;
    }

    if (!errors?.length) {
      return null;
    }

    const uniqueErrors = [
      ...new Map(errors.map((error) => [error?.message, error])).values(),
    ];

    if (uniqueErrors.length === 1) {
      return uniqueErrors[0]?.message;
    }

    return (
      <ul className="ml-4 flex list-disc flex-col gap-1">
        {uniqueErrors.map((error, index) =>
          error?.message ? <li key={index}>{error.message}</li> : null,
        )}
      </ul>
    );
  }, [children, errors]);

  if (!content) {
    return null;
  }

  return (
    <div
      role="alert"
      data-slot="field-error"
      className={cn("cn-field-error font-normal", className)}
      {...props}
    >
      {content}
    </div>
  );
}

export {
  Field,
  FieldLabel,
  FieldDescription,
  FieldError,
  FieldGroup,
  FieldLegend,
  FieldSeparator,
  FieldSet,
  FieldContent,
  FieldTitle,
};
"#,
        ),
        (
            "js/ui/README.md",
            r#"# DX Forge UI Components: Field

This source-owned UI Components field package was materialized by DX Forge.
Compatibility id: `shadcn/ui/field`.
Upstream provenance: shadcn-ui v4, Radix Label, and Radix Separator.
It preserves the real Field, FieldSet, FieldLegend, FieldGroup, FieldLabel, FieldDescription, FieldSeparator, FieldError, FieldContent, FieldTitle, orientation, `data-slot`, `data-orientation`, local `Label`, local `Separator`, local `cn` helper, and deduplicated error rendering API shape while keeping application-owned form semantics outside the package.

Deferred intentionally: full upstream registry sync, class-variance-authority as a template dependency, hosted Radix dependency installation, form validation rules, submit behavior, and app-specific accessibility copy review.
"#,
        ),
    ]
}

fn shadcn_item_templates() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "js/lib/utils.ts",
            r#"export function cn(...inputs: Array<string | false | null | undefined>) {
  return inputs.filter(Boolean).join(" ");
}
"#,
        ),
        (
            "js/ui/slot.tsx",
            r#"import * as React from "react";

export type SlotProps = React.HTMLAttributes<HTMLElement> & {
  children?: React.ReactNode;
};

const SlotRoot = React.forwardRef<HTMLElement, SlotProps>(
  ({ children, ...props }, ref) => {
    if (React.isValidElement(children)) {
      return React.cloneElement(children, {
        ...props,
        ref,
        className: [props.className, (children.props as { className?: string }).className]
          .filter(Boolean)
          .join(" "),
      } as React.HTMLAttributes<HTMLElement>);
    }

    return null;
  },
);

SlotRoot.displayName = "Slot.Root";

export const Slot = {
  Root: SlotRoot,
};
"#,
        ),
        (
            "js/ui/separator.tsx",
            r#"import * as React from "react";

import { cn } from "../../lib/utils";

export type SeparatorOrientation = "horizontal" | "vertical";

export type SeparatorProps = React.ComponentProps<"div"> & {
  orientation?: SeparatorOrientation;
  decorative?: boolean;
};

function Separator({
  className,
  orientation = "horizontal",
  decorative = true,
  ...props
}: SeparatorProps) {
  return (
    <div
      data-slot="separator"
      data-orientation={orientation}
      role={decorative ? "none" : "separator"}
      aria-orientation={decorative ? undefined : orientation}
      className={cn(
        "cn-separator shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch",
        className,
      )}
      {...props}
    />
  );
}

export { Separator };
"#,
        ),
        (
            "js/ui/item.tsx",
            r#"import * as React from "react";

import { cn } from "../../lib/utils";
import { Separator } from "./separator";
import { Slot } from "./slot";

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

function ItemGroup({ className, ...props }: React.ComponentProps<"div">) {
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

function ItemSeparator({
  className,
  ...props
}: React.ComponentProps<typeof Separator>) {
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
}: React.ComponentProps<"div"> &
  ItemVariantOptions & {
    asChild?: boolean;
  }) {
  const Comp = asChild ? Slot.Root : "div";

  return (
    <Comp
      data-slot="item"
      data-variant={variant}
      data-size={size}
      className={itemVariants({ variant, size, className })}
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
}: React.ComponentProps<"div"> & ItemMediaVariantOptions) {
  return (
    <div
      data-slot="item-media"
      data-variant={variant}
      className={itemMediaVariants({ variant, className })}
      {...props}
    />
  );
}

function ItemContent({ className, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="item-content"
      className={cn(
        "cn-item-content flex flex-1 flex-col",
        className,
      )}
      {...props}
    />
  );
}

function ItemTitle({ className, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="item-title"
      className={cn(
        "cn-item-title flex w-fit items-center",
        className,
      )}
      {...props}
    />
  );
}

function ItemDescription({ className, ...props }: React.ComponentProps<"p">) {
  return (
    <p
      data-slot="item-description"
      className={cn(
        "cn-item-description font-normal",
        className,
      )}
      {...props}
    />
  );
}

function ItemActions({ className, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="item-actions"
      className={cn("cn-item-actions flex items-center", className)}
      {...props}
    />
  );
}

function ItemHeader({ className, ...props }: React.ComponentProps<"div">) {
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

function ItemFooter({ className, ...props }: React.ComponentProps<"div">) {
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
"#,
        ),
        (
            "js/ui/README.md",
            r#"# DX Forge UI Components: Item

This source-owned UI Components item package was materialized by DX Forge.
Compatibility id: `shadcn/ui/item`.
Upstream provenance: shadcn-ui v4, Radix Slot, and Radix Separator.
It preserves the real Item, ItemMedia, ItemContent, ItemActions, ItemGroup, ItemSeparator, ItemTitle, ItemDescription, ItemHeader, ItemFooter, `variant`, `size`, `asChild`, `role="list"`, `data-slot`, local `Slot`, local `Separator`, and local `cn` helper API shape while keeping application-owned list semantics outside the package.

Application-owned boundaries: row actions, keyboard reachability, authorization, navigation intent, and responsive content density.

Deferred intentionally: full upstream registry sync, class-variance-authority as a template dependency, hosted Radix dependency installation, and app-specific row semantics review.
"#,
        ),
    ]
}

fn shadcn_card_templates() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "js/lib/utils.ts",
            r#"export function cn(...inputs: Array<string | false | null | undefined>) {
  return inputs.filter(Boolean).join(" ");
}
"#,
        ),
        (
            "js/ui/card.tsx",
            r#"import * as React from "react";

import { cn } from "../../lib/utils";

function Card({
  className,
  size = "default",
  ...props
}: React.ComponentProps<"div"> & { size?: "default" | "sm" }) {
  return (
    <div
      data-slot="card"
      data-size={size}
      className={cn("cn-card group/card flex flex-col", className)}
      {...props}
    />
  );
}

function CardHeader({ className, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="card-header"
      className={cn(
        "cn-card-header group/card-header @container/card-header grid auto-rows-min items-start has-data-[slot=card-action]:grid-cols-[1fr_auto] has-data-[slot=card-description]:grid-rows-[auto_auto]",
        className,
      )}
      {...props}
    />
  );
}

function CardTitle({ className, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="card-title"
      className={cn("cn-card-title cn-font-heading", className)}
      {...props}
    />
  );
}

function CardDescription({ className, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="card-description"
      className={cn("cn-card-description", className)}
      {...props}
    />
  );
}

function CardAction({ className, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="card-action"
      className={cn(
        "cn-card-action col-start-2 row-span-2 row-start-1 self-start justify-self-end",
        className,
      )}
      {...props}
    />
  );
}

function CardContent({ className, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="card-content"
      className={cn("cn-card-content", className)}
      {...props}
    />
  );
}

function CardFooter({ className, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="card-footer"
      className={cn("cn-card-footer flex items-center", className)}
      {...props}
    />
  );
}

export {
  Card,
  CardHeader,
  CardFooter,
  CardTitle,
  CardAction,
  CardDescription,
  CardContent,
};
"#,
        ),
        (
            "js/ui/README.md",
            r#"# DX Forge UI Components: Card

This source-owned UI Components card package was materialized by DX Forge.
Compatibility id: `shadcn/ui/card`.
Upstream provenance: shadcn-ui v4.
It exposes the Card, header, title, action, description, content, and footer primitives without requiring `node_modules` lifecycle execution.
"#,
        ),
    ]
}

fn shadcn_alert_templates() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "js/lib/utils.ts",
            r#"export function cn(...inputs: Array<string | false | null | undefined>) {
  return inputs.filter(Boolean).join(" ");
}
"#,
        ),
        (
            "js/ui/alert.tsx",
            r#"import * as React from "react";

import { cn } from "../../lib/utils";

type AlertVariant = "default" | "destructive";

const variants: Record<AlertVariant, string> = {
  default: "cn-alert-variant-default",
  destructive: "cn-alert-variant-destructive",
};

function Alert({
  className,
  variant = "default",
  ...props
}: React.ComponentProps<"div"> & { variant?: AlertVariant }) {
  return (
    <div
      data-slot="alert"
      data-variant={variant}
      role="alert"
      className={cn(
        "cn-alert relative grid w-full grid-cols-[0_1fr] items-start has-[>svg]:grid-cols-[calc(var(--spacing)*4)_1fr]",
        variants[variant],
        className,
      )}
      {...props}
    />
  );
}

function AlertTitle({ className, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="alert-title"
      className={cn("cn-alert-title col-start-2 line-clamp-1 min-h-4 font-medium tracking-normal", className)}
      {...props}
    />
  );
}

function AlertDescription({ className, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="alert-description"
      className={cn("cn-alert-description col-start-2 grid justify-items-start gap-1 text-sm", className)}
      {...props}
    />
  );
}

export { Alert, AlertTitle, AlertDescription };
"#,
        ),
        (
            "js/ui/README.md",
            r#"# DX Forge UI Components: Alert

This source-owned UI Components alert package was materialized by DX Forge.
Compatibility id: `shadcn/ui/alert`.
Upstream provenance: shadcn-ui v4.
It preserves the Alert, AlertTitle, AlertDescription, variant, role, data-slot, data-variant, and local `cn` helper API shape while keeping app-owned severity language outside the package.

Deferred intentionally: full upstream registry sync, icon package installation, and product-specific alert policy.
"#,
        ),
    ]
}

fn shadcn_avatar_templates() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "js/lib/utils.ts",
            r#"export function cn(...inputs: Array<string | false | null | undefined>) {
  return inputs.filter(Boolean).join(" ");
}
"#,
        ),
        (
            "js/ui/avatar.tsx",
            r#"import * as React from "react";

import { cn } from "../../lib/utils";

function Avatar({ className, ...props }: React.ComponentProps<"span">) {
  return (
    <span
      data-slot="avatar"
      className={cn("cn-avatar relative flex size-8 shrink-0 overflow-hidden rounded-full", className)}
      {...props}
    />
  );
}

function AvatarImage({ className, alt = "", ...props }: React.ComponentProps<"img">) {
  return (
    <img
      data-slot="avatar-image"
      alt={alt}
      className={cn("cn-avatar-image aspect-square size-full object-cover", className)}
      {...props}
    />
  );
}

function AvatarFallback({ className, ...props }: React.ComponentProps<"span">) {
  return (
    <span
      data-slot="avatar-fallback"
      className={cn("cn-avatar-fallback flex size-full items-center justify-center rounded-full bg-muted", className)}
      {...props}
    />
  );
}

export { Avatar, AvatarImage, AvatarFallback };
"#,
        ),
        (
            "js/ui/README.md",
            r#"# DX Forge UI Components: Avatar

This source-owned UI Components avatar package was materialized by DX Forge.
Compatibility id: `shadcn/ui/avatar`.
Upstream provenance: shadcn-ui v4.
It preserves the Avatar, AvatarImage, AvatarFallback, data-slot, and local `cn` helper API shape while keeping app-owned profile loading, fallback initials, and image policy outside the package.

Deferred intentionally: full upstream registry sync, hosted Radix dependency installation, and product-specific identity policy.
"#,
        ),
    ]
}

fn shadcn_skeleton_templates() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "js/lib/utils.ts",
            r#"export function cn(...inputs: Array<string | false | null | undefined>) {
  return inputs.filter(Boolean).join(" ");
}
"#,
        ),
        (
            "js/ui/skeleton.tsx",
            r#"import * as React from "react";

import { cn } from "../../lib/utils";

function Skeleton({ className, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="skeleton"
      className={cn("cn-skeleton animate-pulse rounded-md bg-accent", className)}
      {...props}
    />
  );
}

export { Skeleton };
"#,
        ),
        (
            "js/ui/README.md",
            r#"# DX Forge UI Components: Skeleton

This source-owned UI Components skeleton package was materialized by DX Forge.
Compatibility id: `shadcn/ui/skeleton`.
Upstream provenance: shadcn-ui v4.
It preserves the Skeleton export, data-slot marker, and local `cn` helper API shape while keeping app-owned loading layout and perceived-performance policy outside the package.

Deferred intentionally: full upstream registry sync and product-specific loading-state copy review.
"#,
        ),
    ]
}

fn shadcn_input_templates() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "js/lib/utils.ts",
            r#"export function cn(...inputs: Array<string | false | null | undefined>) {
  return inputs.filter(Boolean).join(" ");
}
"#,
        ),
        (
            "js/ui/input.tsx",
            r#"import * as React from "react";

import { cn } from "../../lib/utils";

function Input({ className, type, ...props }: React.ComponentProps<"input">) {
  return (
    <input
      type={type}
      data-slot="input"
      className={cn(
        "cn-input w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50",
        className,
      )}
      {...props}
    />
  );
}

export { Input };
"#,
        ),
    ]
}

fn shadcn_textarea_templates() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "js/lib/utils.ts",
            r#"export function cn(...inputs: Array<string | false | null | undefined>) {
  return inputs.filter(Boolean).join(" ");
}
"#,
        ),
        (
            "js/ui/textarea.tsx",
            r#"import * as React from "react";

import { cn } from "../../lib/utils";

function Textarea({ className, ...props }: React.ComponentProps<"textarea">) {
  return (
    <textarea
      data-slot="textarea"
      className={cn(
        "cn-textarea flex field-sizing-content min-h-16 w-full outline-none placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50",
        className,
      )}
      {...props}
    />
  );
}

export { Textarea };
"#,
        ),
    ]
}

fn icon_search_templates() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "js/lib/icons.ts",
            r#"export type DxIconProps = {
  size?: number | string;
  title?: string;
  className?: string;
  strokeWidth?: number | string;
  role?: string;
  "aria-hidden"?: boolean | "true" | "false";
  [attribute: string]: string | number | boolean | undefined;
};

export function iconAttrs({
  size = 24,
  className,
  strokeWidth = 2,
  titleId,
}: {
  size?: number | string;
  className?: string;
  strokeWidth?: number | string;
  titleId?: string;
}) {
  return {
    xmlns: "http://www.w3.org/2000/svg",
    width: size,
    height: size,
    viewBox: "0 0 24 24",
    fill: "none",
    stroke: "currentColor",
    strokeWidth,
    strokeLinecap: "round",
    strokeLinejoin: "round",
    className,
    role: titleId ? "img" : undefined,
    "aria-hidden": titleId ? undefined : true,
    "aria-labelledby": titleId,
  };
}
"#,
        ),
        (
            "js/icons/search.tsx",
            r#"import { iconAttrs, type DxIconProps } from "../../lib/icons";

export function SearchIcon({
  size = 24,
  title,
  className,
  strokeWidth = 2,
  ...props
}: DxIconProps) {
  const titleId = title ? "dx-search-icon-title" : undefined;

  return (
    <svg
      {...iconAttrs({ size, className, strokeWidth, titleId })}
      {...props}
    >
      {title ? <title id={titleId}>{title}</title> : null}
      <circle cx="11" cy="11" r="8" />
      <path d="m21 21-4.3-4.3" />
    </svg>
  );
}
"#,
        ),
        (
            "js/icons/README.md",
            r#"# DX Forge Search icon

This selected icon was materialized by DX Forge as editable project source.
It installs only the Search icon plus its tiny local helper and does not create `node_modules`.
"#,
        ),
    ]
}

fn better_auth_google_provider_templates() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "js/auth/better-auth/providers/google/config.ts",
            r#"export type DxGoogleOAuthEnv = Record<string, string | undefined>;

export type DxGoogleOAuthConfig = {
  clientId: string;
  clientSecret: string;
  redirectUri: string;
  scopes: string[];
  stateCookieName: string;
  allowedRedirectOrigin?: string;
};

const DEFAULT_SCOPES = ["openid", "email", "profile"];

export function defaultGoogleOAuthEnv(): DxGoogleOAuthEnv {
  return (globalThis as unknown as { process?: { env?: DxGoogleOAuthEnv } }).process?.env ?? {};
}

export function readGoogleOAuthConfig(env: DxGoogleOAuthEnv = defaultGoogleOAuthEnv()): DxGoogleOAuthConfig {
  const clientId = requiredEnv(env, "GOOGLE_CLIENT_ID");
  const clientSecret = requiredEnv(env, "GOOGLE_CLIENT_SECRET");
  const redirectUri = requiredEnv(env, "GOOGLE_REDIRECT_URI");
  const scopes = (env.GOOGLE_OAUTH_SCOPES ?? DEFAULT_SCOPES.join(" "))
    .split(/[,\s]+/)
    .map((scope) => scope.trim())
    .filter(Boolean);

  return {
    clientId,
    clientSecret,
    redirectUri,
    scopes: scopes.length > 0 ? scopes : DEFAULT_SCOPES,
    stateCookieName: env.DX_GOOGLE_STATE_COOKIE ?? "dx_google_oauth_state",
    allowedRedirectOrigin: env.DX_GOOGLE_ALLOWED_REDIRECT_ORIGIN,
  };
}

export function buildGoogleAuthorizationUrl(config: DxGoogleOAuthConfig, state: string): string {
  const url = new URL("https://accounts.google.com/o/oauth2/v2/auth");
  url.searchParams.set("client_id", config.clientId);
  url.searchParams.set("redirect_uri", config.redirectUri);
  url.searchParams.set("response_type", "code");
  url.searchParams.set("scope", config.scopes.join(" "));
  url.searchParams.set("state", state);
  url.searchParams.set("access_type", "offline");
  url.searchParams.set("prompt", "consent");
  return url.toString();
}

export function createGoogleOAuthState(bytes = 24): string {
  const random = new Uint8Array(bytes);
  globalThis.crypto.getRandomValues(random);
  return Array.from(random, (value) => value.toString(16).padStart(2, "0")).join("");
}

export function serializeGoogleStateCookie(name: string, state: string): string {
  return `${name}=${state}; HttpOnly; Secure; SameSite=Lax; Path=/; Max-Age=600`;
}

export function parseCookies(header: string | null): Record<string, string> {
  return Object.fromEntries(
    (header ?? "")
      .split(";")
      .map((part) => part.trim())
      .filter(Boolean)
      .map((part) => {
        const index = part.indexOf("=");
        return index === -1 ? [part, ""] : [part.slice(0, index), part.slice(index + 1)];
      }),
  );
}

export function assertSafeRedirect(requestUrl: URL, config: DxGoogleOAuthConfig): void {
  if (!config.allowedRedirectOrigin) {
    return;
  }
  if (requestUrl.origin !== config.allowedRedirectOrigin) {
    throw new Error(`Google OAuth callback origin ${requestUrl.origin} is not allowed`);
  }
}

function requiredEnv(env: DxGoogleOAuthEnv, name: string): string {
  const value = env[name]?.trim();
  if (!value) {
    throw new Error(`Missing required Google OAuth env var: ${name}`);
  }
  return value;
}
"#,
        ),
        (
            "js/auth/better-auth/providers/google/route.ts",
            r#"import {
  buildGoogleAuthorizationUrl,
  createGoogleOAuthState,
  defaultGoogleOAuthEnv,
  readGoogleOAuthConfig,
  serializeGoogleStateCookie,
  type DxGoogleOAuthEnv,
} from "./config";

export function handleGoogleAuthStart(
  _request: Request,
  env: DxGoogleOAuthEnv = defaultGoogleOAuthEnv(),
): Response {
  const config = readGoogleOAuthConfig(env);
  const state = createGoogleOAuthState();
  const response = Response.redirect(buildGoogleAuthorizationUrl(config, state), 302);
  response.headers.append("Set-Cookie", serializeGoogleStateCookie(config.stateCookieName, state));
  return response;
}

export const GET = handleGoogleAuthStart;
"#,
        ),
        (
            "js/auth/better-auth/providers/google/callback.ts",
            r#"import {
  assertSafeRedirect,
  defaultGoogleOAuthEnv,
  parseCookies,
  readGoogleOAuthConfig,
  type DxGoogleOAuthEnv,
} from "./config";

export type DxGoogleTokenResponse = {
  access_token?: string;
  expires_in?: number;
  id_token?: string;
  refresh_token?: string;
  scope?: string;
  token_type?: string;
  error?: string;
  error_description?: string;
};

export async function handleGoogleAuthCallback(
  request: Request,
  env: DxGoogleOAuthEnv = defaultGoogleOAuthEnv(),
  fetcher: typeof fetch = fetch,
): Promise<Response> {
  const config = readGoogleOAuthConfig(env);
  const url = new URL(request.url);
  assertSafeRedirect(url, config);

  const error = url.searchParams.get("error");
  if (error) {
    return json({ ok: false, error }, 400);
  }

  const code = url.searchParams.get("code");
  const state = url.searchParams.get("state");
  const cookies = parseCookies(request.headers.get("cookie"));
  if (!code || !state || cookies[config.stateCookieName] !== state) {
    return json({ ok: false, error: "invalid_google_oauth_state" }, 400);
  }

  const token = await exchangeGoogleCode(config, code, fetcher);
  if (token.error) {
    return json({ ok: false, error: token.error, description: token.error_description }, 400);
  }

  return json({
    ok: true,
    token,
    next: "Store this token response in your application session layer.",
  });
}

export const GET = handleGoogleAuthCallback;

async function exchangeGoogleCode(
  config: ReturnType<typeof readGoogleOAuthConfig>,
  code: string,
  fetcher: typeof fetch,
): Promise<DxGoogleTokenResponse> {
  const body = new URLSearchParams({
    client_id: config.clientId,
    client_secret: config.clientSecret,
    code,
    grant_type: "authorization_code",
    redirect_uri: config.redirectUri,
  });
  const response = await fetcher("https://oauth2.googleapis.com/token", {
    method: "POST",
    headers: { "content-type": "application/x-www-form-urlencoded" },
    body,
  });
  return response.json() as Promise<DxGoogleTokenResponse>;
}

function json(body: unknown, status = 200): Response {
  return new Response(JSON.stringify(body), {
    status,
    headers: { "content-type": "application/json; charset=utf-8" },
  });
}
"#,
        ),
        (
            "js/auth/better-auth/providers/google/.env.example",
            r#"# Google OAuth values required by the Authentication package Google provider surface
GOOGLE_CLIENT_ID=
GOOGLE_CLIENT_SECRET=
GOOGLE_REDIRECT_URI=http://localhost:3000/authentication/google/callback
GOOGLE_OAUTH_SCOPES=openid email profile
DX_GOOGLE_STATE_COOKIE=dx_google_oauth_state
DX_GOOGLE_ALLOWED_REDIRECT_ORIGIN=http://localhost:3000
"#,
        ),
        (
            "js/auth/better-auth/providers/google/README.md",
            r#"# Authentication Google OAuth Provider

This source-owned provider surface belongs to the Authentication Forge package. It provides local Google OAuth starter files without running package-manager lifecycle scripts.

Files are editable project code. Keep the env example in sync with your deployment provider, rotate secrets outside the repo, and connect the callback response to your own session store before production launch.
"#,
        ),
    ]
}

fn auth_better_auth_templates() -> Vec<(&'static str, &'static str)> {
    let mut templates = vec![
        (
            "js/auth/better-auth/options.ts",
            r#"import type { BetterAuthOptions } from "better-auth";

export type DxBetterAuthEnv = Record<string, string | undefined>;

export type DxBetterAuthRuntime = {
  appName: string;
  baseURL: string;
  secret: string;
  trustedOrigins: string[];
  socialProviders: NonNullable<BetterAuthOptions["socialProviders"]>;
  emailAndPassword: {
    enabled: boolean;
  };
};

const DEFAULT_APP_NAME = "DX";
const DEFAULT_BASE_URL = "http://localhost:3000";

export function defaultBetterAuthEnv(): DxBetterAuthEnv {
  return (globalThis as unknown as { process?: { env?: DxBetterAuthEnv } }).process?.env ?? {};
}

export function readBetterAuthRuntime(
  env: DxBetterAuthEnv = defaultBetterAuthEnv(),
): DxBetterAuthRuntime {
  const baseURL = optionalEnv(env, "BETTER_AUTH_URL") ?? DEFAULT_BASE_URL;
  const trustedOrigins = readList(env, "BETTER_AUTH_TRUSTED_ORIGINS");

  return {
    appName: optionalEnv(env, "BETTER_AUTH_APP_NAME") ?? DEFAULT_APP_NAME,
    baseURL,
    secret: requiredEnv(env, "BETTER_AUTH_SECRET"),
    trustedOrigins: trustedOrigins.length > 0 ? trustedOrigins : [baseURL],
    socialProviders: readSocialProviders(env),
    emailAndPassword: {
      enabled: env.BETTER_AUTH_EMAIL_PASSWORD_ENABLED !== "false",
    },
  };
}

export function readSocialProviders(
  env: DxBetterAuthEnv = defaultBetterAuthEnv(),
): NonNullable<BetterAuthOptions["socialProviders"]> {
  return {
    ...readOAuthProvider(env, "google", "GOOGLE"),
  };
}

function readOAuthProvider(
  env: DxBetterAuthEnv,
  provider: "google",
  prefix: "GOOGLE",
): NonNullable<BetterAuthOptions["socialProviders"]> {
  const clientId = optionalEnv(env, `${prefix}_CLIENT_ID`);
  const clientSecret = optionalEnv(env, `${prefix}_CLIENT_SECRET`);

  if (!clientId && !clientSecret) {
    return {};
  }

  if (!clientId || !clientSecret) {
    throw new Error(`${prefix}_CLIENT_ID and ${prefix}_CLIENT_SECRET must be configured together`);
  }

  return {
    google: {
      clientId,
      clientSecret,
    },
  };
}

function readList(env: DxBetterAuthEnv, key: string): string[] {
  return (env[key] ?? "")
    .split(/[,\s]+/)
    .map((value) => value.trim())
    .filter(Boolean);
}

function optionalEnv(env: DxBetterAuthEnv, key: string): string | undefined {
  const value = env[key]?.trim();
  return value ? value : undefined;
}

function requiredEnv(env: DxBetterAuthEnv, key: string): string {
  const value = optionalEnv(env, key);
  if (!value) {
    throw new Error(`Missing required Authentication env var: ${key}`);
  }
  return value;
}
"#,
        ),
        (
            "js/auth/better-auth/server.ts",
            r#"import { betterAuth, type BetterAuthOptions } from "better-auth";
import { nextCookies } from "better-auth/next-js";

import {
  readBetterAuthRuntime,
  type DxBetterAuthEnv,
} from "./options";

export type DxBetterAuthDatabase = NonNullable<BetterAuthOptions["database"]>;
export type DxBetterAuthPluginList = NonNullable<BetterAuthOptions["plugins"]>;

export type DxBetterAuthServerOptions = {
  database: DxBetterAuthDatabase;
  env?: DxBetterAuthEnv;
  plugins?: DxBetterAuthPluginList;
  overrides?: Partial<
    Omit<
      BetterAuthOptions,
      | "baseURL"
      | "database"
      | "emailAndPassword"
      | "plugins"
      | "secret"
      | "socialProviders"
      | "trustedOrigins"
    >
  >;
};

export function createDxBetterAuth({
  database,
  env,
  plugins = [],
  overrides = {},
}: DxBetterAuthServerOptions) {
  const runtime = readBetterAuthRuntime(env);

  return betterAuth({
    ...overrides,
    appName: runtime.appName,
    baseURL: runtime.baseURL,
    secret: runtime.secret,
    database,
    trustedOrigins: runtime.trustedOrigins,
    emailAndPassword: runtime.emailAndPassword,
    socialProviders: runtime.socialProviders,
    plugins: [...plugins, nextCookies()],
  });
}
"#,
        ),
        (
            "js/auth/better-auth/client.ts",
            r#""use client";

import { createAuthClient } from "better-auth/react";

export type DxBetterAuthClientEnv = Record<string, string | undefined>;
export type DxBetterAuthClientOptions = Parameters<typeof createAuthClient>[0];

export function defaultBetterAuthClientEnv(): DxBetterAuthClientEnv {
  return (globalThis as unknown as { process?: { env?: DxBetterAuthClientEnv } }).process?.env ?? {};
}

export function readBetterAuthClientBaseURL(
  env: DxBetterAuthClientEnv = defaultBetterAuthClientEnv(),
): string | undefined {
  return env.NEXT_PUBLIC_BETTER_AUTH_URL?.trim() || env.BETTER_AUTH_URL?.trim() || undefined;
}

export function createDxAuthClient(options?: DxBetterAuthClientOptions) {
  const baseURL = options?.baseURL ?? readBetterAuthClientBaseURL();

  return createAuthClient({
    ...options,
    ...(baseURL ? { baseURL } : {}),
  });
}

export const authClient = createDxAuthClient();
export const {
  signIn,
  signUp,
  signOut,
  useSession,
  getSession,
  listSessions,
  revokeSession,
  revokeOtherSessions,
  revokeSessions,
  requestPasswordReset,
  resetPassword,
  changePassword,
  sendVerificationEmail,
  listAccounts,
  linkSocial,
  unlinkAccount,
  getAccessToken,
  updateUser,
  changeEmail,
  deleteUser,
} = authClient;
"#,
        ),
        (
            "js/auth/better-auth/email-password.ts",
            r#""use client";

import { signIn, signUp } from "./client";

export type DxBetterAuthEmailSignInInput = {
  email: string;
  password: string;
  rememberMe?: boolean;
  callbackURL?: string;
};

export type DxBetterAuthEmailSignUpInput = {
  email: string;
  name: string;
  password: string;
  image?: string;
  callbackURL?: string;
};

export type DxBetterAuthEmailSignInResult = Awaited<
  ReturnType<typeof signIn.email>
>;

export type DxBetterAuthEmailSignUpResult = Awaited<
  ReturnType<typeof signUp.email>
>;

export async function signInDxBetterAuthEmail(
  input: DxBetterAuthEmailSignInInput,
): Promise<DxBetterAuthEmailSignInResult> {
  const email = normalizeDxBetterAuthEmail(input.email);
  const password = readDxBetterAuthPassword(input.password);

  return signIn.email({
    email,
    password,
    rememberMe: input.rememberMe ?? true,
    ...(input.callbackURL ? { callbackURL: input.callbackURL } : {}),
  });
}

export async function signUpDxBetterAuthEmail(
  input: DxBetterAuthEmailSignUpInput,
): Promise<DxBetterAuthEmailSignUpResult> {
  const email = normalizeDxBetterAuthEmail(input.email);
  const password = readDxBetterAuthPassword(input.password);
  const name = input.name.trim();

  if (!name) {
    throw new Error("DX Authentication sign-up name is required");
  }

  return signUp.email({
    email,
    name,
    password,
    ...(input.image ? { image: input.image } : {}),
    ...(input.callbackURL ? { callbackURL: input.callbackURL } : {}),
  });
}

export function normalizeDxBetterAuthEmail(email: string): string {
  const normalized = email.trim();

  if (!normalized || !normalized.includes("@")) {
    throw new Error("DX Authentication email address is required");
  }

  return normalized;
}

function readDxBetterAuthPassword(password: string): string {
  if (!password) {
    throw new Error("DX Authentication password is required");
  }

  return password;
}
"#,
        ),
        (
            "js/auth/better-auth/social.ts",
            r#""use client";

import { signIn } from "./client";

export type DxBetterAuthSocialProvider = "google";

export type DxBetterAuthSocialIdToken = {
  token: string;
  nonce?: string;
  accessToken?: string;
  refreshToken?: string;
  expiresAt?: number;
  user?: {
    name?: {
      firstName?: string;
      lastName?: string;
    };
    email?: string;
  };
};

export type DxBetterAuthSocialSignInInput = {
  provider: DxBetterAuthSocialProvider;
  callbackURL?: string;
  errorCallbackURL?: string;
  newUserCallbackURL?: string;
  disableRedirect?: boolean;
  scopes?: string[];
  requestSignUp?: boolean;
  loginHint?: string;
  idToken?: DxBetterAuthSocialIdToken;
  additionalData?: Record<string, unknown>;
};

export type DxBetterAuthSocialSignInResult = Awaited<
  ReturnType<typeof signIn.social>
>;

export async function signInDxBetterAuthSocial(
  input: DxBetterAuthSocialSignInInput,
): Promise<DxBetterAuthSocialSignInResult> {
  return signIn.social({
    provider: readDxBetterAuthSocialProvider(input.provider),
    ...(input.callbackURL ? { callbackURL: input.callbackURL } : {}),
    ...(input.errorCallbackURL
      ? { errorCallbackURL: input.errorCallbackURL }
      : {}),
    ...(input.newUserCallbackURL
      ? { newUserCallbackURL: input.newUserCallbackURL }
      : {}),
    ...(input.disableRedirect === undefined
      ? {}
      : { disableRedirect: input.disableRedirect }),
    ...(input.scopes?.length ? { scopes: input.scopes } : {}),
    ...(input.requestSignUp === undefined
      ? {}
      : { requestSignUp: input.requestSignUp }),
    ...(input.loginHint ? { loginHint: input.loginHint } : {}),
    ...(input.idToken ? { idToken: input.idToken } : {}),
    ...(input.additionalData ? { additionalData: input.additionalData } : {}),
  });
}

export function readDxBetterAuthSocialProvider(
  provider: DxBetterAuthSocialProvider,
): DxBetterAuthSocialProvider {
  if (provider !== "google") {
    throw new Error("DX Authentication social provider must be google");
  }

  return provider;
}
"#,
        ),
        (
            "js/auth/better-auth/accounts.ts",
            r#""use client";

import {
  getAccessToken,
  linkSocial,
  listAccounts,
  unlinkAccount,
} from "./client";
import {
  readDxBetterAuthSocialProvider,
  type DxBetterAuthSocialIdToken,
  type DxBetterAuthSocialProvider,
} from "./social";

export type DxBetterAuthLinkedAccountsResult = Awaited<
  ReturnType<typeof listAccounts>
>;

export type DxBetterAuthAccountLinkInput = {
  provider: DxBetterAuthSocialProvider;
  callbackURL?: string;
  errorCallbackURL?: string;
  disableRedirect?: boolean;
  scopes?: string[];
  requestSignUp?: boolean;
  idToken?: Omit<DxBetterAuthSocialIdToken, "expiresAt" | "user"> & {
    scopes?: string[];
  };
  additionalData?: Record<string, unknown>;
};

export type DxBetterAuthAccountUnlinkInput = {
  providerId: DxBetterAuthSocialProvider;
  accountId?: string;
};

export type DxBetterAuthAccessTokenInput = {
  providerId: DxBetterAuthSocialProvider;
  accountId?: string;
};

export type DxBetterAuthAccountLinkResult = Awaited<
  ReturnType<typeof linkSocial>
>;

export type DxBetterAuthAccountUnlinkResult = Awaited<
  ReturnType<typeof unlinkAccount>
>;

export type DxBetterAuthAccessTokenResult = Awaited<
  ReturnType<typeof getAccessToken>
>;

export async function listDxBetterAuthAccounts(): Promise<DxBetterAuthLinkedAccountsResult> {
  return listAccounts();
}

export async function linkDxBetterAuthSocialAccount(
  input: DxBetterAuthAccountLinkInput,
): Promise<DxBetterAuthAccountLinkResult> {
  return linkSocial({
    provider: readDxBetterAuthSocialProvider(input.provider),
    ...(input.callbackURL ? { callbackURL: input.callbackURL } : {}),
    ...(input.errorCallbackURL
      ? { errorCallbackURL: input.errorCallbackURL }
      : {}),
    ...(input.disableRedirect === undefined
      ? {}
      : { disableRedirect: input.disableRedirect }),
    ...(input.scopes?.length ? { scopes: input.scopes } : {}),
    ...(input.requestSignUp === undefined
      ? {}
      : { requestSignUp: input.requestSignUp }),
    ...(input.idToken ? { idToken: input.idToken } : {}),
    ...(input.additionalData ? { additionalData: input.additionalData } : {}),
  });
}

export async function unlinkDxBetterAuthSocialAccount(
  input: DxBetterAuthAccountUnlinkInput,
): Promise<DxBetterAuthAccountUnlinkResult> {
  return unlinkAccount({
    providerId: readDxBetterAuthSocialProvider(input.providerId),
    ...(input.accountId ? { accountId: input.accountId } : {}),
  });
}

export async function getDxBetterAuthAccessToken(
  input: DxBetterAuthAccessTokenInput,
): Promise<DxBetterAuthAccessTokenResult> {
  return getAccessToken({
    providerId: readDxBetterAuthSocialProvider(input.providerId),
    ...(input.accountId ? { accountId: input.accountId } : {}),
  });
}
"#,
        ),
        (
            "js/auth/better-auth/profile.ts",
            r#""use client";

import { changeEmail, updateUser } from "./client";
import { normalizeDxBetterAuthEmail } from "./email-password";

export type DxBetterAuthProfileUpdateInput = {
  name?: string;
  image?: string | null;
  additionalFields?: Record<string, unknown>;
};

export type DxBetterAuthEmailChangeInput = {
  newEmail: string;
  callbackURL?: string;
};

export type DxBetterAuthProfileUpdateResult = Awaited<
  ReturnType<typeof updateUser>
>;

export type DxBetterAuthEmailChangeResult = Awaited<
  ReturnType<typeof changeEmail>
>;

export async function updateDxBetterAuthUserProfile(
  input: DxBetterAuthProfileUpdateInput,
): Promise<DxBetterAuthProfileUpdateResult> {
  const profile = readDxBetterAuthProfileUpdate(input);

  return updateUser({
    ...(profile.name !== undefined ? { name: profile.name } : {}),
    ...(profile.image !== undefined ? { image: profile.image } : {}),
    ...profile.additionalFields,
  });
}

export async function changeDxBetterAuthEmail(
  input: DxBetterAuthEmailChangeInput,
): Promise<DxBetterAuthEmailChangeResult> {
  return changeEmail({
    newEmail: normalizeDxBetterAuthEmail(input.newEmail),
    ...(input.callbackURL ? { callbackURL: input.callbackURL } : {}),
  });
}

function readDxBetterAuthProfileUpdate(input: DxBetterAuthProfileUpdateInput): {
  name?: string;
  image?: string | null;
  additionalFields: Record<string, unknown>;
} {
  const trimmedName = input.name?.trim();
  const name = trimmedName || undefined;
  const image = readDxBetterAuthProfileImage(input.image);
  const additionalFields = readDxBetterAuthAdditionalProfileFields(
    input.additionalFields,
  );

  if (
    name === undefined &&
    image === undefined &&
    Object.keys(additionalFields).length === 0
  ) {
    throw new Error("DX Authentication profile update requires a field to update");
  }

  return {
    ...(name !== undefined ? { name } : {}),
    ...(image !== undefined ? { image } : {}),
    additionalFields,
  };
}

function readDxBetterAuthProfileImage(
  image: string | null | undefined,
): string | null | undefined {
  if (image === undefined || image === null) {
    return image;
  }

  const safeImage = image.trim();

  return safeImage || null;
}

function readDxBetterAuthAdditionalProfileFields(
  additionalFields: Record<string, unknown> | undefined,
): Record<string, unknown> {
  if (!additionalFields) {
    return {};
  }

  if (Object.prototype.hasOwnProperty.call(additionalFields, "email")) {
    throw new Error(
      "DX Authentication profile updates cannot change email; use changeDxBetterAuthEmail",
    );
  }

  return additionalFields;
}
"#,
        ),
        (
            "js/auth/better-auth/account-deletion.ts",
            r#""use client";

import { deleteUser } from "./client";

export const DX_BETTER_AUTH_DELETE_ACCOUNT_CONFIRMATION =
  "delete my account" as const;

export type DxBetterAuthDeleteAccountInput = {
  confirmation: typeof DX_BETTER_AUTH_DELETE_ACCOUNT_CONFIRMATION;
  callbackURL?: string;
  password?: string;
  token?: string;
  allowFreshSessionDelete?: boolean;
};

export type DxBetterAuthDeleteAccountResult = Awaited<
  ReturnType<typeof deleteUser>
>;

export async function deleteDxBetterAuthAccount(
  input: DxBetterAuthDeleteAccountInput,
): Promise<DxBetterAuthDeleteAccountResult> {
  readDxBetterAuthDeleteConfirmation(input.confirmation);

  const safePassword = readOptionalDxBetterAuthSecret(input.password);
  const safeToken = readOptionalDxBetterAuthSecret(input.token);

  if (!safePassword && !safeToken && input.allowFreshSessionDelete !== true) {
    throw new Error(
      "DX Authentication account deletion requires a password, token, or explicit fresh-session opt-in",
    );
  }

  return deleteUser({
    ...(input.callbackURL ? { callbackURL: input.callbackURL } : {}),
    ...(safePassword ? { password: safePassword } : {}),
    ...(safeToken ? { token: safeToken } : {}),
  });
}

function readDxBetterAuthDeleteConfirmation(
  confirmation: DxBetterAuthDeleteAccountInput["confirmation"],
): void {
  if (confirmation !== DX_BETTER_AUTH_DELETE_ACCOUNT_CONFIRMATION) {
    throw new Error(
      `Type "${DX_BETTER_AUTH_DELETE_ACCOUNT_CONFIRMATION}" to delete the account`,
    );
  }
}

function readOptionalDxBetterAuthSecret(
  value: string | undefined,
): string | undefined {
  const safeValue = value?.trim();

  return safeValue || undefined;
}
"#,
        ),
        (
            "js/auth/better-auth/account-security.ts",
            r#""use client";

import {
  changePassword,
  requestPasswordReset,
  resetPassword,
  sendVerificationEmail,
} from "./client";
import { normalizeDxBetterAuthEmail } from "./email-password";

export type DxBetterAuthPasswordResetRequestInput = {
  email: string;
  redirectTo?: string;
};

export type DxBetterAuthPasswordResetInput = {
  token: string;
  newPassword: string;
};

export type DxBetterAuthPasswordChangeInput = {
  currentPassword: string;
  newPassword: string;
  revokeOtherSessions?: boolean;
};

export type DxBetterAuthVerificationEmailInput = {
  email: string;
  callbackURL?: string;
};

export type DxBetterAuthPasswordResetRequestResult = Awaited<
  ReturnType<typeof requestPasswordReset>
>;

export type DxBetterAuthPasswordResetResult = Awaited<
  ReturnType<typeof resetPassword>
>;

export type DxBetterAuthPasswordChangeResult = Awaited<
  ReturnType<typeof changePassword>
>;

export type DxBetterAuthVerificationEmailResult = Awaited<
  ReturnType<typeof sendVerificationEmail>
>;

export async function requestDxBetterAuthPasswordReset(
  input: DxBetterAuthPasswordResetRequestInput,
): Promise<DxBetterAuthPasswordResetRequestResult> {
  const email = normalizeDxBetterAuthEmail(input.email);

  return requestPasswordReset({
    email,
    ...(input.redirectTo ? { redirectTo: input.redirectTo } : {}),
  });
}

export async function resetDxBetterAuthPassword(
  input: DxBetterAuthPasswordResetInput,
): Promise<DxBetterAuthPasswordResetResult> {
  const token = readDxBetterAuthResetToken(input.token);
  const newPassword = readDxBetterAuthPasswordValue(
    input.newPassword,
    "new password",
  );

  return resetPassword({
    token,
    newPassword,
  });
}

export async function changeDxBetterAuthPassword(
  input: DxBetterAuthPasswordChangeInput,
): Promise<DxBetterAuthPasswordChangeResult> {
  const currentPassword = readDxBetterAuthPasswordValue(
    input.currentPassword,
    "current password",
  );
  const newPassword = readDxBetterAuthPasswordValue(
    input.newPassword,
    "new password",
  );

  return changePassword({
    currentPassword,
    newPassword,
    revokeOtherSessions: input.revokeOtherSessions ?? true,
  });
}

export async function sendDxBetterAuthVerificationEmail(
  input: DxBetterAuthVerificationEmailInput,
): Promise<DxBetterAuthVerificationEmailResult> {
  const email = normalizeDxBetterAuthEmail(input.email);

  return sendVerificationEmail({
    email,
    ...(input.callbackURL ? { callbackURL: input.callbackURL } : {}),
  });
}

function readDxBetterAuthPasswordValue(
  value: string,
  label: string,
): string {
  if (!value) {
    throw new Error(`DX Authentication ${label} is required`);
  }

  return value;
}

function readDxBetterAuthResetToken(token: string): string {
  const safeToken = token.trim();

  if (!safeToken) {
    throw new Error("DX Authentication reset token is required");
  }

  return safeToken;
}
"#,
        ),
        (
            "js/auth/better-auth/route.ts",
            r#"import { toNextJsHandler } from "better-auth/next-js";

import type { createDxBetterAuth } from "./server";

export type DxBetterAuth = ReturnType<typeof createDxBetterAuth>;

export function createDxBetterAuthRouteHandlers(auth: DxBetterAuth) {
  return toNextJsHandler(auth);
}
"#,
        ),
        (
            "js/auth/better-auth/session.ts",
            r#"import { headers } from "next/headers";

import type { DxBetterAuth } from "./route";

export type DxBetterAuthSession = Awaited<
  ReturnType<DxBetterAuth["api"]["getSession"]>
>;

export async function getDxBetterAuthSession(
  auth: DxBetterAuth,
): Promise<DxBetterAuthSession> {
  return auth.api.getSession({
    headers: await headers(),
  });
}

export async function requireDxBetterAuthSession(
  auth: DxBetterAuth,
): Promise<NonNullable<DxBetterAuthSession>> {
  const session = await getDxBetterAuthSession(auth);

  if (!session) {
    throw new Error("DX Authentication session required");
  }

  return session;
}
"#,
        ),
        (
            "js/auth/better-auth/session-management.ts",
            r#"import { headers } from "next/headers";

import type { DxBetterAuth } from "./route";

export type DxBetterAuthSessionList = Awaited<
  ReturnType<DxBetterAuth["api"]["listSessions"]>
>;

export type DxBetterAuthSessionMutation = Awaited<
  ReturnType<DxBetterAuth["api"]["revokeSession"]>
>;

export async function listDxBetterAuthSessions(
  auth: DxBetterAuth,
): Promise<DxBetterAuthSessionList> {
  return auth.api.listSessions({
    headers: await headers(),
  });
}

export async function revokeDxBetterAuthSession(
  auth: DxBetterAuth,
  token: string,
): Promise<DxBetterAuthSessionMutation> {
  const safeToken = token.trim();

  if (!safeToken) {
    throw new Error("DX Authentication session token is required");
  }

  return auth.api.revokeSession({
    headers: await headers(),
    body: {
      token: safeToken,
    },
  });
}

export async function revokeOtherDxBetterAuthSessions(
  auth: DxBetterAuth,
): Promise<DxBetterAuthSessionMutation> {
  return auth.api.revokeOtherSessions({
    headers: await headers(),
  });
}

export async function revokeAllDxBetterAuthSessions(
  auth: DxBetterAuth,
): Promise<DxBetterAuthSessionMutation> {
  return auth.api.revokeSessions({
    headers: await headers(),
  });
}
"#,
        ),
        (
            "js/auth/better-auth/dashboard.ts",
            r#"export type BetterAuthDashboardActionId =
  | "read-session"
  | "update-profile"
  | "link-provider"
  | "revoke-other-sessions";

export type BetterAuthDashboardProvider = "github" | "google";

export type BetterAuthDashboardUser = {
  id: string;
  email: string;
  name: string;
  role: string;
};

export type BetterAuthDashboardProfileDraft = {
  name: string;
  email: string;
  provider: BetterAuthDashboardProvider;
};

export type BetterAuthDashboardSessionSnapshot = {
  status: "signed-in" | "signed-out";
  userLabel: string;
  role: string;
  publicApi: readonly string[];
};

export type BetterAuthDashboardProfileRequest = {
  action: "update-profile";
  method: "client";
  publicApi: readonly string[];
  payload: {
    name: string;
    email: string;
  };
};

export type BetterAuthDashboardActionReceipt = {
  receiptId: string;
  status: "ready" | "missing-config";
  message: string;
  action: BetterAuthDashboardActionId;
  requiredEnv: readonly string[];
  publicApi: readonly string[];
  appOwnedBoundary: string;
};

export const betterAuthDashboardPackage = {
  packageId: "auth/better-auth",
  officialName: "Authentication",
  aliases: ["authentication", "better-auth", "auth/betterauth", "auth/better-auth-next"],
  upstreamPackage: "better-auth",
  sourceMirror: "G:/WWW/inspirations/better-auth",
  docsPath: "docs/packages/authentication.md",
  provenance: {
    repository: "https://github.com/better-auth/better-auth",
    sourceSubpath: "packages/better-auth",
    version: "1.6.11",
  },
  exportedFiles: [
    "auth/better-auth/client.ts",
    "auth/better-auth/profile.ts",
    "auth/better-auth/social.ts",
    "auth/better-auth/accounts.ts",
    "auth/better-auth/session.ts",
    "auth/better-auth/session-management.ts",
    "auth/better-auth/dashboard.ts",
    "auth/better-auth/metadata.ts",
  ],
  requiredEnv: [
    "BETTER_AUTH_SECRET",
    "BETTER_AUTH_URL",
    "BETTER_AUTH_TRUSTED_ORIGINS",
    "NEXT_PUBLIC_BETTER_AUTH_URL",
  ],
  appOwnedBoundaries: [
    "database adapter and migrations",
    "session lifetime, cookie, and trusted-origin policy",
    "OAuth provider credentials and callback URLs",
    "profile-field authorization and email verification policy",
    "account linking, token storage, and session revocation UX",
  ],
  receiptPaths: [
    ".dx/forge/receipts/auth-better-auth.json",
    ".dx/forge/docs/launch-companions/auth-session-status.md",
    ".dx/forge/template-readiness/launch-readiness-bundle.json",
  ],
} as const;

export const dxBetterAuthDashboardActions = [
  {
    id: "read-session",
    label: "Read session",
    publicApi: ["useSession()", "auth.api.getSession({ headers })"],
    appOwnedBoundary: "server request headers and cookie refresh policy",
  },
  {
    id: "update-profile",
    label: "Update profile",
    publicApi: ["authClient.updateUser()", "authClient.changeEmail()"],
    appOwnedBoundary: "profile-field authorization and email verification policy",
  },
  {
    id: "link-provider",
    label: "Link provider",
    publicApi: [
      "authClient.signIn.social()",
      "authClient.listAccounts()",
      "authClient.linkSocial()",
    ],
    appOwnedBoundary: "OAuth credentials, callback URLs, scopes, and consent copy",
  },
  {
    id: "revoke-other-sessions",
    label: "Revoke other sessions",
    publicApi: ["authClient.listSessions()", "authClient.revokeOtherSessions()"],
    appOwnedBoundary: "session revocation UX and audit logging",
  },
] as const satisfies readonly {
  id: BetterAuthDashboardActionId;
  label: string;
  publicApi: readonly string[];
  appOwnedBoundary: string;
}[];

export function createDxBetterAuthDashboardSessionSnapshot(
  user: BetterAuthDashboardUser | null | undefined,
): BetterAuthDashboardSessionSnapshot {
  if (!user) {
    return {
      status: "signed-out",
      userLabel: "No local dashboard session",
      role: "guest",
      publicApi: ["useSession()", "auth.api.getSession({ headers })"],
    };
  }

  return {
    status: "signed-in",
    userLabel: `${user.name} <${user.email}>`,
    role: user.role,
    publicApi: ["useSession()", "auth.api.getSession({ headers })"],
  };
}

export function createDxBetterAuthDashboardProfileRequest(
  draft: BetterAuthDashboardProfileDraft,
): BetterAuthDashboardProfileRequest {
  const name = readDashboardName(draft.name);
  const email = readDashboardEmail(draft.email);

  return {
    action: "update-profile",
    method: "client",
    publicApi: ["authClient.updateUser()", "authClient.changeEmail()"],
    payload: {
      name,
      email,
    },
  };
}

export function createDxBetterAuthDashboardActionReceipt({
  actionId,
  draft,
}: {
  actionId: BetterAuthDashboardActionId;
  draft: BetterAuthDashboardProfileDraft;
}): BetterAuthDashboardActionReceipt {
  const action = readDashboardAction(actionId);
  const status = action.id === "read-session" ? "ready" : "missing-config";
  const target = action.id === "link-provider" ? `-${draft.provider}` : "";

  return {
    receiptId: `better-auth-dashboard-${action.id}${target}`,
    status,
    message:
      status === "ready"
        ? "Local dashboard session snapshot is ready; server cookies still belong to the app runtime."
        : `${action.label} is prepared, but Authentication credentials, routes, database policy, and production UX remain app-owned.`,
    action: action.id,
    requiredEnv: betterAuthDashboardPackage.requiredEnv,
    publicApi: action.publicApi,
    appOwnedBoundary: action.appOwnedBoundary,
  };
}

function readDashboardAction(actionId: BetterAuthDashboardActionId) {
  const action = dxBetterAuthDashboardActions.find((item) => item.id === actionId);
  if (!action) {
    throw new Error("Choose a supported Authentication dashboard action.");
  }
  return action;
}

function readDashboardName(value: string) {
  const name = value.trim();
  if (name.length < 2) {
    throw new Error("Enter a profile name with at least 2 characters.");
  }
  return name;
}

function readDashboardEmail(value: string) {
  const email = value.trim().toLowerCase();
  if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email)) {
    throw new Error("Enter a valid Authentication account email.");
  }
  return email;
}
"#,
        ),
        (
            "js/auth/better-auth/metadata.ts",
            r#"export const dxBetterAuthForgePackage = {
  packageId: "auth/better-auth",
  officialName: "Authentication",
  aliases: [
    "authentication",
    "better-auth",
    "auth/betterauth",
    "auth/better-auth-next",
  ],
  upstreamPackage: "better-auth",
  upstreamVersion: "1.6.11",
  sourceMirror: "G:\\WWW\\inspirations\\better-auth",
  docsPath: "docs/packages/authentication.md",
  provenance: {
    repository: "https://github.com/better-auth/better-auth",
    sourceSubpath: "packages/better-auth",
    packageJson:
      "G:\\WWW\\inspirations\\better-auth\\packages\\better-auth\\package.json",
    version: "1.6.11",
  },
  sourceSurface: [
    "betterAuth from better-auth",
    "createAuthClient from better-auth/react",
    "toNextJsHandler and nextCookies from better-auth/next-js",
    "auth.api.getSession({ headers }) for server-side session reads",
    "auth.api.listSessions and auth.api.revokeSession helpers",
    "authClient.listSessions() and authClient.revokeSession() client actions",
    "authClient.signIn.social() helper for Google provider launches",
    "authClient.listAccounts(), linkSocial(), unlinkAccount(), and getAccessToken() helpers",
    "authClient.updateUser() and authClient.changeEmail() helpers",
    "authClient.deleteUser() guarded account deletion helper",
    "authClient.signIn.email() and authClient.signUp.email() helpers",
    "authClient.requestPasswordReset(), resetPassword(), changePassword(), and sendVerificationEmail() helpers",
    "BetterAuthOptions database adapter boundary",
  ],
  dependencies: [
    {
      name: "better-auth",
      version: "^1.6.11",
      required: true,
    },
    {
      name: "next",
      version: "^14.0.0 || ^15.0.0 || ^16.0.0",
      required: false,
    },
  ],
  requiredEnv: [
    "BETTER_AUTH_SECRET",
    "BETTER_AUTH_URL",
    "BETTER_AUTH_TRUSTED_ORIGINS",
    "NEXT_PUBLIC_BETTER_AUTH_URL",
  ],
  optionalEnv: [
    "BETTER_AUTH_APP_NAME",
    "BETTER_AUTH_EMAIL_PASSWORD_ENABLED",
    "GOOGLE_CLIENT_ID",
    "GOOGLE_CLIENT_SECRET",
  ],
  appOwnedBoundaries: [
    "Database adapter and migration policy",
    "Session lifetime, cookie, trusted-origin, and revocation policy",
    "OAuth provider credentials, callback URLs, and consent copy",
    "Email delivery, verification, password reset, and account deletion hooks",
    "Profile field policy, account linking UX, and token storage policy",
  ],
  receiptPaths: {
    package: ".dx/forge/receipts/auth-better-auth.json",
    dashboard: ".dx/forge/docs/launch-companions/auth-session-status.md",
    templateReadiness: ".dx/forge/template-readiness/launch-readiness-bundle.json",
  },
  dxCheckVisibility: {
    schema: "dx.forge.package.dx_check_visibility",
    currentStatus: "present",
    statuses: [
      "present",
      "stale",
      "missing-receipt",
      "blocked",
      "unsupported-surface",
    ],
    receiptPath:
      "examples/template/.dx/forge/receipts/auth-better-auth.json",
    monitoredSurfaces: [
      "authentication-account-workflow",
      "authentication-session-status",
    ],
  },
  dashboardUsage: {
    launchRoute: "examples/template/app/page.tsx",
    launchShell: "examples/template/template-shell.tsx",
    sessionStatus: "examples/template/auth-session-status.tsx",
    componentMarker: "better-auth-account-dashboard-workflow",
    sessionStatusMarker: "better-auth-session-status-panel",
    emailSignUpMarker: "data-dx-auth-interaction=\"email-sign-up\"",
    boundaryReviewMarker: "data-dx-auth-interaction=\"mark-boundary-reviewed\"",
    networkGateMarker: "data-dx-auth-network-state",
    emailSignUpHelper: "signUpDxBetterAuthEmail(input)",
  },
  exportedFiles: [
    "auth/better-auth/options.ts",
    "auth/better-auth/server.ts",
    "auth/better-auth/client.ts",
    "auth/better-auth/email-password.ts",
    "auth/better-auth/social.ts",
    "auth/better-auth/accounts.ts",
    "auth/better-auth/profile.ts",
    "auth/better-auth/account-deletion.ts",
    "auth/better-auth/account-security.ts",
    "auth/better-auth/route.ts",
    "auth/better-auth/session.ts",
    "auth/better-auth/session-management.ts",
    "auth/better-auth/dashboard.ts",
    "auth/better-auth/metadata.ts",
    "auth/better-auth/.env.example",
    "auth/better-auth/README.md",
  ],
  materializedFiles: [
    "auth/better-auth/options.ts",
    "auth/better-auth/server.ts",
    "auth/better-auth/client.ts",
    "auth/better-auth/email-password.ts",
    "auth/better-auth/social.ts",
    "auth/better-auth/accounts.ts",
    "auth/better-auth/profile.ts",
    "auth/better-auth/account-deletion.ts",
    "auth/better-auth/account-security.ts",
    "auth/better-auth/route.ts",
    "auth/better-auth/session.ts",
    "auth/better-auth/session-management.ts",
    "auth/better-auth/dashboard.ts",
    "auth/better-auth/metadata.ts",
    "auth/better-auth/.env.example",
  ],
  discovery: {
    dxAdd: "dx add authentication --write",
    nextRouteHelper: "createDxBetterAuthRouteHandlers(auth)",
    serverSessionHelper: "getDxBetterAuthSession(auth)",
    sessionManagementHelper: "listDxBetterAuthSessions(auth)",
    emailPasswordHelper: "signInDxBetterAuthEmail(input)",
    socialSignInHelper: "signInDxBetterAuthSocial(input)",
    accountLinkingHelper: "linkDxBetterAuthSocialAccount(input)",
    profileHelper: "updateDxBetterAuthUserProfile(input)",
    accountDeletionHelper: "deleteDxBetterAuthAccount(input)",
    accountSecurityHelper: "requestDxBetterAuthPasswordReset(input)",
    clientSessionActions: "listSessions()",
    clientFactory: "createDxAuthClient()",
  },
} as const;

export type DxBetterAuthForgePackage = typeof dxBetterAuthForgePackage;
"#,
        ),
        (
            "js/auth/better-auth/.env.example",
            r#"# Authentication values required by auth/better-auth
BETTER_AUTH_SECRET=
BETTER_AUTH_URL=http://localhost:3000
BETTER_AUTH_TRUSTED_ORIGINS=http://localhost:3000
BETTER_AUTH_APP_NAME=DX
BETTER_AUTH_EMAIL_PASSWORD_ENABLED=true
NEXT_PUBLIC_BETTER_AUTH_URL=http://localhost:3000

# Optional OAuth providers read by options.ts
GOOGLE_CLIENT_ID=
GOOGLE_CLIENT_SECRET=
GOOGLE_REDIRECT_URI=http://localhost:3000/auth/better-auth/callback
"#,
        ),
        (
            "js/auth/better-auth/README.md",
            r#"# Authentication

This package materializes a source-owned Authentication slice based on upstream better-auth APIs for a Next-style launch app without running package-manager lifecycle scripts or creating `node_modules`.

## Owned Surface

- `options.ts` reads `BETTER_AUTH_*` and Google provider env values.
- `server.ts` creates an upstream better-auth server with a caller-provided database adapter.
- `client.ts` creates the React auth client and exports the common client plus session-management actions.
- `email-password.ts` exposes guarded client helpers around upstream better-auth `signIn.email` and `signUp.email`.
- `social.ts` exposes guarded client helpers around upstream better-auth `signIn.social` for Google launches.
- `accounts.ts` exposes guarded client helpers around linked accounts, account linking, unlinking, and OAuth access-token refresh.
- `profile.ts` exposes guarded client helpers around `updateUser` and `changeEmail`.
- `account-deletion.ts` exposes a confirmation-gated helper around `deleteUser`.
- `account-security.ts` exposes guarded client helpers around password reset, password change, and verification email APIs.
- `route.ts` exposes a typed helper around `toNextJsHandler(auth)`.
- `session.ts` exposes server-side session helpers around `auth.api.getSession({ headers })`.
- `session-management.ts` exposes server-side helpers for `listSessions`, `revokeSession`, `revokeOtherSessions`, and `revokeSessions`.
- `metadata.ts` gives DX CLI and host UIs a stable package-discovery record with aliases, source mirror/provenance, exported files, required env, app-owned boundaries, and receipt paths.

## Next Route Example

```ts
import { createDxBetterAuthRouteHandlers } from "@/auth/better-auth/route";
import { createDxBetterAuth } from "@/auth/better-auth/server";
import { database } from "@/server/database";

export const auth = createDxBetterAuth({ database });
export const { GET, POST } = createDxBetterAuthRouteHandlers(auth);
```

## Server Session Example

```ts
import { getDxBetterAuthSession } from "@/auth/better-auth/session";
import { auth } from "@/app/api/auth/[...all]/route";

export async function LaunchAccountMenu() {
  const session = await getDxBetterAuthSession(auth);
  return <span>{session?.user.email ?? "Signed out"}</span>;
}
```

## Session Management Example

```ts
import { listDxBetterAuthSessions } from "@/auth/better-auth/session-management";
import { auth } from "@/app/api/auth/[...all]/route";

export async function LaunchSessionCount() {
  const sessions = await listDxBetterAuthSessions(auth);
  return <span>{sessions.length} active sessions</span>;
}
```

## Client Session Actions Example

```ts
"use client";

import { listSessions, revokeOtherSessions } from "@/auth/better-auth/client";

export async function refreshLaunchSessions() {
  const sessions = await listSessions();
  return sessions;
}

export async function signOutOtherLaunchDevices() {
  await revokeOtherSessions();
}
```

## Email Password Example

```ts
"use client";

import { signInDxBetterAuthEmail } from "@/auth/better-auth/email-password";

export async function signInFromLaunchForm(email: string, password: string) {
  return signInDxBetterAuthEmail({
    email,
    password,
    callbackURL: "/launch",
  });
}
```

## Social Sign-In Example

```ts
"use client";

import { signInDxBetterAuthSocial } from "@/auth/better-auth/social";

export async function signInWithGoogle() {
  return signInDxBetterAuthSocial({
    provider: "google",
    callbackURL: "/launch",
  });
}
```

## Linked Accounts Example

```ts
"use client";

import {
  linkDxBetterAuthSocialAccount,
  listDxBetterAuthAccounts,
} from "@/auth/better-auth/accounts";

export async function refreshLinkedAccounts() {
  return listDxBetterAuthAccounts();
}

export async function linkGoogleForLaunch() {
  return linkDxBetterAuthSocialAccount({
    provider: "google",
    callbackURL: "/launch",
  });
}
```

## Profile Example

```ts
"use client";

import {
  changeDxBetterAuthEmail,
  updateDxBetterAuthUserProfile,
} from "@/auth/better-auth/profile";

export async function updateLaunchProfile(name: string, newEmail?: string) {
  await updateDxBetterAuthUserProfile({ name });

  if (newEmail) {
    await changeDxBetterAuthEmail({
      newEmail,
      callbackURL: "/launch",
    });
  }
}
```

## Account Deletion Example

```ts
"use client";

import {
  deleteDxBetterAuthAccount,
  DX_BETTER_AUTH_DELETE_ACCOUNT_CONFIRMATION,
} from "@/auth/better-auth/account-deletion";

export async function deleteLaunchAccount(password: string) {
  return deleteDxBetterAuthAccount({
    confirmation: DX_BETTER_AUTH_DELETE_ACCOUNT_CONFIRMATION,
    password,
    callbackURL: "/goodbye",
  });
}
```

## Account Security Example

```ts
"use client";

import { requestDxBetterAuthPasswordReset } from "@/auth/better-auth/account-security";

export async function requestLaunchPasswordReset(email: string) {
  return requestDxBetterAuthPasswordReset({
    email,
    redirectTo: "/reset-password",
  });
}
```

Keep `BETTER_AUTH_SECRET` outside source control, review trusted origins before production, and choose the database adapter in application code so the launch template can pair this slice with Drizzle, Turso, Postgres, or another upstream better-auth adapter. Applications still own profile field policy, email-change enablement, verification email delivery/callback URLs, delete-account email delivery, delete-token expiry, before/after deletion hooks, account export UX, and production auth policy.
"#,
        ),
    ];
    templates.extend(better_auth_google_provider_templates());
    templates
}
