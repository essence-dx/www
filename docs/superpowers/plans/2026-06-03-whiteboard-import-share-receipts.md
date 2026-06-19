# Whiteboard Import And Share Receipts Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add schema-backed whiteboard import validation, export metadata, local share receipts, and filesystem snapshots without adding React, browser UI packages, or hand-authored JavaScript files.

**Architecture:** Keep file workflows source-owned under `examples/whiteboard/lib/whiteboard` and `examples/whiteboard/server/whiteboard`. UI panels consume canonical import/export reports instead of inventing parallel status. Filesystem snapshots redact local paths and report invalid `.dxdraw` files without mutating storage.

**Tech Stack:** DX WWW TSX, TypeScript modules, Bun `node:test`, `.dx` check receipts.

---

### Task 1: Canonical Document Summary

**Files:**
- Create: `examples/whiteboard/lib/whiteboard/document-summary.ts`

- [x] Add `summarizeWhiteboardDocument()` with element counts, visibility counts, locked counts, selection count, revision, bounds, and timestamps.
- [x] Add `revisionFromDocument()` for shared receipt/storage usage.

### Task 2: Import And Export Receipts

**Files:**
- Create: `examples/whiteboard/lib/whiteboard/import/validation.ts`
- Create: `examples/whiteboard/lib/whiteboard/import/index.ts`
- Create: `examples/whiteboard/lib/whiteboard/export/metadata.ts`
- Create: `examples/whiteboard/lib/whiteboard/export/share-receipt.ts`
- Modify: `examples/whiteboard/lib/whiteboard/export/dxdraw.ts`
- Modify: `examples/whiteboard/lib/whiteboard/export/index.ts`

- [x] Add structured import reports for `.dxdraw`, raw canonical documents, malformed JSON, invalid documents, and legacy migrations.
- [x] Embed export metadata in `.dxdraw` output without letting envelope metadata override validated document content.
- [x] Add preview-only share receipts with document summary, redaction flags, and no fake live collaboration claim.

### Task 3: Filesystem Snapshot

**Files:**
- Create: `examples/whiteboard/server/whiteboard/filesystem-snapshot.ts`
- Modify: `examples/whiteboard/server/whiteboard/index.ts`

- [x] Add filesystem snapshot generation for valid and invalid `.dxdraw` files.
- [x] Sort board summaries deterministically.
- [x] Redact absolute local storage roots from snapshot output.

### Task 4: Workflow Panels

**Files:**
- Create: `examples/whiteboard/components/whiteboard/import-panel.tsx`
- Create: `examples/whiteboard/components/whiteboard/share-panel.tsx`
- Modify: `examples/whiteboard/components/whiteboard/export-panel.tsx`
- Modify: `examples/whiteboard/components/whiteboard/whiteboard-app.tsx`
- Modify: `examples/whiteboard/styles/whiteboard.css`
- Modify: `examples/whiteboard/lib/stores/whiteboard-store.ts`

- [x] Add schema-backed import panel for `.dxdraw` and canonical document JSON.
- [x] Add local share receipt panel with copy and receipt download actions.
- [x] Add store reset action for accepted imports.
- [x] Keep panels TSX-only and free of React runtime imports.

### Task 5: Verification And Sync

**Files:**
- Create: `benchmarks/whiteboard-file-workflows.test.ts`
- Modify: `benchmarks/whiteboard-shell-contract.test.ts`
- Modify: `examples/whiteboard/README.md`
- Modify: `examples/whiteboard/TODO.md`
- Modify: `examples/whiteboard/CHANGELOG.md`
- Update: `examples/whiteboard/.dx/**` through `dx check examples/whiteboard --json`

- [x] Add focused tests for import validation, export metadata, share receipts, filesystem snapshots, and shell contract markers.
- [x] Run the focused whiteboard test suite.
- [x] Run no-React and no-JavaScript source scans for `examples/whiteboard`.
- [x] Run `dx check examples/whiteboard --json`.
- [x] Commit with a professional message and push `features` to the configured remote.
