# DX WWW Repo Hygiene Sweep Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use `superpowers:subagent-driven-development` or `superpowers:executing-plans` to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Turn the repo hygiene audit into guarded cleanup that removes junk, exposes structural debt, and prevents the same file/folder sprawl from silently returning.

**Architecture:** Keep destructive cleanup limited to checkpoint-protected, ignored, generated, or clearly obsolete files. Preserve tracked runtime fixtures that tests currently read, and enforce the remaining debt with a TypeScript hygiene audit instead of pretending risky moves are done.

**Tech Stack:** Git checkpoint commits, PowerShell-safe path cleanup, TypeScript `node --test` hygiene checks, repo-local docs, and focused Rust/Node verification.

---

## Task 1: Checkpoint And Safe Junk Cleanup

**Files:**
- Remove: root `NONE`, `NUL`, `*.log`, `.dx-devtools-server.*.log`
- Remove: ignored `.codex-tmp/`, `.tmp/`, `.dx/codex-run/`, `.dx/run/`, `.dx/build/`, `.dx/style/`, `.dx/serializer/`
- Modify: remove tracked zero-byte `index.html`

- [x] Create a checkpoint commit before cleanup.
- [x] Verify the cleanup targets are inside `G:\Dx\www`.
- [x] Remove ignored local junk and generated run output.
- [x] Remove the tracked zero-byte root `index.html`.

## Task 2: Add A Repo Hygiene Audit

**Files:**
- Create: `tools/hygiene/audit-repo-hygiene.ts`
- Create: `benchmarks/repo-hygiene-audit.test.ts`

- [x] Detect forbidden root junk files and root logs.
- [x] Detect oversized hand-authored source files in core framework paths.
- [x] Detect deprecated example folders and typoed folder names.
- [x] Detect tracked generated run output under root `.dx` where it is not explicitly allowed.
- [x] Detect legacy `.js`, `.cjs`, and `.mjs` scripts outside approved runtime/vendor paths.
- [x] Emit a machine-readable summary that future agents can use without broad scans.

## Task 3: Document Remaining Structural Debt

**Files:**
- Create: `docs/repo-hygiene.md`
- Modify: `README.md`

- [x] Document which root folders are canonical crates, fixtures, vendor copies, generated state, or archives.
- [x] Name the tracked fixture exceptions that cannot be moved without updating tests.
- [x] Document the serializer migration boundary from `related-crates/serializer` to `G:\Dx\serializer`.
- [x] List the large files that still need real refactors.

## Task 4: Commit Hygiene Improvements

**Files:**
- All files changed by Tasks 1-3.

- [x] Run `node --test benchmarks/repo-hygiene-audit.test.ts`.
- [x] Run `git diff --check`.
- [x] Commit with a focused hygiene message.
