import {
  DEFAULT_WHITEBOARD_STYLE,
  createWhiteboardDocument,
  makeElementId,
  type WhiteboardDocument,
  type WhiteboardElement,
} from "./model";
import {
  createDiamondElement,
  createEllipseElement,
  createRectangleElement,
  createTextElement,
} from "./scene";

const CREATED_AT = "2026-06-02T00:00:00.000Z";

export function createDemoWhiteboardDocument(): WhiteboardDocument {
  const elements = [
    createRectangleElement({
      id: "wb-lane-map",
      name: "Lane map",
      x: 112,
      y: 96,
      width: 320,
      height: 156,
      style: {
        fill: "#18181b",
        stroke: "#38bdf8",
        strokeWidth: 2,
        textColor: "#f8fafc",
      },
      createdAt: CREATED_AT,
      updatedAt: CREATED_AT,
    }),
    createDiamondElement({
      id: "wb-agent-loop",
      name: "Agent pass",
      x: 508,
      y: 118,
      width: 180,
      height: 132,
      style: {
        fill: "transparent",
        stroke: "#a78bfa",
      },
      createdAt: CREATED_AT,
      updatedAt: CREATED_AT,
    }),
    createConnectorElement("wb-receipt-arrow", "arrow", [
      { x: 702, y: 185 },
      { x: 924, y: 185 },
    ]),
    createTextElement({
      id: "wb-export-note",
      name: "Export note",
      x: 740,
      y: 282,
      width: 320,
      height: 54,
      text: "Export clean receipts",
      style: {
        fill: "transparent",
        fontSize: 28,
        stroke: "#34d399",
        textColor: "#34d399",
      },
      createdAt: CREATED_AT,
      updatedAt: CREATED_AT,
    }),
    createFreehandElement(),
    createEllipseElement({
      id: "wb-storage-proof",
      name: "Local-first proof",
      x: 168,
      y: 310,
      width: 188,
      height: 118,
      style: {
        fill: "transparent",
        stroke: "#fb7185",
        strokeStyle: "dashed",
      },
      createdAt: CREATED_AT,
      updatedAt: CREATED_AT,
    }),
  ];

  return createWhiteboardDocument({
    id: "dx-whiteboard-demo",
    name: "DX Whiteboard",
    elements,
    selection: ["wb-lane-map"],
    activeTool: "select",
    viewport: { x: 0, y: 0, zoom: 1 },
    currentStyle: {
      ...DEFAULT_WHITEBOARD_STYLE,
      stroke: "#111827",
      fill: "transparent",
      strokeWidth: 3,
    },
    createdAt: CREATED_AT,
    updatedAt: CREATED_AT,
    metadata: {
      gridVisible: true,
      snapToGrid: true,
      revision: 0,
    },
  });
}

function createConnectorElement(
  id: string,
  type: "line" | "arrow",
  points: readonly [{ readonly x: number; readonly y: number }, { readonly x: number; readonly y: number }],
): WhiteboardElement {
  return {
    id: makeElementId(id),
    type,
    points,
    locked: false,
    hidden: false,
    style: {
      ...DEFAULT_WHITEBOARD_STYLE,
      stroke: type === "arrow" ? "#f59e0b" : "#111827",
      strokeWidth: 4,
    },
    startArrow: "none",
    endArrow: type === "arrow" ? "triangle" : "none",
    createdAt: CREATED_AT,
    updatedAt: CREATED_AT,
  };
}

function createFreehandElement(): WhiteboardElement {
  return {
    id: makeElementId("wb-drawn-path"),
    type: "freehand",
    points: [
      { x: 162, y: 430 },
      { x: 220, y: 392 },
      { x: 284, y: 426 },
      { x: 356, y: 388 },
      { x: 458, y: 420 },
    ],
    locked: false,
    hidden: false,
    style: {
      ...DEFAULT_WHITEBOARD_STYLE,
      stroke: "#fb7185",
      strokeStyle: "dashed",
      strokeWidth: 3,
    },
    createdAt: CREATED_AT,
    updatedAt: CREATED_AT,
  };
}
