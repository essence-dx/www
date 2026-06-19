# WWW Legacy Scorecard Pass Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Reduce the active `legacy-script-extensions` hygiene flaw by migrating source-owned benchmark `.test.mjs` files to `.test.ts` without hiding the remaining repo-wide debt.

**Architecture:** This pass keeps runtime behavior unchanged. Benchmark test files are renamed to TypeScript extension paths, stale source references are updated, and the hygiene audit gains an explicit guard so `.test.mjs` does not return under `benchmarks/`.

**Tech Stack:** Node test runner, TypeScript erasable syntax under Node, PowerShell, Git, DX WWW hygiene audit.

---

### Task 1: Inventory And Agent Review

**Files:**
- Read: `benchmarks/*.test.mjs`
- Read: `tools/hygiene/audit-repo-hygiene.ts`
- Read: `docs/repo-hygiene.md`

- [ ] **Step 1: Confirm `.mjs` benchmark inventory**

Run:

```powershell
Get-ChildItem -Path benchmarks -Filter *.test.mjs | Select-Object -ExpandProperty Name
```

Expected: the command lists the benchmark `.mjs` files that are in scope for this pass.

- [ ] **Step 2: Dispatch six scoped agents**

Use exactly six existing GPT-5.5 extra-high agents. Give each agent a disjoint read-only scope and ask for risks only:

```text
Agent 1: route/provider .mjs benchmark files.
Agent 2: CLI forge options .mjs benchmark files.
Agent 3: CLI non-forge .mjs benchmark files.
Agent 4: diagnostics and dx-style .mjs benchmark files.
Agent 5: stale references in docs/tools/benchmarks.
Agent 6: focused verification commands and audit risks.
```

Expected: agents report whether any file needs syntax conversion beyond extension rename.

### Task 2: Rename `.test.mjs` Benchmarks

**Files:**
- Rename: `benchmarks/*.test.mjs` to matching `benchmarks/*.test.ts`

- [ ] **Step 1: Rename with git**

Run one `git mv` operation per file:

```powershell
Get-ChildItem -Path benchmarks -Filter *.test.mjs | ForEach-Object {
  git mv $_.FullName ($_.FullName -replace '\.test\.mjs$', '.test.ts')
}
```

Expected: `git status --short` shows `R` entries from `.test.mjs` to `.test.ts`.

- [ ] **Step 2: Run renamed tests**

Run:

```powershell
$tests = Get-ChildItem -Path benchmarks -Filter *.test.ts |
  Where-Object { $_.Name -in @(
    'ai-route-handler-provider-boundary.test.ts',
    'app-api-route-handler-extensions.test.ts',
    'app-router-page-extensions-build-loop.test.ts',
    'diagnostics-code-frame-contract.test.ts',
    'dx-style-compile-boundary.test.ts',
    'route-handler-readiness-interpreters.test.ts'
  ) } |
  Select-Object -ExpandProperty FullName
node --test $tests
```

Expected: representative renamed tests pass before broader reference updates.

### Task 3: Update Stale Source References

**Files:**
- Modify: benchmark files that contain string references to renamed `.test.mjs` paths.
- Modify: `tools/launch-stabilize/coordinator.cjs` if it references renamed benchmark paths.
- Do not modify: `docs/DX_WWW_CURRENT_FEATURE_DOSSIER_2026-05-28.md` because it is unrelated untracked user work.

- [ ] **Step 1: Find stale `.test.mjs` references**

Run:

```powershell
rg "\.test\.mjs" benchmarks tools docs --glob "!docs/DX_WWW_CURRENT_FEATURE_DOSSIER_2026-05-28.md"
```

Expected: every hit is reviewed and either updated to `.test.ts` or intentionally left with a reason.

- [ ] **Step 2: Patch path strings**

For path strings that refer to renamed files, replace `.test.mjs` with `.test.ts`.

Expected: source-owned tests and tools no longer point at the removed paths.

### Task 4: Strengthen Hygiene Proof

**Files:**
- Modify: `benchmarks/repo-hygiene-audit.test.ts`
- Modify: `docs/repo-hygiene.md`

- [ ] **Step 1: Add benchmark `.mjs` regression guard**

Add a test assertion to `benchmarks/repo-hygiene-audit.test.ts`:

```ts
test("benchmark tests do not use .test.mjs extensions", () => {
  const benchmarkMjsTests = fs
    .readdirSync(path.join(repoRoot, "benchmarks"))
    .filter((name) => name.endsWith(".test.mjs"));

  assert.deepEqual(benchmarkMjsTests, []);
});
```

Expected: the test fails if a `.test.mjs` benchmark returns.

- [ ] **Step 2: Update repo hygiene docs**

Record that the `.mjs` benchmark batch is closed while the broader `legacy-script-extensions` flaw remains active because `.cjs` and runtime `.js` scripts still exist.

Expected: docs are honest and do not claim repo-wide 100/100 readiness.

### Task 5: Focused Verification And Commit

**Files:**
- Verify: renamed benchmark tests
- Verify: `benchmarks/repo-hygiene-audit.test.ts`
- Verify: whitespace and conflict markers

- [ ] **Step 1: Run focused tests**

Run:

```powershell
node --test benchmarks/*.test.ts
node --test benchmarks/repo-hygiene-audit.test.ts
```

Expected: all selected tests pass.

- [ ] **Step 2: Run hygiene audit**

Run:

```powershell
node tools/hygiene/audit-repo-hygiene.ts --json
```

Expected: `legacyScripts` is lower than the previous baseline of `349`, `readyFor100` remains `false`, and no blockers appear.

- [ ] **Step 3: Run final lightweight checks**

Run:

```powershell
git diff --check
rg "<<<<<<<|=======|>>>>>>>" --glob "!target/**" --glob "!node_modules/**" --glob "!.dx/www/output/**"
```

Expected: no whitespace errors and no conflict markers.

- [ ] **Step 4: Commit**

Run:

```powershell
git add benchmarks tools docs/repo-hygiene.md docs/superpowers/plans/2026-05-28-www-legacy-scorecard-pass.md
git commit -m "chore: migrate mjs benchmark guards to ts"
```

Expected: one focused commit after the checkpoint commit.

## Execution Notes

- The checkpoint commit for this pass is `fbfb527e`.
- The `.test.mjs` benchmark migration renamed 43 files to `.test.ts`.
- `node --check` passed for all 43 renamed files.
- `node --test benchmarks/repo-hygiene-audit.test.ts` passed after adding the
  regression guard.
- A focused renamed subset passed:
  `ai-route-handler-provider-boundary.test.ts`,
  `app-api-route-handler-extensions.test.ts`,
  `dx-style-compile-boundary.test.ts`,
  `dx-www-parser-launch-extensions.test.ts`, and
  `route-handler-head-method-parity.test.ts`.
- Broad renamed-test execution is not claimed green. Existing source-contract
  drift remains in at least `app-router-page-extensions-build-loop.test.ts`,
  `diagnostics-code-frame-contract.test.ts`,
  `cli-template-options-split-safety.test.ts`, and
  `route-handler-readiness-interpreters.test.ts`.
