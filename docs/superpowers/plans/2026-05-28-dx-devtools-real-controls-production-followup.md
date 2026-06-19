# DX Devtools Real Controls Production Followup Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Tighten the injected DX Devtools controls so the property/value selectors are practical real controls, edge panels open reliably from hold/focus gestures, and the guard remains TypeScript-only.

**Architecture:** Keep the runtime source-owned and dependency-free. Improve the existing framework-owned `runtime.ts` and `devtools.css` directly, then lock behavior with the focused TypeScript guard. Rust still serves the browser asset from `/_dx/devtools/runtime.js`.

**Tech Stack:** Rust-served source-owned TypeScript/CSS, Node `--test` TypeScript guard, no npm/runtime UI packages.

---

### Task 1: Add Searchable Custom Popovers

**Files:**
- Modify: `dx-www/src/cli/devtools/assets/runtime.ts`
- Modify: `dx-www/src/cli/devtools/assets/devtools.css`
- Test: `benchmarks/dx-devtools-framework-integration.test.ts`

- [ ] Add per-popover query state for property and value selectors.
- [ ] Render custom contenteditable search boxes inside popovers; do not use native inputs.
- [ ] Filter large MDN option lists by query while preserving active-descendant keyboard behavior.
- [ ] Add CSS for the search row using the existing Vercel/Geist dark tokens.
- [ ] Extend the TypeScript guard to assert searchable custom selectors and no `.cjs` drift.

### Task 2: Harden Edge Hold Controls

**Files:**
- Modify: `dx-www/src/cli/devtools/assets/runtime.ts`
- Test: `benchmarks/dx-devtools-framework-integration.test.ts`

- [ ] Add pointerover/pointerout and focus/blur paths alongside pointerenter/pointerleave.
- [ ] Keep close cooldown honored by all edge-open paths.
- [ ] Clear pending edge timers during cleanup.
- [ ] Extend the TypeScript guard for the additional edge interaction paths.

### Task 3: Focused Verification

**Files:**
- Test: `benchmarks/dx-devtools-framework-integration.test.ts`

- [ ] Run `node --check .\dx-www\src\cli\devtools\assets\runtime.ts`.
- [ ] Run `node --test .\benchmarks\dx-devtools-framework-integration.test.ts`.
- [ ] Run `cargo fmt --check`.
- [ ] Run `git diff --check` and a conflict-marker scan scoped to touched files.
