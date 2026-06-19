import { createN8nStudioState } from "../../lib/n8n-studio/studio-state";
import type { N8nStudioState } from "../../lib/n8n-studio/types";
import { createCredentialVaultReadiness } from "../../lib/n8n-studio/credential-vault";
import { createExecutionHistoryApiRequestPlan } from "../../lib/n8n-studio/executions";
import { createN8nApiClientReadiness } from "../../lib/n8n-studio/n8n-api-client";
import { createRuntimeTriggerPlan } from "../../lib/n8n-studio/runtime-trigger";
import { createAutomationScope } from "./automation-scope";
import { createStudioBootFromLocalGeneratedSource } from "./generated-catalog-source";

export function createReadinessResponse(
  state: N8nStudioState = createN8nStudioState(),
) {
  const credentialVault = createCredentialVaultReadiness();
  const historyImportPlan = createExecutionHistoryApiRequestPlan(state.document);
  const n8nApiClient = createN8nApiClientReadiness();
  const runtimeTrigger = createRuntimeTriggerPlan(state.document);

  return {
    schema: "dx.n8n-studio.readiness",
    ok: true,
    status: "source-owned-preview",
    providerBoundary: state.execution.providerBoundary,
    liveProviderExecution: state.execution.liveProviderExecution,
    catalogNodeCount: state.catalog.catalogNodes.length,
    catalogNodeFiles: state.catalog.nodeFileCount,
    credentialFiles: state.catalog.credentialFileCount,
    generatedMetadata: state.catalog.generatedMetadata,
    automationScope: createAutomationScope(),
    configuredCredentialReferences: state.credentials.filter(
      (credential) => credential.status === "configured",
    ).length,
    blockedReason: state.execution.blockedReason,
    executionDebug: {
      selectedAttemptId: state.execution.selectedAttemptId,
      attemptCount: state.execution.attempts.length,
      nodeLogCount: state.execution.nodeLogs.length,
      receiptIssueCount: state.execution.receiptIssues.length,
      executionReceiptImported:
        state.execution.receiptBoundary.executionReceiptImported,
      receiptRoot: state.execution.receiptBoundary.receiptRoot,
      receiptImportedAt: state.execution.receiptBoundary.importedAt,
      historyImportAvailable:
        state.execution.availableActions.includes("import-execution-history"),
      historyImportEndpoint: historyImportPlan.requests[0]?.path,
      historyImportCredentialType: historyImportPlan.credentialType,
    },
    credentialVault,
    n8nApiClient,
    runtimeTrigger,
    editorSession: {
      status: state.editorSession.status,
      selectedNodeId: state.editorSession.selectedNodeId,
      selectedNodeName: state.editorSession.selectedNodeName,
      nodeType: state.editorSession.nodeType,
      requestPlanCount: state.editorSession.requestPlans.length,
      dynamicParameterLoadCount:
        state.editorSession.dynamicParameterLoadCount,
      resourceLocatorSearchCount:
        state.editorSession.resourceLocatorSearchCount,
      resourceMapperRequestCount:
        state.editorSession.resourceMapperRequestCount,
      credentialRequestCount: state.editorSession.credentialRequestCount,
      credentialValidationRequestCount:
        state.editorSession.credentialValidationRequestCount,
      fulfilledRequestCount: state.editorSession.fulfilledRequestCount,
      providerBoundary: state.editorSession.providerBoundary,
      liveProviderExecution: state.editorSession.liveProviderExecution,
      secretsIncluded: state.editorSession.secretsIncluded,
      redaction: state.editorSession.redaction,
      issue: state.editorSession.issue,
    },
    receiptRoot: state.receipts.receiptRoot,
    redaction: state.receipts.redaction,
    surfaces: state.receipts.surfaces,
  };
}

export function createReadinessResponseFromLocalGeneratedSource(
  startDirectory = process.cwd(),
) {
  const boot = createStudioBootFromLocalGeneratedSource(startDirectory);

  return createReadinessResponse(boot.state);
}
