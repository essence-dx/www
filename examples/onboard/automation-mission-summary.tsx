"use client";

import { Badge } from "@/components/ui/badge";

import type { LaunchAutomationDashboardState } from "./automations-status";

type LaunchAutomationMissionSummaryProps = {
  state: LaunchAutomationDashboardState;
};

export function LaunchAutomationMissionSummary({
  state,
}: LaunchAutomationMissionSummaryProps) {
  const authSummary = state.authKinds.join(", ") || "none";
  const credentialSummary = state.credentialTypes.join(", ") || "none";
  const requiredEnvSummary = state.requiredEnv.join(", ") || "none";
  const statusLabel =
    state.workflowStatus === "metadata-ready"
      ? "Metadata ready"
      : state.workflowStatus === "credential-gated"
        ? "Credential gated"
        : "Missing config";
  const receiptLabel =
    state.receiptState === "zed-handoff-created"
      ? "Zed run handoff ready"
      : state.receiptState === "draft-created"
        ? "Local receipt drafted"
        : "Receipt idle";

  return (
    <section
      className="grid gap-3 rounded-md border border-border bg-card p-4 text-card-foreground md:grid-cols-[minmax(0,1fr)_auto]"
      data-dx-automation-auth-kinds={state.authKinds.join(",")}
      data-dx-automation-credential-types={state.credentialTypes.join(",")}
      data-dx-automation-dashboard-state={state.receiptState}
      data-dx-automation-required-env={state.requiredEnv.join(",")}
      data-dx-automation-selected-connector={state.connectorId}
      data-dx-automation-trigger-node={state.triggerNode ? "true" : "false"}
      data-dx-automation-usable-as-tool={state.usableAsTool ? "true" : "false"}
      data-dx-automation-workflow-node-readiness={state.workflowNodeReadiness}
      data-dx-automation-workflow-status={state.workflowStatus}
      data-dx-component="launch-automation-mission-summary"
      data-dx-dashboard-card="automation"
      data-dx-dashboard-workflow="automation-release-receipt"
      data-dx-edit-id="launch.automation-mission-summary"
      data-dx-edit-kind="dashboard-summary"
      data-dx-edit-ops="move_reorder_section,update_text_content"
      data-dx-package="automations/n8n"
      data-dx-package-role="automations"
      data-dx-product-surface="launch-dashboard"
      data-dx-reorder-group="launch-main"
      data-dx-style-surface="automation-connectors"
      data-dx-token-scope="automations/n8n"
    >
      <div className="grid gap-2">
        <div className="flex items-center gap-2" data-dx-icon-search="pack:n8n">
          <dx-icon name="pack:n8n" aria-hidden="true" />
          <span className="text-sm font-medium">Automation Connectors</span>
        </div>
        <h3
          className="text-lg font-semibold tracking-normal"
          data-dx-editable-text="launch.automation-mission-title"
        >
          {state.connectorDisplayName}
        </h3>
        <p
          className="text-sm leading-6 text-muted-foreground"
          data-dx-editable-text="launch.automation-mission-copy"
        >
          {state.workflowStatus === "metadata-ready"
            ? "The selected connector can prepare a local redacted launch receipt without provider credentials."
            : `This connector stays honest in missing-config state until the app owns ${credentialSummary}.`}
        </p>
        <p
          className="text-xs text-muted-foreground"
          data-dx-automation-intent-preview={state.intent}
        >
          Intent: {state.intent}
        </p>
      </div>
      <div className="grid content-start gap-2 text-sm">
        <Badge variant={state.workflowStatus === "metadata-ready" ? "secondary" : "outline"}>
          {statusLabel}
        </Badge>
        <Badge variant="outline">{receiptLabel}</Badge>
        <span className="text-xs text-muted-foreground">
          Auth: {authSummary}
        </span>
        <span className="text-xs text-muted-foreground">
          Credentials: {credentialSummary}
        </span>
        <span className="text-xs text-muted-foreground">
          Required env: {requiredEnvSummary}
        </span>
        <span className="text-xs text-muted-foreground">
          Node: {state.workflowNodeReadiness}; tool{" "}
          {state.usableAsTool ? "ready" : "not ready"}; trigger{" "}
          {state.triggerNode ? "yes" : "no"}
        </span>
      </div>
    </section>
  );
}
