# WWW CJS Benchmark Scorecard Pass Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Reduce `legacy-script-extensions` further by migrating top-level `benchmarks/*.test.cjs` files to `.test.ts` without rewriting their CommonJS helper-module dependencies.

**Architecture:** Each renamed benchmark gets a small ESM bridge using `createRequire(import.meta.url)` and `import.meta.dirname`. The pass updates source-owned references to renamed benchmark paths and keeps the hygiene audit honest because non-benchmark `.cjs`/`.js` files still remain.

**Tech Stack:** Node test runner, TypeScript erasable syntax, CommonJS bridge via `node:module`, PowerShell, Git.

---

### Task 1: Rename And Bridge CJS Benchmarks

**Files:**
- Rename: `benchmarks/*.test.cjs` to matching `benchmarks/*.test.ts`

- [ ] **Step 1: Rename every top-level benchmark CJS test**

Run:

```powershell
Get-ChildItem -Path benchmarks -Filter *.test.cjs | ForEach-Object {
  git mv $_.FullName ($_.FullName -replace '\.test\.cjs$', '.test.ts')
}
```

Expected: `git status --short` shows rename entries for the migrated files.

- [ ] **Step 2: Add the CommonJS bridge to renamed files**

For every file renamed in this task, prepend:

```ts
import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;
```

Expected: old `require(...)` and `__dirname` usage keep working when the file is executed as `.ts`.

### Task 2: Update References And Hygiene Proof

**Files:**
- Modify: source-owned references to renamed `benchmarks/*.test.cjs` paths.
- Modify: `benchmarks/repo-hygiene-audit.test.ts`
- Modify: `docs/repo-hygiene.md`

- [ ] **Step 1: Update renamed-path references**

Replace references to old top-level benchmark `.test.cjs` filenames with `.test.ts`.

Expected: active source-owned tools/tests do not point at removed benchmark paths.

- [ ] **Step 2: Add a CJS benchmark regression guard**

Extend `benchmarks/repo-hygiene-audit.test.ts` so it asserts no top-level `benchmarks/*.test.cjs` files exist, while still asserting `legacy-script-extensions` and `readyFor100` remain active/false.

Expected: the migration is guarded without overclaiming full readiness.

- [ ] **Step 3: Document partial progress**

Update `docs/repo-hygiene.md` to say both `.test.mjs` and `.test.cjs` top-level benchmark batches moved to `.test.ts`, but runtime `.js`, tool `.cjs`, nested fixtures, and large Rust files still block 100/100.

Expected: docs distinguish real progress from final readiness.

### Task 3: Verification And Commit

**Files:**
- Verify: renamed benchmark syntax
- Verify: focused migrated test subset
- Verify: hygiene audit and diff checks

- [ ] **Step 1: Syntax-check all newly renamed tests**

Run:

```powershell
$renamed = @(
  git diff --cached --name-status --diff-filter=R -- benchmarks
  git diff --name-status --diff-filter=R -- benchmarks
) |
  ForEach-Object { ($_ -split "`t")[-1] } |
  Where-Object { $_ -like "benchmarks/*.test.ts" } |
  Sort-Object -Unique

foreach ($test in $renamed) { node --check $test }
```

Expected: every renamed file parses under Node.

- [ ] **Step 2: Run focused tests**

Run:

```powershell
node --test benchmarks/repo-hygiene-audit.test.ts
node --test benchmarks/app-router-build-output-shared-segments.test.ts benchmarks/benchmark-report-scope-contract.test.ts
```

Expected: the hygiene audit and a representative migrated CommonJS-bridge subset pass.

- [ ] **Step 3: Run final checks**

Run:

```powershell
node tools/hygiene/audit-repo-hygiene.ts --json
git diff --check
git diff --cached --check
rg "^(<<<<<<<|=======|>>>>>>>)" --glob "!target/**" --glob "!node_modules/**" --glob "!.dx/www/output/**" --glob "!.git/**"
```

Expected: `legacyScripts` is lower than `306`, no blockers appear, whitespace is clean, and conflict-marker scan has no matches.

- [ ] **Step 4: Commit**

Run:

```powershell
git add benchmarks docs/repo-hygiene.md docs/superpowers/plans/2026-05-28-www-cjs-benchmark-scorecard-pass.md
git commit -m "chore: migrate cjs benchmark guards to ts"
```

Expected: one focused commit after checkpoint `975d5dc6`.

## Execution Notes

- The checkpoint commit for this pass is `975d5dc6`.
- Renamed 150 top-level `benchmarks/*.test.cjs` files to `.test.ts`.
- Added a `createRequire(import.meta.url)` bridge to each renamed file so
  existing CommonJS helper-module imports remain source-compatible.
- `node --check` passed for all 150 renamed files.
- Focused migrated tests passed:
  `repo-hygiene-audit.test.ts`,
  `app-router-build-output-shared-segments.test.ts`,
  `benchmark-report-scope-contract.test.ts`,
  `next-rust-merge-audit-comparison.test.ts`, and
  `next-rust-conflict-markers.test.ts`.
- The hygiene audit now reports `legacyScripts: 156`, down from `306` after the
  `.mjs` batch and `349` before both benchmark-extension batches.
- `readyFor100` remains `false`; large Rust files, source-visible fixtures,
  tool/runtime legacy scripts, and readiness-overclaim risk are still active.
