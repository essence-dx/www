import { getElementBounds, translateElement } from "./geometry";
import { remapFrameMetadata } from "./frames";
import {
  makeElementId,
  makeGroupId,
  type WhiteboardDocument,
  type WhiteboardElement,
  type WhiteboardElementId,
  type WhiteboardGroup,
  type WhiteboardGroupId,
  type WhiteboardPoint,
  type WhiteboardRect,
} from "./model";

export type WhiteboardAlignment =
  | "left"
  | "center"
  | "right"
  | "top"
  | "middle"
  | "bottom";

export function duplicateElement(
  element: WhiteboardElement,
  id: WhiteboardElementId,
  offset: WhiteboardPoint,
  now: string,
  copiedIdBySource: ReadonlyMap<WhiteboardElementId, WhiteboardElementId>,
): WhiteboardElement {
  const copy = {
    ...translateElement(element, offset, now),
    id,
    groupId: undefined,
    name: element.name ? `${element.name} copy` : undefined,
    locked: false,
    hidden: false,
    createdAt: now,
    updatedAt: now,
  };

  if (copy.type === "line" || copy.type === "arrow") {
    return remapFrameMetadata({
      ...copy,
      startBinding: remapConnectorBinding(copy.startBinding, copiedIdBySource),
      endBinding: remapConnectorBinding(copy.endBinding, copiedIdBySource),
    }, copiedIdBySource);
  }

  return remapFrameMetadata(copy, copiedIdBySource);
}

export function existingGroups(document: WhiteboardDocument): readonly WhiteboardGroup[] {
  return document.groups ?? [];
}

export function nextGroupId(document: WhiteboardDocument): WhiteboardGroupId {
  const existingIds = new Set(existingGroups(document).map((group) => group.id));
  let index = existingIds.size + 1;
  let id = makeGroupId(`group-${index}`);

  while (existingIds.has(id)) {
    index += 1;
    id = makeGroupId(`group-${index}`);
  }

  return id;
}

export function patchElements(
  document: WhiteboardDocument,
  ids: readonly WhiteboardElementId[],
  patch: (element: WhiteboardElement) => WhiteboardElement,
  now: string,
): WhiteboardDocument {
  const targetIds = new Set(ids);
  if (targetIds.size === 0) return document;

  let changed = false;
  const elements = document.elements.map((element) => {
    if (!targetIds.has(element.id)) return element;
    changed = true;
    return patch(element);
  });

  return changed ? { ...document, elements, updatedAt: now } : document;
}

export function selectedEditableElements(
  document: WhiteboardDocument,
  ids: readonly WhiteboardElementId[],
): readonly WhiteboardElement[] {
  const selectedIds = new Set(ids);
  return document.elements.filter((element) => selectedIds.has(element.id) && !element.locked && !element.hidden);
}

export function targetPointForAlignment(
  sceneBounds: WhiteboardRect,
  elementBounds: WhiteboardRect,
  alignment: WhiteboardAlignment,
): WhiteboardPoint {
  switch (alignment) {
    case "left":
      return { x: sceneBounds.x, y: elementBounds.y };
    case "center":
      return { x: sceneBounds.x + sceneBounds.width / 2 - elementBounds.width / 2, y: elementBounds.y };
    case "right":
      return { x: sceneBounds.x + sceneBounds.width - elementBounds.width, y: elementBounds.y };
    case "top":
      return { x: elementBounds.x, y: sceneBounds.y };
    case "middle":
      return { x: elementBounds.x, y: sceneBounds.y + sceneBounds.height / 2 - elementBounds.height / 2 };
    case "bottom":
      return { x: elementBounds.x, y: sceneBounds.y + sceneBounds.height - elementBounds.height };
  }
}

export function unionElementBounds(elements: readonly WhiteboardElement[]): WhiteboardRect {
  const bounds = elements.map(getElementBounds);
  const minX = Math.min(...bounds.map((bound) => bound.x));
  const minY = Math.min(...bounds.map((bound) => bound.y));
  const maxX = Math.max(...bounds.map((bound) => bound.x + bound.width));
  const maxY = Math.max(...bounds.map((bound) => bound.y + bound.height));

  return {
    x: minX,
    y: minY,
    width: maxX - minX,
    height: maxY - minY,
  };
}

export function makeCopyId(sourceId: WhiteboardElementId, existingIds: Set<WhiteboardElementId>): WhiteboardElementId {
  let index = 1;
  let id = makeElementId(`${sourceId}-copy-${index}`);

  while (existingIds.has(id)) {
    index += 1;
    id = makeElementId(`${sourceId}-copy-${index}`);
  }

  existingIds.add(id);
  return id;
}

function remapConnectorBinding(
  binding: Extract<WhiteboardElement, { type: "line" | "arrow" }>["startBinding"],
  copiedIdBySource: ReadonlyMap<WhiteboardElementId, WhiteboardElementId>,
) {
  if (!binding) return undefined;
  const elementId = copiedIdBySource.get(binding.elementId);
  return elementId ? { ...binding, elementId } : undefined;
}
