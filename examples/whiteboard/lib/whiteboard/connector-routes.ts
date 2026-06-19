import type {
  WhiteboardElement,
  WhiteboardMetadata,
  WhiteboardPoint,
} from "./model";

export type WhiteboardConnectorElement = Extract<WhiteboardElement, { type: "line" | "arrow" }>;
export type WhiteboardConnectorRoute = "straight" | "orthogonal";

export const CONNECTOR_ROUTE_METADATA_KEY = "connectorRoute";

export function connectorRouteForElement(
  element: WhiteboardConnectorElement,
): WhiteboardConnectorRoute {
  return element.metadata?.[CONNECTOR_ROUTE_METADATA_KEY] === "orthogonal"
    ? "orthogonal"
    : "straight";
}

export function connectorRouteMetadata(route: WhiteboardConnectorRoute): WhiteboardMetadata {
  return { [CONNECTOR_ROUTE_METADATA_KEY]: route };
}

export function orthogonalConnectorPoints(
  start: WhiteboardPoint,
  end: WhiteboardPoint,
): readonly [WhiteboardPoint, WhiteboardPoint, ...WhiteboardPoint[]] {
  const middleX = round((start.x + end.x) / 2);
  const points = collapseAdjacentDuplicatePoints([
    roundedPoint(start),
    { x: middleX, y: round(start.y) },
    { x: middleX, y: round(end.y) },
    roundedPoint(end),
  ]);

  const [first, second, ...rest] = points;
  return [first, second, ...rest];
}

function collapseAdjacentDuplicatePoints(
  points: readonly [WhiteboardPoint, WhiteboardPoint, WhiteboardPoint, WhiteboardPoint],
): readonly [WhiteboardPoint, WhiteboardPoint, ...WhiteboardPoint[]] {
  const collapsed: WhiteboardPoint[] = [];

  for (const point of points) {
    const previous = collapsed[collapsed.length - 1];
    if (!previous || previous.x !== point.x || previous.y !== point.y) {
      collapsed.push(point);
    }
  }

  if (collapsed.length === 1) {
    collapsed.push(collapsed[0]);
  }

  const [first, second, ...rest] = collapsed;
  return [first, second, ...rest];
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
