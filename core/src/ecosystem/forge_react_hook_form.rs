pub(super) const REACT_HOOK_FORM_VERSION: &str = "7.75.0-dx.0";

pub(super) fn react_hook_form_templates() -> Vec<(&'static str, &'static str)> {
    vec![
        ("js/forms/react-hook-form/form.tsx", RHF_FORM_TSX),
        ("js/forms/react-hook-form/fields.tsx", RHF_FIELDS_TSX),
        ("js/forms/react-hook-form/resolver.ts", RHF_RESOLVER_TS),
        ("js/forms/react-hook-form/example.tsx", RHF_EXAMPLE_TSX),
        ("js/forms/react-hook-form/metadata.ts", RHF_METADATA_TS),
        ("js/forms/react-hook-form/README.md", RHF_README_MD),
    ]
}

const RHF_FORM_TSX: &str = r#""use client";

import * as React from "react";
import { FormProvider, useForm, useFormContext } from "react-hook-form";
import type {
  FieldValues,
  SubmitErrorHandler,
  SubmitHandler,
  UseFormProps,
  UseFormReturn,
} from "react-hook-form";

export type DxHookFormRenderProps<
  TFieldValues extends FieldValues,
  TContext,
  TTransformedValues,
> = {
  methods: UseFormReturn<TFieldValues, TContext, TTransformedValues>;
  isSubmitting: boolean;
  isValid: boolean;
};

export type DxHookFormProps<
  TFieldValues extends FieldValues,
  TContext = unknown,
  TTransformedValues = TFieldValues,
> = Omit<React.FormHTMLAttributes<HTMLFormElement>, "children" | "onSubmit"> & {
  options?: UseFormProps<TFieldValues, TContext, TTransformedValues>;
  onSubmit: SubmitHandler<TTransformedValues>;
  onInvalid?: SubmitErrorHandler<TFieldValues>;
  children:
    | React.ReactNode
    | ((
        props: DxHookFormRenderProps<
          TFieldValues,
          TContext,
          TTransformedValues
        >,
      ) => React.ReactNode);
};

export function DxHookForm<
  TFieldValues extends FieldValues = FieldValues,
  TContext = unknown,
  TTransformedValues = TFieldValues,
>({
  options,
  onSubmit,
  onInvalid,
  children,
  noValidate = true,
  ...formProps
}: DxHookFormProps<TFieldValues, TContext, TTransformedValues>) {
  const methods = useForm<TFieldValues, TContext, TTransformedValues>(options);

  return (
    <FormProvider {...methods}>
      <form
        {...formProps}
        noValidate={noValidate}
        onSubmit={methods.handleSubmit(onSubmit, onInvalid)}
      >
        {typeof children === "function"
          ? children({
              methods,
              isSubmitting: methods.formState.isSubmitting,
              isValid: methods.formState.isValid,
            })
          : children}
      </form>
    </FormProvider>
  );
}

export function useDxHookForm<
  TFieldValues extends FieldValues = FieldValues,
  TContext = unknown,
  TTransformedValues = TFieldValues,
>() {
  return useFormContext<TFieldValues, TContext, TTransformedValues>();
}
"#;

const RHF_FIELDS_TSX: &str = r#""use client";

import * as React from "react";
import { Controller, useFieldArray } from "react-hook-form";
import type {
  Control,
  ControllerProps,
  FieldArrayPath,
  FieldErrors,
  FieldPath,
  FieldValues,
  RegisterOptions,
  UseFieldArrayProps,
  UseFieldArrayReturn,
  UseFormRegister,
} from "react-hook-form";

import { Input } from "@/components/ui/input";

import { useDxHookForm } from "./form";

export type DxInputFieldProps<
  TFieldValues extends FieldValues,
  TName extends FieldPath<TFieldValues>,
> = Omit<React.ComponentPropsWithoutRef<typeof Input>, "name"> & {
  name: TName;
  label: React.ReactNode;
  register?: UseFormRegister<TFieldValues>;
  rules?: RegisterOptions<TFieldValues, TName>;
  errors?: FieldErrors<TFieldValues>;
  description?: React.ReactNode;
  errorMessage?: string;
};

export function DxInputField<
  TFieldValues extends FieldValues,
  TName extends FieldPath<TFieldValues>,
>({
  name,
  label,
  register,
  rules,
  errors,
  description,
  errorMessage,
  id,
  ...inputProps
}: DxInputFieldProps<TFieldValues, TName>) {
  const methods = useDxHookForm<TFieldValues>();
  const inputId = id ?? String(name).replaceAll(".", "-");
  const message =
    errorMessage ??
    getDxFieldErrorMessage(errors ?? methods.formState.errors, name);
  const describedBy = [description ? `${inputId}-description` : null, message ? `${inputId}-error` : null]
    .filter(Boolean)
    .join(" ");

  return (
    <label data-dx-field={String(name)} htmlFor={inputId}>
      <span>{label}</span>
      <Input
        id={inputId}
        aria-invalid={message ? true : undefined}
        aria-describedby={describedBy || undefined}
        {...inputProps}
        {...(register ?? methods.register)(name, rules)}
      />
      {description ? (
        <span id={`${inputId}-description`}>{description}</span>
      ) : null}
      {message ? (
        <span id={`${inputId}-error`} role="alert">
          {message}
        </span>
      ) : null}
    </label>
  );
}

export type DxControlledFieldProps<
  TFieldValues extends FieldValues,
  TName extends FieldPath<TFieldValues>,
  TTransformedValues = TFieldValues,
> = Pick<
  ControllerProps<TFieldValues, TName, TTransformedValues>,
  "defaultValue" | "disabled" | "name" | "render" | "rules"
> & {
  control?: Control<TFieldValues>;
};

export function DxControlledField<
  TFieldValues extends FieldValues,
  TName extends FieldPath<TFieldValues>,
  TTransformedValues = TFieldValues,
>({
  control,
  ...props
}: DxControlledFieldProps<TFieldValues, TName, TTransformedValues>) {
  const methods = useDxHookForm<TFieldValues, unknown, TTransformedValues>();

  return <Controller control={control ?? methods.control} {...props} />;
}

export function useDxFieldArray<
  TFieldValues extends FieldValues,
  TName extends FieldArrayPath<TFieldValues>,
>(
  props: UseFieldArrayProps<TFieldValues, TName>,
): UseFieldArrayReturn<TFieldValues, TName> {
  return useFieldArray(props);
}

export function getDxFieldErrorMessage<
  TFieldValues extends FieldValues,
  TName extends FieldPath<TFieldValues>,
>(errors: FieldErrors<TFieldValues>, name: TName): string | undefined {
  const error = getNestedFieldError(errors, String(name));
  if (!error) {
    return undefined;
  }

  if (typeof error.message === "string" && error.message.length > 0) {
    return error.message;
  }

  return typeof error.type === "string" ? `Invalid ${error.type}` : "Invalid value";
}

type FieldErrorLike = {
  message?: unknown;
  type?: unknown;
};

function getNestedFieldError(
  errors: FieldErrors<FieldValues>,
  name: string,
): FieldErrorLike | undefined {
  let current: unknown = errors;
  for (const segment of name.split(".")) {
    if (!current || typeof current !== "object") {
      return undefined;
    }
    current = (current as Record<string, unknown>)[segment];
  }

  if (!current || typeof current !== "object") {
    return undefined;
  }

  return current as FieldErrorLike;
}
"#;

const RHF_RESOLVER_TS: &str = r#"import type {
  FieldErrors,
  FieldValues,
  Resolver,
  ResolverResult,
} from "react-hook-form";

export type DxResolverIssue = {
  path: readonly PropertyKey[];
  message: string;
  code?: string;
};

export type DxSafeParseSchema<TOutput> = {
  safeParse?: (value: unknown) => DxSafeParseResult<TOutput>;
  safeParseAsync: (value: unknown) => Promise<DxSafeParseResult<TOutput>>;
};

export type DxSafeParseResult<TOutput> =
  | { success: true; data: TOutput }
  | { success: false; error: { issues: readonly DxResolverIssue[] } };

export type DxResolverOptions = {
  mode?: "async" | "sync";
};

export function createDxZodResolver<
  TFieldValues extends FieldValues,
  TTransformedValues = TFieldValues,
>(
  schema: DxSafeParseSchema<TTransformedValues>,
  options: DxResolverOptions = {},
): Resolver<TFieldValues, unknown, TTransformedValues> {
  return async (values): Promise<ResolverResult<TFieldValues, TTransformedValues>> => {
    const result =
      options.mode === "sync" && schema.safeParse
        ? schema.safeParse(values)
        : await schema.safeParseAsync(values);

    if (result.success) {
      return {
        values: result.data,
        errors: {},
      };
    }

    return {
      values: {},
      errors: issuesToFieldErrors<TFieldValues>(result.error.issues),
    };
  };
}

export function issuesToFieldErrors<TFieldValues extends FieldValues>(
  issues: readonly DxResolverIssue[],
): FieldErrors<TFieldValues> {
  const errors: Record<string, unknown> = {};

  for (const issue of issues) {
    setNestedError(errors, issue.path.length > 0 ? issue.path : ["root"], {
      type: issue.code ?? "validation",
      message: issue.message,
    });
  }

  return errors as FieldErrors<TFieldValues>;
}

function setNestedError(
  target: Record<string, unknown>,
  path: readonly PropertyKey[],
  error: { type: string; message: string },
) {
  let cursor = target;
  const segments = path.map(String);
  const last = segments.at(-1) ?? "root";

  for (const segment of segments.slice(0, -1)) {
    const next = cursor[segment];
    if (!next || typeof next !== "object") {
      cursor[segment] = {};
    }
    cursor = cursor[segment] as Record<string, unknown>;
  }

  cursor[last] = error;
}
"#;

const RHF_EXAMPLE_TSX: &str = r#""use client";

import { z } from "zod";

import { DxHookForm } from "./form";
import { DxInputField } from "./fields";
import { createDxZodResolver } from "./resolver";

const launchSignupSchema = z.object({
  email: z.email(),
  name: z.string().trim().min(2),
});

type LaunchSignup = z.infer<typeof launchSignupSchema>;

export function LaunchSignupForm({
  onSignup,
}: {
  onSignup: (values: LaunchSignup) => void | Promise<void>;
}) {
  return (
    <DxHookForm<LaunchSignup>
      options={{
        resolver: createDxZodResolver(launchSignupSchema),
        defaultValues: {
          email: "",
          name: "",
        },
      }}
      onSubmit={onSignup}
    >
      <DxInputField<LaunchSignup, "email">
        name="email"
        type="email"
        label="Email"
        autoComplete="email"
      />
      <DxInputField<LaunchSignup, "name">
        name="name"
        label="Name"
        autoComplete="name"
      />
      <button type="submit">Join launch</button>
    </DxHookForm>
  );
}
"#;

const RHF_METADATA_TS: &str = r#"export const dxReactHookFormForgePackage = {
  officialPackageName: "Forms",
  packageId: "forms/react-hook-form",
  upstreamPackage: "react-hook-form",
  sourceMirror: "G:/WWW/inspirations/react-hook-form",
  upstreamVersion: "7.75.0",
  forgeVersion: "7.75.0-dx.0",
  framework: "react",
  honestyLabel: "SOURCE-ONLY",
  inspectedSourceFiles: [
    "package.json",
    "LICENSE",
    "src/index.ts",
    "src/useForm.ts",
    "src/useFormContext.tsx",
    "src/controller.tsx",
    "src/useController.ts",
    "src/useFieldArray.ts",
    "src/types/form.ts",
  ],
  surfaces: [
    "form-provider",
    "registered-fields",
    "controlled-fields",
    "field-arrays",
    "zod-style-resolver",
    "template-lead-form",
  ],
  requiredEnv: [],
  appOwnedBoundaries: [
    "Submit handlers",
    "Spam protection",
    "Validation rules",
    "Accessibility review",
    "Persistence",
    "Authorization",
    "Dependency installation",
    "Governed browser runtime proof",
  ],
  receiptPaths: [
    "docs/packages/forms-react-hook-form.md",
    "examples/template/.dx/forge/receipts/2026-05-22-forms-dashboard-workflow.json",
  ],
  publicApi: [
    "useForm",
    "FormProvider",
    "useFormContext",
    "register",
    "handleSubmit",
    "Controller",
    "useController",
    "useFieldArray",
    "Resolver",
    "FieldErrors",
  ],
  materializedFiles: [
    "lib/forms/react-hook-form/form.tsx",
    "lib/forms/react-hook-form/fields.tsx",
    "lib/forms/react-hook-form/resolver.ts",
    "lib/forms/react-hook-form/example.tsx",
    "lib/forms/react-hook-form/metadata.ts",
    "lib/forms/react-hook-form/README.md",
  ],
  requiredDependencies: [
    {
      name: "react-hook-form",
      version: "^7.75.0",
      reason: "useForm, FormProvider, Controller, useFieldArray, and Resolver types.",
    },
    {
      name: "react",
      version: "^18.0.0 || ^19.0.0",
      reason: "React Hook Form peer dependency and client component runtime.",
    },
  ],
  optionalIntegrations: [
    {
      name: "zod",
      version: "^4.4.3",
      reason: "createDxZodResolver accepts the Zod safeParse/safeParseAsync shape.",
    },
  ],
  discovery: {
    dxAdd: "dx add forms --write",
    canonicalPackage: "forms/react-hook-form",
    formComponent: "DxHookForm",
    fieldComponent: "DxInputField",
    resolverHelper: "createDxZodResolver(schema)",
  },
} as const;

export type DxReactHookFormForgePackage =
  typeof dxReactHookFormForgePackage;
"#;

const RHF_README_MD: &str = r#"# DX Forge Forms

This Forms package materializes a small source-owned form adapter around the real `react-hook-form` 7.75 public API. It does not replace React Hook Form internals, run package lifecycle scripts, or pretend to be `@hookform/resolvers`.

## Owned Files

- `form.tsx` wraps `useForm`, `FormProvider`, `useFormContext`, and `handleSubmit` for launch forms.
- `fields.tsx` adds a typed shadcn input binding, controlled-field, field-array, and error-message helpers using React Hook Form types.
- `resolver.ts` maps Zod-style `safeParse` issues to React Hook Form `Resolver` errors.
- `example.tsx` shows a tiny launch signup form that pairs this slice with Zod.
- `metadata.ts` exposes DX CLI and Zed discovery metadata.

## Required App Dependency

Install or provide `react-hook-form` in the host app and pair this package with the source-owned `shadcn/ui/input` slice. Forge owns these adapter files and receipts; React Hook Form remains the form state engine.

## Template Usage

```tsx
import { z } from "zod";
import { DxHookForm } from "@/lib/forms/react-hook-form/form";
import { DxInputField } from "@/lib/forms/react-hook-form/fields";
import { createDxZodResolver } from "@/lib/forms/react-hook-form/resolver";

const schema = z.object({
  email: z.email(),
});

export function SignupForm() {
  return (
    <DxHookForm
      options={{ resolver: createDxZodResolver(schema) }}
      onSubmit={(values) => saveSignup(values)}
    >
      <DxInputField name="email" type="email" label="Email" />
      <button type="submit">Join launch</button>
    </DxHookForm>
  );
}
```
"#;
