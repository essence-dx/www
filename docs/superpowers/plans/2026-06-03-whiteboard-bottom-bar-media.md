# Whiteboard Bottom Bar And Media Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Convert the whiteboard to a clean full-screen canvas with one bottom activity bar, reliable text editing, font controls, DX icon insertion, and media insertion.

**Architecture:** Keep the example source-owned and dependency-free. Use `app/page.tsx` for declarative controls, `public/whiteboard-runtime.ts` for all runtime behavior, and `styles/whiteboard.css` for Vercel-dark layout and controls.

**Tech Stack:** DX WWW TSX example, source-owned browser JavaScript, SVG/HTML overlay objects, DX icon CSS tokens, no node_modules.

---

### Task 1: Bottom Activity Bar Layout

**Files:**
- Modify: `examples/whiteboard/app/page.tsx`
- Modify: `examples/whiteboard/styles/whiteboard.css`

- [x] Remove the visible header from the whiteboard surface.
- [x] Move Clear, Shortcuts, and Panel controls into the activity bar.
- [x] Reposition the toolbar as a centered bottom bar and make tool popovers open upward.
- [x] Keep the canvas full-screen with no page scrolling.

### Task 2: Text Font Controls

**Files:**
- Modify: `examples/whiteboard/app/page.tsx`
- Modify: `examples/whiteboard/public/whiteboard-runtime.ts`
- Modify: `examples/whiteboard/styles/whiteboard.css`

- [x] Keep JetBrains Mono as the default text object font.
- [x] Add a font input/list in the panel that accepts any Google Font family name.
- [x] Load the selected Google Font on demand with a generated Google Fonts stylesheet URL.
- [x] Apply the selected font to the selected text/math object and persist it.
- [x] Preserve double-click text editing.

### Task 3: DX Icon Picker

**Files:**
- Modify: `examples/whiteboard/app/page.tsx`
- Modify: `examples/whiteboard/public/whiteboard-runtime.ts`
- Modify: `examples/whiteboard/styles/whiteboard.css`

- [x] Add a bottom-bar icon picker popover using existing `data-dx-icon` names.
- [x] Insert selected icons as SVG icon objects on the board.
- [x] Make inserted icon objects selectable, movable, colorable, persistent, and double-click controllable.

### Task 4: Media Insertion

**Files:**
- Modify: `examples/whiteboard/app/page.tsx`
- Modify: `examples/whiteboard/public/whiteboard-runtime.ts`
- Modify: `examples/whiteboard/styles/whiteboard.css`

- [x] Add image, audio, and video controls to the bottom bar.
- [x] Use hidden file inputs and data URLs without external packages.
- [x] Add media objects as positioned HTML overlays that move with selected object geometry.
- [x] Make media objects selectable, movable, removable, persistent where safe, and visible on the board.

### Task 5: Shortcuts And Tooltips

**Files:**
- Modify: `examples/whiteboard/public/whiteboard-runtime.ts`
- Modify: `examples/whiteboard/styles/whiteboard.css`

- [x] Update tooltip placements for bottom-bar controls.
- [x] Keep `?`, `Esc`, tool shortcuts, undo/redo, and delete working.
- [x] Ensure Escape closes popovers, media pickers, font editing state, text editor, and tooltips.

### Task 6: Focused Verification

**Files:**
- Verify only with focused checks.

- [x] Browser verify: no header, bottom bar visible, Clear/Panel/Shortcuts in bottom bar.
- [x] Browser verify: shape/icon popovers open upward and close correctly.
- [x] Browser verify: double-click text opens editor, commits on Enter, and leaves the side panel closed.
- [x] Browser verify: icon picker inserts real inline DX icon SVG objects.
- [x] Browser verify: toolbar icons render from DX Icons with no placeholder square bodies and no span-symbol fallbacks.
- [x] Run `node --check examples\whiteboard\public\whiteboard-runtime.ts`.
- [x] Run `cargo fmt --check -p dx-icons -p dx-www`.
- [x] Run `cargo test -j 6 -p dx-icons whiteboard_ --lib`.
- [x] Run `cargo build -j 6 -p dx-www --no-default-features --features cli --bin dx-www`.
- [x] Run `git diff --check -- examples/whiteboard/app/page.tsx examples/whiteboard/public/whiteboard-runtime.ts examples/whiteboard/styles/whiteboard.css`.
