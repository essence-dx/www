import type { CanvasBounds, CanvasPoint, WorkflowNode } from "./types";

export const CANVAS_NODE_WIDTH = 180;
export const CANVAS_NODE_HEIGHT = 76;
export const CANVAS_GRID_SIZE = 24;
export const CANVAS_MIN_ZOOM = 0.4;
export const CANVAS_MAX_ZOOM = 1.6;
export const CANVAS_MAX_FIT_ZOOM = 1.4;

export function clampCanvasZoom(value: number, maxZoom = CANVAS_MAX_ZOOM) {
  return Math.min(maxZoom, Math.max(CANVAS_MIN_ZOOM, value));
}

export function roundedCanvasZoom(value: number) {
  return Number(clampCanvasZoom(value).toFixed(2));
}

export function snapCanvasPosition(value: number) {
  return Math.round(value / CANVAS_GRID_SIZE) * CANVAS_GRID_SIZE;
}

export function createCanvasNodeBounds(nodes: WorkflowNode[]): CanvasBounds {
  if (nodes.length === 0) {
    return { minX: 0, minY: 0, maxX: 0, maxY: 0 };
  }

  return nodes.reduce(
    (bounds, node) => ({
      minX: Math.min(bounds.minX, node.position.x),
      minY: Math.min(bounds.minY, node.position.y),
      maxX: Math.max(bounds.maxX, node.position.x + CANVAS_NODE_WIDTH),
      maxY: Math.max(bounds.maxY, node.position.y + CANVAS_NODE_HEIGHT),
    }),
    {
      minX: Number.POSITIVE_INFINITY,
      minY: Number.POSITIVE_INFINITY,
      maxX: Number.NEGATIVE_INFINITY,
      maxY: Number.NEGATIVE_INFINITY,
    },
  );
}

export function createFitWorkflowViewport(
  nodes: WorkflowNode[],
  viewportSize: { width: number; height: number },
  padding = 64,
) {
  const bounds = createCanvasNodeBounds(nodes);
  const contentWidth = Math.max(1, bounds.maxX - bounds.minX);
  const contentHeight = Math.max(1, bounds.maxY - bounds.minY);
  const zoom = Number(
    clampCanvasZoom(
      Math.min(
        (viewportSize.width - padding * 2) / contentWidth,
        (viewportSize.height - padding * 2) / contentHeight,
      ),
      CANVAS_MAX_FIT_ZOOM,
    ).toFixed(2),
  );

  return {
    x: Math.round(padding - bounds.minX * zoom),
    y: Math.round(padding - bounds.minY * zoom),
    zoom,
  };
}

export function pointDelta(origin: CanvasPoint, current: CanvasPoint): CanvasPoint {
  return {
    x: current.x - origin.x,
    y: current.y - origin.y,
  };
}
