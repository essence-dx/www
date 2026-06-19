import {
  getElementBounds,
  getSceneBounds,
  inflateRect,
  containsPoint,
} from "../../lib/whiteboard/geometry";
import type {
  WhiteboardElement,
  WhiteboardPoint,
  WhiteboardRect,
} from "../../lib/whiteboard/model";

export type WhiteboardBounds = WhiteboardRect;
export type WhiteboardHitPoint = WhiteboardPoint;

function round(value: number): number {
  return Number(value.toFixed(6));
}

function roundedBounds(bounds: WhiteboardRect): WhiteboardBounds {
  return {
    x: round(bounds.x),
    y: round(bounds.y),
    width: round(bounds.width),
    height: round(bounds.height),
  };
}

export function computeElementBounds(element: WhiteboardElement): WhiteboardBounds {
  return roundedBounds(getElementBounds(element));
}

export function computeDocumentBounds(
  elements: readonly WhiteboardElement[],
  options: { padding?: number } = {},
): WhiteboardBounds {
  return roundedBounds(inflateRect(getSceneBounds(elements), options.padding ?? 0));
}

export function boundsContainPoint(
  bounds: WhiteboardBounds,
  point: WhiteboardHitPoint,
): boolean {
  return containsPoint(bounds, point);
}

export function hitTestElements(
  elements: readonly WhiteboardElement[],
  point: WhiteboardHitPoint,
): WhiteboardElement | null {
  for (let index = elements.length - 1; index >= 0; index -= 1) {
    const element = elements[index];

    if (element && !element.locked && !element.hidden && containsPoint(getElementBounds(element), point)) {
      return element;
    }
  }

  return null;
}

