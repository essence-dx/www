"use client";

import { useRef, type KeyboardEvent, type PointerEvent } from "react";

import type { CanvasInteractionAction } from "../../lib/n8n-studio/canvas-interactions";
import type {
  CanvasConnectionEndpoint,
  CanvasProjection,
  WorkflowConnection,
  WorkflowDocument,
} from "../../lib/n8n-studio/types";
import { WorkflowEdgeLayer } from "./workflow-edge-layer";
import { WorkflowNodeCard } from "./workflow-node-card";

export type WorkflowCanvasProps = {
  canvas: CanvasProjection;
  document: WorkflowDocument;
  onCanvasAction?: (canvasAction: CanvasInteractionAction) => void;
};

function clientPoint(event: PointerEvent<HTMLElement>) {
  return {
    x: event.clientX,
    y: event.clientY,
  };
}

function clientPointToCanvasPoint(
  point: { x: number; y: number },
  stage: HTMLElement | null,
  canvas: CanvasProjection,
) {
  const rect = stage?.getBoundingClientRect();

  return {
    x: (point.x - (rect?.left ?? 0) - canvas.viewport.x) / canvas.viewport.zoom,
    y: (point.y - (rect?.top ?? 0) - canvas.viewport.y) / canvas.viewport.zoom,
  };
}

function viewportSize(element: HTMLElement | null) {
  return {
    width: Math.max(1, element?.clientWidth ?? 760),
    height: Math.max(1, element?.clientHeight ?? 420),
  };
}

export function WorkflowCanvas({
  canvas,
  document,
  onCanvasAction,
}: WorkflowCanvasProps) {
  const panelRef = useRef<HTMLElement>(null);
  const stageRef = useRef<HTMLDivElement>(null);

  const dispatchCanvasAction = (canvasAction: CanvasInteractionAction) => {
    onCanvasAction?.(canvasAction);
  };
  const activePort: WorkflowConnection["sourceOutput"] =
    canvas.edgeMode === "ai-tool" ? "ai_tool" : "main";
  const handleBeginNodeDrag = (
    nodeId: string,
    pointerId: number,
    clientPoint: { x: number; y: number },
  ) => {
    dispatchCanvasAction({
      kind: "beginNodeDrag",
      nodeId,
      pointerId,
      canvasPoint: clientPointToCanvasPoint(clientPoint, stageRef.current, canvas),
    });
  };
  const handleBeginConnectionDrag = (
    sourceNodeId: string,
    sourceOutput: WorkflowConnection["sourceOutput"],
    pointerId: number,
    clientPoint: { x: number; y: number },
  ) => {
    dispatchCanvasAction({
      kind: "beginConnectionDrag",
      sourceNodeId,
      sourceOutput,
      pointerId,
      canvasPoint: clientPointToCanvasPoint(clientPoint, stageRef.current, canvas),
    });
  };
  const handleBeginReconnectEdge = (
    edgeId: string,
    endpoint: CanvasConnectionEndpoint,
    pointerId: number,
    clientPoint: { x: number; y: number },
  ) => {
    dispatchCanvasAction({
      kind: "beginReconnectEdge",
      edgeId,
      endpoint,
      pointerId,
      canvasPoint: clientPointToCanvasPoint(clientPoint, stageRef.current, canvas),
    });
  };
  const handleCompleteConnection = (
    nodeId: string,
    endpoint: CanvasConnectionEndpoint,
    inputOrOutput: WorkflowConnection["sourceOutput"] | WorkflowConnection["targetInput"],
  ) => {
    dispatchCanvasAction({
      kind: "completeConnection",
      nodeId,
      endpoint,
      inputOrOutput,
    });
  };
  const handleStagePointerDown = (event: PointerEvent<HTMLDivElement>) => {
    if (event.button !== 0) {
      return;
    }

    event.currentTarget.setPointerCapture?.(event.pointerId);
    dispatchCanvasAction({
      kind: "beginCanvasPan",
      pointerId: event.pointerId,
      canvasPoint: clientPoint(event),
    });
  };
  const handleStagePointerMove = (event: PointerEvent<HTMLDivElement>) => {
    if (!canvas.interaction.activeDrag && !canvas.interaction.edgeDraft) {
      return;
    }
    const mode = canvas.interaction.mode;
    const isCanvasCoordinateMode =
      mode === "node-drag" || mode === "edge-drag" || mode === "edge-reconnect";

    dispatchCanvasAction({
      kind: "movePointer",
      pointerId: event.pointerId,
      canvasPoint: isCanvasCoordinateMode
        ? clientPointToCanvasPoint(clientPoint(event), stageRef.current, canvas)
        : clientPoint(event),
    });
  };
  const handleStagePointerEnd = (event: PointerEvent<HTMLDivElement>) => {
    dispatchCanvasAction({
      kind: "endPointer",
      pointerId: event.pointerId,
    });
  };
  const handleKeyDown = (event: KeyboardEvent<HTMLElement>) => {
    const target = event.target as HTMLElement;
    if (target.matches("input, textarea, select, button")) {
      return;
    }

    if (event.key === "Delete" || event.key === "Backspace") {
      event.preventDefault();
      dispatchCanvasAction({ kind: "deleteSelection" });
    } else if (event.key === "f") {
      event.preventDefault();
      dispatchCanvasAction({
        kind: "fitWorkflow",
        viewportSize: viewportSize(stageRef.current),
      });
    } else if (event.key === "0") {
      event.preventDefault();
      dispatchCanvasAction({
        kind: "zoomViewport",
        delta: 0.9 - canvas.viewport.zoom,
      });
    } else if (event.key === "+" || event.key === "=") {
      event.preventDefault();
      dispatchCanvasAction({ kind: "zoomViewport", delta: 0.1 });
    } else if (event.key === "-") {
      event.preventDefault();
      dispatchCanvasAction({ kind: "zoomViewport", delta: -0.1 });
    } else if (event.key.startsWith("Arrow")) {
      event.preventDefault();
      const amount = event.shiftKey ? 48 : 24;
      const delta =
        event.key === "ArrowUp"
          ? { x: 0, y: -amount }
          : event.key === "ArrowDown"
            ? { x: 0, y: amount }
            : event.key === "ArrowLeft"
              ? { x: -amount, y: 0 }
              : { x: amount, y: 0 };

      dispatchCanvasAction({ kind: "nudgeSelection", delta });
    }
  };

  return (
    <section
      ref={panelRef}
      className="n8ns-canvas-panel"
      aria-label="Workflow canvas"
      data-studio-surface="workflow-canvas"
      data-canvas-edge-mode={canvas.edgeMode}
      data-canvas-interaction-mode={canvas.interaction.mode}
      data-canvas-selected-node-count={canvas.interaction.selectedNodeIds.length}
      onKeyDown={handleKeyDown}
      tabIndex={0}
    >
      <div className="n8ns-canvas-toolbar" aria-label="Canvas controls">
        <button
          aria-label="Fit workflow"
          onClick={() =>
            dispatchCanvasAction({
              kind: "fitWorkflow",
              viewportSize: viewportSize(stageRef.current ?? panelRef.current),
            })
          }
          type="button"
        >
          <dx-icon className="n8ns-icon" name="n8n-studio:fit" />
        </button>
        <button
          aria-label="Tidy workflow"
          onClick={() => dispatchCanvasAction({ kind: "tidyWorkflow" })}
          type="button"
        >
          <dx-icon className="n8ns-icon" name="n8n-studio:tidy" />
        </button>
        <button
          aria-label="Zoom out"
          onClick={() => dispatchCanvasAction({ kind: "zoomViewport", delta: -0.1 })}
          type="button"
        >
          <dx-icon className="n8ns-icon" name="n8n-studio:zoom-out" />
        </button>
        <button
          aria-label="Zoom in"
          onClick={() => dispatchCanvasAction({ kind: "zoomViewport", delta: 0.1 })}
          type="button"
        >
          <dx-icon className="n8ns-icon" name="n8n-studio:zoom-in" />
        </button>
        <span>{Math.round(canvas.viewport.zoom * 100)}%</span>
      </div>
      <div
        ref={stageRef}
        className="n8ns-canvas-stage"
        onPointerCancel={handleStagePointerEnd}
        onPointerDown={handleStagePointerDown}
        onPointerMove={handleStagePointerMove}
        onPointerUp={handleStagePointerEnd}
      >
        <div
          className="n8ns-canvas-content"
          style={{
            transform: `translate(${canvas.viewport.x}px, ${canvas.viewport.y}px) scale(${canvas.viewport.zoom})`,
          }}
        >
          <WorkflowEdgeLayer
            connections={document.connections}
            edgeDraft={canvas.interaction.edgeDraft}
            nodes={document.nodes}
            onBeginReconnectEdge={handleBeginReconnectEdge}
            selectedConnectionId={canvas.interaction.selectedConnectionId}
          />
          {document.nodes.map((node) => (
            <WorkflowNodeCard
              key={node.id}
              node={node}
              onBeginConnectionDrag={handleBeginConnectionDrag}
              onBeginNodeDrag={handleBeginNodeDrag}
              onCompleteConnection={handleCompleteConnection}
              onSelectNode={(nodeId) =>
                dispatchCanvasAction({ kind: "selectNode", nodeId })
              }
              port={activePort}
              selected={node.id === canvas.selectedNodeId}
            />
          ))}
        </div>
      </div>
      <div className="n8ns-canvas-footer">
        <span>{document.nodes.length} nodes</span>
        <span>{document.connections.length} connections</span>
        <span>{document.pinData["Read Connector Manifest"]?.length ?? 0} pinned item</span>
        {canvas.interaction.issue ? <span>{canvas.interaction.issue}</span> : null}
      </div>
    </section>
  );
}
