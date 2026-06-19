import {
  DEFAULT_WHITEBOARD_STYLE,
  type WhiteboardConnectorEndpoint,
  type WhiteboardElement,
  type WhiteboardElementId,
  type WhiteboardPoint,
  type WhiteboardTool,
} from "../render/model";
import { normalizeBounds } from "../render/geometry";
import type { WhiteboardDrawDrag } from "./types";

export function draftFromDrag(drag: WhiteboardDrawDrag): WhiteboardElement {
  if (drag.tool === "line" || drag.tool === "arrow") {
    return connectorElement(makeDraftId("line"), drag.tool, [drag.startWorld, drag.currentWorld]);
  }

  const bounds = normalizeBounds(drag.startWorld, drag.currentWorld);
  return boxElement(makeDraftId("shape"), drag.tool, bounds);
}

export function elementFromDrawDrag(
  id: WhiteboardElementId,
  drag: WhiteboardDrawDrag,
): WhiteboardElement | null {
  if (drag.tool === "line" || drag.tool === "arrow") {
    const points = [drag.startWorld, drag.currentWorld] as const;
    if (Math.hypot(points[1].x - points[0].x, points[1].y - points[0].y) < 2) {
      return null;
    }

    return connectorElement(id, drag.tool, points, {
      startBinding: drag.startBinding,
      endBinding: drag.endBinding,
    });
  }

  const bounds = normalizeBounds(drag.startWorld, drag.currentWorld);
  return bounds.width < 2 || bounds.height < 2 ? null : boxElement(id, drag.tool, bounds);
}

export function freehandElement(
  id: WhiteboardElementId,
  points: readonly WhiteboardPoint[],
): WhiteboardElement {
  const timestamp = new Date().toISOString();
  return {
    id,
    type: "freehand",
    points: collapseDuplicatePoints(points),
    locked: false,
    hidden: false,
    style: DEFAULT_WHITEBOARD_STYLE,
    createdAt: timestamp,
    updatedAt: timestamp,
  };
}

export function textElement(
  id: WhiteboardElementId,
  point: WhiteboardPoint,
): WhiteboardElement {
  const timestamp = new Date().toISOString();
  return {
    id,
    type: "text",
    x: point.x,
    y: point.y,
    width: 180,
    height: 48,
    rotation: 0,
    text: "",
    textAlign: "left",
    verticalAlign: "top",
    locked: false,
    hidden: false,
    style: DEFAULT_WHITEBOARD_STYLE,
    createdAt: timestamp,
    updatedAt: timestamp,
  };
}

function boxElement(
  id: WhiteboardElementId,
  type: Extract<WhiteboardTool, "rectangle" | "ellipse" | "diamond">,
  bounds: { readonly x: number; readonly y: number; readonly width: number; readonly height: number },
): WhiteboardElement {
  const timestamp = new Date().toISOString();
  return {
    id,
    type,
    x: bounds.x,
    y: bounds.y,
    width: bounds.width,
    height: bounds.height,
    rotation: 0,
    locked: false,
    hidden: false,
    style: DEFAULT_WHITEBOARD_STYLE,
    createdAt: timestamp,
    updatedAt: timestamp,
  };
}

function connectorElement(
  id: WhiteboardElementId,
  type: "line" | "arrow",
  points: readonly [WhiteboardPoint, WhiteboardPoint, ...WhiteboardPoint[]],
  bindings: {
    readonly startBinding?: WhiteboardConnectorEndpoint;
    readonly endBinding?: WhiteboardConnectorEndpoint;
  } = {},
): WhiteboardElement {
  const timestamp = new Date().toISOString();
  return {
    id,
    type,
    points,
    locked: false,
    hidden: false,
    style: DEFAULT_WHITEBOARD_STYLE,
    createdAt: timestamp,
    updatedAt: timestamp,
    startBinding: bindings.startBinding,
    endBinding: bindings.endBinding,
    startArrow: "none",
    endArrow: type === "arrow" ? "triangle" : "none",
  };
}

function collapseDuplicatePoints(points: readonly WhiteboardPoint[]): readonly WhiteboardPoint[] {
  const cleaned: WhiteboardPoint[] = [];

  for (const point of points) {
    const previous = cleaned[cleaned.length - 1];
    if (!previous || previous.x !== point.x || previous.y !== point.y) {
      cleaned.push(point);
    }
  }

  return cleaned;
}

function makeDraftId(kind: string): WhiteboardElementId {
  return `__draft_${kind}__` as WhiteboardElementId;
}
