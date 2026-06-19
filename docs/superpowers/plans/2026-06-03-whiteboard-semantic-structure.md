# Whiteboard Semantic Structure Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add real whiteboard semantics for grouping, text commits, element roles, and connector endpoint metadata without changing the WWW runtime stack or adding browser package dependencies.

**Architecture:** Keep the persisted document schema version stable and add optional typed metadata that older documents can ignore. Commands remain the single source of truth for undoable mutations. UI panels expose real reducer-backed operations only.

**Tech Stack:** DX WWW TSX, source-owned whiteboard store, typed command reducer, Bun/node test runner, `.dx` check receipts.

---

### Task 1: Group Semantics

**Files:**
- Modify: `examples/whiteboard/lib/whiteboard/model.ts`
- Modify: `examples/whiteboard/lib/whiteboard/commands.ts`
- Modify: `examples/whiteboard/lib/whiteboard/arrange.ts`
- Modify: `examples/whiteboard/lib/stores/whiteboard-store.ts`
- Test: `benchmarks/whiteboard-rich-commands.test.ts`

- [ ] Add a typed `WhiteboardGroup` structure to the document.
- [ ] Add undoable `group.create` and `group.remove` commands.
- [ ] Keep selection stable and avoid moving/deleting unrelated elements.
- [ ] Test deterministic group creation, ungrouping, and locked/hidden-safe behavior.

### Task 2: Element Role Semantics

**Files:**
- Modify: `examples/whiteboard/lib/whiteboard/model.ts`
- Modify: `examples/whiteboard/lib/whiteboard/element-factory.ts`
- Modify: `examples/whiteboard/lib/whiteboard/library.ts`
- Modify: `examples/whiteboard/lib/whiteboard/persistence/schema.ts`
- Test: `benchmarks/whiteboard-persistence.test.ts`

- [ ] Add optional `role` semantics for `frame`, `sticky-note`, `checklist`, and `label`.
- [ ] Keep renderer compatibility by lowering roles onto existing rectangle/text geometry.
- [ ] Preserve legacy `metadata.preset` while adding typed role fields.
- [ ] Test persistence normalization for role and metadata.

### Task 3: Connector Endpoint Metadata

**Files:**
- Modify: `examples/whiteboard/lib/whiteboard/model.ts`
- Modify: `examples/whiteboard/lib/whiteboard/commands.ts`
- Modify: `examples/whiteboard/lib/whiteboard/persistence/schema.ts`
- Test: `benchmarks/whiteboard-model-commands-geometry.test.ts`

- [ ] Add optional start/end endpoint targets to line and arrow elements.
- [ ] Add command support to update connector endpoints.
- [ ] Do not fake live rebinding; endpoint metadata is declarative proof for later routing.
- [ ] Test endpoint persistence and command updates.

### Task 4: Text Commit Command

**Files:**
- Modify: `examples/whiteboard/lib/whiteboard/commands.ts`
- Modify: `examples/whiteboard/lib/stores/whiteboard-store.ts`
- Modify: `examples/whiteboard/components/whiteboard/text-editor-overlay.tsx`
- Modify: `examples/whiteboard/components/whiteboard/whiteboard-app.tsx`
- Test: `benchmarks/whiteboard-rich-commands.test.ts`

- [ ] Add `text.commit` command for text elements.
- [ ] Wire overlay blur/enter behavior through `whiteboardActions.commitText`.
- [ ] Keep active overlay behavior unchanged.
- [ ] Test that text commit is undoable and ignores non-text elements.

### Task 5: UI Contract And Documentation

**Files:**
- Modify: `examples/whiteboard/components/whiteboard/arrange-panel.tsx`
- Modify: `examples/whiteboard/components/whiteboard/inspector.tsx`
- Modify: `examples/whiteboard/styles/whiteboard.css`
- Modify: `examples/whiteboard/README.md`
- Modify: `examples/whiteboard/TODO.md`
- Modify: `examples/whiteboard/CHANGELOG.md`
- Test: `benchmarks/whiteboard-shell-contract.test.ts`

- [ ] Add real group/ungroup buttons and markers.
- [ ] Show selected element role and connector endpoint metadata in the inspector.
- [ ] Update docs to describe the added semantics honestly.
- [ ] Test shell markers and no React/runtime package dependency.

### Task 6: Focused Verification

**Files:**
- Run: `bun test` on focused whiteboard suites.
- Run: whiteboard no-React and no-JS source scans.
- Run: `dx check examples/whiteboard --json`.
- Commit: stage only whiteboard/docs/benchmark/check receipt files.

- [ ] Verify all focused tests pass.
- [ ] Verify no hand-authored `.js`, `.cjs`, or `.mjs` exists under `examples/whiteboard`.
- [ ] Verify no React imports or dummy hooks appear under `examples/whiteboard`.
- [ ] Commit and push a professional checkpoint.
