import type {
  CurrentWorkflowExportState,
  ExportReceiptDetail,
  N8nStudioState,
  WorkflowDocument,
} from "./types";

const exportRoutePath = "/api/n8n-studio/export" as const;

function baseCurrentWorkflowExportState(
  patch: Partial<CurrentWorkflowExportState>,
): CurrentWorkflowExportState {
  return {
    status: "idle",
    routePath: exportRoutePath,
    providerBoundary: true,
    liveProviderExecution: false,
    secretsIncluded: false,
    redaction: "secret-values-never-included",
    issue: "Current workflow export has not been requested in this editor session.",
    ...patch,
  };
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return Boolean(value) && typeof value === "object" && !Array.isArray(value);
}

function stringValue(value: unknown) {
  return typeof value === "string" && value.trim() ? value : undefined;
}

function numberValue(value: unknown) {
  return typeof value === "number" && Number.isFinite(value) ? value : undefined;
}

function safeErrorMessage(error: unknown) {
  const message =
    error instanceof Error
      ? error.message
      : typeof error === "string"
        ? error
        : undefined;

  if (!message) {
    return "Current workflow export failed.";
  }

  return message.replace(
    /(api[-_]?key|token|password|secret|private[-_]?key|client[-_]?secret)[^,\s]*/gi,
    "[redacted]",
  );
}

function exportReceiptFromResponse(response: unknown) {
  if (!isRecord(response) || response.schema !== "dx.n8n-studio.export") {
    return undefined;
  }

  const receipt = response.receipt;
  if (!isRecord(receipt) || receipt.schema !== "dx.n8n-studio.export.receipt") {
    return undefined;
  }

  return receipt;
}

export function createIdleCurrentWorkflowExportState(
  document?: WorkflowDocument,
): CurrentWorkflowExportState {
  return baseCurrentWorkflowExportState({
    workflowName: document?.name,
    nodeCount: document?.nodes.length,
    connectionCount: document?.connections.length,
  });
}

export function createPendingCurrentWorkflowExportState(
  previous?: CurrentWorkflowExportState,
): CurrentWorkflowExportState {
  return baseCurrentWorkflowExportState({
    ...previous,
    status: "exporting",
    issue:
      "Current workflow export is being generated from the local editor document.",
    errorMessage: undefined,
  });
}

export function createCurrentWorkflowExportStateFromResponse(
  response: unknown,
  exportedAt = new Date().toISOString(),
): CurrentWorkflowExportState {
  const receipt = exportReceiptFromResponse(response);
  if (!receipt) {
    return createFailedCurrentWorkflowExportState(
      "Current workflow export returned an unexpected response shape.",
    );
  }

  return baseCurrentWorkflowExportState({
    status: "exported",
    exportedAt,
    responseSchema: "dx.n8n-studio.export",
    workflowName: stringValue(receipt.workflowName),
    downloadName: stringValue(receipt.downloadName),
    nodeCount: numberValue(receipt.nodeCount),
    connectionCount: numberValue(receipt.connectionCount),
    credentialReferenceCount: numberValue(receipt.credentialReferenceCount),
    issue:
      "Current workflow JSON was exported from the local editor state and remains non-executable until runtime receipts are imported.",
  });
}

export function createFailedCurrentWorkflowExportState(
  error: unknown,
): CurrentWorkflowExportState {
  return baseCurrentWorkflowExportState({
    status: "failed",
    errorMessage: safeErrorMessage(error),
    issue: "Current workflow export failed before a sanitized response was stored.",
  });
}

export function markCurrentWorkflowExportPending(
  state: N8nStudioState,
): N8nStudioState {
  return {
    ...state,
    importExport: {
      ...state.importExport,
      currentExport: createPendingCurrentWorkflowExportState(
        state.importExport.currentExport,
      ),
    },
  };
}

export function applyCurrentWorkflowExportResponseToStudioState(
  state: N8nStudioState,
  response: unknown,
): N8nStudioState {
  const receipt = exportReceiptFromResponse(response) as
    | ExportReceiptDetail
    | undefined;

  return {
    ...state,
    importExport: {
      ...state.importExport,
      ...(receipt ? { exportReceipt: receipt } : {}),
      currentExport: createCurrentWorkflowExportStateFromResponse(response),
    },
  };
}

export function applyCurrentWorkflowExportErrorToStudioState(
  state: N8nStudioState,
  error: unknown,
): N8nStudioState {
  return {
    ...state,
    importExport: {
      ...state.importExport,
      currentExport: createFailedCurrentWorkflowExportState(error),
    },
  };
}
