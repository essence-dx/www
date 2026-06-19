# Whiteboard Frame Membership Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make whiteboard frames first-class semantic containers that can own elements, move their members, duplicate with remapped ownership, and expose that contract through the shell.

**Architecture:** Keep frames as existing whiteboard elements with `role: "frame"` and store membership in primitive metadata as `metadata.frameId`. Add a focused `lib/whiteboard/frames.ts` helper module so scene, arrange, summary, and UI code can share one source of truth without expanding existing files. Commands expose explicit assign/clear operations and frame-aware movement stays inside the reducer path.

**Tech Stack:** DX WWW TSX, source-owned whiteboard reducer/runtime, TypeScript benchmark tests through Bun, no React runtime, no Excalidraw runtime, no hand-authored JavaScript files.

---

## File Structure

- Create `examples/whiteboard/lib/whiteboard/frames.ts`
  - Frame detection, membership metadata helpers, frame-child lookup, id expansion for frame moves, and frame assignment/clear operations.
- Modify `examples/whiteboard/lib/whiteboard/commands.ts`
  - Add `frame.assign` and `frame.clear` commands and mark them undoable.
- Modify `examples/whiteboard/lib/whiteboard/scene.ts`
  - Expand frame ids during translation so moving a frame moves its assigned children while preserving existing group expansion and lock/hidden guards.
- Modify `examples/whiteboard/lib/whiteboard/arrange-helpers.ts`
  - Remap `metadata.frameId` when duplicating a frame with its members.
- Modify `examples/whiteboard/lib/whiteboard/document-summary.ts`
  - Add `framedElementCount` and count valid frame-owned elements from the shared helpers.
- Modify `examples/whiteboard/components/whiteboard/inspector.tsx`
  - Show selected frame membership and explicit assign/clear controls routed through frame commands.
- Modify `examples/whiteboard/components/whiteboard/document-panel.tsx`
  - Mark frame rows and child rows with frame membership data attributes and concise labels.
- Modify `examples/whiteboard/components/whiteboard/canvas-stage.tsx` and `svg-stage-preview.tsx`
  - Add frame-membership shell markers for tests and preview/source contracts.
- Modify `benchmarks/whiteboard-rich-commands.test.ts`
  - Test assign, clear, moving frames with children, locked child protection, and duplicate remapping.
- Modify `benchmarks/whiteboard-shell-contract.test.ts`
  - Assert shell markers and command-backed UI controls exist.
- Modify `examples/whiteboard/README.md`, `TODO.md`, `CHANGELOG.md`, and `.dx/receipts/whiteboard/latest.json`
  - Keep public claims and proof receipts current.

## Tasks

### Task 1: Frame Helpers And Commands

**Files:**
- Create: `examples/whiteboard/lib/whiteboard/frames.ts`
- Modify: `examples/whiteboard/lib/whiteboard/commands.ts`

- [ ] **Step 1: Add failing command tests**

Add assertions in `benchmarks/whiteboard-rich-commands.test.ts` for:

```ts
const framed = whiteboardCommandReducer(document, {
  type: "frame.assign",
  frameId: makeElementId("frame"),
  ids: [makeElementId("card")],
  now: NOW,
});
assert.equal(framed.elements.find((element) => element.id === "card")?.metadata?.frameId, "frame");
```

Also assert `frame.clear` removes only `metadata.frameId` and preserves other metadata keys.

- [ ] **Step 2: Implement frame helper module**

Create helpers with these public names:

```ts
FRAME_ID_METADATA_KEY
isFrameElement
frameIdForElement
frameChildren
assignFrameToElements
clearElementFrames
expandElementIdsForFrames
remapFrameMetadata
```

Use only primitive metadata values. Do not add nested metadata.

- [ ] **Step 3: Add reducer commands**

Add `frame.assign` and `frame.clear` to `WhiteboardCommand`, route them to the helper functions, and include both in `isUndoableWhiteboardCommand`.

- [ ] **Step 4: Run focused tests**

Run:

```powershell
bun test ./benchmarks/whiteboard-rich-commands.test.ts
```

Expected: the new frame command tests pass.

### Task 2: Frame-Aware Movement And Duplication

**Files:**
- Modify: `examples/whiteboard/lib/whiteboard/scene.ts`
- Modify: `examples/whiteboard/lib/whiteboard/arrange-helpers.ts`
- Modify: `benchmarks/whiteboard-rich-commands.test.ts`

- [ ] **Step 1: Add movement and duplicate tests**

Cover these behaviors:

```ts
const moved = whiteboardCommandReducer(framed, {
  type: "element.translate",
  ids: [makeElementId("frame")],
  delta: { x: 20, y: 10 },
  now: NOW,
});
```

The frame and unlocked visible children move. Locked or hidden children do not move.

Duplicate behavior:

```ts
const copied = whiteboardCommandReducer(framed, {
  type: "element.duplicate",
  ids: [makeElementId("frame"), makeElementId("card")],
  offset: { x: 40, y: 40 },
  now: NOW,
});
```

The copied child has `metadata.frameId === "frame-copy-1"`. Duplicating a child without its frame clears or preserves membership only when the original frame still exists; choose the helper behavior and test it explicitly.

- [ ] **Step 2: Expand frame ids during translate**

In `translateElements`, combine existing group expansion with `expandElementIdsForFrames(document, ids)`. Keep existing locked/hidden guards and connector rerouting intact.

- [ ] **Step 3: Remap frame metadata during duplication**

In `duplicateElement`, after connector remapping, apply `remapFrameMetadata` so copied children point at copied frames when both are copied.

- [ ] **Step 4: Run focused tests**

Run:

```powershell
bun test ./benchmarks/whiteboard-rich-commands.test.ts
```

Expected: frame move/duplicate tests pass with existing connector tests still green.

### Task 3: Shell Contract And UI Controls

**Files:**
- Modify: `examples/whiteboard/components/whiteboard/inspector.tsx`
- Modify: `examples/whiteboard/components/whiteboard/document-panel.tsx`
- Modify: `examples/whiteboard/components/whiteboard/canvas-stage.tsx`
- Modify: `examples/whiteboard/components/whiteboard/svg-stage-preview.tsx`
- Modify: `examples/whiteboard/styles/whiteboard.css`
- Modify: `benchmarks/whiteboard-shell-contract.test.ts`

- [ ] **Step 1: Add shell contract assertions**

Assert:

```ts
assert.match(inspector, /data-whiteboard-frame-controls="command-backed"/);
assert.match(inspector, /data-whiteboard-command="frame\.assign"/);
assert.match(inspector, /data-whiteboard-command="frame\.clear"/);
assert.match(documentPanel, /data-whiteboard-frame-id/);
assert.match(canvasStage, /data-whiteboard-frame-membership="metadata-backed"/);
assert.match(svgPreview, /data-whiteboard-frame-id/);
```

- [ ] **Step 2: Add inspector controls**

Show frame membership for the selected element. If the selected element is a frame, add an assign-inside button that assigns currently selected non-frame elements into that frame. If selected element is inside a frame, add a clear button.

- [ ] **Step 3: Add layer markers**

Document panel rows should include `data-whiteboard-frame-id={frameId ?? "none"}` and a small `framed` label for children.

- [ ] **Step 4: Add preview markers**

Preview elements should carry `data-whiteboard-frame-id` and frames should carry `data-whiteboard-role="frame"` for inspection and devtools.

- [ ] **Step 5: Run shell test**

Run:

```powershell
bun test ./benchmarks/whiteboard-shell-contract.test.ts
```

Expected: shell contract passes.

### Task 4: Docs, Receipts, And Hygiene

**Files:**
- Modify: `examples/whiteboard/README.md`
- Modify: `examples/whiteboard/TODO.md`
- Modify: `examples/whiteboard/CHANGELOG.md`
- Modify: `.dx/receipts/whiteboard/latest.json`

- [ ] **Step 1: Update public docs**

State that frames are metadata-backed semantic containers with explicit assign/clear commands, frame-aware movement, and duplicate remapping. Keep browser-level E2E limitations honest.

- [ ] **Step 2: Update follow-ups**

Move first-class frame membership out of TODO and leave frame nesting/browser receipts as future work.

- [ ] **Step 3: Update receipt**

Refresh counts/notes to mention frame membership source tests and shell markers.

- [ ] **Step 4: Run final focused verification**

Run:

```powershell
bun test ./benchmarks/whiteboard-shell-contract.test.ts ./benchmarks/whiteboard-model-commands-geometry.test.ts ./benchmarks/whiteboard-rich-commands.test.ts ./benchmarks/whiteboard-persistence.test.ts ./benchmarks/whiteboard-file-workflows.test.ts ./benchmarks/whiteboard-export.test.ts ./examples/whiteboard/lib/stores/whiteboard-input-controller.test.ts ./examples/whiteboard/lib/whiteboard/input/input-runtime.test.ts ./examples/whiteboard/lib/whiteboard/input/input-runtime-keyboard.test.ts ./examples/whiteboard/lib/whiteboard/render/renderer.test.ts
rg -n 'from "react"|from ''react''|from "react/|from ''react/|from "react-dom"|from ''react-dom''|React\.|useState|useEffect|useMemo|useCallback|useRef|^"use client"|^''use client''' examples\whiteboard -g '*.ts' -g '*.tsx'
rg --files examples\whiteboard | rg '\.(js|jsx|cjs|mjs)$'
rg -n '<<<<<<<|=======|>>>>>>>' examples\whiteboard benchmarks\whiteboard-shell-contract.test.ts benchmarks\whiteboard-rich-commands.test.ts
git diff --check -- examples/whiteboard benchmarks/whiteboard-shell-contract.test.ts benchmarks/whiteboard-rich-commands.test.ts docs/superpowers/plans/2026-06-03-whiteboard-frame-membership.md .dx/receipts/whiteboard/latest.json
dx check examples/whiteboard --json
```

Expected: tests pass, scans produce no prohibited source/runtime hits, `git diff --check` passes, and `dx check` stays source-ready.

- [ ] **Step 5: Commit and push**

Stage only the related whiteboard, benchmark, plan, and receipt files:

```powershell
git add examples/whiteboard benchmarks/whiteboard-rich-commands.test.ts benchmarks/whiteboard-shell-contract.test.ts docs/superpowers/plans/2026-06-03-whiteboard-frame-membership.md .dx/receipts/whiteboard/latest.json
git commit -m "Add semantic whiteboard frame membership"
git push
```

Do not stage unrelated template/framework dirty files.
