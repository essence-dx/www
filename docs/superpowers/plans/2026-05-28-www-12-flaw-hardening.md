# DX WWW 12-Flaw Hardening Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Turn the current repo hygiene/readiness debt into guarded, testable hardening work without pretending unsafe large-file refactors are complete.

**Architecture:** Keep the active checkout protected by a checkpoint commit, then fix source-safe flaws directly: make the hygiene audit publish an explicit 12-flaw scorecard, extract the Devtools style operation test block out of production code, convert the nearest legacy `.cjs` hygiene test to `.ts`, document fixture ownership, and wire focused verification. Large Rust framework files that cannot be split safely in one pass remain measured debt instead of being hidden.

**Tech Stack:** Git checkpoint commits, TypeScript `node --test`, Rust module extraction for Devtools style operation tests, repo-local hygiene contracts, and focused source-only verification.

---

### Task 1: Checkpoint And Safety

**Files:**
- Verify: `G:\Dx\www`
- Commit: empty checkpoint when the tree is clean

- [x] Confirm `git rev-parse --show-toplevel` is `G:/Dx/www`.
- [x] Confirm the current branch is `dev`.
- [x] Confirm the tree starts clean.
- [x] Create checkpoint commit `chore: checkpoint before 12-flaw hardening`.

### Task 2: Publish A 12-Flaw Hygiene Scorecard

**Files:**
- Modify: `tools/hygiene/audit-repo-hygiene.ts`
- Modify: `benchmarks/repo-hygiene-audit.test.ts`

- [x] Add a stable `TWELVE_WORST_HYGIENE_FLAWS` contract with exactly 12 ids:
  `cli-mod-large`, `public-framework-tools-large`, `source-render-large`,
  `dx-check-receipt-large`, `forge-registry-large`, `project-check-large`,
  `devtools-runtime-large`, `devtools-style-ops-large`, `devtools-css-large`,
  `source-visible-fixtures`, `legacy-script-extensions`, and
  `readiness-overclaim-risk`.
- [x] Return `scorecard` and `readiness` fields from `auditRepoHygiene()`.
- [x] Keep `ok` tied to blockers only, and add `readyFor100` so reports cannot confuse "no junk blockers" with "100/100 codebase".
- [x] Add tests asserting the scorecard has 12 stable ids and `readyFor100` is false while debt remains.

### Task 3: Extract Devtools Style Operation Tests

**Files:**
- Modify: `dx-www/src/cli/devtools/style_ops.rs`
- Create: `dx-www/src/cli/devtools/style_ops_tests.rs`

- [x] Replace the inline `#[cfg(test)] mod tests { ... }` block with the sibling test module declaration.
- [x] Move the existing tests into `style_ops_tests.rs` with `use super::*;`.
- [x] Keep production behavior unchanged.
- [x] Verify `style_ops.rs` drops below the hygiene budget.

### Task 4: Convert The Hygiene Ignore Contract Test To TypeScript

**Files:**
- Rename: `benchmarks/generated-artifact-ignore-contract.test.cjs` to `benchmarks/generated-artifact-ignore-contract.test.ts`
- Modify: `benchmarks/generated-artifact-ignore-contract.test.ts`

- [x] Convert CommonJS imports to ESM imports.
- [x] Replace `__dirname` with `fileURLToPath(import.meta.url)`.
- [x] Preserve every existing assertion and fixture list.
- [x] Run `node --test benchmarks/generated-artifact-ignore-contract.test.ts`.

### Task 5: Document Fixture Ownership And Remaining Debt

**Files:**
- Modify: `docs/repo-hygiene.md`
- Modify: `README.md`

- [x] Add the 12-flaw scorecard explanation.
- [x] Document that root `components/`, `lib/`, `pages/`, and `public/` are source-visible Forge/static proof fixtures, not current WWW app authoring roots.
- [x] Document that `readyFor100` is the only hygiene field allowed to support a repo-wide 100/100 claim.
- [x] Keep language honest about the unsplit large framework files.
- [x] Keep repo hygiene separate from `dx.build.readinessGate` until a real hygiene receipt and combined readiness contract exist.

Task 5 documentation is owned by Agent 4 and is intentionally docs-only. It does
not claim that the scorecard implementation, file splitting, or final
verification tasks are complete.

### Task 6: Verification And Commit

**Files:**
- All files changed by Tasks 2-5.

- [x] Run `node --test benchmarks/repo-hygiene-audit.test.ts`.
- [x] Run `node --test benchmarks/generated-artifact-ignore-contract.test.ts`.
- [x] Run `node --test benchmarks/dx-build-readiness-gate.test.ts`.
- [x] Run `cargo fmt --check` if Rust files changed.
- [x] Run `cargo check -j 1 -p dx-www --no-default-features --features cli --bin dx-www`.
- [x] Run `git diff --check`.
- [x] Run a conflict-marker scan excluding generated/vendor output.
- [x] Commit with `chore: harden repo hygiene scorecard`.
