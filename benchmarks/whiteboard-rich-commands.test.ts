import assert from "node:assert/strict";
import { describe, it } from "node:test";

import { whiteboardCommandReducer } from "../examples/whiteboard/lib/whiteboard/commands.ts";
import {
  WHITEBOARD_LIBRARY_TEMPLATE_PRESETS,
  createLibraryPresetElements,
  type WhiteboardLibraryPresetId,
} from "../examples/whiteboard/lib/whiteboard/library.ts";
import {
  createWhiteboardHistory,
  pushWhiteboardCommand,
  redoWhiteboard,
  undoWhiteboard,
} from "../examples/whiteboard/lib/whiteboard/history.ts";
import {
  DEFAULT_WHITEBOARD_STYLE,
  createWhiteboardDocument,
  makeElementId,
  type WhiteboardDocument,
  type WhiteboardElement,
  type WhiteboardPoint,
} from "../examples/whiteboard/lib/whiteboard/model.ts";
import {
  createImageElement,
  createRectangleElement,
  createTextElement,
} from "../examples/whiteboard/lib/whiteboard/scene.ts";

const NOW = "2026-06-03T00:00:00.000Z";
const EMBEDDED_IMAGE_SRC =
  "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 320 200'%3E%3Crect width='320' height='200' fill='%23111827'/%3E%3C/svg%3E";
const NEXT_EMBEDDED_IMAGE_SRC =
  "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAFgwJ/lZXTWQAAAABJRU5ErkJggg==";

describe("whiteboard rich commands", () => {
  it("duplicates selected elements with deterministic ids, offset positions, and selected copies", () => {
    const document = board([
      createRectangleElement({
        id: "card",
        x: 10,
        y: 20,
        width: 100,
        height: 80,
        name: "Card",
      }),
    ]);

    const next = whiteboardCommandReducer(document, {
      type: "element.duplicate",
      ids: [makeElementId("card")],
      offset: { x: 24, y: 12 },
      now: NOW,
    });

    assert.equal(next.elements.length, 2);
    assert.equal(next.elements[1]?.id, makeElementId("card-copy-1"));
    assert.equal(next.elements[1]?.name, "Card copy");
    assert.equal("x" in next.elements[1] ? next.elements[1].x : null, 34);
    assert.equal("y" in next.elements[1] ? next.elements[1].y : null, 32);
    assert.deepEqual(next.selection, [makeElementId("card-copy-1")]);
  });

  it("prevents locked elements from update, translate, and remove while allowing unlock", () => {
    const document = board([
      createRectangleElement({
        id: "locked",
        x: 10,
        y: 20,
        width: 100,
        height: 80,
        locked: true,
      }),
      createRectangleElement({
        id: "open",
        x: 200,
        y: 20,
        width: 100,
        height: 80,
      }),
    ]);

    const moved = whiteboardCommandReducer(document, {
      type: "element.translate",
      ids: [makeElementId("locked"), makeElementId("open")],
      delta: { x: 10, y: 10 },
      now: NOW,
    });
    const removed = whiteboardCommandReducer(moved, {
      type: "element.remove",
      ids: [makeElementId("locked"), makeElementId("open")],
      now: NOW,
    });
    const unlocked = whiteboardCommandReducer(removed, {
      type: "element.unlock",
      ids: [makeElementId("locked")],
      now: NOW,
    });

    assert.equal("x" in moved.elements[0] ? moved.elements[0].x : null, 10);
    assert.equal("x" in moved.elements[1] ? moved.elements[1].x : null, 210);
    assert.deepEqual(removed.elements.map((element) => element.id), [makeElementId("locked")]);
    assert.equal(removed.elements[0]?.locked, true);
    assert.equal(unlocked.elements[0]?.locked, false);
  });

  it("hides and shows elements without deleting them and clears hidden selection", () => {
    const document = createWhiteboardDocument({
      id: "hide-board",
      elements: [
        createRectangleElement({
          id: "target",
          x: 0,
          y: 0,
          width: 100,
          height: 100,
        }),
      ],
      selection: ["target"],
    });

    const hidden = whiteboardCommandReducer(document, {
      type: "element.hide",
      ids: [makeElementId("target")],
      now: NOW,
    });
    const movedWhileHidden = whiteboardCommandReducer(hidden, {
      type: "element.translate",
      ids: [makeElementId("target")],
      delta: { x: 24, y: 24 },
      now: NOW,
    });
    const shown = whiteboardCommandReducer(hidden, {
      type: "element.show",
      ids: [makeElementId("target")],
      now: NOW,
    });

    assert.equal(hidden.elements[0]?.hidden, true);
    assert.deepEqual(hidden.selection, []);
    assert.equal("x" in movedWhileHidden.elements[0] ? movedWhileHidden.elements[0].x : null, 0);
    assert.equal(shown.elements[0]?.hidden, false);
    assert.equal(shown.elements.length, 1);
  });

  it("aligns and distributes selected elements using computed bounds", () => {
    const document = board([
      createRectangleElement({ id: "a", x: 10, y: 10, width: 40, height: 40 }),
      createRectangleElement({ id: "b", x: 120, y: 40, width: 40, height: 40 }),
      createRectangleElement({ id: "c", x: 260, y: 90, width: 40, height: 40 }),
    ]);

    const aligned = whiteboardCommandReducer(document, {
      type: "element.align",
      ids: [makeElementId("a"), makeElementId("b"), makeElementId("c")],
      alignment: "top",
      now: NOW,
    });
    const distributed = whiteboardCommandReducer(aligned, {
      type: "element.distribute",
      ids: [makeElementId("a"), makeElementId("b"), makeElementId("c")],
      distribution: "horizontal",
      now: NOW,
    });

    assert.deepEqual(boxPositions(aligned), [
      ["a", 10, 10],
      ["b", 120, 10],
      ["c", 260, 10],
    ]);
    assert.deepEqual(boxPositions(distributed), [
      ["a", 10, 10],
      ["b", 135, 10],
      ["c", 260, 10],
    ]);
  });

  it("updates grid and snap metadata through source-owned commands", () => {
    const document = createWhiteboardDocument({
      id: "metadata-board",
      metadata: { gridVisible: true, snapToGrid: false },
    });

    const next = whiteboardCommandReducer(document, {
      type: "grid.set",
      settings: { gridVisible: false, snapToGrid: true, gridSize: 32 },
      now: NOW,
    });

    assert.equal(next.metadata?.gridVisible, false);
    assert.equal(next.metadata?.snapToGrid, true);
    assert.equal(next.metadata?.gridSize, 32);
    assert.equal(next.updatedAt, NOW);
  });

  it("creates groups with deterministic ids, metadata, and editable membership", () => {
    const document = board([
      createRectangleElement({ id: "a", x: 0, y: 0, width: 40, height: 40 }),
      createRectangleElement({ id: "b", x: 60, y: 0, width: 40, height: 40 }),
      createRectangleElement({ id: "locked", x: 120, y: 0, width: 40, height: 40, locked: true }),
    ]);

    const grouped = whiteboardCommandReducer(document, {
      type: "group.create",
      ids: [makeElementId("a"), makeElementId("b"), makeElementId("locked")],
      metadata: { purpose: "section" },
      now: NOW,
    });

    assert.equal(grouped.groups?.[0]?.id, "group-1");
    assert.deepEqual(grouped.groups?.[0]?.elementIds, [makeElementId("a"), makeElementId("b")]);
    assert.equal(grouped.groups?.[0]?.metadata?.purpose, "section");
    assert.equal(grouped.elements[0]?.groupId, "group-1");
    assert.equal(grouped.elements[1]?.groupId, "group-1");
    assert.equal(grouped.elements[2]?.groupId, undefined);
    assert.deepEqual(grouped.selection, [makeElementId("a"), makeElementId("b")]);
  });

  it("updates and removes groups without touching unrelated groups", () => {
    const first = whiteboardCommandReducer(
      board([
        createRectangleElement({ id: "a", x: 0, y: 0, width: 40, height: 40 }),
        createRectangleElement({ id: "b", x: 60, y: 0, width: 40, height: 40 }),
        createRectangleElement({ id: "c", x: 120, y: 0, width: 40, height: 40 }),
        createRectangleElement({ id: "d", x: 180, y: 0, width: 40, height: 40 }),
      ]),
      {
        type: "group.create",
        id: "group-alpha",
        ids: [makeElementId("a"), makeElementId("b")],
        now: NOW,
      },
    );
    const second = whiteboardCommandReducer(first, {
      type: "group.create",
      id: "group-beta",
      ids: [makeElementId("c"), makeElementId("d")],
      now: NOW,
    });
    const annotated = whiteboardCommandReducer(second, {
      type: "group.metadata.set",
      id: "group-beta",
      metadata: { owner: "flow" },
      now: NOW,
    });
    const ungrouped = whiteboardCommandReducer(annotated, {
      type: "group.remove",
      elementIds: [makeElementId("c")],
      now: NOW,
    });

    assert.equal(annotated.groups?.find((group) => group.id === "group-beta")?.metadata?.owner, "flow");
    assert.deepEqual(ungrouped.groups?.map((group) => group.id), ["group-alpha"]);
    assert.equal(ungrouped.elements.find((element) => element.id === "c")?.groupId, undefined);
    assert.equal(ungrouped.elements.find((element) => element.id === "d")?.groupId, undefined);
    assert.equal(ungrouped.elements.find((element) => element.id === "a")?.groupId, "group-alpha");
  });

  it("expands grouped members for selection, translation, and removal commands", () => {
    const grouped = whiteboardCommandReducer(
      board([
        createRectangleElement({ id: "a", x: 0, y: 0, width: 40, height: 40 }),
        createRectangleElement({ id: "b", x: 60, y: 0, width: 40, height: 40 }),
        createRectangleElement({ id: "outside", x: 120, y: 0, width: 40, height: 40 }),
      ]),
      {
        type: "group.create",
        id: "group-alpha",
        ids: [makeElementId("a"), makeElementId("b")],
        now: NOW,
      },
    );
    const selected = whiteboardCommandReducer(grouped, {
      type: "selection.set",
      ids: [makeElementId("a")],
      now: NOW,
    });
    const moved = whiteboardCommandReducer(grouped, {
      type: "element.translate",
      ids: [makeElementId("a")],
      delta: { x: 12, y: 8 },
      now: NOW,
    });
    const removed = whiteboardCommandReducer(grouped, {
      type: "element.remove",
      ids: [makeElementId("a")],
      now: NOW,
    });

    assert.deepEqual(selected.selection, [makeElementId("a"), makeElementId("b")]);
    assert.deepEqual(boxPositions(moved), [
      ["a", 12, 8],
      ["b", 72, 8],
      ["outside", 120, 0],
    ]);
    assert.deepEqual(removed.elements.map((element) => element.id), [makeElementId("outside")]);
    assert.equal(removed.groups, undefined);
  });

  it("commits text through an undoable command and ignores non-text elements", () => {
    const document = board([
      createTextElement({
        id: "note",
        x: 0,
        y: 0,
        width: 160,
        height: 64,
        text: "Draft",
      }),
      createRectangleElement({ id: "box", x: 180, y: 0, width: 40, height: 40 }),
    ]);
    const history = createWhiteboardHistory(document);
    const changed = pushWhiteboardCommand(history, {
      type: "text.commit",
      id: makeElementId("note"),
      text: "Committed",
      now: NOW,
    });
    const ignored = pushWhiteboardCommand(changed, {
      type: "text.commit",
      id: makeElementId("box"),
      text: "Nope",
      now: NOW,
    });
    const undone = undoWhiteboard(ignored);
    const redone = redoWhiteboard(undone);

    assert.equal(changed.present.elements[0]?.type === "text" ? changed.present.elements[0].text : "", "Committed");
    assert.equal(ignored, changed);
    assert.equal(undone.present.elements[0]?.type === "text" ? undone.present.elements[0].text : "", "Draft");
    assert.equal(redone.present.elements[0]?.type === "text" ? redone.present.elements[0].text : "", "Committed");
  });

  it("inserts source-owned library presets through the command reducer", () => {
    const document = createWhiteboardDocument({ id: "library-board" });
    const next = whiteboardCommandReducer(document, {
      type: "library.insert",
      preset: "checklist",
      origin: { x: 64, y: 80 },
      idPrefix: "test-checklist",
      now: NOW,
    });

    assert.equal(next.elements.length, 2);
    assert.deepEqual(next.elements.map((element) => element.id), [
      makeElementId("test-checklist-card"),
      makeElementId("test-checklist-items"),
    ]);
    assert.deepEqual(next.selection, [
      makeElementId("test-checklist-card"),
      makeElementId("test-checklist-items"),
    ]);
  });

  it("inserts semantic sticky and frame presets through existing geometry types", () => {
    const document = createWhiteboardDocument({ id: "semantic-library-board" });
    const sticky = whiteboardCommandReducer(document, {
      type: "library.insert",
      preset: "sticky-note",
      idPrefix: "semantic-sticky",
      now: NOW,
    });
    const framed = whiteboardCommandReducer(sticky, {
      type: "library.insert",
      preset: "frame",
      idPrefix: "semantic-frame",
      now: NOW,
    });

    const note = framed.elements.find((element) => element.id === "semantic-sticky-note");
    const frame = framed.elements.find((element) => element.id === "semantic-frame-frame");

    assert.equal(note?.type, "text");
    assert.equal(note?.role, "sticky-note");
    assert.equal(frame?.type, "rectangle");
    assert.equal(frame?.role, "frame");
  });

  it("keeps advanced template preset registry counts aligned with source factories", () => {
    const expected: readonly [WhiteboardLibraryPresetId, number][] = [
      ["flowchart-basic", 11],
      ["kanban-board", 11],
      ["retrospective-board", 14],
      ["system-map", 15],
    ];

    assert.deepEqual(
      WHITEBOARD_LIBRARY_TEMPLATE_PRESETS.map((preset) => [preset.id, preset.elementCount]),
      expected,
    );

    for (const [preset, count] of expected) {
      assert.equal(
        createLibraryPresetElements({
          preset,
          idPrefix: `${preset}-count`,
          now: NOW,
        }).length,
        count,
      );
    }
  });

  it("inserts flowchart templates with deterministic ids and routed connector metadata", () => {
    const next = insertTemplate("flowchart-basic", "adv-flow", { x: 100, y: 120 });
    const expectedIds = [
      "adv-flow-start",
      "adv-flow-start-label",
      "adv-flow-decision",
      "adv-flow-decision-label",
      "adv-flow-approve",
      "adv-flow-approve-label",
      "adv-flow-revise",
      "adv-flow-revise-label",
      "adv-flow-start-to-decision",
      "adv-flow-decision-to-approve",
      "adv-flow-decision-to-revise",
    ];

    assert.deepEqual(elementIds(next.elements), expectedIds);
    assert.deepEqual(next.selection, expectedIds.map(makeElementId));
    assert.equal(next.elements.find((element) => element.id === "adv-flow-start")?.metadata?.template, "flowchart-basic");
    assert.equal(next.elements.find((element) => element.id === "adv-flow-decision")?.metadata?.templateRole, "decision");

    for (const connectorId of [
      "adv-flow-start-to-decision",
      "adv-flow-decision-to-approve",
      "adv-flow-decision-to-revise",
    ]) {
      const connector = connectorById(next, connectorId);

      assert.equal(connector?.metadata?.sourceOwned, true);
      assert.equal(connector?.metadata?.template, "flowchart-basic");
      assert.equal(connector?.metadata?.templateRole, "connector");
      assert.equal(connector?.metadata?.connectorRoute, "orthogonal");
    }
  });

  it("inserts kanban templates with frame-owned child metadata", () => {
    const next = insertTemplate("kanban-board", "kanban", { x: 40, y: 64 });
    const expectedIds = [
      "kanban-frame",
      "kanban-title",
      "kanban-todo-column",
      "kanban-todo-header",
      "kanban-todo-card",
      "kanban-doing-column",
      "kanban-doing-header",
      "kanban-doing-card",
      "kanban-done-column",
      "kanban-done-header",
      "kanban-done-card",
    ];

    assert.deepEqual(elementIds(next.elements), expectedIds);
    assert.deepEqual(next.selection, expectedIds.map(makeElementId));
    assert.equal(next.elements[0]?.role, "frame");
    assert.equal(next.elements[0]?.metadata?.template, "kanban-board");
    assert.equal(next.elements[0]?.metadata?.frameId, undefined);

    for (const element of next.elements.slice(1)) {
      assert.equal(element.metadata?.sourceOwned, true);
      assert.equal(element.metadata?.template, "kanban-board");
      assert.equal(element.metadata?.frameId, makeElementId("kanban-frame"));
    }

    assert.equal(next.elements.find((element) => element.id === "kanban-todo-card")?.role, "sticky-note");
    assert.equal(next.elements.find((element) => element.id === "kanban-doing-column")?.role, "shape");
  });

  it("inserts retrospective templates with four editable quadrants", () => {
    const next = insertTemplate("retrospective-board", "retro");
    const quadrantIds = ["went-well", "improve", "questions", "actions"];

    assert.equal(next.elements.length, 14);
    assert.equal(next.elements[0]?.id, makeElementId("retro-frame"));
    assert.equal(next.elements[1]?.id, makeElementId("retro-title"));

    for (const quadrantId of quadrantIds) {
      for (const suffix of ["zone", "label", "note"]) {
        const element = next.elements.find((item) =>
          item.id === makeElementId(`retro-${quadrantId}-${suffix}`),
        );

        assert.equal(element?.metadata?.sourceOwned, true);
        assert.equal(element?.metadata?.template, "retrospective-board");
        assert.equal(element?.metadata?.frameId, makeElementId("retro-frame"));
      }

      const note = next.elements.find((item) => item.id === makeElementId(`retro-${quadrantId}-note`));
      assert.equal(note?.type, "text");
      assert.equal(note?.role, "sticky-note");
      assert.equal(note?.type === "text" ? note.text : "", "Add note");
    }
  });

  it("inserts system map templates with bound source-owned connectors", () => {
    const next = insertTemplate("system-map", "system", { x: 10, y: 20 });
    const expectedIds = [
      "system-client",
      "system-client-label",
      "system-api",
      "system-api-label",
      "system-worker",
      "system-worker-label",
      "system-queue",
      "system-queue-label",
      "system-database",
      "system-database-label",
      "system-client-to-api",
      "system-api-to-worker",
      "system-api-to-queue",
      "system-worker-to-database",
      "system-queue-to-database",
    ];

    assert.deepEqual(elementIds(next.elements), expectedIds);
    assert.equal(next.elements.find((element) => element.id === "system-database")?.type, "ellipse");
    assert.equal(next.elements.find((element) => element.id === "system-api")?.type, "rectangle");

    const connector = connectorById(next, "system-client-to-api");
    assert.equal(connector?.type, "arrow");
    assert.deepEqual(connector?.points, [
      { x: 190, y: 162 },
      { x: 240, y: 162 },
      { x: 290, y: 162 },
    ]);
    assert.deepEqual(connector?.startBinding, {
      elementId: makeElementId("system-client"),
      anchor: "auto",
    });
    assert.deepEqual(connector?.endBinding, {
      elementId: makeElementId("system-api"),
      anchor: "auto",
    });
    assert.equal(connector?.metadata?.sourceOwned, true);
    assert.equal(connector?.metadata?.template, "system-map");
    assert.equal(connector?.metadata?.connectorRoute, "orthogonal");
  });

  it("keeps advanced template insertion undoable and restores selection on redo", () => {
    const history = createWhiteboardHistory(createWhiteboardDocument({ id: "template-history-board" }));
    const changed = pushWhiteboardCommand(history, {
      type: "library.insert",
      preset: "kanban-board",
      idPrefix: "undo-kanban",
      now: NOW,
    });
    const undone = undoWhiteboard(changed);
    const redone = redoWhiteboard(undone);
    const expectedSelection = [
      "undo-kanban-frame",
      "undo-kanban-title",
      "undo-kanban-todo-column",
      "undo-kanban-todo-header",
      "undo-kanban-todo-card",
      "undo-kanban-doing-column",
      "undo-kanban-doing-header",
      "undo-kanban-doing-card",
      "undo-kanban-done-column",
      "undo-kanban-done-header",
      "undo-kanban-done-card",
    ].map(makeElementId);

    assert.equal(changed.present.elements.length, 11);
    assert.equal(undone.present.elements.length, 0);
    assert.equal(redone.present.elements.length, 11);
    assert.deepEqual(redone.present.selection, expectedSelection);
  });

  it("assigns and clears frame membership while preserving existing metadata", () => {
    const document = board([
      createRectangleElement({
        id: "frame",
        role: "frame",
        x: 0,
        y: 0,
        width: 400,
        height: 260,
      }),
      createRectangleElement({
        id: "card",
        x: 40,
        y: 60,
        width: 120,
        height: 80,
        metadata: { kind: "note" },
      }),
      createRectangleElement({
        id: "locked",
        x: 180,
        y: 60,
        width: 120,
        height: 80,
        locked: true,
      }),
    ]);

    const assigned = whiteboardCommandReducer(document, {
      type: "frame.assign",
      frameId: makeElementId("frame"),
      ids: [makeElementId("frame"), makeElementId("card"), makeElementId("locked")],
      now: NOW,
    });
    const cleared = whiteboardCommandReducer(assigned, {
      type: "frame.clear",
      ids: [makeElementId("card")],
      now: NOW,
    });

    assert.equal(assigned.elements.find((element) => element.id === "card")?.metadata?.frameId, "frame");
    assert.equal(assigned.elements.find((element) => element.id === "card")?.metadata?.kind, "note");
    assert.equal(assigned.elements.find((element) => element.id === "frame")?.metadata?.frameId, undefined);
    assert.equal(assigned.elements.find((element) => element.id === "locked")?.metadata?.frameId, undefined);
    assert.equal(cleared.elements.find((element) => element.id === "card")?.metadata?.frameId, undefined);
    assert.equal(cleared.elements.find((element) => element.id === "card")?.metadata?.kind, "note");
  });

  it("moves frame members with the selected frame while preserving locked and hidden children", () => {
    const document = board([
      createRectangleElement({
        id: "frame",
        role: "frame",
        x: 0,
        y: 0,
        width: 400,
        height: 260,
      }),
      createRectangleElement({
        id: "card",
        x: 40,
        y: 60,
        width: 120,
        height: 80,
        metadata: { frameId: "frame" },
      }),
      createRectangleElement({
        id: "locked-child",
        x: 180,
        y: 60,
        width: 120,
        height: 80,
        locked: true,
        metadata: { frameId: "frame" },
      }),
      createRectangleElement({
        id: "hidden-child",
        x: 40,
        y: 180,
        width: 120,
        height: 80,
        hidden: true,
        metadata: { frameId: "frame" },
      }),
    ]);

    const moved = whiteboardCommandReducer(document, {
      type: "element.translate",
      ids: [makeElementId("frame")],
      delta: { x: 20, y: 10 },
      now: NOW,
    });

    assert.deepEqual(boxPositions(moved), [
      ["frame", 20, 10],
      ["card", 60, 70],
      ["locked-child", 180, 60],
      ["hidden-child", 40, 180],
    ]);
  });

  it("remaps frame membership when duplicating a frame with its children", () => {
    const framed = board([
      createRectangleElement({
        id: "frame",
        role: "frame",
        x: 0,
        y: 0,
        width: 400,
        height: 260,
      }),
      createRectangleElement({
        id: "card",
        x: 40,
        y: 60,
        width: 120,
        height: 80,
        metadata: { frameId: "frame" },
      }),
    ]);
    const copiedSet = whiteboardCommandReducer(framed, {
      type: "element.duplicate",
      ids: [makeElementId("frame"), makeElementId("card")],
      offset: { x: 40, y: 40 },
      now: NOW,
    });
    const copiedChild = copiedSet.elements.find((element) => element.id === "card-copy-1");
    const copiedFrame = copiedSet.elements.find((element) => element.id === "frame-copy-1");

    assert.equal(copiedFrame?.role, "frame");
    assert.equal(copiedChild?.metadata?.frameId, "frame-copy-1");

    const childOnly = whiteboardCommandReducer(framed, {
      type: "element.duplicate",
      ids: [makeElementId("card")],
      offset: { x: 40, y: 40 },
      now: NOW,
    });
    const childCopy = childOnly.elements.find((element) => element.id === "card-copy-1");

    assert.equal(childCopy?.metadata?.frameId, "frame");
  });

  it("duplicates a selected frame with its assigned members", () => {
    const framed = board([
      createRectangleElement({
        id: "frame",
        role: "frame",
        x: 0,
        y: 0,
        width: 400,
        height: 260,
      }),
      createRectangleElement({
        id: "card",
        x: 40,
        y: 60,
        width: 120,
        height: 80,
        metadata: { frameId: "frame" },
      }),
      createRectangleElement({
        id: "outside",
        x: 520,
        y: 60,
        width: 120,
        height: 80,
      }),
    ]);

    const copied = whiteboardCommandReducer(framed, {
      type: "element.duplicate",
      ids: [makeElementId("frame")],
      offset: { x: 40, y: 40 },
      now: NOW,
    });

    assert.deepEqual(copied.selection, [makeElementId("frame-copy-1"), makeElementId("card-copy-1")]);
    assert.equal(copied.elements.find((element) => element.id === "card-copy-1")?.metadata?.frameId, "frame-copy-1");
    assert.equal(copied.elements.find((element) => element.id === "outside-copy-1"), undefined);
  });

  it("inserts source-owned image presets through the command reducer", () => {
    const document = createWhiteboardDocument({ id: "image-library-board" });
    const next = whiteboardCommandReducer(document, {
      type: "library.insert",
      preset: "image",
      idPrefix: "semantic-image",
      origin: { x: 42, y: 64 },
      now: NOW,
    });
    const image = next.elements[0];

    assert.equal(image?.id, makeElementId("semantic-image-image"));
    assert.equal(image?.type, "image");
    assert.equal(image?.role, "image");
    assert.equal(image?.type === "image" ? image.alt : "", "Embedded whiteboard image");
    assert.match(image?.type === "image" ? image.src : "", /^data:image\/svg\+xml,/);
    assert.equal(image?.type === "image" ? image.naturalWidth : null, 320);
    assert.equal(image?.type === "image" ? image.naturalHeight : null, 200);
    assert.equal("x" in image ? image.x : null, 42);
    assert.deepEqual(next.selection, [makeElementId("semantic-image-image")]);
  });

  it("updates image sources through validated undoable commands", () => {
    const document = board([
      createImageElement({
        id: "image-card",
        x: 10,
        y: 20,
        width: 320,
        height: 200,
        src: EMBEDDED_IMAGE_SRC,
        alt: "Original image",
        naturalWidth: 320,
        naturalHeight: 200,
      }),
    ]);
    const history = createWhiteboardHistory(document);
    const changed = pushWhiteboardCommand(history, {
      type: "element.update",
      id: makeElementId("image-card"),
      patch: {
        src: NEXT_EMBEDDED_IMAGE_SRC,
        alt: "Updated image",
        naturalWidth: 1,
        naturalHeight: 1,
      },
      now: NOW,
    });
    const image = changed.present.elements[0];
    const undone = undoWhiteboard(changed);
    const redone = redoWhiteboard(undone);

    assert.equal(image?.type === "image" ? image.src : "", NEXT_EMBEDDED_IMAGE_SRC);
    assert.equal(image?.type === "image" ? image.alt : "", "Updated image");
    assert.equal(undone.present.elements[0]?.type === "image" ? undone.present.elements[0].alt : "", "Original image");
    assert.equal(redone.present.elements[0]?.type === "image" ? redone.present.elements[0].naturalWidth : null, 1);
    assert.throws(
      () =>
        whiteboardCommandReducer(document, {
          type: "element.update",
          id: makeElementId("image-card"),
          patch: { src: "https://example.com/not-owned.svg" },
          now: NOW,
        }),
      /data:image/,
    );
  });

  it("ignores image-only patches on non-image elements", () => {
    const document = board([
      createRectangleElement({
        id: "card",
        x: 10,
        y: 20,
        width: 120,
        height: 80,
      }),
    ]);

    const next = whiteboardCommandReducer(document, {
      type: "element.update",
      id: makeElementId("card"),
      patch: { src: EMBEDDED_IMAGE_SRC, alt: "Should not attach" },
      now: NOW,
    });
    const card = next.elements[0];

    assert.equal(card?.type, "rectangle");
    assert.equal("src" in (card ?? {}) ? (card as { src: unknown }).src : undefined, undefined);
    assert.equal("alt" in (card ?? {}) ? (card as { alt: unknown }).alt : undefined, undefined);
  });

  it("reroutes bound connector endpoints when targets move and resize", () => {
    const document = boundConnectorBoard();
    const moved = whiteboardCommandReducer(document, {
      type: "element.translate",
      ids: [makeElementId("source")],
      delta: { x: 40, y: 10 },
      now: NOW,
    });
    const resized = whiteboardCommandReducer(moved, {
      type: "element.update",
      id: makeElementId("target"),
      patch: { width: 160, height: 120 },
      now: NOW,
    });
    const movedConnector = connectorById(moved, "source-to-target");
    const resizedConnector = connectorById(resized, "source-to-target");

    assert.deepEqual(movedConnector?.points, [
      { x: 140, y: 60 },
      { x: 220, y: 50 },
    ]);
    assert.deepEqual(resizedConnector?.points, [
      { x: 140, y: 60 },
      { x: 220, y: 60 },
    ]);
  });

  it("cleans stale connector bindings when bound targets are hidden or removed", () => {
    const hidden = whiteboardCommandReducer(boundConnectorBoard(), {
      type: "element.hide",
      ids: [makeElementId("source")],
      now: NOW,
    });
    const removed = whiteboardCommandReducer(boundConnectorBoard(), {
      type: "element.remove",
      ids: [makeElementId("target")],
      now: NOW,
    });
    const hiddenConnector = connectorById(hidden, "source-to-target");
    const removedConnector = connectorById(removed, "source-to-target");

    assert.equal(hiddenConnector?.startBinding, undefined);
    assert.equal(hiddenConnector?.endBinding?.elementId, makeElementId("target"));
    assert.deepEqual(hiddenConnector?.points[0], { x: 100, y: 50 });
    assert.equal(removedConnector?.startBinding?.elementId, makeElementId("source"));
    assert.equal(removedConnector?.endBinding, undefined);
    assert.deepEqual(removedConnector?.points[1], { x: 220, y: 50 });
  });

  it("reroutes connectors after align and distribute commands", () => {
    const document = board([
      createRectangleElement({ id: "source", x: 0, y: 80, width: 80, height: 40 }),
      createRectangleElement({ id: "middle", x: 140, y: 20, width: 80, height: 40 }),
      createRectangleElement({ id: "target", x: 300, y: 160, width: 80, height: 40 }),
      connectorElement("source-to-middle", [{ x: 80, y: 100 }, { x: 140, y: 40 }], {
        startBinding: { elementId: makeElementId("source"), anchor: "right" },
        endBinding: { elementId: makeElementId("middle"), anchor: "left" },
      }),
    ]);
    const aligned = whiteboardCommandReducer(document, {
      type: "element.align",
      ids: [makeElementId("source"), makeElementId("middle"), makeElementId("target")],
      alignment: "top",
      now: NOW,
    });
    const distributed = whiteboardCommandReducer(aligned, {
      type: "element.distribute",
      ids: [makeElementId("source"), makeElementId("middle"), makeElementId("target")],
      distribution: "horizontal",
      now: NOW,
    });

    assert.deepEqual(connectorById(aligned, "source-to-middle")?.points, [
      { x: 80, y: 40 },
      { x: 140, y: 40 },
    ]);
    assert.deepEqual(connectorById(distributed, "source-to-middle")?.points, [
      { x: 80, y: 40 },
      { x: 150, y: 40 },
    ]);
  });

  it("preserves connector middle points and supports undoable live routing", () => {
    const document = board([
      createRectangleElement({ id: "source", x: 0, y: 0, width: 100, height: 100 }),
      createRectangleElement({ id: "target", x: 260, y: 0, width: 100, height: 100 }),
      connectorElement("elbow", [
        { x: 100, y: 50 },
        { x: 160, y: 140 },
        { x: 260, y: 50 },
      ], {
        startBinding: { elementId: makeElementId("source"), anchor: "right" },
        endBinding: { elementId: makeElementId("target"), anchor: "left" },
      }),
    ]);
    const changed = pushWhiteboardCommand(createWhiteboardHistory(document), {
      type: "element.translate",
      ids: [makeElementId("target")],
      delta: { x: 40, y: 20 },
      now: NOW,
    });
    const undone = undoWhiteboard(changed);
    const redone = redoWhiteboard(undone);

    assert.deepEqual(connectorById(changed.present, "elbow")?.points, [
      { x: 100, y: 50 },
      { x: 160, y: 140 },
      { x: 300, y: 70 },
    ]);
    assert.deepEqual(connectorById(undone.present, "elbow")?.points, [
      { x: 100, y: 50 },
      { x: 160, y: 140 },
      { x: 260, y: 50 },
    ]);
    assert.deepEqual(connectorById(redone.present, "elbow")?.points[2], { x: 300, y: 70 });
  });

  it("routes orthogonal connectors with deterministic elbow points when bound targets move", () => {
    const document = board([
      createRectangleElement({ id: "source", x: 0, y: 0, width: 100, height: 100 }),
      createRectangleElement({ id: "target", x: 220, y: 0, width: 100, height: 100 }),
      connectorElement("orthogonal", [{ x: 100, y: 50 }, { x: 220, y: 50 }], {
        startBinding: { elementId: makeElementId("source"), anchor: "right" },
        endBinding: { elementId: makeElementId("target"), anchor: "left" },
        metadata: { connectorRoute: "orthogonal" },
      }),
    ]);

    const moved = whiteboardCommandReducer(document, {
      type: "element.translate",
      ids: [makeElementId("target")],
      delta: { x: 40, y: 20 },
      now: NOW,
    });

    assert.deepEqual(connectorById(moved, "orthogonal")?.points, [
      { x: 100, y: 50 },
      { x: 180, y: 50 },
      { x: 180, y: 70 },
      { x: 260, y: 70 },
    ]);
  });

  it("reroutes orthogonal connectors when bound targets resize", () => {
    const document = board([
      createRectangleElement({ id: "source", x: 0, y: 0, width: 100, height: 100 }),
      createRectangleElement({ id: "target", x: 220, y: 0, width: 100, height: 100 }),
      connectorElement("orthogonal-resize", [{ x: 100, y: 50 }, { x: 220, y: 50 }], {
        startBinding: { elementId: makeElementId("source"), anchor: "right" },
        endBinding: { elementId: makeElementId("target"), anchor: "left" },
        metadata: { connectorRoute: "orthogonal" },
      }),
    ]);

    const resized = whiteboardCommandReducer(document, {
      type: "element.update",
      id: makeElementId("target"),
      patch: { width: 180, height: 160 },
      now: NOW,
    });

    assert.deepEqual(connectorById(resized, "orthogonal-resize")?.points, [
      { x: 100, y: 50 },
      { x: 160, y: 50 },
      { x: 160, y: 80 },
      { x: 220, y: 80 },
    ]);
  });

  it("keeps explicit straight connector middle points while rerouting endpoints", () => {
    const document = board([
      createRectangleElement({ id: "source", x: 0, y: 0, width: 100, height: 100 }),
      createRectangleElement({ id: "target", x: 260, y: 0, width: 100, height: 100 }),
      connectorElement("straight-elbow", [
        { x: 100, y: 50 },
        { x: 160, y: 140 },
        { x: 260, y: 50 },
      ], {
        startBinding: { elementId: makeElementId("source"), anchor: "right" },
        endBinding: { elementId: makeElementId("target"), anchor: "left" },
        metadata: { connectorRoute: "straight" },
      }),
    ]);

    const moved = whiteboardCommandReducer(document, {
      type: "element.translate",
      ids: [makeElementId("target")],
      delta: { x: 40, y: 20 },
      now: NOW,
    });

    assert.deepEqual(connectorById(moved, "straight-elbow")?.points, [
      { x: 100, y: 50 },
      { x: 160, y: 140 },
      { x: 300, y: 70 },
    ]);
  });

  it("reroutes immediately when connector route metadata changes to orthogonal", () => {
    const document = board([
      createRectangleElement({ id: "source", x: 0, y: 0, width: 100, height: 100 }),
      createRectangleElement({ id: "target", x: 220, y: 20, width: 100, height: 100 }),
      connectorElement("route-toggle", [
        { x: 100, y: 50 },
        { x: 160, y: 140 },
        { x: 220, y: 70 },
      ], {
        startBinding: { elementId: makeElementId("source"), anchor: "right" },
        endBinding: { elementId: makeElementId("target"), anchor: "left" },
        metadata: { label: "kept" },
      }),
    ]);

    const routed = whiteboardCommandReducer(document, {
      type: "element.update",
      id: makeElementId("route-toggle"),
      patch: { metadata: { label: "kept", connectorRoute: "orthogonal" } },
      now: NOW,
    });
    const connector = connectorById(routed, "route-toggle");

    assert.deepEqual(connector?.points, [
      { x: 100, y: 50 },
      { x: 160, y: 50 },
      { x: 160, y: 70 },
      { x: 220, y: 70 },
    ]);
    assert.equal(connector?.metadata?.connectorRoute, "orthogonal");
    assert.equal(connector?.metadata?.label, "kept");
  });

  it("remaps duplicated connector bindings only when their targets are duplicated too", () => {
    const copiedSet = whiteboardCommandReducer(boundConnectorBoard(), {
      type: "element.duplicate",
      ids: [makeElementId("source"), makeElementId("target"), makeElementId("source-to-target")],
      offset: { x: 20, y: 30 },
      now: NOW,
    });
    const copiedConnector = connectorById(copiedSet, "source-to-target-copy-1");
    const connectorOnly = whiteboardCommandReducer(boundConnectorBoard(), {
      type: "element.duplicate",
      ids: [makeElementId("source-to-target")],
      offset: { x: 20, y: 30 },
      now: NOW,
    });
    const unboundCopy = connectorById(connectorOnly, "source-to-target-copy-1");

    assert.equal(copiedConnector?.startBinding?.elementId, makeElementId("source-copy-1"));
    assert.equal(copiedConnector?.endBinding?.elementId, makeElementId("target-copy-1"));
    assert.deepEqual(copiedConnector?.points, [
      { x: 120, y: 80 },
      { x: 240, y: 80 },
    ]);
    assert.equal(unboundCopy?.startBinding, undefined);
    assert.equal(unboundCopy?.endBinding, undefined);
    assert.deepEqual(unboundCopy?.points, [
      { x: 120, y: 80 },
      { x: 240, y: 80 },
    ]);
  });
});

function board(elements: readonly WhiteboardElement[]) {
  return createWhiteboardDocument({
    id: "rich-command-board",
    elements,
  });
}

function insertTemplate(
  preset: WhiteboardLibraryPresetId,
  idPrefix: string,
  origin: WhiteboardPoint = { x: 0, y: 0 },
): WhiteboardDocument {
  return whiteboardCommandReducer(createWhiteboardDocument({ id: `${idPrefix}-board` }), {
    type: "library.insert",
    preset,
    idPrefix,
    origin,
    now: NOW,
  });
}

function elementIds(elements: readonly WhiteboardElement[]): readonly string[] {
  return elements.map((element) => element.id);
}

function boxPositions(document: ReturnType<typeof board>) {
  return document.elements.map((element) => [
    element.id,
    "x" in element ? element.x : null,
    "y" in element ? element.y : null,
  ]);
}

function boundConnectorBoard() {
  return board([
    createRectangleElement({ id: "source", x: 0, y: 0, width: 100, height: 100 }),
    createRectangleElement({ id: "target", x: 220, y: 0, width: 100, height: 100 }),
    connectorElement("source-to-target", [{ x: 100, y: 50 }, { x: 220, y: 50 }], {
      startBinding: { elementId: makeElementId("source"), anchor: "right" },
      endBinding: { elementId: makeElementId("target"), anchor: "left" },
    }),
  ]);
}

function connectorElement(
  id: string,
  points: readonly [WhiteboardPoint, WhiteboardPoint, ...WhiteboardPoint[]],
  bindings: {
    readonly startBinding?: Extract<WhiteboardElement, { type: "line" | "arrow" }>["startBinding"];
    readonly endBinding?: Extract<WhiteboardElement, { type: "line" | "arrow" }>["endBinding"];
    readonly metadata?: WhiteboardElement["metadata"];
  },
): WhiteboardElement {
  return {
    id: makeElementId(id),
    type: "arrow",
    role: "connector",
    points,
    startBinding: bindings.startBinding,
    endBinding: bindings.endBinding,
    startArrow: "none",
    endArrow: "triangle",
    locked: false,
    hidden: false,
    style: {
      ...DEFAULT_WHITEBOARD_STYLE,
      strokeWidth: 2,
    },
    createdAt: NOW,
    updatedAt: NOW,
    metadata: bindings.metadata,
  };
}

function connectorById(document: WhiteboardDocument, id: string) {
  const element = document.elements.find((item) => item.id === makeElementId(id));
  return element?.type === "line" || element?.type === "arrow" ? element : null;
}
