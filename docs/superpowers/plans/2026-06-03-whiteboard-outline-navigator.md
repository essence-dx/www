# Whiteboard Outline Navigator Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a source-owned board outline navigator that groups whiteboard elements by frame and focuses any item through existing commands.

**Architecture:** Derive outline sections from the canonical `WhiteboardDocument`. Frames become sections, frame children are read from `metadata.frameId`, and unframed elements become a separate section. Focusing an outline item dispatches existing non-undoable `selection.set` plus `viewport.set` commands; no new schema, hidden runtime, or external package is introduced.

**Tech Stack:** DX WWW TSX, TypeScript whiteboard model helpers, existing frame metadata, existing viewport fit math, focused Bun tests.

---

## File Map

- Create `examples/whiteboard/lib/whiteboard/outline.ts`: pure outline model and focus command helpers.
- Create `examples/whiteboard/components/whiteboard/outline-panel.tsx`: grouped outline UI.
- Modify `examples/whiteboard/components/whiteboard/whiteboard-app.tsx`: mount the outline panel.
- Modify `examples/whiteboard/styles/whiteboard.css`: compact outline styling.
- Modify `benchmarks/whiteboard-model-commands-geometry.test.ts`: prove grouping, labels, focus commands, hidden/locked status, and undo safety.
- Modify `benchmarks/whiteboard-shell-contract.test.ts`: prove shell markers and dispatch wiring.
- Modify `examples/whiteboard/README.md`, `examples/whiteboard/CHANGELOG.md`, `.dx/receipts/whiteboard/latest.json`, and `examples/whiteboard/TODO.md`: document exact source-level claim.

## Tasks

### Task 1: Pure Outline Model

- [x] Add `createWhiteboardOutlineModel(document, stageSize)`.
- [x] Return frame sections in document order plus one unframed section.
- [x] Each outline item includes id, label, type, role, hidden, locked, selected, frameId, bounds, and fitted viewport.
- [x] Prefer source labels from element name, text preview, role, and type.
- [x] Add `outlineCommandsForItem(item)` returning explicit `selection.set` with `mode: "replace"` plus `viewport.set`.

### Task 2: Outline Panel

- [x] Add `OutlinePanel` using the pure model.
- [x] Mount it in the side panel near presentation/document navigation.
- [x] Expose stable markers:
  - `data-whiteboard-outline="frame-grouped"`
  - `data-whiteboard-outline-section`
  - `data-whiteboard-outline-section-count`
  - `data-whiteboard-outline-item-id`
  - `data-whiteboard-outline-item-type`
  - `data-whiteboard-outline-frame-id`
  - `data-whiteboard-command="selection.set viewport.set"`
- [x] Dispatch `whiteboardActions.dispatchBatch(outlineCommandsForItem(item))`.

### Task 3: Focused Tests

- [x] Add tests for frame grouping, unframed grouping, text/name labels, hidden/locked markers, and selected state.
- [x] Add tests proving focus commands select and fit viewport without creating undo history.
- [x] Add shell-contract tests proving panel mount, markers, command dispatch, and no React runtime imports.

### Task 4: Docs And Receipt

- [x] Update README feature surface and proof modules.
- [x] Update CHANGELOG.
- [x] Update `.dx/receipts/whiteboard/latest.json` with outline navigator feature and markers.
- [x] Keep browser E2E/visual/server/collaboration proof unclaimed.

### Task 5: Verification And Commit

- [x] Run focused tests:

```powershell
bun test ./benchmarks/whiteboard-shell-contract.test.ts ./benchmarks/whiteboard-model-commands-geometry.test.ts
```

- [x] Run full whiteboard regression:

```powershell
bun test ./benchmarks/whiteboard-shell-contract.test.ts ./benchmarks/whiteboard-model-commands-geometry.test.ts ./benchmarks/whiteboard-rich-commands.test.ts ./benchmarks/whiteboard-persistence.test.ts ./benchmarks/whiteboard-file-workflows.test.ts ./benchmarks/whiteboard-export.test.ts ./examples/whiteboard/lib/stores/whiteboard-input-controller.test.ts ./examples/whiteboard/lib/whiteboard/input/input-runtime.test.ts ./examples/whiteboard/lib/whiteboard/input/input-runtime-keyboard.test.ts ./examples/whiteboard/lib/whiteboard/render/renderer.test.ts
```

- [x] Run hygiene:

```powershell
rg --files examples\whiteboard | rg '\.(js|jsx|cjs|mjs)$'
rg -n 'from "react"|from ''react''|from "react/|from ''react/|from "react-dom"|from ''react-dom''|React\.|useState|useEffect|useMemo|useCallback|useRef|^"use client"|^''use client''' examples\whiteboard -g '*.ts' -g '*.tsx'
rg -n '^(<<<<<<<|=======|>>>>>>>)' examples\whiteboard benchmarks docs\superpowers\plans\2026-06-03-whiteboard-outline-navigator.md .dx\receipts\whiteboard\latest.json
git diff --check -- examples/whiteboard benchmarks/whiteboard-shell-contract.test.ts benchmarks/whiteboard-model-commands-geometry.test.ts docs/superpowers/plans/2026-06-03-whiteboard-outline-navigator.md .dx/receipts/whiteboard/latest.json
```

- [x] Commit only the outline navigator slice:

```powershell
git add -- .dx/receipts/whiteboard/latest.json benchmarks/whiteboard-model-commands-geometry.test.ts benchmarks/whiteboard-shell-contract.test.ts docs/superpowers/plans/2026-06-03-whiteboard-outline-navigator.md examples/whiteboard/CHANGELOG.md examples/whiteboard/README.md examples/whiteboard/TODO.md examples/whiteboard/components/whiteboard/outline-panel.tsx examples/whiteboard/components/whiteboard/whiteboard-app.tsx examples/whiteboard/lib/whiteboard/outline.ts examples/whiteboard/styles/whiteboard.css
git commit -m "Add source-owned whiteboard outline navigator"
git push
```
