# DX WWW Whiteboard

`examples/whiteboard` is a source-owned WWW whiteboard example inspired by the
product shape of Excalidraw, without depending on the Excalidraw runtime,
React DOM, Vite, or browser UI packages.

The local Excalidraw reference clone lives at
`G:\Dx\www\inspirations\excalidraw` and was inspected at commit
`33721492771919e8569964fe0b034a9cf7f25955`. It is reference material only.

## What Is Built

- WWW route shell at `app/page.tsx` with the whiteboard workbench as the first
  screen.
- React-compatible TSX authoring where WWW owns the lowering and delivery path;
  no React DOM bundle is part of the example contract.
- Source-owned model, scene commands, geometry, hit testing, history, render
  adapter, input runtime, persistence, and export modules under `lib/whiteboard`.
- Store-backed shell wired to the same canonical document that editor actions
  mutate; the workbench no longer renders from a disconnected demo constant.
- Local-first storage drivers for browser-like key-value storage and server
  filesystem `.dxdraw` files.
- Deterministic `.dxdraw`, SVG, PNG-rasterization-plan, export metadata, local
  share receipt, and filesystem snapshot helpers.
- Schema-backed import validation for `.dxdraw`, canonical document JSON, and
  legacy document migration reports.
- SVG stage preview layered over the canvas so the route visibly represents the
  canonical document while the source-owned canvas/runtime contract remains in
  place.
- Vercel-dark inspired DX Style theme using `styles/theme.css`,
  `styles/generated.css`, `styles/globals.css`, and `styles/whiteboard.css`.
- DX Style owns generated utility output, grouped class syntax, and event-class
  surfaces; authored whiteboard CSS stays in normal source files where that is
  clearer than utility strings.
- DX Icon `<Icon />` / `dx-icon` support through `components/icons/icon.tsx`,
  the `icons(...)` config, and runtime SVG path rendering; no external icon
  package is needed for the example.
- Focused TypeScript benchmark tests in `benchmarks/whiteboard-*.test.ts`.

## Runtime Contract

The production source of truth is the canonical whiteboard document:

```ts
WhiteboardDocument {
  schemaVersion: 1,
  id,
  name,
  elements,
  selection,
  groups,
  activeTool,
  viewport,
  currentStyle,
  createdAt,
  updatedAt,
  metadata,
}
```

All durable import/export and storage boundaries validate through
`lib/whiteboard/persistence/schema.ts`. Server helpers use the same schema and
store revision in `metadata.revision`, not a parallel top-level document shape.

## Feature Surface

- Tools: select, hand, freehand, line, arrow, rectangle, ellipse, diamond,
  text, and eraser.
- Editing: select, move, resize, duplicate, delete, layer order, lock/unlock,
  hide/show, undo/redo, marquee selection, fit-to-content, grid toggle, snap
  toggle, and zoom controls.
- Keyboard workflows: arrow-key nudge, Shift+Arrow larger nudge, Ctrl/Cmd+D
  duplicate, Ctrl/Cmd+G group, Ctrl/Cmd+Shift+G ungroup, Ctrl/Cmd+L lock,
  Ctrl/Cmd+Shift+L unlock, Ctrl/Cmd+H hide, and Ctrl/Cmd+Shift+H show. These
  route through the focusable canvas and the source-owned command reducer;
  browser-level E2E receipts are not claimed yet.
- Browser pointer/wheel wiring: `CanvasStage` adapts pointer and wheel events
  into the store-backed `whiteboardInputController`. The controller dispatches
  source-owned input-runtime commands into the canonical whiteboard store for
  drawing, selection, move/resize, panning, wheel zoom/pan, draft previews, and
  marquee selection. This is source-level DOM event wiring with focused tests;
  browser-level E2E, CDP, Lighthouse, and visual receipts are not claimed yet.
- Measurements: source-owned selection/document measurement panel derived from
  the canonical whiteboard document, existing selection/content bounds helpers,
  and frame metadata. It reports real bounds plus hidden, locked, framed,
  frame, text, image, and connector counts, and focuses the measured area
  through the existing non-undoable `viewport.set` command. This is
  source-level model, shell marker, and command-wiring coverage; browser-level
  visual measurement receipts are not claimed yet.
- Minimap: source-owned minimap/viewport navigator rendered from the canonical
  whiteboard document. The minimap model maps visible element bounds,
  selection state, and the current viewport into a compact SVG overview, and
  click navigation dispatches the existing `viewport.set` store command. This
  is source-level model, shell marker, and command-wiring coverage;
  browser-level E2E, CDP/Lighthouse, server-run, and visual receipts are not
  claimed yet.
- Grid: source-owned grid settings, grid size, and snap-to-grid behavior for
  drawing, moving, and resizing elements.
- Structure: command-backed groups, group metadata, metadata-backed frame
  membership, semantic element roles for frames, sticky notes, labels,
  checklist cards, shapes, and connectors. Frame membership is stored on child
  elements as `metadata.frameId` and edited through undoable `frame.assign` and
  `frame.clear` reducer commands.
- Presentation: source-owned frame navigator derived from existing
  `role: "frame"` elements and `metadata.frameId` membership. Selecting a
  slide dispatches the existing non-undoable `selection.set` and `viewport.set`
  commands to focus the frame without creating undo history. This is
  source-level model, shell marker, viewport-fit, and command-wiring coverage;
  browser-level visual presentation receipts are not claimed yet.
- Outline: source-owned outline navigator derived from the canonical
  whiteboard document. Frames are shown as grouped sections, child membership
  is read from `metadata.frameId`, unframed elements are grouped separately,
  and focusing an item dispatches existing non-undoable `selection.set` and
  `viewport.set` commands. Hidden, locked, selected, type, role, and frame
  status are surfaced through stable shell markers. This is source-level model,
  shell marker, and command-wiring coverage; browser-level E2E, visual,
  server-run, collaboration, and hosted receipts are not claimed.
- Arrange: align left/center/right/top/middle/bottom and distribute
  horizontally or vertically from computed element bounds, plus group/ungroup
  controls backed by reducer commands. Moving a frame expands movement to its
  assigned, editable children; duplicating a frame with members remaps copied
  children to the copied frame.
- Library: source-owned sticky note, decision diamond, connector, checklist,
  frame, embedded image, flowchart, kanban, retrospective, and system-map
  presets inserted through reducer commands with typed semantic roles while
  preserving legacy metadata markers. Multi-element templates carry flat
  `sourceOwned`, `template`, and `templateRole` metadata; framed templates
  store child membership through `metadata.frameId`, and diagram templates use
  `metadata.connectorRoute` for deterministic orthogonal connectors.
- Images: source-owned embedded image elements with durable `src`, `alt`, and
  optional intrinsic size metadata. Local PNG, JPEG, WebP, and SVG files can be
  imported through validated embedded data URLs and inserted as selectable,
  undoable image elements. Images participate in bounds, hit testing,
  selection, resizing, viewport fitting, document summaries, schema-backed
  persistence, SVG preview, SVG export, and PNG rasterization plans. Canvas
  rendering uses a deterministic framed fallback; remote uploads, remote URL
  ingestion, CDN storage, and media-library storage are not claimed.
- Bottom activity bar: the direct workbench keeps drawing tools, shape
  popovers, media insertion, font controls, and DX icon insertion in the bottom
  control surface. Toolbar icons render from source-owned DX SVG paths, not
  text-symbol spans or missing-icon square fallbacks.
- Styling: stroke, fill, opacity, width, font size, roughness, and stroke
  pattern, line cap, text color, text alignment, vertical alignment, precise
  position, size, rotation, text content, and element name.
- Connectors: line and arrow endpoint bindings use persisted start/end targets
  plus source-owned live endpoint rerouting. Moving, resizing, aligning,
  distributing, hiding, or removing bound targets updates or cleans connector
  point geometry. Connectors can use straight routing or deterministic
  orthogonal elbow routing through `metadata.connectorRoute`. Orthogonal routes
  use horizontal-first elbow points (`start -> midpoint-x/start-y ->
  midpoint-x/end-y -> end`), while straight connectors preserve explicit middle
  points. This is not obstacle avoidance, collision avoidance, or automatic
  diagram layout.
- Files: local import plus JSON, SVG, and PNG export path.
- Sharing: local preview share receipts with document summaries and redaction
  flags. Live remote links require an explicit adapter and are not claimed yet.
- Proof modules: geometry bounds, hit testing, command reduction, local-first
  persistence, `.dxdraw` round-trip, SVG escaping, PNG renderer injection,
  lock/visibility protection, library insertion, source-owned template
  insertion, embedded image validation,
  local embedded image import, import diagnostics, export
  metadata, share receipts, and filesystem snapshots.
  Grid/snap runtime behavior, source-owned selection/document measurements,
  store-backed pointer/wheel controller flows,
  minimap geometry and viewport command wiring, live connector rerouting,
  orthogonal connector route metadata, frame presentation navigation,
  frame-grouped outline navigation, undoable text commit, grouping,
  group-aware edits, frame membership, frame-aware movement, frame duplicate
  remapping, marquee selection, keyboard workflows, semantic roles, and
  semantic summary counts are covered through focused regression tests.

## Verification

Focused checks used for this example:

```powershell
bun test ./benchmarks/whiteboard-shell-contract.test.ts ./benchmarks/whiteboard-model-commands-geometry.test.ts ./benchmarks/whiteboard-rich-commands.test.ts ./benchmarks/whiteboard-persistence.test.ts ./benchmarks/whiteboard-file-workflows.test.ts ./benchmarks/whiteboard-export.test.ts ./examples/whiteboard/lib/stores/whiteboard-input-controller.test.ts ./examples/whiteboard/lib/whiteboard/input/input-runtime.test.ts ./examples/whiteboard/lib/whiteboard/input/input-runtime-keyboard.test.ts ./examples/whiteboard/lib/whiteboard/render/renderer.test.ts
rg -n 'from "react"|from ''react''|from "react/|from ''react/|from "react-dom"|from ''react-dom''|React\.|useState|useEffect|useMemo|useCallback|useRef|^"use client"|^''use client''' examples\whiteboard -g '*.ts' -g '*.tsx'
rg --files examples\whiteboard | rg '\.(js|jsx|cjs|mjs)$'
git diff --check -- examples/whiteboard benchmarks/whiteboard-shell-contract.test.ts benchmarks/whiteboard-model-commands-geometry.test.ts benchmarks/whiteboard-rich-commands.test.ts benchmarks/whiteboard-persistence.test.ts benchmarks/whiteboard-file-workflows.test.ts benchmarks/whiteboard-export.test.ts docs/superpowers/plans/2026-06-03-whiteboard-pointer-runtime.md docs/superpowers/plans/2026-06-03-whiteboard-bottom-bar-media.md
dx check examples/whiteboard --json
```

Latest focused whiteboard regression pass: 121 whiteboard tests passed across 10
files.

## Boundaries

- No Excalidraw package is imported or bundled.
- No `node_modules` folder is required by this WWW example.
- No `.js`, `.cjs`, or `.mjs` whiteboard source files are used.
- The example keeps reference clones out of the runtime and out of production
  output.
- Collaboration, smart orthogonal connector routing, automatic diagram layout,
  remote image upload, remote URL image ingestion, CDN/media-library storage,
  multiplayer presence, validated external library import, remote share links,
  frame nesting, drag-to-nest/autocapture, smart obstacle-avoiding connector
  layout, browser-level measurement receipts, browser-level outline receipts,
  and browser-level E2E/template visual receipts are not claimed in this
  example yet; they should be added as explicit source-owned adapters with
  focused receipts.
