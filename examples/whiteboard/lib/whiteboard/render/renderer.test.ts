import assert from "node:assert/strict";
import test from "node:test";

import {
  DEFAULT_WHITEBOARD_STYLE,
  createWhiteboardDocument,
  makeElementId,
  type WhiteboardDocument,
  type WhiteboardElement,
} from "../model";
import { fitViewportToBounds } from "./geometry";
import {
  createDiamondElement,
  createEllipseElement,
  createImageElement,
  createRectangleElement,
  createTextElement,
} from "../scene";
import {
  renderWhiteboardScene,
  type WhiteboardCanvasContext,
} from "./renderer";

type RecordedOperation = {
  name: string;
  args: unknown[];
};

class RecordingCanvasContext implements WhiteboardCanvasContext {
  operations: RecordedOperation[] = [];
  fillStyle = "";
  strokeStyle = "";
  lineWidth = 1;
  lineCap = "butt";
  lineJoin = "miter";
  font = "";
  textBaseline = "alphabetic";

  save() {
    this.operations.push({ name: "save", args: [] });
  }

  restore() {
    this.operations.push({ name: "restore", args: [] });
  }

  clearRect(...args: [number, number, number, number]) {
    this.operations.push({ name: "clearRect", args });
  }

  translate(...args: [number, number]) {
    this.operations.push({ name: "translate", args });
  }

  scale(...args: [number, number]) {
    this.operations.push({ name: "scale", args });
  }

  beginPath() {
    this.operations.push({ name: "beginPath", args: [] });
  }

  closePath() {
    this.operations.push({ name: "closePath", args: [] });
  }

  moveTo(...args: [number, number]) {
    this.operations.push({ name: "moveTo", args });
  }

  lineTo(...args: [number, number]) {
    this.operations.push({ name: "lineTo", args });
  }

  bezierCurveTo(...args: [number, number, number, number, number, number]) {
    this.operations.push({ name: "bezierCurveTo", args });
  }

  ellipse(...args: [number, number, number, number, number, number, number]) {
    this.operations.push({ name: "ellipse", args });
  }

  rect(...args: [number, number, number, number]) {
    this.operations.push({ name: "rect", args });
  }

  fill() {
    this.operations.push({ name: "fill", args: [this.fillStyle] });
  }

  stroke() {
    this.operations.push({
      name: "stroke",
      args: [this.strokeStyle, this.lineWidth, this.lineCap, this.lineJoin],
    });
  }

  fillText(...args: [string, number, number]) {
    this.operations.push({ name: "fillText", args: [this.font, this.fillStyle, ...args] });
  }

  setLineDash(args: number[]) {
    this.operations.push({ name: "setLineDash", args });
  }
}

const timestamp = "2026-06-02T00:00:00.000Z";
const embeddedImageSrc =
  "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 140 80'%3E%3Crect width='140' height='80' fill='%230f172a'/%3E%3C/svg%3E";

function lineElement(id: string, type: "line" | "arrow", points: [{ x: number; y: number }, { x: number; y: number }]): WhiteboardElement {
  return {
    id: makeElementId(id),
    type,
    points,
    startArrow: "none",
    endArrow: type === "arrow" ? "triangle" : "none",
    locked: false,
    hidden: false,
    style: {
      ...DEFAULT_WHITEBOARD_STYLE,
      stroke: type === "arrow" ? "#0f766e" : "#111827",
    },
    createdAt: timestamp,
    updatedAt: timestamp,
  };
}

function freehandElement(): WhiteboardElement {
  return {
    id: makeElementId("freehand-1"),
    type: "freehand",
    points: [
      { x: 240, y: 180 },
      { x: 260, y: 188 },
      { x: 282, y: 176 },
    ],
    locked: false,
    hidden: false,
    style: {
      ...DEFAULT_WHITEBOARD_STYLE,
      stroke: "#be123c",
    },
    createdAt: timestamp,
    updatedAt: timestamp,
  };
}

function sceneDocument(elements: readonly WhiteboardElement[]): WhiteboardDocument {
  return createWhiteboardDocument({
    id: "render-board",
    name: "Render Board",
    elements,
    selection: ["rect-1"],
    viewport: { x: 12, y: -8, zoom: 1.5 },
    createdAt: timestamp,
    updatedAt: timestamp,
  });
}

test("renders source-owned shapes, arrows, text, and selected handles through canvas operations", () => {
  const context = new RecordingCanvasContext();
  const document = sceneDocument([
    createRectangleElement({
      id: "rect-1",
      x: 20,
      y: 30,
      width: 120,
      height: 70,
      style: { fill: "#f8fafc", stroke: "#1f2937" },
      createdAt: timestamp,
    }),
    createEllipseElement({
      id: "ellipse-1",
      x: 180,
      y: 35,
      width: 90,
      height: 64,
      style: { fill: "transparent", stroke: "#2563eb" },
      createdAt: timestamp,
    }),
    createDiamondElement({
      id: "diamond-1",
      x: 320,
      y: 24,
      width: 96,
      height: 96,
      style: { fill: "#f5f3ff", stroke: "#7c3aed" },
      createdAt: timestamp,
    }),
    lineElement("arrow-1", "arrow", [
      { x: 48, y: 180 },
      { x: 188, y: 220 },
    ]),
    freehandElement(),
    createTextElement({
      id: "text-1",
      x: 360,
      y: 180,
      width: 160,
      height: 48,
      text: "Friday board",
      style: { textColor: "#111827" },
      createdAt: timestamp,
    }),
    createImageElement({
      id: "image-1",
      role: "image",
      x: 48,
      y: 252,
      width: 140,
      height: 80,
      src: embeddedImageSrc,
      alt: "Board image",
      style: {
        fill: "#0f172a",
        stroke: "#38bdf8",
        textColor: "#f8fafc",
      },
      createdAt: timestamp,
      updatedAt: timestamp,
    }),
  ]);

  renderWhiteboardScene(context, {
    document,
    size: { width: 640, height: 360 },
  });

  assert.deepEqual(context.operations.slice(0, 5), [
    { name: "save", args: [] },
    { name: "clearRect", args: [0, 0, 640, 360] },
    { name: "translate", args: [12, -8] },
    { name: "scale", args: [1.5, 1.5] },
    { name: "save", args: [] },
  ]);
  assert.ok(context.operations.some((op) => op.name === "rect" && op.args.join(",") === "20,30,120,70"));
  assert.ok(context.operations.some((op) => op.name === "ellipse" && op.args.join(",") === "225,67,45,32,0,0,6.283185307179586"));
  assert.ok(context.operations.some((op) => op.name === "lineTo" && op.args.join(",") === "416,72"));
  assert.ok(context.operations.some((op) => op.name === "lineTo" && op.args.join(",") === "188,220"));
  assert.ok(context.operations.some((op) => op.name === "fillText" && op.args.includes("Friday board")));
  assert.ok(context.operations.some((op) => op.name === "rect" && op.args.join(",") === "48,252,140,80"));
  assert.ok(context.operations.some((op) => op.name === "fillText" && op.args.includes("Board image")));

  const selectedHandleRects = context.operations.filter(
    (op) => op.name === "rect" && op.args[2] === 8 && op.args[3] === 8,
  );
  assert.equal(selectedHandleRects.length, 8);
});

test("skips hidden image placeholders during canvas rendering", () => {
  const context = new RecordingCanvasContext();
  const document = sceneDocument([
    createImageElement({
      id: "hidden-image",
      x: 48,
      y: 52,
      width: 140,
      height: 80,
      src: embeddedImageSrc,
      alt: "Hidden board image",
      hidden: true,
      createdAt: timestamp,
    }),
  ]);

  renderWhiteboardScene(context, {
    document,
    viewport: { x: 0, y: 0, zoom: 1 },
    size: { width: 240, height: 160 },
    selection: { ids: [] },
  });

  assert.equal(context.operations.some((op) => op.name === "rect" && op.args.join(",") === "48,52,140,80"), false);
  assert.equal(context.operations.some((op) => op.name === "fillText" && op.args.includes("Hidden board image")), false);
});

test("renders draft elements without mutating the document", () => {
  const context = new RecordingCanvasContext();
  const document = sceneDocument([
    lineElement("line-1", "line", [
      { x: 10, y: 10 },
      { x: 90, y: 90 },
    ]),
  ]);

  renderWhiteboardScene(context, {
    document,
    viewport: { x: 0, y: 0, zoom: 1 },
    size: { width: 200, height: 200 },
    selection: { ids: [] },
    draft: createRectangleElement({
      id: "draft-rect",
      x: 80,
      y: 24,
      width: 40,
      height: 64,
      style: { fill: "transparent", stroke: "#64748b" },
      createdAt: timestamp,
    }),
  });

  assert.equal(document.elements.length, 1);
  assert.ok(context.operations.some((op) => op.name === "rect" && op.args.join(",") === "80,24,40,64"));
  assert.ok(context.operations.some((op) => op.name === "lineTo" && op.args.join(",") === "90,90"));
});

test("renders runtime selection area separately from durable selection", () => {
  const context = new RecordingCanvasContext();
  const document = sceneDocument([
    createRectangleElement({
      id: "rect-1",
      x: 20,
      y: 30,
      width: 120,
      height: 70,
      createdAt: timestamp,
    }),
  ]);

  renderWhiteboardScene(context, {
    document,
    viewport: { x: 0, y: 0, zoom: 1 },
    size: { width: 240, height: 180 },
    selection: { ids: [] },
    selectionArea: { x: 4, y: 8, width: 140, height: 96 },
  });

  assert.ok(context.operations.some((op) => op.name === "rect" && op.args.join(",") === "4,8,140,96"));
  assert.ok(context.operations.some((op) => op.name === "fill" && op.args.includes("hsla(212, 100%, 58%, 0.1)")));
  assert.ok(context.operations.some((op) => op.name === "setLineDash" && op.args.join(",") === "6,6"));
});

test("fits the viewport to padded bounds without producing invalid zoom", () => {
  assert.deepEqual(
    fitViewportToBounds({ x: 100, y: 50, width: 200, height: 100 }, { width: 600, height: 400 }, { padding: 50 }),
    { x: -200, y: -50, zoom: 2.5 },
  );

  const tinyFit = fitViewportToBounds(
    { x: 10, y: 20, width: 0, height: 0 },
    { width: 320, height: 240 },
    { padding: 24, maxZoom: 4 },
  );

  assert.equal(Number.isFinite(tinyFit.x), true);
  assert.equal(Number.isFinite(tinyFit.y), true);
  assert.equal(tinyFit.zoom, 4);
});

test("skips hidden elements and hidden selection handles", () => {
  const context = new RecordingCanvasContext();
  const document = createWhiteboardDocument({
    id: "hidden-render-board",
    selection: ["hidden-rect"],
    elements: [
      createRectangleElement({
        id: "hidden-rect",
        x: 20,
        y: 30,
        width: 120,
        height: 70,
        hidden: true,
        createdAt: timestamp,
      }),
    ],
    createdAt: timestamp,
    updatedAt: timestamp,
  });

  renderWhiteboardScene(context, {
    document,
    size: { width: 240, height: 160 },
  });

  assert.equal(context.operations.some((op) => op.name === "rect" && op.args.join(",") === "20,30,120,70"), false);
  assert.equal(
    context.operations.filter((op) => op.name === "rect" && op.args[2] === 8 && op.args[3] === 8).length,
    0,
  );
});
