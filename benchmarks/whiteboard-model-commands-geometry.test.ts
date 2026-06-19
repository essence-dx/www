import assert from "node:assert/strict";
import { describe, it } from "node:test";

import {
  applyWhiteboardCommand,
  createWhiteboardDocument,
} from "../examples/whiteboard/server/whiteboard/commands.ts";
import { whiteboardCommandReducer } from "../examples/whiteboard/lib/whiteboard/commands.ts";
import {
  computeDocumentBounds,
  computeElementBounds,
  hitTestElements,
} from "../examples/whiteboard/server/whiteboard/geometry.ts";
import { connectorAnchorPoint } from "../examples/whiteboard/lib/whiteboard/connectors.ts";
import {
  createWhiteboardMinimapModel,
  minimapPointToViewport,
  minimapPointToWorld,
} from "../examples/whiteboard/lib/whiteboard/minimap.ts";
import {
  createWhiteboardPresentationModel,
  presentationCommandsForSlide,
} from "../examples/whiteboard/lib/whiteboard/presentation.ts";
import {
  createWhiteboardOutlineModel,
  outlineCommandsForItem,
} from "../examples/whiteboard/lib/whiteboard/outline.ts";
import {
  createWhiteboardMeasurementModel,
  measurementCommandsForFocus,
} from "../examples/whiteboard/lib/whiteboard/measurements.ts";
import type { WhiteboardElement } from "../examples/whiteboard/lib/whiteboard/persistence/schema.ts";
import {
  DEFAULT_WHITEBOARD_STYLE,
  createWhiteboardDocument as createModelWhiteboardDocument,
  makeElementId,
  makeGroupId,
} from "../examples/whiteboard/lib/whiteboard/model.ts";
import {
  createRectangleElement,
  createTextElement,
  getAreaSelectionIds,
  getDocumentContentBounds,
  getDocumentSelectionBounds,
} from "../examples/whiteboard/lib/whiteboard/scene.ts";
import { createWhiteboardStore } from "../examples/whiteboard/lib/stores/whiteboard-store.ts";

describe("whiteboard model and commands", () => {
  it("applies add, update, reorder, and delete commands without mutating the source document", () => {
    const createdAt = "2026-06-02T10:00:00.000Z";
    const updatedAt = "2026-06-02T10:00:05.000Z";
    const original = createWhiteboardDocument({
      id: "board-commands",
      name: "Command Board",
      now: () => createdAt,
    });
    const note: WhiteboardElement = {
      id: "note-1",
      type: "text",
      x: 24,
      y: 32,
      width: 180,
      height: 64,
      rotation: 0,
      text: "Ship local-first whiteboard storage",
      style: {
        ...DEFAULT_WHITEBOARD_STYLE,
        fill: "#ffffff",
        stroke: "#111827",
        strokeWidth: 1,
      },
      locked: false,
      hidden: false,
      textAlign: "left",
      verticalAlign: "top",
      createdAt,
      updatedAt: createdAt,
    };

    const added = applyWhiteboardCommand(original, {
      type: "element.add",
      element: note,
      now: () => updatedAt,
    });
    const changed = applyWhiteboardCommand(added, {
      type: "element.update",
      id: "note-1",
      patch: { x: 48, text: "Persist every command" },
      now: () => updatedAt,
    });
    const reordered = applyWhiteboardCommand(changed, {
      type: "element.reorder",
      ids: ["note-1"],
      intent: "front",
      now: () => updatedAt,
    });
    const removed = applyWhiteboardCommand(reordered, {
      type: "element.remove",
      ids: ["note-1"],
      now: () => updatedAt,
    });

    assert.equal(original.elements.length, 0);
    assert.equal(added.elements[0]?.text, "Ship local-first whiteboard storage");
    assert.equal(changed.elements[0]?.x, 48);
    assert.equal(changed.elements[0]?.text, "Persist every command");
    assert.equal(reordered.metadata?.revision, 3);
    assert.equal(removed.elements.length, 0);
    assert.equal(removed.metadata?.revision, 4);
    assert.equal(removed.updatedAt, updatedAt);
  });
});

describe("whiteboard geometry", () => {
  it("computes stable element and document bounds for vector and text elements", () => {
    const stroke: WhiteboardElement = {
      id: "stroke-1",
      type: "freehand",
      points: [
        { x: 10, y: 20 },
        { x: 30, y: 50 },
        { x: 80, y: 35 },
      ],
      style: {
        ...DEFAULT_WHITEBOARD_STYLE,
        fill: "none",
        stroke: "#2563eb",
        strokeWidth: 6,
      },
      locked: false,
      hidden: false,
      createdAt: "2026-06-02T00:00:00.000Z",
      updatedAt: "2026-06-02T00:00:00.000Z",
    };
    const rectangle: WhiteboardElement = {
      id: "rect-1",
      type: "rectangle",
      x: 120,
      y: 40,
      width: 160,
      height: 96,
      rotation: 0,
      style: {
        ...DEFAULT_WHITEBOARD_STYLE,
        fill: "#f8fafc",
        stroke: "#0f172a",
        strokeWidth: 2,
      },
      locked: false,
      hidden: false,
      createdAt: "2026-06-02T00:00:00.000Z",
      updatedAt: "2026-06-02T00:00:00.000Z",
    };

    assert.deepEqual(computeElementBounds(stroke), {
      x: 7,
      y: 17,
      width: 76,
      height: 36,
    });
    assert.deepEqual(computeElementBounds(rectangle), {
      x: 119,
      y: 39,
      width: 162,
      height: 98,
    });
    assert.deepEqual(computeDocumentBounds([stroke, rectangle], { padding: 10 }), {
      x: -3,
      y: 7,
      width: 294,
      height: 140,
    });
  });

  it("returns the topmost hit element by draw order", () => {
    const baseStyle = {
      ...DEFAULT_WHITEBOARD_STYLE,
      fill: "#ffffff",
      stroke: "#111827",
      strokeWidth: 1,
    };
    const elements: WhiteboardElement[] = [
      {
        id: "bottom",
        type: "rectangle",
        x: 0,
        y: 0,
        width: 100,
        height: 100,
        rotation: 0,
        style: baseStyle,
        locked: false,
        hidden: false,
        createdAt: "2026-06-02T00:00:00.000Z",
        updatedAt: "2026-06-02T00:00:00.000Z",
      },
      {
        id: "top",
        type: "ellipse",
        x: 40,
        y: 40,
        width: 100,
        height: 100,
        rotation: 0,
        style: baseStyle,
        locked: false,
        hidden: false,
        createdAt: "2026-06-02T00:00:00.000Z",
        updatedAt: "2026-06-02T00:00:00.000Z",
      },
    ];

    assert.equal(hitTestElements(elements, { x: 50, y: 50 })?.id, "top");
    assert.equal(hitTestElements(elements, { x: 12, y: 12 })?.id, "bottom");
    assert.equal(hitTestElements(elements, { x: 180, y: 180 }), null);
  });

  it("reroutes connector endpoint bindings through the canonical scene reducer", () => {
    const source = createRectangleElement({
      id: "source",
      x: 0,
      y: 0,
      width: 100,
      height: 100,
    });
    const target = createRectangleElement({
      id: "target",
      x: 240,
      y: 100,
      width: 80,
      height: 80,
    });
    const arrow: WhiteboardElement = {
      id: "arrow-1",
      type: "arrow",
      points: [
        { x: 0, y: 0 },
        { x: 100, y: 0 },
      ],
      style: {
        ...DEFAULT_WHITEBOARD_STYLE,
        strokeWidth: 4,
      },
      locked: false,
      hidden: false,
      createdAt: "2026-06-02T00:00:00.000Z",
      updatedAt: "2026-06-02T00:00:00.000Z",
    };
    const document = createModelWhiteboardDocument({
      id: "connector-binding-board",
      elements: [source, target, arrow],
    });

    const rebound = whiteboardCommandReducer(document, {
      type: "connector.bind",
      id: makeElementId("arrow-1"),
      startBinding: { elementId: makeElementId("source"), anchor: "right" },
      endBinding: { elementId: makeElementId("target"), anchor: "left" },
      now: "2026-06-02T01:00:00.000Z",
    });
    const connector = rebound.elements.find((element) => element.id === makeElementId("arrow-1"));

    assert.equal(connector?.type, "arrow");
    assert.equal(connector?.startBinding?.elementId, "source");
    assert.equal(connector?.endBinding?.anchor, "left");
    assert.deepEqual(connector?.points, [
      { x: 100, y: 50 },
      { x: 240, y: 140 },
    ]);
    assert.deepEqual(computeElementBounds(connector as WhiteboardElement), {
      x: 90,
      y: 40,
      width: 160,
      height: 110,
    });
  });

  it("computes fixed and auto connector anchors from box geometry", () => {
    const rotated = createRectangleElement({
      id: "rotated",
      x: 0,
      y: 0,
      width: 100,
      height: 40,
      rotation: 90,
    });
    const auto = createRectangleElement({
      id: "auto-box",
      x: 0,
      y: 0,
      width: 100,
      height: 100,
    });

    assert.deepEqual(connectorAnchorPoint(rotated, "right"), { x: 50, y: 70 });
    assert.deepEqual(connectorAnchorPoint(rotated, "top"), { x: 70, y: 20 });
    assert.deepEqual(connectorAnchorPoint(auto, "auto", { x: 200, y: 50 }), { x: 100, y: 50 });
    assert.deepEqual(connectorAnchorPoint(auto, "auto", { x: 50, y: -50 }), { x: 50, y: 0 });
  });

  it("derives area selection ids and nullable fit bounds from the canonical scene", () => {
    const groupId = makeGroupId("scene-group");
    const document = createModelWhiteboardDocument({
      id: "scene-helper-board",
      elements: [
        createRectangleElement({ id: "locked", x: 0, y: 0, width: 40, height: 40, locked: true }),
        createRectangleElement({ id: "group-a", groupId, x: 80, y: 0, width: 40, height: 40 }),
        createRectangleElement({ id: "group-b", groupId, x: 140, y: 0, width: 40, height: 40 }),
        createRectangleElement({ id: "hidden", x: 0, y: 90, width: 40, height: 40, hidden: true }),
      ],
      groups: [
        {
          id: groupId,
          name: "Scene group",
          elementIds: [makeElementId("group-a"), makeElementId("group-b")],
          createdAt: "2026-06-03T00:00:00.000Z",
          updatedAt: "2026-06-03T00:00:00.000Z",
        },
      ],
      selection: ["locked", "group-a"],
    });

    assert.deepEqual(getAreaSelectionIds(document, { x: -8, y: -8, width: 120, height: 64 }), [
      makeElementId("locked"),
      makeElementId("group-a"),
      makeElementId("group-b"),
    ]);
    assert.deepEqual(getDocumentSelectionBounds(document), { x: -1, y: -1, width: 122, height: 42 });
    assert.deepEqual(getDocumentContentBounds(document), { x: -1, y: -1, width: 182, height: 42 });
    assert.equal(getDocumentContentBounds(createModelWhiteboardDocument({ id: "empty-scene" })), null);
  });

  it("creates a minimap model from visible content bounds and selected element rectangles", () => {
    const document = createModelWhiteboardDocument({
      id: "minimap-board",
      viewport: { x: -100, y: -50, zoom: 1 },
      selection: ["mini-b", "hidden-far"],
      elements: [
        createRectangleElement({ id: "mini-a", x: 0, y: 0, width: 100, height: 50, style: { strokeWidth: 0 } }),
        createRectangleElement({ id: "mini-b", x: 300, y: 150, width: 100, height: 50, style: { strokeWidth: 0 } }),
        createRectangleElement({ id: "hidden-far", x: -1000, y: -1000, width: 3000, height: 3000, hidden: true, style: { strokeWidth: 0 } }),
      ],
    });

    const minimap = createWhiteboardMinimapModel(document, {
      size: { width: 200, height: 100 },
      stageSize: { width: 200, height: 100 },
      padding: 0,
    });

    assert.deepEqual(minimap.contentBounds, { x: 0, y: 0, width: 400, height: 200 });
    assert.equal(minimap.scale, 0.5);
    assert.deepEqual(minimap.elements.map(({ id, rect, selected }) => ({ id, rect, selected })), [
      { id: makeElementId("mini-a"), rect: { x: 0, y: 0, width: 50, height: 25 }, selected: false },
      { id: makeElementId("mini-b"), rect: { x: 150, y: 75, width: 50, height: 25 }, selected: true },
    ]);
    assert.deepEqual(minimap.viewportRect, { x: 50, y: 25, width: 100, height: 50 });
  });

  it("omits hidden elements from minimap content and returns an empty model for hidden-only boards", () => {
    const hiddenOnly = createModelWhiteboardDocument({
      id: "hidden-only-minimap",
      elements: [
        createRectangleElement({ id: "hidden-only", x: -100, y: -100, width: 200, height: 200, hidden: true }),
      ],
    });

    const minimap = createWhiteboardMinimapModel(hiddenOnly, {
      size: { width: 200, height: 100 },
      stageSize: { width: 200, height: 100 },
      padding: 0,
    });

    assert.equal(minimap.contentBounds, null);
    assert.deepEqual(minimap.elements, []);
    assert.equal(minimap.viewportRect, null);
  });

  it("clamps the minimap viewport rectangle and converts clicks into centered viewport commands", () => {
    const minimapElements = [
      createRectangleElement({ id: "mini-a", x: 0, y: 0, width: 100, height: 50, style: { strokeWidth: 0 } }),
      createRectangleElement({ id: "mini-b", x: 300, y: 150, width: 100, height: 50, style: { strokeWidth: 0 } }),
    ];
    const offTopLeft = createWhiteboardMinimapModel(createModelWhiteboardDocument({
      id: "minimap-off-top-left",
      viewport: { x: 200, y: 100, zoom: 1 },
      elements: minimapElements,
    }), {
      size: { width: 200, height: 100 },
      stageSize: { width: 200, height: 100 },
      padding: 0,
    });
    const offBottomRight = createWhiteboardMinimapModel(createModelWhiteboardDocument({
      id: "minimap-off-bottom-right",
      viewport: { x: -600, y: -300, zoom: 1 },
      elements: minimapElements,
    }), {
      size: { width: 200, height: 100 },
      stageSize: { width: 200, height: 100 },
      padding: 0,
    });
    const zoomed = createWhiteboardMinimapModel(createModelWhiteboardDocument({
      id: "minimap-navigation-board",
      viewport: { x: -100, y: -50, zoom: 2 },
      elements: minimapElements,
    }), {
      size: { width: 200, height: 100 },
      stageSize: { width: 200, height: 100 },
      padding: 0,
    });

    assert.deepEqual(offTopLeft.viewportRect, { x: 0, y: 0, width: 100, height: 50 });
    assert.deepEqual(offBottomRight.viewportRect, { x: 100, y: 50, width: 100, height: 50 });
    assert.deepEqual(minimapPointToWorld(zoomed, { x: 150, y: 75 }), { x: 300, y: 150 });
    assert.deepEqual(minimapPointToViewport(zoomed, { x: 150, y: 75 }), {
      x: -500,
      y: -250,
      zoom: 2,
    });
  });

  it("centers minimap click navigation through the current viewport zoom", () => {
    const document = createModelWhiteboardDocument({
      id: "minimap-click-board",
      viewport: { x: -100, y: -40, zoom: 2 },
      elements: [
        createRectangleElement({ id: "left", x: 0, y: 0, width: 100, height: 100 }),
        createRectangleElement({ id: "right", x: 300, y: 180, width: 100, height: 80 }),
      ],
    });
    const minimap = createWhiteboardMinimapModel(document, {
      size: { width: 220, height: 140 },
      stageSize: { width: 400, height: 240 },
      padding: 10,
    });
    const world = minimapPointToWorld(minimap, {
      x: minimap.size.width / 2,
      y: minimap.size.height / 2,
    });
    const viewport = minimapPointToViewport(minimap, {
      x: minimap.size.width / 2,
      y: minimap.size.height / 2,
    });

    assert.deepEqual(world, { x: 200, y: 130 });
    assert.deepEqual(viewport, { x: -200, y: -140, zoom: 2 });
  });

  it("routes minimap viewport navigation without creating undo history", () => {
    const store = createWhiteboardStore({
      document: createModelWhiteboardDocument({
        id: "minimap-store-board",
        viewport: { x: 0, y: 0, zoom: 1 },
        elements: [createRectangleElement({ id: "card", x: 0, y: 0, width: 100, height: 100 })],
      }),
    });

    store.actions.setViewport({ x: -120, y: -80 });

    assert.deepEqual(store.getDocument().viewport, { x: -120, y: -80, zoom: 1 });
    assert.equal(store.getState().history.past.length, 0);
    assert.equal(store.getState().canUndo, false);
  });

  it("derives selection measurements from visible selection bounds and selected item counts", () => {
    const document = createModelWhiteboardDocument({
      id: "measurement-selection-board",
      selection: ["card", "caption", "hidden-card"],
      elements: [
        createRectangleElement({
          id: "frame",
          role: "frame",
          x: 0,
          y: 0,
          width: 400,
          height: 240,
          style: { strokeWidth: 0 },
        }),
        createRectangleElement({
          id: "card",
          x: 40,
          y: 50,
          width: 100,
          height: 60,
          metadata: { frameId: "frame" },
          style: { strokeWidth: 0 },
        }),
        createTextElement({
          id: "caption",
          text: "Measured caption",
          x: 200,
          y: 80,
          width: 120,
          height: 48,
          locked: true,
          style: { strokeWidth: 0 },
        }),
        createRectangleElement({
          id: "hidden-card",
          x: 520,
          y: 100,
          width: 90,
          height: 70,
          hidden: true,
          style: { strokeWidth: 0 },
        }),
      ],
    });

    const measurements = createWhiteboardMeasurementModel(document, { width: 1200, height: 720 });

    assert.equal(measurements.subject, "selection");
    assert.equal(measurements.label, "3 selected");
    assert.deepEqual(measurements.bounds, { x: 40, y: 50, width: 280, height: 78 });
    assert.deepEqual(measurements.counts, {
      items: 3,
      hidden: 1,
      locked: 1,
      framed: 1,
      frames: 0,
      connectors: 0,
      images: 0,
      text: 1,
    });
    assert.equal(measurements.canFocus, true);
    assert.ok((measurements.viewport?.zoom ?? 0) > 0);
  });

  it("falls back to document measurements when no element is selected", () => {
    const document = createModelWhiteboardDocument({
      id: "measurement-document-board",
      elements: [
        createRectangleElement({
          id: "frame",
          role: "frame",
          x: 0,
          y: 0,
          width: 400,
          height: 240,
          style: { strokeWidth: 0 },
        }),
        createRectangleElement({
          id: "child",
          x: 40,
          y: 50,
          width: 100,
          height: 60,
          metadata: { frameId: "frame" },
          style: { strokeWidth: 0 },
        }),
        createTextElement({
          id: "hidden-note",
          text: "Hidden note",
          x: 700,
          y: 700,
          width: 120,
          height: 80,
          hidden: true,
          style: { strokeWidth: 0 },
        }),
      ],
    });

    const measurements = createWhiteboardMeasurementModel(document, { width: 1200, height: 720 });

    assert.equal(measurements.subject, "document");
    assert.equal(measurements.label, "Board contents");
    assert.deepEqual(measurements.bounds, { x: 0, y: 0, width: 400, height: 240 });
    assert.deepEqual(measurements.counts, {
      items: 3,
      hidden: 1,
      locked: 0,
      framed: 1,
      frames: 1,
      connectors: 0,
      images: 0,
      text: 1,
    });
  });

  it("reports empty measurements for an empty board", () => {
    const measurements = createWhiteboardMeasurementModel(
      createModelWhiteboardDocument({ id: "measurement-empty-board" }),
      { width: 800, height: 480 },
    );

    assert.equal(measurements.subject, "empty");
    assert.equal(measurements.label, "Empty board");
    assert.equal(measurements.bounds, null);
    assert.equal(measurements.viewport, null);
    assert.equal(measurements.canFocus, false);
    assert.deepEqual(measurementCommandsForFocus(measurements), []);
  });

  it("focuses measured bounds through a non-undoable viewport command", () => {
    const store = createWhiteboardStore({
      document: createModelWhiteboardDocument({
        id: "measurement-focus-board",
        elements: [
          createRectangleElement({
            id: "target",
            x: 320,
            y: 180,
            width: 140,
            height: 90,
            style: { strokeWidth: 0 },
          }),
        ],
      }),
    });
    const measurements = createWhiteboardMeasurementModel(store.getDocument(), { width: 1200, height: 720 });
    const commands = measurementCommandsForFocus(measurements);

    assert.deepEqual(commands.map((command) => command.type), ["viewport.set"]);
    store.dispatchBatch(commands);

    assert.deepEqual(store.getDocument().viewport, measurements.viewport);
    assert.equal(store.getState().history.past.length, 0);
    assert.equal(store.getState().canUndo, false);
  });

  it("groups outline items by frame and exposes labels plus selected, hidden, and locked status", () => {
    const document = createModelWhiteboardDocument({
      id: "outline-board",
      selection: ["note", "locked-task"],
      elements: [
        createRectangleElement({
          id: "frame-a",
          role: "frame",
          name: "Sprint Frame",
          x: 0,
          y: 0,
          width: 400,
          height: 220,
        }),
        createTextElement({
          id: "note",
          text: "Outline note for launch",
          x: 40,
          y: 48,
          width: 160,
          height: 64,
          metadata: { frameId: "frame-a" },
        }),
        createRectangleElement({
          id: "hidden-card",
          name: "Hidden card",
          x: 240,
          y: 80,
          width: 80,
          height: 64,
          hidden: true,
          metadata: { frameId: "frame-a" },
        }),
        createRectangleElement({
          id: "locked-task",
          role: "checklist",
          x: 520,
          y: 0,
          width: 120,
          height: 80,
          locked: true,
        }),
      ],
    });

    const outline = createWhiteboardOutlineModel(document, { width: 1200, height: 720 });

    assert.equal(outline.empty, false);
    assert.equal(outline.itemCount, 4);
    assert.equal(outline.selectedItemCount, 2);
    assert.deepEqual(outline.sections.map(({ id, title, frameId, itemCount }) => ({
      id,
      title,
      frameId,
      itemCount,
    })), [
      { id: "frame:frame-a", title: "Sprint Frame", frameId: makeElementId("frame-a"), itemCount: 3 },
      { id: "unframed", title: "Unframed", frameId: null, itemCount: 1 },
    ]);
    assert.deepEqual(outline.sections[0]?.items.map((item) => ({
      id: item.id,
      label: item.label,
      role: item.role,
      frameId: item.frameId,
      selected: item.selected,
      hidden: item.hidden,
      locked: item.locked,
    })), [
      {
        id: makeElementId("frame-a"),
        label: "Sprint Frame",
        role: "frame",
        frameId: null,
        selected: false,
        hidden: false,
        locked: false,
      },
      {
        id: makeElementId("note"),
        label: "Outline note for launch",
        role: "text",
        frameId: makeElementId("frame-a"),
        selected: true,
        hidden: false,
        locked: false,
      },
      {
        id: makeElementId("hidden-card"),
        label: "Hidden card",
        role: "rectangle",
        frameId: makeElementId("frame-a"),
        selected: false,
        hidden: true,
        locked: false,
      },
    ]);
    assert.deepEqual(outline.sections[1]?.items.map((item) => ({
      id: item.id,
      label: item.label,
      role: item.role,
      selected: item.selected,
      hidden: item.hidden,
      locked: item.locked,
    })), [
      {
        id: makeElementId("locked-task"),
        label: "Checklist",
        role: "checklist",
        selected: true,
        hidden: false,
        locked: true,
      },
    ]);
  });

  it("keeps orphan frame references in the unframed outline section", () => {
    const document = createModelWhiteboardDocument({
      id: "outline-orphan-board",
      elements: [
        createRectangleElement({
          id: "orphan-card",
          x: 24,
          y: 32,
          width: 120,
          height: 64,
          metadata: { frameId: "missing-frame" },
        }),
      ],
    });

    const outline = createWhiteboardOutlineModel(document, { width: 800, height: 480 });

    assert.deepEqual(outline.sections.map((section) => section.id), ["unframed"]);
    assert.equal(outline.sections[0]?.items[0]?.id, makeElementId("orphan-card"));
    assert.equal(outline.sections[0]?.items[0]?.frameId, makeElementId("missing-frame"));
  });

  it("focuses outline items with selection and viewport commands without adding undo history", () => {
    const store = createWhiteboardStore({
      document: createModelWhiteboardDocument({
        id: "outline-focus-board",
        elements: [
          createRectangleElement({
            id: "target",
            x: 320,
            y: 180,
            width: 140,
            height: 90,
          }),
        ],
      }),
    });
    const outline = createWhiteboardOutlineModel(store.getDocument(), { width: 1200, height: 720 });
    const target = outline.sections
      .flatMap((section) => section.items)
      .find((item) => item.id === makeElementId("target"));

    assert.ok(target);
    const commands = outlineCommandsForItem(target);
    assert.deepEqual(commands.map((command) => command.type), ["selection.set", "viewport.set"]);

    store.dispatchBatch(commands);

    assert.deepEqual(store.getDocument().selection, [makeElementId("target")]);
    assert.deepEqual(store.getDocument().viewport, target.viewport);
    assert.equal(store.getState().history.past.length, 0);
    assert.equal(store.getState().canUndo, false);
  });

  it("derives frame presentation slides from visible frame membership", () => {
    const document = createModelWhiteboardDocument({
      id: "presentation-board",
      selection: ["child-a"],
      elements: [
        createRectangleElement({ id: "frame-a", role: "frame", name: "Intro", x: 0, y: 0, width: 400, height: 220 }),
        createRectangleElement({ id: "child-a", x: 40, y: 60, width: 120, height: 80, metadata: { frameId: "frame-a" } }),
        createRectangleElement({ id: "frame-b", role: "frame", name: "Plan", x: 520, y: 0, width: 360, height: 220 }),
        createRectangleElement({ id: "hidden-frame", role: "frame", hidden: true, x: 0, y: 320, width: 320, height: 180 }),
      ],
    });

    const presentation = createWhiteboardPresentationModel(document, { width: 1200, height: 720 });

    assert.equal(presentation.empty, false);
    assert.equal(presentation.currentIndex, 0);
    assert.equal(presentation.currentSlide?.id, makeElementId("frame-a"));
    assert.equal(presentation.previousSlide, null);
    assert.equal(presentation.nextSlide?.id, makeElementId("frame-b"));
    assert.deepEqual(presentation.slides.map((slide) => [slide.id, slide.title, slide.childCount, slide.selected]), [
      [makeElementId("frame-a"), "Intro", 1, true],
      [makeElementId("frame-b"), "Plan", 0, false],
    ]);
    assert.ok((presentation.currentSlide?.viewport.zoom ?? 0) > 0);
  });

  it("prefers selected frame for presentation current slide and reports empty no-frame boards", () => {
    const selectedFrame = createWhiteboardPresentationModel(createModelWhiteboardDocument({
      id: "presentation-selected-frame",
      selection: ["frame-b"],
      elements: [
        createRectangleElement({ id: "frame-a", role: "frame", x: 0, y: 0, width: 200, height: 160 }),
        createRectangleElement({ id: "frame-b", role: "frame", name: "Selected", x: 320, y: 0, width: 200, height: 160 }),
      ],
    }), { width: 800, height: 480 });
    const empty = createWhiteboardPresentationModel(
      createModelWhiteboardDocument({ id: "presentation-empty" }),
      { width: 800, height: 480 },
    );

    assert.equal(selectedFrame.currentIndex, 1);
    assert.equal(selectedFrame.currentSlide?.title, "Selected");
    assert.equal(selectedFrame.previousSlide?.id, makeElementId("frame-a"));
    assert.equal(selectedFrame.nextSlide, null);
    assert.equal(empty.empty, true);
    assert.equal(empty.currentIndex, -1);
    assert.deepEqual(empty.slides, []);
  });

  it("focuses presentation slides through non-undoable selection and viewport commands", () => {
    const store = createWhiteboardStore({
      document: createModelWhiteboardDocument({
        id: "presentation-store-board",
        elements: [
          createRectangleElement({ id: "frame-a", role: "frame", x: 0, y: 0, width: 300, height: 180 }),
          createRectangleElement({ id: "frame-b", role: "frame", x: 420, y: 0, width: 300, height: 180 }),
        ],
      }),
    });
    const presentation = createWhiteboardPresentationModel(store.getDocument(), { width: 900, height: 540 });
    const target = presentation.slides[1];

    assert.ok(target);
    store.dispatchBatch(presentationCommandsForSlide(target));

    assert.deepEqual(store.getDocument().selection, [makeElementId("frame-b")]);
    assert.deepEqual(store.getDocument().viewport, target.viewport);
    assert.equal(store.getState().history.past.length, 0);
    assert.equal(store.getState().canUndo, false);
  });
});
