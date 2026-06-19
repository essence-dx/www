import {
  DEFAULT_WHITEBOARD_STYLE,
  makeElementId,
  type WhiteboardElement,
  type WhiteboardPoint,
  type WhiteboardStyle,
} from "./model";
import {
  createDiamondElement,
  createImageElement,
  createRectangleElement,
  createTextElement,
} from "./scene";
import {
  createTemplatePresetElements,
  type WhiteboardTemplatePresetId,
} from "./library-templates";

export type WhiteboardLibraryPresetCategory = "single" | "template";

export type WhiteboardLibraryPresetId =
  | "sticky-note"
  | "decision"
  | "connector"
  | "checklist"
  | "frame"
  | "image"
  | WhiteboardTemplatePresetId;

export type WhiteboardLibraryPreset = {
  readonly id: WhiteboardLibraryPresetId;
  readonly label: string;
  readonly description: string;
  readonly category: WhiteboardLibraryPresetCategory;
  readonly elementCount: number;
};

export const WHITEBOARD_LIBRARY_PRESETS: readonly WhiteboardLibraryPreset[] = [
  {
    id: "sticky-note",
    label: "Sticky note",
    description: "A writable note with warm fill and readable text.",
    category: "single",
    elementCount: 1,
  },
  {
    id: "decision",
    label: "Decision",
    description: "A diamond decision node for flows and architecture diagrams.",
    category: "single",
    elementCount: 2,
  },
  {
    id: "connector",
    label: "Connector",
    description: "An arrow connector for linking ideas.",
    category: "single",
    elementCount: 1,
  },
  {
    id: "checklist",
    label: "Checklist",
    description: "A task card with a text checklist.",
    category: "single",
    elementCount: 2,
  },
  {
    id: "frame",
    label: "Frame",
    description: "A named section frame for grouping a board area.",
    category: "single",
    elementCount: 2,
  },
  {
    id: "image",
    label: "Image",
    description: "A source-owned embedded image with editable alt text.",
    category: "single",
    elementCount: 1,
  },
  {
    id: "flowchart-basic",
    label: "Flowchart",
    description: "Start, decision, outcomes, labels, and routed connectors.",
    category: "template",
    elementCount: 11,
  },
  {
    id: "kanban-board",
    label: "Kanban board",
    description: "A framed workflow board with columns and starter cards.",
    category: "template",
    elementCount: 11,
  },
  {
    id: "retrospective-board",
    label: "Retrospective",
    description: "A framed four-quadrant retro with editable prompts.",
    category: "template",
    elementCount: 14,
  },
  {
    id: "system-map",
    label: "System map",
    description: "Client, API, worker, database, queue, and bound connectors.",
    category: "template",
    elementCount: 15,
  },
] as const;
export const WHITEBOARD_LIBRARY_TEMPLATE_PRESETS = WHITEBOARD_LIBRARY_PRESETS.filter(
  (preset) => preset.category === "template",
);

export function createLibraryPresetElements(options: {
  readonly preset: WhiteboardLibraryPresetId;
  readonly origin?: WhiteboardPoint;
  readonly idPrefix?: string;
  readonly now?: string;
  readonly style?: Partial<WhiteboardStyle>;
}): readonly WhiteboardElement[] {
  const origin = options.origin ?? { x: 96, y: 96 };
  const now = options.now ?? new Date().toISOString();
  const prefix = options.idPrefix ?? `library-${options.preset}`;
  const style = options.style ?? {};

  switch (options.preset) {
    case "sticky-note":
      return [
        createTextElement({
          id: `${prefix}-note`,
          role: "sticky-note",
          name: "Sticky note",
          x: origin.x,
          y: origin.y,
          width: 240,
          height: 132,
          text: "New note",
          style: {
            fill: "#fef3c7",
            stroke: "#f59e0b",
            strokeWidth: 2,
            textColor: "#111827",
            fontSize: 22,
            ...style,
          },
          createdAt: now,
          updatedAt: now,
          metadata: { preset: "sticky-note", role: "sticky-note" },
        }),
      ];
    case "decision":
      return [
        createDiamondElement({
          id: `${prefix}-decision`,
          role: "shape",
          name: "Decision",
          x: origin.x,
          y: origin.y,
          width: 220,
          height: 150,
          style: {
            fill: "#ede9fe",
            stroke: "#8b5cf6",
            strokeWidth: 2,
            textColor: "#312e81",
            ...style,
          },
          createdAt: now,
          updatedAt: now,
          metadata: { preset: "decision" },
        }),
        createTextElement({
          id: `${prefix}-decision-label`,
          role: "label",
          name: "Decision label",
          x: origin.x + 56,
          y: origin.y + 58,
          width: 120,
          height: 36,
          text: "Decide",
          style: {
            fill: "transparent",
            stroke: "#312e81",
            textColor: "#312e81",
            fontSize: 20,
            ...style,
          },
          createdAt: now,
          updatedAt: now,
          metadata: { preset: "decision-label" },
        }),
      ];
    case "connector":
      return [
        {
          id: makeElementId(`${prefix}-arrow`),
          type: "arrow",
          role: "connector",
          points: [
            { x: origin.x, y: origin.y },
            { x: origin.x + 260, y: origin.y },
          ],
          locked: false,
          hidden: false,
          style: {
            ...DEFAULT_WHITEBOARD_STYLE,
            stroke: "#38bdf8",
            strokeWidth: 4,
            ...style,
          },
          startArrow: "none",
          endArrow: "triangle",
          createdAt: now,
          updatedAt: now,
          metadata: { preset: "connector", role: "connector" },
        },
      ];
    case "checklist":
      return [
        createRectangleElement({
          id: `${prefix}-card`,
          role: "checklist",
          name: "Checklist",
          x: origin.x,
          y: origin.y,
          width: 280,
          height: 176,
          style: {
            fill: "#f8fafc",
            stroke: "#111827",
            strokeWidth: 2,
            textColor: "#111827",
            ...style,
          },
          createdAt: now,
          updatedAt: now,
          metadata: { preset: "checklist-frame", role: "checklist" },
        }),
        createTextElement({
          id: `${prefix}-items`,
          role: "label",
          name: "Checklist items",
          x: origin.x + 20,
          y: origin.y + 24,
          width: 240,
          height: 120,
          text: "- Capture idea\n- Assign owner\n- Ship receipt",
          style: {
            fill: "transparent",
            stroke: "#111827",
            textColor: "#111827",
            fontSize: 18,
            ...style,
          },
          createdAt: now,
          updatedAt: now,
          metadata: { preset: "checklist-items" },
        }),
      ];
    case "frame":
      return [
        createRectangleElement({
          id: `${prefix}-frame`,
          role: "frame",
          name: "Frame",
          x: origin.x,
          y: origin.y,
          width: 520,
          height: 300,
          style: {
            fill: "transparent",
            stroke: "#71717a",
            strokeStyle: "dashed",
            strokeWidth: 2,
            ...style,
          },
          createdAt: now,
          updatedAt: now,
          metadata: { preset: "frame", frame: true, role: "frame" },
        }),
        createTextElement({
          id: `${prefix}-frame-title`,
          role: "label",
          name: "Frame title",
          x: origin.x + 18,
          y: origin.y + 18,
          width: 260,
          height: 40,
          text: "Frame",
          style: {
            fill: "transparent",
            stroke: "#71717a",
            textColor: "#18181b",
            fontSize: 24,
            ...style,
          },
          createdAt: now,
          updatedAt: now,
          metadata: { preset: "frame-title" },
        }),
      ];
    case "image":
      return [
        createImageElement({
          id: `${prefix}-image`,
          role: "image",
          name: "Image",
          x: origin.x,
          y: origin.y,
          width: 320,
          height: 200,
          src: "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 320 200'%3E%3Crect width='320' height='200' fill='%23111827'/%3E%3Cpath d='M40 150 120 88l54 44 42-32 64 50' fill='none' stroke='%23f8fafc' stroke-width='12' stroke-linecap='round' stroke-linejoin='round'/%3E%3Ccircle cx='230' cy='62' r='22' fill='%2338bdf8'/%3E%3C/svg%3E",
          alt: "Embedded whiteboard image",
          naturalWidth: 320,
          naturalHeight: 200,
          style: {
            fill: "transparent",
            stroke: "#38bdf8",
            strokeWidth: 2,
            textColor: "#f8fafc",
            ...style,
          },
          createdAt: now,
          updatedAt: now,
          metadata: { preset: "image", role: "image", sourceOwned: true },
        }),
      ];
    case "flowchart-basic":
    case "kanban-board":
    case "retrospective-board":
    case "system-map":
      return createTemplatePresetElements({
        preset: options.preset,
        origin,
        idPrefix: prefix,
        now,
        style,
      });
  }
}
