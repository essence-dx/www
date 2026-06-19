import type { ExecutionReadiness, WorkflowDocument } from "./types";
import type {
  WorkflowPublishOptions,
  WorkflowPublishReceipt,
} from "./workflow-publish";

export type RuntimeHandoffOptions = WorkflowPublishOptions & {
  importExecutionHistory?: boolean;
};

export type RuntimeHandoffStatus =
  | "submitted"
  | "submitted-with-history"
  | "submitted-history-blocked";

export type RuntimeHandoffExecutionSummary = {
  status: ExecutionReadiness["status"];
  selectedAttemptId: string;
  attemptCount: number;
  receiptIssueCount: number;
  receiptImported: boolean;
  receiptPath?: string;
  providerBoundary: true;
  liveProviderExecution: false;
  secretsIncluded: false;
};

export type RuntimeHandoffReceipt = {
  schema: "dx.n8n-studio.runtime-handoff.receipt";
  status: RuntimeHandoffStatus;
  workflowId: string;
  workflowName: string;
  nodeCount: number;
  connectionCount: number;
  requestCount: number;
  providerBoundary: true;
  providerWrite: true;
  workflowExecutionRequested: false;
  liveProviderExecution: false;
  executionHistoryImportRequested: boolean;
  executionReceiptImported: boolean;
  secretsIncluded: false;
  sideEffectPolicy: "governed-workflow-publish-and-history-import";
  runtimeBoundary: "n8n-backend-through-governed-dx-bridge";
  sourcePaths: [
    "nodes/N8n/WorkflowDescription.ts",
    "nodes/N8n/ExecutionDescription.ts",
  ];
  coveragePath: "nodes/N8n/n8n-api-coverage.json";
  receiptPath: ".dx/receipts/n8n-studio/runtime/handoff-latest.sr";
  redaction: "secret-values-never-included";
  publish: WorkflowPublishReceipt;
  execution?: RuntimeHandoffExecutionSummary;
  issue: string;
};

function executionSummary(
  readiness: ExecutionReadiness,
): RuntimeHandoffExecutionSummary {
  return {
    status: readiness.status,
    selectedAttemptId: readiness.selectedAttemptId,
    attemptCount: readiness.attempts.length,
    receiptIssueCount: readiness.receiptIssues.length,
    receiptImported: readiness.receiptBoundary.executionReceiptImported,
    receiptPath: readiness.receiptBoundary.receiptPath,
    providerBoundary: true,
    liveProviderExecution: false,
    secretsIncluded: false,
  };
}

function handoffStatus(options: {
  historyRequested: boolean;
  execution?: ExecutionReadiness;
}): RuntimeHandoffStatus {
  if (!options.historyRequested || !options.execution) {
    return "submitted";
  }

  if (
    options.execution.status === "blocked" ||
    !options.execution.receiptBoundary.executionReceiptImported
  ) {
    return "submitted-history-blocked";
  }

  return "submitted-with-history";
}

export function createRuntimeHandoffReceipt(options: {
  document: WorkflowDocument;
  publish: WorkflowPublishReceipt;
  execution?: ExecutionReadiness;
  executionHistoryImportRequested: boolean;
}): RuntimeHandoffReceipt {
  return {
    schema: "dx.n8n-studio.runtime-handoff.receipt",
    status: handoffStatus({
      historyRequested: options.executionHistoryImportRequested,
      execution: options.execution,
    }),
    workflowId: options.publish.workflowId,
    workflowName: options.document.name,
    nodeCount: options.document.nodes.length,
    connectionCount: options.document.connections.length,
    requestCount:
      options.publish.requestCount +
      (options.executionHistoryImportRequested ? 1 : 0),
    providerBoundary: true,
    providerWrite: true,
    workflowExecutionRequested: false,
    liveProviderExecution: false,
    executionHistoryImportRequested:
      options.executionHistoryImportRequested,
    executionReceiptImported:
      options.execution?.receiptBoundary.executionReceiptImported ?? false,
    secretsIncluded: false,
    sideEffectPolicy: "governed-workflow-publish-and-history-import",
    runtimeBoundary: "n8n-backend-through-governed-dx-bridge",
    sourcePaths: [
      "nodes/N8n/WorkflowDescription.ts",
      "nodes/N8n/ExecutionDescription.ts",
    ],
    coveragePath: "nodes/N8n/n8n-api-coverage.json",
    receiptPath: ".dx/receipts/n8n-studio/runtime/handoff-latest.sr",
    redaction: "secret-values-never-included",
    publish: options.publish,
    execution: options.execution
      ? executionSummary(options.execution)
      : undefined,
    issue:
      "Runtime handoff published the workflow through the governed n8n API bridge and, when requested, imported execution history as proof. Studio did not call a generic run endpoint; live provider execution must happen through n8n triggers or manual provider controls.",
  };
}
