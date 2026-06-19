import { aiToolState } from "./ai-tools";
import { n8nCatalogSummary } from "./catalog";
import { createCredentialReadinessForNode } from "./credentials";
import { createEditorSessionReadiness } from "./editor-session-adapter";
import {
  applyExpressionStateToFields,
  createEmptyExpressionEditorState,
  createExpressionEditorState,
} from "./expression-editor";
import { createExecutionReadiness } from "./executions";
import { importExportState } from "./import-export";
import {
  getNodeTypeDescription,
  n8nNodeTypeRegistry,
} from "./node-type-registry";
import type { NodeTypeDescription } from "./node-types/types";
import { createSelectedNodeParameters } from "./parameter-schema";
import { pinnedDataState } from "./pinned-data";
import { receiptSummary } from "./receipts";
import { resourceLocatorState } from "./resource-locator";
import type { CatalogSummary, N8nStudioState } from "./types";
import { createCanvasProjection, studioWorkflowDocument } from "./workflow-document";

export type N8nStudioStateOptions = {
  catalog?: CatalogSummary;
  nodeTypeRegistry?: Record<string, NodeTypeDescription>;
};

export function createN8nStudioState(
  options: N8nStudioStateOptions = {},
): N8nStudioState {
  const nodeTypeRegistry = options.nodeTypeRegistry ?? n8nNodeTypeRegistry;
  const selectedNode =
    studioWorkflowDocument.nodes.find((node) => node.id === "node-http-request") ??
    studioWorkflowDocument.nodes[0];
  const parameters = selectedNode
    ? createSelectedNodeParameters(selectedNode, nodeTypeRegistry)
    : [];
  const expressionEditor = selectedNode
    ? createExpressionEditorState({
        document: studioWorkflowDocument,
        parameters,
        selectedNode,
      })
    : createEmptyExpressionEditorState("");
  const selectedNodeType = selectedNode
    ? getNodeTypeDescription(selectedNode.type, nodeTypeRegistry)
    : undefined;

  return {
    catalog: options.catalog ?? n8nCatalogSummary,
    document: studioWorkflowDocument,
    canvas: createCanvasProjection(studioWorkflowDocument),
    parameters: applyExpressionStateToFields(parameters, expressionEditor.fields),
    expressionEditor,
    credentials:
      selectedNode && selectedNodeType
        ? createCredentialReadinessForNode(selectedNode, selectedNodeType)
        : [],
    editorSession: createEditorSessionReadiness(
      studioWorkflowDocument,
      selectedNode?.id ?? "",
      nodeTypeRegistry,
    ),
    resourceLocator: resourceLocatorState,
    pinnedData: pinnedDataState,
    execution: createExecutionReadiness(studioWorkflowDocument),
    aiTools: aiToolState,
    importExport: importExportState,
    receipts: receiptSummary,
  };
}
