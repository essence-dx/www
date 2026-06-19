# Whiteboard Changelog

## 2026-06-03

- Rewired the app shell to render from the canonical whiteboard store instead of a detached demo document.
- Added reducer-backed duplicate, lock, unlock, hide, show, align, distribute, grid, snap, and library insertion commands.
- Added source-owned library presets for sticky notes, decisions, connectors, checklists, and frames.
- Added arrange and library panels, richer layer controls, precise inspector controls, line cap, text alignment, vertical alignment, element naming, and status metadata.
- Added an SVG stage preview that renders the canonical document while preserving the canvas/input/render modules.
- Hardened lock and visibility behavior across pointer moves, resizing, erasing, keyboard selection, and renderer selection handles.
- Added focused command, renderer, and input runtime regression coverage for the richer editor features.
- Added document summaries, schema-backed import validation, `.dxdraw` export metadata, preview-only share receipts, and filesystem storage snapshots.
- Added import and share panels plus focused workflow tests for import diagnostics, export provenance, share receipts, and snapshot invalid-file reporting.
- Added reducer-owned grid settings and snap-to-grid behavior for drawing, moving, resizing, and refreshed input-runtime hit testing.
- Added command-backed groups, group metadata, schema repair for group membership, and group/ungroup controls.
- Added semantic element roles for sticky notes, frames, labels, checklist cards, shapes, and connectors without changing the renderer element type dispatch.
- Added connector endpoint metadata capture, connector inspector details, and persistence round-trip coverage.
- Added undoable `text.commit` behavior through the overlay and inspector, plus focused undo/redo coverage.
- Added semantic document summary counts for frames, sticky notes, connectors, and groups.
- Added runtime-only marquee selection, source-owned canvas keyboard dispatch,
  fit-to-content viewport math, and command-backed keyboard workflows for
  nudge, duplicate, group, ungroup, lock, unlock, hide, and show.
- Added group-aware select, move, and delete behavior so grouped elements edit as a unit.
- Added source-owned embedded image elements with typed model/factory support,
  embedded-data image validation, document summary counts, a library preset,
  inspector `src`/`alt` controls, deterministic canvas placeholder rendering,
  SVG preview/export `<image>` output, and PNG plan metadata. Remote uploads,
  CDN storage, collaboration, remote share links, and browser-level proof
  remain future adapter work.
- Added source-owned local image file import through validated embedded data
  URLs, creating selectable undoable image elements with import metadata and
  the existing `embedded-data-url-only` policy. Remote uploads, remote URL
  ingestion, CDN/media-library storage, and browser-level visual proof remain
  future adapter work.
- Added source-owned live connector endpoint rerouting for bound line and arrow
  elements. Moving, resizing, aligning, distributing, hiding, removing, and
  duplicating bound targets now updates or cleans connector point geometry while
  preserving middle points and undo/redo snapshots.
- Wired `CanvasStage` browser pointer and wheel handlers into a store-backed
  whiteboard input controller, so pointer and wheel events flow through the
  source-owned input runtime and canonical reducer for drawing, movement,
  resizing, panning, wheel zoom/pan, draft previews, and marquee selection.
- Added a source-owned whiteboard minimap / viewport navigator. A pure minimap
  model maps visible element bounds, selection state, and viewport state into a
  compact SVG overview, while the panel routes click navigation through the
  existing `viewport.set` store command. Guarded with focused minimap geometry
  and shell-contract tests; no browser E2E, CDP/Lighthouse, server-run, or
  visual receipt is claimed.
- Added source-owned multi-element library templates for flowcharts, kanban
  boards, retrospectives, and system maps. Templates insert real model
  elements through the existing `library.insert` reducer path with flat
  `sourceOwned`/`template`/`templateRole` metadata, `metadata.frameId` for
  framed child membership, and `metadata.connectorRoute` for deterministic
  orthogonal diagram connectors. Guarded with focused reducer and shell
  contract tests; browser-level visual receipts are not claimed.
- Added a source-owned frame presentation navigator derived from existing
  `role: "frame"` elements and `metadata.frameId` membership. The panel routes
  slide focus through non-undoable `selection.set` plus `viewport.set`
  commands, with source tests for slide derivation, selected-frame and
  selected-child resolution, empty boards, viewport fitting, shell markers, and
  undo-history safety. Browser-level presentation visuals are not claimed.
- Added a source-owned whiteboard outline navigator. It derives frame-grouped
  sections from the canonical document, uses `metadata.frameId` for child
  membership, keeps unframed elements in a separate section, and focuses an
  item through existing `selection.set` plus `viewport.set` command wiring.
  Source tests cover grouping, labels, hidden/locked/selected status, orphan
  frame references, shell markers, and undo-history safety. Browser-level E2E,
  visual, server-run, collaboration, hosted, and external-runtime proof are
  not claimed.
- Added a source-owned measurements panel. It derives selection or document
  bounds from existing scene geometry, reports real hidden, locked, framed,
  frame, connector, image, and text counts, and focuses the measured area
  through the existing non-undoable `viewport.set` command. Source tests cover
  selected measurements, document fallback, empty boards, shell markers, and
  undo-history safety. Browser-level visual measurement proof is not claimed.
- Added source-level controller and shell-contract coverage for pointer/wheel
  wiring; browser-level E2E, CDP, Lighthouse, and visual receipts remain future
  proof work.
- Added metadata-backed frame membership with `metadata.frameId`,
  `frame.assign`/`frame.clear` commands, frame-aware movement, frame duplicate
  membership remapping, framed-element summary counts, and source-level shell
  markers.
- Added source-owned deterministic orthogonal connector routing for bound
  line/arrow elements via `metadata.connectorRoute`, command-backed connector
  controls, live rerouting, preview/export markers, and focused source tests.
  Straight connectors continue preserving manual middle points; obstacle
  avoidance, automatic diagram layout, and browser-level visual receipts remain
  future proof.
- Added the direct workbench bottom activity bar with source-owned shape
  popovers, media insertion, font controls, inline DX icon objects, and runtime
  SVG icon rendering. The whiteboard now guards against text-symbol icon spans
  and missing-icon square fallbacks in focused shell-contract tests.
- Renamed the file workflow panel copy toward `Open Files` / `Add Image` and
  kept schema-backed file validation plus embedded-data image loading under
  stable source-owned markers.

## 2026-06-02

- Added a source-owned WWW whiteboard example with DX-native TSX, route metadata, dx-style stylesheets, and local icon components.
- Added the canonical whiteboard document model, scene commands, hit testing, render adapters, pointer input runtime, undo/redo history, persistence receipts, and import/export helpers.
- Added focused TypeScript tests for model commands, renderer output, input runtime behavior, persistence, export, and shell source contracts.
- Reworked the app shell to avoid React runtime imports and keep canvas rendering, controls, inspector panels, document metadata, and export actions wired to the local whiteboard store.
