# Whiteboard Feature Expansion Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make `examples/whiteboard` substantially more feature-rich while keeping it WWW-native, source-owned, typed, maintainable, and free of React/runtime package dependencies.

**Architecture:** Keep the canonical document model in `examples/whiteboard/lib/whiteboard`. Add editor capabilities as reducer commands and scene helpers first, then expose them through small TSX panels. The visible shell must read from the same initialized store that actions mutate, not from an unrelated demo constant.

**Tech Stack:** DX WWW TSX, TypeScript modules, source-owned canvas/input/render helpers, Bun `node:test` verification, `.dx` receipts.

---

### Task 1: Store-Backed App Shell

**Files:**
- Create: `examples/whiteboard/lib/whiteboard/demo-document.ts`
- Modify: `examples/whiteboard/components/whiteboard/demo-document.ts`
- Modify: `examples/whiteboard/lib/stores/whiteboard-store.ts`
- Modify: `examples/whiteboard/components/whiteboard/whiteboard-app.tsx`
- Test: `benchmarks/whiteboard-shell-contract.test.ts`

- [x] Move the demo document factory into `lib/whiteboard/demo-document.ts`.
- [x] Re-export the factory from the component-level `demo-document.ts` for compatibility.
- [x] Initialize `whiteboardStore` with `createDemoWhiteboardDocument()`.
- [x] Render `WhiteboardApp` from `whiteboardActions.snapshot()` so the shell and actions share the same canonical document.
- [x] Extend the shell contract test to reject module-scope `createDemoWhiteboardDocument()` usage in `whiteboard-app.tsx`.

### Task 2: Rich Scene Commands

**Files:**
- Create: `examples/whiteboard/lib/whiteboard/arrange.ts`
- Create: `examples/whiteboard/lib/whiteboard/library.ts`
- Modify: `examples/whiteboard/lib/whiteboard/commands.ts`
- Modify: `examples/whiteboard/lib/whiteboard/scene.ts`
- Modify: `examples/whiteboard/lib/stores/whiteboard-store.ts`
- Test: `benchmarks/whiteboard-rich-commands.test.ts`

- [x] Add commands for document metadata updates, element duplicate, lock, unlock, hide, show, align, distribute, and library preset insertion.
- [x] Keep all mutations immutable and timestamped through existing scene helpers.
- [x] Use metadata for grouping/frame tags only when source targets are exact; do not claim multiplayer or external collaboration.
- [x] Add tests proving duplicate offsets, lock-protected movement, visibility filtering, alignment, distribution, and grid/snap metadata toggles.

### Task 3: Feature Panels

**Files:**
- Create: `examples/whiteboard/components/whiteboard/library-panel.tsx`
- Create: `examples/whiteboard/components/whiteboard/arrange-panel.tsx`
- Modify: `examples/whiteboard/components/whiteboard/document-panel.tsx`
- Modify: `examples/whiteboard/components/whiteboard/inspector.tsx`
- Modify: `examples/whiteboard/components/whiteboard/toolbar.tsx`
- Modify: `examples/whiteboard/components/whiteboard/whiteboard-app.tsx`
- Modify: `examples/whiteboard/styles/whiteboard.css`
- Test: `benchmarks/whiteboard-shell-contract.test.ts`

- [x] Add a library panel with sticky note, decision diamond, connector, checklist note, and frame-style presets.
- [x] Add arrange controls for align left/center/right/top/middle/bottom and distribute horizontally/vertically.
- [x] Add document controls for duplicate, lock/unlock, hide/show, bring forward/backward, grid, snap, and selection clearing.
- [x] Add inspector controls for text alignment, vertical alignment, line cap, precise position, size, rotation, and selected element name.
- [x] Ensure controls are professional TSX with accessible labels and no external UI packages.

### Task 4: Canvas And Status Proof

**Files:**
- Modify: `examples/whiteboard/components/whiteboard/canvas-stage.tsx`
- Modify: `examples/whiteboard/components/whiteboard/status-bar.tsx`
- Modify: `examples/whiteboard/lib/whiteboard/render/renderer.ts`
- Test: `examples/whiteboard/lib/whiteboard/render/renderer.test.ts`

- [x] Surface canvas metadata for grid, snap, selected ids, tool, revision, and library preset count.
- [x] Render locked/hidden status through layer rows and status bar text.
- [x] Keep hidden elements out of rendered canvas output.
- [x] Add renderer test coverage for hidden elements and status metadata.

### Task 5: Documentation And Receipts

**Files:**
- Modify: `examples/whiteboard/README.md`
- Modify: `examples/whiteboard/TODO.md`
- Modify: `examples/whiteboard/CHANGELOG.md`
- Update: `examples/whiteboard/.dx/**` through `dx check examples/whiteboard --json`

- [x] Document the richer feature surface honestly.
- [x] Remove outdated “not claimed yet” wording for features implemented in this pass.
- [x] Keep future-only items explicitly named as future work.
- [x] Refresh the whiteboard check receipt after focused tests pass.

### Task 6: Verification And Sync

**Files:**
- Commit only whiteboard-related paths and plan file.

- [x] Run `bun test ./benchmarks/whiteboard-shell-contract.test.ts ./benchmarks/whiteboard-model-commands-geometry.test.ts ./benchmarks/whiteboard-rich-commands.test.ts ./benchmarks/whiteboard-persistence.test.ts ./benchmarks/whiteboard-export.test.ts`.
- [x] Run `bun test examples/whiteboard/lib/whiteboard/render/renderer.test.ts examples/whiteboard/lib/whiteboard/input/input-runtime.test.ts`.
- [x] Run `rg -n '^[\"'']use client[\"'']|from [''\\\"]react|from [''\\\"]react/|from [''\\\"]react-dom|React\\.|use(State|Effect|Memo|Callback|Ref)' examples/whiteboard -g '*.ts' -g '*.tsx'` and expect no matches.
- [x] Run `git diff --check -- examples/whiteboard benchmarks/whiteboard-rich-commands.test.ts benchmarks/whiteboard-shell-contract.test.ts docs/superpowers/plans/2026-06-03-whiteboard-feature-expansion.md`.
- [x] Run `dx check examples/whiteboard --json`.
- [x] Commit with a professional message and push `features` to the configured remote.
