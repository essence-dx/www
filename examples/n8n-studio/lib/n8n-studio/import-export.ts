import type {
  ExportReceiptDetail,
  ImportExportBoundary,
  ImportExportState,
  ImportDraftState,
  ImportPreviewIssueRow,
  ImportSource,
  ImportSourceOption,
  SanitizedImportPreview,
  WorkflowDocument,
} from "./types";
import { createIdleCurrentWorkflowExportState } from "./current-workflow-export";
import { studioWorkflowDocument } from "./workflow-document";
import { sanitizeImportedWorkflow } from "./workflow-import-sanitizer";
import { n8nNodeTypeRegistry } from "./node-type-registry";
import type { NodeTypeDescription } from "./node-types/types";
import type { ImportSanitationIssue } from "./workflow-import-sanitizer-types";

export { exportWorkflowDocument } from "./workflow-export";
export { sanitizeImportedWorkflow } from "./workflow-import-sanitizer";
export type {
  ImportSanitationIssue,
  ImportSanitationResult,
} from "./workflow-import-sanitizer-types";

export const importExportSanitizedFields = [
  "unknownNodeTypes",
  "credentialSecrets",
  "parameterSecrets",
  "malformedConnections",
  "orphanedPinnedData",
  "webhookIdentifiers",
  "templateOnlyResourceLocators",
];

const importSourceOptions: ImportSourceOption[] = [
  {
    source: "clipboard",
    label: "Clipboard",
    status: "source-only",
    providerBoundary: true,
    liveProviderExecution: false,
    issue: "Clipboard import awaits the editor-session adapter.",
  },
  {
    source: "file",
    label: "Workflow file",
    status: "source-only",
    providerBoundary: true,
    liveProviderExecution: false,
    issue: "File import awaits the editor-session adapter.",
  },
  {
    source: "url",
    label: "Workflow URL",
    status: "source-only",
    providerBoundary: true,
    liveProviderExecution: false,
    issue: "URL import awaits governed fetch and receipt import.",
  },
];

function sourceOnlyBoundary(issue: string): ImportExportBoundary {
  return {
    status: "source-only",
    providerBoundary: true,
    liveProviderExecution: false,
    secretsIncluded: false,
    executableAfterImport: false,
    issue,
  };
}

function issueSeverity(issue: ImportSanitationIssue): ImportPreviewIssueRow["severity"] {
  if (
    issue.code === "unknown-node-type" ||
    issue.code === "connection-source-missing" ||
    issue.code === "connection-target-missing"
  ) {
    return "blocker";
  }

  return "warning";
}

function issueAction(issue: ImportSanitationIssue) {
  switch (issue.code) {
    case "unknown-node-type":
      return "Install or generate the missing node type description before enabling this node.";
    case "credential-secret-stripped":
      return "Reconnect the credential through the DX credential picker; secret values were not imported.";
    case "parameter-secret-stripped":
      return "Re-enter the parameter value in a governed credential or expression field.";
    case "connection-source-missing":
      return "Restore the missing source node or keep the dropped connection out of the draft.";
    case "connection-target-missing":
      return "Restore the missing target node or keep the dropped connection out of the draft.";
    case "webhook-id-regenerated":
      return "Review the regenerated local webhook id before exporting or executing.";
    default:
      return "Review this sanitation issue before saving the imported draft.";
  }
}

function createIssueRows(issues: ImportSanitationIssue[]): ImportPreviewIssueRow[] {
  return issues.map((issue) => ({
    code: issue.code,
    message: issue.message,
    nodeName: issue.nodeName,
    severity: issueSeverity(issue),
    action: issueAction(issue),
  }));
}

function countCredentialReferences(document: WorkflowDocument) {
  return document.nodes.reduce(
    (total, node) => total + Object.keys(node.credentials ?? {}).length,
    0,
  );
}

function downloadName(document: WorkflowDocument) {
  return `${document.name
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-|-$/g, "") || "workflow"}.n8n.json`;
}

export function createEmptyImportPreviewState(): SanitizedImportPreview {
  return {
    status: "awaiting-input",
    keptNodeCount: 0,
    droppedIssueCount: 0,
    strippedSecretCount: 0,
    connectionCount: 0,
    pinDataNodeCount: 0,
    regeneratedWebhookCount: 0,
    sanitizedFields: importExportSanitizedFields,
    issues: [],
    boundary: sourceOnlyBoundary(
      "No workflow import has been loaded into this source-only editor session.",
    ),
  };
}

function baseImportDraftState(
  patch: Partial<ImportDraftState> = {},
): ImportDraftState {
  return {
    status: "awaiting-preview",
    canApplyPreview: false,
    canSaveDraft: false,
    providerBoundary: true,
    liveProviderExecution: false,
    secretsIncluded: false,
    persistedToDisk: false,
    editorSessionOnly: true,
    saveReceiptPath: ".dx/receipts/n8n-studio/import/latest.sr",
    issue: "No sanitized import preview is ready to apply.",
    ...patch,
  };
}

export function createImportDraftState(
  importPreview: SanitizedImportPreview,
): ImportDraftState {
  const blocker = importPreview.issues.find((issue) => issue.severity === "blocker");
  if (!importPreview.sanitizedDocument) {
    return baseImportDraftState();
  }
  if (blocker) {
    return baseImportDraftState({
      status: "blocked",
      issue:
        "The import preview still has blocker sanitation issues. Resolve them before applying this workflow.",
    });
  }

  return baseImportDraftState({
    status: "ready-to-apply",
    canApplyPreview: true,
    issue: "Sanitized workflow preview is ready to apply to the local editor session.",
  });
}

export function createImportPreviewState(
  importedWorkflow: unknown,
  source: ImportSource,
  registry: Record<string, NodeTypeDescription> = n8nNodeTypeRegistry,
): SanitizedImportPreview {
  const sanitation = sanitizeImportedWorkflow(importedWorkflow, registry);
  const issues = createIssueRows(sanitation.issues);
  const strippedSecretCount = sanitation.issues.filter(
    (issue) =>
      issue.code === "credential-secret-stripped" ||
      issue.code === "parameter-secret-stripped",
  ).length;
  const droppedIssueCount = sanitation.issues.filter(
    (issue) =>
      issue.code === "unknown-node-type" ||
      issue.code === "connection-source-missing" ||
      issue.code === "connection-target-missing",
  ).length;

  return {
    status: issues.length > 0 ? "sanitized-with-issues" : "sanitized",
    source,
    workflowName: sanitation.document.name,
    keptNodeCount: sanitation.document.nodes.length,
    droppedIssueCount,
    strippedSecretCount,
    connectionCount: sanitation.document.connections.length,
    pinDataNodeCount: Object.keys(sanitation.document.pinData).length,
    regeneratedWebhookCount: sanitation.regeneratedWebhookIds.length,
    sanitizedFields: importExportSanitizedFields,
    issues,
    sanitizedDocument: sanitation.document,
    boundary: sourceOnlyBoundary(
      "Imported workflow is sanitized locally and remains non-executable until credentials and provider receipts are present.",
    ),
  };
}

export function createExportReceiptDetail(document: WorkflowDocument): ExportReceiptDetail {
  return {
    schema: "dx.n8n-studio.export.receipt",
    format: "n8n-workflow-json",
    workflowName: document.name,
    nodeCount: document.nodes.length,
    connectionCount: document.connections.length,
    pinnedNodeCount: Object.keys(document.pinData).length,
    credentialReferenceCount: countCredentialReferences(document),
    routePath: "/api/n8n-studio/export",
    receiptPath: ".dx/receipts/n8n-studio/export/latest.sr",
    downloadName: downloadName(document),
    providerBoundary: true,
    liveProviderExecution: false,
    secretsIncluded: false,
    redaction: "secret-values-never-included",
    status: "source-only",
    issue:
      "Export response is source-owned workflow JSON; live execution proof requires a separate execution receipt.",
  };
}

export function createImportExportState(
  document: WorkflowDocument,
  importPreview: SanitizedImportPreview = createEmptyImportPreviewState(),
  draft: ImportDraftState = createImportDraftState(importPreview),
  currentExport = createIdleCurrentWorkflowExportState(document),
): ImportExportState {
  return {
    importSources: importSourceOptions,
    exportFormat: "n8n-workflow-json",
    sanitizedFields: importExportSanitizedFields,
    importPreview,
    draft,
    exportReceipt: createExportReceiptDetail(document),
    currentExport,
  };
}

export const importExportState: ImportExportState =
  createImportExportState(studioWorkflowDocument);
