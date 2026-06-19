"use client";

import type { KeyboardEvent, PointerEvent } from "react";

import type {
  CanvasConnectionEndpoint,
  WorkflowConnection,
  WorkflowNode,
} from "../../lib/n8n-studio/types";
import { Icon } from "../icons/icon";

export type WorkflowNodeCardProps = {
  node: WorkflowNode;
  selected: boolean;
  onBeginNodeDrag?: (
    nodeId: string,
    pointerId: number,
    clientPoint: { x: number; y: number },
  ) => void;
  onBeginConnectionDrag?: (
    sourceNodeId: string,
    sourceOutput: WorkflowConnection["sourceOutput"],
    pointerId: number,
    clientPoint: { x: number; y: number },
  ) => void;
  onCompleteConnection?: (
    nodeId: string,
    endpoint: CanvasConnectionEndpoint,
    inputOrOutput: WorkflowConnection["sourceOutput"] | WorkflowConnection["targetInput"],
  ) => void;
  onSelectNode?: (nodeId: string) => void;
  port: WorkflowConnection["sourceOutput"];
};

export function WorkflowNodeCard({
  node,
  onBeginConnectionDrag,
  onBeginNodeDrag,
  onCompleteConnection,
  onSelectNode,
  port,
  selected,
}: WorkflowNodeCardProps) {
  const handlePointerDown = (event: PointerEvent<HTMLElement>) => {
    if (event.button !== 0) {
      return;
    }

    event.stopPropagation();
    onBeginNodeDrag?.(node.id, event.pointerId, {
      x: event.clientX,
      y: event.clientY,
    });
  };
  const handleKeyDown = (event: KeyboardEvent<HTMLElement>) => {
    if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      onSelectNode?.(node.id);
    }
  };
  const handleSourcePointerDown = (event: PointerEvent<HTMLButtonElement>) => {
    if (event.button !== 0) {
      return;
    }

    event.stopPropagation();
    onBeginConnectionDrag?.(node.id, port, event.pointerId, {
      x: event.clientX,
      y: event.clientY,
    });
  };
  const handleSourcePointerUp = (event: PointerEvent<HTMLButtonElement>) => {
    event.stopPropagation();
    onCompleteConnection?.(node.id, "source", port);
  };
  const handleTargetPointerUp = (event: PointerEvent<HTMLButtonElement>) => {
    event.stopPropagation();
    onCompleteConnection?.(node.id, "target", port);
  };

  return (
    <article
      className="n8ns-workflow-node"
      aria-selected={selected}
      data-selected={selected ? "true" : "false"}
      onClick={() => onSelectNode?.(node.id)}
      onKeyDown={handleKeyDown}
      onPointerDown={handlePointerDown}
      role="button"
      style={{ left: `${node.position.x}px`, top: `${node.position.y}px` }}
      tabIndex={0}
    >
      <button
        aria-label={`Connect to ${node.name}`}
        className="n8ns-node-edge-handle n8ns-node-edge-handle-target"
        data-edge-handle="target"
        onClick={(event) => event.stopPropagation()}
        onPointerDown={(event) => event.stopPropagation()}
        onPointerUp={handleTargetPointerUp}
        type="button"
      />
      <div className="n8ns-node-icon">
        <Icon className="n8ns-icon" name="n8n-studio:node" />
      </div>
      <div>
        <strong>{node.name}</strong>
        <span>{node.type}</span>
      </div>
      <button
        aria-label={`Connect from ${node.name}`}
        className="n8ns-node-edge-handle n8ns-node-edge-handle-source"
        data-edge-handle="source"
        onClick={(event) => event.stopPropagation()}
        onPointerDown={handleSourcePointerDown}
        onPointerUp={handleSourcePointerUp}
        type="button"
      />
    </article>
  );
}
