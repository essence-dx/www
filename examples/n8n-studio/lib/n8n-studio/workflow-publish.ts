import type { WorkflowDocument, WorkflowNode } from "./types";
import { exportWorkflowDocument } from "./workflow-export";
import { stripSecretValues } from "./workflow-import/secret-redaction";
import type { ImportSanitationIssue } from "./workflow-import/types";

export type WorkflowPublishOperation = "create" | "update";

export type WorkflowPublishOptions = {
  targetWorkflowId?: string;
  activate?: boolean;
};

export type WorkflowPublishBody = {
  name: string;
  nodes: Array<Record<string, unknown>>;
  connections: Record<string, unknown>;
  settings: Record<string, unknown>;
  staticData: Record<string, unknown> | null;
};

export type WorkflowPublishActivationReceipt = {
  status: "submitted" | "not-requested";
  workflowId?: string;
  providerWrite: true;
  liveProviderExecution: false;
  secretsIncluded: false;
};

export type WorkflowPublishReceipt = {
  schema: "dx.n8n-studio.workflow-publish.receipt";
  status: "submitted";
  operation: WorkflowPublishOperation;
  workflowId: string;
  workflowName: string;
  nodeCount: number;
  connectionCount: number;
  credentialReferenceCount: number;
  requestCount: number;
  providerBoundary: true;
  providerWrite: true;
  liveProviderExecution: false;
  secretsIncluded: false;
  sideEffectPolicy: "governed-workflow-publish";
  sourcePath: "nodes/N8n/WorkflowDescription.ts";
  coveragePath: "nodes/N8n/n8n-api-coverage.json";
  receiptPath: ".dx/receipts/n8n-studio/workflows/publish-latest.sr";
  redaction: "secret-values-never-included";
  activation: WorkflowPublishActivationReceipt;
  issue: string;
};

function isRecord(value: unknown): value is Record<string, unknown> {
  return Boolean(value) && typeof value === "object" && !Array.isArray(value);
}

function recordValue(value: unknown): Record<string, unknown> {
  return isRecord(value) ? value : {};
}

function nodeCredentialReferenceCount(node: WorkflowNode) {
  return Object.keys(node.credentials ?? {}).length;
}

function credentialReferenceCount(document: WorkflowDocument) {
  return document.nodes.reduce(
    (total, node) => total + nodeCredentialReferenceCount(node),
    0,
  );
}

function strippedWorkflowObject(document: WorkflowDocument) {
  const issues: ImportSanitationIssue[] = [];

  return stripSecretValues(
    exportWorkflowDocument(document),
    issues,
    undefined,
    "parameter-secret-stripped",
  );
}

export function createWorkflowPublishBody(
  document: WorkflowDocument,
): WorkflowPublishBody {
  const exportedWorkflow = recordValue(strippedWorkflowObject(document));

  return {
    name:
      typeof exportedWorkflow.name === "string" && exportedWorkflow.name.trim()
        ? exportedWorkflow.name
        : document.name,
    nodes: Array.isArray(exportedWorkflow.nodes)
      ? (exportedWorkflow.nodes as Array<Record<string, unknown>>)
      : [],
    connections: recordValue(exportedWorkflow.connections),
    settings: recordValue(exportedWorkflow.settings),
    staticData: isRecord(exportedWorkflow.staticData)
      ? exportedWorkflow.staticData
      : null,
  };
}

export function workflowPublishOperation(
  options: WorkflowPublishOptions = {},
): WorkflowPublishOperation {
  return options.targetWorkflowId ? "update" : "create";
}

export function workflowIdFromPublishResponse(
  response: unknown,
  fallbackWorkflowId: string,
) {
  const responseRecord = recordValue(response);
  const id = responseRecord.id;

  return typeof id === "string" && id.trim() ? id : fallbackWorkflowId;
}

export function createWorkflowPublishReceipt(options: {
  document: WorkflowDocument;
  operation: WorkflowPublishOperation;
  workflowId: string;
  requestCount: number;
  activationRequested: boolean;
}): WorkflowPublishReceipt {
  return {
    schema: "dx.n8n-studio.workflow-publish.receipt",
    status: "submitted",
    operation: options.operation,
    workflowId: options.workflowId,
    workflowName: options.document.name,
    nodeCount: options.document.nodes.length,
    connectionCount: options.document.connections.length,
    credentialReferenceCount: credentialReferenceCount(options.document),
    requestCount: options.requestCount,
    providerBoundary: true,
    providerWrite: true,
    liveProviderExecution: false,
    secretsIncluded: false,
    sideEffectPolicy: "governed-workflow-publish",
    sourcePath: "nodes/N8n/WorkflowDescription.ts",
    coveragePath: "nodes/N8n/n8n-api-coverage.json",
    receiptPath: ".dx/receipts/n8n-studio/workflows/publish-latest.sr",
    redaction: "secret-values-never-included",
    activation: {
      status: options.activationRequested ? "submitted" : "not-requested",
      workflowId: options.activationRequested ? options.workflowId : undefined,
      providerWrite: true,
      liveProviderExecution: false,
      secretsIncluded: false,
    },
    issue:
      "Workflow JSON was submitted through the governed n8n API bridge. This is provider write proof only; execution proof still requires an imported execution receipt.",
  };
}
