import type {
  CanvasConnectionEndpoint,
  WorkflowConnection,
  WorkflowDocument,
  WorkflowNode,
} from "./types";

export type CanvasConnectionEndpointDraft = {
  sourceNodeId: string;
  sourceOutput: WorkflowConnection["sourceOutput"];
  targetNodeId: string;
  targetInput: WorkflowConnection["targetInput"];
};

export type CanvasConnectionValidationResult =
  | {
      valid: true;
      connection: WorkflowConnection;
    }
  | {
      valid: false;
      issue: string;
    };

function nodeById(document: WorkflowDocument, nodeId: string) {
  return document.nodes.find((node) => node.id === nodeId);
}

function nodeByName(document: WorkflowDocument, nodeName: string) {
  return document.nodes.find((node) => node.name === nodeName);
}

function connectionIdForDraft(draft: CanvasConnectionEndpointDraft) {
  return `edge-${draft.sourceNodeId}-to-${draft.targetNodeId}-${draft.sourceOutput}-${draft.targetInput}`;
}

function connectionMatchesDraft(
  connection: WorkflowConnection,
  draft: CanvasConnectionEndpointDraft,
  sourceNode: WorkflowNode,
  targetNode: WorkflowNode,
) {
  return (
    connection.sourceNode === sourceNode.name &&
    connection.targetNode === targetNode.name &&
    connection.sourceOutput === draft.sourceOutput &&
    connection.targetInput === draft.targetInput
  );
}

export function validConnectionTargetNodeIds(
  document: WorkflowDocument,
  sourceNodeId: string,
  sourceOutput: WorkflowConnection["sourceOutput"],
  targetInput: WorkflowConnection["targetInput"] = "main",
  excludedEdgeId?: string,
) {
  const sourceNode = nodeById(document, sourceNodeId);
  if (!sourceNode) {
    return [];
  }

  return document.nodes
    .filter((targetNode) => targetNode.id !== sourceNodeId)
    .filter((targetNode) => {
      const draft = {
        sourceNodeId,
        sourceOutput,
        targetNodeId: targetNode.id,
        targetInput,
      };

      return validateCanvasConnection(document, draft, excludedEdgeId).valid;
    })
    .map((node) => node.id);
}

export function validReconnectNodeIds(
  document: WorkflowDocument,
  edgeId: string,
  endpoint: CanvasConnectionEndpoint,
  inputOrOutput: WorkflowConnection["sourceOutput"] | WorkflowConnection["targetInput"],
) {
  return document.nodes
    .filter((node) =>
      reconnectCanvasConnection(document, edgeId, endpoint, node.id, inputOrOutput)
        .valid,
    )
    .map((node) => node.id);
}

export function validateCanvasConnection(
  document: WorkflowDocument,
  draft: CanvasConnectionEndpointDraft,
  excludedEdgeId?: string,
): CanvasConnectionValidationResult {
  const sourceNode = nodeById(document, draft.sourceNodeId);
  const targetNode = nodeById(document, draft.targetNodeId);

  if (!sourceNode) {
    return {
      valid: false,
      issue: `Source node ${draft.sourceNodeId} is not available in the workflow document.`,
    };
  }
  if (!targetNode) {
    return {
      valid: false,
      issue: `Target node ${draft.targetNodeId} is not available in the workflow document.`,
    };
  }
  if (sourceNode.id === targetNode.id) {
    return {
      valid: false,
      issue: "A workflow node cannot connect to itself.",
    };
  }

  const duplicate = document.connections.find(
    (connection) =>
      connection.id !== excludedEdgeId &&
      connectionMatchesDraft(connection, draft, sourceNode, targetNode),
  );
  if (duplicate) {
    return {
      valid: false,
      issue: `${sourceNode.name} is already connected to ${targetNode.name}.`,
    };
  }

  return {
    valid: true,
    connection: {
      id: excludedEdgeId ?? connectionIdForDraft(draft),
      sourceNode: sourceNode.name,
      targetNode: targetNode.name,
      sourceOutput: draft.sourceOutput,
      targetInput: draft.targetInput,
      index: document.connections.length,
    },
  };
}

export function createCanvasConnection(
  document: WorkflowDocument,
  draft: CanvasConnectionEndpointDraft,
): CanvasConnectionValidationResult {
  const validation = validateCanvasConnection(document, draft);
  if (!validation.valid) {
    return validation;
  }

  return {
    valid: true,
    connection: {
      ...validation.connection,
      index: document.connections.length,
    },
  };
}

export function reconnectCanvasConnection(
  document: WorkflowDocument,
  edgeId: string,
  endpoint: CanvasConnectionEndpoint,
  nodeId: string,
  inputOrOutput: WorkflowConnection["sourceOutput"] | WorkflowConnection["targetInput"],
): CanvasConnectionValidationResult {
  const existingConnection = document.connections.find(
    (connection) => connection.id === edgeId,
  );
  if (!existingConnection) {
    return {
      valid: false,
      issue: `Connection ${edgeId} is not available in the workflow document.`,
    };
  }

  const existingSourceNode = nodeByName(document, existingConnection.sourceNode);
  const existingTargetNode = nodeByName(document, existingConnection.targetNode);
  if (!existingSourceNode || !existingTargetNode) {
    return {
      valid: false,
      issue: `Connection ${edgeId} points at nodes that are not available in the workflow document.`,
    };
  }

  const draft =
    endpoint === "target"
      ? {
          sourceNodeId: existingSourceNode.id,
          sourceOutput: existingConnection.sourceOutput,
          targetNodeId: nodeId,
          targetInput: inputOrOutput as WorkflowConnection["targetInput"],
        }
      : {
          sourceNodeId: nodeId,
          sourceOutput: inputOrOutput as WorkflowConnection["sourceOutput"],
          targetNodeId: existingTargetNode.id,
          targetInput: existingConnection.targetInput,
        };
  const validation = validateCanvasConnection(document, draft, edgeId);
  if (!validation.valid) {
    return validation;
  }

  return {
    valid: true,
    connection: {
      ...validation.connection,
      id: edgeId,
      index: existingConnection.index,
    },
  };
}
