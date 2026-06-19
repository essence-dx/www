import fs from "node:fs";
import path from "node:path";

import {
  createRuntimeExecutionProofReceipt,
  RuntimeExecutionProofReceipt,
} from "../../lib/n8n-studio/runtime-execution-proof";
import type { RuntimeTriggerReceipt } from "../../lib/n8n-studio/runtime-trigger";
import type {
  ExecutionReadiness,
  WorkflowDocument,
} from "../../lib/n8n-studio/types";

const relativeExecutionProofReceiptPath =
  "n8n-studio/runtime/execution-proof-latest.sr";
const defaultRetryMaxAttempts = 4;
const retryBaseDelayMs = 1000;
const retryMaxDelayMs = 30000;

export type RuntimeExecutionProofReceiptExport = {
  schema: "dx.n8n-studio.runtime-execution-proof.export";
  receiptRoot: string;
  receiptPath: string;
  relativeReceiptPath: typeof relativeExecutionProofReceiptPath;
  status: RuntimeExecutionProofReceipt["status"];
  providerBoundary: true;
  liveProviderExecution: false;
  secretsIncluded: false;
  redaction: "secret-values-never-included";
  runtimeProof: {
    liveProviderExecution: RuntimeExecutionProofReceipt["liveProviderExecution"];
    executionReceiptImported:
      RuntimeExecutionProofReceipt["executionReceiptImported"];
  };
  issue: string;
};

export type RuntimeExecutionProofRetryPlanStatus =
  | "complete"
  | "scheduled"
  | "exhausted";

export type RuntimeExecutionProofRetryPlan = {
  schema: "dx.n8n-studio.runtime-execution-proof.retry-plan";
  status: RuntimeExecutionProofRetryPlanStatus;
  workflowId: string;
  workflowName: string;
  providerBoundary: true;
  liveProviderExecution: false;
  secretsIncluded: false;
  historyImportCredentialType: "n8nApi";
  sideEffectPolicy: "read-only-history-import";
  receiptPath: RuntimeExecutionProofReceipt["receiptPath"];
  attempt: number;
  maxAttempts: number;
  nextAction:
    | "none"
    | "import-execution-history"
    | "operator-review-required";
  nextAttempt?: {
    attempt: number;
    delayMs: number;
  };
  sourcePath: "nodes/N8n/ExecutionDescription.ts";
  coveragePath: "nodes/N8n/n8n-api-coverage.json";
  redaction: "secret-values-never-included";
  issue: string;
};

export type RuntimeExecutionProofReceiptWriteOptions = {
  receiptRoot?: string;
  proof: RuntimeExecutionProofReceipt;
};

export type RuntimeExecutionProofRetryPlanOptions = {
  proof: RuntimeExecutionProofReceipt;
  attempt?: number;
  maxAttempts?: number;
};

export type RuntimeExecutionProofHistoryImportRequest = {
  attempt: number;
  maxAttempts: number;
  previousProofStatus?: RuntimeExecutionProofReceipt["status"];
};

export type RuntimeExecutionProofHistoryImporter = (
  request: RuntimeExecutionProofHistoryImportRequest,
) => Promise<ExecutionReadiness>;

export type RuntimeExecutionProofRetryRunOptions = {
  document: WorkflowDocument;
  trigger: RuntimeTriggerReceipt;
  receiptRoot?: string;
  maxAttempts?: number;
  importExecutionHistory: RuntimeExecutionProofHistoryImporter;
};

export type RuntimeExecutionProofRetryResult = {
  schema: "dx.n8n-studio.runtime-execution-proof.retry-result";
  status: "proved" | "exhausted";
  workflowId: string;
  workflowName: string;
  attemptCount: number;
  maxAttempts: number;
  retryPlans: RuntimeExecutionProofRetryPlan[];
  proof: RuntimeExecutionProofReceipt;
  receiptExport: RuntimeExecutionProofReceiptExport;
  providerBoundary: true;
  liveProviderExecution: false;
  secretsIncluded: false;
  redaction: "secret-values-never-included";
  issue: string;
};

function defaultReceiptRoot() {
  return path.join(process.cwd(), ".dx", "receipts");
}

function executionProofReceiptPath(receiptRoot: string) {
  return path.join(
    receiptRoot,
    ...relativeExecutionProofReceiptPath.split("/"),
  );
}

function writeJsonReceipt(filePath: string, value: unknown) {
  fs.mkdirSync(path.dirname(filePath), { recursive: true });
  fs.writeFileSync(filePath, `${JSON.stringify(value, null, 2)}\n`, "utf8");
}

function normalizedAttempt(value: number | undefined) {
  if (value === undefined || !Number.isFinite(value)) {
    return 0;
  }

  return Math.max(0, Math.floor(value));
}

function normalizedMaxAttempts(value: number | undefined) {
  if (value === undefined || !Number.isFinite(value)) {
    return defaultRetryMaxAttempts;
  }

  return Math.max(1, Math.floor(value));
}

function retryDelayMs(attempt: number) {
  return Math.min(retryMaxDelayMs, retryBaseDelayMs * 2 ** attempt);
}

function normalizedProof(
  proof: RuntimeExecutionProofReceipt,
): RuntimeExecutionProofReceipt {
  return {
    schema: "dx.n8n-studio.runtime-execution-proof.receipt",
    status: proof.status,
    workflowId: proof.workflowId,
    workflowName: proof.workflowName,
    providerBoundary: true,
    workflowExecutionRequested: proof.workflowExecutionRequested,
    liveProviderExecution: proof.liveProviderExecution,
    executionReceiptImported: proof.executionReceiptImported,
    secretsIncluded: false,
    redaction: "secret-values-never-included",
    sideEffectPolicy: "governed-live-trigger-and-history-proof",
    trigger: {
      receiptPath: proof.trigger.receiptPath,
      triggerNodeId: proof.trigger.triggerNodeId,
      triggerNodeName: proof.trigger.triggerNodeName,
      triggerMode: "webhook",
      httpMethod: proof.trigger.httpMethod,
      targetOrigin: proof.trigger.targetOrigin,
      targetUrlStored: false,
      providerStatusCode: proof.trigger.providerStatusCode,
      providerResponseBodyStored: false,
    },
    execution: {
      status: proof.execution.status,
      selectedAttemptId: proof.execution.selectedAttemptId,
      attemptCount: proof.execution.attemptCount,
      nodeLogCount: proof.execution.nodeLogCount,
      receiptIssueCount: proof.execution.receiptIssueCount,
      receiptPath: proof.execution.receiptPath,
    },
    receiptPath: ".dx/receipts/n8n-studio/runtime/execution-proof-latest.sr",
    issue: proof.issue,
  };
}

export function writeRuntimeExecutionProofReceipt(
  options: RuntimeExecutionProofReceiptWriteOptions,
): RuntimeExecutionProofReceiptExport {
  const receiptRoot = path.resolve(options.receiptRoot ?? defaultReceiptRoot());
  const receiptPath = executionProofReceiptPath(receiptRoot);
  const proof = normalizedProof(options.proof);

  writeJsonReceipt(receiptPath, proof);

  return {
    schema: "dx.n8n-studio.runtime-execution-proof.export",
    receiptRoot,
    receiptPath,
    relativeReceiptPath: relativeExecutionProofReceiptPath,
    status: proof.status,
    providerBoundary: true,
    liveProviderExecution: false,
    secretsIncluded: false,
    redaction: "secret-values-never-included",
    runtimeProof: {
      liveProviderExecution: proof.liveProviderExecution,
      executionReceiptImported: proof.executionReceiptImported,
    },
    issue:
      "Runtime execution proof receipt was written for DX/Zed discovery. The writer does not call n8n; delayed provider history must be imported through the governed n8n API retry plan.",
  };
}

export function createRuntimeExecutionProofRetryPlan(
  options: RuntimeExecutionProofRetryPlanOptions,
): RuntimeExecutionProofRetryPlan {
  const attempt = normalizedAttempt(options.attempt);
  const maxAttempts = normalizedMaxAttempts(options.maxAttempts);
  const proofComplete = options.proof.status === "proved";
  const exhausted = !proofComplete && attempt >= maxAttempts;
  const scheduled = !proofComplete && !exhausted;

  return {
    schema: "dx.n8n-studio.runtime-execution-proof.retry-plan",
    status: proofComplete ? "complete" : exhausted ? "exhausted" : "scheduled",
    workflowId: options.proof.workflowId,
    workflowName: options.proof.workflowName,
    providerBoundary: true,
    liveProviderExecution: false,
    secretsIncluded: false,
    historyImportCredentialType: "n8nApi",
    sideEffectPolicy: "read-only-history-import",
    receiptPath: options.proof.receiptPath,
    attempt,
    maxAttempts,
    nextAction: proofComplete
      ? "none"
      : exhausted
        ? "operator-review-required"
        : "import-execution-history",
    ...(scheduled
      ? {
          nextAttempt: {
            attempt: attempt + 1,
            delayMs: retryDelayMs(attempt),
          },
        }
      : {}),
    sourcePath: "nodes/N8n/ExecutionDescription.ts",
    coveragePath: "nodes/N8n/n8n-api-coverage.json",
    redaction: "secret-values-never-included",
    issue: proofComplete
      ? "Execution proof is already imported. No retry is required."
      : exhausted
        ? "Execution proof is still missing after the configured import attempts. Operator review is required before enabling live automation."
        : "Execution proof is waiting for n8n execution history to become available. Retry the governed read-only history import after the delay.",
  };
}

export async function runRuntimeExecutionProofRetry(
  options: RuntimeExecutionProofRetryRunOptions,
): Promise<RuntimeExecutionProofRetryResult> {
  const maxAttempts = normalizedMaxAttempts(options.maxAttempts);
  const retryPlans: RuntimeExecutionProofRetryPlan[] = [];
  let latestProof: RuntimeExecutionProofReceipt | undefined;

  for (let attempt = 1; attempt <= maxAttempts; attempt += 1) {
    const execution = await options.importExecutionHistory({
      attempt,
      maxAttempts,
      previousProofStatus: latestProof?.status,
    });
    latestProof = createRuntimeExecutionProofReceipt({
      document: options.document,
      trigger: options.trigger,
      execution,
    });

    if (latestProof.status === "proved") {
      return {
        schema: "dx.n8n-studio.runtime-execution-proof.retry-result",
        status: "proved",
        workflowId: options.document.id,
        workflowName: options.document.name,
        attemptCount: attempt,
        maxAttempts,
        retryPlans,
        proof: latestProof,
        receiptExport: writeRuntimeExecutionProofReceipt({
          receiptRoot: options.receiptRoot,
          proof: latestProof,
        }),
        providerBoundary: true,
        liveProviderExecution: false,
        secretsIncluded: false,
        redaction: "secret-values-never-included",
        issue:
          "Execution proof became available through a governed read-only n8n history import retry. The retry executor did not submit a workflow run or store secret-bearing provider payloads.",
      };
    }

    retryPlans.push(
      createRuntimeExecutionProofRetryPlan({
        proof: latestProof,
        attempt,
        maxAttempts,
      }),
    );
  }

  if (!latestProof) {
    throw new Error("Runtime execution proof retry requires at least one attempt.");
  }

  return {
    schema: "dx.n8n-studio.runtime-execution-proof.retry-result",
    status: "exhausted",
    workflowId: options.document.id,
    workflowName: options.document.name,
    attemptCount: maxAttempts,
    maxAttempts,
    retryPlans,
    proof: latestProof,
    receiptExport: writeRuntimeExecutionProofReceipt({
      receiptRoot: options.receiptRoot,
      proof: latestProof,
    }),
    providerBoundary: true,
    liveProviderExecution: false,
    secretsIncluded: false,
    redaction: "secret-values-never-included",
    issue:
      "Execution proof was still unavailable after the bounded read-only n8n history import attempts. Keep live automation disabled until a later imported receipt proves the execution.",
  };
}
