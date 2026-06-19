import type { ExecutionReadiness, WorkflowDocument } from "./types";
import type { RuntimeTriggerReceipt } from "./runtime-trigger";

export type RuntimeExecutionProofStatus = "proved" | "proof-blocked";

export type RuntimeExecutionProofTriggerSummary = {
  receiptPath: RuntimeTriggerReceipt["receiptPath"];
  triggerNodeId: string;
  triggerNodeName: string;
  triggerMode: "webhook";
  httpMethod: RuntimeTriggerReceipt["httpMethod"];
  targetOrigin: string;
  targetUrlStored: false;
  providerStatusCode?: number;
  providerResponseBodyStored: false;
};

export type RuntimeExecutionProofExecutionSummary = {
  status: ExecutionReadiness["status"];
  selectedAttemptId: string;
  attemptCount: number;
  nodeLogCount: number;
  receiptIssueCount: number;
  receiptPath?: string;
};

export type RuntimeExecutionProofReceipt = {
  schema: "dx.n8n-studio.runtime-execution-proof.receipt";
  status: RuntimeExecutionProofStatus;
  workflowId: string;
  workflowName: string;
  providerBoundary: true;
  workflowExecutionRequested: true;
  liveProviderExecution: true;
  executionReceiptImported: boolean;
  secretsIncluded: false;
  redaction: "secret-values-never-included";
  sideEffectPolicy: "governed-live-trigger-and-history-proof";
  trigger: RuntimeExecutionProofTriggerSummary;
  execution: RuntimeExecutionProofExecutionSummary;
  receiptPath: ".dx/receipts/n8n-studio/runtime/execution-proof-latest.sr";
  issue: string;
};

function proofStatus(execution: ExecutionReadiness): RuntimeExecutionProofStatus {
  if (
    execution.status === "blocked" ||
    !execution.receiptBoundary.executionReceiptImported
  ) {
    return "proof-blocked";
  }

  return "proved";
}

function triggerSummary(
  trigger: RuntimeTriggerReceipt,
): RuntimeExecutionProofTriggerSummary {
  return {
    receiptPath: trigger.receiptPath,
    triggerNodeId: trigger.triggerNodeId,
    triggerNodeName: trigger.triggerNodeName,
    triggerMode: trigger.triggerMode,
    httpMethod: trigger.httpMethod,
    targetOrigin: trigger.targetOrigin,
    targetUrlStored: trigger.targetUrlStored,
    providerStatusCode: trigger.providerResponse.statusCode,
    providerResponseBodyStored: trigger.providerResponse.bodyStored,
  };
}

function executionSummary(
  execution: ExecutionReadiness,
): RuntimeExecutionProofExecutionSummary {
  return {
    status: execution.status,
    selectedAttemptId: execution.selectedAttemptId,
    attemptCount: execution.attempts.length,
    nodeLogCount: execution.nodeLogs.length,
    receiptIssueCount: execution.receiptIssues.length,
    receiptPath: execution.receiptBoundary.receiptPath,
  };
}

export function createRuntimeExecutionProofReceipt(options: {
  document: WorkflowDocument;
  trigger: RuntimeTriggerReceipt;
  execution: ExecutionReadiness;
}): RuntimeExecutionProofReceipt {
  const status = proofStatus(options.execution);

  return {
    schema: "dx.n8n-studio.runtime-execution-proof.receipt",
    status,
    workflowId: options.document.id,
    workflowName: options.document.name,
    providerBoundary: true,
    workflowExecutionRequested: true,
    liveProviderExecution: true,
    executionReceiptImported:
      options.execution.receiptBoundary.executionReceiptImported,
    secretsIncluded: false,
    redaction: "secret-values-never-included",
    sideEffectPolicy: "governed-live-trigger-and-history-proof",
    trigger: triggerSummary(options.trigger),
    execution: executionSummary(options.execution),
    receiptPath: ".dx/receipts/n8n-studio/runtime/execution-proof-latest.sr",
    issue:
      status === "proved"
        ? "Webhook trigger request was submitted and n8n execution history was imported as proof. Secret-bearing trigger URLs, request bodies, and provider response bodies were not stored."
        : "Webhook trigger request was submitted, but execution proof is still blocked until a matching n8n execution receipt is imported.",
  };
}
