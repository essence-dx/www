import {
  getBoxCenter,
  getElementBounds,
  rotatePoint,
  toBoxLocalPoint,
} from "./geometry";
import { connectorRouteForElement, orthogonalConnectorPoints } from "./connector-routes";
import type {
  WhiteboardBoxElement,
  WhiteboardConnectorAnchor,
  WhiteboardConnectorEndpoint,
  WhiteboardDocument,
  WhiteboardElement,
  WhiteboardElementId,
  WhiteboardPoint,
} from "./model";

type ConnectorElement = Extract<WhiteboardElement, { type: "line" | "arrow" }>;

export function connectorAnchorPoint(
  element: WhiteboardElement,
  anchor: WhiteboardConnectorAnchor | undefined,
  opposite?: WhiteboardPoint,
): WhiteboardPoint {
  if (!isBoxElement(element)) {
    return elementCenter(element);
  }

  const center = getBoxCenter(element);
  const resolved = anchor ?? "auto";

  if (resolved === "auto") {
    return autoAnchorPoint(element, opposite);
  }

  switch (resolved) {
    case "top":
      return roundedPoint(rotatePoint({ x: center.x, y: element.y }, center, element.rotation));
    case "right":
      return roundedPoint(rotatePoint({ x: element.x + element.width, y: center.y }, center, element.rotation));
    case "bottom":
      return roundedPoint(rotatePoint({ x: center.x, y: element.y + element.height }, center, element.rotation));
    case "left":
      return roundedPoint(rotatePoint({ x: element.x, y: center.y }, center, element.rotation));
    case "center":
      return roundedPoint(center);
  }
}

export function rerouteBoundConnectors(
  document: WhiteboardDocument,
  changedIds: readonly WhiteboardElementId[],
  now = new Date().toISOString(),
): WhiteboardDocument {
  const changedIdSet = new Set(changedIds);
  if (changedIdSet.size === 0) return document;

  const targetById = new Map(
    document.elements
      .filter((element) => !element.hidden && isBoxElement(element))
      .map((element) => [element.id, element]),
  );
  let changed = false;
  const elements = document.elements.map((element) => {
    if (!isConnectorElement(element)) return element;

    const next = rerouteConnector(element, targetById, changedIdSet, now);
    if (next !== element) changed = true;
    return next;
  });

  return changed ? { ...document, elements, updatedAt: now } : document;
}

function rerouteConnector(
  connector: ConnectorElement,
  targetById: ReadonlyMap<WhiteboardElementId, WhiteboardElement>,
  changedIds: ReadonlySet<WhiteboardElementId>,
  now: string,
): ConnectorElement {
  const startTarget = targetForBinding(connector.startBinding, targetById);
  const endTarget = targetForBinding(connector.endBinding, targetById);
  const startBinding = startTarget ? connector.startBinding : undefined;
  const endBinding = endTarget ? connector.endBinding : undefined;
  const bindingsChanged = startBinding !== connector.startBinding || endBinding !== connector.endBinding;
  const routeChanged =
    changedIds.has(connector.id) ||
    (connector.startBinding !== undefined && changedIds.has(connector.startBinding.elementId)) ||
    (connector.endBinding !== undefined && changedIds.has(connector.endBinding.elementId));

  if (!bindingsChanged && !routeChanged) return connector;

  if (connector.locked || connector.hidden) {
    return bindingsChanged
      ? {
          ...connector,
          startBinding,
          endBinding,
          updatedAt: now,
        }
      : connector;
  }

  const firstPoint = connector.points[0];
  const lastPoint = connector.points[connector.points.length - 1];
  const middlePoints = connector.points.slice(1, -1);
  const nextFirst = startTarget && startBinding
    ? connectorAnchorPoint(startTarget, startBinding.anchor, connector.points[1] ?? elementCenter(startTarget))
    : firstPoint;
  const nextLast = endTarget && endBinding
    ? connectorAnchorPoint(
        endTarget,
        endBinding.anchor,
        connector.points[connector.points.length - 2] ?? elementCenter(endTarget),
      )
    : lastPoint;
  const nextPoints = connectorRouteForElement(connector) === "orthogonal"
    ? orthogonalConnectorPoints(nextFirst, nextLast)
    : [nextFirst, ...middlePoints, nextLast] as readonly [
        WhiteboardPoint,
        WhiteboardPoint,
        ...WhiteboardPoint[],
      ];

  if (!bindingsChanged && samePoints(connector.points, nextPoints)) return connector;

  return {
    ...connector,
    points: nextPoints,
    startBinding,
    endBinding,
    updatedAt: now,
  };
}

function targetForBinding(
  binding: WhiteboardConnectorEndpoint | undefined,
  targetById: ReadonlyMap<WhiteboardElementId, WhiteboardElement>,
): WhiteboardElement | undefined {
  return binding ? targetById.get(binding.elementId) : undefined;
}

function elementCenter(element: WhiteboardElement): WhiteboardPoint {
  if (isBoxElement(element)) {
    return roundedPoint(getBoxCenter(element));
  }

  const bounds = getElementBounds(element);
  return {
    x: round(bounds.x + bounds.width / 2),
    y: round(bounds.y + bounds.height / 2),
  };
}

function autoAnchorPoint(element: WhiteboardBoxElement, opposite: WhiteboardPoint | undefined): WhiteboardPoint {
  const center = getBoxCenter(element);
  if (!opposite || element.width <= 0 || element.height <= 0) {
    return roundedPoint(center);
  }

  const localOpposite = toBoxLocalPoint(element, opposite);
  const dx = localOpposite.x - center.x;
  const dy = localOpposite.y - center.y;
  if (dx === 0 && dy === 0) {
    return roundedPoint(center);
  }

  const boundary = localBoundaryPoint(element, center, dx, dy);
  return roundedPoint(rotatePoint(boundary, center, element.rotation));
}

function localBoundaryPoint(
  element: WhiteboardBoxElement,
  center: WhiteboardPoint,
  dx: number,
  dy: number,
): WhiteboardPoint {
  const halfWidth = Math.max(0, element.width / 2);
  const halfHeight = Math.max(0, element.height / 2);
  if (halfWidth === 0 || halfHeight === 0) return center;

  if (element.type === "ellipse") {
    const scale = 1 / Math.sqrt((dx * dx) / (halfWidth * halfWidth) + (dy * dy) / (halfHeight * halfHeight));
    return {
      x: center.x + dx * scale,
      y: center.y + dy * scale,
    };
  }

  if (element.type === "diamond") {
    const scale = 1 / (Math.abs(dx) / halfWidth + Math.abs(dy) / halfHeight);
    return {
      x: center.x + dx * scale,
      y: center.y + dy * scale,
    };
  }

  const scale = 1 / Math.max(Math.abs(dx) / halfWidth, Math.abs(dy) / halfHeight);
  return {
    x: center.x + dx * scale,
    y: center.y + dy * scale,
  };
}

function samePoints(left: readonly WhiteboardPoint[], right: readonly WhiteboardPoint[]): boolean {
  return left.length === right.length && left.every((point, index) => {
    const other = right[index];
    return other !== undefined && point.x === other.x && point.y === other.y;
  });
}

function isConnectorElement(element: WhiteboardElement): element is ConnectorElement {
  return element.type === "line" || element.type === "arrow";
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

function roundedPoint(point: WhiteboardPoint): WhiteboardPoint {
  return {
    x: round(point.x),
    y: round(point.y),
  };
}

function round(value: number): number {
  return Number(value.toFixed(12));
}
