# Whiteboard Live Connectors Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make bound whiteboard line/arrow endpoints reroute when their connected elements move, resize, align, distribute, hide, or are removed.

**Architecture:** Keep connector routing source-owned and model-level. Add a small connector routing module that computes anchor points from existing element geometry, then call it from scene and arrange mutation paths after elements change. Do not add browser-only DOM measuring or external diagram libraries.

**Tech Stack:** DX WWW whiteboard TypeScript, source-owned scene commands, Bun/Node test files, `.dx` check receipts.

---

## File Map

- Create: `examples/whiteboard/lib/whiteboard/connectors.ts`
  - Anchor point math, connector endpoint refresh, stale binding cleanup, document-level connector reroute helpers.
- Modify: `examples/whiteboard/lib/whiteboard/scene.ts`
  - Reroute connectors after `insertElements`, `updateElement`, `removeElements`, and `translateElements`.
- Modify: `examples/whiteboard/lib/whiteboard/arrange.ts`
  - Reroute connectors after duplicate, group, lock/hide, align, and distribute paths that mutate element geometry or visibility.
- Modify: `examples/whiteboard/components/whiteboard/connector-info.tsx`
  - Show connector binding mode and anchor names so the inspector is useful.
- Modify: `benchmarks/whiteboard-rich-commands.test.ts`
  - Prove move, resize, hide, remove, align, distribute, undo/redo connector rerouting.
- Modify: `benchmarks/whiteboard-shell-contract.test.ts`
  - Prove the connector info panel exposes live routing contract markers.
- Modify: `examples/whiteboard/README.md`, `examples/whiteboard/CHANGELOG.md`, `examples/whiteboard/TODO.md`
  - Move live connector rerouting from boundary to feature surface after tests pass.

## Tasks

### Task 1: Connector Routing Model

- [x] Create `examples/whiteboard/lib/whiteboard/connectors.ts`.
- [x] Export `connectorAnchorPoint(element, anchor)` using `getElementBounds` and these anchor rules:
  - `center`: center of bounds.
  - `top/right/bottom/left`: edge midpoint.
  - `auto`: choose the side facing the other endpoint when possible; fall back to center.
- [x] Export `rerouteBoundConnectors(document, changedIds, now)` that updates line/arrow `points[0]` and/or last point when a binding target still exists.
- [x] Remove stale `startBinding` / `endBinding` when a bound target no longer exists or is hidden.
- [x] Preserve all middle points for multi-point connectors.

### Task 2: Scene Integration

- [x] Import `rerouteBoundConnectors` in `scene.ts`.
- [x] After element insertion, update, removal, and translation, reroute affected connectors.
- [x] Do not reroute locked/hidden connectors unless their binding metadata must be cleaned after target removal/hidden state.
- [x] Keep source document immutable.

### Task 3: Arrange Integration

- [x] Import `rerouteBoundConnectors` in `arrange.ts`.
- [x] Reroute after duplicate, lock/hide, align, and distribute operations.
- [x] Duplicated connectors should clear bindings unless the copied target elements are also explicitly duplicated in the same command.
- [x] Hide/remove operations should clean connector bindings to hidden targets.

### Task 4: Tests

- [x] Add focused tests to `benchmarks/whiteboard-rich-commands.test.ts`.
- [x] Test moving a bound rectangle updates an arrow endpoint.
- [x] Test resizing a bound rectangle updates the anchor point.
- [x] Test hiding/removing a bound target clears stale binding metadata.
- [x] Test align/distribute reroutes connectors.
- [x] Test undo/redo restores routed connector geometry.

### Task 5: Inspector And Docs

- [x] Update `connector-info.tsx` with live routing markers and visible anchor labels.
- [x] Update shell contract tests for the routing marker.
- [x] Update README feature surface and boundaries without overclaiming external routing libraries or smart elbow paths.
- [x] Update CHANGELOG/TODO.

### Task 6: Verification And Checkpoint

- [x] Run the focused whiteboard test suite.
- [x] Run no-React and no `.js/.jsx/.cjs/.mjs` whiteboard scans.
- [x] Run `git diff --check` on changed whiteboard paths.
- [x] Run `dx check examples/whiteboard --json`.
- [x] Stage only whiteboard-related files, commit with `Add live whiteboard connector routing`, and push.
