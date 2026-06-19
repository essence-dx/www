import assert from "node:assert/strict";
import { test } from "node:test";

import { createRectangleElement } from "../whiteboard/scene";
import {
  DEFAULT_WHITEBOARD_VIEWPORT,
  createWhiteboardDocument,
  makeElementId,
} from "../whiteboard/model";
import { createWhiteboardStore } from "./whiteboard-store";
import {
  createWhiteboardInputController,
  whiteboardKeyboardInputRecognized,
} from "./whiteboard-input-controller";

test("store-backed input controller draws through the canonical reducer", () => {
  const store = createWhiteboardStore({
    document: createWhiteboardDocument({ id: "controller-draw-board" }),
  });
  const controller = createWhiteboardInputController(store, {
    idFactory: () => "drawn-rectangle",
  });

  store.actions.setTool("rectangle");
  assert.equal(controller.pointerDown({ pointerId: 1, clientX: 16, clientY: 24, button: 0 }).commands.length, 0);
  assert.equal(controller.pointerMove({ pointerId: 1, clientX: 136, clientY: 104 }).state.draft?.type, "rectangle");
  const result = controller.pointerUp({ pointerId: 1, clientX: 136, clientY: 104 });
  const document = store.getDocument();

  assert.deepEqual(result.commands.map((command) => command.type), ["element.add", "tool.set"]);
  assert.equal(document.elements[0]?.id, makeElementId("drawn-rectangle"));
  assert.equal(document.elements[0]?.type, "rectangle");
  assert.deepEqual(document.selection, [makeElementId("drawn-rectangle")]);
  assert.equal(document.activeTool, "select");
  store.undo();
  assert.equal(store.getDocument().elements.length, 0);
  assert.equal(store.getDocument().activeTool, "rectangle");
  controller.destroy();
});

test("store-backed input controller preserves drag state while moving selected elements", () => {
  const store = createWhiteboardStore({
    document: createWhiteboardDocument({
      id: "controller-move-board",
      elements: [
        createRectangleElement({
          id: "card",
        x: 20,
        y: 20,
        width: 120,
        height: 80,
        style: { fill: "#ffffff" },
      }),
      ],
    }),
  });
  const controller = createWhiteboardInputController(store);

  assert.deepEqual(controller.pointerDown({ pointerId: 1, clientX: 80, clientY: 60, button: 0 }).commands, [
    { type: "selection.set", ids: [makeElementId("card")], mode: "replace" },
  ]);
  assert.equal(controller.pointerMove({ pointerId: 1, clientX: 108, clientY: 92 }).commands[0]?.type, "element.update");
  controller.pointerUp({ pointerId: 1, clientX: 108, clientY: 92 });

  const moved = store.getDocument().elements[0];
  assert.equal("x" in (moved ?? {}) ? moved.x : null, 48);
  assert.equal("y" in (moved ?? {}) ? moved.y : null, 52);
  controller.destroy();
});

test("store-backed input controller applies wheel pan and anchored zoom", () => {
  const store = createWhiteboardStore({
    document: createWhiteboardDocument({
      id: "controller-wheel-board",
      viewport: DEFAULT_WHITEBOARD_VIEWPORT,
    }),
  });
  const controller = createWhiteboardInputController(store);

  controller.wheel({ clientX: 50, clientY: 60, deltaX: 12, deltaY: 16 });
  assert.deepEqual(store.getDocument().viewport, { x: -12, y: -16, zoom: 1 });

  controller.wheel({ clientX: 50, clientY: 60, ctrlKey: true, deltaX: 0, deltaY: -1 });
  assert.equal(store.getDocument().viewport.zoom, 1.1);
  controller.destroy();
});

test("store-backed input controller syncs after external store commands", () => {
  const store = createWhiteboardStore({
    document: createWhiteboardDocument({ id: "controller-sync-board" }),
  });
  const controller = createWhiteboardInputController(store, {
    idFactory: () => "synced-text",
  });

  store.actions.setTool("text");
  const result = controller.pointerDown({ pointerId: 1, clientX: 40, clientY: 48, button: 0 });
  const text = store.getDocument().elements[0];

  assert.deepEqual(result.commands.map((command) => command.type), ["element.add", "tool.set"]);
  assert.equal(text?.type, "text");
  assert.equal(text?.id, makeElementId("synced-text"));
  controller.destroy();
});

test("keyboard recognition keeps no-op shortcuts from leaking to the page", () => {
  assert.equal(whiteboardKeyboardInputRecognized({ key: "Delete" }), true);
  assert.equal(whiteboardKeyboardInputRecognized({ key: "d", ctrlKey: true }), true);
  assert.equal(whiteboardKeyboardInputRecognized({ key: "ArrowLeft" }), true);
  assert.equal(whiteboardKeyboardInputRecognized({ key: "?" }), false);
});
