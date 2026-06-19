import assert from "node:assert/strict";
import test from "node:test";

import { createWhiteboardDocument, type WhiteboardDocument } from "../model";
import { createRectangleElement, createTextElement } from "../scene";
import { createWhiteboardInputRuntime } from "./input-runtime";

const timestamp = "2026-06-02T00:00:00.000Z";

const baseDocument: WhiteboardDocument = createWhiteboardDocument({
  id: "input-keyboard-board",
  name: "Input Keyboard Board",
  viewport: { x: 0, y: 0, zoom: 1 },
  selection: ["rect-1"],
  elements: [
    createRectangleElement({
      id: "rect-1",
      x: 20,
      y: 20,
      width: 100,
      height: 80,
      style: { fill: "#ffffff", stroke: "#111827" },
      createdAt: timestamp,
    }),
    createTextElement({
      id: "text-1",
      x: 180,
      y: 32,
      width: 120,
      height: 44,
      text: "Note",
      style: { textColor: "#111827" },
      createdAt: timestamp,
    }),
  ],
  createdAt: timestamp,
  updatedAt: timestamp,
});

test("pan, wheel zoom, eraser, text, and freehand emit compatible commands", () => {
  const runtime = createWhiteboardInputRuntime({
    document: baseDocument,
    tool: "pan",
    viewport: { x: 0, y: 0, zoom: 1 },
    selection: { ids: [baseDocument.elements[0].id] },
    idFactory: () => "generated-1",
  });

  runtime.pointerDown({ pointerId: 1, clientX: 10, clientY: 10, button: 0 });
  assert.deepEqual(runtime.pointerMove({ pointerId: 1, clientX: 42, clientY: 50 }).commands, [
    { type: "viewport.set", viewport: { zoom: 1, x: 32, y: 40 } },
  ]);
  runtime.pointerUp({ pointerId: 1, clientX: 42, clientY: 50 });

  assert.equal(runtime.wheel({ clientX: 100, clientY: 100, deltaX: 0, deltaY: -100, ctrlKey: true }).commands[0]?.type, "viewport.set");

  runtime.setTool("eraser");
  assert.deepEqual(runtime.pointerDown({ pointerId: 2, clientX: 50, clientY: 50, button: 0 }).commands, [
    { type: "element.remove", ids: [baseDocument.elements[0].id] },
  ]);

  runtime.setTool("text");
  const textCommands = runtime.pointerDown({ pointerId: 3, clientX: 240, clientY: 96, button: 0 }).commands;
  assert.equal(textCommands[0]?.type, "element.add");
  assert.equal(textCommands[1]?.type, "tool.set");

  runtime.setTool("freehand");
  runtime.pointerDown({ pointerId: 4, clientX: 10, clientY: 10, button: 0 });
  runtime.pointerMove({ pointerId: 4, clientX: 20, clientY: 24 });
  assert.equal(runtime.pointerUp({ pointerId: 4, clientX: 30, clientY: 28 }).commands[0]?.type, "element.add");
});

test("keyboard shortcuts emit command-backed selection, viewport, edit, and tool workflows", () => {
  const runtime = createWhiteboardInputRuntime({
    document: baseDocument,
    tool: "select",
    viewport: { x: 0, y: 0, zoom: 1 },
    selection: { ids: [baseDocument.elements[0].id] },
  });

  assert.deepEqual(runtime.keyDown({ key: "Delete" }).commands, [
    { type: "element.remove", ids: [baseDocument.elements[0].id] },
  ]);

  runtime.setDocumentState({ selection: { ids: [] } });
  assert.deepEqual(runtime.keyDown({ key: "a", ctrlKey: true }).commands, [
    { type: "selection.set", ids: baseDocument.elements.map((element) => element.id) },
  ]);
  assert.deepEqual(runtime.keyDown({ key: "0", ctrlKey: true }).commands, [
    { type: "viewport.set", viewport: { x: 0, y: 0, zoom: 1 } },
  ]);
  runtime.setDocumentState({ selection: { ids: [baseDocument.elements[0].id] } });
  assert.deepEqual(runtime.keyDown({ key: "ArrowRight", shiftKey: true }).commands, [
    { type: "element.translate", ids: [baseDocument.elements[0].id], delta: { x: 10, y: 0 } },
  ]);
  assert.deepEqual(runtime.keyDown({ key: "d", ctrlKey: true }).commands, [
    { type: "element.duplicate", ids: [baseDocument.elements[0].id] },
  ]);
  assert.deepEqual(runtime.keyDown({ key: "g", ctrlKey: true }).commands, []);
  runtime.setDocumentState({ selection: { ids: baseDocument.elements.map((element) => element.id) } });
  assert.deepEqual(runtime.keyDown({ key: "g", ctrlKey: true }).commands, [
    { type: "group.create", ids: baseDocument.elements.map((element) => element.id) },
  ]);
  assert.deepEqual(runtime.keyDown({ key: "g", ctrlKey: true, shiftKey: true }).commands, [
    { type: "group.remove", elementIds: baseDocument.elements.map((element) => element.id) },
  ]);
  assert.deepEqual(runtime.keyDown({ key: "l", ctrlKey: true }).commands, [
    { type: "element.lock", ids: baseDocument.elements.map((element) => element.id) },
  ]);
  assert.deepEqual(runtime.keyDown({ key: "l", ctrlKey: true, shiftKey: true }).commands, [
    { type: "element.unlock", ids: baseDocument.elements.map((element) => element.id) },
  ]);
  assert.deepEqual(runtime.keyDown({ key: "h", ctrlKey: true }).commands, [
    { type: "element.hide", ids: baseDocument.elements.map((element) => element.id) },
  ]);
  runtime.setDocumentState({ selection: { ids: baseDocument.elements.map((element) => element.id) } });
  assert.deepEqual(runtime.keyDown({ key: "h", ctrlKey: true, shiftKey: true }).commands, [
    { type: "element.show", ids: baseDocument.elements.map((element) => element.id) },
  ]);
  assert.deepEqual(runtime.keyDown({ key: "r" }).commands, [{ type: "tool.set", tool: "rectangle" }]);
});
