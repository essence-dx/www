import { createCredentialReadinessForNode } from "./credentials";
import { createEditorSessionReadiness } from "./editor-session-adapter";
import {
  applyExpressionStateToFields,
  createEmptyExpressionEditorState,
  createExpressionEditorState,
} from "./expression-editor";
import { getNodeTypeDescription } from "./node-type-registry";
import { createSelectedNodeParameters } from "./parameter-schema";
import {
  createFitWorkflowViewport,
  pointDelta,
  roundedCanvasZoom,
  snapCanvasPosition,
} from "./canvas-geometry";
import {
  createCanvasConnection,
  reconnectCanvasConnection,
  validConnectionTargetNodeIds,
  validReconnectNodeIds,
} from "./canvas-edge-validation";
import { createCanvasInteractionState } from "./canvas-interaction-state";
import type {
  CanvasConnectionEndpoint,
  CanvasInteractionState,
  CanvasPoint,
  N8nStudioState,
  WorkflowConnection,
  WorkflowDocument,
  WorkflowNode,
} from "./types";

export { createCanvasInteractionState } from "./canvas-interaction-state";

export type CanvasInteractionAction =
  | { kind: "selectNode"; nodeId: string; additive?: boolean }
  | { kind: "selectAll" }
  | {
      kind: "beginNodeDrag";
      nodeId: string;
      pointerId: number;
      canvasPoint: CanvasPoint;
      snapToGrid?: boolean;
    }
  | {
      kind: "beginCanvasPan";
      pointerId: number;
      canvasPoint: CanvasPoint;
    }
  | {
      kind: "movePointer";
      pointerId: number;
      canvasPoint: CanvasPoint;
    }
  | { kind: "endPointer"; pointerId: number }
  | { kind: "panViewport"; delta: CanvasPoint }
  | { kind: "zoomViewport"; delta: number }
  | { kind: "fitWorkflow"; viewportSize: { width: number; height: number } }
  | { kind: "tidyWorkflow" }
  | { kind: "deleteSelection" }
  | { kind: "nudgeSelection"; delta: CanvasPoint }
  | {
      kind: "beginConnectionDrag";
      sourceNodeId: string;
      sourceOutput: WorkflowConnection["sourceOutput"];
      pointerId: number;
      canvasPoint: CanvasPoint;
    }
  | {
      kind: "beginReconnectEdge";
      edgeId: string;
      endpoint: CanvasConnectionEndpoint;
      pointerId: number;
      canvasPoint: CanvasPoint;
    }
  | {
      kind: "completeConnection";
      targetNodeId: string;
      targetInput: WorkflowConnection["targetInput"];
      endpoint?: "target";
    }
  | {
      kind: "completeConnection";
      nodeId: string;
      endpoint: CanvasConnectionEndpoint;
      inputOrOutput: WorkflowConnection["sourceOutput"] | WorkflowConnection["targetInput"];
    };

function selectedNode(document: WorkflowDocument, selectedNodeId: string) {
  return document.nodes.find((node) => node.id === selectedNodeId);
}

function refreshSelectedNodeSurfaces(
  state: N8nStudioState,
  selectedNodeId: string,
): Pick<
  N8nStudioState,
  "parameters" | "expressionEditor" | "credentials" | "editorSession"
> {
  const node = selectedNode(state.document, selectedNodeId);
  if (!node) {
    return {
      parameters: [],
      expressionEditor: createEmptyExpressionEditorState(selectedNodeId),
      credentials: [],
      editorSession: createEditorSessionReadiness(state.document, selectedNodeId),
    };
  }

  try {
    const description = getNodeTypeDescription(node.type);
    const parameters = createSelectedNodeParameters(node);
    const expressionEditor = createExpressionEditorState({
      document: state.document,
      parameters,
      selectedNode: node,
    });
    return {
      parameters: applyExpressionStateToFields(parameters, expressionEditor.fields),
      expressionEditor,
      credentials: createCredentialReadinessForNode(node, description),
      editorSession: createEditorSessionReadiness(state.document, selectedNodeId),
    };
  } catch {
    return {
      parameters: [],
      expressionEditor: createEmptyExpressionEditorState(selectedNodeId),
      credentials: [],
      editorSession: createEditorSessionReadiness(state.document, selectedNodeId),
    };
  }
}

function withCanvasDocument(
  state: N8nStudioState,
  document: WorkflowDocument,
  selectedNodeId: string,
  selectedNodeIds: string[],
  interactionPatch: Partial<CanvasInteractionState> = {},
): N8nStudioState {
  const nextState = {
    ...state,
    document,
    canvas: {
      ...state.canvas,
      selectedNodeId,
      nodes: document.nodes,
      connections: document.connections,
      interaction: createCanvasInteractionState(
        document,
        selectedNodeId,
        selectedNodeIds,
        interactionPatch,
      ),
    },
  };
  const selectedSurfaces = refreshSelectedNodeSurfaces(nextState, selectedNodeId);

  return {
    ...nextState,
    ...selectedSurfaces,
  };
}

function withInteractionIssue(state: N8nStudioState, issue: string) {
  return {
    ...state,
    canvas: {
      ...state.canvas,
      interaction: {
        ...state.canvas.interaction,
        issue,
      },
    },
  };
}

function withInteractionPatch(
  state: N8nStudioState,
  patch: Partial<CanvasInteractionState>,
) {
  return {
    ...state,
    canvas: {
      ...state.canvas,
      interaction: createCanvasInteractionState(
        state.document,
        state.canvas.selectedNodeId,
        state.canvas.interaction.selectedNodeIds,
        patch,
      ),
    },
  };
}

function withViewport(
  state: N8nStudioState,
  viewport: N8nStudioState["canvas"]["viewport"],
  interactionPatch: Partial<CanvasInteractionState> = {},
) {
  return {
    ...state,
    canvas: {
      ...state.canvas,
      viewport,
      interaction: createCanvasInteractionState(
        state.document,
        state.canvas.selectedNodeId,
        state.canvas.interaction.selectedNodeIds,
        interactionPatch,
      ),
    },
  };
}

function moveNodes(
  document: WorkflowDocument,
  selectedNodeIds: string[],
  delta: CanvasPoint,
  snapToGrid: boolean,
  startPositions: Map<string, CanvasPoint>,
): WorkflowDocument {
  const selectedIds = new Set(selectedNodeIds);

  return {
    ...document,
    nodes: document.nodes.map((node) => {
      if (!selectedIds.has(node.id)) {
        return node;
      }
      const startPosition = startPositions.get(node.id) ?? node.position;
      const position = {
        x: startPosition.x + delta.x,
        y: startPosition.y + delta.y,
      };

      return {
        ...node,
        position: snapToGrid
          ? {
              x: snapCanvasPosition(position.x),
              y: snapCanvasPosition(position.y),
            }
          : position,
      };
    }),
  };
}

function startPositionsForNodes(
  document: WorkflowDocument,
  selectedNodeIds: string[],
) {
  const selectedIds = new Set(selectedNodeIds);

  return new Map(
    document.nodes
      .filter((node) => selectedIds.has(node.id))
      .map((node) => [node.id, node.position]),
  );
}

function connectionTouchesDeletedNode(
  connection: WorkflowConnection,
  deletedNodeNames: Set<string>,
) {
  return (
    deletedNodeNames.has(connection.sourceNode) ||
    deletedNodeNames.has(connection.targetNode)
  );
}

function nextSelectionAfterDelete(
  deletedNodes: WorkflowNode[],
  remainingNodes: WorkflowNode[],
) {
  const deletedCenterX =
    deletedNodes.reduce((sum, node) => sum + node.position.x, 0) /
    Math.max(1, deletedNodes.length);

  return [...remainingNodes].sort(
    (left, right) =>
      Math.abs(left.position.x - deletedCenterX) -
      Math.abs(right.position.x - deletedCenterX),
  )[0]?.id;
}

function deleteSelectedNodes(state: N8nStudioState) {
  const selectedIds = new Set(state.canvas.interaction.selectedNodeIds);
  if (selectedIds.size === 0) {
    return state;
  }

  const deletedNodes = state.document.nodes.filter((node) => selectedIds.has(node.id));
  const remainingNodes = state.document.nodes.filter(
    (node) => !selectedIds.has(node.id),
  );
  if (remainingNodes.length === 0) {
    return withInteractionIssue(state, "Cannot delete every node in the workflow.");
  }

  const deletedNodeNames = new Set(deletedNodes.map((node) => node.name));
  const selectedNodeId =
    nextSelectionAfterDelete(deletedNodes, remainingNodes) ?? remainingNodes[0].id;
  const nextPinData = Object.fromEntries(
    Object.entries(state.document.pinData).filter(
      ([nodeName]) => !deletedNodeNames.has(nodeName),
    ),
  );
  const document = {
    ...state.document,
    nodes: remainingNodes,
    connections: state.document.connections.filter(
      (connection) => !connectionTouchesDeletedNode(connection, deletedNodeNames),
    ),
    pinData: nextPinData,
  };

  return withCanvasDocument(state, document, selectedNodeId, [selectedNodeId]);
}

function tidyWorkflow(state: N8nStudioState) {
  const nodes = [...state.document.nodes]
    .sort((left, right) => left.position.x - right.position.x)
    .map((node, index) => ({
      ...node,
      position: {
        x: 84 + index * 300,
        y: 208,
      },
    }));

  return withCanvasDocument(
    state,
    { ...state.document, nodes },
    state.canvas.selectedNodeId,
    state.canvas.interaction.selectedNodeIds,
  );
}

function fitWorkflow(
  state: N8nStudioState,
  viewportSize: { width: number; height: number },
) {
  return withViewport(
    state,
    createFitWorkflowViewport(state.document.nodes, viewportSize),
  );
}

function updatePointer(state: N8nStudioState, pointerId: number, canvasPoint: CanvasPoint) {
  const activeDrag = state.canvas.interaction.activeDrag;
  const edgeDraft = state.canvas.interaction.edgeDraft;
  if (
    edgeDraft &&
    edgeDraft.pointerId === pointerId &&
    (state.canvas.interaction.mode === "edge-drag" ||
      state.canvas.interaction.mode === "edge-reconnect")
  ) {
    return withInteractionPatch(state, {
      mode: state.canvas.interaction.mode,
      edgeDraft: {
        ...edgeDraft,
        lastPoint: canvasPoint,
      },
      selectedConnectionId: state.canvas.interaction.selectedConnectionId,
    });
  }

  if (!activeDrag || activeDrag.pointerId !== pointerId) {
    return state;
  }

  const delta = pointDelta(activeDrag.origin, canvasPoint);

  if (state.canvas.interaction.mode === "canvas-pan" && activeDrag.startViewport) {
    return withViewport(
      state,
      {
        x: activeDrag.startViewport.x + delta.x,
        y: activeDrag.startViewport.y + delta.y,
        zoom: state.canvas.viewport.zoom,
      },
      {
        mode: "canvas-pan",
        activeDrag: {
          ...activeDrag,
          lastPoint: canvasPoint,
        },
      },
    );
  }

  const document = moveNodes(
    state.document,
    state.canvas.interaction.selectedNodeIds,
    delta,
    activeDrag.snapToGrid,
    activeDrag.nodeId && activeDrag.startPosition
      ? new Map([[activeDrag.nodeId, activeDrag.startPosition]])
      : startPositionsForNodes(state.document, state.canvas.interaction.selectedNodeIds),
  );

  return withCanvasDocument(
    state,
    document,
    state.canvas.selectedNodeId,
    state.canvas.interaction.selectedNodeIds,
    {
      mode: "node-drag",
      activeDrag: {
        ...activeDrag,
        lastPoint: canvasPoint,
      },
    },
  );
}

function beginConnectionDrag(
  state: N8nStudioState,
  sourceNodeId: string,
  sourceOutput: WorkflowConnection["sourceOutput"],
  pointerId: number,
  canvasPoint: CanvasPoint,
) {
  if (!selectedNode(state.document, sourceNodeId)) {
    return withInteractionIssue(
      state,
      `Source node ${sourceNodeId} is not available in the workflow document.`,
    );
  }

  return withCanvasDocument(
    state,
    state.document,
    sourceNodeId,
    [sourceNodeId],
    {
      mode: "edge-drag",
      edgeDraft: {
        pointerId,
        sourceNodeId,
        sourceOutput,
        origin: canvasPoint,
        lastPoint: canvasPoint,
        validEndpointNodeIds: validConnectionTargetNodeIds(
          state.document,
          sourceNodeId,
          sourceOutput,
        ),
      },
    },
  );
}

function beginReconnectEdge(
  state: N8nStudioState,
  edgeId: string,
  endpoint: CanvasConnectionEndpoint,
  pointerId: number,
  canvasPoint: CanvasPoint,
) {
  const connection = state.document.connections.find((edge) => edge.id === edgeId);
  if (!connection) {
    return withInteractionIssue(
      state,
      `Connection ${edgeId} is not available in the workflow document.`,
    );
  }
  const sourceNode = state.document.nodes.find(
    (node) => node.name === connection.sourceNode,
  );
  const targetNode = state.document.nodes.find(
    (node) => node.name === connection.targetNode,
  );
  if (!sourceNode || !targetNode) {
    return withInteractionIssue(
      state,
      `Connection ${edgeId} points at nodes that are not available.`,
    );
  }
  const inputOrOutput =
    endpoint === "target" ? connection.targetInput : connection.sourceOutput;

  return withInteractionPatch(state, {
    mode: "edge-reconnect",
    selectedConnectionId: edgeId,
    edgeDraft: {
      pointerId,
      edgeId,
      reconnectEndpoint: endpoint,
      sourceNodeId: sourceNode.id,
      sourceOutput: connection.sourceOutput,
      targetNodeId: targetNode.id,
      targetInput: connection.targetInput,
      origin: canvasPoint,
      lastPoint: canvasPoint,
      validEndpointNodeIds: validReconnectNodeIds(
        state.document,
        edgeId,
        endpoint,
        inputOrOutput,
      ),
    },
  });
}

function completeConnection(
  state: N8nStudioState,
  nodeId: string,
  endpoint: CanvasConnectionEndpoint,
  inputOrOutput: WorkflowConnection["sourceOutput"] | WorkflowConnection["targetInput"],
) {
  const edgeDraft = state.canvas.interaction.edgeDraft;
  if (!edgeDraft) {
    return withInteractionIssue(state, "No connection draft is active.");
  }
  if (!edgeDraft.edgeId && endpoint !== "target") {
    return withInteractionPatch(state, {
      mode: "idle",
      issue: "New workflow connections must end on a target input handle.",
    });
  }
  if (
    edgeDraft.edgeId &&
    edgeDraft.reconnectEndpoint &&
    edgeDraft.reconnectEndpoint !== endpoint
  ) {
    return withInteractionPatch(state, {
      mode: "idle",
      selectedConnectionId: edgeDraft.edgeId,
      issue: `Use a ${edgeDraft.reconnectEndpoint} handle to reconnect this endpoint.`,
    });
  }

  const validation =
    edgeDraft.edgeId && edgeDraft.reconnectEndpoint
      ? reconnectCanvasConnection(
          state.document,
          edgeDraft.edgeId,
          edgeDraft.reconnectEndpoint,
          nodeId,
          inputOrOutput,
        )
      : createCanvasConnection(state.document, {
          sourceNodeId: edgeDraft.sourceNodeId,
          sourceOutput: edgeDraft.sourceOutput,
          targetNodeId: nodeId,
          targetInput: inputOrOutput as WorkflowConnection["targetInput"],
        });

  if (!validation.valid) {
    return withInteractionPatch(state, {
      mode: "idle",
      selectedConnectionId: edgeDraft.edgeId,
      issue: validation.issue,
    });
  }

  const document =
    edgeDraft.edgeId && edgeDraft.reconnectEndpoint
      ? {
          ...state.document,
          connections: state.document.connections.map((connection) =>
            connection.id === validation.connection.id
              ? validation.connection
              : connection,
          ),
        }
      : {
          ...state.document,
          connections: [...state.document.connections, validation.connection],
        };

  return withCanvasDocument(
    state,
    document,
    state.canvas.selectedNodeId,
    state.canvas.interaction.selectedNodeIds,
    {
      mode: "idle",
      selectedConnectionId: validation.connection.id,
    },
  );
}

export function applyCanvasInteractionToStudioState(
  state: N8nStudioState,
  action: CanvasInteractionAction,
): N8nStudioState {
  switch (action.kind) {
    case "selectNode": {
      if (!selectedNode(state.document, action.nodeId)) {
        return withInteractionIssue(
          state,
          `Node ${action.nodeId} is not available in the workflow document.`,
        );
      }

      const selectedNodeIds = action.additive
        ? Array.from(new Set([...state.canvas.interaction.selectedNodeIds, action.nodeId]))
        : [action.nodeId];

      return withCanvasDocument(
        state,
        state.document,
        action.nodeId,
        selectedNodeIds,
        { mode: "idle" },
      );
    }

    case "selectAll":
      return withCanvasDocument(
        state,
        state.document,
        state.canvas.selectedNodeId,
        state.document.nodes.map((node) => node.id),
      );

    case "beginNodeDrag": {
      const selected = applyCanvasInteractionToStudioState(state, {
        kind: "selectNode",
        nodeId: action.nodeId,
      });
      const node = selectedNode(selected.document, action.nodeId);
      if (!node) {
        return selected;
      }

      return withCanvasDocument(
        selected,
        selected.document,
        action.nodeId,
        selected.canvas.interaction.selectedNodeIds,
        {
          mode: "node-drag",
          activeDrag: {
            pointerId: action.pointerId,
            nodeId: action.nodeId,
            origin: action.canvasPoint,
            lastPoint: action.canvasPoint,
            startPosition: node.position,
            snapToGrid: action.snapToGrid ?? true,
          },
        },
      );
    }

    case "beginCanvasPan":
      return withViewport(
        state,
        state.canvas.viewport,
        {
          mode: "canvas-pan",
          activeDrag: {
            pointerId: action.pointerId,
            origin: action.canvasPoint,
            lastPoint: action.canvasPoint,
            startViewport: {
              x: state.canvas.viewport.x,
              y: state.canvas.viewport.y,
            },
            snapToGrid: false,
          },
        },
      );

    case "movePointer":
      return updatePointer(state, action.pointerId, action.canvasPoint);

    case "endPointer": {
      const activeDrag = state.canvas.interaction.activeDrag;
      const edgeDraft = state.canvas.interaction.edgeDraft;
      if (edgeDraft && edgeDraft.pointerId === action.pointerId) {
        return withInteractionPatch(state, {
          mode: "idle",
          selectedConnectionId: state.canvas.interaction.selectedConnectionId,
        });
      }
      if (!activeDrag || activeDrag.pointerId !== action.pointerId) {
        return state;
      }

      return {
        ...state,
        canvas: {
          ...state.canvas,
          interaction: createCanvasInteractionState(
            state.document,
            state.canvas.selectedNodeId,
            state.canvas.interaction.selectedNodeIds,
          ),
        },
      };
    }

    case "panViewport":
      return withViewport(state, {
        x: state.canvas.viewport.x + action.delta.x,
        y: state.canvas.viewport.y + action.delta.y,
        zoom: state.canvas.viewport.zoom,
      });

    case "zoomViewport":
      return withViewport(state, {
        ...state.canvas.viewport,
        zoom: roundedCanvasZoom(state.canvas.viewport.zoom + action.delta),
      });

    case "fitWorkflow":
      return fitWorkflow(state, action.viewportSize);

    case "tidyWorkflow":
      return tidyWorkflow(state);

    case "deleteSelection":
      return deleteSelectedNodes(state);

    case "nudgeSelection": {
      const document = moveNodes(
        state.document,
        state.canvas.interaction.selectedNodeIds,
        action.delta,
        true,
        startPositionsForNodes(state.document, state.canvas.interaction.selectedNodeIds),
      );

      return withCanvasDocument(
        state,
        document,
        state.canvas.selectedNodeId,
        state.canvas.interaction.selectedNodeIds,
        { mode: "keyboard" },
      );
    }

    case "beginConnectionDrag":
      return beginConnectionDrag(
        state,
        action.sourceNodeId,
        action.sourceOutput,
        action.pointerId,
        action.canvasPoint,
      );

    case "beginReconnectEdge":
      return beginReconnectEdge(
        state,
        action.edgeId,
        action.endpoint,
        action.pointerId,
        action.canvasPoint,
      );

    case "completeConnection":
      return completeConnection(
        state,
        "nodeId" in action ? action.nodeId : action.targetNodeId,
        action.endpoint ?? "target",
        "inputOrOutput" in action ? action.inputOrOutput : action.targetInput,
      );
  }
}
