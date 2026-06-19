# Whiteboard Image Import Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add real source-owned local image import to the WWW whiteboard example through validated embedded data URLs.

**Architecture:** Keep image persistence on the existing `image` element shape and `embedded-data-url-only` policy. Add a small pure import helper for file/data-url validation and element creation, then wire the import panel to read local files and insert/select an image through the existing store/reducer command path.

**Tech Stack:** DX WWW TSX, source-owned whiteboard reducer/store, embedded image data URLs, Bun/node tests, no React runtime and no external browser UI packages.

---

## File Map

- Create `examples/whiteboard/lib/whiteboard/image-import.ts`
  - Validate importable image MIME types, convert data URLs into whiteboard image element options, calculate fit size, and create image elements through the existing factory.
- Modify `examples/whiteboard/components/whiteboard/import-panel.tsx`
  - Add a second file picker for images, read the selected file as a data URL, and dispatch an undoable `element.add` command.
- Modify `examples/whiteboard/lib/stores/whiteboard-store.ts`
  - Add a narrow `insertImageFile`-style action only if the UI cannot stay clean with direct dispatch.
- Test `benchmarks/whiteboard-file-workflows.test.ts`
  - Cover accepted MIME type import, rejected MIME type import, size fitting, metadata/alt text, and non-mutation on invalid input.
- Test `benchmarks/whiteboard-shell-contract.test.ts`
  - Cover stable image-import panel markers and embedded policy markers.
- Update `examples/whiteboard/README.md`, `examples/whiteboard/TODO.md`, `examples/whiteboard/CHANGELOG.md`, and `.dx/receipts/whiteboard/latest.json`
  - Claim local embedded image import only; keep CDN/media-library uploads as future adapters.

## Tasks

### Task 1: Pure Image Import Helper

**Files:**
- Create: `examples/whiteboard/lib/whiteboard/image-import.ts`
- Test: `benchmarks/whiteboard-file-workflows.test.ts`

- [ ] Add tests that call the helper with a supported `data:image/png;base64,...` source and assert it creates an `image` element with embedded policy metadata.
- [ ] Add tests that reject `text/plain`, remote URLs, empty alt text, and oversized/zero dimensions with explicit diagnostics.
- [ ] Implement `createImportedImageElement(options)` using `createImageElement`, `assertEmbeddedImageSource`, and existing ID/date options.
- [ ] Implement deterministic fit sizing with a max width/height that preserves aspect ratio.

### Task 2: TSX Import Panel Wiring

**Files:**
- Modify: `examples/whiteboard/components/whiteboard/import-panel.tsx`
- Optional Modify: `examples/whiteboard/lib/stores/whiteboard-store.ts`
- Test: `benchmarks/whiteboard-shell-contract.test.ts`

- [ ] Add an image import control beside the existing `.dxdraw` import control.
- [ ] Mark it with `data-dx-whiteboard-image-import="embedded-data-url-only"`.
- [ ] Read the selected `File` as a data URL via `FileReader`.
- [ ] Dispatch `element.add` with a generated image element, then write a visible status message.
- [ ] Keep failure status visible and do not mutate the document on invalid files.

### Task 3: Documentation And Receipts

**Files:**
- Modify: `examples/whiteboard/README.md`
- Modify: `examples/whiteboard/TODO.md`
- Modify: `examples/whiteboard/CHANGELOG.md`
- Modify: `.dx/receipts/whiteboard/latest.json`

- [ ] Update the README to say local image file import is supported through embedded data URLs.
- [ ] Keep CDN/media-library upload and remote storage as explicit future work.
- [ ] Update the receipt proof command after verification, not before.

### Task 4: Verification And Commit

**Commands:**

```powershell
bun test ./benchmarks/whiteboard-shell-contract.test.ts ./benchmarks/whiteboard-file-workflows.test.ts ./benchmarks/whiteboard-export.test.ts ./benchmarks/whiteboard-persistence.test.ts
bun test ./benchmarks/whiteboard-shell-contract.test.ts ./benchmarks/whiteboard-model-commands-geometry.test.ts ./benchmarks/whiteboard-rich-commands.test.ts ./benchmarks/whiteboard-persistence.test.ts ./benchmarks/whiteboard-file-workflows.test.ts ./benchmarks/whiteboard-export.test.ts ./examples/whiteboard/lib/stores/whiteboard-input-controller.test.ts ./examples/whiteboard/lib/whiteboard/input/input-runtime.test.ts ./examples/whiteboard/lib/whiteboard/input/input-runtime-keyboard.test.ts ./examples/whiteboard/lib/whiteboard/render/renderer.test.ts
rg -n 'from "react"|from ''react''|from "react/|from ''react/|from "react-dom"|from ''react-dom''|React\.|useState|useEffect|useMemo|useCallback|useRef|^"use client"|^''use client''' examples\whiteboard -g '*.ts' -g '*.tsx'
rg --files examples\whiteboard | rg '\.(js|jsx|cjs|mjs)$'
rg -n '^(<<<<<<<|=======|>>>>>>>)' examples\whiteboard benchmarks docs\superpowers\plans\2026-06-03-whiteboard-image-import.md
git diff --check -- examples/whiteboard benchmarks/whiteboard-shell-contract.test.ts benchmarks/whiteboard-file-workflows.test.ts docs/superpowers/plans/2026-06-03-whiteboard-image-import.md .dx/receipts/whiteboard/latest.json
```

**Commit:**

```powershell
git add -- examples/whiteboard/lib/whiteboard/image-import.ts examples/whiteboard/components/whiteboard/import-panel.tsx benchmarks/whiteboard-file-workflows.test.ts benchmarks/whiteboard-shell-contract.test.ts examples/whiteboard/README.md examples/whiteboard/TODO.md examples/whiteboard/CHANGELOG.md docs/superpowers/plans/2026-06-03-whiteboard-image-import.md .dx/receipts/whiteboard/latest.json
git commit -m "Add source-owned whiteboard image import"
git push
```
