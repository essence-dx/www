import {
  createExportReceiptDetail,
  exportWorkflowDocument,
} from "../../lib/n8n-studio/import-export";
import type {
  CatalogSummary,
  N8nStudioState,
  WorkflowConnection,
  WorkflowDocument,
  WorkflowNode,
} from "../../lib/n8n-studio/types";
import { sanitizeCredentials } from "../../lib/n8n-studio/workflow-import/credential-redaction";
import { stripSecretValues } from "../../lib/n8n-studio/workflow-import/secret-redaction";
import type { ImportSanitationIssue } from "../../lib/n8n-studio/workflow-import-sanitizer-types";
import { studioWorkflowDocument } from "../../lib/n8n-studio/workflow-document";
import { createStudioBootFromLocalGeneratedSource } from "./generated-catalog-source";

type ExportResponseOptions = {
  document?: WorkflowDocument;
  catalog?: CatalogSummary;
};

function isRecord(value: unknown): value is Record<string, unknown> {
  return Boolean(value) && typeof value === "object" && !Array.isArray(value);
}

function stringValue(value: unknown, fallback: string) {
  return typeof value === "string" && value.trim() ? value : fallback;
}

function numberValue(value: unknown, fallback: number) {
  return typeof value === "number" && Number.isFinite(value) ? value : fallback;
}

function workflowPosition(value: unknown): WorkflowNode["position"] {
  if (!isRecord(value)) {
    return { x: 0, y: 0 };
  }

  return {
    x: numberValue(value.x, 0),
    y: numberValue(value.y, 0),
  };
}

function workflowNode(value: unknown, issues: ImportSanitationIssue[]): WorkflowNode | undefined {
  if (!isRecord(value)) {
    return undefined;
  }

  const name = stringValue(value.name, "Workflow node");
  const parameters = isRecord(value.parameters)
    ? (stripSecretValues(
        value.parameters,
        issues,
        name,
        "parameter-secret-stripped",
      ) as Record<string, unknown>)
    : {};
  const credentials = sanitizeCredentials(value.credentials, issues, name);

  return {
    id: stringValue(value.id, name),
    name,
    type: stringValue(value.type, "n8n-nodes-base.noOp"),
    typeVersion: numberValue(value.typeVersion, 1),
    position: workflowPosition(value.position),
    parameters,
    ...(credentials ? { credentials } : {}),
    ...(typeof value.disabled === "boolean" ? { disabled: value.disabled } : {}),
    ...(typeof value.notes === "string" ? { notes: value.notes } : {}),
  };
}

function workflowConnection(value: unknown): WorkflowConnection | undefined {
  if (!isRecord(value)) {
    return undefined;
  }

  const sourceOutput = value.sourceOutput === "ai_tool" ? "ai_tool" : "main";
  const targetInput = value.targetInput === "ai_tool" ? "ai_tool" : "main";

  return {
    id: stringValue(value.id, "workflow-connection"),
    sourceNode: stringValue(value.sourceNode, ""),
    targetNode: stringValue(value.targetNode, ""),
    sourceOutput,
    targetInput,
    index: numberValue(value.index, 0),
  };
}

function sanitizeWorkflowDocumentForExport(value: unknown): WorkflowDocument {
  if (!isRecord(value)) {
    return studioWorkflowDocument;
  }

  const issues: ImportSanitationIssue[] = [];
  const nodes = Array.isArray(value.nodes)
    ? value.nodes.flatMap((node) => {
        const sanitized = workflowNode(node, issues);
        return sanitized ? [sanitized] : [];
      })
    : studioWorkflowDocument.nodes;
  const nodeNames = new Set(nodes.map((node) => node.name));
  const connections = Array.isArray(value.connections)
    ? value.connections.flatMap((connection) => {
        const sanitized = workflowConnection(connection);
        return sanitized &&
          nodeNames.has(sanitized.sourceNode) &&
          nodeNames.has(sanitized.targetNode)
          ? [sanitized]
          : [];
      })
    : [];

  return {
    schemaVersion: numberValue(value.schemaVersion, 1),
    id: stringValue(value.id, "dx-n8n-studio-workflow"),
    projectId: stringValue(value.projectId, "local-dx-workspace"),
    name: stringValue(value.name, "DX n8n Studio workflow"),
    active: typeof value.active === "boolean" ? value.active : false,
    nodes,
    connections,
    tags: Array.isArray(value.tags)
      ? value.tags.filter((tag): tag is string => typeof tag === "string")
      : [],
    pinData: isRecord(value.pinData)
      ? (stripSecretValues(
          value.pinData,
          issues,
          undefined,
          "parameter-secret-stripped",
        ) as WorkflowDocument["pinData"])
      : {},
    meta: {
      source: "dx-www-n8n-studio",
      liveProviderExecution: false,
    },
  };
}

export function createExportResponse(options: ExportResponseOptions = {}) {
  const document = options.document ?? studioWorkflowDocument;

  return {
    schema: "dx.n8n-studio.export",
    format: "n8n-workflow-json",
    providerBoundary: true,
    liveProviderExecution: false,
    catalogNodeCount: options.catalog?.catalogNodes.length,
    generatedMetadata: options.catalog?.generatedMetadata,
    receipt: createExportReceiptDetail(document),
    workflow: exportWorkflowDocument(document),
  };
}

export function createExportResponseFromStudioState(state: N8nStudioState) {
  return createExportResponse({
    document: state.document,
    catalog: state.catalog,
  });
}

export function createExportResponseFromPayload(payload: unknown) {
  const document = isRecord(payload)
    ? sanitizeWorkflowDocumentForExport(payload.document)
    : studioWorkflowDocument;

  return createExportResponse({ document });
}

export function createExportResponseFromLocalGeneratedSource(
  startDirectory = process.cwd(),
) {
  const boot = createStudioBootFromLocalGeneratedSource(startDirectory);

  return createExportResponseFromStudioState(boot.state);
}
