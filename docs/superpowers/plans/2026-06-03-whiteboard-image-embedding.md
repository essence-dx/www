# Whiteboard Image Embedding Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add source-owned image embedding to `examples/whiteboard` without React runtime packages, external browser UI libraries, or hidden Excalidraw dependencies.

**Architecture:** Images are first-class whiteboard box elements with durable `src`, `alt`, and optional intrinsic size metadata. Canvas rendering uses a deterministic placeholder in tests and the SVG preview/export emits real `<image>` tags, while schema validation keeps unsafe or empty image values out of documents.

**Tech Stack:** WWW TSX, source-owned whiteboard model/reducer/schema, Bun `node:test`, DX check receipts.

---

### Task 1: Model And Factory

**Files:**
- Modify: `examples/whiteboard/lib/whiteboard/model.ts`
- Modify: `examples/whiteboard/lib/whiteboard/element-factory.ts`
- Modify: `examples/whiteboard/lib/whiteboard/geometry.ts`
- Modify: `examples/whiteboard/lib/whiteboard/render/geometry.ts`

- [ ] Add `WhiteboardImageElement` as a box element with `type: "image"`, `src`, `alt`, and optional `naturalWidth` / `naturalHeight`.
- [ ] Include images in the `WhiteboardElement` union and `WhiteboardElementPatch`.
- [ ] Add `createImageElement()` with the same base element path as rectangle/text.
- [ ] Treat images as box elements for bounds, hit testing, resizing, selection handles, and viewport fitting.

### Task 2: Schema And Persistence

**Files:**
- Modify: `examples/whiteboard/lib/whiteboard/persistence/schema.ts`
- Test: `benchmarks/whiteboard-persistence.test.ts`

- [ ] Accept `type: "image"` with non-empty `src` and `alt`.
- [ ] Preserve optional finite positive `naturalWidth` and `naturalHeight`.
- [ ] Reject empty image `src` through the existing precise validation path.
- [ ] Preserve image fields through validate/migrate round trips.

### Task 3: Render And Export

**Files:**
- Modify: `examples/whiteboard/lib/whiteboard/render/renderer.ts`
- Modify: `examples/whiteboard/lib/whiteboard/export/svg.ts`
- Modify: `examples/whiteboard/lib/whiteboard/export/metadata.ts`
- Test: `examples/whiteboard/lib/whiteboard/render/renderer.test.ts`
- Test: `benchmarks/whiteboard-export.test.ts`

- [ ] Render images in the canvas renderer as a stable framed placeholder with image label text.
- [ ] Export images as SVG `<image>` tags with escaped `href`, `alt`-like metadata, dimensions, and opacity.
- [ ] Count images in export metadata or document summary if the existing metadata surface supports element counts.
- [ ] Add tests for canvas placeholder and SVG image export escaping.

### Task 4: UI And Library

**Files:**
- Modify: `examples/whiteboard/lib/whiteboard/library.ts`
- Modify: `examples/whiteboard/components/whiteboard/inspector.tsx`
- Modify: `examples/whiteboard/components/whiteboard/library-panel.tsx` only if preset labeling needs support
- Modify: `examples/whiteboard/styles/whiteboard.css` only if existing classes cannot present the image preset cleanly
- Test: `benchmarks/whiteboard-rich-commands.test.ts`
- Test: `benchmarks/whiteboard-shell-contract.test.ts`

- [ ] Add a source-owned image placeholder library preset.
- [ ] Add inspector controls for selected image `src` and `alt`.
- [ ] Keep controls connected to `element.update`; no fake file uploads.
- [ ] Add shell contract coverage for image preset and image inspector controls.

### Task 5: Documentation And Receipts

**Files:**
- Modify: `examples/whiteboard/README.md`
- Modify: `examples/whiteboard/CHANGELOG.md`
- Modify: `examples/whiteboard/TODO.md`
- Modify after verification: `examples/whiteboard/.dx/**`

- [ ] Document image embedding as source-owned and explicit.
- [ ] Keep remote uploads/CDN adapters as future work unless implemented.
- [ ] Refresh `dx check examples/whiteboard --json` after tests pass.

### Task 6: Verification

Run from `G:\Dx\www`:

```powershell
bun test ./benchmarks/whiteboard-shell-contract.test.ts ./benchmarks/whiteboard-model-commands-geometry.test.ts ./benchmarks/whiteboard-rich-commands.test.ts ./benchmarks/whiteboard-persistence.test.ts ./benchmarks/whiteboard-file-workflows.test.ts ./benchmarks/whiteboard-export.test.ts ./examples/whiteboard/lib/whiteboard/input/input-runtime.test.ts ./examples/whiteboard/lib/whiteboard/input/input-runtime-keyboard.test.ts ./examples/whiteboard/lib/whiteboard/render/renderer.test.ts
rg -n 'from "react"|from ''react''|from "react/|from ''react/|from "react-dom"|from ''react-dom''|React\.|useState|useEffect|useMemo|useCallback|useRef|^"use client"|^''use client''' examples\whiteboard -g '*.ts' -g '*.tsx'
rg --files examples\whiteboard | rg '\.(js|jsx|cjs|mjs)$'
git diff --check -- examples/whiteboard benchmarks/whiteboard-shell-contract.test.ts benchmarks/whiteboard-model-commands-geometry.test.ts benchmarks/whiteboard-rich-commands.test.ts benchmarks/whiteboard-persistence.test.ts benchmarks/whiteboard-export.test.ts docs/superpowers/plans/2026-06-03-whiteboard-image-embedding.md
dx check examples/whiteboard --json
```

Expected:
- Bun tests pass with no failures.
- React and JS scans produce no matches; `rg` exit code `1` is success for those scans.
- `git diff --check` exits `0`.
- `dx check` reports source-ready approved and engine score `100/100`; skipped runtime proof warnings are acceptable unless browser runtime proof was explicitly run.
