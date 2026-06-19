# DX WWW Scorecard Closure Pass 2 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Close the next source-safe 12-flaw scorecard items without hiding the remaining large Rust/fixture/script debt.

**Architecture:** Preserve runtime behavior by splitting large source-owned assets into ordered fragments and assembling them at the Rust serve boundary or CSS import boundary. Keep `dx.build.readinessGate` separate from repo hygiene until DX has a dedicated hygiene receipt. Heavy Rust files that need real domain extraction stay measured unless a safe boundary is identified.

**Tech Stack:** Rust `include_str!`/`concat!` for Devtools assets, CSS `@import` for standalone Devtools style fragments, TypeScript `node --test` guards, and the repo hygiene scorecard.

---

### Task 1: Checkpoint And Baseline

**Files:**
- Verify: `G:\Dx\www`
- Commit: empty checkpoint before edits

- [x] Confirm repo root is `G:/Dx/www`.
- [x] Confirm branch is `dev`.
- [x] Confirm the worktree starts clean.
- [x] Create checkpoint commit `chore: checkpoint before scorecard closure pass`.
- [x] Capture baseline audit: `11/12` active, `readyFor100: false`.

### Task 2: Split Injected Devtools Runtime Asset

**Files:**
- Modify: `dx-www/src/cli/devtools/assets.rs`
- Modify: `dx-www/src/cli/devtools/assets/runtime.ts`
- Create: `dx-www/src/cli/devtools/assets/runtime/part-01-boot.ts`
- Create: `dx-www/src/cli/devtools/assets/runtime/part-02-protocol.ts`
- Create: `dx-www/src/cli/devtools/assets/runtime/part-03-controls.ts`
- Create: `dx-www/src/cli/devtools/assets/runtime/part-04-render.ts`
- Create: `dx-www/src/cli/devtools/assets/runtime/part-05-events.ts`
- Modify: `benchmarks/dx-devtools-framework-integration.test.ts`

- [x] Move the large browser runtime body into ordered fragments without changing bytes except line endings.
- [x] Keep `runtime.ts` as a source-owned manifest that names the fragments and explains that Rust serves the concatenated runtime.
- [x] Change `assets.rs` so `RUNTIME_JS` uses `concat!(include_str!(...))` over the fragments.
- [x] Update the Devtools benchmark to test the assembled runtime source instead of only `runtime.ts`.
- [x] Run `node --check` on the assembled runtime through a temporary generated file.

### Task 3: Split Standalone Devtools CSS

**Files:**
- Modify: `dx-devtools/styles/devtools.css`
- Create: `dx-devtools/styles/devtools/part-01-foundation.css`
- Create: `dx-devtools/styles/devtools/part-02-panels.css`
- Create: `dx-devtools/styles/devtools/part-03-inspector.css`
- Create: `dx-devtools/styles/devtools/part-04-controls.css`
- Create: `dx-devtools/styles/devtools/part-05-responsive.css`

- [x] Move the standalone Devtools CSS body into ordered imported fragments.
- [x] Keep `devtools.css` as the stable public import used by `dx-devtools/app/layout.tsx`.
- [x] Verify `dx-devtools/styles/devtools.css` drops below the hygiene budget.

### Task 4: Reclassify Closed Scorecard Items With Tests

**Files:**
- Modify: `tools/hygiene/audit-repo-hygiene.ts`
- Modify: `benchmarks/repo-hygiene-audit.test.ts`
- Modify: `docs/repo-hygiene.md`

- [x] Keep the 12 stable scorecard ids unchanged.
- [x] Ensure `devtools-runtime-large`, `devtools-style-ops-large`, and `devtools-css-large` report `passing` when their canonical files are below budget.
- [x] Add a test that exactly those three ids are passing after this pass.
- [x] Update docs to state that the Devtools asset/file split is complete and the remaining scorecard items are still active.

### Task 5: Six-Agent Heavy-Debt Triage

**Files:**
- Read-only unless explicitly assigned by the controller.

- [x] Agent 1: inspect `dx-www/src/cli/mod.rs` for the smallest safe extraction boundary.
- [x] Agent 2: inspect `dx-www/src/cli/public_framework_tools.rs` for the smallest safe extraction boundary.
- [x] Agent 3: inspect `dx-www/src/cli/app_router_execution/source_render.rs` for the smallest safe extraction boundary.
- [x] Agent 4: inspect `core/src/ecosystem/dx_check_receipt.rs` and `core/src/ecosystem/forge_registry.rs` for generated/test-data extraction options.
- [x] Agent 5: inspect `core/src/ecosystem/project_check.rs` and source-visible fixture debt.
- [x] Agent 6: inspect legacy `.js/.cjs/.mjs` debt and propose a safe migration order.

### Task 6: Verification And Commit

**Files:**
- All files changed by Tasks 2-4.

- [x] Run `node --test benchmarks/dx-devtools-framework-integration.test.ts`.
- [x] Run `node --test benchmarks/repo-hygiene-audit.test.ts`.
- [x] Run `node --test benchmarks/generated-artifact-ignore-contract.test.ts`.
- [x] Run `cargo fmt --check`.
- [x] Run `cargo check -j 1 -p dx-www --no-default-features --features cli --bin dx-www`.
- [x] Run `git diff --check`.
- [x] Run a conflict-marker scan excluding generated/vendor output.
- [x] Commit with `chore: split devtools scorecard assets`.
