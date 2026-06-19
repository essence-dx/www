# Whiteboard Orthogonal Connector Routing Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add source-owned orthogonal connector routing so bound line/arrow elements can route through deterministic elbow points instead of only straight endpoints.

**Architecture:** Keep connector geometry as normal `points` arrays and persist the route mode through primitive connector metadata as `metadata.connectorRoute`. `connectors.ts` owns endpoint rerouting and chooses either straight endpoint preservation or orthogonal elbow generation. UI exposes the current route mode through `ConnectorInfo` and reducer-backed `element.update` commands.

**Tech Stack:** DX WWW TSX, source-owned whiteboard model/reducer/render pipeline, Bun TypeScript benchmark tests, no external routing package, no React runtime.

---

## File Structure

- Create `examples/whiteboard/lib/whiteboard/connector-routes.ts`
  - Route metadata constants, route-mode parser, metadata patch helpers, and deterministic orthogonal point generation.
- Modify `examples/whiteboard/lib/whiteboard/connectors.ts`
  - Use route mode when rerouting bound connectors.
- Modify `examples/whiteboard/components/whiteboard/connector-info.tsx`
  - Add command-backed route controls and shell markers.
- Modify `examples/whiteboard/components/whiteboard/svg-stage-preview.tsx`
  - Add route-mode markers to connector preview elements.
- Modify `examples/whiteboard/lib/whiteboard/export/svg.ts`
  - Add route-mode marker to connector SVG paths.
- Modify `benchmarks/whiteboard-rich-commands.test.ts`
  - Cover orthogonal rerouting on move/resize and straight-mode preservation.
- Modify `benchmarks/whiteboard-shell-contract.test.ts`
  - Cover route markers and command-backed route controls.
- Modify docs/receipt files to keep public claims current.

## Tasks

### Task 1: Route Helpers

**Files:**
- Create: `examples/whiteboard/lib/whiteboard/connector-routes.ts`
- Modify: `examples/whiteboard/lib/whiteboard/connectors.ts`

- [ ] **Step 1: Add route helper module**

Implement:

```ts
export type WhiteboardConnectorRoute = "straight" | "orthogonal";
export const CONNECTOR_ROUTE_METADATA_KEY = "connectorRoute";
export function connectorRouteForElement(element: ConnectorElement): WhiteboardConnectorRoute;
export function connectorRouteMetadata(route: WhiteboardConnectorRoute): WhiteboardMetadata;
export function orthogonalConnectorPoints(start: WhiteboardPoint, end: WhiteboardPoint): readonly WhiteboardPoint[];
```

Use a horizontal-first elbow path: start -> midpoint x at start y -> midpoint x at end y -> end. Collapse duplicate adjacent points only if the final tuple still has at least two points.

- [ ] **Step 2: Wire rerouting**

In `connectors.ts`, when a bound connector route is `orthogonal`, replace middle points with the orthogonal points from the helper. Keep straight connectors preserving existing middle points.

- [ ] **Step 3: Run focused command test**

Run:

```powershell
bun test ./benchmarks/whiteboard-rich-commands.test.ts
```

### Task 2: UI And Export Markers

**Files:**
- Modify: `examples/whiteboard/components/whiteboard/connector-info.tsx`
- Modify: `examples/whiteboard/components/whiteboard/svg-stage-preview.tsx`
- Modify: `examples/whiteboard/lib/whiteboard/export/svg.ts`
- Modify: `benchmarks/whiteboard-shell-contract.test.ts`

- [ ] **Step 1: Add route controls**

In `ConnectorInfo`, add a compact route mode toolbar with buttons:

```tsx
data-whiteboard-connector-route-controls="command-backed"
data-whiteboard-command="element.update"
data-whiteboard-connector-route="straight"
data-whiteboard-connector-route="orthogonal"
```

Each button dispatches `element.update` with `patch.metadata = { ...element.metadata, connectorRoute: route }`.

- [ ] **Step 2: Add preview/export markers**

Add `data-whiteboard-connector-route={connectorRouteForElement(element)}` on connector preview polylines and SVG export connector paths.

- [ ] **Step 3: Run shell test**

Run:

```powershell
bun test ./benchmarks/whiteboard-shell-contract.test.ts ./benchmarks/whiteboard-export.test.ts
```

### Task 3: Tests, Docs, Receipts, Commit

**Files:**
- Modify: `benchmarks/whiteboard-rich-commands.test.ts`
- Modify: `benchmarks/whiteboard-export.test.ts`
- Modify: `examples/whiteboard/README.md`
- Modify: `examples/whiteboard/TODO.md`
- Modify: `examples/whiteboard/CHANGELOG.md`
- Modify: `.dx/receipts/whiteboard/latest.json`

- [ ] **Step 1: Add tests**

Add tests for:

- Orthogonal connector points after moving a bound target.
- Orthogonal connector points after resizing a bound target.
- Straight connector with manual middle point still preserves that middle point.
- SVG export includes `data-whiteboard-connector-route="orthogonal"`.

- [ ] **Step 2: Update docs**

Document that orthogonal routing is deterministic elbow routing, not a full obstacle-avoidance solver.

- [ ] **Step 3: Run final focused verification**

Run:

```powershell
bun test ./benchmarks/whiteboard-shell-contract.test.ts ./benchmarks/whiteboard-model-commands-geometry.test.ts ./benchmarks/whiteboard-rich-commands.test.ts ./benchmarks/whiteboard-persistence.test.ts ./benchmarks/whiteboard-file-workflows.test.ts ./benchmarks/whiteboard-export.test.ts ./examples/whiteboard/lib/stores/whiteboard-input-controller.test.ts ./examples/whiteboard/lib/whiteboard/input/input-runtime.test.ts ./examples/whiteboard/lib/whiteboard/input/input-runtime-keyboard.test.ts ./examples/whiteboard/lib/whiteboard/render/renderer.test.ts
rg -n 'from "react"|from ''react''|from "react/|from ''react/|from "react-dom"|from ''react-dom''|React\.|useState|useEffect|useMemo|useCallback|useRef|^"use client"|^''use client''' examples\whiteboard -g '*.ts' -g '*.tsx'
rg --files examples\whiteboard | rg '\.(js|jsx|cjs|mjs)$'
rg -n -i 'excalidraw|@excalidraw' examples\whiteboard benchmarks -g '*.ts' -g '*.tsx' -g '!README.md' -g '!TODO.md' -g '!CHANGELOG.md'
rg -n '^(<<<<<<<|=======|>>>>>>>)' examples\whiteboard benchmarks docs\superpowers\plans\2026-06-03-whiteboard-orthogonal-connectors.md
git diff --check -- examples/whiteboard benchmarks/whiteboard-shell-contract.test.ts benchmarks/whiteboard-rich-commands.test.ts benchmarks/whiteboard-export.test.ts docs/superpowers/plans/2026-06-03-whiteboard-orthogonal-connectors.md .dx/receipts/whiteboard/latest.json
```

- [ ] **Step 4: Commit and push**

Stage only this slice and commit:

```powershell
git add examples/whiteboard benchmarks/whiteboard-shell-contract.test.ts benchmarks/whiteboard-rich-commands.test.ts benchmarks/whiteboard-export.test.ts docs/superpowers/plans/2026-06-03-whiteboard-orthogonal-connectors.md .dx/receipts/whiteboard/latest.json
git commit -m "Add orthogonal whiteboard connector routing"
git push
```
