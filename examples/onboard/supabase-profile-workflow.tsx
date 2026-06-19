"use client";

import * as React from "react";

import { dxSupabaseForgePackage } from "@/lib/supabase/metadata";
import {
  createDxSupabaseProfilePreview,
  createDxSupabaseProfileUpsertReceipt,
  dxSupabaseInitialProfileDraft,
  dxSupabaseLocalProfile,
  dxSupabaseProfileApi,
  dxSupabaseProfileFields,
  readDxSupabaseProfileConfigStatus,
  updateDxSupabaseProfileDraft,
  type DxSupabaseProfile,
  type DxSupabaseProfileInput,
  type DxSupabaseProfileUpsertReceipt,
} from "@/lib/supabase/profile-workflow";

type DxIconProps = React.HTMLAttributes<HTMLElement> & {
  name: string;
};

declare global {
  namespace JSX {
    interface IntrinsicElements {
      "dx-icon": DxIconProps;
    }
  }
}

export function LaunchSupabaseProfileWorkflow() {
  const [configStatus, refreshConfig] = React.useReducer(
    () => readDxSupabaseProfileConfigStatus(),
    undefined,
    readDxSupabaseProfileConfigStatus,
  );
  const [profile, setProfile] = React.useState<DxSupabaseProfile | null>(null);
  const [profileDraft, setProfileDraft] =
    React.useState<DxSupabaseProfileInput>(dxSupabaseInitialProfileDraft);
  const [receipt, setReceipt] =
    React.useState<DxSupabaseProfileUpsertReceipt | null>(null);

  const selectedProfile = profile ?? dxSupabaseLocalProfile;
  const previewProfile = createDxSupabaseProfilePreview(
    selectedProfile,
    profileDraft,
  );
  const receiptState = receipt?.status ?? "idle";
  const updateProfileDraft = React.useCallback(
    (field: keyof DxSupabaseProfileInput, value: string) => {
      setProfileDraft((current) =>
        updateDxSupabaseProfileDraft(current, field, value),
      );
    },
    [],
  );

  return (
    <section
      className="grid gap-3 rounded-md border p-3"
      data-dx-dashboard-card="account-profile"
      data-dx-dashboard-workflow="account-profile-settings"
      data-dx-component="supabase-profile-workflow"
      data-dx-edit-id="launch.account-data.supabase-profile"
      data-dx-edit-kind="dashboard-workflow"
      data-dx-edit-ops="update_text_content,move_reorder_section"
      data-dx-icon-search="database:supabase"
      data-dx-node-modules="not-required"
      data-dx-package="supabase/client"
      data-dx-style-surface="backend-platform-client"
      data-dx-supabase-config-status={configStatus.kind}
      data-dx-supabase-receipt-path="examples/template/.dx/forge/receipts/2026-05-22-supabase-client-dashboard-workflow.json"
      data-dx-supabase-receipt-state={receiptState}
      data-dx-supabase-workflow="profile-settings"
      data-dx-token-scope="supabase/client"
    >
      <div className="flex flex-wrap items-start justify-between gap-3">
        <div className="flex items-start gap-2">
          <dx-icon name="database:supabase" aria-hidden="true" className="mt-0.5 size-4" />
          <div>
            <p className="text-sm font-medium">Supabase profile workflow</p>
            <p className="text-xs leading-5 text-muted-foreground">
              Uses {dxSupabaseForgePackage.packageId} profile helpers for the
              account boundary: read the signed-in user, load `profiles`, and
              prepare a safe upsert.
            </p>
          </div>
        </div>
        <button
          className="rounded-md border px-3 py-2 text-xs font-medium"
          data-dx-supabase-action="refresh-profile-config"
          type="button"
          onClick={refreshConfig}
        >
          Recheck env
        </button>
      </div>

      <div className="grid gap-2 sm:grid-cols-2">
        <button
          className="rounded-md border px-3 py-2 text-left text-xs font-medium"
          data-dx-supabase-action="load-profile-fixture"
          type="button"
          onClick={() => {
            setProfile(dxSupabaseLocalProfile);
            setProfileDraft(dxSupabaseInitialProfileDraft);
          }}
        >
          Load local profile fixture
        </button>
        <button
          className="rounded-md border px-3 py-2 text-left text-xs font-medium"
          data-dx-supabase-action="prepare-profile-upsert"
          type="button"
          onClick={() => {
            setProfile(previewProfile);
            setReceipt(
              createDxSupabaseProfileUpsertReceipt(
                configStatus,
                previewProfile,
                profileDraft,
              ),
            );
          }}
        >
          Prepare profile upsert
        </button>
      </div>

      <div className="grid gap-2 sm:grid-cols-3" data-dx-dashboard-form="account-profile">
        {dxSupabaseProfileFields.map((field) => (
          <label
            className="grid gap-1 text-xs font-medium"
            data-dx-supabase-profile-label={field.key}
            key={field.key}
          >
            {field.label}
            <input
              autoComplete={field.autoComplete}
              className="rounded-md border bg-background px-3 py-2 font-normal text-foreground"
              data-dx-supabase-profile-field={field.key}
              type={field.inputType}
              value={profileDraft[field.key] ?? ""}
              onChange={(event) =>
                updateProfileDraft(field.key, event.target.value)
              }
            />
          </label>
        ))}
      </div>

      <div
        className="grid gap-2 rounded-md bg-muted p-3"
        data-dx-supabase-profile-id={previewProfile.id}
        data-dx-supabase-profile-loaded={profile ? "true" : "false"}
      >
        <p className="text-xs font-medium">{previewProfile.fullName}</p>
        <p className="text-xs leading-5 text-muted-foreground">
          @{previewProfile.username} - {previewProfile.website}
        </p>
      </div>

      <div className="grid gap-2 rounded-md bg-muted p-3">
        <p
          className="text-xs leading-5 text-muted-foreground"
          data-dx-supabase-config-message={configStatus.kind}
          role={configStatus.kind === "missing-config" ? "status" : undefined}
        >
          {configStatus.message}
        </p>
        <p className="text-xs leading-5 text-muted-foreground">
          Public API: {dxSupabaseProfileApi.readCurrent.name} +{" "}
          {dxSupabaseProfileApi.upsert.name}
        </p>
      </div>

      {receipt ? (
        <div
          className="grid gap-2 rounded-md border p-3"
          data-dx-supabase-upsert-operation={receipt.operation}
          data-dx-supabase-upsert-status={receipt.status}
        >
          <p className="text-xs font-medium">{receipt.operation}</p>
          <p className="text-xs leading-5 text-muted-foreground">
            {receipt.boundary}
          </p>
          <dl className="grid gap-2 text-xs sm:grid-cols-2">
            <div>
              <dt className="text-muted-foreground">User</dt>
              <dd className="font-medium">{receipt.userId}</dd>
            </div>
            <div>
              <dt className="text-muted-foreground">Full name</dt>
              <dd className="font-medium">{receipt.input.fullName}</dd>
            </div>
          </dl>
        </div>
      ) : null}
    </section>
  );
}

export default LaunchSupabaseProfileWorkflow;
