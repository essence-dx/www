"use client";

import * as React from "react";

import { Badge } from "@/components/ui/badge";
import {
  filterDxN8nConnectors,
  normalizeDxN8nConnector,
  workflowStatusForConnector,
  type DxN8nConnectorFilter,
  type DxN8nLaunchConnector,
} from "@/lib/automations/n8n/catalog";
import { buildDxN8nCredentialReadiness } from "@/lib/automations/n8n/readiness";
import {
  createDxN8nRunReceipt,
  type DxN8nRunReceipt,
} from "@/lib/automations/n8n/receipt";

import {
  automationConnectorHighlights,
  automationRoutes,
  automationSummary,
} from "./automations/automations-metadata";

type AutomationConnector = DxN8nLaunchConnector;

const visibleConnectors: AutomationConnector[] = [
  ...automationConnectorHighlights
    .filter((connector) => connector.status === "ready")
    .slice(0, 2),
  ...automationConnectorHighlights
    .filter((connector) => connector.status === "needs_credential")
    .slice(0, 4),
];

const connectorFilters: { id: DxN8nConnectorFilter; label: string }[] = [
  { id: "all", label: "All" },
  { id: "ready", label: "Local ready" },
  { id: "missing-config", label: "Needs config" },
  { id: "tool-ready", label: "Tool-ready" },
];

const launchAutomationReceiptPath =
  "G:/Dx/.dx/receipts/automations/launch-release-notification.json";
const launchAutomationRunReceiptPath =
  "G:/Dx/.dx/receipts/automations/run-latest.json";

export type LaunchAutomationDashboardState = {
  connectorId: string;
  connectorDisplayName: string;
  workflowStatus: "metadata-ready" | "missing-config" | "credential-gated";
  workflowNodeReadiness: "metadata-ready" | "credential-gated";
  authKinds: readonly string[];
  credentialTypes: readonly string[];
  requiredEnv: readonly string[];
  usableAsTool: boolean;
  triggerNode: boolean;
  receiptState: "idle" | "draft-created" | "zed-handoff-created";
  intent: string;
};

export type LaunchAutomationBridgeStatusProps = {
  surface?: "dashboard" | "connector-readiness";
  onWorkflowChange?: (state: LaunchAutomationDashboardState) => void;
};

export function LaunchAutomationBridgeStatus({
  onWorkflowChange,
  surface = "connector-readiness",
}: LaunchAutomationBridgeStatusProps) {
  const normalizedConnectors = React.useMemo(
    () => visibleConnectors.map((connector) => normalizeDxN8nConnector(connector)),
    [],
  );
  const [selectedConnectorId, setSelectedConnectorId] = React.useState(
    normalizedConnectors[0]?.id ?? "",
  );
  const [connectorFilter, setConnectorFilter] =
    React.useState<DxN8nConnectorFilter>("all");
  const [draftReceipt, setDraftReceipt] =
    React.useState<DxN8nRunReceipt | null>(null);
  const [zedHandoffReceipt, setZedHandoffReceipt] =
    React.useState<DxN8nRunReceipt | null>(null);
  const [workflowIntent, setWorkflowIntent] = React.useState(
    "Notify the launch channel when a release receipt is ready for Friday review.",
  );
  const filteredConnectors = React.useMemo(
    () => filterDxN8nConnectors(normalizedConnectors, connectorFilter),
    [connectorFilter, normalizedConnectors],
  );
  const selectedConnector =
    normalizedConnectors.find((connector) => connector.id === selectedConnectorId) ??
    normalizedConnectors[0];
  const readiness = buildDxN8nCredentialReadiness(selectedConnector);
  const workflowStatus = workflowStatusForConnector(selectedConnector);
  const selectedResources = selectedConnector.resources.slice(0, 4);
  const selectedOperations = selectedConnector.operations.slice(0, 4);
  const selectedAuthKinds =
    selectedConnector.authKinds.length > 0 ? selectedConnector.authKinds : ["none"];
  const selectedCredentialNames =
    selectedConnector.credentials.length > 0
      ? selectedConnector.credentials
      : ["none"];
  const selectedAuthKindList = selectedAuthKinds.join(",");
  const selectedCredentialNameList = selectedCredentialNames.join(",");
  const requiredEnvList = readiness.requiredEnv.join(",");
  const receiptState = zedHandoffReceipt
    ? "zed-handoff-created"
    : draftReceipt
      ? "draft-created"
      : "idle";
  const dashboardState = React.useMemo<LaunchAutomationDashboardState>(
    () => ({
      connectorId: selectedConnector.id,
      connectorDisplayName: selectedConnector.displayName,
      workflowStatus,
      workflowNodeReadiness: selectedConnector.workflowNode.runMode,
      authKinds: selectedAuthKindList.split(",").filter(Boolean),
      credentialTypes: selectedCredentialNameList.split(",").filter(Boolean),
      requiredEnv: requiredEnvList.split(",").filter(Boolean),
      usableAsTool: selectedConnector.workflowNode.usableAsTool,
      triggerNode: selectedConnector.workflowNode.trigger,
      receiptState,
      intent: workflowIntent,
    }),
    [
      receiptState,
      selectedAuthKindList,
      selectedConnector.displayName,
      selectedConnector.id,
      selectedConnector.workflowNode.runMode,
      selectedConnector.workflowNode.trigger,
      selectedConnector.workflowNode.usableAsTool,
      selectedCredentialNameList,
      requiredEnvList,
      workflowIntent,
      workflowStatus,
    ],
  );

  React.useEffect(() => {
    onWorkflowChange?.(dashboardState);
  }, [dashboardState, onWorkflowChange]);

  function selectFilter(filter: DxN8nConnectorFilter) {
    const nextConnector = filterDxN8nConnectors(normalizedConnectors, filter)[0];

    setConnectorFilter(filter);
    setDraftReceipt(null);
    setZedHandoffReceipt(null);

    if (nextConnector) {
      setSelectedConnectorId(nextConnector.id);
    }
  }

  function prepareLocalDraft() {
    setDraftReceipt(
      createDxN8nRunReceipt({
        workflowId:
          surface === "dashboard"
            ? "launch-release-notification"
            : "connector-readiness",
        connector: selectedConnector,
        intent: workflowIntent,
        mode: "dry-run",
      }),
    );
    setZedHandoffReceipt(null);
  }

  function prepareZedRunHandoff() {
    setZedHandoffReceipt(
      createDxN8nRunReceipt({
        workflowId: "launch-release-notification",
        connector: selectedConnector,
        intent: workflowIntent,
        mode: "run",
      }),
    );
  }

  return (
    <div
      className="grid gap-4"
      data-dx-automation-route="/automations"
      data-dx-automation-view="launch-bridge"
      data-dx-automation-workflow="connector-readiness"
      data-dx-automation-dashboard-connector={dashboardState.connectorId}
      data-dx-automation-dashboard-node-readiness={dashboardState.workflowNodeReadiness}
      data-dx-automation-dashboard-state={dashboardState.receiptState}
      data-dx-automation-dashboard-card="launch-release-notification"
      data-dx-automation-receipt-path={launchAutomationReceiptPath}
      data-dx-automation-run-receipt-path={launchAutomationRunReceiptPath}
      data-dx-automation-required-env={readiness.requiredEnv.join(",")}
      data-dx-automation-surface={surface}
      data-dx-component="launch-automation-connector-workflow"
      data-dx-dashboard-workflow="automation-release-receipt"
      data-dx-editable="content"
      data-dx-edit-id="launch.automations.status"
      data-dx-edit-kind="status"
      data-dx-edit-ops="move_reorder_section,update_text_content"
      data-dx-content-key="launch.automations"
      data-dx-node-modules="forbidden"
      data-dx-package="automations/n8n"
      data-dx-source="examples/template/automations-status.tsx"
      data-dx-style-surface="automation-connectors"
      data-dx-token-scope="automations/n8n"
    >
      <div className="flex items-center gap-2" data-dx-icon-search="pack:n8n">
        <dx-icon name="pack:n8n" aria-hidden="true" />
        <span className="text-sm font-medium">Automation Connectors</span>
      </div>

      <dl
        className="grid gap-3 sm:grid-cols-3"
        data-dx-component="launch-automation-catalog-summary"
      >
        <div className="rounded-md border border-border p-3">
          <dt className="text-xs text-muted-foreground">Connectors</dt>
          <dd className="mt-1 text-2xl font-semibold">
            {automationSummary.connectorCount}
          </dd>
        </div>
        <div className="rounded-md border border-border p-3">
          <dt className="text-xs text-muted-foreground">Credentials</dt>
          <dd className="mt-1 text-2xl font-semibold">
            {automationSummary.credentialCount}
          </dd>
        </div>
        <div className="rounded-md border border-border p-3">
          <dt className="text-xs text-muted-foreground">Tool-ready</dt>
          <dd className="mt-1 text-2xl font-semibold">
            {automationSummary.toolReadyCount}
          </dd>
        </div>
      </dl>

      <div className="grid gap-2 text-sm">
        <p className="text-muted-foreground">
          <span data-dx-editable-text="launch.automations-summary">
            Automation Connectors metadata from {automationSummary.sourceProvenance}
            powers a release notification workflow with redacted receipt handoff
            and app-owned credential boundaries.
          </span>
        </p>
        <code className="min-w-0 overflow-hidden text-ellipsis rounded-md border border-border bg-muted px-3 py-2 text-xs text-muted-foreground">
          {automationSummary.commands.connectors}
        </code>
      </div>

      <div
        className="grid gap-3"
        data-dx-automation-interaction="connector-picker"
      >
        <div
          className="flex flex-wrap gap-2"
          data-dx-automation-interaction="connector-filter"
        >
          {connectorFilters.map((filter) => {
            const selected = filter.id === connectorFilter;

            return (
              <button
                key={filter.id}
                aria-pressed={selected}
                className="rounded-md border border-border px-3 py-1.5 text-xs font-medium text-muted-foreground transition hover:bg-muted hover:text-foreground aria-pressed:bg-muted aria-pressed:text-foreground"
                data-dx-automation-filter={filter.id}
                data-dx-automation-filter-active={selected ? "true" : "false"}
                onClick={() => selectFilter(filter.id)}
                type="button"
              >
                {filter.label}
              </button>
            );
          })}
        </div>

        <div className="flex flex-wrap gap-2">
          {filteredConnectors.map((connector) => {
            const selected = connector.id === selectedConnector.id;

            return (
              <button
                key={connector.id}
                aria-pressed={selected}
                className="rounded-md border border-border px-3 py-2 text-left text-sm transition hover:bg-muted aria-pressed:bg-muted"
                data-dx-automation-connector={connector.id}
                data-dx-automation-connector-id={connector.id}
                data-dx-automation-connector-status={workflowStatusForConnector(connector)}
                data-dx-automation-selected={selected ? "true" : "false"}
                onClick={() => {
                  setSelectedConnectorId(connector.id);
                  setDraftReceipt(null);
                  setZedHandoffReceipt(null);
                }}
                type="button"
              >
                <span className="block font-medium">{connector.displayName}</span>
                <span className="block text-xs text-muted-foreground">
                  {connector.workflowNode.usableAsTool
                    ? "Tool-ready"
                    : "Workflow metadata"}
                </span>
              </button>
            );
          })}
        </div>

        <div
          className="grid gap-3 rounded-md border border-border bg-card p-3"
          data-dx-automation-interaction="workflow-readiness"
          data-dx-automation-missing-config={
            readiness.missingCredentials.length > 0 ? "true" : "false"
          }
          data-dx-automation-required-env={readiness.requiredEnv.join(",")}
          data-dx-automation-readiness-card={workflowStatus}
          data-dx-automation-receipt-status={
            draftReceipt ? draftReceipt.status : "idle"
          }
          data-dx-automation-selected-connector={selectedConnector.id}
          data-dx-automation-workflow-status={workflowStatus}
        >
          <div className="flex flex-wrap items-center justify-between gap-2">
            <div>
              <p className="font-medium">{selectedConnector.displayName}</p>
              <p className="text-xs text-muted-foreground">
                {selectedConnector.sourceFile}
              </p>
            </div>
            <Badge
              variant={workflowStatus === "metadata-ready" ? "secondary" : "outline"}
            >
              {workflowStatus === "metadata-ready"
                ? "Local metadata ready"
                : "Missing config"}
            </Badge>
          </div>

          <p className="text-sm text-muted-foreground">
            {workflowStatus === "metadata-ready"
              ? "This connector can be drafted locally without credentials. Live execution remains operator gated."
              : `Needs app-owned credentials: ${readiness.missingCredentials.join(", ")}.`}
          </p>

          <label
            className="grid gap-2 text-sm"
            data-dx-automation-interaction="workflow-intent"
          >
            Release workflow intent
            <textarea
              className="min-h-20 rounded-md border border-border bg-background px-3 py-2 text-sm text-foreground"
              data-dx-automation-intent-input="release-notification"
              onChange={(event) => {
                setWorkflowIntent(event.currentTarget.value);
                setDraftReceipt(null);
                setZedHandoffReceipt(null);
              }}
              value={workflowIntent}
            />
          </label>

          <div
            className="grid gap-3 lg:grid-cols-3"
            data-dx-automation-capability-map={selectedConnector.id}
          >
            <div
              className="grid gap-2"
              data-dx-automation-auth-kinds={selectedAuthKinds.join(",")}
              data-dx-automation-credential-schema={selectedConnector.id}
              data-dx-automation-credential-types={
                selectedConnector.credentials.join(",") || "none"
              }
              data-dx-automation-required-env={readiness.requiredEnv.join(",")}
              data-dx-automation-trigger-node={
                selectedConnector.workflowNode.trigger ? "true" : "false"
              }
              data-dx-automation-usable-as-tool={
                selectedConnector.workflowNode.usableAsTool ? "true" : "false"
              }
              data-dx-automation-workflow-node-readiness={
                selectedConnector.workflowNode.runMode
              }
            >
              <p className="text-xs font-medium text-muted-foreground">
                Credential boundary
              </p>
              <div className="flex flex-wrap gap-2">
                {selectedAuthKinds.map((authKind) => (
                  <Badge
                    key={authKind}
                    data-dx-automation-auth-kind={authKind}
                    variant="outline"
                  >
                    {authKind}
                  </Badge>
                ))}
                {selectedCredentialNames.map((credentialName) => (
                  <Badge
                    key={credentialName}
                    data-dx-automation-credential-type={credentialName}
                    variant="outline"
                  >
                    {credentialName}
                  </Badge>
                ))}
              </div>
              <p className="text-xs text-muted-foreground">
                Node mode {selectedConnector.workflowNode.runMode}; tool{" "}
                {selectedConnector.workflowNode.usableAsTool ? "ready" : "not ready"};
                trigger {selectedConnector.workflowNode.trigger ? "yes" : "no"}.
              </p>
              <p className="text-xs text-muted-foreground">
                Required env: {readiness.requiredEnv.join(", ")}
              </p>
            </div>

            <div
              className="grid gap-2"
              data-dx-automation-resource-list={selectedConnector.id}
            >
              <p className="text-xs font-medium text-muted-foreground">Resources</p>
              <div className="flex flex-wrap gap-2">
                {selectedResources.map((resource) => (
                  <Badge
                    key={resource.value}
                    data-dx-automation-resource={resource.value}
                    variant="outline"
                  >
                    {resource.name}
                  </Badge>
                ))}
                {selectedResources.length === 0 ? (
                  <span
                    className="text-xs text-muted-foreground"
                    data-dx-automation-resource="metadata-only"
                  >
                    Metadata only
                  </span>
                ) : null}
              </div>
            </div>

            <div
              className="grid gap-2"
              data-dx-automation-operation-list={selectedConnector.id}
            >
              <p className="text-xs font-medium text-muted-foreground">Operations</p>
              <div className="flex flex-wrap gap-2">
                {selectedOperations.map((operation) => (
                  <Badge
                    key={operation.value}
                    data-dx-automation-operation={operation.value}
                    title={operation.action}
                    variant="outline"
                  >
                    {operation.name}
                  </Badge>
                ))}
                {selectedOperations.length === 0 ? (
                  <span
                    className="text-xs text-muted-foreground"
                    data-dx-automation-operation="metadata-only"
                  >
                    Metadata only
                  </span>
                ) : null}
              </div>
            </div>
          </div>

          <div className="flex flex-wrap gap-2">
            <button
              className="w-fit rounded-md border border-border bg-primary px-3 py-2 text-sm font-medium text-primary-foreground transition hover:bg-primary/90"
              data-dx-automation-action="preview-run-receipt"
              data-dx-automation-local-receipt="draft-workflow-receipt"
              data-dx-automation-safe-action="prepare-dry-run-receipt"
              onClick={prepareLocalDraft}
              type="button"
            >
              Local receipt preview
            </button>

            <button
              className="w-fit rounded-md border border-border px-3 py-2 text-sm font-medium text-foreground transition hover:bg-muted"
              data-dx-automation-handoff="zed-run-receipt"
              data-dx-automation-run-receipt-path={launchAutomationRunReceiptPath}
              data-dx-automation-safe-action="prepare-zed-run-handoff"
              onClick={prepareZedRunHandoff}
              type="button"
            >
              Prepare Zed run handoff
            </button>
          </div>

          <div
            className="rounded-md border border-border bg-muted px-3 py-2 text-xs text-muted-foreground"
            data-dx-automation-draft-state={draftReceipt ? "ready" : "idle"}
            data-dx-automation-intent-preview={workflowIntent}
            data-dx-automation-receipt-intent={
              draftReceipt?.workflowIntent ?? workflowIntent
            }
            data-dx-automation-receipt-path={launchAutomationReceiptPath}
            data-dx-automation-receipt-state={draftReceipt ? "created" : "idle"}
            data-dx-automation-receipt-status={draftReceipt ? draftReceipt.status : "idle"}
          >
            {draftReceipt ? (
              <span>
                Drafted {draftReceipt.connectorName} receipt for{" "}
                <code>{draftReceipt.commands.run}</code>; receipt intent{" "}
                <strong>{draftReceipt.workflowIntent || "No intent entered"}</strong>;
                status{" "}
                <strong>{draftReceipt.status}</strong>; path{" "}
                <code>{launchAutomationReceiptPath}</code>.
              </span>
            ) : (
              <span>Select a connector, then prepare a local draft receipt.</span>
            )}
          </div>

          <div
            className="rounded-md border border-border bg-muted px-3 py-2 text-xs text-muted-foreground"
            data-dx-automation-run-receipt-output="zed-handoff"
            data-dx-automation-run-receipt-intent={
              zedHandoffReceipt?.workflowIntent ?? workflowIntent
            }
            data-dx-automation-run-receipt-path={launchAutomationRunReceiptPath}
            data-dx-automation-zed-run-state={
              zedHandoffReceipt ? "created" : "idle"
            }
          >
            {zedHandoffReceipt ? (
              <span>
                Zed handoff ready for{" "}
                <code>{zedHandoffReceipt.commands.run}</code>; mode{" "}
                <strong>{zedHandoffReceipt.mode}</strong>; status{" "}
                <strong>{zedHandoffReceipt.status}</strong>; receipt{" "}
                <code>{launchAutomationRunReceiptPath}</code>; receipt intent{" "}
                <strong>
                  {zedHandoffReceipt.workflowIntent || "No intent entered"}
                </strong>.
                Credentials stay app-owned and redacted until operator approval.
              </span>
            ) : (
              <span>
                Prepare a Rust/Zed handoff after the connector and workflow
                intent look right.
              </span>
            )}
          </div>
        </div>
      </div>

      <div className="flex flex-wrap gap-2">
        {automationRoutes.map((route) => (
          <Badge key={route.href} variant="outline">
            {route.href}
          </Badge>
        ))}
      </div>
    </div>
  );
}
