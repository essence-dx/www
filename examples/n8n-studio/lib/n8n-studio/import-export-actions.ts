import { createCredentialReadinessForNode } from "./credentials";
import { createEditorSessionReadiness } from "./editor-session-adapter";
import {
  applyExpressionStateToFields,
  createEmptyExpressionEditorState,
  createExpressionEditorState,
} from "./expression-editor";
import { createExecutionReadiness } from "./executions";
import {
  createImportDraftState,
  createImportExportState,
  createImportPreviewState,
} from "./import-export";
import {
  getNodeTypeDescription,
  n8nNodeTypeRegistry,
} from "./node-type-registry";
import type { NodeTypeDescription } from "./node-types/types";
import { createSelectedNodeParameters } from "./parameter-schema";
import { createPinnedDataState } from "./pinned-data";
import type {
  CredentialReadiness,
  ImportDraftState,
  ImportSource,
  N8nStudioState,
  WorkflowDocument,
  WorkflowNode,
} from "./types";
import { createCanvasProjection } from "./workflow-document";

export type ImportExportAction =
  | {
      kind: "loadImportPreview";
      source: ImportSource;
      importedWorkflow: unknown;
    }
  | {
      kind: "applyImportPreview";
      appliedAt?: string;
    }
  | {
      kind: "saveImportedDraft";
      savedAt?: string;
    };

function currentTimestamp(timestamp: string | undefined) {
  return timestamp ?? new Date().toISOString();
}

function selectedImportNode(document: WorkflowDocument) {
  return (
    document.nodes.find(
      (node) =>
        !node.type.toLowerCase().includes("trigger") &&
        (Object.keys(node.parameters).length > 0 ||
          Object.keys(node.credentials ?? {}).length > 0),
    ) ??
    document.nodes[0] ??
    null
  );
}

function selectedNodeSurfaces(
  document: WorkflowDocument,
  selectedNode: WorkflowNode | null,
  registry: Record<string, NodeTypeDescription>,
) {
  if (!selectedNode) {
    return {
      parameters: [],
      expressionEditor: createEmptyExpressionEditorState(""),
      credentials: [],
      editorSession: createEditorSessionReadiness(document, "", registry),
    };
  }

  let credentials: CredentialReadiness[] = [];
  try {
    credentials = createCredentialReadinessForNode(
      selectedNode,
      getNodeTypeDescription(selectedNode.type, registry),
    );
  } catch {
    credentials = [];
  }
  const parameters = createSelectedNodeParameters(selectedNode, registry);
  const expressionEditor = createExpressionEditorState({
    document,
    parameters,
    selectedNode,
  });

  return {
    parameters: applyExpressionStateToFields(parameters, expressionEditor.fields),
    expressionEditor,
    credentials,
    editorSession: createEditorSessionReadiness(document, selectedNode.id, registry),
  };
}

function withImportDraft(
  state: N8nStudioState,
  document: WorkflowDocument,
  draft: ImportDraftState,
): N8nStudioState {
  return {
    ...state,
    importExport: createImportExportState(
      document,
      state.importExport.importPreview,
      draft,
    ),
  };
}

function applyImportedDocument(
  state: N8nStudioState,
  document: WorkflowDocument,
  appliedAt: string,
  registry: Record<string, NodeTypeDescription>,
): N8nStudioState {
  const selectedNode = selectedImportNode(document);
  const selectedNodeId = selectedNode?.id ?? "";
  const selectedSurfaces = selectedNodeSurfaces(document, selectedNode, registry);
  const draft: ImportDraftState = {
    ...state.importExport.draft,
    status: "applied",
    canApplyPreview: false,
    canSaveDraft: true,
    appliedAt,
    lastAppliedWorkflowId: document.id,
    issue:
      "Imported workflow is applied to the local editor session and ready for source-only draft save.",
  };

  return {
    ...state,
    document,
    canvas: createCanvasProjection(document, selectedNodeId),
    ...selectedSurfaces,
    pinnedData: createPinnedDataState(document),
    execution: createExecutionReadiness(document),
    importExport: createImportExportState(
      document,
      state.importExport.importPreview,
      draft,
    ),
  };
}

function blockedDraft(
  draft: ImportDraftState,
  issue: string,
): ImportDraftState {
  return {
    ...draft,
    status: "blocked",
    canApplyPreview: false,
    canSaveDraft: false,
    issue,
  };
}

export function applyImportExportActionToStudioState(
  state: N8nStudioState,
  action: ImportExportAction,
  registry: Record<string, NodeTypeDescription> = n8nNodeTypeRegistry,
): N8nStudioState {
  switch (action.kind) {
    case "loadImportPreview": {
      const importPreview = createImportPreviewState(
        action.importedWorkflow,
        action.source,
        registry,
      );
      const draft = createImportDraftState(importPreview);

      return {
        ...state,
        importExport: createImportExportState(state.document, importPreview, draft),
      };
    }

    case "applyImportPreview": {
      const preview = state.importExport.importPreview;
      const blocker = preview.issues.find((issue) => issue.severity === "blocker");
      if (!preview.sanitizedDocument) {
        return withImportDraft(
          state,
          state.document,
          blockedDraft(
            state.importExport.draft,
            "Load a sanitized import preview before applying it to the editor session.",
          ),
        );
      }
      if (blocker) {
        return withImportDraft(
          state,
          state.document,
          blockedDraft(
            state.importExport.draft,
            "The import preview still has blocker sanitation issues. Resolve them before applying this workflow.",
          ),
        );
      }

      return applyImportedDocument(
        state,
        preview.sanitizedDocument,
        currentTimestamp(action.appliedAt),
        registry,
      );
    }

    case "saveImportedDraft": {
      const draft = state.importExport.draft;
      if (!draft.lastAppliedWorkflowId || draft.lastAppliedWorkflowId !== state.document.id) {
        return withImportDraft(
          state,
          state.document,
          blockedDraft(
            draft,
            "Apply a sanitized import preview before saving the imported draft.",
          ),
        );
      }

      return withImportDraft(state, state.document, {
        ...draft,
        status: "saved",
        canApplyPreview: false,
        canSaveDraft: true,
        savedAt: currentTimestamp(action.savedAt),
        lastSavedWorkflowId: state.document.id,
        issue:
          "Imported workflow draft is saved in the source-owned editor session; filesystem persistence still belongs to a governed receipt writer.",
      });
    }
  }
}
