import { getSceneBounds } from "./geometry";
import { frameIdForElement, isFrameElement } from "./frames";
import type {
  WhiteboardDocument,
  WhiteboardElement,
  WhiteboardRect,
} from "./model";

export type WhiteboardDocumentSummary = {
  readonly id: string;
  readonly name: string;
  readonly schemaVersion: number;
  readonly elementCount: number;
  readonly visibleElementCount: number;
  readonly hiddenElementCount: number;
  readonly lockedElementCount: number;
  readonly frameCount: number;
  readonly framedElementCount: number;
  readonly stickyNoteCount: number;
  readonly connectorCount: number;
  readonly imageCount: number;
  readonly groupCount: number;
  readonly selectionCount: number;
  readonly revision: number;
  readonly bounds: WhiteboardRect;
  readonly createdAt: string;
  readonly updatedAt: string;
};

export function summarizeWhiteboardDocument(document: WhiteboardDocument): WhiteboardDocumentSummary {
  return {
    id: document.id,
    name: document.name,
    schemaVersion: document.schemaVersion,
    elementCount: document.elements.length,
    visibleElementCount: countElements(document.elements, (element) => !element.hidden),
    hiddenElementCount: countElements(document.elements, (element) => element.hidden),
    lockedElementCount: countElements(document.elements, (element) => element.locked),
    frameCount: countElements(document.elements, isFrameElement),
    framedElementCount: countFramedElements(document),
    stickyNoteCount: countElements(document.elements, (element) => element.role === "sticky-note"),
    connectorCount: countElements(document.elements, (element) => element.type === "line" || element.type === "arrow"),
    imageCount: countElements(document.elements, (element) => element.type === "image"),
    groupCount: document.groups?.length ?? 0,
    selectionCount: document.selection.length,
    revision: revisionFromDocument(document),
    bounds: getSceneBounds(document.elements),
    createdAt: document.createdAt,
    updatedAt: document.updatedAt,
  };
}

export function revisionFromDocument(document: WhiteboardDocument): number {
  const revision = document.metadata?.revision;
  return typeof revision === "number" && Number.isFinite(revision) ? revision : document.elements.length;
}

function countFramedElements(document: WhiteboardDocument): number {
  const frameIds = new Set(
    document.elements.filter(isFrameElement).map((element) => element.id),
  );

  return countElements(document.elements, (element) => {
    const frameId = frameIdForElement(element);
    return Boolean(frameId && frameIds.has(frameId) && !isFrameElement(element));
  });
}

function countElements(
  elements: readonly WhiteboardElement[],
  predicate: (element: WhiteboardElement) => boolean,
): number {
  return elements.reduce((count, element) => count + (predicate(element) ? 1 : 0), 0);
}
