import type {
  WhiteboardDocument,
  WhiteboardElementPatch,
  WhiteboardMetadata,
  WhiteboardPoint,
  WhiteboardRect,
} from "./model";

export type WhiteboardGridSettings = {
  readonly gridVisible: boolean;
  readonly snapToGrid: boolean;
  readonly gridSize: number;
};

export type WhiteboardGridSettingsPatch = Partial<WhiteboardGridSettings>;

export const DEFAULT_WHITEBOARD_GRID_SIZE = 24;

export function getGridSettings(document: WhiteboardDocument): WhiteboardGridSettings {
  return {
    gridVisible: Boolean(document.metadata?.gridVisible),
    snapToGrid: Boolean(document.metadata?.snapToGrid),
    gridSize: normalizeGridSize(document.metadata?.gridSize),
  };
}

export function gridMetadataFromSettings(settings: WhiteboardGridSettingsPatch): WhiteboardMetadata {
  return {
    ...(settings.gridVisible === undefined ? {} : { gridVisible: settings.gridVisible }),
    ...(settings.snapToGrid === undefined ? {} : { snapToGrid: settings.snapToGrid }),
    ...(settings.gridSize === undefined ? {} : { gridSize: normalizeGridSize(settings.gridSize) }),
  };
}

export function snapCoordinate(value: number, settings: WhiteboardGridSettings): number {
  if (!settings.snapToGrid) return value;
  return roundToPrecision(Math.round(value / settings.gridSize) * settings.gridSize);
}

export function snapPoint(point: WhiteboardPoint, settings: WhiteboardGridSettings): WhiteboardPoint {
  return {
    x: snapCoordinate(point.x, settings),
    y: snapCoordinate(point.y, settings),
  };
}

export function snapRect(rect: WhiteboardRect, settings: WhiteboardGridSettings): WhiteboardRect {
  return {
    x: snapCoordinate(rect.x, settings),
    y: snapCoordinate(rect.y, settings),
    width: Math.max(settings.gridSize, snapCoordinate(rect.width, settings)),
    height: Math.max(settings.gridSize, snapCoordinate(rect.height, settings)),
  };
}

export function snapElementPatch(
  patch: WhiteboardElementPatch,
  settings: WhiteboardGridSettings,
): WhiteboardElementPatch {
  if (!settings.snapToGrid) return patch;

  return {
    ...patch,
    ...(patch.x === undefined ? {} : { x: snapCoordinate(patch.x, settings) }),
    ...(patch.y === undefined ? {} : { y: snapCoordinate(patch.y, settings) }),
    ...(patch.width === undefined ? {} : { width: Math.max(settings.gridSize, snapCoordinate(patch.width, settings)) }),
    ...(patch.height === undefined
      ? {}
      : { height: Math.max(settings.gridSize, snapCoordinate(patch.height, settings)) }),
    ...(patch.points === undefined
      ? {}
      : { points: patch.points.map((point) => snapPoint(point, settings)) }),
  };
}

function normalizeGridSize(value: unknown): number {
  return typeof value === "number" && Number.isFinite(value) && value >= 4
    ? Math.round(value)
    : DEFAULT_WHITEBOARD_GRID_SIZE;
}

function roundToPrecision(value: number): number {
  return Number(value.toFixed(12));
}
