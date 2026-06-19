# DX Devtools UI Polish No-Build Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Fix Devtools panel closing and upgrade the source-owned injected Devtools UI to a Vercel-dark, JetBrains Mono, inspector-first bottom-sheet experience without rebuilding until explicitly approved.

**Architecture:** Keep the runtime source-owned and dependency-free in `dx-www/src/cli/devtools/assets.rs`. Harden interactions in the plain JS runtime, map computed CSS into grouped inspector data, and keep style preview/apply routed through the existing structured protocol.

**Tech Stack:** Rust-served static assets, plain browser JavaScript, CSS variables, Node test runner.

---

### Task 1: Panel Close And Edge Hold

**Files:**
- Modify: `dx-www/src/cli/devtools/assets.rs`
- Test: `benchmarks/dx-devtools-framework-integration.test.ts`

- [x] **Step 1: Add close-button source test**

Assert the runtime contains `data-dx-devtools-close` and that `closePanel(edge)` deletes the active edge from `STATE.openPanels`.

- [x] **Step 2: Harden close behavior**

Add delegated root click handling for `[data-dx-devtools-close]`, clear pending edge timers in `closePanel(edge)`, and add a short edge cooldown so closing a panel does not instantly re-open it.

- [x] **Step 3: Verify with focused Node test**

Run: `node --test benchmarks\dx-devtools-framework-integration.test.ts`

### Task 2: Vercel Dark Theme And Bottom Sheet

**Files:**
- Modify: `dx-www/src/cli/devtools/assets.rs`
- Test: `benchmarks/dx-devtools-framework-integration.test.ts`

- [x] **Step 1: Add theme assertions**

Assert the CSS consumes Vercel/Geist variables, defaults to JetBrains Mono, and keeps Devtools assets dependency-free.

- [x] **Step 2: Patch CSS tokens**

Define `--dx-devtools-*` theme tokens with `--ds-*` / `--geist-*` fallbacks, use the tokens in panel, puck, menu, control, overlay, and box-model styles, and constrain mobile panels to bottom-sheet dimensions.

### Task 3: Inspector And CSS Mapping

**Files:**
- Modify: `dx-www/src/cli/devtools/assets.rs`
- Test: `benchmarks/dx-devtools-framework-integration.test.ts`

- [x] **Step 1: Group computed CSS**

Add `CSS_GROUPS` for text, box, spacing, border, paint, and layout CSS values.

- [x] **Step 2: Render grouped CSS panels**

Render computed CSS groups as selectable rows that can seed style edits.

### Task 4: Custom Controls And Color/Gradient Picker

**Files:**
- Modify: `dx-www/src/cli/devtools/assets.rs`
- Test: `benchmarks/dx-devtools-framework-integration.test.ts`

- [x] **Step 1: Add custom control shell**

Wrap text inputs in Devtools-owned themed control shells with native appearance disabled.

- [x] **Step 2: Add picker presets**

Add solid color presets plus linear, radial, and mesh gradient presets that feed structured style preview/apply operations.

### Task 5: No-Build Verification

**Files:**
- Test: `benchmarks/dx-devtools-framework-integration.test.ts`

- [x] **Step 1: Run source-level focused tests only**

Run: `node --test benchmarks\dx-devtools-framework-integration.test.ts`

- [ ] **Step 2: Rebuild/install later only after user approval**

Deferred because the user explicitly said not to build until told.
