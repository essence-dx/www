import { createCanvasNodeBounds } from "./canvas-geometry";
import type {
  CanvasInteractionState,
  WorkflowDocument,
} from "./types";

export const canvasKeyboardShortcuts = [
  { key: "Delete", action: "Delete selected node when another node remains" },
  { key: "Backspace", action: "Delete selected node when another node remains" },
  { key: "f", action: "Fit workflow to canvas" },
  { key: "0", action: "Reset viewport zoom" },
  { key: "+", action: "Zoom in" },
  { key: "-", action: "Zoom out" },
  { key: "ArrowUp", action: "Move selection up" },
  { key: "ArrowDown", action: "Move selection down" },
  { key: "ArrowLeft", action: "Move selection left" },
  { key: "ArrowRight", action: "Move selection right" },
];

function normalizeSelection(
  document: WorkflowDocument,
  selectedNodeIds: string[],
  fallbackNodeId: string,
) {
  const existingIds = new Set(document.nodes.map((node) => node.id));
  const normalized = selectedNodeIds.filter((nodeId) => existingIds.has(nodeId));
  if (normalized.length > 0) {
    return normalized;
  }
  return existingIds.has(fallbackNodeId) ? [fallbackNodeId] : [];
}

export function createCanvasInteractionState(
  document: WorkflowDocument,
  selectedNodeId: string,
  selectedNodeIds: string[] = [selectedNodeId],
  patch: Partial<CanvasInteractionState> = {},
): CanvasInteractionState {
  const normalizedSelection = normalizeSelection(
    document,
    selectedNodeIds,
    selectedNodeId,
  );

  return {
    mode: "idle",
    selectedNodeIds: normalizedSelection,
    focusedNodeId: normalizedSelection[0],
    keyboardShortcuts: canvasKeyboardShortcuts,
    bounds: createCanvasNodeBounds(document.nodes),
    canDeleteSelection:
      normalizedSelection.length > 0 &&
      document.nodes.length - normalizedSelection.length > 0,
    ...patch,
  };
}
