# Whiteboard Measurements Panel Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a source-owned measurements panel that exposes real selection and document metrics through the canonical whiteboard document.

**Architecture:** Derive measurements from `WhiteboardDocument`, existing bounds helpers, and existing frame metadata. Focusing the measured region dispatches the existing non-undoable `viewport.set` command. No external package, React runtime, hidden schema, or mock data is introduced.

**Tech Stack:** DX WWW TSX, TypeScript whiteboard model helpers, existing scene geometry, existing viewport fit math, focused Bun tests.

---

## File Map

- Create `examples/whiteboard/lib/whiteboard/measurements.ts`: pure measurements model and focus command helper.
- Create `examples/whiteboard/components/whiteboard/measurement-panel.tsx`: selection/document metrics UI.
- Modify `examples/whiteboard/components/whiteboard/whiteboard-app.tsx`: mount the measurements panel.
- Modify `examples/whiteboard/styles/whiteboard.css`: compact measurements styling.
- Modify `benchmarks/whiteboard-model-commands-geometry.test.ts`: prove selection/document/empty measurements and focus commands.
- Modify `benchmarks/whiteboard-shell-contract.test.ts`: prove shell markers and dispatch wiring.
- Modify `examples/whiteboard/README.md`, `examples/whiteboard/CHANGELOG.md`, `.dx/receipts/whiteboard/latest.json`, and `examples/whiteboard/TODO.md`: document exact source-level claim.

## Tasks

### Task 1: Pure Measurements Model

- [x] Add `createWhiteboardMeasurementModel(document, stageSize)`.
- [x] Prefer selection bounds when selection exists, otherwise document content bounds.
- [x] Expose subject, label, item counts, hidden/locked/framed/frame/connector/image/text counts, bounds, and fitted viewport.
- [x] Add `measurementCommandsForFocus(model)` returning `viewport.set` only when bounds exist.

### Task 2: Measurements Panel

- [x] Add `MeasurementPanel` using the pure model.
- [x] Mount it near Inspector/Arrange so selection metrics are immediately visible.
- [x] Expose stable markers:
  - `data-whiteboard-measurements`
  - `data-whiteboard-measurement-subject`
  - `data-whiteboard-measurement-item-count`
  - `data-whiteboard-measurement-can-focus`
  - `data-whiteboard-command="viewport.set"`
- [x] Dispatch `whiteboardActions.dispatchBatch(measurementCommandsForFocus(model))`.

### Task 3: Focused Tests

- [x] Add tests for selected-element measurements.
- [x] Add tests for document-level measurements when nothing is selected.
- [x] Add tests for empty-board measurements.
- [x] Add tests proving focus commands fit the viewport without creating undo history.
- [x] Add shell-contract tests proving panel mount, markers, command dispatch, and no React runtime imports.

### Task 4: Docs And Receipt

- [x] Update README feature surface and proof modules.
- [x] Update CHANGELOG.
- [x] Update `.dx/receipts/whiteboard/latest.json` with measurements feature and markers.
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
rg -n '^(<<<<<<<|=======|>>>>>>>)' examples\whiteboard benchmarks docs\superpowers\plans\2026-06-03-whiteboard-measurements-panel.md .dx\receipts\whiteboard\latest.json
git diff --check -- examples/whiteboard benchmarks/whiteboard-shell-contract.test.ts benchmarks/whiteboard-model-commands-geometry.test.ts docs/superpowers/plans/2026-06-03-whiteboard-measurements-panel.md .dx/receipts/whiteboard/latest.json
```

- [x] Commit only the measurements slice:

```powershell
git add -- .dx/receipts/whiteboard/latest.json benchmarks/whiteboard-model-commands-geometry.test.ts benchmarks/whiteboard-shell-contract.test.ts docs/superpowers/plans/2026-06-03-whiteboard-measurements-panel.md examples/whiteboard/CHANGELOG.md examples/whiteboard/README.md examples/whiteboard/TODO.md examples/whiteboard/components/whiteboard/measurement-panel.tsx examples/whiteboard/components/whiteboard/whiteboard-app.tsx examples/whiteboard/lib/whiteboard/measurements.ts examples/whiteboard/styles/whiteboard.css
git commit -m "Add source-owned whiteboard measurements panel"
git push
```
