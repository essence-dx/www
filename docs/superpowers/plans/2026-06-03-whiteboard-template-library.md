# Whiteboard Template Library Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add source-owned, command-backed multi-element whiteboard templates that make the example materially closer to a professional drawing/diagramming tool.

**Architecture:** Keep templates in the existing `lib/whiteboard/library.ts` boundary and insert them through the existing `library.insert` reducer command. Templates must produce real model elements with stable IDs, semantic roles, frame metadata where useful, and no decorative shell-only cards.

**Tech Stack:** DX WWW TSX, TypeScript whiteboard model/reducer modules, focused Bun tests, source-level receipts.

---

## File Map

- Modify `examples/whiteboard/lib/whiteboard/library.ts`: expand preset IDs and add multi-element template factories.
- Modify `examples/whiteboard/components/whiteboard/library-panel.tsx`: expose preset categories, counts, and stable template markers.
- Modify `benchmarks/whiteboard-rich-commands.test.ts`: prove each advanced template inserts real elements with semantics and frame metadata.
- Modify `benchmarks/whiteboard-shell-contract.test.ts`: prove shell markers and no React runtime imports.
- Modify `examples/whiteboard/README.md`, `examples/whiteboard/CHANGELOG.md`, `.dx/receipts/whiteboard/latest.json`: document exactly what is claimed.

## Tasks

### Task 1: Model Rich Template Presets

- [x] Extend `WhiteboardLibraryPresetId` with professional templates:
  - `flowchart-basic`
  - `kanban-board`
  - `retrospective-board`
  - `system-map`
- [x] Add a preset category field such as `"single"` or `"template"` so the panel can group presets without hardcoded label checks.
- [x] Keep existing presets stable.
- [x] Use existing rectangle, diamond, text, image, and arrow element types. Do not add a new element type.
- [x] Add semantic metadata:
  - `template`
  - `templateRole`
  - `frameId` for children inside template frames
  - `connectorRoute: "orthogonal"` for diagram connectors where appropriate.

### Task 2: Panel Rendering

- [x] Update `LibraryPanel` to show the total number of presets plus a template count.
- [x] Add stable markers:
  - `data-whiteboard-library="command-backed"`
  - `data-whiteboard-library-template-count`
  - `data-whiteboard-preset-category`
  - `data-whiteboard-template="source-owned"` for template presets.
- [x] Keep button actions routed through `whiteboardActions.insertLibraryPreset`.

### Task 3: Focused Tests

- [x] Add reducer tests proving advanced templates insert more than one element, select all inserted elements, and preserve deterministic IDs.
- [x] Add tests proving frame-backed templates assign `metadata.frameId` to child cards.
- [x] Add tests proving flow/system templates include bound or route-aware connectors with `metadata.connectorRoute`.
- [x] Add shell-contract tests proving panel markers, categories, and no React runtime imports.

### Task 4: Docs And Receipt

- [x] Update README feature list to describe source-owned templates honestly.
- [x] Update CHANGELOG with template-library scope and proof.
- [x] Update `.dx/receipts/whiteboard/latest.json` with template-library feature markers and the new focused test count after verification.
- [x] Do not claim browser E2E, visual, server, or collaboration proof.

### Task 5: Verification And Commit

- [x] Run focused tests:

```powershell
bun test ./benchmarks/whiteboard-shell-contract.test.ts ./benchmarks/whiteboard-rich-commands.test.ts
```

- [x] Run full whiteboard regression:

```powershell
bun test ./benchmarks/whiteboard-shell-contract.test.ts ./benchmarks/whiteboard-model-commands-geometry.test.ts ./benchmarks/whiteboard-rich-commands.test.ts ./benchmarks/whiteboard-persistence.test.ts ./benchmarks/whiteboard-file-workflows.test.ts ./benchmarks/whiteboard-export.test.ts ./examples/whiteboard/lib/stores/whiteboard-input-controller.test.ts ./examples/whiteboard/lib/whiteboard/input/input-runtime.test.ts ./examples/whiteboard/lib/whiteboard/input/input-runtime-keyboard.test.ts ./examples/whiteboard/lib/whiteboard/render/renderer.test.ts
```

- [x] Run hygiene:

```powershell
rg --files examples\whiteboard | rg '\.(js|jsx|cjs|mjs)$'
rg -n 'from "react"|from ''react''|from "react/|from ''react/|React\.|useState|useEffect|useMemo|useCallback|useRef|^"use client"|^''use client''' examples\whiteboard -g '*.ts' -g '*.tsx'
rg -n '^(<<<<<<<|=======|>>>>>>>)' examples\whiteboard benchmarks docs\superpowers\plans\2026-06-03-whiteboard-template-library.md
git diff --check -- examples/whiteboard benchmarks/whiteboard-shell-contract.test.ts benchmarks/whiteboard-rich-commands.test.ts docs/superpowers/plans/2026-06-03-whiteboard-template-library.md .dx/receipts/whiteboard/latest.json
```

- [x] Commit only the template-library slice:

```powershell
git add -- .dx/receipts/whiteboard/latest.json benchmarks/whiteboard-rich-commands.test.ts benchmarks/whiteboard-shell-contract.test.ts docs/superpowers/plans/2026-06-03-whiteboard-template-library.md examples/whiteboard/CHANGELOG.md examples/whiteboard/README.md examples/whiteboard/TODO.md examples/whiteboard/components/whiteboard/library-panel.tsx examples/whiteboard/lib/whiteboard/library.ts examples/whiteboard/lib/whiteboard/library-templates.ts examples/whiteboard/styles/whiteboard.css
git commit -m "Add source-owned whiteboard templates"
git push
```
