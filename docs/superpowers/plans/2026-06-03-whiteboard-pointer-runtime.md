# Whiteboard Pointer Runtime Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Wire browser pointer and wheel events into the existing source-owned whiteboard input runtime so drawing, selection, movement, panning, wheel zoom/pan, and draft previews are real app behavior.

**Architecture:** Keep the input runtime in `lib/whiteboard/input` as the only pointer semantics engine. Add a small store-level controller that owns one runtime instance, dispatches emitted reducer commands, and synchronizes runtime document/tool/selection/viewport state after store changes. The TSX stage only adapts DOM events into typed runtime inputs and renders runtime draft/selection-area state; it does not implement whiteboard semantics.

**Tech Stack:** DX WWW TSX, source-owned whiteboard store, source-owned input runtime, Bun tests, `dx check` receipts.

---

## File Map

- Create: `examples/whiteboard/lib/stores/whiteboard-input-controller.ts`
  - Store-backed input controller, runtime synchronization, pointer/wheel/keyboard command dispatch.
- Create: `examples/whiteboard/lib/stores/whiteboard-input-controller.test.ts`
  - Focused controller tests proving browser-style pointer flows mutate the canonical store.
- Modify: `examples/whiteboard/components/whiteboard/canvas-stage.tsx`
  - Add typed pointer/wheel props and DOM event adaptation.
  - Pass runtime draft and selection-area state into the SVG preview.
- Modify: `examples/whiteboard/components/whiteboard/svg-stage-preview.tsx`
  - Render runtime draft elements and marquee selection area as ephemeral preview geometry.
- Modify: `examples/whiteboard/components/whiteboard/whiteboard-app.tsx`
  - Use the new input controller for pointer/wheel/keyboard events.
  - Pass controller state into `CanvasStage`.
- Modify: `benchmarks/whiteboard-shell-contract.test.ts`
  - Assert pointer runtime markers, controller imports, event handlers, and ephemeral preview markers.
- Modify: `examples/whiteboard/README.md`, `examples/whiteboard/CHANGELOG.md`, `examples/whiteboard/TODO.md`
  - Move pointer browser wiring into the feature surface while keeping browser-level receipt boundaries honest.

## Tasks

### Task 1: Store-Backed Input Controller

- [x] Create `examples/whiteboard/lib/stores/whiteboard-input-controller.ts`.
- [x] Export `createWhiteboardInputController(storeApi, options)` with `pointerDown`, `pointerMove`, `pointerUp`, `wheel`, `keyDown`, `snapshot`, `sync`, and `destroy`.
- [x] Dispatch every emitted `WhiteboardCommand` through the canonical store API.
- [x] After each command batch, synchronize runtime state from `storeApi.getDocument()`.
- [x] Subscribe to store changes so external toolbar/inspector commands refresh runtime hit testing.

### Task 2: Browser Event Adaptation

- [x] Add pointer/wheel handler props to `CanvasStage`.
- [x] Convert DOM `clientX`, `clientY`, `pointerId`, `button`, `shiftKey`, `deltaX`, `deltaY`, `ctrlKey`, and `metaKey` into typed runtime inputs.
- [x] Use pointer capture when available on pointer down, release it when available on pointer up.
- [x] Prevent default on handled interactions and left-button pointer starts so panning and drawing keep stable pointer capture.
- [x] Keep the existing keyboard handler path compatible.

### Task 3: Runtime Preview Rendering

- [x] Pass `inputState` from `WhiteboardApp` to `CanvasStage`.
- [x] Render `inputState.draft` through `SvgStagePreview` as an ephemeral element with `data-whiteboard-runtime-draft`.
- [x] Render `inputState.selectionArea` as an ephemeral rectangle with `data-whiteboard-selection-area-preview`.
- [x] Do not persist draft or marquee rectangles into the document model.

### Task 4: App Wiring

- [x] Import the singleton input controller in `whiteboard-app.tsx`.
- [x] Replace manual keyboard dispatch with `whiteboardInputController.keyDown`.
- [x] Add pointer/wheel handlers that call `whiteboardInputController.pointerDown`, `pointerMove`, `pointerUp`, and `wheel`.
- [x] Keep toolbar, inspector, document panel, library, import/export, and share workflows unchanged.

### Task 5: Tests And Contracts

- [x] Add controller tests for rectangle drawing, selected-element move, wheel pan/zoom, and external document sync.
- [x] Update shell contract tests for pointer controller import, CanvasStage handler props, DOM pointer handlers, wheel handler, and ephemeral preview markers.
- [x] Keep input-runtime unit tests focused on runtime semantics, not DOM adaptation.
- [x] Update the README verification command to include the controller test file.

### Task 6: Verification And Checkpoint

- [x] Run focused controller and shell tests.
- [x] Run the full whiteboard semantic Bun test bundle.
- [x] Run no-React and no `.js/.jsx/.cjs/.mjs` scans.
- [x] Run scoped `git diff --check`.
- [x] Run `dx check examples/whiteboard --json`.
- [x] Commit and push `Wire whiteboard pointer runtime`.
