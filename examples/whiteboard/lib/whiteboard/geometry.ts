import type {
  WhiteboardBoxElement,
  WhiteboardElement,
  WhiteboardPathElement,
  WhiteboardPoint,
  WhiteboardRect,
} from "./model";

export const EMPTY_WHITEBOARD_RECT: WhiteboardRect = {
  x: 0,
  y: 0,
  width: 0,
  height: 0,
};

export function normalizeRect(rect: WhiteboardRect): WhiteboardRect {
  const x = rect.width < 0 ? rect.x + rect.width : rect.x;
  const y = rect.height < 0 ? rect.y + rect.height : rect.y;

  return {
    x,
    y,
    width: Math.abs(rect.width),
    height: Math.abs(rect.height),
  };
}

export function rectFromPoints(points: readonly WhiteboardPoint[]): WhiteboardRect {
  if (points.length === 0) return EMPTY_WHITEBOARD_RECT;

  let minX = points[0]?.x ?? 0;
  let maxX = minX;
  let minY = points[0]?.y ?? 0;
  let maxY = minY;

  for (const point of points) {
    minX = Math.min(minX, point.x);
    maxX = Math.max(maxX, point.x);
    minY = Math.min(minY, point.y);
    maxY = Math.max(maxY, point.y);
  }

  return {
    x: minX,
    y: minY,
    width: maxX - minX,
    height: maxY - minY,
  };
}

export function unionRects(rects: readonly WhiteboardRect[]): WhiteboardRect {
  const first = rects[0];
  if (!first) return EMPTY_WHITEBOARD_RECT;

  let minX = first.x;
  let minY = first.y;
  let maxX = first.x + first.width;
  let maxY = first.y + first.height;

  for (const rect of rects.slice(1)) {
    minX = Math.min(minX, rect.x);
    minY = Math.min(minY, rect.y);
    maxX = Math.max(maxX, rect.x + rect.width);
    maxY = Math.max(maxY, rect.y + rect.height);
  }

  return {
    x: minX,
    y: minY,
    width: maxX - minX,
    height: maxY - minY,
  };
}

export function inflateRect(rect: WhiteboardRect, amount: number): WhiteboardRect {
  return {
    x: rect.x - amount,
    y: rect.y - amount,
    width: rect.width + amount * 2,
    height: rect.height + amount * 2,
  };
}

export function containsPoint(rect: WhiteboardRect, point: WhiteboardPoint): boolean {
  const normalized = normalizeRect(rect);
  return (
    point.x >= normalized.x &&
    point.x <= normalized.x + normalized.width &&
    point.y >= normalized.y &&
    point.y <= normalized.y + normalized.height
  );
}

export function rectsIntersect(left: WhiteboardRect, right: WhiteboardRect): boolean {
  const a = normalizeRect(left);
  const b = normalizeRect(right);

  return (
    a.x <= b.x + b.width &&
    a.x + a.width >= b.x &&
    a.y <= b.y + b.height &&
    a.y + a.height >= b.y
  );
}

export function rectHasMinimumExtent(rect: WhiteboardRect, minimumExtent = 2): boolean {
  const normalized = normalizeRect(rect);
  return normalized.width >= minimumExtent || normalized.height >= minimumExtent;
}

export function distanceBetweenPoints(
  left: WhiteboardPoint,
  right: WhiteboardPoint,
): number {
  return Math.hypot(left.x - right.x, left.y - right.y);
}

export function distancePointToSegment(
  point: WhiteboardPoint,
  start: WhiteboardPoint,
  end: WhiteboardPoint,
): number {
  const dx = end.x - start.x;
  const dy = end.y - start.y;
  const lengthSquared = dx * dx + dy * dy;

  if (lengthSquared === 0) return distanceBetweenPoints(point, start);

  const t = Math.max(
    0,
    Math.min(1, ((point.x - start.x) * dx + (point.y - start.y) * dy) / lengthSquared),
  );
  return distanceBetweenPoints(point, {
    x: start.x + t * dx,
    y: start.y + t * dy,
  });
}

export function rotatePoint(
  point: WhiteboardPoint,
  center: WhiteboardPoint,
  degrees: number,
): WhiteboardPoint {
  if (degrees === 0) return point;

  const radians = (degrees * Math.PI) / 180;
  const cos = Math.cos(radians);
  const sin = Math.sin(radians);
  const dx = point.x - center.x;
  const dy = point.y - center.y;

  return {
    x: center.x + dx * cos - dy * sin,
    y: center.y + dx * sin + dy * cos,
  };
}

export function getBoxCenter(element: WhiteboardBoxElement): WhiteboardPoint {
  return {
    x: element.x + element.width / 2,
    y: element.y + element.height / 2,
  };
}

export function toBoxLocalPoint(
  element: WhiteboardBoxElement,
  point: WhiteboardPoint,
): WhiteboardPoint {
  return rotatePoint(point, getBoxCenter(element), -element.rotation);
}

export function getElementBounds(element: WhiteboardElement): WhiteboardRect {
  const strokePadding = Math.max(0, element.style.strokeWidth / 2);

  if (isBoxElement(element)) {
    return inflateRect(rectFromPoints(getBoxCorners(element)), strokePadding);
  }

  const arrowPadding = element.type === "arrow" ? Math.max(8, element.style.strokeWidth) : 0;
  return inflateRect(rectFromPoints(element.points), strokePadding + arrowPadding);
}

export function getElementSegments(
  element:
    | WhiteboardPathElement
    | Extract<WhiteboardElement, { type: "freehand" | "line" | "arrow" }>,
): readonly (readonly [WhiteboardPoint, WhiteboardPoint])[] {
  const segments: [WhiteboardPoint, WhiteboardPoint][] = [];
  const points = element.points;

  for (let index = 1; index < points.length; index += 1) {
    const start = points[index - 1];
    const end = points[index];
    if (start && end) segments.push([start, end]);
  }

  if (element.type === "path" && element.closed && points.length > 2) {
    const first = points[0];
    const last = points[points.length - 1];
    if (first && last) segments.push([last, first]);
  }

  return segments;
}

export function getSceneBounds(elements: readonly WhiteboardElement[]): WhiteboardRect {
  return unionRects(elements.filter((element) => !element.hidden).map(getElementBounds));
}

export function getVisibleSceneBounds(elements: readonly WhiteboardElement[]): WhiteboardRect | null {
  const visibleBounds = elements.filter((element) => !element.hidden).map(getElementBounds);
  return visibleBounds.length > 0 ? unionRects(visibleBounds) : null;
}

export function isPointInBoxElement(
  element: WhiteboardBoxElement,
  point: WhiteboardPoint,
  tolerance = 0,
): boolean {
  const localPoint = toBoxLocalPoint(element, point);

  return containsPoint(
    inflateRect(
      {
        x: element.x,
        y: element.y,
        width: element.width,
        height: element.height,
      },
      tolerance,
    ),
    localPoint,
  );
}

export function isPointInEllipseElement(
  element: Extract<WhiteboardBoxElement, { type: "ellipse" }>,
  point: WhiteboardPoint,
  tolerance = 0,
): boolean {
  const localPoint = toBoxLocalPoint(element, point);
  const radiusX = Math.max(0, element.width / 2 + tolerance);
  const radiusY = Math.max(0, element.height / 2 + tolerance);

  if (radiusX === 0 || radiusY === 0) return false;

  const center = getBoxCenter(element);
  const normalized =
    ((localPoint.x - center.x) * (localPoint.x - center.x)) / (radiusX * radiusX) +
    ((localPoint.y - center.y) * (localPoint.y - center.y)) / (radiusY * radiusY);

  return normalized <= 1;
}

export function getDiamondPoints(
  element: Extract<WhiteboardBoxElement, { type: "diamond" }>,
): readonly WhiteboardPoint[] {
  const center = getBoxCenter(element);
  const points = [
    { x: center.x, y: element.y },
    { x: element.x + element.width, y: center.y },
    { x: center.x, y: element.y + element.height },
    { x: element.x, y: center.y },
  ];

  return points.map((point) => rotatePoint(point, center, element.rotation));
}

export function isPointInPolygon(
  point: WhiteboardPoint,
  polygon: readonly WhiteboardPoint[],
): boolean {
  let inside = false;

  for (let index = 0, previous = polygon.length - 1; index < polygon.length; previous = index++) {
    const currentPoint = polygon[index];
    const previousPoint = polygon[previous];
    if (!currentPoint || !previousPoint) continue;

    const crosses =
      currentPoint.y > point.y !== previousPoint.y > point.y &&
      point.x <
        ((previousPoint.x - currentPoint.x) * (point.y - currentPoint.y)) /
          (previousPoint.y - currentPoint.y) +
          currentPoint.x;

    if (crosses) inside = !inside;
  }

  return inside;
}

export function translateElement(
  element: WhiteboardElement,
  delta: WhiteboardPoint,
  updatedAt: string,
): WhiteboardElement {
  if (delta.x === 0 && delta.y === 0) return element;

  if (isBoxElement(element)) {
    return {
      ...element,
      x: element.x + delta.x,
      y: element.y + delta.y,
      updatedAt,
    };
  }

  switch (element.type) {
    case "line":
    case "arrow":
      return {
        ...element,
        points: translateConnectorPoints(element.points, delta),
        updatedAt,
      };
    case "path":
      return {
        ...element,
        points: translatePoints(element.points, delta),
        updatedAt,
      };
    case "freehand":
      return {
        ...element,
        points: translatePoints(element.points, delta),
        updatedAt,
      };
  }
}

function translatePoints(
  points: readonly WhiteboardPoint[],
  delta: WhiteboardPoint,
): readonly WhiteboardPoint[] {
  return points.map((point) => ({
    x: point.x + delta.x,
    y: point.y + delta.y,
  }));
}

function translateConnectorPoints(
  points: readonly [WhiteboardPoint, WhiteboardPoint, ...WhiteboardPoint[]],
  delta: WhiteboardPoint,
): readonly [WhiteboardPoint, WhiteboardPoint, ...WhiteboardPoint[]] {
  const [first, second, ...rest] = points;
  return [
    translatePoint(first, delta),
    translatePoint(second, delta),
    ...rest.map((point) => translatePoint(point, delta)),
  ];
}

function translatePoint(point: WhiteboardPoint, delta: WhiteboardPoint): WhiteboardPoint {
  return {
    x: point.x + delta.x,
    y: point.y + delta.y,
  };
}

function getBoxCorners(element: WhiteboardBoxElement): readonly WhiteboardPoint[] {
  const center = getBoxCenter(element);
  const corners = [
    { x: element.x, y: element.y },
    { x: element.x + element.width, y: element.y },
    { x: element.x + element.width, y: element.y + element.height },
    { x: element.x, y: element.y + element.height },
  ];

  return corners.map((corner) => rotatePoint(corner, center, element.rotation));
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
