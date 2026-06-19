# Whiteboard Minimap Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a source-owned minimap/viewport navigator to the WWW whiteboard example.

**Architecture:** Build a pure geometry helper that maps document content bounds and viewport state into minimap coordinates. Render a compact SVG minimap panel from the canonical document and send click navigation through the existing `viewport.set` store command.

**Tech Stack:** DX WWW TSX, source-owned whiteboard geometry/model/store, SVG preview primitives, Bun/node tests, no React runtime and no external UI packages.

---

## File Map

- Create `examples/whiteboard/lib/whiteboard/minimap.ts`
  - Compute minimap scene bounds, element rectangles, viewport rectangle, and click-to-viewport transforms.
- Create `examples/whiteboard/components/whiteboard/minimap-panel.tsx`
  - Render compact SVG overview, selected/hidden states, and viewport window.
- Modify `examples/whiteboard/components/whiteboard/whiteboard-app.tsx`
  - Mount the minimap in the side stack with current document and viewport.
- Modify `examples/whiteboard/styles/whiteboard.css`
  - Add compact minimap styles that fit the existing Vercel-dark whiteboard theme.
- Test `benchmarks/whiteboard-model-commands-geometry.test.ts`
  - Cover minimap bounds, viewport mapping, and click transform.
- Test `benchmarks/whiteboard-shell-contract.test.ts`
  - Cover minimap panel markers and command-backed viewport wiring.
- Update `examples/whiteboard/README.md`, `examples/whiteboard/CHANGELOG.md`, and `.dx/receipts/whiteboard/latest.json`
  - Claim source-level minimap only; browser visual receipts remain future evidence.

## Tasks

### Task 1: Pure Minimap Geometry

**Files:**
- Create: `examples/whiteboard/lib/whiteboard/minimap.ts`
- Test: `benchmarks/whiteboard-model-commands-geometry.test.ts`

- [ ] Implement `createWhiteboardMinimapModel(document, options)` returning content bounds, scaled element rectangles, selected ids, and viewport rectangle.
- [ ] Implement `minimapPointToViewport(minimap, point)` returning a `Partial<WhiteboardViewport>` centered on the clicked document point while preserving zoom.
- [ ] Ignore hidden elements for content bounds and element rectangles.
- [ ] Clamp viewport rectangle to the minimap surface when the viewport is outside content.

### Task 2: TSX Panel And Store Wiring

**Files:**
- Create: `examples/whiteboard/components/whiteboard/minimap-panel.tsx`
- Modify: `examples/whiteboard/components/whiteboard/whiteboard-app.tsx`
- Modify: `examples/whiteboard/styles/whiteboard.css`
- Test: `benchmarks/whiteboard-shell-contract.test.ts`

- [ ] Render a compact SVG overview with `data-whiteboard-minimap="model-backed"`.
- [ ] Mark viewport overlay with `data-whiteboard-minimap-viewport="viewport-state"`.
- [ ] Mark click navigation with `data-whiteboard-command="viewport.set"`.
- [ ] Wire `onClick` to compute the next viewport and call `whiteboardActions.setViewport`.

### Task 3: Docs, Receipts, And Verification

**Files:**
- Modify: `examples/whiteboard/README.md`
- Modify: `examples/whiteboard/CHANGELOG.md`
- Modify: `.dx/receipts/whiteboard/latest.json`

**Commands:**

```powershell
bun test ./benchmarks/whiteboard-shell-contract.test.ts ./benchmarks/whiteboard-model-commands-geometry.test.ts
bun test ./benchmarks/whiteboard-shell-contract.test.ts ./benchmarks/whiteboard-model-commands-geometry.test.ts ./benchmarks/whiteboard-rich-commands.test.ts ./benchmarks/whiteboard-persistence.test.ts ./benchmarks/whiteboard-file-workflows.test.ts ./benchmarks/whiteboard-export.test.ts ./examples/whiteboard/lib/stores/whiteboard-input-controller.test.ts ./examples/whiteboard/lib/whiteboard/input/input-runtime.test.ts ./examples/whiteboard/lib/whiteboard/input/input-runtime-keyboard.test.ts ./examples/whiteboard/lib/whiteboard/render/renderer.test.ts
rg -n 'from "react"|from ''react''|from "react/|from ''react/|from "react-dom"|from ''react-dom''|React\.|useState|useEffect|useMemo|useCallback|useRef|^"use client"|^''use client''' examples\whiteboard -g '*.ts' -g '*.tsx'
rg --files examples\whiteboard | rg '\.(js|jsx|cjs|mjs)$'
rg -n '^(<<<<<<<|=======|>>>>>>>)' examples\whiteboard benchmarks docs\superpowers\plans\2026-06-03-whiteboard-minimap.md
git diff --check -- examples/whiteboard benchmarks/whiteboard-shell-contract.test.ts benchmarks/whiteboard-model-commands-geometry.test.ts docs/superpowers/plans/2026-06-03-whiteboard-minimap.md .dx/receipts/whiteboard/latest.json
```

**Commit:**

```powershell
git add -- examples/whiteboard/lib/whiteboard/minimap.ts examples/whiteboard/components/whiteboard/minimap-panel.tsx examples/whiteboard/components/whiteboard/whiteboard-app.tsx examples/whiteboard/styles/whiteboard.css benchmarks/whiteboard-model-commands-geometry.test.ts benchmarks/whiteboard-shell-contract.test.ts examples/whiteboard/README.md examples/whiteboard/CHANGELOG.md docs/superpowers/plans/2026-06-03-whiteboard-minimap.md .dx/receipts/whiteboard/latest.json
git commit -m "Add source-owned whiteboard minimap"
git push
```
