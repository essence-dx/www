import assert from "node:assert/strict";
import test from "node:test";

import { createWhiteboardDocument, makeElementId, makeGroupId, type WhiteboardDocument } from "../model";
import { createRectangleElement, createTextElement } from "../scene";
import { createWhiteboardInputRuntime } from "./input-runtime";

const timestamp = "2026-06-02T00:00:00.000Z";

const baseDocument: WhiteboardDocument = createWhiteboardDocument({
  id: "input-board",
  name: "Input Board",
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

test("rectangle tool normalizes drag bounds and emits a source-owned insert command", () => {
  const runtime = createWhiteboardInputRuntime({
    document: baseDocument,
    tool: "rectangle",
    viewport: { x: 10, y: 20, zoom: 2 },
    selection: { ids: [] },
    idFactory: () => "shape-1",
  });

  assert.deepEqual(runtime.pointerDown({ pointerId: 1, clientX: 210, clientY: 220, button: 0 }).commands, []);
  assert.equal(runtime.pointerMove({ pointerId: 1, clientX: 110, clientY: 80 }).state.draft?.type, "rectangle");

  const result = runtime.pointerUp({ pointerId: 1, clientX: 110, clientY: 80 });
  const addCommand = result.commands[0];

  assert.equal(addCommand?.type, "element.add");
  assert.equal(addCommand.type === "element.add" ? addCommand.element.id : null, "shape-1");
  assert.deepEqual(
    addCommand.type === "element.add"
      ? {
          type: addCommand.element.type,
          x: "x" in addCommand.element ? addCommand.element.x : null,
          y: "y" in addCommand.element ? addCommand.element.y : null,
          width: "width" in addCommand.element ? addCommand.element.width : null,
          height: "height" in addCommand.element ? addCommand.element.height : null,
        }
      : null,
    { type: "rectangle", x: 50, y: 30, width: 50, height: 70 },
  );
  assert.equal(result.commands[1]?.type, "tool.set");
});

test("select tool hit-tests, drags elements, and resizes from selection handles", () => {
  const runtime = createWhiteboardInputRuntime({
    document: baseDocument,
    tool: "select",
    viewport: { x: 0, y: 0, zoom: 1 },
    selection: { ids: [baseDocument.elements[0].id] },
  });

  assert.deepEqual(runtime.pointerDown({ pointerId: 1, clientX: 40, clientY: 40, button: 0 }).commands, [
    { type: "selection.set", ids: [baseDocument.elements[0].id], mode: "replace" },
  ]);

  const move = runtime.pointerMove({ pointerId: 1, clientX: 56, clientY: 68 }).commands[0];

  assert.equal(move?.type, "element.update");
  assert.deepEqual(move?.type === "element.update" ? move.patch : null, { x: 36, y: 48 });

  runtime.pointerUp({ pointerId: 1, clientX: 56, clientY: 68 });

  assert.equal(runtime.pointerDown({ pointerId: 2, clientX: 120, clientY: 100, button: 0 }).commands[0]?.type, "selection.set");
  assert.equal(runtime.pointerMove({ pointerId: 2, clientX: 150, clientY: 130 }).commands[0]?.type, "element.update");
});

test("select tool supports marquee selection and empty-click clearing", () => {
  const runtime = createWhiteboardInputRuntime({
    document: baseDocument,
    tool: "select",
    viewport: { x: 0, y: 0, zoom: 1 },
    selection: { ids: [baseDocument.elements[1].id] },
  });

  assert.deepEqual(runtime.pointerDown({ pointerId: 1, clientX: 4, clientY: 4, button: 0 }).commands, []);
  assert.deepEqual(runtime.pointerMove({ pointerId: 1, clientX: 150, clientY: 120 }).state.selectionArea, {
    x: 4,
    y: 4,
    width: 146,
    height: 116,
  });
  assert.deepEqual(runtime.pointerUp({ pointerId: 1, clientX: 150, clientY: 120 }).commands, [
    { type: "selection.set", ids: [baseDocument.elements[0].id], mode: "replace" },
  ]);

  assert.deepEqual(runtime.pointerDown({ pointerId: 2, clientX: 4, clientY: 4, button: 0 }).commands, []);
  assert.deepEqual(runtime.pointerUp({ pointerId: 2, clientX: 4, clientY: 4 }).commands, [
    { type: "selection.clear" },
  ]);
});

test("select marquee can extend the existing selection", () => {
  const runtime = createWhiteboardInputRuntime({
    document: baseDocument,
    tool: "select",
    viewport: { x: 0, y: 0, zoom: 1 },
    selection: { ids: [baseDocument.elements[0].id] },
  });

  runtime.pointerDown({ pointerId: 1, clientX: 170, clientY: 20, button: 0, shiftKey: true });
  const result = runtime.pointerUp({ pointerId: 1, clientX: 320, clientY: 96 });

  assert.deepEqual(result.commands, [
    {
      type: "selection.set",
      ids: [baseDocument.elements[0].id, baseDocument.elements[1].id],
      mode: "replace",
    },
  ]);
});

test("select tool expands grouped elements for movement", () => {
  const groupId = makeGroupId("group-alpha");
  const groupedDocument = createWhiteboardDocument({
    id: "grouped-input-board",
    elements: [
      createRectangleElement({
        id: "group-a",
        groupId,
        x: 20,
        y: 20,
        width: 60,
        height: 60,
        style: { fill: "#ffffff", stroke: "#111827" },
        createdAt: timestamp,
      }),
      createRectangleElement({
        id: "group-b",
        groupId,
        x: 100,
        y: 20,
        width: 60,
        height: 60,
        style: { fill: "#ffffff", stroke: "#111827" },
        createdAt: timestamp,
      }),
    ],
    groups: [
      {
        id: groupId,
        name: "Input group",
        elementIds: ["group-a", "group-b"].map((id) => makeElementId(id)),
        createdAt: timestamp,
        updatedAt: timestamp,
      },
    ],
    createdAt: timestamp,
    updatedAt: timestamp,
  });
  const runtime = createWhiteboardInputRuntime({
    document: groupedDocument,
    tool: "select",
    viewport: { x: 0, y: 0, zoom: 1 },
    selection: { ids: [] },
  });

  assert.deepEqual(runtime.pointerDown({ pointerId: 1, clientX: 40, clientY: 40, button: 0 }).commands, [
    { type: "selection.set", ids: groupedDocument.elements.map((element) => element.id), mode: "replace" },
  ]);

  const moveCommands = runtime.pointerMove({ pointerId: 1, clientX: 52, clientY: 48 }).commands;

  assert.deepEqual(
    moveCommands.map((command) => (command.type === "element.update" ? [command.id, command.patch] : null)),
    [
      ["group-a", { x: 32, y: 28 }],
      ["group-b", { x: 112, y: 28 }],
    ],
  );
});

test("select tool expands framed elements for movement", () => {
  const framedDocument = createWhiteboardDocument({
    id: "framed-input-board",
    elements: [
      createRectangleElement({
        id: "frame",
        role: "frame",
        x: 20,
        y: 20,
        width: 300,
        height: 200,
        style: { fill: "transparent", stroke: "#71717a" },
        createdAt: timestamp,
      }),
      createRectangleElement({
        id: "frame-card",
        x: 88,
        y: 72,
        width: 80,
        height: 60,
        metadata: { frameId: "frame" },
        style: { fill: "#ffffff", stroke: "#111827" },
        createdAt: timestamp,
      }),
      createRectangleElement({
        id: "locked-child",
        x: 188,
        y: 72,
        width: 80,
        height: 60,
        locked: true,
        metadata: { frameId: "frame" },
        style: { fill: "#ffffff", stroke: "#111827" },
        createdAt: timestamp,
      }),
      createRectangleElement({
        id: "hidden-child",
        x: 88,
        y: 152,
        width: 80,
        height: 60,
        hidden: true,
        metadata: { frameId: "frame" },
        style: { fill: "#ffffff", stroke: "#111827" },
        createdAt: timestamp,
      }),
      createRectangleElement({
        id: "outside",
        x: 420,
        y: 40,
        width: 80,
        height: 60,
        style: { fill: "#ffffff", stroke: "#111827" },
        createdAt: timestamp,
      }),
    ],
    createdAt: timestamp,
    updatedAt: timestamp,
  });
  const runtime = createWhiteboardInputRuntime({
    document: framedDocument,
    tool: "select",
    viewport: { x: 0, y: 0, zoom: 1 },
    selection: { ids: [] },
  });

  assert.deepEqual(runtime.pointerDown({ pointerId: 1, clientX: 22, clientY: 40, button: 0 }).commands, [
    {
      type: "selection.set",
      ids: [
        makeElementId("frame"),
        makeElementId("frame-card"),
        makeElementId("locked-child"),
        makeElementId("hidden-child"),
      ],
      mode: "replace",
    },
  ]);

  const moveCommands = runtime.pointerMove({ pointerId: 1, clientX: 34, clientY: 48 }).commands;

  assert.deepEqual(
    moveCommands.map((command) => (command.type === "element.update" ? [command.id, command.patch] : null)),
    [
      ["frame", { x: 32, y: 28 }],
      ["frame-card", { x: 100, y: 80 }],
    ],
  );
});

test("connector drawing emits endpoint bindings before reducer rerouting", () => {
  const runtime = createWhiteboardInputRuntime({
    document: baseDocument,
    tool: "arrow",
    viewport: { x: 0, y: 0, zoom: 1 },
    selection: { ids: [] },
    idFactory: () => "connector-1",
  });

  runtime.pointerDown({ pointerId: 1, clientX: 40, clientY: 40, button: 0 });
  const result = runtime.pointerUp({ pointerId: 1, clientX: 200, clientY: 48 });
  const addCommand = result.commands[0];

  assert.equal(addCommand?.type, "element.add");
  assert.equal(addCommand?.type === "element.add" ? addCommand.element.type : "", "arrow");
  assert.equal(
    addCommand?.type === "element.add" && addCommand.element.type === "arrow"
      ? addCommand.element.startBinding?.elementId
      : "",
    "rect-1",
  );
  assert.equal(
    addCommand?.type === "element.add" && addCommand.element.type === "arrow"
      ? addCommand.element.endBinding?.elementId
      : "",
    "text-1",
  );
  assert.deepEqual(
    addCommand?.type === "element.add" && addCommand.element.type === "arrow"
      ? addCommand.element.points
      : [],
    [
      { x: 40, y: 40 },
      { x: 200, y: 48 },
    ],
  );
  assert.equal(
    addCommand?.type === "element.add" && addCommand.element.type === "arrow"
      ? addCommand.element.metadata?.connectorRoute
      : "missing",
    undefined,
  );
});

test("locked and hidden elements are protected from pointer edits", () => {
  const document = createWhiteboardDocument({
    ...baseDocument,
    selection: ["rect-1", "text-1"],
    elements: [
      {
        ...baseDocument.elements[0],
        locked: true,
      },
      {
        ...baseDocument.elements[1],
        hidden: true,
      },
    ],
  });
  const runtime = createWhiteboardInputRuntime({
    document,
    tool: "select",
    viewport: { x: 0, y: 0, zoom: 1 },
    selection: { ids: document.selection },
  });

  assert.deepEqual(runtime.pointerDown({ pointerId: 1, clientX: 40, clientY: 40, button: 0 }).commands, [
    { type: "selection.set", ids: [document.elements[0].id], mode: "replace" },
  ]);
  assert.deepEqual(runtime.pointerMove({ pointerId: 1, clientX: 80, clientY: 88 }).commands, []);
  runtime.pointerUp({ pointerId: 1, clientX: 80, clientY: 88 });

  assert.deepEqual(runtime.pointerDown({ pointerId: 2, clientX: 200, clientY: 48, button: 0 }).commands, []);
  assert.deepEqual(runtime.pointerUp({ pointerId: 2, clientX: 200, clientY: 48 }).commands, [
    { type: "selection.clear" },
  ]);

  runtime.setTool("eraser");
  assert.deepEqual(runtime.pointerDown({ pointerId: 3, clientX: 40, clientY: 40, button: 0 }).commands, []);
  assert.deepEqual(runtime.pointerDown({ pointerId: 4, clientX: 200, clientY: 48, button: 0 }).commands, []);
  assert.deepEqual(runtime.keyDown({ key: "a", ctrlKey: true }).commands, [
    { type: "selection.set", ids: [document.elements[0].id] },
  ]);

  const marqueeRuntime = createWhiteboardInputRuntime({
    document,
    tool: "select",
    viewport: { x: 0, y: 0, zoom: 1 },
    selection: { ids: [] },
  });
  marqueeRuntime.pointerDown({ pointerId: 5, clientX: 0, clientY: 0, button: 0 });
  assert.deepEqual(marqueeRuntime.pointerUp({ pointerId: 5, clientX: 160, clientY: 120 }).commands, [
    { type: "selection.set", ids: [document.elements[0].id], mode: "replace" },
  ]);
});

test("snap-to-grid affects drawing, movement, and resize patches", () => {
  const snapDocument = createWhiteboardDocument({
    id: "snap-board",
    viewport: { x: 0, y: 0, zoom: 1 },
    metadata: { snapToGrid: true, gridSize: 24 },
    elements: [
      createRectangleElement({
        id: "snap-rect",
        x: 24,
        y: 24,
        width: 48,
        height: 48,
        style: { fill: "#ffffff", stroke: "#111827" },
        createdAt: timestamp,
      }),
    ],
    selection: ["snap-rect"],
    createdAt: timestamp,
    updatedAt: timestamp,
  });

  const drawRuntime = createWhiteboardInputRuntime({
    document: snapDocument,
    tool: "rectangle",
    idFactory: () => "snapped-shape",
  });
  drawRuntime.pointerDown({ pointerId: 1, clientX: 13, clientY: 14, button: 0 });
  drawRuntime.pointerMove({ pointerId: 1, clientX: 57, clientY: 62 });
  const drawCommand = drawRuntime.pointerUp({ pointerId: 1, clientX: 57, clientY: 62 }).commands[0];

  assert.equal(drawCommand?.type, "element.add");
  assert.deepEqual(
    drawCommand?.type === "element.add" && "x" in drawCommand.element
      ? {
          x: drawCommand.element.x,
          y: drawCommand.element.y,
          width: drawCommand.element.width,
          height: drawCommand.element.height,
        }
      : null,
    { x: 24, y: 24, width: 24, height: 48 },
  );

  const moveRuntime = createWhiteboardInputRuntime({
    document: snapDocument,
    tool: "select",
    selection: { ids: [snapDocument.elements[0].id] },
  });
  moveRuntime.pointerDown({ pointerId: 2, clientX: 30, clientY: 30, button: 0 });
  const moveCommand = moveRuntime.pointerMove({ pointerId: 2, clientX: 58, clientY: 58 }).commands[0];

  assert.equal(moveCommand?.type, "element.update");
  assert.deepEqual(moveCommand?.type === "element.update" ? moveCommand.patch : null, { x: 48, y: 48 });

  const resizeRuntime = createWhiteboardInputRuntime({
    document: snapDocument,
    tool: "select",
    selection: { ids: [snapDocument.elements[0].id] },
  });
  resizeRuntime.pointerDown({ pointerId: 3, clientX: 73, clientY: 73, button: 0 });
  const resizeCommand = resizeRuntime.pointerMove({ pointerId: 3, clientX: 91, clientY: 91 }).commands[0];

  assert.equal(resizeCommand?.type, "element.update");
  assert.deepEqual(
    resizeCommand?.type === "element.update"
      ? {
          x: resizeCommand.patch.x,
          y: resizeCommand.patch.y,
          width: resizeCommand.patch.width,
          height: resizeCommand.patch.height,
        }
      : null,
    { x: 24, y: 24, width: 72, height: 72 },
  );
});

test("setDocumentState refreshes the document used for hit testing", () => {
  const runtime = createWhiteboardInputRuntime({
    document: createWhiteboardDocument({ id: "empty-board", elements: [] }),
    tool: "select",
    selection: { ids: [] },
  });
  const nextDocument = createWhiteboardDocument({
    id: "next-board",
    elements: [
      createRectangleElement({
        id: "fresh-rect",
        x: 20,
        y: 20,
        width: 80,
        height: 60,
        style: { fill: "#ffffff", stroke: "#111827" },
        createdAt: timestamp,
      }),
    ],
    createdAt: timestamp,
    updatedAt: timestamp,
  });

  runtime.setDocumentState({ document: nextDocument });

  assert.deepEqual(runtime.pointerDown({ pointerId: 1, clientX: 30, clientY: 30, button: 0 }).commands, [
    { type: "selection.set", ids: [nextDocument.elements[0].id], mode: "replace" },
  ]);
});
