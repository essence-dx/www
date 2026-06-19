# DX Devtools Real Controls And TS Guards Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace placeholder-like Devtools picker interactions with actual source-owned controls and move the focused Devtools regression guard from CommonJS to TypeScript.

**Architecture:** Keep the browser runtime dependency-free and framework-owned. Add custom pointer-driven controls in the injected runtime using buttons/divs/contenteditable spans, not native form controls, and keep the test executable with `node --test` as `.ts`.

**Tech Stack:** Rust-served vanilla JS/CSS runtime, Node built-in test runner, TypeScript syntax in benchmark guards, no npm packages.

---

### Task 1: Convert Guard Test To TS

**Files:**
- Rename: `benchmarks/dx-devtools-framework-integration.test.cjs` -> `benchmarks/dx-devtools-framework-integration.test.ts`

- [ ] Replace CommonJS `require(...)` imports with TypeScript/ESM imports from `node:*`.
- [ ] Add narrow type annotations for helper arguments and return values.
- [ ] Keep assertions and source guards equivalent.
- [ ] Run `node --test benchmarks\dx-devtools-framework-integration.test.ts`.

### Task 2: Add Real Picker Controls

**Files:**
- Modify: `dx-www/src/cli/devtools/assets.rs`

- [ ] Add pointer-driven slider controls for RGB, HSL, alpha, and gradient numeric channels.
- [ ] Add a dial-style angle control for linear gradients.
- [ ] Add a position pad for radial gradients.
- [ ] Add a mesh stage with draggable layer points.
- [ ] Keep quick swatches as shortcuts only.
- [ ] Keep no native `input`, `select`, or `textarea`.

### Task 3: Strengthen Guards

**Files:**
- Modify: `benchmarks/dx-devtools-framework-integration.test.ts`

- [ ] Assert slider/dial/position/mesh control builders exist.
- [ ] Assert custom controls use pointer events.
- [ ] Assert no `.cjs` Devtools guard remains.
- [ ] Assert the runtime stays dependency-free.

### Task 4: Verify And Install

**Files:**
- No source edits.

- [ ] Run `node --test benchmarks\dx-devtools-framework-integration.test.ts`.
- [ ] Run runtime JS syntax parse.
- [ ] Run `cargo fmt --check`.
- [ ] Run `cargo build -j 6 -p dx-www --no-default-features --features cli --bin dx-www`.
- [ ] Install to `G:\Dx\bin`.
- [ ] Restart template on `127.0.0.1:3000`.
