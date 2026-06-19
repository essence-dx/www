"use client";

import type {
  CanvasConnectionDraft,
  CanvasConnectionEndpoint,
  WorkflowConnection,
  WorkflowNode,
} from "../../lib/n8n-studio/types";

export type WorkflowEdgeLayerProps = {
  connections: WorkflowConnection[];
  edgeDraft?: CanvasConnectionDraft;
  nodes: WorkflowNode[];
  selectedConnectionId?: string;
  onBeginReconnectEdge?: (
    edgeId: string,
    endpoint: CanvasConnectionEndpoint,
    pointerId: number,
    clientPoint: { x: number; y: number },
  ) => void;
};

type EdgeRenderGeometry = {
  source: { x: number; y: number };
  target: { x: number; y: number };
};

function nodeByName(nodes: WorkflowNode[], nodeName: string) {
  return nodes.find((node) => node.name === nodeName);
}

function nodeById(nodes: WorkflowNode[], nodeId: string) {
  return nodes.find((node) => node.id === nodeId);
}

function edgeGeometryForConnection(
  connection: WorkflowConnection,
  nodes: WorkflowNode[],
): EdgeRenderGeometry | undefined {
  const sourceNode = nodeByName(nodes, connection.sourceNode);
  const targetNode = nodeByName(nodes, connection.targetNode);
  if (!sourceNode || !targetNode) {
    return undefined;
  }

  return {
    source: {
      x: sourceNode.position.x + 180,
      y: sourceNode.position.y + 38,
    },
    target: {
      x: targetNode.position.x,
      y: targetNode.position.y + 38,
    },
  };
}

function edgePath(geometry: EdgeRenderGeometry) {
  const controlOffset = Math.max(72, Math.abs(geometry.target.x - geometry.source.x) / 2);

  return [
    `M ${geometry.source.x} ${geometry.source.y}`,
    `C ${geometry.source.x + controlOffset} ${geometry.source.y}`,
    `${geometry.target.x - controlOffset} ${geometry.target.y}`,
    `${geometry.target.x} ${geometry.target.y}`,
  ].join(" ");
}

function draftGeometry(
  edgeDraft: CanvasConnectionDraft | undefined,
  nodes: WorkflowNode[],
): EdgeRenderGeometry | undefined {
  if (!edgeDraft) {
    return undefined;
  }
  const sourceNode = nodeById(nodes, edgeDraft.sourceNodeId);
  if (!sourceNode) {
    return undefined;
  }
  if (edgeDraft.reconnectEndpoint === "source" && edgeDraft.targetNodeId) {
    const targetNode = nodeById(nodes, edgeDraft.targetNodeId);
    if (!targetNode) {
      return undefined;
    }

    return {
      source: edgeDraft.lastPoint,
      target: {
        x: targetNode.position.x,
        y: targetNode.position.y + 38,
      },
    };
  }

  return {
    source: {
      x: sourceNode.position.x + 180,
      y: sourceNode.position.y + 38,
    },
    target: edgeDraft.lastPoint,
  };
}

export function WorkflowEdgeLayer({
  connections,
  edgeDraft,
  nodes,
  onBeginReconnectEdge,
  selectedConnectionId,
}: WorkflowEdgeLayerProps) {
  const draft = draftGeometry(edgeDraft, nodes);

  return (
    <>
      <svg className="n8ns-edge-layer" viewBox="0 0 1080 640" aria-hidden="true">
        {connections.map((connection) => {
          const geometry = edgeGeometryForConnection(connection, nodes);
          if (!geometry) {
            return null;
          }

          return (
            <path
              d={edgePath(geometry)}
              data-connection-id={connection.id}
              data-selected={selectedConnectionId === connection.id ? "true" : "false"}
              key={connection.id}
            />
          );
        })}
        {draft ? (
          <path
            d={edgePath(draft)}
            data-edge-draft="true"
            data-source-node-id={edgeDraft?.sourceNodeId}
          />
        ) : null}
      </svg>
      {connections.map((connection) => {
        const geometry = edgeGeometryForConnection(connection, nodes);
        if (!geometry) {
          return null;
        }

        return (
          <div className="n8ns-edge-reconnect-pair" key={`${connection.id}-handles`}>
            <button
              aria-label={`Reconnect source for ${connection.sourceNode} to ${connection.targetNode}`}
              className="n8ns-edge-reconnect-handle n8ns-edge-reconnect-handle-source"
              data-connection-id={connection.id}
              data-edge-reconnect-endpoint="source"
              onPointerDown={(event) => {
                event.preventDefault();
                event.stopPropagation();
                onBeginReconnectEdge?.(connection.id, "source", event.pointerId, {
                  x: event.clientX,
                  y: event.clientY,
                });
              }}
              style={{
                left: `${geometry.source.x - 7}px`,
                top: `${geometry.source.y - 7}px`,
              }}
              type="button"
            />
            <button
              aria-label={`Reconnect target for ${connection.sourceNode} to ${connection.targetNode}`}
              className="n8ns-edge-reconnect-handle n8ns-edge-reconnect-handle-target"
              data-connection-id={connection.id}
              data-edge-reconnect-endpoint="target"
              onPointerDown={(event) => {
                event.preventDefault();
                event.stopPropagation();
                onBeginReconnectEdge?.(connection.id, "target", event.pointerId, {
                  x: event.clientX,
                  y: event.clientY,
                });
              }}
              style={{
                left: `${geometry.target.x - 7}px`,
                top: `${geometry.target.y - 7}px`,
              }}
              type="button"
            />
          </div>
        );
      })}
    </>
  );
}
