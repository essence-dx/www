import {
  containsPoint,
  getElementBounds,
  normalizeRect,
  rectFromPoints,
  unionRects,
} from "../geometry";
import { hitTestElements } from "../hit-test";
import type {
  WhiteboardBoxElement,
  WhiteboardElement,
  WhiteboardElementId,
  WhiteboardElementPatch,
  WhiteboardPoint,
  WhiteboardRect,
  WhiteboardSize,
  WhiteboardViewport,
} from "../model";

export type WhiteboardSelectionHandle =
  | "nw"
  | "n"
  | "ne"
  | "e"
  | "se"
  | "s"
  | "sw"
  | "w";

export type WhiteboardHandleRect = WhiteboardRect & {
  readonly handle: WhiteboardSelectionHandle;
};

export type FitViewportOptions = {
  readonly padding?: number;
  readonly minZoom?: number;
  readonly maxZoom?: number;
  readonly minimumBoundsSize?: number;
};

const HANDLE_ORDER: readonly WhiteboardSelectionHandle[] = ["nw", "n", "ne", "e", "se", "s", "sw", "w"];

export function roundCoordinate(value: number): number {
  return Number(value.toFixed(12));
}

export function screenToWorld(point: WhiteboardPoint, viewport: WhiteboardViewport): WhiteboardPoint {
  return {
    x: roundCoordinate((point.x - viewport.x) / viewport.zoom),
    y: roundCoordinate((point.y - viewport.y) / viewport.zoom),
  };
}

export function worldToScreen(point: WhiteboardPoint, viewport: WhiteboardViewport): WhiteboardPoint {
  return {
    x: roundCoordinate(point.x * viewport.zoom + viewport.x),
    y: roundCoordinate(point.y * viewport.zoom + viewport.y),
  };
}

export function boundsToScreenRect(bounds: WhiteboardRect, viewport: WhiteboardViewport): WhiteboardRect {
  const origin = worldToScreen(bounds, viewport);
  return {
    x: origin.x,
    y: origin.y,
    width: roundCoordinate(bounds.width * viewport.zoom),
    height: roundCoordinate(bounds.height * viewport.zoom),
  };
}

export function clampViewportZoom(zoom: number, minZoom = 0.05, maxZoom = 8): number {
  return roundCoordinate(Math.max(minZoom, Math.min(maxZoom, zoom)));
}

export function fitViewportToBounds(
  bounds: WhiteboardRect,
  size: WhiteboardSize,
  options: FitViewportOptions = {},
): WhiteboardViewport {
  const padding = Math.max(0, options.padding ?? 48);
  const minimumBoundsSize = Math.max(1, options.minimumBoundsSize ?? 1);
  const usableWidth = Math.max(1, size.width - padding * 2);
  const usableHeight = Math.max(1, size.height - padding * 2);
  const normalizedBounds = normalizeRect(bounds);
  const boundsWidth = Math.max(minimumBoundsSize, normalizedBounds.width);
  const boundsHeight = Math.max(minimumBoundsSize, normalizedBounds.height);
  const zoom = clampViewportZoom(
    Math.min(usableWidth / boundsWidth, usableHeight / boundsHeight),
    options.minZoom,
    options.maxZoom,
  );
  const center = {
    x: normalizedBounds.x + normalizedBounds.width / 2,
    y: normalizedBounds.y + normalizedBounds.height / 2,
  };

  return {
    x: roundCoordinate(size.width / 2 - center.x * zoom),
    y: roundCoordinate(size.height / 2 - center.y * zoom),
    zoom,
  };
}

export function normalizeBounds(start: WhiteboardPoint, end: WhiteboardPoint): WhiteboardRect {
  return normalizeRect({
    x: start.x,
    y: start.y,
    width: end.x - start.x,
    height: end.y - start.y,
  });
}

export function elementBounds(element: WhiteboardElement): WhiteboardRect {
  return getElementBounds(element);
}

export function elementsById(
  elements: readonly WhiteboardElement[],
  ids: readonly WhiteboardElementId[],
): WhiteboardElement[] {
  const idSet = new Set(ids);
  return elements.filter((element) => idSet.has(element.id));
}

export function selectionBounds(elements: readonly WhiteboardElement[]): WhiteboardRect | null {
  if (elements.length === 0) {
    return null;
  }

  return unionRects(elements.map(getElementBounds));
}

export function selectionHandleRects(bounds: WhiteboardRect, handleSize = 8): WhiteboardHandleRect[] {
  const half = handleSize / 2;
  const centerX = bounds.x + bounds.width / 2;
  const centerY = bounds.y + bounds.height / 2;
  const right = bounds.x + bounds.width;
  const bottom = bounds.y + bounds.height;

  const centers: Record<WhiteboardSelectionHandle, WhiteboardPoint> = {
    nw: { x: bounds.x, y: bounds.y },
    n: { x: centerX, y: bounds.y },
    ne: { x: right, y: bounds.y },
    e: { x: right, y: centerY },
    se: { x: right, y: bottom },
    s: { x: centerX, y: bottom },
    sw: { x: bounds.x, y: bottom },
    w: { x: bounds.x, y: centerY },
  };

  return HANDLE_ORDER.map((handle) => ({
    handle,
    x: roundCoordinate(centers[handle].x - half),
    y: roundCoordinate(centers[handle].y - half),
    width: handleSize,
    height: handleSize,
  }));
}

export function detectSelectionHandle(
  bounds: WhiteboardRect,
  point: WhiteboardPoint,
  handleSize = 8,
): WhiteboardSelectionHandle | null {
  for (const rect of selectionHandleRects(bounds, handleSize)) {
    if (containsPoint(rect, point)) {
      return rect.handle;
    }
  }

  return null;
}

export function resizeBoundsFromHandle(
  initialBounds: WhiteboardRect,
  handle: WhiteboardSelectionHandle,
  point: WhiteboardPoint,
  minimumSize = 8,
): WhiteboardRect {
  let left = initialBounds.x;
  let top = initialBounds.y;
  let right = initialBounds.x + initialBounds.width;
  let bottom = initialBounds.y + initialBounds.height;

  if (handle.includes("w")) {
    left = Math.min(point.x, right - minimumSize);
  }
  if (handle.includes("e")) {
    right = Math.max(point.x, left + minimumSize);
  }
  if (handle.includes("n")) {
    top = Math.min(point.y, bottom - minimumSize);
  }
  if (handle.includes("s")) {
    bottom = Math.max(point.y, top + minimumSize);
  }

  return {
    x: roundCoordinate(left),
    y: roundCoordinate(top),
    width: roundCoordinate(right - left),
    height: roundCoordinate(bottom - top),
  };
}

export function patchElementToBounds(element: WhiteboardElement, next: WhiteboardRect): WhiteboardElementPatch {
  const current = getElementBounds(element);

  if (isBoxElement(element)) {
    return {
      x: next.x,
      y: next.y,
      width: next.width,
      height: next.height,
    };
  }

  const points = "points" in element ? element.points : [];
  const scaleX = current.width === 0 ? 1 : next.width / current.width;
  const scaleY = current.height === 0 ? 1 : next.height / current.height;

  return {
    points: points.map((point) => ({
      x: roundCoordinate(next.x + (point.x - current.x) * scaleX),
      y: roundCoordinate(next.y + (point.y - current.y) * scaleY),
    })),
  };
}

export function translateElementPatch(element: WhiteboardElement, delta: WhiteboardPoint): WhiteboardElementPatch {
  if (isBoxElement(element)) {
    return {
      x: roundCoordinate(element.x + delta.x),
      y: roundCoordinate(element.y + delta.y),
    };
  }

  return {
    points: element.points.map((point) => ({
      x: roundCoordinate(point.x + delta.x),
      y: roundCoordinate(point.y + delta.y),
    })),
  };
}

export function hitTestDocument(
  elements: readonly WhiteboardElement[],
  point: WhiteboardPoint,
  tolerance = 6,
): WhiteboardElement | null {
  return hitTestElements(elements, point, { tolerance })?.element ?? null;
}

export function boundsFromPoints(points: readonly WhiteboardPoint[]): WhiteboardRect {
  return rectFromPoints(points);
}

function isBoxElement(element: WhiteboardElement): element is WhiteboardBoxElement {
  return (
    element.type === "rectangle" ||
    element.type === "ellipse" ||
    element.type === "diamond" ||
    element.type === "text" ||
    element.type === "image"
  );
}
