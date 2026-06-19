import type {
  ExecutionAttemptStatus,
  ExecutionReadiness,
  WorkflowDocument,
} from "./types";
import { importExecutionReceipt } from "./execution-receipt-import";

const receiptRoot = ".dx/receipts/n8n-studio/executions";
const sourcePath = "nodes/N8n/ExecutionDescription.ts";
const coveragePath = "nodes/N8n/n8n-api-coverage.json";

type ExecutionHistoryApiRequest = {
  method: "GET";
  path: "/executions" | "/executions/{id}";
  query: Record<string, string | number | boolean>;
};

export type ExecutionHistoryApiRequestPlan = {
  providerBoundary: true;
  liveProviderExecution: false;
  secretsIncluded: false;
  credentialType: "n8nApi";
  sideEffectPolicy: "read-only-history-import";
  sourcePath: typeof sourcePath;
  coveragePath: typeof coveragePath;
  requests: ExecutionHistoryApiRequest[];
};

export type ExecutionHistoryReceipt = {
  schema: "dx.n8n-studio.execution.receipt";
  executionId: string;
  workflowId?: string;
  mode: "manual" | "partial" | "webhook";
  status: ExecutionAttemptStatus;
  startedAt?: string;
  finishedAt?: string;
  durationMs?: number;
  receiptPath: string;
  nodeLogs: ExecutionHistoryNodeLogReceipt[];
};

type ExecutionHistoryNodeLogReceipt = {
  nodeName: string;
  status: "success" | "error" | "blocked";
  inputItemCount: number;
  outputItemCount: number;
  durationMs?: number;
  dataPreview: {
    label: string;
  };
  providerError?: {
    message: string;
  };
};

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function stringValue(value: unknown) {
  return typeof value === "string" ? value : undefined;
}

function numberValue(value: unknown) {
  return typeof value === "number" && Number.isFinite(value) ? value : undefined;
}

function recordValue(value: unknown) {
  return isRecord(value) ? value : undefined;
}

function arrayValue(value: unknown) {
  return Array.isArray(value) ? value : [];
}

function selectedExecutionRecord(response: unknown) {
  const responseRecord = recordValue(response);
  const data = responseRecord ? arrayValue(responseRecord.data) : [];
  return recordValue(data[0]) ?? responseRecord ?? {};
}

function resultDataRecord(execution: Record<string, unknown>) {
  const data = recordValue(execution.data);
  return data ? recordValue(data.resultData) : undefined;
}

function runDataRecord(execution: Record<string, unknown>) {
  return resultDataRecord(execution)?.runData;
}

function countMainItems(runDataItem: Record<string, unknown>) {
  const data = recordValue(runDataItem.data);
  const mainOutputs = arrayValue(data?.main);

  return mainOutputs.reduce((total, output) => {
    const outputItems = arrayValue(output);
    return total + outputItems.length;
  }, 0);
}

function providerErrorMessage(runDataItem: Record<string, unknown>) {
  return stringValue(recordValue(runDataItem.error)?.message);
}

function nodeLogStatus(runDataItem: Record<string, unknown>) {
  return providerErrorMessage(runDataItem) ? "error" : "success";
}

function executionStatus(value: unknown): ExecutionAttemptStatus {
  if (
    value === "queued" ||
    value === "running" ||
    value === "success" ||
    value === "error" ||
    value === "cancelled"
  ) {
    return value;
  }

  if (value === "waiting") {
    return "running";
  }

  return "blocked";
}

function dataPreviewLabel(outputItemCount: number, hasProviderError: boolean) {
  if (hasProviderError) {
    return "n8n API history captured a provider error";
  }

  if (outputItemCount === 1) {
    return "n8n API history reported 1 output item";
  }

  return `n8n API history reported ${outputItemCount} output items`;
}

function historyNodeLogs(execution: Record<string, unknown>) {
  const runData = recordValue(runDataRecord(execution));
  if (!runData) {
    return [];
  }

  const logs: ExecutionHistoryNodeLogReceipt[] = [];
  for (const [nodeName, nodeExecutions] of Object.entries(runData)) {
    const executionItems = arrayValue(nodeExecutions);
    const runDataItem = recordValue(executionItems[executionItems.length - 1]);
    if (!runDataItem) {
      continue;
    }

    const outputItemCount = countMainItems(runDataItem);
    const errorMessage = providerErrorMessage(runDataItem);
    const logReceipt: ExecutionHistoryNodeLogReceipt = {
      nodeName,
      status: nodeLogStatus(runDataItem),
      inputItemCount: 0,
      outputItemCount,
      durationMs: numberValue(runDataItem.executionTime),
      dataPreview: {
        label: dataPreviewLabel(outputItemCount, Boolean(errorMessage)),
      },
    };

    if (errorMessage) {
      logReceipt.providerError = { message: errorMessage };
    }

    logs.push(logReceipt);
  }

  return logs;
}

function executionMode(value: unknown): "manual" | "partial" | "webhook" {
  if (value === "partial" || value === "webhook") {
    return value;
  }

  return "manual";
}

function executionDurationMs(execution: Record<string, unknown>) {
  const explicitDuration =
    numberValue(execution.durationMs) ?? numberValue(execution.executionTime);
  if (explicitDuration !== undefined) {
    return explicitDuration;
  }

  const startedAt = stringValue(execution.startedAt);
  const finishedAt =
    stringValue(execution.stoppedAt) ?? stringValue(execution.finishedAt);
  if (!startedAt || !finishedAt) {
    return undefined;
  }

  const started = Date.parse(startedAt);
  const finished = Date.parse(finishedAt);
  if (
    !Number.isFinite(started) ||
    !Number.isFinite(finished) ||
    finished < started
  ) {
    return undefined;
  }

  return finished - started;
}

function receiptExecutionId(execution: Record<string, unknown>) {
  return stringValue(execution.id) ?? "api-history-import";
}

function receiptFileName(executionId: string) {
  const safeId = executionId.replace(/[^a-zA-Z0-9._-]+/g, "-");
  return `api-history-${safeId || "execution"}.sr`;
}

export function createExecutionHistoryApiRequestPlan(
  document: WorkflowDocument,
): ExecutionHistoryApiRequestPlan {
  return {
    providerBoundary: true,
    liveProviderExecution: false,
    secretsIncluded: false,
    credentialType: "n8nApi",
    sideEffectPolicy: "read-only-history-import",
    sourcePath,
    coveragePath,
    requests: [
      {
        method: "GET",
        path: "/executions",
        query: {
          workflowId: document.id,
          includeData: true,
          limit: 100,
        },
      },
      {
        method: "GET",
        path: "/executions/{id}",
        query: {
          includeData: true,
        },
      },
    ],
  };
}

export function createExecutionReceiptFromHistoryApi(
  document: WorkflowDocument,
  response: unknown,
): ExecutionHistoryReceipt {
  const execution = selectedExecutionRecord(response);
  const executionId = receiptExecutionId(execution);
  const finishedAt =
    stringValue(execution.stoppedAt) ?? stringValue(execution.finishedAt);

  return {
    schema: "dx.n8n-studio.execution.receipt",
    executionId,
    workflowId: stringValue(execution.workflowId) ?? document.id,
    mode: executionMode(execution.mode),
    status: executionStatus(execution.status),
    startedAt: stringValue(execution.startedAt),
    finishedAt,
    durationMs: executionDurationMs(execution),
    receiptPath: `${receiptRoot}/${receiptFileName(executionId)}`,
    nodeLogs: historyNodeLogs(execution),
  };
}

export function importExecutionHistoryApiResponse(
  document: WorkflowDocument,
  response: unknown,
): ExecutionReadiness {
  return importExecutionReceipt(
    document,
    createExecutionReceiptFromHistoryApi(document, response),
  );
}
