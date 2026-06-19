import { getElementBounds, getVisibleSceneBounds, unionRects } from "./geometry";
import { roundCoordinate, screenToWorld } from "./render/geometry";
import type {
  WhiteboardDocument,
  WhiteboardElementId,
  WhiteboardPoint,
  WhiteboardRect,
  WhiteboardSize,
  WhiteboardViewport,
} from "./model";

export const WHITEBOARD_MINIMAP_DEFAULT_SIZE: WhiteboardSize = {
  width: 220,
  height: 140,
};

export const WHITEBOARD_MINIMAP_DEFAULT_STAGE_SIZE: WhiteboardSize = {
  width: 1200,
  height: 720,
};

export type WhiteboardMinimapOptions = {
  readonly size?: Partial<WhiteboardSize>;
  readonly stageSize?: Partial<WhiteboardSize>;
  readonly padding?: number;
};

export type WhiteboardMinimapElement = {
  readonly id: WhiteboardElementId;
  readonly type: string;
  readonly role?: string;
  readonly selected: boolean;
  readonly bounds: WhiteboardRect;
  readonly rect: WhiteboardRect;
};

export type WhiteboardMinimapModel = {
  readonly empty: boolean;
  readonly size: WhiteboardSize;
  readonly stageSize: WhiteboardSize;
  readonly padding: number;
  readonly contentBounds: WhiteboardRect | null;
  readonly documentBounds: WhiteboardRect;
  readonly viewportWorldBounds: WhiteboardRect;
  readonly viewportRect: WhiteboardRect | null;
  readonly origin: WhiteboardPoint;
  readonly scale: number;
  readonly elements: readonly WhiteboardMinimapElement[];
};

export function createWhiteboardMinimapModel(
  document: WhiteboardDocument,
  options: WhiteboardMinimapOptions = {},
): WhiteboardMinimapModel {
  const size = normalizeSize(options.size, WHITEBOARD_MINIMAP_DEFAULT_SIZE);
  const stageSize = normalizeSize(options.stageSize, WHITEBOARD_MINIMAP_DEFAULT_STAGE_SIZE);
  const padding = Math.max(0, options.padding ?? 10);
  const contentBounds = getVisibleSceneBounds(document.elements);
  const viewportWorldBounds = viewportWorldRect(document.viewport, stageSize);
  const documentBounds = contentBounds
    ? normalizeNonZeroRect(contentBounds)
    : normalizeNonZeroRect(viewportWorldBounds);
  const fit = fitDocumentToMinimap(documentBounds, size, padding);
  const selectedIds = new Set(document.selection);

  return {
    empty: !contentBounds,
    size,
    stageSize,
    padding,
    contentBounds,
    documentBounds,
    viewportWorldBounds,
    viewportRect: contentBounds
      ? clampRectPositionToSurface(mapWorldRect(viewportWorldBounds, documentBounds, fit), size)
      : null,
    origin: fit.origin,
    scale: fit.scale,
    elements: document.elements
      .filter((element) => !element.hidden)
      .map((element) => {
        const bounds = getElementBounds(element);
        return {
          id: element.id,
          type: element.type,
          role: element.role,
          selected: selectedIds.has(element.id),
          bounds,
          rect: mapWorldRect(bounds, documentBounds, fit),
        };
      }),
  };
}

export function minimapPointToViewport(
  model: WhiteboardMinimapModel,
  point: WhiteboardPoint,
): Partial<WhiteboardViewport> {
  const world = minimapPointToWorld(model, point);

  return {
    x: roundCoordinate(model.stageSize.width / 2 - world.x * currentZoom(model)),
    y: roundCoordinate(model.stageSize.height / 2 - world.y * currentZoom(model)),
    zoom: currentZoom(model),
  };
}

export function minimapPointToWorld(
  model: WhiteboardMinimapModel,
  point: WhiteboardPoint,
): WhiteboardPoint {
  const clamped = {
    x: clamp(point.x, 0, model.size.width),
    y: clamp(point.y, 0, model.size.height),
  };

  return {
    x: roundMinimapCoordinate((clamped.x - model.origin.x) / model.scale + model.documentBounds.x),
    y: roundMinimapCoordinate((clamped.y - model.origin.y) / model.scale + model.documentBounds.y),
  };
}

function viewportWorldRect(
  viewport: WhiteboardViewport,
  stageSize: WhiteboardSize,
): WhiteboardRect {
  const topLeft = screenToWorld({ x: 0, y: 0 }, viewport);
  const bottomRight = screenToWorld({ x: stageSize.width, y: stageSize.height }, viewport);

  return normalizeNonZeroRect({
    x: topLeft.x,
    y: topLeft.y,
    width: bottomRight.x - topLeft.x,
    height: bottomRight.y - topLeft.y,
  });
}

function fitDocumentToMinimap(
  bounds: WhiteboardRect,
  size: WhiteboardSize,
  padding: number,
): { readonly origin: WhiteboardPoint; readonly scale: number } {
  const innerWidth = Math.max(1, size.width - padding * 2);
  const innerHeight = Math.max(1, size.height - padding * 2);
  const scale = Math.min(innerWidth / bounds.width, innerHeight / bounds.height);
  const fittedWidth = bounds.width * scale;
  const fittedHeight = bounds.height * scale;

  return {
    origin: {
      x: roundCoordinate(padding + (innerWidth - fittedWidth) / 2),
      y: roundCoordinate(padding + (innerHeight - fittedHeight) / 2),
    },
    scale: roundCoordinate(scale),
  };
}

function mapWorldRect(
  rect: WhiteboardRect,
  documentBounds: WhiteboardRect,
  fit: { readonly origin: WhiteboardPoint; readonly scale: number },
): WhiteboardRect {
  return {
    x: roundCoordinate(fit.origin.x + (rect.x - documentBounds.x) * fit.scale),
    y: roundCoordinate(fit.origin.y + (rect.y - documentBounds.y) * fit.scale),
    width: roundCoordinate(rect.width * fit.scale),
    height: roundCoordinate(rect.height * fit.scale),
  };
}

function clampRectPositionToSurface(rect: WhiteboardRect, size: WhiteboardSize): WhiteboardRect {
  const width = Math.min(Math.max(0, rect.width), size.width);
  const height = Math.min(Math.max(0, rect.height), size.height);

  return {
    x: roundCoordinate(clamp(rect.x, 0, size.width - width)),
    y: roundCoordinate(clamp(rect.y, 0, size.height - height)),
    width: roundCoordinate(width),
    height: roundCoordinate(height),
  };
}

function normalizeNonZeroRect(rect: WhiteboardRect): WhiteboardRect {
  return {
    x: rect.x,
    y: rect.y,
    width: Math.max(1, rect.width),
    height: Math.max(1, rect.height),
  };
}

function normalizeSize(
  value: Partial<WhiteboardSize> | undefined,
  fallback: WhiteboardSize,
): WhiteboardSize {
  return {
    width: Math.max(1, value?.width ?? fallback.width),
    height: Math.max(1, value?.height ?? fallback.height),
  };
}

function currentZoom(model: WhiteboardMinimapModel): number {
  return model.stageSize.width / model.viewportWorldBounds.width;
}

function clamp(value: number, min: number, max: number): number {
  return Math.max(min, Math.min(max, value));
}

function roundMinimapCoordinate(value: number): number {
  return Number(value.toFixed(6));
}
