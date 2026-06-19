import type {
  ExecutionAttemptStatus,
  ExecutionAttemptSummary,
  ExecutionNodeLogRow,
  ExecutionNodeLogStatus,
  ExecutionReadiness,
  ExecutionReceiptImportIssue,
  WorkflowDocument,
  WorkflowNode,
} from "./types";

const receiptRoot = ".dx/receipts/n8n-studio/executions";
const importedExecutionIssue =
  "Execution data comes from an imported receipt; Studio did not run a live provider in this session.";

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function stringValue(value: unknown) {
  return typeof value === "string" ? value : undefined;
}

function numberValue(value: unknown) {
  return typeof value === "number" && Number.isFinite(value) ? value : undefined;
}

function secretLikeKey(key: string) {
  return /(api[-_]?key|access[-_]?token|token|password|secret|private[-_]?key|client[-_]?secret|stack)/i.test(key);
}

function collectSecretIssues(
  value: unknown,
  issues: ExecutionReceiptImportIssue[],
  nodeName?: string,
) {
  if (Array.isArray(value)) {
    for (const item of value) {
      collectSecretIssues(item, issues, nodeName);
    }
    return;
  }

  if (!isRecord(value)) {
    return;
  }

  for (const [key, child] of Object.entries(value)) {
    if (secretLikeKey(key)) {
      issues.push({
        code: "secret-field-stripped",
        severity: "warning",
        message: "Removed a secret-like execution receipt field.",
        nodeName,
      });
      continue;
    }

    collectSecretIssues(child, issues, nodeName);
  }
}

function attemptStatus(value: unknown): ExecutionAttemptStatus {
  if (
    value === "queued" ||
    value === "running" ||
    value === "success" ||
    value === "error" ||
    value === "cancelled"
  ) {
    return value;
  }

  return "blocked";
}

function nodeLogStatus(value: unknown): ExecutionNodeLogStatus {
  if (
    value === "success" ||
    value === "ready" ||
    value === "blocked" ||
    value === "waiting" ||
    value === "skipped" ||
    value === "error"
  ) {
    return value;
  }

  return "blocked";
}

function nodeLogReceipts(receipt: Record<string, unknown>) {
  return Array.isArray(receipt.nodeLogs) ? receipt.nodeLogs : [];
}

function nodeByReceiptReference(
  document: WorkflowDocument,
  receipt: Record<string, unknown>,
) {
  const nodeId = stringValue(receipt.nodeId);
  const nodeName = stringValue(receipt.nodeName);

  return document.nodes.find(
    (node) => node.id === nodeId || node.name === nodeName,
  );
}

function dataPreviewLabel(receipt: Record<string, unknown>) {
  const preview = receipt.dataPreview;
  if (isRecord(preview)) {
    return stringValue(preview.label) ?? "Imported execution data available";
  }

  return "Imported execution data available";
}

function providerErrorMessage(receipt: Record<string, unknown>) {
  const providerError = receipt.providerError;
  if (!isRecord(providerError)) {
    return undefined;
  }

  return stringValue(providerError.message);
}

function fallbackNodeLog(node: WorkflowNode, attemptId: string): ExecutionNodeLogRow {
  return {
    id: `${attemptId}-${node.id}`,
    attemptId,
    nodeId: node.id,
    nodeName: node.name,
    nodeType: node.type,
    status: "blocked",
    inputItemCount: 0,
    outputItemCount: 0,
    dataPreviewLabel: "No imported execution data for this node",
    redaction: "secret-values-never-included",
    issue: "No matching node log was present in the imported execution receipt.",
  };
}

function importedNodeLog(
  node: WorkflowNode,
  executionId: string,
  logReceipt: Record<string, unknown>,
): ExecutionNodeLogRow {
  return {
    id: `${executionId}-${node.id}`,
    attemptId: executionId,
    nodeId: node.id,
    nodeName: node.name,
    nodeType: node.type,
    status: nodeLogStatus(logReceipt.status),
    inputItemCount: numberValue(logReceipt.inputItemCount) ?? 0,
    outputItemCount: numberValue(logReceipt.outputItemCount) ?? 0,
    durationMs: numberValue(logReceipt.durationMs),
    dataPreviewLabel: dataPreviewLabel(logReceipt),
    redaction: "secret-values-never-included",
    providerErrorMessage: providerErrorMessage(logReceipt),
    issue: importedExecutionIssue,
  };
}

export function importExecutionReceipt(
  document: WorkflowDocument,
  importedReceipt: unknown,
): ExecutionReadiness {
  const receipt = isRecord(importedReceipt) ? importedReceipt : {};
  const issues: ExecutionReceiptImportIssue[] = [];
  const receiptWorkflowId = stringValue(receipt.workflowId);
  const executionId = stringValue(receipt.executionId) ?? "imported-execution";
  const receiptPath = stringValue(receipt.receiptPath) ?? `${receiptRoot}/imported.sr`;

  if (receiptWorkflowId && receiptWorkflowId !== document.id) {
    issues.push({
      code: "workflow-id-mismatch",
      severity: "blocker",
      message: `Imported execution receipt targets workflow "${receiptWorkflowId}".`,
    });
  }

  const importedLogs = new Map<string, ExecutionNodeLogRow>();
  for (const logReceipt of nodeLogReceipts(receipt)) {
    if (!isRecord(logReceipt)) {
      issues.push({
        code: "malformed-node-log",
        severity: "warning",
        message: "Dropped malformed execution node log from imported receipt.",
      });
      continue;
    }

    const nodeName = stringValue(logReceipt.nodeName);
    collectSecretIssues(logReceipt, issues, nodeName);
    const node = nodeByReceiptReference(document, logReceipt);
    if (!node) {
      issues.push({
        code: "unknown-node-log",
        severity: "warning",
        message: `Dropped execution log for unknown node "${nodeName ?? "unknown"}".`,
        nodeName,
      });
      continue;
    }

    importedLogs.set(node.id, importedNodeLog(node, executionId, logReceipt));
  }

  const attempt: ExecutionAttemptSummary = {
    id: executionId,
    workflowId: receiptWorkflowId ?? document.id,
    workflowName: document.name,
    mode:
      receipt.mode === "partial" || receipt.mode === "webhook"
        ? receipt.mode
        : "manual",
    status: attemptStatus(receipt.status),
    triggerNodeId: document.nodes.find((node) =>
      node.type.toLowerCase().includes("trigger"),
    )?.id,
    selectedNodeId: document.nodes[1]?.id ?? document.nodes[0]?.id,
    startedAt: stringValue(receipt.startedAt),
    finishedAt: stringValue(receipt.finishedAt),
    durationMs: numberValue(receipt.durationMs),
    inputItemCount: document.nodes.reduce(
      (total, node) => total + (importedLogs.get(node.id)?.inputItemCount ?? 0),
      0,
    ),
    outputItemCount: document.nodes.reduce(
      (total, node) => total + (importedLogs.get(node.id)?.outputItemCount ?? 0),
      0,
    ),
    receiptPath,
    providerBoundary: true,
    liveProviderExecution: false,
    secretsIncluded: false,
    issue: importedExecutionIssue,
  };

  return {
    status: issues.some((issue) => issue.severity === "blocker")
      ? "blocked"
      : "configured",
    providerBoundary: true,
    liveProviderExecution: false,
    activeDebugView: "logs",
    debugViews: ["validation", "runs", "logs", "receipts"],
    availableActions: [
      "validate",
      "export",
      "inspect-readiness",
      "inspect-node-logs",
      "import-execution-receipt",
      "import-execution-history",
    ],
    selectedAttemptId: attempt.id,
    attempts: [attempt],
    nodeLogs: document.nodes.map(
      (node) => importedLogs.get(node.id) ?? fallbackNodeLog(node, attempt.id),
    ),
    receiptBoundary: {
      providerBoundary: true,
      liveProviderExecution: false,
      executionReceiptImported: true,
      secretsIncluded: false,
      receiptRoot,
      receiptPath,
      importedAt: stringValue(receipt.importedAt),
      issue:
        "Execution receipt imported for local debugging; Studio still did not execute a live provider call.",
    },
    receiptIssues: issues,
    blockedReason: importedExecutionIssue,
  };
}
