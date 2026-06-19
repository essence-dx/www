import { cn } from "../../lib/utils";
import { Label, type LabelProps } from "./label";
import { Separator } from "./separator";
import type { DxElementProps } from "./types";

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

function FieldSet({ className, ...props }: DxElementProps) {
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
}: DxElementProps & { variant?: "legend" | "label" }) {
  return (
    <legend
      data-slot="field-legend"
      data-variant={variant}
      className={cn("cn-field-legend", className)}
      {...props}
    />
  );
}

function FieldGroup({ className, ...props }: DxElementProps) {
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
}: DxElementProps & { orientation?: FieldOrientation }) {
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

function FieldContent({ className, ...props }: DxElementProps) {
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

function FieldLabel({ className, ...props }: LabelProps) {
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

function FieldTitle({ className, ...props }: DxElementProps) {
  return (
    <div
      data-slot="field-label"
      className={cn("cn-field-title flex w-fit items-center", className)}
      {...props}
    />
  );
}

function FieldDescription({ className, ...props }: DxElementProps) {
  return (
    <p
      data-slot="field-description"
      className={cn("cn-field-description", className)}
      {...props}
    />
  );
}

function FieldSeparator({ children, className, ...props }: DxElementProps) {
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

function uniqueFieldErrors(errors?: Array<{ message?: string } | undefined>) {
  if (!errors?.length) {
    return [];
  }

  return [...new Map(errors.map((error) => [error?.message, error])).values()]
    .filter((error) => Boolean(error?.message));
}

function FieldError({
  className,
  children,
  errors,
  ...props
}: DxElementProps & {
  errors?: Array<{ message?: string } | undefined>;
}) {
  const uniqueErrors = uniqueFieldErrors(errors);
  const content = children || uniqueErrors[0]?.message;

  if (!content && uniqueErrors.length === 0) {
    return null;
  }

  return (
    <div
      role="alert"
      data-slot="field-error"
      className={cn("cn-field-error font-normal", className)}
      {...props}
    >
      {children ? children : null}
      {!children && uniqueErrors.length === 1 ? uniqueErrors[0]?.message : null}
      {!children && uniqueErrors.length > 1 ? (
        <ul className="ml-4 flex list-disc flex-col gap-1">
          {uniqueErrors.map((error, index) =>
            error?.message ? <li key={index}>{error.message}</li> : null,
          )}
        </ul>
      ) : null}
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
