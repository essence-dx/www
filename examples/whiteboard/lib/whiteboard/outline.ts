import { frameIdForElement, isFrameElement } from "./frames";
import { getElementBounds } from "./geometry";
import type { WhiteboardCommand } from "./commands";
import type {
  WhiteboardDocument,
  WhiteboardElement,
  WhiteboardElementId,
  WhiteboardElementType,
  WhiteboardRect,
  WhiteboardSize,
  WhiteboardViewport,
} from "./model";
import { fitViewportToBounds } from "./render/geometry";

export type WhiteboardOutlineItem = {
  readonly id: WhiteboardElementId;
  readonly label: string;
  readonly type: WhiteboardElementType;
  readonly role: string;
  readonly frameId: WhiteboardElementId | null;
  readonly hidden: boolean;
  readonly locked: boolean;
  readonly selected: boolean;
  readonly bounds: WhiteboardRect;
  readonly viewport: WhiteboardViewport;
};

export type WhiteboardOutlineSection = {
  readonly id: string;
  readonly title: string;
  readonly frameId: WhiteboardElementId | null;
  readonly itemCount: number;
  readonly items: readonly WhiteboardOutlineItem[];
};

export type WhiteboardOutlineModel = {
  readonly sections: readonly WhiteboardOutlineSection[];
  readonly itemCount: number;
  readonly selectedItemCount: number;
  readonly empty: boolean;
};

export function createWhiteboardOutlineModel(
  document: WhiteboardDocument,
  stageSize: WhiteboardSize,
): WhiteboardOutlineModel {
  const selectedIds = new Set(document.selection);
  const items = document.elements.map((element) => createOutlineItem(element, selectedIds, stageSize));
  const frameItems = items.filter((item) => isFrameItem(item));
  const sectionByFrameId = new Map<WhiteboardElementId, WhiteboardOutlineItem[]>(
    frameItems.map((frame) => [frame.id, []]),
  );
  const unframedItems: WhiteboardOutlineItem[] = [];

  for (const item of items) {
    if (isFrameItem(item)) continue;

    const sectionItems = item.frameId ? sectionByFrameId.get(item.frameId) : undefined;
    if (sectionItems) {
      sectionItems.push(item);
    } else {
      unframedItems.push(item);
    }
  }

  const frameSections = frameItems.map((frame) => {
    const children = sectionByFrameId.get(frame.id) ?? [];
    const sectionItems = [frame, ...children];

    return {
      id: `frame:${frame.id}`,
      title: frame.label,
      frameId: frame.id,
      itemCount: sectionItems.length,
      items: sectionItems,
    };
  });
  const sections = unframedItems.length > 0
    ? [
        ...frameSections,
        {
          id: "unframed",
          title: "Unframed",
          frameId: null,
          itemCount: unframedItems.length,
          items: unframedItems,
        },
      ]
    : frameSections;

  return {
    sections,
    itemCount: items.length,
    selectedItemCount: items.filter((item) => item.selected).length,
    empty: items.length === 0,
  };
}

export function outlineCommandsForItem(
  item: WhiteboardOutlineItem,
): readonly WhiteboardCommand[] {
  return [
    {
      type: "selection.set",
      ids: [item.id],
      mode: "replace",
    },
    {
      type: "viewport.set",
      viewport: item.viewport,
    },
  ];
}

function createOutlineItem(
  element: WhiteboardElement,
  selectedIds: ReadonlySet<WhiteboardElementId>,
  stageSize: WhiteboardSize,
): WhiteboardOutlineItem {
  const bounds = getElementBounds(element);

  return {
    id: element.id,
    label: elementLabel(element),
    type: element.type,
    role: isFrameElement(element) ? "frame" : element.role ?? element.type,
    frameId: frameIdForElement(element),
    hidden: element.hidden,
    locked: element.locked,
    selected: selectedIds.has(element.id),
    bounds,
    viewport: fitViewportToBounds(bounds, stageSize, { padding: 96 }),
  };
}

function elementLabel(element: WhiteboardElement): string {
  const name = element.name?.trim();
  if (name) return name;

  if (element.type === "text") {
    const text = element.text.trim().replace(/\s+/g, " ");
    if (text) return text.length > 36 ? `${text.slice(0, 33)}...` : text;
  }

  if (element.role === "sticky-note") return "Sticky note";
  if (element.role === "connector") return "Connector";
  if (element.role === "checklist") return "Checklist";
  if (element.role === "image") return "Image";
  if (isFrameElement(element)) return "Frame";

  return element.type;
}

function isFrameItem(item: WhiteboardOutlineItem): boolean {
  return item.role === "frame";
}
