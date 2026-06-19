import {
  containsPoint,
  distancePointToSegment,
  getDiamondPoints,
  getElementBounds,
  getElementSegments,
  inflateRect,
  isPointInBoxElement,
  isPointInEllipseElement,
  isPointInPolygon,
  toBoxLocalPoint,
} from "./geometry";
import type {
  WhiteboardBoxElement,
  WhiteboardDocument,
  WhiteboardElement,
  WhiteboardElementId,
  WhiteboardPoint,
  WhiteboardRect,
} from "./model";

export type WhiteboardHitArea = "fill" | "stroke" | "bounds";

export interface WhiteboardHit {
  readonly elementId: WhiteboardElementId;
  readonly element: WhiteboardElement;
  readonly bounds: WhiteboardRect;
  readonly hitArea: WhiteboardHitArea;
  readonly distance: number;
}

export interface WhiteboardHitTestOptions {
  readonly tolerance?: number;
  readonly includeHidden?: boolean;
  readonly includeLocked?: boolean;
  readonly excludeElementIds?: readonly WhiteboardElementId[];
}

export function hitTestScene(
  document: WhiteboardDocument,
  point: WhiteboardPoint,
  options: WhiteboardHitTestOptions = {},
): WhiteboardHit | null {
  return hitTestElements(document.elements, point, options);
}

export function hitTestConnectorTarget(
  document: WhiteboardDocument,
  point: WhiteboardPoint,
  options: WhiteboardHitTestOptions = {},
): WhiteboardHit | null {
  return hitTestElements(
    document.elements.filter((element) => element.type !== "line" && element.type !== "arrow"),
    point,
    options,
  );
}

export function hitTestElements(
  elements: readonly WhiteboardElement[],
  point: WhiteboardPoint,
  options: WhiteboardHitTestOptions = {},
): WhiteboardHit | null {
  for (let index = elements.length - 1; index >= 0; index -= 1) {
    const element = elements[index];
    if (!element) continue;
    const hit = hitTestElement(element, point, options);
    if (hit) return hit;
  }

  return null;
}

export function hitTestElement(
  element: WhiteboardElement,
  point: WhiteboardPoint,
  options: WhiteboardHitTestOptions = {},
): WhiteboardHit | null {
  if (!options.includeHidden && element.hidden) return null;
  if (options.includeLocked === false && element.locked) return null;
  if (options.excludeElementIds?.includes(element.id)) return null;

  const tolerance = options.tolerance ?? 4;
  const bounds = getElementBounds(element);
  if (!containsPoint(inflateRect(bounds, tolerance), point)) return null;

  if (element.type === "rectangle" || element.type === "text") {
    return hitTestBoxElement(element, point, bounds, tolerance);
  }

  if (element.type === "ellipse") {
    return hitTestEllipseElement(element, point, bounds, tolerance);
  }

  if (element.type === "diamond") {
    return hitTestDiamondElement(element, point, bounds, tolerance);
  }

  if (element.type === "path") {
    const hit = hitTestPathStroke(element, point, bounds, tolerance);
    if (hit) return hit;
    if (element.closed && hasVisibleFill(element) && isPointInPolygon(point, element.points)) {
      return createHit(element, bounds, "fill", 0);
    }
    return null;
  }

  return hitTestPathStroke(element, point, bounds, tolerance);
}

function hitTestBoxElement(
  element: Extract<WhiteboardBoxElement, { type: "rectangle" | "text" }>,
  point: WhiteboardPoint,
  bounds: WhiteboardRect,
  tolerance: number,
): WhiteboardHit | null {
  if (!isPointInBoxElement(element, point, tolerance)) return null;
  if (element.type === "text" || hasVisibleFill(element)) {
    return createHit(element, bounds, "fill", 0);
  }

  const localPoint = toBoxLocalPoint(element, point);
  const edgeDistance = Math.min(
    Math.abs(localPoint.x - element.x),
    Math.abs(localPoint.x - (element.x + element.width)),
    Math.abs(localPoint.y - element.y),
    Math.abs(localPoint.y - (element.y + element.height)),
  );

  return edgeDistance <= strokeTolerance(element, tolerance)
    ? createHit(element, bounds, "stroke", edgeDistance)
    : null;
}

function hitTestEllipseElement(
  element: Extract<WhiteboardBoxElement, { type: "ellipse" }>,
  point: WhiteboardPoint,
  bounds: WhiteboardRect,
  tolerance: number,
): WhiteboardHit | null {
  if (!isPointInEllipseElement(element, point, tolerance)) return null;
  if (hasVisibleFill(element) && isPointInEllipseElement(element, point, 0)) {
    return createHit(element, bounds, "fill", 0);
  }
  return createHit(element, bounds, "stroke", 0);
}

function hitTestDiamondElement(
  element: Extract<WhiteboardBoxElement, { type: "diamond" }>,
  point: WhiteboardPoint,
  bounds: WhiteboardRect,
  tolerance: number,
): WhiteboardHit | null {
  const points = getDiamondPoints(element);
  if (hasVisibleFill(element) && isPointInPolygon(point, points)) {
    return createHit(element, bounds, "fill", 0);
  }

  let bestDistance = Number.POSITIVE_INFINITY;
  for (let index = 0; index < points.length; index += 1) {
    const start = points[index];
    const end = points[(index + 1) % points.length];
    if (!start || !end) continue;
    bestDistance = Math.min(bestDistance, distancePointToSegment(point, start, end));
  }

  return bestDistance <= strokeTolerance(element, tolerance)
    ? createHit(element, bounds, "stroke", bestDistance)
    : null;
}

function hitTestPathStroke(
  element: Extract<WhiteboardElement, { type: "path" | "freehand" | "line" | "arrow" }>,
  point: WhiteboardPoint,
  bounds: WhiteboardRect,
  tolerance: number,
): WhiteboardHit | null {
  let bestDistance = Number.POSITIVE_INFINITY;

  for (const [start, end] of getElementSegments(element)) {
    bestDistance = Math.min(bestDistance, distancePointToSegment(point, start, end));
  }

  return bestDistance <= strokeTolerance(element, tolerance)
    ? createHit(element, bounds, "stroke", bestDistance)
    : null;
}

function createHit(
  element: WhiteboardElement,
  bounds: WhiteboardRect,
  hitArea: WhiteboardHitArea,
  distance: number,
): WhiteboardHit {
  return {
    elementId: element.id,
    element,
    bounds,
    hitArea,
    distance,
  };
}

function strokeTolerance(element: WhiteboardElement, tolerance: number): number {
  return Math.max(tolerance, element.style.strokeWidth / 2 + tolerance);
}

function hasVisibleFill(element: WhiteboardElement): boolean {
  const fill = element.style.fill.trim().toLowerCase();
  return element.style.opacity > 0 && fill !== "none" && fill !== "transparent";
}
