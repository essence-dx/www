import { getElementBounds, getVisibleSceneBounds, rectsIntersect, translateElement, unionRects } from "./geometry";
import { expandElementIdsForGroups } from "./groups";
import { clearMissingFrameReferences, expandElementIdsForFrames } from "./frames";
import { rerouteBoundConnectors } from "./connectors";
import { normalizedImagePatch } from "./scene-image-patches";
import {
  DEFAULT_WHITEBOARD_VIEWPORT,
  makeElementId,
  type WhiteboardDocument,
  type WhiteboardElement,
  type WhiteboardElementId,
  type WhiteboardElementPatch,
  type WhiteboardPoint,
  type WhiteboardRect,
  type WhiteboardStyle,
  type WhiteboardTool,
  type WhiteboardViewport,
} from "./model";
import type { InsertElementsOptions, SceneChangeOptions, SelectElementsOptions } from "./scene-types";

export {
  createDiamondElement,
  createEllipseElement,
  createImageElement,
  createRectangleElement,
  createTextElement,
} from "./element-factory";
export type {
  CreateBoxElementOptions,
  CreateElementBaseOptions,
  CreateImageElementOptions,
  CreateTextElementOptions,
} from "./element-factory";
export type { InsertElementsOptions, SceneChangeOptions, SelectElementsOptions } from "./scene-types";

export type WhiteboardReorderIntent = "front" | "back" | "forward" | "backward";

export function insertElements(
  document: WhiteboardDocument,
  elements: readonly WhiteboardElement[],
  options: InsertElementsOptions = {},
): WhiteboardDocument {
  if (elements.length === 0) return document;

  const updatedAt = options.now ?? new Date().toISOString();
  const insertedIds = new Set(elements.map((element) => element.id));
  const keptElements = document.elements.filter((element) => !insertedIds.has(element.id));
  const index = clampInteger(options.index ?? keptElements.length, 0, keptElements.length);
  const nextElements = [
    ...keptElements.slice(0, index),
    ...elements,
    ...keptElements.slice(index),
  ];

  return rerouteBoundConnectors(
    touchDocument(
      document,
      {
        elements: nextElements,
        selection: options.select
          ? elements.map((element) => element.id)
          : filterExistingIds(document.selection, nextElements),
      },
      updatedAt,
    ),
    elements.map((element) => element.id),
    updatedAt,
  );
}

export function updateElement(
  document: WhiteboardDocument,
  id: string | WhiteboardElementId,
  patch: WhiteboardElementPatch,
  options: SceneChangeOptions = {},
): WhiteboardDocument {
  const elementId = makeElementId(id);
  let changed = false;
  const updatedAt = options.now ?? new Date().toISOString();
  const nextElements = document.elements.map((element) => {
    if (element.id !== elementId) return element;
    if (element.locked && patch.locked !== false) return element;
    changed = true;
    return patchElement(element, patch, updatedAt);
  });

  return changed
    ? rerouteBoundConnectors(touchDocument(document, { elements: nextElements }, updatedAt), [elementId], updatedAt)
    : document;
}

export function removeElements(
  document: WhiteboardDocument,
  ids: readonly (string | WhiteboardElementId)[],
  options: SceneChangeOptions = {},
): WhiteboardDocument {
  const removeIds = new Set(expandElementIdsForGroups(document, ids));
  if (removeIds.size === 0) return document;

  const keptElements = document.elements.filter((element) => !removeIds.has(element.id) || element.locked);
  if (keptElements.length === document.elements.length) return document;

  const removedIds = new Set(
    document.elements
      .filter((element) => removeIds.has(element.id) && !element.locked)
      .map((element) => element.id),
  );

  const updatedAt = options.now ?? new Date().toISOString();
  const nextElements = clearMissingFrameReferences(keptElements, updatedAt);
  const nextGroups = document.groups
    ?.map((group) => ({
      ...group,
      elementIds: group.elementIds.filter((id) => !removedIds.has(id)),
      updatedAt,
    }))
    .filter((group) => group.elementIds.length > 1);

  return rerouteBoundConnectors(
    touchDocument(
      document,
      {
        elements: nextElements,
        groups: nextGroups && nextGroups.length > 0 ? nextGroups : undefined,
        selection: document.selection.filter((id) => !removedIds.has(id)),
      },
      updatedAt,
    ),
    [...removedIds],
    updatedAt,
  );
}

export function translateElements(
  document: WhiteboardDocument,
  ids: readonly (string | WhiteboardElementId)[],
  delta: WhiteboardPoint,
  options: SceneChangeOptions = {},
): WhiteboardDocument {
  if (delta.x === 0 && delta.y === 0) return document;

  const moveIds = new Set(expandElementIdsForFrames(document, expandElementIdsForGroups(document, ids)));
  if (moveIds.size === 0) return document;

  const updatedAt = options.now ?? new Date().toISOString();
  let changed = false;
  const movedIds: WhiteboardElementId[] = [];
  const nextElements = document.elements.map((element) => {
    if (!moveIds.has(element.id) || element.locked || element.hidden) return element;
    changed = true;
    movedIds.push(element.id);
    return translateElement(element, delta, updatedAt);
  });

  return changed
    ? rerouteBoundConnectors(touchDocument(document, { elements: nextElements }, updatedAt), movedIds, updatedAt)
    : document;
}

export function selectElements(
  document: WhiteboardDocument,
  ids: readonly (string | WhiteboardElementId)[],
  options: SelectElementsOptions = {},
): WhiteboardDocument {
  const requestedIds = expandElementIdsForGroups(document, uniqueIds(ids.map((id) => makeElementId(id))));
  const existingIds = filterExistingIds(requestedIds, document.elements);
  const mode = options.mode ?? "replace";
  const selected =
    mode === "extend"
      ? uniqueIds([...document.selection, ...existingIds])
      : mode === "toggle"
        ? toggleSelection(document.selection, existingIds)
        : existingIds;

  if (sameIds(selected, document.selection)) return document;
  return touchDocument(document, { selection: selected }, options.now);
}

export function clearSelection(
  document: WhiteboardDocument,
  options: SceneChangeOptions = {},
): WhiteboardDocument {
  if (document.selection.length === 0) return document;
  return touchDocument(document, { selection: [] }, options.now);
}

export function setActiveTool(
  document: WhiteboardDocument,
  activeTool: WhiteboardTool,
  options: SceneChangeOptions = {},
): WhiteboardDocument {
  if (document.activeTool === activeTool) return document;
  return touchDocument(document, { activeTool }, options.now);
}

export function setViewport(
  document: WhiteboardDocument,
  viewport: Partial<WhiteboardViewport>,
  options: SceneChangeOptions = {},
): WhiteboardDocument {
  const nextViewport = {
    ...DEFAULT_WHITEBOARD_VIEWPORT,
    ...document.viewport,
    ...viewport,
    zoom: clampZoom(viewport.zoom ?? document.viewport.zoom),
  };

  if (
    nextViewport.x === document.viewport.x &&
    nextViewport.y === document.viewport.y &&
    nextViewport.zoom === document.viewport.zoom
  ) {
    return document;
  }

  return touchDocument(document, { viewport: nextViewport }, options.now);
}

export function setCurrentStyle(
  document: WhiteboardDocument,
  style: Partial<WhiteboardStyle>,
  options: SceneChangeOptions = {},
): WhiteboardDocument {
  const nextStyle = { ...document.currentStyle, ...style };
  if (JSON.stringify(nextStyle) === JSON.stringify(document.currentStyle)) return document;
  return touchDocument(document, { currentStyle: nextStyle }, options.now);
}

export function renameDocument(
  document: WhiteboardDocument,
  name: string,
  options: SceneChangeOptions = {},
): WhiteboardDocument {
  const nextName = name.trim();
  if (!nextName || nextName === document.name) return document;
  return touchDocument(document, { name: nextName }, options.now);
}

export function reorderElements(
  document: WhiteboardDocument,
  ids: readonly (string | WhiteboardElementId)[],
  intent: WhiteboardReorderIntent,
  options: SceneChangeOptions = {},
): WhiteboardDocument {
  const targetIds = new Set(ids.map((id) => makeElementId(id)));
  if (targetIds.size === 0) return document;

  const moving = document.elements.filter((element) => targetIds.has(element.id));
  if (moving.length === 0) return document;

  const nextElements = reorder(document.elements, targetIds, intent);
  return arraysEqual(nextElements, document.elements)
    ? document
    : touchDocument(document, { elements: nextElements }, options.now);
}

export function getSelectedElements(document: WhiteboardDocument): readonly WhiteboardElement[] {
  const selectedIds = new Set(document.selection);
  return document.elements.filter((element) => selectedIds.has(element.id));
}

export function getAreaSelectionIds(
  document: WhiteboardDocument,
  bounds: WhiteboardRect,
): readonly WhiteboardElementId[] {
  const intersectingIds = document.elements
    .filter((element) => !element.hidden && rectsIntersect(getElementBounds(element), bounds))
    .map((element) => element.id);

  return expandElementIdsForFrames(document, expandElementIdsForGroups(document, intersectingIds));
}

export function getDocumentSelectionBounds(
  document: WhiteboardDocument,
  ids: readonly WhiteboardElementId[] = document.selection,
): WhiteboardRect | null {
  const selectedIds = new Set(ids);
  const bounds = document.elements
    .filter((element) => selectedIds.has(element.id) && !element.hidden)
    .map(getElementBounds);

  return bounds.length > 0 ? unionRects(bounds) : null;
}

export function getDocumentContentBounds(document: WhiteboardDocument): WhiteboardRect | null {
  return getVisibleSceneBounds(document.elements);
}

function patchElement(
  element: WhiteboardElement,
  patch: WhiteboardElementPatch,
  updatedAt: string,
): WhiteboardElement {
  const { style, src, alt, naturalWidth, naturalHeight, ...rest } = patch;
  const imagePatch =
    element.type === "image"
      ? normalizedImagePatch({ src, alt, naturalWidth, naturalHeight })
      : {};

  return {
    ...element,
    ...rest,
    ...imagePatch,
    style: style ? { ...element.style, ...style } : element.style,
    updatedAt,
  } as WhiteboardElement;
}

function touchDocument(
  document: WhiteboardDocument,
  changes: Partial<WhiteboardDocument>,
  now = new Date().toISOString(),
): WhiteboardDocument {
  return {
    ...document,
    ...changes,
    updatedAt: now,
  };
}

function reorder(
  elements: readonly WhiteboardElement[],
  ids: ReadonlySet<WhiteboardElementId>,
  intent: WhiteboardReorderIntent,
): readonly WhiteboardElement[] {
  if (intent === "front") {
    return [
      ...elements.filter((element) => !ids.has(element.id)),
      ...elements.filter((element) => ids.has(element.id)),
    ];
  }

  if (intent === "back") {
    return [
      ...elements.filter((element) => ids.has(element.id)),
      ...elements.filter((element) => !ids.has(element.id)),
    ];
  }

  const next = [...elements];
  const start = intent === "forward" ? next.length - 2 : 1;
  const end = intent === "forward" ? -1 : next.length;
  const step = intent === "forward" ? -1 : 1;

  for (let index = start; index !== end; index += step) {
    const neighborIndex = intent === "forward" ? index + 1 : index - 1;
    const element = next[index];
    const neighbor = next[neighborIndex];
    if (!element || !neighbor || !ids.has(element.id) || ids.has(neighbor.id)) continue;
    next[index] = neighbor;
    next[neighborIndex] = element;
  }

  return next;
}

function filterExistingIds(
  ids: readonly WhiteboardElementId[],
  elements: readonly WhiteboardElement[],
): readonly WhiteboardElementId[] {
  const existingIds = new Set(elements.map((element) => element.id));
  return ids.filter((id) => existingIds.has(id));
}

function toggleSelection(
  current: readonly WhiteboardElementId[],
  requested: readonly WhiteboardElementId[],
): readonly WhiteboardElementId[] {
  const next = new Set(current);

  for (const id of requested) {
    if (next.has(id)) {
      next.delete(id);
    } else {
      next.add(id);
    }
  }

  return [...next];
}

function uniqueIds(ids: readonly WhiteboardElementId[]): readonly WhiteboardElementId[] {
  return [...new Set(ids)];
}

function sameIds(left: readonly WhiteboardElementId[], right: readonly WhiteboardElementId[]) {
  return left.length === right.length && left.every((id, index) => id === right[index]);
}

function arraysEqual<T>(left: readonly T[], right: readonly T[]): boolean {
  return left.length === right.length && left.every((item, index) => item === right[index]);
}

function clampZoom(zoom: number): number {
  return Math.max(0.05, Math.min(8, zoom));
}

function clampInteger(value: number, min: number, max: number): number {
  return Math.max(min, Math.min(max, Math.trunc(value)));
}
