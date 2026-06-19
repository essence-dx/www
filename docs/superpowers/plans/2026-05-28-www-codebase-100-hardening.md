# DX WWW Codebase 100 Hardening Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Convert "100 out of 100 codebase" from a vague slogan into executable gates that keep DX WWW honest after the hygiene scorecard is closed.

**Architecture:** Keep the closed 12-item hygiene scorecard as one gate, then add follow-up checks only where they are source-owned, testable, and not product/browser/provider overclaims. Do not hide remaining work; classify it as product readiness, fixture/vendor ownership, or future lane work.

**Tech Stack:** Rust workspace, Node native test runner with TypeScript tests, repo-owned hygiene tooling, Superpowers plan and subagent workflow.

---

## Task 1: Re-verify The Closed Hygiene Scorecard

**Files:**
- Read: `tools/hygiene/audit-repo-hygiene.ts`
- Read: `benchmarks/repo-hygiene-audit.test.ts`

- [x] **Step 1: Run the machine audit**

Run:

```powershell
node tools/hygiene/audit-repo-hygiene.ts --json
```

Expected: `blockers: []`, `debt: []`, `scorecardOpen: 0`, `readyFor100: true`.

- [x] **Step 2: Run the executable scorecard tests**

Run:

```powershell
node --test benchmarks/repo-hygiene-audit.test.ts
```

Expected: all 10 subtests pass.

## Task 2: Keep Split Child Files Inside The Same Budgets

**Files:**
- Read: `tools/hygiene/audit-repo-hygiene.ts`
- Read: `dx-www/src/cli/devtools/assets/runtime/*.ts`
- Read: `dx-devtools/styles/devtools/*.css`

- [x] **Step 1: Audit child roots for every split item**

Ensure `largeSourceChildRoots` includes the child directories for all split large-file scorecard entries, including Devtools runtime and CSS fragments.

- [x] **Step 2: Verify no hidden large child file can pass**

Run:

```powershell
node tools/hygiene/audit-repo-hygiene.ts --json
```

Expected: the audit still passes while checking parent files and child split roots.

## Task 3: Keep Docs From Overclaiming Product Readiness

**Files:**
- Modify: `docs/repo-hygiene.md`
- Modify: `docs/superpowers/plans/2026-05-28-www-complete-hygiene-scorecard.md`

- [x] **Step 1: Rewrite stale active-failure wording**

The stable scorecard id list must describe what each gate tracks, not claim already-closed files are still too large.

- [x] **Step 2: Name actual landed split files**

The plan must name the real landed split directories: `mod_parts`, `source_render_parts`, `panel_parts`, `readiness_parts`, and `forge_registry_parts`.

- [x] **Step 3: Preserve the boundary**

Docs must say `readyFor100` is a hygiene-readiness claim only. It must not claim browser proof, provider proof, full runtime parity, or launch readiness.

## Task 4: Run Rust And Repository Safety Checks

**Files:**
- Read: Rust workspace files
- Read: git worktree

- [x] **Step 1: Format check**

Run:

```powershell
cargo fmt --check
```

Expected: exit 0.

- [x] **Step 2: CLI compile check**

Run:

```powershell
cargo check -j 1 -p dx-www --no-default-features --features cli --bin dx-www
```

Expected: exit 0.

- [x] **Step 3: Whitespace and conflict marker checks**

Run:

```powershell
git diff --check
rg -n "^(<<<<<<<|=======|>>>>>>>)" --glob '!target/**' --glob '!node_modules/**' --glob '!.dx/www/output/**' --glob '!.git/**'
```

Expected: no whitespace errors and no conflict markers.

## Task 5: Report Remaining Non-Hygiene Work Honestly

**Files:**
- Read: `PLAN.md`
- Modify: `docs/repo-hygiene.md`

- [x] **Step 1: Check root plan status**

Read `PLAN.md`. It is an auto-import production plan with unchecked implementation tasks, so it is future product work rather than a closed hygiene gate.

- [x] **Step 2: Final report boundary**

Final reporting must distinguish:

- closed 12-item hygiene scorecard
- this follow-up audit hardening
- remaining product work such as auto-import parity, browser proof, provider proof, and full launch readiness

- [x] **Step 3: Record the non-scorecard backlog**

Add a concise `Non-Scorecard Backlog` section to `docs/repo-hygiene.md` covering
the next known large or ambiguous areas without reopening the closed 12-item
scorecard.

## Task 6: Fix Rustdoc Warnings Found During Verification

**Files:**
- Modify: `core/src/codegen.rs`
- Modify: `core/src/turso.rs`

- [x] **Step 1: Escape generic and URL placeholders in doc comments**

Wrap `Vec<u8>` / `Vec<String>` and `turso://<db>.turso.io` examples in prose or
code spans so rustdoc no longer treats them as invalid HTML tags.

- [x] **Step 2: Re-run rustdoc verification**

Run:

```powershell
cargo doc -p dx-www-compiler --features oxc --no-deps -j1 --message-format=short
```

Expected: exit 0 without the previous HTML-tag warnings for `core/src/codegen.rs`
or `core/src/turso.rs`.

## Task 7: Commit The Follow-Up

**Files:**
- Modify: `tools/hygiene/audit-repo-hygiene.ts`
- Modify: `docs/repo-hygiene.md`
- Modify: `docs/superpowers/plans/2026-05-28-www-complete-hygiene-scorecard.md`
- Create: `docs/superpowers/plans/2026-05-28-www-codebase-100-hardening.md`
- Modify: `core/src/codegen.rs`
- Modify: `core/src/turso.rs`

- [x] **Step 1: Stage only the hardening files**

Run:

```powershell
git add tools/hygiene/audit-repo-hygiene.ts benchmarks/repo-hygiene-audit.test.ts docs/repo-hygiene.md docs/superpowers/plans/2026-05-28-www-complete-hygiene-scorecard.md docs/superpowers/plans/2026-05-28-www-codebase-100-hardening.md core/src/codegen.rs core/src/turso.rs
```

Expected: the unrelated dossier file remains untracked.

- [x] **Step 2: Commit**

Run:

```powershell
git commit -m "chore: harden codebase readiness boundaries"
```

Expected: a focused commit containing only this follow-up hardening pass.
