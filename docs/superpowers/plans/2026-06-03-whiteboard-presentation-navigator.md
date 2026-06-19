# Whiteboard Presentation Navigator Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a source-owned frame presentation navigator so whiteboard frames can behave like ordered slides with command-backed viewport jumps.

**Architecture:** Derive presentation slides from existing `role: "frame"` elements and `metadata.frameId` membership. Do not add a new schema field or reducer command; focusing a slide dispatches existing non-undoable `selection.set` and `viewport.set` commands.

**Tech Stack:** DX WWW TSX, TypeScript whiteboard model helpers, existing frame metadata, existing fit-to-bounds viewport math, focused Bun tests.

---

## File Map

- Create `examples/whiteboard/lib/whiteboard/presentation.ts`: pure presentation model and command helpers.
- Create `examples/whiteboard/components/whiteboard/presentation-panel.tsx`: frame slide UI with previous/next/list focus actions.
- Modify `examples/whiteboard/components/whiteboard/whiteboard-app.tsx`: mount the presentation panel.
- Modify `examples/whiteboard/styles/whiteboard.css`: compact slide-list styling.
- Modify `benchmarks/whiteboard-model-commands-geometry.test.ts`: prove derived model, current slide detection, and non-undoable commands.
- Modify `benchmarks/whiteboard-shell-contract.test.ts`: prove shell markers and real command dispatch.
- Modify `examples/whiteboard/README.md`, `examples/whiteboard/CHANGELOG.md`, `.dx/receipts/whiteboard/latest.json`, and `examples/whiteboard/TODO.md`: document exact claims.

## Tasks

### Task 1: Pure Presentation Model

- [x] Add `createWhiteboardPresentationModel(document, stageSize)` that returns ordered visible frames as slides.
- [x] Each slide should include frame id, title, index, bounds, child count, selected state, and a fitted viewport.
- [x] Current slide should prefer selected frame; otherwise selected child frame membership; otherwise first slide.
- [x] Add `presentationCommandsForSlide(slide)` returning `selection.set` and `viewport.set`.

### Task 2: Presentation Panel

- [x] Add `PresentationPanel` using the pure model.
- [x] Mount it in the side panel.
- [x] Expose stable markers:
  - `data-whiteboard-presentation="frame-navigator"`
  - `data-whiteboard-presentation-slide-count`
  - `data-whiteboard-presentation-current-index`
  - `data-whiteboard-presentation-slide-id`
  - `data-whiteboard-command="selection.set viewport.set"`
- [x] Buttons must dispatch `whiteboardActions.dispatchBatch(presentationCommandsForSlide(slide))`.

### Task 3: Tests

- [x] Add model tests for slide ordering, child counts, selected frame current slide, selected child current slide, and no-frame empty state.
- [x] Add reducer/history test proving presentation commands do not create undo history.
- [x] Add shell-contract tests proving panel mount, markers, command dispatch, and no React runtime imports.

### Task 4: Docs And Receipt

- [x] Update README feature surface and proof modules.
- [x] Update CHANGELOG.
- [x] Update `.dx/receipts/whiteboard/latest.json` with presentation navigator feature and marker.
- [x] Keep browser E2E/visual/server proof unclaimed.

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
rg -n '^(<<<<<<<|=======|>>>>>>>)' examples\whiteboard benchmarks docs\superpowers\plans\2026-06-03-whiteboard-presentation-navigator.md .dx\receipts\whiteboard\latest.json
git diff --check -- examples/whiteboard benchmarks/whiteboard-shell-contract.test.ts benchmarks/whiteboard-model-commands-geometry.test.ts docs/superpowers/plans/2026-06-03-whiteboard-presentation-navigator.md .dx/receipts/whiteboard/latest.json
```

- [x] Commit only the presentation navigator slice:

```powershell
git add -- .dx/receipts/whiteboard/latest.json benchmarks/whiteboard-model-commands-geometry.test.ts benchmarks/whiteboard-shell-contract.test.ts docs/superpowers/plans/2026-06-03-whiteboard-presentation-navigator.md examples/whiteboard/CHANGELOG.md examples/whiteboard/README.md examples/whiteboard/TODO.md examples/whiteboard/components/whiteboard/presentation-panel.tsx examples/whiteboard/components/whiteboard/whiteboard-app.tsx examples/whiteboard/lib/whiteboard/presentation.ts examples/whiteboard/styles/whiteboard.css
git commit -m "Add source-owned whiteboard presentation navigator"
git push
```
