import type {
  ExecutionAttemptSummary,
  ExecutionNodeLogRow,
  ExecutionReadiness,
  WorkflowDocument,
  WorkflowNode,
} from "./types";
import { studioWorkflowDocument } from "./workflow-document";

export { importExecutionReceipt } from "./execution-receipt-import";
export {
  createExecutionHistoryApiRequestPlan,
  createExecutionReceiptFromHistoryApi,
  importExecutionHistoryApiResponse,
} from "./execution-history-api";

const localReadinessAttemptId = "attempt-local-readiness";
const executionReceiptPath = ".dx/receipts/n8n-studio/executions/local-readiness.sr";
const executionBlockedReason =
  "Live runs require n8n provider configuration, credential readiness, project scope, and receipt import.";
const executionAvailableActions = [
  "validate",
  "export",
  "inspect-readiness",
  "inspect-node-logs",
  "import-execution-receipt",
  "import-execution-history",
] as const;

function nodeCanValidateWithoutProvider(node: WorkflowNode) {
  return node.type === "n8n-nodes-base.manualTrigger";
}

function countPinnedItems(document: WorkflowDocument, nodeName: string) {
  return document.pinData[nodeName]?.length ?? 0;
}

function createAttempt(document: WorkflowDocument): ExecutionAttemptSummary {
  const triggerNode = document.nodes.find((node) =>
    node.type.toLowerCase().includes("trigger"),
  );

  return {
    id: localReadinessAttemptId,
    workflowId: document.id,
    workflowName: document.name,
    mode: "manual",
    status: "blocked",
    triggerNodeId: triggerNode?.id,
    selectedNodeId: document.nodes[1]?.id ?? document.nodes[0]?.id,
    inputItemCount: 0,
    outputItemCount: 0,
    receiptPath: executionReceiptPath,
    providerBoundary: true,
    liveProviderExecution: false,
    secretsIncluded: false,
    issue: executionBlockedReason,
  };
}

function createNodeLog(
  document: WorkflowDocument,
  node: WorkflowNode,
  attemptId: string,
): ExecutionNodeLogRow {
  const sourceOnlyReady = nodeCanValidateWithoutProvider(node);
  const pinnedItemCount = countPinnedItems(document, node.name);

  return {
    id: `${attemptId}-${node.id}`,
    attemptId,
    nodeId: node.id,
    nodeName: node.name,
    nodeType: node.type,
    status: sourceOnlyReady ? "ready" : "blocked",
    inputItemCount: pinnedItemCount,
    outputItemCount: 0,
    dataPreviewLabel:
      pinnedItemCount > 0
        ? `${pinnedItemCount} pinned item available for local inspection`
        : "No execution data imported",
    redaction: "secret-values-never-included",
    issue: sourceOnlyReady
      ? "Node can be inspected from source metadata, but no live run has executed."
      : "Node output is blocked until provider configuration and an execution receipt exist.",
  };
}

export function createExecutionReadiness(
  document: WorkflowDocument,
): ExecutionReadiness {
  const attempt = createAttempt(document);

  return {
    status: "blocked",
    providerBoundary: true,
    liveProviderExecution: false,
    activeDebugView: "validation",
    debugViews: ["validation", "runs", "logs", "receipts"],
    availableActions: [...executionAvailableActions],
    selectedAttemptId: attempt.id,
    attempts: [attempt],
    nodeLogs: document.nodes.map((node) => createNodeLog(document, node, attempt.id)),
    receiptBoundary: {
      providerBoundary: true,
      liveProviderExecution: false,
      executionReceiptImported: false,
      secretsIncluded: false,
      receiptRoot: ".dx/receipts/n8n-studio/executions",
      issue:
        "Execution receipts must be imported before Studio can claim node output, timings, or provider-side errors.",
    },
    receiptIssues: [],
    blockedReason: executionBlockedReason,
  };
}

export const executionReadiness: ExecutionReadiness =
  createExecutionReadiness(studioWorkflowDocument);
