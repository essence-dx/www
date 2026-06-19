"use client";

import { useCallback, useState } from "react";

import {
  applyParameterMutationToStudioState,
  type ParameterMutation,
} from "../../lib/n8n-studio/parameter-mutations";
import {
  applyCanvasInteractionToStudioState,
  type CanvasInteractionAction,
} from "../../lib/n8n-studio/canvas-interactions";
import { applyImportExportActionToStudioState } from "../../lib/n8n-studio/import-export-actions";
import {
  applyCurrentWorkflowExportErrorToStudioState,
  applyCurrentWorkflowExportResponseToStudioState,
  markCurrentWorkflowExportPending,
} from "../../lib/n8n-studio/current-workflow-export";
import { applyNodeCreatorActionToStudioState } from "../../lib/n8n-studio/node-creator-actions";
import { n8nNodeTypeRegistry } from "../../lib/n8n-studio/node-type-registry";
import type { NodeTypeDescription } from "../../lib/n8n-studio/node-types/types";
import type { N8nStudioState, ParameterValuePath } from "../../lib/n8n-studio/types";
import { AiToolsPanel } from "./ai-tools-panel";
import { CredentialReadinessPanel } from "./credential-readiness-panel";
import { ExecutionPanel } from "./execution-panel";
import { ImportExportPanel } from "./import-export-panel";
import { NodeCatalogPanel } from "./node-catalog-panel";
import { ParameterInspector } from "./parameter-inspector";
import { PinnedDataPanel } from "./pinned-data-panel";
import { ReceiptCheckPanel } from "./receipt-check-panel";
import { ResourceLocatorPanel } from "./resource-locator-panel";
import { StudioTopbar } from "./studio-topbar";
import { WorkflowCanvas } from "./workflow-canvas";

export type N8nStudioAppProps = {
  nodeTypeRegistry?: Record<string, NodeTypeDescription>;
  state: N8nStudioState;
};

export function N8nStudioApp({
  nodeTypeRegistry = n8nNodeTypeRegistry,
  state,
}: N8nStudioAppProps) {
  const [studioState, setStudioState] = useState(state);

  const applySelectedNodeMutation = useCallback((mutation: ParameterMutation) => {
    setStudioState((currentState) =>
      applyParameterMutationToStudioState(
        currentState,
        currentState.canvas.selectedNodeId,
        mutation,
      ),
    );
  }, []);

  const handleCanvasAction = useCallback((canvasAction: CanvasInteractionAction) => {
    setStudioState((currentState) =>
      applyCanvasInteractionToStudioState(currentState, canvasAction),
    );
  }, []);
  const handleApplyImportPreview = useCallback(() => {
    setStudioState((currentState) =>
      applyImportExportActionToStudioState(
        currentState,
        {
          kind: "applyImportPreview",
        },
        nodeTypeRegistry,
      ),
    );
  }, [nodeTypeRegistry]);
  const handleSaveImportedDraft = useCallback(() => {
    setStudioState((currentState) =>
      applyImportExportActionToStudioState(
        currentState,
        {
          kind: "saveImportedDraft",
        },
        nodeTypeRegistry,
      ),
    );
  }, [nodeTypeRegistry]);
  const handleExportCurrentWorkflow = useCallback(async () => {
    const exportDocument = studioState.document;

    setStudioState((currentState) =>
      markCurrentWorkflowExportPending(currentState),
    );

    try {
      const response = await fetch("/api/n8n-studio/export", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ document: exportDocument }),
      });

      if (!response.ok) {
        throw new Error("Current workflow export failed.");
      }

      const payload = await response.json();
      setStudioState((currentState) =>
        applyCurrentWorkflowExportResponseToStudioState(currentState, payload),
      );
    } catch (error) {
      setStudioState((currentState) =>
        applyCurrentWorkflowExportErrorToStudioState(currentState, error),
      );
    }
  }, [studioState.document]);
  const handleAddCatalogNode = useCallback((catalogNodeId: string) => {
    setStudioState((currentState) =>
      applyNodeCreatorActionToStudioState(
        currentState,
        {
          kind: "addCatalogNode",
          catalogNodeId,
        },
        nodeTypeRegistry,
      ),
    );
  }, [nodeTypeRegistry]);

  const handleAddCollectionItem = useCallback(
    (collectionPath: ParameterValuePath) => {
      applySelectedNodeMutation({
        kind: "addCollectionItem",
        collectionPath,
      });
    },
    [applySelectedNodeMutation],
  );

  const handleRemoveCollectionItem = useCallback(
    (collectionPath: ParameterValuePath, itemIndex: number) => {
      applySelectedNodeMutation({
        kind: "removeCollectionItem",
        collectionPath,
        itemIndex,
      });
    },
    [applySelectedNodeMutation],
  );

  const handleUpdateParameterValue = useCallback(
    (valuePath: ParameterValuePath, value: unknown) => {
      applySelectedNodeMutation({
        kind: "updateValue",
        valuePath,
        value,
      });
    },
    [applySelectedNodeMutation],
  );

  const selectedNode =
    studioState.document.nodes.find(
      (node) => node.id === studioState.canvas.selectedNodeId,
    ) ?? studioState.document.nodes[0];

  return (
    <main
      className="n8n-studio"
      data-dx-surface="n8n-studio"
      data-dx-state-runtime="source-owned"
      data-provider-boundary="visible"
      data-live-provider-execution="false"
    >
      <StudioTopbar state={studioState} />
      <section className="n8ns-workbench" aria-label="n8n Studio workspace">
        <NodeCatalogPanel
          catalog={studioState.catalog}
          nodeTypeRegistry={nodeTypeRegistry}
          onAddCatalogNode={handleAddCatalogNode}
        />
        <WorkflowCanvas
          canvas={studioState.canvas}
          document={studioState.document}
          onCanvasAction={handleCanvasAction}
        />
        <aside className="n8ns-inspector" aria-label="Node inspector">
          <ParameterInspector
            expressionEditor={studioState.expressionEditor}
            node={selectedNode}
            onAddCollectionItem={handleAddCollectionItem}
            onRemoveCollectionItem={handleRemoveCollectionItem}
            onUpdateParameterValue={handleUpdateParameterValue}
            parameters={studioState.parameters}
          />
          <CredentialReadinessPanel credentials={studioState.credentials} />
          <ResourceLocatorPanel resourceLocator={studioState.resourceLocator} />
          <PinnedDataPanel pinnedData={studioState.pinnedData} />
          <ExecutionPanel execution={studioState.execution} />
          <AiToolsPanel aiTools={studioState.aiTools} />
          <ImportExportPanel
            importExport={studioState.importExport}
            onApplyImportPreview={handleApplyImportPreview}
            onExportCurrentWorkflow={handleExportCurrentWorkflow}
            onSaveImportedDraft={handleSaveImportedDraft}
          />
          <ReceiptCheckPanel receipts={studioState.receipts} />
        </aside>
      </section>
    </main>
  );
}
