"use client";

import * as React from "react";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
  createDxDashboardSettingsReceipt,
  dxDashboardSettingsExample,
  dxDashboardSettingsInvalidExample,
  formatDxDashboardSettingsIssues,
  safeParseDxDashboardSettingsForm,
  type DxDashboardSettingsFormInput,
} from "@/lib/validation/zod/dashboard-settings";

export function LaunchZodDashboardSettings() {
  const [draft, setDraft] = React.useState<DxDashboardSettingsFormInput>(
    dxDashboardSettingsInvalidExample,
  );
  const [submitted, setSubmitted] = React.useState(false);
  const result = safeParseDxDashboardSettingsForm(draft);
  const issues = formatDxDashboardSettingsIssues(draft);
  const receipt = createDxDashboardSettingsReceipt(draft);
  const status = result.success ? "valid" : "invalid";
  const output = result.success ? "accepted" : "blocked";

  const updateDraft = React.useCallback(
    (key: keyof DxDashboardSettingsFormInput, value: string | boolean) => {
      setDraft((current) => ({
        ...current,
        [key]: value,
      }));
      setSubmitted(false);
    },
    [],
  );

  return (
    <form
      className="grid gap-4 rounded-md border bg-card p-4 text-card-foreground"
      data-dx-package="validation/zod"
      data-dx-component="zod-dashboard-settings-form"
      data-dx-style-surface="validation-schemas"
      data-dx-token-scope="validation/zod"
      data-dx-zod-settings-state={status}
      data-dx-zod-settings-issues={issues.length}
      data-dx-zod-settings-output={output}
      data-dx-node-modules="forbidden"
      onSubmit={(event) => {
        event.preventDefault();
        setSubmitted(true);
      }}
    >
      <div className="flex items-start justify-between gap-3">
        <div className="grid gap-1">
          <div className="flex items-center gap-2">
            <span
              className="flex size-8 items-center justify-center rounded-md border bg-background"
              data-dx-icon="pack:validation-zod"
            >
              <dx-icon name="pack:validation-zod" aria-hidden="true" />
            </span>
            <p className="text-sm font-medium">Dashboard settings validation</p>
          </div>
          <p className="text-xs leading-5 text-muted-foreground">
            Zod validates editable launch-dashboard settings before they become
            app state.
          </p>
        </div>
        <span
          className="rounded-md border px-2 py-1 text-xs"
          data-dx-zod-settings-status={status}
        >
          {status}
        </span>
      </div>

      <div className="grid gap-3 md:grid-cols-2">
        <label className="grid gap-1 text-xs">
          <span>Workspace</span>
          <Input
            aria-invalid={issues.some(
              (issue) => issue.path === "workspaceName",
            )}
            data-dx-zod-settings-field="workspaceName"
            onChange={(event) =>
              updateDraft("workspaceName", event.currentTarget.value)
            }
            value={String(draft.workspaceName ?? "")}
          />
        </label>

        <label className="grid gap-1 text-xs">
          <span>Contact email</span>
          <Input
            aria-invalid={issues.some(
              (issue) => issue.path === "contactEmail",
            )}
            data-dx-zod-settings-field="contactEmail"
            onChange={(event) =>
              updateDraft("contactEmail", event.currentTarget.value)
            }
            value={String(draft.contactEmail ?? "")}
          />
        </label>

        <label className="grid gap-1 text-xs">
          <span>Locale</span>
          <select
            className="h-9 rounded-md border bg-background px-3 text-sm text-foreground shadow-sm"
            data-dx-zod-settings-field="defaultLocale"
            onChange={(event) =>
              updateDraft("defaultLocale", event.currentTarget.value)
            }
            value={String(draft.defaultLocale ?? "en")}
          >
            <option value="en">English</option>
            <option value="bn">Bangla</option>
            <option value="hi">Hindi</option>
          </select>
        </label>

        <label className="grid gap-1 text-xs">
          <span>Theme</span>
          <select
            className="h-9 rounded-md border bg-background px-3 text-sm text-foreground shadow-sm"
            data-dx-zod-settings-field="theme"
            onChange={(event) => updateDraft("theme", event.currentTarget.value)}
            value={String(draft.theme ?? "system")}
          >
            <option value="system">System</option>
            <option value="light">Light</option>
            <option value="dark">Dark</option>
          </select>
        </label>

        <label className="grid gap-1 text-xs">
          <span>Preview mode</span>
          <select
            className="h-9 rounded-md border bg-background px-3 text-sm text-foreground shadow-sm"
            data-dx-zod-settings-field="previewMode"
            onChange={(event) =>
              updateDraft("previewMode", event.currentTarget.value)
            }
            value={String(draft.previewMode ?? "stable")}
          >
            <option value="stable">Stable</option>
            <option value="preview">Preview</option>
          </select>
        </label>

        <label className="grid gap-1 text-xs">
          <span>Launch score target</span>
          <Input
            aria-invalid={issues.some(
              (issue) => issue.path === "launchScoreTarget",
            )}
            data-dx-zod-settings-field="launchScoreTarget"
            inputMode="numeric"
            onChange={(event) =>
              updateDraft("launchScoreTarget", event.currentTarget.value)
            }
            value={String(draft.launchScoreTarget ?? "")}
          />
        </label>
      </div>

      <label className="flex items-center gap-2 text-xs">
        <input
          checked={Boolean(draft.packageReceiptsRequired)}
          className="size-4 accent-primary"
          data-dx-zod-settings-field="packageReceiptsRequired"
          onChange={(event) =>
            updateDraft("packageReceiptsRequired", event.currentTarget.checked)
          }
          type="checkbox"
        />
        Require source receipts before dashboard launch
      </label>

      <div className="flex flex-wrap gap-2">
        <Button
          type="button"
          variant="outline"
          data-dx-zod-settings-action="load-invalid"
          onClick={() => {
            setDraft(dxDashboardSettingsInvalidExample);
            setSubmitted(false);
          }}
        >
          Load invalid
        </Button>
        <Button
          type="button"
          variant="secondary"
          data-dx-zod-settings-action="load-valid"
          onClick={() => {
            setDraft(dxDashboardSettingsExample);
            setSubmitted(false);
          }}
        >
          Load valid
        </Button>
        <Button type="submit" data-dx-zod-settings-action="validate">
          Validate settings
        </Button>
      </div>

      <div
        className="grid gap-2 text-xs text-muted-foreground"
        data-dx-zod-settings-issues={issues.length}
      >
        {issues.length > 0 ? (
          <ul className="grid gap-1">
            {issues.map((issue) => (
              <li key={`${issue.path}:${issue.code}`}>
                {issue.path}: {issue.message}
              </li>
            ))}
          </ul>
        ) : (
          <p>
            Settings are valid for {result.success ? result.data.workspaceName : "launch"}.
          </p>
        )}
      </div>

      <pre
        className="max-h-48 overflow-auto rounded-md border bg-muted p-3 text-xs text-foreground"
        data-dx-zod-settings-output={output}
        data-dx-zod-settings-submitted={submitted ? "true" : "false"}
      >
        {JSON.stringify(receipt, null, 2)}
      </pre>
    </form>
  );
}
