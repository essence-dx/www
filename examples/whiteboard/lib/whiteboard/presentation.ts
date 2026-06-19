import { frameChildren, frameIdForElement, isFrameElement } from "./frames";
import { getElementBounds } from "./geometry";
import type { WhiteboardCommand } from "./commands";
import type {
  WhiteboardDocument,
  WhiteboardElementId,
  WhiteboardRect,
  WhiteboardSize,
  WhiteboardViewport,
} from "./model";
import { fitViewportToBounds } from "./render/geometry";

export type WhiteboardPresentationSlide = {
  readonly id: WhiteboardElementId;
  readonly title: string;
  readonly index: number;
  readonly bounds: WhiteboardRect;
  readonly childCount: number;
  readonly selected: boolean;
  readonly viewport: WhiteboardViewport;
};

export type WhiteboardPresentationModel = {
  readonly slides: readonly WhiteboardPresentationSlide[];
  readonly currentIndex: number;
  readonly currentSlide: WhiteboardPresentationSlide | null;
  readonly previousSlide: WhiteboardPresentationSlide | null;
  readonly nextSlide: WhiteboardPresentationSlide | null;
  readonly empty: boolean;
};

export function createWhiteboardPresentationModel(
  document: WhiteboardDocument,
  stageSize: WhiteboardSize,
): WhiteboardPresentationModel {
  const frameElements = document.elements.filter((element) => isFrameElement(element) && !element.hidden);
  const frameIds = new Set(frameElements.map((element) => element.id));
  const currentFrameId = resolveCurrentFrameId(document, frameIds);
  const selectedFrameId = currentFrameId ?? frameElements[0]?.id ?? null;
  const slides = frameElements.map((frame, index) => {
    const bounds = getElementBounds(frame);

    return {
      id: frame.id,
      title: frame.name?.trim() || `Frame ${index + 1}`,
      index,
      bounds,
      childCount: frameChildren(document, frame.id).length,
      selected: selectedFrameId === frame.id,
      viewport: fitViewportToBounds(bounds, stageSize, { padding: 72 }),
    };
  });
  const currentIndex = slides.findIndex((slide) => slide.selected);
  const safeIndex = currentIndex >= 0 ? currentIndex : -1;

  return {
    slides,
    currentIndex: safeIndex,
    currentSlide: safeIndex >= 0 ? slides[safeIndex] ?? null : null,
    previousSlide: safeIndex > 0 ? slides[safeIndex - 1] ?? null : null,
    nextSlide: safeIndex >= 0 ? slides[safeIndex + 1] ?? null : null,
    empty: slides.length === 0,
  };
}

export function presentationCommandsForSlide(
  slide: WhiteboardPresentationSlide,
): readonly WhiteboardCommand[] {
  return [
    {
      type: "selection.set",
      ids: [slide.id],
      mode: "replace",
    },
    {
      type: "viewport.set",
      viewport: slide.viewport,
    },
  ];
}

function resolveCurrentFrameId(
  document: WhiteboardDocument,
  frameIds: ReadonlySet<WhiteboardElementId>,
): WhiteboardElementId | null {
  const elementById = new Map(document.elements.map((element) => [element.id, element]));

  for (const id of document.selection) {
    const element = elementById.get(id);
    if (element && isFrameElement(element) && frameIds.has(element.id)) {
      return element.id;
    }
  }

  for (const id of document.selection) {
    const element = elementById.get(id);
    if (!element) continue;

    const frameId = frameIdForElement(element);
    if (frameId && frameIds.has(frameId)) {
      return frameId;
    }
  }

  return null;
}
