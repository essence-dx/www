"use client";

import { Button } from "@/components/ui/button";
import {
  Field,
  FieldDescription,
  FieldError,
  FieldGroup,
  FieldLabel,
} from "@/components/ui/field";
import { Textarea } from "@/components/ui/textarea";
import { DxInputField } from "@/lib/forms/react-hook-form/fields";
import {
  DxHookForm,
  useDxHookForm,
} from "@/lib/forms/react-hook-form/form";
import { createDxZodResolver } from "@/lib/forms/react-hook-form/resolver";
import { z } from "zod";

const launchLeadSchema = z.object({
  email: z.email(),
  notes: z.string().trim().max(280),
});

export type LaunchLeadFormValues = z.infer<typeof launchLeadSchema>;

const launchLeadDefaults = {
  email: "",
  notes: "",
} satisfies LaunchLeadFormValues;

type LaunchLeadFormProps = {
  onLead?: (values: LaunchLeadFormValues) => void | Promise<void>;
};

function LaunchLeadFields() {
  const form = useDxHookForm<LaunchLeadFormValues>();

  return (
    <FieldGroup
      data-dx-component="launch-lead-fields"
      data-dx-editable="form-fields"
      data-dx-package="shadcn/ui/field"
    >
      <DxInputField<LaunchLeadFormValues, "email">
        name="email"
        type="email"
        label="Launch email"
        placeholder="team@example.com"
        description="Used for release readiness notifications."
      />
      <Field data-dx-content-key="launch.lead.notes">
        <FieldLabel htmlFor="launch-notes">Launch notes</FieldLabel>
        <Textarea
          id="launch-notes"
          rows={4}
          maxLength={280}
          placeholder="Release blockers, owner notes, or final signoff context"
          aria-invalid={Boolean(form.formState.errors.notes)}
          aria-describedby="launch-notes-help launch-notes-error"
          {...form.register("notes")}
        />
        <FieldDescription id="launch-notes-help">
          Optional launch context passed to the app-owned submit handler.
        </FieldDescription>
        <FieldError
          id="launch-notes-error"
          errors={[form.formState.errors.notes]}
        />
      </Field>
    </FieldGroup>
  );
}

export function LaunchLeadForm({ onLead }: LaunchLeadFormProps) {
  return (
    <DxHookForm<LaunchLeadFormValues>
      className="grid gap-4"
      data-dx-component="template-lead-form"
      data-dx-edit-id="launch.lead-form"
      data-dx-edit-kind="form"
      data-dx-edit-ops="update_text_content,update_design_token"
      data-dx-insert-slot="template-lead-form"
      data-dx-package="forms/react-hook-form"
      options={{
        defaultValues: launchLeadDefaults,
        resolver: createDxZodResolver<
          LaunchLeadFormValues,
          LaunchLeadFormValues
        >(launchLeadSchema),
      }}
      onSubmit={onLead}
    >
      <LaunchLeadFields />
      <Button
        type="submit"
        className="w-fit"
        data-dx-editable-text="launch-lead-submit"
      >
        Save launch lead
      </Button>
    </DxHookForm>
  );
}
