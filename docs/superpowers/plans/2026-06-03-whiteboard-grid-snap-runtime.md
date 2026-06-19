# Whiteboard Grid Snap Runtime Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make the whiteboard grid controls affect source-owned drawing, movement, and resize behavior instead of only storing metadata.

**Architecture:** Add a focused grid helper module, expose a reducer-owned `grid.set` command, wire the document panel through the store action, and apply snapping inside the existing input runtime. Keep hit testing unsnapped so selection remains natural, and snap emitted element patches so reducers stay canonical.

**Tech Stack:** DX WWW TSX, TypeScript modules, Bun `node:test`.

---

### Task 1: Grid Settings Helpers

**Files:**
- Create: `examples/whiteboard/lib/whiteboard/grid.ts`

- [x] Add typed grid settings and metadata conversion.
- [x] Add point, rect, coordinate, and element-patch snapping helpers.

### Task 2: Reducer And UI Wiring

**Files:**
- Modify: `examples/whiteboard/lib/whiteboard/commands.ts`
- Modify: `examples/whiteboard/lib/stores/whiteboard-store.ts`
- Modify: `examples/whiteboard/components/whiteboard/document-panel.tsx`
- Modify: `examples/whiteboard/components/whiteboard/canvas-stage.tsx`

- [x] Add `grid.set` as an undoable source-owned command.
- [x] Add `whiteboardActions.setGridSettings`.
- [x] Route grid/snap toggles and grid size through `grid.set`.
- [x] Surface `data-whiteboard-grid-size` on the canvas stage.

### Task 3: Runtime Behavior

**Files:**
- Modify: `examples/whiteboard/lib/whiteboard/input/input-runtime.ts`

- [x] Snap draw start/end points for shape and connector tools.
- [x] Snap move and resize update patches before they reach reducers.
- [x] Keep freehand and hit-testing unsnapped.
- [x] Allow `setDocumentState()` to refresh the runtime document used for hit testing.

### Task 4: Verification

**Files:**
- Modify: `benchmarks/whiteboard-rich-commands.test.ts`
- Modify: `examples/whiteboard/lib/whiteboard/input/input-runtime.test.ts`
- Modify: `benchmarks/whiteboard-shell-contract.test.ts`

- [x] Test `grid.set` metadata updates.
- [x] Test snapped draw, move, and resize behavior.
- [x] Test refreshed document hit testing.
- [x] Run focused whiteboard tests and `dx check`.
- [x] Commit with a professional message and push `features`.
