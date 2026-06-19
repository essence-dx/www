# Whiteboard Selection And Keyboard Workflows Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make the whiteboard editor faster to operate by adding source-owned marquee selection and command-backed keyboard workflows.

**Architecture:** Keep selection-area state ephemeral in the input runtime, not in the persisted document schema. Keyboard shortcuts emit existing reducer commands so history, groups, and source-owned tests stay the authority.

**Tech Stack:** DX WWW TSX, source-owned whiteboard input runtime, typed command reducer, Bun/node focused tests, `.dx` check receipts.

---

### Task 1: Marquee Selection Runtime

**Files:**
- Modify: `examples/whiteboard/lib/whiteboard/input/types.ts`
- Modify: `examples/whiteboard/lib/whiteboard/input/selection.ts`
- Modify: `examples/whiteboard/lib/whiteboard/input/input-runtime.ts`
- Test: `examples/whiteboard/lib/whiteboard/input/input-runtime.test.ts`

- [ ] Add an ephemeral selection-area drag state with start/current world points.
- [ ] Start area selection when the select tool drags on empty canvas.
- [ ] Select visible, unlocked elements whose bounds intersect the area on pointer-up.
- [ ] Preserve shift-extend behavior for area selection.

### Task 2: Keyboard Workflows

**Files:**
- Modify: `examples/whiteboard/lib/whiteboard/input/keyboard.ts`
- Test: `examples/whiteboard/lib/whiteboard/input/input-runtime.test.ts`

- [ ] Add arrow-key nudge using `element.translate`.
- [ ] Use Shift+Arrow for larger nudge distance.
- [ ] Add Ctrl/Cmd+D duplicate selection.
- [ ] Add Ctrl/Cmd+G group selection.
- [ ] Add Ctrl/Cmd+Shift+G ungroup selection.
- [ ] Add Ctrl/Cmd+L lock and Ctrl/Cmd+Shift+L unlock.
- [ ] Add Ctrl/Cmd+H hide and Ctrl/Cmd+Shift+H show.

### Task 3: Shell Contract And Docs

**Files:**
- Modify: `examples/whiteboard/components/whiteboard/canvas-stage.tsx`
- Modify: `examples/whiteboard/components/whiteboard/status-bar.tsx`
- Modify: `examples/whiteboard/README.md`
- Modify: `examples/whiteboard/CHANGELOG.md`
- Modify: `examples/whiteboard/TODO.md`
- Test: `benchmarks/whiteboard-shell-contract.test.ts`

- [ ] Add honest source markers for selection-area and keyboard workflows.
- [ ] Update docs without claiming browser E2E proof.
- [ ] Keep wording clear that selection-area state is runtime-only.

### Task 4: Verification And Check Receipt

**Files:**
- Run focused whiteboard tests.
- Run no-React and no-JS source scans.
- Run `dx check examples/whiteboard --json`.
- Commit only whiteboard-focused files and receipts.

- [ ] Verify focused tests pass.
- [ ] Verify no React runtime or dummy hooks appear under `examples/whiteboard`.
- [ ] Verify no `.js`, `.cjs`, or `.mjs` source files appear under `examples/whiteboard`.
- [ ] Commit and push a professional checkpoint.
