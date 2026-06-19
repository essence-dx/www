import {
  makeElementId,
  type WhiteboardDocument,
  type WhiteboardElement,
  type WhiteboardElementId,
  type WhiteboardMetadata,
} from "./model";

export const FRAME_ID_METADATA_KEY = "frameId";

export function isFrameElement(element: WhiteboardElement): boolean {
  return element.role === "frame";
}

export function frameIdForElement(element: WhiteboardElement): WhiteboardElementId | null {
  const frameId = element.metadata?.[FRAME_ID_METADATA_KEY];
  return typeof frameId === "string" && frameId.trim() ? makeElementId(frameId) : null;
}

export function frameChildren(
  document: WhiteboardDocument,
  frameId: string | WhiteboardElementId,
): readonly WhiteboardElement[] {
  const targetFrameId = makeElementId(frameId);
  return document.elements.filter(
    (element) => element.id !== targetFrameId && frameIdForElement(element) === targetFrameId,
  );
}

export function assignFrameToElements(
  document: WhiteboardDocument,
  frameId: string | WhiteboardElementId,
  ids: readonly (string | WhiteboardElementId)[],
  now = new Date().toISOString(),
): WhiteboardDocument {
  const targetFrameId = makeElementId(frameId);
  const frame = document.elements.find((element) => element.id === targetFrameId);
  if (!frame || !isFrameElement(frame)) return document;

  const targetIds = new Set(ids.map((id) => makeElementId(id)));
  if (targetIds.size === 0) return document;

  let changed = false;
  const elements = document.elements.map((element) => {
    if (
      !targetIds.has(element.id) ||
      element.id === targetFrameId ||
      element.locked ||
      isFrameElement(element)
    ) {
      return element;
    }

    if (frameIdForElement(element) === targetFrameId) return element;
    changed = true;
    return {
      ...element,
      metadata: {
        ...(element.metadata ?? {}),
        [FRAME_ID_METADATA_KEY]: targetFrameId,
      },
      updatedAt: now,
    };
  });

  return changed ? { ...document, elements, updatedAt: now } : document;
}

export function clearElementFrames(
  document: WhiteboardDocument,
  ids: readonly (string | WhiteboardElementId)[],
  now = new Date().toISOString(),
): WhiteboardDocument {
  const targetIds = new Set(ids.map((id) => makeElementId(id)));
  if (targetIds.size === 0) return document;

  let changed = false;
  const elements = document.elements.map((element) => {
    if (!targetIds.has(element.id) || element.locked || !frameIdForElement(element)) return element;
    changed = true;
    return {
      ...element,
      metadata: withoutFrameMetadata(element.metadata),
      updatedAt: now,
    };
  });

  return changed ? { ...document, elements, updatedAt: now } : document;
}

export function clearMissingFrameReferences(
  elements: readonly WhiteboardElement[],
  now = new Date().toISOString(),
): readonly WhiteboardElement[] {
  const frameIds = new Set(elements.filter(isFrameElement).map((element) => element.id));
  let changed = false;

  const nextElements = elements.map((element) => {
    const frameId = frameIdForElement(element);
    if (!frameId || frameIds.has(frameId)) return element;

    changed = true;
    return {
      ...element,
      metadata: withoutFrameMetadata(element.metadata),
      updatedAt: now,
    };
  });

  return changed ? nextElements : elements;
}

export function expandElementIdsForFrames(
  document: WhiteboardDocument,
  ids: readonly (string | WhiteboardElementId)[],
): readonly WhiteboardElementId[] {
  const expandedIds = new Set(ids.map((id) => makeElementId(id)));
  const frameIds = new Set(
    document.elements
      .filter((element) => expandedIds.has(element.id) && isFrameElement(element))
      .map((element) => element.id),
  );

  if (frameIds.size === 0) {
    return document.elements.filter((element) => expandedIds.has(element.id)).map((element) => element.id);
  }

  for (const element of document.elements) {
    const frameId = frameIdForElement(element);
    if (frameId && frameIds.has(frameId)) {
      expandedIds.add(element.id);
    }
  }

  return document.elements.filter((element) => expandedIds.has(element.id)).map((element) => element.id);
}

export function remapFrameMetadata(
  element: WhiteboardElement,
  copiedIdBySource: ReadonlyMap<WhiteboardElementId, WhiteboardElementId>,
): WhiteboardElement {
  const frameId = frameIdForElement(element);
  if (!frameId) return element;

  const copiedFrameId = copiedIdBySource.get(frameId);
  if (!copiedFrameId) return element;

  return {
    ...element,
    metadata: {
      ...(element.metadata ?? {}),
      [FRAME_ID_METADATA_KEY]: copiedFrameId,
    },
  };
}

function withoutFrameMetadata(metadata: WhiteboardMetadata | undefined): WhiteboardMetadata | undefined {
  if (!metadata) return undefined;

  const { [FRAME_ID_METADATA_KEY]: _frameId, ...rest } = metadata;
  return Object.keys(rest).length > 0 ? rest : undefined;
}
