import { frameIdForElement, isFrameElement } from "./frames";
import type { WhiteboardCommand } from "./commands";
import type {
  WhiteboardDocument,
  WhiteboardElement,
  WhiteboardRect,
  WhiteboardSize,
  WhiteboardViewport,
} from "./model";
import {
  getDocumentContentBounds,
  getDocumentSelectionBounds,
  getSelectedElements,
} from "./scene";
import { fitViewportToBounds } from "./render/geometry";

export type WhiteboardMeasurementSubject = "selection" | "document" | "empty";

export type WhiteboardMeasurementCounts = {
  readonly items: number;
  readonly hidden: number;
  readonly locked: number;
  readonly framed: number;
  readonly frames: number;
  readonly connectors: number;
  readonly images: number;
  readonly text: number;
};

export type WhiteboardMeasurementModel = {
  readonly subject: WhiteboardMeasurementSubject;
  readonly label: string;
  readonly counts: WhiteboardMeasurementCounts;
  readonly bounds: WhiteboardRect | null;
  readonly viewport: WhiteboardViewport | null;
  readonly canFocus: boolean;
};

export function createWhiteboardMeasurementModel(
  document: WhiteboardDocument,
  stageSize: WhiteboardSize,
): WhiteboardMeasurementModel {
  const selectedElements = getSelectedElements(document);
  const subject: WhiteboardMeasurementSubject = selectedElements.length > 0
    ? "selection"
    : document.elements.length > 0
      ? "document"
      : "empty";
  const measuredElements = subject === "selection" ? selectedElements : document.elements;
  const bounds = subject === "selection"
    ? getDocumentSelectionBounds(document)
    : getDocumentContentBounds(document);
  const viewport = bounds ? fitViewportToBounds(bounds, stageSize, { padding: 84 }) : null;

  return {
    subject,
    label: measurementLabel(subject, selectedElements),
    counts: measurementCounts(measuredElements),
    bounds,
    viewport,
    canFocus: Boolean(viewport),
  };
}

export function measurementCommandsForFocus(
  model: WhiteboardMeasurementModel,
): readonly WhiteboardCommand[] {
  return model.viewport
    ? [
        {
          type: "viewport.set",
          viewport: model.viewport,
        },
      ]
    : [];
}

function measurementLabel(
  subject: WhiteboardMeasurementSubject,
  selectedElements: readonly WhiteboardElement[],
): string {
  if (subject === "empty") return "Empty board";
  if (subject === "document") return "Board contents";

  if (selectedElements.length === 1) {
    const element = selectedElements[0];
    return element?.name?.trim() || element?.role || element?.type || "Selection";
  }

  return `${selectedElements.length} selected`;
}

function measurementCounts(elements: readonly WhiteboardElement[]): WhiteboardMeasurementCounts {
  return {
    items: elements.length,
    hidden: elements.filter((element) => element.hidden).length,
    locked: elements.filter((element) => element.locked).length,
    framed: elements.filter((element) => Boolean(frameIdForElement(element))).length,
    frames: elements.filter(isFrameElement).length,
    connectors: elements.filter((element) =>
      element.type === "line" || element.type === "arrow" || element.role === "connector"
    ).length,
    images: elements.filter((element) => element.type === "image").length,
    text: elements.filter((element) => element.type === "text").length,
  };
}
